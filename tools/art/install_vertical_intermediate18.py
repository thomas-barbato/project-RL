"""Install the accepted C11/C12 intermediate outfits without repainting C09."""
from pathlib import Path
import hashlib,json
from PIL import Image
from install_vertical18 import compact

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
ART=ROOT/'assets/source-art/vertical-walk-review-18'
OUT=ROOT/'assets/animations/vertical-18'

def read(path):return json.loads(path.read_text('utf8'))
def write(path,data):path.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n','utf8')
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def rel(path):return path.relative_to(ROOT).as_posix()

def main():
    for review in ['C11-back-r02','C12']:
        accepted=read(DOC/f'acceptance-{review}.json')
        assert accepted['status'].startswith('visual_accepted')
        bank=DOC/accepted['candidate_resources']
        assert digest(bank)==accepted['candidate_resources_sha256']
        for path,sha in read(bank)['sha256'].items():assert digest(ROOT/path)==sha,path
    baseline=read(DOC/'runtime-assets-C09.json')['sha256']
    for path,sha in baseline.items():
        if path!='assets/animations/vertical-18/manifest.json':assert digest(ROOT/path)==sha,path
    manifest=read(OUT/'manifest.json')
    # Reinstallation replaces only this installer's twelve owned entries.
    data=read(ART/'grips-C12/recipes.json')
    owned={p['id'] for p in data['profiles']}
    original=[p for p in manifest['profiles'] if p['id'] not in owned]
    assert len(original)==8
    installed=[]
    for p in data['profiles']:
        families={}
        for name,pose in p['family_poses'].items():
            body=Image.open(ROOT/pose['body']).convert('RGBA')
            hands=Image.open(ROOT/pose['hands']).convert('RGBA')
            _,box=compact(Image.alpha_composite(body,hands),256)
            w,h=box[2]-box[0],box[3]-box[1];files={}
            for kind,source in [('body',body),('hands',hands)]:
                atlas=Image.new('RGBA',(w*6,h*3))
                for n in range(13):
                    atlas.paste(source.crop((n%6*256+box[0],n//6*256+box[1],n%6*256+box[2],n//6*256+box[3])),(n%6*w,n//6*h))
                path=OUT/'intermediate'/f'{p["id"]}-{name}-{kind}.png'
                path.parent.mkdir(exist_ok=True);atlas.save(path);files[kind+'_file']=rel(path)
            families[name]={**files,'cell':[w,h],'origin':list(box[:2]),'frames':[{'head_anchor':[v+64 for v in f['head_anchor']]} for f in pose['frames']]}
        fixed=[]
        for loadout,main,left in [('dual-knives',70,70),('pistol-knife',104,70)]:
            for head in [0,115,116,117,118,119,120]:
                source=ART/'knife-contact-C11/back-r02/composites'/f'{p["id"]}-{loadout}-H{head}-B-back-r02.png'
                atlas,box=compact(Image.open(source).convert('RGBA'),256)
                path=OUT/'intermediate'/f'{source.stem}.png';atlas.save(path)
                fixed.append({'main':main,'left':left,'helmet':head or None,'image':{'file':rel(path),'cell':[box[2]-box[0],box[3]-box[1]],'anchor':[128-box[0],159-box[1]],'offset':0,'shadow':True,'shadow_width':17}})
        installed.append({'id':p['id'],'outfit':p['outfit'],'anchor':[128,159],'families':families,'cases':[{**{k:v for k,v in c.items() if k!='name'},'left':False} for c in p['cases']],'fixed_loadouts':fixed})
    manifest['profiles']=original+installed
    assert len(manifest['profiles'])==20 and manifest['profiles'][:8]==original
    write(OUT/'manifest.json',manifest)
    # The existing directory inventory already includes all runtime descendants.
    inventory=read(ROOT/'tools/distribution/runtime-assets.json')
    assert rel(OUT) in inventory['directories'] and rel(OUT/'manifest.json') in inventory['manifests']
    hashes={rel(p):digest(p) for p in sorted(OUT.rglob('*')) if p.is_file()}
    previous=read(DOC/'runtime-assets-C12.json') if (DOC/'runtime-assets-C12.json').exists() else {}
    status=previous.get('status','installed_pending_runtime_checks') if previous.get('sha256')==hashes else 'installed_pending_runtime_checks'
    write(DOC/'runtime-assets-C12.json',{'review':'VERT18-C11-BACK-R02+C12','status':status,'normal_game_enabled':True,'native_tile_px':64,'previous_runtime_images_unchanged':len(baseline)-1,'copied_and_compacted_without_resampling':True,'sha256':hashes})
    print(json.dumps({'player_profiles':20,'C12_two_hand_bindings':12*74,'C11_fixed_loadouts':12*14,'C09_images_unchanged':len(baseline)-1,'runtime_files':len(hashes)}))

if __name__=='__main__':main()
