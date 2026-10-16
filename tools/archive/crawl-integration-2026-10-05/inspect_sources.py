import json, hashlib
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
SOURCE=ROOT/'artifacts/crawl-methode-2026-10-05'
paths=[p for folder in ('src','assets/sprites/cyberpunk','content/core','tools/art') for p in (ROOT/folder).rglob('*') if p.is_file() and '__pycache__' not in p.parts]
baseline=OUT/'baseline.json'
if not baseline.exists():
    baseline.write_text(json.dumps({p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},indent=2))
canvas=Image.new('RGB',(1120,1000),'#202a30');d=ImageDraw.Draw(canvas)
for i in range(4):
    im=Image.open(SOURCE/f'tiles/grip-{i+1:02}.png').convert('RGBA')
    x,y=i%2*560,i//2*500
    large=im.resize((400,400),Image.Resampling.NEAREST)
    canvas.paste(large,(x+50,y+40),large)
    for a in range(0,40,2):
        d.text((x+50+a*10,y+20),str(a),fill='white')
        d.text((x+22,y+40+a*10),str(a),fill='white')
    for a in range(0,41,2):
        d.line((x+50+a*10,y+40,x+50+a*10,y+440),fill='#465159')
        d.line((x+50,y+40+a*10,x+450,y+40+a*10),fill='#465159')
canvas.save(OUT/'grip-coordinates.png')
m=json.loads((ROOT/'assets/sprites/cyberpunk/manifest.json').read_text())
equipment=json.loads((ROOT/'artifacts/sprites-cyberpunk-crawl-review-2026-10-04/equipment-catalog.json').read_text(encoding='utf-8'))['models']
print('Held:',len(m['held_weapons']),'items:',len(m['items']))
print('Special models:',[(a['id'],a['category']) for a in equipment if a['category'] in ('firearms','melee')])
