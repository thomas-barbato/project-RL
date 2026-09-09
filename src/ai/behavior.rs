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
    pub target_position: GridPos,
    pub occupied_positions: &'a BTreeSet<GridPos>,
    pub profile: AiProfile,
    pub preferred_attack: Option<AttackProfile>,
}

pub fn decide_action(situation: AiSituation<'_>) -> AiAction {
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
    if !visible.contains(&situation.target_position) {
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

    let path = find_path(
        situation.map,
        situation.actor_position,
        situation.target_position,
        situation.profile.maximum_path_search,
        |position| !situation.occupied_positions.contains(&position),
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
            || situation.occupied_positions.contains(&destination)
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
}
