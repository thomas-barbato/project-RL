"""Condition whole imagegen assets and source-authored modular layers for review.

Never writes a production atlas or changes content, equipment or combat rules.
Masks select existing painted pixels; no body, glove or weapon is painted here.
"""
from pathlib import Path
import colorsys
import hashlib
import json
import math
import sys
from PIL import Image, ImageDraw, ImageChops

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/art-finish-review-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01'
OUT = ROOT / 'artifacts/art-quality-audit-01'
sys.path.insert(0, str(ROOT / 'tools/art'))
SCALE = 8

NAMES = [
    ('FIN-01', 'Bastion', 'core:bastion', (44, 60)),
    ('FIN-02', 'Armure Cerbère', 'core:armure_cerbere', (44, 60)),
    ('FIN-03', 'Brigandine Cognefer', 'core:brigandine_cognefer', (40, 60)),
    ('FIN-04', 'Carapace composite', 'core:composite_carapace', (40, 60)),
    ('FIN-05', 'Veste matelassée', 'core:veste_matelassee', (40, 60)),
    ('FIN-06', 'Cultiste au fusil', '164 A', (38, 60)),
    ('FIN-07', 'Cultiste à la hache', '165 A', (40, 64)),
    ('FIN-08', 'Possédé', '005 A', (62, 60)),
    ('FIN-09', 'Écorcheur écarlate', '144 A', (118, 83)),
    ('FIN-10', 'Ailé d’obsidienne', '145 A', (112, 92)),
    ('FIN-11', 'Drone de soutien', 'agent:d57', (60, 42)),
    ('FIN-12', 'Fusil d’assaut', 'core:fusil_d_assaut', (60, 32)),
    ('FIN-13', 'Pistolet Cogneur P01', 'core:modele_cogneur_p01', (60, 51)),
]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def normalized(code, size, canvas=128, factor=1, foot=(64, 96)):
    im = Image.open(ART / f'{code}-source.png').convert('RGBA')
    box = im.getchannel('A').point(lambda a: 255 if a > 128 else 0).getbbox()
    assert box and min(box[:2]) > 0 and box[2] < im.width and box[3] < im.height, code
    body = im.crop(box)
    body.putalpha(body.getchannel('A').point(lambda a: 0 if a < 40 else a))
    ratio = min(size[0] / body.width, size[1] / body.height) * factor
    body = body.resize((round(body.width * ratio), round(body.height * ratio)), Image.Resampling.LANCZOS)
    result = Image.new('RGBA', (canvas * factor, canvas * factor))
    at = (foot[0] * factor - body.width // 2, foot[1] * factor - body.height)
    result.alpha_composite(body, at)
    assert result.getbbox()[0] > 0 and result.getbbox()[2] < result.width, code
    return result


def selected(source, polygons):
    mask = Image.new('L', source.size)
    draw = ImageDraw.Draw(mask)
    for poly in polygons:
        draw.polygon([(round(x * SCALE), round(y * SCALE)) for x, y in poly], fill=255)
    result = source.copy()
    result.putalpha(ImageChops.multiply(source.getchannel('A'), mask))
    return result, mask


def save_layer(image, file):
    image.resize((128, 128), Image.Resampling.LANCZOS).save(ART / 'native' / file)


def modular(code):
    """One stable core; new arm poses and fingers come from the second painting."""
    base = normalized(code, (44, 60), factor=SCALE)
    save_layer(base, f'{code}.png')
    alternate = normalized(code + '-RIFLE', (44, 60), factor=SCALE)
    # Both full source figures are registered to the same native feet/height.
    # The fixed head, central torso, belt and legs always come from base.
    arm_polys = [
        [(27, 46), (53, 46), (57, 51), (55, 57), (53, 69), (27, 69)],
        [(75, 46), (101, 46), (101, 69), (76, 69), (73, 57), (71, 51)],
    ]
    old_arms, old_mask = selected(base, arm_polys)
    core = base.copy()
    core.putalpha(ImageChops.multiply(base.getchannel('A'), ImageChops.invert(old_mask)))
    # Do not take the alternate central torso/head/legs when extracting sleeves.
    rifle_polys = [
        [(40, 46), (55, 46), (59, 53), (65, 57), (64, 68), (43, 68)],
        [(72, 46), (88, 46), (88, 69), (66, 69), (64, 59), (70, 53)],
    ]
    rifle_arms, _ = selected(alternate, rifle_polys)
    # Native hand anchors are measured from these source poses, not invented
    # colored glove shapes. A clear layer manifest retains every selection.
    anchors = {
        'FIN-01': {'single': (81, 61), 'rifle': [(59, 64), (73, 61)]},
        'FIN-02': {'single': (82, 63), 'rifle': [(58, 64), (74, 61)]},
    }[code]
    config = {}
    for pose, arms in [('single', old_arms), ('rifle', rifle_arms)]:
        posed = base.copy() if pose == 'single' else core.copy()
        if pose != 'single':
            posed.alpha_composite(arms)
        weapon_id = 'FIN-13' if pose == 'single' else 'FIN-12'
        weapon_source = Image.open(ART / f'{weapon_id}-source.png').convert('RGBA')
        box = weapon_source.getchannel('A').point(lambda a: 255 if a > 128 else 0).getbbox()
        weapon_source = weapon_source.crop(box)
        weapon_source.putalpha(weapon_source.getchannel('A').point(lambda a: 0 if a < 40 else a))
        # A real icon stays separate from the physically smaller held layer.
        length = 17 if pose == 'single' else 34
        weapon_source.thumbnail((length * SCALE, 18 * SCALE), Image.Resampling.LANCZOS)
        weapon = Image.new('RGBA', base.size)
        if pose == 'single':
            x, y = anchors['single']
            # Trigger hand is about one quarter along the pistol image.
            at = (round(x * SCALE - weapon_source.width * .24), round(y * SCALE - weapon_source.height * .64))
            fingers, _ = selected(posed, [[(x-2,y-2),(x+1,y-2),(x+2,y+2),(x-2,y+2)]])
        else:
            left, right = anchors['rifle']
            at = (round(left[0] * SCALE - weapon_source.width * .29), round(left[1] * SCALE - weapon_source.height * .60))
            fingers, _ = selected(posed, [[(x-2,y-2),(x+2,y-2),(x+2,y+2),(x-2,y+2)] for x,y in (left,right)])
        weapon.alpha_composite(weapon_source, at)
        filenames = [f'{code}-{pose}-{part}.png' for part in ['body', 'weapon', 'fingers']]
        for im, name in zip([posed, weapon, fingers], filenames):
            save_layer(im, name)
        composite = posed.copy()
        composite.alpha_composite(weapon)
        composite.alpha_composite(fingers)
        save_layer(composite, f'{code}-{pose}.png')
        config[pose] = dict(layers=filenames, anchor=anchors[pose], held_length=length)
    write(ART / f'{code}-layers.json', dict(review='ART-FINISH-01', code=code,
          fixed_core_source=f'{code}-source.png', arms_source=f'{code}-RIFLE-source.png',
          arm_selection=arm_polys, rifle_selection=rifle_polys, configurations=config,
          body_weapon_and_fingers_separate=True))
    still = Image.open(ART / 'native' / f'{code}.png')
    for pose in ['single', 'rifle']:
        other = Image.open(ART / 'native' / f'{code}-{pose}-body.png')
        for fixed in [(0,0,128,44),(0,72,128,128)]:
            assert ImageChops.difference(still.crop(fixed),other.crop(fixed)).getbbox() is None, (code,pose,fixed)
    return config


def palettes(code, number, old_hue):
    # Reuse the accepted authoring recipes, freeze NEW masks/maps for review.
    from prepare_monster_material_palettes import target
    from prepare_monster_blue_palettes import target as blue
    image = Image.open(ART / 'native' / f'{code}.png').convert('RGBA')
    membership = []
    for pixel in image.get_flattened_data():
        r, g, b, a = pixel
        h, l, s = colorsys.rgb_to_hls(r/255, g/255, b/255)
        hue = h * 360
        selected = bool(a and s > .12 and .075 < l < .85 and
                        ((hue < 30 or hue > 340) if old_hue == 'red' else 185 < hue < 265))
        membership.append(selected)
    records = []
    for label, kind in [('orange', 'orange'), ('green', 'green'), ('blue', 'blue')]:
        result = image.copy()
        def material_color(rgb):
            if kind == 'blue':
                return blue(rgb, False, number)
            if kind == 'orange':
                # Distinguish the proposed orange skin from the painted red A.
                _, light, _ = colorsys.rgb_to_hls(*(c/255 for c in rgb))
                color = colorsys.hls_to_rgb(24/360, light, .60)
                old = sum(c*w for c,w in zip(rgb,(.2126,.7152,.0722)))
                new = sum(c*255*w for c,w in zip(color,(.2126,.7152,.0722)))
                return [round(c*255*min(1,old/max(1,new))) for c in color]
            return target(rgb, kind, 'chair et membranes', number)
        result.putdata([tuple(material_color(p[:3])) + (p[3],)
                        if marked else p for p, marked in zip(image.get_flattened_data(), membership)])
        name = f'{code}-{label}.png'
        result.save(ART / 'native' / name)
        protected = all(a == b for a,b,m in zip(image.get_flattened_data(), result.get_flattened_data(), membership) if not m)
        assert protected
        records.append(dict(file=name, material=label, protected_pixels_unchanged=True,
                            source_alpha_unchanged=image.getchannel('A').tobytes()==result.getchannel('A').tobytes()))
    return records


def build():
    if (DOC / 'acceptance.json').exists():
        raise SystemExit('ART-FINISH-01 is accepted: frozen exports must not be regenerated. Use integrate_art_finish_review.py to verify/package them.')
    DOC.mkdir(parents=True, exist_ok=True)
    (ART / 'native').mkdir(parents=True, exist_ok=True)
    baseline = DOC / 'production-before.json'
    if not baseline.exists():
        before = json.loads((ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-quality-audit-01/art-before-audit.json').read_text(encoding='utf-8'))
        for file in ['street.json', 'street.png', 'layout.txt', 'collision-repairs.json']:
            p = ROOT / 'assets/maps/prison-street-expanded-01' / file
            before[p.relative_to(ROOT).as_posix()] = sha(p)
        write(baseline, before)
    entries = []
    for code, label, identity, size in NAMES:
        if not (ART / f'{code}-source.png').exists():
            continue
        canvas, foot = (64, (32, 46)) if code in ['FIN-12', 'FIN-13'] else (128, (64, 96))
        tile = normalized(code, size, canvas=canvas, foot=foot)
        tile.save(ART / 'native' / f'{code}.png')
        entry = dict(code=code,label=label,identity=identity,size=list(tile.size),bounds=list(tile.getbbox()),
                     foot=list(foot),source_sha256=sha(ART / f'{code}-source.png'),
                     native_sha256=sha(ART / 'native' / f'{code}.png'),status='pending_visual_review')
        if code in ['FIN-01','FIN-02'] and (ART/f'{code}-RIFLE-source.png').exists():
            entry['configurations'] = modular(code)
            entry['native_sha256'] = sha(ART / 'native' / f'{code}.png')
            entry['bounds'] = list(Image.open(ART / 'native' / f'{code}.png').getbbox())
            entry['head_and_legs_identical_between_configurations'] = True
        if code in ['FIN-09','FIN-10']:
            entry['palettes'] = palettes(code,144 if code=='FIN-09' else 145,'red' if code=='FIN-09' else 'blue')
        entries.append(entry)
    write(DOC / 'review.json', dict(review='ART-FINISH-01',status='pending_visual_review',
          scope='Complete pilot covering all ART-QUALITY-01 recommendations, not the entire bestiary',
          ordinary_runtime_unchanged=True,entries=entries))
    for file, expected in json.loads(baseline.read_text(encoding='utf-8')).items():
        assert sha(ROOT / file) == expected, file
    print(json.dumps(dict(review='ART-FINISH-01',candidates=len(entries),runtime_unchanged=True)))


if __name__ == '__main__':
    build()
