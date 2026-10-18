"""Condition the new cryogenics module and append it to the accepted r05 camp.

Original imagegen sources stay intact. Existing camp pixels and native sprites
are translated only; new pods are conditioned separately from the architecture.
"""
import copy
import hashlib
import json
from collections import deque
from pathlib import Path

from PIL import Image, ImageChops
from prepare_camp_reference_r05 import binary, components

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/r06'
BASE = ROOT / 'assets/prototypes/handcrafted-camp-01/r05'
OUT = ROOT / 'assets/prototypes/handcrafted-camp-01/r06'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    c = copy.deepcopy(json.loads((BASE / 'camp.json').read_text(encoding='utf-8')))
    old = Image.open(BASE / 'camp-floor-native.png').convert('RGBA')
    old.alpha_composite(Image.open(BASE / 'camp-structure-native.png').convert('RGBA'))
    arch = Image.new('RGBA', (2432, 1152), old.getpixel((0, 0)))
    # Remove source padding and condition the generated module to game scale.
    room = Image.open(SOURCE / 'architecture-open-source-r06.png').convert('RGBA')
    room = room.crop((80, 115, 1322, 1054)).resize((640, 576), Image.Resampling.LANCZOS)
    arch.alpha_composite(room, (0, 244))
    arch.alpha_composite(old, (640, 0))
    assert ImageChops.difference(arch.crop((640, 0, 2432, 1152)), old).getbbox() is None

    base_rows = (BASE / 'camp-layout.txt').read_text(encoding='utf-8').splitlines()
    grid = [list('#' * 10 + row) for row in base_rows]
    for y in range(5, 11):
        for x in range(1, 7):
            grid[y][x] = '.'
    for x in range(7, 11):
        grid[8][x] = '.'
    (OUT / 'camp-layout.txt').write_text('\n'.join(''.join(r) for r in grid) + '\n', encoding='utf-8')
    layers = [Image.new('RGBA', arch.size), Image.new('RGBA', arch.size)]
    for y, row in enumerate(grid):
        for x, tile in enumerate(row):
            box = (x * 64, y * 64, (x + 1) * 64, (y + 1) * 64)
            layers[int(tile == '#')].alpha_composite(arch.crop(box), box[:2])
    for image, name in zip(layers, ['camp-floor-native.png', 'camp-structure-native.png']):
        image.save(OUT / name, optimize=True)

    sheet = binary(Image.open(SOURCE / 'pods-source-r06.png'))
    pods = []
    for i, (label, width) in enumerate([('Cuve fermée', 44), ('Cuve ouverte', 48)]):
        crop = sheet.crop((i * sheet.width // 2, 0, (i + 1) * sheet.width // 2, sheet.height))
        body = max(components(crop), key=len)
        mask = Image.new('L', crop.size)
        for p in body:
            mask.putpixel(p, 255)
        crop.putalpha(mask)
        crop = crop.crop(crop.getbbox()).resize((width, 96), Image.Resampling.LANCZOS)
        alpha = binary(crop).getchannel('A')
        crop = crop.convert('RGB').quantize(colors=32, dither=Image.Dither.NONE).convert('RGBA')
        crop.putalpha(alpha)
        canvas = Image.new('RGBA', (128, 128))
        canvas.alpha_composite(crop, (64 - width // 2, 0))
        code = f'DEC-CAMP-{15 + i:02}'
        path = OUT / f'{code}-r06.png'
        canvas.save(path, optimize=True)
        pods.append(dict(id=code, label=label, file=path.relative_to(ROOT).as_posix(),
                         native_size=[width, 96], foot_anchor=[64, 96], sha256=sha(path)))

    for a in c['actors']:
        a['at'][0] += 10
    c['actors'][0]['at'] = [2, 8]
    for p in c['props']:
        p['at'][0] += 10
    occupied = [[x, y] for y in [6, 10] for x in [2, 4, 6]]
    for at in occupied:
        c['props'].append(dict(id=pods[0]['id'], file=pods[0]['file'], at=at, width=1, height=1))
    c['props'].append(dict(id=pods[1]['id'], file=pods[1]['file'], at=[1, 8], width=1, height=1))
    c['prop_sources'].extend(pods)
    for at in c['passages']:
        at[0] += 10
    c['passages'].append([8, 8])
    c.update(revision='r06', width=38,
             floor='assets/prototypes/handcrafted-camp-01/r06/camp-floor-native.png',
             structure='assets/prototypes/handcrafted-camp-01/r06/camp-structure-native.png',
             layout='assets/prototypes/handcrafted-camp-01/r06/camp-layout.txt',
             exits=[[37, 8]], art_approved=False, layout_approved=False,
             accepted_base_revision='r05', accepted_base_pixel_offset=[640, 0],
             accepted_base_pixels_unchanged=True, cryo_room=[0, 244, 640, 576],
             closed_pods=occupied, open_pod=[1, 8], spawn=[2, 8],
             capture_targets=[dict(name='camp', at=[2, 8]), dict(name='hall', at=[22, 8]),
                              dict(name='missions', at=[17, 4])],
             new_pod_gameplay=False)
    blocked = {(p['at'][0] + dx, p['at'][1] + dy) for p in c['props']
               for dx in range(p['width']) for dy in range(p['height'])}
    blocked |= {tuple(a['at']) for a in c['actors'][1:]}
    seen = {tuple(c['spawn'])}
    queue = deque(seen)
    while queue:
        x, y = queue.popleft()
        for nx, ny in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
            p = (nx, ny)
            if 0 <= nx < 38 and 0 <= ny < 18 and grid[ny][nx] == '.' and p not in blocked and p not in seen:
                seen.add(p)
                queue.append(p)
    for a in c['actors'][1:]:
        x, y = a['at']
        assert any(p in seen for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]), a['id']
    assert all(tuple(p) in seen for p in c['exits'] + c['passages'])
    c.update(reachable_floor_cells=len(seen), all_npcs_approachable=True)
    content = json.dumps(c, ensure_ascii=False, indent=2) + '\n'
    (OUT / 'camp.json').write_text(content, encoding='utf-8')
    (REVIEW / 'full-camp-r06.json').write_text(content, encoding='utf-8')
    print(json.dumps(dict(size=list(arch.size), accepted_base_pixels_unchanged=True,
                          closed_pods=6, open_pods=1, spawn=c['spawn'], reachable=True)))


if __name__ == '__main__':
    main()
