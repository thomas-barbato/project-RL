"""Read-only runtime audit and preparation of this numbered review."""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
OUT = Path(__file__).resolve().parent

def read(path):
    return json.loads(path.read_text('utf-8-sig'))

def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

review = read(OUT / 'review.json')
for entry in review['protected_hashes']:
    assert digest(ROOT / entry['file']) == entry['sha256'], entry['file']
prior_drawings = [p for n in (44, 45, 46) for p in (BASE / f'skill-icon-review-{n}').glob('*.png')]
write(OUT / 'previous-candidates-protected.json', [
    {'file': p.relative_to(ROOT).as_posix(), 'sha256': digest(p)} for p in prior_drawings
])

catalogue = read(ROOT / 'assets/skills/agent/catalogue.json')
ledger = read(BASE / 'skill-icon-acceptance.json')
approved_illustrations = {s['id']: s for s in ledger['accepted']}
present = []
missing = []
for skill in catalogue['skills']:
    sid = skill['id']
    art = catalogue['art'].get(sid, {})
    if 'illustration' in art:
        path = ROOT / 'assets/skills/agent' / art['illustration']
        original = approved_illustrations[sid]['files']['192']
        assert path.is_file() and digest(path) == original['sha256'], sid
        present.append({'id': sid, 'title': skill['title'], 'file': path.relative_to(ROOT).as_posix(), 'sha256': digest(path)})
    else:
        missing.append({'id': sid, 'title': skill['title'], 'branch': skill['branch'],
                        'compact_available': 'compact' in art, 'status': 'dedicated_illustration_to_prepare_and_review'})
write(OUT / 'illustration-coverage-audit.json', {
    'date': '2026-10-11', 'skills': len(catalogue['skills']),
    'existing_illustrations_verified': len(present), 'missing_illustrations': len(missing),
    'missing_with_accepted_compact': sum(s['compact_available'] for s in missing),
    'cause': 'The catalogue contains no illustration entry for these skills; draw_agent_skill_detail renders a large image only when that entry exists.',
    'uniform_policy': 'Dedicated 192 px illustration centered after roleplay and before numerical effects; compact icons remain separate.',
    'policy_reference': 'docs/DIRECTION_ARTISTIQUE_TILES.md:952',
    'user_policy_confirmation': 'euh, ca doit respecter la meme politique partout :) donc la meme regle qu\u0027ailleurs',
    'runtime_changed': False, 'present': present, 'pending': missing
})

renderer = (BASE / 'skill-icon-review-46/render_review_board.cjs').read_text('utf-8')
renderer = renderer.replace('HL04-C03', 'HL04-C04').replace('SKILLS-IC46', 'SKILLS-IC47')
renderer = renderer.replace('Acier et signe « % » pour le bonus de dégâts.', 'Le même chevron bleu que la maîtrise à distance.')
renderer = renderer.replace('signe %', 'chevron bleu').replace('signe pourcentage', 'chevron bleu')
renderer = renderer.replace('Nouvelle silhouette d’arme lourde en acier, avec un signe « % » pour évoquer le bonus de dégâts.', 'Le double chevron bleu reprend celui de Maîtrise des attaques à distance.')
(OUT / 'render_review_board.cjs').write_text(renderer, 'utf-8')

previous = BASE / 'skill-icon-review-46/review.json'
previous_data = read(previous)
previous_data['status'] = 'superseded_by_requested_symbol_revision'
previous_data['candidates'][0]['visual_status'] = 'revision_requested'
previous_data['candidates'][0]['latest_proposal_reference'] = '../skill-icon-review-47/review.json'
write(previous, previous_data)
full = BASE / 'skill-icon-review-44/review.json'
full_data = read(full)
for candidate in full_data['candidates']:
    if candidate['id'] == 'HL04':
        candidate['latest_proposal_reference'] = '../skill-icon-review-47/review.json'
        candidate['revision_variant'] = 'HL04-C04'
write(full, full_data)
page = BASE / 'skill-icon-review-44/index.html'
page.write_text(page.read_text('utf-8').replace('skill-icon-review-46', 'skill-icon-review-47').replace('HL04-C03', 'HL04-C04'), 'utf-8')

for entry in review['protected_hashes']:
    assert digest(ROOT / entry['file']) == entry['sha256'], entry['file']
for entry in read(OUT / 'previous-candidates-protected.json'):
    assert digest(ROOT / entry['file']) == entry['sha256'], entry['file']
print(json.dumps({'illustrations_verified': len(present), 'missing': len(missing),
                  'previous_pngs_preserved': len(prior_drawings), 'runtime_files_preserved': len(review['protected_hashes'])}))
