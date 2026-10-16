from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
import hashlib, json, re

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
CRAWL = Path(r'C:\Users\User\Desktop\crawl-master\crawl-master\crawl-ref\source')
TILES = CRAWL/'rltiles'
baseline = {}
for folder in ('src', 'assets/sprites/cyberpunk', 'content/core', 'tools/art'):
    for p in (ROOT/folder).rglob('*'):
        if p.is_file() and '__pycache__' not in p.parts:
            baseline[p.relative_to(ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
(OUT/'baseline.json').write_text(json.dumps(baseline, indent=2), encoding='utf-8')

def load(path):
    return Image.open(TILES/path).convert('RGBA')
def composed(paths):
    result=Image.new('RGBA',(32,32))
    for path in paths: result.alpha_composite(load(path))
    return result

paths=['player/base/human_m.png', 'player/legs/pants_black.png',
       'player/boots/short_brown.png', 'player/body/plate_black.png',
       'player/gloves/glove_gray.png', 'player/head/chain.png']
paths=[p for p in paths if (TILES/p).exists()]
unarmed=composed(paths)
weapons=sorted((TILES/'player/hand1').glob('*sword*.png'))[:3]
items=[('Corps',load(paths[0])),('Armure',load('player/body/plate_black.png')),
       ('Casque',load('player/head/chain.png')),('Assemblage',unarmed)]
for p in weapons: items.append(('Arme séparée',Image.alpha_composite(unarmed,Image.open(p).convert('RGBA'))))
for p in ['mon/humanoids/gnoll.png','mon/animals/black_bear.png','mon/animals/bat.png',
          'mon/animals/boulder_beetle.png','mon/animals/adder.png']:
    if (TILES/p).exists(): items.append((Path(p).stem,load(p)))
font=ImageFont.truetype(r'C:\Windows\Fonts\consola.ttf',16)
board=Image.new('RGB',(6*192,2*235),(25,29,34)); draw=ImageDraw.Draw(board)
measure=[]
for i,(name,img) in enumerate(items):
    x,y=(i%6)*192,(i//6)*235
    draw.text((x+8,y+5),name,font=font,fill='#dfe8ec')
    board.paste(img,(x+80,y+32),img)
    zoom=img.resize((160,160),Image.Resampling.NEAREST)
    board.paste(zoom,(x+16,y+70),zoom)
    measure.append({'name':name,'size':img.size,'bounds':img.getbbox(),
                    'colors':len(set(img.getdata()))})
board.save(OUT/'reference-crawl.png')
(OUT/'reference-measures.json').write_text(json.dumps(measure,indent=2),encoding='utf-8')
# Transparent style-only tile strip supplied to ImageGen; no borrowed pixels in our assets.
strip=Image.new('RGBA',(6*256,2*256))
for i,(_,img) in enumerate(items):
    strip.alpha_composite(img.resize((256,256),Image.Resampling.NEAREST),((i%6)*256,(i//6)*256))
strip.save(OUT/'reference-style.png')

catalog=(ROOT/'artifacts/sprites-cyberpunk-crawl-review-2026-10-04/face-catalog.js').read_text(encoding='utf-8')
names=re.findall(r"\['([^']+)',",catalog.split('enemies:')[1])
(OUT/'bestiary.json').write_text(json.dumps(names,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'active_files':len(baseline),'crawl_samples':len(items),'bestiary':len(names)}))
