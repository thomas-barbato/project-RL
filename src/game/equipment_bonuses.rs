//! Instance bonuses are overlays. Intrinsic attributes and weapon definitions
//! remain unchanged; learned-skill prerequisites still use intrinsic values.
use super::*;

impl GameState {
    pub(super) fn drop_actor_weapon(&mut self, entity: EntityId, actor: &Actor) {
        let Some(weapon) = actor.equipped_weapon() else {
            return;
        };
        let at = actor.position();
        let ground_item = self
            .ground_items
            .spawn_remains(at, weapon.item().clone(), weapon.modifiers().cloned())
            .expect("a single carried weapon fits the ground item ID space");
        self.events.push(GameEvent::ActorEquipmentDropped {
            entity,
            ground_item,
            at,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{ArmorRules, DamageType, HitRules, MeleeImpactProfile};
    use crate::entity::MagicItemModifiers;
    use crate::stats::{BodyProfile, PhysicalRules};
    use crate::weapon::WeaponCatalog;

    fn fixture() -> (GameState, ItemInstanceId, ItemInstanceId) {
        let id: WeaponId = "test:blade".parse().unwrap();
        let attack = AttackProfile::melee(DamageType::Kinetic, 5)
            .with_melee_impact(MeleeImpactProfile {
                material_cap: 100,
                impact_modifier: 0,
            })
            .unwrap();
        let mut weapons = WeaponCatalog::default();
        weapons
            .register(
                WeaponDefinition::new(id.clone(), "name".into(), "description".into(), attack)
                    .unwrap(),
            )
            .unwrap();
        let game = GameState::new_with_rules(
            Map::filled(8, 6, Terrain::Floor).unwrap(),
            GridPos::new(2, 2),
            12,
            GameRules {
                weapons,
                physical_rules: Some(PhysicalRules::default()),
                hit_rules: Some(HitRules::default()),
                armor_rules: Some(ArmorRules::default()),
                player_system_resources: Some(crate::game::SystemResourceRules::default()),
                player_body_profile: Some(BodyProfile::new(20, 0).unwrap()),
                player_weapon_slots: vec![
                    "test:slot".parse().unwrap(),
                    "test:aux".parse().unwrap(),
                ],
                player_starting_weapons: vec![id.clone()],
                player_starting_equipment: vec![Some(id.clone())],
                ..GameRules::default()
            },
        )
        .unwrap()
        .with_starting_magic_equipment([(
            id,
            MagicItemModifiers::weapon_bonuses([2, 2, 2, 1, 1], 12, 3).unwrap(),
        )])
        .unwrap();
        let plain = game
            .player_inventory
            .iter()
            .find(|entry| entry.magic_modifiers().is_none())
            .unwrap()
            .instance();
        let magic = game
            .player_inventory
            .iter()
            .find(|entry| entry.magic_modifiers().is_some())
            .unwrap()
            .instance();
        (game, plain, magic)
    }

    #[test]
    fn equipped_primary_bonuses_change_real_stats_without_rewriting_base_or_healing() {
        let (mut game, plain, magic) = fixture();
        let base = game.player_primary_attributes().unwrap();
        let authored = game.equipped_player_weapon(0).unwrap().attack();
        let hp = game.actors.get(game.player).unwrap().integrity();
        let evasion = game.actor_evasion(game.player).unwrap();
        let damage = game
            .resolved_attack_damage(game.player, authored)
            .unwrap()
            .raw_total();
        game.equip_player_weapon(0, magic).unwrap();
        assert_eq!(game.player_primary_attributes(), Some(base));
        assert_eq!(game.equipped_player_weapon(0).unwrap().attack(), authored);
        assert_eq!(
            game.player_effective_primary_attributes()
                .unwrap()
                .value(PrimaryAttribute::Power),
            base.value(PrimaryAttribute::Power) + 2
        );
        assert!(game.actor_evasion(game.player).unwrap() > evasion);
        assert_eq!(
            game.resolved_attack_damage(game.player, authored)
                .unwrap()
                .raw_total(),
            damage + 4
        );
        assert!(game.actors.get(game.player).unwrap().maximum_integrity() > hp);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
        game.equip_player_weapon(0, plain).unwrap();
        game.equip_player_weapon(0, magic).unwrap();
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
        // Moving an instance to a second slot never doubles its character bonus.
        game.equip_player_weapon(1, magic).unwrap();
        assert_eq!(game.player_attribute_bonus(PrimaryAttribute::Power), 2);
    }

    #[test]
    fn equipped_handling_bonuses_are_shared_and_survive_drop_pickup_and_snapshot() {
        let (mut game, plain, magic) = fixture();
        game.equip_player_weapon(1, magic).unwrap();
        let plain_attack = game.attack_details(game.player, 0).unwrap().1;
        let bonus_attack = game.attack_details(game.player, 1).unwrap().1;
        assert_eq!(plain_attack.accuracy_modifier(), 12);
        assert_eq!(plain_attack.damage().armor_penetration(), 3);
        assert_eq!(bonus_attack.accuracy_modifier(), 12);
        assert_eq!(bonus_attack.damage().armor_penetration(), 3);
        assert_eq!(game.player_item_attack_profile(plain), Some(plain_attack));
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(3, 2), 100)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_body_profile(BodyProfile::new(100, 0).unwrap().with_base_armor(4)),
            )
            .unwrap();
        let packet = game
            .resolved_attack_damage(game.player, bonus_attack)
            .unwrap();
        game.process_player_command(GameCommand::Attack { slot: 1, target });
        assert!(game.events().iter().any(
            |e| matches!(e, GameEvent::DamageApplied { target: t, amount, .. } | GameEvent::DamageImpactApplied { target: t, amount, .. }
            if *t == target && *amount == packet.raw_total() - 1)
        ), "{:?}", game.events());
        let bonus = game.player_inventory.get(magic).unwrap().magic_modifiers();
        game.process_player_command(GameCommand::DropItem { item: magic });
        assert_eq!(game.player_attribute_bonus(PrimaryAttribute::Power), 0);
        assert_eq!(
            game.ground_items.iter().next().unwrap().1.magic_modifiers(),
            bonus
        );
        game.process_player_command(GameCommand::PickUp);
        let recovered = game
            .player_inventory
            .iter()
            .find(|entry| entry.magic_modifiers() == bonus)
            .unwrap()
            .instance();
        game.equip_player_weapon(1, recovered).unwrap();
        game.drain_events();
        let encoded = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        let restored =
            GameState::from_snapshot(bincode::deserialize(&encoded).unwrap(), game.rules.clone());
        assert_eq!(
            restored.player_effective_primary_attributes(),
            game.player_effective_primary_attributes()
        );
        assert_eq!(
            restored.attack_details(restored.player, 1).unwrap().1,
            bonus_attack
        );
    }
    #[test]
    fn resource_bonuses_change_capacity_and_cooling_without_free_refills() {
        let (mut game, plain, _) = fixture();
        let bonus = MagicItemModifiers::rpg_bonuses([0; 5], 0, 0, 10, 15, 3).unwrap();
        let item = game
            .player_inventory
            .add_magic("test:blade".parse().unwrap(), None, bonus.clone())
            .unwrap();
        let base_hp = game.actors.get(game.player).unwrap().maximum_integrity();
        let base_energy = game.player_energy.capacity();
        let base_cooling = game.player_heat.unwrap().dissipation_per_phase();
        game.player_energy.spend(30).unwrap();
        game.player_heat.as_mut().unwrap().add(20);
        game.equip_player_weapon(0, item).unwrap();
        assert_eq!(
            game.actors.get(game.player).unwrap().maximum_integrity(),
            base_hp + 10
        );
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), base_hp);
        assert_eq!(game.player_energy.capacity(), base_energy + 15);
        assert_eq!(game.player_energy.available(), base_energy - 30);
        assert_eq!(game.player_heat.unwrap().current(), 20);
        assert_eq!(
            game.player_heat.unwrap().dissipation_per_phase(),
            base_cooling + 3
        );
        game.dissipate_player_heat();
        assert_eq!(game.player_heat.unwrap().current(), 20 - base_cooling - 3);
        game.drain_events();
        let restored = GameState::from_snapshot(game.snapshot().unwrap(), game.rules.clone());
        assert_eq!(restored.player_heat(), game.player_heat());
        assert_eq!(restored.player_energy(), game.player_energy());
        assert_eq!(
            restored
                .actors
                .get(game.player)
                .unwrap()
                .maximum_integrity(),
            base_hp + 10
        );
        game.equip_player_weapon(0, plain).unwrap();
        assert_eq!(
            game.actors.get(game.player).unwrap().maximum_integrity(),
            base_hp
        );
        assert_eq!(game.player_energy.capacity(), base_energy);
        assert_eq!(
            game.player_heat.unwrap().dissipation_per_phase(),
            base_cooling
        );
        let heat = game.player_heat.unwrap().current();
        game.equip_player_weapon(0, item).unwrap();
        assert_eq!(game.player_energy.available(), base_energy - 30);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), base_hp);
        assert_eq!(game.player_heat.unwrap().current(), heat);
        game.process_player_command(GameCommand::DropItem { item });
        assert_eq!(
            game.ground_items.iter().next().unwrap().1.magic_modifiers(),
            Some(bonus)
        );
    }

    #[test]
    fn hp_bonus_also_works_without_physical_rules_and_totals_may_exceed_intrinsic_cap() {
        let (mut game, plain, _) = fixture();
        game.rules.physical_rules = None;
        let bonus = MagicItemModifiers::rpg_bonuses([10, 0, 0, 0, 0], 0, 0, 10, 0, 0).unwrap();
        let item = game
            .player_inventory
            .add_magic("test:blade".parse().unwrap(), None, bonus)
            .unwrap();
        game.equip_player_weapon(0, item).unwrap();
        assert_eq!(
            game.actors.get(game.player).unwrap().maximum_integrity(),
            game.rules.player_maximum_integrity + 10
        );
        assert!(
            game.player_effective_primary_attributes()
                .unwrap()
                .value(PrimaryAttribute::Power)
                > 10
        );
        assert_eq!(
            game.player_primary_attributes(),
            Some(game.rules.player_starting_attributes)
        );
        game.equip_player_weapon(0, plain).unwrap();
        assert!(
            game.actors.get(game.player).unwrap().integrity()
                <= game.rules.player_maximum_integrity
        );
    }
}

impl GameState {
    /// Scenario construction only. Effects are attached to existing instances,
    /// including the initially equipped items, without a runtime reroll.
    pub fn with_starting_weapon_effects(
        mut self,
        effects: impl IntoIterator<Item = (crate::item::ItemId, crate::content::ContentId)>,
    ) -> Result<Self, String> {
        if self.turn != 0 || self.phase != super::TurnPhase::AwaitingPlayer {
            return Err("Starting effects can only be supplied before turn one".into());
        }
        for (item, effect) in effects {
            if self
                .rules
                .weapons
                .resolve_instance(&item, Some(&effect))
                .is_none()
            {
                return Err(format!(
                    "Unknown weapon or effect affix '{item}' / '{effect}'"
                ));
            }
            let instances: Vec<_> = self
                .player_inventory
                .iter()
                .filter(|entry| entry.item() == &item)
                .map(|entry| entry.instance())
                .collect();
            if instances.is_empty() {
                return Err(format!("Missing starting weapon '{item}'"));
            }
            for instance in instances {
                self.player_inventory
                    .attach_effect_affix(instance, effect.clone())
                    .map_err(|error| error.to_string())?;
            }
        }
        Ok(self)
    }

    fn equipped_bonus_total(
        &self,
        value: impl Fn(&crate::entity::MagicItemModifiers) -> u16,
    ) -> u16 {
        self.player_equipment
            .iter()
            .filter_map(|(_, item)| self.player_inventory.get(item))
            .filter_map(|entry| entry.magic_modifiers())
            .map(|bonus| value(&bonus))
            .fold(0, u16::saturating_add)
    }

    pub(super) fn apply_equipped_attack_bonuses(&self, attack: AttackProfile) -> AttackProfile {
        let accuracy = self.equipped_bonus_total(crate::entity::MagicItemModifiers::accuracy_bonus);
        let penetration =
            self.equipped_bonus_total(crate::entity::MagicItemModifiers::armor_penetration_bonus);
        crate::entity::MagicItemModifiers::weapon_bonuses([0; 5], accuracy, penetration)
            .map_or(attack, |bonus| bonus.modify_weapon_attack(attack))
    }

    pub fn player_attribute_bonus(&self, attribute: PrimaryAttribute) -> u16 {
        self.player_equipment
            .iter()
            .filter_map(|(_, item)| self.player_inventory.get(item))
            .filter_map(|entry| entry.magic_modifiers())
            .map(|bonus| u16::from(bonus.attribute_bonus(attribute)))
            .fold(0, u16::saturating_add)
    }

    pub fn actor_effective_primary_attributes(
        &self,
        entity: EntityId,
    ) -> Option<PrimaryAttributes> {
        let mut attributes = self.actors.get(entity)?.primary_attributes()?;
        if entity != self.player {
            return self.actors.get(entity)?.equipment_primary_attributes();
        }
        if entity == self.player {
            for attribute in PrimaryAttribute::ALL {
                let total = u16::from(attributes.value(attribute))
                    .saturating_add(self.player_attribute_bonus(attribute));
                // The 1..10 rule is for intrinsic character data, not an
                // implicit deletion of equipment bonuses above that value.
                attributes =
                    attributes.with_value(attribute, u8::try_from(total).unwrap_or(u8::MAX));
            }
        }
        Some(attributes)
    }

    pub fn player_effective_primary_attributes(&self) -> Option<PrimaryAttributes> {
        self.actor_effective_primary_attributes(self.player)
    }

    /// Resolve only this item's intrinsic and rolled effects, never those of
    /// another equipped weapon. Numerical bonuses retain their equipment scope.
    pub fn player_item_weapon(
        &self,
        item: ItemInstanceId,
    ) -> Option<crate::weapon::WeaponDefinition> {
        let entry = self.player_inventory().get(item)?;
        let modifiers = entry.magic_modifiers();
        self.rules().weapons.resolve_instance(
            entry.item(),
            modifiers.as_ref().and_then(|bonus| bonus.effect_affix()),
        )
    }

    pub fn resolved_equipped_player_weapon(
        &self,
        slot: u8,
    ) -> Option<crate::weapon::WeaponDefinition> {
        self.player_item_weapon(self.equipped_player_weapon_item(slot)?)
    }

    pub fn player_item_attack_profile(&self, item: ItemInstanceId) -> Option<AttackProfile> {
        let entry = self.player_inventory.get(item)?;
        let attack = self.rules.weapons.get(entry.item())?.attack();
        if self.player_equipment.slot_of(item).is_some() {
            return Some(self.apply_equipped_attack_bonuses(attack));
        }
        Some(
            entry
                .magic_modifiers()
                .map_or(attack, |bonus| bonus.modify_weapon_attack(attack)),
        )
    }

    pub(super) fn refresh_equipment_resources(&mut self) {
        let bonuses: Vec<_> = self
            .player_equipment
            .iter()
            .filter_map(|(_, item)| self.player_inventory.get(item))
            .filter_map(|entry| entry.magic_modifiers())
            .collect();
        let hp = bonuses
            .iter()
            .map(|b| b.maximum_hit_points_bonus())
            .fold(0, u16::saturating_add);
        let energy = bonuses
            .iter()
            .map(|b| b.energy_capacity_bonus())
            .fold(0, u16::saturating_add);
        let cooling = bonuses
            .iter()
            .map(|b| b.heat_dissipation_bonus())
            .fold(0, u16::saturating_add);
        let maximum = match (
            self.rules.physical_rules,
            self.actors.get(self.player).and_then(Actor::body_profile),
        ) {
            (Some(rules), Some(body)) => rules.hit_points.maximum_for_body(
                body,
                self.player_effective_primary_attributes(),
                0,
            ),
            _ => self.rules.player_maximum_integrity,
        }
        .saturating_add(hp);
        if let Some(actor) = self.actors.get_mut(self.player) {
            actor.resize_integrity_without_healing(maximum);
        }
        self.player_energy
            .set_capacity(self.rules.player_energy_capacity.saturating_add(energy));
        if let (Some(heat), Some(rules)) = (
            self.player_heat.as_mut(),
            self.rules.player_system_resources,
        ) {
            heat.set_dissipation_per_phase(
                rules.heat_dissipation_per_phase.saturating_add(cooling),
            );
        }
    }
}
