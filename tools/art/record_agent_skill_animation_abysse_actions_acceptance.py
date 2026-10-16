"""Freeze Abysse's three shown own-action compositions after user validation."""
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-45'
receipt=OUT/'visual-acceptance-01.json'
assert not receipt.exists()
review=json.loads((OUT/'review.json').read_text(encoding='utf-8'))
checks=json.loads((OUT/'verification.json').read_text(encoding='utf-8'))
assert len(review['candidates'])==3 and len(checks['controls'])==29 and len(checks['animations'])==30
assert all(c['walls_clear'] and c['timers_require_explicit_turns'] for c in checks['controls'])
files=sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*') if p.is_file()
    and p.name not in ('review.json','validation.json','review-protection.json',receipt.name))
proof={name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date':'2026-10-13','user_reply':"note que je valide la compétence qu'on vient de créer :) elle est tres bien",
    'presented_preview':'skill-animation-review-45/abysse-actions-64.gif',
    'accepted':[{'number':c['number'],'id':c['id'],'variant':c['variant']} for c in review['candidates']],
    'scope':'Abysse own actions: Allonge, Fente abyssale and Pas d’ombre compositions shown in the preceding native-scale preview and gallery. Approved BODY02 retained.',
    'retained_rules':'Previously accepted numerical rules keep their status. No new campaign damage, HP, energy, visibility, AI, loot or save implementation.',
    'campaign_integration':False,'artifact_hashes':proof},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
review['status']='visually_approved';review['acceptance']=receipt.name
for c in review['candidates']:
    c['visual_status']='visually_approved';c['acceptance']=receipt.name
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation['visual_approval']='Three Abysse own-action compositions visually approved.'
validation['acceptance']=receipt.name
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
assert all(hashlib.sha256((OUT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
print(json.dumps({'approved_actions':3,'hashed_artifacts':len(proof)}))
