from pathlib import Path
root=Path(__file__).resolve().parents[2]
p=root/'src/effects_lab.rs';s=p.read_text(encoding='utf-8')
# Reference rifle lives in the two-hand slot.
start=s.index('    fn cone_preview_follows_the_instance_not_the_base_model()');end=s.index('\n    #[test]',start)
block=s[start:end].replace('slot: 0, item','slot: 2, item').replace('app.active_weapon_slot = 0','app.active_weapon_slot = 2');s=s[:start]+block+s[end:]
start=s.index('    fn lab_parallax_keyboard_fire_chains_without_movement_or_waiting()');end=s.index('\n    #[test]',start)
block=s[start:end].replace('key("Key1")','key("Key3")');s=s[:start]+block+s[end:]
start=s.index('    fn pointer_target_selection_drives_the_next_direct_ranged_attack()');end=s.index('\n    #[test]',start)
block=s[start:end].replace('slot: 0,','slot: app.active_weapon_slot,');s=s[:start]+block+s[end:]
start=s.index('    fn inventory_groups_preserve_selected_instances_when_equipping_with_keyboard_or_mouse()');end=s.index('\n    #[test]',start)
block=s[start:end].replace('let equip = layout.actions[1];','let equip = layout.actions[2];').replace('Some("Arme secondaire · en main")','Some("Deux mains · actif")').replace('Some("Arme secondaire")','Some("Deux mains")').replace('        assert_eq!(app.inventory_selection, 1);','        assert_eq!(app.inventory_entries()[app.inventory_selection].instance(), weapon);');s=s[:start]+block+s[end:]
p.write_text(s,encoding='utf-8',newline='\n')
