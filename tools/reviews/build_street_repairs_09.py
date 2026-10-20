"""Authored collision footprints and non-destructive U01 visual repair review."""
from pathlib import Path
import json
import hashlib
import copy
from PIL import Image, ImageDraw

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/first-sector-review-01/u01-repairs-r09'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-repairs-review-09'
PARTS=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r06'
ACTIVE=ROOT/'assets/maps/prison-street-expanded-01'
OUT=ROOT/'artifacts/street-repairs-review-09'
def read(p):return json.loads(p.read_text('utf-8'))
def write(p,v):p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def collisions():
    """Ground footprints are authored independently of rendered sprite alpha."""
    scene=read(ACTIVE/'street.json');mask=Image.new('L',(1664,4224));draw=ImageDraw.Draw(mask);shapes=[]
    def polygon(name,at,points):
        world=[(x+at[0],y+at[1]) for x,y in points]
        draw.polygon(world,fill=255);shapes.append(dict(id=name,at_px=at,polygon_px=points))
    for m in read(PARTS/'module-placements.json'):
        ident=m['id'];w,h=Image.open(ROOT/m['file']).size
        if ident=='warehouse':points=[(8,7),(646,7),(646,193),(190,193),(190,395),(8,395)]
        elif ident.startswith('hedge-'):
            points=[(20,10),(60,10),(82,42),(82,100),(60,121),(18,121),(4,91),(4,42)]
        elif ident=='court-machines':points=[(12,28),(132,28),(132,134),(12,134)]
        elif ident=='court-bin':points=[(8,32),(128,32),(128,104),(8,104)]
        elif ident=='collapsed-wall':points=[(15,18),(130,15),(195,89),(160,116),(24,102)]
        elif ident.startswith('warehouse-'):points=[(3,4),(w-4,4),(w-4,h-3),(3,h-3)]
        else:points=[(10,9),(w-10,9),(w-10,h-12),(10,h-12)]
        polygon(ident,m['at'],points)
    for m in read(ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r04/new-buildings.json'):
        w,h=Image.open(ROOT/m['file']).size
        polygon(m['code'],m['at_px'],[(18,12),(w-18,12),(w-18,h-24),(18,h-24)])
    footprints={
        'wall-fall':[(32,28),(140,25),(170,92),(124,126),(36,114)],
        'blast-crater':[(55,38),(107,38),(132,62),(127,104),(100,121),(55,117),(31,88),(34,59)],
        'van-wreck':[(8,20),(120,20),(120,63),(8,63)],
        'car-wreck':[(8,17),(122,17),(122,60),(8,60)],
        'war-barrier':[(12,38),(175,28),(188,71),(16,84)],
        'position':[(12,18),(62,18),(68,59),(177,59),(177,109),(12,109)],
    }
    for i,p in enumerate(read(PARTS/'decor-placements.json')):
        if p['blocked_cells']:
            assert p['asset'] in footprints,p['asset']
            polygon(f'{p["asset"]}-{i}',p['at_px'],footprints[p['asset']])
    layout=(ACTIVE/'layout.txt').read_text().splitlines()
    legacy=set(map(tuple,read(ACTIVE/'collision-repairs.json')['cells']))
    vision=set(map(tuple,read(ACTIVE/'room-vision.json')['cells']))
    obstacles=set(map(tuple,scene['obstructions']))
    doors={tuple(r['door']) for r in scene['roofs'] if r['door']}
    disk=Image.new('L',(21,21));ImageDraw.Draw(disk).ellipse((0,0,20,20),fill=255)
    cells=[]
    for y in range(2,49):
        for x in range(2,25):
            cell=(x,y)
            was_blocked=cell not in legacy and (layout[y][x]=='#' or cell in obstacles)
            if not was_blocked or cell in doors:continue
            cx,cy=x*64+32,y*64+32
            patch=mask.crop((cx-10,cy-10,cx+11,cy+11))
            overlaps=sum(bool(a and b) for a,b in zip(patch.get_flattened_data(),disk.get_flattened_data()))
            if overlaps<12:cells.append([x,y])
    assert all(p in cells for p in [[14,16],[15,40],[2,30],[2,31],[15,32],[16,32],[17,32]])
    assert [16,33] not in cells,'Vehicle body must remain solid'
    # The continuation sidewalk was accidentally kept as the old map edge.
    cells.extend([[20,49],[20,50],[20,51]])
    assert len(cells)<256
    result=dict(review='COL-U01-09',revision=2,status='requested_collision_bugfix',date='2026-10-20',cells=cells,
        reason='Authored ground footprints and a 10px contact disk; rendered image alpha never defines collision',
        legacy_revision='COL-U01-01',native_cell=64,contact_radius_px=10,footprints=shapes,
        preserved=['solid buildings','vehicle body','closed doors','city boundaries','south arrival'],
        required_clear=[[14,16],[15,40],[2,30],[2,31],[15,32],[16,32],[17,32],[15,33],[17,33]],
        required_solid=[[16,33],[10,14],[11,14],[3,5],[0,31],[25,31]])
    write(ART/'collision-repairs-r09.json',result)
    mask.save(OUT/'authored-ground-footprints.png')
    print('Authored collision corrections:',len(cells),cells)
    return result

def visual():
    import build_street_paving_review as paving
    import build_street_expansion_review as lighting
    tiles=[Image.open(paving.ART/f'{n}-native.png').convert('RGBA') for n in ['PAV-U01-06','PAV-U01-06B','PAV-U01-06C']]
    mix=[tiles[0]]+[tiles[1]]*6+[tiles[2]]*5
    grass=read(paving.ART/'composition-checks.json')['grass_at_px']
    baseline=paving.compose(mix,grass)
    # Retain the accepted predecessor once: subsequent rebuilds must never
    # derive a second body/light relocation from an already integrated scene.
    baseline_dir=ART/'baseline'
    if not baseline_dir.exists():
        import shutil
        baseline_dir.mkdir()
        for file in ['street.png','street.json','correction.png','irradiance.png']:
            shutil.copy2(ACTIVE/file,baseline_dir/file)
    active=Image.open(baseline_dir/'street.png').convert('RGBA')
    assert baseline.tobytes()==active.tobytes(),'Rebuild must preserve accepted active composition'
    before={p.relative_to(ACTIVE).as_posix():sha(p) for p in ACTIVE.rglob('*') if p.is_file()}
    # Retain the established asphalt, replacing only the accidentally sampled
    # curb/drain band. The standalone generated material supplies that band.
    original=Image.open(ROOT/'assets/maps/prison-street-01/street.png').convert('RGBA')
    asphalt=original.crop((1036,0,1196,256))
    clean=Image.open(ART/'ROAD-U01-09-native.png').convert('RGBA')
    for x in range(160):
        edge=min(x,159-x)
        weight=1. if edge<32 else max(0.,(48-edge)/16)
        if not weight:continue
        for y in range(256):
            a=asphalt.getpixel((x,y));b=clean.getpixel((x,y))
            asphalt.putpixel((x,y),tuple(round(a[c]*(1-weight)+b[c]*weight) for c in range(4)))
    asphalt.alpha_composite(asphalt.crop((65,0,71,256)),(78,0))
    asphalt.save(ART/'ROAD-U01-09-conditioned.png')
    image=paving.compose(mix,grass,warehouse=ART/'warehouse-native.png',
        asphalt=asphalt,repair_join=True)
    image.save(ART/'street-candidate.png')
    scene=read(baseline_dir/'street.json');scene['background']=(ART/'street-candidate.png').relative_to(ROOT).as_posix()
    moved_props=['SANG-U01-04-11','SANG-U01-05-19','CIV-U01-05-36']
    for prop in scene['scenery_props']:
        if prop.get('id') in moved_props:
            prop['at_px']=[prop['at_px'][0]-144,prop['at_px'][1]+32]
    cfg=scene['atmosphere'];old_lights=copy.deepcopy(cfg['lights'])
    cfg['lights']=[l for l in cfg['lights'] if l['id'] not in ['north-east','north-east-upper']]
    lamp=copy.deepcopy(next(l for l in old_lights if l['id']=='west-garage-door'))
    lamp['id']='join-closed-door';sx,sy=224/326,212/309
    lamp['at']=[1352+(lamp['at'][0]-176)*sx,3136+(lamp['at'][1]-2400)*sy]
    l,t,r,b=lamp['box'];lamp['box']=[round(1352+(l-176)*sx),round(3136+(t-2400)*sy),round(1352+(r-176)*sx),round(3136+(b-2400)*sy)]
    lamp['origin']=[21,53];cfg['lights'].append(lamp)
    floor={(x,y) for y,row in enumerate((ACTIVE/'layout.txt').read_text().splitlines()) for x,c in enumerate(row) if c=='.'}
    floor.update(map(tuple,read(ACTIVE/'collision-repairs.json')['cells']))
    floor.update(map(tuple,read(ACTIVE/'collision-repairs-r09.json')['cells']))
    lighting.ART=ART
    lighting.build_lightmaps(image,floor,cfg,cfg['lights'])
    # Preserve technical emission masks for the existing non-painted post lenses.
    correction=Image.open(ART/'correction.png').convert('RGBA');d=ImageDraw.Draw(correction)
    for light in cfg['lights']:
        if light['id'].startswith('post-') and light['box']:
            l,t,r,b=light['box'];d.rounded_rectangle((l+1,t+1,r-1,b-1),radius=2,
                fill=tuple(round(v*255) for v in light['lens_color'])+(255,))
    correction.save(ART/'correction.png')
    cfg.update(correction=(ART/'correction.png').relative_to(ROOT).as_posix(),irradiance=(ART/'irradiance.png').relative_to(ROOT).as_posix())
    write(ART/'street.json',scene)
    assert before=={p.relative_to(ACTIVE).as_posix():sha(p) for p in ACTIVE.rglob('*') if p.is_file()}
    write(DOC/'visual-candidate.json',dict(review='U01-REPAIRS-09',status='visual_approval_pending',
        active_art_unchanged=True,active_sha256=before,baseline_rebuild_exact=True,
        new_sources=['warehouse-source.png','ROAD-U01-09-source.png'],native_cell=64,
        changes=['warehouse material/linework harmonization','standalone asphalt replaces curb-contaminated extraction',
            'old southern exterior void removed along facade silhouette','complete hedge modules drawn after arrival layer',
            'entire truncated north-east component replaced by an existing complete closed building'],
        replacement_building=dict(source='west-garage',at_px=[1352,3136],native_size=[224,212]),
        coherent_ground_scenery_relocation=dict(ids=moved_props,delta_px=[-144,32],reason='Keep existing body/blood together on ground, away from replacement roof'),
        lighting='existing dusk renderer, relocated closed red door marker; no baked glow',
        collision_fix='COL-U01-09 integrated separately',candidate_sha256=sha(ART/'street-candidate.png')))
    print('Visual candidate rebuilt; active map art unchanged')

if __name__=='__main__':
    import sys
    ART.mkdir(exist_ok=True);DOC.mkdir(exist_ok=True);OUT.mkdir(exist_ok=True)
    if '--visual' in sys.argv:visual()
    else:collisions()
