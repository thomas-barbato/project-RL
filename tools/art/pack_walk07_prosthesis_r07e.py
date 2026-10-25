"""Condition complete retouched figures; do not patch or move individual limbs."""
import hashlib,json,shutil
from pathlib import Path
from PIL import Image
from pack_walk07_cycle_repair import strip
from build_player_walk_review_04 import components
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
GEN=Path('C:/Users/User/.codex/generated_images/01a16812-c028-73a3-9d94-d6e3343cb79c')
generated={1:'exec-44d9b107-878b-4a31-83b1-be82388f3b38.png',2:'exec-d594ebbc-2e0b-4f79-945e-a113f6cb4c6f.png'}
for col,name in generated.items():shutil.copy2(GEN/name,SRC/f'CLASS-10-r07e-right-pose-{col}.png')
rejected_row=SRC/'BASTION-TRANSFER-10-r07d-right-rejected.png'
if not rejected_row.exists():shutil.copy2(GEN/'exec-7c719c45-d946-442e-8ccc-c8bc67807716.png',rejected_row)
base=strip(SRC/'BASTION-TRANSFER-10-r07c-right.png',7)
corrected_row=strip(rejected_row,7)
selected=[];records=[]
for col,old in enumerate(base):
    if col in generated:
        path=SRC/f'CLASS-10-r07e-right-pose-{col}.png';im=Image.open(path).convert('RGBA')
        part=max(components(im),key=lambda p:p['area']);figure=im.crop(part['box'])
        scale=old.height/figure.height
        figure=figure.resize((round(figure.width*scale),old.height),Image.Resampling.LANCZOS)
        record={'column':col,'file':path.relative_to(ROOT).as_posix(),'whole_figure_scale':scale}
    elif col==3:
        # The whole-row edit failed in columns1/2, but this complete pose has
        # the correct calf and a clean cutout. Retain its complete drawing.
        figure=corrected_row[col];scale=old.height/figure.height
        figure=figure.resize((round(figure.width*scale),old.height),Image.Resampling.LANCZOS)
        record={'column':col,'file':rejected_row.relative_to(ROOT).as_posix(),'source_column':col,'whole_figure_scale':scale}
    else:
        figure=old;record={'column':col,'file':'assets/source-art/player-walk-review-07/BASTION-TRANSFER-10-r07c-right.png','source_column':col,'whole_figure_scale':1}
    assert figure.width<310 and figure.height<650
    selected.append(figure);records.append(record)
row=Image.new('RGBA',(7*320,768))
for col,figure in enumerate(selected):row.alpha_composite(figure,(col*320+(320-figure.width)//2,640-figure.height))
dest=SRC/'BASTION-TRANSFER-10-r07e-right.png';row.save(dest)
jobs=json.loads((DOC/'unarmed-jobs.json').read_text('utf8'));job=next(j for j in jobs if j['indices']==[10])
for side,version in [('left','r07b'),('right','r07e')]:
    p=f'assets/source-art/player-walk-review-07/BASTION-TRANSFER-10-{version}-{side}.png'
    job['bastion_transfer_sources'][side]=p;job[f'prosthesis_{side}_source']=p
job['review_revision']='r07e-same-hip-and-calf-all-poses'
(DOC/'unarmed-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
revision=json.loads((DOC/'review-revision.json').read_text('utf8'))
revision.update(material_revision='r07e',prosthesis_feedback='non le probleme est toujours là .. (alternance de jambe armurée)',
    anatomical_ownership_revision='r07c rejected: time halves do not identify limbs; preserve the same visible hip-knee-calf chain throughout each direction',
    gait_feedback='seems_ok; retain existing pose targets; no integration approval',runtime_integration=False)
revision['candidate_revisions']['10']='r07e';revision['rejected_material_revisions']=['r07c','r07d-full-row']
(DOC/'review-revision.json').write_text(json.dumps(revision,ensure_ascii=False,indent=2)+'\n','utf8')
report={'id':'CLASS-10-prosthesis-r07e','source_tool':'built-in image_gen; individual whole-figure material edits',
    'selected_left_source':job['bastion_transfer_sources']['left'],'selected_right_source':job['bastion_transfer_sources']['right'],
    'right_row_sha256':hashlib.sha256(dest.read_bytes()).hexdigest(),'pose_sources':records,
    'no_limb_patches_or_mirroring':True,'no_runtime_integration':True,
    'rejected_halo_outputs':['exec-927eb688-f988-4bab-b7df-081e8c74d023.png','exec-dc1fec19-0e21-4e4b-a6ca-6188ca886b9d.png'],
    'prompts':['prompts/CLASS-10-prosthesis-r07e-right-1.txt','prompts/CLASS-10-prosthesis-r07e-right-2.txt','prompts/CLASS-10-prosthesis-r07d-right.txt']}
(DOC/'prosthesis-sources-r07e.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n','utf8')
print(json.dumps({'id':report['id'],'whole_figures':len(selected),'runtime_integration':False}))
