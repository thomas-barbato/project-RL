"""Transfer the edited conduit materials into the arrival review."""
from pathlib import Path
import json

from PIL import Image, ImageChops

from build_city_cyberpunk12 import normalized, read, sha


ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/city-cyberpunk12'
OUT = ART / 'lot03-conduites-r02'
ARRIVAL = (64, 3136, 1632, 4064)

# These bands follow the existing runs in the native reference crop.
RUNS = [
    ('V1-C01-maison', 'Gaines de la maison', [
        (107, 104, 129, 252), (119, 91, 306, 116), (291, 56, 315, 105),
        (307, 51, 508, 77), (496, 58, 518, 238),
    ]),
    ('V1-C02-commerce', 'Conduites du commerce', [
        (685, 90, 939, 112), (667, 100, 701, 195), (933, 91, 954, 256),
        (812, 273, 825, 329), (932, 277, 949, 337),
    ]),
    ('V1-C03-garage-haut', 'Conduites du petit garage', [
        (1408, 20, 1458, 53), (1397, 145, 1487, 161),
    ]),
    ('V1-C04-garage-est', 'Conduites du garage est', [
        (1408, 334, 1473, 372), (1389, 516, 1518, 547),
    ]),
    ('V1-C05-ruine', 'Conduites de la ruine', [
        (800, 675, 1441, 688), (795, 682, 833, 700), (782, 686, 809, 795),
        (794, 781, 854, 804), (797, 830, 813, 886), (896, 835, 908, 891),
        (946, 833, 964, 898), (1008, 844, 1061, 855), (1048, 849, 1061, 900),
    ]),
]


def material_mask(original, edit, bands):
    mask = Image.new('L', original.size)
    old = original.load()
    new = edit.load()
    for left, top, right, bottom in bands:
        for y in range(top, bottom):
            for x in range(left, right):
                red, green, blue, _ = old[x, y]
                edited_red, edited_green, edited_blue, _ = new[x, y]
                warmth = red - max(green, blue)
                edited_warmth = edited_red - max(edited_green, edited_blue)
                if red > 39 and warmth > 14 and green < red * .79 and blue < red * .79:
                    # Copy the edited material where the input and output agree
                    # that the red accent was removed. Dark outlines stay put.
                    if edited_warmth < max(10, warmth * .49) and max(new[x, y][:3]) < 156:
                        mask.putpixel((x, y), 255)
    return mask


def main():
    manifest_path = OUT / 'manifest.json'
    if manifest_path.exists():
        assert not read(manifest_path).get('presented_for_user_review'), 'Keep the reviewed revision.'
    parent_path = ART / 'lot03/manifest.json'
    parent = read(parent_path)
    assert sha(ROOT / parent['candidate']) == parent['candidate_sha256']
    before = Image.open(ROOT / parent['candidate']).convert('RGBA')
    original = before.crop(ARRIVAL)
    source = OUT / 'arrivee-conduites-source-r02.png'
    edited, scale = normalized(source, original.size)
    result = before.copy()
    authorized = Image.new('L', before.size)
    parts = []
    for ident, label, bands in RUNS:
        mask = material_mask(original, edited, bands)
        bounds = mask.getbbox()
        assert bounds is not None, ident
        pixels = edited.crop(bounds)
        pixels.putalpha(mask.crop(bounds))
        offset = (ARRIVAL[0] + bounds[0], ARRIVAL[1] + bounds[1])
        result.alpha_composite(pixels, offset)
        coverage = Image.new('L', before.size)
        coverage.paste(mask.crop(bounds), offset)
        authorized = ImageChops.lighter(authorized, coverage)
        layer_path = OUT / f'{ident}-native-r02.png'
        pixels.save(layer_path)
        parts.append({'id': ident, 'at_px': list(offset), 'native_size': list(pixels.size),
                      'parts': [{'id': ident + '-materiau', 'label': label,
                                 'file': layer_path.relative_to(ROOT).as_posix(),
                                 'sha256': sha(layer_path), 'bbox': [0, 0, *pixels.size]}]})

    diff = ImageChops.difference(before, result)
    assert diff.getbbox(alpha_only=False) is not None
    diff.paste((0, 0, 0, 0), mask=authorized)
    assert diff.getbbox(alpha_only=False) is None
    assert before.getchannel('A').tobytes() == result.getchannel('A').tobytes()
    mask_path = OUT / 'authorized-mask-r02.png'
    authorized.save(mask_path)
    candidate = OUT / 'street-candidate-r02.png'
    result.save(candidate)
    manifest = dict(parent)
    manifest.update(review='CITY-CYBERPUNK12-LOT03-CONDUITES-r02',
                    status='artwork-awaiting-user-prevalidation',
                    parent_review=parent_path.relative_to(ROOT).as_posix(),
                    base=parent['candidate'], base_sha256=parent['candidate_sha256'],
                    candidate=candidate.relative_to(ROOT).as_posix(), candidate_sha256=sha(candidate),
                    source=source.relative_to(ROOT).as_posix(), source_sha256=sha(source),
                    source_uniform_scale=scale, prompt='docs/reviews/city-cyberpunk12/CONDUITES-r02-PROMPT.txt',
                    authorized_mask=mask_path.relative_to(ROOT).as_posix(),
                    items=parent['items'] + parts, presented_for_user_review=False)
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

    current_path = ART / 'review-current.json'
    current = read(current_path)
    replacements = {'lot03-maison': ('conduites-r02-maison', parts[0]['id']),
                    'lot03-commerce': ('conduites-r02-commerce', parts[1]['id']),
                    'lot03-secteur': ('conduites-r02-secteur', '')}
    for view in current['views']:
        if view['id'] in replacements:
            view['id'], view['item'] = replacements[view['id']]
        if view['id'] == 'conduites-r02-secteur':
            view['name'] = "Secteur d'arrivée · vue d'ensemble"
    current.update(review=manifest['review'], manifest=manifest_path.relative_to(ROOT).as_posix(),
                   before=manifest['base'], after=manifest['candidate'],
                   before_roof_overrides=current['roof_overrides'],
                   default_view='conduites-r02-maison', before_label='conduites rouges',
                   after_label='gaines sombres', review_doc='CONDUITES-r02.md',
                   revision_note='Les conduites du secteur d’arrivée adoptent des gaines sombres et des métaux patinés.',
                   revision_note_detail='Les vues Maison, Commerce et Secteur comparent les finitions à 64 px par case.')
    current_path.write_text(json.dumps(current, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(f'{len(parts)} conduit layers composed on the arrival map.')


if __name__ == '__main__':
    main()
