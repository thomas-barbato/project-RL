"""Prepare M51-M56 from the remaining low multi-limbed source silhouettes."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-07'
SRC = ROOT / 'assets/source-art/player-walk-review-07'
target = DOC / 'creature-jobs-lot09.json'
if target.exists():
    raise SystemExit('Preserve existing selected lot09 jobs and corrections')
records = {r['id']: r for r in json.loads((ROOT / 'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']}
base = json.loads((DOC / 'creature-jobs-lot08.json').read_text('utf8'))[0]
specs = [
    ('MON-037-0', 'Petit rampant pâle', 4, 'creatures-lot05/MON-035-0',
     'FOUR short outer clawed supporting limbs, two fore and two hind, around the small pale dome body. Preserve the cream/pink flesh, dark rear dorsal covering, red lower front opening and original dark claw tips. Compact low four-foot alternating support, with stable body volume, original low mound proportions and little bob. No extra insect legs, human torso, new horns, tail, eyes, tusks or extra mouth copied from INPUT2.'),
    ('MON-038-0', 'Créature à ouverture cerclée', 4, 'creatures-lot07/MON-143-0',
     'FOUR existing supporting limbs, two prominent pale front limbs with dark claws and two smaller rear supports beneath the body. Preserve the dark blue-gray upper body/covering, its original raised top corners and the large orange/pink-rimmed dark front opening. The opening faces with the whole body in both directions, retaining its shape and border. Compact four-support crawl, no new head or face inside the opening, human arms, tusks, pale segmented shell, horns, antennae or tail copied from INPUT2.'),
    ('MON-067-0', 'Rampant sombre à bouche ronde', 4, 'creatures-lot04/MON-081-0',
     'FOUR short existing supporting limbs, around the low dark tapered body. Preserve the round pink/rust front opening, dark charcoal rough dorsal ridges/plates, pale joint areas, dark pointed feet and original short tapering rear outline. Mouth/front and supports face the same direction. Small low compact four-foot crawl with restrained bob and constant body length/volume. Do not add a long tail, tusks, extra legs, armored helmet, eyes, spikes or olive material from INPUT2.'),
    ('MON-069-0', 'Créature rouge à six pattes', 6, 'creatures-lot02/MON-031-0',
     'SIX existing angular walking legs in THREE paired rows along the body. Preserve the RED/charcoal chitin body, original raised red back segments/spines, pale joint bands, dark sharp foot tips, original small lowered front head and mouth/mandibles. Keep front, middle and rear leg identities distinct while alternating compact supporting groups. No extra seventh/eighth legs, antennae, wings, blue shell, yellow legs, new tail or oversized human face copied from INPUT2. Constant body length, volume and supported height; no wide split, hopping or bouncing.'),
    ('MON-090-0', 'Créature brune à six pattes claires', 6, 'creatures-lot02/MON-031-0',
     'SIX existing pale articulated clawed walking legs, THREE pairs along the low elongated brown/red dorsal body. Preserve the brown segmented upper covering, cream leg material, dark curved foot tips and small obscured lowered front head. Keep the original long low silhouette and distinct front/middle/rear leg attachments, with compact alternating support groups. No added pincers, tusks, antennae, wings, metal plates, blue/yellow palette, extra legs or tail copied from INPUT2. The body does not inflate or shorten during the gait.'),
    ('MON-109-0', 'Créature brune aux longues pattes', 4, 'creatures-lot08/MON-113-0',
     'FOUR large OUTER angular pale walking supports with dark curved tips around the narrow brown/ridged central body. ALSO preserve its original short central pale front/downward appendage as drawn in INPUT1; do not erase it, duplicate it or turn it into an extra long walking leg. Preserve the brown dorsal ridges, original small front shape, pale bent limb segments and dark ends. Compact four-outer-support walking, original leg lengths and body proportions, little bob. No ivory flat body copied from INPUT2, extra head, eyes, horns, shell, wings, tail, equipment or human hands.')
]
jobs = []
for number, (key, label, legs, guide, motion) in enumerate(specs, 51):
    identity = DOC / 'references' / f'{key}-identity-detail.png'
    native = DOC / 'references' / f'{key}-native-current.png'
    guide = SRC / 'accepted' / f'{guide}-whole-body-native.png'
    assert identity.exists() and native.exists() and guide.exists(), key
    assert base['prompt'].count(base['motion']) == 1
    prompt = base['prompt'].replace(base['id'], key).replace(base['motion'], motion)
    prompt_path = DOC / 'prompts' / f'{key}-lot09-r01.txt'
    prompt_path.write_text(prompt, 'utf8')
    jobs.append({'id': key, 'review_id': f'WALK07-M{number:02}', 'kind': 'creature', 'label': label,
                 'record': records[key], 'locomotor_legs': legs, 'motion': motion,
                 'reference': str(identity), 'references': [str(identity), str(guide)],
                 'source_destination': str(SRC / f'{key}-cycle-lot09-r01.png'),
                 'source_columns': 7, 'source_rows': 2, 'selected_source_indices': list(range(14)),
                 'shadow': True, 'strict_source_outer_margin': True,
                 'prompt_file': str(prompt_path.relative_to(ROOT)), 'prompt': prompt})
target.write_text(json.dumps(jobs, ensure_ascii=False, indent=2) + '\n', 'utf8')
print(json.dumps({'review_ids': [j['review_id'] for j in jobs], 'supports': [j['locomotor_legs'] for j in jobs], 'runtime_integration': False}))
