"""Record the player's approval of the two models presented in the T-1 review."""
from pathlib import Path
import hashlib
import json

OUT = Path(__file__).resolve().parent
RECORD = 'review-decisions-2026-10-06.json'
NUMBERS = {59, 66}


def read(name):
    return json.loads((OUT / name).read_text(encoding='utf-8'))


def write(name, value):
    (OUT / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


before = {p.name: digest(p) for p in (OUT / 'tiles').glob('*.png')}
manifest = read('manifest.json')
decisions = []
for entry in manifest:
    if entry['number'] in NUMBERS:
        decisions.append(dict(
            number=entry['number'], review_id=entry['review_id'], name=entry['name'],
            status='accepted-art', scope=['living', 'corpse'],
            sources={key: dict(path=entry[key], sha256=digest(OUT / entry[key]))
                     for key in ('source', 'living', 'corpse')},
        ))
assert {d['number'] for d in decisions} == NUMBERS
assert not any(e['number'] == 60 for e in manifest)
write(RECORD, dict(
    date='2026-10-06', user_statement="ok j'accepte :)",
    reviewed_round='t1-and-merged-guardian', accepted=2, decisions=decisions,
    accepted_merge=dict(retired_number=60, retained_number=66, shield_removed=True),
    review_capture=dict(path='review/59-66-propositions-r3.png',
                        sha256=digest(OUT / 'review/59-66-propositions-r3.png')),
    scope='Current 59 and 66 living sprites and dry wrecks; merger of 60 into 66. No gameplay, talent or statistics changes inferred.',
    integration_status='accepted art; not integrated',
))
for name in ('catalog.json', 'manifest.json'):
    entries = read(name)
    for entry in entries:
        if entry['number'] in NUMBERS:
            entry.update(status='accepted-art', art_approval_date='2026-10-06',
                         art_approval_record=RECORD, integration_status='not integrated')
    write(name, entries)
current = read('revision-current.json')
current.update(review_status='accepted-art', approval_record=RECORD,
               integration_status='accepted art; not integrated')
write('revision-current.json', current)
report = read('validation.json')
report.update(current_revision=current, review_status='current revision accepted; other candidates retain their existing review status',
              art_approval_record=RECORD)
write('validation.json', report)
retired = read('retired-candidates.json')
for entry in retired:
    if entry['number'] == 60:
        entry.update(merge_approval_date='2026-10-06', merge_approval_record=RECORD)
write('retired-candidates.json', retired)
readme = OUT / 'README.md'
text = readme.read_text(encoding='utf-8')
note = "Le joueur a validé les versions courantes du **59, Artilleur**, et du **66, Gardien**, ainsi que la fusion du 60 dans le 66, par « ok j'accepte :) ». Leurs sources et leurs empreintes sont conservées dans `review-decisions-2026-10-06.json`. Cette validation couvre les sprites actifs et leurs épaves sans sang ; l'intégration et les capacités de jeu restent à traiter.\n\n"
if note not in text:
    heading, body = text.split('\n\n', 1)
    readme.write_text(heading + '\n\n' + note + body, encoding='utf-8')
assert before == {p.name: digest(p) for p in (OUT / 'tiles').glob('*.png')}
assert {e['number'] for e in read('manifest.json') if e['status'] == 'accepted-art'} == NUMBERS
print(json.dumps(dict(accepted=sorted(NUMBERS), merged={60: 66}, unchanged_tiles=len(before), record=str(OUT / RECORD)), ensure_ascii=False))
