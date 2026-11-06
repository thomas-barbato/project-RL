"""Compose the three scoped edits on the live arrival map for review."""
import json

from PIL import Image, ImageChops, ImageDraw

from build_city_cyberpunk12 import ART, DOC, ROOT, normalized, read, sha


OUT = ART / 'depart-r01'
MAP = ROOT / 'assets/maps/prison-street-expanded-01'
BASE = MAP / 'street.png'


def relative(path):
    return path.relative_to(ROOT).as_posix()


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def interior(size):
    approved = read(ART / 'lot03-conduites-r02/manifest.json')
    source = ROOT / approved['candidate']
    assert sha(source) == approved['candidate_sha256']
    mask = Image.new('L', size)
    for item in approved['items']:
        if item['id'] not in {'V1-I03-maison-arrivee', 'V1-C01-maison'}:
            continue
        for part in item['parts']:
            path = ROOT / part['file']
            assert sha(path) == part['sha256']
            alpha = Image.open(path).getchannel('A')
            mask.paste(255, tuple(item['at_px']), alpha)
    bounds = mask.getbbox()
    pixels = Image.open(source).convert('RGBA').crop(bounds)
    pixels.putalpha(mask.crop(bounds))
    return (pixels, bounds[:2], {'source': relative(source), 'source_sha256': sha(source)})


def registered_study(name, size, transform):
    source = OUT / f'{name}-source.png'
    pixels, _ = normalized(source, size)
    pixels = pixels.transform(size, Image.Transform.AFFINE, transform, Image.Resampling.BICUBIC)
    prompt = DOC / f'DEPART-r01-{name.upper()}-PROMPT.txt'
    return pixels, {'source': relative(source), 'source_sha256': sha(source), 'prompt': relative(prompt)}


def east_patch(name, transform, boundary, protected=()):
    size = (416, 304)
    pixels, source = registered_study(name, size, transform)
    mask = Image.new('L', size)
    ImageDraw.Draw(mask).polygon(boundary, fill=255)
    for box in protected:
        mask.paste(0, box)
    bounds = mask.getbbox()
    pixels = pixels.crop(bounds)
    pixels.putalpha(mask.crop(bounds))
    return pixels, (1248 + bounds[0], 3104 + bounds[1]), source


def east_roof():
    # Follow the inner corners and leave the original stone parapet in place.
    return east_patch('maison-est', (1.022, 0, -1.3, 0, 1.025, .8), [
        (133, 53), (185, 53), (185, 55), (203, 55), (203, 53),
        (229, 53), (229, 55), (248, 55), (248, 53), (301, 53),
        (301, 72), (310, 72), (310, 148), (307, 148), (307, 163),
        (130, 163), (130, 155), (125, 155), (125, 146), (120, 146),
        (120, 73), (123, 73), (123, 65), (131, 65), (133, 63),
    ])


def east_facade():
    return east_patch('facade-est', (1.005, 0, -0.5, 0, 1.015, -0.35), [
        (134, 177), (300, 177), (300, 192), (292, 192), (292, 218),
        (278, 240), (229, 244), (218, 234), (134, 234),
    ], protected=[(168, 182, 198, 192)])


def perimeter_wall():
    pixels, source = registered_study('mur', (1152, 384), (1.008, 0, 0, 0, 1.055, -10.3))
    # The cap and foot match the remaining wall. Overlap the former end to
    # close the seam, and end inside the existing eastern frame.
    bounds = (314, 222, 1093, 320)
    return pixels.crop(bounds), (512 + bounds[0], 3744 + bounds[1]), source


def main():
    manifest_path = OUT / 'manifest.json'
    if manifest_path.exists():
        assert not read(manifest_path).get('presented_for_user_review'), 'Keep the reviewed revision.'
    scope = read(DOC / 'SCOPE.json')
    for file, digest in scope['baseline_sha256'].items():
        assert sha(ROOT / file) == digest, file

    base = Image.open(BASE).convert('RGBA')
    candidate = base.copy()
    allowed = Image.new('L', base.size)
    items = {}
    for group, ident, label, (layer, at, source) in [
        ('DEP-I01', 'DEP-I01', 'Intérieur choisi de la maison visitable', interior(base.size)),
        ('DEP-B01', 'DEP-B01-toit', 'Petite maison est · toit à verrière', east_roof()),
        ('DEP-B01', 'DEP-B01-facade', 'Petite maison est · façade', east_facade()),
        ('DEP-M01', 'DEP-M01', 'Mur continu devant la ruine', perimeter_wall()),
    ]:
        path = OUT / f'{ident}-native.png'
        layer.save(path)
        alpha = layer.getchannel('A')
        allowed.paste(255, at, alpha)
        candidate.alpha_composite(layer, at)
        item = items.setdefault(group, {'id': group, 'parts': []})
        item['parts'].append({'id': ident + '-r01', 'label': label, 'file': relative(path),
                              'sha256': sha(path), 'native_size': list(layer.size),
                              'at_px': list(at), **source})

    diff = ImageChops.difference(base, candidate).convert('RGB')
    diff.paste((0, 0, 0), mask=allowed)
    assert diff.getbbox() is None
    assert base.getchannel('A').tobytes() == candidate.getchannel('A').tobytes()
    candidate_path = OUT / 'street-candidate-r01.png'
    mask_path = OUT / 'authorized-mask-r01.png'
    candidate.save(candidate_path)
    allowed.save(mask_path)
    manifest = {
        'review': 'CITY-CYBERPUNK12-DEPART-r01',
        'status': 'artwork-awaiting-user-prevalidation',
        'base': relative(BASE), 'base_sha256': sha(BASE),
        'candidate': relative(candidate_path), 'candidate_sha256': sha(candidate_path),
        'authorized_mask': relative(mask_path), 'items': list(items.values()), 'roofs': [],
        'campaign_integrated': False, 'presented_for_user_review': False,
    }
    write(manifest_path, manifest)
    views = [
        {'id': 'depart-r01-secteur', 'name': 'Secteur de départ', 'rect': [64, 3104, 1600, 960],
         'actor': [7, 62], 'hide_roofs': [], 'item': ''},
        {'id': 'depart-r01-interieur', 'name': 'Maison visitable', 'rect': [64, 3120, 640, 448],
         'actor': [6, 52], 'hide_roofs': [0], 'item': 'DEP-I01'},
        {'id': 'depart-r01-maisons', 'name': 'Maisons de droite', 'rect': [1088, 3104, 576, 720],
         'actor': [18, 54], 'hide_roofs': [], 'item': 'DEP-B01'},
        {'id': 'depart-r01-mur', 'name': 'Mur du périmètre', 'rect': [392, 3744, 1272, 320],
         'actor': [7, 62], 'hide_roofs': [], 'item': 'DEP-M01'},
    ]
    current = {
        'review': manifest['review'], 'status': manifest['status'], 'manifest': relative(manifest_path),
        'title': 'Première ville · carte de départ r01',
        'summary': 'La carte de départ actuelle sert de base aux trois retouches.',
        'intro': 'La maison visitable, les deux maisons de droite et la fermeture du périmètre sont visibles à taille de jeu.',
        'revision_note': 'Retouches du secteur de départ.',
        'revision_note_detail': 'Intérieur choisi, toiture distincte et mur continu devant la ruine.',
        'review_doc': 'DEPART-r01.md', 'before': manifest['base'], 'after': manifest['candidate'],
        'before_label': 'carte actuelle', 'after_label': 'retouches r01',
        'default_view': views[0]['id'], 'before_roof_overrides': {}, 'roof_overrides': {}, 'views': views,
    }
    write(ART / 'review-current.json', current)


if __name__ == '__main__':
    main()
