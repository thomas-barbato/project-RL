"""Present only the five unapproved heavy-weapon drawings, without edits."""
from pathlib import Path
import hashlib
import json
import shutil

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent
SOURCE = BASE / 'skill-icon-review-44'

def read(path):
    return json.loads(path.read_text('utf-8-sig'))

def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

assert not (OUT / 'visual-acceptance-01.json').exists(), 'Accepted reviews are frozen.'
ledger = read(BASE / 'skill-icon-acceptance.json')
approved = {c['id'] for c in ledger['compact_accepted']}
accepted_files = [f for c in ledger['accepted'] + ledger['compact_accepted'] for f in c['files'].values()]
for file in accepted_files:
    assert digest(ROOT / file['file']) == file['sha256'], file['file']
protected_paths = [*(ROOT / 'assets/skills/agent').rglob('*'), BASE / 'skill-icon-acceptance.json', BASE / 'skill-window-review-05/skills.json']
protected = {p.relative_to(ROOT).as_posix(): digest(p) for p in protected_paths if p.is_file()}
source_review = read(SOURCE / 'review.json')
candidates = [c.copy() for c in source_review['candidates'] if c['id'] != 'HL04']
assert [c['number'] for c in candidates] == [78, 79, 80, 82, 83]
(OUT / 'prompts').mkdir(exist_ok=True)
for candidate in candidates:
    assert candidate['id'] not in approved, candidate['id']
    assert candidate['visual_status'] == 'awaiting_visual_review'
    for file in [candidate['master'], *candidate['files'].values()]:
        src = SOURCE / file['file']
        assert digest(src) == file['sha256'], src
        shutil.copyfile(src, OUT / file['file'])
        assert digest(OUT / file['file']) == file['sha256'], file['file']
    prompt = SOURCE / candidate['prompt']
    shutil.copyfile(prompt, OUT / candidate['prompt'])
    candidate['source_review'] = 'SKILLS-IC44'
    candidate['source_review_reference'] = '../skill-icon-review-44/review.json'
    candidate['reused_without_drawing_changes'] = True
review = {'review_id': 'SKILLS-IC49', 'title': 'Armes lourdes · cinq icônes restantes',
          'date': '2026-10-11', 'status': 'awaiting_visual_review',
          'scope': 'The five remaining compact icons from IC44. HL04-C05 is already accepted and excluded.',
          'generation_mode': 'reuse_existing_built_in_outputs', 'no_new_drawings': True,
          'candidates': candidates, 'protected_hashes': [{'file': file, 'sha256': sha} for file, sha in protected.items()]}
write(OUT / 'review.json', review)
write(OUT / 'validation.json', {'review_id': 'SKILLS-IC49', 'status': 'awaiting_visual_review',
      'candidate_count': 5, 'candidate_numbers': [78, 79, 80, 82, 83],
      'reused_exports_verified': 25, 'reused_masters_verified': 5,
      'accepted_export_hashes_verified': len(accepted_files), 'runtime_changes': False,
      'preserved_runtime_and_catalogue_files': len(protected)})
for file, sha in protected.items():
    assert digest(ROOT / file) == sha, file
print(json.dumps({'review': 'SKILLS-IC49', 'candidates': 5, 'exports_reused': 25,
                  'accepted_exports_preserved': len(accepted_files), 'protected_files': len(protected)}))
