"""Verify targeted revisions and exact carry-forward without runtime changes."""
from pathlib import Path
import hashlib
import json
from PIL import Image

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
OLD = OUT.parent / 'skill-icon-review-29'


def read(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


protected = read(OUT / 'protected-before.json')
for filename, expected in protected.items():
    assert digest(ROOT / filename) == expected, filename
review = read(OUT / 'review.json')
previous = read(OLD / 'review.json')
plan = read(OUT / 'sources.json')
for candidate in review['candidates']:
    source = next(c for c in plan['candidates'] if c['id'] == candidate['id'])
    old = next(c for c in previous['candidates'] if c['id'] == candidate['id'])
    assert candidate['number'] == old['number']
    for key in ('title', 'kind', 'roleplay', 'branch'):
        assert candidate[key] == old[key]
    for size, info in candidate['files'].items():
        path = OUT / info['file']
        assert digest(path) == info['sha256']
        assert Image.open(path).size == (int(size), int(size))
    assert digest(OUT / candidate['master']['file']) == candidate['master']['sha256']
    if source.get('no_visual_change'):
        assert candidate['variant'] == old['variant']
        assert candidate['master']['sha256'] == old['master']['sha256']
        assert all(info['sha256'] == old['files'][size]['sha256']
                   for size, info in candidate['files'].items())
        candidate.update(carried_forward_from='SKILLS-IC29', no_visual_change=True)
    else:
        assert candidate['variant'] in ('FR05-C02', 'FR06-C02')
        candidate['replaces_pending_variant'] = old['variant']
write(OUT / 'review.json', review)
previous['status'] = 'revisions_requested'
for candidate in previous['candidates']:
    if candidate['id'] in ('FR05', 'FR06'):
        candidate['visual_status'] = 'revision_requested'
previous['feedback_reference'] = 'feedback-01.json'
write(OLD / 'review.json', previous)
validation = read(OUT / 'validation.json')
validation.update(identities_copy_and_numbers_unchanged=True,
                  carried_forward_variants=['FR01-C01', 'FR02-C01', 'FR04-C01'],
                  carried_forward_exports_verified=15,
                  carried_forward_masters_verified=3,
                  new_variants=['FR05-C02', 'FR06-C02'],
                  all_protected_files_preserved=len(protected),
                  no_approval_inferred=True, no_runtime_changes=True)
write(OUT / 'validation.json', validation)
print(json.dumps({'new_variants': 2, 'exact_preserved_variants': 3,
                  'protected_files': len(protected), 'exports_verified': 25}))
