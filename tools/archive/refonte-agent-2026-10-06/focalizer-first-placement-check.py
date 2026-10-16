"""Condition the first source and inspect approved hand/arm alignment."""
from pathlib import Path
import json,runpy,shutil
from PIL import Image,ImageDraw,ImageFont

BASE=Path(__file__).resolve().parent
ROOT=BASE.parents[1]
OUT=BASE/'focalizer-review-01'
OUT.mkdir(exist_ok=True)
source=Path('C:/Users/User/.codex/generated_images/01a11010-433f-7fc2-ac29-ffb696dec5ea/exec-8844ec1b-3a07-407f-b2ad-f158bbe4001a.png')
(OUT/'sources').mkdir(exist_ok=True)
shutil.copy2(source,OUT/'sources/EQ-F01.png')
im=Image.open(source).convert('RGBA')
bbox=im.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()
crop=im.crop(bbox)
sample=crop.resize((14,27),Image.Resampling.NEAREST)
colors=[p[:3] for p in sample.get_flattened_data() if p[3]>=128]
strip=Image.new('RGB',(len(colors),1));strip.putdata(colors)
palette=strip.quantize(colors=12,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE)
held=crop.resize((9,18),Image.Resampling.NEAREST)
alpha=held.getchannel('A').point(lambda a:255 if a>=128 else 0)
held=held.convert('RGB').quantize(palette=palette,dither=Image.Dither.NONE).convert('RGBA');held.putalpha(alpha)
held.save(OUT/'EQ-F01-held.png')
tile=Image.new('RGBA',(48,48));tile.alpha_composite(held,(20,18))
H=runpy.run_path(str(BASE/'build_weapon_silhouette_review_01.py'),run_name='helpers')
rig=json.loads((ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json').read_text(encoding='utf-8'))
board=Image.new('RGB',(920,300),(12,19,27));d=ImageDraw.Draw(board)
d.text((18,12),'EQ-F01 · contrôle des prises',font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',21),fill='white')
d.text((18,45),'Corps et bras fixes · tête orientée dans le prolongement de l’avant-bras',font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',13),fill=(160,182,196))
bank=BASE/'weapon-silhouette-review-03'
floor=Image.open(bank/'review-floor-native32.png').convert('RGBA')
for p in rig['profiles']:
    n=p['number'];p['body']=Image.open(bank/f'body-{n}-native32.png').convert('RGBA')
    d.text((18+(n-1)*180,81),p['class'],font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',13),fill='white')
    for s,side in enumerate(('L','R')):
        layer,qa=H['one_hand'](p,6,side,tile,[24,33])
        frame=Image.open(bank/f'C{n}-Lnone-Rnone.png').convert('RGBA');frame.alpha_composite(layer)
        scene=Image.new('RGBA',(64,64))
        for y in (0,32):
            for x in (0,32):scene.alpha_composite(floor,(x,y))
        scene.alpha_composite(frame)
        x=18+(n-1)*180;y=106+s*88
        board.paste(scene,(x,y))
        board.paste(scene.resize((128,128),Image.Resampling.NEAREST).crop((16,16,112,112)),(x+64,y-10))
board.save(OUT/'EQ-F01-first-placement.png')
print('10 placements checked, existing arms retained.')
