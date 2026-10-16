from pathlib import Path
import hashlib
import json

OUT = Path(__file__).resolve().parent
PREVIOUS = OUT.parent / 'skill-icon-review-52'
protected = json.loads((OUT / 'prior-grenade-drawings.json').read_text('utf-8'))
assert len(protected) == 30
for file, sha in protected.items():
    assert hashlib.sha256((PREVIOUS / file).read_bytes()).hexdigest() == sha, file
review = json.loads((OUT / 'review.json').read_text('utf-8'))
assert [c['variant'] for c in review['candidates']] == ['SG01-C02', 'SG02-C02']
assert all(c['visual_status'] == 'awaiting_visual_review' for c in review['candidates'])
validation = json.loads((OUT / 'validation.json').read_text('utf-8'))
validation['prior_grenade_drawings_preserved'] = len(protected)
(OUT / 'validation.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n', 'utf-8')
print('All thirty prior grenade drawing files unchanged; both corrections await approval.')
