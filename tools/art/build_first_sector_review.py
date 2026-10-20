"""Build FIRST-SECTOR-01 proposal only. Never writes campaign/editor data."""
from pathlib import Path
from collections import deque
import hashlib
import json
import shutil
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/first-sector-review-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/first-sector-review-01'
ART = ROOT / 'assets/source-art/first-sector-review-01'
TILE_NAMES = [
    'Dalles de béton', 'Plaques de service', 'Grille métallique', 'Seuil dégagé',
    'Mur horizontal nord', 'Mur horizontal sud', 'Mur vertical ouest', 'Mur vertical est',
    'Angle nord-ouest', 'Angle nord-est', 'Angle sud-ouest', 'Angle sud-est',
    'Jonction T · branche sud', 'Jonction T · branche est',
    'Jonction T · branche nord', 'Jonction T · branche ouest',
]
W, H, CELL = 44, 30, 64
OUT.mkdir(parents=True, exist_ok=True)
DOC.mkdir(parents=True, exist_ok=True)
shutil.copyfile(ROOT / 'tools/art/templates/first_sector_review.html', OUT / 'index.html')
decor_manifest = json.loads((DOC / 'decor-manifest.json').read_text(encoding='utf-8'))
for item, name in zip(decor_manifest['items'], TILE_NAMES):
    item['name'] = name
(DOC / 'decor-manifest.json').write_text(
    json.dumps(decor_manifest, ensure_ascii=False, indent=2)+'\n', encoding='utf-8'
)
floor = set()


def cells(rect):
    x, y, w, h = rect
    return {(a, b) for a in range(x, x+w) for b in range(y, y+h)}


modules = [
    {'code': 'M01', 'name': 'Arrivée du camp', 'rect': [2, 11, 9, 9], 'focus': [6, 15]},
    {'code': 'M02', 'name': 'Jonction', 'rect': [16, 9, 12, 13], 'focus': [21, 15]},
    {'code': 'M03', 'name': 'Maintenance · détour', 'rect': [11, 2, 13, 5], 'focus': [18, 5]},
    {'code': 'M04', 'name': 'Alcôve facultative', 'rect': [19, 25, 8, 3], 'focus': [22, 25]},
    {'code': 'M05', 'name': 'Poste de contrôle · passage principal', 'rect': [31, 11, 10, 11], 'focus': [35, 16]},
]
for m in modules:
    floor |= cells(m['rect'])
for rect in [(1, 14, 2, 3), (10, 14, 7, 3), (12, 6, 3, 9),
             (23, 4, 4, 3), (24, 6, 3, 5), (21, 21, 3, 5),
             (27, 15, 5, 3), (40, 15, 3, 3)]:
    floor |= cells(rect)
pillars = cells((20, 11, 2, 2)) | cells((23, 18, 2, 2)) | cells((35, 12, 3, 2))
props = [
    {'at': [3, 12], 'file': 'assets/maps/cryogenic-camp-01/DEC-CAMP-14-r05.png'},
    {'at': [14, 3], 'file': 'assets/maps/cryogenic-camp-01/DEC-CAMP-14-r05.png'},
    {'at': [20, 3], 'file': 'assets/maps/cryogenic-camp-01/DEC-CAMP-09-r05.png'},
    {'at': [25, 26], 'file': 'assets/maps/cryogenic-camp-01/DEC-CAMP-14-r05.png'},
]
walkable = floor - pillars - {tuple(p['at']) for p in props}
start, entrance, exit_at = (4, 15), (1, 15), (41, 16)


def path(start_at, goal, allowed):
    todo = deque([start_at])
    previous = {start_at: None}
    while todo:
        p = todo.popleft()
        if p == goal:
            result = []
            while p is not None:
                result.append(list(p))
                p = previous[p]
            return result[::-1]
        for dx, dy in [(0, -1), (1, 0), (0, 1), (-1, 0)]:
            n = (p[0]+dx, p[1]+dy)
            if n in allowed and n not in previous:
                previous[n] = p
                todo.append(n)
    return []


route = path(start, exit_at, walkable)
assert route and path(start, entrance, walkable)
assert all(path(start, p, walkable) for p in walkable)
assert not path(start, exit_at, walkable - cells(modules[-1]['rect']))
assert path(start, exit_at, walkable - cells(modules[2]['rect']))
assert path(start, (18, 5), walkable - cells((12, 7, 3, 7)))
assert path(start, (18, 5), walkable - cells((24, 7, 3, 2)))
rows = [''.join('.' if (x, y) in walkable else '#' for x in range(W)) for y in range(H)]
layout = {
    'review': 'FIRST-SECTOR-01', 'layout': 'MAP-ACCESS-01-r01',
    'status': 'proposal_not_integrated', 'size': [W, H], 'cell_px': CELL,
    'start': start, 'entrance': entrance, 'exit': exit_at, 'modules': modules,
    'props': props, 'rows': rows, 'route': route,
    'ordinary_corridor_width': 3,
    'note': 'One representative review map, not the complete first sector. No quests, enemies, loot, boss, respawn or travel-cost rules added.',
}
(DOC / 'layout.json').write_text(json.dumps(layout, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
(DOC / 'layout.txt').write_text('\n'.join(rows)+'\n', encoding='utf-8')

# Review composition from the numbered, unapproved tile atlas. Wall strips are
# placed on the logical floor boundaries. No turned/mirrored wall lighting.
tiles = [Image.open(ART / f'DEC-A{i:02}-r01.png').convert('RGBA') for i in range(1, 17)]
native = Image.new('RGBA', (W*CELL, H*CELL), (12, 17, 20, 255))
for x, y in sorted(floor):
    steel = (31 <= x <= 40 and 11 <= y <= 21) or (11 <= x <= 23 and 2 <= y <= 6)
    tile = tiles[1 if steel else 0]
    if steel and (x*17+y*11) % 23 == 0:
        tile = tiles[2]
    native.alpha_composite(tile, (x*CELL, y*CELL))
dark = ImageDraw.Draw(native)
for x, y in pillars:
    dark.rectangle((x*CELL, y*CELL, (x+1)*CELL, (y+1)*CELL), fill=(20, 25, 28, 255))
surfaces = floor - pillars
horizontal, vertical, vertices = set(), set(), set()
for x, y in surfaces:
    if (x, y-1) not in surfaces:
        horizontal.add((x, y))
    if (x, y+1) not in surfaces:
        horizontal.add((x, y+1))
    if (x-1, y) not in surfaces:
        if not (x == 1 and 14 <= y <= 16):
            vertical.add((x, y))
    if (x+1, y) not in surfaces:
        if not (x+1 == 43 and 15 <= y <= 17):
            vertical.add((x+1, y))
for x, y in horizontal:
    vertices.update([(x, y), (x+1, y)])
for x, y in vertical:
    vertices.update([(x, y), (x, y+1)])
hwall_n = tiles[4].crop((0, 1, 64, 27))
hwall_s = tiles[5].crop((0, 34, 64, 64))
vwall_w = tiles[6].crop((1, 0, 21, 64))
vwall_e = tiles[7].crop((43, 0, 64, 64))
cap = tiles[4].crop((1, 1, 15, 15))
for x, y in sorted(horizontal):
    strip = hwall_n if (x, y) in surfaces else hwall_s
    native.alpha_composite(strip, (x*CELL, y*CELL-strip.height//2))
for x, y in sorted(vertical):
    strip = vwall_w if (x, y) in surfaces else vwall_e
    native.alpha_composite(strip, (x*CELL-strip.width//2, y*CELL))
for x, y in sorted(vertices):
    native.alpha_composite(cap, (x*CELL-7, y*CELL-7))
for p in props:
    im = Image.open(ROOT / p['file']).convert('RGBA')
    # One-cell prop uses the same 128px overflow canvas and (64,96) anchor
    # as the live camp renderer. Never shrink this canvas to one tile.
    assert im.size == (128, 128)
    native.alpha_composite(im, (p['at'][0]*CELL-32, p['at'][1]*CELL-32))
player_file = 'assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png'
player = Image.open(ROOT / player_file).convert('RGBA')
assert player.size == (128, 128)
# Approved 64px actor uses a 128px transparent overflow canvas, with foot
# anchor (64,96), exactly as the accepted camp manifest specifies.
native.alpha_composite(player, (start[0]*CELL-32, start[1]*CELL-32))
native.convert('RGB').save(OUT / 'map-native-r01.png')
native.crop((0, 576, 1280, 1344)).convert('RGB').save(OUT / 'arrival-native-r01.png')
native.crop((1536, 576, 2816, 1344)).convert('RGB').save(OUT / 'hall-native-r01.png')

# Diagram is a separate, deliberately reduced map; character art is never zoomed.
S, pad = 18, 28
plan = Image.new('RGB', (W*S+pad*2, H*S+pad*2), '#0e161b')
d = ImageDraw.Draw(plan)
for x, y in walkable:
    d.rectangle((pad+x*S, pad+y*S, pad+(x+1)*S-2, pad+(y+1)*S-2), fill='#34424a')
palette = ['#617683', '#637664', '#736b8b', '#867455', '#936454']
for m, color in zip(modules, palette):
    for x, y in cells(m['rect']) & walkable:
        d.rectangle((pad+x*S, pad+y*S, pad+(x+1)*S-2, pad+(y+1)*S-2), fill=color)
for a, b in zip(route, route[1:]):
    d.line((pad+(a[0]+.5)*S, pad+(a[1]+.5)*S, pad+(b[0]+.5)*S, pad+(b[1]+.5)*S), fill='#e5b66a', width=3)
for point, color in [(entrance, '#79ca9a'), (exit_at, '#e79f77')]:
    x, y = point
    d.ellipse((pad+x*S+2, pad+y*S+2, pad+(x+1)*S-3, pad+(y+1)*S-3), fill=color)
font = ImageFont.truetype('C:/Windows/Fonts/consola.ttf', 17)
for m in modules:
    x, y, w, h = m['rect']
    d.text((pad+(x+w/2)*S, pad+(y+h/2)*S), m['code'], anchor='mm', font=font, fill='#faf4e9', stroke_width=2, stroke_fill='#152027')
plan.save(OUT / 'layout-overview-r01.png')
checks = {
    'review': 'FIRST-SECTOR-01', 'status': 'proposal_only',
    'all_walkable_cells_reachable': len(walkable), 'start_to_exit_steps': len(route)-1,
    'return_to_camp_path': True, 'mandatory_module_cannot_be_bypassed': True,
    'optional_maintenance_loop_has_two_connections': True,
    'optional_detour_not_required_for_exit': True,
    'native_map_size': list(native.size), 'player_canvas': list(player.size),
    'player_source_scale': 1, 'player_foot_anchor': [64, 96],
    'player_visible_bounds': player.getchannel('A').getbbox(),
    'numbered_decor_tiles': 16, 'normal_game_changed': False,
    'map_editor_untouched': True,
    'limits': ['Fixed review layout, not the procedural generator.', 'No actors or rewards are spawned.', 'Image conditioning only; strips/caps in this composition are candidates too.', 'Lighting, collision/render overlap and integration require native engine verification after acceptance.'],
    'layout_sha256': hashlib.sha256((DOC / 'layout.json').read_bytes()).hexdigest(),
}
(DOC / 'layout-checks.json').write_text(json.dumps(checks, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
payload = {'layout': layout, 'checks': checks, 'decor': json.loads((DOC / 'decor-manifest.json').read_text(encoding='utf-8')), 'player': player_file}
(OUT / 'review-data.js').write_text('window.REVIEW='+json.dumps(payload, ensure_ascii=False)+';\n', encoding='utf-8')

board = Image.new('RGB', (1280, 570), '#111b20')
d = ImageDraw.Draw(board)
head = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 23)
body = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 15)
code = ImageFont.truetype('C:/Windows/Fonts/consola.ttf', 16)
d.text((24, 16), 'DEC-ACCESS-01 r01 · seize pièces candidates à 64 px', font=head, fill='#dec898')
d.text((24, 51), 'Sols, orientations, angles et jonctions en T · aucune modification du jeu', font=body, fill='#aabbc0')
for i, item in enumerate(decor_manifest['items']):
    x, y = 24+(i % 4)*310, 94+(i//4)*112
    d.rectangle((x, y, x+298, y+100), fill='#1a282e', outline='#3d515a')
    tile = tiles[i]
    board.paste(tile, (x+12, y+18), tile)
    d.text((x+90, y+18), item['code'], font=code, fill='#dec898')
    words = item['name'].replace('Jonction T ·', 'T ·').split()
    lines = ['']
    for word in words:
        if len(lines[-1])+len(word) > 22:
            lines.append(word)
        else:
            lines[-1] += (' ' if lines[-1] else '')+word
    for j, line in enumerate(lines):
        d.text((x+90, y+47+j*19), line, font=body, fill='#d5dddc')
board.save(OUT / 'decor-board-native-r01.png')
print(json.dumps(checks, ensure_ascii=False, indent=2))
