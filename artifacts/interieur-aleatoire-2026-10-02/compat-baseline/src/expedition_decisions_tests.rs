//! Diagnostic decisions only: no player automation or combat rule changes.
use super::*;
use project_rl::entity::{InventoryEntry, MagicItemModifiers};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

/// Bound local shelter detours per region visit. Without this memory, repeatedly
/// seeking cover can cancel route progress forever. This is a pilot safeguard,
/// not a combat rule; warning escapes remain independent of this budget.
#[derive(Default)]
pub(super) struct CoverBudget(u8);

impl CoverBudget {
    pub(super) fn available(&self) -> bool {
        self.0 < 24
    }

    pub(super) fn record_move(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub(super) fn enter_region(&mut self) {
        self.0 = 0;
    }
}

#[test]
fn expedition_cover_budget_bounds_repeated_shelter_detours() {
    let mut budget = CoverBudget::default();
    for _ in 0..24 {
        assert!(budget.available());
        budget.record_move();
    }
    assert!(!budget.available());
    // Ordinary route steps or healing must not reset the budget and recreate
    // the cover / route / cover oscillation in the same region visit.
    assert!(!budget.available());
    budget.enter_region();
    assert!(budget.available());
}

fn bonuses(entry: &InventoryEntry) -> [u16; 11] {
    let mut values = [0; 11];
    if let Some(m) = entry.magic_modifiers() {
        for (i, attribute) in project_rl::stats::PrimaryAttribute::ALL
            .into_iter()
            .enumerate()
        {
            values[i] = u16::from(m.attribute_bonus(attribute));
        }
        values[5..].copy_from_slice(&[
            m.accuracy_bonus(),
            m.armor_penetration_bonus(),
            m.maximum_hit_points_bonus(),
            m.energy_capacity_bonus(),
            m.heat_dissipation_bonus(),
            m.armor_bonus(),
        ]);
    }
    values
}

/// Same base, no lost numeric bonus, same special effect. No arbitrary exchange
/// rate between HP and damage, no change of ammunition model or combat role.
pub(super) fn weapon_upgrade(game: &WorldState) -> Option<GameCommand> {
    for slot in 0..2 {
        let current = game
            .player_inventory()
            .get(game.equipped_player_weapon_item(slot)?)?;
        let base = bonuses(current);
        let effect = current
            .magic_modifiers()
            .and_then(|m| m.effect_affix().cloned());
        if let Some(candidate) = game
            .player_inventory()
            .iter()
            .filter(|e| {
                let other = bonuses(e);
                e.item() == current.item()
                    && game.player_equipment().slot_of(e.instance()).is_none()
                    && e.magic_modifiers().and_then(|m| m.effect_affix().cloned()) == effect
                    && other.iter().zip(base).all(|(&a, b)| a >= b)
                    && other != base
            })
            .min_by_key(|e| e.instance())
        {
            return Some(GameCommand::EquipWeapon {
                slot,
                item: candidate.instance(),
            });
        }
    }
    None
}

fn exposure(game: &WorldState, at: GridPos, visible: &[(EntityId, GridPos)]) -> u32 {
    if game.map().is_protected(at) {
        return 0;
    }
    visible
        .iter()
        .filter(|(id, from)| {
            if game.map().is_protected(*from) {
                return false;
            }
            game.actors().get(*id).is_some_and(|actor| {
                actor.attacks().iter().any(|attack| {
                    attack.is_in_range(*from, at)
                        && (!attack.requires_line_of_sight()
                            || project_rl::world::has_line_of_sight(game.map(), *from, at, true))
                })
            })
        })
        .count() as u32
}

/// Seek a reachable shelter within six real cardinal steps. A firing lane is
/// a conservative potential attack, not a prediction of AI or hit rolls.
/// Only supplied visible hostiles enter the threat estimate; terrain is known
/// as in the baseline pilot. Warnings, walls, doors and actors block the search.
pub(super) fn cover_step(
    game: &WorldState,
    goal: GridPos,
    warnings: &BTreeSet<GridPos>,
    visible: &[(EntityId, GridPos)],
) -> Option<GameCommand> {
    let start = game.player_position()?;
    if exposure(game, start, visible) == 0 {
        return None;
    }
    let distance = |p: GridPos| p.x.abs_diff(goal.x) + p.y.abs_diff(goal.y);
    let mut open = BinaryHeap::from([Reverse((0_u32, distance(start), start, 0_u8, start))]);
    let mut costs = BTreeMap::from([((start, 0_u8), 0_u32)]);
    let mut risks = BTreeMap::new();
    while let Some(Reverse((cost, _, at, steps, first))) = open.pop() {
        if costs.get(&(at, steps)) != Some(&cost) {
            continue;
        }
        let risk = *risks
            .entry(at)
            .or_insert_with(|| exposure(game, at, visible));
        if steps > 0 && risk == 0 {
            return Some(GameCommand::Move(Direction::from_delta(
                first.x - start.x,
                first.y - start.y,
            )?));
        }
        if steps == 6 {
            continue;
        }
        for next in at.cardinal_neighbors() {
            if next == start
                || !game.map().is_walkable(next)
                || warnings.contains(&next)
                || game.actors().entity_at(next).is_some()
            {
                continue;
            }
            let risk = *risks
                .entry(next)
                .or_insert_with(|| exposure(game, next, visible));
            let next_cost = cost + 1 + 6 * risk;
            let key = (next, steps + 1);
            if costs.get(&key).is_none_or(|&old| next_cost < old) {
                costs.insert(key, next_cost);
                open.push(Reverse((
                    next_cost,
                    distance(next),
                    next,
                    steps + 1,
                    if steps == 0 { next } else { first },
                )));
            }
        }
    }
    None
}

#[test]
fn expedition_cover_uses_walls_without_crossing_actors_or_warnings() {
    let map = project_rl::world::Map::from_ascii(
        "#########\n#...#...#\n#...#...#\n#...#...#\n#.......#\n#########",
    )
    .unwrap();
    let mut game = WorldState::single(
        GameState::new_with_rules(map, GridPos::new(3, 4), 0, GameRules::default()).unwrap(),
    );
    let target =
        game.spawn_actor(Actor::new(GridPos::new(7, 4), 20).unwrap().with_attack(
            AttackProfile::new(
                7,
                DistanceMetric::Euclidean,
                true,
                DamageType::Piercing,
                2,
                0,
            ),
        ))
        .unwrap();
    let visible = [(target, GridPos::new(7, 4))];
    let goal = GridPos::new(1, 4);
    assert_eq!(
        cover_step(&game, goal, &BTreeSet::new(), &visible),
        Some(GameCommand::Move(Direction::North))
    );
    let warnings = BTreeSet::from([GridPos::new(3, 3)]);
    assert_ne!(
        cover_step(&game, goal, &warnings, &visible),
        Some(GameCommand::Move(Direction::North))
    );
    game.spawn_actor(Actor::new(GridPos::new(3, 3), 10).unwrap())
        .unwrap();
    assert_ne!(
        cover_step(&game, goal, &BTreeSet::new(), &visible),
        Some(GameCommand::Move(Direction::North))
    );
    assert_eq!(
        cover_step(&game, goal, &BTreeSet::new(), &[]),
        None,
        "no hidden threat knowledge"
    );
}

#[test]
fn expedition_weapon_swap_keeps_spent_ammo_and_does_not_heal() {
    let (rules, _, _, _) = ascii_game_content().unwrap();
    let mut rules = super::expedition_survival_tests::finite_rules(&rules);
    rules.hit_rules = None;
    let rifle: ContentId = "core:fusil_de_patrouille".parse().unwrap();
    let game = GameState::new_with_rules(
        project_rl::world::Map::filled(9, 3, project_rl::world::Terrain::Floor).unwrap(),
        GridPos::new(1, 1),
        0,
        rules.clone(),
    )
    .unwrap()
    .with_starting_magic_equipment([(
        rifle.clone(),
        MagicItemModifiers::rpg_bonuses([0, 0, 2, 0, 2], 0, 0, 0, 0, 0).unwrap(),
    )])
    .unwrap();
    let mut game = WorldState::single(game);
    let target = game
        .spawn_actor(Actor::new(GridPos::new(5, 1), 100).unwrap())
        .unwrap();
    assert_eq!(
        game.process_player_command(GameCommand::Attack { slot: 1, target }),
        CommandOutcome::Applied
    );
    let before = (
        super::expedition_survival_tests::hp(&game),
        game.player_weapon_ammunition(&rifle),
        game.player_inventory().clone(),
    );
    assert_eq!(before.1, Some((11, 12)));
    let command = weapon_upgrade(&game).unwrap();
    assert!(matches!(command, GameCommand::EquipWeapon { slot: 1, .. }));
    assert_eq!(
        game.process_player_command(command),
        CommandOutcome::Applied
    );
    assert_eq!(
        before,
        (
            super::expedition_survival_tests::hp(&game),
            game.player_weapon_ammunition(&rifle),
            game.player_inventory().clone()
        )
    );
    assert_eq!(
        game.actors()
            .get(game.player_id())
            .unwrap()
            .maximum_integrity(),
        30
    );
    assert_eq!(
        weapon_upgrade(&game),
        None,
        "never oscillate back to the ordinary copy"
    );
    game.drain_events();
    let bytes = game.recovery_snapshot_bytes().unwrap();
    let restored = WorldState::from_recovery_snapshot_bytes(&bytes, rules).unwrap();
    assert_eq!(bytes, restored.recovery_snapshot_bytes().unwrap());
    assert_eq!(weapon_upgrade(&restored), None);
}
