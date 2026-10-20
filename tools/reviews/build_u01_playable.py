"""Package the reviewed street for native64, without painting source pixels."""
from collections import deque
import hashlib
import json
from pathlib import Path
import shutil

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/maps/prison-street-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/first-sector-review-01'
OUT = ROOT / 'artifacts/first-sector-review-01'
ACCEPTED_STYLE = (DOC/'visual-acceptance-r07.json').exists()
if ACCEPTED_STYLE:
    accepted=json.loads((DOC/'visual-acceptance-r07.json').read_text(encoding='utf-8'))
    for relative,expected in accepted['source_sha256'].items():
        assert hashlib.sha256((ROOT/relative).read_bytes()).hexdigest()==expected, f'Accepted reference changed: {relative}'
SOURCE = ROOT / ('assets/source-art/first-sector-review-01/u01-style-r07/SCENE-U01-07-r01.png' if ACCEPTED_STYLE else 'assets/source-art/first-sector-review-01/u01-scene-r03/SCENE-U01-06-r01.png')
ART.mkdir(parents=True, exist_ok=True)
OUT.mkdir(parents=True, exist_ok=True)
shutil.copyfile(SOURCE, ART / 'street.png')
car_source = ROOT / 'assets/source-art/first-sector-review-01/u01-r03/DEC-U01-17-r04.png'
shutil.copyfile(car_source, ART / 'car.png')
background = Image.open(SOURCE).convert('RGBA')
assert background.size == (1536, 1024)
car = Image.open(car_source).convert('RGBA')
assert car.size == (128, 64)

# Shift the illustration 32px horizontally so the house opening centres on a
# logical cell. Pixels remain unchanged; every simulation cell stays 64px.
origin_px = (96,64)
floor = {(x,y) for y in range(7,11) for x in range(2,19)}
floor |= {(x,6) for x in range(3,19)}
floor |= {(19,y) for y in range(7,11)}
floor |= {(x,10) for x in range(20,25)}
floor |= {(x,y) for y in range(1,7) for x in (18,19)}
floor |= {(x,y) for y in (4,5) for x in range(20,25)}
floor |= {(x,y) for y in range(11,14) for x in (7,8,9)}
floor |= {(x,y) for y in range(14,17) for x in (7,8)}
floor |= {(x,y) for y in (15,16) for x in range(2,25)}
floor |= {(x,y) for y in (3,4) for x in range(4,8)}
floor |= {(4,5),(4,6)}
entrance = (7,14)
car_at = (15,8)
car_cells = {(15,8),(16,8)}
low_obstacles = {(5,6),(9,6),(10,6),(15,6),(18,10)} | car_cells
# The car occupies its physical two cells; it does not hide a hidden third cell.
floor |= car_cells
layout = '\n'.join(''.join('.' if (x,y) in floor else '#' for x in range(26)) for y in range(18))+'\n'
(ART/'layout.txt').write_text(layout, encoding='utf-8')
walkable = floor-low_obstacles
previous = {entrance: None}
pending = deque([entrance])
while pending:
    at = pending.popleft()
    for dx,dy in [(0,-1),(1,0),(0,1),(-1,0)]:
        nxt = at[0]+dx,at[1]+dy
        if nxt in walkable and nxt not in previous:
            previous[nxt]=at
            pending.append(nxt)
assert len(previous)==len(walkable), sorted(walkable-previous.keys())
assert (6,4) in previous, 'Home unreachable'
assert (19,2) in previous, 'Northeast street unreachable'
manifest = {'id':'MAP-U01-06-r01','width':26,'height':18,'cell':64,
            'image_origin':[1,1], 'image_offset_px':[32,0], 'image_size':[24,16],
            'background':'assets/maps/prison-street-01/street.png',
            'entrance':list(entrance),'home_entry':[4,5],'home_inside':[6,4],
            'car_at':list(car_at),'car_file':'assets/maps/prison-street-01/car.png',
            'car_size':[2,1],'obstructions':[list(p) for p in sorted(low_obstacles)],
            'source':str(SOURCE.relative_to(ROOT)).replace('\\','/'),
            'background_sha256':hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
            'layout_sha256':hashlib.sha256(layout.encode()).hexdigest(),
            'walkable_cells':len(walkable),'all_walkable_connected':True}
if ACCEPTED_STYLE:
    light_art=ROOT/'assets/source-art/first-sector-review-01/u01-style-r07'
    doorway_art=ROOT/'assets/source-art/first-sector-review-01/u01-doorway-fix-r01'
    if (doorway_art/'lighting.json').exists():
        light_art=doorway_art
        manifest['doorway']=json.loads((doorway_art/'doorway.json').read_text(encoding='utf-8'))
        manifest['doorway_fix']='DOOR-U01-FIX-01'
    atmosphere=json.loads((light_art/'lighting.json').read_text(encoding='utf-8'))
    for key in ('correction','irradiance'):
        file=Path(atmosphere[key]).name
        shutil.copyfile(ROOT/atmosphere[key],ART/file)
        atmosphere[key]=(ART/file).relative_to(ROOT).as_posix()
    manifest.update(visual_review='SCENE-U01-07-r01',lighting_review='LUM-U01-07-r01',style='STYLE-RL-2026-10-18-r01',atmosphere=atmosphere)
roof_doc=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-roofs-review-01'
if (roof_doc/'acceptance-r01.json').exists():
    agreement=json.loads((roof_doc/'acceptance-r01.json').read_text(encoding='utf-8'))
    for relative,expected in agreement['source_sha256'].items():
        assert hashlib.sha256((ROOT/relative).read_bytes()).hexdigest()==expected, relative
    layers=[]
    for code,filename,door,interior in [
        ('ROOF-U01-01A','home-roof.png',[4,5],[(x,y) for y in (3,4) for x in range(4,8)]),
        ('ROOF-U01-01B','shop-roof.png',None,[]),
    ]:
        im=Image.open(ROOT/f'assets/source-art/first-sector-review-01/street-roofs-r01/{code}-r01.png').convert('RGBA')
        bounds=im.getbbox()
        im.crop(bounds).save(ART/filename)
        alpha=im.getchannel('A')
        cells=[]
        for y in range(18):
            for x in range(26):
                a,b=x*64-origin_px[0],y*64-origin_px[1]
                if (x,y) not in interior and alpha.crop((a,b,a+64,b+64)).getbbox():
                    cells.append([x,y])
        layers.append(dict(id=code,file=(ART/filename).relative_to(ROOT).as_posix(),
                           at_px=list(bounds[:2]),door=door,interior=[list(p) for p in interior],
                           exterior=cells,sha256=hashlib.sha256((ART/filename).read_bytes()).hexdigest()))
    manifest.update(roofs=layers,roof_review='ROOF-U01-01-r01')
(ART/'street.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')

player_path = ROOT/'assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png'
player=Image.open(player_path).convert('RGBA')
def at_player(scene, at):
    scene.alpha_composite(player, (at[0]*64-origin_px[0]-32,at[1]*64-origin_px[1]-32))
native=background.copy()
native.alpha_composite(car,(car_at[0]*64-origin_px[0],car_at[1]*64-origin_px[1]))
at_player(native,entrance)
native.convert('RGB').save(OUT/'u01-playable-native-r02.png')
home_preview=background.copy()
at_player(home_preview,(6,4))
home_preview.crop((0,0,640,448)).convert('RGB').save(OUT/'u01-home-native-r02.png')
grid=native.copy(); d=ImageDraw.Draw(grid,'RGBA')
for x,y in floor:
    box=(x*64-origin_px[0],y*64-origin_px[1],(x+1)*64-origin_px[0]-1,(y+1)*64-origin_px[1]-1)
    d.rectangle(box,outline=(100,220,190,130) if (x,y) not in low_obstacles else (240,75,55,180))
grid.convert('RGB').save(OUT/'u01-navigation-review-r02.png')
(DOC/'u01-playable-packaging-r02.json').write_text(json.dumps({**manifest,'status':'accepted_integrated' if ACCEPTED_STYLE else 'projection_accepted_integration_in_progress','player_scale':1,'player_source_sha256':hashlib.sha256(player_path.read_bytes()).hexdigest()},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps(manifest,ensure_ascii=False))
