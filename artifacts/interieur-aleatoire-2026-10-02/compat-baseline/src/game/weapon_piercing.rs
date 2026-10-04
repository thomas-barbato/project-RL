//! Deterministic impact continuation. No visibility filter or secondary attack.
use super::*;
use crate::world::PropagationCell;

fn piercing_cells(
    map: &Map,
    origin: GridPos,
    impact: GridPos,
    length: u16,
) -> Vec<PropagationCell> {
    let dx = i64::from(impact.x) - i64::from(origin.x);
    let dy = i64::from(impact.y) - i64::from(origin.y);
    let major = dx.abs().max(dy.abs());
    if major == 0 || !map.contains(impact) {
        return Vec::new();
    }
    let mut cells = Vec::new();
    let mut previous = impact;
    for step in 1..=length {
        // Symmetric nearest-cell rasterization; oblique shots retain their
        // slope instead of snapping to one of eight directions. i64 prevents
        // overflow even for extreme i32 coordinates and maximum u16 lengths.
        let offset =
            |delta: i64| ((delta.abs() * i64::from(step) + major / 2) / major) * delta.signum();
        let (Ok(x), Ok(y)) = (
            i32::try_from(i64::from(impact.x) + offset(dx)),
            i32::try_from(i64::from(impact.y) + offset(dy)),
        ) else {
            break;
        };
        let at = GridPos::new(x, y);
        if !map.contains(at)
            || map.blocks_vision(at)
            || map.is_protected(at)
            || !has_line_of_sight(map, previous, at, true)
        {
            break;
        }
        cells.push(PropagationCell {
            position: at,
            cost: step,
            step,
        });
        previous = at;
    }
    cells
}

impl GameState {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn apply_weapon_piercing(
        &mut self,
        attacker: EntityId,
        origin: GridPos,
        impact: GridPos,
        length: u16,
        damage: DamagePacket,
        primary_targets: &BTreeSet<EntityId>,
        weapon_effect: Option<(WeaponId, usize)>,
    ) {
        let cells = piercing_cells(&self.map, origin, impact, length);
        if cells.is_empty() {
            return;
        }
        // Snapshot recipients before deaths, explosions or reactions can move
        // another recipient onto a later cell and accidentally hit it twice.
        let targets: Vec<EntityId> = cells
            .iter()
            .filter_map(|cell| self.actors.entity_at(cell.position))
            .filter(|target| *target != attacker && !primary_targets.contains(target))
            .collect();
        self.events.push(GameEvent::PropagationResolved {
            source: Some(attacker),
            origin: impact,
            cells,
            weapon_effect,
        });
        for target in targets {
            let _ = self.apply_damage_to(Some(attacker), target, damage);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{ArmorRules, ConeAttack, DamageType, HitRules};
    use crate::weapon::WeaponCatalog;

    fn fixture(delivery: AttackDelivery, trigger: WeaponEffectTrigger, cone: bool) -> GameState {
        let mut attack = AttackProfile::new(
            10,
            DistanceMetric::Chebyshev,
            true,
            DamageType::Kinetic,
            5,
            0,
        )
        .with_delivery(delivery);
        if cone {
            attack = attack.with_area(AttackArea::Cone(ConeAttack::new(1, 1, 1).unwrap()));
        }
        let id: WeaponId = "test:piercing".parse().unwrap();
        let mut weapons = WeaponCatalog::default();
        weapons
            .register(
                WeaponDefinition::new(id.clone(), "name".into(), "description".into(), attack)
                    .unwrap()
                    .with_effects([WeaponEffect::piercing_line(
                        3,
                        DamagePacket::new(3, DamageType::Piercing, 0),
                        trigger,
                    )
                    .unwrap()]),
            )
            .unwrap();
        GameState::new_with_rules(
            Map::filled(18, 12, Terrain::Floor).unwrap(),
            GridPos::new(2, 3),
            7,
            GameRules {
                player_base_attacks: vec![attack],
                player_weapon_slots: vec!["test:slot".parse().unwrap()],
                player_starting_weapons: vec![id.clone()],
                player_starting_equipment: vec![Some(id)],
                weapons,
                ..GameRules::default()
            },
        )
        .unwrap()
    }

    fn actor(game: &mut GameState, x: i32, y: i32, hp: u16) -> EntityId {
        game.spawn_actor(Actor::new(GridPos::new(x, y), hp).unwrap())
            .unwrap()
    }

    fn traces(game: &GameState) -> Vec<(GridPos, Vec<GridPos>)> {
        game.events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::PropagationResolved {
                    origin,
                    cells,
                    weapon_effect: Some(_),
                    ..
                } => Some((*origin, cells.iter().map(|cell| cell.position).collect())),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn piercing_extends_melee_and_ranged_hits_without_hitting_the_primary_target_again() {
        for delivery in [AttackDelivery::Melee, AttackDelivery::Ranged] {
            for lethal in [false, true] {
                let mut game = fixture(delivery, WeaponEffectTrigger::OnHit, false);
                let x = if delivery == AttackDelivery::Melee {
                    3
                } else {
                    7
                };
                let target = actor(&mut game, x, 3, if lethal { 5 } else { 30 });
                let first = actor(&mut game, x + 1, 3, 30);
                let last = actor(&mut game, x + 3, 3, 30);
                let beyond = actor(&mut game, x + 4, 3, 30);
                let side = actor(&mut game, x + 1, 4, 30);
                assert_eq!(
                    game.process_player_command(GameCommand::Attack { slot: 0, target }),
                    CommandOutcome::Applied
                );
                assert_eq!(
                    traces(&game),
                    vec![(
                        GridPos::new(x, 3),
                        (x + 1..=x + 3).map(|x| GridPos::new(x, 3)).collect()
                    )]
                );
                assert_eq!(
                    game.actors.get(target).map(Actor::integrity),
                    if lethal { None } else { Some(25) }
                );
                for id in [first, last] {
                    assert_eq!(game.actors.get(id).unwrap().integrity(), 27);
                }
                for id in [beyond, side] {
                    assert_eq!(game.actors.get(id).unwrap().integrity(), 30);
                }
            }
        }
    }

    #[test]
    fn piercing_rasterization_preserves_oblique_slopes_and_mirror_symmetry() {
        let map = Map::filled(30, 30, Terrain::Floor).unwrap();
        for (dx, dy) in [(1, 0), (0, 1), (1, 1), (4, 1), (1, 4)] {
            for sx in [-1, 1] {
                for sy in [-1, 1] {
                    let origin = GridPos::new(15 - dx * sx, 15 - dy * sy);
                    let impact = GridPos::new(15, 15);
                    let cells = piercing_cells(&map, origin, impact, 4);
                    assert_eq!(cells.len(), 4);
                    assert_eq!(
                        cells.last().unwrap().position,
                        GridPos::new(15 + dx * sx * 4 / dx.max(dy), 15 + dy * sy * 4 / dx.max(dy))
                    );
                    assert!(cells.iter().all(|cell| cell.position != impact));
                    assert!(
                        cells
                            .windows(2)
                            .all(|pair| pair[0].position != pair[1].position)
                    );
                }
            }
        }
        assert!(piercing_cells(&map, GridPos::new(15, 15), GridPos::new(15, 15), 4).is_empty());
        assert_eq!(
            piercing_cells(&map, GridPos::new(26, 15), GridPos::new(27, 15), u16::MAX).len(),
            2
        );
        assert!(
            piercing_cells(
                &map,
                GridPos::new(i32::MIN, 0),
                GridPos::new(i32::MAX, 0),
                u16::MAX
            )
            .is_empty()
        );
    }

    #[test]
    fn piercing_respects_closed_doors_and_passes_open_doors_or_water() {
        for terrain in [
            Terrain::Door(DoorState::Closed),
            Terrain::Door(DoorState::Locked),
            Terrain::Door(DoorState::Unpowered),
            Terrain::Door(DoorState::Open),
            Terrain::ShallowWater,
            Terrain::DeepWater,
        ] {
            let mut map = Map::filled(12, 6, Terrain::Floor).unwrap();
            map.set_terrain(GridPos::new(5, 3), terrain).unwrap();
            let cells = piercing_cells(&map, GridPos::new(2, 3), GridPos::new(3, 3), 3);
            assert_eq!(cells.len(), if terrain.blocks_vision() { 1 } else { 3 });
        }
    }

    #[test]
    fn piercing_keeps_the_old_impact_and_skips_a_primary_victim_knocked_into_the_line() {
        let mut game = fixture(AttackDelivery::Melee, WeaponEffectTrigger::OnHit, false);
        game.rules.physical_rules = Some(crate::stats::PhysicalRules::default());
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(3, 3), 100)
                    .unwrap()
                    .with_body_profile(
                        crate::stats::BodyProfile::new(100, 0)
                            .unwrap()
                            .with_displacement_profile(
                                crate::stats::DisplacementProfile::new(1_000, 0).unwrap(),
                            ),
                    ),
            )
            .unwrap();
        let behind = actor(&mut game, 6, 3, 30);
        let mut prepared = game
            .prepare_targeted_attack(game.player, 0, target)
            .unwrap();
        prepared.attack = prepared
            .attack
            .with_melee_impact(crate::combat::MeleeImpactProfile::new(20, 0))
            .unwrap();
        prepared.forced_movement = Some(ForcedMovement::new(2, 0));
        game.resolve_prepared_attack(prepared, ActionOrigin::Normal)
            .unwrap();
        assert_eq!(
            game.actors.get(target).unwrap().position(),
            GridPos::new(5, 3)
        );
        assert_eq!(traces(&game)[0].0, GridPos::new(3, 3));
        assert_eq!(game.actors.get(behind).unwrap().integrity(), 27);
        assert_eq!(game.events().iter().filter(|e| matches!(e, GameEvent::DamageApplied { target: id, .. } | GameEvent::DamageImpactApplied { target: id, .. } if *id == target)).count(), 1);
    }

    #[test]
    fn piercing_stops_at_walls_protected_ground_and_closed_corners() {
        for protected in [false, true] {
            let mut game = fixture(AttackDelivery::Ranged, WeaponEffectTrigger::OnHit, false);
            let target = actor(&mut game, 7, 3, 30);
            let before = actor(&mut game, 8, 3, 30);
            let behind = actor(&mut game, 10, 3, 30);
            if protected {
                game.map.set_protected(GridPos::new(9, 3), true).unwrap();
            } else {
                game.map
                    .set_terrain(GridPos::new(9, 3), Terrain::Wall)
                    .unwrap();
            }
            game.process_player_command(GameCommand::Attack { slot: 0, target });
            assert_eq!(traces(&game)[0].1, vec![GridPos::new(8, 3)]);
            assert_eq!(game.actors.get(before).unwrap().integrity(), 27);
            assert_eq!(game.actors.get(behind).unwrap().integrity(), 30);
        }
        let mut map = Map::filled(8, 8, Terrain::Floor).unwrap();
        for at in [GridPos::new(4, 3), GridPos::new(3, 4)] {
            map.set_terrain(at, Terrain::Wall).unwrap();
        }
        assert!(piercing_cells(&map, GridPos::new(2, 2), GridPos::new(3, 3), 3).is_empty());
    }

    #[test]
    fn piercing_requires_a_qualifying_hit_and_never_triggers_on_reaction() {
        for mode in ["miss", "armor-hit", "armor-damage", "reaction"] {
            let trigger = if mode == "armor-damage" {
                WeaponEffectTrigger::OnDamage
            } else {
                WeaponEffectTrigger::OnHit
            };
            let mut game = fixture(AttackDelivery::Ranged, trigger, false);
            let target = actor(&mut game, 7, 3, 30);
            let behind = actor(&mut game, 8, 3, 30);
            if mode == "miss" {
                game.rules.hit_rules = Some(HitRules {
                    minimum_hit_chance: 0,
                    maximum_hit_chance: 0,
                    ..HitRules::default()
                });
            }
            if mode.starts_with("armor") {
                game.rules.armor_rules = Some(ArmorRules::default());
                let state = game.actors.get(target).unwrap().clone().with_body_profile(
                    crate::stats::BodyProfile::new(30, 0)
                        .unwrap()
                        .with_base_armor(100),
                );
                *game.actors.get_mut(target).unwrap() = state;
            }
            if mode == "reaction" {
                let prepared = game
                    .prepare_targeted_attack(game.player, 0, target)
                    .unwrap();
                game.resolve_prepared_attack(prepared, ActionOrigin::Reaction)
                    .unwrap();
            } else {
                game.process_player_command(GameCommand::Attack { slot: 0, target });
            }
            assert_eq!(traces(&game).is_empty(), mode != "armor-hit", "{mode}");
            assert_eq!(
                game.actors.get(behind).unwrap().integrity(),
                if mode == "armor-hit" { 27 } else { 30 }
            );
        }
    }

    #[test]
    fn piercing_from_a_cone_resolves_once_and_never_retouches_primary_victims() {
        let mut game = fixture(AttackDelivery::Ranged, WeaponEffectTrigger::OnHit, true);
        let target = actor(&mut game, 6, 3, 30);
        let behind = actor(&mut game, 7, 3, 30);
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(traces(&game).len(), 1);
        assert_eq!(traces(&game)[0].0, GridPos::new(6, 3));
        assert_eq!(game.actors.get(behind).unwrap().integrity(), 25);
    }

    #[test]
    fn piercing_uses_a_real_impact_when_an_empty_aimed_cell_hits_an_actor() {
        for populated in [false, true] {
            let mut game = fixture(AttackDelivery::Ranged, WeaponEffectTrigger::OnHit, true);
            if populated {
                actor(&mut game, 6, 3, 30);
            }
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: GridPos::new(8, 3),
            });
            assert_eq!(traces(&game).len(), usize::from(populated));
            if populated {
                assert_eq!(traces(&game)[0].0, GridPos::new(6, 3));
            }
        }
    }

    #[test]
    fn piercing_simulation_does_not_use_player_visibility_or_trigger_life_steal() {
        let mut game = fixture(AttackDelivery::Ranged, WeaponEffectTrigger::OnHit, false);
        let target = actor(&mut game, 3, 3, 30);
        let behind = game
            .spawn_actor(
                Actor::new(GridPos::new(5, 3), 30)
                    .unwrap()
                    .with_tags(["test:healable".parse().unwrap()]),
            )
            .unwrap();
        game.player_visibility.recompute(
            &game.map,
            GridPos::new(2, 3),
            crate::world::FieldOfViewRules {
                radius: 2,
                ..Default::default()
            },
        );
        assert!(!game.player_visibility.is_visible(GridPos::new(5, 3)));
        game.actors.get_mut(game.player).unwrap().apply_damage(10);
        let mut prepared = game
            .prepare_targeted_attack(game.player, 0, target)
            .unwrap();
        prepared.weapon_effects.push(
            WeaponEffect::life_steal(
                100,
                3,
                "test:healable".parse().unwrap(),
                WeaponEffectTrigger::OnDamage,
            )
            .unwrap(),
        );
        game.resolve_prepared_attack(prepared, ActionOrigin::Normal)
            .unwrap();
        assert_eq!(game.actors.get(behind).unwrap().integrity(), 27);
        assert!(
            !game
                .events()
                .iter()
                .any(|e| matches!(e, GameEvent::IntegrityRestored { .. }))
        );
    }

    #[test]
    fn piercing_replays_identically_after_snapshot_round_trip() {
        let mut game = fixture(AttackDelivery::Ranged, WeaponEffectTrigger::OnHit, false);
        let target = actor(&mut game, 7, 3, 30);
        actor(&mut game, 9, 3, 30);
        game.drain_events();
        let bytes = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        let mut restored =
            GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), game.rules.clone());
        for state in [&mut game, &mut restored] {
            state.process_player_command(GameCommand::Attack { slot: 0, target });
        }
        assert_eq!(format!("{game:?}"), format!("{restored:?}"));
        assert_eq!(game.events(), restored.events());
    }
}
