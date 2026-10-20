"""U01 r02: preserve the southern departure and extend only towards the north.

Source conditioning and isolated review data only. Active map/saves are untouched.
"""
from collections import deque
import copy
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw
import build_street_expansion_review as lighting

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r02'
PARTS = ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r01'
BASE = ROOT / 'assets/maps/prison-street-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-expansion-review-01'
OUT = ROOT / 'artifacts/street-expansion-review-01'
W, H, OLD_DY = 26, 66, 48
OLD_AT = (96, 3136)


def rel(p):
    return p.relative_to(ROOT).as_posix()


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def translated(p, d):
    return [a + b for a, b in zip(p, d)]


def build():
    DOC.mkdir(parents=True, exist_ok=True)
    OUT.mkdir(parents=True, exist_ok=True)
    old = json.loads((BASE / 'street.json').read_text(encoding='utf-8'))
    active_hashes = {p.name: sha(p) for p in BASE.iterdir() if p.is_file()}
    image = Image.new('RGBA', (W * 64, H * 64), '#101719')
    for file, y in [(ART / 'north-source-r02.png', 64),
                    (ART / 'middle-source-r02.png', 1088),
                    (PARTS / 'east-panel-r01.png', 2112),
                    (BASE / 'street.png', 3136)]:
        panel = Image.open(file).convert('RGBA')
        assert panel.size == (1536, 1024), (file, panel.size)
        image.alpha_composite(panel, (96, y))
    rooms = [dict(id='ROOF-U01-02A', source='house-a-source.png', at=[448, 1152],
                  door=[11, 22], inside=[12, 20],
                  cells=[[x, y] for y in (20, 21) for x in range(9, 13) if [x, y] != [10, 20]],
                  floor_source=[784, 1312, 64, 48], opening_at=[704, 1440], lamp_at=[868, 1458]),
             dict(id='ROOF-U01-02B', source='house-b-source.png', at=[464, 1648],
                  door=[11, 30], inside=[12, 28],
                  cells=[[x, y] for y in (27, 28, 29) for x in range(10, 13)],
                  floor_source=[736, 1808, 64, 48], opening_at=[704, 1944], lamp_at=[864, 1958])]
    for room in rooms:
        image.alpha_composite(Image.open(PARTS / room['source']).convert('RGBA'), room['at'])

    # Native approved camp wall strips close the other perimeter edges.
    wall = Image.open(ROOT / 'assets/maps/cryogenic-camp-01/structure.png').convert('RGBA')
    horizontal = wall.crop((960, 320, 1024, 366))
    left = wall.crop((868, 192, 900, 256))
    right = wall.crop((1360, 192, 1392, 256))
    for y in range(1, H - 1):
        image.alpha_composite(left, (64, y * 64))
        image.alpha_composite(right, (1600, y * 64))
    for x in range(1, W - 1):
        if x not in (15, 16):
            image.alpha_composite(horizontal, (x * 64, 64))
        image.alpha_composite(horizontal, (x * 64, (H - 1) * 64))
    asphalt = Image.open(BASE / 'street.png').convert('RGBA').crop((1056, 400, 1120, 464))
    for x in (15, 16):
        image.alpha_composite(asphalt, (x * 64, 0))
    # Compare the original departure gate, unaffected by the perimeter closure.
    gate = (240, 820, 560, 930)
    assert image.crop((gate[0] + OLD_AT[0], gate[1] + OLD_AT[1], gate[2] + OLD_AT[0], gate[3] + OLD_AT[1])).tobytes() == Image.open(BASE / 'street.png').convert('RGBA').crop(gate).tobytes()
    image.save(ART / 'street-candidate.png')

    floor = {(x, y + OLD_DY) for y, row in enumerate((BASE / 'layout.txt').read_text().splitlines())
             for x, ch in enumerate(row) if ch == '.'}
    def rect(box, band=0):
        a, b, c, d = box
        for y in range(1, H - 1):
            for x in range(2, W - 1):
                sx, sy = x * 64 - 64, y * 64 - 32 - band
                if a <= sx < c and b <= sy < d:
                    floor.add((x, y))
    for box in [(840, 0, 1030, 960), (1040, 145, 1140, 940), (0, 960, 1536, 1024),
                (280, 300, 550, 715), (550, 300, 800, 380), (650, 540, 840, 700),
                (310, 540, 780, 715), (700, 390, 1040, 500), (1150, 715, 1470, 780)]:
        rect(box)
    for box in [(1024, 0, 1160, 1024), (340, 160, 960, 390),
                (96, 225, 340, 390), (96, 380, 1024, 435), (0, 430, 1160, 550), (250, 640, 960, 920),
                (250, 920, 1160, 960), (0, 960, 1536, 1024)]:
        rect(box, 1024)
    for box in [(1100, 0, 1216, 1024), (400, 280, 1030, 535), (400, 210, 1050, 265),
                (1024, 200, 1100, 620), (360, 210, 420, 625), (400, 580, 1216, 720),
                (0, 640, 1216, 700), (0, 960, 1536, 1024)]:
        rect(box, 2048)
    for room in rooms:
        a, b = room['at']
        width, height = Image.open(PARTS / room['source']).size
        floor = {p for p in floor if not (a <= p[0] * 64 + 32 < a + width and b <= p[1] * 64 + 32 < b + height)}
        floor.update(map(tuple, room['cells']))
        x, y = room['door']
        floor.update([(x, y), (x, y - 1), (x, y + 1)])
    # Travel is triggered on the gate tile before the map edge, like other
    # authored transitions. The engine requires the outermost row to be sealed.
    floor.update((x, 1) for x in (15, 16))
    obstacles = {(x, y + OLD_DY) for x, y in old['obstructions']}
    start = tuple(translated(old['entrance'], (0, OLD_DY)))
    connected, pending = {start}, deque([start])
    while pending:
        x, y = pending.popleft()
        for p in ((x-1, y), (x+1, y), (x, y-1), (x, y+1)):
            if p in floor and p not in obstacles and p not in connected:
                connected.add(p)
                pending.append(p)
    for p in [(15, 1), (16, 1), *(tuple(r['inside']) for r in rooms)]:
        assert p in connected, ('Intended location disconnected', p)
    removed = len(floor - obstacles - connected)
    assert removed == 0, ('Disconnected floor fragments', sorted(floor-obstacles-connected))
    border = {p for p in floor if p[0] in (0, W-1) or p[1] in (0, H-1)}
    assert not border, border
    layout = '\n'.join(''.join('.' if (x,y) in floor else '#' for x in range(W)) for y in range(H))+'\n'
    (ART / 'layout.txt').write_text(layout, encoding='utf-8')
    scene = copy.deepcopy(old)
    scene.update(id='MAP-U01-EXP-01-r02', width=W, height=H, image_origin=[0,0], image_offset_px=[0,0], image_size=[W,H],
                 background=rel(ART/'street-candidate.png'), obstructions=[list(p) for p in sorted(obstacles)],
                 next_exit=[15,1], next_zone='U02', candidate=True, walkable_cells=len(connected),
                 background_sha256=sha(ART/'street-candidate.png'),layout_sha256=hashlib.sha256(layout.encode()).hexdigest(),
                 visual_review='SCENE-U01-08-r02',lighting_review='LUM-U01-08-r02')
    for key in ('entrance','home_entry','home_inside','car_at'):
        scene[key] = translated(old[key], (0, OLD_DY))
    doorway = scene['doorway']
    for key in ('opening_at','lamp_at','closed_at'):
        doorway[key] = translated(doorway[key], OLD_AT)
    for key in ('floor_source','lamp_source','closed_source'):
        doorway[key][:2] = translated(doorway[key][:2], OLD_AT)
    for roof in scene['roofs']:
        roof['at_px'] = translated(roof['at_px'], OLD_AT)
        for key in ('interior','exterior'):
            roof[key] = [translated(p, (0, OLD_DY)) for p in roof[key]]
        if roof['door']:
            roof['door'] = translated(roof['door'], (0, OLD_DY))
    scene['extra_doorways'] = []
    for room in rooms:
        roof_path = PARTS/(room['id']+'-native.png')
        width,height = Image.open(roof_path).size
        a,b = room['at']
        exterior = [[x,y] for y in range(H) for x in range(W)
                    if a<=x*64+32<a+width and b<=y*64+32<b+height and [x,y] not in room['cells']]
        scene['roofs'].append(dict(id=room['id'],file=rel(roof_path),at_px=room['at'],door=room['door'],interior=room['cells'],exterior=exterior))
        scene['extra_doorways'].append(dict(door=room['door'],floor_source=room['floor_source'],opening_at=room['opening_at'],
            lamp_source=doorway['lamp_source'],lamp_at=room['lamp_at'],closed_source=doorway['closed_source'],closed_at=room['opening_at']))
    config = copy.deepcopy(old['atmosphere'])
    lights=config['lights']
    red=[1.,.055,.025]
    for light in lights:
        light['at']=translated(light['at'],(0,OLD_DY*64))
        light['origin']=translated(light['origin'],(0,OLD_DY))
        if light['box']:
            light['box']=[*translated(light['box'][:2],OLD_AT),*translated(light['box'][2:],OLD_AT)]
        closed=light['id'] in ('shop-door','garage','ruin-west','ruin-east')
        light['color']=red if closed else [1.,.67,.28]
        light['lens_color']=red if closed else [1.,.82,.46]
    fixtures=[
        ('north-house',(1300,630),(1278,622,1320,637),True),
        ('north-gate-west',(842,90),(834,82,850,98),False),
        ('north-gate-east',(1040,90),(1032,82,1048,98),False),
        ('north-post',(1134,612),(1126,596,1143,627),False),
        ('middle-closed',(1406,1814),(1384,1808,1426,1823),True),
        ('middle-round',(950,1342),(939,1332,960,1351),False),
        ('middle-post',(82,1542),(75,1525,90,1560),False),
        ('court-closed',(616,2213),(596,2207,634,2218),True),
        ('court-east-closed',(1355,2313),(1336,2307,1372,2319),True),
        ('court-south-closed',(1414,2752),(1390,2744,1432,2758),True),
    ]
    for name,at,box,closed in fixtures:
        at=translated(at,(96,64))
        origin=min(connected,key=lambda p:(p[0]*64+32-at[0])**2+(p[1]*64+32-at[1])**2)
        lights.append(dict(id=name,at=at,origin=origin,box=[*translated(box[:2],(96,64)),*translated(box[2:],(96,64))],
                           radius=230,power=.8,color=red if closed else [1.,.72,.38],lens_color=red if closed else [1.,.82,.46]))
    for room in rooms:
        lamp=room['lamp_at']
        for name,at,origin in [('door',[lamp[0]+25,lamp[1]+9],[room['door'][0],room['door'][1]+1]),
                              ('inside',[room['inside'][0]*64+32,room['inside'][1]*64+32],room['inside'])]:
            lights.append(dict(id=room['id']+'-'+name,at=at,origin=origin,box=None,radius=215,power=.8,
                               color=[1.,.67,.28],lens_color=[1.,.82,.46]))
    # Reuse the current camp/street numerical occlusion model; no painted halos.
    lighting.ART,lighting.W,lighting.H=ART,W,H
    lighting.build_lightmaps(image,floor,config,lights)
    config.update(correction=rel(ART/'correction.png'),irradiance=rel(ART/'irradiance.png'),colored_emitters=True,
                  fan=[(1460+OLD_AT[0])/(W*64),(366+OLD_AT[1])/(H*64),12/(W*64),12/(H*64)])
    scene['atmosphere']=config
    scene['ambience']=dict(steam=translated([746,418],OLD_AT),embers=True)
    scene['review_views']=[['gate',scene['entrance']],['red-door',[13,54]],['yellow-door',[4,54]],['garage',[22,58]],
        ['courtyard',[12,39]],['north-yard',[8,9]],['exit',[15,1]],['house-a-closed',[11,23]],
        ['house-a-open',[12,20]],['house-b-closed',[11,31]],['house-b-open',[12,28]]]
    scene['review_animations']=[['fan',[24,53]],['steam',[12,55]],['embers',[15,57]]]
    (ART/'street.json').write_text(json.dumps(scene,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    assert active_hashes=={p.name:sha(p) for p in BASE.iterdir() if p.is_file()}
    report=dict(review='MAP-U01-EXP-01-r02 / LUM-U01-08-r02 / AMB-U01-08-r02',status='candidate_not_integrated',
        supersedes='MAP-U01-EXP-01-r01: direction corrected by user',extension_direction='north only',
        old_dimensions=[26,18],dimensions=[W,H],area_ratio=W*H/(26*18),native_cell=64,
        old_walkable=old['walkable_cells'],walkable=len(connected),connected=True,outer_boundary_sealed=True,perimeter_gate=dict(at=[15,1],width=2),
        old_departure=old['entrance'],departure=scene['entrance'],old_area_offset_cells=[0,48],departure_gate_pixels_preserved=True,
        next_exit=[15,1],next_zone_not_generated=True,visitable_buildings=3,active_map_unchanged=True,
        active_hashes=active_hashes,source_sha256={rel(p):sha(p) for p in ART.glob('*.png')})
    (DOC/'candidate-r02.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    navigation=image.copy();draw=ImageDraw.Draw(navigation,'RGBA')
    for x,y in floor:
        draw.rectangle((x*64,y*64,(x+1)*64-1,(y+1)*64-1),outline=(80,210,180,180))
    navigation.save(OUT/'navigation-native.png')
    print(json.dumps({k:v for k,v in report.items() if k not in ('source_sha256','active_hashes')},ensure_ascii=False))


if __name__=='__main__':
    build()
