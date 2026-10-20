"""Condition reviewed source panels and build an isolated, bounded U01 candidate.

No writes to the active map, player saves or map-editor work. Native cells stay64.
"""
from collections import deque
import copy
import hashlib
import json
import math
from pathlib import Path
import shutil

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-expansion-review-01'
OUT = ROOT / 'artifacts/street-expansion-review-01'
BASE = ROOT / 'assets/maps/prison-street-01'
W, H = 48, 36
OFFSET = (160, 128)
DOC.mkdir(parents=True, exist_ok=True)
OUT.mkdir(parents=True, exist_ok=True)


def relative(p):
    return p.relative_to(ROOT).as_posix()


def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def px(p):
    return [p[0] + OFFSET[0], p[1] + OFFSET[1]]


def cell(p):
    return [p[0] + 1, p[1] + 1]


def floor_from_rect(floor, box):
    a, b, c, d = box
    for y in range(2, H - 2):
        for x in range(2, W - 2):
            sx, sy = x * 64 + 32 - OFFSET[0], y * 64 + 32 - OFFSET[1]
            if a <= sx < c and b <= sy < d:
                floor.add((x, y))


def build():
    old = json.loads((BASE / 'street.json').read_text(encoding='utf-8'))
    before = {p.name: digest(p) for p in BASE.iterdir() if p.is_file()}
    im = Image.new('RGBA', (W * 64, H * 64), '#101719')
    master = Image.open(ART / 'SCENE-U01-08-r01.png').convert('RGBA')
    im.alpha_composite(master, OFFSET)
    # Reuse only approved, already-native camp wall pixels; no new painted wall.
    walls = Image.open(ROOT / 'assets/maps/cryogenic-camp-01/structure.png').convert('RGBA')
    horizontal = walls.crop((960, 320, 1024, 366))
    left = walls.crop((868, 192, 900, 256))
    right = walls.crop((1360, 192, 1392, 256))
    post = walls.crop((868, 64, 932, 128))
    asphalt = master.crop((1056, 400, 1120, 464))
    # The only perimeter opening is the northern continuation of the east road.
    for x in range(1, W - 1):
        if x not in (39, 40):
            im.alpha_composite(horizontal, (x * 64, 64))
        im.alpha_composite(horizontal, (x * 64, (H - 2) * 64))
    for y in range(1, H - 1):
        im.alpha_composite(left, (64, y * 64))
        im.alpha_composite(right, ((W - 2) * 64, y * 64))
    for x, y in [(1, 1), (W - 2, 1), (1, H - 2), (W - 2, H - 2)]:
        im.alpha_composite(post, (x * 64, y * 64))
    for x in (39, 40):
        for y in (0, 1):
            im.alpha_composite(asphalt, (x * 64, y * 64))
    im.save(ART / 'street-candidate.png')

    floor = {(x + 1, y + 1) for y, row in enumerate((BASE / 'layout.txt').read_text().splitlines())
             for x, ch in enumerate(row) if ch == '.'}
    for rect in [
        (2300, 0, 2430, 1024), (1580, 560, 2420, 725),
        (1650, 315, 2240, 535), (1590, 210, 2240, 280),
        (1620, 260, 1690, 620), (2170, 255, 2245, 620),
        (0, 950, 2752, 1000), (0, 1030, 2752, 1240),
        (1340, 1200, 1540, 1870), (0, 1760, 1340, 1850),
        (560, 1230, 670, 1820), (1240, 1240, 1330, 1830),
        (1550, 1760, 2700, 1880), (1560, 1280, 1650, 1820),
        (1810, 1440, 1890, 1860),
    ]:
        floor_from_rect(floor, rect)
    # New interiors and deliberately clear approaches. Furniture is conservative.
    rooms = [dict(id='ROOF-U01-02A', at=[704, 1392], size=[544, 282],
                  door=[17, 28], inside=[18, 26], cells=[[x, y] for y in (25, 26, 27) for x in range(15, 19)],
                  floor_source=[1040, 1600, 64, 48], opening_at=[928, 1672], lamp_at=[1068, 1690]),
             dict(id='ROOF-U01-02B', at=[1904, 1472], size=[512, 292],
                  door=[36, 29], inside=[36, 27], cells=[[x, y] for y in (26, 27, 28) for x in range(34, 38)],
                  floor_source=[2144, 1640, 64, 48], opening_at=[2144, 1760], lamp_at=[2250, 1780])]
    for room in rooms:
        floor.update(map(tuple, room['cells']))
        x, y = room['door']
        floor.update([(x, y), (x, y + 1), (x, y + 2)])
    floor.update((x, y) for x in (39, 40) for y in range(3))
    # No cells on the remaining edges. Buildings and the continuous wall close it.
    floor = {(x, y) for x, y in floor if 2 <= x < W - 2 and 2 <= y < H - 2
             or x in (39, 40) and y in (0, 1)}
    obstacles = {tuple(cell(p)) for p in old['obstructions']}
    start = tuple(cell(old['entrance']))
    connected = {start}
    todo = deque([start])
    while todo:
        x, y = todo.popleft()
        for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
            if p in floor - obstacles and p not in connected:
                connected.add(p)
                todo.append(p)
    # Reject disconnected drawing fragments rather than silently adding bridges.
    missing = [p for p in [(39, 0), (40, 0), *(tuple(r['inside']) for r in rooms)] if p not in connected]
    assert not missing, ('Unreachable intended location', missing)
    removed = len(floor - obstacles - connected)
    floor = connected | obstacles
    layout = '\n'.join(''.join('.' if (x, y) in floor else '#' for x in range(W)) for y in range(H)) + '\n'
    (ART / 'layout.txt').write_text(layout, encoding='utf-8')
    border = {(x, y) for x, y in floor if x in (0, W - 1) or y in (0, H - 1)}
    assert border == {(39, 0), (40, 0)}, border

    scene = copy.deepcopy(old)
    scene.update(id='MAP-U01-EXP-01-r01', width=W, height=H, image_origin=[0, 0],
                 image_offset_px=[0, 0], image_size=[W, H], background=relative(ART / 'street-candidate.png'),
                 obstructions=[list(p) for p in sorted(obstacles)], next_exit=[39, 0],
                 next_zone='U02', walkable_cells=len(connected), layout_sha256=hashlib.sha256(layout.encode()).hexdigest())
    for key in ('entrance', 'home_entry', 'home_inside', 'car_at'):
        scene[key] = cell(old[key])
    doorway = scene['doorway']
    for key in ('opening_at', 'lamp_at', 'closed_at'):
        doorway[key] = px(doorway[key])
    for key in ('floor_source', 'lamp_source', 'closed_source'):
        doorway[key][:2] = px(doorway[key][:2])
    for roof in scene['roofs']:
        roof['at_px'] = px(roof['at_px'])
        roof['interior'] = [cell(p) for p in roof['interior']]
        roof['exterior'] = [cell(p) for p in roof['exterior']]
        if roof['door']:
            roof['door'] = cell(roof['door'])
    source_dir = Path('C:/Users/User/.codex/generated_images/01a145b1-fd86-78f3-bee1-e0d66af002bd')
    scene['extra_doorways'] = []
    for room, generated in zip(rooms, ['exec-d0b5effd-85ac-46bf-a01e-ccc3329fb3e3.png', 'exec-35fb9d53-e808-48b6-989e-69ad5b8eda22.png']):
        original = ART / (room['id'] + '-source.png')
        if not original.exists():
            shutil.copyfile(source_dir / generated, original)
        full_height = 360 if room['id'].endswith('A') else 368
        roof = Image.open(original).convert('RGBA').resize((room['size'][0], full_height), Image.Resampling.LANCZOS)
        roof = roof.crop((0, 0, *room['size']))
        roof_path = ART / (room['id'] + '-native.png')
        roof.save(roof_path)
        at = px(room['at'])
        exterior = [[x, y] for y in range(H) for x in range(W)
                    if at[0] <= x * 64 + 32 < at[0] + roof.width and at[1] <= y * 64 + 32 < at[1] + roof.height
                    and [x, y] not in room['cells']]
        scene['roofs'].append(dict(id=room['id'], file=relative(roof_path), at_px=at,
                                   door=room['door'], interior=room['cells'], exterior=exterior))
        scene['extra_doorways'].append(dict(door=room['door'], floor_source=[*px(room['floor_source'][:2]), *room['floor_source'][2:]],
            opening_at=px(room['opening_at']), lamp_source=scene['doorway']['lamp_source'], lamp_at=px(room['lamp_at']),
            closed_source=scene['doorway']['closed_source'], closed_at=px(room['opening_at'])))

    config = copy.deepcopy(old['atmosphere'])
    lights = config['lights']
    for light in lights:
        light['at'] = [v + 64 for v in light['at']]
        light['origin'] = cell(light['origin'])
        if light['box']:
            light['box'] = [*px(light['box'][:2]), *px(light['box'][2:])]
        light['color'] = [1., .67, .28]
        light['lens_color'] = [1., .82, .46]
    red = [1., .055, .025]
    for light in lights:
        if light['id'] in ('shop-door', 'garage', 'ruin-west', 'ruin-east'):
            light.update(color=red, lens_color=red)
    # Generated housings: a mixture of posts, round wall lamps and existing strips.
    new_lights = [
        ('east-closed', (1830, 165), (32, 5), (1810, 159, 1847, 169), True),
        ('east-home-closed', (2568, 265), (42, 7), (2553, 260, 2581, 271), True),
        ('east-south-closed', (2629, 704), (43, 14), (2610, 698, 2650, 709), True),
        ('south-west-closed', (255, 686 + 1024), (6, 30), (232, 682 + 1024, 276, 693 + 1024), True),
        ('south-east-closed', (2645, 700 + 1024), (43, 30), (2620, 693 + 1024, 2670, 705 + 1024), True),
        ('post-west', (260, 153 + 1024), (7, 22), (252, 142 + 1024, 267, 159 + 1024), False),
        ('post-centre', (1269, 164 + 1024), (21, 22), (1261, 154 + 1024, 1277, 172 + 1024), False),
        ('post-east', (1590, 164 + 1024), (27, 22), (1582, 154 + 1024, 1598, 172 + 1024), False),
        ('post-far-east', (2415, 164 + 1024), (41, 22), (2408, 154 + 1024, 2423, 172 + 1024), False),
    ]
    for name, at, origin, box, closed in new_lights:
        if tuple(origin) not in floor:
            # The emitter must originate in its actual approach, never inside a wall.
            origin = min(connected, key=lambda p: (p[0] * 64 + 32 - (at[0] + OFFSET[0])) ** 2 + (p[1] * 64 + 32 - (at[1] + OFFSET[1])) ** 2)
        lights.append(dict(id=name, at=px(at), origin=origin, box=[*px(box[:2]), *px(box[2:])],
                           radius=205 if closed else 260, power=.8, color=red if closed else [1., .72, .38],
                           lens_color=red if closed else [1., .82, .46]))
    for room in rooms:
        lamp = px(room['lamp_at'])
        at = [lamp[0] + 25, lamp[1] + 9]
        lights.append(dict(id=room['id'] + '-door', at=at, origin=[room['door'][0], room['door'][1] + 1],
                           box=None, radius=195, power=.8, color=[1., .67, .28], lens_color=[1., .82, .46]))
        lights.append(dict(id=room['id'] + '-inside', at=[room['inside'][0] * 64 + 32, room['inside'][1] * 64 + 32],
                           origin=room['inside'], box=None, radius=220, power=.7, color=[1., .67, .28], lens_color=[1., .82, .46]))
    build_lightmaps(im, floor, config, lights)
    config.update(correction=relative(ART / 'correction.png'), irradiance=relative(ART / 'irradiance.png'),
                  colored_emitters=True, fan=[(1448 + OFFSET[0]) / im.width, (348 + OFFSET[1]) / im.height, 16 / im.width, 16 / im.height])
    scene['atmosphere'] = config
    scene['ambience'] = dict(steam=[*px([746, 418])], embers=True)
    scene['candidate'] = True
    (ART / 'street.json').write_text(json.dumps(scene, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    assert before == {p.name: digest(p) for p in BASE.iterdir() if p.is_file()}, 'Active map changed'
    report = dict(review='MAP-U01-EXP-01-r01 / LUM-U01-08 / AMB-U01-08', status='candidate_not_integrated',
                  date='2026-10-19', old_dimensions=[26, 18], dimensions=[W, H], native_cell=64,
                  area_ratio=W * H / (26 * 18), old_walkable_cells=old['walkable_cells'], walkable_cells=len(connected),
                  connected=True, inaccessible_fragments_removed=removed, perimeter_openings=sorted(map(list, border)),
                  next_exit=[39, 0], next_zone='U02', next_zone_not_generated=True,
                  entrance=scene['entrance'], visitable_buildings=3, roofs_and_door_rules='Door open or player inside hides the roof',
                  yellow='Openable, including when the door is closed', red='Permanently inaccessible',
                  animations=['slow fan', 'brief drain steam', 'car embers'], active_map_unchanged=True,
                  sources={relative(p): digest(p) for p in ART.glob('*.png')})
    (DOC / 'candidate-r01.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    nav = im.copy()
    d = ImageDraw.Draw(nav, 'RGBA')
    for x, y in floor:
        d.rectangle((x * 64, y * 64, (x + 1) * 64 - 1, (y + 1) * 64 - 1), outline=(65, 210, 170, 180))
    nav.save(OUT / 'navigation-native.png')
    print(json.dumps({k: v for k, v in report.items() if k != 'sources'}, ensure_ascii=False))


def build_lightmaps(ink, floor, config, lights):
    # Same wall-occlusion and first-wall receivers as the existing camp/street.
    import numpy as np
    scale = 8
    # Later reviews reuse this helper with different authored map dimensions.
    yy, xx = np.mgrid[0:ink.height:scale, 0:ink.width:scale].astype(float)
    xx += scale / 2
    yy += scale / 2
    rgb = np.broadcast_to(np.array(config['ambient']), (*xx.shape, 3)).copy()
    def visible(light, tx, ty):
        ox, oy = light['origin']
        sx = min(max(light['at'][0], ox * 64 + 1), ox * 64 + 63)
        sy = min(max(light['at'][1], oy * 64 + 1), oy * 64 + 63)
        gx, gy = int(tx // 64), int(ty // 64)
        steps = max(1, math.ceil(math.hypot(tx - sx, ty - sy) / 8))
        for i in range(1, steps + 1):
            x, y = int((sx + (tx - sx) * i / steps) // 64), int((sy + (ty - sy) * i / steps) // 64)
            if (x, y) != (gx, gy) and (x, y) not in floor:
                return False
        return True
    for light in lights:
        radius = light['radius']
        nearby = {(x, y) for x, y in floor if (x * 64 + 32 - light['at'][0]) ** 2 + (y * 64 + 32 - light['at'][1]) ** 2 < (radius + 64) ** 2
                  and visible(light, x * 64 + 32, y * 64 + 32)}
        faces = {(x + dx, y + dy) for x, y in nearby for dx, dy in [(0, -1), (1, 0), (0, 1), (-1, 0)]}
        d2 = (xx - light['at'][0]) ** 2 + (yy - light['at'][1]) ** 2
        mask = d2 < radius ** 2
        ys, xs = np.nonzero(mask)
        for y, x in zip(ys, xs):
            p = (int(xx[y, x] // 64), int(yy[y, x] // 64))
            seen = visible(light, xx[y, x], yy[y, x]) if p in floor else p in faces
            if seen:
                k = (math.exp(-3 * d2[y, x] / radius ** 2) - math.exp(-3)) / (1 - math.exp(-3)) * light['power']
                rgb[y, x] += np.array(light['color']) * k
    Image.fromarray(np.clip(np.rint(rgb * 127.5), 0, 255).astype('uint8')).save(ART / 'irradiance.png')
    correction = Image.new('RGBA', ink.size, (128, 128, 128, 0))
    for light in lights:
        if not light['box']:
            continue
        a, b, c, d = light['box']
        tint = tuple(round(v * 255) for v in light['lens_color']) + (255,)
        for y in range(b, d + 1):
            for x in range(a, c + 1):
                r, g, bl, _ = ink.getpixel((x, y))
                if r > 160 and g > 145 and bl > 80:
                    correction.putpixel((x, y), tint)
    correction.save(ART / 'correction.png')


if __name__ == '__main__':
    build()
