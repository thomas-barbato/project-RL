"""Refresh native grip comparisons from real compositor exports, no redraw."""
from pathlib import Path
from PIL import Image, ImageDraw, ImageOps

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-11/grips'
AFTER = DOC / 'r03/after'
NAMES = ['Bastion', 'Technomancien', 'Traqueuse', 'Arcaniste', 'Metamorphe']


def old_frames(path):
    image = Image.open(path)
    result = []
    for n in range(image.n_frames):
        image.seek(n)
        result.append((image.convert('RGB').copy(), image.info['duration']))
    return result


def frame_at(frames, elapsed):
    for image, duration in frames:
        if elapsed < duration:
            return image
        elapsed -= duration
    return frames[-1][0]


def main():
    for cls, name in enumerate(NAMES):
        before = old_frames(DOC / f'{name}-before-grip.gif')
        after = [ImageOps.flip(Image.open(AFTER / f'CLASS-{cls*5+rank:02}-weapon067.png').convert('RGBA')) for rank in range(5)]
        frames = []; durations = []; elapsed = 0
        for phase in range(28):
            local = phase % 14; step = phase // 14; moving = local < 12
            pose = min(local // 4, 2) + step * 3 + 1 if moving else 0
            progress = min(local / 12, 1)
            duration = [30, 40, 30][local % 3] if moving else 260
            board = Image.new('RGB', (520, 720), '#253136'); draw = ImageDraw.Draw(board)
            draw.text((8, 6), f'{name} - 400ms/case, native64 - GRIP r03', fill='white')
            draw.text((12, 26), 'AVANT - mains decalees', fill='white')
            draw.text((270, 26), 'APRES - raccords corriges', fill='white')
            old = frame_at(before, elapsed)
            for rank in range(5):
                y = 62 + rank * 130; old_y = 36 + rank * 130
                draw.text((6, y-10), str(rank+1), fill='white')
                for direction in range(2):
                    yy = y + direction * 58
                    board.paste(old.crop((650, old_y+direction*58, 778, old_y+direction*58+64)), (40, yy))
                    draw.rectangle((300, yy, 427, yy+63), outline='#46545a')
                    offset = round((1-progress if direction == 0 else progress) * 64)
                    # Atlas has fourteen poses, without the front-reference cell.
                    col = direction * 7 + pose
                    im = after[rank].crop((col*128, 0, (col+1)*128, 128))
                    board.paste(im, (300+offset-32, yy-32), im)
            frames.append(board); durations.append(duration); elapsed += duration
        frames[0].save(DOC / f'{name}-prises-avant-apres.gif', save_all=True,
                       append_images=frames[1:], duration=durations, loop=0, optimize=False)
    print('Refreshed five native64 grip comparisons, GRIP-r03')


if __name__ == '__main__':
    main()
