"""Apply the user's IC32 exception without changing the other approved drawings."""
from pathlib import Path
import copy
import hashlib
import json
import runpy
import shutil

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-icon-review-32'
OUT = Path(__file__).resolve().parent
RECEIPT = REVIEW / 'visual-acceptance-02.json'

def read(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))

def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

assert not RECEIPT.exists(), 'Correction already recorded'
ledger_path = BASE / 'skill-icon-acceptance.json'
old = read(ledger_path)
ledger = copy.deepcopy(old)
removed = [c for c in ledger['compact_accepted'] if c['id'] == 'A02']
assert len(removed) == 1 and removed[0]['variant'] == 'A02-C01'
kept = [c for c in ledger['compact_accepted'] if c['id'] != 'A02']
assert len(kept) == 61
catalogue_path = ROOT / 'assets/skills/agent/catalogue.json'
before = read(catalogue_path)
protected = {p.relative_to(ROOT).as_posix(): digest(p)
             for p in (ROOT / 'assets/skills/agent').glob('*.png')
             if p.name != 'A02-compact.png'}
for c in [*old['accepted'], *old['compact_accepted'], *old['superseded_compact']]:
    for f in [*c['files'].values(), *([c['master']] if 'master' in c else [])]:
        assert digest(ROOT / f['file']) == f['sha256']
        protected[f['file']] = f['sha256']
canonical = BASE / 'skill-window-review-05/skills.json'
protected[canonical.relative_to(ROOT).as_posix()] = digest(canonical)
write(OUT / 'protected-before-correction.json', protected)
shutil.copyfile(OUT / 'integration-validation.json', OUT / 'integration-validation-before-correction.json')
write(OUT / 'ledger-before-correction.json', old)
write(OUT / 'catalogue-before-correction.json', before)

write(RECEIPT, {
    'review_id': 'SKILLS-IC32-ACCEPT02', 'date': '2026-10-11',
    'user_correction': "en faite je valide sauf pour l'image de déflagration occulte, je n'aime pas cette image.",
    'user_art_direction': 'dessine juste une explosion occulte :)',
    'supersedes': 'visual-acceptance-01.json',
    'scope': 'Withdraw approval only for A02-C01; the five other compact drawings retain their exact approval. No gameplay or copy changes.',
    'withdrawn_compact': ['A02-C01'], 'withdrawn_numbers': [60],
    'accepted_compact': ['A01-C01', 'HY05-C01', 'OC04-C01', 'OC05-C01', 'OC07-C01'],
    'accepted_numbers': [59, 61, 62, 63, 64],
})
entry = removed[0]
entry['visual_status'] = 'approval_withdrawn'
entry['correction_reference'] = RECEIPT.relative_to(ROOT).as_posix()
ledger.setdefault('withdrawn_compact', []).append(entry)
ledger['compact_accepted'] = kept
ledger['compact_accepted_count'] = len(kept)
ledger['accepted_drawing_count'] = len(kept) + len(ledger['accepted'])
write(ledger_path, ledger)
review = read(REVIEW / 'review.json')
review['status'] = 'accepted_with_one_revision_requested'
review['correction_reference'] = RECEIPT.relative_to(ROOT).as_posix()
for c in review['candidates']:
    if c['id'] == 'A02':
        c['visual_status'] = 'revision_requested'
        c['correction_reference'] = RECEIPT.relative_to(ROOT).as_posix()
write(REVIEW / 'review.json', review)

active = ROOT / 'assets/skills/agent/A02-compact.png'
assert active.resolve().parent == (ROOT / 'assets/skills/agent').resolve()
assert digest(active) == removed[0]['files']['64']['sha256']
archive = OUT / 'withdrawn-art'
archive.mkdir(exist_ok=True)
shutil.copyfile(active, archive / 'A02-C01-compact.png')
assert digest(archive / 'A02-C01-compact.png') == digest(active)
active.unlink()
runpy.run_path(str(ROOT / 'tools/art/package_agent_skill_window.py'), run_name='__main__')
after = read(catalogue_path)
assert after['skills'] == before['skills'] and after['labels'] == before['labels']
assert 'A02' not in after['art']
assert {k:v for k,v in before['art'].items() if k != 'A02'} == after['art']
for file, sha in protected.items():
    assert digest(ROOT / file) == sha, file
assert ledger['accepted'] == old['accepted']
assert ledger['superseded_compact'] == old['superseded_compact']
assert ledger['compact_accepted'] == [c for c in old['compact_accepted'] if c['id'] != 'A02']
current = read(OUT / 'integration-validation.json')
current.update(approved_variants=['A01-C01', 'HY05-C01', 'OC04-C01', 'OC05-C01', 'OC07-C01'],
               compact_count=61, new_exports_verified=25, new_masters_verified=5,
               withdrawn_variant='A02-C01', correction_reference=RECEIPT.relative_to(ROOT).as_posix(),
               correction_preserved_hashes=len(protected),
               rejected_icon_not_in_active_bank=True,
               original_captures_status='Historical captures taken before the approval correction')
write(OUT / 'integration-validation.json', current)
print(json.dumps({'compact': 61, 'illustrations': 16, 'withdrawn': 'A02-C01',
                  'preserved_hashes': len(protected), 'other_five_drawings_unchanged': True}))
