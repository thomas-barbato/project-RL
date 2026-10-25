"""First reviewed morphology cycles, with exact source IDs and support counts."""
import json
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07'
REF=DOC/'references';SRC=ROOT/'assets/source-art/player-walk-review-07'
baseline=json.loads((ROOT/'docs/reviews/player-walk-04/baseline-data.json').read_text('utf-8'))
records={e['id']:e for e in baseline['records']}
specs=[
('MON-021-0','Arachnide','Eight walking legs in four pairs, compact staggered support, alternating groups of four. Keep all eight legs attached around the orange armored body. No legs transform into arms.',8),
('MON-030-0','Insecte','Six walking legs in three pairs. Alternating tripod supports: front+rear on one side and middle on the opposite, then reverse. Three low compact phases per step, no hopping.',6),
('MON-075-0','Quadrupède','Exactly four weight-bearing legs. Slow compact four-legged walk with alternating diagonal supports, front and hind paws distinct. Heavy rocky body stays low and stable, no human knees.',4),
('MON-141-0','Ver','NO legs, no arms. Preserve the upright segmented worm and its continuous coiled lower body. Subtle whole-body traveling contraction over six phases; the base coil undulates softly and never becomes feet. Keep the same apparent body length and head shape.',0),
('MON-124-0','Esprit','NO walking legs. Preserve the exact hooded torn-cloth spirit, empty dark face and ragged floating lower silhouette. Very slight coherent drifting of the whole shroud during travel, tiny cloth motion, no invented feet or added glowing effects.',0),
('DRONE-GUNNER','Chenilles','NO legs. Exactly the source mechanical tracked chassis with its two treads, red/black armor and original weapon barrels. Only restrained continuous tread phase progression in six steps; chassis stays level. Do not bend shafts/barrels or turn treads into feet.',0),
('MON-144-0','Créature ailée','Exactly two hind legs, two arms and the same two wings as the source. This grounded walk alternates the two feet compactly, wings remain carried in their existing posture with slight movement at joints, no new flight or humanized costume.',2)]
jobs=[]
for number,(key,label,motion,legs) in enumerate(specs):
    record=records[key];source=REF/f'{key}-current.png'
    if not source.exists():
        site=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/sites/player-walk/dist')
        Image.open(site/record['image']).save(source)
    im=Image.open(source).convert('RGBA');box=im.getchannel('A').point(lambda a:255 if a>=48 else 0).getbbox()
    detail=im.crop(box);scale=512/max(detail.size);detail=detail.resize((round(detail.width*scale),round(detail.height*scale)),Image.Resampling.NEAREST)
    reference=REF/f'{key}-identity-detail.png';detail.save(reference)
    prompt=f'''Use case: identity-preserve. Asset: whole-creature lateral movement atlas for existing game monster {key}.
INPUT1 is the exact current creature design. Preserve its species, morphology, exact number of locomotor supports, head/face, appendages, materials, colors, armor, carried weapons and distinctive details. No new anatomy, equipment, attacks, effects or redesign. Match this existing game's matte detailed native64 finish.
Output one regular FOUR-COLUMN FOUR-ROW transparent atlas, sixteen COMPLETE separate creatures,1536square. Every384square cell has generous transparent margins, roughly maxbodyheight280/maxbodywidth280, sole/base baseline336. No cropped heads, legs, wings, tails or neighbor fragments. No labels, floor, background, grid, shadow or particles.
TOP TWO ROWS (cells0..7) ALL front-left three quarters directed screenLEFT. BOTTOM TWO ROWS (cells8..15) ALL front-right directedRIGHT. Near-front viewpoint like the input, showing the front and side together, never pure profile or back. Head and whole body/chassis face the same direction.
Exact reading order LEFT cells0..7: idleL, movementL0, L1, L2, L3, L4, L5, duplicate idleL. RIGHT cells8..15: idleR, movementR0,R1,R2,R3,R4,R5,duplicate idleR. The six movement poses form one coherent full cycle: first three accompany one grid cell, next three the next cell. Both idle poses are truly stable at rest; no unsupported lifted limb frozen at arrival.
SPECIFIC ANATOMY AND MOVEMENT: {motion}
Draw the ENTIRE actual creature in every pose. Maintain its body volume and apparent size across all sixteen. Small measured motion suitable for400ms per cell, no wide spread, exaggerated bounce or large pose changes. Preserve all anatomical joints and connections; intentional cloth holes/air between legs stay transparent. Do not merge, remove or invent legs across frames. Do not impose a humanoid gait on this creature. Exact source identity and materials in every frame. True transparency.'''
    jobs.append({'id':key,'kind':'creature','label':label,'record':record,'locomotor_legs':legs,'motion':motion,
        'reference':str(reference),'source_destination':str(SRC/f'{key}-cycle-r01.png'),'prompt':prompt})
    (DOC/'prompts'/f'{key}-cycle-r01.txt').write_text(prompt,'utf-8')
(DOC/'creature-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf-8')
print(json.dumps({'first_morphology_cycles':len(jobs),'supports':[j['locomotor_legs'] for j in jobs],
    'remaining_catalogue_records_still_pending':len([e for e in baseline['records'] if e['category']!='Classes'])-len(jobs)}))
