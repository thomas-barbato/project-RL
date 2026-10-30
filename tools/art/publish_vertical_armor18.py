"""C10 intermediate-outfit review, with existing shared one-hand equipment.

No normal manifest is changed. Two-hand grips remain a separate later review.
"""
from pathlib import Path
import hashlib,json,math
from PIL import Image,ImageDraw,ImageFont,ImageChops
from finalize_knife_grip18 import posed,cell,angle as knife_angle
from finalize_pistol_tilt18 import helmet,angle as pistol_angle
from pack_vertical_walk_size18 import structural_box

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18'
OUT=ART/'armor-C10'
DOC=ROOT/'docs/reviews/vertical-walk-18'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/armor-C10'
LOADS={'unarmed':[], 'pistol':[(104,False)],'dual-knives':[(70,False),(70,True)],
       'dual-pistols':[(104,False),(104,True)],'pistol-knife':[(104,False),(70,True)]}
NAMES={'bastion':'Bastion','traqueuse':'Traqueuse','arcaniste':'Arcaniste','metamorphe':'Métamorphe'}

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
def rel(p):return p.relative_to(ROOT).as_posix()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    poses=read(OUT/'conditioned/manifest.json');assert len(poses['profiles'])==12 and not poses['conditioning_errors']
    recipes=read(ART/'extension-C06/conditioned-weapons/recipes.json')
    knife=read(ART/'candidate-C08/knife-angles.json');pistol=read(ART/'candidate-C09/pistol-angles.json')
    images={}
    def image(path):
        if path not in images:images[path]=Image.open(ROOT/path).convert('RGBA')
        return images[path]
    gear={k:image(recipes['headgear'][k]) for k in ['front','back','opening','ownership']}
    destination=OUT/'composites';destination.mkdir(exist_ok=True)
    cases=[];profiles=[];contacts=[];heights=[]
    for p in poses['profiles']:
        family=p['family_poses']['one'];core=image(family['body']);fingers=image(family['hands']);complete=image(family['complete'])
        assert Image.alpha_composite(core,fingers).tobytes()==complete.tobytes(),p['id']
        hands=[cell(fingers,n) for n in range(13)];layers={}
        for model,left in {(m,l) for load in LOADS.values() for m,l in load}:
            frames=[]
            for n,f in enumerate(family['frames']):
                side=int(not left)
                if f['direction']=='back':side=1-side
                b=next(b for b in recipes['bindings'] if b['model']==model and b['left']==(side==0))
                r={'pivot':b['anchor'],'target':[v+64 for v in f['grips'][side]],'scale':1.,'flip':False,'left_source':b['left'],'angle':-math.pi/2 if side==0 else math.pi/2}
                r['angle']=knife_angle(model,r,knife);r['angle']=pistol_angle(model,r,pistol)
                weapon=posed(image(b['palettes'][0]['file']),r);frames.append(weapon)
                overlap=ImageChops.multiply(weapon.getchannel('A'),hands[n].getchannel('A')).getbbox()
                contacts.append({'profile':p['id'],'model':model,'left':left,'pose':n,'visible_pixels_touch_fist':overlap is not None})
            layers[(model,left)]=frames
        values=[structural_box(cell(complete,n).crop((64,64,192,192)))[3]-structural_box(cell(complete,n).crop((64,64,192,192)))[1] for n in range(13)]
        heights.append({'profile':p['id'],'target':p['target_height'],'idle':values[12],'moving_range':[min(values[:12]),max(values[:12])],'row_uniform_xy_scales':p['row_static_scales']})
        entries=[]
        for head in [0,115,116,117,118,119,120]:
            bodies=[cell(core,n) if not head else helmet(cell(core,n),gear,family['frames'][n]['head_anchor'],6<=n<12,head) for n in range(13)]
            for loadout,load in LOADS.items():
                atlas=Image.new('RGBA',(1536,768))
                for n in range(13):
                    frame=bodies[n].copy()
                    for binding in load:frame.alpha_composite(layers[binding][n])
                    frame.alpha_composite(hands[n]);atlas.paste(frame,(n%6*256,n//6*256))
                file=destination/f'{p["id"]}-{loadout}-H{head}.png';atlas.save(file)
                ident=file.stem
                cases.append({'id':ident,'file':rel(file),'kind':'player-C10-one-hand-review','side':256,'foot':[128,159],'save':head in [0,120] and loadout=='pistol-knife' or head==0 and loadout=='unarmed',**({'camp_group':p['id'].split('-')[0]} if head==0 and loadout=='pistol-knife' else {})})
                entries.append({'loadout':loadout,'head':head,'file':rel(file),'case':ident})
        profiles.append({**p,'label':f'{p["number"]:02} · {NAMES[p["id"].split("-")[0]]} · armure {p["rank"]}','cases':entries})
    assert len(cases)==420
    write(OUT/'engine-cases.json',cases)
    write(OUT/'review-catalogue.json',{'review':'VERT18-C10','normal_game_enabled':False,'native_tile_px':64,'profiles':profiles,'loadouts':list(LOADS),'head_choices':[0,115,116,117,118,119,120],'weapon_palette':'Original','two_handed':'not yet authored; separate follow-up after silhouette review','accepted_weapon_geometry':['C08-B','C09-B']})
    assert all(c['visible_pixels_touch_fist'] for c in contacts),[c for c in contacts if not c['visible_pixels_touch_fist']]
    assert all(abs(h['idle']-h['target'])<=1 and h['moving_range'][1]-h['moving_range'][0]<=4 for h in heights),heights
    write(DOC/'ANALYSE-C10.json',{'review':'VERT18-C10','status':'candidate_prepared_pending_native_engine_review','normal_game_enabled':False,'body_profiles':12,'moving_and_idle_poses':156,'head_choices':7,'representative_one_hand_loadouts':5,'prepared_reference_atlases':420,'reference_poses':5460,'whole_figure_reassembly_exact':True,'hand_contacts':len(contacts),'hand_contact_failures':0,'heights':heights,'source_grip_landmarks':'C06 same-class hints locally fitted to the actual C10 pixels; visually inspected, not a new weapon model','two_handed_grips':'not authored in this stage','torso_palettes':'Original only'})
    PREVIEW.mkdir(parents=True,exist_ok=True)
    # Only asset URLs are exposed by the read-only local review server.
    import shutil
    for kind in ['html','js']:
        shutil.copy2(ROOT/f'tools/art/vertical_armor18.{kind}',PREVIEW/('apercu.html' if kind=='html' else 'app.js'))
    data=read(OUT/'review-catalogue.json')
    resources=PREVIEW/'resources';resources.mkdir(exist_ok=True)
    for profile in data['profiles']:
        for entry in profile['cases']:
            source=ROOT/entry['file'];target=resources/source.name
            if not target.exists() or sha(source)!=sha(target):shutil.copy2(source,target)
            entry['url']='resources/'+source.name
        source=image(profile['family_poses']['one']['complete']);small=Image.new('RGBA',(480,240))
        for n in range(13):small.paste(cell(source,n).crop((88,88,168,168)),(n%6*80,n//6*80))
        file=resources/(profile['id']+'-overview.png');small.save(file)
        profile['overview']='resources/'+file.name
    write(PREVIEW/'catalogue.json',data)
    # Numbered native figures alongside accepted base and heavy bodies.
    accepted=read(ART/'extension-C06/conditioned-weapons/manifest.json')
    board=Image.new('RGB',(840,688),'#1e2831');draw=ImageDraw.Draw(board);font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),14)
    draw.text((16,8),'C10 · armures intermédiaires · taille native 64 · face / dos',fill='white',font=font)
    for row,name in enumerate(NAMES):
        draw.text((16,40+row*160),NAMES[name],fill='white',font=font)
        base=next(p for p in accepted['profiles'] if p['id']==name);heavy=next(p for p in accepted['profiles'] if p['id'].startswith(name+'-lourd'))
        group=[base]+[p for p in poses['profiles'] if p['id'].startswith(name)]+[heavy]
        for col,p in enumerate(group):
            x=16+col*164;y=64+row*160;pose=p['family_poses']['one'];atlas=Image.alpha_composite(image(pose['body']),image(pose['hands']))
            draw.text((x,y),f'{p["number"]:02} · C10' if 'rank' in p else ('Base · validée' if col==0 else 'Lourde · validée'),font=font,fill='#ffcb7a' if 'rank' in p else '#bac7cf')
            for slot,n in enumerate([12,6]):
                im=cell(atlas,n).crop((64,64,192,192));board.paste(im,(x-24+slot*76,y+18),im)
    board.save(PREVIEW/'C10-native64.png')
    resources={rel(p):sha(p) for p in OUT.rglob('*') if p.is_file()}
    write(DOC/'candidate-resources-C10.json',{'review':'VERT18-C10','status':'unapproved_candidates','mode':'builtin_imagegen','normal_game_enabled':False,'sha256':resources})
    print(json.dumps({'profiles':len(profiles),'candidate_atlases':len(cases),'poses':len(cases)*13,'contacts':len(contacts),'normal_game_enabled':False}))

if __name__=='__main__':main()
