use super::*;
use crate::game::StatusRemovalReason;

impl GameState {
    /// Consume at most one guard for an entire HP impact, after armor and
    /// resistances. Component durability is deliberately outside this contract.
    pub(super) fn consume_damage_guard(
        &mut self,
        target: EntityId,
        at: GridPos,
        damage: u16,
    ) -> u16 {
        if damage == 0 {
            return 0;
        }
        let Some(actor) = self.actors.get(target) else {
            return damage;
        };
        // Different statuses coexist, but their guard values are not added.
        // Strongest first; equal values use stable ascending status identity.
        let mut selected: Option<(StatusId, u16)> = None;
        for instance in actor.statuses() {
            let Some(definition) = self.rules.statuses.get(&instance.definition) else {
                continue;
            };
            for modifier in definition.modifiers() {
                if let StatusModifier::DamageGuard { amount } = modifier
                    && selected
                        .as_ref()
                        .is_none_or(|(_, previous)| amount > previous)
                {
                    selected = Some((instance.definition.clone(), *amount));
                }
            }
        }
        let Some((status, capacity)) = selected else {
            return damage;
        };
        self.actors.get_mut(target).unwrap().remove_status(&status);
        let amount = damage.min(capacity);
        self.events.push(GameEvent::DamageGuardAbsorbed {
            target,
            at,
            status: status.clone(),
            amount,
        });
        self.events.push(GameEvent::StatusRemoved {
            target,
            status,
            reason: StatusRemovalReason::Consumed,
        });
        damage - amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{ArmorRules, ConeAttack, DamageComponent, DamageType, HitRules};
    use crate::status::{StatusDefinitionError, StatusStacking, StatusTransition};
    use crate::weapon::WeaponCatalog;

    fn id() -> StatusId {
        "test:guard".parse().unwrap()
    }

    fn definition(name: &str, amount: u16) -> StatusDefinition {
        StatusDefinition::new(
            name.parse().unwrap(),
            Some(3),
            StatusStacking::KeepExisting,
            vec![],
        )
        .unwrap()
        .with_modifiers([StatusModifier::DamageGuard { amount }])
        .unwrap()
    }

    fn fixture(cone: bool) -> GameState {
        let mut statuses = StatusCatalog::default();
        statuses.register(definition("test:guard", 3)).unwrap();
        let mut attack = AttackProfile::new(
            10,
            DistanceMetric::Chebyshev,
            true,
            DamageType::Kinetic,
            4,
            0,
        );
        if cone {
            attack = attack.with_area(AttackArea::Cone(ConeAttack::new(1, 1, 1).unwrap()));
        }
        let weapon: WeaponId = "test:guard_weapon".parse().unwrap();
        let mut weapons = WeaponCatalog::default();
        weapons
            .register(
                WeaponDefinition::new(weapon.clone(), "name".into(), "description".into(), attack)
                    .unwrap()
                    .with_effects([WeaponEffect::apply_bearer_status(
                        ApplyStatusEffect::new(id(), 1).unwrap(),
                        WeaponEffectTrigger::OnDamage,
                    )
                    .unwrap()]),
            )
            .unwrap();
        GameState::new_with_rules(
            Map::filled(14, 7, Terrain::Floor).unwrap(),
            GridPos::new(2, 3),
            7,
            GameRules {
                statuses,
                weapons,
                player_base_attacks: vec![attack],
                player_weapon_slots: vec!["test:slot".parse().unwrap()],
                player_starting_weapons: vec![weapon.clone()],
                player_starting_equipment: vec![Some(weapon)],
                ..GameRules::default()
            },
        )
        .unwrap()
    }

    fn grant(game: &mut GameState) {
        game.apply_status_to(
            Some(game.player),
            game.player,
            &ApplyStatusEffect::new(id(), 1).unwrap(),
        )
        .unwrap();
    }

    fn has_guard(game: &GameState) -> bool {
        game.actors
            .get(game.player)
            .unwrap()
            .status(&id())
            .is_some()
    }

    fn absorptions(game: &GameState) -> Vec<u16> {
        game.events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::DamageGuardAbsorbed { amount, .. } => Some(*amount),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn guard_requires_finite_positive_nonrefreshable_definition() {
        for (duration, stacking, modifiers, expected) in [
            (
                None,
                StatusStacking::KeepExisting,
                vec![StatusModifier::DamageGuard { amount: 3 }],
                StatusDefinitionError::UnboundedDamageGuard,
            ),
            (
                Some(3),
                StatusStacking::RefreshDuration,
                vec![StatusModifier::DamageGuard { amount: 3 }],
                StatusDefinitionError::UnboundedDamageGuard,
            ),
            (
                Some(3),
                StatusStacking::AddStacks {
                    maximum_stacks: 3,
                    refresh_duration: false,
                },
                vec![StatusModifier::DamageGuard { amount: 3 }],
                StatusDefinitionError::UnboundedDamageGuard,
            ),
            (
                Some(3),
                StatusStacking::KeepExisting,
                vec![StatusModifier::DamageGuard { amount: 0 }],
                StatusDefinitionError::ZeroDamageGuard,
            ),
            (
                Some(3),
                StatusStacking::KeepExisting,
                vec![StatusModifier::DamageGuard { amount: 3 }; 2],
                StatusDefinitionError::DuplicateDamageGuard,
            ),
        ] {
            assert_eq!(
                StatusDefinition::new(id(), duration, stacking, vec![])
                    .unwrap()
                    .with_modifiers(modifiers),
                Err(expected)
            );
        }
    }

    #[test]
    fn guard_absorbs_once_discards_unused_capacity_and_preserves_zero_impacts() {
        for incoming in [1, 3, 4, 8] {
            let mut game = fixture(false);
            grant(&mut game);
            let hp = game.actors.get(game.player).unwrap().integrity();
            game.apply_damage_to(
                None,
                game.player,
                DamagePacket::new(0, DamageType::Electrical, 0),
            )
            .unwrap();
            assert!(has_guard(&game));
            assert!(absorptions(&game).is_empty());
            let result = game
                .apply_damage_to(
                    None,
                    game.player,
                    DamagePacket::new(incoming, DamageType::Electrical, 0),
                )
                .unwrap();
            assert_eq!(result.amount, incoming.saturating_sub(3));
            assert_eq!(absorptions(&game), [incoming.min(3)]);
            assert!(!has_guard(&game));
            game.apply_damage_to(
                None,
                game.player,
                DamagePacket::new(1, DamageType::Electrical, 0),
            )
            .unwrap();
            assert_eq!(
                game.actors.get(game.player).unwrap().integrity(),
                hp - incoming.saturating_sub(3) - 1
            );
            assert_eq!(absorptions(&game).len(), 1);
        }
    }

    #[test]
    fn guard_resolves_after_armor_and_once_for_mixed_damage() {
        let mut game = fixture(false);
        game.rules.armor_rules = Some(ArmorRules::default());
        let actor = game
            .actors
            .get(game.player)
            .unwrap()
            .clone()
            .with_body_profile(
                crate::stats::BodyProfile::new(30, 0)
                    .unwrap()
                    .with_base_armor(2),
            );
        *game.actors.get_mut(game.player).unwrap() = actor;
        grant(&mut game);
        assert_eq!(
            game.apply_damage_to(
                None,
                game.player,
                DamagePacket::new(2, DamageType::Kinetic, 0)
            )
            .unwrap()
            .amount,
            0
        );
        assert!(has_guard(&game));
        let impact = DamageImpact::mixed(
            [
                DamageComponent::new(4, DamageType::Kinetic),
                DamageComponent::new(4, DamageType::Electrical),
            ],
            0,
            [],
        )
        .unwrap();
        assert_eq!(
            game.apply_damage_impact_to(None, game.player, impact)
                .unwrap()
                .amount,
            3
        );
        assert_eq!(absorptions(&game), [3]);
        assert!(game.events().iter().any(|event| matches!(event,
            GameEvent::DamageImpactApplied { amount: 3, absorbed_by_armor: 2, components, .. }
                if components.iter().map(|c| c.amount).sum::<u16>() == 6)));
    }

    #[test]
    fn guard_does_not_stack_refresh_or_replace_other_timed_effects() {
        let mut game = fixture(false);
        grant(&mut game);
        let other: StatusId = "test:other".parse().unwrap();
        game.rules
            .statuses
            .register(
                StatusDefinition::new(
                    other.clone(),
                    Some(9),
                    StatusStacking::RefreshDuration,
                    vec![],
                )
                .unwrap(),
            )
            .unwrap();
        game.apply_status_to(
            None,
            game.player,
            &ApplyStatusEffect::new(other.clone(), 1).unwrap(),
        )
        .unwrap();
        game.process_player_command(GameCommand::Wait);
        grant(&mut game);
        let status = game.actors.get(game.player).unwrap().status(&id()).unwrap();
        assert_eq!(status.remaining_turns, Some(2));
        assert_eq!(status.stacks, 1);
        game.process_player_command(GameCommand::Wait);
        grant(&mut game);
        game.process_player_command(GameCommand::Wait);
        assert!(!has_guard(&game));
        assert!(
            game.actors
                .get(game.player)
                .unwrap()
                .status(&other)
                .is_some()
        );
        assert!(game.events().iter().any(|event| matches!(event, GameEvent::StatusRemoved { status, reason: StatusRemovalReason::Expired, .. } if status == &id())));
    }

    #[test]
    fn guard_proc_is_once_per_normal_damaging_attack_including_a_lethal_hit() {
        for (count, hp) in [(0, 30), (1, 30), (3, 30), (1, 1)] {
            let mut game = fixture(true);
            for x in 5..5 + count {
                game.spawn_actor(Actor::new(GridPos::new(x, 3), hp).unwrap())
                    .unwrap();
            }
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: GridPos::new(8, 3),
            });
            assert_eq!(game.events().iter().filter(|event| matches!(event, GameEvent::StatusApplied { target, status, .. } if *target == game.player && status == &id())).count(), usize::from(count > 0));
            assert_eq!(has_guard(&game), count > 0);
        }
    }

    #[test]
    fn guard_proc_requires_direct_damage_and_does_not_trigger_from_reactions() {
        for denial in ["miss", "armor", "protected", "reaction"] {
            let mut game = fixture(false);
            let target = game
                .spawn_actor(Actor::new(GridPos::new(8, 3), 30).unwrap())
                .unwrap();
            match denial {
                "miss" => {
                    game.rules.hit_rules = Some(HitRules {
                        minimum_hit_chance: 0,
                        maximum_hit_chance: 0,
                        ..HitRules::default()
                    })
                }
                "armor" => {
                    game.rules.armor_rules = Some(ArmorRules::default());
                    let actor = game.actors.get(target).unwrap().clone().with_body_profile(
                        crate::stats::BodyProfile::new(30, 0)
                            .unwrap()
                            .with_base_armor(100),
                    );
                    *game.actors.get_mut(target).unwrap() = actor;
                }
                "protected" => game.map.set_protected(GridPos::new(8, 3), true).unwrap(),
                _ => {}
            }
            if denial == "reaction" {
                let attack = game
                    .prepare_targeted_attack(game.player, 0, target)
                    .unwrap();
                game.resolve_prepared_attack(attack, ActionOrigin::Reaction)
                    .unwrap();
            } else {
                game.process_player_command(GameCommand::Attack { slot: 0, target });
            }
            assert!(!has_guard(&game), "{denial}");
        }
    }

    #[test]
    fn strongest_guard_is_consumed_alone_and_does_not_trigger_expiration_transition() {
        let mut game = fixture(false);
        let next: StatusId = "test:next".parse().unwrap();
        game.rules
            .statuses
            .register(
                StatusDefinition::new(next.clone(), Some(2), StatusStacking::KeepExisting, vec![])
                    .unwrap(),
            )
            .unwrap();
        let strong = definition("test:strong", 5)
            .with_expiration_transition(StatusTransition::new(next.clone(), 1).unwrap());
        game.rules.statuses.register(strong).unwrap();
        grant(&mut game);
        game.apply_status_to(
            None,
            game.player,
            &ApplyStatusEffect::new("test:strong".parse().unwrap(), 1).unwrap(),
        )
        .unwrap();
        assert_eq!(
            game.apply_damage_to(
                None,
                game.player,
                DamagePacket::new(8, DamageType::Electrical, 0)
            )
            .unwrap()
            .amount,
            3
        );
        assert_eq!(absorptions(&game), [5]);
        assert!(has_guard(&game));
        assert!(
            game.actors
                .get(game.player)
                .unwrap()
                .status(&next)
                .is_none()
        );
    }

    #[test]
    fn guard_preserves_protected_actors_and_component_durability_is_not_shielded() {
        let mut game = fixture(false);
        let component: BodyComponentId = "test:component".parse().unwrap();
        let body = BodyComponentProfile::new(
            component.clone(),
            "component".into(),
            10,
            2,
            ComponentFailureEffect::DisableMovement,
        )
        .unwrap();
        let actor = game
            .actors
            .get(game.player)
            .unwrap()
            .clone()
            .with_body_components([body]);
        *game.actors.get_mut(game.player).unwrap() = actor;
        grant(&mut game);
        let at = game.actors.get(game.player).unwrap().position();
        game.map.set_protected(at, true).unwrap();
        assert_eq!(
            game.apply_damage_to(
                None,
                game.player,
                DamagePacket::new(8, DamageType::Electrical, 0)
            )
            .unwrap()
            .amount,
            0
        );
        assert!(has_guard(&game));
        game.map.set_protected(at, false).unwrap();
        let impact = DamageImpact::single(DamagePacket::new(2, DamageType::Electrical, 0));
        assert_eq!(
            game.apply_damage_impact_to_component(None, game.player, &component, impact)
                .unwrap()
                .amount,
            2
        );
        assert!(has_guard(&game));
        assert!(absorptions(&game).is_empty());
        // Periodic/status-origin damage shares the HP path even with hooks off.
        assert_eq!(
            game.apply_damage_impact_to_with_status_hooks(None, game.player, impact, false)
                .unwrap()
                .amount,
            0
        );
        assert!(!has_guard(&game));
        assert_eq!(absorptions(&game), [2]);
    }

    #[test]
    fn guard_does_not_prevent_lethal_excess_or_hide_unknown_bearer_statuses() {
        let mut game = fixture(false);
        grant(&mut game);
        let hp = game.actors.get(game.player).unwrap().integrity();
        let result = game
            .apply_damage_to(
                None,
                game.player,
                DamagePacket::new(u16::MAX, DamageType::Electrical, 0),
            )
            .unwrap();
        assert!(result.target_destroyed);
        assert_eq!(result.amount, hp);
        assert_eq!(absorptions(&game), [3]);
        let mut rules = fixture(false).rules;
        rules.statuses = StatusCatalog::default();
        assert_eq!(
            first_unknown_weapon_status(&rules),
            Some(("test:guard_weapon".parse().unwrap(), id()))
        );
    }

    #[test]
    fn active_guard_survives_snapshot_round_trip_without_new_save_fields() {
        let mut game = fixture(false);
        grant(&mut game);
        game.drain_events();
        let encoded = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        let mut restored =
            GameState::from_snapshot(bincode::deserialize(&encoded).unwrap(), game.rules.clone());
        for state in [&mut game, &mut restored] {
            state
                .apply_damage_to(
                    None,
                    state.player,
                    DamagePacket::new(5, DamageType::Electrical, 0),
                )
                .unwrap();
            state.process_player_command(GameCommand::Wait);
        }
        assert_eq!(format!("{game:?}"), format!("{restored:?}"));
        assert_eq!(game.events(), restored.events());
    }
}
