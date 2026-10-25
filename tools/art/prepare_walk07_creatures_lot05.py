"""Prepare M27-M32 from inspected source anatomies, without runtime changes."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-07'
SRC = ROOT / 'assets/source-art/player-walk-review-07'
target = DOC / 'creature-jobs-lot05.json'
if target.exists():
    raise SystemExit('Preserve existing selected lot05 jobs and corrections')
records = {r['id']: r for r in json.loads((ROOT / 'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']}
specs = [
    ('MON-035-0', 'Petit quadrupède pâle', 4, 'creatures-lot02/MON-064-0',
     'Exactly FOUR weight-bearing limbs, two front and two rear. Preserve the low pale flesh mound, red frontal mouth/opening, short dark claws and red creases of INPUT1. Small compact alternating four-foot supports, not a humanoid walk. Do not copy dorsal plates, a tail, armor, horns or an extra head from the guide.'),
    # Close inspection after r01 corrected the initial four-support wording.
    # The exact original prompt remains in prompts/ and the generation journal.
    ('MON-076-0', 'Créature aux appendices recourbés', 2, 'creatures-lot03/MON-103-0',
     'TWO main supporting feet and TWO existing curled lateral appendages, four visible limbs total, kept distinct. Preserve the tall pale flesh back with the long red front cleft, dark charcoal dorsal ridge, strong short front claws and curled lateral appendages. Do not turn the curls into extra walking legs or erase them. Two compact supporting steps with little torso bob. Preserve the original heavy arched silhouette and height, no added face, equipment, horns or new tail.'),
    ('MON-101-0', 'Quadrupède à carapace côtelée', 4, 'creatures-lot03/MON-103-0',
     'Exactly FOUR supporting limbs: large pale front limbs with dark spread claws and shorter rear limbs under the shell. Preserve the huge domed BRONZE and CHARCOAL vertically ribbed shell, pale flesh, low obscured face and short body of INPUT1. Shell remains rigid, fixed size and attached to the body through the compact quadruped gait; never turn it into armor on a human, add a tail or copy the guide dorsal plates.'),
    ('MON-070-0', 'Créature aux avant-bras recourbés', 4, 'creatures-lot03/MON-103-0',
     'Exactly FOUR long angular supporting legs and TWO separate raised curved/scythe-like upper forelimbs, six limbs total. The two scythes are arms, NOT extra feet. Preserve the upright tapered olive/ochre head and thorax, pale reddish segmented center, long dark-tipped lower legs, two curved forearms and original lean chitin silhouette. Compact alternating four-foot gait underneath a stable upright thorax; scythe arms move only subtly. No wings, new tail, human feet or added limbs.'),
    ('MON-065-0', 'Créature au corps enroulé', 0, 'creatures-lot01/MON-141-0',
     'NO walking legs or feet. Preserve the olive/brown scaled SNAKE lower body, continuous cream segmented belly and thick coiled base, TWO clawed arms, hunched reptilian upper torso and pointed reptilian head. The continuous coiled body advances through restrained traveling contractions; arms remain arms, never become supports. Constant upper-body height and apparent length; no vertical springing, shrinking, new tail copies, book, robe or worm head from INPUT2. Preserve the existing single continuous tapering tail.'),
    ('MON-066-0', 'Créature aux bras ailés', 2, 'creatures-lot04/MON-147-0',
     'Exactly TWO hind walking legs and TWO original wing-bearing upper limbs, with hands/claws at the raised wing joints as in INPUT1. Do not invent a second extra pair of free arms by copying INPUT2. Preserve the lean reddish/brown body, cream chest and abdomen, two pointed ivory horns, the broad low-hanging reddish wing membranes and dark talons. Grounded compact two-foot walk, restrained bob; wings held spread with only small joint motion. No purple materials, new armor, tail, weapon or flight.')
]
jobs = []
for number, (key, label, legs, guide, motion) in enumerate(specs, 27):
    identity = DOC / 'references' / f'{key}-identity-detail.png'
    native = DOC / 'references' / f'{key}-native-current.png'
    guide = SRC / 'accepted' / f'{guide}-whole-body-native.png'
    assert identity.exists() and native.exists() and guide.exists(), key
    prompt = f'''Use case: identity-preserve. Complete lateral game-creature animation for existing identity {key}.
INPUT1 is the exact canonical APPEARANCE. Preserve anatomy, head/face, supports, all original appendages, materials, colors and proportions. Isolated colored fringe specks in the old export are not glow or anatomy.
INPUT2 is ONLY an accepted WHOLE-FIGURE motion and layout guide. Do not copy its species, face, bulk, armor, color, limbs or objects. Use compact supports and the same matte native64-readable game-art finish.
SPECIFIC ANATOMY AND MOVEMENT: {motion}
OUTPUT EXACTLY FOURTEEN separate COMPLETE figures, exactly TWO ROWS of exactly SEVEN figures on a wide transparent atlas approximately1792x640. No extra rows, eighth column, labels or duplicates. Each whole figure occupies at most75percent of its cell WIDTH AND HEIGHT, all using the SAME scale. Generous transparent margins around every claw, coil, wing and appendage; at least24px from the OUTER canvas edge. No figures touch and no body parts are clipped. Do not fill the cells.
TOP ROW: ALL face screenLEFT in near-front three quarters, idle then L0,L1,L2,L3,L4,L5. BOTTOM ROW: ALL face RIGHT, idle then R0,R1,R2,R3,R4,R5. Head, torso and feet/base face the SAME direction together. No back view, pure profile, head-only reversal or direction changes inside a row. The two rows must look visibly different in their lateral orientation.
Six motion poses form ONE full compact cycle, three phases on one grid cell and the next three on the next cell,400ms per cell. Real stable resting idle. Use actual source morphology, never a human gait for a legless form.
Draw EACH creature WHOLE and coherent. Constant apparent scale, mass, head size, anatomy and materials across all14 poses; restrained bob, no great split, stretching, hopping or swelling. No grafted parts, procedural strips or detached limbs. Canonical source proportions stay unchanged despite empty cell space.
Genuine transparent alpha, clean dark matte game-art contours and readable shapes. No floor, ground shadow, text, grid, effects, particles, halo, colored fringe/specks, extra equipment or redesign.'''
    prompt_path = DOC / 'prompts' / f'{key}-lot05-r01.txt'
    prompt_path.write_text(prompt, 'utf8')
    jobs.append({'id': key, 'review_id': f'WALK07-M{number:02}', 'kind': 'creature', 'label': label,
                 'record': records[key], 'locomotor_legs': legs, 'motion': motion,
                 'reference': str(identity), 'references': [str(identity), str(guide)],
                 'source_destination': str(SRC / f'{key}-cycle-lot05-r01.png'),
                 'source_columns': 7, 'source_rows': 2, 'selected_source_indices': list(range(14)),
                 'shadow': True, 'strict_source_outer_margin': True,
                 'prompt_file': str(prompt_path.relative_to(ROOT)), 'prompt': prompt})
target.write_text(json.dumps(jobs, ensure_ascii=False, indent=2) + '\n', 'utf8')
print(json.dumps({'review_ids': [j['review_id'] for j in jobs], 'supports': [j['locomotor_legs'] for j in jobs], 'runtime_integration': False}))
