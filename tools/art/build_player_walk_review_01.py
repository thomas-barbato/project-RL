"""Package generated walk poses for isolated review; never touch runtime art."""
from __future__ import annotations

import argparse
import base64
import hashlib
import json
import math
import shutil
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'artifacts/player-walk-review-01'
SOURCE = ROOT / 'assets/source-art/player-walk-review-01'
DOC = ROOT / 'docs/reviews/player-walk-01'
REFERENCE = ROOT / 'assets/source-art/class-readability-01/bastion-copper-wave-01/accepted/native64/BA-CU01-P04-r01-native64.png'
VIS = Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/player-walk.html')
DIRECTIONS = ['S', 'SE', 'E', 'NE', 'N', 'NW', 'W', 'SW']


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def data_uri(path: Path) -> str:
    return 'data:image/png;base64,' + base64.b64encode(path.read_bytes()).decode('ascii')


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--sheet', type=Path, required=True)
    parser.add_argument('--idle-sheet', type=Path, default=SOURCE/'bastion-idle-eight-directions-source.png')
    args = parser.parse_args()
    for directory in (ART, SOURCE, DOC, VIS.parent):
        directory.mkdir(parents=True, exist_ok=True)
    original = SOURCE / 'bastion-walk-eight-directions-source.png'
    if args.sheet.resolve() != original.resolve():
        shutil.copy2(args.sheet, original)
    sheet = Image.open(original).convert('RGBA')
    if sheet.width % 6 or sheet.height % 8:
        raise ValueError(f'Expected exact 6 by 8 atlas, got {sheet.size}')
    cw, ch = sheet.width // 6, sheet.height // 8
    crops = []
    bounds = []
    for row in range(8):
        for col in range(6):
            cell = sheet.crop((col*cw, row*ch, (col+1)*cw, (row+1)*ch))
            box = cell.getchannel('A').point(lambda a: 255 if a >= 48 else 0).getbbox()
            if box is None:
                raise ValueError(f'Empty cell {row},{col}')
            crops.append(cell)
            bounds.append(box)
    scale = min(60 / max(b[3]-b[1] for b in bounds), 60 / max(b[2]-b[0] for b in bounds))
    atlas = Image.new('RGBA', (6*64, 8*64))
    entries = []
    for i, (cell, box) in enumerate(zip(crops, bounds)):
        cropped = cell.crop(box)
        native = cropped.resize((max(1, round(cropped.width*scale)), max(1, round(cropped.height*scale))), Image.Resampling.LANCZOS)
        frame = Image.new('RGBA', (64,64))
        # One global scale and one foot baseline across all 48 cells.
        frame.alpha_composite(native, (32-native.width//2, 63-native.height))
        atlas.alpha_composite(frame, ((i%6)*64, (i//6)*64))
        entries.append({'direction': DIRECTIONS[i//6], 'frame': i%6,
                        'source_box': list(box), 'native_bbox': list(frame.getbbox()),
                        'rgba_sha256': hashlib.sha256(frame.tobytes()).hexdigest()})
    atlas_path = ART / 'bastion-walk-native64.png'
    atlas.save(atlas_path, optimize=True)
    idle_original = SOURCE/'bastion-idle-eight-directions-source.png'
    if args.idle_sheet.resolve() != idle_original.resolve():
        shutil.copy2(args.idle_sheet, idle_original)
    idle_sheet = Image.open(idle_original).convert('RGBA')
    iw, ih = idle_sheet.width/4, idle_sheet.height/2
    idle_cells = [idle_sheet.crop(tuple(round(v) for v in (i%4*iw, i//4*ih, (i%4+1)*iw, (i//4+1)*ih))) for i in range(8)]
    idle_boxes = [im.getchannel('A').point(lambda a: 255 if a >= 48 else 0).getbbox() for im in idle_cells]
    assert all(idle_boxes)
    idle_scale = min(60/max(b[3]-b[1] for b in idle_boxes), 60/max(b[2]-b[0] for b in idle_boxes))
    idles = Image.new('RGBA', (4*64,2*64))
    for i,(im,box) in enumerate(zip(idle_cells,idle_boxes)):
        cropped = im.crop(box)
        native = cropped.resize((round(cropped.width*idle_scale),round(cropped.height*idle_scale)),Image.Resampling.LANCZOS)
        idles.alpha_composite(native,(i%4*64+32-native.width//2,i//4*64+63-native.height))
    idle_path = ART/'bastion-idle-native64.png'
    idles.save(idle_path, optimize=True)
    shutil.copy2(REFERENCE, ART/'bastion-front-native64.png')
    # A native contact sheet on both review grounds, reproducible and disposable.
    for name, color in [('dark', '#293132'), ('light', '#b5afa0')]:
        board = Image.new('RGB', atlas.size, color)
        board.paste(atlas, (0,0), atlas)
        board.save(ART/f'poses-native-{name}.png', optimize=True)
    assert len({entry['rgba_sha256'] for entry in entries}) == 48
    assert all(0 <= e['native_bbox'][0] < e['native_bbox'][2] <= 64 and
               0 <= e['native_bbox'][1] < e['native_bbox'][3] <= 64 for e in entries)
    payload = {'atlas': data_uri(atlas_path), 'front': data_uri(REFERENCE), 'idle': data_uri(idle_path)}
    template = (ROOT/'tools/art/player_walk_review_01.html').read_text('utf-8')
    fragment = template.replace('__WALK_DATA__', json.dumps(payload, separators=(',', ':')))
    assert '__WALK_DATA__' not in fragment
    assert len(fragment.encode('utf-8')) < 1_000_000
    VIS.write_text(fragment, encoding='utf-8')
    (ART/'player-walk-fragment.html').write_text(fragment, encoding='utf-8')
    report = {'review_id': 'PLAYER-WALK-01', 'status': 'awaiting_visual_review',
              'reference': str(REFERENCE.relative_to(ROOT)), 'reference_sha256': digest(REFERENCE),
              'source': str(original.relative_to(ROOT)), 'source_sha256': digest(original),
              'source_dimensions': list(sheet.size), 'native_atlas_dimensions': list(atlas.size),
              'idle_source_sha256':digest(idle_original),'idle_count':8,
              'directions': DIRECTIONS, 'frames_per_direction': 6, 'frame_count': 48,
              'uniform_resize_scale': scale, 'native_canvas': [64,64], 'baseline': 63,
              'fragment_bytes': len(fragment.encode('utf-8')), 'frames': entries,
              'limits': ['Unarmed demonstration only', 'Generated directional drawings are proposals',
                         'No runtime integration or engine capture', 'No multi-outfit compatibility claim']}
    (DOC/'packaging.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print(json.dumps({k:v for k,v in report.items() if k not in ['frames','limits']}, indent=2))
    print(VIS)


if __name__ == '__main__':
    main()
