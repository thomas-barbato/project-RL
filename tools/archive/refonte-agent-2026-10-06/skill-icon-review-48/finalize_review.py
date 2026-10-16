"""Prepare the numbered presentation; do not change game drawings."""
from pathlib import Path
import hashlib
import json

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent

def read(path):
    return json.loads(path.read_text('utf-8-sig'))

def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

previous = BASE / 'skill-icon-review-47'
prior_review = read(previous / 'review.json')
candidate = prior_review['candidates'][0]
for item in [candidate['master'], *candidate['files'].values()]:
    assert digest(previous / item['file']) == item['sha256'], item['file']

renderer = (previous / 'render_review_board.cjs').read_text('utf-8')
renderer = renderer.replace('HL04-C04', 'HL04-C05').replace('SKILLS-IC47', 'SKILLS-IC48')
renderer = renderer.replace('Le même chevron bleu que la maîtrise à distance.', 'Chargeur épaissi et perspective corrigée.')
renderer = renderer.replace('Le double chevron bleu reprend celui de Maîtrise des attaques à distance.', 'Le chargeur montre désormais une face principale, un côté épais et une face supérieure cohérente avec l’arme.')
renderer = renderer.replace('chevron bleu</title>', 'perspective du chargeur</title>')
(OUT / 'render_review_board.cjs').write_text(renderer, 'utf-8')

feedback = {'date': '2026-10-11', 'number': 81, 'skill_id': 'HL04',
            'affected_variant': 'HL04-C04', 'next_variant': 'HL04-C05',
            'user_feedback': "y’a un probleme de perspective sur l’arme : son chargeur est applatit.",
            'scope': 'Magazine perspective only; keep blue chevron and other weapon parts.',
            'status': 'revision_requested', 'no_approval_inferred': True}
write(previous / 'feedback-01.json', feedback)
prior_review['status'] = 'superseded_by_requested_magazine_revision'
prior_review['candidates'][0]['visual_status'] = 'revision_requested'
prior_review['candidates'][0]['latest_proposal_reference'] = '../skill-icon-review-48/review.json'
write(previous / 'review.json', prior_review)
write(OUT / 'revision-feedback.json', feedback)

full_path = BASE / 'skill-icon-review-44/review.json'
full = read(full_path)
for entry in full['candidates']:
    if entry['id'] == 'HL04':
        entry['latest_proposal_reference'] = '../skill-icon-review-48/review.json'
        entry['revision_variant'] = 'HL04-C05'
write(full_path, full)
page = BASE / 'skill-icon-review-44/index.html'
page.write_text(page.read_text('utf-8').replace('skill-icon-review-47', 'skill-icon-review-48'), 'utf-8')

review = read(OUT / 'review.json')
for entry in review['protected_hashes']:
    assert digest(ROOT / entry['file']) == entry['sha256'], entry['file']
for item in [candidate['master'], *candidate['files'].values()]:
    assert digest(previous / item['file']) == item['sha256'], item['file']
validation_path = OUT / 'validation.json'
validation = read(validation_path)
validation['previous_candidate_master_and_exports_preserved'] = True
validation['runtime_changes'] = False
write(validation_path, validation)
print('Previous candidate and all runtime drawings preserved.')
