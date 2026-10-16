"""Integrate only M02-C03, explicitly approved with the attached image."""
from pathlib import Path
import hashlib
import json
import runpy
from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-icon-review-41'
OUT = Path(__file__).resolve().parent
ATTACHMENT = Path('C:/Users/User/AppData/Local/Temp/codex-clipboard-8efefcc6-848f-4c3a-b3df-3ad81b12f039.png')


def read(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


ledger_path = BASE / 'skill-icon-acceptance.json'
ledger = read(ledger_path)
before = read(ledger_path)
catalogue_path = ROOT / 'assets/skills/agent/catalogue.json'
old_catalogue = read(catalogue_path)
review = read(REVIEW / 'review.json')
assert len(review['candidates']) == 1
c = review['candidates'][0]
assert (c['id'], c['number'], c['variant']) == ('M02', 69, 'M02-C03')
assert len(ledger['compact_accepted']) == 66
assert not any(e['id'] == 'M02' for e in ledger['compact_accepted'])
receipt = REVIEW / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Approval already recorded'
master = REVIEW / c['master']['file']
a, b = Image.open(ATTACHMENT).convert('RGBA'), Image.open(master).convert('RGBA')
assert a.size == b.size
assert ImageChops.difference(a.convert('RGB'), b.convert('RGB')).getbbox() is None
assert ImageChops.difference(a.getchannel('A'), b.getchannel('A')).getbbox() is None
write(OUT / 'catalogue-before.json', old_catalogue)
write(OUT / 'ledger-before.json', before)
write(OUT / 'manifest-before.json', read(ROOT / 'assets/skills/agent/manifest.json'))

protected = {}
for category in ('accepted', 'compact_accepted', 'superseded_compact', 'withdrawn_compact'):
    for entry in ledger.get(category, []):
        for info in [*entry['files'].values(), *([entry['master']] if 'master' in entry else [])]:
            p = ROOT / info['file']
            assert digest(p) == info['sha256'], p
            protected[info['file']] = digest(p)
for p in (ROOT / 'assets/skills/agent').glob('*.png'):
    protected[p.relative_to(ROOT).as_posix()] = digest(p)
for p in (BASE / 'skill-icon-review-39').rglob('*'):
    if p.is_file():
        protected[p.relative_to(ROOT).as_posix()] = digest(p)
canonical = BASE / 'skill-window-review-05/skills.json'
protected[canonical.relative_to(ROOT).as_posix()] = digest(canonical)
write(OUT / 'protected-before.json', protected)

files = {}
for size, info in c['files'].items():
    p = REVIEW / info['file']
    assert digest(p) == info['sha256'], p
    files[size] = {'file': p.relative_to(ROOT).as_posix(), 'sha256': info['sha256']}
assert digest(master) == c['master']['sha256']
entry = {key: c[key] for key in ('id', 'number', 'variant', 'title')}
entry.update(kind='compact', visual_status='accepted', acceptance_review='SKILLS-IC41',
             acceptance_reference=receipt.relative_to(ROOT).as_posix(), files=files,
             master={'file': master.relative_to(ROOT).as_posix(), 'sha256': digest(master)})
write(receipt, {'review_id': 'SKILLS-IC41-ACCEPT01', 'date': '2026-10-11',
               'user_approval': 'ok je valide ceci',
               'context': 'Approval identifies the attached green icon, whose RGB and alpha pixels exactly match M02-C03. Ambient IC13 browser URL is not the approval target.',
               'accepted_compact': ['M02-C03'], 'accepted_numbers': [69],
               'attached_image': {'file': str(ATTACHMENT), 'sha256': digest(ATTACHMENT),
                                  'pixel_match_to_master': True, 'dimensions': list(a.size)},
               'scope': 'Only the exact M02-C03 compact drawing. HY04-C01 and HY06-C01 remain unapproved; mechanics, copy, animation and damage type unchanged.'})
ledger['compact_accepted'].append(entry)
ledger['compact_accepted_count'] = len(ledger['compact_accepted'])
ledger['accepted_drawing_count'] = len(ledger['accepted']) + len(ledger['compact_accepted'])
write(ledger_path, ledger)
review['status'] = 'accepted'
review['acceptance_reference'] = receipt.relative_to(ROOT).as_posix()
review['no_new_approval_inferred'] = False
review['candidates'][0]['visual_status'] = 'accepted'
write(REVIEW / 'review.json', review)

runpy.run_path(str(ROOT / 'tools/art/package_agent_skill_window.py'), run_name='__main__')
for filename, old_hash in protected.items():
    assert digest(ROOT / filename) == old_hash, filename
current = read(catalogue_path)
assert current['skills'] == old_catalogue['skills']
assert current['labels'] == old_catalogue['labels']
assert all(current['art'][key].get(category) == filename
           for key, value in old_catalogue['art'].items()
           for category, filename in value.items())
assert ledger['compact_accepted'][:-1] == before['compact_accepted']
for category in ('accepted', 'superseded_compact', 'withdrawn_compact'):
    assert ledger.get(category) == before.get(category)
assert digest(ROOT / 'assets/skills/agent/M02-compact.png') == files['64']['sha256']
assert not any(e['id'] in ('HY04', 'HY06') for e in ledger['compact_accepted'])
write(OUT / 'integration-validation.json', {'review_id': 'SKILLS-NATIVE15',
      'approved_variants': ['M02-C03'], 'compact_count': 67, 'illustration_count': 16,
      'preserved_hashes': len(protected), 'new_exports_verified': 5, 'new_masters_verified': 1,
      'attached_image_pixel_match': True, 'packaged_icon_matches_approved_source': True,
      'prior_ledger_entries_unchanged': True, 'catalogue_skills_and_labels_unchanged': True,
      'HY04_and_HY06_still_pending': True, 'skill_count': len(current['skills'])})
print(json.dumps({'compact': 67, 'illustration': 16, 'protected_hashes': len(protected), 'exact_new_icons': 1}))
