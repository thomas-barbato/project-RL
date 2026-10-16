from pathlib import Path
from PIL import Image, ImageChops
import json, hashlib
root=Path('.')
out=root/'artifacts/joueur-tenues-completes-2026-10-04'
old_inventory=root/'artifacts/combat-visuel-2026-10-04/integration/terminal-inventory-live-equipped/cold-start.png'
new_inventory=out/'terminal-inventory-live-equipped/cold-start.png'
rect=(32,30,1248,770)
diff=ImageChops.difference(Image.open(old_inventory).convert('RGB').crop(rect),Image.open(new_inventory).convert('RGB').crop(rect))
def function(path,name):
 s=path.read_text(encoding='utf-8')
 start=s.index('fn '+name+'(')
 brace=s.index('{',start)
 depth=1
 end=brace+1
 while depth:
  depth+=(s[end]=='{')-(s[end]=='}')
  end+=1
 return s[start:end]
wireframe=function(root/'src/game_art_player.rs','draw_wireframe')
old_wireframe=function(root/'artifacts/joueur-equipe-2026-10-04/avant/game_art_player.rs','draw_wireframe')
data={
 'inventory_rect':rect,'inventory_rgb_identical':diff.getbbox() is None,
 'wireframe_function_unchanged':wireframe==old_wireframe,
 'other_equipment_slots_and_game_stats':'unchanged; only renderer and review screens edited',
 'weapon_categories':{'melee':'shared sword','one_hand_ranged':'shared pistol','two_hand_ranged':'shared rifle'},
 'source_hashes':{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [
 root/'src/inventory_terminal.rs',root/'src/game_art.rs',
 root/'assets/sprites/cyberpunk/items-128.png',root/'assets/sprites/cyberpunk/actors-32.png',
 root/'assets/sprites/cyberpunk/actors-128.png',root/'assets/sprites/cyberpunk/ground-32.png']}
}
(out/'preservation.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
print(json.dumps({k:v for k,v in data.items() if k!='source_hashes'},indent=2))

