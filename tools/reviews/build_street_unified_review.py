"""U01 r03: one world-space ground plan; individual native scenery modules.

This is a tile/scene conditioner, not a stack of complete map illustrations.
Only the accepted southern departure retains its original ground composition.
"""
from collections import deque
import copy
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageFilter
import build_street_expansion_review as lighting

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'assets/maps/prison-street-01'
PARTS=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r01'
PREVIOUS=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r02'
ART=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r03'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-expansion-review-01'
OUT=ROOT/'artifacts/street-expansion-review-01'
W,H=26,66
SIZE=(W*64,H*64)
OLD_AT=(96,3136)
AXIS,ROAD_WIDTH=1212,160
BRANCHES=[(256,AXIS,900),(288,AXIS,2100),(256,AXIS,2870)]


def rel(p):return p.relative_to(ROOT).as_posix()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def move(p,d):return [a+b for a,b in zip(p,d)]


def repeat(tile, size=SIZE, phase=(0,0)):
    out=Image.new('RGBA',size)
    for y in range(-tile.height+phase[1]%tile.height,size[1],tile.height):
        for x in range(-tile.width+phase[0]%tile.width,size[0],tile.width):out.alpha_composite(tile,(x,y))
    return out


def ground(original, paving=None, asphalt_tile=None, paving_overrides=None):
    # Native 64px paving and asphalt from the accepted street. A single origin
    # and texture phase run across the entire extension, including old joins.
    replacement=paving is not None
    paving=original.crop((342,328,406,392)) if paving is None else paving
    if isinstance(paving,list):
        assert paving and all(tile.size==(64,64) for tile in paving)
        # Static art placement only: world-coordinate hash, no gameplay RNG.
        # Repeated references in the list provide the art mix weights.
        paved=Image.new('RGBA',SIZE)
        turns=[None,Image.Transpose.ROTATE_90,Image.Transpose.ROTATE_180,Image.Transpose.ROTATE_270]
        for gy in range(H):
            for gx in range(W):
                value=((gx*0x1f123bb5)^(gy*0x5f356495)^0x6a09e667)&0xffffffff
                value^=value>>16;value=(value*0x45d9f3b)&0xffffffff;value^=value>>16
                tile=(paving_overrides or {}).get((gx,gy),paving[value%len(paving)]);turn=turns[(value>>12)%4]
                paved.alpha_composite(tile if turn is None else tile.transpose(turn),(gx*64,gy*64))
    else:
        assert paving.size==(64,64)
        paved=repeat(paving)
    result=paved.copy()
    asphalt=original.crop((1036,0,1196,256)) if asphalt_tile is None else repeat(asphalt_tile,(160,256))
    # Separate the painted centre stripe from the texture; its cadence below
    # follows the world axis rather than restarting at illustration borders.
    if asphalt_tile is None:asphalt.alpha_composite(asphalt.crop((65,0,71,256)),(78,0))
    main=repeat(asphalt,SIZE,(AXIS-80,OLD_AT[1]))
    road=Image.new('L',SIZE);d=ImageDraw.Draw(road)
    d.rectangle((AXIS-80,0,AXIS+79,OLD_AT[1]+8),fill=255)
    for x0,x1,y in BRANCHES:d.rounded_rectangle((x0,y-80,x1+79,y+79),radius=48,fill=255)
    outer=road.filter(ImageFilter.MaxFilter(17))
    inner=road.filter(ImageFilter.MaxFilter(5))
    shadow=Image.new('RGBA',SIZE,(0,0,0,0));shadow.putalpha(outer.filter(ImageFilter.GaussianBlur(4)).point(lambda a:a//3))
    result.alpha_composite(shadow)
    result.paste((28,31,29,255),(0,0,*SIZE),outer)
    curb=paved.copy()
    joints=Image.new('RGBA',SIZE,(0,0,0,0));j=ImageDraw.Draw(joints)
    for x in range(0,SIZE[0],64):j.line((x,0,x,SIZE[1]),fill=(30,32,28,230),width=2)
    for y in range(0,SIZE[1],64):j.line((0,y,SIZE[0],y),fill=(30,32,28,230),width=2)
    if not replacement:
        curb.alpha_composite(joints)
    ring=ImageChops.subtract(outer.filter(ImageFilter.MinFilter(5)),inner)
    result.paste(curb,(0,0),ring)
    result.paste(main,(0,0),road)
    # Side streets use the exact same 160px carriageway and curb definition.
    sideways=repeat(asphalt.transpose(Image.Transpose.ROTATE_90))
    for x0,x1,y in BRANCHES:
        mask=Image.new('L',SIZE);md=ImageDraw.Draw(mask)
        md.rounded_rectangle((x0,y-80,x1+79,y+79),radius=48,fill=255)
        # Blend texture orientation inside the junction while retaining the
        # main avenue's uninterrupted asphalt phase.
        for x in range(AXIS-112,AXIS+80):
            md.line((x,y-80,x,y+79),fill=max(0,round(255*(AXIS+16-x)/128)))
        result.paste(sideways,(0,0),mask)
    lines=Image.new('RGBA',SIZE,(0,0,0,0));ld=ImageDraw.Draw(lines)
    for y in range(OLD_AT[1]%96-96,OLD_AT[1],96):
        ld.rectangle((AXIS-2,y,AXIS+1,y+31),fill=(178,132,34,235))
    for x0,x1,y in BRANCHES:
        for x in range(x0+96,AXIS-112,96):ld.rectangle((x,y-2,x+31,y+1),fill=(178,132,34,235))
    result.alpha_composite(lines)
    return result,road


def build():
    ART.mkdir(parents=True,exist_ok=True);DOC.mkdir(parents=True,exist_ok=True);OUT.mkdir(parents=True,exist_ok=True)
    old=json.loads((BASE/'street.json').read_text(encoding='utf-8'))
    active_hashes={p.name:sha(p) for p in BASE.iterdir() if p.is_file()}
    original=Image.open(BASE/'street.png').convert('RGBA')
    image,road=ground(original)
    footprint=Image.new('L',SIZE)
    modules=[]
    lights=[]
    red=[1.,.055,.025]

    def light(name,at,box,closed=False,radius=230,power=.8):
        lights.append(dict(id=name,at=list(at),origin=None,box=list(box) if box else None,radius=radius,power=power,
            color=red if closed else [1.,.67,.28],lens_color=red if closed else [1.,.82,.46]))

    def module(name,source,polygon,at,lamps=()):
        src=Image.open(source).convert('RGBA')
        xs,ys=zip(*polygon);box=(min(xs),min(ys),max(xs)+1,max(ys)+1)
        mask=Image.new('L',src.size);ImageDraw.Draw(mask).polygon(polygon,fill=255)
        piece=src.crop(box);piece.putalpha(mask.crop(box))
        file=ART/(name+'.png');piece.save(file)
        image.alpha_composite(piece,at)
        global_poly=[(x-box[0]+at[0],y-box[1]+at[1]) for x,y in polygon]
        ImageDraw.Draw(footprint).polygon(global_poly,fill=255)
        modules.append(dict(id=name,file=rel(file),source=rel(source),source_box=box,at=list(at),polygon=global_poly))
        for suffix,centre,lens in lamps:
            light(name+'-'+suffix,move([centre[0]-box[0],centre[1]-box[1]],at),
                  [*move([lens[0]-box[0],lens[1]-box[1]],at),*move([lens[2]-box[0],lens[3]-box[1]],at)],True)
        return box

    north=PREVIOUS/'north-source-r02.png'
    east=PARTS/'east-panel-r01.png'
    middle=PREVIOUS/'middle-source-r02.png'
    module('warehouse',north,[(52,78),(702,78),(702,282),(259,282),(259,480),(81,480),(81,334),(52,334)],(160,256))
    module('east-small',east,[(1287,17),(1520,17),(1520,239),(1498,239),(1498,316),(1287,316)],(1350,320),
        [('door',(1355,265),(1336,260,1372,272))])
    module('east-long',north,[(1188,125),(1452,125),(1452,687),(1188,687)],(1320,1120),
        [('door',(1300,630),(1278,622,1320,637))])
    module('east-blue',middle,[(1283,385),(1515,385),(1515,868),(1283,868)],(1340,2176),
        [('door',(1406,790),(1384,784,1426,801))])
    module('west-garage',east,[(38,314),(363,314),(363,622),(38,622)],(176,2400),
        [('door',(145,531),(125,526,165,537))])
    module('east-south',east,[(1297,386),(1516,386),(1516,744),(1297,744)],(1350,2740),
        [('door',(1414,704),(1390,698,1432,710))])

    # Houses remain separate native modules, never carrying their own street.
    rooms=[dict(id='ROOF-U01-02A',source='house-a-source.png',at=[448,1152],door=[11,22],inside=[12,20],
                cells=[[x,y] for y in (20,21) for x in range(9,13) if [x,y]!=[10,20]],
                floor_source=[784,1312,64,48],opening_at=[704,1440],lamp_at=[868,1458]),
           dict(id='ROOF-U01-02B',source='house-b-source.png',at=[464,1648],door=[11,30],inside=[12,28],
                cells=[[x,y] for y in (27,28,29) for x in range(10,13)],
                floor_source=[736,1808,64,48],opening_at=[704,1944],lamp_at=[864,1958])]
    for r in rooms:
        src=Image.open(PARTS/r['source']).convert('RGBA')
        # Follow the building's actual exterior instead of pasting the old
        # rectangular pavement/background surrounding it.
        width,height=src.size
        poly=[(10,10),(width-8,10),(width-8,284),(width-10,height-12),(width-64,height-12),
              (width-64,328),(64,328),(64,height-12),(10,height-12)]
        module(r['id']+'-body',PARTS/r['source'],poly,move(r['at'],(10,10)))

    # Sparse individual props give each block a purpose; shared paving stays
    # visible around them. These cutouts do not include rectangular ground.
    module('collapsed-wall',north,[(107,614),(165,626),(234,682),(270,715),(284,740),(103,740),(77,716)],(256,780))
    module('court-machines',east,[(558,274),(661,274),(661,326),(696,326),(696,410),(554,410)],(688,2500))
    module('court-bin',east,[(912,340),(1004,354),(1038,406),(1038,458),(949,476),(901,425)],(784,2720))
    # A few native hedge fragments, staggered rather than panel-wide hedges.
    for i,(x,y) in enumerate([(128,700),(832,672),(128,1024),(960,2304),(896,3024)]):
        module('hedge-'+str(i),north,[(20,346),(77,337),(101,392),(91,450),(30,457),(15,414)],(x,y))

    # Stitch into the old departure along the backs of its buildings, not along
    # a straight horizontal crop. All house and departure-gate pixels survive.
    mask=Image.new('L',original.size,255)
    md=ImageDraw.Draw(mask)
    md.rectangle((0,0,41,127),fill=0)
    md.rectangle((42,0,239,59),fill=0)
    md.rectangle((240,0,515,23),fill=0)
    accepted=original.copy();accepted.putalpha(mask)
    image.alpha_composite(accepted,OLD_AT)
    # Vegetation covers the natural rear plot boundary with small silhouettes.
    for x in (616,800,960):
        src=original.crop((540,0,636,96))
        alpha=Image.new('L',src.size);ad=ImageDraw.Draw(alpha)
        ad.ellipse((3,4,93,91),fill=255);src.putalpha(alpha)
        image.alpha_composite(src,(x,3090))

    # Reuse native perimeter and gate pieces. There is one northward gate.
    walls=Image.open(ROOT/'assets/maps/cryogenic-camp-01/structure.png').convert('RGBA')
    for y in range(1,H-1):
        image.alpha_composite(walls.crop((868,192,900,256)),(64,y*64))
        image.alpha_composite(walls.crop((1360,192,1392,256)),(1600,y*64))
    for x in range(1,W-1):
        if x not in (18,19):image.alpha_composite(walls.crop((960,320,1024,366)),(x*64,64))
        image.alpha_composite(walls.crop((960,320,1024,366)),(x*64,(H-1)*64))
    gate_src=Image.open(north).convert('RGBA')
    for a,b in ((810,879),(1008,1068)):
        image.alpha_composite(gate_src.crop((a,0,b,151)),(1078+a-810,0))
    for name,at in [('gate-west',(1110,90)),('gate-east',(1308,90))]:
        light(name,at,[at[0]-7,at[1]-7,at[0]+7,at[1]+7])
    # Varied, existing housings are reused as local props. Their illumination
    # is generated later, not included in the prop/ground texture.
    for i,(x,y) in enumerate([(1040,680),(1350,900),(1040,1536),(1350,1984),(1040,2560),(1350,3008)]):
        src=gate_src.crop((1122,574,1148,686))
        alpha=Image.new('L',src.size);ImageDraw.Draw(alpha).rounded_rectangle((3,0,25,110),radius=10,fill=255)
        src.putalpha(alpha);image.alpha_composite(src,(x,y))
        light('post-'+str(i),(x+12,y+36),(x+7,y+26,x+20,y+51),radius=220,power=.65)

    gate_roi=(240,820,560,930)
    dst=(*move(gate_roi[:2],OLD_AT),*move(gate_roi[2:],OLD_AT))
    assert image.crop(dst).tobytes()==original.crop(gate_roi).tobytes()
    image.save(ART/'street-candidate.png')

    # Geometry and scenery come from the same footprints. No road-width or
    # floor coordinates are inherited from any of the rejected panel layouts.
    floor=set()
    for y in range(2,49):
        for x in range(2,25):
            box=(x*64,y*64,(x+1)*64,(y+1)*64)
            if sum(footprint.crop(box).histogram()[1:])<96:floor.add((x,y))
    floor.update((x,y+48) for y,row in enumerate((BASE/'layout.txt').read_text().splitlines()) for x,ch in enumerate(row) if ch=='.')
    for r in rooms:
        floor.update(map(tuple,r['cells']))
        x,y=r['door'];floor.update([(x,y),(x,y-1),(x,y+1)])
    floor.update([(18,1),(19,1)])
    obstacles={(x,y+48) for x,y in old['obstructions']}
    start=tuple(move(old['entrance'],(0,48)))
    connected,pending={start},deque([start])
    while pending:
        x,y=pending.popleft()
        for p in ((x-1,y),(x+1,y),(x,y-1),(x,y+1)):
            if p in floor and p not in obstacles and p not in connected:connected.add(p);pending.append(p)
    required=[(18,1),(19,1),*(tuple(r['inside']) for r in rooms)]
    assert all(p in connected for p in required), [p for p in required if p not in connected]
    isolated=len(floor-obstacles-connected)
    floor=connected|obstacles
    assert not any(x in (0,W-1) or y in (0,H-1) for x,y in floor)
    layout='\n'.join(''.join('.' if (x,y) in floor else '#' for x in range(W)) for y in range(H))+'\n'
    (ART/'layout.txt').write_text(layout,encoding='utf-8')
    for y in (256,1024,1856,2500,3072):
        span=road.crop((0,y,SIZE[0],y+1)).getbbox()
        assert span==(AXIS-80,0,AXIS+80,1),(y,span)

    scene=copy.deepcopy(old)
    scene.update(id='MAP-U01-EXP-01-r03',width=W,height=H,image_origin=[0,0],image_offset_px=[0,0],image_size=[W,H],
        background=rel(ART/'street-candidate.png'),obstructions=[list(p) for p in sorted(obstacles)],next_exit=[18,1],candidate=True,
        walkable_cells=len(connected),visual_review='SCENE-U01-08-r03',lighting_review='LUM-U01-08-r03',
        background_sha256=sha(ART/'street-candidate.png'),layout_sha256=hashlib.sha256(layout.encode()).hexdigest())
    for key in ('entrance','home_entry','home_inside','car_at'):scene[key]=move(old[key],(0,48))
    for key in ('opening_at','lamp_at','closed_at'):scene['doorway'][key]=move(scene['doorway'][key],OLD_AT)
    for key in ('floor_source','lamp_source','closed_source'):scene['doorway'][key][:2]=move(scene['doorway'][key][:2],OLD_AT)
    for roof in scene['roofs']:
        roof['at_px']=move(roof['at_px'],OLD_AT)
        for key in ('interior','exterior'):roof[key]=[move(p,(0,48)) for p in roof[key]]
        if roof['door']:roof['door']=move(roof['door'],(0,48))
    scene['extra_doorways']=[]
    for r in rooms:
        path=PARTS/(r['id']+'-native.png');rw,rh=Image.open(path).size;a,b=r['at']
        exterior=[[x,y] for y in range(H) for x in range(W) if a<=x*64+32<a+rw and b<=y*64+32<b+rh and [x,y] not in r['cells']]
        scene['roofs'].append(dict(id=r['id'],file=rel(path),at_px=r['at'],door=r['door'],interior=r['cells'],exterior=exterior))
        scene['extra_doorways'].append(dict(door=r['door'],floor_source=r['floor_source'],opening_at=r['opening_at'],
            lamp_source=scene['doorway']['lamp_source'],lamp_at=r['lamp_at'],closed_source=scene['doorway']['closed_source'],closed_at=r['opening_at']))
        lamp=r['lamp_at'];light(r['id']+'-door',(lamp[0]+25,lamp[1]+9),None)
        light(r['id']+'-inside',(r['inside'][0]*64+32,r['inside'][1]*64+32),None)
    config=copy.deepcopy(old['atmosphere'])
    for l in config['lights']:
        l['at']=move(l['at'],(0,3072));l['origin']=move(l['origin'],(0,48))
        if l['box']:l['box']=[*move(l['box'][:2],OLD_AT),*move(l['box'][2:],OLD_AT)]
        closed=l['id'] in ('shop-door','garage','ruin-west','ruin-east')
        l.update(color=red if closed else [1.,.67,.28],lens_color=red if closed else [1.,.82,.46])
    for l in lights:
        l['origin']=min(connected,key=lambda p:(p[0]*64+32-l['at'][0])**2+(p[1]*64+32-l['at'][1])**2)
    config['lights']+=lights
    lighting.ART,lighting.W,lighting.H=ART,W,H
    lighting.build_lightmaps(image,floor,config,config['lights'])
    config.update(correction=rel(ART/'correction.png'),irradiance=rel(ART/'irradiance.png'),colored_emitters=True,
        fan=[(1460+OLD_AT[0])/SIZE[0],(366+OLD_AT[1])/SIZE[1],12/SIZE[0],12/SIZE[1]])
    scene['atmosphere']=config;scene['ambience']=dict(steam=move([746,418],OLD_AT),embers=True)
    scene['review_views']=[['gate',scene['entrance']],['red-door',[13,54]],['yellow-door',[4,54]],['garage',[22,58]],
        ['courtyard',[16,40]],['north-yard',[12,9]],['exit',[18,1]],['join-north',[18,17]],['join-middle',[18,33]],['join-south',[18,49]],
        ['house-a-closed',[11,23]],['house-a-open',[12,20]],['house-b-closed',[11,31]],['house-b-open',[12,28]]]
    scene['review_animations']=[['fan',[24,53]],['steam',[12,55]],['embers',[15,57]]]
    (ART/'street.json').write_text(json.dumps(scene,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    plan=dict(revision='r03',coordinate_system='single native world pixel grid',road_axis=AXIS,road_width=ROAD_WIDTH,
        branches=BRANCHES,old_at=OLD_AT,modules=modules,complete_map_panels_used=0,accepted_departure_used_once=True)
    (ART/'layout-plan.json').write_text(json.dumps(plan,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    assert active_hashes=={p.name:sha(p) for p in BASE.iterdir() if p.is_file()}
    report=dict(review='MAP-U01-EXP-01-r03',status='candidate_not_integrated',extension_direction='north only',dimensions=[W,H],
        old_dimensions=[26,18],native_cell=64,old_walkable=173,walkable=len(connected),connected=True,old_departure_preserved=True,
        complete_map_panels_used=0,road_axis=AXIS,road_width=ROAD_WIDTH,road_width_sample_rows=[256,1024,1856,2500,3072],
        clipped_isolated_floor_cells=isolated,next_exit=[18,1],next_zone_not_generated=True,visitable_buildings=3,
        active_hashes=active_hashes,active_map_unchanged=True,source_sha256={rel(p):sha(p) for p in ART.glob('*.png')})
    (DOC/'candidate-r03.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    nav=image.copy();draw=ImageDraw.Draw(nav,'RGBA')
    for x,y in floor:draw.rectangle((x*64,y*64,(x+1)*64-1,(y+1)*64-1),outline=(80,210,180,180))
    nav.save(OUT/'navigation-native.png')
    print(json.dumps({k:v for k,v in report.items() if k not in ('source_sha256','active_hashes')},ensure_ascii=False))


if __name__=='__main__':build()
