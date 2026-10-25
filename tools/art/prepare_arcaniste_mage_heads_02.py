"""Revise Arcaniste heads via image_gen; preserve outfit pixels exactly."""
import json
import shutil
import sys
from PIL import Image,ImageDraw
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,core_box,font,sha,write_json,gallery
from prepare_arcaniste_channeler_wave_01 import boards,LABELS
from prepare_technomancien_no_robe_heads_02 import fixed_project
from prepare_traqueuse_sniper_wave_01 import reference_sprites

NAME='arcaniste-mage-heads-02'
RID='ARCANISTE-MAGE-HEADS02'
ART,DEST,DOCS=SOURCES/NAME,OUT/NAME,REVIEW/NAME
OLD=REVIEW/'arcaniste-channeler-wave-01/candidates.json'
HEAD_LABELS=['Tête nue','Bandeau','Capuche de mage','Capuche bordée','Diadème de mage']
POLYGON=[(29,0),(35,0),(38,3),(38,8),(35,10),(29,10),(25,8),(25,3)]
REQUEST="alors j'aime la tenue mais les têtes ne vont pas (avec le casque) il faut faire quelque chose de plus.. magicien de jeu vidéo tu sais."


def plan():
    for folder in (ART,DEST,DOCS):folder.mkdir(parents=True,exist_ok=True)
    assert not (DOCS/'prompts.json').exists(),'Plan exists; preserve prompt provenance.'
    old=json.loads(OLD.read_text('utf-8'))
    descriptions={
        3:'INTERMEDIATE: replace the current close-fitting mechanical coif with a clearly recognizable fantasy videogame wizard HOOD. A matte plum cloth hood, gently pointed peaked top, fitted close around the human face, one broad folded cloth rim. White fringe and pale face visible. No ceramic helmet shell, no ear discs. Simple pure cloth silhouette, barely wider than the original head.',
        4:'HEAVY: replace the current ivory military open helmet with an unmistakable battlemage CLOTH HOOD. Plum hood with a broad warm ivory cloth lining framing the open human face, and a low narrow copper brow band resting just above the eyebrows. The cloth hood has a modest tapered peaked top. No hard cap or ear discs; most of the exterior remains visibly soft plum cloth.',
        5:'VERY HEAVY: replace the closed scifi helmet with an unmistakable archmage hood and low ivory DIADEM. Matte plum hood with a modest tapered peaked top; a single broad warm ivory ceramic diadem traces the forehead and the temples, with a tiny low central upward point. Human pale face, eyes, nose and mouth completely exposed in a clear light facial mass. Ivory hood lining connects visually to the coat. No visor, faceplate, jaw plate, ear discs or full hard helmet shell. The diadem is solid and simple, no individual jewels or spikes.'}
    jobs=[]
    for e in old['entries']:
        if e['rank']<3:continue
        rank=e['rank'];source=ROOT/e['source'];size=Image.open(source).size
        prompt=f'''Use case: precise-object-edit.
Asset: one whole transparent character source, Arcaniste armor study {rank:02d} for Project RL, intended for an actual 64x64 sprite.
Input image 1 is the EDIT TARGET. Preserve the entire existing character below the jaw exactly, including every garment, plate, collar, copper neck connector, color, arm, hand, leg, boot, pose, framing and position. Change ONLY the headwear above the collar. Keep the same pale male face identity and short white hair where visible.
User feedback: the outfit is liked, but the heads look too much like science-fiction soldiers and must read as a videogame MAG WIZARD.
Head design: {descriptions[rank]}
Keep this mage's face readable at tiny size. Use broad quiet shapes, a light face against the plum hood, two or three strong value groups; no tiny embroidery, runes, lights, floating dots, dangling gadgets, cables, gemstone speckles or texture noise. Match the existing plum, ivory and restrained copper palette. A compact hood, not a huge head: same crown height and chin position as the source, normal head-to-body proportions, no extended neck. Keep the full head within approximately the original head area, with only a small lateral expansion of the hood.
Match the source's restrained game-illustration finish and light direction. No changes to the outfit or silhouette below the chin. Do not add staff, weapons, effects or backdrop. Full figure on a genuinely transparent RGBA background. Preserve the input canvas {size[0]}x{size[1]}, subject coordinates, body scale, feet and empty margins. Output the edited full image, not a head crop or a board.'''
        jobs.append({'rank':rank,'candidate_id':f'AR-MH02-P{rank:02d}-r01','tool':'built_in_image_gen',
            'prompt':prompt,'transparent_background':True,
            'references':[{'path':e['source'],'sha256':sha(source),'role':'edit_target; whole outfit and framing locked, headwear only changes'}],
            'generated_file':None})
    write_json(DOCS/'prompts.json',{'review_id':RID,'date':'2026-10-22','user_request':REQUEST,
        'mode':'built_in_image_gen','unchanged_ranks':[1,2],'jobs':jobs})
    paths=[]
    for root in (SOURCES,OUT,REVIEW):
        paths.extend(p for p in root.rglob('*.png') if NAME not in p.parts)
    paths.extend(REVIEW.rglob('acceptance.json'))
    write_json(DOCS/'prior-images.json',{p.relative_to(ROOT).as_posix():sha(p) for p in paths})
    print(json.dumps({'jobs':3,'protected_files':len(paths)}))


def main():
    if (REVIEW/'arcaniste-same-face-03/candidates.json').exists():raise SystemExit('Later identity review exists; do not downgrade live preview.')
    assert not (DOCS/'acceptance.json').exists(),'Accepted review is frozen.'
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    old=json.loads(OLD.read_text('utf-8'))
    jobs={j['rank']:j for j in json.loads((DOCS/'prompts.json').read_text('utf-8'))['jobs']}
    mask=Image.new('L',(64,64));ImageDraw.Draw(mask).polygon(POLYGON,fill=255)
    sprites=[];heads=[];bodies=[];originals=[];entries=[]
    for e in old['entries']:
        rank=e['rank'];cid=f'AR-MH02-P{rank:02d}-r01'
        source=Image.open(ROOT/e['source']).convert('RGBA')
        original=Image.open(ROOT/e['native']).convert('RGBA')
        native=original.copy();final_source=source.copy()
        source_mask=Image.new('L',source.size)
        if rank>=3:
            job=jobs[rank];generated=Image.open(ROOT/job['generated_file']).convert('RGBA')
            assert generated.size==source.size,(rank,generated.size,source.size)
            projected,offset,size,frame=fixed_project(source,source,e['framing'])
            assert projected.tobytes()==original.tobytes()
            edited,_,_,_=fixed_project(generated,source,e['framing'])
            native.paste(edited,(0,0),mask)
            dx,dy=offset
            local_mask=mask.crop((dx,dy,dx+size[0],dy+size[1]))
            source_mask.paste(local_mask.resize((frame[2]-frame[0],frame[3]-frame[1]),Image.Resampling.NEAREST),(frame[0],frame[1]))
            final_source.paste(generated,(0,0),source_mask)
        changed_outside=sum(original.getpixel((x,y))!=native.getpixel((x,y))
            for y in range(64) for x in range(64) if not mask.getpixel((x,y)))
        assert changed_outside==0
        assert native.crop((0,11,64,64)).tobytes()==original.crop((0,11,64,64)).tobytes()
        assert core_box(native)[3]==62
        source_path=ART/f'{cid}-source.png';final_source.save(source_path)
        source_mask_path=ART/f'{cid}-head-mask-source.png';source_mask.save(source_mask_path)
        preserved=final_source.copy();preserved.paste(source,(0,0),source_mask)
        assert preserved.tobytes()==source.tobytes()
        native_path=ART/f'{cid}-native64.png';native.save(native_path);shutil.copyfile(native_path,DEST/native_path.name)
        head=Image.new('RGBA',(64,64));head.paste(native,(0,0),mask)
        body=native.copy();body.paste((0,0,0,0),(0,0,64,64),mask)
        rebuilt=body.copy();rebuilt.alpha_composite(head);assert rebuilt.tobytes()==native.tobytes()
        layers={}
        for part,im in [('head',head),('body',body),('head-mask',mask)]:
            path=ART/f'{cid}-{part}-native64.png';im.save(path)
            layers[part]={'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)}
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,'label':LABELS[rank-1],
            'status':'awaiting_head_visual_review','original_candidate_id':e['candidate_id'],
            'body_feedback':'liked_by_user; preserved_exactly; no runtime integration',
            'head_label':HEAD_LABELS[rank-1],'head_changed':rank>=3,
            'source':source_path.relative_to(ROOT).as_posix(),'source_sha256':sha(source_path),
            'source_head_mask':source_mask_path.relative_to(ROOT).as_posix(),
            'source_head_mask_sha256':sha(source_mask_path),
            'source_pixels_outside_head_mask_exact':True,
            'generated_head_edit':jobs[rank]['generated_file'] if rank>=3 else None,
            'generated_head_edit_sha256':sha(ROOT/jobs[rank]['generated_file']) if rank>=3 else None,
            'native':native_path.relative_to(ROOT).as_posix(),'native_sha256':sha(native_path),
            'framing':e['framing'],'native_dimensions':[64,64],'native_core_box':list(core_box(native)),
            'head_region_polygon':POLYGON,'layers':layers,'non_head_native_pixel_changes':changed_outside,
            'all_pixels_below_y11_exact':True,'head_body_recomposition_exact':True,
            'foot_anchor':[32,61],'rig_status':'study only; independent helmet replacement and weapon grips pending'})
        sprites.append(native);heads.append(head);bodies.append(body);originals.append(original)
    references=[];approvals=[]
    for folder,label in [('bastion-copper-wave-01','Bastion · validé'),('technomancien-no-robe-heads-02','Technomancien · validé'),('traqueuse-sniper-wave-01','Traqueuse · validée')]:
        actors,acceptance=reference_sprites(folder);references.append((label,actors));approvals.append(acceptance)
    boards(sprites,heads,bodies,references,destination=DEST,head_labels=HEAD_LABELS)
    comparison=Image.new('RGB',(400,272),'#243036');draw=ImageDraw.Draw(comparison)
    draw.text((10,8),'Arcaniste · têtes avant / après · ×4',font=font(15),fill='#efddbd')
    for row,(label,actors) in enumerate([('Avant',originals),('Après · 01 et 02 inchangés',sprites)]):
        y=36+row*116;draw.text((10,y),label,font=font(12),fill='#d8e2df')
        for i,actor in enumerate(actors):
            crop=actor.crop((24,0,40,16)).resize((64,64),Image.Resampling.NEAREST)
            comparison.paste(crop,(i*80+8,y+22),crop)
            draw.text((i*80+8,y+89),f'{i+1:02d}',font=font(10),fill='#d8e2df')
    comparison.save(DEST/'heads-before-after-x4.png')
    write_json(DOCS/'candidates.json',{'review_id':RID,'date':'2026-10-22','class_id':'core:arcaniste',
        'status':'awaiting_head_visual_review','native_dimensions':[64,64],
        'body_feedback':'liked; no outfit pixels changed','source_user_request':REQUEST,
        'reference_acceptances':approvals,'runtime_integration':False,'entries':entries})
    section=f'''<section id="arcaniste-channeler-wave-01" style="max-width:760px"><h2 id="{NAME}">Arcaniste · coiffes de mage</h2>
<p>Les tenues sont conservées. Les versions 03–05 portent maintenant des capuches prune,
avec une bordure ivoire puis un diadème de mage. Le visage reste découvert ; 01 et 02 ne changent pas.</p>
<div class="native-scroll"><img class="board" width="400" height="400" src="{NAME}/progression-native64.png" alt="Cinq Arcanistes à taille réelle 64 px, avec trois nouvelles coiffes de mage, sur trois fonds"></div>
<p class="note">01–05 : très légère à très lourde. Chaque case fait 64 × 64. Tous les pixels du corps et des vêtements sous la tête sont inchangés.</p>
<details><summary>Comparer les têtes avant / après au zoom entier ×4</summary><div class="native-scroll"><img class="board" width="400" height="272" src="{NAME}/heads-before-after-x4.png" alt="Anciennes têtes et nouvelles coiffes de mage au zoom entier quatre"></div></details>
<details><summary>Comparer les quatre classes à taille réelle</summary><div class="native-scroll"><img class="board" width="400" height="496" src="{NAME}/four-classes-native64.png" alt="Trois classes validées et Arcaniste aux coiffes de mage dans une même grille de 64 px"></div></details>
<details><summary>Examiner les coiffes de mage</summary><div class="native-scroll"><img class="board" width="400" height="224" src="{NAME}/head-progression-native-x4.png" alt="Cinq têtes natives puis au zoom entier quatre"></div></details>
<details><summary>Examiner les cinq silhouettes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="{NAME}/progression-zoom-x3.png" alt="Tenues conservées et nouvelles capuches de mage au zoom entier trois"></div></details>
<p class="note">Nouvelles coiffes à examiner. Elles appartiennent à la tête par défaut du torse.
La rue est un montage. Casques équipables indépendants, variantes, prises d’armes et intégration restent différés.
<a href="../../docs/reviews/class-readability-01/{NAME}/REVIEW.md">Dossier de la reprise</a> ·
<a href="../../docs/reviews/class-readability-01/{NAME}/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    (OUT/'arcaniste-channeler-wave-01/section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'));gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':RID,'candidate_count':5,'edited_heads':[3,4,5],'unchanged_heads':[1,2],
        'native_dimensions':[64,64],'non_head_native_pixel_changes':[e['non_head_native_pixel_changes'] for e in entries],
        'all_pixels_below_y11_exact':True,'head_body_recomposition_exact':True,'foot_baseline_y':61,
        'source_pixels_outside_head_masks_exact':True,
        'previous_images_and_acceptances_unchanged':len(prior),'approved_compositions_unchanged':15,
        'runtime_integration':False,'engine_execution':False,'ui_execution':'pending_browser_check'}
    write_json(DOCS/'checks.json',checks);print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':
    if '--plan' in sys.argv:plan()
    else:main()
