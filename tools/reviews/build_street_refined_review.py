"""U01 r05: reconstruct the audited scene from isolated native modules.

Normal game data and previous candidate sources are never overwritten.
"""
from collections import deque
import copy
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont
import build_street_unified_review as unified
import build_street_expansion_review as lighting

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'assets/maps/prison-street-01'
OLD=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r03'
WAR=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r04'
ART=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r05'
OUT=ROOT/'artifacts/street-expansion-review-01'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-expansion-review-01'
W,H=26,66
SOUTH_EDGE=4064


def rel(p):return p.relative_to(ROOT).as_posix()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_text(encoding='utf-8'))
def write(p,v):p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')


def isolated(source,size,*,bottom=True):
    """Condition one complete generated object, preserving its proportions."""
    im=Image.open(source).convert('RGBA')
    alpha=im.getchannel('A')
    # Ignore almost invisible generation noise when measuring the object.
    # The original source is kept intact; exports contain no stray alpha specks.
    bb=alpha.point(lambda a:255 if a>128 else 0).getbbox()
    assert bb and bb[0]>0 and bb[1]>0 and bb[2]<im.width and bb[3]<im.height,source
    im=im.crop(bb)
    im.putalpha(im.getchannel('A').point(lambda a:0 if a<40 else a))
    factor=min(size[0]/im.width,size[1]/im.height)
    im=im.resize((round(im.width*factor),round(im.height*factor)),Image.Resampling.LANCZOS)
    out=Image.new('RGBA',size)
    out.alpha_composite(im,((size[0]-im.width)//2,size[1]-im.height if bottom else 0))
    return out


def export_sources():
    inputs=read(ART/'module-input.json')
    exports={}
    for r in inputs:
        size=tuple(r['source_size'])
        if r['id']=='court-bin':size=(138,110)
        im=isolated(ART/(r['id']+'-source.png'),size)
        p=ART/(r['id']+'-native.png');im.save(p);exports[r['id']]=p
    warehouse=Image.open(OLD/'warehouse.png').convert('RGBA')
    source=ART/'WAREHOUSE-U01-FIX-source.png'
    im=isolated(source,warehouse.size)
    im.save(ART/'warehouse-native.png')
    exports['warehouse']=ART/'warehouse-native.png'
    for key,size in [('collapsed-wall',(208,127)),('fallen-light',(160,54)),
                     ('north-gateway',(258,151)),('street-post',(26,112))]:
        p=ART/(key+'-native.png');isolated(ART/(key+'-source.png'),size).save(p);exports[key]=p
    roofs=read(OLD/'street.json')['roofs']
    for i,r in enumerate(roofs):
        original=Image.open(ROOT/r['file']).convert('RGBA')
        im=isolated(ART/f'roof-{i}-source.png',original.size)
        p=ART/f'roof-{i}-native.png';im.save(p)
    return exports


def lamp_strips(im):
    a=np.asarray(im)
    mask=(a[:,:,0]>210)&(a[:,:,1]>180)&(a[:,:,2]<230)&((a[:,:,0].astype(int)-a[:,:,2])>20)&(a[:,:,3]>180)
    mask[:int(im.height*.65)]=False
    boxes=[]
    while mask.any():
        sy,sx=np.argwhere(mask)[0];todo=deque([(int(sx),int(sy))]);mask[sy,sx]=False;points=[]
        while todo:
            x,y=todo.popleft();points.append((x,y))
            for xx,yy in ((x-1,y),(x+1,y),(x,y-1),(x,y+1)):
                if 0<=xx<im.width and 0<=yy<im.height and mask[yy,xx]:mask[yy,xx]=False;todo.append((xx,yy))
        xs,ys=zip(*points);l,t,r,b=min(xs),min(ys),max(xs),max(ys)
        if r-l>=8 and b-t<=8 and r-l>=3*(b-t+1):
            for old in boxes:
                if min(r,old[2])>=max(l,old[0]) and max(t,old[1])<=min(b,old[3])+6:
                    old[:]=[min(l,old[0]),min(t,old[1]),max(r,old[2]),max(b,old[3])];break
            else:boxes.append([l,t,r,b])
    return boxes


def build(exports=None, revision='r05', extra_props=()):
    ART.mkdir(exist_ok=True);OUT.mkdir(exist_ok=True)
    if exports is None:exports=export_sources()
    scene=read(WAR/'street.json');prior=copy.deepcopy(scene)
    config=scene['atmosphere'];plan=read(OLD/'layout-plan.json')
    original=Image.open(BASE/'street.png').convert('RGBA')
    ground,road=unified.ground(original)
    image=Image.new('RGBA',(W*64,H*64),(0,0,0,255))
    image.paste(ground.crop((64,64,1632,SOUTH_EDGE)),(64,64))
    image.paste(ground.crop((1132,0,1292,64)),(1132,0))
    footprint=Image.new('L',image.size)
    low_shapes=[];module_records=[]
    old_ids={'east-small-door','east-long-door','east-blue-door','west-garage-door','east-south-door'}
    removed_lights={'post-2','post-4'}
    config['lights']=[l for l in config['lights'] if l['id'] not in old_ids|removed_lights]

    def module(file,at,solid_wall=True):
        im=Image.open(file).convert('RGBA');image.alpha_composite(im,at)
        if solid_wall:footprint.paste(255,at,im.getchannel('A').point(lambda a:255 if a>128 else 0))
        else:low_shapes.append((im,at))
        return im

    for m in plan['modules']:
        ident=m['id'];file=exports.get(ident,ROOT/m['file'])
        if ident in ('hedge-0','hedge-2'):continue
        if ident.startswith('hedge-'):file=exports['hedge-0']
        at=(624,2656) if ident=='court-bin' else tuple(m['at'])
        solid=ident not in ('court-machines','court-bin','collapsed-wall') and not ident.startswith('hedge-')
        im=module(file,at,solid)
        module_records.append(dict(id=ident,file=rel(file),at=list(at)))
        if ident in ('east-small','east-long','east-blue','west-garage','east-south'):
            boxes=lamp_strips(im);assert boxes,ident
            expected=next(l['at'] for l in prior['atmosphere']['lights'] if l['id']==ident+'-door')
            boxes=[min(boxes,key=lambda b:(at[0]+(b[0]+b[2])/2-expected[0])**2+(at[1]+(b[1]+b[3])/2-expected[1])**2)]
            for i,(l,t,r,b) in enumerate(boxes):
                config['lights'].append(dict(id=ident+'-door' if i==0 else ident+'-door-'+str(i),
                    at=[at[0]+(l+r)/2,at[1]+(t+b)/2],origin=None,box=[at[0]+l,at[1]+t,at[0]+r,at[1]+b],
                    radius=105,power=.28,color=[1.,.055,.025],lens_color=[1.,.055,.025]))
    for prop in extra_props:
        module(ROOT/prop['file'],tuple(prop['at']),False)
        module_records.append(prop)
    # Shrubs now have an actual leaf silhouette; no ellipse of old background.
    for x in (616,800,960):module(exports['hedge-0'],(x,3090),False)
    mask=Image.new('L',original.size,255);md=ImageDraw.Draw(mask)
    md.rectangle((0,0,41,127),fill=0);md.rectangle((42,0,239,59),fill=0);md.rectangle((240,0,515,23),fill=0)
    accepted=original.copy();accepted.putalpha(mask);image.alpha_composite(accepted,(96,3136))
    # Houses with exposed rooms are condemned by their visible rubble/walls,
    # not by an undrawn transparent rectangle around their native sprite.
    new_buildings=read(WAR/'new-buildings.json')
    for b in new_buildings:module(ROOT/b['file'],tuple(b['at_px']))

    walls=Image.open(ROOT/'assets/maps/cryogenic-camp-01/structure.png').convert('RGBA')
    for y in range(64,4000,64):
        image.alpha_composite(walls.crop((868,192,900,256)),(64,y))
        image.alpha_composite(walls.crop((1344,192,1376,256)),(1600,y))
    for x in range(64,1600,64):
        if x not in (1152,1216):image.alpha_composite(walls.crop((960,88,1024,134)),(x,64))
    corner_sources={'nw':(868,88,932,152),'ne':(1312,88,1376,152),'sw':(868,960,932,1024),'se':(1312,960,1376,1024)}
    corners={}
    for name,at in [('nw',(64,64)),('ne',(1568,64)),('sw',(64,4000)),('se',(1568,4000))]:
        im=walls.crop(corner_sources[name]);p=ART/('wall-corner-'+name+'.png');im.save(p);image.alpha_composite(im,at)
        corners[name]=dict(file=rel(p),at=list(at))
    gate_at=(1078,0)
    gate=Image.open(exports['north-gateway']).convert('RGBA')
    image.alpha_composite(gate,gate_at)
    # Optical locations are measured on each complete standalone asset.
    # No floor or neighboring pixels are taken from an illustrated map.
    for ident,box in [('gate-west',(0,0,129,151)),('gate-east',(129,0,258,151))]:
        a=np.asarray(gate.crop(box));mask=(a[:,:,0]>225)&(a[:,:,1]>210)&(a[:,:,2]>150)&(a[:,:,2]<245)&(a[:,:,3]>180)
        ys,xs=np.nonzero(mask);assert len(xs),ident
        l,r=int(xs.min()+box[0]),int(xs.max()+box[0]);t,b=int(ys.min()),int(ys.max())
        light=next(lamp for lamp in config['lights'] if lamp['id']==ident)
        light.update(at=[gate_at[0]+(l+r)/2,(t+b)/2],box=[gate_at[0]+l,t,gate_at[0]+r,b],origin=None)
    # Two redundant street fixtures are physically removed, rather than only
    # switching off their hidden lights. The covered old post-1 stays removed.
    for i,(x,y) in enumerate([(1040,680),(1350,900),(1040,1536),(1350,1984),(1040,2560),(1350,3008)]):
        if i in (1,2,4):continue
        im=Image.open(exports['street-post']).convert('RGBA');image.alpha_composite(im,(x,y))
        light=next(lamp for lamp in config['lights'] if lamp['id']=='post-'+str(i))
        # The lens occupies this measured area in the native conditioned pole.
        light.update(at=[x+13,y+31],box=[x+10,y+24,x+16,y+38],origin=None)

    floor=set()
    for y in range(2,49):
        for x in range(2,25):
            a=np.asarray(footprint.crop((x*64,y*64,(x+1)*64,(y+1)*64)))
            if np.count_nonzero(a)>=96:continue
            floor.add((x,y))
    floor.update((x,y+48) for y,row in enumerate((BASE/'layout.txt').read_text().splitlines()) for x,c in enumerate(row) if c=='.' and y+48<=62)
    for r in scene['roofs']:
        if r['door'] and r['door'][1]<48:
            floor.update(map(tuple,r['interior']));x,y=r['door'];floor.update(((x,y),(x,y-1),(x,y+1)))
    floor.update(((18,1),(19,1)))
    blockers={tuple(p) for p in read(OLD/'street.json')['obstructions'] if p[1]<=62}
    protected={tuple(scene['entrance']),tuple(scene['next_exit'])}
    for r in scene['roofs']:
        protected.update(map(tuple,r['interior']))
        if r['door']:
            x,y=r['door'];protected.update((x+dx,y+dy) for dx in (-1,0,1) for dy in (-1,0,1))
    protected.update((x,y) for x in (18,19) for y in range(1,51))

    def obstruction(im,at):
        x,y=at;out=[];a=im.getchannel('A')
        for gy in range(y//64,(y+im.height-1)//64+1):
            for gx in range(x//64,(x+im.width-1)//64+1):
                crop=np.asarray(a.crop((gx*64-x,gy*64-y,(gx+1)*64-x,(gy+1)*64-y)))
                if np.count_nonzero(crop>128)>=4096*.23 and (gx,gy) in floor and (gx,gy) not in protected:
                    blockers.add((gx,gy));out.append([gx,gy])
        return out
    for im,at in low_shapes:obstruction(im,at)
    catalogue=read(WAR/'decor-catalogue.json');placements=[]
    catalogue['fallen-light']['file']=rel(exports['fallen-light'])
    write(ART/'decor-catalogue.json',catalogue)
    # Integrated roof images replace all generic pasted windows/shutters/slabs.
    skip={'window','shutter','roof-hole','roof-collapse'}
    for p in read(WAR/'decor-placements.json'):
        if p['asset'] in skip or p['layer']=='roof' or p['at_px']==[1344,3040]:continue
        p=copy.deepcopy(p);im=Image.open(ROOT/catalogue[p['asset']]['file']).convert('RGBA')
        # Pull the workshop debris off the former post location, and keep
        # damage concentrated next to a genuinely ruined building.
        if p['at_px']==[930,2464]:p['at_px']=[944,2448]
        if p['asset']=='position' and p['at_px']==[130,690]:p['at_px']=[160,832]
        image.alpha_composite(im,tuple(p['at_px']))
        p['blocked_cells']=obstruction(im,tuple(p['at_px'])) if p['blocked_cells'] else []
        placements.append(p)
    for i,r in enumerate(scene['roofs']):r['file']=rel(exports.get(f'roof-{i}',ART/f'roof-{i}-native.png'))
    # No painted road, sidewalk or second wall behind the prison. This leaves
    # only the front foundation of the existing portal at the scene boundary.
    ImageDraw.Draw(image).rectangle((0,SOUTH_EDGE,W*64,H*64),fill=(0,0,0,255))
    assert not any(y>62 for x,y in floor)
    connected={tuple(scene['entrance'])};todo=deque(connected)
    while todo:
        x,y=todo.popleft()
        for p in ((x-1,y),(x+1,y),(x,y-1),(x,y+1)):
            if p in floor and p not in blockers and p not in connected:connected.add(p);todo.append(p)
    assert connected==floor-blockers,('Disconnected refined floor',sorted(floor-blockers-connected))
    assert all(tuple(r['door']) in connected for r in scene['roofs'] if r['door'])
    assert all(tuple(r['interior'][-1]) in connected for r in scene['roofs'] if r['door'])

    off={'post-0','post-5','west','north-east-upper','garage-west','gate-east'}
    for l in config['lights']:
        if l['id'] in off:
            l.update(power=0.,lens_color=[.065,.07,.065],state='off')
        else:
            l['state']='on'
            # Preserve the marker color while reducing large red light pools.
            if l['color']==[1.,.055,.025]:l.update(radius=105,power=.28)
        if l['origin'] is None or tuple(l['origin']) not in connected:
            l['origin']=min(connected,key=lambda p:(p[0]*64+32-l['at'][0])**2+(p[1]*64+32-l['at'][1])**2)
    image.save(ART/'street-candidate.png')
    layout='\n'.join(''.join('.' if (x,y) in floor else '#' for x in range(W)) for y in range(H))+'\n'
    (ART/'layout.txt').write_text(layout,encoding='utf-8')
    lighting.ART,lighting.W,lighting.H=ART,W,H
    lighting.build_lightmaps(image,floor,config,config['lights'])
    # Dark, non-emissive source lenses receive their engine emission mask.
    # This is technical shader data, not a painted halo in the scenery sprite.
    correction=Image.open(ART/'correction.png').convert('RGBA');cd=ImageDraw.Draw(correction)
    for lamp in config['lights']:
        if lamp['id'].startswith('post-') and lamp['box']:
            l,t,r,b=lamp['box'];tint=tuple(round(v*255) for v in lamp['lens_color'])+(255,)
            cd.rounded_rectangle((l+1,t+1,r-1,b-1),radius=2,fill=tint)
    correction.save(ART/'correction.png')
    config.update(correction=rel(ART/'correction.png'),irradiance=rel(ART/'irradiance.png'))
    scene.update(id='MAP-U01-EXP-01-'+revision,background=rel(ART/'street-candidate.png'),visual_review='SCENE-U01-08-'+revision,lighting_review='LUM-U01-08-'+revision,
        obstructions=[list(p) for p in sorted(blockers)],walkable_cells=len(connected),south_visible_edge=SOUTH_EDGE,
        background_sha256=sha(ART/'street-candidate.png'),layout_sha256=sha(ART/'layout.txt'))
    for views in (scene['review_views'],scene['review_animations']):
        for r in views:
            if tuple(r[1]) not in connected and not r[0].startswith('house-'):
                wanted=r[1];r[1]=list(min(connected,key=lambda p:(p[0]-wanted[0])**2+(p[1]-wanted[1])**2))
            assert tuple(r[1]) in connected,r
    for label,at in [('lighting-off',[17,11]),('court-clean',[15,41]),('west-corner',[2,2]),('east-corner',[23,2])]:
        p=min(connected,key=lambda p:(p[0]-at[0])**2+(p[1]-at[1])**2);scene['review_views'].append([label,list(p)])
    # Detect unintended physical overlaps with every remaining optic, including
    # extinguished fixtures; no cover may be mistaken for an emissive lens.
    overlaps=[]
    for p in placements:
        im=Image.open(ROOT/catalogue[p['asset']]['file']).getchannel('A');x,y=p['at_px']
        for l in config['lights']:
            if not l['box']:continue
            a,b,c,d=map(int,l['box']);left,top,right,bottom=max(a,x),max(b,y),min(c+1,x+im.width),min(d+1,y+im.height)
            if right>left and bottom>top and np.count_nonzero(np.asarray(im.crop((left-x,top-y,right-x,bottom-y)))>128):overlaps.append((p['asset'],l['id']))
    assert not overlaps,overlaps
    write(ART/'street.json',scene);write(ART/'decor-placements.json',placements);write(ART/'module-placements.json',module_records)
    write(ART/'lighting-states.json',[dict(id=l['id'],state=l['state'],at=l['at'],power=l['power']) for l in config['lights']])
    prior_report=read(DOC/'candidate-r04.json');current={p.name:sha(p) for p in BASE.iterdir() if p.is_file()}
    assert current==prior_report['active_hashes']
    report=dict(review='MAP-U01-EXP-01-'+revision,status='candidate_not_integrated',native_cell=64,dimensions=[W,H],walkable=len(connected),
        connected=True,prison_back_floor_cells=0,south_visible_edge=SOUTH_EDGE,corners=corners,
        removed_fixtures=sorted(removed_lights),off_lights=sorted(off),active_lights=sum(l['power']>0 for l in config['lights']),
        total_lights=len(config['lights']),optic_overlaps=overlaps,additional_buildings=6,visitable_buildings=3,
        active_map_unchanged=True,active_hashes=current,road_plan_unchanged=True,next_zone_not_generated=True,
        source_sha256={rel(p):sha(p) for p in ART.glob('*.png')})
    write(DOC/('candidate-'+revision+'.json'),report)
    print(json.dumps({k:v for k,v in report.items() if k not in ('source_sha256','active_hashes','corners')},ensure_ascii=False))


if __name__=='__main__':build()
