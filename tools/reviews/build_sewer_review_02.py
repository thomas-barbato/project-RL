"""EG-DEC-02: enrich the approved sewer without modifying its source assets.

New drawings, additional footprints and support-cultist placements are reviewed
separately. This does not activate a dungeon transition or resurrection AI.
"""
from pathlib import Path
from collections import Counter
import copy
import json
import random
import shutil
from PIL import Image,ImageDraw
import build_sewer_review_01 as base

ROOT=base.ROOT
ART=ROOT/'assets/source-art/sewer-review-02'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/sewer-review-02'
OUT=ROOT/'artifacts/sewer-review-02'
SEEDS=base.SEEDS
read,write,rel,sha=base.read,base.write,base.rel,base.sha

def accepted_unchanged():
    agreement=read(base.DOC/'acceptance.json')
    for name,digest in agreement['accepted_assets_sha256'].items():
        assert sha(ROOT/name)==digest,('Accepted asset changed',name)
    assert base.hashes()==agreement['normal_street_sha256_at_acceptance']

def package():
    (ART/'kit').mkdir(parents=True,exist_ok=True)
    for key,row in read(DOC/'source-inputs.json').items():
        target=ART/'kit'/f'{key}-source.png'
        if not target.exists():shutil.copy2(row['source'],target)
        im=Image.open(target).convert('RGBA')
        box=im.getchannel('A').point(lambda a:255 if a>100 else 0).getbbox()
        assert box and box[0]>0 and box[1]>0 and box[2]<im.width and box[3]<im.height,(key,box)
        im=im.crop(box);im.thumbnail(tuple(row['fit']),Image.Resampling.LANCZOS)
        native=Image.new('RGBA',tuple(row['native']))
        native.alpha_composite(im,((native.width-im.width)//2,(native.height-im.height)//2))
        native.save(ART/'kit'/f'{key}-native.png')
    for key in ('trash','litter'):
        im=Image.open(ART/'kit'/f'{key}-native.png')
        for angle in (90,180,270):im.rotate(angle,expand=True).save(ART/'kit'/f'{key}-r{angle}-native.png')

def wall_routes(floor):
    raw=[];vertices={};offset=14
    for x,y in sorted(floor):
        sides=[((0,-1),((x*64,y*64),((x+1)*64,y*64))),
               ((1,0),(((x+1)*64,y*64),((x+1)*64,(y+1)*64))),
               ((0,1),((x*64,(y+1)*64),((x+1)*64,(y+1)*64))),
               ((-1,0),((x*64,y*64),(x*64,(y+1)*64)))]
        for normal,(a,b) in sides:
            if (x+normal[0],y+normal[1]) in floor:continue
            raw.append((a,b,normal))
            for v in (a,b):vertices.setdefault(v,[]).append(normal)
    assert all(len(v) in (2,4) for v in vertices.values())
    joints={};corners=[]
    for vertex,normals in vertices.items():
        for i,na in enumerate(normals):
            for nb in normals[i+1:]:
                if na[0]*nb[0]+na[1]*nb[1]!=0:continue
                centre=(vertex[0]+(na[0]+nb[0])*offset,vertex[1]+(na[1]+nb[1])*offset)
                if (centre[0]//64,centre[1]//64) in floor:continue
                for n in (na,nb):
                    assert (vertex,n) not in joints
                    joints[(vertex,n)]=centre
                corners.append(centre)
    edges=[];degree=Counter()
    for a,b,n in raw:
        # Extend outside corners and trim inside corners to one shared socket.
        # Extra little L segments would protrude into the walkable room.
        pa=joints.get((a,n),(a[0]+n[0]*offset,a[1]+n[1]*offset))
        pb=joints.get((b,n),(b[0]+n[0]*offset,b[1]+n[1]*offset))
        edges.append((pa,pb,n));degree.update((pa,pb))
        assert pa[0]==pb[0] or pa[1]==pb[1]
        for t in (0.,.25,.5,.75,1.):
            point=(int(pa[0]+(pb[0]-pa[0])*t),int(pa[1]+(pb[1]-pa[1])*t))
            assert (point[0]//64,point[1]//64) not in floor
    assert all(d==2 for d in degree.values()),'Unjoined wall pipe endpoint'
    return edges,corners

def finish_walls(image,plan,floor,wallcells):
    """Wall surface modules with shared straight/inside/outside sockets."""
    edges,corners=wall_routes(floor);d=ImageDraw.Draw(image);offset=14
    for centre in corners:
        cx,cy=centre
        # Complete junction block with the same downward front-face orientation
        # at all four corners; no rotating a north wall upside down.
        box=(cx-14,cy-14,cx+13,cy+13)
        d.rectangle(box,fill=(21,28,28,255))
        d.rectangle((cx-12,cy-12,cx+12,cy+12),fill=(86,94,86,255))
        d.rectangle((cx-11,cy+6,cx+11,cy+11),fill=(46,55,50,255))
        d.line((cx-11,cy-11,cx+11,cy-11),fill=(127,132,119,255),width=1)
        d.line((cx-11,cy-10,cx-11,cy+5),fill=(111,117,105,255),width=1)
        for dx in (-8,8):
            for dy in (-8,8):
                d.ellipse((cx+dx-1,cy+dy-1,cx+dx+1,cy+dy+1),fill=(29,36,34,255))
    def pipe(points):
        d.line(points,fill=(15,22,22,255),width=18,joint='curve')
        d.line(points,fill=(86,72,54,255),width=12,joint='curve')
        d.line([(x-1,y-1) for x,y in points],fill=(119,101,73,255),width=8,joint='curve')
        d.line([(x-3,y-3) for x,y in points],fill=(146,130,99,255),width=1,joint='curve')
    for a,b,_ in edges:pipe([a,b])
    for a,b,normal in edges:
        # Regular functional brackets; corrosion varies independently of rooms.
        if ((a[0]+a[1])//64)%2:continue
        cx,cy=(a[0]+b[0])//2,(a[1]+b[1])//2
        if a[1]==b[1]:
            d.rectangle((cx-3,cy-6,cx+3,cy+5),fill=(48,57,52,255))
            d.line((cx-2,cy-5,cx-2,cy+4),fill=(105,107,86,255),width=1)
        else:
            d.rectangle((cx-6,cy-3,cx+5,cy+3),fill=(48,57,52,255))
            d.line((cx-5,cy-2,cx+4,cy-2),fill=(105,107,86,255),width=1)
    for p in plan['props']:
        if p.get('kind')=='manifold':
            x,y=p['at_px'];main=y+58-offset
            for port in (y+32,y+55):pipe([(x-4,main),(x-4,port),(x+7,port)])
        elif p.get('kind')=='pump':
            x,y=p['at_px'];main=y+32-offset
            pipe([(x+107,main),(x+107,y+9)])
    plan['wall_module_checks']=dict(boundary_segments=len(edges),joined_corners=len(corners),
        corner_orientations_preserved=True,pipes_integrated_in_wall_surface=True,
        no_main_pipe_in_walkable_floor=True)

def decorate(seed):
    plan=copy.deepcopy(base.generate(seed));rng=random.Random(seed^0xDEC02)
    grid=plan['layout'];spawn=tuple(plan['spawn'])
    floor={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c!='#'}
    dry={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c in '.='}
    bridges={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c=='='}
    blocked=set(map(tuple,plan['blocked']));dry-=blocked
    original_members={tuple(m['at']) for g in plan['groups'] for m in g['members']}
    original_members|={tuple(v['at']) for v in plan['views']}
    protected={spawn,(23,36),(23,37)}|bridges|original_members
    for p in list(bridges):protected.update(base.neighbours(p))
    # Main room-to-room circulation stays open, as do complete narrow galleries.
    roomcells=[]
    for room in plan['rooms']:
        x0,y0,w,h=room['bounds']
        roomcells.append({(x,y) for y in range(y0,y0+h) for x in range(x0,x0+w) if (x,y) in floor})
        centre=min(dry,key=lambda p:abs(p[0]-room['centre'][0])+abs(p[1]-room['centre'][1]))
        protected.update(base.route(dry,spawn,centre))
    room_union=set().union(*roomcells)
    protected.update(floor-room_union)
    for p in list(original_members):protected.update(base.neighbours(p))
    for p in list(bridges):protected.update(base.neighbours(p))
    visual_occupied=set(blocked)
    added=[]
    def prop(key,at,feet=(),extra=None):
        entry=dict(file=rel(ART/'kit'/f'{key}-native.png'),at_px=list(at),
                   blocked_cells=[list(p) for p in sorted(feet)],review='EG-DEC-02',kind=key)
        if extra:entry.update(extra)
        plan['props'].append(entry);added.append(entry)
    for index,room in enumerate(plan['rooms']):
        x0,y0,w,h=room['bounds'];kind=room['kind'];area=roomcells[index]
        # Heavy machinery occupies an honest two-cell base, clear of all actors,
        # crossings and protected paths. Its upper silhouette is also reserved.
        key='pump' if kind in ('collector','basin') else 'workbench' if kind in ('maintenance','alcove') else None
        if key:
            options=[(x,y0) for x in range(x0,x0+w-1)]
            if key!='pump':options += [(x,y0+h-1) for x in range(x0,x0+w-1)]
            rng.shuffle(options)
            for x,y in options:
                feet={(x,y),(x+1,y)};whole=feet|{(x,y-1),(x+1,y-1)}
                if not feet<=dry or whole&(protected|visual_occupied):continue
                # Real room boundary, not an arbitrary margin inside the room.
                if not all((px,py-1) not in floor for px,py in feet) and not all((px,py+1) not in floor for px,py in feet):continue
                if len(base.flood(dry-feet,spawn))!=len(dry)-2:continue
                prop(key,(x*64,y*64-(32 if key=='pump' else 16)),feet,extra={'against_wall':True})
                blocked|=feet;dry-=feet;visual_occupied|=whole;break
        # A wall fixture is attached to a complete existing north-wall run.
        row=y0-1;available=[x for x in range(x0,x0+w) if (x,row) not in floor and (x,y0) in floor]
        runs=[]
        for x in available:
            if not runs or x!=runs[-1][-1]+1:runs.append([x])
            else:runs[-1].append(x)
        if runs:
            run=max(runs,key=len);wet=kind in ('collector','basin')
            minimum=5 if wet else 3
            if len(run)>=minimum:
                x=run[0]+(2 if wet else 0)
                prop('manifold',(x*64+8,y0*64-58),extra={'wall_attached':True})
        # Small discarded material belongs to wall edges and maintenance recesses,
        # not a repetitive scatter across every tile or an invisible obstacle.
        candidates=[p for p in area&dry if p not in protected|visual_occupied
                    and any(n not in floor for n in base.neighbours(p))]
        rng.shuffle(candidates);placed=[]
        for x,y in candidates:
            if len(placed)>=rng.randint(3,5):break
            if any(abs(x-a)+abs(y-b)<2 for a,b in placed):continue
            angle=(0,90,180,270)[len(placed)%4]
            family='trash' if len(placed)%2==0 else 'litter'
            key=family if angle==0 else f'{family}-r{angle}'
            # All rotations use whole floor-level sprites and retain native scale.
            dims=(72,48) if family=='trash' else (48,32)
            fit=(64,38) if family=='trash' else (40,26)
            if angle in (90,270):dims=dims[::-1];fit=fit[::-1]
            jx=max(0,(64-fit[0])//2-2);jy=max(0,(64-fit[1])//2-2)
            at=(x*64+(64-dims[0])//2+rng.randint(-jx,jx),y*64+(64-dims[1])//2+rng.randint(-jy,jy))
            prop(key,at,extra={'walkable_low_debris':True});placed.append((x,y))
        # Existing small material assets vary the occupied service corners.
        if kind in ('maintenance','junction','basin','access'):
            positions=[p for p in area&dry if p not in protected|visual_occupied
                       and sum(n in dry for n in base.neighbours(p))>=2]
            positions.sort(key=lambda p:min(p[0]-x0,x0+w-1-p[0],p[1]-y0,y0+h-1-p[1]))
            for p in positions:
                if len(base.flood(dry-{p},spawn))!=len(dry)-1:continue
                file='assets/source-art/first-sector-review-01/u01-expansion-r06/barrel-native.png'
                entry=dict(file=file,at_px=[p[0]*64+20,p[1]*64+28],blocked_cells=[list(p)],
                           review='EG-DEC-02',kind='old-barrel')
                plan['props'].append(entry);added.append(entry);blocked.add(p);dry.remove(p);visual_occupied.add(p);break
    reached=base.flood(dry,spawn);assert len(reached)==len(dry)
    # Support placement is a proposal. The capture still uses existing combat
    # profiles; no raising ability, reward duplication or new AI runs here.
    occupied=original_members|blocked
    support_groups=[g for g in plan['groups'] if g['family']=='zombie']
    random.Random(seed^0xA13).shuffle(support_groups)
    for group in support_groups[:2]:
        cx,cy=group['centre']
        candidates=[p for p in dry if p not in occupied|bridges|protected
                    and 2<=abs(p[0]-cx)+abs(p[1]-cy)<=4 and reached[p]>=8
                    and max(abs(p[0]-spawn[0]),abs(p[1]-spawn[1]))>6
                    and sum(n in dry for n in base.neighbours(p))>=2]
        if not candidates:
            candidates=[p for p in dry if p not in occupied|bridges and 2<=abs(p[0]-cx)+abs(p[1]-cy)<=4
                        and max(abs(p[0]-spawn[0]),abs(p[1]-spawn[1]))>6]
        assert candidates,('No support position',seed,group['id'])
        p=max(sorted(candidates),key=lambda p:reached[p]);occupied.add(p)
        group['members'].append(dict(at=list(p),number=132,variant=0,level=1,role='reanimator_candidate'))
    for rat in plan['rats']:
        points=[(int((p[0]-24)/64),int((p[1]-40)/64)) for p in rat['path']]
        if not all(p in dry for p in points):
            valid=set(points)&dry
            if len(valid)>=2:
                a=min(valid);dist=base.flood(valid,a);b=max(dist,key=dist.get)
                path=base.route(valid,a,b)
                rat['path']=[[x*64+24,y*64+40] for x,y in path]
    plan['rats']=[r for r in plan['rats'] if len(r['path'])>=2]
    casters=[m for g in plan['groups'] for m in g['members'] if m.get('role')=='reanimator_candidate']
    if casters:
        caster=casters[0]
        view=min(dry-occupied,key=lambda p:abs(abs(p[0]-caster['at'][0])+abs(p[1]-caster['at'][1])-3))
        plan['views'].append(dict(id='reanimator',at=list(view)))
    def maintenance_room(prop):
        px,py=prop['blocked_cells'][0]
        return next((r for r in plan['rooms'] if r['kind']=='maintenance'
                     and r['bounds'][0]<=px<r['bounds'][0]+r['bounds'][2]
                     and r['bounds'][1]<=py<r['bounds'][1]+r['bounds'][3]),None)
    benches=[p for p in added if p['kind']=='workbench' and maintenance_room(p)]
    if benches:
        centre=maintenance_room(benches[0])['centre']
        view=min(dry-occupied,key=lambda p:(abs(p[0]-centre[0])+abs(p[1]-centre[1]),reached[p]))
        plan['views'].append(dict(id='maintenance',at=list(view)))
    corner_tiles=[p for p in dry-occupied if any((p[0]+dx,p[1]) not in floor and (p[0],p[1]+dy) not in floor
                                                for dx in (-1,1) for dy in (-1,1))]
    if corner_tiles:
        corner=min(corner_tiles,key=lambda p:abs(p[0]-spawn[0])+abs(p[1]-spawn[1]))
        nearby=base.flood(dry,corner)
        view=min(dry-occupied,key=lambda p:(abs(nearby.get(p,999)-2),reached[p]))
        plan['views'].append(dict(id='corners',at=list(view)))
    plan.update(review='EG-DEC-02',blocked=[list(p) for p in sorted(blocked)],floor_count=len(dry),
                added_scenery=added,roster=[22,24,120,121,132],resurrection_ai_active=False,
                integrated_wall_pipes=True)
    # No base actor, water cell, room, corridor or tile changes were needed.
    assert plan['layout']==base.generate(seed)['layout']
    assert all(tuple(m['at']) in dry for g in plan['groups'] for m in g['members'])
    assert len({tuple(m['at']) for g in plan['groups'] for m in g['members']})==sum(len(g['members']) for g in plan['groups'])
    return plan

def build():
    accepted_unchanged();package();OUT.mkdir(parents=True,exist_ok=True)
    # Shared, accepted materials are referenced directly, never copied/repainted.
    (ART/'entry').mkdir(exist_ok=True)
    write(ART/'entry/street.json',read(base.ART/'entry/street.json'))
    for i in (1,2):write(ART/f'street-population-{i}.json',read(base.ART/f'street-population-{i}.json'))
    for seed in range(300):
        plan=decorate(seed)
        assert plan==decorate(seed),('Not deterministic',seed)
        wall_routes({(x,y) for y,row in enumerate(plan['layout']) for x,c in enumerate(row) if c!='#'})
    counts=[]
    for i,seed in enumerate(SEEDS,1):
        p=decorate(seed);base.render(p,ART/f'seed-{i}',wall_finish=finish_walls)
        counts.append(dict(seed=seed,added_scenery=len(p['added_scenery']),
            heavy_fixtures=sum(bool(p['blocked_cells']) for p in p['added_scenery']),
            support_cultists=sum(m['number']==132 for g in p['groups'] for m in g['members']),
            monsters=sum(len(g['members']) for g in p['groups']),walkable=p['floor_count'],wall_module_checks=p['wall_module_checks']))
    write(DOC/'proposal.json',dict(review='EG-DEC-02',status='awaiting_new_decoration_and_support_review',
        approved_base=rel(base.DOC/'acceptance.json'),approved_sources_unchanged=True,
        dimensions=[47,44],native_cell=64,normal_campaign_activation=False,
        seeds=counts,new_drawings=list(read(DOC/'source-inputs.json')),generation_audit_seeds=300,
        requested_wall_refinement='Pipes integrated in wall scenery, including inside and outside corners',
        accepted_layouts_and_existing_monster_positions_preserved=True,
        normal_street_sha256=base.hashes(),resurrection_rules=read(DOC/'resurrection-proposal.json')))
    accepted_unchanged();print(json.dumps(counts))

if __name__=='__main__':build()
