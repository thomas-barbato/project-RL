"""U01 r04: native damage/decor modules on the continuous r03 map.

Generated art is conditioned to native size, then placed in world coordinates.
The map geometry is never regenerated from a new complete illustration.
"""
from collections import deque
import copy
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont
import build_street_expansion_review as lighting
import build_street_unified_review as unified

ROOT=Path(__file__).resolve().parents[2]
PREVIOUS=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r03'
ART=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r04'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-expansion-review-01'
OUT=ROOT/'artifacts/street-expansion-review-01'
BASE=ROOT/'assets/maps/prison-street-01'


def rel(p):return p.relative_to(ROOT).as_posix()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()


def assets():
    # Native exports only: alpha and proportions of the generated artwork stay
    # intact. The valleys are transparent gutters in the two source atlases.
    definitions=[
        ('barriers','Barrières débordées',(128,80)),
        ('fence','Grillage renversé',(128,64)),
        ('bench','Banc retourné',(112,60)),
        ('kiosk','Borne vandalisée',(64,80)),
        ('dumpster','Benne renversée',(96,96)),
        ('trolley','Chariot abandonné',(72,72)),
        ('crates','Caisses éventrées',(128,96)),
        ('bikes','Vélos écrasés',(128,80)),
        ('charred','Bois calciné',(128,72)),
        ('glass','Verre et papiers',(96,72)),
        ('riot','Équipement abandonné',(64,64)),
        ('soot','Sol brûlé',(128,128)),
        ('metal','Tôle arrachée',(96,64)),
        ('rubble','Maçonnerie brisée',(128,80)),
        ('furniture','Mobilier sorti dans la rue',(96,80)),
        ('papers','Affaires dispersées',(96,72)),
        ('roof-hole','Couverture arrachée',(144,120)),
        ('window','Vitrine brisée',(128,52)),
        ('shutter','Rideau métallique forcé',(100,48)),
        ('roof-collapse','Bord de toiture abîmé',(128,96)),
        ('torn-fence','Clôture enfoncée',(140,64)),
        ('scorch','Traînée de brûlure',(160,112)),
        ('loot-trail','Traces de pillage',(128,80)),
        ('boards','Barricade disloquée',(128,56)),
    ]
    atlases=[('AFTERMATH-U01-01-camera-r02-source.png',[0,313,621,941,1254],[0,334,645,945,1254]),
             ('AFTERMATH-U01-02-source.png',[0,444,887,1330,1774],[0,444,887])]
    result={};index=0
    for file,xs,ys in atlases:
        source=Image.open(ART/file).convert('RGBA')
        for row in range(len(ys)-1):
            for col in range(4):
                name,title,size=definitions[index]
                sprite=source.crop((xs[col],ys[row],xs[col+1],ys[row+1]))
                box=sprite.getchannel('A').point(lambda a:255 if a>12 else 0).getbbox()
                assert box,name
                sprite=sprite.crop(box)
                sprite.thumbnail(size,Image.Resampling.LANCZOS)
                path=ART/(name+'-native.png');sprite.save(path)
                result[name]=dict(code=f'DECO-U01-{index+1:03}',title=title,file=rel(path),
                    size=list(sprite.size),source=file,source_box=[xs[col],ys[row],xs[col+1],ys[row+1]])
                index+=1
    war_source=Image.open(ART/'WAR-U01-GROUND-source.png').convert('RGBA')
    war=[('blast-crater','Impact dans la chaussée',(160,160),(0,0,444,480)),
         ('wall-fall','Pan de mur effondré',(192,144),(444,0,960,480)),
         ('car-wreck','Voiture détruite',(128,64),(960,0,1365,480)),
         ('van-wreck','Fourgon détruit',(128,64),(1365,0,1774,480)),
         ('position','Position détruite',(192,128),(0,480,462,887)),
         ('blast-ground','Sol ravagé',(192,160),(462,480,928,887)),
         ('fallen-light','Lampadaire abattu',(160,80),(928,480,1364,887)),
         ('war-barrier','Barricade détruite',(192,96),(1364,480,1774,887))]
    for i,(name,title,size,source_box) in enumerate(war):
        im=war_source.crop(source_box)
        im=im.crop(im.getchannel('A').point(lambda a:255 if a>12 else 0).getbbox())
        im.thumbnail(size,Image.Resampling.LANCZOS)
        path=ART/(name+'-native.png');im.save(path)
        result[name]=dict(code=f'WAR-U01-{i+1:02}',title=title,file=rel(path),size=list(im.size),source='WAR-U01-GROUND-source.png',source_box=source_box)
    # Upright furniture from the atlas is intentionally not used in the map:
    # the other low/fallen props match the established top-down camera better.
    result['furniture']['used']=False
    (ART/'decor-catalogue.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    reviewed=['barriers','fence','riot','soot','metal','rubble','roof-hole','window','shutter','roof-collapse',
              'torn-fence','scorch','boards',*(w[0] for w in war)]
    board=Image.new('RGB',(960,6*180+45),'#111b21');draw=ImageDraw.Draw(board)
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',13)
    title_font=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',19)
    draw.text((14,9),'U01 · dégâts et mobilier abandonné · exports natifs',font=title_font,fill='#dec898')
    for index,name in enumerate(reviewed):
        item=result[name]
        x=(index%4)*240;y=45+(index//4)*180
        if name=='furniture':continue
        draw.rectangle((x+5,y+5,x+234,y+174),outline='#35474f')
        draw.text((x+12,y+8),item['code'],font=font,fill='#dec898')
        draw.text((x+12,y+27),item['title'],font=font,fill='#d8dfdb')
        im=Image.open(ROOT/item['file']).convert('RGBA')
        board.paste(im,(x+120-im.width//2,y+104-im.height//2),im)
        draw.text((x+12,y+150),f'{im.width} × {im.height} px',font=font,fill='#9caaa9')
    board.save(OUT/'aftermath-native.png')
    return result


def buildings():
    source=Image.open(ART/'WAR-U01-BUILDINGS-source.png').convert('RGBA')
    definitions=[('habitat-a',(180,0,630,455),(256,320),(128,1088)),
        ('habitat-b',(750,0,1240,455),(256,320),(128,1600)),
        ('habitat-c',(1310,0,1774,455),(256,320),(832,192)),
        ('habitat-d',(0,455,835,887),(512,192),(448,576)),
        ('habitat-e',(836,455,1395,887),(448,256),(576,2208)),
        ('habitat-f',(1396,455,1774,887),(224,288),(1344,768))]
    result=[]
    for i,(name,box,size,at) in enumerate(definitions):
        im=source.crop(box)
        im=im.crop(im.getchannel('A').point(lambda a:255 if a>12 else 0).getbbox())
        im.thumbnail(size,Image.Resampling.LANCZOS)
        file=ART/(name+'-native.png');im.save(file)
        result.append(dict(code=f'BLDG-U01-{i+1:02}',file=rel(file),at_px=list(at),size=list(im.size),source_box=box))
    (ART/'new-buildings.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    board=Image.new('RGBA',(1280,1325),'#111b21');draw=ImageDraw.Draw(board)
    title=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',20)
    normal=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',15)
    draw.text((16,10),'U01 · six bâtiments supplémentaires · taille native',font=title,fill='#dec898')
    player=Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png').convert('RGBA')
    for i,b in enumerate(result):
        x=(i%2)*640;y=45+(i//2)*420
        draw.rectangle((x+6,y+4,x+633,y+413),outline='#35474f')
        draw.text((x+18,y+14),b['code'],font=title,fill='#dec898')
        im=Image.open(ROOT/b['file']).convert('RGBA')
        board.alpha_composite(im,(x+20,y+56))
        board.alpha_composite(player,(x+min(490,im.width+25),y+max(72,im.height-40)))
        draw.text((x+18,y+389),f'{im.width} × {im.height} px · personnage à la même échelle',font=normal,fill='#a6b6b5')
    board.convert('RGB').save(OUT/'buildings-native.png')
    return result


def build():
    ART.mkdir(parents=True,exist_ok=True);OUT.mkdir(parents=True,exist_ok=True)
    catalogue=assets()
    scene=json.loads((PREVIOUS/'street.json').read_text(encoding='utf-8'))
    image=Image.open(ROOT/scene['background']).convert('RGBA')
    w,h=scene['width'],scene['height']
    floor={(x,y) for y,row in enumerate((PREVIOUS/'layout.txt').read_text().splitlines()) for x,ch in enumerate(row) if ch=='.'}
    old_obstacles={tuple(p) for p in scene['obstructions']}
    blockers=set(old_obstacles)
    protected={tuple(scene['entrance']),tuple(scene['next_exit']),tuple(scene['home_inside'])}
    for r in scene['roofs']:
        if r['door']:
            x,y=r['door'];protected.update((x+dx,y+dy) for dx in (-1,0,1) for dy in (-1,0,1))
        protected.update(map(tuple,r['interior']))
    # Keep the through avenue and the southern entry usable. No prop covers a
    # working threshold or luminaire; neither light colors nor links change.
    protected.update((x,y) for x in (18,19) for y in range(1,51))
    protected.update((x,y) for x in range(5,10) for y in range(60,65))
    placements=[]
    new_buildings=buildings()
    config=scene['atmosphere']
    # One old lamp occupied the lot of the new eastern ruin. Restore its
    # underlying native paving and remove that emitter before adding the house.
    common_ground,_=unified.ground(Image.open(BASE/'street.png').convert('RGBA'))
    cleared=(1348,895,1382,1018)
    image.paste(common_ground.crop(cleared),cleared[:2])
    config['lights']=[l for l in config['lights'] if l['id']!='post-1']
    removed_by_buildings=set()
    for b in new_buildings:
        im=Image.open(ROOT/b['file']).convert('RGBA');x,y=b['at_px']
        image.alpha_composite(im,(x,y));alpha=im.getchannel('A')
        for gy in range(y//64,(y+im.height-1)//64+1):
            for gx in range(x//64,(x+im.width-1)//64+1):
                crop=np.asarray(alpha.crop((gx*64-x,gy*64-y,(gx+1)*64-x,(gy+1)*64-y)))
                if np.count_nonzero(crop>128)>=256:
                    assert (gx,gy) not in protected,(b['code'],(gx,gy))
                    removed_by_buildings.add((gx,gy));floor.discard((gx,gy))
        # Detect the actual pale lamp strips on the short front faces. Their
        # emission and red closed-door light are then calculated by the engine.
        a=np.asarray(im);mask=(a[:,:,0]>210)&(a[:,:,1]>180)&(a[:,:,3]>180)
        mask[:int(im.height*.65)]=False
        lamp_boxes=[]
        while mask.any():
            sy,sx=np.argwhere(mask)[0];todo=deque([(int(sx),int(sy))]);mask[sy,sx]=False;points=[]
            while todo:
                px,py=todo.popleft();points.append((px,py))
                for qx,qy in ((px-1,py),(px+1,py),(px,py-1),(px,py+1)):
                    if 0<=qx<im.width and 0<=qy<im.height and mask[qy,qx]:mask[qy,qx]=False;todo.append((qx,qy))
            xs,ys=zip(*points);left,right,top,bottom=min(xs),max(xs),min(ys),max(ys)
            if right-left>=8 and bottom-top<=8 and right-left>=3*(bottom-top+1):
                lamp_boxes.append([left,top,right,bottom])
        # A bright strip can be split into components by a dark pixel in its
        # frame. Merge these before creating one emitter per real light.
        merged=[]
        for box in sorted(lamp_boxes):
            for other in merged:
                if min(box[2],other[2])>=max(box[0],other[0]) and max(box[1],other[1])<=min(box[3],other[3])+6:
                    other[:]=[min(box[0],other[0]),min(box[1],other[1]),max(box[2],other[2]),max(box[3],other[3])];break
            else:merged.append(box)
        assert merged,('Missing closed-door emitter',b['code'])
        for left,top,right,bottom in merged:
            at=[x+(left+right)/2,y+(top+bottom)/2]
            config['lights'].append(dict(id=b['code']+'-'+str(left),at=at,origin=None,
                box=[x+left,y+top,x+right,y+bottom],radius=210,power=.8,color=[1.,.055,.025],lens_color=[1.,.055,.025]))

    def put(name,at,group,solid=False,roof=None):
        item=catalogue[name];im=Image.open(ROOT/item['file']).convert('RGBA')
        x,y=at
        layer=image if roof is None else roof[0]
        offset=(x,y) if roof is None else (x-roof[1][0],y-roof[1][1])
        assert offset[0]>=0 and offset[1]>=0 and offset[0]+im.width<=layer.width and offset[1]+im.height<=layer.height,(name,at)
        layer.alpha_composite(im,offset)
        occupied=[]
        if solid:
            alpha=im.getchannel('A')
            for gy in range(y//64,(y+im.height-1)//64+1):
                for gx in range(x//64,(x+im.width-1)//64+1):
                    crop=np.asarray(alpha.crop((gx*64-x,gy*64-y,(gx+1)*64-x,(gy+1)*64-y)))
                    if np.count_nonzero(crop>128)>=4096*.23 and (gx,gy) in floor and (gx,gy) not in protected:
                        occupied.append([gx,gy]);blockers.add((gx,gy))
        placements.append(dict(code=item['code'],asset=name,at_px=list(at),group=group,
            layer='roof' if roof else 'ground_or_facade',blocked_cells=occupied))

    # Structural damage and broken defensive positions dominate the scene.
    for name,at in [('roof-hole',(566,278)),('shutter',(392,390)),('window',(610,414)),
                    ('roof-hole',(210,360))]:put(name,at,'entrepot')
    for name,at,solid in [
        ('wall-fall',(830,516),True),('torn-fence',(288,740),False),('position',(130,690),True),
        ('war-barrier',(772,768),True),('riot',(1000,832),False),('blast-ground',(436,776),False),
        ('car-wreck',(664,885),True),('blast-crater',(930,964),True),('fallen-light',(382,1008),False),
        ('rubble',(1376,612),False),('soot',(1296,1090),False),
    ]:put(name,at,'cour-nord',solid)
    # Western homes form a residential strip instead of a broad empty court.
    for name,at,solid in [
        ('wall-fall',(144,1408),True),('blast-ground',(218,1456),False),
        ('window',(478,1434),False),('rubble',(1008,1328),False),('boards',(176,1538),False),
        ('riot',(936,1536),False),('soot',(1298,1734),False),('metal',(1454,1748),False),
    ]:put(name,at,'logement',solid)
    # The central street has wrecks, impacts and a shattered defensive position.
    for name,at,solid in [
        ('window',(486,1940),False),('wall-fall',(152,1908),True),('scorch',(626,2024),False),
        ('war-barrier',(176,2096),True),('blast-crater',(386,2120),True),
        ('van-wreck',(992,2076),True),('riot',(816,2160),False),('fallen-light',(1298,1900),False),
    ]:put(name,at,'boutique',solid)
    # Southern extension: heavy structural debris around the workshop and
    # another ruined dwelling, with the north-south through road kept clear.
    for name,at,solid in [
        ('shutter',(270,2618),False),('roof-hole',(1396,2320),False),
        ('blast-ground',(176,2264),False),('wall-fall',(930,2464),True),('rubble',(500,2560),False),
        ('metal',(576,2616),False),('blast-crater',(800,2728),True),('war-barrier',(380,2848),True),
        ('riot',(1024,2910),False),('scorch',(648,2960),False),('fallen-light',(176,2992),False),
        ('wall-fall',(1344,3040),False),('roof-collapse',(1388,2824),False),
    ]:put(name,at,'atelier',solid)
    # The original southern approach gains only local aftermath. Its geometry,
    # native gate artwork, door fixtures, car dimensions and return link stay.
    for name,at in [
        ('scorch',(308,3510)),('riot',(636,3694)),('boards',(1042,3720)),
        ('scorch',(896,3650)),('torn-fence',(214,3824)),('metal',(1030,3898)),
        ('soot',(1408,3890)),('rubble',(1370,3760)),('charred',(912,3596)),
    ]:put(name,at,'approche-prison')
    # Roof scars are separate layers and disappear with the corresponding roof.
    for index,roof in enumerate(scene['roofs']):
        roof_image=Image.open(ROOT/roof['file']).convert('RGBA')
        origin=roof['at_px'];x,y=origin
        if index==0:put('roof-collapse',(x+266,y+52),'toiture-depart',roof=(roof_image,origin))
        elif index==1:put('roof-hole',(x+42,y+40),'toiture-commerce',roof=(roof_image,origin))
        elif index==2:put('roof-collapse',(x+80,y+68),'toiture-logement',roof=(roof_image,origin))
        else:put('roof-hole',(x+230,y+74),'toiture-boutique',roof=(roof_image,origin))
        target=ART/f'roof-{index}-native.png';roof_image.save(target);roof['file']=rel(target)

    start=tuple(scene['entrance']);connected={start};pending=deque([start])
    while pending:
        x,y=pending.popleft()
        for p in ((x-1,y),(x+1,y),(x,y-1),(x,y+1)):
            if p in floor and p not in blockers and p not in connected:connected.add(p);pending.append(p)
    assert connected==floor-blockers,('Decoration severed a route',sorted(floor-blockers-connected),sorted(blockers-old_obstacles))
    assert not (blockers-old_obstacles)&protected
    assert all(tuple(r['door']) in connected for r in scene['roofs'] if r['door'])
    # The live entry gate is preserved byte for byte, although nearby traces
    # are added. All logical floor cells remain exactly those of r03.
    gate=(336,3956,656,4066)
    old_image=Image.open(ROOT/scene['background']).convert('RGBA')
    assert image.crop(gate).tobytes()==old_image.crop(gate).tobytes()
    image.save(ART/'street-candidate.png')
    layout='\n'.join(''.join('.' if (x,y) in floor else '#' for x in range(w)) for y in range(h))+'\n'
    (ART/'layout.txt').write_text(layout,encoding='utf-8')
    scene.update(id='MAP-U01-EXP-01-r04',visual_review='SCENE-U01-08-r04',lighting_review='LUM-U01-08-r04',
        background=rel(ART/'street-candidate.png'),obstructions=[list(p) for p in sorted(blockers)],
        walkable_cells=len(connected),background_sha256=sha(ART/'street-candidate.png'),layout_sha256=sha(ART/'layout.txt'))
    lighting.ART,lighting.W,lighting.H=ART,w,h
    for l in config['lights']:
        if l['origin'] is None or tuple(l['origin']) not in floor:
            l['origin']=min(connected,key=lambda p:(p[0]*64+32-l['at'][0])**2+(p[1]*64+32-l['at'][1])**2)
    lighting.build_lightmaps(image,floor,scene['atmosphere'],scene['atmosphere']['lights'])
    scene['atmosphere'].update(correction=rel(ART/'correction.png'),irradiance=rel(ART/'irradiance.png'))
    scene['review_views']=[(label,[17,8] if label=='north-yard' else
        list(min(connected,key=lambda p:(p[0]-at[0])**2+(p[1]-at[1])**2)) if label=='courtyard' else at)
        for label,at in scene['review_views']]
    for label,wanted in [('war-houses',(6,24)),('war-annex',(15,13)),('war-blast',(16,42))]:
        p=min(connected,key=lambda p:(p[0]-wanted[0])**2+(p[1]-wanted[1])**2)
        scene['review_views'].append([label,list(p)])
    for _,p in scene['review_views']+scene['review_animations']:assert tuple(p) in connected,p
    (ART/'street.json').write_text(json.dumps(scene,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    (ART/'decor-placements.json').write_text(json.dumps(placements,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    original=json.loads((DOC/'candidate-r03.json').read_text(encoding='utf-8'))
    active_hashes={p.name:sha(p) for p in BASE.iterdir() if p.is_file()}
    assert active_hashes==original['active_hashes']
    report=dict(review='MAP-U01-EXP-01-r04',status='candidate_not_integrated',dimensions=[w,h],native_cell=64,
        connected=True,walkable=len(connected),old_walkable=173,road_plan_unchanged=True,
        old_departure_preserved=True,active_map_unchanged=True,active_hashes=active_hashes,
        next_zone_not_generated=True,visitable_buildings=3,placements=len(placements),
        assets_used=len({p['code'] for p in placements}),new_obstructions=len(blockers-old_obstacles),
        additional_buildings=len(new_buildings),new_building_footprint_cells=len(removed_by_buildings),
        protected_accesses_clear=True,complete_map_panels_used=0,
        source_sha256={rel(p):sha(p) for p in ART.glob('*.png')})
    (DOC/'candidate-r04.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({k:v for k,v in report.items() if k not in ('source_sha256','active_hashes')},ensure_ascii=False))


if __name__=='__main__':build()
