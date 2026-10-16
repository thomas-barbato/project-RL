"""Read-only baseline and contact sheets; never modifies active art/code."""
from pathlib import Path
import hashlib, json, re
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
POSES = ('unarmed','side','two-handed','raised','melee-two-handed','dual-melee','dual-raised','dual-mixed')
manifest = {}
for directory in ('src','assets/sprites/cyberpunk','content/core','tools/art'):
    for path in sorted((ROOT/directory).rglob('*')):
        if path.is_file() and '__pycache__' not in path.parts:
            manifest[path.relative_to(ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
(OUT/'baseline.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
text = (ROOT/'src/game_art_palette.rs').read_text(encoding='utf-8').split('struct ItemSources')[0]
for name in ('painted','color','held_material','face_pixel','visor_pixel'):
    text = text.replace(f'fn {name}(',f'pub(super) fn {name}(')
(OUT/'current.rs').write_text(text,encoding='utf-8')
models = json.loads((ROOT/'assets/sprites/cyberpunk/player-poses.json').read_text())['torso_models']
labels = {}
for path in (ROOT/'content/core/locales').glob('fr*.json5'):
    labels.update(re.findall(r'"(?:equipment|item)\.([^\"]+)\.name"\s*:\s*"([^\"]+)"', path.read_text(encoding='utf-8')))
names = ['BRÈCHE','VIGIE','CREUSET'] + [labels.get(model,model) for model in models]
(OUT/'modeles.json').write_text(json.dumps([{'number':n+1,'index':n,'name':name,'id':(['breche','vigie','creuset']+models)[n]} for n,name in enumerate(names)],ensure_ascii=False,indent=2),encoding='utf-8')
font = ImageFont.truetype('C:/Windows/Fonts/arial.ttf',15)
for page,start in enumerate((0,6,12),1):
    count=min(6,17-start)
    board=Image.new('RGBA',(1740,90+count*220),(17,23,28,255)); draw=ImageDraw.Draw(board)
    for pose,name in enumerate(POSES): draw.text((180+pose*192,8),name,font=font,fill='white')
    for row,index in enumerate(range(start,start+count)):
        y=55+row*220
        draw.text((8,y+70),f'{index+1:02d} {names[index]}',font=font,fill='white')
        for pose,name in enumerate(POSES):
            atlas=Image.open(ROOT/f'assets/sprites/cyberpunk/player-poses-{name}-48.png').convert('RGBA')
            x=index%16*48; sy=index//16*48
            tile=atlas.crop((x,sy,x+48,sy+48)).resize((192,192),Image.Resampling.NEAREST)
            board.alpha_composite(tile,(180+pose*192,y))
    board.save(OUT/f'actuel-page-{page}.png')
print(f'Baseline {len(manifest)} files; 17 numbered models and three contact sheets.')
