"""Compose the two compact beds on the playable first city for review."""
from PIL import Image, ImageChops, ImageDraw

from build_city_cyberpunk12 import ART, DOC, ROOT, normalized, read, sha
from patch_city_cyberpunk12_lit_r05 import relative, write

OUT = ART / 'lits-r01'
BASE = ROOT / 'assets/maps/prison-street-expanded-01/street-with-sewer-entry.png'
STUDIES = [
    ('arrivee', 'V1-I03-maison-arrivee-lit', (128, 3216), (204, 240), [
        (63, 37), (125, 37), (127, 43), (127, 171), (123, 176), (69, 176), (63, 173),
    ]),
    ('intermediaire', 'V1-I01-logement-D05-lit', (768, 1192), (240, 232), [
        (99, 31), (168, 31), (174, 36), (174, 219), (168, 225),
        (101, 225), (93, 220), (93, 69), (95, 62), (95, 37),
    ]),
]


def main():
    manifest_path = OUT / 'manifest.json'
    if manifest_path.exists():
        assert not read(manifest_path).get('presented_for_user_review'), 'Keep the reviewed revision.'
    previous = read(DOC / 'ACCORD-DEPART-r01.json')
    assert sha(BASE) == previous['active_files_sha256'][relative(BASE)]
    approval_path = DOC / 'LIT-ACCEPTANCE-r05.json'
    approval = read(approval_path)
    assert sha(ROOT / approval['candidate']) == approval['candidate_sha256']
    assert sha(ROOT / approval['layer']) == approval['layer_sha256']
    base = Image.open(BASE).convert('RGBA')
    after = base.copy()
    allowed = Image.new('L', base.size)
    items = []
    for name, ident, origin, size, footprint in STUDIES:
        source = OUT / f'{name}-source.png'
        art, scale = normalized(source, size)
        mask = Image.new('L', size)
        ImageDraw.Draw(mask).polygon(footprint, fill=255)
        mask = ImageChops.multiply(mask, art.getchannel('A'))
        bounds = mask.getbbox()
        layer = art.crop(bounds)
        layer.putalpha(mask.crop(bounds))
        at = (origin[0] + bounds[0], origin[1] + bounds[1])
        after.alpha_composite(layer, at)
        allowed.paste(255, at, layer.getchannel('A'))
        file = OUT / f'{ident}-native.png'
        layer.save(file)
        items.append(dict(id=ident, source=relative(source), source_sha256=sha(source),
                          source_uniform_scale=scale, prompt=relative(DOC / f'LITS-r01-{name.upper()}-PROMPT.txt'),
                          parts=[dict(id=ident + '-r01', label='Lit simple compact', file=relative(file),
                                      sha256=sha(file), native_size=list(layer.size), at_px=list(at))]))
    delta = ImageChops.difference(base, after)
    assert delta.getbbox(alpha_only=False)
    delta.paste((0, 0, 0, 0), mask=allowed)
    assert not delta.getbbox(alpha_only=False)
    assert base.getchannel('A').tobytes() == after.getchannel('A').tobytes()
    candidate = OUT / 'street-candidate-r01.png'
    mask_path = OUT / 'authorized-mask-r01.png'
    after.save(candidate)
    allowed.save(mask_path)
    manifest = dict(review='CITY-CYBERPUNK12-LITS-r01', status='artwork-awaiting-user-prevalidation',
                    base=relative(BASE), base_sha256=sha(BASE), candidate=relative(candidate),
                    candidate_sha256=sha(candidate), authorized_mask=relative(mask_path),
                    scale_reference=relative(approval_path), items=items, roofs=[],
                    campaign_integrated=False, presented_for_user_review=False)
    write(manifest_path, manifest)
    views = [
        dict(id='lits-r01-arrivee', name='Maison du départ', rect=[64, 3120, 640, 448],
             actor=[6, 52], hide_roofs=[0], item=items[0]['id']),
        dict(id='lits-r01-intermediaire', name='Logement intermédiaire', rect=[384, 1088, 704, 512],
             actor=[12, 21], hide_roofs=[2], item=items[1]['id']),
    ]
    current = dict(review=manifest['review'], status=manifest['status'], manifest=relative(manifest_path),
                   title='Première ville · lits simples r01',
                   summary='Les deux logements jouables reprennent les proportions du lit simple validé.',
                   intro='Le comparatif montre les lits avec le personnage à 64 px par case.',
                   revision_note='Lits adaptés aux deux logements.',
                   revision_note_detail='Le raccord suit le sol de chaque pièce sous l’ancien cadre.',
                   review_doc='LITS-r01.md', before=manifest['base'], after=manifest['candidate'],
                   before_label='carte actuelle', after_label='lit compact', default_view=views[0]['id'],
                   roof_overrides={}, before_roof_overrides={}, views=views)
    write(ART / 'review-current.json', current)


if __name__ == '__main__':
    main()
