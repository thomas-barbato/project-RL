"""Frame five modern-druid outfits and all 25 class references, without integration."""
import json
import shutil
from PIL import Image, ImageDraw
from prepare_class_readability_review_01 import (
    ROOT, SOURCES, OUT, REVIEW, native_image, core_box, sha, write_json, gallery,
)
from prepare_traqueuse_sniper_wave_01 import reference_sprites
from prepare_arcaniste_channeler_wave_01 import boards, LABELS

NAME = 'metamorphe-biodruid-wave-03'
RID = 'METAMORPHE-BIODRUID-W03'
ART, DEST, DOCS = SOURCES / NAME, OUT / NAME, REVIEW / NAME
HEAD_LABELS = ['Tête nue', 'Bandeau léger', 'Bandeau souple', 'Tempes protégées', 'Menton protégé']
HEAD_POLYGONS = {
    1: [(29,4),(35,4),(38,8),(38,12),(35,16),(29,16),(26,12),(26,8)],
    2: [(28,4),(36,4),(38,9),(37,14),(34,17),(30,17),(27,13),(26,9)],
    4: [(28,4),(36,4),(39,9),(38,14),(35,17),(30,17),(26,13),(26,9)],
    5: [(28,4),(36,4),(39,8),(39,13),(35,18),(30,17),(26,13),(26,8)],
}


def main():
    if (DOCS / 'acceptance.json').exists():
        raise SystemExit('Accepted review is frozen; use a new revision.')
    for p in (ART, DEST, DOCS):
        p.mkdir(parents=True, exist_ok=True)
    prior = json.loads((DOCS / 'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT / p) == h for p, h in prior.items())
    plan = json.loads((DOCS / 'prompts.json').read_text('utf-8'))
    for ref in plan['references']:
        assert sha(ROOT / ref['path']) == ref['sha256']
    middle = json.loads((REVIEW / 'metamorphe-biodruid-study-02/candidates.json').read_text('utf-8'))['entries'][0]
    actors, heads, bodies, entries = [], [], [], []
    for job in sorted(plan['jobs'], key=lambda j: j['rank']):
        rank, cid = job['rank'], job['candidate_id']
        source = ROOT / job['source']
        assert sha(source) == job['source_sha256']
        framed, measure = native_image(source, 56)
        native = ART / f'{cid}-native64.png'
        if job.get('reuse_native'):
            shutil.copyfile(ROOT / job['reuse_native'], native)
            actor = Image.open(native).convert('RGBA')
            assert framed.tobytes() == actor.tobytes()
        else:
            actor = framed
            actor.save(native)
        shutil.copyfile(native, DEST / native.name)
        if rank == 3:
            polygon = middle['head_ownership_polygon']
            mask = Image.open(ROOT / middle['layers']['head-mask']['path']).convert('L')
            head = Image.open(ROOT / middle['layers']['head']['path']).convert('RGBA')
            body = Image.open(ROOT / middle['layers']['body']['path']).convert('RGBA')
        else:
            polygon = HEAD_POLYGONS[rank]
            mask = Image.new('L', (64,64))
            ImageDraw.Draw(mask).polygon(polygon, fill=255)
            head = Image.new('RGBA', (64,64))
            head.paste(actor, (0,0), mask)
            body = actor.copy()
            body.paste((0,0,0,0), (0,0,64,64), mask)
        rebuilt = body.copy()
        rebuilt.alpha_composite(head)
        assert rebuilt.tobytes() == actor.tobytes()
        layers = {}
        for part, image in [('head',head),('body',body),('head-mask',mask)]:
            p = ART / f'{cid}-{part}-native64.png'
            if rank == 3:
                shutil.copyfile(ROOT / middle['layers'][part]['path'],p)
            else:
                image.save(p)
            layers[part] = {'path':p.relative_to(ROOT).as_posix(),'sha256':sha(p)}
        original = Image.open(source).convert('RGBA')
        alpha = original.getchannel('A')
        corners = [(0,0),(original.width-1,0),(0,original.height-1),(original.width-1,original.height-1)]
        assert all(alpha.getpixel(p)==0 for p in corners)
        bounds = core_box(actor)
        assert actor.size==(64,64) and bounds[3]==62
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,
            'label':LABELS[rank-1],'class_id':'core:metamorphe','class_name':'Métamorphe',
            'presentation_gender':'female','form':'human_resting',
            'status':'awaiting_native64_visual_review','method':job['method'],
            'source':job['source'],'source_sha256':sha(source),
            'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),
            'native_dimensions':[64,64],'framing':measure,'native_core_box':list(bounds),
            'native_core_height':bounds[3]-bounds[1],'native_core_width':bounds[2]-bounds[0],
            'solid_silhouette_pixels':sum(actor.getchannel('A').histogram()[128:]),
            'layers':layers,'head_label':HEAD_LABELS[rank-1],
            'head_ownership_polygon':polygon,'head_body_recomposition_exact':True,
            'foot_anchor':[32,61],'head_anchor':[32,13],
            'head_is_default_appearance_of_torso':True,
            'rig_status':'study layers; independent helmet override and weapon grips pending'})
        actors.append(actor)
        heads.append(head)
        bodies.append(body)
    references, approvals, joint_entries = [], [], []
    for folder, label in [
        ('bastion-copper-wave-01','Bastion · validé'),
        ('technomancien-no-robe-heads-02','Technomancien · validé'),
        ('traqueuse-sniper-wave-01','Traqueuse · validée'),
        ('arcaniste-red-robe-wave-05','Arcaniste · validé'),
    ]:
        sprites, approval = reference_sprites(folder)
        references.append((label,sprites))
        approvals.append(approval)
        frozen = json.loads((ROOT / approval['path']).read_text('utf-8'))
        joint_entries.append({'class':label,'status':'accepted','acceptance':approval,
                              'candidate_ids':[e.get('candidate_id') for e in frozen['entries']]})
    boards(actors,heads,bodies,references,destination=DEST,head_labels=HEAD_LABELS,
           class_name='Métamorphe',descriptor='druide futuriste',
           review_label='Métamorphe · proposée',comparison_filename='five-classes-native64.png',
           head_crop_box=(24,4,40,20),audit_crop_box=(20,3,44,20))
    write_json(DOCS / 'candidates.json',{'review_id':RID,'date':'2026-10-22',
        'status':'awaiting_native64_visual_review','class_id':'core:metamorphe',
        'native_dimensions':[64,64],'native_height':56,'candidate_count':5,
        'reference_acceptances':approvals,'entries':entries,
        'runtime_integration':False,'color_variants':'deferred_by_user'})
    joint_entries.append({'class':'Métamorphe','status':'awaiting_native64_visual_review',
                          'candidate_ids':[e['candidate_id'] for e in entries]})
    write_json(DOCS / 'joint-native64-review.json',{'review_id':RID+'-ALL25',
        'native_dimensions':[64,64],'outfit_count':25,'accepted_outfit_count':20,
        'pending_outfit_count':5,'classes':joint_entries,
        'board':(DEST / 'five-classes-native64.png').relative_to(ROOT).as_posix(),
        'runtime_integration':False})
    section = f'''<section id="{NAME}" style="max-width:760px"><h2>Métamorphe · cinq armures de druide futuriste</h2>
<p>01–05 — Du vêtement très léger à la protection très lourde : textiles olive, fourrure courte,
plaques composites arrondies. Les bras, les jambes, les bottes et la tête évoluent avec la tenue.</p>
<div class="native-scroll"><img class="board" width="400" height="400" src="{NAME}/progression-native64.png" alt="Cinq Métamorphes du très léger au très lourd, en 64 px sur fond sombre, clair et rue"></div>
<p class="note">01 reprend exactement ta référence légère ; 03 conserve le modèle intermédiaire déjà montré.
02, 04 et 05 sont nouveaux. Les têtes restent compactes, avec le visage découvert.</p>
<details open><summary id="all-armors-native64-biodruid-03">Voir les 25 armures des cinq classes à taille réelle</summary>
<p>Vingt tenues validées conservées à l’identique ; cinq Métamorphes proposées sur la dernière ligne.</p>
<div class="native-scroll"><img class="board" width="400" height="608" src="{NAME}/five-classes-native64.png" alt="Les 25 armures des cinq classes : vingt références validées et cinq Métamorphes proposées, en grille native 64 px"></div></details>
<details><summary>Examiner les têtes : natif et zoom ×4</summary><div class="native-scroll"><img class="board" width="400" height="224" src="{NAME}/head-progression-native-x4.png" alt="Cinq têtes par défaut : cheveux, bandeaux, protections des tempes puis du menton"></div></details>
<details><summary>Examiner les cinq tenues au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="{NAME}/progression-zoom-x3.png" alt="Cinq tenues complètes de la Métamorphe au zoom entier trois"></div></details>
<p class="note">Gamme à examiner. La rue est un montage sur une capture existante.
La Métamorphe est montrée sous forme humaine, sans arme ni transformation.
Casques indépendants, prises d’armes, variantes et intégration restent différés.
<a href="../../docs/reviews/class-readability-01/{NAME}/REVIEW.md">Dossier de la gamme</a> ·
<a href="../../docs/reviews/class-readability-01/{NAME}/prompts.json">Sources et prompts</a></p></section>'''
    (DEST / 'section.html').write_text(section,encoding='utf-8')
    root = json.loads((REVIEW / 'candidates.json').read_text('utf-8'))
    root['current_visual_study'] = f'docs/reviews/class-readability-01/{NAME}/candidates.json'
    root['current_native64_overview'] = f'docs/reviews/class-readability-01/{NAME}/joint-native64-review.json'
    root['latest_steering'] = '22 October 2026: user authorizes continuing the modern-druid direction. Five female Metamorphe outfits prepared: very-light reference and medium study reused exactly, three new stages. All 25 class outfits shown at native64; 20 accepted references preserved, 5 Metamorphe compositions pending review. Integration deferred.'
    root['approval_scopes']['body_armor_variants'] = '20 other-class outfits accepted; five Metamorphe modern-druid outfits pending native64 review.'
    root['approval_scopes']['default_head_appearances'] = '20 accepted heads preserved; five Metamorphe default heads proposed, face visible and compact.'
    write_json(REVIEW / 'candidates.json',root)
    rule = json.loads((REVIEW / 'head-appearance-rule.json').read_text('utf-8'))
    rule['current_visual_study'] = root['current_visual_study']
    rule['joint_armor_review'] = root['current_native64_overview']
    rule['visual_status'] = 'four_classes_accepted_metamorphe_five_stage_range_pending'
    rule['metamorphe_default_head_appearances'] = 'five proposed heads: bare hair, cloth bands, compact temples and chin guards; no broad forehead plate'
    write_json(REVIEW / 'head-appearance-rule.json',rule)
    gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT / p)==h for p,h in prior.items())
    reuse_checks = {str(e['rank']):sha(ROOT / j['reuse_native'])==e['native_sha256']
                   for e,j in zip(entries,plan['jobs']) if j.get('reuse_native')}
    assert all(reuse_checks.values())
    checks = {'review_id':RID,'candidate_count':5,'new_source_count':3,'reused_native_count':2,
        'native_dimensions':[64,64],'native_core_heights':[e['native_core_height'] for e in entries],
        'native_widths':[e['native_core_width'] for e in entries],
        'solid_silhouette_pixels':[e['solid_silhouette_pixels'] for e in entries],
        'foot_baseline_y':61,'source_corners_transparent':True,'head_body_recomposition_exact':True,
        'exact_reuse_by_rank':reuse_checks,'joint_outfit_count':25,'accepted_other_class_outfits':20,
        'previous_images_and_acceptances_unchanged':len(prior),'runtime_integration':False,
        'engine_execution':False,'browser_verification':'pending'}
    write_json(DOCS / 'checks.json',checks)
    (DOCS / 'REVIEW.md').write_text('''# METAMORPHE-BIODRUID-W03 — cinq armures de druide futuriste

Le joueur répond « ok continue » à la reprise moins médiévale. Cette continuation
autorise la déclinaison de la direction, sans intégrer les nouveaux dessins.

01 reprend exactement la source et le sprite natif de sa référence très légère.
03 conserve exactement l’étude intermédiaire `ME-BD02-P03-r01`, tête et calques compris.
02, 04 et 05 sont trois nouveaux dessins image_gen : couverture et épaisseur
augmentent sur le corps, les bras, les gants, les jambes et les bottes.
La tête garde sa taille humaine : cheveux libres, bandeaux, protections des tempes,
puis un renfort bas du menton, sans plaque sur le front ni couronne de branches.

L’identité reste une femme musclée, en forme humaine au repos, avec tissus techniques
olive, cuir sombre, fourrure courte et composites ivoire aux contours naturels.
Aucun effet, pouvoir, chiffre d’équipement ou comportement de transformation ne change.

La revue montre les cinq tenues sur fonds sombre, clair et sol de rue, puis les
25 armures des cinq classes à taille native. Les vingt compositions acceptées
des autres classes sont lues depuis leurs références figées, avec empreintes vérifiées.
La rue est un montage sur une capture, pas une exécution du jeu.

Les calques tête/corps sont des découpages de revue à recomposition exacte.
Le remplacement par un casque indépendant et les prises d’armes attendent leurs
propres dessins et vérifications. Variantes et intégration restent différées.
La nouvelle gamme de la Métamorphe attend sa revue visuelle en 64 × 64.

Les sources et calques sont sous `assets/source-art/class-readability-01/`.
`prompts.json` conserve les prompts exacts, références, origines et réutilisations ;
le mode employé est l’outil image_gen intégré. `checks.json` donne le cadrage,
les réutilisations exactes et la préservation des images et accords précédents.
`joint-native64-review.json` distingue les vingt tenues acceptées des cinq proposées.
''',encoding='utf-8')
    print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':
    main()
