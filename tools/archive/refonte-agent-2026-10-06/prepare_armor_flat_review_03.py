"""Record explicit visual acceptance and prepare four pending catalogue edits."""
from pathlib import Path
import json, hashlib, shutil

BASE=Path(__file__).resolve().parent
ROOT=BASE.parents[1]
OLD=BASE/'armor-flat-review-01'
ACCEPTED=BASE/'armor-flat-review-02'
OUT=BASE/'armor-flat-review-03'
for d in ('sources','audit','palettes','previous','prompts'):(OUT/d).mkdir(parents=True,exist_ok=True)

def dump(path,data):path.write_text(json.dumps(data,ensure_ascii=False,indent=2),'utf-8')
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()

# Freeze the exact reviewed drawings, palettes and all 16 equipment views.
data=json.loads((ACCEPTED/'review.json').read_text('utf-8'))
ids=[f'AR-N{i:02}' for i in range(7,15)]
frozen={}
for e in data['entries']:
    if e['id'] not in ids:continue
    paths=[e['source'],f'palettes/{e["id"]}.json']
    paths += [p for view in e['views'].values() for p in view.values()]
    frozen.update({p:sha(ACCEPTED/p) for p in paths})
acceptance={
 'review_id':'ARMOR-FLAT02-ACCEPT01','accepted_on':'2026-10-10',
 'user_quote':'Je les valides et on continue :)',
 'accepted_visual_ids':ids,'palettes_per_outfit':3,'equipment_configurations':16,
 'scope':'Exact drawings, three displayed palettes and common equipment compositions of AR-N07 through AR-N14.',
 'names':'Review labels only; no new final object names are inferred.',
 'mechanics':'No new statistics or attribute requirements approved by this visual acceptance.',
 'active_game_integration':'Pending catalogue consolidation.',
 'frozen_files':frozen,
}
acceptance_path=ACCEPTED/'visual-acceptance-01.json'
if acceptance_path.exists():
    existing=json.loads(acceptance_path.read_text('utf-8'))
    assert existing['frozen_files']==frozen,'Accepted art changed'
else:dump(acceptance_path,acceptance)

catalogue=json.loads((OLD/'catalogue.json').read_text('utf-8'))
pending=['AR-08','AR-10','AR-11','AR-12']
entries=[]
for e in catalogue['entries']:
    if e['id'] not in pending:continue
    e=dict(e)
    e.update(category='revision',status='revision_pending_review',
             source=f'sources/{e["id"]}-poses.png',canonical=None,
             source_reference=f'artifacts/refonte-agent-2026-10-06/armor-flat-review-01/portraits/{e["id"]}-V0-00.png',
             variants=['Référence corrigée','Variante 2','Variante 3'])
    entries.append(e)
catalogue.update(review_id='ARMOR-FLAT03',status='review_only',entries=entries,
    accepted_reference='../armor-flat-review-02/visual-acceptance-01.json',
    withdrawn_ids=['AR-02','AR-04','AR-13'],active_game_content_changed=False)
dump(OUT/'catalogue.json',catalogue)

edits={
 'AR-08':('Change ONLY the helmet paint in ALL FOUR cells. Replace its bone-white half and its blue half with ONE coherent warm copper metal helmet with broad copper midtones, dark bronze cel shadows and pale metallic highlights. Preserve the exact helmet shape, black horizontal visor, black contour and shading geometry. Remove every blue patch from the entire helmet, including the upper edge. Both halves belong to the same copper material, with ordinary light/shadow only. Keep the ivory apron, blue sleeves, blue trousers, charcoal guards, gloves and boots exactly unchanged. No recoloring below the helmet.', ['blue','ivory','red','bone','olive','ivory']),
 'AR-10':('Change ONLY the colored clothing/armor palette in ALL FOUR cells. Replace the yellow-green coat, sleeves, face mask and boots with a muted dark petrol-blue textile palette, broad readable cel shadows and lighter blue-grey highlights. Replace the saturated cobalt helmet and chest plate with a coherent neutral brushed-steel palette, silver-grey midtones and bright silver edge highlights. Keep charcoal leather, trousers, visor, gloves, soles and all black ink contours unchanged. Absolutely no yellow-green or bright cobalt remains on these materials. Preserve the exact shape, seams, folds, body proportions and every pose.', ['workpetrol',None,'red',None,'olive',None]),
 'AR-11':('Change ONLY the colored suit/armor palette in ALL FOUR cells. Replace ALL red armor panels, helmet, hips, knees and shin plates with muted STEEL-BLUE cel paint, broad dark blue-grey shadows and clear lighter slate-blue highlights. Replace ALL cyan chest, upper arms, forearms, neck insets and tiny helmet insert with warm IVORY cel paint. Preserve charcoal joints, gloves, visor, soles and black ink contours unchanged. Absolutely no red or cyan remains. Preserve the exact shape, seams, shading geometry, proportions and every pose.', ['steelblue','ivory','olive','bone','red','steelblue']),
}
specs=[]
for ident,(edit,materials) in edits.items():
    prompt=('Use case: precise-object-edit. Asset: transparent four-pose full-body outfit sheet. '
      'Image 1 is the EDIT TARGET. Keep the same square sheet size and exactly the same equal 2 by 2 cells. '
      'Keep every character at exactly the same position, scale, silhouette and arm/hand orientation. '
      'Preserve the clear bande dessinee ink contours and simple cel shading. '+edit+
      ' True transparent background. No weapons, held objects, text, frames, floor, glow, new details or design changes.')
    specs.append(dict(id=ident,prompt=prompt,references=[str(OLD/f'sources/{ident}-poses.png')],output=str(OUT/f'sources/{ident}-poses.png')))
    next(e for e in entries if e['id']==ident)['materials']=materials

brief=('A futuristic equipment technician, entirely redesigned from the construction-worker clothing. '
 'A fitted graphite technical jumpsuit with a LONG dark petrol asymmetric split work coat ending above the knees, '
 'large off-white shoulder yoke and forearm guard shells, charcoal gloves and low protective boots. '
 'A compact raised flexible collar, a broad diagonal integral chest harness with two large steel rectangular ports, '
 'simple knee fabric panels, no heavy combat plating. Head: narrow flat smooth steel face mask with THREE short vertical '
 'dark sensor slots and visible charcoal soft hood around it, no hardhat, no brim, no T visor, no building helmet. '
 'Clear broad masses and modest futuristic industrial construction, distinct from armored soldiers and round bubble-visor starter. '
 'No yellow safety vest, no purple, no magenta, no visible wiring micro-details, no backpack, no dangling objects.')
prompt=('Use case: stylized-concept. Asset: one transparent outfit source sheet, exactly four full-body copies in an equal 2 by 2 grid. '
 'Image 1 is ONLY an anatomical pose and scale guide. Copy its joint positions and its cell layout exactly, NEVER its helmet, armor plates or clothes. '
 'Preserve human head height, leg length, shoulder centers, elbow centers, wrist centers, hand centers and floor line. '
 'Upper-left: independent empty clenched hands extended outward at waist height, forearms horizontal. '
 'Upper-right: rifle grip pose. Lower-left: horizontal heavy weapon grip at waist. '
 'Lower-right: two-handed diagonal melee grip, character left hand/viewer right upper grip turned upward/underhand, '
 'character right hand/viewer left lower grip turned downward/overhand. '
 'Draw NO weapons, rods, handles, shields or held objects. Dark charcoal rounded gloves with broad visible fingers, never black square hand blocks. '
 'STRICT frontal, no three-quarter rotation. Identical head, torso, lower clothing, legs and feet in all four cells; only arms change. '
 'Bande dessinee ink rendering: clear black contours, bold flat masses, two or three broad cel shadows per material, '
 'no photoreal texture or micro details. Match guide scale and margins in each cell. Clothing continuous behind moving arms. '
 'Plain true transparent background, no floor, shadow, halo, text, labels or frames. Outfit design: '+brief)
specs.append(dict(id='AR-12',prompt=prompt,references=[str(OLD/'references/common-four-poses.png')],output=str(OUT/'sources/AR-12-poses.png')))
next(e for e in entries if e['id']=='AR-12')['materials']=['workpetrol',None,'olive',None,'red',None]
dump(OUT/'catalogue.json',catalogue)
dump(OUT/'generation-specs.json',dict(mode='built-in imagegen',specs=specs))
for s in specs:(OUT/'prompts'/f'{s["id"]}.txt').write_text(s['prompt']+'\n','utf-8')

# Later steering: retain the accepted very-heavy drawing; repair only the
# incomplete source material assignment that left yellow shadow islands.
n14=dict(next(e for e in data['entries'] if e['id']=='AR-N14'))
n14.pop('views',None)
n14.update(category='revision',status='palette_artifact_correction_pending_review',
    source='sources/AR-N14-poses.png',
    source_reference='artifacts/refonte-agent-2026-10-06/armor-flat-review-02/portraits/AR-N14-V2-00.png')
entries.insert(0,n14)
shutil.copy2(ACCEPTED/n14['source'],OUT/n14['source'])
palette=json.loads((ACCEPTED/'palettes/AR-N14.json').read_text('utf-8'))
extra=[i for i,(r,g,b) in enumerate(palette['source_palette'])
       if r>=g and g>b+3 and r-b>5 and i not in palette['material_indices']['principal']]
for i in extra:
    for indices in palette['material_indices'].values():
        if i in indices:indices.remove(i)
    palette['material_indices']['principal'].append(i)
palette['material_indices']['principal'].sort()
palette['explicit_assignment_correction']=extra
palette['method']='Explicit indexed material palette; all yellow paint shadows belong to principal, neutral steel remains protected'
dump(OUT/'palettes/AR-N14.json',palette)
dump(OUT/'catalogue.json',catalogue)
dump(OUT/'palette-artifact-correction.json',dict(
 id='AR-N14',user_request='sur l’armure très lourde, artefacts jaunes un peu partout',
 cause='Yellow shadow colors incorrectly protected as brown/neutral colors',
 reassigned_source_palette_indices=extra,
 source_geometry_unchanged=True,reference_palette_unchanged=True,
 unchanged_accepted_source_sha256=sha(ACCEPTED/n14['source']),
 visual_acceptance_of_correction_pending=True))
print(json.dumps(dict(accepted_ids=ids,frozen_files=len(frozen),pending_ids=pending,output=str(OUT))))
