"""Package all five armor models with their image_gen red/blue/green variants."""
import json
import shutil

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import ROOT, font, sha, write_json
from prepare_bastion_progression_01 import ART, DEST, DOCS, NECK_Y, compose_body, pixels_hash

COLORS = [('red', 'Rouge'), ('blue', 'Bleue'), ('green', 'Verte')]
LABELS = ['Très légère', 'Légère', 'Interméd.', 'Lourde', 'Très lourde']


def matrix(rows, filename, background):
    board = Image.new('RGB', (400, 518), '#152025')
    draw = ImageDraw.Draw(board)
    draw.text((8, 9), 'Cinq armures · trois couleurs · 64 × 64 natifs', font=font(14), fill='#efddbd')
    for column, label in enumerate(['Originale', 'Rouge', 'Bleue', 'Verte']):
        draw.text((80+column*80+14, 38), label, font=font(11), fill='#d5dfdc')
    for index, sprites in enumerate(rows):
        y = 70+index*88
        draw.text((8, y+16), f'{index+1:02d}', font=font(13), fill='#efddbd')
        draw.text((8, y+37), LABELS[index], font=font(11), fill='#d5dfdc')
        draw.rectangle((80, y-2, 398, y+66), fill=background)
        for column, sprite in enumerate(sprites):
            board.paste(sprite, (88+column*80, y), sprite)
    board.save(DEST/filename)


def zoom_row(sprites, number):
    board = Image.new('RGB', (832, 258), '#243036')
    draw = ImageDraw.Draw(board)
    draw.text((10, 8), f'{number:02d} · {LABELS[number-1]} · même tête · zoom entier ×3', font=font(16), fill='#efddbd')
    for column, (sprite, label) in enumerate(zip(sprites, ['Originale', 'Rouge', 'Bleue', 'Verte'])):
        enlarged = sprite.resize((192, 192), Image.Resampling.NEAREST)
        board.paste(enlarged, (8+column*208, 36), enlarged)
        draw.text((14+column*208, 232), label, font=font(12), fill='#d5dfdc')
    board.save(DEST/f'all-colors-P{number:02d}-zoom-x3.png')


def main():
    if (DOCS/'acceptance.json').exists():
        raise SystemExit('Accepted wave is frozen; create a new revision.')
    plan = json.loads((DOCS/'all-armor-color-prompts.json').read_text('utf-8'))
    assert len(plan['entries'])==12
    assert {(entry['model_number'], entry['color']) for entry in plan['entries']} == {
        (f'{n:02d}', color) for n in range(1, 5) for color, _ in COLORS}
    progression = json.loads((DOCS/'progression.json').read_text('utf-8'))
    heavy = json.loads((DOCS/'readability-pilot.json').read_text('utf-8'))
    existing = json.loads((DOCS/'colors.json').read_text('utf-8'))
    head = Image.open(ROOT/progression['head_layer']).convert('RGBA')
    head_hash = pixels_hash(head.crop((0, 0, 64, NECK_Y)))
    bases = progression['entries'][:4]+[heavy]
    assert len(bases)==5
    protected = {entry['source']:sha(ROOT/entry['source']) for entry in bases}
    rows, entries = [], []
    for index, base in enumerate(bases):
        number = f'{index+1:02d}'
        original = Image.open(ROOT/base['native']).convert('RGBA')
        row = [original]
        variants = []
        for color, label in COLORS:
            if index==4:
                item = next(e for e in existing['entries'] if e['candidate_id'].endswith('-'+color))
                sprite = Image.open(ROOT/item['native']).convert('RGBA')
                entry = dict(item)
                entry['reused_existing_proposal'] = True
            else:
                item = next(e for e in plan['entries'] if e['model_number']==number and e['color']==color)
                source = ART/(item['candidate_id']+'-body-source.png')
                shutil.copyfile(item['generated_file'], source)
                body, sprite, measurements = compose_body(source, head)
                native = DEST/(item['candidate_id']+'-native64.png')
                body.save(DEST/(item['candidate_id']+'-body-native64.png'))
                sprite.save(native)
                entry = {'candidate_id':item['candidate_id'], 'status':'awaiting_visual_approval',
                    'source':source.relative_to(ROOT).as_posix(), 'source_sha256':sha(source),
                    'native':native.relative_to(ROOT).as_posix(), 'native_sha256':sha(native),
                    'measurements':measurements, 'reused_existing_proposal':False}
            assert sprite.size==(64, 64)
            assert pixels_hash(sprite.crop((0, 0, 64, NECK_Y)))==head_hash
            original_alpha, variant_alpha = original.getchannel('A').tobytes(), sprite.getchannel('A').tobytes()
            intersection = sum(a>=128 and b>=128 for a,b in zip(original_alpha, variant_alpha))
            union = sum(a>=128 or b>=128 for a,b in zip(original_alpha, variant_alpha))
            similarity = intersection/union
            entry.update({'model_number':number, 'model_candidate':base['candidate_id'], 'color':color,
                'color_label':label, 'head_region_sha256':head_hash,
                'solid_alpha_intersection_over_union_vs_base':similarity,
                'silhouette_review':'image_gen proposal; compare the native64 board before approval',
                'strict_pixel_palette_mask':'pending_production_after_visual_review'})
            variants.append(entry); row.append(sprite)
        assert pixels_hash(original.crop((0, 0, 64, NECK_Y)))==head_hash
        rows.append(row)
        entries.append({'number':number, 'label':LABELS[index], 'base':base['candidate_id'],
            'base_native':base['native'], 'variants':variants,
            'colored_material':'protective_vest_and_light_reinforcements' if index==0 else 'protective_panels_and_plates'})
        zoom_row(row, index+1)
    assert all(sha(ROOT/path)==digest for path, digest in protected.items())
    matrix(rows, 'all-armors-colors-native64.png', '#343b3f')
    matrix(rows, 'all-armors-colors-light-native64.png', '#858478')
    write_json(DOCS/'all-armor-colors.json', {
        'review_id':'BASTION-W01', 'revision':'all-five-color-variants', 'date':'2026-10-21',
        'status':'awaiting_visual_review', 'model_count':5, 'colored_variants':15, 'views_with_originals':20,
        'new_color_sources_generated':12, 'very_heavy_sources_reused':3,
        'colors':['red','blue','green'], 'head_identical_rgba_pixels_for_all_20':True,
        'head_region':[0,0,64,NECK_Y], 'head_region_sha256':head_hash,
        'all_exports_native64':True, 'base_sources_unchanged':protected,
        'skin_and_underlayer_palette_intended_unchanged':True,
        'very_light_material_exception':'protective textile vest recolored; ordinary trousers and exposed skin retain their palette',
        'palette_variants_are_not_additional_models':True, 'entries':entries,
        'runtime_integration':False, 'other_classes_started':False})
    details=''.join(f'<details><summary>{index+1:02d} · {label} : zoom entier ×3</summary><div class="native-scroll"><img class="board" width="832" height="258" src="bastion-wave-01/all-colors-P{index+1:02d}-zoom-x3.png" alt="{label}, originale puis variantes rouge, bleue et verte en zoom entier ×3"></div></details>' for index,label in enumerate(LABELS))
    section='''<h3 id="bastion-colors">Les cinq armures · rouge, bleu et vert pour chacune</h3>
<p><strong>5 modèles × 3 couleurs = 15 variantes.</strong> Chaque ligne montre l’originale, puis ses versions rouge, bleue et verte. La très lourde utilise l’essai simplifié.</p>
<p class="note">La très légère n’a pas de plaques rigides : ses couleurs portent sur le gilet protecteur et ses renforts clairs. La peau, les matières sombres et le pantalon conservent leurs teintes. Sur les autres tenues, les panneaux et plaques de protection prennent une seule teinte avec leurs ombres et lumières.</p>
<div class="native-scroll"><img class="board" width="400" height="518" src="bastion-wave-01/all-armors-colors-native64.png" alt="Les cinq armures du Bastion, chacune originale, rouge, bleue et verte, vingt vues en 64 × 64 natifs"></div>
<details><summary>Comparer les vingt vues sur fond clair</summary><div class="native-scroll"><img class="board" width="400" height="518" src="bastion-wave-01/all-armors-colors-light-native64.png" alt="Les cinq armures et toutes leurs variantes rouge, bleue et verte à taille native sur fond clair"></div></details>
'''+details+'''
<p class="note">La tête et l’augmentation sont strictement identiques sur les vingt vues. Ces couleurs restent des variantes des cinq mêmes modèles. La simplification des quatre premières tenues et les masques de palettes du jeu restent en revue.</p>
<p><a href="../../docs/reviews/class-readability-01/bastion-wave-01/all-armor-color-prompts.json">Prompts image_gen des variantes supplémentaires</a> · <a href="../../docs/reviews/class-readability-01/bastion-wave-01/all-armor-colors.json">Catalogue des quinze variantes et contrôles</a></p>'''
    (DEST/'all-colors-section.html').write_text(section, 'utf-8')
    print(json.dumps({'models':5,'colored_variants':15,'views':20,'head_pixels_identical':True}))


if __name__=='__main__':
    main()
