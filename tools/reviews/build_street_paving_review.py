"""Recompose approved whole scenery modules, changing only the paving input.

The reconstructed baseline must match the active background byte for pixel.
This never writes active assets, lightmaps, layouts or collision data.
"""
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw
import build_street_unified_review as unified

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/first-sector-review-01/u01-corrections-r06'
PARTS = ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r06'
ACTIVE = ROOT / 'assets/maps/prison-street-expanded-01'
OUT = ROOT / 'artifacts/street-defects-review-01'


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def compose(paving=None, grass=(), *, warehouse=None, asphalt=None, repair_join=False, omit_modules=(), paving_overrides=None, omit_decor=()):
    original = Image.open(ROOT / 'assets/maps/prison-street-01/street.png').convert('RGBA')
    ground, _ = unified.ground(original, paving, asphalt, paving_overrides)
    image = Image.new('RGBA', (1664, 4224), (0, 0, 0, 255))
    image.paste(ground.crop((64, 64, 1632, 4064)), (64, 64))
    image.paste(ground.crop((1132, 0, 1292, 64)), (1132, 0))
    # Sparse independent plants belong to the ground layer, below whole props.
    for at in grass:
        image.alpha_composite(Image.open(ART / 'HERB-U01-06-native.png').convert('RGBA'), at)
    modules = read(PARTS / 'module-placements.json')
    for module in modules:
        if module['id'] in omit_modules:
            continue
        file=warehouse if warehouse and module['id']=='warehouse' else ROOT/module['file']
        image.alpha_composite(Image.open(file).convert('RGBA'), tuple(module['at']))
    hedge = ROOT / next(m['file'] for m in modules if m['id'] == 'hedge-1')
    if not repair_join:
        for x in (616, 800, 960):
            image.alpha_composite(Image.open(hedge).convert('RGBA'), (x, 3090))
    # Preserve the already approved southern arrival as a complete component.
    mask = Image.new('L', original.size, 255)
    d = ImageDraw.Draw(mask)
    for box in [(0, 0, 41, 127), (42, 0, 239, 59), (240, 0, 515, 23)]:
        d.rectangle(box, fill=0)
    if repair_join:
        # Remove only the old exterior void, following the complete facade.
        d.polygon([(0,0),(520,0),(520,29),(239,29),(239,63),(41,63),(41,325),(0,325)],fill=0)
        # Remove the ENTIRE old edge-truncated north-east component, including
        # its trees/cabinets. The independent bollard below stays whole.
        d.rectangle((1250,0,1535,210),fill=0)
        d.rectangle((1220,0,1249,172),fill=0)
    original.putalpha(mask)
    image.alpha_composite(original, (96, 3136))
    if repair_join:
        # Whole standalone plants are drawn AFTER the arrival component;
        # their upper halves can no longer be overwritten at the old seam.
        bush=Image.open(hedge).convert('RGBA')
        for x,y,turn in [(616,3088,None),(690,3104,Image.Transpose.ROTATE_90),
                (790,3082,Image.Transpose.FLIP_LEFT_RIGHT),(855,3100,Image.Transpose.ROTATE_270),
                (954,3094,None)]:
            image.alpha_composite(bush if turn is None else bush.transpose(turn),(x,y))
        complete=next(m for m in modules if m['id']=='west-garage')
        annex=Image.open(ROOT/complete['file']).convert('RGBA').resize((224,212),Image.Resampling.LANCZOS)
        image.alpha_composite(annex,(1352,3136))
    for b in read(ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r04/new-buildings.json'):
        image.alpha_composite(Image.open(ROOT / b['file']).convert('RGBA'), tuple(b['at_px']))
    # These are the established modular wall-atlas cells, not scene cutouts.
    walls = Image.open(ROOT / 'assets/maps/cryogenic-camp-01/structure.png').convert('RGBA')
    for y in range(64, 4000, 64):
        image.alpha_composite(walls.crop((868, 192, 900, 256)), (64, y))
        image.alpha_composite(walls.crop((1344, 192, 1376, 256)), (1600, y))
    for x in range(64, 1600, 64):
        if x not in (1152, 1216):
            image.alpha_composite(walls.crop((960, 88, 1024, 134)), (x, 64))
    for name, at in [('nw', (64, 64)), ('ne', (1568, 64)), ('sw', (64, 4000)), ('se', (1568, 4000))]:
        image.alpha_composite(Image.open(PARTS / f'wall-corner-{name}.png').convert('RGBA'), at)
    prior = ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r05'
    image.alpha_composite(Image.open(prior / 'north-gateway-native.png').convert('RGBA'), (1078, 0))
    for at in [(1040, 680), (1350, 1984), (1350, 3008)]:
        image.alpha_composite(Image.open(prior / 'street-post-native.png').convert('RGBA'), at)
    catalogue = read(PARTS / 'decor-catalogue.json')
    for p in read(PARTS / 'decor-placements.json'):
        if p['code'] in omit_decor:
            continue
        image.alpha_composite(Image.open(ROOT / catalogue[p['asset']]['file']).convert('RGBA'), tuple(p['at_px']))
    ImageDraw.Draw(image).rectangle((0, 4064, 1664, 4224), fill=(0, 0, 0, 255))
    return image


def build():
    ART.mkdir(parents=True, exist_ok=True)
    OUT.mkdir(parents=True, exist_ok=True)
    hashes = {p.relative_to(ACTIVE).as_posix(): sha(p) for p in ACTIVE.rglob('*') if p.is_file()}
    baseline = compose()
    active = Image.open(ACTIVE / 'street.png').convert('RGBA')
    delta = ImageChops.difference(baseline, active)
    # After approval the active floor differs by design. Its retained prior
    # source remains the exact baseline for rebuilding this composition.
    approval = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-paving-review-06/acceptance.json'
    if approval.exists():
        prior = Image.open(ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r06/street-candidate.png').convert('RGBA')
        delta = ImageChops.difference(baseline, prior)
    assert delta.convert('RGB').getbbox() is None, ('Baseline reconstruction differs', delta.convert('RGB').getbbox())
    # A few irregular clusters near walls/curbs. No grid-based plant population.
    grass = [(116, 531), (123, 544), (819, 554), (827, 559), (1118, 1174),
             (1122, 1185), (1031, 2054), (1037, 2060), (100, 2298),
             (106, 2312), (598, 2700), (1583, 2607), (1578, 2621),
             (837, 3005), (844, 3014), (1081, 3069)]
    tiles = [Image.open(ART / f'{name}-native.png').convert('RGBA')
             for name in ['PAV-U01-06', 'PAV-U01-06B', 'PAV-U01-06C']]
    # Rare intact joints, mostly interrupted or nearly erased ones. The four
    # square orientations diversify the boundary patterns without resampling.
    mix = [tiles[0]] + [tiles[1]] * 6 + [tiles[2]] * 5
    candidate = compose(mix, grass)
    candidate.save(ART / 'street-candidate.png')
    report = dict(review='PAV-U01-06', status='candidate_not_integrated', native_cell=64,
                  world_pixels=list(active.size), baseline_reconstruction_identical=True,
                  revision='joints-irreguliers',
                  changes=['one slab per cell; no internal joints', 'mostly faint, interrupted or erased joints with static coordinate-based variation', 'grass removed from repeated tile; sparse independent tufts'],
                  joint_mix=dict(intact=1,fragmented=6,nearly_erased=5),
                  joint_rotation_degrees=[0,90,180,270],
                  joint_selection='offline coordinate hash; fixed across frames and visits; no gameplay RNG',
                  grass_at_px=[list(p) for p in grass], grass_native_size=[12, 10],
                  accepted_surface_style='STYLE-MAP-U01-04',
                  active_assets_unchanged=True, active_sha256=hashes,
                  candidate_sha256=sha(ART / 'street-candidate.png'),
                  preserved=['module geometry', 'south arrival', 'roads', 'doors', 'roofs', 'lighting', 'collisions', 'actors'])
    assert hashes == {p.relative_to(ACTIVE).as_posix(): sha(p) for p in ACTIVE.rglob('*') if p.is_file()}
    (ART / 'composition-checks.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({k: v for k, v in report.items() if k != 'active_sha256'}, ensure_ascii=False))


if __name__ == '__main__':
    build()
