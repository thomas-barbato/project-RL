"""Freeze review43's six actions and shown common hit reactions."""
import fnmatch
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-43'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists()
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
validation = json.loads((OUT / 'validation.json').read_text(encoding='utf-8'))
assert len(review['candidates']) == 6
assert all(c['variant'].endswith('-A02') and c['status'] == 'awaiting_visual_review' for c in review['candidates'])
assert validation['focused_tests']['total'] == 95
assert validation['native_views'] == 64 and validation['animations'] == 22
assert validation['gallery']['native_canvas_count'] == 10
exclusions = review['excluded_trials']
files = sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*') if p.is_file()
    and p.name not in ('review.json', 'validation.json', 'review-protection.json', receipt.name)
    and not any(fnmatch.fnmatch(p.relative_to(OUT).as_posix(), pattern) for pattern in exclusions))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date': '2026-10-13', 'user_reply': 'parfait on peut continuer',
    'presented_preview': 'skill-animation-review-43/cuirasse-possede-actions-64.gif',
    'presented_gallery': 'skill-animation-review-43/index.html',
    'accepted': [{'number': c['number'], 'id': c['id'], 'variant': c['variant']} for c in review['candidates']],
    'common_reaction': review['common_reaction'],
    'scope': 'Six shown A02 own-action compositions, closed-plates body/badge, target hit movement and four isolated common damage/harmful-status cases. Four map directions and obstacle clipping.',
    'retained_rules': 'Numerical rules keep their previous status. Shared game damage/status feedback connected. New refonte combat abilities, actual damage, HP, energy, AI, loot and saves not implemented by this visual approval.',
    'excluded_trials': exclusions, 'new_refonte_combat_abilities_integrated': False,
    'artifact_hashes': proof}, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
for c in review['candidates']:
    c['status'] = 'visually_approved'
    c['acceptance'] = receipt.name
review['status'] = 'visually_approved'
review['acceptance'] = receipt.name
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
validation['new_compositions'] = 'Six A02 actions and shown common reactions visually approved.'
validation['acceptance'] = receipt.name
(OUT / 'validation.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
assert all(hashlib.sha256((OUT / name).read_bytes()).hexdigest() == digest for name, digest in proof.items())
print(json.dumps({'approved_actions': 6, 'common_reaction_cases': 4, 'hashed_artifacts': len(proof)}))
