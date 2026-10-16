from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
def edit(path, pairs):
    p=ROOT/path
    s=p.read_text(encoding='utf-8')
    for a,b in pairs:
        assert a in s, (path,a[:150])
        s=s.replace(a,b,1)
    p.write_text(s,encoding='utf-8')

edit('src/entity/mod.rs', [('mod equipment;','mod equipment;\nmod equipment_palette;'), ('pub use equipment::{','pub use equipment_palette::EquipmentPalette;\npub use equipment::{')])
edit('src/entity/inventory.rs',[
 ('    magic_modifiers: Option<MagicItemModifiers>,\n}', '    magic_modifiers: Option<MagicItemModifiers>,\n    #[serde(default)]\n    palette: Option<super::EquipmentPalette>,\n}'),
 ('        entry.finish()', '        if self.palette() != super::EquipmentPalette::Original {\n            entry.field("palette", &self.palette());\n        }\n        entry.finish()'),
 ('impl InventoryEntry {', '''impl InventoryEntry {
    pub fn palette(&self) -> super::EquipmentPalette {
        self.palette.unwrap_or_default()
    }

    pub(crate) const fn color_variant(&self) -> Option<super::EquipmentPalette> {
        self.palette
    }
'''),
 ('    pub fn iter(&self) -> impl Iterator<Item = &InventoryEntry> {', '''    pub fn set_palette(&mut self, instance: ItemInstanceId, palette: super::EquipmentPalette) -> Result<(), InventoryError> {
        self.retain_palette(instance, Some(palette))
    }

    pub(crate) fn retain_palette(&mut self, instance: ItemInstanceId, palette: Option<super::EquipmentPalette>) -> Result<(), InventoryError> {
        let entry = self.entries.iter_mut().find(|entry| entry.instance == instance)
            .ok_or(InventoryError::UnknownInstance(instance))?;
        entry.palette = palette;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &InventoryEntry> {'''),
 ('                magic_modifiers: None,\n            });', '                magic_modifiers: None,\n                palette: None,\n            });'),
 ('            magic_modifiers: Some(modifiers),\n        });', '            magic_modifiers: Some(modifiers),\n            palette: None,\n        });'),
])
edit('src/entity/ground_item.rs',[
 ('    magic_modifiers: Option<super::MagicItemModifiers>,\n}', '    magic_modifiers: Option<super::MagicItemModifiers>,\n    #[serde(default)]\n    palette: Option<super::EquipmentPalette>,\n}'),
 ('        item.finish()', '        if self.palette() != super::EquipmentPalette::Original {\n            item.field("palette", &self.palette());\n        }\n        item.finish()'),
 ('impl GroundItem {', '''impl GroundItem {
    pub fn palette(&self) -> super::EquipmentPalette {
        self.palette.unwrap_or_default()
    }

    pub(crate) const fn color_variant(&self) -> Option<super::EquipmentPalette> {
        self.palette
    }
'''),
 ('                magic_modifiers,\n            },','                magic_modifiers,\n                palette: None,\n            },'),
 ('    pub fn item_at(&self, position: GridPos)', '''    pub fn set_palette(&mut self, id: GroundItemId, palette: super::EquipmentPalette) {
        self.retain_palette(id, Some(palette));
    }

    pub(crate) fn retain_palette(&mut self, id: GroundItemId, palette: Option<super::EquipmentPalette>) {
        if let Some(item) = self.items.get_mut(&id) { item.palette = palette; }
    }

    pub fn item_at(&self, position: GridPos)'''),
 ('            magic_modifiers: item.magic_modifiers.clone(),','            magic_modifiers: item.magic_modifiers.clone(),\n            palette: item.palette,'),
])
edit('src/game/rules.rs',[
 ('    pub weighted_equipment_bonuses: bool,','    pub weighted_equipment_bonuses: bool,\n    pub equipment_color_variants: bool,'),
 ('        if self.weighted_equipment_bonuses {','        if self.equipment_color_variants { rules.field("equipment_color_variants", &true); }\n        if self.weighted_equipment_bonuses {'),
 ('            weighted_equipment_bonuses: false,','            weighted_equipment_bonuses: false,\n            equipment_color_variants: false,'),
])
edit('src/game/game_state.rs',[
 ('    pub(super) rng: GameRng,', '    pub(super) rng: GameRng,\n    cosmetic_rng: Option<GameRng>,'),
 ('    rng: GameRng,\n    player_visibility: VisibilityState,','    rng: GameRng,\n    #[serde(default)]\n    cosmetic_rng: Option<GameRng>,\n    player_visibility: VisibilityState,'),
 ('            rng: self.rng,','            rng: self.rng,\n            cosmetic_rng: self.cosmetic_rng,'),
 ('            rng: snapshot.rng,','            rng: snapshot.rng,\n            cosmetic_rng: snapshot.cosmetic_rng,'),
 ('        state.finish()','        if let Some(rng) = self.cosmetic_rng { state.field("cosmetic_rng", &rng); }\n        state.finish()'),
 ('        Ok(Self {\n            map,','        let mut state = Self {\n            map,'),
 ('            rng: GameRng::from_seed(rng_state),','            rng: GameRng::from_seed(rng_state),\n            cosmetic_rng: rules.equipment_color_variants.then(|| GameRng::from_seed(rng_state ^ 0x5041_4c45_5454_4553)),'),
 ('            sustained_fire: None,\n        })','            sustained_fire: None,\n        };\n        state.assign_equipment_palettes();\n        Ok(state)'),
 ('        Ok(ground_item)\n    }\n\n    pub fn process_player_command','        self.assign_equipment_palettes();\n        Ok(ground_item)\n    }\n\n    pub fn process_player_command'),
 ('        outcome\n    }\n\n    fn player_command_action_kind','        if matches!(outcome, CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime) {\n            self.assign_equipment_palettes();\n        }\n        outcome\n    }\n\n    fn player_command_action_kind'),
 ('        let witnesses = self.property_take_witnesses(position, stack.owner());','        for &instance in &affected_instances {\n            next_inventory.retain_palette(instance, stack.color_variant()).map_err(PickUpError::Inventory)?;\n        }\n        let witnesses = self.property_take_witnesses(position, stack.owner());'),
 ('        if let Some(bonus) = entry.magic_modifiers().filter(|bonus| {','        next_ground_items.retain_palette(ground_item, entry.color_variant());\n        if let Some(bonus) = entry.magic_modifiers().filter(|bonus| {'),
 ('            PrimaryAttribute::ALL\n                .iter()', '            self.rules.equipment_color_variants || PrimaryAttribute::ALL\n                .iter()'),
])
edit('src/game/mod.rs',[('mod equipment_upgrade;','mod equipment_upgrade;\nmod equipment_colors;')])
edit('src/ascii_app.rs',[
 ('const CURRENT_GENERATION_VERSION: u8 = WEAPON_HANDS_GENERATION_VERSION;', 'const EQUIPMENT_COLORS_GENERATION_VERSION: u8 = WEAPON_HANDS_GENERATION_VERSION + 1;\nconst CURRENT_GENERATION_VERSION: u8 = EQUIPMENT_COLORS_GENERATION_VERSION;'),
 ('    rules.weighted_equipment_bonuses = version >= BONUS_RARITY_GENERATION_VERSION;', '    rules.weighted_equipment_bonuses = version >= BONUS_RARITY_GENERATION_VERSION;\n    rules.equipment_color_variants = version >= EQUIPMENT_COLORS_GENERATION_VERSION;'),
])
edit('src/suspension.rs',[('pub const MAX_GENERATION_VERSION: u8 = 144;','pub const MAX_GENERATION_VERSION: u8 = 145;')])
edit('src/game/expedition.rs',[
 ('b"RLWS\\x0b"','b"RLWS\\x0c"'),
 ('pub struct TradeResaleView {\n', 'pub struct TradeResaleView {\n    pub palette: crate::entity::EquipmentPalette,\n'),
 ('pub struct TradeSellView {\n', 'pub struct TradeSellView {\n    pub palette: crate::entity::EquipmentPalette,\n'),
 ('#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]\nstruct MerchantResaleState', '#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]\nstruct MerchantResaleState'),
 ('    maximum_stack: u16,\n    magic_modifiers: Option<MagicItemModifiers>,','    maximum_stack: u16,\n    magic_modifiers: Option<MagicItemModifiers>,\n    #[serde(default)]\n    palette: Option<crate::entity::EquipmentPalette>,'),
 ('                .map(|listing| TradeResaleView {','                .map(|listing| TradeResaleView {\n                    palette: listing.palette.unwrap_or_default(),'),
 ('                        .map(|(price, _, _)| TradeSellView {','                        .map(|(price, _, _)| TradeSellView {\n                            palette: entry.palette(),'),
 ('            magic_modifiers,\n        });', '            magic_modifiers,\n            palette: entry.color_variant(),\n        });'),
 ('                .map(|_| ())\n        } else {\n            next_inventory\n                .add(entry.item.clone(), 1, entry.maximum_stack)\n                .map(|_| ())', '                .map(|instance| vec![instance])\n        } else {\n            next_inventory\n                .add(entry.item.clone(), 1, entry.maximum_stack)'),
 ('        if added.is_err() {\n            return reject(CommandRejection::InventoryCannotFitItem);\n        }\n        let mut next_merchant', '        let Ok(added) = added else { return reject(CommandRejection::InventoryCannotFitItem); };\n        for instance in added {\n            next_inventory.retain_palette(instance, entry.palette).expect("newly added resale instance");\n        }\n        let mut next_merchant'),
])

with (ROOT/'src/game/expedition.rs').open('a',encoding='utf-8') as f:
    f.write('''
impl Debug for MerchantResaleState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("MerchantResaleState");
        state.field("listing", &self.listing).field("item", &self.item).field("price", &self.price)
            .field("maximum_stack", &self.maximum_stack).field("magic_modifiers", &self.magic_modifiers);
        if self.palette.unwrap_or_default() != crate::entity::EquipmentPalette::Original { state.field("palette", &self.palette); }
        state.finish()
    }
}
''')
