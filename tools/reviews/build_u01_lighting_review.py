"""Numerical light data for the existing camp shaders; no source pixels edited."""
import hashlib
import argparse
import json
import math
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
parser=argparse.ArgumentParser()
parser.add_argument('--doorway-fix',action='store_true')
args=parser.parse_args()
BASE = ROOT/'assets/source-art/first-sector-review-01/u01-style-r07'
ART = ROOT/'assets/source-art/first-sector-review-01/u01-doorway-fix-r01' if args.doorway_fix else BASE
DOC = ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-roofs-review-01' if args.doorway_fix else ROOT/'docs/reviews/refonte-histoire-2026-10-16/first-sector-review-01'
ART.mkdir(parents=True,exist_ok=True)
rows = (ROOT/'assets/maps/prison-street-01/layout.txt').read_text().splitlines()
W,H=len(rows[0])*64,len(rows)*64
ambient=[.56,.61,.72]
# Emitter position in source pixels, floor origin for wall occlusion, lens box.
definitions=[
 ('home', (386,65),(7,3),(368,58,408,70),230,.72),
 ('home-door',(190,307),(4,6),(169,302,211,313),225,.70),
 ('shop-door',(749,288),(13,6),(730,282,770,294),225,.70),
 ('west',(58,437),(2,8),(53,418,64,456),230,.65),
 ('north-west',(978,173),(18,3),(973,157,984,196),230,.65),
 ('north-east',(1251,93),(19,2),(1245,78,1258,115),235,.65),
 ('north-east-upper',(1243,27),(19,1),(1237,13,1249,46),185,.50),
 ('garage-west',(1178,423),(19,8),(1172,408,1184,439),230,.66),
 ('garage',(1280,543),(22,10),(1259,537,1302,548),240,.70),
 ('gate-left',(279,870),(7,14),(273,851,285,887),215,.68),
 ('gate-right',(538,870),(8,14),(532,851,544,887),215,.68),
 ('ruin-west',(828,841),(14,15),(808,835,849,847),225,.68),
 ('ruin-east',(1292,842),(21,15),(1272,836,1311,848),225,.68),
 ('car',(964,474),(16,8),None,170,.42),
]
if args.doorway_fix:
    doorway=json.loads((ART/'doorway.json').read_text(encoding='utf-8'))
    old=doorway['lamp_lens_center_in_source'];rect=doorway['lamp_source'];at=doorway['lamp_at']
    centre=(old[0]-rect[0]+at[0],old[1]-rect[1]+at[1])
    definitions=[(name,centre,tuple(doorway['light_origin']),box,radius,power) if name=='home-door' else (name,position,origin,box,radius,power)
                 for name,position,origin,box,radius,power in definitions]
lights=[]
for name,(x,y),origin,box,radius,power in definitions:
    assert rows[origin[1]][origin[0]]=='.', (name,origin)
    lights.append(dict(id=name,at=[x+96,y+64],origin=origin,box=box,radius=radius,power=power*1.3))

def floor(x,y):
    return 0<=x<len(rows[0]) and 0<=y<len(rows) and rows[y][x]=='.'

def visible(light,tx,ty):
    ox,oy=light['origin'];ax,ay=light['at']
    sx=min(max(ax,ox*64+1),ox*64+63)
    sy=min(max(ay,oy*64+1),oy*64+63)
    gx,gy=int(tx//64),int(ty//64)
    steps=max(1,math.ceil(math.hypot(tx-sx,ty-sy)/8))
    for i in range(1,steps+1):
        x=int((sx+(tx-sx)*i/steps)//64);y=int((sy+(ty-sy)*i/steps)//64)
        if (x,y)!=(gx,gy) and not floor(x,y): return False
    return True

# A first wall face can receive light from an adjacent visible floor, but
# receivers behind that wall cannot. This follows the camp's existing model.
faces=[]
for light in lights:
    lit={(x,y) for y in range(len(rows)) for x in range(len(rows[0]))
         if floor(x,y) and visible(light,x*64+32,y*64+32)}
    faces.append({(x+dx,y+dy) for x,y in lit for dx,dy in [(0,-1),(1,0),(0,1),(-1,0)]})
scale=8
image=Image.new('RGB',(W//scale,H//scale))
stats=[dict(id=l['id'],lit_samples=0,blocked_samples=0) for l in lights]
for y in range(image.height):
    ty=y*scale+scale/2
    for x in range(image.width):
        tx=x*scale+scale/2;rgb=list(ambient)
        for i,l in enumerate(lights):
            d2=(tx-l['at'][0])**2+(ty-l['at'][1])**2
            if d2>=l['radius']**2:continue
            gx,gy=int(tx//64),int(ty//64)
            seen=visible(l,tx,ty) if floor(gx,gy) else (gx,gy) in faces[i]
            if not seen:stats[i]['blocked_samples']+=1;continue
            k=(math.exp(-3*d2/l['radius']**2)-math.exp(-3))/(1-math.exp(-3))*l['power']
            for c,tint in enumerate([1.,.67,.28]):rgb[c]+=k*tint
            stats[i]['lit_samples']+=1
        image.putpixel((x,y),tuple(min(255,round(v*127.5)) for v in rgb))
image.save(ART/'irradiance.png')
# Identity correction for the newly neutral source. Alpha identifies lamp
# lenses so the same shader can turn their emission off for the comparison.
correction=Image.new('RGBA',(1536,1024),(128,128,128,0))
source=BASE/'SCENE-U01-07-r01.png'
ink=Image.open(source).convert('RGB')
for l in lights:
    if l['box']:
        x0,y0,x1,y1=l['box']
        for y in range(y0,y1+1):
            for x in range(x0,x1+1):
                r,g,b=ink.getpixel((x,y))
                if r>165 and g>150 and b>90:
                    correction.putpixel((x,y),(128,128,128,255))
correction.save(ART/'illumination-correction.png')
config=dict(correction=(ART/'illumination-correction.png').relative_to(ROOT).as_posix(),irradiance=(ART/'irradiance.png').relative_to(ROOT).as_posix(),ambient=ambient,lights=lights,lightmap_scale=scale,procedural_emitters=True)
(ART/'lighting.json').write_text(json.dumps(config,indent=2)+'\n',encoding='utf-8')
source=BASE/'SCENE-U01-07-r01.png'
report=dict(review_id='DOOR-U01-FIX-01' if args.doorway_fix else 'LUM-U01-07-r01',status='authorized_bug_fix' if args.doorway_fix else 'candidate_not_integrated',source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),source_art_unchanged=True,light_count=len(lights),lightmap_size=list(image.size),shadows='Static occlusion by the actual wall grid; same model as the camp',actors_receive_light_in_composite=True,stats=stats)
assert not visible(lights[1],416,288), 'Door lamp must not cross the adjacent solid home wall'
assert visible(lights[1],288,416), 'Door approach must receive its own light'
report.update(wall_probe_blocked=True,door_approach_lit=True)
(DOC/('doorway-lighting-data-r01.json' if args.doorway_fix else 'u01-lighting-data-r07.json')).write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps(report))
