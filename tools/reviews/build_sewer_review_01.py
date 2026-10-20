"""EG01: review-only modular sewer generation and a reversible street proposal.

No active map, balance rule or save is written. Material textures are tiled;
isolated objects are fitted whole. Geometry supplies connected wall corners.
"""
from collections import deque
from pathlib import Path
import copy
import hashlib
import json
import math
import random
import sys
import numpy as np
from PIL import Image, ImageChops, ImageDraw, ImageEnhance
import build_street_paving_review as paving
import build_street_expansion_review as lighting
import build_street_repairs_09 as repairs

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/sewer-review-01'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/sewer-review-01'
OUT=ROOT/'artifacts/sewer-review-01'
ACTIVE=ROOT/'assets/maps/prison-street-expanded-01'
W,H=47,44
CELL=64
DIRS=((0,-1),(1,0),(0,1),(-1,0))
SEEDS=(206014,206027,206053)

def read(p):return json.loads(p.read_text(encoding='utf-8-sig'))
def rel(p):return p.relative_to(ROOT).as_posix()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,v):
    p.parent.mkdir(parents=True,exist_ok=True)
    p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
def hashes():return {rel(p):sha(p) for p in ACTIVE.rglob('*') if p.is_file()}
def neighbours(p):return [(p[0]+dx,p[1]+dy) for dx,dy in DIRS]
def flood(cells,start):
    result={start:0};todo=deque([start])
    while todo:
        p=todo.popleft()
        for q in neighbours(p):
            if q in cells and q not in result:result[q]=result[p]+1;todo.append(q)
    return result

def route(cells,start,end):
    parents={start:None};todo=deque([start])
    while todo and end not in parents:
        p=todo.popleft()
        for q in neighbours(p):
            if q in cells and q not in parents:parents[q]=p;todo.append(q)
    if end not in parents:return []
    result=[];p=end
    while p is not None:result.append(p);p=parents[p]
    return result[::-1]

def generate(seed):
    """Attach compatible rooms, then vary presets and spatial group attempts."""
    rng=random.Random(seed)
    start=(1,3);nodes=[start];edges=[]
    while len(nodes)<10:
        choices=[(a,b) for a in nodes for b in neighbours(a)
                 if 0<=b[0]<3 and 0<=b[1]<4 and b not in nodes]
        a,b=rng.choice(choices);nodes.append(b);edges.append((a,b))
    extras=[(a,b) for a in nodes for b in neighbours(a) if b in nodes and a<b
            and (a,b) not in edges and (b,a) not in edges]
    rng.shuffle(extras);edges.extend(extras[:2])
    centres={n:(8+15*n[0]+rng.randint(-1,1),6+10*n[1]+rng.randint(-1,1)) for n in nodes}
    centres[start]=(23,36)
    floor=set();rooms=[];water=set();bridges=set();corridors=[]
    kinds=['collector','basin','junction','maintenance','collector','alcove','basin','maintenance','junction']
    rng.shuffle(kinds)
    for i,n in enumerate(nodes):
        cx,cy=centres[n];kind='access' if n==start else kinds.pop()
        rw,rh=(9,7) if kind=='access' else (rng.randint(8,11),rng.randint(6,8))
        x0,y0=cx-rw//2,cy-rh//2
        cells={(x,y) for y in range(y0,y0+rh) for x in range(x0,x0+rw)}
        if kind=='alcove':
            cells-={(x,y) for x,y in cells if x<x0+2 and y<y0+2}
        floor|=cells
        rooms.append(dict(id=i,kind=kind,centre=[cx,cy],bounds=[x0,y0,rw,rh],cells=cells))
        if kind in ('collector','basin'):
            # Two walkable banks and a complete cross-bridge, not a fake gap.
            ww=2 if kind=='collector' else 3
            pool={(x,y) for y in range(y0+2,y0+rh-2) for x in range(cx-1,cx-1+ww)}
            water|=pool
            bridges|={p for p in pool if p[1]==cy}
    for a,b in edges:
        ax,ay=centres[a];bx,by=centres[b]
        horizontal=a[1]==b[1]
        mid=(bx,ay) if horizontal else (ax,by)
        line=[]
        for (x0,y0),(x1,y1) in [((ax,ay),mid),(mid,(bx,by))]:
            if x0==x1:line.extend((x0,y) for y in range(min(y0,y1),max(y0,y1)+1))
            else:line.extend((x,y0) for x in range(min(x0,x1),max(x0,x1)+1))
        wide=rng.random()<.6
        radius=2 if wide else 1
        for x,y in line:
            floor|={(x+dx,y+dy) for dx in range(-radius,radius+1) for dy in range(-radius,radius+1)}
        # Main collectors carry sewage between chambers; branch galleries stay dry.
        if wide:
            for j,p in enumerate(line):
                if min(abs(p[0]-ax)+abs(p[1]-ay),abs(p[0]-bx)+abs(p[1]-by))>=4:
                    water.add(p)
                    if j%7==0:bridges.add(p)
        corridors.append(dict(a=nodes.index(a),b=nodes.index(b),width=2*radius+1,line=line))
    floor={p for p in floor if 1<p[0]<W-2 and 1<p[1]<H-2}
    water&=floor;bridges&=water
    spawn=(23,38)
    # Every room's centre is a dry crossing connecting its banks and entrances.
    for room in rooms:
        cx,cy=room['centre']
        for x,y in list(water):
            if abs(x-cx)<=2 and y==cy:bridges.add((x,y))
    dry=floor-water|bridges
    # A connecting grate is installed where a channel would isolate a bank.
    reached=flood(dry,spawn)
    while set(reached)!=dry:
        missing=dry-set(reached)
        choices=[p for p in water-bridges if any(n in reached for n in neighbours(p))]
        assert choices,'Disconnected room geometry'
        p=min(choices,key=lambda q:min(abs(q[0]-m[0])+abs(q[1]-m[1]) for m in missing))
        bridges.add(p);dry.add(p);reached=flood(dry,spawn)
    assert len(reached)==len(dry)
    # Props are placed whole against walls. Their footprint is independent of
    # the transparent sprite canvas and must leave every remaining tile reachable.
    props=[];blocked=set();reserved={(23,36),(23,37)}
    for room in rooms:
        x0,y0,rw,rh=room['bounds'];kind=room['kind']
        if kind in ('maintenance','access'):
            p=(x0+1,y0+1)
            file='assets/source-art/first-sector-review-01/u01-expansion-r06/cabinet-native.png'
            if p in dry and p not in bridges and len(flood(dry-{p},spawn))==len(dry)-1:
                props.append(dict(file=file,at_px=[p[0]*64+12,p[1]*64+10],blocked_cells=[list(p)]));blocked.add(p);dry.remove(p)
        if kind=='maintenance':
            feet={(x0+rw-3,y0+rh-2),(x0+rw-2,y0+rh-2)}
            if feet<=dry and not feet&bridges and len(flood(dry-feet,spawn))==len(dry)-2:
                props.append(dict(file='assets/source-art/first-sector-review-01/u01-expansion-r05/court-machines-native.png',
                    at_px=[(x0+rw-3)*64-9,(y0+rh-2)*64-75],blocked_cells=[list(p) for p in sorted(feet)]))
                blocked|=feet;dry-=feet
        if kind=='basin':
            p=(x0+rw-2,y0+1)
            if p in dry and p not in bridges and len(flood(dry-{p},spawn))==len(dry)-1:
                props.append(dict(file='assets/source-art/first-sector-review-01/u01-expansion-r06/barrel-native.png',
                    at_px=[p[0]*64+20,p[1]*64+28],blocked_cells=[list(p)]));blocked.add(p);dry.remove(p)
    reached=flood(dry,spawn)
    assert len(reached)==len(dry)
    target=max(reached,key=lambda p:reached[p])
    grid=[['#']*W for _ in range(H)]
    for x,y in floor:grid[y][x]='~' if (x,y) in water else '.'
    for x,y in bridges:grid[y][x]='='
    # Independent streams keep cosmetic variations from changing encounters.
    pop=random.Random(seed^0xD2A11)
    occupied={spawn,target}|reserved|blocked;groups=[];centres_used=[]
    for room in sorted(rooms,key=lambda r:reached.get(tuple(r['centre']),0)):
        if room['kind']=='access':continue
        candidates=[p for p in room['cells']&dry if reached[p]>=8 and p not in occupied and p not in bridges
                    and max(abs(p[0]-spawn[0]),abs(p[1]-spawn[1]))>6
                    and sum(n in dry for n in neighbours(p))>=3
                    and all(abs(p[0]-c[0])+abs(p[1]-c[1])>=6 for c in centres_used)]
        # Room area controls attempts. An empty room remains a valid outcome.
        attempts=max(1,len(room['cells'])//55)
        for _ in range(attempts):
            if not candidates or pop.random()>.85:continue
            centre=pop.choice(sorted(candidates));family=pop.choices(['zombie','cultist'],weights=[3,2])[0]
            lo,hi=(3,5) if family=='zombie' else (2,4)
            count=pop.randint(lo,hi)
            nearby=[p for p in room['cells']&dry if p not in occupied and p not in bridges and reached[p]>=8
                    and max(abs(p[0]-spawn[0]),abs(p[1]-spawn[1]))>6
                    and abs(p[0]-centre[0])+abs(p[1]-centre[1])<=4
                    and sum(n in dry for n in neighbours(p))>=3]
            pop.shuffle(nearby);nearby.sort(key=lambda p:abs(p[0]-centre[0])+abs(p[1]-centre[1]))
            members=[]
            for i,p in enumerate(nearby[:count]):
                number=pop.choice([120,121]) if family=='zombie' else (24 if i==count-1 else 22)
                members.append(dict(at=list(p),number=number,variant=pop.randint(0,1)))
                occupied.add(p)
            if members:
                groups.append(dict(id=len(groups)+1,family=family,centre=list(centre),members=members))
                centres_used.append(centre)
            candidates=[]
    # Same configured roster in each example. No new powers or stat edits.
    assert groups and all(max(abs(m['at'][0]-spawn[0]),abs(m['at'][1]-spawn[1]))>6
                          for g in groups for m in g['members'])
    views=[dict(id='access',at=list(spawn))]
    wet=next(r for r in rooms if r['kind']=='collector')
    wet_at=min(wet['cells']&dry-occupied,key=lambda p:abs(p[0]-wet['centre'][0])+abs(p[1]-wet['centre'][1]))
    views.append(dict(id='collector',at=list(wet_at)))
    encounter=next(g for g in groups if g['family']=='zombie') if any(g['family']=='zombie' for g in groups) else groups[0]
    centre=tuple(encounter['centre']);free=dry-occupied
    at=min(free,key=lambda p:abs(abs(p[0]-centre[0])+abs(p[1]-centre[1])-3))
    # Break equal-distance ties spatially, with a dry path to the group.
    at=min((p for p in free if abs(p[0]-centre[0])+abs(p[1]-centre[1])==abs(at[0]-centre[0])+abs(at[1]-centre[1])),key=lambda p:reached[p])
    views.append(dict(id='encounter',at=list(at)))
    rat_routes=[]
    for room in rooms:
        if room['kind'] not in ('collector','basin'):continue
        candidates=room['cells']&dry-reserved
        if len(candidates)<6:continue
        a=min(candidates);dist=flood(candidates,a);b=max(dist,key=lambda p:dist[p])
        points=route(candidates,a,b)
        if len(points)>=4:
            rat_routes.append(dict(path=[[x*64+24,y*64+40] for x,y in points],speed=52.,phase=float(room['id']*41)))
    return dict(review='EG-GEN-01',seed=seed,width=W,height=H,cell=64,
        spawn=list(spawn),target=list(target),layout=[''.join(row) for row in grid],
        rooms=[{k:v for k,v in r.items() if k!='cells'} for r in rooms],corridors=corridors,
        groups=groups,views=views,props=props,blocked=[list(p) for p in sorted(blocked)],rats=rat_routes,
        floor_count=len(dry),water_count=len(water-bridges),
        all_walkable_connected=True,roster=[22,24,120,121],normal_activation=False)

def package():
    ART.mkdir(parents=True,exist_ok=True);(ART/'kit').mkdir(exist_ok=True)
    sources=read(DOC/'source-inputs.json')
    for name,row in sources.items():
        source=Path(row['source']);target=ART/'kit'/f'{name}-source.png'
        if not target.exists():target.write_bytes(source.read_bytes())
        im=Image.open(target).convert('RGBA')
        if row.get('frames'):
            atlas=Image.new('RGBA',tuple(row['native']));fw,fh=row['frame_size']
            for i in range(row['frames']):
                frame=im.crop((i*im.width//row['frames'],0,(i+1)*im.width//row['frames'],im.height))
                box=frame.getchannel('A').point(lambda a:255 if a>100 else 0).getbbox()
                assert box and box[0]>0 and box[2]<frame.width,'Rat silhouette crosses a frame boundary'
                frame=frame.crop(box);frame.thumbnail(tuple(row['fit']),Image.Resampling.LANCZOS)
                atlas.alpha_composite(frame,(i*fw+(fw-frame.width)//2,fh-2-frame.height))
            im=atlas
        elif row.get('isolated'):
            box=im.getchannel('A').point(lambda a:255 if a>100 else 0).getbbox()
            assert box and min(box[:2])>0 and box[2]<im.width and box[3]<im.height
            im=im.crop(box);im.thumbnail(tuple(row['fit']),Image.Resampling.LANCZOS)
            canvas=Image.new('RGBA',tuple(row['native']));canvas.alpha_composite(im,((canvas.width-im.width)//2,(canvas.height-im.height)//2));im=canvas
        else:im=im.resize(tuple(row['native']),Image.Resampling.LANCZOS)
        im.save(ART/'kit'/f'{name}-native.png')

def street_population(seed):
    """The same room/group principles on an authored street's subareas."""
    s=read(ACTIVE/'street.json');rows=(ACTIVE/'layout.txt').read_text().splitlines()
    cells={(x,y) for y,row in enumerate(rows) for x,c in enumerate(row) if c=='.'}
    cells-=set(map(tuple,s['obstructions']))
    for filename in ['collision-repairs.json','collision-repairs-r09.json','warehouse-clearance-r09.json']:
        cells|=set(map(tuple,read(ACTIVE/filename)['cells']))
    doors=[tuple(r['door']) for r in s['roofs'] if r.get('door')]
    for roof in s['roofs']:cells-=set(map(tuple,roof['interior']))
    cells-=set(doors)
    start=tuple(s['entrance']);distance=flood(cells,start);cells=set(distance)
    rng=random.Random(seed);occupied={start,(14,14),(18,36)};groups=[];centres=[]
    # Courts and stretches of road substitute for D2's engine room regions.
    regions=[(2,50,23,59),(13,41,24,50),(13,30,23,40),(2,19,23,29),(2,11,23,17),(15,2,23,10)]
    for region in regions:
        x0,y0,x1,y1=region
        area={p for p in cells if x0<=p[0]<=x1 and y0<=p[1]<=y1}
        candidates=[p for p in area if p not in occupied and distance[p]>=8
                    and max(abs(p[0]-start[0]),abs(p[1]-start[1]))>6
                    and max(abs(p[0]-2),abs(p[1]-14))>3
                    and all(max(abs(p[0]-d[0]),abs(p[1]-d[1]))>1 for d in doors)
                    and sum(n in cells for n in neighbours(p))>=3]
        for _ in range(max(1,len(area)//100)):
            options=[p for p in candidates if all(abs(p[0]-c[0])+abs(p[1]-c[1])>=8 for c in centres)]
            if not options or rng.random()>.92:continue
            centre=rng.choice(sorted(options));family=rng.choices(['cultist','zombie'],[3,2])[0]
            target=rng.randint(2,4) if family=='cultist' else rng.randint(3,5)
            local=flood(area,centre)
            members=[]
            possible=[p for p in candidates if p not in occupied and local.get(p,99)<=4]
            rng.shuffle(possible);possible.sort(key=lambda p:local.get(p,99))
            for i,p in enumerate(possible[:target]):
                number=rng.choice([120,121]) if family=='zombie' else (24 if i==target-1 else 22)
                members.append(dict(at=list(p),number=number,variant=rng.randint(0,1),level=1 if p[1]>=28 else 2));occupied.add(p)
            if members:groups.append(dict(id=len(groups)+1,family=family,centre=list(centre),members=members));centres.append(centre)
    return dict(review='POP-D2-02',seed=seed,width=26,height=66,spawn=list(start),layout=rows,
                groups=groups,views=[dict(id='north',at=[14,14]),dict(id='central',at=[18,36])],
                native_cell=64,normal_activation=False,existing_combat_profiles_unchanged=True,
                entry_exclusion=[2,14,3],initial_arrival_minimum_path_distance=8,
                initial_arrival_clear_radius=6)

def tiled(texture,xy):
    x,y=xy;w,h=texture.size
    return texture.crop((x%w,y%h,x%w+64,y%h+64))

def render(plan,folder,wall_finish=None):
    folder.mkdir(parents=True,exist_ok=True)
    floor=Image.open(ART/'kit/floor-native.png').convert('RGBA')
    water=Image.open(ART/'kit/water-native.png').convert('RGBA')
    wall=Image.open(ART/'kit/wall-native.png').convert('RGBA')
    wall=ImageEnhance.Color(wall).enhance(.6)
    wall_cap=ImageEnhance.Brightness(wall).enhance(.78)
    wall_mass=ImageEnhance.Brightness(wall).enhance(.30)
    wall_face=ImageEnhance.Brightness(wall).enhance(.45)
    bridge=Image.open(ART/'kit/grate-native.png').convert('RGBA')
    image=Image.new('RGBA',(W*64,H*64),(11,14,15,255));draw=ImageDraw.Draw(image)
    grid=plan['layout'];region={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c!='#'}
    dry={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c in '.='}
    wallcells={q for p in region for q in [(p[0]+dx,p[1]+dy) for dx in (-1,0,1) for dy in (-1,0,1)] if q not in region}
    for x,y in sorted(region):
        t=tiled(water if grid[y][x] in '~=' else floor,(x*64,y*64))
        image.alpha_composite(t,(x*64,y*64))
    # Contiguous wall tops and front faces are constructed from the same material.
    # Adjacency supplies all corners, inner corners, ends and T connections.
    for x,y in sorted(wallcells,key=lambda p:(p[1],p[0])):
        if not(0<=x<W and 0<=y<H):continue
        image.alpha_composite(tiled(wall_mass,(x*64,y*64)),(x*64,y*64))
        d=ImageDraw.Draw(image)
        l,t=x*64,y*64
        # Short connected border bands retain the camp's low wall silhouette.
        # Diagonal neighbours get a complete corner rather than a floating end.
        cap=tiled(wall_cap,(l,t))
        for dx in (-1,1):
            for dy in (-1,1):
                if (x+dx,y+dy) in region:
                    box=(0 if dx<0 else 40,0 if dy<0 else 26,24 if dx<0 else 64,24 if dy<0 else 64)
                    image.alpha_composite(cap.crop(box),(l+box[0],t+box[1]))
        for dx,dy in DIRS:
            if (x+dx,y+dy) not in region:continue
            l,t=x*64,y*64
            if dy==1:
                image.alpha_composite(cap.crop((0,26,64,64)),(l,t+26))
                face=tiled(wall_face,(l,t)).crop((0,0,64,22));image.alpha_composite(face,(l,t+42))
                d.line((l,t+27,l+63,t+27),fill=(119,119,103,255),width=1)
                d.line((l,t+41,l+63,t+41),fill=(100,102,89,255),width=2)
                d.line((l,t+63,l+63,t+63),fill=(15,19,19,255),width=2)
            elif dy==-1:
                image.alpha_composite(cap.crop((0,0,64,24)),(l,t))
                d.line((l,t+1,l+63,t+1),fill=(96,98,89,255),width=3)
            elif dx==1:
                image.alpha_composite(cap.crop((40,0,64,64)),(l+40,t))
                d.rectangle((l+60,t,l+63,t+63),fill=(29,35,34,255))
                d.line((l+41,t,l+41,t+63),fill=(101,105,91,255),width=1)
            else:
                image.alpha_composite(cap.crop((0,0,24,64)),(l,t))
                d.rectangle((l,t,l+3,t+63),fill=(30,35,34,255))
                d.line((l+22,t,l+22,t+63),fill=(97,100,87,255),width=1)
    if wall_finish is not None:wall_finish(image,plan,region,wallcells)
    # The channel rims are level and continuous, with complete metal grates.
    d=ImageDraw.Draw(image)
    for x,y in sorted(region):
        if grid[y][x] not in '~=':continue
        l,t=x*64,y*64
        for dx,dy in DIRS:
            q=(x+dx,y+dy)
            if q not in dry or grid[q[1]][q[0]]=='=':continue
            if dy==-1:box=(l,t,l+63,t+4)
            elif dy==1:box=(l,t+59,l+63,t+63)
            elif dx==-1:box=(l,t,l+4,t+63)
            else:box=(l+59,t,l+63,t+63)
            d.rectangle(box,fill=(90,88,70,255))
        if grid[y][x]=='=':image.alpha_composite(bridge,(l,t))
    # Complete conduits follow usable wall runs, with localized chemical runoff.
    rng=random.Random(plan['seed']^0xA470)
    lights=[]
    for i,room in enumerate(plan['rooms']):
        x0,y0,rw,rh=room['bounds'];cx,cy=room['centre']
        row=y0-1
        available=[x for x in range(x0,x0+rw) if (x,row) in wallcells and (x,y0) in region]
        runs=[]
        for x in available:
            if not runs or x!=runs[-1][-1]+1:runs.append([x])
            else:runs[-1].append(x)
        if runs:
            run=max(runs,key=len);left,right=run[0]*64, (run[-1]+1)*64
            color=[(101,70,43,255),(72,86,82,255),(100,100,81,255)][i%3]
            d=ImageDraw.Draw(image)
            py=row*64+52
            if not plan.get('integrated_wall_pipes'):
                d.line((left+10,py,right-10,py),fill=(17,22,22,255),width=11)
                d.line((left+10,py-1,right-10,py-1),fill=color,width=6)
                for px in range(left+28,right-12,96):
                    d.rectangle((px,py-6,px+5,py+5),fill=(41,47,45,255))
            if room['kind'] in ('collector','basin') and len(run)>=2:
                pipe=Image.open(ART/'kit/pipe-native.png').convert('RGBA')
                at=(left+4,y0*64-48)
                sludge=Image.open(ART/'kit/sludge-native.png').convert('RGBA')
                image.alpha_composite(sludge,(at[0]+73,at[1]+52))
                image.alpha_composite(pipe,at)
            lx=run[-1]
            lampx,lampy=lx*64+26,row*64+49
            # A small unlit caged bulkhead; emission is a separate lightmap.
            d.rounded_rectangle((lampx-10,lampy-8,lampx+10,lampy+8),radius=4,fill=(15,21,23,255),outline=(104,103,88,255),width=2)
            on=room['kind']=='access' or rng.random()<.68
            d.ellipse((lampx-5,lampy-4,lampx+5,lampy+4),fill=(194,171,113,255) if on else (51,57,53,255))
            d.line((lampx-7,lampy,lampx+7,lampy),fill=(35,41,37,255),width=2)
            if on:
                lights.append(dict(id=f'bulkhead-{i}',at=[lampx,lampy],origin=[lx,y0],
                    box=[lampx-5,lampy-4,lampx+5,lampy+4],radius=260,power=.72,
                    color=[.95,.72,.39],lens_color=[1.,.85,.52]))
    # Whole fixtures preserve their source scale; floor/collision footprints were
    # checked before choosing monsters and small ambient rat routes.
    decor=[]
    for p in plan.get('props',[]):
        if p.get('wall_attached') or p.get('ground_decal'):
            image.alpha_composite(Image.open(ROOT/p['file']).convert('RGBA'),tuple(p['at_px']))
        else:decor.append(dict(file=p['file'],at_px=p['at_px']))
    decor.append(dict(file=rel(ART/'kit/stairs-native.png'),at_px=[plan['spawn'][0]*64-8,plan['spawn'][1]*64-144]))
    bodies=[ACTIVE/'decor/civilians-08/CIV-U01-06-r90-native.png']
    if 'decorative_bodies' in plan:
        decor.extend(copy.deepcopy(plan['decorative_bodies']))
    for i,g in enumerate([] if 'decorative_bodies' in plan else plan['groups']):
        if i%3:continue
        x,y=g['centre'];candidates=[p for p in dry if abs(p[0]-x)+abs(p[1]-y)==3]
        occupied={tuple(m['at']) for group in plan['groups'] for m in group['members']}
        candidates=[p for p in candidates if p not in occupied and list(p) not in plan['blocked']]
        if candidates:
            p=min(candidates);decor.append(dict(file=rel(bodies[0]),at_px=[p[0]*64+5,p[1]*64+12]))
    image.save(folder/'background.png')
    config=dict(ambient=[.48,.54,.57],colored_emitters=True,procedural_emitters=False,lights=lights)
    lighting.ART=folder
    # Water belongs to the optical room even though it blocks walking.
    lighting.build_lightmaps(image,region,config,lights)
    config.update(correction=rel(folder/'correction.png'),irradiance=rel(folder/'irradiance.png'))
    scene=copy.deepcopy(read(ACTIVE/'street.json'))
    scene.update(id='EG01',width=W,height=H,image_origin=[0,0],image_offset_px=[0,0],image_size=[W,H],
        background=rel(folder/'background.png'),entrance=plan['spawn'],home_entry=plan['spawn'],
        home_inside=plan['spawn'],car_at=[-100,-100],car_size=[0,0],roofs=[],doorway=None,
        extra_doorways=[],ambience=None,scenery_props=decor,atmosphere=config,obstructions=plan['blocked'])
    write(folder/'street.json',scene);write(folder/'plan.json',plan)
    (folder/'layout.txt').write_text('\n'.join(row.replace('~','#').replace('=','.') for row in grid)+'\n',encoding='utf-8')

def entry():
    target=ART/'entry';target.mkdir(exist_ok=True)
    before=Image.open(ACTIVE/'street.png').convert('RGBA')
    tiles=[Image.open(paving.ART/f'{n}-native.png').convert('RGBA') for n in ['PAV-U01-06','PAV-U01-06B','PAV-U01-06C']]
    mix=[tiles[0]]+[tiles[1]]*6+[tiles[2]]*5
    common=dict(warehouse=repairs.ART/'warehouse-native.png',
        asphalt=Image.open(repairs.ART/'ROAD-U01-09-conditioned.png').convert('RGBA'),
        repair_join=True,paving_overrides={(x,y):tiles[1+(x+y)%2] for y in (7,8) for x in range(9,13)})
    grass=read(paving.ART/'composition-checks.json')['grass_at_px']
    removed=['collapsed-wall'];removed_decor=['DECO-U01-021','WAR-U01-05','WAR-U01-06']
    oldomit=['warehouse-crate','warehouse-cabinet','warehouse-barrel']
    baseline=paving.compose(mix,grass,omit_modules=oldomit,**common)
    assert baseline.tobytes()==before.tobytes(),'Source-layer reconstruction must exactly match the live background'
    proposal=paving.compose(mix,grass,omit_modules=oldomit+removed,omit_decor=removed_decor,**common)
    proposal.save(target/'background.png')
    scene=copy.deepcopy(read(ACTIVE/'street.json'));scene['background']=rel(target/'background.png')
    moved=[]
    for prop in scene['scenery_props']:
        if prop.get('id')=='CIV-U01-06-31':
            moved.append(dict(id=prop['id'],before=prop['at_px'],after=[520,918]));prop['at_px']=[520,918]
    # Revised on request: left sidewalk, not the carriageway at the former (6,14).
    # Frame centre at (2,14); the lid folds north, wholly clear of the curb.
    scene['scenery_props'].append(dict(file=rel(ART/'kit/entry-native.png'),at_px=[112,836]))
    scene['visual_review']='EG-ENT-01, candidate only'
    write(target/'street.json',scene)
    return dict(at=[2,14],previous_requested_at=[6,14],location='left sidewalk',
                removed_modules=removed,removed_decor=removed_decor,
                corpse_moved=moved,whole_objects_removed=True,baseline_reconstruction_identical=True)

def build():
    ART.mkdir(parents=True,exist_ok=True);DOC.mkdir(parents=True,exist_ok=True);OUT.mkdir(parents=True,exist_ok=True)
    baseline=hashes();package();entrance=entry()
    checks=[]
    # Audit determinism/connectivity/spacing over more than the shown layouts.
    for seed in range(300):
        p=generate(seed)
        assert p==generate(seed)
    for i,seed in enumerate(SEEDS):
        p=generate(seed);render(p,ART/f'seed-{i+1}')
        checks.append(dict(seed=seed,rooms=len(p['rooms']),corridors=len(p['corridors']),groups=len(p['groups']),
            monsters=sum(len(g['members']) for g in p['groups']),walkable=p['floor_count'],
            rats=len(p['rats']),
            connected=p['all_walkable_connected'],layout_sha256=hashlib.sha256('\n'.join(p['layout']).encode()).hexdigest()))
    street_checks=[]
    for i,seed in enumerate(SEEDS[:2]):
        p=street_population(seed);write(ART/f'street-population-{i+1}.json',p)
        street_checks.append(dict(seed=seed,groups=len(p['groups']),monsters=sum(len(g['members']) for g in p['groups'])))
    assert len({c['layout_sha256'] for c in checks})==3
    assert baseline==hashes(),'Active street changed'
    write(DOC/'proposal.json',dict(review='EG01',status='awaiting_visual_and_rules_review',
        entrance=entrance,dimensions=[W,H],native_cell=64,seeds=checks,
        roster={'zombies':[120,121],'cultists':[22,24]},
        placement=dict(method='room density attempts, weighted family choice, clustered pack members on valid floor',
            zombies_group=[3,5],cultists_group=[2,4],entry_minimum_path_distance=8,
            entry_clear_radius=6,
            groups_not_always_present=True,water_and_bridges_clear_of_spawn=True),
        proposed_rules=['same generated layout on return within the run','new seed generates a different layout',
            'water is nonwalkable; grates connect the banks','rats and chemical deposits are decorative only','no sector boss yet'],
        rat_frame_size=[20,12],rat_silhouette_max_length=18,
        feedback_deferred=['difficulty too low in previous street trial','XP progression too fast'],
        balance_or_xp_changed=False,normal_activation=False,normal_map_sha256=baseline,
        street_placement_review='POP-D2-02',street_seeds=street_checks,
        generation_audit_seeds=300,
        sources=['https://github.com/ThePhrozenKeep/D2MOO/blob/master/source/D2Common/src/Drlg/DrlgMaze.cpp',
                 'https://github.com/ThePhrozenKeep/D2MOO/blob/master/source/D2Game/src/MONSTER/MonsterRegion.cpp',
                 'https://github.com/ThePhrozenKeep/D2MOO/blob/master/source/D2Game/src/MONSTER/MonsterChoose.cpp']))
    print(json.dumps(checks,ensure_ascii=False))

if __name__=='__main__':build()
