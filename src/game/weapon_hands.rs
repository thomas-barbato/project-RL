//! Hand slots coexist; the client selects either the pair or the two-hand weapon.
use super::{GameRules, GameState};
use crate::combat::AttackDelivery;
use crate::weapon::WeaponDefinition;

pub const WEAPON_HAND_SLOTS: [&str; 3] = ["core:left_hand", "core:right_hand", "core:both_hands"];
pub const WEAPON_SET_SLOTS: [&str; 4] = [
    "core:left_hand",
    "core:right_hand",
    "core:reserve_left_hand",
    "core:reserve_right_hand",
];
const LEGACY_WEAPON_SLOTS: [&str; 3] = [
    "core:close_combat_channel",
    "core:ranged_combat_channel",
    "core:auxiliary_combat_channel",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponHands {
    One,
    Two,
}

impl WeaponHands {
    pub fn from_family(family: Option<&str>, delivery: AttackDelivery) -> Self {
        match delivery {
            AttackDelivery::Melee if matches!(family, Some("core:axes" | "core:swords")) => {
                Self::Two
            }
            AttackDelivery::Melee => Self::One,
            AttackDelivery::Ranged
                if matches!(
                    family,
                    Some(
                        "core:pistols"
                            | "core:submachine_guns"
                            | "core:handguns"
                            | "core:revolvers"
                    )
                ) =>
            {
                Self::One
            }
            AttackDelivery::Ranged => Self::Two,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::One => "Une main",
            Self::Two => "Deux mains",
        }
    }
}

impl GameRules {
    pub fn uses_weapon_hands(&self) -> bool {
        self.uses_weapon_sets() || self.uses_legacy_weapon_hands()
    }
    pub fn uses_legacy_weapon_hands(&self) -> bool {
        self.player_weapon_slots
            .iter()
            .map(|slot| slot.as_str())
            .eq(WEAPON_HAND_SLOTS)
    }
    pub fn uses_weapon_sets(&self) -> bool {
        self.player_weapon_slots
            .iter()
            .map(|slot| slot.as_str())
            .eq(WEAPON_SET_SLOTS)
    }
    pub fn weapon_hands(&self, weapon: &WeaponDefinition) -> WeaponHands {
        let family = self
            .monster_equipment_loot
            .as_ref()
            .and_then(|catalog| catalog.family(weapon.id()));
        WeaponHands::from_family(family.map(|id| id.as_str()), weapon.attack().delivery())
    }
    pub fn weapon_fits_slot(&self, weapon: &WeaponDefinition, slot: usize) -> bool {
        slot < self.player_weapon_slots.len()
            && (self.uses_weapon_sets()
                || !self.uses_weapon_hands()
                || matches!(
                    (self.weapon_hands(weapon), slot),
                    (WeaponHands::One, 0 | 1) | (WeaponHands::Two, 2)
                ))
    }
    /// Keep all starting weapons in the bag; equip up to two singles and one double.
    pub fn enable_weapon_hands(&mut self) {
        if self.uses_weapon_sets() {
            // Historical rule projections must recover the old hand layout.
            // All starting instances remain in the bag even if that layout
            // cannot carry a fourth weapon.
            self.player_weapon_slots = LEGACY_WEAPON_SLOTS.map(|id| id.parse().unwrap()).into();
        }
        if !self.uses_weapon_hands()
            && !self
                .player_weapon_slots
                .iter()
                .map(|slot| slot.as_str())
                .eq(LEGACY_WEAPON_SLOTS)
        {
            return;
        }
        if self.uses_weapon_hands()
            && self
                .player_starting_equipment
                .iter()
                .enumerate()
                .all(|(slot, id)| {
                    id.as_ref().is_none_or(|id| {
                        self.weapons
                            .get(id)
                            .is_some_and(|weapon| self.weapon_fits_slot(weapon, slot))
                    })
                })
        {
            return;
        }
        let preferred = self
            .player_starting_equipment
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        let mut equipped = vec![None; 3];
        // The old third slot already housed the alternative heavy weapon.
        // Retain that choice when multiple starting weapons need both hands.
        if let Some(id) = self
            .player_starting_equipment
            .get(2)
            .and_then(Option::as_ref)
        {
            if self
                .weapons
                .get(id)
                .is_some_and(|weapon| self.weapon_hands(weapon) == WeaponHands::Two)
            {
                equipped[2] = Some(id.clone());
            }
        }
        for id in preferred {
            let Some(weapon) = self.weapons.get(&id) else {
                continue;
            };
            let candidates: &[usize] = match self.weapon_hands(weapon) {
                WeaponHands::One => &[0, 1],
                WeaponHands::Two => &[2],
            };
            if let Some(&slot) = candidates.iter().find(|&&slot| equipped[slot].is_none()) {
                equipped[slot] = Some(id);
            }
        }
        self.player_weapon_slots = WEAPON_HAND_SLOTS.map(|id| id.parse().unwrap()).into();
        self.player_starting_equipment = equipped;
    }

    pub fn enable_weapon_sets(&mut self) {
        if self.uses_weapon_sets() {
            return;
        }
        self.enable_weapon_hands();
        if !self.uses_legacy_weapon_hands() {
            return;
        }
        let mut equipment = self.player_starting_equipment.clone();
        equipment.resize(4, None);
        if equipment[0].is_none() && equipment[1].is_none() {
            equipment[0] = equipment[2].take();
        }
        self.player_weapon_slots = WEAPON_SET_SLOTS.map(|id| id.parse().unwrap()).into();
        self.player_starting_equipment = equipment;
    }
}

impl GameState {
    pub fn player_has_single_hand_pair(&self, selected: u8) -> bool {
        if self.rules().uses_weapon_sets() && selected < 4 {
            let first = selected & !1;
            self.player_weapon_slot_owner(first) == first
                && self.player_weapon_slot_owner(first + 1) == first + 1
        } else {
            self.rules().uses_legacy_weapon_hands() && selected < 2
        }
    }
    pub fn player_active_weapon_set(&self) -> u8 {
        self.active_weapon_set
    }

    /// An instance stays in one real slot; the other hand can refer to a two-hand grip.
    pub fn player_weapon_slot_owner(&self, slot: u8) -> u8 {
        if !self.rules().uses_weapon_sets() || slot >= 4 {
            return slot;
        }
        let raw = |index: u8| {
            self.rules()
                .player_weapon_slots
                .get(index as usize)
                .and_then(|id| self.player_equipment().equipped(id))
        };
        if raw(slot).is_some() {
            return slot;
        }
        let other = slot ^ 1;
        if raw(other)
            .and_then(|id| self.player_inventory().get(id))
            .and_then(|entry| self.rules().weapons.get(entry.item()))
            .is_some_and(|weapon| self.rules().weapon_hands(weapon) == WeaponHands::Two)
        {
            other
        } else {
            slot
        }
    }

    pub fn player_weapon_configuration_slots(&self, selected: u8) -> Vec<u8> {
        if self.rules().uses_weapon_sets() && selected < 4 {
            let first = selected & !1;
            (first..first + 2)
                .filter(|&slot| self.player_weapon_slot_owner(slot) == slot)
                .collect()
        } else if self.rules().uses_legacy_weapon_hands() && selected < 2 {
            vec![0, 1]
        } else {
            vec![selected]
        }
    }

    pub(super) fn equipment_slot_is_active(&self, slot: &crate::entity::EquipmentSlotId) -> bool {
        !self.rules().uses_weapon_sets()
            || self
                .rules()
                .player_weapon_slots
                .iter()
                .position(|id| id == slot)
                .is_none_or(|index| index / 2 == usize::from(self.active_weapon_set))
    }

    /// Both one-hand slots belong to the current configuration. A two-hand
    /// weapon in reserve is never considered, nor are legacy channel slots.
    pub fn player_weapon_slot_for_delivery(
        &self,
        selected: u8,
        delivery: AttackDelivery,
    ) -> Option<u8> {
        let matches = |slot| {
            self.equipped_player_weapon(slot)
                .is_some_and(|weapon| weapon.attack().delivery() == delivery)
        };
        if matches(selected) {
            return Some(selected);
        }
        let first = if self.rules().uses_weapon_sets() && selected < 4 {
            selected & !1
        } else if self.rules().uses_legacy_weapon_hands() && selected < 2 {
            0
        } else {
            return None;
        };
        (first..first + 2).find(|&slot| matches(slot))
    }

    /// Translate an ordinary attack intent into one explicit command slot.
    /// This query neither spends resources nor weakens any range/LOS checks.
    pub fn player_default_attack_slot_from(
        &self,
        selected: u8,
        origin: crate::world::GridPos,
        target: crate::world::GridPos,
    ) -> u8 {
        if !self.player_has_single_hand_pair(selected) {
            return selected;
        }
        let melee = self.player_weapon_slot_for_delivery(selected, AttackDelivery::Melee);
        let ranged = self.player_weapon_slot_for_delivery(selected, AttackDelivery::Ranged);
        match (melee, ranged) {
            (Some(melee), Some(ranged)) => {
                if origin.x.abs_diff(target.x).max(origin.y.abs_diff(target.y)) <= 1 {
                    melee
                } else {
                    ranged
                }
            }
            _ if self.rules().uses_weapon_sets() => melee.or(ranged).unwrap_or(selected),
            _ => selected,
        }
    }

    pub fn player_default_attack_slot(&self, selected: u8, target: crate::world::GridPos) -> u8 {
        self.player_position().map_or(selected, |origin| {
            self.player_default_attack_slot_from(selected, origin, target)
        })
    }

    pub fn player_weapon_slot_for_technique(
        &self,
        selected: u8,
        technique: &crate::skills::TechniqueId,
    ) -> u8 {
        let required = crate::skills::agent::from_technique(technique)
            .and_then(|s| s.required_weapon_delivery())
            .or_else(|| {
                self.rules()
                    .skills
                    .technique(technique)
                    .and_then(|s| s.action())
                    .and_then(|a| a.required_weapon_delivery())
            });
        required
            .and_then(|delivery| self.player_weapon_slot_for_delivery(selected, delivery))
            .unwrap_or(selected)
    }
}

/// Version 15 and earlier omit the new byte at the GameState boundary.
pub(crate) mod set_codec {
    use serde::{Deserialize, Serialize};
    thread_local! { static LEGACY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    pub(crate) fn legacy<T>(operation: impl FnOnce() -> T) -> T {
        struct Reset(bool);
        impl Drop for Reset {
            fn drop(&mut self) {
                LEGACY.set(self.0);
            }
        }
        let _reset = Reset(LEGACY.replace(true));
        operation()
    }
    pub fn serialize<S: serde::Serializer>(value: &u8, s: S) -> Result<S::Ok, S::Error> {
        if LEGACY.get() && !s.is_human_readable() {
            ().serialize(s)
        } else {
            value.serialize(s)
        }
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
        if LEGACY.get() && !d.is_human_readable() {
            <()>::deserialize(d)?;
            Ok(0)
        } else {
            let value = u8::deserialize(d)?;
            if value > 1 {
                Err(serde::de::Error::custom("Invalid weapon set"))
            } else {
                Ok(value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{AttackProfile, DamageType};
    use crate::game::{CommandOutcome, CommandRejection, GameCommand, GameState, WorldState};
    use crate::item::ItemCatalog;
    use crate::loot::{EquipmentBaseDefinition, EquipmentLootCatalog};
    use crate::weapon::{WeaponCatalog, WeaponId};
    use crate::world::{GridPos, Map};

    fn fixture() -> GameState {
        let models = [
            ("knife", "knives", AttackDelivery::Melee),
            ("pistol", "pistols", AttackDelivery::Ranged),
            ("rifle", "rifles", AttackDelivery::Ranged),
            ("axe", "axes", AttackDelivery::Melee),
            ("sword", "swords", AttackDelivery::Melee),
            ("smg", "submachine_guns", AttackDelivery::Ranged),
        ];
        let mut weapons = WeaponCatalog::default();
        for (name, _, delivery) in models {
            weapons
                .register(
                    WeaponDefinition::new(
                        format!("test:{name}").parse().unwrap(),
                        "name".into(),
                        "description".into(),
                        AttackProfile::melee(DamageType::Kinetic, 3).with_delivery(delivery),
                    )
                    .unwrap(),
                )
                .unwrap();
        }
        let mut catalog = EquipmentLootCatalog::default();
        for (name, family, _) in models {
            let base: EquipmentBaseDefinition = json5::from_str(&format!("{{item:'test:{name}', family:'core:{family}', tier:1, grammar:'ms', form:'humanoid_equipment', sources:['humanoid_equipped'], stats:[{{affix:'affix:accuracy',weight:1}}]}}" )).unwrap();
            catalog
                .register(base, &ItemCatalog::default(), &weapons)
                .unwrap();
        }
        let ids: Vec<WeaponId> = models
            .iter()
            .map(|(name, _, _)| format!("test:{name}").parse().unwrap())
            .collect();
        let mut rules = GameRules {
            weapons,
            monster_equipment_loot: Some(catalog),
            player_inventory_capacity: 12,
            player_starting_weapons: ids.clone(),
            player_weapon_slots: LEGACY_WEAPON_SLOTS.map(|id| id.parse().unwrap()).into(),
            player_starting_equipment: ids.iter().take(3).cloned().map(Some).collect(),
            ..GameRules::default()
        };
        rules.enable_weapon_hands();
        GameState::new_with_rules(
            Map::from_ascii("#####\n#...#\n#####").unwrap(),
            GridPos::new(1, 1),
            42,
            rules,
        )
        .unwrap()
    }
    fn equip(game: &mut GameState, name: &str, slot: u8) -> CommandOutcome {
        let item = game
            .player_inventory()
            .iter()
            .find(|entry| entry.item().as_str() == format!("test:{name}"))
            .unwrap()
            .instance();
        game.process_player_command(GameCommand::EquipWeapon { slot, item })
    }
    fn sets_fixture() -> GameState {
        let legacy = fixture();
        let mut rules = legacy.rules().clone();
        rules.enable_weapon_sets();
        GameState::new_with_rules(
            legacy.map().clone(),
            legacy.player_position().unwrap(),
            42,
            rules,
        )
        .unwrap()
    }
    #[test]
    fn weapon_sets_can_hold_four_distinct_one_hand_instances() {
        let base = sets_fixture();
        let mut rules = base.rules().clone();
        rules
            .player_starting_weapons
            .push("test:knife".parse().unwrap());
        rules.player_starting_equipment = ["test:knife", "test:pistol", "test:smg", "test:knife"]
            .map(|id| Some(id.parse().unwrap()))
            .into();
        let game = GameState::new_with_rules(
            base.map().clone(),
            base.player_position().unwrap(),
            42,
            rules,
        )
        .unwrap();
        let slots = (0..4)
            .map(|slot| game.equipped_player_weapon_item(slot).unwrap())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(slots.len(), 4);
        assert_eq!(game.player_equipment().iter().count(), 4);
        assert_eq!(game.player_weapon_configuration_slots(2), vec![2, 3]);
    }
    #[test]
    fn weapon_sets_keep_two_free_grips_and_never_duplicate_or_destroy_instances() {
        let mut game = sets_fixture();
        let owned = game.player_inventory().clone();
        assert_eq!(equip(&mut game, "axe", 1), CommandOutcome::Applied);
        assert_eq!(
            game.equipped_player_weapon_item(0),
            game.equipped_player_weapon_item(1)
        );
        assert_eq!(
            game.equipped_player_weapon(2).unwrap().id().as_str(),
            "test:rifle"
        );
        assert_eq!(
            game.equipped_player_weapon_item(2),
            game.equipped_player_weapon_item(3)
        );
        assert_eq!(game.player_equipment().iter().count(), 2);
        assert_eq!(equip(&mut game, "knife", 1), CommandOutcome::Applied);
        assert!(game.equipped_player_weapon_item(0).is_none());
        assert_eq!(
            game.equipped_player_weapon(1).unwrap().id().as_str(),
            "test:knife"
        );
        assert_eq!(equip(&mut game, "pistol", 3), CommandOutcome::Applied);
        assert!(game.equipped_player_weapon_item(2).is_none());
        assert_eq!(equip(&mut game, "knife", 2), CommandOutcome::Applied);
        assert!(game.equipped_player_weapon_item(1).is_none());
        assert_eq!(game.player_inventory(), &owned);
        assert_eq!(
            game.player_weapon_slot_for_delivery(3, AttackDelivery::Melee),
            Some(2)
        );
        assert_eq!(game.player_default_attack_slot(3, GridPos::new(2, 1)), 2);
        assert_eq!(game.player_default_attack_slot(2, GridPos::new(3, 1)), 3);
        assert_eq!(
            game.player_weapon_slot_for_delivery(0, AttackDelivery::Ranged),
            None
        );
    }
    #[test]
    fn weapon_sets_switch_without_time_rng_or_refill_and_restore_the_active_bonus_sources() {
        let base = sets_fixture();
        let bonus =
            crate::entity::MagicItemModifiers::rpg_bonuses([0; 5], 0, 0, 10, 15, 0).unwrap();
        let mut rules = base.rules().clone();
        rules
            .equipment_agent_skills
            .insert("test:rifle".parse().unwrap(), [("A01".into(), 2)].into());
        let mut game = GameState::new_with_rules(
            base.map().clone(),
            base.player_position().unwrap(),
            42,
            rules,
        )
        .unwrap()
        .with_starting_magic_equipment([("test:rifle".parse().unwrap(), bonus)])
        .unwrap();
        let item = game
            .player_inventory()
            .iter()
            .find(|entry| entry.magic_modifiers().is_some())
            .unwrap()
            .instance();
        assert_eq!(
            game.process_player_command(GameCommand::EquipWeapon { slot: 2, item }),
            CommandOutcome::Applied
        );
        game.refresh_equipment_resources();
        let base_hp = game
            .actors()
            .get(game.player_id())
            .unwrap()
            .maximum_integrity();
        let turn = game.turn();
        let energy = game.player_energy().available();
        game.drain_events();
        let before = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        assert_eq!(game.player_agent_equipment_skill_rank("A01"), 0);
        assert_eq!(
            game.process_player_command(GameCommand::SelectWeaponSet { set: 1 }),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(
            game.actors()
                .get(game.player_id())
                .unwrap()
                .maximum_integrity(),
            base_hp + 10
        );
        assert_eq!(
            game.actors().get(game.player_id()).unwrap().integrity(),
            base_hp
        );
        assert_eq!(game.player_energy().available(), energy);
        assert_eq!(game.player_agent_equipment_skill_rank("A01"), 2);
        game.drain_events();
        let world = WorldState::single(game);
        let saved = world.recovery_snapshot_bytes().unwrap();
        assert!(saved.starts_with(b"RLWS\x10"));
        let mut restored =
            WorldState::from_recovery_snapshot_bytes(&saved, world.rules().clone()).unwrap();
        assert_eq!(restored.recovery_snapshot_bytes().unwrap(), saved);
        assert_eq!(restored.player_active_weapon_set(), 1);
        assert_eq!(
            restored.process_player_command(GameCommand::SelectWeaponSet { set: 0 }),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(restored.player_agent_equipment_skill_rank("A01"), 0);
        assert_eq!(restored.turn(), turn);
        restored.drain_events();
        assert_eq!(
            bincode::serialize(&restored.snapshot().unwrap()).unwrap(),
            before
        );
    }
    #[test]
    fn weapon_sets_reject_invalid_sets_and_reserve_attacks_atomically() {
        let mut game = sets_fixture();
        game.drain_events();
        let before = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        for command in [
            GameCommand::SelectWeaponSet { set: 2 },
            GameCommand::SelectWeaponSet { set: 255 },
            GameCommand::AttackAt {
                slot: 2,
                target: GridPos::new(3, 1),
            },
        ] {
            assert!(matches!(
                game.process_player_command(command),
                CommandOutcome::Rejected(_)
            ));
            assert_eq!(
                bincode::serialize(&game.snapshot().unwrap()).unwrap(),
                before
            );
            assert!(game.events().is_empty());
        }
    }
    #[test]
    fn mixed_hands_choose_contact_and_ranged_weapons_without_modifying_the_game() {
        let mut game = fixture();
        game.drain_events();
        let before = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        for selected in [0, 1] {
            assert_eq!(
                game.player_default_attack_slot(selected, GridPos::new(2, 1)),
                0
            );
            assert_eq!(
                game.player_default_attack_slot(selected, GridPos::new(3, 1)),
                1
            );
            assert_eq!(
                game.player_weapon_slot_for_delivery(selected, AttackDelivery::Melee),
                Some(0)
            );
            assert_eq!(
                game.player_weapon_slot_for_delivery(selected, AttackDelivery::Ranged),
                Some(1)
            );
            assert_eq!(
                game.player_weapon_slot_for_technique(
                    selected,
                    &crate::skills::agent::technique("B01")
                ),
                0
            );
            assert_eq!(
                game.player_weapon_slot_for_technique(
                    selected,
                    &crate::skills::agent::technique("DI01")
                ),
                1
            );
        }
        assert_eq!(
            bincode::serialize(&game.snapshot().unwrap()).unwrap(),
            before
        );
        assert!(game.events().is_empty());
        assert_eq!(equip(&mut game, "pistol", 0), CommandOutcome::Applied);
        assert_eq!(equip(&mut game, "knife", 1), CommandOutcome::Applied);
        assert_eq!(game.player_default_attack_slot(0, GridPos::new(2, 1)), 1);
        assert_eq!(game.player_default_attack_slot(1, GridPos::new(3, 1)), 0);
    }

    #[test]
    fn mixed_hands_do_not_use_the_reserve_or_change_legacy_channels() {
        let game = fixture();
        assert_eq!(game.player_default_attack_slot(2, GridPos::new(2, 1)), 2);
        assert_eq!(
            game.player_weapon_slot_for_delivery(2, AttackDelivery::Melee),
            None
        );
        let mut rules = game.rules().clone();
        rules.player_weapon_slots = LEGACY_WEAPON_SLOTS.map(|id| id.parse().unwrap()).into();
        let legacy = GameState::new_with_rules(
            game.map().clone(),
            game.player_position().unwrap(),
            42,
            rules,
        )
        .unwrap();
        assert_eq!(legacy.player_default_attack_slot(0, GridPos::new(3, 1)), 0);
        assert_eq!(
            legacy.player_weapon_slot_for_delivery(0, AttackDelivery::Ranged),
            None
        );
    }
    #[test]
    fn weapon_hands_preserve_both_configurations_and_displaced_items() {
        let mut game = fixture();
        assert_eq!(
            game.equipped_player_weapon(0).unwrap().id().as_str(),
            "test:knife"
        );
        assert_eq!(
            game.equipped_player_weapon(1).unwrap().id().as_str(),
            "test:pistol"
        );
        assert_eq!(
            game.equipped_player_weapon(2).unwrap().id().as_str(),
            "test:rifle"
        );
        assert_eq!(equip(&mut game, "axe", 2), CommandOutcome::Applied);
        assert_eq!(
            game.equipped_player_weapon(0).unwrap().id().as_str(),
            "test:knife"
        );
        assert_eq!(
            game.equipped_player_weapon(1).unwrap().id().as_str(),
            "test:pistol"
        );
        assert_eq!(equip(&mut game, "smg", 0), CommandOutcome::Applied);
        assert_eq!(
            game.equipped_player_weapon(2).unwrap().id().as_str(),
            "test:axe"
        );
        assert_eq!(game.player_inventory().len(), 6);
        assert_eq!(equip(&mut game, "pistol", 0), CommandOutcome::Applied);
        assert!(game.equipped_player_weapon(1).is_none());
        assert_eq!(game.player_inventory().len(), 6);
        game.drain_events();
        let world = WorldState::single(game);
        let bytes = world.recovery_snapshot_bytes().unwrap();
        let restored =
            WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone()).unwrap();
        assert_eq!(restored.recovery_snapshot_bytes().unwrap(), bytes);
    }
    #[test]
    fn weapon_hands_invalid_slots_are_atomic() {
        let mut game = fixture();
        game.drain_events();
        for (name, slot) in [
            ("knife", 2),
            ("pistol", 2),
            ("smg", 2),
            ("axe", 0),
            ("sword", 1),
            ("rifle", 1),
        ] {
            let before = bincode::serialize(&game.snapshot().unwrap()).unwrap();
            assert!(matches!(
                equip(&mut game, name, slot),
                CommandOutcome::Rejected(CommandRejection::WeaponIncompatibleWithHands { .. })
            ));
            assert_eq!(
                bincode::serialize(&game.snapshot().unwrap()).unwrap(),
                before
            );
            assert!(game.events().is_empty());
        }
    }
    #[test]
    fn weapon_hands_follow_families_including_future_pistols_and_smg() {
        for family in ["core:axes", "core:swords"] {
            assert_eq!(
                WeaponHands::from_family(Some(family), AttackDelivery::Melee),
                WeaponHands::Two
            );
        }
        for family in ["core:knives", "core:spears", "core:war_hammers"] {
            assert_eq!(
                WeaponHands::from_family(Some(family), AttackDelivery::Melee),
                WeaponHands::One
            );
        }
        for family in ["core:pistols", "core:submachine_guns"] {
            assert_eq!(
                WeaponHands::from_family(Some(family), AttackDelivery::Ranged),
                WeaponHands::One
            );
        }
        assert_eq!(
            WeaponHands::from_family(Some("core:rifles"), AttackDelivery::Ranged),
            WeaponHands::Two
        );
    }
}
