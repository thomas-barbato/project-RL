"""DEFENSE-U01-10: isolated, numbered map proposal; no production writes."""
from pathlib import Path
import copy
import json
import hashlib
import numpy as np
from PIL import Image, ImageDraw
import build_street_expansion_review as lighting

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/first-sector-review-01/u01-defense-r10'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-defense-review-10'
OUT=ROOT/'artifacts/street-defense-review-10'
MAP=ROOT/'assets/maps/prison-street-expanded-01'
def read(p):return json.loads(p.read_text('utf-8-sig'))
def write(p,v):p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
def rel(p):return p.relative_to(ROOT).as_posix()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def build():
    before={rel(p):sha(p) for p in MAP.rglob('*') if p.is_file()}
    assets=[]
    for code,revision,size,limit in [
        ('GARD-U01-01','-r03',(64,64),(62,62)),
        ('GARD-U01-02','-r03',(64,64),(62,62)),
        ('SACS-U01-02','-r03',(80,72),(76,68))]:
        source_file=ART/f'{code}{revision}-source.png'
        native_file=ART/f'{code}{revision}-native.png'
        source=Image.open(source_file).convert('RGBA')
        box=source.getchannel('A').point(lambda a:255 if a>128 else 0).getbbox()
        assert box and min(box)>0 and box[2]<source.width and box[3]<source.height,code
        sprite=source.crop(box)
        sprite.thumbnail(limit,Image.Resampling.LANCZOS)
        native=Image.new('RGBA',size)
        foot_y=size[1]-(1 if code.startswith('GARD') else 2)
        native.alpha_composite(sprite,((size[0]-sprite.width)//2,foot_y-sprite.height))
        native.save(native_file)
        assets.append(dict(id=code,canvas=list(size),drawn_size=list(sprite.size),source_box=list(box),
            revision=revision or 'r01',file=rel(native_file),source=rel(source_file),
            foot_anchor=[size[0]//2,foot_y],native_sha256=sha(native_file),source_sha256=sha(source_file)))
    scene=copy.deepcopy(read(MAP/'street.json'))
    relocated=[]
    for item in scene['scenery_props']:
        if item['id'] in ['CIV-U01-04-20','SANG-U01-04-01','SANG-U01-05-13']:
            before_at=item['at_px'].copy()
            item['at_px']=[item['at_px'][0]+24,item['at_px'][1]-104]
            relocated.append(dict(id=item['id'],before=before_at,after=item['at_px'],reason='clear the interior of the defensive post; keep body and blood ahead of cover'))
    floor={(x,y) for y,row in enumerate((MAP/'layout.txt').read_text().splitlines()) for x,c in enumerate(row) if c=='.'}
    for file in ['collision-repairs.json','collision-repairs-r09.json','warehouse-clearance-r09.json']:
        floor.update(map(tuple,read(MAP/file)['cells']))
    floor.difference_update(map(tuple,scene['obstructions']))
    floor.update(map(tuple,read(MAP/'collision-repairs-r09.json')['cells']))
    floor.update(map(tuple,read(MAP/'warehouse-clearance-r09.json')['cells']))
    additions=[]
    def prop(code,file,at,context):
        item=dict(id=code,file=rel(file),at_px=list(at),context=context)
        additions.append(item);scene['scenery_props'].append(item)
    covers=[]
    bag=Image.open(ART/'SACS-U01-02-r03-native.png').convert('RGBA')
    bag.transpose(Image.Transpose.FLIP_LEFT_RIGHT).save(ART/'SACS-U01-02-r03-east-native.png')
    for side,x,mirror,file in [('ouest',416,False,ART/'SACS-U01-02-r03-native.png'),
                             ('est',560,True,ART/'SACS-U01-02-r03-east-native.png')]:
        prop('SACS-U01-02-'+side,file,(x,3864),'barricade en L avec retour contre le montant du portail')
        covers.append(dict(at_px=[x,3864],mirror=mirror,file=rel(file),native_size=[80,72]))
    # r04 obstacles were refused. Keep the shown corpses at their exact positions.
    accepted=read(DOC/'rejected-r04/defense.json')
    shown_positions={b['id']:tuple(b['at_px']) for b in accepted['new_corpses']}
    street_clear_rect=(448,3520,640,3840)
    body_plan=[
        ('cultiste','CORP-U01-02',0,(466,3690),'attaque contre le poste de garde'),
        ('cultiste','CORP-U01-02',180,(702,3446),'repli après le carrefour sud'),
        ('cultiste','CORP-U01-02',90,(1156,2248),'approche du véhicule détruit'),
        ('cultiste','CORP-U01-02',270,(1124,870),'combat sur l’avenue nord'),
        ('militaire','CORP-U01-03',180,(370,3616),'flanc du dernier point de défense'),
        ('militaire','CORP-U01-03',90,(1164,2810),'repli au carrefour sud'),
        ('militaire','CORP-U01-03',0,(634,2530),'près de la cour dévastée'),
        ('militaire','CORP-U01-03',270,(1210,1504),'défense de l’avenue centrale'),
        ('civil','CIV-U01-04',180,(1140,3360),'fuite depuis les logements sud'),
        ('civil','CIV-U01-06',90,(1420,2040),'trottoir près des bâtiments endommagés'),
        ('civil','CIV-U01-07',0,(302,2750),'sortie de l’atelier abandonné'),
        ('civil','CIV-U01-05',180,(930,610),'fuite près de la façade nord détruite'),
        ('cultiste','CORP-U01-02',180,(626,3778),'assaut brisé en avant du garde est'),
        ('cultiste','CORP-U01-02',90,(472,3552),'attaquant tombé dans l’approche de la prison'),
        ('cultiste','CORP-U01-02',270,(640,3384),'progression interrompue vers la prison'),
        ('militaire','CORP-U01-03',0,(250,3714),'défenseur au flanc ouest de l’approche'),
        ('militaire','CORP-U01-03',180,(1160,3176),'position perdue au débouché sud'),
        ('militaire','CORP-U01-03',90,(584,2350),'repli sur le trottoir central'),
        ('civil','CIV-U01-05',0,(722,3626),'civil abattu à proximité de la clôture est'),
        ('civil','CIV-U01-07',180,(1330,2754),'sortie du logement près de la cour'),
        ('civil','CIV-U01-06',270,(1240,1190),'fuite depuis les habitations de l’avenue'),
    ]
    rotations={0:None,90:Image.Transpose.ROTATE_90,180:Image.Transpose.ROTATE_180,270:Image.Transpose.ROTATE_270}
    occupied=[]
    for p in scene['scenery_props']:
        if p['id'].startswith(('CORP-','CIV-')):
            im=Image.open(ROOT/p['file']);occupied.append((p['at_px'][0]+im.width/2,p['at_px'][1]+im.height/2))
    body_records=[]
    for i,(kind,code,angle,wanted,context) in enumerate(body_plan):
        source=MAP/'decor'/f'{code}.png' if kind!='civil' else MAP/'decor/civilians-08'/f'{code}-native.png'
        sprite=Image.open(source).convert('RGBA')
        if angle:sprite=sprite.transpose(rotations[angle])
        file=ART/f'MORT-U01-10-{i+1:02}.png';sprite.save(file)
        # Removing obstacles must never reposition the bodies just accepted.
        def fits(at):
            x,y=at;cx,cy=x+sprite.width/2,y+sprite.height/2
            points=[(int((x+px)//64),int((y+py)//64)) for px,py in
                [(sprite.width/2,sprite.height/2),(10,10),(sprite.width-10,sprite.height-10)]]
            return all(p in floor for p in points) and all((cx-a)**2+(cy-b)**2>78**2 for a,b in occupied)
        at=shown_positions[f'MORT-U01-10-{i+1:02}']
        assert fits(at),(code,at)
        occupied.append((at[0]+sprite.width/2,at[1]+sprite.height/2))
        prop(f'MORT-U01-10-{i+1:02}',file,at,context)
        body_records.append(dict(id=f'MORT-U01-10-{i+1:02}',kind=kind,source=rel(source),rotation=angle,at_px=list(at),context=context))
        if i in [0,2,4,6,8,11,12,13,15,18,20]:
            blood=MAP/'decor/civilians-08/SANG-U01-04-native.png'
            prop(f'SANG-U01-10-{i+1:02}',blood,(at[0]+6,at[1]+sprite.height//2-10),'sang associé au corps '+str(i+1))
    guards=[dict(id='GARD-U01-01',file=assets[0]['file'],cell=[7,61],pixel_offset=[-12,-37],foot_anchor=assets[0]['foot_anchor'],phase=0.),
            dict(id='GARD-U01-02',file=assets[1]['file'],cell=[9,61],pixel_offset=[-22,-37],foot_anchor=assets[1]['foot_anchor'],phase=1.8)]
    fires=[dict(id='FEU-U01-10-01',foot=[1110,3910],size=[64,80],phase=0,context='bois de la ruine près du portail'),
           dict(id='FEU-U01-10-02',foot=[1072,2134],size=[64,76],phase=3,context='moteur de la camionnette détruite'),
           dict(id='FEU-U01-10-03',foot=[222,1482],size=[56,72],phase=5,context='débris de la façade effondrée ouest'),
           dict(id='FEU-U01-10-04',foot=[1020,2526],size=[56,72],phase=2,context='poutres dans les gravats de la cour')]
    cfg=scene['atmosphere']
    ordinary_lights=copy.deepcopy(cfg['lights'])
    for fire in fires:
        # A fire can sit on a solid wreck/rubble tile. Its emitter traces start
        # at the adjacent exposed ground, like the existing wall fixtures.
        origin=min(floor,key=lambda p:(p[0]*64+32-fire['foot'][0])**2+(p[1]*64+32-fire['foot'][1])**2)
        cfg['lights'].append(dict(id=fire['id'],at=fire['foot'],origin=list(origin),box=None,
            radius=172,power=.74,color=[1.,.32,.07],lens_color=[1.,.55,.10]))
    lighting.ART=ART
    ink=Image.open(MAP/'street.png').convert('RGBA')
    lighting.build_lightmaps(ink,floor,cfg,ordinary_lights)
    correction=Image.open(ART/'correction.png').convert('RGBA')
    irradiance=np.array(Image.open(ART/'irradiance.png').convert('RGB'),dtype=np.uint16)
    # Visible rubble and roof surfaces can receive a fire's light even when
    # their gameplay cells are solid. These are emitter-specific EXTERIOR
    # receiver masks; they never open collisions, interiors or player vision.
    receivers=[[(x,y) for y in range(59,62) for x in range(16,21)],
        [(16,33)],[(3,23),(3,22),(4,23)],[(15,39),(15,40),(16,39),(16,40)]]
    light_records=[]
    for fire,extra in zip(fires,receivers):
        light=copy.deepcopy(next(l for l in cfg['lights'] if l['id']==fire['id']))
        light['origin']=[int(v//64) for v in fire['foot']]
        emission_cfg=copy.deepcopy(cfg);emission_cfg['ambient']=[0.,0.,0.]
        lighting.build_lightmaps(ink,floor|set(extra),emission_cfg,[light])
        irradiance+=np.array(Image.open(ART/'irradiance.png').convert('RGB'),dtype=np.uint16)
        light_records.append(dict(id=fire['id'],ray_origin=light['origin'],exterior_receiver_cells=extra))
    Image.fromarray(np.clip(irradiance,0,255).astype('uint8')).save(ART/'irradiance.png')
    write(ART/'lighting-receivers.json',dict(scope='cosmetic exterior surface lightmaps only',lights=light_records))
    d=ImageDraw.Draw(correction)
    for light in cfg['lights']:
        if light['id'].startswith('post-') and light['box']:
            l,t,r,b=light['box'];d.rounded_rectangle((l+1,t+1,r-1,b-1),radius=2,fill=tuple(round(v*255) for v in light['lens_color'])+(255,))
    correction.save(ART/'correction.png')
    cfg.update(correction=rel(ART/'correction.png'),irradiance=rel(ART/'irradiance.png'))
    write(ART/'street.json',scene)
    clear_cells=[[8,y] for y in range(60,63)]+[[x,y] for y in range(55,60) for x in range(7,10)]
    solid_cells=[[7,60],[9,60],[7,61],[9,61]]
    assert all(tuple(c) in floor and c not in solid_cells for c in clear_cells)
    data=dict(review='DEFENSE-U01-10-r05',native_cell=64,status='preview_only',guards=guards,fires=fires,covers=covers,blockers=[],
        proposed_solid_cells=solid_cells,central_free_cells=clear_cells,
        circulation=dict(street_clear_rect_px=list(street_clear_rect),street_width_cells=3,existing_gate_width_cells=1,
            rejected_extra_obstacles_removed=True,
            note='three columns remain open ahead of the existing gate; bodies are nonblocking scenery'),
        new_corpses=body_records,new_scenery=additions,assets=assets,repositioned_existing_scenery=relocated,
        behavior='visual demonstration only; no damage, AI, turn, quest or live actor additions',
        production_sha256=before)
    write(ART/'defense.json',data);write(DOC/'proposal.json',data)
    assert before=={rel(p):sha(p) for p in MAP.rglob('*') if p.is_file()}
    print('Prepared r05: extra obstacles removed; exact 21 corpse placements, 11 blood props, guards, sandbags and fires preserved; production unchanged.')

if __name__=='__main__':build()
