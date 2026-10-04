//! Simulation-owned memories and deadlines. Never driven by animation time.
use super::*;
use crate::world::PropagationCell;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(super) struct WeaponEchoState {
    previous: BTreeMap<EntityId, PreviousImpact>,
    pending: Vec<PendingEcho>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PreviousImpact {
    target: EntityId,
    expires_on_turn: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PendingEcho {
    source: EntityId,
    at: GridPos,
    resolve_on_turn: u64,
    damage: DamagePacket,
}
impl WeaponEchoState {
    pub(super) fn is_empty(&self) -> bool {
        self.previous.is_empty() && self.pending.is_empty()
    }
}

// Stable straight link. LOS checks every step too, so neither corners nor
// protected tiles can become an invisible conduit. Only the endpoint is hurt.
fn echo_path(map: &Map, from: GridPos, to: GridPos, range: u16) -> Option<Vec<PropagationCell>> {
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
    /// Read-only marker, filtered by the client for current perception.
    pub fn alternation_previous(&self, source: EntityId) -> Option<(EntityId, u16)> {
        let memory = self.weapon_echoes.previous.get(&source)?;
        (memory.expires_on_turn > self.turn
            && self.actors.get(memory.target).is_some_and(Actor::is_alive))
        .then_some((
            memory.target,
            memory
                .expires_on_turn
                .saturating_sub(self.turn)
                .min(u64::from(u16::MAX)) as u16,
        ))
    }

    pub fn pending_weapon_echoes(&self) -> impl Iterator<Item = (GridPos, u16)> + '_ {
        self.weapon_echoes.pending.iter().map(|echo| {
            (
                echo.at,
                echo.resolve_on_turn
                    .saturating_add(1)
                    .saturating_sub(self.turn)
                    .min(u64::from(u16::MAX)) as u16,
            )
        })
    }

    /// Same conditions as resolution; no RNG, mutation or visibility oracle.
    pub fn alternation_can_link(&self, source: EntityId, target: EntityId, range: u16) -> bool {
        if self
            .actors
            .get(source)
            .is_none_or(|actor| !actor.is_alive() || self.map.is_protected(actor.position()))
        {
            return false;
        }
        let Some((previous, _)) = self.alternation_previous(source) else {
            return false;
        };
        if previous == target {
            return false;
        }
        let Some(from) = self.actors.get(target).map(Actor::position) else {
            return false;
        };
        let Some(to) = self.actors.get(previous).map(Actor::position) else {
            return false;
        };
        echo_path(&self.map, from, to, range).is_some()
    }

    pub(super) fn apply_weapon_echo(
        &mut self,
        source: EntityId,
        target: EntityId,
        at: GridPos,
        kind: &WeaponEffectKind,
        weapon_effect: Option<(WeaponId, usize)>,
    ) {
        if self.map.is_protected(at)
            || self
                .actors
                .get(source)
                .is_none_or(|actor| !actor.is_alive() || self.map.is_protected(actor.position()))
        {
            return;
        }
        match kind {
            WeaponEffectKind::Alternation {
                range,
                memory_turns,
                damage,
            } => {
                let old = self.alternation_previous(source).map(|(target, _)| target);
                // Capture new primary target, never the secondary recipient. A
                // lethal hit may send a return, but leaves no dead-target memory.
                self.weapon_echoes.previous.remove(&source);
                if self.actors.get(target).is_some_and(Actor::is_alive) {
                    self.weapon_echoes.previous.insert(
                        source,
                        PreviousImpact {
                            target,
                            expires_on_turn: self.turn.saturating_add(u64::from(*memory_turns)),
                        },
                    );
                }
                if let Some(previous) = old.filter(|old| *old != target && *old != source)
                    && let Some(to) = self.actors.get(previous).map(Actor::position)
                    && let Some(cells) = echo_path(&self.map, at, to, *range)
                {
                    self.events.push(GameEvent::PropagationResolved {
                        source: Some(source),
                        origin: at,
                        cells,
                        weapon_effect,
                    });
                    let _ = self.apply_damage_to(Some(source), previous, *damage);
                }
            }
            WeaponEffectKind::DelayedEcho {
                delay_turns,
                damage,
            } => {
                // No stacking, refreshing or postponing an already committed
                // echo on this cell, even if a different bearer strikes it.
                if self.weapon_echoes.pending.iter().any(|echo| echo.at == at) {
                    return;
                }
                self.weapon_echoes.pending.push(PendingEcho {
                    source,
                    at,
                    resolve_on_turn: self.turn.saturating_add(u64::from(*delay_turns)),
                    damage: *damage,
                });
                self.events.push(GameEvent::WeaponEchoScheduled {
                    source,
                    at,
                    delay_turns: *delay_turns,
                });
            }
            _ => {}
        }
    }

    pub(super) fn resolve_weapon_echoes(&mut self) {
        self.weapon_echoes.previous.retain(|source, previous| {
            previous.expires_on_turn > self.turn.saturating_add(1)
                && self.actors.get(*source).is_some_and(Actor::is_alive)
                && self
                    .actors
                    .get(previous.target)
                    .is_some_and(Actor::is_alive)
        });
        let pending = std::mem::take(&mut self.weapon_echoes.pending);
        for echo in pending {
            if echo.resolve_on_turn > self.turn {
                self.weapon_echoes.pending.push(echo);
                continue;
            }
            // Committed echoes survive weapon swaps and the source's death.
            // A new wall or protected area, however, cannot receive the strike.
            if self.map.is_protected(echo.at) || self.map.blocks_vision(echo.at) {
                continue;
            }
            self.events.push(GameEvent::WeaponEchoResolved {
                source: echo.source,
                at: echo.at,
            });
            if let Some(target) = self
                .actors
                .entity_at(echo.at)
                .filter(|target| *target != echo.source)
            {
                let _ = self.apply_damage_to(Some(echo.source), target, echo.damage);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{ConeAttack, DamageType, HitRules};
    use crate::weapon::WeaponCatalog;

    fn damage() -> DamagePacket {
        DamagePacket::new(3, DamageType::Kinetic, 0)
    }
    fn alternation() -> WeaponEffect {
        WeaponEffect::alternation(4, 5, damage()).unwrap()
    }
    fn echo() -> WeaponEffect {
        WeaponEffect::delayed_echo(2, damage()).unwrap()
    }
    fn fixture(delivery: AttackDelivery, cone: bool, effects: Vec<WeaponEffect>) -> GameState {
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
        let id: WeaponId = "test:echo".parse().unwrap();
        let mut weapons = WeaponCatalog::default();
        weapons
            .register(
                WeaponDefinition::new(id.clone(), "name".into(), "description".into(), attack)
                    .unwrap()
                    .with_effects(effects),
            )
            .unwrap();
        GameState::new_with_rules(
            Map::filled(18, 12, Terrain::Floor).unwrap(),
            GridPos::new(2, 3),
            7,
            GameRules {
                weapons,
                player_base_attacks: vec![attack],
                player_weapon_slots: vec!["test:slot".parse().unwrap()],
                player_starting_weapons: vec![id.clone()],
                player_starting_equipment: vec![Some(id)],
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
    fn hit(game: &mut GameState, target: EntityId) {
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
    }
    fn wait(game: &mut GameState) {
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
    }
    fn links(game: &GameState) -> usize {
        game.events()
            .iter()
            .filter(|e| matches!(e, GameEvent::PropagationResolved { .. }))
            .count()
    }
    fn resolved(game: &GameState) -> usize {
        game.events()
            .iter()
            .filter(|e| matches!(e, GameEvent::WeaponEchoResolved { .. }))
            .count()
    }

    #[test]
    fn alternation_melee_and_ranged_reward_switching_not_repeated_hits() {
        for delivery in [AttackDelivery::Melee, AttackDelivery::Ranged] {
            let mut game = fixture(delivery, false, vec![alternation(), alternation()]);
            let x = if delivery == AttackDelivery::Melee {
                3
            } else {
                7
            };
            let a = actor(&mut game, x, 3, 100);
            let b = actor(&mut game, x, 4, 100);
            hit(&mut game, a);
            hit(&mut game, a);
            assert_eq!(links(&game), 0);
            hit(&mut game, b);
            assert_eq!(links(&game), 1);
            assert_eq!(game.actors.get(a).unwrap().integrity(), 87);
            assert_eq!(game.actors.get(b).unwrap().integrity(), 95);
            assert_eq!(game.alternation_previous(game.player), Some((b, 4)));
            hit(&mut game, a);
            assert_eq!(game.actors.get(b).unwrap().integrity(), 92);
        }
    }

    #[test]
    fn alternation_uses_live_previous_position_but_never_hurts_link_bystanders() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![alternation()]);
        let a = actor(&mut game, 7, 3, 100);
        let b = actor(&mut game, 7, 6, 100);
        let bystander = actor(&mut game, 7, 5, 100);
        hit(&mut game, a);
        game.actors.move_to(a, GridPos::new(7, 4)).unwrap();
        assert!(game.alternation_can_link(game.player, b, 4));
        hit(&mut game, b);
        assert_eq!(game.actors.get(a).unwrap().integrity(), 92);
        assert_eq!(game.actors.get(bystander).unwrap().integrity(), 100);
        assert!(game.events().iter().any(|event| matches!(event, GameEvent::PropagationResolved { origin, cells, .. } if *origin == GridPos::new(7, 6) && cells.last().unwrap().position == GridPos::new(7, 4))));
    }

    #[test]
    fn alternation_respects_range_walls_closed_corners_protection_and_memory_expiry() {
        for reason in ["range", "wall", "protected", "expired", "dead"] {
            let mut game = fixture(AttackDelivery::Ranged, false, vec![alternation()]);
            let a = actor(&mut game, 7, 3, 100);
            let b = actor(&mut game, 7, 6, 100);
            hit(&mut game, a);
            match reason {
                "range" => {
                    game.actors.move_to(a, GridPos::new(7, 11)).unwrap();
                }
                "wall" => {
                    game.map
                        .set_terrain(GridPos::new(7, 4), Terrain::Wall)
                        .unwrap();
                }
                "protected" => {
                    game.map.set_protected(GridPos::new(7, 4), true).unwrap();
                }
                "expired" => {
                    for _ in 0..4 {
                        wait(&mut game);
                    }
                }
                "dead" => {
                    game.apply_damage_to(None, a, DamagePacket::new(1000, DamageType::Kinetic, 0))
                        .unwrap();
                }
                _ => unreachable!(),
            }
            assert!(!game.alternation_can_link(game.player, b, 4), "{reason}");
            hit(&mut game, b);
            assert_eq!(links(&game), 0, "{reason}");
            if let Some(actor) = game.actors.get(a) {
                assert_eq!(actor.integrity(), 95, "{reason}");
            }
        }
        let mut map = Map::filled(5, 5, Terrain::Floor).unwrap();
        map.set_terrain(GridPos::new(2, 1), Terrain::Wall).unwrap();
        map.set_terrain(GridPos::new(1, 2), Terrain::Wall).unwrap();
        assert!(echo_path(&map, GridPos::new(1, 1), GridPos::new(2, 2), 4).is_none());
    }

    #[test]
    fn alternation_lethal_new_target_returns_once_but_does_not_remember_a_corpse() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![alternation()]);
        let a = actor(&mut game, 7, 3, 100);
        let b = actor(&mut game, 7, 5, 5);
        hit(&mut game, a);
        hit(&mut game, b);
        assert_eq!(links(&game), 1);
        assert_eq!(game.alternation_previous(game.player), None);
        assert_eq!(game.actors.get(a).unwrap().integrity(), 92);
    }

    #[test]
    fn echo_melee_and_ranged_wait_two_future_turns_and_never_repeat() {
        for delivery in [AttackDelivery::Melee, AttackDelivery::Ranged] {
            let mut game = fixture(delivery, false, vec![echo(), echo()]);
            let x = if delivery == AttackDelivery::Melee {
                3
            } else {
                7
            };
            let a = actor(&mut game, x, 3, 100);
            hit(&mut game, a);
            assert_eq!(
                game.pending_weapon_echoes().collect::<Vec<_>>(),
                vec![(GridPos::new(x, 3), 2)]
            );
            assert_eq!(game.actors.get(a).unwrap().integrity(), 95);
            wait(&mut game);
            assert_eq!(resolved(&game), 0);
            assert_eq!(game.pending_weapon_echoes().next().unwrap().1, 1);
            wait(&mut game);
            assert_eq!(resolved(&game), 1);
            assert_eq!(game.actors.get(a).unwrap().integrity(), 92);
            assert!(game.pending_weapon_echoes().next().is_none());
            wait(&mut game);
            assert_eq!(resolved(&game), 0);
            assert_eq!(game.actors.get(a).unwrap().integrity(), 92);
        }
    }

    #[test]
    fn echo_stays_on_old_cell_after_movement_or_lethal_hit_and_hits_new_occupant() {
        for lethal in [false, true] {
            let mut game = fixture(AttackDelivery::Ranged, false, vec![echo()]);
            let a = actor(&mut game, 7, 3, if lethal { 5 } else { 100 });
            hit(&mut game, a);
            if !lethal {
                game.actors.move_to(a, GridPos::new(8, 3)).unwrap();
            }
            let b = actor(&mut game, 7, 3, 100);
            wait(&mut game);
            wait(&mut game);
            assert_eq!(game.actors.get(b).unwrap().integrity(), 97);
            if !lethal {
                assert_eq!(game.actors.get(a).unwrap().integrity(), 95);
            }
            assert!(game.ground_effects.is_empty());
        }
    }

    #[test]
    fn echo_does_not_stack_refresh_or_multiply_on_duplicate_properties_and_repeated_hits() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![echo(), echo()]);
        let a = actor(&mut game, 7, 3, 100);
        hit(&mut game, a);
        hit(&mut game, a);
        assert_eq!(
            game.pending_weapon_echoes().collect::<Vec<_>>(),
            vec![(GridPos::new(7, 3), 1)]
        );
        assert!(
            !game
                .events()
                .iter()
                .any(|e| matches!(e, GameEvent::WeaponEchoScheduled { .. }))
        );
        hit(&mut game, a);
        assert_eq!(resolved(&game), 1);
        assert_eq!(game.actors.get(a).unwrap().integrity(), 82);
        assert_eq!(game.pending_weapon_echoes().count(), 0);
    }

    #[test]
    fn echo_empty_cell_protected_cell_new_wall_and_bearer_cannot_take_echo_damage() {
        for condition in ["empty", "protected", "wall", "bearer"] {
            let mut game = fixture(AttackDelivery::Ranged, false, vec![echo()]);
            let a = actor(&mut game, 7, 3, 100);
            hit(&mut game, a);
            let player_hp = game.actors.get(game.player).unwrap().integrity();
            match condition {
                "empty" | "bearer" => {
                    game.actors.move_to(a, GridPos::new(8, 3)).unwrap();
                }
                "protected" => {
                    game.map.set_protected(GridPos::new(7, 3), true).unwrap();
                }
                "wall" => {
                    game.map
                        .set_terrain(GridPos::new(7, 3), Terrain::Wall)
                        .unwrap();
                }
                _ => unreachable!(),
            }
            if condition == "bearer" {
                game.actors
                    .move_to(game.player, GridPos::new(7, 3))
                    .unwrap();
            }
            wait(&mut game);
            wait(&mut game);
            assert_eq!(game.actors.get(a).unwrap().integrity(), 95);
            assert_eq!(game.actors.get(game.player).unwrap().integrity(), player_hp);
            assert_eq!(game.pending_weapon_echoes().count(), 0);
        }
    }

    #[test]
    fn echoes_require_normal_actual_hits_not_misses_reactions_or_empty_ground() {
        for condition in [
            "miss",
            "reaction",
            "empty",
            "protected_source",
            "protected_target",
        ] {
            let mut game = fixture(AttackDelivery::Ranged, false, vec![alternation(), echo()]);
            let a = game
                .spawn_actor(Actor::new(GridPos::new(7, 3), 100).unwrap())
                .unwrap();
            if condition == "miss" {
                game.rules.hit_rules = Some(HitRules {
                    minimum_hit_chance: 0,
                    maximum_hit_chance: 0,
                    ..HitRules::default()
                });
            }
            if condition == "protected_source" {
                game.map.set_protected(GridPos::new(2, 3), true).unwrap();
            }
            if condition == "protected_target" {
                game.map.set_protected(GridPos::new(7, 3), true).unwrap();
            }
            if condition == "reaction" {
                let prepared = game.prepare_targeted_attack(game.player, 0, a).unwrap();
                game.resolve_prepared_attack(prepared, ActionOrigin::Reaction)
                    .unwrap();
            } else if condition == "empty" {
                game.process_player_command(GameCommand::AttackAt {
                    slot: 0,
                    target: GridPos::new(2, 8),
                });
            } else {
                game.process_player_command(GameCommand::Attack { slot: 0, target: a });
            }
            assert!(game.weapon_echoes.is_empty(), "{condition}");
        }
    }

    #[test]
    fn echo_multitarget_uses_aimed_hit_then_stable_fallback_only_once() {
        for aim in [7, 9] {
            let mut game = fixture(
                AttackDelivery::Ranged,
                true,
                vec![alternation(), echo(), echo()],
            );
            let a = actor(&mut game, 5, 3, 100);
            let b = actor(&mut game, 7, 3, 100);
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: GridPos::new(aim, 3),
            });
            let chosen = if aim == 7 { b } else { a };
            assert_eq!(game.alternation_previous(game.player), Some((chosen, 4)));
            assert_eq!(
                game.pending_weapon_echoes().collect::<Vec<_>>(),
                vec![(GridPos::new(if aim == 7 { 7 } else { 5 }, 3), 2)]
            );
        }
    }

    #[test]
    fn echo_state_snapshot_replays_and_secondary_strikes_do_not_schedule_or_change_memory() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![alternation(), echo()]);
        let a = actor(&mut game, 7, 3, 100);
        let b = actor(&mut game, 7, 5, 100);
        hit(&mut game, a);
        game.drain_events();
        let encoded = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        let mut restored =
            GameState::from_snapshot(bincode::deserialize(&encoded).unwrap(), game.rules.clone());
        for state in [&mut game, &mut restored] {
            hit(state, b);
            assert_eq!(state.alternation_previous(state.player), Some((b, 4)));
            assert_eq!(state.pending_weapon_echoes().count(), 2);
            wait(state);
            assert_eq!(state.alternation_previous(state.player), Some((b, 3)));
            assert_eq!(state.pending_weapon_echoes().count(), 1);
        }
        assert_eq!(format!("{game:?}"), format!("{restored:?}"));
        assert_eq!(game.events(), restored.events());
    }

    #[test]
    fn scheduled_echo_survives_source_death_and_another_bearer_cannot_steal_or_duplicate_it() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![echo()]);
        let source = actor(&mut game, 3, 4, 5);
        let target = actor(&mut game, 7, 3, 100);
        let at = GridPos::new(7, 3);
        game.apply_weapon_echo(source, target, at, echo().kind(), None);
        game.apply_weapon_echo(game.player, target, at, echo().kind(), None);
        assert_eq!(game.pending_weapon_echoes().count(), 1);
        assert_eq!(game.weapon_echoes.pending[0].source, source);
        game.apply_damage_to(None, source, DamagePacket::new(100, DamageType::Kinetic, 0))
            .unwrap();
        for _ in 0..3 {
            wait(&mut game);
        }
        assert_eq!(game.actors.get(target).unwrap().integrity(), 97);
        assert_eq!(resolved(&game), 1);
        assert!(game.weapon_echoes.is_empty());
        // A bearer already dead cannot schedule a new echo or remember a target.
        game.apply_weapon_echo(source, target, at, echo().kind(), None);
        game.apply_weapon_echo(source, target, at, alternation().kind(), None);
        assert!(game.weapon_echoes.is_empty());
    }

    #[test]
    fn absorbed_hits_still_seed_echoes_but_protected_bearer_cannot_link() {
        use crate::combat::ArmorRules;
        use crate::stats::BodyProfile;
        let mut game = fixture(AttackDelivery::Ranged, false, vec![alternation(), echo()]);
        game.rules.armor_rules = Some(ArmorRules::default());
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(7, 3), 100)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_body_profile(BodyProfile::new(100, 0).unwrap().with_base_armor(100)),
            )
            .unwrap();
        hit(&mut game, target);
        assert_eq!(game.actors.get(target).unwrap().integrity(), 100);
        assert_eq!(game.alternation_previous(game.player), Some((target, 4)));
        assert_eq!(game.pending_weapon_echoes().count(), 1);
        game.map
            .set_protected(game.actors.get(game.player).unwrap().position(), true)
            .unwrap();
        let second = actor(&mut game, 7, 5, 100);
        assert!(!game.alternation_can_link(game.player, second, 4));
    }
}
