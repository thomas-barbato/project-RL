"""Prepare a separate, review-only anatomical lot after M01-M07 acceptance."""
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
SRC=ROOT/'assets/source-art/player-walk-review-07'
if (DOC/'creature-jobs-lot02.json').exists():
    raise SystemExit('Preserve selected corrections/deferred entries; inspect existing creature-jobs-lot02.json instead of overwriting it')
records={r['id']:r for r in json.loads((ROOT/'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']}
specs=[
    ('MON-031-0','Insecte',6,'MON-030-0',
     'Six walking legs, three attached pairs. Blue-green chitin shell and amber segmented legs. Alternate compact tripod supports: front and rear on one side with middle on the other. Keep the same six leg chains; no extra arms or pincers.'),
    ('MON-071-0','Arachnide',8,'MON-021-0',
     'Eight walking legs in four attached pairs, around the red/dark abdomen. Retain the large red-and-charcoal abdomen, small head and pale pointed feet. Compact alternating groups of four supports; rear legs may be partly occluded, never removed or fused into the abdomen.'),
    ('MON-083-0','Scorpion',8,'MON-021-0',
     'Exactly eight WALKING LEGS, plus TWO distinct large front PINCERS, plus ONE segmented arched TAIL and its dark stinger. Pincers and tail are not walking legs. Preserve the red-brown chitin, ivory joints and dark claw tips. The eight feet walk in compact staggered supports; pincers remain carried in front and the tail keeps its stable arch, with slight joint motion only.'),
    ('MON-064-0','Quadrupède',4,'MON-075-0',
     'Exactly four weight-bearing limbs. Preserve this low pale fleshy crawler, its dark red open back/rib cavity, large forelimb claws and short rear limbs. Front and hind supports alternate with a low stable body. No armor, rock shell, horns, tentacles or human upright posture from the motion reference.'),
    ('MON-029-0','Larve',0,None,
     'NO walking legs, wings or arms. Preserve the small pale cream/pink segmented larva with the reddish dark open mouth. Body stays LOW and HORIZONTAL against the floor, with a compact traveling contraction from rear segments toward the head. The rear stays a rounded larval end: never a foot, tail barb or upright coil. Preserve the small length and body volume.'),
    ('MON-125-0','Esprit',0,'MON-124-0',
     'NO walking legs. Preserve the exact narrow floating hooded figure and its long ivory/tan shredded cloth strips, empty dark face and thin hanging silhouette. The split lower strips remain cloth, never boots or feet. Slight coherent shroud drift and small folds during travel, no glowing aura or new effects.'),
    ('MON-145-0','Créature ailée',2,'MON-144-0',
     'Exactly TWO hind walking legs, TWO arms, TWO batlike wings, and ONE long dark tail. Preserve the lean muscular blue-gray demon, dark membranes, long horns and narrow face. Compact grounded alternating two-foot walk. Wings stay carried in the existing spread, with slight joint motion; tail follows subtly. No flight, no red coloration, no heavy stocky build from the reference.')
]
jobs=[]
for number,(key,label,legs,guide,motion) in enumerate(specs,8):
    identity=DOC/'references'/f'{key}-identity-detail.png'
    native=DOC/'references'/f'{key}-native-current.png'
    assert identity.exists() and native.exists(),key
    refs=[identity]
    if guide:refs.append(SRC/'accepted/creatures-lot01'/f'{guide}-whole-body-native.png')
    prompt=f'''Use case: identity-preserve.
Asset: complete lateral movement atlas for the EXISTING game creature {key}.
INPUT1 is the exact canonical APPEARANCE reference. Preserve its anatomy, head, appendages, materials, colors, distinctive shapes and existing equipment. Ignore isolated colored edge specks from the old game export: no new glowing dots.
'''+('INPUT2 shows an accepted native movement reference: TOP ROW seven complete LEFT-facing poses (idle then six walk); BOTTOM ROW seven RIGHT-facing poses. Use its compact cadence, coherent whole figures and matte native64-readable finish. It is NOT the target species, material, bulk or body shape. Do not copy its armor or anatomy.\n' if guide else '')+f'''
SPECIFIC ANATOMY AND MOVEMENT: {motion}
Output ONE regular FOUR-COLUMN FOUR-ROW atlas, sixteen COMPLETE figures on genuine transparent alpha,1536square. Each384square cell has generous transparent margins; keep heads, feet, wings, tail and pincers fully inside the cell. No neighboring fragment, floor, ground shadow, labels, text, halos, effects, background or grid.
TOP TWO ROWS, cells0..7, all near-front LEFT three quarters, head and entire body directed screenLEFT. BOTTOM TWO ROWS, cells8..15, all near-front RIGHT three quarters directedRIGHT. Show front and side together; no back view, no pure side profile, no head-only reversal. Do not flip direction during a half.
Exact row-major order LEFT: idleL,L0,L1,L2,L3,L4,L5,duplicate idleL. RIGHT: idleR,R0,R1,R2,R3,R4,R5,duplicate idleR. Six walking poses make one full compact cycle: three phases on a grid cell and three on the next cell,400ms per cell. Idle is a real stable resting pose.
Draw each WHOLE creature coherently. Constant body volume, apparent scale and anatomy across all16 figures; no shrinking, growing, extra limbs, swapped materials, wide splits or excessive bob. Keep original source proportions. Keep joints, supports and intentional gaps connected correctly. Do not graft limbs from INPUT2 or apply a human gait to a nonhuman body. Matte game-art shapes, readable contours, controlled detail. Genuine transparent cutout.'''
    prompt_file=DOC/'prompts'/f'{key}-lot02-r01.txt'
    prompt_file.write_text(prompt,'utf8')
    jobs.append({'id':key,'review_id':f'WALK07-M{number:02}','kind':'creature','label':label,
                 'record':records[key],'locomotor_legs':legs,'motion':motion,'reference':str(identity),
                 'references':[str(p) for p in refs],'source_destination':str(SRC/f'{key}-cycle-lot02-r01.png'),
                 'prompt_file':str(prompt_file.relative_to(ROOT)),'prompt':prompt})
(DOC/'creature-jobs-lot02.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
print(json.dumps({'lot':2,'review_ids':[j['review_id'] for j in jobs],'legs':[j['locomotor_legs'] for j in jobs],'runtime_integration':False}))
