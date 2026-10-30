"""Pack the next whole-figure VERT18 roster without touching accepted C01."""
from pathlib import Path
import hashlib
import json
import statistics
from PIL import Image, ImageDraw, ImageFont
from pack_vertical_walk_review18 import figures, opaque_box

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/vertical-walk-review-18'
OUT = ART / 'candidate-C02'
PREVIEW = ROOT / 'artifacts/vertical-walk-review-18/current/roster-C02'


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    PREVIEW.mkdir(parents=True, exist_ok=True)
    requests = json.loads((ROOT / 'docs/reviews/vertical-walk-18/prompts-C02.json').read_text('utf8'))
    board = Image.new('RGB', (960, 920), '#1e2831')
    draw = ImageDraw.Draw(board)
    font = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Regular.ttf'), 14)
    profiles = []
    for row, request in enumerate(requests['calls']):
        selected_source = request.get('selected_output', request['output'])
        source = ROOT / selected_source
        cells = figures(Image.open(source).convert('RGBA'))
        old = Image.open(ROOT / request['references'][0]).convert('RGBA')
        original_box = opaque_box(old)
        height = original_box[3] - original_box[1]
        scale = height / statistics.median(opaque_box(im)[3] - opaque_box(im)[1] for im in cells)
        native = []
        for im in cells:
            reduced = im.resize((round(im.width * scale), round(im.height * scale)), Image.Resampling.LANCZOS)
            bounds = opaque_box(reduced)
            canvas = Image.new('RGBA', (128, 128))
            canvas.alpha_composite(reduced, (round(64 - (bounds[0] + bounds[2]) / 2), original_box[3] - bounds[3]))
            native.append(canvas)
        sheet = Image.new('RGBA', (768, 256))
        frames = []
        for direction, indexes in [('front', [1, 2, 3, 5, 6, 7]), ('back', [9, 10, 11, 13, 14, 15])]:
            back = direction == 'back'
            for phase, n in enumerate(indexes):
                im = native[n]
                bounds = opaque_box(im)
                assert 28 <= bounds[0] < bounds[2] <= 100 and 28 <= bounds[1] < bounds[3] <= 98
                sheet.paste(im, (phase * 128, back * 128))
                frames.append({'direction': direction, 'phase': phase, 'box': list(bounds),
                               'palms': [[51, 68], [77, 68]], 'palms_reviewed': False})
                y = row * 300 + back * 144
                board.paste(im, (150 + phase * 128, y + 18), im)
            draw.text((8, row * 300 + back * 144 + 5), request['label'] + (' / dos' if back else ' / face'), font=font, fill='white')
        path = OUT / (request['id'] + '.png')
        standing = OUT / (request['id'] + '-standing-original.png')
        sheet.save(path)
        old.save(standing)
        profiles.append({'id': request['id'], 'selector': {'kind': 'player', 'body': request['body'], 'equipment': [None] * 4},
                         'file': path.relative_to(ROOT).as_posix(), 'standing': standing.relative_to(ROOT).as_posix(),
                         'source': selected_source, 'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                         'source_scale_xy_uniform': scale, 'original_height_px': height, 'original_foot_y': original_box[3],
                         'equipment_scope': 'unarmed_only_pending_authored_grip_review', 'unarmed_only': True, 'frames': frames})
    board.save(PREVIEW / 'contact-native64.png')
    (OUT / 'manifest.json').write_text(json.dumps({'review': 'VERT18-C02', 'status': 'review_only', 'normal_game_enabled': False,
        'native_tile_px': 64, 'canvas_px': 128, 'whole_authored_figures_only': True, 'profiles': profiles}, ensure_ascii=False, indent=2) + '\n', 'utf8')
    print(json.dumps({'profiles': len(profiles), 'authored_moving_figures': len(profiles) * 12}))


if __name__ == '__main__':
    main()
