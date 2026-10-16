"""Prepare numbered armor proposals and a source-authored common pose sheet."""
from pathlib import Path
import json, hashlib
from PIL import Image, ImageDraw, ImageFont
import build_flat_rig_review_02 as rig

BASE=Path(__file__).resolve().parent; ROOT=BASE.parents[1]; OUT=BASE/'armor-flat-review-01'
OUT.mkdir(exist_ok=True)
for sub in ('sources','references','audit','palettes','native32','display64','normal'):(OUT/sub).mkdir(exist_ok=True)
old=json.loads((BASE/'comic-catalogue-review-03/catalogue.json').read_text('utf-8'))
outfits=[e for e in old['entries'] if e['kind']=='outfit']
pieces=[e for e in old['entries'] if e['kind'] in ('head','hands','feet')]
font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',18)
pose_names=['independent','rifle','heavy','pole']
sheet=Image.new('RGBA',(2508,2508))
for i,pose in enumerate(pose_names):sheet.alpha_composite(rig.source(1,pose),(i%2*1254,i//2*1254))
sheet.save(OUT/'references/common-four-poses.png')
for start in (0,9):
    group=outfits[start:start+9];board=Image.new('RGB',(1200,430*((len(group)+2)//3)),(14,21,29));d=ImageDraw.Draw(board)
    for j,e in enumerate(group):
        x=j%3*400;y=j//3*430;d.text((x+8,y+5),f"AR-{start+j+1:02} / {e['review_id']} · {e['name']}",font=font,fill='white')
        src=Image.open(ROOT/e['source_path']).convert('RGBA');bb=src.getchannel('A').getbbox();src=src.crop(bb)
        s=min(370/src.width,370/src.height);src=src.resize((round(src.width*s),round(src.height*s)),Image.Resampling.LANCZOS)
        board.paste(src,(x+(400-src.width)//2,y+40),src)
    board.save(OUT/'references'/f'current-outfits-{start//9+1}.png')

new=[
 ('N01','Mage · robe de canalisation','new_mage','A long deep-blue technical robe with a clear ivory vertical front panel, a rounded reinforced hood framing an angular dark mask, broad fabric folds, dark steel pauldrons and simple technological wrist cuffs. A readable restrained occult mark on the chest; no aura, staff or book.'),
 ('N02','Mage · manteau rituel','new_mage','A long crimson ritual coat over a black protective suit, open split lower panels, a high dark collar and enclosed narrow angular hood, light-grey reinforced shoulder pieces. Broad dark occult geometric bands woven into the robe, readable as black-magic attire, with practical cyberpunk connectors; no medieval wizard hat, no aura or held items.'),
 ('N03','Combat · armure de choc','new_combat','A heavy industrial assault armor with substantial blue-steel and grey protective plates and burnt-orange fabric inserts. Broad angular shoulder guards, a flat closed helmet with a horizontal dark visor, reinforced thighs and shins. Technological and brutal, no medieval armor, no oversized limbs or held items.'),
 ('N04','Combat · tenue d’assaut','new_combat','A mobile tactical combat outfit with a dark petrol-blue armored vest, sand-grey fabric sleeves and trousers, compact shoulder plates, a practical closed commando helmet, a wide visible protective chest panel, knee pads and substantial boots. Distinct from a heavy full plate suit; no guns, belt bullets or held items.'),
 ('N05','Métamorphe · combinaison organique','metamorphe_replacement','A flexible dark violet-black augmented-human suit with substantial muted turquoise membrane panels, visible bone-ivory organic rib shapes on the chest and collar, short angular shoulder shells with two small biological protrusions, a predatory closed mask under a low narrow hood. Human anatomy in untransformed state. The silhouette and broad organic materials suggest future metamorphosis rather than generic hard robot armor. Black articulated gloves and boots; no antenna helmet, no full animal head, tail, wings, aura or transformation.'),
 ('N06','Métamorphe · manteau de mutation','metamorphe_replacement','An occult mutation outfit in moss-green and bone-ivory, a short split organic mantle over a flexible charcoal bodysuit, broad leathery overlapping chest and shoulder scales, a high ribbed living collar around a closed dark facemask, a short pointed hood and ivory organic knee shells. All elements clearly blend augmented technology and biological occult mutation. Human untransformed body, not an armored robot or animal. Black articulated gloves and boots; no antenna helmet, huge horns, wings, tail, weapon, aura or transformation.')
]
entries=[]
for j,e in enumerate(outfits,1):
    item={'id':f'AR-{j:02}','old_review_id':e['review_id'],'content_id':e.get('content_id'),'name':e['name'],'kind':'outfit','category':'current','source_reference':e['source_path'],'source':f"sources/AR-{j:02}-poses.png",'canonical':{7:1,9:4}.get(j),'status':'current_model_front_adaptation_pending_review','variants':['Originale','Variante 2','Variante 3']}
    entries.append(item)
for number,name,cat,brief in new:
    entries.append({'id':f'AR-{number}','old_review_id':None,'content_id':None,'name':name,'kind':'outfit','category':cat,'source_reference':None,'source':f'sources/AR-{number}-poses.png','brief':brief,'status':'new_design_pending_review','replaces_candidate':'AR-03' if cat=='metamorphe_replacement' else None,'variants':['Originale','Variante 2','Variante 3']})
for j,e in enumerate(pieces,1):
    entries.append({'id':f'AR-P{j:02}','old_review_id':e['review_id'],'content_id':e.get('content_id'),'name':e['name'],'kind':e['kind'],'category':'current_piece','source_reference':e['source_path'],'source':e['source_path'],'status':'current_inventory_piece_no_body_change','variants':['Illustration d’inventaire']})
common=('Use case: stylized-concept. Asset: ONE outfit rig sheet, four poses of the SAME outfit, transparent background. '
 'Image 1 is the exact approved shared frontal pose template, in a strict 2 by 2 grid. Copy its adult human anatomy, head height, leg length, foot line, shoulder positions, elbow positions, wrist centers and clenched black-gloved grip centers in each cell. '
 'Upper-left: independent empty hands extended outward at waist height, forearms horizontal. Upper-right: rifle grip pose. Lower-left: waist-level horizontal heavy-weapon pose. Lower-right: diagonal two-handed melee pose, character left hand (viewer right upper grip) turned upward/underhand, character right hand (viewer left lower grip) turned downward/overhand. '
 'No actual weapons in any cell. Every outfit must work with every equipment configuration regardless of class. '
 'Strictly face-on, no three-quarter or profile torso or head, symmetrical legs in the same stable stance. Do not change the front of the outfit, helmet, head, torso, hips, legs or feet between cells: only the arms change. '
 'Bande dessinee rendering matching the pose template: bold clean black ink, strong readable flat color masses, two or three broad cel shades per material, no noisy texture or tiny details. '
 'Both arms remain anatomically continuous with distinct rounded charcoal gloves and readable fingers, never black rectangles. Sleeves use the outfit materials. '
 'Fit each complete figure in its equal grid cell, with the same scale and margins as the template, no overlap or clipping. Use a large square canvas. '
 'No pink or magenta, photorealism, floor, shadow, text, labels, frame lines, aura, decorative particles, weapon, shield, pet or separate floating body parts. True transparent background. ')
specs=[]
for e in entries:
    if e['kind']!='outfit' or e.get('canonical'):continue
    refs=[str(OUT/'references/common-four-poses.png')]
    if e['source_reference']:
        refs.append(str(ROOT/e['source_reference']))
        request='Image 2 is the identity and clothing reference for this existing outfit. Preserve its distinctive helmet, garments, protective shapes and broad original colors while adapting it to the template anatomy and poses. It is not a pose reference. Do not replace it with the ivory template outfit. This is '+e['name']+'.'
    else:request='Create the following distinct proposed outfit on the exact template rig: '+e['brief']
    specs.append({'id':e['id'],'prompt':common+request,'references':refs,'output':str(OUT/e['source'])})
active=json.loads((ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json').read_text('utf-8'))
protected={e['path']:hashlib.sha256((ROOT/e['path']).read_bytes()).hexdigest() for e in active['files']}
data={'review_id':'ARMOR-FLAT01','status':'review_only','entries':entries,'pose_names':pose_names,'protected_active_files':protected,'accepted_template':'BD-FLAT01 revision 2','rules':{'torso_defines_complete_outfit':True,'same_rig_for_all_outfits':True,'color_variants_preserve_geometry_stats_and_attachment_points':True,'stat_requirements_planned_values_not_defined':True,'deletion_requires_user_selected_ids':True},'new_names':'Review labels only; no player-facing names or mechanical effects are integrated.'}
(OUT/'catalogue.json').write_text(json.dumps(data,ensure_ascii=False,indent=2),'utf-8')
(OUT/'generation-specs.json').write_text(json.dumps({'mode':'built-in imagegen','specs':specs},ensure_ascii=False,indent=2),'utf-8')
print(json.dumps({'existing_outfits':len(outfits),'new_proposals':len(new),'inventory_pieces':len(pieces),'sheets_to_generate':len(specs)},ensure_ascii=False))
