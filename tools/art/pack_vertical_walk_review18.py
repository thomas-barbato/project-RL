"""Condition complete authored figures; no procedural limb animation or repainting."""
from pathlib import Path
import hashlib
import json
import statistics
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/vertical-walk-review-18'
OUT = ART / 'candidate-r01'
PREVIEW = ROOT / 'artifacts/vertical-walk-review-18/current'
PROFILES = [
    ('arcaniste', 'arcaniste-r01-source.png',
     'assets/source-art/player-walk-review-11/CLASS-15-front-native128.png'),
    ('npc-garde', 'npc-garde-r02-source.png',
     'assets/maps/cryogenic-camp-01/actors/NPC-01-r03-native64.png'),
]


def opaque_box(image):
    return image.getchannel('A').point(lambda a: 255 if a >= 32 else 0).getbbox()


def figures(image):
    alpha = image.getchannel('A').point(lambda a: 255 if a >= 32 else 0)
    groups = []
    for y in range(image.height):
        if sum(alpha.crop((0, y, image.width, y + 1)).tobytes()) // 255 >= 4:
            if not groups or y - groups[-1][-1] > 10:
                groups.append([])
            groups[-1].append(y)
    assert len(groups) == 4, f'Expected four separate authored rows, got {len(groups)}'
    edges = [0] + [(a[-1] + b[0]) // 2 for a, b in zip(groups, groups[1:])] + [image.height]
    result = []
    for row in range(4):
        for col in range(4):
            cell = image.crop((round(col * image.width / 4), edges[row],
                               round((col + 1) * image.width / 4), edges[row + 1]))
            box = opaque_box(cell)
            assert box, (row, col)
            assert box[0] > 1 and box[2] < cell.width - 1, (row, col, 'horizontal crop')
            assert box[1] > 1 and box[3] < cell.height - 1, (row, col, 'vertical crop')
            result.append(cell.crop((max(0, box[0]-2), max(0, box[1]-2),
                                     min(cell.width, box[2]+2), min(cell.height, box[3]+2))))
    return result


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    PREVIEW.mkdir(parents=True, exist_ok=True)
    profiles = []
    font = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Regular.ttf'), 14)
    board = Image.new('RGB', (960, 620), '#1e2831')
    draw = ImageDraw.Draw(board)
    for row, (identity, name, original) in enumerate(PROFILES):
        source = ART / name
        whole = Image.open(source).convert('RGBA')
        cells = figures(whole)
        old = Image.open(ROOT / original).convert('RGBA')
        original_box = opaque_box(old)
        height = original_box[3] - original_box[1]
        scale = height / statistics.median(opaque_box(im)[3]-opaque_box(im)[1] for im in cells)
        native = []
        for im in cells:
            reduced = im.resize((round(im.width*scale), round(im.height*scale)), Image.Resampling.LANCZOS)
            bounds = opaque_box(reduced)
            canvas = Image.new('RGBA', (128, 128))
            canvas.alpha_composite(reduced, (round(64-(bounds[0]+bounds[2])/2), original_box[3]-bounds[3]))
            native.append(canvas)
        sheet = Image.new('RGBA', (6*128, 2*128))
        records = []
        for direction, rows in [('front', (0, 1)), ('back', (2, 3))]:
            for phase, n in enumerate([rows[0]*4+1, rows[0]*4+2, rows[0]*4+3,
                                       rows[1]*4+1, rows[1]*4+2, rows[1]*4+3]):
                im = native[n]
                sheet.paste(im, (phase*128, (direction == 'back')*128))
                b = opaque_box(im)
                assert b[0] >= 30 and b[2] <= 98 and b[1] >= 30 and b[3] <= 98
                palms = []
                for x, y in [(51, 66 if identity == 'arcaniste' else 71),
                             (77, 67 if identity == 'arcaniste' else 71)]:
                    points = [(xx, yy, im.getpixel((xx, yy))[3])
                              for yy in range(y-2, y+3) for xx in range(x-2, x+3)
                              if im.getpixel((xx, yy))[3] >= 128
                              and max(im.getpixel((xx, yy))[:3]) < 100
                              and im.getpixel((xx, yy))[0]-im.getpixel((xx, yy))[1] < 30]
                    assert points, (identity, direction, phase, 'hand landmark')
                    weight = sum(p[2] for p in points)
                    palms.append([round(sum(p[k]*p[2] for p in points)/weight, 2) for k in [0, 1]])
                records.append({'direction': direction, 'phase': phase, 'box': list(b), 'palms': palms})
        path = OUT / (identity + '.png')
        sheet.save(path)
        original_path = OUT / (identity + '-standing-original.png')
        old.save(original_path)
        selector = ({'kind': 'player', 'body': 6, 'equipment': [None]*4}
                    if identity == 'arcaniste' else {'kind': 'camp_npc', 'index': 1})
        profiles.append({'id': identity, 'selector': selector, 'file': path.relative_to(ROOT).as_posix(),
                         'standing': original_path.relative_to(ROOT).as_posix(),
                         'source': source.relative_to(ROOT).as_posix(),
                         'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                         'source_scale_xy_uniform': scale, 'original_height_px': height,
                         'original_foot_y': original_box[3], 'frames': records})
        for direction in range(2):
            y = row*300 + direction*144
            draw.text((8, y+4), identity + (' / face' if direction == 0 else ' / dos'), font=font, fill='white')
            for phase in range(6):
                im = sheet.crop((phase*128, direction*128, (phase+1)*128, (direction+1)*128))
                board.paste(im, (150+phase*128, y+16), im)
    board.save(PREVIEW / 'contact-native64.png')
    (OUT / 'manifest.json').write_text(json.dumps({'review': 'VERT18-C01', 'status': 'review_only',
        'native_tile_px': 64, 'canvas_px': 128, 'normal_game_enabled': False,
        'whole_authored_figures_only': True, 'profiles': profiles}, ensure_ascii=False, indent=2)+'\n', encoding='utf8')
    print(json.dumps({'profiles': len(profiles), 'authored_moving_figures': 24,
                      'manifest': str(OUT / 'manifest.json')}))


if __name__ == '__main__':
    main()
