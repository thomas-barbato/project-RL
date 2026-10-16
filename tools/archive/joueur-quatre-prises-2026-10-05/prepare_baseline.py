from pathlib import Path
import shutil
root=Path(__file__).resolve().parents[2]
stage=Path(__file__).resolve().parent
baseline=stage/'compat-baseline'
baseline.mkdir(exist_ok=True)
for name in ['Cargo.toml','Cargo.lock','build.rs']:
 shutil.copy2(root/name,baseline/name)
shutil.copytree(root/'src',baseline/'src',dirs_exist_ok=True)
shutil.copytree(root/'assets',baseline/'assets',dirs_exist_ok=True)
for name in ['ascii_app.rs','effects_lab.rs','game_art.rs','game_art_worn.rs','game_art_catalog.rs','combat_visuals_capture.rs','inventory_terminal.rs']:
 source=stage/'avant'/name
 if not source.exists(): source=stage/'avant/src'/name
 shutil.copy2(source,baseline/'src'/name)
for name in ['game/game_state.rs','loot/equipment.rs','suspension.rs','arsenal_app.rs']:
 shutil.copy2(stage/'avant/src'/name,baseline/'src'/name)
for p in (stage/'avant').glob('player-poses-*.png'):
 shutil.copy2(p,baseline/'assets/sprites/cyberpunk'/p.name)
shutil.copy2(stage/'avant/manifest.json',baseline/'assets/sprites/cyberpunk/manifest.json')
p=baseline/'src/game/mod.rs';s=p.read_text(encoding='utf-8');s=s.replace('mod weapon_hands;\n','').replace('pub use weapon_hands::{WeaponHands, WEAPON_HAND_SLOTS};\n','');p.write_text(s,encoding='utf-8')
p=baseline/'src/game_art_player.rs';s=p.read_text(encoding='utf-8');s=s.replace('    hands: Option<[Option<WeaponHold>; 2]>,\n','').replace('weapon, hands, side, opacity','weapon, side, opacity');p.write_text(s,encoding='utf-8')
p=baseline/'src/cyberpunk_capture.rs';s=p.read_text(encoding='utf-8');start=s.index('        if rules.uses_weapon_hands() {');end=s.index('        rules.player_starting_items.push',start);s=s[:start]+'        rules.player_starting_equipment[0] = Some(rifle.clone());\n'+s[end:];p.write_text(s,encoding='utf-8')
