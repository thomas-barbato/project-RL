"""EG-HAB-03: grouped makeshift dwellings, preserving accepted EG01/02 art.

Only review assets are written. Each whole sprite has a separate, honest ground
footprint. Encampments cannot occupy protected routes, canals or rat paths.
"""
import copy
import random
import shutil
from functools import lru_cache
from PIL import Image
import build_sewer_review_02 as previous

base = previous.base
ROOT = base.ROOT
ART = ROOT/'assets/source-art/sewer-review-03'
DOC = ROOT/'docs/reviews/refonte-histoire-2026-10-16/sewer-review-03'
OUT = ROOT/'artifacts/sewer-review-03'
SEEDS = base.SEEDS
read, write, rel, sha = base.read, base.write, base.rel, base.sha

def accepted_unchanged():
    previous.accepted_unchanged()
    agreement=read(previous.DOC/'acceptance.json')
    for file,digest in agreement['accepted_assets_sha256'].items():
        assert sha(ROOT/file)==digest,('Approved EG-DEC-02 source changed',file)
    for file,digest in agreement['accepted_proofs_sha256'].items():
        assert sha(ROOT/file)==digest,('Approved EG-DEC-02 proof changed',file)
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

@lru_cache(None)
def bounds(file):
    with Image.open(ROOT/file) as im:
        return im.convert('RGBA').getchannel('A').point(lambda a:255 if a>30 else 0).getbbox()

def rectangle(prop):
    x,y=prop['at_px'];a,b,c,d=bounds(prop['file'])
    return x+a,y+b,x+c,y+d

def overlap(a,b,gap=3):
    return a[0]<b[2]+gap and b[0]<a[2]+gap and a[1]<b[3]+gap and b[1]<a[3]+gap

SHAPES={'shelter':(2,2,-8,0),'canopy':(2,2,-8,0),
        'bedroll':(1,2,8,0),'belongings':(1,1,-8,0),'barrels':(1,1,-8,-4)}
TEMPLATES=[ [('shelter',0,0),('belongings',2,0),('barrels',2,1)],
            [('canopy',0,0),('bedroll',2,0)],
            [('bedroll',0,0),('belongings',1,0),('barrels',1,1)] ]

def decorate(seed):
    approved=previous.decorate(seed);plan=copy.deepcopy(approved)
    rng=random.Random(seed^0xAB03);spawn=tuple(plan['spawn']);grid=plan['layout']
    floor={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c!='#'}
    dry={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c in '.='}
    blocked=set(map(tuple,plan['blocked']));dry-=blocked
    actor_cells={tuple(m['at']) for g in plan['groups'] for m in g['members']}
    protected={spawn,(23,36),(23,37)}|actor_cells|{tuple(v['at']) for v in plan['views']}
    bridges={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c=='='}
    for p in bridges|actor_cells:protected.update(base.neighbours(p))
    for r in plan['rats']:
        protected.update((int((p[0]-24)/64),int((p[1]-40)/64)) for p in r['path'])
    room_union=set()
    for r in plan['rooms']:
        x0,y0,w,h=r['bounds']
        room_union.update((x,y) for y in range(y0,y0+h) for x in range(x0,x0+w))
        centre=min(dry,key=lambda p:abs(p[0]-r['centre'][0])+abs(p[1]-r['centre'][1]))
        protected.update(base.route(dry,spawn,centre))
    protected.update(floor-room_union)
    rectangles=[rectangle(p) for p in plan['props'] if not p.get('wall_attached')]
    # The shared renderer also places accepted corpses and arrival stairs. They
    # count as existing scenery even though they are not stored in plan.props.
    raw_dry={(x,y) for y,row in enumerate(grid) for x,c in enumerate(row) if c in '.='}
    for index,group in enumerate(plan['groups']):
        if index%3:continue
        cx,cy=group['centre']
        cells=[p for p in raw_dry if abs(p[0]-cx)+abs(p[1]-cy)==3
               and p not in actor_cells|blocked]
        if cells:
            px,py=min(cells)
            rectangles.append(rectangle(dict(file=rel(base.ACTIVE/'decor/civilians-08/CIV-U01-06-r90-native.png'),
                                               at_px=[px*64+5,py*64+12])))
    rectangles.append(rectangle(dict(file=rel(base.ART/'kit/stairs-native.png'),
                                    at_px=[spawn[0]*64-8,spawn[1]*64-144])))
    camps=[];used_rooms=set();new=[]
    for template_index,template in enumerate(TEMPLATES):
        options=[]
        for room in plan['rooms']:
            if room['id'] in used_rooms or room['kind']=='access':continue
            x0,y0,w,h=room['bounds']
            tw=max(dx+SHAPES[key][0] for key,dx,dy in template)
            th=max(dy+SHAPES[key][1] for key,dx,dy in template)
            for y in range(y0,y0+h-th+1):
                for x in range(x0,x0+w-tw+1):
                    # Group furniture at an actual room boundary. The shelter
                    # opening always faces down; never flip its front elevation.
                    if (x,y-1) in floor and (x-1,y) in floor and (x+tw,y) in floor:continue
                    parts=[];feet=set()
                    for key,dx,dy in template:
                        fw,fh,ox,oy=SHAPES[key];px,py=x+dx,y+dy
                        f={(a,b) for b in range(py,py+fh) for a in range(px,px+fw)}
                        prop=dict(file=rel(ART/'kit'/f'{key}-native.png'),at_px=[px*64+ox,py*64+oy],
                                  blocked_cells=[list(p) for p in sorted(f)],review='EG-HAB-03',kind=key)
                        parts.append(prop);feet|=f
                    if not feet<=dry or feet&protected:continue
                    boxes=[rectangle(p) for p in parts]
                    if any(overlap(a,b) for a in boxes for b in rectangles):continue
                    if any(overlap(a,b) for i,a in enumerate(boxes) for b in boxes[i+1:]):continue
                    # A visible object must fit within its declared ground base.
                    if any(not all((int(xx//64),int(yy//64)) in feet for xx,yy in [(b[0],b[1]),(b[2]-1,b[3]-1)]) for b in boxes):continue
                    if len(base.flood(dry-feet,spawn))!=len(dry)-len(feet):continue
                    preferred={'alcove':0,'maintenance':1,'junction':2}.get(room['kind'],4)
                    options.append((preferred,rng.random(),room,x,y,parts,feet,boxes))
        if not options:continue
        _,_,room,x,y,parts,feet,boxes=min(options,key=lambda p:p[:2])
        used_rooms.add(room['id']);dry-=feet;blocked|=feet;rectangles.extend(boxes)
        plan['props'].extend(parts);new.extend(parts)
        camp=dict(id=f'EG-HAB-03-{len(camps)+1}',room=room['id'],template=template_index,
                  anchor=[x,y],parts=parts,blocked_cells=[list(p) for p in sorted(feet)])
        camps.append(camp)
        centre=(x+1,y+2)
        # A walkable cell can still be visually covered by a tall fixture.
        # Keep the demonstration actor's entire cell clear of those silhouettes.
        candidates=[p for p in dry-actor_cells-protected
                    if not any(overlap((p[0]*64,p[1]*64,(p[0]+1)*64,(p[1]+1)*64),b,8)
                               for b in rectangles)]
        assert candidates,('No visually clear capture position',seed)
        view=min(candidates,key=lambda p:(abs(abs(p[0]-centre[0])+abs(p[1]-centre[1])-2),abs(p[0]-centre[0])+abs(p[1]-centre[1]),p))
        plan['views'].append(dict(id=f'habitat-{len(camps)}',at=list(view)))
    assert len(base.flood(dry,spawn))==len(dry)
    assert all(tuple(m['at']) in dry for g in plan['groups'] for m in g['members'])
    assert plan['groups']==approved['groups'] and plan['layout']==approved['layout']
    plan.update(review='EG-HAB-03',habitats=camps,habitat_props=new,
                blocked=[list(p) for p in sorted(blocked)],floor_count=len(dry),
                habitat_checks=dict(all_dry_cells_connected=True,protected_routes_preserved=True,
                    no_prop_overlap=True,no_new_actor=True,approved_monsters_preserved=True,
                    sleeping_mattress_max_length_px=66,demonstration_actor_clear_of_scenery=True))
    return plan

def share_approved_rasters(index):
    """Reuse identical accepted wall/lighting images, retaining only new props."""
    folder=ART/f'seed-{index}';approved=previous.ART/f'seed-{index}'
    scene=read(folder/'street.json')
    for name,field in [('background.png','background'),('correction.png','correction'),
                       ('irradiance.png','irradiance')]:
        candidate=folder/name;original=approved/name
        assert candidate.resolve().is_relative_to(ART.resolve())
        assert original.resolve().is_relative_to(previous.ART.resolve())
        if candidate.exists() and sha(candidate)==sha(original):
            target=scene if field=='background' else scene['atmosphere']
            target[field]=rel(original)
            candidate.unlink()
    write(folder/'street.json',scene)

def build():
    accepted_unchanged();package();OUT.mkdir(parents=True,exist_ok=True)
    (ART/'entry').mkdir(exist_ok=True)
    write(ART/'entry/street.json',read(previous.ART/'entry/street.json'))
    for i in (1,2):write(ART/f'street-population-{i}.json',read(previous.ART/f'street-population-{i}.json'))
    audits=[]
    for seed in range(100):
        p=decorate(seed)
        assert p==decorate(seed),('Non-deterministic',seed)
        audits.append(len(p['habitats']))
    counts=[]
    for i,seed in enumerate(SEEDS,1):
        p=decorate(seed)
        assert len(p['habitats'])==3,('Incomplete shown habitat set',seed,len(p['habitats']))
        base.render(p,ART/f'seed-{i}',wall_finish=previous.finish_walls)
        share_approved_rasters(i)
        counts.append(dict(seed=seed,habitats=len(p['habitats']),new_props=len(p['habitat_props']),
            walkable=p['floor_count'],habitat_checks=p['habitat_checks']))
    write(DOC/'proposal.json',dict(review='EG-HAB-03',status='awaiting_new_habitat_visual_review',
        accepted_base=rel(base.DOC/'acceptance.json'),accepted_decoration=rel(previous.DOC/'acceptance.json'),
        native_cell=64,dimensions=[47,44],seeds=counts,audit_seeds=len(audits),
        minimum_habitats_over_audit=min(audits),maximum_habitats_over_audit=max(audits),
        normal_campaign_activation=False,new_actors=False,new_dialogue=False,
        resurrection_rules_status='awaiting_separate_numeric_approval',
        normal_street_sha256=base.hashes()))
    accepted_unchanged();print(counts)

if __name__=='__main__':build()
