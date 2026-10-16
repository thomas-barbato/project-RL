"""Freeze ANIM-S20's presented swap and domination compositions."""
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-49'
receipt=OUT/'visual-acceptance-01.json'
assert not receipt.exists(), 'Accord déjà consigné ; préserver la preuve.'
review=json.loads((OUT/'review.json').read_text(encoding='utf-8'))
checks=json.loads((OUT/'verification.json').read_text(encoding='utf-8'))
assert review['status']=='awaiting_visual_review'
assert [(c['number'],c['id'],c['variant']) for c in review['candidates']]==[(49,'HY04','HY04-A01'),(50,'HY06','HY06-A01')]
assert len(checks['controls'])==27 and len(checks['animations'])==28
assert all(c['walls_clear'] and c['timers_require_explicit_turns'] and c['positions_cargo_or_status_targets_checked'] for c in checks['controls'])
pack=json.loads((OUT/'drawing-pack.json').read_text(encoding='utf-8'))
for name,digest in pack['sources'].items():
    assert hashlib.sha256((OUT/name).read_bytes()).hexdigest()==digest
html_path=OUT/'index.html'
html=html_path.read_text(encoding='utf-8').replace('rendu à valider','rendus validés').replace('-A01 · proposé','-A01 · validé')
html_path.write_text(html,encoding='utf-8')
excluded={'review.json','validation.json','review-protection.json',receipt.name}
files=sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*') if p.is_file() and p.name not in excluded)
proof={name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
record={
    'date':'2026-10-13','user_reply':'parfait je valide :)',
    'presented_preview':'skill-animation-review-49/echange-domination-64.gif',
    'accepted':[{'number':c['number'],'id':c['id'],'name':c['name'],'variant':c['variant']} for c in review['candidates']],
    'scope':'Presented rapid dark-fragment disappearance/reformation of exchanged player/drone, and persistent upright purple overhead domination symbol, common victim reaction and shown directions, walls, cargo, replacement and expiry cases.',
    'retained_rules':'Previously accepted rules retain their status. No new global distance metric, intermediate obstacle traversal, failed-control costs, monster AI or campaign mechanics are approved by this visual review.',
    'temporary_actors':'Existing32px monster/drone drawings in the fixture remain scheduled for adaptation after skills/animations.',
    'campaign_integration':False,'artifact_hashes':proof
}
receipt.write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
review.update(status='visually_approved',acceptance=receipt.name,accepted_date='2026-10-13')
for candidate in review['candidates']:
    candidate.update(visual_status='visually_approved',acceptance=receipt.name)
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation.update(visual_status='visually_approved',visual_approval='HY04-A01 and HY06-A01 accepted.',acceptance=receipt.name)
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
assert all(hashlib.sha256((OUT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
print(json.dumps({'approved':['HY04-A01','HY06-A01'],'hashed_artifacts':len(proof),'campaign_integration':False}))
