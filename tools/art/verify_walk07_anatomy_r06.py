"""Check current pose identity through equipment; inspect distinct supports."""
import hashlib,json
from pathlib import Path
from PIL import Image,ImageOps
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';ART=ROOT/'artifacts/player-walk-review-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
read=lambda p:json.loads((DOC/p).read_text('utf8'))
write=lambda p,v:(DOC/p).write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n','utf8')
assert read('review-revision.json')['revision']=='r06-anatomical-side-and-alternating-support'
results=[];total=0
for index in [10,24]:
    folder=ART/f'layered-final/CLASS-{index:02}';data=json.loads((folder/'layers.json').read_text('utf8'));expected=[];poses=[];cache={}
    for side in range(2):
        sheet=Image.open(SRC/f'rigs/CLASS-{index:02}-{side}.png').convert('RGBA')
        for col in range(7):
            core=sheet.crop((0,col*128,128,(col+1)*128));expected.append(core)
            for part in range(1,5):assert sheet.getchannel('A').crop((part*128,col*128+67,(part+1)*128,(col+1)*128)).getbbox() is None
            a=core.getchannel('A')
            lowest=lambda lo,hi:max(y for y in range(80,100) for x in range(lo,hi) if a.getpixel((x,y))>=128)
            soles=[lowest(45,64),lowest(64,86)];clearance=[95-y for y in soles]
            # A complete native pixel row below the boot is an actual gap.
            # Inspected B1-right heel clears by1px (toe higher), a low passing
            # foot rather than another planted toe. Keep measured values in QA.
            if col==2:assert clearance[0]==0 and clearance[1]>=1,(index,side,col,'A1 support',clearance)
            if col==5:assert clearance[1]==0 and clearance[0]>=1,(index,side,col,'B1 support',clearance)
            metal=[]
            if index==10:
                for lo,hi in [(45,64),(64,86)]:
                    metal.append(sum(1 for y in range(80,94) for x in range(lo,hi) if
                        (lambda p:p[3]>=128 and min(p[:3])>=90 and max(p[:3])-min(p[:3])<70)(core.getpixel((x,y)))))
                assert metal[0]>metal[1],(side,col,'silver shin ambiguous',metal)
            poses.append({'direction':'left' if side==0 else 'right','column':col,'viewer_left_right_foot_clearance_px':clearance,'silver_shin_visible_pixels_left_right':metal})
    for config in data['configs']:
        assert len(config['frames'])==14
        for n,commands in enumerate(config['frames']):
            sid,x,y=commands[0]
            if sid not in cache:cache[sid]=ImageOps.flip(Image.open(folder/data['sprites'][sid]['file']).convert('RGBA'))
            core=Image.new('RGBA',(128,128));core.paste(cache[sid],(x,y))
            assert core.tobytes()==expected[n].tobytes(),(index,config['label'],n,'fixed body changed')
            total+=1
    results.append({'id':f'CLASS-{index:02}','loadouts':len(data['configs']),'equipment_pose_checks':len(data['configs'])*14,'poses':poses})
runtime=json.loads((ROOT/'assets/animations/walk-06/manifest.json').read_text('utf8'))
for entry in runtime['files']:assert hashlib.sha256((ROOT/entry['file']).read_bytes()).hexdigest()==entry['sha256']
report={'review_id':'WALK07-r06','date':'2026-10-24','status':'technical_checks_passed_artistic_review_pending',
    'runtime_integration':False,'user_visual_acceptance':False,'pose_checks':total,'all_five_runtime_atlases_unchanged':True,
    'different_supporting_feet_verified_in_each_direction':True,'minimum_clear_native_rows_below_passing_boot':1,'legs_excluded_from_arm_ownership':True,'results':results,
    'limits':'Static pose and compositing guards; whole-cycle visual quality remains for player review. Native boards viewed, no browser or game run of r06.'}
write('anatomy-checks-r06.json',report)
sources=[]
for j in read('unarmed-jobs.json')+read('creature-jobs.json'):
    if j.get('anatomy_sources'):files=[ROOT/p for p in j['anatomy_sources'].values()]
    elif j.get('local_retouch_sources'):files=[ROOT/p for p in j['local_retouch_sources'].values()]
    elif j.get('direction_sources'):files=[Path(p) for s in j['direction_sources'].values() for p in s.values()]
    else:files=[Path(j['source_destination'])]
    for p in files:sources.append({'id':j['id'],'file':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
write('source-manifest.json',{'source_tool':'built-in image_gen; whole figures extracted and uniformly scaled; no leg grafting or mirroring','selected_source_count':len(sources),'runtime_integration':False,'entries':sources})
validation=read('validation.json');validation.update({'review_id':'WALK07-r06','date':'2026-10-24','status':report['status'],
    'source_tool':'built-in image_gen; whole-figure edits and extraction, no generation CLI',
    'approved_gait_pixels_preserved':False,'anatomy_checks':'anatomy-checks-r06.json','previous_revision_status':'r05_rejected_for_leg_identity_and_gait'})
validation.pop('frozen_gait_checks',None);validation['checks'].pop('native_protected_source_comparisons',None)
validation['checks']['complete_figures_and_native_height_checks']='Whole figures normalized by uniform scale; alternate planted/passing feet verified for both directions; silver shin remains viewer-left'
validation['checks']['current_pose_body_source_comparisons']=total
write('validation.json',validation)
print(json.dumps({'pose_checks':total,'alternate_support_checks':8,'silver_shin_checks':14,'runtime_unchanged':True,'visual_review_pending':True}))
