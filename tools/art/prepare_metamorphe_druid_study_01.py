"""Package one druid direction study; never edits game resources or approved art."""
import json
import shutil
from PIL import Image, ImageDraw
from prepare_class_readability_review_01 import (
    ROOT, SOURCES, OUT, REVIEW, STREET, native_image, core_box, font, sha,
    write_json, gallery,
)

NAME = 'metamorphe-druid-study-01'
RID = 'METAMORPHE-DRUID-ST01'
CID = 'ME-DR01-P03-r01'
ART, DEST, DOCS = SOURCES / NAME, OUT / NAME, REVIEW / NAME
HEAD_POLYGON = [(28, 5), (36, 5), (38, 10), (37, 15), (34, 18),
                (30, 17), (27, 13), (26, 9)]


def main():
    if (DOCS / 'acceptance.json').exists():
        raise SystemExit('Accepted study is frozen; create a sibling revision.')
    for folder in (ART, DEST, DOCS):
        folder.mkdir(parents=True, exist_ok=True)
    prior = json.loads((REVIEW / 'metamorphe-adaptive-wave-01/prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT / p) == h for p, h in prior.items())
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
    for part, image in [('head', head), ('body', body), ('head-mask', mask)]:
        p = ART / f'{CID}-{part}-native64.png'
        image.save(p)
        layers[part] = {'path': p.relative_to(ROOT).as_posix(), 'sha256': sha(p)}

    board = Image.new('RGB', (400, 186), '#152025')
    draw = ImageDraw.Draw(board)
    draw.text((12, 8), '01 · Métamorphe · piste druide', font=font(17), fill='#efddbd')
    draw.text((12, 35), 'Même tenue · trois fonds · cases de 64 × 64', font=font(12), fill='#bdcdc8')
    street = Image.open(STREET).convert('RGB').crop((80, 160, 160, 232))
    for i, (label, color) in enumerate([('Sombre', '#343b3f'), ('Clair', '#858478'), ('Rue · montage', None)]):
        x = 20 + i * 126
        if color:
            draw.rectangle((x, 65, x+79, 136), fill=color)
        else:
            board.paste(street, (x, 65))
        board.paste(actor, (x+8, 69), actor)
        draw.text((x, 145), label, font=font(12), fill='#bdcdc8')
    board.save(DEST / 'study-native64.png')
    zoom = Image.new('RGB', (400, 324), '#243036')
    draw = ImageDraw.Draw(zoom)
    draw.text((12, 8), '01 · même dessin au zoom entier ×4', font=font(16), fill='#efddbd')
    enlarged = actor.resize((256, 256), Image.Resampling.NEAREST)
    zoom.paste(enlarged, (72, 40), enlarged)
    draw.text((12, 302), 'Cuir · fourrure · étoffes vertes · bois', font=font(12), fill='#bdcdc8')
    zoom.save(DEST / 'study-zoom-x4.png')
    audit = Image.new('RGB', (384, 154), '#243036')
    draw = ImageDraw.Draw(audit)
    for i, (label, image) in enumerate([('Entier', actor), ('Tête', head), ('Corps', body)]):
        draw.text((i*128+8, 5), label, font=font(12), fill='#efddbd')
        crop = image.crop((20, 3, 44, 24)).resize((120, 105), Image.Resampling.NEAREST)
        audit.paste(crop, (i*128+4, 30), crop)
    audit.save(DEST / 'head-layers-audit-x5.png')

    plan = json.loads((DOCS / 'prompts.json').read_text('utf-8'))
    plan.update(source=source.relative_to(ROOT).as_posix(), source_sha256=sha(source),
                generated_origin='exec-de63ea82-46df-403c-bf30-82db57ddea62.png')
    write_json(DOCS / 'prompts.json', plan)
    entry = {'number': '01', 'candidate_id': CID, 'intended_progression_rank': 3,
             'class_id': 'core:metamorphe', 'class_name': 'Métamorphe',
             'presentation_gender': 'female', 'form': 'human_resting',
             'status': 'direction_study_awaiting_visual_review',
             'source': source.relative_to(ROOT).as_posix(), 'source_sha256': sha(source),
             'native': native.relative_to(ROOT).as_posix(), 'native_sha256': sha(native),
             'native_dimensions': [64, 64], 'framing': framing,
             'layers': layers, 'head_ownership_polygon': HEAD_POLYGON,
             'head_body_recomposition_exact': True, 'foot_anchor': [32, 61],
             'head_anchor': [32, 13],
             'rig_status': 'study split only; production helmet override and weapon grips pending'}
    write_json(DOCS / 'candidates.json', {'review_id': RID, 'date': '2026-10-22',
        'status': 'direction_study_awaiting_visual_review', 'candidate_count': 1,
        'supersedes_direction': 'METAMORPHE-ADAPTIVE-W01', 'entries': [entry],
        'runtime_integration': False, 'color_variants': 'deferred_by_user'})

    section = f'''<section id="{NAME}" style="max-width:760px"><h2>Métamorphe · une armure de druide</h2>
<p>01 — Une première tenue intermédiaire : cuir brun, fourrure aux épaules, étoffes vert mousse
et protections de bois. Un petit cercle de branches accompagne le visage humain.</p>
<div class="native-scroll"><img class="board" width="400" height="186" src="{NAME}/study-native64.png" alt="Candidat 01 : même Métamorphe druide en 64 px sur fond sombre, clair et sol de rue"></div>
<p class="note">Un seul modèle montré sur trois fonds, à taille réelle. La gamme légère à lourde viendra
après cette étude de direction. La rue est un montage sur une capture existante.</p>
<div class="native-scroll"><img class="board" width="400" height="324" src="{NAME}/study-zoom-x4.png" alt="Même tenue de druide au zoom entier quatre sans nouveau détail"></div>
<details><summary>Examiner le découpage de la tête</summary><div class="native-scroll"><img class="board" width="384" height="154" src="{NAME}/head-layers-audit-x5.png" alt="Prototype des couches tête et corps, recomposition exacte"></div></details>
<p class="note">Étude à examiner ; les vingt tenues validées des autres classes sont conservées.
Casques indépendants, prises d’armes, variantes et intégration restent différés.
<a href="../../{source.relative_to(ROOT).as_posix()}">Dessin source</a> ·
<a href="../../docs/reviews/class-readability-01/{NAME}/prompts.json">Prompt exact</a></p></section>'''
    (DEST / 'section.html').write_text(section, encoding='utf-8')
    root = json.loads((REVIEW / 'candidates.json').read_text('utf-8'))
    root['current_visual_study'] = f'docs/reviews/class-readability-01/{NAME}/candidates.json'
    root['latest_steering'] = '22 October 2026: user rejects the adaptive armor direction and requests a druid-like armor for the female Metamorphe. One medium outfit study prepared, awaiting native64 visual review. Twenty accepted other-class outfits unchanged; integration deferred.'
    root['approval_scopes']['body_armor_variants'] = 'Twenty outfits for four other classes accepted; one female Metamorphe druid direction study pending review.'
    root['approval_scopes']['default_head_appearances'] = 'Twenty accepted heads preserved; Metamorphe druid circlet and human face proposed, not yet accepted.'
    write_json(REVIEW / 'candidates.json', root)
    rule = json.loads((REVIEW / 'head-appearance-rule.json').read_text('utf-8'))
    rule['current_visual_study'] = root['current_visual_study']
    rule['visual_status'] = 'four_classes_accepted_metamorphe_druid_study_pending'
    rule['metamorphe_default_head_appearances'] = 'one proposed druid circlet and human face; direction study only'
    write_json(REVIEW / 'head-appearance-rule.json', rule)
    gallery(root['entries'], root['equipment_inventory'])
    assert all(sha(ROOT / p) == h for p, h in prior.items())
    original = Image.open(source).convert('RGBA')
    alpha = original.getchannel('A')
    checks = {'review_id': RID, 'candidate_count': 1, 'native_dimensions': [64, 64],
              'framing': framing, 'source_alpha_extrema': alpha.getextrema(),
              'source_corner_alpha': [alpha.getpixel(p) for p in [(0,0), (1023,0), (0,1535), (1023,1535)]],
              'head_body_recomposition_exact': True,
              'previous_images_and_acceptances_unchanged': len(prior),
              'approved_other_class_outfits_unchanged': 20, 'runtime_integration': False,
              'engine_execution': False, 'browser_verification': 'pending'}
    write_json(DOCS / 'checks.json', checks)
    (DOCS / 'REVIEW.md').write_text('''# METAMORPHE-DRUID-ST01 — première armure de druide

Le joueur refuse la direction de protections adaptatives et demande une armure
« plutot druide ». Le candidat 01, `ME-DR01-P03-r01`, propose une tenue intermédiaire
en cuir brun, fourrure, étoffes vert mousse et bois. Il reste à examiner en 64 × 64.
Ce n’est pas encore une gamme de cinq paliers ni une validation des nouveaux pixels.

La Métamorphe reste une femme humaine au repos. Mue prédatrice et Puissance
métamorphe motivent une silhouette robuste et mobile, sans nouveau pouvoir,
transformation, effet ou changement de règles. Le cercle de bois est porté.

Le torse fournit la tenue complète et sa coiffe par défaut. Le découpage tête/corps
est un prototype avec recomposition exacte ; le masque de remplacement par un casque
indépendant et les prises d’armes restent à préparer. L’intégration est différée.
Les vingt tenues validées des autres classes et leurs accords sont inchangés.

`prompts.json` conserve le prompt exact, la référence d’identité, le mode intégré
image_gen, la source choisie et son empreinte. `checks.json` conserve le cadrage,
les contrôles d’alpha, de recomposition et de préservation des fichiers précédents.
Les planches sont des aperçus reproductibles sous `artifacts/class-armor-review-01/`.
''', encoding='utf-8')
    print(json.dumps(checks, ensure_ascii=False))


if __name__ == '__main__':
    main()
