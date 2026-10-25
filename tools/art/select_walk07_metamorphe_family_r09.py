"""Select material-corrected Metamorphe outfits without modifying accepted art."""
import hashlib,json,shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
GEN=Path('C:/Users/User/.codex/generated_images/01a16812-c028-73a3-9d94-d6e3343cb79c')
corrected={(21,'left'):'exec-611e5109-2920-429f-ab8a-961ea9ddf00f.png',(21,'right'):'exec-0f3c48cc-8a20-417f-a3ac-8bcfb4ef3d95.png',
 (22,'left'):'exec-93c61052-5c6e-47f2-81e0-914488b6f4e0.png',(22,'right'):'exec-61b5d040-cd10-4a12-b859-77c54bae986d.png',
 (23,'left'):'exec-0103c9ca-f668-41fa-816d-dabce3b007ee.png',(23,'right'):'exec-e6783e28-0248-4167-9632-c08ae68498d3.png'}
read=lambda p:json.loads(p.read_text('utf8'))
write=lambda p,d:p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
for stem in ['review-revision','unarmed-jobs','bastion-transfer-checks-r07','cycle-repair-packing','equipment-preview-pack']:
    p=DOC/f'{stem}-before-metamorphe-r09.json'
    if not p.exists():p.write_bytes((DOC/f'{stem}.json').read_bytes())
jobs=read(DOC/'unarmed-jobs.json');records=[]
for index in range(20,24):
    for side in ['left','right']:
        version='r09' if index==20 else 'r09b';path=SRC/f'BASTION-TRANSFER-{index:02}-{version}-{side}.png'
        if (index,side) in corrected:shutil.copy2(GEN/corrected[index,side],path)
        assert path.exists()
        j=next(j for j in jobs if j['indices']==[index]);j.setdefault('bastion_transfer_sources',{})[side]=path.relative_to(ROOT).as_posix()
        j['leg_material_mode']='canonical_outfit_materials';j['review_revision']='r09-metamorphe-family-extension'
        records.append({'index':index,'side':side,'file':path.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
            'prompt':f'prompts/CLASS-{index:02}-family-r09-{side}.txt',
            'material_prompt':f'prompts/CLASS-{index:02}-material-r09b-{side}.txt' if index!=20 else None})
write(DOC/'unarmed-jobs.json',jobs)
r=read(DOC/'review-revision.json')
r.update(status='metamorphe_family_awaiting_visual_review',material_revision='r09b-metamorphe',
    repair_pilot_indices=[10,11,12,13,14,20,21,22,23,24],accepted_class_indices=[10,11,12,13,14],
    awaiting_visual_class_indices=[20,21,22,23,24],leg_material_scope_class_indices=[10,11,12,13,14],runtime_integration=False)
r.setdefault('family_candidate_revisions',{})['metamorphe']='r09b'
r.setdefault('family_reference_modes',{})['metamorphe']='exact_original_bastion_canvas02'
for index in range(20,24):r['candidate_revisions'][str(index)]='r09' if index==20 else 'r09b'
write(DOC/'review-revision.json',r)
write(DOC/'metamorphe-family-sources-r09.json',{'review_id':'WALK07-ME-FAMILY-r09b',
    'source_tool':'built-in image_gen; whole Metamorphe pose target plus canonical outfit identity and material correction',
    'new_candidates':[20,21,22,23],'previous_pilot':24,'metamorphe_visual_acceptance':False,'runtime_integration':False,'entries':records})
print(json.dumps({'new_metamorphe_outfits':4,'preserved_pilot':24,'accepted_traqueuse_indices':list(range(10,15))}))
