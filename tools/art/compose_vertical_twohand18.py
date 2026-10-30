"""C12 shared two-hand weapons on complete intermediate-outfit poses."""
from pathlib import Path
import json,math,hashlib,shutil
from PIL import Image,ImageDraw,ImageFont
from vertical_pose_fast18 import posed
from finalize_knife_grip18 import cell
from finalize_pistol_tilt18 import helmet

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18'
OUT=ART/'grips-C12'
DOC=ROOT/'docs/reviews/vertical-walk-18'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/grips-C12'
REP={'ranged':2,'melee-wide':66,'melee-close':88}

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
def rel(p):return p.relative_to(ROOT).as_posix()
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def recipe(binding,frame,scale):
    pivot=binding['anchor'][:];support=binding['support'][:];back=frame['direction']=='back'
    if back:pivot[0]=128-pivot[0];support[0]=128-support[0]
    a,b=frame['grips'];dx,dy=support[0]-pivot[0],support[1]-pivot[1]
    angle=math.atan2(b[1]-a[1],b[0]-a[0])-math.atan2(dy,dx);cs,sn=math.cos(angle),math.sin(angle)
    target=[(a[0]+b[0])/2+64-(cs*dx-sn*dy)*scale/2,(a[1]+b[1])/2+64-(sn*dx+cs*dy)*scale/2]
    return {'pivot':pivot,'target':target,'scale':scale,'angle':angle,'flip':back,'behind_body':back,'left_source':binding['left']}

def distance(weapon,hands,point):
    x,y=[round(v)+64 for v in point];a=weapon.getchannel('A');h=hands.getchannel('A');best=999
    for yy in range(y-2,y+3):
        for xx in range(x-2,x+3):
            if h.getpixel((xx,yy))<32:continue
            for dy in range(-4,5):
                for dx in range(-4,5):
                    d=dx*dx+dy*dy
                    if d<best and a.getpixel((xx+dx,yy+dy))>=32:best=d
    return math.sqrt(best)

def main():
    if (DOC/'acceptance-C12.json').exists():
        raise SystemExit('C12 is accepted and frozen; use a new revision for further visual changes.')
    data=read(OUT/'conditioned/manifest.json');assert data['prepared_families']==36 and not data['failures']
    old=read(ART/'extension-C06/conditioned-weapons/recipes.json');bindings=[b for b in old['bindings'] if b['two_handed'] and not b['left']];assert len(bindings)==74
    source_images={}
    def image(path):
        if path not in source_images:source_images[path]=Image.open(ROOT/path).convert('RGBA')
        return source_images[path]
    gear={k:image(old['headgear'][k]) for k in ['front','back','opening','ownership']}
    composites=OUT/'composites';composites.mkdir(exist_ok=True)
    resources=PREVIEW/'resources';resources.mkdir(exist_ok=True)
    exported={}
    def resource(path):
        if path not in exported:
            file=ROOT/path;target=resources/file.name
            assert not target.exists() or file.name not in exported.values()
            if not target.exists() or digest(target)!=digest(file):shutil.copy2(file,target)
            exported[path]='resources/'+file.name
        return exported[path]
    profiles=[];engine=[];failures=[];checked=0;maximum=0.;palettes_checked=0
    for binding in bindings:
        original=image(binding['palettes'][0]['file']).getchannel('A').tobytes()
        for palette in binding['palettes']:
            assert image(palette['file']).getchannel('A').tobytes()==original
            palettes_checked+=1
    for p in data['profiles']:
        name=p['id'].split('-')[0];accepted=next(v for v in old['profiles'] if v['profile']==name);cases=[]
        family_images={family:([cell(image(v['body']),n) for n in range(13)],[cell(image(v['hands']),n) for n in range(13)]) for family,v in p['family_poses'].items()}
        for binding in bindings:
            model=binding['model'];previous=next(c for c in accepted['cases'] if c['model']==model and not c['left']);family=previous['family'];scales={f['scale'] for f in previous['frames']};assert len(scales)==1;scale=next(iter(scales));pose=p['family_poses'][family];cores,hands=family_images[family]
            source=image(binding['palettes'][0]['file']);transforms=[];weapons=[]
            for n,f in enumerate(pose['frames']):
                r=recipe(binding,f,scale);transforms.append(r);weapon=posed(source,r);weapons.append(weapon)
                if not r['behind_body']:
                    for point in f['grips']:
                        delta=distance(weapon,hands[n],point);checked+=1;maximum=max(maximum,delta)
                        if delta>2.25:failures.append({'profile':p['id'],'model':model,'family':family,'pose':n,'distance':delta,'point':point})
            cases.append({'model':model,'name':previous['name'],'family':family,'two_handed':True,'frames':transforms})
            for head in ([0,115,116,117,118,119,120] if model==REP[family] else [0]):
                file=composites/f'{p["id"]}-W{model:03}-H{head}.png';atlas=Image.new('RGBA',(1536,768))
                for n,r in enumerate(transforms):
                    body=cores[n] if not head else helmet(cores[n],gear,pose['frames'][n]['head_anchor'],6<=n<12,head)
                    frame=Image.new('RGBA',(256,256))
                    for layer in ([weapons[n],body,hands[n]] if r['behind_body'] else [body,weapons[n],hands[n]]):frame.alpha_composite(layer)
                    atlas.paste(frame,(n%6*256,n//6*256))
                atlas.save(file)
                engine.append({'id':file.stem,'file':rel(file),'kind':'C12-two-handed-'+family,'side':256,'foot':[128,159],'save':model==REP[family] and head in [0,120],**({'camp_group':name+'-'+family} if model==REP[family] and head==0 else {})})
        profiles.append({**p,'cases':cases})
        print(json.dumps({'profile':p['id'],'models':len(cases),'contact_failures_so_far':len(failures)}),flush=True)
    write(OUT/'recipes.json',{'review':'VERT18-C12','normal_game_enabled':False,'profiles':profiles,'bindings':bindings,'headgear':old['headgear'],'checks':{'weapon_models':74,'body_profiles':12,'two_finger_contact_checks':checked,'max_contact_distance_px':maximum,'limit_px':2.25,'contact_failures':failures,'palette_alpha_checks':palettes_checked,'fixed_scale_through_cycle':True}})
    write(OUT/'engine-cases.json',{'review':'VERT18-C12','cases':engine})
    browser=read(OUT/'recipes.json')
    for p in browser['profiles']:
        for pose in p['family_poses'].values():
            for key in ['body','hands']:pose[key]=resource(pose[key])
    for binding in browser['bindings']:
        for palette in binding['palettes']:palette['file']=resource(palette['file'])
    for key in ['front','back','opening','ownership']:browser['headgear'][key]=resource(browser['headgear'][key])
    write(PREVIEW/'catalogue.json',browser)
    shutil.copy2(ROOT/'tools/art/vertical_twohand18.html',PREVIEW/'apercu.html')
    shared=(ROOT/'tools/art/vertical_extension18.js').read_text('utf8');head=shared[shared.index('async function helmet('):shared.index('async function prepareWeapons(')].replace('data.weapons.headgear','data.headgear')
    script=(ROOT/'tools/art/vertical_twohand18.js').read_text('utf8').replace('// SHARED_HELMET',head);(PREVIEW/'app.js').write_text(script,'utf8')
    # Native full figures in each newly authored family, alongside one another.
    board=Image.new('RGB',(960,696),'#1e2831');draw=ImageDraw.Draw(board);font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),13)
    for i,p in enumerate(profiles):
        x=i%4*240;y=i//4*232;draw.text((x+4,y+6),f'{p["number"]:02} · '+p['id'],font=font,fill='white')
        for row,(family,model) in enumerate(REP.items()):
            sheet=Image.open(composites/f'{p["id"]}-W{model:03}-H0.png').convert('RGBA');draw.text((x+2,y+29+row*65),f'W{model:03}',font=font,fill='#f4c785')
            for col,n in enumerate([12,6]):
                im=cell(sheet,n).crop((64,88,192,168));board.paste(im,(x+40+col*88,y+22+row*65),im)
    board.save(PREVIEW/'C12-native64.png')
    report={'review':'VERT18-C12','status':'prepared_pending_contacts_or_native_review' if failures else 'prepared_pending_engine_review','normal_game_enabled':False,'whole_body_families':36,'body_profiles':12,'two_handed_models':74,'binding_configurations':12*74,'engine_configurations':len(engine),'contact_checks':checked,'contact_failures':failures,'contact_max_px':maximum,'palette_geometry_checks':palettes_checked,'weapon_colors':7,'head_choices':7,'sampler_validation':read(DOC/'sampler-check-C12.json')}
    write(DOC/'ANALYSE-C12.json',report)
    write(DOC/'candidate-resources-C12.json',{'review':'VERT18-C12','status':'unapproved_candidates','mode':'builtin_imagegen','sha256':{rel(p):digest(p) for p in OUT.rglob('*') if p.is_file()}})
    print(json.dumps({k:v for k,v in report.items() if k!='contact_failures'}))
    print(json.dumps({'contact_failures':len(failures),'first_failures':failures[:10]}))

if __name__=='__main__':main()
