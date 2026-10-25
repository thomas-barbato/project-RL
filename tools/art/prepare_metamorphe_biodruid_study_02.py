"""Package a modern druid revision and the user's two earlier outfit references."""
import json
import shutil
from PIL import Image, ImageDraw
from prepare_class_readability_review_01 import (
    ROOT, SOURCES, OUT, REVIEW, STREET, native_image, core_box, font, sha,
    write_json, gallery,
)

NAME = 'metamorphe-biodruid-study-02'
RID = 'METAMORPHE-BIODRUID-ST02'
CID = 'ME-BD02-P03-r01'
ART, DEST, DOCS = SOURCES / NAME, OUT / NAME, REVIEW / NAME
HEAD_POLYGON = [(28, 4), (36, 4), (38, 9), (37, 14), (34, 17),
                (30, 17), (27, 13), (26, 9)]


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
    source = ART / f'{CID}-source.png'
    actor, framing = native_image(source, 56)
    native = ART / f'{CID}-native64.png'
    actor.save(native)
    shutil.copyfile(native, DEST / native.name)
    mask = Image.new('L', (64, 64))
    ImageDraw.Draw(mask).polygon(HEAD_POLYGON, fill=255)
    head = Image.new('RGBA', (64, 64))
    head.paste(actor, (0, 0), mask)
    body = actor.copy()
    body.paste((0, 0, 0, 0), (0, 0, 64, 64), mask)
    rebuilt = body.copy()
    rebuilt.alpha_composite(head)
    assert rebuilt.tobytes() == actor.tobytes()
    layers = {}
    for part, im in [('head', head), ('body', body), ('head-mask', mask)]:
        p = ART / f'{CID}-{part}-native64.png'
        im.save(p)
        layers[part] = {'path': p.relative_to(ROOT).as_posix(), 'sha256': sha(p)}
    references = []
    reference_images = []
    for index, ref in enumerate(plan['references'][:2], 2):
        image, measure = native_image(ROOT / ref['path'], 56)
        p = ART / f'user-reference-{index:02d}-native64.png'
        image.save(p)
        reference_images.append(image)
        references.append({'number': f'{index:02d}', 'source': ref['path'],
            'source_sha256': ref['sha256'], 'native': p.relative_to(ROOT).as_posix(),
            'native_sha256': sha(p), 'framing': measure,
            'status': 'user_liked_reference; no new acceptance or integration inferred'})

    board = Image.new('RGB', (400, 300), '#152025')
    draw = ImageDraw.Draw(board)
    draw.text((12, 8), '01 · Métamorphe · druide futuriste', font=font(17), fill='#efddbd')
    draw.text((12, 35), 'Reprise proposée · même tenue sur trois fonds', font=font(12), fill='#bdcdc8')
    street = Image.open(STREET).convert('RGB').crop((80,160,160,232))
    for i, (label, color) in enumerate([('Sombre', '#343b3f'), ('Clair', '#858478'), ('Rue · montage', None)]):
        x = 20 + i*126
        if color:
            draw.rectangle((x, 63, x+79, 134), fill=color)
        else:
            board.paste(street, (x,63))
        board.paste(actor, (x+8,67), actor)
        draw.text((x,143), label, font=font(12), fill='#bdcdc8')
    draw.text((12, 174), '02–03 · tes références · même échelle', font=font(13), fill='#efddbd')
    for i, (label, image) in enumerate(zip(['02 · Blindée', '03 · Légère'], reference_images)):
        x = 58 + i*180
        draw.text((x,199), label, font=font(12), fill='#bdcdc8')
        draw.rectangle((x,219,x+79,290), fill='#343b3f')
        board.paste(image,(x+8,223),image)
    board.save(DEST / 'study-native64.png')
    zoom = Image.new('RGB', (400,300), '#243036')
    draw = ImageDraw.Draw(zoom)
    draw.text((12,8), '01 · reprise proposée au zoom entier ×4', font=font(15), fill='#efddbd')
    z = actor.resize((256,256),Image.Resampling.NEAREST)
    zoom.paste(z,(72,35),z)
    zoom.save(DEST / 'study-zoom-x4.png')
    audit = Image.new('RGB',(384,154),'#243036')
    draw = ImageDraw.Draw(audit)
    for i, (label,image) in enumerate([('Entier',actor),('Tête',head),('Corps',body)]):
        draw.text((i*128+8,5),label,font=font(12),fill='#efddbd')
        crop = image.crop((20,2,44,23)).resize((120,105),Image.Resampling.NEAREST)
        audit.paste(crop,(i*128+4,30),crop)
    audit.save(DEST / 'head-layers-audit-x5.png')
    plan.update(source=source.relative_to(ROOT).as_posix(),source_sha256=sha(source),
                generated_origin='exec-9316296a-e359-4e5c-ab6c-17d6bb9e7ab6.png')
    write_json(DOCS / 'prompts.json',plan)
    entry = {'number':'01','candidate_id':CID,'intended_progression_rank':3,
        'class_id':'core:metamorphe','presentation_gender':'female','form':'human_resting',
        'status':'awaiting_native64_visual_review','source':plan['source'],
        'source_sha256':sha(source),'native':native.relative_to(ROOT).as_posix(),
        'native_sha256':sha(native),'native_dimensions':[64,64],'framing':framing,
        'layers':layers,'head_ownership_polygon':HEAD_POLYGON,
        'head_body_recomposition_exact':True,'foot_anchor':[32,61],'head_anchor':[32,13],
        'rig_status':'study split; production helmet override and weapon grips pending'}
    write_json(DOCS / 'candidates.json',{'review_id':RID,'date':'2026-10-22',
        'status':'awaiting_native64_visual_review','entries':[entry],
        'user_liked_reference_outfits':references,'previous_study':'METAMORPHE-DRUID-ST01',
        'runtime_integration':False,'color_variants':'deferred_by_user'})
    section = f'''<section id="{NAME}" style="max-width:760px"><h2>Métamorphe · druide futuriste</h2>
<p>01 — Tissus techniques vert mousse, fourrure courte et plaques composites claires.
La tête retrouve un simple bandeau souple dans les cheveux.</p>
<div class="native-scroll"><img class="board" width="400" height="300" src="{NAME}/study-native64.png" alt="01 : nouvelle tenue druide futuriste sur trois fonds en 64 px ; 02 et 03 : les deux références fournies à même échelle"></div>
<p class="note">01 est la reprise proposée. 02–03 sont tes deux références, conservées telles quelles.
La rue est un montage. Chaque personnage occupe une case de 64 × 64.</p>
<div class="native-scroll"><img class="board" width="400" height="300" src="{NAME}/study-zoom-x4.png" alt="La reprise proposée 01 au zoom entier quatre"></div>
<details><summary>Examiner le découpage de la tête</summary><div class="native-scroll"><img class="board" width="384" height="154" src="{NAME}/head-layers-audit-x5.png" alt="Calques provisoires de tête et corps avec recomposition exacte"></div></details>
<p class="note">Étude à examiner avant la gamme légère à lourde ; les vingt tenues validées des autres classes sont conservées.
Casques indépendants, prises d’armes, variantes et intégration restent différés.
<a href="../../{source.relative_to(ROOT).as_posix()}">Dessin source</a> ·
<a href="../../docs/reviews/class-readability-01/{NAME}/prompts.json">Prompt exact</a></p></section>'''
    (DEST / 'section.html').write_text(section,encoding='utf-8')
    root = json.loads((REVIEW / 'candidates.json').read_text('utf-8'))
    root['current_visual_study'] = f'docs/reviews/class-readability-01/{NAME}/candidates.json'
    root['latest_steering'] = '22 October 2026: user likes the attached Metamorphe outfits but requests less medieval druid styling and a revised head. One modern druid outfit with composite plates and soft headband proposed; two user references preserved. New composition pending native64 review; no integration.'
    root['approval_scopes']['body_armor_variants'] = 'Twenty accepted other-class outfits preserved; one female Metamorphe modern-druid revision pending native64 review. User reference preferences recorded separately.'
    root['approval_scopes']['default_head_appearances'] = 'Twenty accepted heads preserved; Metamorphe soft cloth headband proposed, not yet accepted.'
    write_json(REVIEW / 'candidates.json',root)
    rule = json.loads((REVIEW / 'head-appearance-rule.json').read_text('utf-8'))
    rule['current_visual_study'] = root['current_visual_study']
    rule['visual_status'] = 'four_classes_accepted_metamorphe_modern_druid_study_pending'
    rule['metamorphe_default_head_appearances'] = 'short hair and soft plain cloth headband proposed; branches and broad forehead plate omitted'
    write_json(REVIEW / 'head-appearance-rule.json',rule)
    gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT / p)==h for p,h in prior.items())
    original = Image.open(source).convert('RGBA')
    alpha = original.getchannel('A')
    corners = [(0,0),(original.width-1,0),(0,original.height-1),(original.width-1,original.height-1)]
    checks = {'review_id':RID,'candidate_count':1,'user_reference_count':2,
        'native_dimensions':[64,64],'framing':framing,
        'source_alpha_extrema':alpha.getextrema(),'source_corner_alpha':[alpha.getpixel(p) for p in corners],
        'head_body_recomposition_exact':True,'previous_images_and_acceptances_unchanged':len(prior),
        'approved_other_class_outfits_unchanged':20,'runtime_integration':False,
        'engine_execution':False,'browser_verification':'pending'}
    assert checks['source_corner_alpha']==[0,0,0,0]
    assert core_box(actor)[3]==62
    write_json(DOCS / 'checks.json',checks)
    (DOCS / 'REVIEW.md').write_text('''# METAMORPHE-BIODRUID-ST02 — druide moins médiévale

Le joueur apprécie les références jointes mais demande une tenue moins médiévale
et une autre coiffe. Les trois pièces jointes sont conservées dans les références
de source. Les deux premières donnent les vêtements modernes, les composites,
le visage féminin, les cheveux courts et la corpulence. La troisième apporte
le mantelet et la lecture druide, à moderniser.

Le nouveau candidat 01, `ME-BD02-P03-r01`, combine un vêtement technique olive,
des plaques composites ivoire et une fourrure courte. Le bois sculpté, les bandes
de mollets et la boucle d’os laissent place à des protections et attaches modernes.
Un simple bandeau souple remplace la couronne ; aucune plaque ne barre le front.
Le visage reste humain et la nuque courte. Aucun pouvoir ni règle ne change.

La planche montre 01 sur trois fonds et les deux références 02–03 à même échelle,
sans modification de leurs sources. Cet intérêt du joueur ne constitue pas un
accord anticipé pour les nouveaux pixels ni pour leur intégration. La progression
de cinq armures, les casques indépendants et les prises d’armes restent à préparer.
Les vingt compositions validées des autres classes restent inchangées.

Sources et calques sont sous `assets/source-art/class-readability-01/`.
`prompts.json` conserve le prompt exact, les rôles et empreintes des références
et le mode image_gen intégré. Le découpage tête/corps est provisoire, avec
recomposition exacte. Les planches sont reproductibles sous `artifacts/`.
''',encoding='utf-8')
    print(json.dumps(checks,ensure_ascii=False))


if __name__ == '__main__':
    main()
