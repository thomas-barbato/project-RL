"""Prepare M15-M20 with canonical anatomy and the accepted placement rule."""
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
target=DOC/'creature-jobs-lot03.json'
if target.exists():raise SystemExit('Preserve selected lot03 jobs and corrections; do not overwrite')
records={r['id']:r for r in json.loads((ROOT/'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']}
specs=[
    ('MON-063-0','Quadrupède à queue',4,'creatures-lot02/MON-064-0',
     'Exactly four weight-bearing limbs and the ONE existing long thin brown tail. Preserve the tan/yellow limbs, brown upper body and central back crest, broad dark claws and low crouched posture. No added head ornament, extra legs, armor or open back cavity from INPUT2. Compact four-limb walk with distinct fore/hind support; tail follows softly without growing.'),
    ('MON-087-0','Quadrupède trapu',4,'creatures-lot01/MON-075-0',
     'Exactly four legs on the broad toadlike body. Preserve the olive-brown surfaces, large rounded ochre belly, low wide head and short front/hind limbs. No tail, shell, tusks, human torso or upright posture. Compact low four-foot crawl, NOT a hop: belly volume stays constant and the body never jumps.'),
    ('MON-103-0','Quadrupède à plaques',4,'creatures-lot01/MON-075-0',
     'Exactly four weight-bearing limbs. Preserve the pale tan low body, broad dark claws and existing large charcoal dorsal plates. Keep the original small/obscured front head between the forelimbs; do not add a large face, horns, extra shell or equipment. Measured four-foot walk, distinct fore/hind supports, low stable body. Plates retain their identity and never bend like cloth.'),
    ('MON-039-0','Petite créature ailée',2,'creatures-lot01/MON-144-0',
     'Exactly two walking hind legs, two arms and two wings. Preserve the small reddish-brown demon with its ochre chest/belly, tall pointed ears/horns, dark wing membranes, long fingers and clawed feet. No tail or equipment absent from the canonical input; do not copy the reference demon\'s bulky proportions. Compact grounded two-foot walk; wings carried with tiny joint motion, never flight.'),
    ('MON-126-0','Esprit étroit',0,'creatures-lot01/MON-124-0',
     'NO walking legs or feet. Preserve the exceptionally NARROW upright hooded spirit, ivory/tan long garment, charcoal face void and slim dark side strips. Preserve the original narrow width relative to height; do not widen it to fill the cell or add broad cloak wings, eyes, arms or feet. Very slight coherent drifting and cloth folds, with constant height and body volume.'),
    ('MON-146-0','Créature ailée dorée',2,'creatures-lot01/MON-144-0',
     'Exactly two hind walking legs, two arms and two broad wings. Preserve the existing golden/ochre plated surfaces, large horns, dark membranes, dark waist cloth and dark claw tips. Keep the original broad wingspan well outside both arms. No new tail, weapon, costume, red skin or blue skin. Heavy but compact grounded alternating two-foot walk; wings remain carried spread with slight joint movement, no flight.')
]
jobs=[]
for number,(key,label,legs,guide,motion) in enumerate(specs,15):
    identity=DOC/'references'/f'{key}-identity-detail.png'
    native=DOC/'references'/f'{key}-native-current.png'
    guide=SRC/'accepted'/f'{guide}-whole-body-native.png'
    assert identity.exists() and native.exists() and guide.exists()
    prompt=f'''Use case: identity-preserve. Complete lateral movement cycle of existing game creature {key}.
INPUT1 is the exact canonical APPEARANCE: preserve species, proportions, number of limbs, face, plates, wings/tail where present, materials and colors. Isolated colored edge specks in the old export are not part of the anatomy or glow.
INPUT2 is ONLY the accepted coherent WHOLE-FIGURE movement and layout reference: two rows, seven poses per row. Do not copy its species, body bulk, colors, armor or appendages. Use compact supports and the same matte native64-readable game-art finish.
SPECIFIC ANATOMY AND MOVEMENT: {motion}
OUTPUT EXACTLY FOURTEEN complete separate creatures: exactly TWO ROWS with exactly SEVEN FIGURES per row on a wide transparent canvas, approximately1792x640. No third/fourth row, no eighth column, no extra idle duplicates. Generous transparent margins around every complete figure, including all horn/wing/claw/tail tips. Adjacent figures must never touch.
TOP ROW: ALL seven face screenLEFT in near-front three quarters; idle then L0,L1,L2,L3,L4,L5. BOTTOM ROW: ALL face RIGHT; idle then R0,R1,R2,R3,R4,R5. Head, torso, knees/feet or chassis face together. Never back view, pure profile, head-only reversal or direction change within a row.
Each set of six walking poses is ONE compact cycle: three poses on one grid cell, next three on the following cell,400ms per cell. Real stable resting idle, no frozen raised foot at arrival. Low restrained bob and clear alternating supports appropriate to this anatomy; no wide splits, stretching, hopping or swelling.
Draw each WHOLE creature coherently, with constant apparent scale, height, mass, limb ownership and materials across all14 poses. Keep canonical source proportions even when there is extra cell space. No grafted head/legs or procedural strips. Transparent gaps and intentional holes remain real alpha.
Genuine transparent alpha background, clean dark contours, matte useful detail and readable forms. No labels, grid, floor, ground shadow, halos, colored fringe/specks, effects or particles. No new equipment or redesign.'''
    prompt_path=DOC/'prompts'/f'{key}-lot03-r01.txt';prompt_path.write_text(prompt,'utf8')
    jobs.append({'id':key,'review_id':f'WALK07-M{number:02}','kind':'creature','label':label,
                 'record':records[key],'locomotor_legs':legs,'motion':motion,'reference':str(identity),
                 'shadow':records[key]['rig']!='float','strict_source_outer_margin':True,
                 'references':[str(identity),str(guide)],'source_destination':str(SRC/f'{key}-cycle-lot03-r01.png'),
                 'source_columns':7,'source_rows':2,'selected_source_indices':list(range(14)),
                 'prompt_file':str(prompt_path.relative_to(ROOT)),'prompt':prompt})
target.write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
print(json.dumps({'review_ids':[j['review_id'] for j in jobs],'identities':[j['id'] for j in jobs],'supports':[j['locomotor_legs'] for j in jobs],'runtime_integration':False}))
