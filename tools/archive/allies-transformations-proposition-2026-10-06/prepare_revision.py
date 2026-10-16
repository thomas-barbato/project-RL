"""Prepare numbered machine revisions; never modify active game assets."""
from pathlib import Path
from PIL import Image
import json, hashlib, shutil

OUT=Path(__file__).resolve().parent; ROOT=OUT.parents[1]
OLD=ROOT/'artifacts/bestiaire-proposition-2026-10-05'
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
baseline=json.loads((OUT/'baseline.json').read_text(encoding='utf-8'))
changed=[p for p,digest in baseline.items() if sha(ROOT/p)!=digest]
assert not changed, changed
history=OUT/'revisions/r1'; history.mkdir(parents=True,exist_ok=True)
for filename in ('catalog.json','manifest.json','gallery.html','validation.json'):
 shutil.copyfile(OUT/filename,history/filename)
shutil.copytree(OUT/'tiles',history/'tiles',dirs_exist_ok=True)
shutil.copytree(OUT/'review',history/'review',dirs_exist_ok=True)
catalog=json.loads((OUT/'catalog.json').read_text(encoding='utf-8'))
previous={e['number']:e for e in json.loads((OUT/'manifest.json').read_text(encoding='utf-8'))}
old={e['number']:e for e in json.loads((OLD/'manifest.json').read_text(encoding='utf-8'))}
invariants={str(p.relative_to(ROOT)).replace('\\','/'):sha(p) for p in (OUT/'tiles').glob('*.png')}
for number in (21,22,23,24,42,43):
 e=old[number]
 for kind in ('living','corpse'):
  shutil.copyfile(OLD/e[kind],OUT/e[kind])
 invariants[str((OLD/e['living']).relative_to(ROOT)).replace('\\','/')]=sha(OLD/e['living'])
 catalog.append(dict(number=number,review_id=f'visual-proposal:{number:03}',name=e['name'],name_status='nom conservé',family='Machines ennemies — épaves',status='awaiting-corpse-art-review',usage='enemy-machine',content_id=e.get('content_id'),description='Existing mechanical enemy; living artwork preserved exactly. Dry collapsed wreck with no blood.',tile_size=32,maximum=[30,30],flying=False,source=f'sources/{number:03}-r2.png',source_kind='corpse-only',living=e['living'],corpse=e['corpse'],inactive_label='Épave',corpse_includes_blood=False,gameplay_status='visual revision only; no gameplay change',equipment_status='existing living artwork preserved',living_approval='accepted 2026-10-06; original living pixels unchanged',review_description='Dessin vivant conservé ; épave mécanique sans sang.'))

descriptions={
 57:'A simple driverless cargo crate mounted DIRECTLY on exactly TWO broad dark caterpillar tracks. NO separate robot body: the crate IS the entire body. Broad ivory rectangular cargo box with two dark vertical securing bands, orange reinforced lower edge, black charcoal tracks with three large wheel masses. Slight three-quarter front view. NO face, eyes, front sensor, head, arms, claw, antenna, gun, humanoid features or extra luggage. Minimal brutally industrial freight container on tracks.',
 59:'Low broad industrial assault drone on exactly TWO chunky black caterpillar tracks. A single large lime-green armored rectangular chassis, broad deep-violet turret, exactly TWO stout straight steel gun barrels firmly attached to the same turret, directed diagonally toward the viewer-right. Barrels project clearly beyond the chassis. Three-quarter front view showing both tracks. NO humanoid head, face, arms, legs, tool claws, tiny antenna, missiles, floating gun or muzzle flash. Clearly a compact tracked gun platform.',
 60:'Heavy upright industrial security robot, exactly TWO arms and TWO short thick piston-driven legs with broad flat work-machine feet. Cobalt squared torso, ivory bulky shoulder housing, visible charcoal elbow and hip joints, recessed small dark sensor head with one red sensor strip. Industrial loading-machine proportions: compact recessed head, massive chassis, broad squared shoulders, thick hydraulic forearms, no human waist or smooth skin. Its LEFT arm firmly HOLDS a large separate IVORY rectangular riot shield with a thick charcoal frame and ONE broad cobalt panel. Shield is visible on viewer-right in a three-quarter angle, from shoulder level to near ankle, covers roughly one-third of the full silhouette width; hand wraps a handle behind the inner edge, shield does not merge with torso. Right arm has a chunky empty mechanical fist. Shield is a clearly bounded substantial object rather than a skinny forearm plate. No heraldry, medieval shield, gun, spikes, wires or tiny rivets.',
 66:'New guardian tank robot, squat broad brutal industrial biped. Exactly TWO short thick mechanical legs with heavy flat feet, two thick attached arms, a small recessed amber sensor head and dark purple rectangular chassis. Holds ONE HUGE opaque rectangular pale-steel shield squarely in front of its body, shield nearly full body height and half the silhouette width, ivory front plate with ONE broad turquoise reinforcement bar, charcoal steel perimeter and squared beveled corners. Shield rests just above floor; thick hand attached to a rear handle visible at one edge, other arm thick stabilizing industrial claw behind shield. Broad exposed purple shoulder and dark hydraulic joint visibly separate from the light shield. Distinct from blue Rempart: this guardian is lower, wider and mostly concealed behind the front-facing massive portable barrier. No gun, medieval crest, sword, human anatomy, thin wires, microdetails or shield floating apart from robot.'
}
french={57:'Une caisse de transport sur deux chenilles, sans visage ni bras.',59:'Plateforme chenillée armée : tourelle violette, châssis vert et deux canons reliés.',60:'Robot industriel massif ; grand bouclier ivoire bordé de métal, séparé du châssis bleu.',66:'Gardien violet trapu derrière un grand bouclier frontal clair renforcé de turquoise.'}
for e in catalog:
 if e['usage']=='player-companion':
  e['corpse_includes_blood']=False; e['inactive_label']='Épave';e['revision']=2
  e['blood_layer_status']='none; dry mechanical wreck, no blood or replacement fluid pool'
  e['source']=f'sources/{e["number"]:03}-r2.png'
  e['source_kind']='pair' if e['number'] in descriptions else 'corpse-only'
  if e['number'] in descriptions:e['description']=descriptions[e['number']];e['review_description']=french[e['number']]
  if e['number']==59:e['name']='Artilleur';e['maximum']=[30,25]
  if e['number']==57:e['maximum']=[30,24]
catalog.append(dict(number=66,review_id='visual-proposal:066',name='Gardien',name_status='provisoire',family='Drones du joueur',status='awaiting-art-review',usage='player-companion',content_id=None,description=descriptions[66],tile_size=32,maximum=[30,30],flying=False,source='sources/066.png',source_kind='pair',living='tiles/066-vivant.png',corpse='tiles/066-cadavre.png',inactive_label='Épave',corpse_includes_blood=False,gameplay_status='visual proposal only; intended guardian role not implemented',equipment_status='shield and limbs drawn together with proper grip',review_description=french[66],revision=1))
catalog.sort(key=lambda e:e['number'])
(OUT/'catalog.json').write_text(json.dumps(catalog,ensure_ascii=False,indent=2),encoding='utf-8')
(OUT/'revisions/r2-invariants.json').write_text(json.dumps(invariants,indent=2),encoding='utf-8')

style='Original futuristic industrial roguelike pixel sprite. Match the chunky readable shapes, dark outlines and limited large color blocks of the supplied references. Intended NATIVE size 32x32 pixels: very few large details, hard nearest-neighbor pixel edges, maximum 24-color palette, two or three broad material colors, no gradients, realistic texture, scattered noise, glow or particles. Actual alpha transparency everywhere outside the subject; no painted backdrop, shadow, checkerboard, ground tile or text.'
jobs=[]
for e in catalog:
 n=e['number']
 if n not in (21,22,23,24,42,43,56,57,58,59,60,66):continue
 if e.get('source_kind')=='corpse-only':
  target=OUT/f'references/{n:03}-epave-avant-512.png'
  Image.open(OUT/e['corpse']).resize((512,512),Image.Resampling.NEAREST).save(target)
  prompt=f'Precise object edit of the supplied collapsed ROBOT sprite, reference is the EDIT TARGET. Remove ALL the red blood puddle, red droplets and red stain on the floor. The robot is entirely mechanical and DOES NOT BLEED. Do not replace blood with oil, red coolant, another colored puddle, sparks or smoke. Preserve the machine shell, colors, collapsed pose and outline as closely as possible. Repair only the small mechanical edge previously hidden by the blood if necessary. A dry, inert collapsed robot on fully TRANSPARENT alpha with nothing spilled beneath it. One single centered horizontal wreck only; do NOT draw a living robot, panel pair, standing robot, new equipment or floor. {style}'
  refs=[str(target)]
 else:
  refs=[str(OUT/'sources'/f'{n:03}.png')] if n!=66 else [str(OUT/'sources/060.png'),str(OUT/'references/joueur-reference.png')]
  prompt=f'Create the requested revised machine as an original production sprite pair. Existing reference shows the style and OLD design, which must be redesigned as specified. Identity #{n}: {e["description"]} {style} Layout: wide transparent canvas, exactly TWO clearly separated subjects. LEFT: complete active machine in its standing or tracked operating pose. RIGHT: the SAME machine and equipment collapsed onto its side, a low horizontal dry broken-down wreck; recognizable same colors, chassis, tracks or shield. Machine is entirely mechanical: absolutely NO blood, blood-colored puddle, any other fluid pool, gore, flesh, smoke, sparks or ground shadow in either panel. Generous transparent gap between panels; both subjects fully contained, complete contours, no fragments of adjacent subject. Keep the active body compact with scale matching the reference human; the shield may fill most of the robot silhouette but must remain a distinct solid shape. No labels or typography.'
 jobs.append(dict(number=n,source=e['source'],source_kind=e['source_kind'],prompt=prompt,referenced_image_paths=refs,transparent_background=True,tool='built-in imagegen'))
(OUT/'revision-prompts.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(dict(jobs=len(jobs),numbers=[e['number'] for e in jobs],unchanged_active_files_checked=len(baseline)),ensure_ascii=False))
