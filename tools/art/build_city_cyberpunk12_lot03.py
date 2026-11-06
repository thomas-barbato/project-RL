"""Compose the three arrival modules over the approved r04 map."""
from pathlib import Path
import json

from PIL import Image, ImageChops, ImageDraw

from build_city_cyberpunk12 import normalized, read, sha


ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/city-cyberpunk12'
OUT = ART / 'lot03'

HOUSE = [(55, 94), (65, 86), (242, 86), (242, 52), (432, 52),
         (438, 65), (438, 176), (431, 178), (431, 229), (212, 229),
         (209, 233), (209, 258), (192, 258), (188, 270), (126, 270),
         (122, 258), (57, 258), (55, 234)]
SHOP = [(46, 47), (310, 44), (318, 59), (318, 185), (300, 193),
        (300, 201), (53, 201), (53, 196), (41, 192), (41, 63)]


def rectangle(size, box):
    mask = Image.new('L', size)
    mask.paste(255, box)
    return mask


def interior_masks(size, boundary, fixtures, protected=()):
    floor = Image.new('L', size)
    ImageDraw.Draw(floor).polygon(boundary, fill=255)
    for box in protected:
        floor.paste(0, box)
    for name, box in fixtures:
        area = ImageChops.multiply(floor, rectangle(size, box))
        yield name, area
        floor = ImageChops.subtract(floor, area)
    yield 'sol', floor


def masks(index, size):
    if index == 0:
        return list(interior_masks(size, HOUSE, [
            ('lit', (57, 95, 126, 238)),
            ('chevet', (124, 94, 151, 138)),
            ('cuisine', (151, 86, 248, 145)),
            ('frigo', (246, 52, 291, 119)),
            ('modules-du-fond', (290, 52, 397, 104)),
            ('etageres', (393, 52, 439, 229)),
            ('tapis-et-caisse', (57, 138, 155, 260)),
        ]))
    if index == 1:
        parts = list(interior_masks(size, SHOP, [
            ('rayonnages', (50, 49, 106, 123)),
            ('etabli-du-fond', (151, 51, 236, 125)),
            ('armoires-et-ventilation', (225, 45, 314, 113)),
            ('rangement-ouest', (41, 110, 96, 201)),
            ('comptoir', (151, 143, 269, 201)),
            ('rangement-est', (258, 107, 319, 201)),
        ], protected=[(108, 46, 151, 111)]))
        return [*parts, ('auvent', rectangle(size, (197, 212, 299, 252)))]
    return [
        ('pilier-ouest', rectangle(size, (84, 82, 154, 214))),
        ('pilier-est', rectangle(size, (318, 82, 394, 214))),
        ('armoire-de-service', rectangle(size, (29, 136, 81, 214))),
    ]


def main():
    manifest_path = OUT / 'manifest.json'
    if manifest_path.exists():
        assert not read(manifest_path).get('presented_for_user_review'), 'Preserve the reviewed lot.'
    parent = read(ART / 'lit-r04/manifest.json')
    recipe = read(OUT / 'recipe.json')
    assert parent['status'] == 'artwork-prevalidated'
    assert sha(ROOT / recipe['base']) == parent['candidate_sha256'] == recipe['base_sha256']
    before = Image.open(ROOT / recipe['base']).convert('RGBA')
    after = before.copy()
    allowed = Image.new('L', before.size)
    arrivals = []
    for index, item in enumerate(recipe['items']):
        x, y, right, bottom = item['rect']
        size = (right - x, bottom - y)
        drawing, scale = normalized(ROOT / item['source'], size)
        records = []
        for part_name, mask in masks(index, size):
            layer = drawing.copy()
            layer.putalpha(mask)
            ident = f"{item['id']}-{part_name}"
            file = OUT / f'{ident}-native-r01.png'
            layer.save(file)
            after.alpha_composite(layer, (x, y))
            bounds = mask.getbbox()
            world = (x + bounds[0], y + bounds[1], x + bounds[2], y + bounds[3])
            allowed.paste(255, world, mask.crop(bounds))
            records.append({'id': ident, 'file': file.relative_to(ROOT).as_posix(),
                            'sha256': sha(file), 'bbox': list(bounds)})
        arrivals.append({**item, 'at_px': [x, y], 'native_size': list(size),
                         'source_sha256': sha(ROOT / item['source']),
                         'source_uniform_scale': scale, 'parts': records})

    outside = ImageChops.difference(before, after).convert('RGB')
    outside.paste((0, 0, 0), mask=allowed)
    assert outside.getbbox() is None
    assert before.getchannel('A').tobytes() == after.getchannel('A').tobytes()
    candidate = OUT / 'street-candidate-r01.png'
    after.save(candidate)
    mask_path = OUT / 'authorized-mask-r01.png'
    allowed.save(mask_path)
    manifest = {
        'review': 'CITY-CYBERPUNK12-LOT03-ARRIVEE-r01',
        'status': 'artwork-awaiting-user-prevalidation',
        'base': recipe['base'], 'base_sha256': recipe['base_sha256'],
        'candidate': candidate.relative_to(ROOT).as_posix(),
        'candidate_sha256': sha(candidate),
        'authorized_mask': mask_path.relative_to(ROOT).as_posix(),
        'parent_review': 'assets/source-art/city-cyberpunk12/lit-r04/manifest.json',
        'items': parent['items'] + arrivals, 'roofs': parent['roofs'],
        'security_revision': parent['security_revision'],
        'campaign_integrated': False,
    }
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    current_path = ART / 'review-current.json'
    current = read(current_path)
    views = [
        {'id': 'lot03-maison', 'name': 'Maison du secteur · intérieur', 'rect': [64, 3120, 640, 448],
         'actor': [6, 52], 'hide_roofs': [0], 'item': arrivals[0]['id']},
        {'id': 'lot03-commerce', 'name': 'Commerce · vue en coupe', 'rect': [628, 3152, 512, 448],
         'actor': [12, 55], 'hide_roofs': [1], 'item': arrivals[1]['id']},
        {'id': 'lot03-portail', 'name': 'Portail · mécanismes', 'rect': [216, 3776, 520, 288],
         'actor': [7, 61], 'hide_roofs': [], 'item': arrivals[2]['id']},
        {'id': 'lot03-secteur', 'name': 'Secteur d’arrivée · vue d’ensemble', 'rect': [64, 3136, 1568, 928],
         'actor': [7, 62], 'hide_roofs': [], 'item': ''},
    ]
    current.update(review=manifest['review'], status=manifest['status'],
                   manifest=manifest_path.relative_to(ROOT).as_posix(),
                   title='Première ville · secteur d’arrivée',
                   summary='Maison visitable, commerce de réparation et portail du périmètre.',
                   review_doc='LOT03-ARRIVEE.md',
                   before=recipe['base'], after=manifest['candidate'],
                   before_roof_overrides=current['roof_overrides'],
                   before_label='avant reprise du secteur', after_label='lot 03 / candidat',
                   default_view=views[0]['id'],
                   revision_note='Lot 03 à inspecter. Le logement r04 au lit raccourci est prévalidé.',
                   revision_note_detail='Le menu Vue donne accès à la maison, au commerce en coupe, au portail et au secteur entier.',
                   views=views + [v for v in current['views'] if not v['id'].startswith('lot03-')])
    current_path.write_text(json.dumps(current, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(f"Arrival candidate ready with {sum(len(i['parts']) for i in arrivals)} independent layers.")


if __name__ == '__main__':
    main()
