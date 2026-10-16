"""Prepare only #59 and merged #66 revisions, preserving earlier proposals."""
from pathlib import Path
import json,hashlib,shutil

OUT=Path(__file__).resolve().parent;ROOT=OUT.parents[1]
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
history=OUT/'revisions/r2'
assert not history.exists(),'Revision snapshot already exists; do not overwrite.'
history.mkdir(parents=True)
for filename in ('catalog.json','manifest.json','gallery.html','README.md','validation.json','browser-validation-r2.json'):
 shutil.copyfile(OUT/filename,history/filename)
for folder in ('tiles','review'):
 shutil.copytree(OUT/folder,history/folder)
invariants={p.relative_to(ROOT).as_posix():sha(p) for p in (OUT/'tiles').glob('*.png')}
(OUT/'revisions/r3-invariants.json').write_text(json.dumps(invariants,indent=2),encoding='utf-8')
shutil.copyfile(Path('C:/Users/User/AppData/Local/Temp/codex-clipboard-4b850b76-9293-4cf1-bd2e-f126774d60cb.png'),OUT/'references/t1-user-reference.png')
for n in (59,66):
 shutil.copyfile(OUT/f'tiles/{n:03}-vivant.png',OUT/f'references/{n:03}-vivant-avant-t1.png')
 shutil.copyfile(OUT/f'tiles/{n:03}-cadavre.png',OUT/f'references/{n:03}-epave-avant-t1.png')
catalog=json.loads((OUT/'catalog.json').read_text(encoding='utf-8'))
retired=next(e for e in catalog if e['number']==60)
retired.update(status='retired-merged-into-066',merged_into=66,retirement_reason='User requests a single guardian based on #66, removes its shield and changes its colors; #60 no longer a separate model.')
(OUT/'retired-candidates.json').write_text(json.dumps([retired],ensure_ascii=False,indent=2),encoding='utf-8')
catalog=[e for e in catalog if e['number']!=60]
for e in catalog:
 if e['number'] not in (59,66):continue
 e['source']=f'sources/{e["number"]:03}-r{3 if e["number"]==59 else 2}.png'
 e['source_kind']='pair';e['revision']=3 if e['number']==59 else 2
 e['previous_living']=f'references/{e["number"]:03}-vivant-avant-t1.png'
 e['previous_corpse']=f'references/{e["number"]:03}-epave-avant-t1.png'
 e['review_round']='t1-and-merged-guardian'
 e['maximum']=[30,30];e['corpse_includes_blood']=False
 e['blood_layer_status']='none; dry mechanical wreck'
 if e['number']==59:
  e['review_description']='Silhouette inspirée du T‑1 : deux chenilles, corps central surélevé et deux armes latérales. Acier et rouge sombre.'
  e['description']='Industrial tracked combat robot inspired by user-supplied T-1 reference; steel/charcoal with broad dark red panels, no green/purple. Exactly two tracks and two lateral rotary gun pods.'
  e['equipment_status']='two rotary weapons fixed to lateral mechanical mounts, not human hands'
  e['design_reference']='references/t1-user-reference.png'
 else:
  e['review_description']='Forme du 66 conservée : torse dégagé, mains libres, épaules orange et plaques d’acier bleuté. Modèle unique issu de 60/66.'
  e['description']='Existing #66 guardian humanoid machine, shield removed from both living and wreck; body shape retained, orange and blue-gray steel palette.'
  e['equipment_status']='no held shield or weapon; two empty mechanical hands'
  e['merged_review_numbers']=[60];e['search_aliases']=['Rempart']
  e['gameplay_status']='visual proposal only; single intended guardian role retained, no tank behavior or stats implemented'
(OUT/'catalog.json').write_text(json.dumps(catalog,ensure_ascii=False,indent=2),encoding='utf-8')

common='Original industrial futuristic roguelike pixel sprite, readable like native 32x32 tiles. Very few large forms, square pixel edges, dark one-pixel outlines, large flat material zones with only 2-3 shade values per material, maximum24 opaque colors. No realistic texture, noise, tiny rivets, wire spaghetti, tiny greebles, gradients, glow or blur. Native footprint about30px wide by28-30px tall, do not shrink the torso because of equipment. Actual alpha transparency, no checkerboard, painted black backdrop, ground plane, shadow or text.'
layout='Wide transparent canvas with exactly TWO complete clearly separated subjects and generous transparent margins: LEFT operating robot; RIGHT the identical design and equipment collapsed horizontally into a low dry inert wreck, on its side, preserving its colors and major mechanical parts. Absolutely NO BLOOD, stains, substituted oil/coolant pools, smoke, sparks or spilled fluid in either panel. No extra robot, human, props or labels.'
jobs=[dict(number=59,source='sources/059-r3.png',source_kind='pair',tool='built-in imagegen',referenced_image_paths=[str(OUT/'references/t1-user-reference.png'),str(OUT/'references/joueur-reference.png')],transparent_background=True,prompt=f'Create the new #59 Artilleur robot. Reference 1 is the user-provided T-1 from Terminator; use its distinctive silhouette and construction as inspiration, simplify it greatly for the pixel game. Reference 2 is STYLE/SCALE ONLY: do not add that human, helmet or body to the robot. A compact tall tracked battle machine: exactly TWO broad continuous caterpillar track pods supporting a raised central angular armored steel torso, central body narrow at its mechanical swivel waist and wider across shoulders. Exactly TWO chunky armlike rigid shoulder mounts extend to either side; each carries ONE short clearly recognizable ROTARY GATLING weapon. Guns project forward/outward, firmly connected to their mounts, barrel axes straight, not hands holding pistols. The upper body is elevated over the track chassis like the supplied T-1 image, not just a turret perched on a small tank, and the paired guns create a strong broad horizontal silhouette. Two tiny square sensor modules are integrated on top, no humanoid head, face, helmet, legs or hands. Three-quarter near-front view clearly shows both track pods, angled central torso and BOTH gun pods. Base is stable, gun silhouette complete. Palette: medium and light gunmetal STEEL for central armor and tracks, charcoal joints and outlines; broad DARK RED/BURGUNDY armored shoulder/gun housings as the second material zone. At least one broad steel highlight plane contrasts with the dark red. Absolutely NO GREEN, VIOLET, PURPLE, lime, teal or neon paint. Emphasize five big readable masses: two track pods, central torso, two attached rotary-gun pods. No trademark logos, copied film markings, photographic wear or fine cables. {common} {layout}'),dict(number=66,source='sources/066-r2.png',source_kind='pair',tool='built-in imagegen',referenced_image_paths=[str(OUT/'sources/066.png')],transparent_background=True,prompt=f'Precise redesign of reference #66 Gardien. The reference is the EDIT TARGET, showing its active pose on the left and collapsed pose on the right. User likes this specific robot body shape. Retain that same upright near-front biped robot: same broad rounded-square shoulder silhouette, small square helmet-like sensor head with visor, same two short thick arms and two sturdy legs, same feet and overall body proportions. REMOVE the entire large front SHIELD from BOTH active and wreck subjects. The ivory/turquoise vertical object covering the chest and legs is a separate shield, NOT the torso: remove ALL of it, its border, turquoise band and handle. Reveal a coherent moderate-width armored mechanical torso, squared chest and simple segmented abdomen, connected pelvis and two legs behind it. Keep the original broad shoulders and stance; do not invent a new forklift chassis or redesign the head silhouette. Both arms are present with empty chunky mechanical hands, lowered comfortably beside the hips, natural joined shoulder/elbow/wrist alignment. No shield of ANY size, forearm shield, chest shield substitute, large handheld front panel, weapons, accessories or extra limbs. Change its armor colors completely: broad BURNT ORANGE shoulder plates and upper chest/thigh panels; broad BLUE-GRAY STEEL lower torso, forearms, shins and head casing, charcoal joints and outlines, a small pale cyan visor. Steel panels should be distinctly lighter than orange shadow and charcoal joints. NO purple/violet, green, beige/ivory armor or turquoise shield stripe from the old version. It must remain visibly the same #66 body revealed without its shield, now in orange and steel-blue colors. Right collapsed version is the SAME new orange/steel robot lying on its side, both empty hands visible or naturally occluded, absolutely no leftover shield rectangle on the ground. {common} {layout}')]
(OUT/'t1-guardian-prompts.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2),encoding='utf-8')
(OUT/'revision-current.json').write_text(json.dumps(dict(round='t1-and-merged-guardian',numbers=[59,66],retired=[60],guardian_count=1,integration_status='preview only; awaiting user opinion'),ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(dict(jobs=len(jobs),numbers=[59,66],retired=[60],preservation_snapshot=len(invariants)),ensure_ascii=False))
