"""Show two independent replacements on the original court floor."""
from PIL import Image, ImageChops

from build_city_cyberpunk12 import ART, DOC, ROOT, read, sha
from build_city_cyberpunk12_cour_r01 import BASE, MODULE, MODULE_AT, IDENT
from build_city_cyberpunk12_mobilier_r01 import FLOOR, recompose_frozen_street
from patch_city_cyberpunk12_lit_r05 import relative, write

OUT = ART / 'cour-r02'
VARIANTS = [('A', 'pompe', 'Groupe de pompage'), ('B', 'caisson', 'Caisson de raccordement')]


def native_sprite(source, size):
    image = Image.open(source).convert('RGBA')
    alpha = image.getchannel('A')
    assert alpha.getextrema() == (0, 255)
    bounds = alpha.point(lambda value: 255 if value >= 64 else 0).getbbox()
    image = image.crop(bounds)
    scale = min(size[0] / image.width, size[1] / image.height)
    return image.resize(tuple(round(n * scale) for n in image.size), Image.Resampling.LANCZOS)


def original_court_floor(base):
    frozen = Image.open(FLOOR / 'street-0.png').convert('RGBA')
    original = Image.open(MODULE).convert('RGBA')
    left, top, right, bottom = original.getbbox()
    x, y = MODULE_AT[0] + left, MODULE_AT[1] + top
    rect = (x, y, MODULE_AT[0] + right, MODULE_AT[1] + bottom)
    assert base.crop(rect).tobytes() == frozen.crop(rect).tobytes()
    floor = recompose_frozen_street(base.size, frozen, omit_modules=('court-machines',)).crop(rect)
    old_ink = original.crop((left, top, right, bottom)).getchannel('A')
    restored = ImageChops.difference(base.crop(rect), floor)
    restored.paste((0, 0, 0, 0), mask=old_ink.point(lambda value: 255 if value else 0))
    assert not restored.getbbox(alpha_only=False)
    return (x, y), floor, old_ink


def main():
    manifest_path = OUT / 'manifest.json'
    if manifest_path.exists():
        assert not read(manifest_path).get('presented_for_user_review'), 'Keep the reviewed revision.'
    previous_path = ART / 'cour-r01/manifest.json'
    previous = read(previous_path)
    assert previous['base'] == relative(BASE) and sha(BASE) == previous['base_sha256']
    base = Image.open(BASE).convert('RGBA')
    (x, y), floor, old_ink = original_court_floor(base)

    items, views = [], []
    for letter, name, label in VARIANTS:
        source = OUT / f'{name}-source.png'
        sprite = native_sprite(source, floor.size)
        placement = ((floor.width - sprite.width) // 2, floor.height - sprite.height)
        ident = f'{IDENT}-r02{letter}'
        sprite_path = OUT / f'{ident}-native.png'
        sprite.save(sprite_path)
        local = floor.copy()
        local.alpha_composite(sprite, placement)
        new_ink = Image.new('L', floor.size)
        new_ink.paste(sprite.getchannel('A'), placement)
        scope = ImageChops.lighter(old_ink, new_ink).point(lambda value: 255 if value else 0)
        local.putalpha(scope)
        patch = OUT / f'{ident}-compose.png'
        local.save(patch)
        parts = [dict(id=ident, label=label, file=relative(patch), sha256=sha(patch),
                      at_px=[x, y], sprite=relative(sprite_path), sprite_sha256=sha(sprite_path),
                      sprite_native_size=list(sprite.size))]
        items.append(dict(id=ident, source=relative(source), source_sha256=sha(source),
                          prompt=relative(DOC / f'COUR-r02{letter}-PROMPT.txt'), parts=parts))
        views.append(dict(id=f'cour-r02-{name}', name=f'{letter} · {label}', rect=[560, 2384, 512, 400],
                          actor=[13, 41], hide_roofs=[], item=ident,
                          after_patch=dict(file=relative(patch), at_px=[x, y])))
    manifest = dict(review='CITY-CYBERPUNK12-COUR-r02', status='artwork-awaiting-user-prevalidation',
                    base=relative(BASE), base_sha256=sha(BASE), previous_review=relative(previous_path),
                    items=items, roofs=[], presented_for_user_review=False, campaign_integrated=False)
    write(manifest_path, manifest)
    current = dict(review=manifest['review'], status=manifest['status'], manifest=relative(manifest_path),
                   title='Cour centrale · deux propositions', summary='Deux silhouettes pour la machine de la cour.',
                   intro='Chaque option se compare au boîtier actuel à 64 px par case.',
                   revision_note='Deux variantes à inspecter.',
                   revision_note_detail='Choisis A pour le groupe de pompage ou B pour le caisson de raccordement.',
                   review_doc='COUR-r02.md', before=manifest['base'], after=manifest['base'],
                   before_label='carte actuelle', after_label='proposition r02', default_view=views[0]['id'],
                   roof_overrides={}, before_roof_overrides={}, views=views)
    write(ART / 'review-current.json', current)


if __name__ == '__main__':
    main()
