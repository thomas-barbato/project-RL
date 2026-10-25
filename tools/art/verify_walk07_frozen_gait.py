"""Prove every proposed loadout retains its OWN approved head and leg poses."""
import hashlib,json
from pathlib import Path
from PIL import Image,ImageOps
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';ART=ROOT/'artifacts/player-walk-review-07'
read=lambda file:json.loads((DOC/file).read_text('utf8'))
write=lambda file,value:(DOC/file).write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n','utf8')
assert read('review-revision.json')['revision']=='r05-local-retouch-frozen-approved-gait','Historical r05 check only; use verify_walk07_anatomy_r06.py for r06'
manifest=json.loads((ROOT/'assets/source-art/player-walk-review-07/rigs/manifest.json').read_text('utf8'))
results=[];total=0
for index,slug,stage in [(10,'traqueuse',0),(24,'metamorphe',4)]:
    accepted=Image.open(ROOT/f'assets/animations/walk-06/{slug}-rifle.png').convert('RGBA')
    folder=ART/f'layered-final/CLASS-{index:02}';data=json.loads((folder/'layers.json').read_text('utf8'));cache={}
    for config in data['configs']:
        assert len(config['frames'])==14
        for pose,commands in enumerate(config['frames']):
            sid,x,y=commands[0]
            if sid not in cache:cache[sid]=ImageOps.flip(Image.open(folder/data['sprites'][sid]['file']).convert('RGBA'))
            core=Image.new('RGBA',(128,128));core.paste(cache[sid],(x,y))
            column=pose%7;row=stage*128+(pose//7)*64
            expected=Image.new('RGBA',(128,128));expected.paste(accepted.crop((column*64,row,column*64+64,row+64)),(32,32))
            for bounds in [(0,0,128,52),(0,67,128,128)]:
                assert core.crop(bounds).tobytes()==expected.crop(bounds).tobytes(),(index,config['label'],pose,'protected source changed')
            total+=1
    for side in range(2):
        sheet=Image.open(ROOT/f'assets/source-art/player-walk-review-07/rigs/CLASS-{index:02}-{side}.png').convert('RGBA')
        for pose in range(7):
            for part in range(1,5):assert sheet.getchannel('A').crop((part*128,pose*128+67,part*128+128,(pose+1)*128)).getbbox() is None
    records=[m for m in manifest if m['id']==f'CLASS-{index:02}']
    results.append({'id':f'CLASS-{index:02}','own_approved_source':f'assets/animations/walk-06/{slug}-rifle.png',
        'loadouts':len(data['configs']),'pose_checks':len(data['configs'])*14,'head_pixels_exact':True,'lower_body_pixels_exact':True,
        'arm_and_finger_sources_contain_no_protected_leg_pixels':True,
        'pixels_previously_assigned_to_arms_restored_to_body':sum(r.get('protected_lower_pixels_restored',0) for r in records)})
runtime=json.loads((ROOT/'assets/animations/walk-06/manifest.json').read_text('utf8'))
for entry in runtime['files']:assert hashlib.sha256((ROOT/entry['file']).read_bytes()).hexdigest()==entry['sha256']
report={'review_id':'WALK07-r05','date':'2026-10-24','status':'technical_checks_passed_artistic_review_pending',
    'runtime_integration':False,'user_visual_acceptance':False,'pose_checks':total,'frozen_native_rows':[[0,20],[35,64]],
    'edited_native_rows':[20,35],'all_five_accepted_runtime_atlases_unchanged':True,'results':results,
    'occlusion_note':'Actual held weapons and posed arms can overlap the protected body; the body source itself remains exact.',
    'javascript_checks':'renderer-checks.json','native_boards':['CLASS-10-r05-protected-poses-x2.png','CLASS-24-r05-protected-poses-x2.png']}
write('frozen-gait-checks-r05.json',report)
jobs=read('unarmed-jobs.json')+read('creature-jobs.json');sources=[]
for job in jobs:
    if job.get('local_retouch_sources'):
        files=[ROOT/file for file in job['local_retouch_sources'].values()]
    elif job.get('direction_sources'):files=[Path(file) for side in job['direction_sources'].values() for file in side.values()]
    else:files=[Path(job['source_destination'])]
    for p in files:sources.append({'id':job['id'],'file':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
write('source-manifest.json',{'source_tool':'built-in image_gen for local upper-body restoration; accepted native poses preserved elsewhere',
    'selected_source_count':len(sources),'runtime_integration':False,'entries':sources})
validation=read('validation.json');validation.update({'review_id':'WALK07-r05','status':report['status'],'date':'2026-10-24',
    'approved_gait_pixels_preserved':True,'frozen_gait_checks':'frozen-gait-checks-r05.json','previous_revision_status':'r04_rejected_by_user'})
validation['checks']['native_protected_source_comparisons']=total
validation['checks']['complete_figures_and_native_height_checks']='Approved head and lower-body RGBA exactly preserved; only native rows20..34 retouched'
validation.pop('native_height_ranges',None)
write('validation.json',validation)
print(json.dumps({'pose_checks':total,'lower_body_exact':True,'head_exact':True,
    'leg_pixels_removed_from_arm_ownership':sum(r['pixels_previously_assigned_to_arms_restored_to_body'] for r in results),
    'runtime_atlases_unchanged':True,'user_visual_acceptance':False}))
