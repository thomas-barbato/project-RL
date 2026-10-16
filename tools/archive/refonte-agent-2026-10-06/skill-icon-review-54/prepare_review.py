"""Present only the three unapproved grenade drawings, without edits."""
from pathlib import Path
import hashlib
import json
import shutil
from PIL import Image

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent
SOURCE = BASE / 'skill-icon-review-52'
IDS = ['SG00', 'SG03', 'SG04']


def read(path):
    return json.loads(path.read_text('utf-8-sig'))


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


assert not (OUT / 'visual-acceptance-01.json').exists(), 'Accepted reviews are frozen.'
ledger = read(BASE / 'skill-icon-acceptance.json')
approved = {c['id'] for c in ledger['compact_accepted']}
assert len(approved) == 91 and {'SG01', 'SG02'} <= approved
accepted_files = [f for c in ledger['accepted'] + ledger['compact_accepted'] for f in c['files'].values()]
for file in accepted_files:
    assert digest(ROOT / file['file']) == file['sha256'], file['file']
protected_paths = [*(ROOT / 'assets/skills/agent').rglob('*'), BASE / 'skill-icon-acceptance.json', BASE / 'skill-window-review-05/skills.json']
protected = {p.relative_to(ROOT).as_posix(): digest(p) for p in protected_paths if p.is_file()}
source_review = read(SOURCE / 'review.json')
candidates = [c.copy() for c in source_review['candidates'] if c['id'] in IDS]
assert [c['id'] for c in candidates] == IDS
assert [c['number'] for c in candidates] == [92, 95, 96]
(OUT / 'prompts').mkdir(exist_ok=True)
prompts = []
for candidate in candidates:
    assert candidate['id'] not in approved, candidate['id']
    assert candidate['visual_status'] == 'awaiting_visual_review'
    assert candidate['variant'] == candidate['id'] + '-C01'
    for file in [candidate['master'], *candidate['files'].values()]:
        src = SOURCE / file['file']
        assert digest(src) == file['sha256'], src
        shutil.copyfile(src, OUT / file['file'])
        assert digest(OUT / file['file']) == file['sha256'], file['file']
        with Image.open(OUT / file['file']) as image:
            assert image.size == (file['width'], file['height'])
    prompt = SOURCE / candidate['prompt']
    shutil.copyfile(prompt, OUT / candidate['prompt'])
    prompts.append(candidate['variant'] + '\n\n' + prompt.read_text('utf-8'))
    candidate['source_review'] = 'SKILLS-IC52'
    candidate['source_review_reference'] = '../skill-icon-review-52/review.json'
    candidate['reused_without_drawing_changes'] = True
(OUT / 'prompts.txt').write_text('\n\n'.join(prompts), 'utf-8')
review = {'review_id': 'SKILLS-IC54', 'title': 'Grenades · trois icônes restantes',
          'date': '2026-10-11', 'status': 'awaiting_visual_review',
          'scope': 'Only the three remaining compact drawings from IC52. SG01-C02 and SG02-C02 are already accepted and excluded.',
          'generation_mode': 'reuse_existing_built_in_outputs', 'no_new_drawings': True,
          'candidates': candidates, 'protected_hashes': [{'file': file, 'sha256': sha} for file, sha in protected.items()]}
write(OUT / 'review.json', review)
write(OUT / 'validation.json', {'review_id': 'SKILLS-IC54', 'status': 'awaiting_visual_review',
      'candidate_count': 3, 'candidate_numbers': [92, 95, 96],
      'reused_exports_verified': 15, 'reused_masters_verified': 3,
      'accepted_export_hashes_verified': len(accepted_files), 'runtime_changes': False,
      'preserved_runtime_and_catalogue_files': len(protected)})
for file, sha in protected.items():
    assert digest(ROOT / file) == sha, file
print(json.dumps({'review': 'SKILLS-IC54', 'candidates': 3, 'exports_reused': 15,
                  'accepted_exports_preserved': len(accepted_files), 'protected_files': len(protected)}))
