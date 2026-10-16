"""Consigner l'accord visuel du joueur pour le Possédé TR64-BODY05."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-47'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Accord déjà consigné ; ne pas remplacer la preuve.'
review_path = OUT / 'review.json'
review = json.loads(review_path.read_text(encoding='utf-8'))
assert review['candidate'] == 36 and review['variant'] == 'TR64-BODY05'
assert review['status'] == 'awaiting_visual_review'
assert hashlib.sha256((OUT / 'possede-source.png').read_bytes()).hexdigest() == review['source_sha256']
html_path = OUT / 'index.html'
html = html_path.read_text(encoding='utf-8')
html = html.replace('TR64-BODY05 · proposition de recoloration', 'TR64-BODY05 · apparence validée')
html = html.replace('La variante attend votre validation visuelle.', 'Cette apparence a été validée le 13 octobre 2026.')
html_path.write_text(html, encoding='utf-8')
files = sorted(p.relative_to(OUT).as_posix() for p in OUT.rglob('*')
    if p.is_file() and p.name not in ('review.json', receipt.name))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
acceptance = {
    'date': '2026-10-13',
    'user_reply': 'oui là je valide cette transformation',
    'accepted': [{'number': 36, 'id': 'TR64', 'name': 'Possédé', 'body': 'TR64-BODY05'}],
    'scope': 'Shown body recolour: red muscles, flesh-coloured tendons across upper and lower body, ivory bones and red eyes. High-definition source and direct64px export.',
    'retained_rules': 'Previously approved transformation mechanics and animations retain their own status. This review changes only the body appearance.',
    'previous_trials': 'BODY02, BODY03 and BODY04 remain preserved; BODY05 is the accepted appearance reference.',
    'campaign_integration': False,
    'artifact_hashes': proof,
}
receipt.write_text(json.dumps(acceptance, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
review.update(status='visually_approved', body_status='visually_approved',
              acceptance=receipt.name, accepted_date='2026-10-13',
              canonical_body_reference='TR64-BODY05 / skill-animation-review-47')
review_path.write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
assert all(hashlib.sha256((OUT / name).read_bytes()).hexdigest() == digest for name, digest in proof.items())
print(json.dumps({'variant': 'TR64-BODY05', 'status': review['status'], 'hashed_artifacts': len(proof)}))
