"""Integrate only the exact M03-C05 icon explicitly approved in IC37."""
from pathlib import Path
import copy
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-icon-review-37'
OUT = Path(__file__).resolve().parent

def read(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))

def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

ledger_path = BASE / 'skill-icon-acceptance.json'
old = read(ledger_path)
ledger = copy.deepcopy(old)
catalogue_before = read(ROOT / 'assets/skills/agent/catalogue.json')
manifest_before = read(ROOT / 'assets/skills/agent/manifest.json')
review = read(REVIEW / 'review.json')
assert len(review['candidates']) == 1
c = review['candidates'][0]
assert (c['id'], c['number'], c['variant']) == ('M03', 8, 'M03-C05')
index = next(i for i, e in enumerate(ledger['compact_accepted']) if e['id'] == 'M03')
previous = ledger['compact_accepted'][index]
assert previous['variant'] == 'M03-C02'
receipt = REVIEW / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Approval already recorded'
protected = {}
for category in ('accepted', 'compact_accepted', 'superseded_compact', 'withdrawn_compact'):
    for entry in old.get(category, []):
        for info in [*entry['files'].values(), *([entry['master']] if 'master' in entry else [])]:
            assert digest(ROOT / info['file']) == info['sha256']
            protected[info['file']] = info['sha256']
for p in (ROOT / 'assets/skills/agent').glob('*.png'):
    if p.name != 'M03-compact.png':
        protected[p.relative_to(ROOT).as_posix()] = digest(p)
for directory in ('skill-icon-review-34', 'skill-icon-review-35', 'skill-icon-review-36'):
    for p in (BASE / directory).rglob('*'):
        if p.is_file():
            protected[p.relative_to(ROOT).as_posix()] = digest(p)
canonical = BASE / 'skill-window-review-05/skills.json'
protected[canonical.relative_to(ROOT).as_posix()] = digest(canonical)
files = {}
for size, info in c['files'].items():
    p = REVIEW / info['file']
    assert digest(p) == info['sha256']
    files[size] = {'file':p.relative_to(ROOT).as_posix(), 'sha256':info['sha256']}
master = REVIEW / c['master']['file']
assert digest(master) == c['master']['sha256']
protected[master.relative_to(ROOT).as_posix()] = digest(master)
for info in files.values():
    protected[info['file']] = info['sha256']
write(OUT / 'ledger-before.json', old)
write(OUT / 'catalogue-before.json', catalogue_before)
write(OUT / 'manifest-before.json', manifest_before)
write(OUT / 'protected-before.json', protected)
write(receipt, {
    'review_id':'SKILLS-IC37-ACCEPT01', 'date':'2026-10-11',
    'user_approval':'oui ca me va :)',
    'context':'Reply to IC37, displaying only the symbol of four claw scratch marks for Puissance métamorphe.',
    'accepted_compact':['M03-C05'], 'accepted_numbers':[8],
    'scope':'Exact compact icon only. Copy, mechanics and detailed illustration remain unchanged. IC34 candidates are not approved by this reply.'
})
entry = {k:c[k] for k in ('id','number','variant','title')}
entry.update(kind='compact', visual_status='accepted', acceptance_review='SKILLS-IC37',
             acceptance_reference=receipt.relative_to(ROOT).as_posix(), files=files,
             master={'file':master.relative_to(ROOT).as_posix(), 'sha256':digest(master)})
superseded = copy.deepcopy(previous)
superseded['superseded_by'] = 'M03-C05'
superseded['supersession_reference'] = receipt.relative_to(ROOT).as_posix()
ledger['superseded_compact'].append(superseded)
ledger['compact_accepted'][index] = entry
ledger['compact_accepted_count'] = len(ledger['compact_accepted'])
ledger['accepted_drawing_count'] = len(ledger['accepted']) + len(ledger['compact_accepted'])
write(ledger_path, ledger)
review['status'] = 'accepted'
review['acceptance_reference'] = receipt.relative_to(ROOT).as_posix()
review['active_variant'] = 'M03-C05'
c['visual_status'] = 'accepted'
write(REVIEW / 'review.json', review)
runpy.run_path(str(ROOT / 'tools/art/package_agent_skill_window.py'), run_name='__main__')
for file, sha in protected.items():
    assert digest(ROOT / file) == sha, file
assert read(ROOT / 'assets/skills/agent/catalogue.json') == catalogue_before
assert digest(ROOT / 'assets/skills/agent/M03-compact.png') == files['64']['sha256']
for category in ('accepted', 'withdrawn_compact'):
    assert ledger[category] == old[category]
assert ledger['superseded_compact'][:-1] == old['superseded_compact']
assert [e for e in ledger['compact_accepted'] if e['id'] != 'M03'] == [e for e in old['compact_accepted'] if e['id'] != 'M03']
manifest = read(ROOT / 'assets/skills/agent/manifest.json')
unchanged = lambda m: [e for e in m['drawings'] if (e['skill'],e['category']) != ('M03','compact')]
assert unchanged(manifest) == unchanged(manifest_before)
assert {k:v for k,v in manifest.items() if k != 'drawings'} == {k:v for k,v in manifest_before.items() if k != 'drawings'}
write(OUT / 'integration-validation.json', {
    'review_id':'SKILLS-NATIVE13', 'approved_variants':['M03-C05'], 'replaced_variant':'M03-C02',
    'compact_count':len(ledger['compact_accepted']), 'illustration_count':len(ledger['accepted']),
    'skill_count':len(catalogue_before['skills']), 'preserved_hashes':len(protected),
    'packaged_icon_matches_approved_source':True, 'catalogue_unchanged':True,
    'other_ledger_entries_unchanged':True, 'old_variant_archived':True,
    'IC34_candidates_and_statuses_unchanged':True, 'detailed_illustration_unchanged':True
})
print(json.dumps({'compact':len(ledger['compact_accepted']), 'illustrations':len(ledger['accepted']),
                  'exact_icon':'M03-C05', 'preserved_hashes':len(protected)}))
