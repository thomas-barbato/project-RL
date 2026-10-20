"""Compare unmodified engine screenshots at native pixel size."""
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageFont, ImageDraw, ImageStat

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/first-sector-review-01'
CAP=OUT/'engine-r07'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/first-sector-review-01'
checks=json.loads((CAP/'checks.json').read_text(encoding='utf-8'))
assert checks['lighting_review'] and checks['camp_round_trip']
measurements=[]
for scene in checks['scenes']:
    name=scene['scene']
    on=Image.open(CAP/f'street-{name}.png').convert('RGB')
    off=Image.open(CAP/f'street-{name}-off.png').convert('RGB')
    assert on.size==off.size and on.width>=1280 and on.height>=720
    x,y,w,h=map(int,scene['cell'])
    # Central helmet/chest, excluding the cell hover outline and scenery.
    box=(x+25,y+9,x+39,y+31)
    diff=ImageChops.difference(on.crop(box),off.crop(box))
    mean=sum(ImageStat.Stat(diff).mean)/3
    measurements.append(dict(scene=name,actor_rgb_mean_delta=round(mean,3)))
    if name in ('gate','home-entry'):assert mean>2, (name,mean)
    # The dark bezel beside the world must not be illuminated by the composite.
    assert max(on.getpixel((1558,500)))<45

font=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',18)
frames=[]
for enabled in (False,True):
    source=Image.open(CAP/('street-gate.png' if enabled else 'street-gate-off.png')).convert('RGB')
    crop=source.crop((768,238,1216,622))
    frame=Image.new('RGB',(448,422),'#10191e')
    frame.paste(crop,(0,38))
    ImageDraw.Draw(frame).text((12,7),'Lumières allumées' if enabled else 'Lumières éteintes',font=font,fill='#dec898')
    frames.append(frame)
palette_source=Image.new('RGB',(896,422))
for i,frame in enumerate(frames):palette_source.paste(frame,(448*i,0))
palette=palette_source.quantize(colors=256,dither=Image.Dither.NONE)
gif=[f.quantize(palette=palette,dither=Image.Dither.NONE) for f in frames]
gif[0].save(OUT/'u01-lights-native-r07.gif',save_all=True,append_images=gif[1:],duration=[1100,1900],loop=0,optimize=False)
Image.open(CAP/'street-road.png').crop((464,200,1488,720)).save(OUT/'u01-road-native-r07.png')
Image.open(CAP/'street-home.png').crop((688,238,1168,558)).save(OUT/'u01-home-native-r07.png')
report=dict(review='SCENE-U01-07 / LUM-U01-07',native_cell=64,framebuffer_size=list(on.size),actor_illumination_measured=measurements,hud_clip_checked=True,render_read_only=all(s['render_read_only'] for s in checks['scenes']),camp_round_trip=checks['camp_round_trip'],captures={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(CAP.glob('*.png'))})
(DOC/'u01-lighting-engine-r07.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
page='''<!doctype html><meta charset="utf-8"><title>U01 · style et éclairage</title><style>body{background:#10191e;color:#d7dfdc;font:17px Segoe UI,sans-serif;margin:24px}h1,h2{color:#dec898}.scroll{overflow:auto;border:1px solid #415159;padding:8px;margin:16px 0}img{display:block;max-width:none}button{padding:10px 18px;background:#26383e;color:#dec898;border:1px solid #63747a;cursor:pointer}p{max-width:960px;line-height:1.5}</style><h1>SCENE-U01-07 · LUM-U01-07</h1><p>Textures simplifiées, contours renforcés et pénombre froide. Les luminaires émettent dans le shader ; leur lumière atteint aussi le personnage. Les captures gardent les cases de 64 px et les règles de visibilité du jeu. Variante en revue avant activation.</p><div class="scroll"><img src="u01-lights-native-r07.gif" alt="Comparaison à taille native des lumières allumées et éteintes"></div><button id="toggle">Éteindre les lumières</button>'''
for name,label in [('gate','Portail sud'),('road','Rue'),('home-entry','Entrée de la maison'),('home','Maison'),('north','Au nord')]:
    page+=f'<h2>{label} · capture réelle du jeu</h2><div class="scroll"><img data-scene="{name}" src="engine-r07/street-{name}.png" alt="{label} à taille native"></div>'
page+='''<script>let on=true;document.getElementById('toggle').onclick=()=>{on=!on;for(const img of document.querySelectorAll('[data-scene]'))img.src='engine-r07/street-'+img.dataset.scene+(on?'':'-off')+'.png';document.getElementById('toggle').textContent=on?'Éteindre les lumières':'Allumer les lumières';};</script>'''
if (DOC/'visual-acceptance-r07.json').exists():
    page=page.replace('Variante en revue avant activation.','Rendu et lumières validés, actifs dans les nouvelles parties de génération 155. Les prochaines cartes seront plus grandes, avec la même échelle et la même référence de style.')
if (ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-roofs-review-01/acceptance-r01.json').exists():
    page=page.replace('<div class="scroll">','<p>Ces captures conservent la revue d’éclairage 07. La version actuelle avec les toits et la porte est présentée dans <a href="../street-roofs-review-01/index.html">la revue des toits</a>, pour les nouvelles parties de génération 156.</p><div class="scroll">',1)
(OUT/'index.html').write_text(page,encoding='utf-8')
print(json.dumps({k:v for k,v in report.items() if k!='captures'},ensure_ascii=False))
