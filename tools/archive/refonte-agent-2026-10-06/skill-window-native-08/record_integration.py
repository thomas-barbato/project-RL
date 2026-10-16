"""Record the exact IC27 approval and package the four reviewed compact icons."""
from pathlib import Path
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-icon-review-27'
OUT = Path(__file__).resolve().parent


def read(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


ledger_path = BASE / 'skill-icon-acceptance.json'
ledger = read(ledger_path)
old_ledger = read(ledger_path)
catalogue_path = ROOT / 'assets/skills/agent/catalogue.json'
old_catalogue = read(catalogue_path)
review = read(REVIEW / 'review.json')
expected = {'ES03': (46, 'ES03-C01'), 'ES05': (47, 'ES05-C03'),
            'ES06': (48, 'ES06-C02'), 'T02': (49, 'T02-C04')}
assert len(review['candidates']) == 4
assert {c['id']: (c['number'], c['variant']) for c in review['candidates']} == expected
assert not (expected.keys() & {c['id'] for c in ledger['compact_accepted']})
receipt_path = REVIEW / 'visual-acceptance-01.json'
assert not receipt_path.exists(), 'Approval receipt already exists'

protected = {}
for category in ('accepted', 'compact_accepted', 'superseded_compact'):
    for entry in ledger[category]:
        for info in [*entry['files'].values(), *([entry['master']] if 'master' in entry else [])]:
            path = ROOT / info['file']
            assert digest(path) == info['sha256'], path
            protected[info['file']] = digest(path)
for path in (ROOT / 'assets/skills/agent').glob('*.png'):
    protected[path.relative_to(ROOT).as_posix()] = digest(path)
canonical = BASE / 'skill-window-review-05/skills.json'
protected[canonical.relative_to(ROOT).as_posix()] = digest(canonical)
write(OUT / 'protected-before.json', protected)

approved = []
for candidate in review['candidates']:
    files = {}
    for size, info in candidate['files'].items():
        path = REVIEW / info['file']
        assert digest(path) == info['sha256'], path
        files[size] = {'file': path.relative_to(ROOT).as_posix(), 'sha256': info['sha256']}
    master = REVIEW / candidate['master']['file']
    assert digest(master) == candidate['master']['sha256']
    entry = {key: candidate[key] for key in ('id', 'number', 'variant', 'title')}
    entry.update(kind='compact', visual_status='accepted', acceptance_review='SKILLS-IC27',
                 acceptance_reference=receipt_path.relative_to(ROOT).as_posix(), files=files,
                 master={'file': master.relative_to(ROOT).as_posix(), 'sha256': digest(master)})
    approved.append(entry)

write(receipt_path, {'review_id': 'SKILLS-IC27-ACCEPT01', 'date': '2026-10-11',
                    'user_approval': 'ok on valide',
                    'context': 'Approval of the four icons shown together in skill-icon-review-27.',
                    'accepted_compact': [entry['variant'] for entry in approved],
                    'accepted_numbers': [entry['number'] for entry in approved],
                    'scope': 'These four exact compact drawings only; no mechanics, copy, detailed illustration or older pending variant approval implied.'})
ledger['compact_accepted'].extend(approved)
ledger['compact_accepted_count'] = len(ledger['compact_accepted'])
ledger['accepted_drawing_count'] = len(ledger['accepted']) + len(ledger['compact_accepted'])
write(ledger_path, ledger)
review['status'] = 'accepted'
review['acceptance_reference'] = receipt_path.relative_to(ROOT).as_posix()
review['no_new_approval_inferred'] = False
for candidate in review['candidates']:
    candidate['visual_status'] = 'accepted'
write(REVIEW / 'review.json', review)

runpy.run_path(str(ROOT / 'tools/art/package_agent_skill_window.py'), run_name='__main__')
for filename, before in protected.items():
    assert digest(ROOT / filename) == before, filename
current = read(catalogue_path)
assert current['skills'] == old_catalogue['skills']
assert current['labels'] == old_catalogue['labels']
assert all(current['art'][key] == value for key, value in old_catalogue['art'].items())
assert ledger['accepted'] == old_ledger['accepted']
assert ledger['superseded_compact'] == old_ledger['superseded_compact']
assert ledger['compact_accepted'][:-4] == old_ledger['compact_accepted']
for entry in approved:
    assert digest(ROOT / 'assets/skills/agent' / (entry['id'] + '-compact.png')) == entry['files']['64']['sha256']
write(OUT / 'integration-validation.json', {'review_id': 'SKILLS-NATIVE08',
      'approved_variants': [entry['variant'] for entry in approved],
      'compact_count': ledger['compact_accepted_count'], 'illustration_count': len(ledger['accepted']),
      'preserved_hashes': len(protected), 'new_exports_verified': 20, 'new_masters_verified': 4,
      'packaged_icons_match_approved_sources': True, 'prior_ledger_entries_unchanged': True,
      'catalogue_skills_and_labels_unchanged': True, 'skill_count': len(current['skills'])})
print(json.dumps({'compact': ledger['compact_accepted_count'], 'illustration': len(ledger['accepted']),
                  'protected_hashes': len(protected), 'exact_new_icons': 4}))
