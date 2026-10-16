"""Freeze the explicitly accepted ANIM-S23 visual candidates."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-52'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists(), 'This visual approval is already recorded.'
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
checks = json.loads((OUT / 'verification.json').read_text(encoding='utf-8'))
assert review['status'] == 'awaiting_visual_review'
assert [(c['number'], c['id'], c['variant']) for c in review['candidates']] == [
    (55, 'BD04', 'BD04-A01'), (56, 'BD06', 'BD06-A01')]
assert len(checks['controls']) == 27 and len(checks['animations']) == 28
assert all(c['world_effects_and_recoil_blocked_by_walls']
           and c['HUD_transfer_separate_from_damage']
           and c['passive_counts_and_resource_limits']
           and c['no_idle_actions_or_artificial_buff_timer'] for c in checks['controls'])
assert json.loads((OUT / 'gallery-controller-verification.json').read_text(encoding='utf-8'))['controller'] == 'passed'

protection = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
assert all(hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == digest
           for name, digest in protection.items()), 'A protected reference changed.'
for name in ['index.html', 'cote-a-cote.html']:
    path = OUT / name
    path.write_text(path.read_text(encoding='utf-8')
                    .replace('rendu à valider', 'rendus validés')
                    .replace('-A01 · proposé', '-A01 · validé'), encoding='utf-8')

excluded = {'review.json', 'validation.json', 'review-protection.json', receipt.name}
drafts = {'native-feedback-draft', 'native-menu-draft'}
files = sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*')
               if p.is_file() and p.name not in excluded and not drafts.intersection(p.parts))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
record = {
    'date': '2026-10-13',
    'user_reply': 'je valide :)',
    'presented_preview': 'skill-animation-review-52/cote-a-cote.html',
    'accepted': review['candidates'],
    'scope': 'BD04-A01 tightening red reticle, charge icon and precision hover; '
             'BD06-A01 violet resource transfer to the energy gauge and numerical gain; '
             'physical wound blood, shared reaction, four directions and the shown '
             'waiting, movement, untimed-menu, resource-limit and excluded-source cases.',
    'numeric_mechanics': review['approved_values'],
    'numeric_references': review['approved_cards'],
    'prepared_numbers': review['prepared_numbers'],
    'review_scope': review['scope'],
    'pending_cases': review['pending'],
    'temporary_actors': review['temporary_actors'],
    'excluded_drafts': sorted(drafts),
    'campaign_integration': False,
    'artifact_hashes': proof,
}
receipt.write_text(json.dumps(record, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
review.update(status='visually_approved', acceptance=receipt.name, accepted_date='2026-10-13')
for candidate in review['candidates']:
    candidate.update(visual_status='visually_approved', acceptance=receipt.name)
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
validation = json.loads((OUT / 'validation.json').read_text(encoding='utf-8'))
validation.update(visual_status='visually_approved', acceptance=receipt.name,
                  visual_approval='BD04-A01 and BD06-A01 accepted.')
(OUT / 'validation.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
assert all(hashlib.sha256((OUT / name).read_bytes()).hexdigest() == digest
           for name, digest in proof.items())
print(json.dumps({'approved': ['BD04-A01', 'BD06-A01'], 'hashed_artifacts': len(proof),
                  'protected_references': len(protection), 'campaign_integration': False}))
