"""Prepare review-only concepts and record the user's existing bestiary choices."""
from pathlib import Path
from PIL import Image
import json, hashlib

OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[1]
OLD=ROOT/'artifacts/bestiaire-proposition-2026-10-05'
for folder in ('sources','tiles','references','review'):(OUT/folder).mkdir(parents=True,exist_ok=True)

subjects=[
 (9,'Pillard','Pillard à reprendre','Near-front human scavenger in a solid industrial teal helmet and amber respirator, broad amber work jacket over teal armored vest, navy work trousers and steel-toed boots. Two ordinary human arms and two legs, matching the current human player height. Hold ONE short heavy steel impact baton in the right hand, straight shaft, thick blunt end, absolutely no sword guard. Left hand empty. Thick utility belt, only one broad chest pocket. NO hood, cloak, robe, medieval helmet, leather jerkin or fantasy ornament.',(27,30),False),
 (56,'Éclaireur','Drones du joueur','Compact airborne inspection drone: large ivory oval central shell with cyan underside and a broad dark rectangular front sensor. Exactly TWO short side lift pods attached by thick cyan horizontal struts. No exposed thin rotors, no wings, no legs, no manipulator arms, no separate props. Distinct horizontal silhouette, floating body with open transparent space beneath.',(28,18),True),
 (57,'Mule','Drones du joueur','Low industrial supply robot, blocky burnt orange hull, broad ivory cargo block on top, exactly TWO thick dark caterpillar tracks visible left and right. Exactly ONE short articulated grabbing arm at front, its claw attached to its wrist. No cannon, no legs, no medieval cart. Front three-quarter view. Cargo block is a large readable shape, not many tiny crates.',(30,24),False),
 (58,'Traqueur','Drones du joueur','Mechanical dog-like quadruped with a wedge-shaped ivory head, broad purple back, charcoal joints and cyan lower legs. Exactly FOUR sturdy articulated legs and no other limbs. One large dark visor at front, no tail or antenna. Low forward-leaning broad silhouette, three-quarter view; distinguish head from torso with a clear short neck.',(30,24),False),
 (59,'Atelier','Drones du joueur','Compact upright industrial maintenance robot with a lime green barrel torso, broad violet lower chassis, ivory squarish sensor head. Exactly THREE short wheeled supports in a triangular arrangement, wheels large and clearly attached. Exactly TWO short articulated tool arms attached to shoulders: one steel pincer and one blunt dark welding nozzle. No hand-held weapon, no sparks, no smoke. Distinct three-legged machine silhouette.',(28,27),False),
 (60,'Rempart','Drones du joueur','Upright heavy humanoid security robot, broad cobalt torso and shoulders, ivory limbs and front armor, charcoal joints, small red horizontal sensor in a blunt square head. Exactly TWO arms and TWO short thick legs. A broad shield-like cobalt plate is integrated into the left forearm and a chunky fist on right arm, no detached shield or gun. Broad upper body, human-sized height, flat feet, no horns.',(28,30),False),
 (61,'Colosse biomécanique','Transformations du joueur','Player transformation, near-front full humanoid body with exactly TWO arms and TWO legs. Large ivory bony plates over crimson muscle, dark blue joints, very broad squared shoulders and short powerful forearms, small recessed helmet-like head. Thick fists empty; two short broad feet. Brutal biomechanical silhouette without a held weapon, no horns, no exposed organs, no floating particles. Keep body about same thirty-pixel height as human reference, just bulkier.',(30,30),False),
 (62,'Prédateur','Transformations du joueur','Player transformation, lean upright predatory humanoid, near-front slight three-quarter stance. Steel-blue elongated angular head and chest, broad copper-orange shoulder and thigh masses, charcoal waist. Exactly TWO bent claw arms and TWO digitigrade legs, connected anatomy. Three large hooked fingers on each empty hand, claws point outward. Distinct taller narrow predator silhouette, no extra tails, wings, carried weapons or spikes scattered everywhere. Thirty-pixel height.',(28,30),False),
 (63,'Cuirassé','Transformations du joueur','Player transformation into upright chitinous humanoid, near-front. Broad emerald shell plates on shoulders and chest, ivory abdominal segments, violet joints and boots. Exactly TWO arms, TWO legs and one broad armored insect-like head, tiny paired mandibles. Heavy folded forearms end in attached broad pincers, both empty. Human-like standing posture with wide feet. No additional insect legs, no wings, no carried blades or shields. Distinct layered carapace, not a recolor of biomechanical brute. Thirty-pixel height.',(28,30),False),
 (64,'Possédé','Transformations du joueur','Player transformation into near-front demonic humanoid, exactly TWO arms and TWO legs. Crimson upper torso and bent thick arms, broad violet hip and thigh masses, ivory face and exactly TWO short curved horns pointing outward. Angular shoulders with one large blunt protrusion per side. Strong empty claw hands, planted broad feet. Small recognizable face, no robe, staff, fire, weapons, armor crest or medieval clothing. Human-sized thirty-pixel height.',(28,30),False),
 (65,'Abysse','Transformations du joueur','Player transformation into upright lovecraftian humanoid, near-front. Deep violet head and torso framing a single broad ivory eye, bright teal thick forearms and lower legs, charcoal hollow oval aperture at chest center with TRUE transparency inside. Exactly TWO main arms and TWO planted legs. Exactly TWO short thick purple folded tendrils connected behind shoulders, no thin filaments, no more appendages. Empty hands. Compact readable thirty-pixel silhouette, no robe, equipment, glow or smoke.',(29,30),False),
]
catalog=[]
for number,name,family,description,maximum,flying in subjects:
 catalog.append(dict(number=number,review_id=f'visual-proposal:{number:03}',name=name,name_status='provisoire' if number!=9 else 'nom conservé',family=family,status='awaiting-art-review',usage='enemy' if number==9 else 'player-companion' if number<=60 else 'player-transformation',content_id=None,description=description,tile_size=32,maximum=list(maximum),flying=flying,source=f'sources/{number:03}.png',living=f'tiles/{number:03}-vivant.png',corpse=f'tiles/{number:03}-cadavre.png',inactive_label='Hors service' if 56<=number<=60 else 'Cadavre',corpse_includes_blood=True,gameplay_status='visual proposal only; stats, skills and behaviors not defined',equipment_status='not yet authored; empty transformation hands and integrated robot tools'))
(OUT/'catalog.json').write_text(json.dumps(catalog,ensure_ascii=False,indent=2),encoding='utf-8')

baseline={}
for directory in ('src','assets','content','docs','tools/art'):
 for p in (ROOT/directory).rglob('*'):
  if p.is_file() and '__pycache__' not in p.parts:baseline[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
(OUT/'baseline.json').write_text(json.dumps(baseline,indent=2),encoding='utf-8')

actors=Image.open(ROOT/'assets/sprites/cyberpunk/actors-32.png').convert('RGBA')
player=actors.crop((0,0,32,32));player.save(OUT/'references/joueur-natif.png')
player.resize((512,512),Image.Resampling.NEAREST).save(OUT/'references/joueur-reference.png')
for name,n in [('robot',23),('monstre',1),('pillard',9)]:
 im=Image.open(OLD/f'tiles/{n:03}-vivant.png')
 im.resize((512,512),Image.Resampling.NEAREST).save(OUT/f'references/{name}-reference.png')

previous=json.loads((OLD/'manifest.json').read_text(encoding='utf-8'))
decisions=[]
for entry in previous:
 accepted=entry['number']!=9
 decisions.append(dict(number=entry['number'],name=entry['name'],status='accepted-art' if accepted else 'rejected-art',scope=['living','corpse'],reason=None if accepted else 'Pillard trop médiéval',sources={k:dict(path=entry[k],sha256=hashlib.sha256((OLD/entry[k]).read_bytes()).hexdigest()) for k in ('living','corpse')}))
review=dict(date='2026-10-06',user_statement='je les acceptes tous sauf sauf le pillard car je le trouve trop medieval.',accepted=54,rejected=[9],decisions=decisions,scope='visual acceptance; no new species statistics, encounters or talent changes inferred')
(OLD/'review-decisions-2026-10-06.json').write_text(json.dumps(review,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(dict(candidates=[e['number'] for e in catalog],active_files_fingerprinted=len(baseline),previous_accepted=54),ensure_ascii=False))
