use std::collections::BTreeSet;

use crate::combat::AttackProfile;
use crate::world::{
    Direction, DistanceMetric, FieldOfViewRules, GridPos, Map, compute_visible_tiles, find_path,
};

use super::{AiBehavior, AiProfile};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AiAction {
    Wait,
    Move(Direction),
    Attack { slot: u8 },
}

pub struct AiSituation<'a> {
    pub map: &'a Map,
    pub actor_position: GridPos,
    pub home_position: Option<GridPos>,
    pub target_position: GridPos,
    pub occupied_positions: &'a BTreeSet<GridPos>,
    pub profile: AiProfile,
    pub preferred_attack: Option<AttackProfile>,
}

pub fn decide_action(situation: AiSituation<'_>) -> AiAction {
    decide_action_with_visibility(situation, false)
}

/// Follows a remembered position without granting knowledge of the player.
/// Callers must omit an attack while searching, so this can only navigate.
pub fn decide_known_action(situation: AiSituation<'_>) -> AiAction {
    decide_action_with_visibility(situation, true)
}

fn decide_action_with_visibility(situation: AiSituation<'_>, target_is_known: bool) -> AiAction {
    if situation.profile.behavior == AiBehavior::Idle {
        return AiAction::Wait;
    }

    let visible = compute_visible_tiles(
        situation.map,
        situation.actor_position,
        FieldOfViewRules {
            radius: situation.profile.perception_radius,
            distance_metric: DistanceMetric::Euclidean,
            block_closed_corners: true,
        },
    );
    let target_visible = target_is_known
        || (visible.contains(&situation.target_position)
            && !situation.map.is_protected(situation.target_position));
    if let Some((home, maximum_distance)) = pursuit_leash(&situation) {
        let target_inside_leash =
            chebyshev_distance(home, situation.target_position) <= u32::from(maximum_distance);
        if !target_visible || !target_inside_leash {
            return move_toward(&situation, home, Some((home, maximum_distance)));
        }
    } else if !target_visible {
        return AiAction::Wait;
    }

    let target_distance = chebyshev_distance(situation.actor_position, situation.target_position);
    if situation.profile.behavior == AiBehavior::Skirmisher
        && target_distance < u32::from(situation.profile.preferred_minimum_distance)
        && let Some(direction) = retreat_direction(&situation)
    {
        return AiAction::Move(direction);
    }

    if situation.preferred_attack.is_some_and(|attack| {
        attack.is_in_range(situation.actor_position, situation.target_position)
    }) {
        return AiAction::Attack {
            slot: situation.profile.preferred_attack_slot,
        };
    }

    if situation.profile.behavior == AiBehavior::Sentry {
        return AiAction::Wait;
    }

    let leash = pursuit_leash(&situation);
    move_toward(&situation, situation.target_position, leash)
}

fn pursuit_leash(situation: &AiSituation<'_>) -> Option<(GridPos, u16)> {
    situation
        .home_position
        .zip(situation.profile.maximum_pursuit_distance())
}

fn move_toward(
    situation: &AiSituation<'_>,
    destination: GridPos,
    leash: Option<(GridPos, u16)>,
) -> AiAction {
    if situation.actor_position == destination {
        return AiAction::Wait;
    }
    let path = find_path(
        situation.map,
        situation.actor_position,
        destination,
        situation.profile.maximum_path_search,
        |position| {
            !situation.occupied_positions.contains(&position)
                && !situation.map.is_protected(position)
                && leash.is_none_or(|(home, maximum_distance)| {
                    chebyshev_distance(home, position) <= u32::from(maximum_distance)
                })
        },
    );
    let Some(next_position) = path.and_then(|path| path.get(1).copied()) else {
        return AiAction::Wait;
    };
    let delta_x = next_position.x - situation.actor_position.x;
    let delta_y = next_position.y - situation.actor_position.y;

    Direction::from_delta(delta_x, delta_y)
        .map(AiAction::Move)
        .unwrap_or(AiAction::Wait)
}

fn retreat_direction(situation: &AiSituation<'_>) -> Option<Direction> {
    let mut best = None;
    let mut best_distance = chebyshev_distance(situation.actor_position, situation.target_position);

    for direction in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        let destination = situation.actor_position.step(direction);
        if !situation.map.is_walkable(destination)
            || situation.map.is_protected(destination)
            || situation.occupied_positions.contains(&destination)
            || pursuit_leash(situation).is_some_and(|(home, maximum_distance)| {
                chebyshev_distance(home, destination) > u32::from(maximum_distance)
            })
        {
            continue;
        }
        let distance = chebyshev_distance(destination, situation.target_position);
        if distance > best_distance {
            best = Some(direction);
            best_distance = distance;
        }
    }

    best
}

fn chebyshev_distance(first: GridPos, second: GridPos) -> u32 {
    let delta_x = (i64::from(first.x) - i64::from(second.x)).unsigned_abs();
    let delta_y = (i64::from(first.y) - i64::from(second.y)).unsigned_abs();
    delta_x.max(delta_y).min(u64::from(u32::MAX)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;

    fn corridor() -> Map {
        Map::from_ascii("#######\n#.....#\n#######")
            .unwrap_or_else(|error| panic!("valid test map failed to parse: {error}"))
    }

    #[test]
    fn sentry_does_not_move_when_target_is_out_of_weapon_range() {
        let occupied = BTreeSet::new();
        let action = decide_action(AiSituation {
            map: &corridor(),
            actor_position: GridPos::new(5, 1),
            home_position: None,
            target_position: GridPos::new(1, 1),
            occupied_positions: &occupied,
            profile: AiProfile::sentry(8, 0),
            preferred_attack: Some(AttackProfile::melee(DamageType::Kinetic, 2)),
        });

        assert_eq!(action, AiAction::Wait);
    }

    #[test]
    fn skirmisher_retreats_when_target_is_too_close() {
        let occupied = BTreeSet::new();
        let action = decide_action(AiSituation {
            map: &corridor(),
            actor_position: GridPos::new(2, 1),
            home_position: None,
            target_position: GridPos::new(1, 1),
            occupied_positions: &occupied,
            profile: AiProfile::skirmisher(8, 0, 2),
            preferred_attack: Some(AttackProfile::new(
                5,
                DistanceMetric::Chebyshev,
                true,
                DamageType::Kinetic,
                2,
                0,
            )),
        });

        assert_eq!(action, AiAction::Move(Direction::East));
    }

    #[test]
    fn leashed_hunter_returns_home_when_the_target_is_too_far_away() {
        let occupied = BTreeSet::new();
        let profile = AiProfile::hunter(8, 0).with_maximum_pursuit_distance(
            std::num::NonZeroU16::new(3).expect("constant is non-zero"),
        );
        let action = decide_action(AiSituation {
            map: &corridor(),
            actor_position: GridPos::new(4, 1),
            home_position: Some(GridPos::new(5, 1)),
            target_position: GridPos::new(1, 1),
            occupied_positions: &occupied,
            profile,
            preferred_attack: Some(AttackProfile::melee(DamageType::Kinetic, 2)),
        });

        assert_eq!(action, AiAction::Move(Direction::East));
    }

    #[test]
    fn leashed_hunter_never_steps_beyond_its_territory() {
        let occupied = BTreeSet::new();
        let profile = AiProfile::hunter(8, 0).with_maximum_pursuit_distance(
            std::num::NonZeroU16::new(2).expect("constant is non-zero"),
        );
        let action = decide_action(AiSituation {
            map: &corridor(),
            actor_position: GridPos::new(3, 1),
            home_position: Some(GridPos::new(5, 1)),
            target_position: GridPos::new(1, 1),
            occupied_positions: &occupied,
            profile,
            preferred_attack: Some(AttackProfile::melee(DamageType::Kinetic, 2)),
        });

        assert_eq!(action, AiAction::Move(Direction::East));
    }

    #[test]
    fn leashed_hunter_returns_home_after_losing_sight_of_the_target() {
        let occupied = BTreeSet::new();
        let profile = AiProfile::hunter(2, 0).with_maximum_pursuit_distance(
            std::num::NonZeroU16::new(5).expect("constant is non-zero"),
        );
        let action = decide_action(AiSituation {
            map: &corridor(),
            actor_position: GridPos::new(4, 1),
            home_position: Some(GridPos::new(5, 1)),
            target_position: GridPos::new(1, 1),
            occupied_positions: &occupied,
            profile,
            preferred_attack: Some(AttackProfile::melee(DamageType::Kinetic, 2)),
        });

        assert_eq!(action, AiAction::Move(Direction::East));
    }

    #[test]
    fn leashed_skirmisher_does_not_retreat_beyond_its_territory() {
        let occupied = BTreeSet::new();
        let profile = AiProfile::skirmisher(8, 0, 2).with_maximum_pursuit_distance(
            std::num::NonZeroU16::new(2).expect("constant is non-zero"),
        );
        let action = decide_action(AiSituation {
            map: &corridor(),
            actor_position: GridPos::new(3, 1),
            home_position: Some(GridPos::new(5, 1)),
            target_position: GridPos::new(4, 1),
            occupied_positions: &occupied,
            profile,
            preferred_attack: Some(AttackProfile::melee(DamageType::Kinetic, 2)),
        });

        assert_eq!(action, AiAction::Attack { slot: 0 });
    }
}
