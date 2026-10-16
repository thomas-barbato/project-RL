"""Record the exact IC33 compact approval; preserve the rejected IC32 audit."""
from pathlib import Path
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-icon-review-33'
OUT = Path(__file__).resolve().parent

def read(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))

def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

ledger_path = BASE / 'skill-icon-acceptance.json'
old = read(ledger_path)
ledger = read(ledger_path)
old_catalogue = read(ROOT / 'assets/skills/agent/catalogue.json')
review = read(REVIEW / 'review.json')
assert len(review['candidates']) == 1
c = review['candidates'][0]
assert (c['id'], c['number'], c['variant']) == ('A02', 60, 'A02-C02')
assert not any(e['id'] == 'A02' for e in ledger['compact_accepted'])
assert any(e['variant'] == 'A02-C01' for e in ledger['withdrawn_compact'])
receipt = REVIEW / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Approval already recorded'
write(OUT / 'ledger-before.json', old)
write(OUT / 'catalogue-before.json', old_catalogue)
protected = {}
for category in ('accepted', 'compact_accepted', 'superseded_compact', 'withdrawn_compact'):
    for entry in old.get(category, []):
        for info in [*entry['files'].values(), *([entry['master']] if 'master' in entry else [])]:
            assert digest(ROOT / info['file']) == info['sha256']
            protected[info['file']] = info['sha256']
for p in (ROOT / 'assets/skills/agent').glob('*.png'):
    protected[p.relative_to(ROOT).as_posix()] = digest(p)
canonical = BASE / 'skill-window-review-05/skills.json'
protected[canonical.relative_to(ROOT).as_posix()] = digest(canonical)
write(OUT / 'protected-before.json', protected)
files = {}
for size, info in c['files'].items():
    path = REVIEW / info['file']
    assert digest(path) == info['sha256']
    files[size] = {'file': path.relative_to(ROOT).as_posix(), 'sha256': info['sha256']}
master = REVIEW / c['master']['file']
assert digest(master) == c['master']['sha256']
write(receipt, {'review_id':'SKILLS-IC33-ACCEPT01', 'date':'2026-10-11',
                'user_approval':'parfait on continue :)',
                'context':'Reply to the IC33 page and the final review of the new explosion-only candidate 60.',
                'accepted_compact':['A02-C02'], 'accepted_numbers':[60],
                'scope':'This exact compact drawing only; copy, mechanics, illustrations and the rejected A02-C01 remain unchanged.'})
entry = {k:c[k] for k in ('id','number','variant','title')}
entry.update(kind='compact', visual_status='accepted', acceptance_review='SKILLS-IC33',
             acceptance_reference=receipt.relative_to(ROOT).as_posix(), files=files,
             master={'file':master.relative_to(ROOT).as_posix(), 'sha256':digest(master)})
ledger['compact_accepted'].append(entry)
ledger['compact_accepted_count'] = len(ledger['compact_accepted'])
ledger['accepted_drawing_count'] = len(ledger['compact_accepted']) + len(ledger['accepted'])
write(ledger_path, ledger)
review['status'] = 'accepted'
review['acceptance_reference'] = receipt.relative_to(ROOT).as_posix()
review['no_new_approval_inferred'] = False
c['visual_status'] = 'accepted'
write(REVIEW / 'review.json', review)
runpy.run_path(str(ROOT / 'tools/art/package_agent_skill_window.py'), run_name='__main__')
for path, sha in protected.items():
    assert digest(ROOT / path) == sha, path
current = read(ROOT / 'assets/skills/agent/catalogue.json')
assert current['skills'] == old_catalogue['skills']
assert current['labels'] == old_catalogue['labels']
assert {k:v for k,v in current['art'].items() if k != 'A02'} == old_catalogue['art']
assert digest(ROOT / 'assets/skills/agent/A02-compact.png') == files['64']['sha256']
for category in ('accepted', 'superseded_compact', 'withdrawn_compact'):
    assert ledger[category] == old[category]
assert ledger['compact_accepted'][:-1] == old['compact_accepted']
write(OUT / 'integration-validation.json', {
    'review_id':'SKILLS-NATIVE12', 'approved_variants':['A02-C02'],
    'compact_count':62, 'illustration_count':16, 'skill_count':94,
    'preserved_hashes':len(protected), 'new_exports_verified':5, 'new_masters_verified':1,
    'packaged_icons_match_approved_sources':True, 'prior_ledger_entries_unchanged':True,
    'catalogue_skills_and_labels_unchanged':True, 'withdrawn_A02_C01_preserved':True})
print(json.dumps({'compact':62,'illustrations':16,'exact_new_icons':1,'protected_hashes':len(protected)}))
