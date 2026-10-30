"""C11 rear correction: anatomical weapon reflection, true hands, rear depth.

Copies the seven approved front/idle cells exactly from B-r01. Reuses entire
authored C12 backs; unsuccessful dedicated rear drawing studies are not used.
"""
from pathlib import Path
import json,math,hashlib,shutil
from PIL import Image
from finalize_knife_grip18 import cell
from finalize_pistol_tilt18 import angle as pistol_angle,helmet
from vertical_pose_fast18 import posed

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18'
BASE=ART/'knife-contact-C11';OUT=BASE/'back-r02'
DOC=ROOT/'docs/reviews/vertical-walk-18'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/knife-contact-C11'

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
def rel(p):return p.relative_to(ROOT).as_posix()
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    if (DOC/'acceptance-C11-back-r02.json').exists():
        raise SystemExit('C11-BACK-R02 is accepted and frozen; use a new revision for further visual changes.')
    original=read(PREVIEW/'catalogue.json');assert original['review']=='VERT18-C11'
    # Always rebuild from the frozen r01 baseline, not a previously published r02.
    recipes=read(ART/'extension-C06/conditioned-weapons/recipes.json');poses=read(ART/'grips-C12/conditioned/manifest.json')
    knife=read(ART/'candidate-C08/knife-angles.json');pistol=read(ART/'candidate-C09/pistol-angles.json')
    composites=OUT/'composites';composites.mkdir(exist_ok=True);resources=PREVIEW/'resources'
    images={}
    def image(path):
        if path not in images:images[path]=Image.open(ROOT/path).convert('RGBA')
        return images[path]
    gear={k:image(recipes['headgear'][k]) for k in ['front','back','opening','ownership']}
    cases=[];front_copies=0;profiles=[];transforms=[]
    for p in original['profiles']:
        body=next(v for v in poses['profiles'] if v['id']==p['id']);pose=body['family_poses']['ranged'];entries=[]
        # Reuse the entire authored forward-carry back, never graft arm strips.
        rearfile=OUT/'rear-actors'/(p['id']+'.png');rearfile.parent.mkdir(exist_ok=True);shutil.copy2(ROOT/pose['complete'],rearfile)
        updated_frames=[dict(f) for f in p['frames']]
        for n in range(6,12):
            points=sorted(pose['frames'][n]['grips'],key=lambda v:v[0])
            updated_frames[n].update(knife_grips=points,grips=points,head_anchor=pose['frames'][n]['head_anchor'],bounds=pose['frames'][n]['bounds'],grips_occluded=True,knife_foreground_rects=[],rear_body_file=rel(rearfile))
        for entry in p['cases']:
            load=entry['loadout'];head=entry['head'];oldfile=BASE/'composites'/f'{p["id"]}-{load}-H{head}-B.png';baseline=image(rel(oldfile));atlas=baseline.copy()
            weapons=[(70,False),(70,True)] if load=='dual-knives' else [(104,False),(70,True)]
            for n in range(6,12):
                f=updated_frames[n];frame=Image.new('RGBA',(256,256))
                for model,left in weapons:
                    side=int(left);binding=next(b for b in recipes['bindings'] if b['model']==model and b['left']==left)
                    pivot=[128-binding['anchor'][0],binding['anchor'][1]];r={'pivot':pivot,'target':[v+64 for v in f['knife_grips'][side]],'scale':1.,'flip':True,'behind_body':True,'left_source':left}
                    if model==70:
                        axis=math.pi-knife['knife_axes'][f'{model}-{int(left)}'];desired=math.pi-math.radians(20) if side==0 else math.radians(20);r['angle']=desired-axis
                        assert abs(math.sin(axis+r['angle']-desired))<1e-8
                    else:r['angle']=-pistol_angle(model,{'left_source':left,'angle':0},pistol)
                    frame.alpha_composite(posed(image(binding['palettes'][0]['file']),r))
                    if head==0:transforms.append({'profile':p['id'],'loadout':load,'model':model,'pose':n,**r})
                actor=cell(image(rel(rearfile)),n)
                if head:actor=helmet(actor,gear,pose['frames'][n]['head_anchor'],True,head)
                frame.alpha_composite(actor);atlas.paste(frame,(n%6*256,n//6*256))
            for n in [0,1,2,3,4,5,12]:assert cell(atlas,n).tobytes()==cell(baseline,n).tobytes();front_copies+=1
            file=composites/f'{p["id"]}-{load}-H{head}-B-back-r02.png';atlas.save(file);shutil.copy2(file,resources/file.name)
            entries.append({'loadout':load,'head':head,'A':'resources/'+oldfile.name,'B':'resources/'+file.name,'case_A':oldfile.stem+'-old-back','case_B':file.stem})
            cases.append({'id':file.stem,'file':rel(file),'kind':'C11-back-r02-corrected-depth','side':256,'foot':[128,159],'save':head in [0,120],**({'camp_group':p['id'].split('-')[0]} if head==0 and load=='pistol-knife' else {})})
            if head==0:cases.append({'id':oldfile.stem+'-old-back','file':rel(oldfile),'kind':'C11-back-r01-baseline','side':256,'foot':[128,159],'save':True})
        profiles.append({**p,'frames':updated_frames,'cases':entries})
    assert front_copies==1176 and len(cases)==192
    write(OUT/'engine-cases.json',{'review':'VERT18-C11-BACK-R02','cases':cases});write(OUT/'rear-attachments.json',{'review':'VERT18-C11-BACK-R02','normal_game_enabled':False,'front_poses_preserved_exactly':front_copies,'rear_body_drawing_changed':True,'rear_pose_source':'whole forward-carry back figures authored in C12, same outfit and canonical C10 head','weapon_reflection':'same anatomical source, reflected for the back view','rear_depth':'all weapons behind the whole rear-view actor','rear_pistol_target':'actual forward-carried hands rather than old cuff','frames':transforms})
    write(OUT/'browser-catalogue.json',{'review':'VERT18-C11','revision':'back-r02','normal_game_enabled':False,'profiles':profiles})
    write(DOC/'ANALYSE-C11-back-r02.json',{'review':'VERT18-C11-BACK-R02','status':'prepared_pending_native_review','normal_game_enabled':False,'front_only_approval':'decision-C11.json','front_and_idle_exact_copies':1176,'rear_configurations':168,'rear_body_pixels_changed':True,'rear_pose':'whole authored C12 forward-carry back, with separate weapons and no arm graft','rear_depth_corrected':True,'rear_pistol_target_corrected':True,'rear_weapon_anatomical_reflection':True,'unused_imagegen_rear_studies':2,'sources':'assets/source-art/vertical-walk-review-18/knife-contact-C11/back-r02/'})
    write(DOC/'candidate-resources-C11-back-r02.json',{'review':'VERT18-C11-BACK-R02','status':'unapproved_rear_candidates_front_approved','sha256':{rel(p):digest(p) for p in OUT.rglob('*') if p.is_file()}})
    print(json.dumps({'front_exact_copies':front_copies,'rear_candidates':168,'engine_configurations':len(cases)}))

if __name__=='__main__':main()
