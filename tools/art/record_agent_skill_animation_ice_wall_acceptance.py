"""Record the explicit visual approval of Mur de glace FR03-A02 only."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-24'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Approval already recorded; preserve the receipt.'
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
candidate = next(c for c in review['candidates'] if c['skill_id'] == 'FR03')
assert candidate['variant'] == 'FR03-A02'
checks = json.loads((OUT / 'revision-verification.json').read_text(encoding='utf-8'))
assert checks['wall_timelines_identical'] and checks['gel_uses_exact_original_atlas']
files = ['wall-source.png', 'wall-atlas.png', 'imagegen-wall-prompt.txt',
         'drawing-pack.json', 'render-source.rs', 'clip-source.rs', 'gallery-script.js',
         'index.html', 'verification.json', 'revision-verification.json',
         'native/capture.json', 'mur-varie-comparaison-64.gif', 'mur-varie-cloison-64.gif']
files += sorted(p.name for p in OUT.glob('FR03-A02-*-64.gif'))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date': '2026-10-12', 'user_reply': 'okay je valide :)',
    'reviewed_artifact': 'mur-varie-comparaison-64.gif',
    'accepted': {'number': 16, 'skill_id': 'FR03', 'variant': 'FR03-A02',
        'status': 'visually_approved', 'tile_size': 64, 'block_models': 3,
        'states': ['intact', 'cracked', 'shattered'], 'shared_ground_base': True,
        'growth_seconds': .22, 'independent_collapse_seconds': .30},
    'scope': 'Visual approval of the revised ice wall shown in review 24: three joined irregular models, continuous rise, distinct cracks and independent collapse. No new player copy, numerical combat rules or campaign integration.',
    'other_candidates': 'FR02-A01 Nova and GEL-A03 retain their existing pending review state; this wall-only comparison does not implicitly approve them.',
    'temporal_policy': 'Wall duration, Gel and cooldowns advance only by explicit game turns. The rise and collapse animations do not advance those timers.',
    'artifact_hashes': proof,
}, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
candidate['status'] = 'visually_approved'
candidate['acceptance'] = receipt.name
review['status'] = 'wall_visually_approved_other_candidates_pending'
review['wall_acceptance'] = receipt.name
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
print('FR03-A02 visually approved. Other candidates retain their review state.')
