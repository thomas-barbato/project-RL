"""Pack WHOLE generated walking figures for preview; never modify runtime art."""
from __future__ import annotations

import base64
import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont
from build_player_walk_review_04 import components

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'artifacts/player-walk-review-05'
DOC = ROOT / 'docs/reviews/player-walk-05'
SOURCE = ROOT / 'assets/source-art/player-walk-review-05'
VIS = Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/player-walk-directions.html')


def uri(path):
    return 'data:image/png;base64,' + base64.b64encode(path.read_bytes()).decode('ascii')


def whole_figures(image):
    """Assign complete connected silhouettes; rectangular cells may cut a muzzle."""
    parts = components(image)
    main = sorted(parts, key=lambda part: part['area'], reverse=True)[:6]
    assert len(main) == 6 and min(p['area'] for p in main) > 10000
    main = sorted(main, key=lambda p: p['center'][1])
    main = sorted(main[:3], key=lambda p: p['center'][0]) + sorted(main[3:], key=lambda p: p['center'][0])
    groups = [[part] for part in main]
    for part in parts:
        if any(part is p for p in main) or part['area'] < 4:
            continue
        index = min(range(6), key=lambda i: sum((part['center'][axis] - main[i]['center'][axis]) ** 2 for axis in (0, 1)))
        groups[index].append(part)
    result = []
    for group in groups:
        bounds = (max(0, min(p['box'][0] for p in group) - 3),
                  max(0, min(p['box'][1] for p in group) - 3),
                  min(image.width, max(p['box'][2] for p in group) + 3),
                  min(image.height, max(p['box'][3] for p in group) + 3))
        # Only crop/translate the WHOLE figure, never alter its internal pixels.
        crop = image.crop(bounds)
        assert crop.width < 512 and crop.height <= 512, bounds
        result.append((crop, bounds))
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--corrected-heads', action='store_true')
    args = parser.parse_args()
    revision = 'head02' if args.corrected_heads else 'r03'
    suffix = '-head02' if args.corrected_heads else ''
    ART.mkdir(parents=True, exist_ok=True)
    cells = []
    records = []
    for direction in ('left', 'right'):
        source = SOURCE / f'traqueuse-rifle-{direction}-{revision}.png'
        image = Image.open(source).convert('RGBA')
        assert image.size == (1536, 1024), image.size
        for frame, (cell, bounds) in enumerate(whole_figures(image)):
            cells.append(cell)
            records.append({'direction': direction, 'frame': frame, 'source_bounds': list(bounds)})
    # One shared scale for both orientations and every pose, as for Bastion01.
    scale = min(60 / max(cell.height for cell in cells),
                60 / max(cell.width for cell in cells))
    atlas = Image.new('RGBA', (384, 128))
    for index, (cell, record) in enumerate(zip(cells, records)):
        whole = cell
        whole = whole.resize((round(whole.width * scale), round(whole.height * scale)), Image.Resampling.LANCZOS)
        frame = Image.new('RGBA', (64, 64))
        frame.alpha_composite(whole, (32 - whole.width // 2, 63 - whole.height))
        # Preserve generated alpha and all equipment; no masks or limb edits.
        atlas.alpha_composite(frame, (index % 6 * 64, index // 6 * 64))
        record['native_bounds'] = list(frame.getbbox())
        record['sha256'] = hashlib.sha256(frame.tobytes()).hexdigest()
    assert len({r['sha256'] for r in records}) == 12
    atlas_path = ART / f'traqueuse-rifle-lateral{suffix}-native64.png'
    atlas.save(atlas_path, optimize=True)

    original = Image.open(ROOT / 'artifacts/player-walk-review-01/bastion-walk-native64.png').convert('RGBA')
    reference = Image.new('RGBA', (384, 128))
    reference.alpha_composite(original.crop((0, 7 * 64, 384, 8 * 64)), (0, 0))
    reference.alpha_composite(original.crop((0, 64, 384, 128)), (0, 64))
    reference_path = ART / 'bastion-approved-lateral-native64.png'
    reference.save(reference_path, optimize=True)

    font = ImageFont.truetype('C:/Windows/Fonts/arial.ttf', 14)
    for ground, color in [('dark', '#293339'), ('light', '#b9b5a8')]:
        board = Image.new('RGB', (512, 300), color)
        draw = ImageDraw.Draw(board)
        ink = '#f0ede5' if ground == 'dark' else '#252c30'
        for group, (name, sheet) in enumerate([('01 - Traqueuse / fusil', atlas), ('Bastion - reference', reference)]):
            yy = group * 150
            draw.text((8, yy + 5), name, font=font, fill=ink)
            for row, label in enumerate(['Gauche', 'Droite']):
                draw.text((8, yy + 42 + row * 64), label, font=font, fill=ink)
                strip = sheet.crop((0, row * 64, 384, (row + 1) * 64))
                board.paste(strip, (100, yy + 22 + row * 64), strip)
        board.save(ART / f'directions{suffix}-native-{ground}.png')

    payload = {'traqueuse': uri(atlas_path), 'bastion': uri(reference_path)}
    template = (ROOT / 'tools/art/player_walk_direction_review_05.html').read_text('utf-8')
    fragment = template.replace('__WALK05_DATA__', json.dumps(payload, separators=(',', ':')))
    assert '__WALK05_DATA__' not in fragment and len(fragment.encode('utf-8')) < 1_000_000
    VIS.write_text(fragment, encoding='utf-8')
    (ART / f'direction-preview{suffix}-fragment.html').write_text(fragment, encoding='utf-8')
    report = {
        'review_id': 'WALK05-HEAD02' if args.corrected_heads else 'WALK05-DIR01',
        'status': 'rejected_by_player',
        'rejection': 'Head correction does not fix the gait; user rejects WALK05-HEAD02' if args.corrected_heads else 'Does not reproduce approved Bastion gait; repeated leading leg and different presentation',
        'coverage': 'CLASS-10 / rifle / left and right only; remaining bank pending',
        'sources': [str((SOURCE / f'traqueuse-rifle-{d}-{revision}.png').relative_to(ROOT)) for d in ('left', 'right')],
        'method': 'whole figures; one common scale; common baseline; no limb manipulation',
        'scale': scale, 'baseline': 63, 'frames': records,
        'default_cell_duration_ms': 400, 'choices_ms': [120, 200, 400],
        'game_integration': False, 'vertical_movement_modified': False,
        'directions_manually_checked': ['all six left-facing', 'all six right-facing'],
        'limits': ['Gait quality still requires player review', 'No claim that the complete bank is ready'],
        'fragment_bytes': len(fragment.encode('utf-8')),
    }
    report_name = 'head-correction-packaging.json' if args.corrected_heads else 'direction-correction-packaging.json'
    (DOC / report_name).write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({k: v for k, v in report.items() if k != 'frames'}, indent=2))


if __name__ == '__main__':
    main()
