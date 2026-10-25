"""Remove chairs, add medical cots, and condition two corrected doorway details."""
import copy
import json
from collections import deque
from pathlib import Path

from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'assets/prototypes/handcrafted-camp-01/r07'
OUT = ROOT / 'assets/prototypes/handcrafted-camp-01/r08'
SOURCE = ROOT / 'assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/r08'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def main():
    c = json.loads((BASE / 'camp.json').read_text(encoding='utf-8'))
    old = Image.open(ROOT / c['floor']).convert('RGBA')
    old.alpha_composite(Image.open(ROOT / c['structure']).convert('RGBA'))
    arch = old.copy()
    atlas = Image.open(SOURCE / 'doorways-edited-source-r08.png').convert('RGBA').resize((512, 256), Image.Resampling.LANCZOS)
    patches = []
    for i, x in enumerate([992, 1824]):
        crop = atlas.crop((i * 256 + 80, 80, i * 256 + 224, 144))
        # Feather the conditioning seam while keeping the original canvas
        # outside the two documented correction rectangles exactly intact.
        alpha = Image.new('L', crop.size)
        for y in range(crop.height):
            for xx in range(crop.width):
                edge = min(xx, y, crop.width - 1 - xx, crop.height - 1 - y)
                alpha.putpixel((xx, y), min(255, edge * 255 // 6))
        crop.putalpha(alpha)
        arch.alpha_composite(crop, (x + 80, 720))
        patches.append([x + 80, 720, x + 224, 784])
    outside = ImageChops.difference(arch, old)
    for box in patches:
        outside.paste((0, 0, 0, 0), box)
    assert outside.getbbox() is None
    rows = (ROOT / c['layout']).read_text(encoding='utf-8').splitlines()
    layers = [Image.new('RGBA', arch.size), Image.new('RGBA', arch.size)]
    for y, row in enumerate(rows):
        for x, tile in enumerate(row):
            box = x * 64, y * 64, (x + 1) * 64, (y + 1) * 64
            layers[int(tile == '#')].alpha_composite(arch.crop(box), box[:2])
    OUT.mkdir(parents=True, exist_ok=True)
    for layer, name in zip(layers, ['camp-floor-native.png', 'camp-structure-native.png']):
        layer.save(OUT / name, optimize=True)

    removed = [p for p in c['props'] if p['id'] == 'DEC-CAMP-02']
    c['props'] = [p for p in c['props'] if p['id'] != 'DEC-CAMP-02']
    bed = next(p for p in c['props'] if p['id'] == 'DEC-CAMP-10')
    for at in [[16, 13], [20, 13]]:
        new = copy.deepcopy(bed)
        new['at'] = at
        c['props'].append(new)
    c.update(revision='r08', floor='assets/prototypes/handcrafted-camp-01/r08/camp-floor-native.png',
             structure='assets/prototypes/handcrafted-camp-01/r08/camp-structure-native.png',
             chairs_removed=len(removed), medical_beds=3, modified_architecture_rects=patches,
             lighting_proposal='LUM-CAMP-01', new_lighting_system_implemented=False,
             new_ambient_animations_implemented=False,
             capture_targets=[dict(name='camp', at=[2, 8]), dict(name='hall', at=[22, 8]),
                              dict(name='infirmerie', at=[17, 13])])
    blocked = {(p['at'][0] + dx, p['at'][1] + dy) for p in c['props']
               for dx in range(p['width']) for dy in range(p['height'])}
    blocked |= {tuple(a['at']) for a in c['actors'][1:]}
    seen = {tuple(c['spawn'])}
    queue = deque(seen)
    while queue:
        x, y = queue.popleft()
        for nx, ny in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
            p = nx, ny
            if 0 <= nx < c['width'] and 0 <= ny < c['height'] and rows[ny][nx] == '.' and p not in blocked and p not in seen:
                seen.add(p)
                queue.append(p)
    for a in c['actors'][1:]:
        x, y = a['at']
        assert any(p in seen for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]), a['id']
    assert all(tuple(p) in seen for p in c['passages'] + c['exits'])
    assert (17, 13) in seen
    c.update(all_npcs_approachable=True, reachable_floor_cells=len(seen))
    data = json.dumps(c, ensure_ascii=False, indent=2) + '\n'
    (OUT / 'camp.json').write_text(data, encoding='utf-8')
    (REVIEW / 'full-camp-r08.json').write_text(data, encoding='utf-8')
    print(json.dumps(dict(chairs_removed=len(removed), medical_beds=3,
                          architecture_patches=patches, other_pixels_preserved=True, reachable=True)))


if __name__ == '__main__':
    main()
