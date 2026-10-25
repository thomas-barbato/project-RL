"""Verify the review composition without confusing checks with art approval."""
import hashlib,json
from pathlib import Path
from PIL import Image,ImageOps,ImageDraw,ImageFont,ImageChops
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';ART=ROOT/'artifacts/player-walk-review-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
read=lambda p:json.loads((DOC/p).read_text('utf8'))
write=lambda p,v:(DOC/p).write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n','utf8')
revision=read('review-revision.json')
assert revision['revision']=='r07-exact-bastion-whole-pose-transfer'
bilateral=revision.get('leg_material_mode')=='bilateral_metal_calves'
if bilateral:assert '2 jambes en metal' in revision.get('material_change_user_quote','')
reference=ROOT/'assets/source-art/player-walk-review-01/accepted/native64'
walk=Image.open(reference/'bastion-walk-native64.png').convert('RGBA');idle=Image.open(reference/'bastion-idle-native64.png').convert('RGBA')
indices=revision.get('repair_pilot_indices',[10,24])
board=Image.new('RGB',(896,(len(indices)+1)*288),'#293132');draw=ImageDraw.Draw(board);font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
for side,row in enumerate([7,1]):
    frames=[idle.crop((row%4*64,row//4*64,row%4*64+64,row//4*64+64))]+[walk.crop((n*64,row*64,n*64+64,row*64+64)) for n in range(6)]
    for col,im in enumerate(frames):
        z=im.resize((128,128),Image.Resampling.NEAREST);board.paste(z,(col*128,side*144+16),z)
        draw.text((col*128+2,side*144+2),'Bastion '+('G' if side==0 else 'D')+' / '+('repos' if not col else str(col-1)),font=font,fill='#eeeade')
results=[];total=0;steel_checks=[];source_rows=json.loads((SRC/'rigs/manifest.json').read_text('utf8'))
for rank,index in enumerate(indices):
    folder=ART/f'layered-final/CLASS-{index:02}';data=json.loads((folder/'layers.json').read_text('utf8'));cache={};cores=[];whole=[];movement=[]
    for side in range(2):
        sheet=Image.open(SRC/f'rigs/CLASS-{index:02}-{side}.png').convert('RGBA');side_bodies=[]
        for col in range(7):
            pieces=[sheet.crop((part*128,col*128,(part+1)*128,(col+1)*128)) for part in range(5)];cores.append(pieces[0]);body=pieces[0].copy()
            for part in [pieces[1],pieces[3]]:body.alpha_composite(part)
            whole.append(body);side_bodies.append(body)
            if index in revision.get('accepted_class_indices',[]):
                if index==10:
                    frozen_path=SRC/'accepted/traqueuse-r07f/whole-body-native64.png'
                elif 11<=index<=14:
                    frozen_path=SRC/f'accepted/traqueuse-r08/CLASS-{index:02}-whole-body-native64.png'
                else:
                    assert 20<=index<=24,('accepted family requires an explicit frozen path',index)
                    frozen_path=SRC/f'accepted/metamorphe-r09b/CLASS-{index:02}-whole-body-native64.png'
                frozen=Image.open(frozen_path).convert('RGBA').crop((col*64,side*64,(col+1)*64,(side+1)*64))
                assert body.crop((32,32,96,96)).tobytes()==frozen.tobytes(),(index,side,col,'accepted whole pose changed')
            if 10<=index<=14:
                # Reviewed regions for these fourteen authored poses only.
                # A time half is NEVER an anatomical limb identifier. The
                # right-facing far calf stays silver in every walking pose.
                identified=(53,80,69,90) if side==0 else (48,77,63,88)
                opposite=((66,77,81,89) if bilateral else (69,80,82,90)) if side==0 else (63,80,77,90)
                steel=sum(1 for y in range(identified[1],identified[3]) for x in range(identified[0],identified[2]) if
                    (lambda p:p[3]>=128 and min(p[:3])>=130 and max(p[:3])-min(p[:3])<80)(body.getpixel((x,y))))
                other=sum(1 for y in range(opposite[1],opposite[3]) for x in range(opposite[0],opposite[2]) if
                    (lambda p:p[3]>=128 and min(p[:3])>=130 and max(p[:3])-min(p[:3])<80)(body.getpixel((x,y))))
                assert steel>=6,(index,side,col,'prosthesis disappeared from its identified shin',steel)
                if bilateral:
                    # Explicit user request supersedes the former one-metal-
                    # calf identity: both visible calves must retain metal.
                    assert other>=4,(index,side,col,'second calf lost requested metal',other)
                else:
                    assert other<steel,(index,side,col,'second shin material ambiguous',steel,other)
                    if side==1:assert other<=3,(index,side,col,'metal returned to opposite front calf',other)
                steel_checks.append({'class_index':index,'side':side,'column':col,'identified_region':identified,'opposite_region':opposite,
                    'identified_shin_steel_pixels':steel,'other_leg_region_steel_pixels':other})
            record=next(m for m in source_rows if m['id']==f'CLASS-{index:02}' and m['facing']==(-1 if side==0 else 1) and m['sheet_row']==col)
            mask=Image.open(ROOT/record['leg_ownership_mask']).convert('L')
            assert Image.composite(body,Image.new('RGBA',body.size),mask).tobytes()==Image.composite(pieces[0],Image.new('RGBA',body.size),mask).tobytes()
            for part in pieces[1:]:assert not any(a and b for a,b in zip(part.getchannel('A').tobytes(),mask.tobytes()))
        neutral=side_bodies[0].getchannel('A').crop((32,67,96,96)).point(lambda a:255 if a>=128 else 0)
        changes=[]
        for body in side_bodies[1:]:
            a=body.getchannel('A').crop((32,67,96,96)).point(lambda a:255 if a>=128 else 0)
            changes.append(sum(v>0 for v in ImageChops.difference(a,neutral).tobytes()))
        assert min(changes)>0,(index,side,'stationary walking silhouette')
        movement.append({'side':side,'lower_body_silhouette_pixels_changed_from_idle':changes})
    def sprite(sid):
        if sid not in cache:cache[sid]=ImageOps.flip(Image.open(folder/data['sprites'][sid]['file']).convert('RGBA'))
        return cache[sid]
    for config in data['configs']:
        assert len(config['frames'])==14
        for n,commands in enumerate(config['frames']):
            sid,x,y=commands[0];core=Image.new('RGBA',(128,128));core.paste(sprite(sid),(x,y))
            assert core.tobytes()==cores[n].tobytes(),(index,config['label'],n,'body source changed with weapon')
            total+=1
    rifle=next(c for c in data['configs'] if c['weapon']==2 and c['hands'] is None)
    for pose,commands in enumerate(rifle['frames']):
        im=Image.new('RGBA',(128,128))
        for sid,x,y in commands:im.alpha_composite(sprite(sid),(x,y))
        side,col=divmod(pose,7);y=(rank+1)*288+side*144;z=im.crop((32,32,96,96)).resize((128,128),Image.Resampling.NEAREST)
        board.paste(z,(col*128,y+16),z);draw.text((col*128+2,y+2),f'{index+1:02} fusil '+('G' if side==0 else 'D')+' / '+('repos' if not col else str(col-1)),font=font,fill='#eeeade')
    results.append({'id':f'CLASS-{index:02}','loadouts':len(data['configs']),'equipment_pose_checks':len(data['configs'])*14,'movement_geometry':movement})
board.save(ART/('r09-bastion-and-equipped-outfits-x2.png' if 20 in indices else ('r08-bastion-and-equipped-outfits-x2.png' if len(indices)>2 else 'r07-bastion-and-equipped-pilots-x2.png')))
runtime=json.loads((ROOT/'assets/animations/walk-06/manifest.json').read_text('utf8'))
for e in runtime['files']:assert hashlib.sha256((ROOT/e['file']).read_bytes()).hexdigest()==e['sha256']
for filename in ['visual-acceptance-r07f.json','visual-acceptance-traqueuse-r08.json','visual-acceptance-metamorphe-r09.json']+revision.get('accepted_creature_reviews',[]):
    acceptance_path=DOC/filename
    if acceptance_path.exists():
        for e in json.loads(acceptance_path.read_text('utf8'))['files']:
            assert hashlib.sha256((ROOT/e['file']).read_bytes()).hexdigest()==e['sha256'],'accepted whole-figure art changed'
pending=[i for i in indices if i not in revision.get('accepted_class_indices',[])]
report={'review_id':'WALK07-r07','date':'2026-10-24','status':'visual_accepted' if not pending else 'awaiting_visual_review','runtime_integration':False,'user_visual_acceptance':not pending,
    'acceptance_scope_class_indices':indices,'accepted_creature_ids':revision.get('accepted_creature_ids',[]),'full_bestiary_acceptance':False,
    'exact_motion_reference':'original canvas02 Bastion rows7 and1','whole_drawn_figures_only':True,'pose_checks':total,'leg_mask_excluded_from_arms':True,
    'all_five_runtime_atlases_unchanged':True,'results':results,'material_revision':revision.get('material_revision'),
    'leg_material_mode':revision.get('leg_material_mode','single_metal_calf'),'identified_prosthesis_checks':steel_checks,'browser_execution':False,
    'browser_limit':'Browser access policy rejected local file URL protocol; no alternative browser or server workaround attempted.',
    'artistic_limit':'Silhouette changes, matching clocks and source identity do not prove a good walk; examine synchronized comparison with the exact Bastion.'}
write('bastion-transfer-checks-r07.json',report)
sources=[]
source_jobs=read('unarmed-jobs.json')+read('creature-jobs.json')
for path in sorted(DOC.glob('creature-jobs-lot*.json')):source_jobs+=read(path.name)
for j in source_jobs:
    if j.get('bastion_transfer_sources'):files=[ROOT/p for p in j['bastion_transfer_sources'].values()]
    elif j.get('anatomy_sources'):files=[ROOT/p for p in j['anatomy_sources'].values()]
    elif j.get('local_retouch_sources'):files=[ROOT/p for p in j['local_retouch_sources'].values()]
    elif j.get('direction_sources'):files=[Path(p) for s in j['direction_sources'].values() for p in s.values()]
    else:files=[Path(j['source_destination'])]
    for p in files:sources.append({'id':j['id'],'file':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
write('source-manifest.json',{'source_tool':'built-in image_gen; whole-figure player pose transfer and anatomy-specific creature cycles; uniform scaling only','selected_source_count':len(sources),'runtime_integration':False,'entries':sources})
v=read('validation.json');v.update({'review_id':'WALK07-r07','status':revision['status'],'accepted_class_indices':revision.get('accepted_class_indices',[]),'accepted_creature_ids':revision.get('accepted_creature_ids',[]),'full_bestiary_visual_accepted':False,'date':'2026-10-24','source_tool':'built-in image_gen; exact original Bastion whole-pose transfer','previous_revision_status':'r06_rejected_by_user','bastion_transfer_checks':'bastion-transfer-checks-r07.json'})
v.pop('creatures_visual_accepted',None)
v['awaiting_visual_creature_ids']=revision.get('awaiting_visual_creature_ids',[])
v['deferred_creature_ids']=revision.get('deferred_creature_ids',[])
v.pop('anatomy_checks',None);v['checks']['complete_figures_and_native_height_checks']='Whole drawn poses; one uniform scale per direction; native height varies by at most2px for the two checked player families. Their review and creature lot01 are accepted; next creatures require separate review.'
v['checks']['current_pose_body_source_comparisons']=total;v['checks']['browser_or_game_execution_of_walk07']=False
write('validation.json',v)
print(json.dumps({'pose_checks':total,'whole_pose_outfits':len(indices),'runtime_unchanged':True,'pending_checked_outfits':pending,'accepted_creature_count':len(revision.get('accepted_creature_ids',[]))}))
