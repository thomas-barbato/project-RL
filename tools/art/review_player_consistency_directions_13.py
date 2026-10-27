"""Show C13 candidates alone, with opposite journeys and explicit facing checks.

This reads proposal-only whole figures; it does not edit any runtime resource.
The amber-head marker is a useful row-order check, not a proof of anatomy.
Whole heads, chests and boots must also be inspected on the contact sheet.
"""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont
from pack_player_consistency_13 import box, native
from audit_walk09_body_volume import measure

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / 'assets/source-art/player-consistency-13'
DOC = ROOT / 'docs/reviews/player-consistency-13'
BG = '#253136'
FONT = ImageFont.truetype('C:/Windows/Fonts/arial.ttf', 13)


def label(draw, xy, text):
    draw.text(xy, text, fill='white', font=FONT)


def amber_front_offset(figure):
    bounds = box(figure)
    head = figure.crop((0, bounds[1], figure.width,
                       bounds[1] + round((bounds[3] - bounds[1]) * .2)))
    head_box = box(head)
    marker = []
    for y in range(head.height):
        for x in range(head.width):
            r, g, b, a = head.getpixel((x, y))
            if a >= 96 and r > 130 and r-g > 35 and g-b > 35:
                marker.append((x, r-g))
    assert marker, 'Missing amber marker: inspect the complete head manually'
    return sum(x*w for x, w in marker) / sum(w for _, w in marker) - (head_box[0]+head_box[2])/2


def load_candidates():
    collections, records = [], []
    for rank in range(1, 6):
        atlas = Image.open(SRC / f'TE-C13-{rank:02}-whole-native128.png').convert('RGBA')
        figures = [atlas.crop((i*128, 0, (i+1)*128, 128)) for i in range(13)]
        offsets = [amber_front_offset(im) for im in figures[1:]]
        assert all(v < -1 for v in offsets[:6]), (rank, 'left row', offsets)
        assert all(v > 1 for v in offsets[6:]), (rank, 'right row', offsets)
        collections.append(figures)
        records.append({'candidate': f'TE-C13-{rank:02}',
                        'left_columns': list(range(1, 7)), 'right_columns': list(range(7, 13)),
                        'left_amber_front_offsets_px': [round(v, 3) for v in offsets[:6]],
                        'right_amber_front_offsets_px': [round(v, 3) for v in offsets[6:]],
                        'atlas_sha256': hashlib.sha256(atlas.tobytes()).hexdigest()})
    return collections, records


def direction_animation(collections):
    frames, durations, timeline = [], [], []
    for step in range(2):
        for sample in range(11):
            moving = sample < 10
            phase = min(sample*3//10, 2) + step*3
            board = Image.new('RGB', (540, 630), BG)
            draw = ImageDraw.Draw(board)
            label(draw, (8, 5), 'C13 r02 — Technomancien — candidats uniquement — natif 64 px')
            label(draw, (32, 27), '← VERS LA GAUCHE')
            label(draw, (292, 27), 'VERS LA DROITE →')
            label(draw, (8, 48), '1 case / 400 ms' if moving else 'Arrêt : retour de face')
            travel = []
            for direction in range(2):
                start = 2-step if direction == 0 else step
                end = start + (-1 if direction == 0 else 1)
                col = 1 + direction*6 + phase if moving else 0
                x = round((start + (end-start)*sample/10)*64)
                assert end-start == (-1 if direction == 0 else 1)
                assert col == 0 or (1 <= col <= 6 if direction == 0 else 7 <= col <= 12)
                left = 32 + direction*260
                for rank in range(5):
                    y = 100 + rank*105
                    label(draw, (8, y-21), f'TE-C13-{rank+1:02} · {rank+1}/5')
                    for tile in range(3):
                        draw.rectangle((left+tile*64, y, left+tile*64+63, y+63), outline='#46545a')
                    im = collections[rank][col]
                    board.paste(im, (left+x-32, y-32), im)
                travel.append({'facing': -1 if direction == 0 else 1,
                               'start_cell': start, 'end_cell': end,
                               'relative_x_px': x, 'atlas_column': col})
            frames.append(board)
            durations.append(40 if moving else 300)
            timeline.append({'step': step, 'sample': sample, 'moving': moving,
                             'duration_ms': durations[-1], 'lanes': travel})
    frames[0].save(DOC/'Technomancien-C13-r02-directions-native64.gif', save_all=True,
                   append_images=frames[1:], duration=durations, loop=0, optimize=False)
    frames[0].save(DOC/'Technomancien-C13-r02-directions-contact.png')
    frames[10].save(DOC/'Technomancien-C13-r02-retour-face.png')
    return timeline


def all_poses(collections):
    board = Image.new('RGB', (948, 1390), BG)
    draw = ImageDraw.Draw(board)
    label(draw, (8, 5), 'C13 r02 — contrôle des 60 figures entières : zoom x2')
    for rank, figures in enumerate(collections):
        for direction in range(2):
            y = 56 + (rank*2+direction)*132
            label(draw, (8, y-20), f'{rank+1}/5 ' + ('GAUCHE ←' if direction == 0 else 'DROITE →'))
            for phase in range(6):
                tile = figures[1+direction*6+phase].crop((32, 32, 96, 96))
                x = 100 + phase*140
                zoom = tile.resize((128, 128), Image.Resampling.NEAREST)
                board.paste(zoom, (x+4, y-7), zoom)
    board.save(DOC/'Technomancien-C13-r02-toutes-phases-zoom2.png')
    native_board = Image.new('RGB', (540, 930), BG)
    draw = ImageDraw.Draw(native_board)
    label(draw, (8, 5), 'C13 r02 — les 60 poses à taille native 64 px')
    for rank, figures in enumerate(collections):
        for direction in range(2):
            y = 56 + (rank*2+direction)*86
            label(draw, (8, y+22), f'{rank+1}/5 ' + ('← gauche' if direction == 0 else 'droite →'))
            for phase in range(6):
                tile = figures[1+direction*6+phase].crop((32, 32, 96, 96))
                native_board.paste(tile, (108+phase*70, y), tile)
    native_board.save(DOC/'Technomancien-C13-r02-toutes-phases-native64.png')


def frontal_revision():
    board = Image.new('RGB', (580, 210), BG)
    draw = ImageDraw.Draw(board)
    label(draw, (8, 6), 'TE-C13-01 — tenue 1/5 de face — natif 64 px et zoom x2')
    metrics = []
    for pos, revision in enumerate([1, 2]):
        im = Image.open(SRC/f'TE-C13-01-front-r{revision:02}-source.png').convert('RGBA')
        b = box(im)
        im = native(im, 58/(b[3]-b[1]))
        metrics.append(measure(im))
        tile = im.crop((32, 32, 96, 96))
        x = 12+pos*290
        label(draw, (x, 35), 'C13 précédent' if pos == 0 else 'C13 r02 · plus rempli')
        board.paste(tile, (x, 72), tile)
        zoom = tile.resize((128, 128), Image.Resampling.NEAREST)
        board.paste(zoom, (x+76, 62), zoom)
    board.save(DOC/'Technomancien-C13-r02-face-01-avant-apres.png')
    return metrics


def main():
    collections, records = load_candidates()
    timeline = direction_animation(collections)
    all_poses(collections)
    metrics = frontal_revision()
    result = {'review_id': 'PLAYER-CONSISTENCY13-r02', 'status': 'pending_visual_approval',
              'reported_left_facing_inversion_reproduced_in_C13_sources': False,
              'diagnosis': 'No EXIF orientation; source top row, packed columns1..6 and leftward journeys agree. '
                           'Source bottom row, columns7..12 and rightward journeys agree. '
                           'New review removes current/candidate moving-pane comparison and shows explicit opposite lanes.',
              'art_revisions': 'Outfit1 whole front and twelve whole walking figures redrawn fuller; other four Tech cycles unchanged.',
              'automated_check_limit': 'Amber marker proves row order only. Complete head, chest and boot orientation also requires visual inspection.',
              'outfits': records, 'front1_before_after': metrics,
              'movement_ms_per_cell': 400, 'preview_rest_ms': 300,
              'timeline': timeline, 'runtime_changed': False}
    (DOC/'direction-review-r02.json').write_text(json.dumps(result, indent=2)+'\n', encoding='utf-8')
    print(json.dumps({'checked_walking_figures': sum(len(r['left_columns'])+len(r['right_columns']) for r in records),
                      'front1_upper_body_mean_width_before_after': [m['upper_body_mean_occupied_width'] for m in metrics],
                      'preview_total_ms': sum(t['duration_ms'] for t in timeline), 'runtime_changed': False}))


if __name__ == '__main__':
    main()
