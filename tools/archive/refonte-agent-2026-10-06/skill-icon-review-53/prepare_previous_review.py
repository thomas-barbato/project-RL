from pathlib import Path
import hashlib
import json

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
PREVIOUS = BASE / 'skill-icon-review-52'

def read(path):
    return json.loads(path.read_text('utf-8'))

def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')

review = read(PREVIOUS / 'review.json')
assert len(review['candidates']) == 5
protected = {}
for c in review['candidates']:
    for f in [c['master'], *c['files'].values()]:
        actual = hashlib.sha256((PREVIOUS / f['file']).read_bytes()).hexdigest()
        assert actual == f['sha256'], f['file']
        protected[f['file']] = actual
    if c['id'] in ('SG01', 'SG02'):
        c['visual_status'] = 'revision_requested'
        c['revision_review'] = 'SKILLS-IC53'
review['status'] = 'two_revisions_requested_three_candidates_awaiting_approval'
write(PREVIOUS / 'review.json', review)
write(OUT / 'prior-grenade-drawings.json', protected)
write(OUT / 'revision-request.json', {
    'review_id': 'SKILLS-IC53', 'date': '2026-10-11',
    'user_requests': [
        'Grenade aveuglantes et leurrent devraient être des grenades plutot de forme cylindriques (et sans haut parleur pour le leur...) .',
        'change juste leurs couleurs pour les differencier :)'
    ],
    'scope': 'Same cylindrical model for SG01 and SG02; color only differentiates them, without a speaker or surrounding effect symbols. Creation authorized; drawings require their own visual approval.',
    'unmodified_pending_candidates': ['SG00-C01', 'SG03-C01', 'SG04-C01'],
    'no_visual_acceptance_inferred': True
})
page = PREVIOUS / 'index.html'
page.write_text(page.read_text('utf-8').replace('Ces cinq dessins attendent leur validation.', 'Les candidats 93 et 94 sont repris sous forme cylindrique. <a href="../skill-icon-review-53/index.html">Corrections de Grenade aveuglante et Leurre</a>. Les trois autres propositions restent à valider.'), 'utf-8')
validation = read(PREVIOUS / 'validation.json')
validation['status'] = review['status']
write(PREVIOUS / 'validation.json', validation)
print('Two revisions recorded; all thirty prior grenade drawings preserved.')
