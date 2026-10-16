"""Record review41's two bodies and shown transitions, independently of combat integration."""
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-41'
receipt=OUT/'visual-acceptance-01.json';assert not receipt.exists()
review=json.loads((OUT/'review.json').read_text(encoding='utf-8'))
assert {c['variant'] for c in review['candidates']}=={'TR63-A01','TR64-A01'}
assert all(c['visual_status']==c['body_status']=='awaiting_visual_review' for c in review['candidates'])
checks=json.loads((OUT/'verification.json').read_text(encoding='utf-8'))
assert len(checks['controls'])==14 and len(checks['animations'])==9
assert all(c['walls_clear'] and c['timers_require_explicit_turns'] for c in checks['controls'])
files=sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*') if p.is_file()
    and p.name not in ('review.json','validation.json','review-protection.json','palette-check-possession.gif')
    and p.suffix in ('.png','.gif','.rs','.py','.txt','.html','.js','.cjs','.json','.ps1'))
proof={name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({'date':'2026-10-13','user_reply':'ok je valide :)',
    'presented_preview':'skill-animation-review-41/transformations-haut-64.gif',
    'presented_gallery':'skill-animation-review-41/index.html',
    'accepted':[{'number':c['number'],'id':c['id'],'body':c['body_variant'],'transition':c['variant'],'native_tile_size':64} for c in review['candidates']],
    'scope':'Both direct64px bodies and shown entry/return/renewal transitions, four map orientations and obstacle clipping. Own actions are not included.',
    'retained_rules':'Previously agreed numerical rules keep their status; no actual stats, damage, energy, HP, reserve, healing, AI, equipment, loot or saves implemented.',
    'excluded_trials':['palette-check-possession.gif'],'campaign_integration':False,'artifact_hashes':proof},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
for c in review['candidates']:c['visual_status']=c['body_status']='visually_approved';c['acceptance']=receipt.name
review['status']='visually_approved';review['acceptance']=receipt.name
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation['visual_approval']='TR63-BODY01, TR64-BODY01, TR63-A01 and TR64-A01 visually approved.';validation['acceptance']=receipt.name
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
assert all(hashlib.sha256((OUT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
print(json.dumps({'approved_bodies':2,'approved_transitions':2,'hashed_artifacts':len(proof),'campaign_integration':False}))
