"""Prepare M21-M26 with their actual supports and carried-book identity."""
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
target=DOC/'creature-jobs-lot04.json'
if target.exists():raise SystemExit('Preserve existing selected lot04 jobs and corrections')
records={r['id']:r for r in json.loads((ROOT/'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']}
specs=[
 ('MON-081-0','Quadrupède cuirassé',4,'creatures-lot03/MON-103-0',True,
  'Exactly four short weight-bearing legs, the existing short pointed rear appendage, pink/red underside and rough olive-brown dorsal covering. Keep the original low broad mound, head/front opening and short clawed feet. Compact low four-foot walking supports. No black beak, new plates, horns or extra feet copied from the reference.'),
 ('MON-092-0','Quadrupède pâle',4,'creatures-lot02/MON-064-0',True,
  'Exactly four weight-bearing limbs. Preserve the pale high arched back, red underside/creases, broad dark front claws and shorter rear limbs. Preserve its original narrow/obscured head instead of inventing a large face. Low compact four-limb gait, stable back, distinct fore/hind support. No added dorsal plates, horns, tail or costume.'),
 ('MON-127-0','Esprit aux bras osseux',0,'creatures-lot03/MON-126-0',False,
  'NO walking feet or legs. Preserve the skeletal hooded figure, original ivory skull/face, dark charcoal robe, ivory bands, two existing skeletal arms/hands and its long divided ragged lower cloth. The trailing strips stay cloth, never boots. Small coherent gliding/cloth movement, with constant standing height, face and proportions. No new staff, weapon, glowing eyes or aura.'),
 ('MON-139-0','Créature au livre',0,'creatures-lot01/MON-141-0',True,
  'NO humanoid walking legs. Preserve this exact green-hooded pale-faced creature, TWO skeletal arms, brown/gold OPEN BOOK and all the existing pale coiled lower-body appendages under the robe. The coiled base creeps with tiny traveling contractions; do not turn coils into feet, remove coils or create extra limbs. Upper body stays level. The open book is RIGID, remains supported by the SAME anatomical holding hand through all14 poses, and stays on the viewer RIGHT in BOTH near-front views like INPUT1; the empty extended claw stays viewer LEFT. Never mirror the book across the torso, swap hands, duplicate the book or bend its spine/pages. INPUT2 guides subtle coiled-base motion ONLY: do not copy its worm head or replace the green hood, arms, book or body.'),
 ('MON-147-0','Créature ailée violette',2,'creatures-lot01/MON-144-0',True,
  'Exactly TWO hind walking legs, TWO arms and TWO broad wings. Preserve the purple flesh, pale violet wing membranes, original horns, dark torso/waist cloth and original source appendages. Do not add any tail or equipment absent from INPUT1. Compact grounded two-foot walk, no great split. Wings carried in the original wide spread with very slight joint motion, no flight and no change of skin/membrane color.'),
 ('MON-148-0','Créature ailée massive',2,'creatures-lot03/MON-146-0',True,
  'Exactly TWO hind walking legs, TWO arms and TWO wings. Preserve the massive brown body, large curved ivory horns, dark hair/beard around the head/chest, reddish dark wing membranes, dark waist cloth and lower legs. No golden or purple material from the reference, no tail, weapon or equipment absent from INPUT1. Heavy compact grounded alternating supports, restrained bob, wings remain carried spread; no flight or growing/shrinking body.')
]
jobs=[]
for number,(key,label,legs,guide,shadow,motion) in enumerate(specs,21):
 identity=DOC/'references'/f'{key}-identity-detail.png';native=DOC/'references'/f'{key}-native-current.png';guide=SRC/'accepted'/f'{guide}-whole-body-native.png'
 assert identity.exists() and native.exists() and guide.exists(),key
 prompt=f'''Use case: identity-preserve. Complete lateral game-creature animation for existing identity {key}.
INPUT1 is the exact canonical APPEARANCE. Preserve anatomy, head/face, supports, all original appendages, materials, colors, carried objects and proportions. Isolated colored fringe specks in the old export are not glow or anatomy.
INPUT2 is ONLY a compatible accepted WHOLE-FIGURE motion and layout guide. Do not copy its species, face, bulk, armor, color or objects. Use compact supports and the same matte native64-readable game-art finish.
SPECIFIC ANATOMY AND MOVEMENT: {motion}
OUTPUT EXACTLY FOURTEEN separate COMPLETE figures, exactly TWO ROWS of exactly SEVEN figures on a wide transparent atlas approximately1792x640. No extra rows, eighth column, labels or duplicates. Each whole figure must occupy at most75percent of its cell WIDTH AND HEIGHT, all using the SAME scale. Leave generous transparent margins around every horn, wing, claw, coil, book and tail, at least24px from the OUTER canvas edge. No neighboring figures may touch; no clipped appendages. Do not fill the cells.
TOP ROW: ALL face screenLEFT in near-front three quarters, idle then L0,L1,L2,L3,L4,L5. BOTTOM ROW: ALL face RIGHT, idle then R0,R1,R2,R3,R4,R5. Head, torso and feet/base face together. No back view, pure profile, head-only reversal or changing direction within a row. Preserve asymmetric object ownership from INPUT1.
Six motion poses form ONE full compact cycle, three phases on one grid cell and the next three on the following cell,400ms per cell. Real stable resting idle; no frozen raised foot at arrival. Use the actual source morphology, not a human gait for a legless form.
Draw EACH creature WHOLE and coherent. Constant apparent scale, mass, head size, anatomy, materials and object identity across all14 poses; low restrained bob, no wide spread, stretching, hopping or swelling. No grafted parts, procedural strips or detached limbs. Canonical source proportions stay unchanged despite empty cell space.
Genuine transparent alpha, clean dark matte game-art contours and readable shapes. No floor, ground shadow, text, grid, effects, particles, halo, colored fringe/specks, extra equipment or redesign.'''
 prompt_path=DOC/'prompts'/f'{key}-lot04-r01.txt';prompt_path.write_text(prompt,'utf8')
 jobs.append({'id':key,'review_id':f'WALK07-M{number:02}','kind':'creature','label':label,'record':records[key],
  'locomotor_legs':legs,'motion':motion,'reference':str(identity),'references':[str(identity),str(guide)],
  'source_destination':str(SRC/f'{key}-cycle-lot04-r01.png'),'source_columns':7,'source_rows':2,
  'selected_source_indices':list(range(14)),'shadow':shadow,'strict_source_outer_margin':True,
  'prompt_file':str(prompt_path.relative_to(ROOT)),'prompt':prompt})
target.write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
print(json.dumps({'review_ids':[j['review_id'] for j in jobs],'supports':[j['locomotor_legs'] for j in jobs],'runtime_integration':False}))
