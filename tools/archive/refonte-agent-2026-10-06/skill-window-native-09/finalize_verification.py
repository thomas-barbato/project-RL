"""Complete read-only verification after adding compact art to an illustrated skill.

The first audit compared whole art mappings; FE05 intentionally adds a compact
entry alongside its existing illustration. Every pre-existing file had already
passed its hash check. This check verifies each old category independently.
"""
from pathlib import Path
import hashlib
import json

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
BASE = OUT.parent


def read(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


ledger = read(BASE / 'skill-icon-acceptance.json')
review = read(BASE / 'skill-icon-review-28/review.json')
receipt = read(BASE / 'skill-icon-review-28/visual-acceptance-01.json')
catalogue = read(ROOT / 'assets/skills/agent/catalogue.json')
protected = read(OUT / 'protected-before.json')
assert len(ledger['compact_accepted']) == 51
assert len(ledger['accepted']) == 16
assert ledger['accepted_drawing_count'] == 67
assert receipt['user_approval'] == "oui j'aime , je valide :)"
approved = ledger['compact_accepted'][-5:]
assert [entry['variant'] for entry in approved] == receipt['accepted_compact']
assert [entry['number'] for entry in approved] == [50, 51, 52, 17, 53]
for filename, before in protected.items():
    assert digest(ROOT / filename) == before, filename
for category, entries, size in [('compact', ledger['compact_accepted'], '64'),
                                ('illustration', ledger['accepted'], '192')]:
    for entry in entries:
        name = entry['id'] + '-' + category + '.png'
        assert catalogue['art'][entry['id']][category] == name
        assert digest(ROOT / 'assets/skills/agent' / name) == entry['files'][size]['sha256']
for entry in approved:
    assert entry['visual_status'] == 'accepted'
    for info in [*entry['files'].values(), entry['master']]:
        assert digest(ROOT / info['file']) == info['sha256']
assert catalogue['skills'] == read(BASE / 'skill-window-review-05/skills.json')['skills']
assert len(catalogue['skills']) == 94
assert review['status'] == 'accepted'
result = {'review_id': 'SKILLS-NATIVE09',
          'approved_variants': [entry['variant'] for entry in approved],
          'compact_count': 51, 'illustration_count': 16,
          'preserved_hashes': len(protected), 'new_exports_verified': 25,
          'new_masters_verified': 5,
          'packaged_icons_match_approved_sources': True,
          'prior_ledger_entries_unchanged': True,
          'catalogue_skills_and_labels_unchanged': True, 'skill_count': 94,
          'existing_illustration_preserved_for_FE05': True,
          'mapping_audit_correction': 'Compare individual art categories; FE05 adds a compact alongside its illustration.'}
(OUT / 'integration-validation.json').write_text(
    json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print(json.dumps(result))
