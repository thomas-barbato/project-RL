"""Record the explicit approval of FE05-A07's 0.35-second flight."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-22'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Approval already recorded.'
files = ['render-source.rs', 'clip-source.rs', 'gallery-script.js', 'verification.json',
         'burst-atlas.png', 'projectile-atlas.png']
files += sorted(p.name for p in OUT.glob('*.gif'))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date': '2026-10-12', 'user_reply': 'ok je valide :)',
    'accepted': {'number': 13, 'skill_id': 'FE05', 'variant': 'FE05-A07',
                 'status': 'visually_approved', 'flight_seconds': .35},
    'retained_approval': '../skill-animation-review-20/visual-acceptance-01.json',
    'scope': 'Final fireball pre-impact movement speed shown in review 22. The approved projectile, explosion, walls and Burn are preserved. No new skill numbers, damage rules or campaign integration.',
    'temporal_policy': 'Instantaneous impact ends automatically; Burn and cooldowns remain driven by game turns.',
    'art_direction': 'ANIM-RAPIDE01: fast activation and short readable impacts for active skills.',
    'artifact_hashes': proof,
}, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
review['status'] = 'visually_approved'
review['acceptance'] = receipt.name
for candidate in review['candidates']:
    candidate['status'] = 'visually_approved'
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
print('FE05-A07 approved: 0.35-second flight, previous lot approvals retained.')
