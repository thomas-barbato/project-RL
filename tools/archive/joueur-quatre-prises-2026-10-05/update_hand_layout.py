from pathlib import Path
root=Path(__file__).resolve().parents[2]
p=root/'src/ascii_app.rs';s=p.read_text(encoding='utf-8')
s=s.replace('inventory_terminal::EquipmentLayout::new(&layout, w, h);','inventory_terminal::EquipmentLayout::new(&layout, w, h).for_hands(app.game.rules().uses_weapon_hands());')
s=s.replace('inventory_terminal::EquipmentLayout::new(&layout, width, height).hit(pointer)','inventory_terminal::EquipmentLayout::new(&layout, width, height).for_hands(self.game.rules().uses_weapon_hands()).hit(pointer)')
s=s.replace('inventory_terminal::EquipmentLayout::new(&layout, self.ui_width(), self.ui_height());','inventory_terminal::EquipmentLayout::new(&layout, self.ui_width(), self.ui_height()).for_hands(self.game.rules().uses_weapon_hands());')
# Both single items show active while the pair is selected.
old='''let active = self
                .game
                .rules()
                .player_weapon_slots
                .get(usize::from(self.active_weapon_slot))
                .is_some_and(|slot| {
                    self.game.player_equipment().slot_of(entry.instance()) == Some(slot)
                });'''
new='''let active = self.game.rules().player_weapon_slots.iter().enumerate().any(|(index, slot)| {
                self.weapon_configuration_active(index) && self.game.player_equipment().slot_of(entry.instance()) == Some(slot)
            });'''
assert old in s;s=s.replace(old,new)
p.write_text(s,encoding='utf-8',newline='\n')
p=root/'src/inventory_terminal.rs';s=p.read_text(encoding='utf-8')
s=s.replace('let equipment = EquipmentLayout::new(layout, width, height);','let equipment = EquipmentLayout::new(layout, width, height).for_hands(self.game.rules().uses_weapon_hands());')
s=s.replace('let equipment = EquipmentLayout::new(&layout, 1280.0, 800.0);','let equipment = EquipmentLayout::new(&layout, 1280.0, 800.0).for_hands(app.game.rules().uses_weapon_hands());')
p.write_text(s,encoding='utf-8',newline='\n')
