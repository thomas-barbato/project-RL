"""Record the explicit visual approval of review 28, without changing gameplay."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-28'
receipt = OUT/'visual-acceptance-01.json'
assert not receipt.exists(), 'Approval already recorded; preserve its evidence.'
review = json.loads((OUT/'review.json').read_text(encoding='utf-8'))
assert {c['variant'] for c in review['candidates']} == {'GEL-A05','FR06-A03'}
assert all(c['status'] == 'awaiting_visual_review' for c in review['candidates'])
checks = json.loads((OUT/'foreground-verification.json').read_text(encoding='utf-8'))
assert all(c['head_clear'] and c['torso_frost_visible'] and c['leg_frost_visible'] for c in checks['checks'])
validation = json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation['visual_approval'] = 'approved'
validation['acceptance'] = receipt.name
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')

files = ['gel-source.png','gel-atlas.png','imagegen-prompt.txt','drawing-pack.json',
    'render-source.rs','crystalline-source.rs','clip-source.rs','index.html',
    'gallery-script.js','verification.json','foreground-verification.json',
    'gallery-controller-verification.json','native/capture.json','validation.json',
    'Ouvrir_laboratoire.ps1']
files += sorted(p.name for p in OUT.glob('*.gif'))
files += sorted(p.relative_to(OUT).as_posix() for p in (OUT/'frames').rglob('*.png'))
files += sorted(p.relative_to(OUT).as_posix() for p in (OUT/'native').rglob('timeline.json'))
proof = {name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
dependencies = [
    'artifacts/refonte-agent-2026-10-06/skill-animation-review-27/gel-source.png',
    'artifacts/refonte-agent-2026-10-06/skill-animation-review-27/gel-atlas.png',
    'artifacts/refonte-agent-2026-10-06/skill-animation-review-26/burst-source.png',
    'artifacts/refonte-agent-2026-10-06/skill-animation-review-26/burst-atlas.png',
]
receipt.write_text(json.dumps({
    'date':'2026-10-12',
    'user_reply':'je valide oui',
    'reviewed_page':'skill-animation-review-28/index.html',
    'accepted':[{'number':c['number'],'id':c['id'],'variant':c['variant'],
        'status':'visually_approved','native_tile_size':64} for c in review['candidates']],
    'scope':'Review 28 visuals: translucent foreground frost on player/enemy torso and legs, clear head and sparse rear crystals; persistent shimmer, removal on expiry or Burn replacement, and the shown crystalline explosion assembly consuming Gel. No new numerical combat rules, damage, targeting geometry, AI or campaign integration.',
    'timing':'Visual motion remains independent of turns. Only explicit turns reduce status durations. Crystalline shatter remains one-shot and removes Gel on cast.',
    'other_candidates':'Other skill reviews retain their recorded status. Earlier Gel variants remain archived drafts; approval selects GEL-A05 and FR06-A03 as the current references.',
    'artifact_hashes':proof,
    'dependency_hashes':{name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest() for name in dependencies},
},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
for candidate in review['candidates']:
    candidate['status'] = 'visually_approved'
    candidate['acceptance'] = receipt.name
review['status'] = 'visually_approved'
review['acceptance'] = receipt.name
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'accepted':['GEL-A05','FR06-A03'],'hashed_artifacts':len(proof),'campaign_integration':False}))
