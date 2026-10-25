"""Package image_gen proposals for the second class wave, without runtime changes."""
import hashlib
import html
import json
import shutil

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import ROOT, SOURCES, OUT, REVIEW, STREET, core_box, font, native_image, sha, write_json

WAVE = 'technomancien-wave-02'
ART, DEST, DOCS = SOURCES/WAVE, OUT/WAVE, REVIEW/WAVE
SEAM = 14
COLORS = [('original', 'Originale'), ('red', 'Rouge'), ('blue', 'Bleue'), ('green', 'Verte')]
LABELS = ['Très légère', 'Légère', 'Interméd.', 'Lourde', 'Très lourde']


def pixels_hash(image):
    return hashlib.sha256(image.tobytes()).hexdigest()


def compose(source, head):
    framed, measurements = native_image(source, 48)
    body = Image.new('RGBA', (64, 64))
    body.paste(framed.crop((0, SEAM, 64, 64)), (0, SEAM))
    sprite = body.copy()
    sprite.paste(head.crop((0, 0, 64, SEAM)), (0, 0))
    return body, sprite, measurements


def matrix(rows, filename, background):
    board = Image.new('RGB', (400, 518), '#152025')
    draw = ImageDraw.Draw(board)
    draw.text((8, 9), 'Technomancien · cinq tenues · 64 × 64 natifs', font=font(14), fill='#efddbd')
    for column, (_, label) in enumerate(COLORS):
        draw.text((94+column*80, 38), label, font=font(11), fill='#d5dfdc')
    for index, sprites in enumerate(rows):
        y = 70+index*88
        draw.text((8, y+16), f'{index+1:02d}', font=font(13), fill='#efddbd')
        draw.text((8, y+37), LABELS[index], font=font(11), fill='#d5dfdc')
        draw.rectangle((80, y-2, 398, y+66), fill=background)
        for column, sprite in enumerate(sprites):
            board.paste(sprite, (88+column*80, y), sprite)
    board.save(DEST/filename)


def comparisons(rows, reference):
    board = Image.new('RGB', (480, 134), '#343b3f')
    draw = ImageDraw.Draw(board)
    draw.text((8, 7), 'Corps de référence et cinq tenues · taille native', font=font(15), fill='#efddbd')
    for index, sprite in enumerate([reference]+[row[0] for row in rows]):
        board.paste(sprite, (8+index*80, 34), sprite)
        label = 'Base CL02-r04' if index==0 else f'{index:02d} · {LABELS[index-1]}'
        draw.text((index*80+4, 105), label, font=font(10), fill='#d5dfdc')
    board.save(DEST/'base-and-five-native64.png')
    street = Image.open(STREET).convert('RGBA')
    patch = street.crop((80, 160, 400, 288))
    board = Image.new('RGB', (400, 186), '#152025')
    draw = ImageDraw.Draw(board)
    draw.text((9, 8), 'Même sol · montage de revue à taille native', font=font(14), fill='#efddbd')
    for index, row in enumerate(rows):
        x = index*80
        board.paste(patch.crop((40+index*30, 16, 120+index*30, 144)), (x, 36))
        board.paste(row[0], (x+8, 68), row[0])
        draw.text((x+32, 167), f'{index+1:02d}', font=font(12), fill='#d3dedc')
    board.save(DEST/'scene-native64.png')
    for index, row in enumerate(rows):
        board = Image.new('RGB', (832, 258), '#243036')
        draw = ImageDraw.Draw(board)
        draw.text((10, 8), f'{index+1:02d} · {LABELS[index]} · même tête · zoom entier ×3', font=font(16), fill='#efddbd')
        for column, (sprite, (_, label)) in enumerate(zip(row, COLORS)):
            enlarged = sprite.resize((192, 192), Image.Resampling.NEAREST)
            board.paste(enlarged, (8+column*208, 36), enlarged)
            draw.text((14+column*208, 232), label, font=font(12), fill='#d5dfdc')
        board.save(DEST/f'P{index+1:02d}-zoom-x3.png')


def main():
    if (DOCS/'rejection.json').exists():
        raise SystemExit('This series was rejected; prepare a separate identity revision.')
    if (DOCS/'acceptance.json').exists():
        raise SystemExit('Accepted wave is frozen; create a new revision.')
    for folder in (ART, DEST, DOCS):
        folder.mkdir(parents=True, exist_ok=True)
    plan = json.loads((DOCS/'prompts.json').read_text('utf-8'))
    assert len(plan['entries'])==5
    protected = {}
    bastion = json.loads((REVIEW/'bastion-wave-01/acceptance.json').read_text('utf-8'))
    for item in bastion['entries']:
        for key in ('native', 'body_layer'):
            protected[item[key]['path']] = item[key]['sha256']
        protected[item['source']] = item['source_sha256']
    for relative in ['content/core/classes/technomancien.json5', 'assets/sprites/art-finish-02/FIN-14.png']:
        protected[relative] = sha(ROOT/relative)
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    reference = Image.open(ROOT/plan['head_reference_native']).convert('RGBA')
    head = Image.new('RGBA', (64,64))
    head.paste(reference.crop((0,0,64,SEAM)), (0,0))
    head_path = ART/'TE-W02-common-head-native64.png'
    head.save(head_path)
    head_hash = pixels_hash(head.crop((0,0,64,SEAM)))
    rows, entries = [], []
    for model in plan['entries']:
        variants = {item['color']:item for item in model.get('variants', [])}
        row, views = [], []
        for color, label in COLORS:
            item = model if color=='original' else variants[color]
            candidate = item['candidate_id']
            source = ART/(candidate+'-body-source.png')
            shutil.copyfile(item['generated_file'], source)
            source_image = Image.open(source).convert('RGBA')
            corners = [(0,0),(source_image.width-1,0),(0,source_image.height-1),
                       (source_image.width-1,source_image.height-1)]
            assert all(source_image.getpixel(point)[3]==0 for point in corners)
            body, sprite, measurements = compose(source, head)
            assert sprite.size==(64,64)
            assert pixels_hash(sprite.crop((0,0,64,SEAM)))==head_hash
            bounds = core_box(sprite)
            assert bounds[0]>0 and bounds[2]<64 and bounds[3]<=62
            native, layer = DEST/(candidate+'-native64.png'), DEST/(candidate+'-body-native64.png')
            sprite.save(native)
            body.save(layer)
            views.append({'candidate_id':candidate, 'color':color, 'color_label':label,
                'status':'awaiting_visual_approval', 'source':source.relative_to(ROOT).as_posix(),
                'source_sha256':sha(source), 'native':native.relative_to(ROOT).as_posix(),
                'native_sha256':sha(native), 'body_layer':layer.relative_to(ROOT).as_posix(),
                'body_layer_sha256':sha(layer), 'measurements':measurements,
                'head_region_sha256':head_hash, 'assembled_core_box':list(bounds),
                'weapon_grips':'pending_separate_review', 'strict_material_masks':'pending_production'})
            row.append(sprite)
        original_alpha = row[0].getchannel('A').tobytes()
        for view, sprite in zip(views, row):
            alpha = sprite.getchannel('A').tobytes()
            intersection = sum(a>=128 and b>=128 for a,b in zip(original_alpha, alpha))
            union = sum(a>=128 or b>=128 for a,b in zip(original_alpha, alpha))
            view['solid_alpha_intersection_over_union_vs_original'] = intersection/union
        rows.append(row)
        entries.append({'number':model['number'], 'label':model['label'],
            'model_candidate':model['candidate_id'], 'item_id':None, 'views':views})
    matrix(rows, 'all-armors-colors-native64.png', '#343b3f')
    matrix(rows, 'all-armors-colors-light-native64.png', '#858478')
    matrix(rows, 'all-armors-colors-dim-native64.png', '#12181d')
    comparisons(rows, reference)
    class_definition = json.loads((ROOT/'content/core/classes/technomancien.json5').read_text('utf-8'))
    skill_catalogue = json.loads((ROOT/'assets/skills/agent/catalogue.json').read_text('utf-8'))
    skills = {item['id']:item['title'] for item in skill_catalogue['skills']}
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    write_json(DOCS/'candidates.json', {'review_id':'TECHNOMANCIEN-W02', 'date':'2026-10-21',
        'status':'awaiting_visual_approval', 'class_id':'core:technomancien',
        'starting_skills':[{'id':key,'title':skills[key]} for key in class_definition['agent_start']['skills']],
        'models':5, 'colored_variants':15, 'views':20, 'native_cell':[64,64],
        'head_reference':plan['head_reference'], 'head_reference_sha256':sha(ROOT/plan['head_reference']),
        'head_layer':head_path.relative_to(ROOT).as_posix(), 'head_layer_sha256':sha(head_path),
        'head_region':[0,0,64,SEAM], 'head_region_sha256':head_hash,
        'body_scope':'complete_outfit_below_head_including_hands_legs_boots',
        'attachment_points':{'head_center_x':32,'neck_seam_y':SEAM,'foot_baseline_y':61},
        'entries':entries, 'runtime_integration':False,
        'integration_gate':'All armor waves, helmets and weapon hand positions must be ready and reviewed first.',
        'checks':{'all_views_native64':True,'heads_identical_rgba':True,
            'sources_framed_without_distortion':True,'bastion_and_active_resources_unchanged':protected,
            'native_visual_review_required':True,'scene_is_montage_not_engine_validation':True,
            'generated_color_silhouettes_are_proposals_not_exact_material_mask_recolors':True}})
    zooms = ''.join(f'<details><summary>{i+1:02d} · {label} : zoom entier ×3</summary><div class="native-scroll"><img class="board" width="832" height="258" src="{WAVE}/P{i+1:02d}-zoom-x3.png" alt="Technomancien {label}, originale puis rouge, bleue et verte en zoom entier ×3"></div></details>' for i,label in enumerate(LABELS))
    section = f'''<section id="technomancien-wave-02"><h2>Vague 02 · Technomancien · à examiner</h2>
<p>Un humain presque entièrement robotique, spécialiste des drones et des algorithmes de combat. Cinq tenues complètes, de très légère à très lourde ; bras, mains, jambes et bottes évoluent avec le torse. Une seule tête et les mêmes augmentations sur les vingt vues.</p>
<h3 id="technomancien-colors">Cinq armures · originale, rouge, bleue et verte</h3>
<p><strong>Lecture en 64 × 64 réels.</strong> De grandes pièces de protection sur un châssis sombre. Les couleurs concernent les protections et le gilet de la très légère ; les articulations, le tissu sombre et les petits témoins ambre gardent leur palette.</p>
<div class="native-scroll"><img class="board" width="400" height="518" src="{WAVE}/all-armors-colors-native64.png" alt="Technomancien, cinq armures numérotées de très légère à très lourde et leurs trois couleurs, vingt vues natives 64 × 64"></div>
<details><summary>Comparer sur fond clair</summary><div class="native-scroll"><img class="board" width="400" height="518" src="{WAVE}/all-armors-colors-light-native64.png" alt="Les vingt propositions sur fond clair à taille native"></div></details>
<details><summary>Comparer en pénombre</summary><div class="native-scroll"><img class="board" width="400" height="518" src="{WAVE}/all-armors-colors-dim-native64.png" alt="Les vingt propositions sur fond sombre à taille native"></div></details>
{zooms}
<h3>Corps de référence et progression</h3>
<div class="native-scroll"><img class="board" width="480" height="134" src="{WAVE}/base-and-five-native64.png" alt="Le Technomancien CL02-r04 et les cinq tenues proposées, taille native"></div>
<div class="native-scroll"><img class="board" width="400" height="186" src="{WAVE}/scene-native64.png" alt="Les cinq tenues du Technomancien sur le sol de la rue, montage natif"></div>
<p class="note">Montage sur une capture acceptée ; le rendu avec l’éclairage moteur reste à vérifier. Ces tenues sont des propositions : les variantes peuvent présenter de petits écarts de contour avant les masques de production.</p>
<p>Cette vague attend ta validation. Le Bastion validé est conservé. L’intégration attendra toutes les armures, les casques et les bonnes prises d’armes ; cette planche garde les mains vides.</p>
<p><a href="../../docs/reviews/class-readability-01/{WAVE}/REVIEW.md">Périmètre et candidats</a> · <a href="../../docs/reviews/class-readability-01/{WAVE}/prompts.json">Prompts image_gen</a> · <a href="../../docs/reviews/class-readability-01/{WAVE}/candidates.json">Sources et contrôles</a></p></section>'''
    (DEST/'section.html').write_text(section, 'utf-8')
    print(json.dumps({'wave':'TECHNOMANCIEN-W02','models':5,'colored_variants':15,'native_views':20,'heads_identical':True,'bastion_unchanged':True}))


if __name__=='__main__':
    main()
