"""Freeze accepted C09-B geometry and reference compositions, keeping C08-B."""
from pathlib import Path
import hashlib,json,math
from PIL import Image,ImageChops
from finalize_knife_grip18 import angle as knife_angle,posed,cell

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
ART=ROOT/'assets/source-art/vertical-walk-review-18'
OUT=ART/'candidate-C09'

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def angle(model,frame,definition):
    if not 101<=model<=112:return frame['angle']
    left=frame['left_source']
    return definition['baseline_angles'][str(int(left))]+(1 if left else -1)*math.radians(definition['outward_degrees'])

def apply(manifest,definition):
    changed=0
    for p in manifest['profiles']:
        for c in p['cases']:
            if not 101<=c['model']<=112:continue
            for f in c['frames']:f['angle']=angle(c['model'],f,definition);changed+=1
    assert changed==2496
    if 'VERT18-C09-B' not in manifest.setdefault('accepted_overrides',[]):manifest['accepted_overrides'].append('VERT18-C09-B')
    return changed

def helmet(image,gear,anchor,back,model):
    image=image.copy();x,y=[round(v)+64-16 for v in anchor]
    original=image.crop((x,y,x+32,y+32));mask=gear['ownership'].getchannel('A').point(lambda a:255 if a else 0)
    image.paste((0,0,0,0),(x,y,x+32,y+32),mask)
    cover=gear['back' if back else 'front'].crop(((model-115)*32,0,(model-114)*32,32))
    opening=gear['opening'].crop(((model-115)*32,0,(model-114)*32,32)).getchannel('A').point(lambda a:255 if a else 0)
    restore=ImageChops.multiply(mask,cover.getchannel('A').point(lambda a:255 if a<220 else 0)) if back and model in [115,119] else Image.new('L',(32,32)) if back else opening
    patch=image.crop((x,y,x+32,y+32));patch.paste(original,(0,0),restore);image.paste(patch,(x,y));image.alpha_composite(cover,(x,y))
    return image

def main():
    accepted=read(DOC/'acceptance-C09.json');assert accepted['selected']=='B'
    for name in ['candidate-resources-C06','candidate-resources-C08','candidate-resources-C09']:
        bank=DOC/(name+'.json')
        if bank.exists():
            for path,sha in read(bank)['sha256'].items():assert digest(ROOT/path)==sha,path
    OUT.mkdir(parents=True,exist_ok=True);definition_path=OUT/'pistol-angles.json'
    if definition_path.exists():
        assert digest(definition_path)==accepted['approved_geometry_sha256'];definition=read(definition_path)
    else:
        preview=read(ROOT/'artifacts/vertical-walk-review-18/current/pistol-C09/catalogue.json');values={False:set(),True:set()}
        for p in preview['weapons']['profiles']:
            for c in p['cases']:
                if 101<=c['model']<=112:
                    for f in c['frames']:values[f['left_source']].add(f['angle'])
        assert all(len(v)==1 for v in values.values())
        definition={'review':'VERT18-C09','selected':'B','outward_degrees':preview['pistol_tilt_degrees'],'baseline_angles':{str(int(k)):next(iter(v)) for k,v in values.items()},'pistol_models':list(range(101,113)),'body_profiles':[p['id'] for p in preview['poses']['profiles']],'knife_geometry':'assets/source-art/vertical-walk-review-18/candidate-C08/knife-angles.json'}
        assert definition['outward_degrees']==10;write(definition_path,definition)
    recipes=read(ART/'extension-C06/conditioned-weapons/recipes.json');poses=read(ART/'extension-C06/conditioned-weapons/manifest.json');knife=read(ROOT/definition['knife_geometry'])
    refs=OUT/'composites-original';refs.mkdir(exist_ok=True);images={}
    def image(path):
        if path not in images:images[path]=Image.open(ROOT/path).convert('RGBA')
        return images[path]
    gear={k:image(recipes['headgear'][k]) for k in ['front','back','opening','ownership']};count=0
    for p in poses['profiles']:
        source=p['family_poses']['one'];body,hands=image(source['body']),image(source['hands']);cases=next(q for q in recipes['profiles'] if q['profile']==p['id'])['cases']
        loadouts=[(n,None,None) for n in range(101,113)]+[(104,None,h) for h in range(115,121)]+[(104,l,None) for l in [69,70,104]]
        for main,left,head in loadouts:
            name=f'{p["id"]}-W{main:03}' if left is None else f'{p["id"]}-DUAL-{main}-{left}'
            if head:name+=f'-H{head}'
            file=refs/(name+'.png')
            if not file.exists():
                atlas=Image.new('RGBA',(1536,768))
                for n in range(13):
                    frame=cell(body,n)
                    if head:frame=helmet(frame,gear,source['frames'][n]['head_anchor'],6<=n<12,head)
                    for model,is_left in [(main,False)]+([] if left is None else [(left,True)]):
                        case=next(c for c in cases if c['model']==model and c['left']==is_left);r=dict(case['frames'][n]);r['angle']=knife_angle(model,r,knife);r['angle']=angle(model,r,definition)
                        binding=next(b for b in recipes['bindings'] if b['model']==model and b['left']==r['left_source'])
                        frame.alpha_composite(posed(image(binding['palettes'][0]['file']),r))
                    frame.alpha_composite(cell(hands,n));atlas.paste(frame,(n%6*256,n//6*256))
                atlas.save(file)
            count+=1
    assert count==168
    sha={p.relative_to(ROOT).as_posix():digest(p) for p in sorted(OUT.rglob('*')) if p.is_file()}
    write(DOC/'candidate-resources-C09.json',{'review':'VERT18-C09','selected':'B','source_art_redrawn':False,'reference_atlases':count,'sha256':sha})
    accepted.update(approved_geometry=definition_path.relative_to(ROOT).as_posix(),approved_geometry_sha256=digest(definition_path),candidate_resources='candidate-resources-C09.json');write(DOC/'acceptance-C09.json',accepted)
    print(json.dumps({'review':'VERT18-C09','selected':'B','reference_atlases':count,'whole_reference_poses':count*13,'frozen_files':len(sha)}))

if __name__=='__main__':main()
