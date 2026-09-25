//! Bounded, non-recursive weapon follow-ups. No client visibility filter.
use super::*;
use crate::combat::ConeAttack;
use crate::world::PropagationCell;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;
    use crate::effects::DamageFalloff;
    use crate::status::{StatusHook, StatusStacking};
    use crate::weapon::WeaponCatalog;
    use crate::world::{NeighborMode, TerrainPropagationPolicy};

    fn id(name: &str) -> StatusId {
        format!("test:{name}").parse().unwrap()
    }
    fn ricochet() -> WeaponEffect {
        WeaponEffect::ricochet(4, DamagePacket::new(3, DamageType::Piercing, 0)).unwrap()
    }
    fn cone() -> WeaponEffect {
        WeaponEffect::catalytic_cone(
            7,
            DamagePacket::new(2, DamageType::Thermal, 0),
            id("burning"),
            RadialDamageEffect {
                maximum_cost: 1,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: DamagePacket::new(6, DamageType::Thermal, 0),
                falloff: DamageFalloff::None,
            },
        )
        .unwrap()
    }
    fn fixture(delivery: AttackDelivery, effects: Vec<WeaponEffect>) -> GameState {
        let attack = AttackProfile::new(
            10,
            DistanceMetric::Chebyshev,
            true,
            DamageType::Kinetic,
            5,
            0,
        )
        .with_delivery(delivery);
        let mut weapons = WeaponCatalog::default();
        weapons
            .register(
                WeaponDefinition::new(id("weapon"), "name".into(), "description".into(), attack)
                    .unwrap()
                    .with_effects(effects),
            )
            .unwrap();
        let mut statuses = StatusCatalog::default();
        for name in ["burning", "corroded"] {
            statuses
                .register(
                    StatusDefinition::new(
                        id(name),
                        Some(4),
                        StatusStacking::RefreshDuration,
                        vec![StatusHook::new(
                            StatusTrigger::TurnEnd,
                            vec![StatusEffectPrimitive::DealDamage {
                                packet: DamagePacket::new(1, DamageType::Thermal, 0),
                                multiply_by_stacks: false,
                            }],
                        )],
                    )
                    .unwrap(),
                )
                .unwrap();
        }
        GameState::new_with_rules(
            Map::filled(18, 12, Terrain::Floor).unwrap(),
            GridPos::new(2, 3),
            7,
            GameRules {
                weapons,
                statuses,
                player_base_attacks: vec![attack],
                player_weapon_slots: vec![id("slot")],
                player_starting_weapons: vec![id("weapon")],
                player_starting_equipment: vec![Some(id("weapon"))],
                ..GameRules::default()
            },
        )
        .unwrap()
    }
    fn actor(game: &mut GameState, x: i32, y: i32, hp: u16) -> EntityId {
        game.spawn_actor(
            Actor::new(GridPos::new(x, y), hp)
                .unwrap()
                .with_evasion_disabled(),
        )
        .unwrap()
    }
    fn status(game: &mut GameState, target: EntityId, name: &str) {
        game.apply_status_to(
            Some(game.player),
            target,
            &ApplyStatusEffect::new(id(name), 1).unwrap(),
        )
        .unwrap();
    }
    fn attack(game: &mut GameState, target: EntityId) {
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
    }
    fn bursts(game: &GameState) -> Vec<GridPos> {
        game.events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::CatalyticExplosion { at, .. } => Some(*at),
                _ => None,
            })
            .collect()
    }
    fn damage(game: &GameState, victim: EntityId) -> u16 {
        game.events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::DamageApplied { target, amount, .. }
                | GameEvent::DamageImpactApplied { target, amount, .. }
                    if *target == victim =>
                {
                    Some(*amount)
                }
                _ => None,
            })
            .sum()
    }

    #[test]
    fn ricochet_is_a_single_nearest_rebound_even_with_duplicate_properties_and_lethal_primary() {
        let mut game = fixture(AttackDelivery::Ranged, vec![ricochet(), ricochet()]);
        let target = actor(&mut game, 7, 3, 5);
        let near = actor(&mut game, 8, 3, 50);
        let far = actor(&mut game, 9, 3, 50);
        attack(&mut game, target);
        assert!(game.actors.get(target).is_none());
        assert_eq!(damage(&game, near), 3);
        assert_eq!(damage(&game, far), 0);
        assert_eq!(damage(&game, game.player), 0);
        let paths: Vec<_> = game
            .events()
            .iter()
            .filter_map(|e| match e {
                GameEvent::PropagationResolved { origin, cells, .. } => Some((*origin, cells)),
                _ => None,
            })
            .collect();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].0, GridPos::new(7, 3));
        assert_eq!(paths[0].1.last().unwrap().position, GridPos::new(8, 3));
    }

    #[test]
    fn ricochet_does_not_trigger_from_melee_reactions_or_without_an_admissible_neighbor() {
        for case in ["melee", "reaction", "far", "wall", "protected"] {
            let mut game = fixture(
                if case == "melee" {
                    AttackDelivery::Melee
                } else {
                    AttackDelivery::Ranged
                },
                vec![ricochet()],
            );
            let target = actor(&mut game, 3, 3, 50);
            let other = actor(&mut game, if case == "far" { 9 } else { 5 }, 3, 50);
            if case == "wall" {
                game.map
                    .set_terrain(GridPos::new(4, 3), Terrain::Wall)
                    .unwrap();
            }
            if case == "protected" {
                game.map.set_protected(GridPos::new(4, 3), true).unwrap();
            }
            game.drain_events();
            if case == "reaction" {
                let prepared = game
                    .prepare_targeted_attack(game.player, 0, target)
                    .unwrap();
                game.resolve_prepared_attack(prepared, ActionOrigin::Reaction)
                    .unwrap();
            } else {
                attack(&mut game, target);
            }
            assert_eq!(damage(&game, other), 0, "{case}");
            assert!(
                !game
                    .events()
                    .iter()
                    .any(|e| matches!(e, GameEvent::PropagationResolved { .. })),
                "{case}"
            );
        }
    }

    #[test]
    fn rebound_paths_stop_at_closed_corners_and_break_ties_deterministically() {
        let mut map = Map::filled(8, 8, Terrain::Floor).unwrap();
        map.set_terrain(GridPos::new(4, 3), Terrain::Wall).unwrap();
        map.set_terrain(GridPos::new(3, 4), Terrain::Wall).unwrap();
        assert!(ricochet_path(&map, GridPos::new(3, 3), GridPos::new(4, 4), 4).is_none());
        let mut game = fixture(AttackDelivery::Ranged, vec![ricochet()]);
        let target = actor(&mut game, 7, 3, 50);
        let b = actor(&mut game, 8, 3, 50);
        let a = actor(&mut game, 7, 4, 50);
        attack(&mut game, target);
        let expected = [(GridPos::new(8, 3), b), (GridPos::new(7, 4), a)]
            .into_iter()
            .min()
            .unwrap()
            .1;
        assert_eq!(damage(&game, expected), 3);
        assert_eq!(damage(&game, a) + damage(&game, b), 3);
    }

    #[test]
    fn cone_starts_at_bearer_burns_its_footprint_and_only_existing_burns_explode() {
        for delivery in [AttackDelivery::Melee, AttackDelivery::Ranged] {
            let mut game = fixture(delivery, vec![cone(), cone()]);
            let target = actor(&mut game, 3, 3, 100);
            let mid = actor(&mut game, 6, 3, 100);
            let behind = actor(&mut game, 1, 3, 100);
            attack(&mut game, target);
            assert!(bursts(&game).is_empty());
            assert_eq!(damage(&game, mid), 3); // Flame hit + first burn tick.
            assert_eq!(damage(&game, behind), 0);
            assert_eq!(damage(&game, game.player), 0);
            assert_eq!(game.events().iter().filter(|e| matches!(e, GameEvent::WeaponFlameConeResolved { origin, .. } if *origin == GridPos::new(2, 3))).count(), 1);
            status(&mut game, mid, "corroded");
            attack(&mut game, target);
            assert_eq!(bursts(&game), [GridPos::new(3, 3), GridPos::new(6, 3)]);
            let survivor = game.actors.get(mid).unwrap();
            assert_eq!(survivor.status(&id("burning")).unwrap().stacks, 1);
            assert!(survivor.status(&id("corroded")).is_some());
        }
    }

    #[test]
    fn fresh_burns_from_the_same_primary_attack_do_not_fuel_the_cone() {
        let mut game = fixture(
            AttackDelivery::Ranged,
            vec![
                WeaponEffect::apply_status(
                    ApplyStatusEffect::new(id("burning"), 1).unwrap(),
                    WeaponEffectTrigger::OnHit,
                )
                .unwrap(),
                cone(),
            ],
        );
        let target = actor(&mut game, 6, 3, 100);
        let already_lit = actor(&mut game, 7, 3, 100);
        let fresh = actor(&mut game, 8, 3, 100);
        status(&mut game, already_lit, "burning");
        attack(&mut game, target);
        assert_eq!(bursts(&game), [GridPos::new(7, 3)]);
        for entity in [target, fresh] {
            assert!(
                game.actors
                    .get(entity)
                    .unwrap()
                    .status(&id("burning"))
                    .is_some()
            );
        }
    }

    #[test]
    fn all_primed_bursts_survive_lethal_primary_cone_and_neighbor_explosions() {
        for hp in [2, 6, 10] {
            let mut game = fixture(AttackDelivery::Ranged, vec![cone()]);
            let target = actor(&mut game, 6, 3, hp);
            let neighbor = actor(&mut game, 7, 3, hp);
            status(&mut game, target, "burning");
            status(&mut game, neighbor, "burning");
            attack(&mut game, target);
            assert_eq!(
                bursts(&game),
                [GridPos::new(6, 3), GridPos::new(7, 3)],
                "hp={hp}"
            );
            assert!(game.actors.get(target).is_none());
        }
    }

    #[test]
    fn cone_preview_is_non_mutating_and_matches_the_executed_secondary_cells() {
        let mut game = fixture(AttackDelivery::Ranged, vec![cone()]);
        let target = actor(&mut game, 6, 3, 100);
        game.map
            .set_terrain(GridPos::new(8, 3), Terrain::Wall)
            .unwrap();
        game.map.set_protected(GridPos::new(6, 4), true).unwrap();
        let before = format!("{game:?}");
        let preview = game.player_attack_preview(0, GridPos::new(6, 3)).unwrap();
        assert!(preview.cells().len() > 1);
        assert_eq!(format!("{game:?}"), before);
        assert_eq!(
            game.equipped_player_weapon(0).unwrap().attack().area(),
            AttackArea::Single
        );
        attack(&mut game, target);
        let executed: BTreeSet<_> = game
            .events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::WeaponFlameConeResolved { cells, .. } => {
                    Some(cells.iter().map(|c| c.position))
                }
                _ => None,
            })
            .flatten()
            .collect();
        assert_eq!(
            preview
                .cells()
                .iter()
                .map(|c| c.position)
                .collect::<BTreeSet<_>>(),
            executed
        );
    }

    #[test]
    fn cone_preview_includes_each_known_primed_explosion_without_burning_new_targets() {
        let mut game = fixture(AttackDelivery::Ranged, vec![cone()]);
        let target = actor(&mut game, 3, 3, 100);
        let side = actor(&mut game, 5, 4, 100);
        status(&mut game, target, "burning");
        status(&mut game, side, "burning");
        let preview = game.player_attack_preview(0, GridPos::new(3, 3)).unwrap();
        assert!(
            preview
                .cells()
                .iter()
                .any(|c| c.position == GridPos::new(4, 5))
        );
        let before = format!("{game:?}");
        assert_eq!(
            game.player_attack_footprint(0, GridPos::new(3, 3)).unwrap(),
            preview
        );
        assert_eq!(format!("{game:?}"), before);
        attack(&mut game, target);
        let mut executed: BTreeSet<_> = game
            .events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::WeaponFlameConeResolved { cells, .. }
                | GameEvent::PropagationResolved { cells, .. } => {
                    Some(cells.iter().map(|c| c.position))
                }
                _ => None,
            })
            .flatten()
            .collect();
        executed.remove(&GridPos::new(2, 3)); // The bearer is immune.
        assert_eq!(
            preview
                .cells()
                .iter()
                .map(|c| c.position)
                .collect::<BTreeSet<_>>(),
            executed
        );
    }

    #[test]
    fn single_target_cone_aim_keeps_primary_range_and_empty_confirmation_costs_nothing() {
        let mut game = fixture(AttackDelivery::Melee, vec![cone()]);
        let at = GridPos::new(3, 3);
        assert!(game.player_attack_footprint(0, at).unwrap().cells().len() > 1);
        assert_eq!(
            game.player_attack_preview(0, at),
            Err(CommandRejection::AttackTargetHasNoActor(at))
        );
        let before = game.turn();
        assert_eq!(
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: at
            }),
            CommandOutcome::Rejected(CommandRejection::AttackTargetHasNoActor(at))
        );
        assert_eq!(game.turn(), before);
        actor(&mut game, 3, 3, 100);
        assert!(game.player_attack_preview(0, at).is_ok());
        assert_eq!(
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: at
            }),
            CommandOutcome::Applied
        );
        assert!(
            game.events()
                .iter()
                .any(|e| matches!(e, GameEvent::WeaponFlameConeResolved { .. }))
        );
    }

    #[test]
    fn cone_does_not_pass_through_walls_or_protected_paths_and_ignites_no_ground() {
        for protected in [false, true] {
            let mut game = fixture(AttackDelivery::Ranged, vec![cone()]);
            let target = actor(&mut game, 3, 3, 100);
            let hidden = actor(&mut game, 7, 3, 100);
            for y in 0..12 {
                if protected {
                    game.map.set_protected(GridPos::new(5, y), true).unwrap();
                } else {
                    game.map
                        .set_terrain(GridPos::new(5, y), Terrain::Wall)
                        .unwrap();
                }
            }
            status(&mut game, hidden, "burning");
            attack(&mut game, target);
            assert!(bursts(&game).is_empty());
            assert_eq!(damage(&game, hidden), 1); // Only its existing burn.
            assert!(
                !game
                    .events()
                    .iter()
                    .any(|e| matches!(e, GameEvent::GroundEffectCreated { .. }))
            );
            for e in game.events() {
                if let GameEvent::WeaponFlameConeResolved { cells, .. } = e {
                    assert!(cells.iter().all(|cell| cell.position.x < 5));
                }
            }
        }
    }
}

fn ricochet_path(
    map: &Map,
    from: GridPos,
    to: GridPos,
    range: u16,
) -> Option<Vec<PropagationCell>> {
    let dx = i64::from(to.x) - i64::from(from.x);
    let dy = i64::from(to.y) - i64::from(from.y);
    let steps = dx.abs().max(dy.abs());
    if steps == 0 || steps > i64::from(range) || !has_line_of_sight(map, from, to, true) {
        return None;
    }
    let mut cells = Vec::new();
    let mut previous = from;
    for step in 0..=steps {
        let offset = |delta: i64| ((delta.abs() * step + steps / 2) / steps) * delta.signum();
        let at = GridPos::new(
            (i64::from(from.x) + offset(dx)) as i32,
            (i64::from(from.y) + offset(dy)) as i32,
        );
        if !map.contains(at)
            || map.blocks_vision(at)
            || map.is_protected(at)
            || !has_line_of_sight(map, previous, at, true)
        {
            return None;
        }
        cells.push(PropagationCell {
            position: at,
            cost: step as u16,
            step: step as u16,
        });
        previous = at;
    }
    Some(cells)
}

impl GameState {
    /// Shared geometry: presentation never reproduces the cone or wall rules.
    fn catalytic_cone_cells(
        &self,
        origin: GridPos,
        impact: GridPos,
        range: u16,
    ) -> Vec<PropagationCell> {
        AttackProfile::new(
            range,
            DistanceMetric::Chebyshev,
            true,
            crate::combat::DamageType::Thermal,
            1,
            0,
        )
        .with_area(AttackArea::Cone(
            ConeAttack::new(1, 2, 2).expect("authored cone"),
        ))
        .affected_cells(&self.map, origin, impact)
        .into_iter()
        .filter(|cell| ricochet_path(&self.map, origin, cell.position, range).is_some())
        .map(|cell| PropagationCell {
            position: cell.position,
            cost: cell.step,
            step: cell.step,
        })
        .collect()
    }

    pub(super) fn preview_with_weapon_followups(&self, prepared: &PreparedAttack) -> AttackPreview {
        let Some(WeaponEffectKind::CatalyticCone {
            range,
            burning_status,
            explosion,
            ..
        }) = prepared
            .weapon_effects
            .iter()
            .map(WeaponEffect::kind)
            .find(|kind| matches!(kind, WeaponEffectKind::CatalyticCone { .. }))
        else {
            return AttackPreview::new(
                prepared.origin,
                prepared.target_at,
                prepared.affected_cells.clone(),
            );
        };
        let cone = self.catalytic_cone_cells(prepared.origin, prepared.target_at, *range);
        let cone_positions: BTreeSet<_> = cone.iter().map(|cell| cell.position).collect();
        let mut cells: BTreeMap<_, _> = prepared
            .affected_cells
            .iter()
            .copied()
            .filter(|cell| {
                cell.position != prepared.origin && !self.map.is_protected(cell.position)
            })
            .map(|cell| (cell.position, cell.step))
            .collect();
        for cell in cone {
            cells.entry(cell.position).or_insert(cell.step);
        }
        // Preview only known preparations. A hidden burning actor must never
        // reveal its existence by projecting an extra visible blast outline.
        for (id, actor) in self.actors.iter() {
            if id == self.player
                || !actor.is_alive()
                || !self.player_visibility.is_visible(actor.position())
                || !cone_positions.contains(&actor.position())
                || actor.status(burning_status).is_none()
            {
                continue;
            }
            for cell in explosion.affected_cells(&self.map, actor.position()) {
                if cell.position != prepared.origin && !self.map.is_protected(cell.position) {
                    cells.entry(cell.position).or_insert(cell.step);
                }
            }
        }
        AttackPreview::new(
            prepared.origin,
            prepared.target_at,
            cells
                .into_iter()
                .map(|(position, step)| AttackAreaCell { position, step })
                .collect(),
        )
    }

    pub(super) fn apply_weapon_ricochet(
        &mut self,
        source: EntityId,
        impact: GridPos,
        range: u16,
        damage: DamagePacket,
        primary_targets: &BTreeSet<EntityId>,
        weapon_effect: Option<(WeaponId, usize)>,
    ) {
        if self.map.is_protected(impact)
            || self
                .actors
                .get(source)
                .is_none_or(|actor| !actor.is_alive() || self.map.is_protected(actor.position()))
        {
            return;
        }
        // Stable nearest admissible recipient. No return to a primary victim,
        // bearer, inaccessible actor, or actor already killed by this attack.
        let candidate = self
            .actors
            .iter()
            .filter(|(id, actor)| {
                *id != source && !primary_targets.contains(id) && actor.is_alive()
            })
            .filter_map(|(id, actor)| {
                ricochet_path(&self.map, impact, actor.position(), range)
                    .map(|cells| (cells.len(), actor.position(), id, cells))
            })
            .min_by_key(|(length, position, id, _)| (*length, *position, *id));
        if let Some((_, _, target, cells)) = candidate {
            self.events.push(GameEvent::PropagationResolved {
                source: Some(source),
                origin: impact,
                cells,
                weapon_effect,
            });
            let _ = self.apply_damage_to(Some(source), target, damage);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn apply_catalytic_cone(
        &mut self,
        source: EntityId,
        origin: GridPos,
        impact: GridPos,
        kind: &WeaponEffectKind,
        fuel: &BTreeSet<EntityId>,
        hit_impacts: &BTreeMap<GridPos, EntityId>,
        weapon_effect: Option<(WeaponId, usize)>,
    ) {
        let WeaponEffectKind::CatalyticCone {
            range,
            damage,
            burning_status,
            explosion,
        } = kind
        else {
            return;
        };
        if self.map.is_protected(origin)
            || self
                .actors
                .get(source)
                .is_none_or(|actor| !actor.is_alive() || self.map.is_protected(actor.position()))
        {
            return;
        }
        let cells = self.catalytic_cone_cells(origin, impact, *range);
        let positions: BTreeSet<GridPos> = cells.iter().map(|cell| cell.position).collect();
        // Capture recipients AND every explosion origin together. A burst can
        // kill another primed target without deleting its already committed burst.
        let targets: Vec<(EntityId, GridPos)> = self
            .actors
            .iter()
            .filter_map(|(id, actor)| {
                (id != source && actor.is_alive() && positions.contains(&actor.position()))
                    .then_some((id, actor.position()))
            })
            .collect();
        let mut bursts: BTreeMap<EntityId, GridPos> = targets
            .iter()
            .filter(|(id, _)| fuel.contains(id))
            .copied()
            .collect();
        for (&at, &id) in hit_impacts {
            if self.actors.get(id).is_none() && fuel.contains(&id) && positions.contains(&at) {
                bursts.insert(id, at);
            }
        }
        self.events.push(GameEvent::WeaponFlameConeResolved {
            source,
            origin,
            cells,
        });
        let burn = ApplyStatusEffect::new(burning_status.clone(), 1).expect("one burn");
        for (target, _) in targets {
            let _ = self.apply_damage_to(Some(source), target, *damage);
            if self.actors.get(target).is_some_and(Actor::is_alive) {
                let _ = self.apply_status_to(Some(source), target, &burn);
            }
        }
        for (_, at) in bursts {
            self.events
                .push(GameEvent::CatalyticExplosion { source, at });
            self.apply_radial_damage_excluding(
                Some(source),
                at,
                explosion,
                Some(source),
                weapon_effect.clone(),
            );
        }
    }
}
