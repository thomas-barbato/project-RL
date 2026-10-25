"""Compare actual Rust-rendered helmet exports without modifying game artwork."""
from pathlib import Path
import argparse
import hashlib
import json

from PIL import Image, ImageDraw, ImageOps

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'artifacts/walk-helmet-01'
OUT = ROOT / 'docs/reviews/player-walk-12/helmet-attachment-01'
NAMES = ['Bastion', 'Technomancien', 'Traqueuse', 'Arcaniste', 'Metamorphe']


def atlas(folder, index, head):
    return ImageOps.flip(Image.open(folder / f'CLASS-{index:02}-head{head}-unarmed.png').convert('RGBA'))


def sprite(sheet, column):
    return sheet.crop((column * 128, 0, column * 128 + 128, 128))


def main():
    args = argparse.ArgumentParser()
    args.add_argument('--oriented', action='store_true')
    oriented = args.parse_args().oriented
    OUT.mkdir(parents=True, exist_ok=True)
    before = ART / 'after' if oriented else ART / 'before/players'
    after = ROOT / 'artifacts/walk-helmet-02/after' if oriented else ART / 'after'
    fronts = unhelmeted = 0
    for index in range(25):
        for head in range(7):
            old, new = atlas(before, index, head), atlas(after, index, head)
            assert sprite(old, 0).tobytes() == sprite(new, 0).tobytes()
            fronts += 1
            if head == 0:
                assert old.tobytes() == new.tobytes()
                unhelmeted += 1

    # Native64 contact sheets: all six helmets, five outfits, both directions,
    # all six walking frames. Column0 is the unchanged canonical front.
    for cls, name in enumerate(NAMES):
        board = Image.new('RGB', (980, 30 * 92 + 24), '#253136')
        draw = ImageDraw.Draw(board)
        draw.text((8, 5), f'{name} - native64 - front / gauche x6 / droite x6', fill='white')
        for rank in range(5):
            for head in range(1, 7):
                y = 24 + (rank * 6 + head - 1) * 92
                draw.text((4, y + 10), f'T{rank+1} C{head}', fill='white')
                sheet = atlas(after, cls * 5 + rank, head)
                for n, col in enumerate([0, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14]):
                    tile = sprite(sheet, col)
                    board.paste(tile, (66 + n * 69 - 32, y - 20), tile)
        board.save(OUT / f'{name}-tous-les-casques-native64.png')

    shown_head = 5 if oriented else 3
    pairs = [(atlas(before, cls * 5, shown_head), atlas(after, cls * 5, shown_head)) for cls in range(5)]
    frames, durations = [], []
    for move, (start, end) in enumerate([(1, 0), (0, 1), (1, 2), (2, 1)]):
        direction = 0 if end < start else 1
        for sample in range(11):
            moving = sample < 10
            phase = min(sample * 3 // 10, 2) + (move % 2) * 3
            col = direction * 7 + phase + 2 if moving else 0
            progress = sample / 10
            x = round((start + (end - start) * progress) * 64)
            board = Image.new('RGB', (540, 570), '#253136')
            draw = ImageDraw.Draw(board)
            draw.text((8, 6), f'Casque {shown_head} - tenue 1 - natif64 - 400 ms / case', fill='white')
            draw.text((44, 28), 'AVANT', fill='white')
            draw.text((305, 28), 'CORRIGE', fill='white')
            for cls, name in enumerate(NAMES):
                y = 64 + cls * 100
                draw.text((8, y - 12), name, fill='white')
                for pane in range(2):
                    left = 32 + pane * 260
                    for tile in range(3):
                        draw.rectangle((left + tile * 64, y, left + tile * 64 + 63, y + 63), outline='#46545a')
                    image = sprite(pairs[cls][pane], col)
                    board.paste(image, (left + x - 32, y - 32), image)
            frames.append(board)
            durations.append(40 if moving else 300)
    preview = 'casques-orientes-avant-apres-native64.gif' if oriented else 'casques-avant-apres-native64.gif'
    frames[0].save(OUT / preview, save_all=True,
                   append_images=frames[1:], duration=durations, loop=0, optimize=False)
    frames[4].save(OUT / 'controle-native64.png')
    report = {'revision': 'WALK12-HELMET-ORIENTATION-r02' if oriented else 'WALK12-HELMET-ATTACHMENT-r01',
              'shell_turns_with_walking_head': oriented,
              'open_helmets_keep_the_authored_walking_face': oriented,
              'source': 'actual normal Rust compositor exports',
              'outfits': 25, 'helmet_models': 6, 'directions': 2,
              'walking_poses_reviewed': 25 * 6 * 2 * 6,
              'additional_lateral_reference_idles': 25 * 6 * 2,
              'unchanged_front_comparisons': fronts,
              'unchanged_unhelmeted_atlases': unhelmeted,
              'preview_ms_per_cell': 400, 'preview_only_rest_ms': 300,
              'native_tile_px': 64,
              'source_sha256': {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
                                for p in [ROOT / 'src/game_art_walk_rig.rs',
                                          ROOT / 'src/walk_proportions_player_export.rs']}}
    (OUT / 'review.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
