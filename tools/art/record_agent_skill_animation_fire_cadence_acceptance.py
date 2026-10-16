"""Record the approved cadence separately from the requested turn-lifetime fix."""
import hashlib
import json
from datetime import date
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-14'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Acceptance already recorded.'
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
accepted = []
for candidate in review['candidates']:
    source = OUT / (candidate['variant'] + '-64.gif')
    accepted.append(dict(candidate, file=source.name, sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
                         accepted_parts=['drawing', 'eight_drawn_frames_per_second', 'attached_burn_drawing'],
                         status='cadence_and_drawing_approved'))
    candidate['status'] = 'cadence_and_drawing_approved'
review['status'] = 'cadence_and_drawing_approved_with_turn_lifetime_revision_requested'
review['acceptance'] = 'visual-acceptance-01.json'
review['timing_revision'] = 'Keep visual effect present until the relevant game turn passes; no automatic expiry in seconds.'
proof = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(OUT.glob('*'))
         if p.is_file() and (p.suffix == '.gif' or p.name.endswith('source.rs'))}
receipt.write_text(json.dumps({'date':str(date.today()),'review':review['batch'],'user_reply':'d\u0027accord je valide :)',
    'user_clarification':'L\u0027animation doit durée tant que le tour (si la compétence ne dure qu\u0027un tour) n\u0027est pas passé',
    'accepted':accepted,'scope':'Drawing and slower cadence accepted. Automatic lifetime in seconds must be replaced by turn-driven visual persistence. No campaign integration or damage changes.',
    'artifact_hashes':proof},ensure_ascii=False,indent=2),encoding='utf-8')
(OUT / 'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2),encoding='utf-8')
html = (OUT / 'index.html').read_text(encoding='utf-8')
html = html.replace('essai à examiner','dessin et cadence validés · maintien par tours à corriger')
(OUT / 'index.html').write_text(html,encoding='utf-8')
print('Approved cadence recorded; visual lifetime correction remains separate.')
