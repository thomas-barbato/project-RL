"""Freeze ANIM-S19's two presented compositions after explicit user approval."""
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-48'
receipt=OUT/'visual-acceptance-01.json'
assert not receipt.exists(), 'Accord déjà consigné ; préserver la preuve.'
review=json.loads((OUT/'review.json').read_text(encoding='utf-8'))
checks=json.loads((OUT/'verification.json').read_text(encoding='utf-8'))
assert review['status']=='awaiting_visual_review'
assert [(c['number'],c['id'],c['variant']) for c in review['candidates']]==[(47,'M02','M02-A01'),(48,'HY07','HY07-A01')]
assert len(checks['controls'])==27 and len(checks['animations'])==28
assert all(c['walls_clear'] and c['timers_require_explicit_turns'] and c['instant_vfx_end_before_frame35'] for c in checks['controls'])
pack=json.loads((OUT/'drawing-pack.json').read_text(encoding='utf-8'))
for atlas in pack['atlases']:
    assert hashlib.sha256((OUT/(atlas['kind']+'-source.png')).read_bytes()).hexdigest()==atlas['source_sha256']
html_path=OUT/'index.html'
html=html_path.read_text(encoding='utf-8')
html=html.replace('rendu à valider','rendus validés').replace('-A01 · proposé','-A01 · validé')
html_path.write_text(html,encoding='utf-8')
excluded={'review.json','validation.json','review-protection.json',receipt.name}
files=sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*') if p.is_file() and p.name not in excluded)
proof={name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
record={
    'date':'2026-10-13','user_reply':'je valide',
    'presented_preview':'skill-animation-review-48/hybrides-64.gif',
    'accepted':[{'number':c['number'],'id':c['id'],'name':c['name'],'variant':c['variant']} for c in review['candidates']],
    'scope':'Presented M02 green projectile, single-target impact and short form-extension pulse in human and all five accepted forms; HY07 red drone core, fire and fragments, four directions and shown wall/refusal/ally-protection cases.',
    'retained_rules':'Previously accepted numerical rules and descriptions retain their status. No additional curves, global geometry, monster appearances or campaign mechanics are approved by this visual review.',
    'temporary_actors':'Existing32px monster/drone drawings in the fixture remain scheduled for adaptation after skills/animations.',
    'campaign_integration':False,'artifact_hashes':proof
}
receipt.write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
review.update(status='visually_approved',acceptance=receipt.name,accepted_date='2026-10-13')
for candidate in review['candidates']:
    candidate.update(visual_status='visually_approved',acceptance=receipt.name)
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation.update(visual_status='visually_approved',visual_approval='M02-A01 and HY07-A01 accepted.',acceptance=receipt.name)
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
assert all(hashlib.sha256((OUT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
print(json.dumps({'approved':['M02-A01','HY07-A01'],'hashed_artifacts':len(proof),'campaign_integration':False}))
