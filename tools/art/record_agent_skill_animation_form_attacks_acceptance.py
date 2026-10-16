"""Record the six reviewed form attacks and shown statuses, without campaign integration."""
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-40'
receipt=OUT/'visual-acceptance-01.json'
assert not receipt.exists()
review=json.loads((OUT/'review.json').read_text(encoding='utf-8'))
assert len(review['candidates'])==6 and all(c['status']=='awaiting_visual_review' for c in review['candidates'])
checks=json.loads((OUT/'verification.json').read_text(encoding='utf-8'))
assert checks['native_views']==42 and len(checks['animations'])==16
assert all(c['walls_clear'] and c['no_clock_driven_hits'] and c['turn_based_statuses'] for c in checks['controls'])
files=sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*') if p.is_file()
             and 'tint-trial' not in p.relative_to(OUT).parts
             and p.name not in ('review.json','validation.json','review-protection.json','inspection-first.png')
             and p.suffix in ('.png','.gif','.rs','.py','.txt','.html','.js','.cjs','.json','.ps1'))
proof={name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date':'2026-10-13','user_reply':'tres bien :)',
    'presented_preview':'skill-animation-review-40/attaques-transformees-64.gif',
    'presented_gallery':'skill-animation-review-40/index.html',
    'accepted':[{'number':c['number'],'id':c['id'],'variant':c['variant'],'native_tile_size':64} for c in review['candidates']],
    'scope':'Six shown natural attacks: steel/ivory claws, heavy impacts, compact Bond without damage, radial shock without knockback, physical wound blood spray, falling Bleeding drops and orbiting Stun marker. Approved bodies remain unchanged.',
    'retained_rules':'Previously agreed numerical rules keep their status. Prepared footprints/status successes do not implement real combat, damage, energy, RNG, AI, equipment, loot or saves.',
    'excluded_trials':['tint-trial','inspection-first.png'],
    'campaign_integration':False,'artifact_hashes':proof,
},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
for c in review['candidates']:c['status']='visually_approved';c['acceptance']=receipt.name
review['status']='visually_approved';review['acceptance']=receipt.name
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation['visual_approval']='All six shown attacks, wound spray, Bleeding drops and Stun marker visually approved.'
validation['acceptance']=receipt.name
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
assert all(hashlib.sha256((OUT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
print(json.dumps({'approved_attacks':6,'hashed_artifacts':len(proof),'campaign_integration':False}))
