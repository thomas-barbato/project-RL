"""Record ANIM-S06 approval, preserving the separately requested flight-speed revision."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-20'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Approval already recorded.'
files = ['explosion-source.png', 'burst-atlas.png', 'projectile-atlas.png',
         'render-source.rs', 'clip-source.rs', 'gallery-script.js', 'verification.json']
files += sorted(p.name for p in OUT.glob('*.gif'))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date': '2026-10-12',
    'user_replies': ['je valide :)', "je valide mais j'aimerais qu'on accelere legerement le mouvement de la boule de feu (avant qu'elle n'explose)"],
    'accepted': [
        {'number': 13, 'skill_id': 'FE05', 'variant': 'FE05-A05', 'status': 'visually_approved',
         'scope': 'Shown projectile, six-step explosion, wall clipping and attached Burn. Flight movement speed is requested for a separate revision.'},
        {'number': 14, 'skill_id': 'FE06', 'variant': 'FE06-A03', 'status': 'visually_approved',
         'scope': 'Shown targeted shock, finite animation and prepared Burn cases.'}],
    'temporal_policy': 'ANIM-TEMPS02: instantaneous impacts finish in real time; attached statuses and cooldowns retain their game-turn counters.',
    'pending_revision': 'Slightly faster fireball movement before impact. Keep the accepted explosion and other effects.',
    'scope': 'Visual approval of this review; no new skill figures, damage rules or campaign integration.',
    'artifact_hashes': proof,
}, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
review['status'] = 'visually_approved_with_flight_speed_revision_requested'
review['acceptance'] = receipt.name
for candidate in review['candidates']:
    candidate['status'] = 'visually_approved'
    if candidate['skill_id'] == 'FE05':
        candidate['pending_revision'] = 'Flight movement speed only.'
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
print('FE05-A05 and FE06-A03 approved; fireball flight-speed revision recorded separately.')
