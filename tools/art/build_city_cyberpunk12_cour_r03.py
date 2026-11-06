"""Reveal the accepted street ground beneath the original court machine."""
from PIL import Image, ImageDraw

from build_city_cyberpunk12 import ART, ROOT, read, sha
from build_city_cyberpunk12_cour_r01 import BASE, IDENT, MODULE_AT
from build_city_cyberpunk12_cour_r02 import original_court_floor
from patch_city_cyberpunk12_lit_r05 import relative, write

OUT = ART / 'cour-r03'
MAP = ROOT / 'assets/maps/prison-street-expanded-01'
ENTRY = ROOT / 'assets/maps/prison-sewer-01/entry.json'


def clearance(size):
    # Use the authored ground footprint, not the outline of the tall artwork.
    data = read(MAP / 'collision-repairs-r09.json')
    footprints = {'machine': Image.new('1', size), 'others': Image.new('1', size)}
    for prop in data['footprints']:
        x, y = prop['at_px']
        is_machine = prop['id'] == 'court-machines'
        if is_machine:
            assert (x, y) == MODULE_AT
        kind = 'machine' if is_machine else 'others'
        ImageDraw.Draw(footprints[kind]).polygon(
            [(x + dx, y + dy) for dx, dy in prop['polygon_px']], fill=1)
    layouts = (MAP / 'layout.txt').read_text(encoding='utf-8').splitlines()
    active_scenes = [read(MAP / 'street.json'), read(ENTRY)]
    blocked = set(map(tuple, active_scenes[0]['obstructions']))
    cells = []
    for x, y in sorted(blocked):
        center = x * 64 + 32, y * 64 + 32
        if footprints['machine'].getpixel(center):
            assert not footprints['others'].getpixel(center)
            assert layouts[y][x] == '.'
            assert [x, y] in active_scenes[1]['obstructions']
            cells.append([x, y])
    assert cells == [[11, 39], [11, 40], [12, 39], [12, 40]]
    return cells


def main():
    path = OUT / 'manifest.json'
    if path.exists():
        assert not read(path).get('presented_for_user_review'), 'Keep the reviewed revision.'
    OUT.mkdir(exist_ok=True)
    previous = ART / 'cour-r02/manifest.json'
    prior = read(previous)
    assert prior['base'] == relative(BASE) and sha(BASE) == prior['base_sha256']
    base = Image.open(BASE).convert('RGBA')
    at, floor, old_ink = original_court_floor(base)
    floor.putalpha(old_ink.point(lambda value: 255 if value else 0))
    layer = OUT / f'{IDENT}-retrait-native-r03.png'
    floor.save(layer)
    ident = f'{IDENT}-retrait-r03'
    patch = dict(file=relative(layer), at_px=list(at))
    part = dict(id=ident, label='Retrait de la machine', sha256=sha(layer), **patch)
    manifest = dict(review='CITY-CYBERPUNK12-COUR-r03', status='artwork-awaiting-user-prevalidation',
                    base=relative(BASE), base_sha256=sha(BASE), previous_review=relative(previous),
                    items=[dict(id=IDENT, parts=[part])], roofs=[], exact_floor_recomposition=True,
                    proposed_clearance=dict(remove_obstructions=clearance(base.size),
                                            scenes=[relative(MAP / 'street.json'), relative(ENTRY)]),
                    campaign_integrated=False, presented_for_user_review=False)
    write(path, manifest)
    views = [
        dict(id='cour-r03-detail', name='Emplacement de la machine', rect=[560, 2384, 512, 400],
             actor=[13, 41], hide_roofs=[], item=IDENT, after_patch=patch),
        dict(id='cour-r03-secteur', name='Cour centrale', rect=[448, 2176, 768, 688],
             actor=[13, 41], hide_roofs=[], item=IDENT, after_patch=patch),
    ]
    current = dict(review=manifest['review'], status=manifest['status'], manifest=relative(path),
                   title='Première ville · cour dégagée', summary='Retrait de la machine au milieu de la cour.',
                   intro='Deux cadrages à 64 px par case avec le personnage de référence.',
                   revision_note='Cour dégagée, variante r03.',
                   revision_note_detail='Le béton d’origine réapparaît sous l’ancien boîtier.',
                   review_doc='COUR-r03.md', before=manifest['base'], after=manifest['base'],
                   before_label='carte actuelle', after_label='cour dégagée', default_view=views[1]['id'],
                   roof_overrides={}, before_roof_overrides={}, views=views)
    write(ART / 'review-current.json', current)


if __name__ == '__main__':
    main()
