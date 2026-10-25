"""Select the four new review outfits, preserving the accepted pilot exactly."""
import hashlib,json,shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
GEN=Path('C:/Users/User/.codex/generated_images/01a16812-c028-73a3-9d94-d6e3343cb79c')
files={
 (11,'left'):'exec-54b8e53f-f595-4921-a9b9-37e846db50c2.png',
 (11,'right'):'exec-04cd27fc-a7c5-46cd-be80-370733cb95ee.png',
 (12,'left'):'exec-c34c4859-c076-4477-ac65-640f4eaf4c0f.png',
 (12,'right'):'exec-0b9583e8-64d0-43ea-89ea-26131e3ffc4e.png',
 (13,'left'):'exec-0c7b7017-39b3-4053-859a-10b4c2db7c4e.png',
 (13,'right'):'exec-952d1890-32dc-459a-8c5e-ba38f72cd94f.png',
 (14,'left'):'exec-ba574e4b-1c39-4f74-a55c-a14ca1bdb6ef.png',
 (14,'right'):'exec-d1549aa6-65bd-4eb3-8ca4-02465ecc07a0.png'}
read=lambda p:json.loads(p.read_text('utf8'))
write=lambda p,d:p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
for stem in ['review-revision','unarmed-jobs','bastion-transfer-checks-r07','cycle-repair-packing','equipment-preview-pack']:
    p=DOC/f'{stem}-before-family-r08.json'
    if not p.exists():p.write_bytes((DOC/f'{stem}.json').read_bytes())
records=[];jobs=read(DOC/'unarmed-jobs.json')
for (index,side),name in files.items():
    p=SRC/f'BASTION-TRANSFER-{index:02}-r08-{side}.png';shutil.copy2(GEN/name,p)
    j=next(j for j in jobs if j['indices']==[index])
    j.setdefault('bastion_transfer_sources',{})[side]=p.relative_to(ROOT).as_posix()
    j['leg_material_mode']='bilateral_metal_calves';j['review_revision']='r08-traqueuse-family-extension'
    records.append({'index':index,'side':side,'file':p.relative_to(ROOT).as_posix(),
        'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'generated_file':str(GEN/name),
        'prompt':f'prompts/CLASS-{index:02}-family-r08-{side}.txt'})
write(DOC/'unarmed-jobs.json',jobs)
r=read(DOC/'review-revision.json')
r.update(status='extensions_awaiting_visual_review',material_revision='r08-family',repair_pilot_indices=[10,11,12,13,14,24],
    accepted_class_indices=[10],awaiting_visual_class_indices=[11,12,13,14,24],
    accepted_pilot_review='visual-acceptance-r07f.json',family_reference_modes={'traqueuse':'accepted_traqueuse_r07f'},
    family_candidate_revisions={'traqueuse':'r08'},runtime_integration=False)
for index in [11,12,13,14]:r['candidate_revisions'][str(index)]='r08'
write(DOC/'review-revision.json',r)
write(DOC/'traqueuse-family-sources-r08.json',{'review_id':'WALK07-TR-FAMILY-r08',
    'source_tool':'built-in image_gen; accepted whole-figure motion plus canonical outfit identity',
    'accepted_pilot':10,'new_candidates':[11,12,13,14],'runtime_integration':False,'entries':records})
print(json.dumps({'accepted_pilot_unchanged':10,'new_outfits':4,'whole_pose_source_rows':8}))
