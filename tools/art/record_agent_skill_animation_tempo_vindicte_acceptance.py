"""Freeze the explicitly accepted ANIM-S22 visual candidates."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-51'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists(), 'This visual approval is already recorded.'
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
checks = json.loads((OUT / 'verification.json').read_text(encoding='utf-8'))
assert review['status'] == 'awaiting_visual_review'
assert [(c['number'], c['id'], c['variant']) for c in review['candidates']] == [
    (53, 'BM03', 'BM03-A01'), (54, 'BM05', 'BM05-A01')]
assert len(checks['controls']) == 28 and len(checks['animations']) == 29
assert all(c['world_walls_clear'] and c['waiting_does_not_replay_impact']
           and c['charges_or_turn_duration_checked'] and c['preview_ends_on_start']
           for c in checks['controls'])
assert all(c['received_damage_retained_and_exclusions_checked']
           for c in checks['controls'] if c['variant'].startswith('BM05-'))

protection = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
assert all(hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == digest
           for name, digest in protection.items()), 'A protected reference changed.'
for name in ['index.html', 'cote-a-cote.html']:
    path = OUT / name
    path.write_text(path.read_text(encoding='utf-8')
                    .replace('rendu à valider', 'rendus validés')
                    .replace('-A01 · proposé', '-A01 · validé'), encoding='utf-8')

excluded = {'review.json', 'validation.json', 'review-protection.json', receipt.name}
files = sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*')
               if p.is_file() and p.name not in excluded
               and 'native-glyph-draft' not in p.parts)
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
record = {
    'date': '2026-10-13',
    'user_reply': 'je valide :)',
    'presented_preview': 'skill-animation-review-51/cote-a-cote.html',
    'accepted': review['candidates'],
    'scope': 'BM03-A01 automatic charge icon, hover bonus and included damage bonus; '
             'BM05-A01 original magic projectile returned with unchanged appearance; '
             'shared harmful-hit reaction, physical wound blood without magical blood, '
             'four directions and the shown waiting, reset, exclusion and expiry cases.',
    'numeric_mechanics': review['approved_values'],
    'prepared_numbers': review['prepared_numbers'],
    'pending_cases': review['pending'],
    'turn_scope': review['turn_scope'],
    'temporary_actors': review['temporary_actors'],
    'excluded_draft': 'native-glyph-draft is an unreviewed capture and is not accepted.',
    'campaign_integration': False,
    'artifact_hashes': proof,
}
receipt.write_text(json.dumps(record, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
review.update(status='visually_approved', acceptance=receipt.name, accepted_date='2026-10-13')
for candidate in review['candidates']:
    candidate.update(visual_status='visually_approved', acceptance=receipt.name)
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n',
                               encoding='utf-8')
validation = json.loads((OUT / 'validation.json').read_text(encoding='utf-8'))
validation.update(visual_status='visually_approved', acceptance=receipt.name,
                  visual_approval='BM03-A01 and BM05-A01 accepted.')
(OUT / 'validation.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n',
                                   encoding='utf-8')
assert all(hashlib.sha256((OUT / name).read_bytes()).hexdigest() == digest
           for name, digest in proof.items())
print(json.dumps({'approved': ['BM03-A01', 'BM05-A01'], 'hashed_artifacts': len(proof),
                  'protected_references': len(protection), 'campaign_integration': False}))
