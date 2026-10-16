"""Record review39's accepted bodies and transitions without campaign integration."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-39'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Preserve existing approval evidence.'
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
assert {c['variant'] for c in review['candidates']} == {'M01-A01', 'TR61-A01'}
assert all(c['visual_status'] == 'awaiting_visual_review' and c['body_status'] == 'awaiting_visual_review' for c in review['candidates'])
checks = json.loads((OUT / 'verification.json').read_text(encoding='utf-8'))
assert len(checks['animations']) == 9 and len(checks['controls']) == 14
assert all(c['walls_clear'] and c['timers_require_explicit_turns'] and c['renewal_or_expiry_checked'] for c in checks['controls'])
files = sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*') if p.is_file()
               and p.name not in ('review.json', 'validation.json', 'review-protection.json')
               and p.suffix in ('.png', '.gif', '.rs', '.py', '.txt', '.html', '.js', '.json', '.ps1'))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date': '2026-10-13', 'user_reply': 'ok je valide on peut continuer',
    'presented_preview': 'skill-animation-review-39/transformations-haut-64.gif',
    'presented_gallery': 'skill-animation-review-39/index.html',
    'accepted': [{'number': c['number'], 'id': c['id'], 'transition': c['variant'], 'body': c['body_variant'], 'native_tile_size': 64} for c in review['candidates']],
    'scope': 'Both directly rendered64px bodies and their shown entry, return and renewal transitions. Four map orientations, wall clipping and explicit-turn duration retained.',
    'retained_rules': 'Previously agreed numerical rules keep their approval status; isolated counters do not implement actual combat, HP/energy changes or form attacks.',
    'campaign_integration': False, 'artifact_hashes': proof,
}, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
for candidate in review['candidates']:
    candidate['visual_status'] = candidate['body_status'] = 'visually_approved'
    candidate['acceptance'] = receipt.name
review['status'] = 'visually_approved'; review['acceptance'] = receipt.name
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
validation = json.loads((OUT / 'validation.json').read_text(encoding='utf-8'))
validation['visual_approval'] = 'M01-BODY01, TR61-BODY01, M01-A01 and TR61-A01 visually approved.'
validation['acceptance'] = receipt.name
(OUT / 'validation.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
assert all(hashlib.sha256((OUT / name).read_bytes()).hexdigest() == digest for name, digest in proof.items())
print(json.dumps({'approved_bodies': ['M01-BODY01', 'TR61-BODY01'], 'approved_transitions': ['M01-A01', 'TR61-A01'], 'hashed_artifacts': len(proof), 'campaign_integration': False}))
