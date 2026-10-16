from pathlib import Path
import re
root=Path(__file__).resolve().parents[2]
for name in ['src/effects_lab.rs','src/arsenal_app.rs']:
 p=root/name
 # Keep a precise baseline for the newly touched arsenal helper.
 backup=root/'artifacts/joueur-quatre-prises-2026-10-05/avant'/name
 if not backup.exists(): backup.parent.mkdir(parents=True,exist_ok=True);backup.write_bytes(p.read_bytes())
 s=p.read_text(encoding='utf-8')
 s=re.sub(r'(self|app)\.execute_command\(GameCommand::EquipWeapon\s*\{\s*slot:\s*([012]),\s*item(?::\s*(\w+))?\s*,?\s*\}\)',lambda m:f'{m[1]}.equip_diagnostic_weapon({m[3] or "item"}, {m[2]})',s)
 s=re.sub(r'(self|app)\.execute_command\(GameCommand::(Attack|AttackAt)\s*\{\s*slot:\s*[012],',lambda m:f'{m[1]}.execute_command(GameCommand::{m[2]} {{ slot: {m[1]}.active_weapon_slot,',s)
 # Synthetic reference rules also get a valid starting layout before construction.
 s=s.replace('        let game = GameState::new_with_rules(self.game.map().clone(), LAB_START, LAB_SEED, rules)','        rules.enable_weapon_hands();\n        let game = GameState::new_with_rules(self.game.map().clone(), LAB_START, LAB_SEED, rules)')
 s=s.replace('        self.game = WorldState::single(game);\n        self.actor_glyphs.clear();','        self.game = WorldState::single(game);\n        self.active_weapon_slot = self.preferred_starting_weapon_slot();\n        self.actor_glyphs.clear();')
 # Helpers now own the active slot, rather than overwriting it with a legacy index.
 s=s.replace('        self.active_weapon_slot = 0;','        self.active_weapon_slot = self.preferred_starting_weapon_slot();')
 p.write_text(s,encoding='utf-8',newline='\n')
