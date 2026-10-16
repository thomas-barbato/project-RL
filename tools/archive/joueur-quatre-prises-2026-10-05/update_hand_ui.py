from pathlib import Path
root=Path(__file__).resolve().parents[2]
p=root/'src/ascii_app.rs';s=p.read_text(encoding='utf-8')
s=s.replace('weapon_slot_label(index),','self.hand_slot_label(index),')
s=s.replace('if index == usize::from(self.active_weapon_slot) {\n                    " · en main"','if self.weapon_configuration_active(index) {\n                    " · actif" if self.game.rules().uses_weapon_hands() else { " · en main" }')
s=s.replace('selected_is_weapon,\n                )','selected_is_weapon && selected.is_some_and(|entry| self.game.rules().weapons.get(entry.item()).is_some_and(|weapon| self.game.rules().weapon_fits_slot(weapon, index))),\n                )')
s=s.replace('if self.game.rules().weapons.get(entry.item()).is_some() {\n                    "ARME".to_owned()', 'if let Some(weapon) = self.game.rules().weapons.get(entry.item()) {\n                    self.weapon_category_label(weapon)')
s=s.replace('format!("L\'emplacement d\'arme {} est vide.", slot + 1)', 'format!("{} : emplacement vide.", self.hand_slot_label(usize::from(slot)))')
s=s.replace('format!("ACTIVE [{}] {name}", slot + 1)', 'format!("ACTIVE [{}] {name} · {}", slot + 1, if self.game.rules().uses_weapon_hands() && slot < 2 { "Configuration à une main" } else { "Configuration à deux mains" })')
s=s.replace('format!("Emplacement d\'arme {} maintenant actif.", slot + 1)', 'format!("{} : configuration active.", self.hand_slot_label(usize::from(slot)))')
s=s.replace('"{item_name} équipé et actif sur l\'emplacement d\'arme {}.",\n                    slot + 1', '"{item_name} équipé · {} : configuration active.",\n                    self.hand_slot_label(usize::from(slot))')
for owner in ['fresh','next_run']:
 old=f'{owner}.active_weapon_slot = {owner}\n            '+('    ' if owner=='next_run' else '')+'.game'
 # Locate the exact existing chain, preserving surrounding unrelated edits.
 import re
 s=re.sub(rf'{owner}\.active_weapon_slot = {owner}\s*\.game\s*\.rules\(\)\s*\.player_starting_equipment\s*\.iter\(\)\s*\.position\(Option::is_some\)\s*\.unwrap_or\(0\) as u8;',f'{owner}.active_weapon_slot = {owner}.preferred_starting_weapon_slot();',s)
# Derive the startup slot before moving game/rules into the app.
needle='let terminal = TerminalView::new(sector.decor, game.map(), game.player_visibility());'
s=s.replace(needle,needle+'\n        let active_weapon_slot = game.rules().player_starting_weapons.iter().find_map(|id| {\n            (0..game.rules().player_weapon_slots.len()).find(|&slot| game.equipped_player_weapon(slot as u8).is_some_and(|w| w.id() == id))\n        }).unwrap_or(0) as u8;')
s=s.replace('            active_weapon_slot: 0,','            active_weapon_slot,',1)
p.write_text(s,encoding='utf-8',newline='\n')
p=root/'src/effects_lab.rs';s=p.read_text(encoding='utf-8');needle='        let mut app = Self::from_seed_base(\n            LAB_SEED,';s=s.replace(needle,'        rules.enable_weapon_hands();\n'+needle);p.write_text(s,encoding='utf-8',newline='\n')
