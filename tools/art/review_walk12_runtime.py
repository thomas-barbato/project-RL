"""Review actual Rust-compositor exports at native64, without repainting assets."""
from pathlib import Path
import json
import sys
from PIL import Image, ImageDraw, ImageOps

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-12'
EXPORT = ROOT / 'artifacts/player-walk-review-12/equipped-export'
NAMES = ['Bastion', 'Technomancien', 'Traqueuse', 'Arcaniste', 'Metamorphe']


def frames_at(path):
    source = Image.open(path)
    frames = []
    for n in range(source.n_frames):
        source.seek(n)
        frames.append((source.convert('RGB').copy(), source.info['duration']))
    return frames


def at_time(frames, elapsed):
    for frame, duration in frames:
        if elapsed < duration:
            return frame
        elapsed -= duration
    return frames[-1][0]


def put(board, sprite, x, y):
    board.paste(sprite, (round(x), round(y)), sprite)


def main():
    proposed = '--proposal' in sys.argv
    export = ROOT / 'artifacts/player-walk-review-12/proposed-export' if proposed else EXPORT
    DOC.mkdir(parents=True, exist_ok=True)
    report = []
    for cls, name in enumerate(NAMES):
        previous = frames_at(DOC / f'{name}-bras-et-retour-face-native64.gif' if proposed else ROOT / f'docs/reviews/player-walk-11/grips/{name}-prises-avant-apres.gif')
        after = [ImageOps.flip(Image.open(export / f'CLASS-{cls*5+rank:02}-weapon067.png').convert('RGBA')) for rank in range(5)]
        fronts = [ImageOps.flip(Image.open(export / f'CLASS-{cls*5+rank:02}-weapon067-front.png').convert('RGBA')) for rank in range(5)]
        frames = []
        durations = []
        elapsed = 0
        for phase in range(28):
            local, step = phase % 14, phase // 14
            moving = local < 12
            pose = min(local // 4, 2) + step * 3 + 1
            progress = min(local / 12, 1)
            duration = [30, 40, 30][local % 3] if moving else 260
            board = Image.new('RGB', (520, 720), '#253136')
            draw = ImageDraw.Draw(board)
            draw.text((8, 6), f'{name} - natif64 - 400ms / case', fill='white')
            draw.text((16, 27), 'VERSION INTEGREE' if proposed else 'AVANT : prise par image', fill='white')
            draw.text((275, 27), 'PROPOSITION : MEME DEMARCHE' if proposed else 'CORRIGE : prise fixe + face', fill='white')
            old = at_time(previous, elapsed)
            for rank in range(5):
                y = 62 + rank * 130
                draw.text((5, y-10), f'{rank+1}', fill='white')
                for direction in range(2):
                    yy = y + direction * 58
                    board.paste(old.crop((300, yy, 428, yy+64)), (35, yy))
                    for tile in range(2):
                        draw.rectangle((300+tile*64, yy, 363+tile*64, yy+63), outline='#46545a')
                    offset = round((1-progress if direction == 0 else progress) * 64)
                    col = direction * 7 + pose
                    sprite = after[rank].crop((col*128, 0, (col+1)*128, 128)) if moving else fronts[rank]
                    put(board, sprite, 300+offset-32, yy-32)
            frames.append(board)
            durations.append(duration)
            elapsed += duration
        destination = DOC / (f'{name}-demarche-commune-proposee.gif' if proposed else f'{name}-bras-et-retour-face-native64.gif')
        frames[0].save(destination, save_all=True, append_images=frames[1:], duration=durations, loop=0, optimize=False)
        for phase in [4, 9, 12]:
            frames[phase].save(DOC / f'{name}-{"proposition" if proposed else "controle"}-phase{phase:02}.png')
        report.append({'class': name, 'outfits': 5, 'directions': 2, 'weapon_model': 67,
                       'source': 'actual Rust compositor', 'file': destination.name,
                       'movement_ms_per_cell': 400, 'preview_rest_ms': 520,
                       'frontal_at_arrival': True, 'new_leg_art': proposed,
                       'normal_game_changed_by_proposal': False})
    scene = ROOT / 'artifacts/player-walk-review-12/native/actual-scene'
    names = ['step-100ms.png', 'step-200ms.png', 'front-at-400ms.png']
    images = [Image.open(scene / name).convert('RGB').crop((280, 250, 545, 523)) for name in names]
    images[0].save(DOC / 'partie-reelle-marche-et-arret.gif', save_all=True,
                   append_images=images[1:], duration=[100, 200, 800], loop=0, optimize=False)
    (DOC / ('proposed-native-review.json' if proposed else 'native-review.json')).write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print(json.dumps({'classes': len(report), 'outfits': 25, 'native_gifs': len(report)+1}))


if __name__ == '__main__':
    main()
