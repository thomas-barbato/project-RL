"""Freeze the explicitly accepted ANIM-S21 animations, independent of navigation."""
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-50'
receipt=OUT/'visual-acceptance-01.json'
assert not receipt.exists()
review=json.loads((OUT/'review.json').read_text(encoding='utf-8'))
checks=json.loads((OUT/'verification.json').read_text(encoding='utf-8'))
assert review['status']=='awaiting_visual_review'
assert [(c['number'],c['id']) for c in review['candidates']]==[(51,'BM01'),(52,'BM02')]
assert len(checks['controls'])==22 and len(checks['animations'])==23
assert all(c['world_vfx_walls_clear'] and c['turn_timers_and_icon_expiry_checked'] for c in checks['controls'])
pack=json.loads((OUT/'drawing-pack.json').read_text(encoding='utf-8'))
assert hashlib.sha256((OUT/'jaw-source.png').read_bytes()).hexdigest()==pack['source_sha256']
for name in ['index.html','cote-a-cote.html']:
    p=OUT/name
    if p.exists():p.write_text(p.read_text(encoding='utf-8').replace('rendu à valider','rendus validés').replace('-A01 · proposé','-A01 · validé'),encoding='utf-8')
excluded={'review.json','validation.json','review-protection.json',receipt.name}
files=sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*') if p.is_file() and p.name not in excluded and 'native-timing-draft' not in p.parts)
proof={name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
record={'date':'2026-10-13','user_reply':'ok ca me va :) je valide',
    'presented_preview':'skill-animation-review-50/index.html',
    'accepted':review['candidates'],
    'scope':'BM01-A01 colored fire damage and persistent buff icon; BM02-A01 closing vampire jaw and healing feedback, common recoil, physical wound blood, directions and shown buff timer cases.',
    'numeric_mechanics':'Previously accepted cards retained. Prepared hit20, total34 and heal8 do not establish real weapon/monster statistics or new balance.',
    'navigation_repair':'Subsequent report that native .ps1 link displayed source is handled by a direct browser comparison; navigation repair is distinct from animation approval.',
    'pending_cases':review['pending_cases'],'campaign_integration':False,'artifact_hashes':proof}
receipt.write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
review.update(status='visually_approved',acceptance=receipt.name,accepted_date='2026-10-13')
for c in review['candidates']:c.update(visual_status='visually_approved',acceptance=receipt.name)
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation.update(visual_status='visually_approved',acceptance=receipt.name,visual_approval='BM01-A01 and BM02-A01 accepted.')
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
assert all(hashlib.sha256((OUT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
print(json.dumps({'approved':['BM01-A01','BM02-A01'],'hashed_artifacts':len(proof),'campaign_integration':False}))
