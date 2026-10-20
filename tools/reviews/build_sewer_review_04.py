"""EG-COMP-04 / EG-POP-04: functional scenery ensembles and denser packs.

The accepted kit/topology remains intact. This is a separate review proposal;
neither population nor new scenery is activated in the campaign.
"""
import copy
import random
import shutil
from PIL import Image
import build_sewer_review_03 as habitat

previous=habitat.previous
base=habitat.base
ROOT=base.ROOT
ART=ROOT/'assets/source-art/sewer-review-04'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/sewer-review-04'
OUT=ROOT/'artifacts/sewer-review-04'
SEEDS=base.SEEDS
read,write,rel,sha=base.read,base.write,base.rel,base.sha
rectangle,overlap=habitat.rectangle,habitat.overlap

def kit(key):
    folder=ART if key in ('shelf','pallet','runoff') else habitat.ART if key in habitat.SHAPES else previous.ART
    return rel(folder/'kit'/f'{key}-native.png')

def package():
    (ART/'kit').mkdir(parents=True,exist_ok=True)
    for key,row in read(DOC/'source-inputs.json').items():
        source=ART/'kit'/f'{key}-source.png'
        if not source.exists():shutil.copy2(row['source'],source)
        im=Image.open(source).convert('RGBA')
        box=im.getchannel('A').point(lambda a:255 if a>100 else 0).getbbox()
        assert box and min(box[:2])>0 and box[2]<im.width and box[3]<im.height
        im=im.crop(box);im.thumbnail(tuple(row['fit']),Image.Resampling.LANCZOS)
        canvas=Image.new('RGBA',tuple(row['native']))
        canvas.alpha_composite(im,((canvas.width-im.width)//2,(canvas.height-im.height)//2))
        canvas.save(ART/'kit'/f'{key}-native.png')
    im=Image.open(ART/'kit/runoff-native.png')
    for angle in (90,180,270):im.rotate(angle,expand=True).save(ART/'kit'/f'runoff-r{angle}-native.png')

SHAPES={**habitat.SHAPES,'shelf':(2,1,-8,-40),'pallet':(2,1,-8,-40),
        'workbench':(2,1,0,-16),'pump':(2,1,0,-32)}

def existing_bodies(plan):
    dry={(x,y) for y,row in enumerate(plan['layout']) for x,c in enumerate(row) if c in '.='}
    actors={tuple(m['at']) for g in plan['groups'] for m in g['members']}
    blocked=set(map(tuple,plan['blocked']));bodies=[]
    for i,g in enumerate(plan['groups']):
        if i%3:continue
        x,y=g['centre']
        cells=[p for p in dry if abs(p[0]-x)+abs(p[1]-y)==3 and p not in actors|blocked]
        if cells:
            px,py=min(cells)
            bodies.append(dict(file=rel(base.ACTIVE/'decor/civilians-08/CIV-U01-06-r90-native.png'),at_px=[px*64+5,py*64+12]))
    return bodies

def decorate(seed):
    approved=previous.decorate(seed);plan=copy.deepcopy(approved);rng=random.Random(seed^0xC04)
    grid=plan['layout'];spawn=tuple(plan['spawn'])
    floor={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c!='#'}
    raw_dry={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c in '.='}
    blocked=set(map(tuple,plan['blocked']));dry=raw_dry-blocked
    actors={tuple(m['at']) for g in plan['groups'] for m in g['members']}
    bridges={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c=='='}
    protected={spawn,(23,36),(23,37)}|actors|bridges
    for p in bridges:protected.update(base.neighbours(p))
    room_union=set();areas={}
    for room in plan['rooms']:
        x0,y0,w,h=room['bounds'];area={(x,y) for y in range(y0,y0+h) for x in range(x0,x0+w)}&floor
        areas[room['id']]=area;room_union|=area
        centre=min(dry,key=lambda p:(abs(p[0]-room['centre'][0])+abs(p[1]-room['centre'][1]),p))
        protected.update(base.route(dry,spawn,centre))
    protected.update(floor-room_union)
    for r in plan['rats']:protected.update((int((p[0]-24)/64),int((p[1]-40)/64)) for p in r['path'])
    bodies=existing_bodies(plan);plan['decorative_bodies']=bodies
    rectangles=[rectangle(p) for p in plan['props'] if not p.get('wall_attached')]
    rectangles += [rectangle(p) for p in bodies]
    rectangles += [(x*64+7,y*64+3,x*64+57,y*64+62) for x,y in actors]
    rectangles.append(rectangle(dict(file=rel(base.ART/'kit/stairs-native.png'),at_px=[spawn[0]*64-8,spawn[1]*64-144])))
    additions=[];ensembles=[]
    def place(room,template,theme,anchor=None):
        nonlocal dry,blocked
        x0,y0,w,h=room['bounds'];options=[]
        tw=max(dx+SHAPES[key][0] for key,dx,dy in template)
        th=max(dy+SHAPES[key][1] for key,dx,dy in template)
        for y in range(y0,y0+h-th+1):
            for x in range(x0,x0+w-tw+1):
                # Ensembles occupy edges, leaving an open fighting area. Never
                # rotate upright furniture or its short front face.
                north=all((a,y-1) not in floor for a in range(x,x+tw))
                west=all((x-1,b) not in floor for b in range(y,y+th))
                east=all((x+tw,b) not in floor for b in range(y,y+th))
                if not(north or west or east):continue
                parts=[];feet=set()
                for key,dx,dy in template:
                    fw,fh,ox,oy=SHAPES[key];px,py=x+dx,y+dy
                    f={(a,b) for b in range(py,py+fh) for a in range(px,px+fw)}
                    part=dict(file=kit(key),at_px=[px*64+ox,py*64+oy],blocked_cells=[list(p) for p in sorted(f)],
                              kind=key,review='EG-COMP-04',theme=theme,room=room['id'])
                    # The opening of a low shelter must lead onto a clear dry
                    # strip; a north-facing wall cannot close its front.
                    if key in ('shelter','canopy') and not {(px,py+2),(px+1,py+2)}<=dry:
                        parts=[];break
                    parts.append(part);feet|=f
                if not parts or not feet<=dry or feet&protected:continue
                boxes=[rectangle(p) for p in parts]
                if any(overlap(a,b,4) for a in boxes for b in rectangles):continue
                if any(overlap(a,b) for i,a in enumerate(boxes) for b in boxes[i+1:]):continue
                if len(base.flood(dry-feet,spawn))!=len(dry)-len(feet):continue
                # Related objects remain near their ensemble's first unit.
                distance=0 if anchor is None else abs(x-anchor[0])+abs(y-anchor[1])
                score=(distance,0 if north else 1,rng.random())
                options.append((score,x,y,parts,feet,boxes))
        if not options:return None
        _,x,y,parts,feet,boxes=min(options,key=lambda p:p[0])
        dry-=feet;blocked|=feet;rectangles.extend(boxes);plan['props'].extend(parts);additions.extend(parts)
        ensembles.append(dict(room=room['id'],theme=theme,anchor=[x,y],parts=parts))
        return x,y
    # One real encampment instead of three isolated tents spread over the map.
    for room in plan['rooms']:
        kind=room['kind'];anchor=None
        if kind=='alcove':
            for template in [ [('shelter',0,0),('belongings',2,0),('barrels',2,1)],
                              [('canopy',0,0),('bedroll',2,0)],
                              [('bedroll',0,0),('bedroll',1,0),('belongings',0,2),('barrels',1,2)],
                              [('shelf',0,0),('belongings',2,0)] ]:
                p=place(room,template,'refuge',anchor)
                if p and anchor is None:anchor=p
        elif kind=='maintenance':
            for template in [ [('shelf',0,0),('pallet',2,0)], [('pallet',0,0),('barrels',2,0)],
                              [('shelf',0,0)], [('workbench',0,0),('belongings',2,0)] ]:
                p=place(room,template,'workshop',anchor)
                if p and anchor is None:anchor=p
        elif kind=='junction':
            for template in [ [('pallet',0,0),('pallet',2,0)], [('shelf',0,0),('barrels',2,0)],
                              [('pallet',0,0),('barrels',2,0)], [('barrels',0,0),('barrels',1,0)] ]:
                p=place(room,template,'store',anchor)
                if p and anchor is None:anchor=p
        elif kind in ('collector','basin'):
            for template in [ [('shelf',0,0),('barrels',2,0)], [('pallet',0,0),('barrels',2,0)],
                              [('barrels',0,0),('barrels',1,0)], [('shelf',0,0)] ]:
                p=place(room,template,'hydraulic',anchor)
                if p and anchor is None:anchor=p
        else:
            place(room,[('shelf',0,0),('barrels',2,0)],'access')
    # Flat silt belongs beside an outlet/channel, not sprinkled on every tile.
    for room in plan['rooms']:
        if room['kind'] not in ('collector','basin'):continue
        water={p for p in areas[room['id']] if grid[p[1]][p[0]]=='~'}
        options=[]
        for x,y in sorted(areas[room['id']]&dry):
            if not any(abs(x-wx)+abs(y-wy)<=2 for wx,wy in water):continue
            prop=dict(file=kit('runoff'),at_px=[x*64-64,y*64-24],blocked_cells=[],kind='runoff',
                      review='EG-COMP-04',ground_decal=True,room=room['id'])
            box=rectangle(prop)
            cells={(a,b) for b in range(box[1]//64,(box[3]-1)//64+1) for a in range(box[0]//64,(box[2]-1)//64+1)}
            if not cells<=floor or cells&bridges:continue
            if any(overlap(box,b,0) for b in rectangles):continue
            options.append((0 if any(n not in floor for n in base.neighbours((x,y))) else 1,rng.random(),prop,box))
        if options:
            _,_,prop,box=min(options,key=lambda p:p[:2]);plan['props'].append(prop);additions.append(prop);rectangles.append(box)
    # Small drains are arranged at hydraulic/service edges as a coherent strip.
    for room in plan['rooms']:
        if room['kind'] not in ('maintenance','collector','basin'):continue
        x0,y0,w,h=room['bounds']
        for x,y in [(x0+w-3,y0+1),(x0+1,y0+h-2)]:
            cells={(x,y),(x+1,y)}
            if not cells<=dry or cells&actors:continue
            parts=[dict(file=rel(base.ART/'kit/grate-native.png'),at_px=[a*64,b*64],blocked_cells=[],
                        kind='drain',review='EG-COMP-04',ground_decal=True,room=room['id']) for a,b in sorted(cells)]
            boxes=[rectangle(p) for p in parts]
            if any(overlap(a,b) for a in boxes for b in rectangles):continue
            plan['props'].extend(parts);additions.extend(parts);rectangles.extend(boxes);break
    # Review viewpoints are reserved before filling rooms with enemies.
    clear={p for p in dry if p not in actors and not any(overlap((p[0]*64,p[1]*64,(p[0]+1)*64,(p[1]+1)*64),r,6) for r in rectangles)}
    views=[dict(id='access',at=list(spawn))]
    for theme,identifier in [('refuge','refuge'),('workshop','workshop'),('store','store'),('hydraulic','collector')]:
        units=[e for e in ensembles if e['theme']==theme]
        if not units:continue
        unit=max(units,key=lambda u:sum(e['room']==u['room'] for e in units))
        local=clear&areas[unit['room']]
        if local:
            x,y=unit['anchor'];p=min(local,key=lambda p:(abs(abs(p[0]-x-1)+abs(p[1]-y-1)-3),p))
            views.append(dict(id=identifier,at=list(p)))
    plan.update(props=plan['props'],blocked=[list(p) for p in sorted(blocked)],floor_count=len(dry),
                review='EG-COMP-04',ensembles=ensembles,composition_props=additions,views=views)
    populate(plan,dry,areas,clear,seed)
    assert len(base.flood(dry,spawn))==len(dry)
    assert plan['layout']==approved['layout']
    assert plan['decorative_bodies']==bodies
    plan['composition_checks']=dict(all_walkable_connected=True,no_solid_prop_overlap=True,
        accepted_wall_network_preserved=True,no_main_pipe_on_floor=True,original_members_retained=True,
        entrance_safe_radius=6,arrival_minimum_path_distance=8,existing_body_placements_preserved=True)
    return plan

def populate(plan,dry,areas,clear,seed):
    rng=random.Random(seed^0xD204);spawn=tuple(plan['spawn']);distance=base.flood(dry,spawn)
    occupied={tuple(m['at']) for g in plan['groups'] for m in g['members']}
    original=copy.deepcopy(plan['groups']);camera={tuple(v['at']) for v in plan['views']}
    valid={p for p in dry if p not in occupied|camera and distance[p]>=8
           and max(abs(p[0]-spawn[0]),abs(p[1]-spawn[1]))>6}
    # Furniture alpha can extend above its base. Keep new actors visibly clear.
    props=[rectangle(p) for p in plan['props'] if not p.get('wall_attached') and not p.get('ground_decal')]
    valid={p for p in valid if not any(overlap((p[0]*64+7,p[1]*64+3,p[0]*64+57,p[1]*64+62),b,0) for b in props)}
    bridges={(x,y) for y,row in enumerate(plan['layout']) for x,c in enumerate(row) if c=='='}
    def add(group,area,count):
        centre=tuple(group['centre']);dist=base.flood(dry,centre)
        points=[p for p in valid&area if p not in occupied and p not in bridges and dist.get(p,999)<=4]
        rng.shuffle(points);points.sort(key=lambda p:dist[p])
        for i,p in enumerate(points[:count]):
            number=rng.choice([120,121]) if group['family']=='zombie' else (24 if (i+len(group['members']))%5==4 else 22)
            group['members'].append(dict(at=list(p),number=number,variant=rng.randint(0,1),level=1))
            occupied.add(p);valid.discard(p)
    room_group={}
    for group in plan['groups']:
        p=tuple(group['centre']);room=min(plan['rooms'],key=lambda r:abs(r['centre'][0]-p[0])+abs(r['centre'][1]-p[1]))
        room_group.setdefault(room['id'],[]).append(group)
        target=rng.randint(8,10) if group['family']=='zombie' else rng.randint(5,7)
        add(group,areas[room['id']],max(0,target-len(group['members'])))
    for room in sorted(plan['rooms'],key=lambda r:distance.get(tuple(r['centre']),0)):
        if room['kind']=='access':continue
        area=areas[room['id']]&dry;present=room_group.get(room['id'],[])
        goal=2 if len(area)>=58 and room['kind']!='alcove' else 1
        for _ in range(max(0,goal-len(present))):
            candidates=[p for p in valid&area if p not in occupied and p not in bridges
                        and sum(n in dry for n in base.neighbours(p))>=3
                        and all(abs(p[0]-g['centre'][0])+abs(p[1]-g['centre'][1])>=5 for g in present)]
            if not candidates:break
            # Away from throats; two clusters have breathing room between them.
            weights=[1+sum(q in area for q in base.neighbours(p)) for p in sorted(candidates)]
            p=rng.choices(sorted(candidates),weights)[0]
            family=rng.choices(['zombie','cultist'],[3,2])[0]
            group=dict(id=len(plan['groups'])+1,family=family,centre=list(p),members=[])
            add(group,area,rng.randint(6,9) if family=='zombie' else rng.randint(4,6))
            if group['members']:plan['groups'].append(group);present.append(group)
    # Every original member is kept; density changes do not change profiles,
    # damage, XP rules, loot or enable the unapproved resurrection ability.
    assert all(m in plan['groups'][i]['members'] for i,g in enumerate(original) for m in g['members'])
    assert len(occupied)==sum(len(g['members']) for g in plan['groups'])
    assert all(tuple(m['at']) in dry for g in plan['groups'] for m in g['members'])
    assert all(max(abs(p[0]-spawn[0]),abs(p[1]-spawn[1]))>6 for p in occupied)
    # One close-up specifically demonstrates the larger pack with room to move.
    group=max(plan['groups'],key=lambda g:len(g['members']))
    centre=tuple(group['centre']);options=clear-occupied-camera
    if options:
        p=min(options,key=lambda p:(abs(abs(p[0]-centre[0])+abs(p[1]-centre[1])-4),p))
        plan['views'].append(dict(id='encounter',at=list(p)))
    plan['population_proposal']=dict(review='EG-POP-04',original_count=sum(len(g['members']) for g in original),
        proposed_count=len(occupied),groups=len(plan['groups']),supports=sum(m['number']==132 for g in plan['groups'] for m in g['members']),
        unchanged_combat_profiles=True,damage_or_xp_rebalance=False,resurrection_active=False)

def build():
    habitat.accepted_unchanged();package();OUT.mkdir(parents=True,exist_ok=True)
    (ART/'entry').mkdir(exist_ok=True);write(ART/'entry/street.json',read(previous.ART/'entry/street.json'))
    for i in (1,2):write(ART/f'street-population-{i}.json',read(previous.ART/f'street-population-{i}.json'))
    audit=[]
    for seed in range(100):
        p=decorate(seed);assert p==decorate(seed),seed
        audit.append(p['population_proposal']['proposed_count'])
    counts=[]
    for i,seed in enumerate(SEEDS,1):
        p=decorate(seed);base.render(p,ART/f'seed-{i}',wall_finish=previous.finish_walls)
        counts.append(dict(seed=seed,ensembles=len(p['ensembles']),new_scenery=len(p['composition_props']),
            walkable=p['floor_count'],population=p['population_proposal']))
    write(DOC/'proposal.json',dict(review='EG-COMP-04',status='awaiting_composition_and_density_review',
        accepted_base=rel(base.DOC/'acceptance.json'),accepted_decor=rel(previous.DOC/'acceptance.json'),
        pending_previous_habitat_art=True,seeds=counts,audit_seeds=100,population_audit_range=[min(audit),max(audit)],
        normal_campaign_activation=False,resurrection_rules_status='still_pending_separate_numeric_approval',
        new_drawings=['EG-C01 shelf','EG-C02 pallet','EG-C03 runoff'],normal_street_sha256=base.hashes()))
    habitat.accepted_unchanged();print(counts)

if __name__=='__main__':build()
