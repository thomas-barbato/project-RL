"""Compose the proposed northern approach over the current playable street."""
import json

from PIL import Image, ImageChops, ImageDraw

from build_city_cyberpunk12 import ART, DOC, ROOT, normalized, read, sha


OUT = ART / 'sortie-nord-r01'
MAP = ROOT / 'assets/maps/prison-street-expanded-01/street-with-sewer-entry.png'
REVIEW = 'CITY-CYBERPUNK12-SORTIE-NORD-r01'

STUDIES = {
    'poste': ((1280, 288), (368, 448), 'V1-N01', [
        ('toit', 'Installation de distribution', [[
            (111, 76), (260, 76), (260, 101), (272, 101), (272, 104), (281, 104),
            (281, 224), (273, 224), (273, 234), (268, 234), (268, 243), (214, 243),
            (214, 235), (183, 235), (183, 243), (110, 243), (110, 235), (104, 235),
            (104, 224), (95, 224), (95, 103), (102, 103), (102, 82), (111, 82),
        ]]),
        ('coffret', 'Coffret de raccordement', [[(233, 281), (267, 281), (269, 313), (233, 313)]]),
        ('gaine', 'Gaine sombre en façade', [[
            (173, 272), (180, 272), (180, 314), (182, 318),
            (181, 325), (172, 325), (171, 319), (173, 313),
        ]], 'gaine'),
    ]),
    'atelier': ((800, 136), (304, 400), 'V1-N02', [
        ('ventilation', 'Ventilation et gaines de l’atelier', [
            [(72, 91), (131, 91), (131, 166), (73, 166)],
            [(143, 79), (173, 79), (173, 119), (157, 137), (143, 137)],
        ]),
        ('etabli', 'Établi de diagnostic sous les gravats', [[
            (176, 183), (184, 164), (199, 163), (218, 179), (230, 187),
            (224, 204), (228, 244), (225, 263), (209, 274), (191, 281),
            (150, 284), (132, 272), (126, 254), (128, 215), (136, 198), (164, 198),
        ]]),
    ]),
}


def relative(path):
    return path.relative_to(ROOT).as_posix()


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def compose_study(name, description, candidate, scope):
    origin, size, ident, parts = description
    source = OUT / f'{name}-source.png'
    image, scale = normalized(source, size)
    prompt = DOC / f'SORTIE-NORD-r01-{name.upper()}-PROMPT.txt'
    item = {'id': ident, 'source': relative(source), 'source_sha256': sha(source),
            'prompt': relative(prompt), 'source_uniform_scale': scale, 'parts': []}
    for part, label, polygons, *alternate in parts:
        drawing = image
        details = {}
        if alternate:
            source_name = alternate[0]
            part_source = OUT / f'{source_name}-source.png'
            part_prompt = DOC / f'SORTIE-NORD-r01-{source_name.upper()}-PROMPT.txt'
            drawing, part_scale = normalized(part_source, size)
            details = {'source': relative(part_source), 'source_sha256': sha(part_source),
                       'prompt': relative(part_prompt), 'source_uniform_scale': part_scale}
        mask = Image.new('L', size)
        draw = ImageDraw.Draw(mask)
        for polygon in polygons:
            draw.polygon(polygon, fill=255)
        mask = ImageChops.multiply(mask, drawing.getchannel('A'))
        bounds = mask.getbbox()
        layer = drawing.crop(bounds)
        layer.putalpha(mask.crop(bounds))
        at = (origin[0] + bounds[0], origin[1] + bounds[1])
        path = OUT / f'{ident}-{part}-native.png'
        layer.save(path)
        scope.paste(255, at, layer.getchannel('A'))
        candidate.alpha_composite(layer, at)
        item['parts'].append({'id': f'{ident}-{part}', 'label': label, 'file': relative(path),
                              'sha256': sha(path), 'at_px': list(at), 'native_size': list(layer.size),
                              **details})
    return item


def main():
    manifest_path = OUT / 'manifest.json'
    if manifest_path.exists():
        assert not read(manifest_path).get('presented_for_user_review'), 'Keep the reviewed revision.'
    departure = read(DOC / 'ACCORD-DEPART-r01.json')
    for path, digest in departure['active_files_sha256'].items():
        assert sha(ROOT / path) == digest, path
    base = Image.open(MAP).convert('RGBA')
    candidate = base.copy()
    scope = Image.new('L', base.size)
    items = [compose_study(name, study, candidate, scope) for name, study in STUDIES.items()]
    diff = ImageChops.difference(base, candidate)
    assert diff.getbbox(alpha_only=False) is not None
    diff.paste((0, 0, 0, 0), mask=scope)
    assert diff.getbbox(alpha_only=False) is None
    assert base.getchannel('A').tobytes() == candidate.getchannel('A').tobytes()
    candidate_path = OUT / 'street-candidate-r01.png'
    scope_path = OUT / 'authorized-mask-r01.png'
    candidate.save(candidate_path)
    scope.save(scope_path)
    manifest = {
        'review': REVIEW, 'status': 'artwork-awaiting-user-prevalidation',
        'base': relative(MAP), 'base_sha256': sha(MAP),
        'candidate': relative(candidate_path), 'candidate_sha256': sha(candidate_path),
        'authorized_mask': relative(scope_path), 'items': items, 'roofs': [],
        'campaign_integrated': False, 'presented_for_user_review': False,
    }
    write(manifest_path, manifest)
    views = [
        {'id': 'sortie-nord-r01-secteur', 'name': 'Approche de la sortie nord',
         'rect': [672, 0, 976, 736], 'actor': [18, 4], 'hide_roofs': [], 'item': ''},
        {'id': 'sortie-nord-r01-poste', 'name': 'Poste de distribution',
         'rect': [1104, 256, 544, 512], 'actor': [19, 7], 'hide_roofs': [], 'item': 'V1-N01'},
        {'id': 'sortie-nord-r01-atelier', 'name': 'Atelier effondré',
         'rect': [672, 104, 512, 480], 'actor': [17, 7], 'hide_roofs': [], 'item': 'V1-N02'},
    ]
    current = {
        'review': REVIEW, 'status': manifest['status'], 'manifest': relative(manifest_path),
        'title': 'Première ville · sortie nord r01',
        'summary': 'Deux bâtiments de la première ville, près de la sortie nord.',
        'intro': 'Le comparatif se lit à 64 px par case avec le personnage de référence.',
        'revision_note': 'Proposition pour le secteur de la sortie nord.',
        'revision_note_detail': 'Un poste de distribution à l’est et un atelier de réparation effondré à l’ouest.',
        'review_doc': 'SORTIE-NORD-r01.md', 'before': manifest['base'], 'after': manifest['candidate'],
        'before_label': 'carte actuelle', 'after_label': 'proposition r01',
        'default_view': views[0]['id'], 'before_roof_overrides': {}, 'roof_overrides': {}, 'views': views,
    }
    write(ART / 'review-current.json', current)


if __name__ == '__main__':
    main()
