"""Install accepted C05/C06 pixels as compact runtime banks, without repainting."""
from pathlib import Path
import hashlib,json,shutil
from PIL import Image
from finalize_knife_grip18 import apply as apply_knife_c08
from finalize_pistol_tilt18 import apply as apply_pistol_c09

ROOT=Path(__file__).resolve().parents[2]
SRC=ROOT/'assets/source-art/vertical-walk-review-18'
OUT=ROOT/'assets/animations/vertical-18'
DOC=ROOT/'docs/reviews/vertical-walk-18'

def read(path):return json.loads(Path(path).read_text('utf8'))
def write(path,value):Path(path).write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n','utf8')
def rel(path):return path.relative_to(ROOT).as_posix()

def compact(sheet,side,extra=None):
    cells=[sheet.crop((n%6*side,n//6*side,(n%6+1)*side,(n//6+1)*side)) for n in range(13)]
    boxes=[im.getchannel('A').getbbox() for im in cells]
    if extra:boxes+=extra
    bounds=(max(0,min(b[0] for b in boxes if b)-1),max(0,min(b[1] for b in boxes if b)-1),min(side,max(b[2] for b in boxes if b)+1),min(side,max(b[3] for b in boxes if b)+1))
    w,h=bounds[2]-bounds[0],bounds[3]-bounds[1];out=Image.new('RGBA',(w*6,h*3))
    for n,im in enumerate(cells):
        part=im.crop(bounds);out.paste(part,(n%6*w,n//6*h))
        restored=Image.new('RGBA',(side,side));restored.paste(part,bounds[:2]);assert restored.tobytes()==im.tobytes()
    return out,bounds

def main():
    if (DOC/'acceptance-C12.json').exists():
        raise SystemExit('C09 is preserved; install accepted intermediate outfits with install_vertical_intermediate18.py.')
    frozen=read(DOC/'candidate-resources-C06.json')['sha256']
    for name,digest in frozen.items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest,name
    # NPC-01 and the C05 references retain their own, earlier approval bank.
    previous=read(DOC/'acceptance-C05.json')['sha256']
    for name,digest in previous.items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest,name
    acceptance=DOC/'acceptance-C06.json'
    if not acceptance.exists():
        write(acceptance,{'review':'VERT18-C06','date':'2026-10-29','status':'visual_accepted_pending_integration','user_quote':'ok, :) je valide :)','scope':read(DOC/'ANALYSE-C06.json')['scope'],'normal_game_enabled':False,'accepted_candidate_hashes':frozen,'previous_approvals_preserved':True,'not_approved':['intermediate armor bodies absent from C05','legacy Technomancien','other NPC bodies','forms and drones','new drawings beyond the frozen C06 bank']})
    OUT.mkdir(parents=True,exist_ok=True)
    recipes=read(SRC/'extension-C06/conditioned-weapons/recipes.json');poses=read(SRC/'extension-C06/conditioned-weapons/manifest.json')
    profiles=[];copies=[]
    for p in poses['profiles']:
        families={}
        for family,data in p['family_poses'].items():
            body=Image.open(ROOT/data['body']).convert('RGBA');hands=Image.open(ROOT/data['hands']).convert('RGBA')
            # Both layers share one crop and all source pixels, including fringe.
            complete=Image.alpha_composite(body,hands);_,box=compact(complete,256)
            w,h=box[2]-box[0],box[3]-box[1];paths={}
            for kind,source in [('body',body),('hands',hands)]:
                target=Image.new('RGBA',(w*6,h*3))
                for n in range(13):target.paste(source.crop((n%6*256+box[0],n//6*256+box[1],n%6*256+box[2],n//6*256+box[3])),(n%6*w,n//6*h))
                path=OUT/'players'/f'{p["id"]}-{family}-{kind}.png';path.parent.mkdir(exist_ok=True);target.save(path);paths[kind+'_file']=rel(path)
            families[family]={**paths,'cell':[w,h],'origin':list(box[:2]),'frames':[{'head_anchor':[v+64 for v in f['head_anchor']]} for f in data['frames']]}
        r=next(r for r in recipes['profiles'] if r['profile']==p['id'])
        cases=[{k:v for k,v in c.items() if k!='name'} for c in r['cases']]
        body=p['selector']['body'];rank=4 if p['selector']['equipment'][0]==103 else 0
        profiles.append({'id':p['id'],'outfit':(body-3)*5+rank,'families':families,'cases':cases})
    bindings=[]
    for b in recipes['bindings']:
        colors=[]
        for c in b['palettes']:
            source=ROOT/c['file'];target=OUT/'weapons'/source.name;target.parent.mkdir(exist_ok=True);shutil.copy2(source,target);colors.append({'index':c['index'],'file':rel(target)})
        bindings.append({'model':b['model'],'left':b['left'],'colors':colors})
    gear={}
    for key in ['front','back','opening','ownership']:
        target=OUT/'heads'/Path(recipes['headgear'][key]).name;target.parent.mkdir(exist_ok=True);shutil.copy2(ROOT/recipes['headgear'][key],target);gear[key+'_file']=rel(target)
    monsters=[];m=read(SRC/'extension-C06/conditioned-monsters/manifest.json')['profiles'];colors=read(SRC/'extension-C06/conditioned-monsters/palettes/manifest.json')['entries']
    canonical=read(ROOT/'assets/sprites/campaign-monsters/manifest.json')['entries']
    for c in colors:
        p=next(p for p in m if p['number']==c['number']);source=Image.open(ROOT/c['file']).convert('RGBA');image,box=compact(source,c['side']);target=OUT/'monsters'/f'MON-{c["number"]:03}-v{c["variant"]}.png';target.parent.mkdir(exist_ok=True);image.save(target)
        art=next(e for e in canonical if e['number']==c['number'] and e['variant']==0)
        monsters.append({'number':c['number'],'variant':c['variant'],'file':rel(target),'cell':[box[2]-box[0],box[3]-box[1]],'anchor':[p['foot'][0]-box[0],p['foot'][1]-box[1]],'offset':p['front_vertical_offset'],'shadow':p['shadow'],'shadow_width':art.get('shadow_width',17)})
    old=read(SRC/'candidate-C05/manifest.json')['profiles'];npc=next(p for p in old if p['selector']=={'kind':'camp_npc','index':1});sheet=Image.open(ROOT/npc['file']).convert('RGBA');idle=Image.open(ROOT/npc['standing']).convert('RGBA');atlas=Image.new('RGBA',(768,384));atlas.paste(sheet,(0,0));atlas.paste(idle,(0,256));atlas,box=compact(atlas,128);target=OUT/'NPC-01.png';atlas.save(target)
    npc={'file':rel(target),'cell':[box[2]-box[0],box[3]-box[1]],'anchor':[64-box[0],96-box[1]],'offset':0,'shadow':True,'shadow_width':17}
    manifest={'format_version':1,'native_tile_px':64,'review':'VERT18-C06','profiles':profiles,'bindings':bindings,'gear':gear,'monsters':monsters,'npc':npc}
    c08=DOC/'acceptance-C08.json';override=None
    if c08.exists():
        accepted=read(c08)
        if accepted.get('selected')=='B' and accepted.get('status','').startswith('visual_accepted'):
            definition=ROOT/accepted['approved_geometry']
            assert hashlib.sha256(definition.read_bytes()).hexdigest()==accepted['approved_geometry_sha256']
            apply_knife_c08(manifest,read(definition));override='C08'
    c09=DOC/'acceptance-C09.json'
    if c09.exists():
        accepted=read(c09)
        if accepted.get('selected')=='B' and accepted.get('status','').startswith('visual_accepted'):
            assert override=='C08','C09 requires accepted knife B'
            definition=ROOT/accepted['approved_geometry']
            assert hashlib.sha256(definition.read_bytes()).hexdigest()==accepted['approved_geometry_sha256']
            apply_pistol_c09(manifest,read(definition));override='C09'
    write(OUT/'manifest.json',manifest)
    # Package only runtime banks, never source art or preparation captures.
    plan=ROOT/'tools/distribution/runtime-assets.json';d=read(plan);name=rel(OUT/'manifest.json')
    if name not in d['manifests']:d['manifests'].append(name)
    directory=rel(OUT)
    if directory not in d['directories']:d['directories'].append(directory)
    write(plan,d)
    installed={rel(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(OUT.rglob('*')) if p.is_file()}
    write(DOC/f'runtime-assets-{override or "C06"}.json',{'review':f'VERT18-{override or "C06"}','copied_and_compacted_without_resampling':True,'native_tile_px':64,'sha256':installed,'source_candidate_hashes_verified':len(frozen),'status':'installed_pending_runtime_checks'})
    print(json.dumps({'profiles':len(profiles),'monsters':len(monsters),'runtime_files':len(installed),'frozen_sources_verified':len(frozen)}))

if __name__=='__main__':main()
