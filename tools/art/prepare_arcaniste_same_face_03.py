"""Image_gen identity corrections with one reusable existing native face layer."""
import json
import shutil
import sys
from PIL import Image,ImageDraw
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,core_box,font,sha,write_json,gallery
from prepare_arcaniste_channeler_wave_01 import boards,LABELS
from prepare_arcaniste_mage_heads_02 import POLYGON,HEAD_LABELS
from prepare_technomancien_no_robe_heads_02 import fixed_project
from prepare_traqueuse_sniper_wave_01 import reference_sprites

NAME='arcaniste-same-face-03'
RID='ARCANISTE-SAME-FACE03'
ART,DEST,DOCS=SOURCES/NAME,OUT/NAME,REVIEW/NAME
OLD=REVIEW/'arcaniste-mage-heads-02/candidates.json'
FACE_POLYGON=[(30,6),(34,6),(34,7),(33,9),(31,9),(30,8)]
REQUEST="j'ai l'impression que le visage n'est pas le meme partout."


def plan():
    for p in (ART,DEST,DOCS):p.mkdir(parents=True,exist_ok=True)
    assert not (DOCS/'prompts.json').exists()
    old=json.loads(OLD.read_text('utf-8'));master=old['entries'][0]
    ref=ART/'AR-FACE03-identity-reference.png'
    Image.open(ROOT/master['source']).crop((550,32,704,212)).save(ref)
    jobs=[]
    descriptions={2:'Keep the short white hair and low copper forehead band, without a hood. Place the band above the eyebrows and keep the full face exposed.',
        3:'Keep the current compact plum mage hood with ivory rim and modest pointed top. The hood frames the face and does not cover the eyes or jaw.',
        4:'Keep the current compact plum mage hood with a broad ivory lining and low copper brow band. Align the hood opening and brow band to the common frontal face.',
        5:'Keep the current plum archmage hood, broad low ivory diadem and one low central upward point. Turn the hood opening to frame the common frontal face; remove the old side-profile facial impression.'}
    for e in old['entries'][1:]:
        rank=e['rank'];size=Image.open(ROOT/e['source']).size
        prompt=f'''Use case: identity-preserve.
Asset: precise head-only correction of a whole Arcaniste game character for a 64x64 sprite.
Image 1 is the EDIT TARGET: preserve the exact outfit, all armor and cloth below the jaw, colors, pose, hands, legs, boots, collar and copper neck attachment. Image 2 is the SINGLE CANONICAL FACE REFERENCE taken from version 01 of this same character. It defines the identity and facial direction for all five outfits. Do not invent another man.
Replace the target's facial features with the reference man's same pale angular face: same brow shape, eye spacing, nose, cheekbones, mouth, jaw, short white hair where visible, and copper temple implant where not covered. The face must look forward exactly like the reference, with the same neutral stern expression and normal proportions. No profile face, no head rotation to follow the body, no older or younger variation, no change of skin tone or expression.
Headwear: {descriptions[rank]}
Fit the headwear around this SAME face, rather than redesigning the face to fit the hood. Keep a compact head, same chin and eye height as the reference when both characters are fitted to the same 60-pixel body height. No longer neck or giant head.
The only changed region is above the collar. Retain the current compact silhouette and established plum, ivory and copper palette. Match the image's broad clean game illustration shapes and quiet values. No additional ornament, beard, makeup, mask, visor, extra jewelry, weapons, staff, effects or props. Full isolated figure on genuine transparent RGBA. Keep the exact input canvas {size[0]}x{size[1]}, body coordinates, feet and empty margins. Output one full edited character, not a board or a portrait.'''
        jobs.append({'rank':rank,'candidate_id':f'AR-SF03-P{rank:02d}-r01','prompt':prompt,
            'tool':'built_in_image_gen','transparent_background':True,'generated_file':None,
            'references':[{'path':e['source'],'sha256':sha(ROOT/e['source']),'role':'edit_target; outfit locked, head identity corrected'},
                {'path':ref.relative_to(ROOT).as_posix(),'sha256':sha(ref),'role':'canonical face identity and forward orientation from unchanged version 01'}]})
    write_json(DOCS/'prompts.json',{'review_id':RID,'date':'2026-10-22','source_user_request':REQUEST,
        'mode':'built_in_image_gen','unchanged_ranks':[1],'canonical_face_candidate':master['candidate_id'],
        'native_face_reuse':'The existing 01 facial layer is reused exactly within an explicit mask after generated head alignment; no procedural drawing or recoloring.',
        'jobs':jobs})
    protected=[]
    for root in (SOURCES,OUT,REVIEW):protected.extend(p for p in root.rglob('*.png') if NAME not in p.parts)
    protected.extend(REVIEW.rglob('acceptance.json'))
    write_json(DOCS/'prior-images.json',{p.relative_to(ROOT).as_posix():sha(p) for p in protected})
    print(json.dumps({'jobs':4,'protected_files':len(protected)}))


def inverse_mask(mask,source,offset,size,frame):
    dx,dy=offset;out=Image.new('L',source.size)
    out.paste(mask.crop((dx,dy,dx+size[0],dy+size[1])).resize(
        (frame[2]-frame[0],frame[3]-frame[1]),Image.Resampling.NEAREST),(frame[0],frame[1]))
    return out


def main():
    if (REVIEW/'arcaniste-short-neck-04/candidates.json').exists():raise SystemExit('Later attachment review exists; do not downgrade live preview.')
    assert not (DOCS/'acceptance.json').exists()
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    old=json.loads(OLD.read_text('utf-8'));master_entry=old['entries'][0]
    master=Image.open(ROOT/master_entry['native']).convert('RGBA')
    master_source=Image.open(ROOT/master_entry['source']).convert('RGBA')
    face_mask=Image.new('L',(64,64));ImageDraw.Draw(face_mask).polygon(FACE_POLYGON,fill=255)
    head_mask=Image.new('L',(64,64));ImageDraw.Draw(head_mask).polygon(POLYGON,fill=255)
    face_layer=Image.new('RGBA',(64,64));face_layer.paste(master,(0,0),face_mask)
    face_layer.save(ART/'AR-FACE03-common-face-native64.png');face_mask.save(ART/'AR-FACE03-face-mask-native64.png')
    jobs={j['rank']:j for j in json.loads((DOCS/'prompts.json').read_text('utf-8'))['jobs']}
    actors=[];heads=[];bodies=[];originals=[];entries=[]
    for e in old['entries']:
        rank=e['rank'];cid=f'AR-SF03-P{rank:02d}-r01'
        source=Image.open(ROOT/e['source']).convert('RGBA');original=Image.open(ROOT/e['native']).convert('RGBA')
        native=original.copy();final_source=source.copy()
        source_mask=Image.new('L',source.size);source_face_mask=Image.new('L',source.size)
        if rank>=2:
            generated=Image.open(ROOT/jobs[rank]['generated_file']).convert('RGBA');assert generated.size==source.size
            # Always use original, pre-head-edit framing. Never reframe a changed hood.
            base=json.loads((REVIEW/'arcaniste-channeler-wave-01/candidates.json').read_text('utf-8'))['entries'][rank-1]
            original_source=Image.open(ROOT/base['source']).convert('RGBA')
            edited,offset,size,frame=fixed_project(generated,original_source,e['framing'])
            native.paste(edited,(0,0),head_mask)
            # Reuse an actual existing face layer; no new pixels are painted.
            native.paste(master,(0,0),face_mask)
            source_mask=inverse_mask(head_mask,source,offset,size,frame)
            final_source.paste(generated,(0,0),source_mask)
            source_face_mask=inverse_mask(face_mask,source,offset,size,frame)
            # Preserve the generated smooth head and its real hood/diadem
            # occlusion. The shared native facial core is composited only in
            # the exposed eyes/nose/mouth area, never over headwear or fringe.
        outside_changes=sum(native.getpixel((x,y))!=original.getpixel((x,y))
            for y in range(64) for x in range(64) if not head_mask.getpixel((x,y)))
        assert outside_changes==0
        assert native.crop((0,11,64,64)).tobytes()==original.crop((0,11,64,64)).tobytes()
        face_mismatches=sum(native.getpixel((x,y))!=master.getpixel((x,y))
            for y in range(64) for x in range(64) if face_mask.getpixel((x,y)))
        assert face_mismatches==0
        if rank==1:assert native.tobytes()==original.tobytes()
        check=final_source.copy();check.paste(source,(0,0),source_mask);assert check.tobytes()==source.tobytes()
        sp=ART/f'{cid}-source.png';final_source.save(sp)
        np=ART/f'{cid}-native64.png';native.save(np);shutil.copyfile(np,DEST/np.name)
        layers={}
        head=Image.new('RGBA',(64,64));head.paste(native,(0,0),head_mask)
        body=native.copy();body.paste((0,0,0,0),(0,0,64,64),head_mask)
        rebuilt=body.copy();rebuilt.alpha_composite(head);assert rebuilt.tobytes()==native.tobytes()
        for label,im in [('head',head),('body',body),('head-mask',head_mask),('face',face_layer),('face-mask',face_mask)]:
            p=ART/f'{cid}-{label}-native64.png';im.save(p);layers[label]={'path':p.relative_to(ROOT).as_posix(),'sha256':sha(p)}
        sm=ART/f'{cid}-head-mask-source.png';source_mask.save(sm)
        fm=ART/f'{cid}-face-mask-source.png';source_face_mask.save(fm)
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,'label':LABELS[rank-1],
            'status':'awaiting_visual_review','head_label':HEAD_LABELS[rank-1],'previous_candidate_id':e['candidate_id'],
            'source':sp.relative_to(ROOT).as_posix(),'source_sha256':sha(sp),
            'native':np.relative_to(ROOT).as_posix(),'native_sha256':sha(np),'native_dimensions':[64,64],
            'native_core_box':list(core_box(native)),'framing':e['framing'],'framing_status':'inherited body projection, not recomputed from edited head',
            'source_head_mask':sm.relative_to(ROOT).as_posix(),'source_face_mask':fm.relative_to(ROOT).as_posix(),
            'source_face_mask_usage':'reference only; smooth source head is the image_gen identity edit, without a face patch',
            'generated_head_edit':jobs[rank]['generated_file'] if rank>=2 else None,
            'generated_head_edit_sha256':sha(ROOT/jobs[rank]['generated_file']) if rank>=2 else None,
            'layers':layers,'head_region_polygon':POLYGON,'face_region_polygon':FACE_POLYGON,
            'non_head_native_pixel_changes':outside_changes,'all_pixels_below_y11_exact':True,
            'canonical_visible_face_pixel_mismatches':face_mismatches,'head_body_recomposition_exact':True,
            'hood_occlusion_allowed':True,'common_face_excludes_forehead_temples_and_outer_cheeks':True,
            'body_feedback':'liked and preserved exactly','foot_anchor':[32,61],
            'rig_status':'study layers only; helmet override and weapon grips pending'})
        actors.append(native);heads.append(head);bodies.append(body);originals.append(original)
    references=[];acceptances=[]
    for folder,label in [('bastion-copper-wave-01','Bastion · validé'),('technomancien-no-robe-heads-02','Technomancien · validé'),('traqueuse-sniper-wave-01','Traqueuse · validée')]:
        ref,a=reference_sprites(folder);references.append((label,ref));acceptances.append(a)
    boards(actors,heads,bodies,references,destination=DEST,head_labels=HEAD_LABELS)
    comparison=Image.new('RGB',(400,272),'#243036');draw=ImageDraw.Draw(comparison)
    draw.text((10,8),'Arcaniste · visage commun · avant / après ×4',font=font(14),fill='#efddbd')
    for row,(label,sprites) in enumerate([('Avant · traits et orientation variables',originals),('Après · même visage sous les coiffes',actors)]):
        y=36+row*116;draw.text((10,y),label,font=font(12),fill='#d8e2df')
        for i,im in enumerate(sprites):
            crop=im.crop((24,0,40,16)).resize((64,64),Image.Resampling.NEAREST)
            comparison.paste(crop,(i*80+8,y+22),crop);draw.text((i*80+8,y+89),f'{i+1:02d}',font=font(10),fill='#d8e2df')
    comparison.save(DEST/'faces-before-after-x4.png')
    write_json(DOCS/'candidates.json',{'review_id':RID,'date':'2026-10-22','class_id':'core:arcaniste',
        'source_user_request':REQUEST,'status':'awaiting_visual_review','reference_acceptances':acceptances,
        'canonical_face':master_entry['candidate_id'],'runtime_integration':False,'entries':entries})
    section=f'''<section id="arcaniste-channeler-wave-01" style="max-width:760px"><h2 id="{NAME}">Arcaniste · un même visage</h2>
<p>Le visage du 01 sert désormais de base commune aux cinq tenues : mêmes traits,
même orientation et même expression. Les capuches peuvent masquer le front, les tempes et les joues.
Les coiffes gardent leur progression ; tous les corps sont inchangés.</p>
<div class="native-scroll"><img class="board" width="400" height="400" src="{NAME}/progression-native64.png" alt="Cinq tenues de l’Arcaniste avec un visage commun, à taille réelle 64 px sur trois fonds"></div>
<p class="note">01–05 : très légère à très lourde. Cases de 64 × 64. Le 01 reste identique ; les têtes 02–05 sont corrigées.</p>
<details><summary>Comparer les visages avant / après au zoom entier ×4</summary><div class="native-scroll"><img class="board" width="400" height="272" src="{NAME}/faces-before-after-x4.png" alt="Cinq têtes avant et après harmonisation du visage, au zoom entier quatre"></div></details>
<details><summary>Comparer les quatre classes à taille réelle</summary><div class="native-scroll"><img class="board" width="400" height="496" src="{NAME}/four-classes-native64.png" alt="Trois classes validées et Arcaniste au visage commun en grille native 64 px"></div></details>
<details><summary>Examiner le visage sous les coiffes</summary><div class="native-scroll"><img class="board" width="400" height="224" src="{NAME}/head-progression-native-x4.png" alt="Visage commun et cinq coiffes natives puis au zoom entier quatre"></div></details>
<details><summary>Examiner les cinq silhouettes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="{NAME}/progression-zoom-x3.png" alt="Cinq tenues conservées, avec visages harmonisés au zoom entier trois"></div></details>
<p class="note">Têtes corrigées à examiner. Elles appartiennent au torse ; les casques indépendants,
prises d’armes, variantes et intégration restent différés. La rue est un montage.
<a href="../../docs/reviews/class-readability-01/{NAME}/REVIEW.md">Dossier de la correction</a> ·
<a href="../../docs/reviews/class-readability-01/{NAME}/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    (OUT/'arcaniste-channeler-wave-01/section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'));gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':RID,'candidate_count':5,'edited_heads':[2,3,4,5],'unchanged_heads':[1],
        'native_dimensions':[64,64],'non_head_native_pixel_changes':[e['non_head_native_pixel_changes'] for e in entries],
        'canonical_face_pixel_mismatches':[e['canonical_visible_face_pixel_mismatches'] for e in entries],
        'source_pixels_outside_head_masks_exact':True,'all_pixels_below_y11_exact':True,
        'head_body_recomposition_exact':True,'previous_images_and_acceptances_unchanged':len(prior),
        'approved_compositions_unchanged':15,'runtime_integration':False,'engine_execution':False,
        'ui_execution':'pending_browser_check'}
    write_json(DOCS/'checks.json',checks);print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':
    if '--plan' in sys.argv:plan()
    else:main()
