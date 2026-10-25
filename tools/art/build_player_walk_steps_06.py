"""Preview complete drawn walk poses AND idle poses with Bastion02's scheduler."""
from __future__ import annotations

import base64
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont
from build_player_walk_direction_review_05 import whole_figures

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'assets/source-art/player-walk-review-05/step03'
ART = ROOT / 'artifacts/player-walk-review-05'
DOC = ROOT / 'docs/reviews/player-walk-05'
VIS = Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/player-walk-directions.html')


def whole(path):
    image = Image.open(path).convert('RGBA')
    bounds = image.getchannel('A').point(lambda a: 255 if a >= 48 else 0).getbbox()
    assert bounds, path
    return image.crop(bounds)


def pack_group(crops):
    # Separately generated single-frame canvases and a multi-frame atlas have
    # different source resolutions. One uniform WHOLE-figure scale per group.
    scale = min(58 / max(im.height for im in crops), 58 / max(im.width for im in crops))
    result = []
    for image in crops:
        sprite = image.resize((round(image.width * scale), round(image.height * scale)), Image.Resampling.LANCZOS)
        tile = Image.new('RGBA', (64, 64))
        tile.alpha_composite(sprite, (32 - sprite.width // 2, 63 - sprite.height))
        result.append(tile)
    return result, scale


def uri(path):
    return 'data:image/png;base64,' + base64.b64encode(path.read_bytes()).decode('ascii')


def main():
    ART.mkdir(parents=True, exist_ok=True)
    left_sources = sorted(SOURCE.glob('left-0[0-5]-*.png'))
    assert len(left_sources) == 6
    left, left_scale = pack_group([whole(p) for p in left_sources])
    right_source = Image.open(SOURCE / 'right-six.png').convert('RGBA')
    right, right_scale = pack_group([whole_crop for whole_crop, _ in whole_figures(right_source)])
    idles, idle_scale = pack_group([whole(SOURCE / f'idle-{direction}.png') for direction in ('left', 'right')])
    # Same atlas layout as the approved Bastion. Only rows1/7 are populated;
    # the six unused rows are empty, never generated or displayed.
    atlas = Image.new('RGBA', (384, 512))
    idle = Image.new('RGBA', (256, 128))
    for row, frames in [(7, left), (1, right)]:
        for frame, tile in enumerate(frames):
            atlas.alpha_composite(tile, (frame * 64, row * 64))
    for row, tile in [(7, idles[0]), (1, idles[1])]:
        idle.alpha_composite(tile, (row % 4 * 64, row // 4 * 64))
    atlas_path = ART / 'traqueuse-step03-walk-native64.png'
    idle_path = ART / 'traqueuse-step03-idle-native64.png'
    atlas.save(atlas_path, optimize=True)
    idle.save(idle_path, optimize=True)
    hashes = [hashlib.sha256(tile.tobytes()).hexdigest() for tile in left + right]
    assert len(set(hashes)) == 12
    for index, tile in enumerate(idles):
        assert tile.getbbox()
        assert all(tile.tobytes() != pose.tobytes() for pose in (left if index == 0 else right))
    original_atlas = ROOT / 'artifacts/player-walk-review-01/bastion-walk-native64.png'
    original_idle = ROOT / 'artifacts/player-walk-review-01/bastion-idle-native64.png'
    data = {'actors': [{'atlas': uri(original_atlas), 'idle': uri(original_idle)},
                       {'atlas': uri(atlas_path), 'idle': uri(idle_path)}]}
    template = (ROOT / 'tools/art/player_walk_steps_06.html').read_text('utf-8')
    fragment = template.replace('__WALK_STEP_DATA__', json.dumps(data, separators=(',', ':')))
    assert '__WALK_STEP_DATA__' not in fragment and len(fragment.encode('utf-8')) < 1_000_000
    VIS.write_text(fragment, encoding='utf-8')
    (ART / 'step03-preview-fragment.html').write_text(fragment, encoding='utf-8')

    font = ImageFont.truetype('C:/Windows/Fonts/arial.ttf', 14)
    for name, ground, ink in [('dark', '#293132', '#eeeade'), ('light', '#b5afa0', '#293132')]:
        board = Image.new('RGB', (560, 200), ground)
        draw = ImageDraw.Draw(board)
        for row, frames in enumerate([left, right]):
            draw.text((10, row * 100 + 6), ['Gauche / repos puis 6 poses', 'Droite / repos puis 6 poses'][row], font=font, fill=ink)
            for column, tile in enumerate([idles[row]] + frames):
                board.paste(tile, (column * 80 + 8, row * 100 + 28), tile)
        board.save(ART / f'step03-native-{name}.png')
        board.resize((board.width * 2, board.height * 2), Image.Resampling.NEAREST).save(ART / f'step03-zoom2-{name}.png')

    # Disposable native-size motion evidence. Three images400ms, then real
    # idle520ms, exactly as the original viewer; never hold a lifted walk frame.
    frames = []
    durations = []
    for half in range(2):
        for phase in range(3):
            for tick in range(8):
                t = (phase + tick / 8) / 3
                move = 1 - (1 - t) ** 2
                image = Image.new('RGB', (576, 152), '#293132')
                draw = ImageDraw.Draw(image)
                draw.text((10, 5), 'Gauche / native64', font=font, fill='#eeeade')
                draw.text((298, 5), 'Droite / native64', font=font, fill='#eeeade')
                image.paste(left[half * 3 + phase], (round(176 - 64 * move), 58), left[half * 3 + phase])
                image.paste(right[half * 3 + phase], (round(320 + 64 * move), 58), right[half * 3 + phase])
                frames.append(image);durations.append(20 if tick < (6 if phase == 2 else 5) else 10)
        rest_image = Image.new('RGB', (576, 152), '#293132')
        draw = ImageDraw.Draw(rest_image)
        draw.text((10, 5), 'Gauche / repos', font=font, fill='#eeeade')
        draw.text((298, 5), 'Droite / repos', font=font, fill='#eeeade')
        rest_image.paste(idles[0], (112, 58), idles[0])
        rest_image.paste(idles[1], (384, 58), idles[1])
        frames.append(rest_image);durations.append(520)
    # GIF timings are only an inspection aid; the interactive view uses an exact
    # continuous clock. Keep total contact phase about400ms at centisecond scale.
    frames[0].save(ART / 'step03-native-proof.gif', save_all=True, append_images=frames[1:],
                   duration=durations, loop=0, disposal=2, optimize=False)
    report = {
        'review_id': 'WALK05-STEP03', 'status': 'awaiting_visual_review',
        'scope': 'CLASS-10, rifle only; complete bank remains pending',
        'left_sources': [str(p.relative_to(ROOT)) for p in left_sources],
        'right_source': str((SOURCE / 'right-six.png').relative_to(ROOT)),
        'idle_sources': [str((SOURCE / f'idle-{direction}.png').relative_to(ROOT)) for direction in ('left', 'right')],
        'native_canvas': [64, 64], 'body_height_limit': 58, 'baseline': 63,
        'source_group_uniform_scales': {'left': left_scale, 'right': right_scale, 'idle': idle_scale},
        'full_figure_only': True, 'limb_deformation_or_grafting': False,
        'source_frame_hashes': hashes, 'frames_per_direction': 6, 'idle_count': 2,
        'default_cell_ms': 400, 'idle_gap_ms': 520, 'frames_per_cell': 3,
        'renderer_fix': 'Use actual idle atlas whenever moving is false; never freeze a walking pose at arrival',
        'original_bastion_images_unchanged': True, 'runtime_integration': False,
        'vertical_movement_modified': False, 'fragment_bytes': len(fragment.encode('utf-8')),
        'limits': ['Player acceptance pending', 'No browser or game execution claimed by packaging',
                   'Single-frame generation can introduce detail drift; current sources still require review'],
    }
    acceptance = DOC / 'visual-acceptance-01.json'
    if acceptance.exists():
        decision = json.loads(acceptance.read_text('utf-8'))
        expected = [entry['sha256'] for entry in decision['assets']]
        actual = [hashlib.sha256(p.read_bytes()).hexdigest() for p in (atlas_path, idle_path)]
        if expected == actual and decision['status'] == 'accepted':
            report['status'] = 'accepted'
            report['acceptance'] = str(acceptance.relative_to(ROOT))
            report['limits'][0] = 'Accepted CLASS-10 rifle only; full bank remains pending'
    (DOC / 'step03-packaging.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({key: value for key, value in report.items() if key != 'source_frame_hashes'}, indent=2))


if __name__ == '__main__':
    main()
