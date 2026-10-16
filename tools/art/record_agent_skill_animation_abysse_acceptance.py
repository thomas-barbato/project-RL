"""Freeze Abysse BODY02 and the entry/return/renewal compositions of review44."""
import hashlib
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-44'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists()
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
checks = json.loads((OUT / 'verification.json').read_text(encoding='utf-8'))
assert len(review['candidates']) == 1 and review['candidates'][0]['body_variant'] == 'TR65-BODY02'
assert len(checks['controls']) == 7 and len(checks['animations']) == 10
assert all(c['walls_clear'] and c['timers_require_explicit_turns'] for c in checks['controls'])
files = sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*') if p.is_file()
    and p.name not in ('review.json', 'validation.json', 'review-protection.json', receipt.name, 'cyan-trial-source.png'))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date': '2026-10-13', 'user_reply': 'oui ca me plait bien mieux comme ca :)',
    'presented_preview': 'skill-animation-review-44/transformations-haut-64.gif',
    'accepted': [{'number': 43, 'id': 'TR65', 'body': 'TR65-BODY02', 'transition': 'TR65-A01'}],
    'scope': 'Abysse without cyan, direct64px body and shown entry/return/renewal, four orientations and nearby wall clipping. Own attacks excluded.',
    'retained_rules': 'Previously accepted numerical rules keep their status; no stats, damage, HP, energy, AI, loot, inventory or save implementation.',
    'excluded_trials': ['cyan-trial-source.png'], 'campaign_integration': False, 'artifact_hashes': proof}, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
review['status'] = 'visually_approved'
review['acceptance'] = receipt.name
for c in review['candidates']:
    c['visual_status'] = c['body_status'] = 'visually_approved'
    c['acceptance'] = receipt.name
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
validation = json.loads((OUT / 'validation.json').read_text(encoding='utf-8'))
validation['visual_approval'] = 'TR65-BODY02 and TR65-A01 visually approved.'
validation['acceptance'] = receipt.name
(OUT / 'validation.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
assert all(hashlib.sha256((OUT / name).read_bytes()).hexdigest() == digest for name, digest in proof.items())
print(json.dumps({'body': 'TR65-BODY02', 'transition': 'TR65-A01', 'hashed_artifacts': len(proof)}))
