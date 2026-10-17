"""Package the approved EQUIPMENT-NATIVE64-01 pixels, without changing content.

The original twelve outfit banks are read-only. Seven additional outfits keep
the historical measured rig and source-authored fingers for arbitrary weapons.
No game asset depends on artifacts/, and no preview programs are compiled.
"""
from pathlib import Path
from functools import lru_cache
from types import SimpleNamespace
import ast
import hashlib
import json
import math

from PIL import Image, ImageChops, ImageDraw, ImageFont
import numpy as np

from prepare_equipment64_review import fitted, cell, ROOT, REVIEW, SOURCES, SPRITES

OUT = SPRITES / 'equipment-native64'
ARCHIVE = ROOT / 'tools/archive/refonte-agent-2026-10-06'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_algorithms(filename, functions, constants=(), **scope):
    """Reuse only pure geometry/color functions, never archived I/O entrypoints."""
    scope.update(Image=Image, ImageChops=ImageChops, ImageDraw=ImageDraw,
                 ImageFont=ImageFont, lru_cache=lru_cache, math=math)
    path = ARCHIVE / filename
    tree = ast.parse(path.read_text('utf-8'))
    selected = []
    for node in tree.body:
        if isinstance(node, ast.FunctionDef) and node.name in functions:
            selected.append(node)
        elif isinstance(node, ast.Assign) and any(
                isinstance(t, ast.Name) and t.id in constants for t in node.targets):
            selected.append(node)
    exec(compile(ast.Module(body=selected, type_ignores=[]), str(path), 'exec'), scope)
    return SimpleNamespace(**scope)


def rig_algorithms():
    affine = load_algorithms('build_comic_catalogue_03.py', {'affine'}, MS=1024, BASE=SOURCES)
    rig = load_algorithms('build_flat_rig_review_02.py',
                          {'native', 'master', 'normalize', 'solve3', 'mask', 'selected',
                           'source', 'coefficients', 'registered', 'body', 'fingers'},
                          {'JOINTS', 'POLYS'}, F=16, MS=1024, S=29 / 1100, old=affine,
                          OUT=SOURCES / 'flat-rig-review-02')
    catalog_dir = REVIEW.parent / 'armor-flat-review-01'
    catalogue = json.loads((catalog_dir / 'catalogue.json').read_text('utf-8'))
    art = load_algorithms('build_armor_flat_review_01.py',
                         {'source', 'joints', 'affine_points', 'arm_polys', 'registered',
                          'body', 'fingers', 'rgb', 'distance', 'material_variant'},
                         {'RAMPS', 'MATERIALS'}, rig=rig, OUT=catalog_dir, F=16, MS=1024,
                         POSES=['independent', 'rifle', 'heavy', 'pole'],
                         ENTRIES={e['id']: e for e in catalogue['entries']})
    return art


def material_data(art, ident):
    """Recover the original deterministic per-source palette assignment.

    Preserve the archived algorithm; only its former preview/cache writes are
    omitted. No material classification is performed by the runtime renderer.
    """
    spec = art.MATERIALS[ident]
    im = Image.new('RGBA', (1254, 1254))
    for j, pose in enumerate(art.POSES):
        im.alpha_composite(art.source(ident, pose).resize((627, 627)), (j % 2 * 627, j // 2 * 627))
    im = im.resize((900, 900), Image.Resampling.LANCZOS)
    q = im.convert('RGB').quantize(colors=64, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE)
    flat = q.getpalette()[:64 * 3]
    colors = [flat[i:i + 3] for i in range(0, len(flat), 3)]
    protected = [(n, n, n) for n in (0, 12, 24, 40, 60, 90, 125, 170, 220, 250)] + [(90, 59, 51), (147, 103, 85)]
    if spec[0] not in ('ivory', 'bone') and spec[1] not in ('ivory', 'bone'):
        protected += [art.rgb(s) for s in art.RAMPS['bone']]
    prototypes = {'protected': protected, 'principal': [art.rgb(s) for s in art.RAMPS[spec[0]]]}
    if spec[1]:
        prototypes['secondary'] = [art.rgb(s) for s in art.RAMPS[spec[1]]]
    indices = {key: [] for key in prototypes}
    for i, color in enumerate(colors):
        key = min(prototypes, key=lambda k: min(art.distance(color, t) for t in prototypes[k]))
        indices[key].append(i)
    return dict(id=ident, source_palette=colors, material_indices=indices,
                source_materials=list(spec[:2]), variants=[list(spec[:2]), list(spec[2:4]), list(spec[4:6])])


def atlas(images, side, name):
    result = Image.new('RGBA', (16 * side, ((len(images) + 15) // 16) * side))
    for i, image in enumerate(images):
        assert image.size == (side, side), (name, i, image.size)
        result.paste(image, (i % 16 * side, i // 16 * side))
    result.save(OUT / name, optimize=True)


def material_mask(image):
    # Same persistent paint-label policy as the historical inventory bank:
    # label 0 protects neutral steel, bone, dark outlines and transparent pixels.
    pixels = np.asarray(image)
    high = pixels[:, :, :3].max(axis=2).astype('int16')
    low = pixels[:, :, :3].min(axis=2).astype('int16')
    paint = (pixels[:, :, 3] > 0) & (high > 45) & (high - low > high * .25)
    mask = np.zeros(pixels.shape, dtype='uint8')
    mask[paint] = [1, 0, 0, 255]
    return Image.fromarray(mask)


def main():
    reviewed = json.loads((REVIEW / 'review.json').read_text('utf-8'))
    approval_path = REVIEW / 'visual-acceptance-01.json'
    approval = json.loads(approval_path.read_text('utf-8'))
    assert approval['review_sha256'] == digest(REVIEW / 'review.json')
    defaults_path = REVIEW / 'palette-defaults-01.json'
    defaults = json.loads(defaults_path.read_text('utf-8'))['default_source_variants'] if defaults_path.exists() else {}
    protected = {str(p.relative_to(ROOT)): digest(p) for p in (SPRITES / 'flat-agents').iterdir() if p.is_file()}
    OUT.mkdir(parents=True, exist_ok=True)
    mappings = {}
    import re
    for ident, index in re.findall(r'"([^"]+)" => ItemArt \{ icon: \d+, illustration: (\d+) \}',
                                  (ROOT / 'src/game_art_catalog.rs').read_text('utf-8')):
        mappings[ident] = int(index)
    mappings.update({'core:tenue_assaut': 200, 'core:tenue_technique': 201,
                     'core:tenue_traque': 202, 'core:tenue_rituelle': 203, 'core:tenue_metamorphe': 204})
    detailed_items = Image.open(SPRITES / 'cyberpunk/items-128.png').convert('RGBA')
    icons = []
    bindings = {}
    def reviewed_icon(entry):
        source = ROOT / entry['source']
        assert digest(source) == entry['source_sha256'], source
        image = Image.open(source).convert('RGBA')
        if source.name == 'items-128.png':
            image = cell(detailed_items, mappings[entry['id']], 128)
        image = fitted(image)
        assert hashlib.sha256(image.tobytes()).hexdigest() == entry['rgba_sha256'], entry['review_id']
        return image
    for entry in reviewed['icons']:
        image = reviewed_icon(entry)
        if entry['variant'] == 0 and entry['id'] in defaults:
            replacement = next(e for e in reviewed['icons'] if e['id'] == entry['id'] and e['variant'] == defaults[entry['id']])
            image = reviewed_icon(replacement)
        i = len(icons)
        illustration = mappings[entry['id']]
        if illustration not in bindings:
            bindings[illustration] = [i, False]
        else:
            assert i == bindings[illustration][0] + entry['variant']
            bindings[illustration][1] = True
        icons.append(image)
    atlas(icons, 64, 'icons-64.png')
    atlas([im.resize((32, 32), Image.Resampling.LANCZOS) for im in icons], 32, 'icons-32.png')
    atlas([material_mask(im) for im in icons], 64, 'materials-64.png')
    atlas([material_mask(im.resize((32, 32), Image.Resampling.LANCZOS)) for im in icons], 32, 'materials-32.png')
    extended = [e for e in reviewed['worn'] if e['status'] == 'pending_native64_worn_review']
    exact = []
    bodies = []
    fingers = []
    art = rig_algorithms()
    palettes = {}
    torso_bindings = []
    for i, entry in enumerate(extended):
        ident = entry['id']
        torso_bindings.append((mappings[entry['content_id']], 212 + i))
        for source in entry['sources']:
            assert digest(ROOT / source['path']) == source['sha256']
        default = defaults.get(entry['content_id'], 0)
        for frame in range(48):
            variant, config = divmod(frame, 16)
            source = entry['sources'][(default if variant == 0 else variant) * 16 + config]
            path = ROOT / source['path']
            assert digest(path) == source['sha256']
            image = Image.open(path).convert('RGBA')
            assert image.size == (128, 128)
            exact.append(image)
        data = material_data(art, ident)
        data['default_source_variant'] = default
        palettes[ident] = data
        for variant in range(3):
            for pose in art.POSES:
                source_variant = default if variant == 0 else variant
                bodies.append(art.material_variant(art.body(ident, pose), data, source_variant)
                              .resize((128, 128), Image.Resampling.LANCZOS))
                for side in (0, 1):
                    fingers.append(art.material_variant(art.fingers(ident, pose, side), data, source_variant)
                                   .resize((128, 128), Image.Resampling.LANCZOS))
        print(f'{ident}: native64 body, source fingers and 48 exact reviewed configurations', flush=True)
    assert len(exact) == 336 and len(bodies) == 84 and len(fingers) == 168
    atlas(exact, 128, 'outfits-128.png')
    atlas([im.resize((64, 64), Image.Resampling.LANCZOS) for im in exact], 64, 'outfits-64.png')
    atlas(bodies, 128, 'bodies-128.png')
    atlas(fingers, 128, 'fingers-128.png')
    generated = ['// Generated by tools/art/build_equipment_native64.py; approved EQUIPMENT-NATIVE64-01.',
                 'pub(super) const FLAT_OUTFIT_COUNT: u16 = 19;',
                 'pub(super) const EXTENDED_TORSOS: &[(u16, u16)] = &[']
    generated += [f'    ({old}, {new}),' for old, new in torso_bindings]
    generated += ['];', 'const NATIVE_ITEMS: &[(u16, u16, bool)] = &[']
    generated += [f'    ({illustration}, {index}, {str(variants).lower()}),' for illustration, (index, variants) in sorted(bindings.items())]
    generated += ['];', '']
    (ROOT / 'src/game_art_equipment_catalog.rs').write_text('\n'.join(generated), 'utf-8')
    manifest = dict(review_id='EQUIPMENT-NATIVE64-01', approved_by=str(approval_path.relative_to(ROOT)).replace('\\', '/'),
                    icon_count=len(icons), icon_bindings=bindings, torso_bindings=torso_bindings,
                    extended_outfits=[e['id'] for e in extended], material_palettes=palettes,
                    source_conditioning='Historical measured joints, source-authored sleeves/fingers and indexed material algorithm; original 12 banks unchanged',
                    exact_reviewed_configurations=len(exact),
                    default_source_variants=defaults,
                    generated={p.name: digest(p) for p in OUT.glob('*.png')})
    (OUT / 'manifest.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n', 'utf-8')
    assert all(digest(ROOT / path) == before for path, before in protected.items())
    print(json.dumps(dict(icons=len(icons), extended_outfits=len(extended), exact_compositions=len(exact), original_files_unchanged=len(protected))))


if __name__ == '__main__':
    main()
