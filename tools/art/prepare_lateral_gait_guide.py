"""Document the requested anatomy for imagegen; this is not game artwork."""
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/source-art/animation-polish-01/lateral-identity-02/r02'


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    image = Image.new('RGB', (1792, 840), '#14212a')
    draw = ImageDraw.Draw(image)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 15)
    small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 13)
    # All figures use the same head, torso, hips and ground. Coordinates below
    # describe whole-figure pose instructions, never pixels used by the game.
    near = [
        [(144, 218), (119, 280), (94, 338), (77, 350)],
        [(144, 218), (140, 280), (137, 338), (115, 350)],
        [(144, 218), (161, 280), (176, 335), (156, 350)],
        [(144, 218), (162, 280), (187, 329), (169, 350)],
        [(144, 218), (124, 272), (147, 315), (127, 323)],
        [(144, 218), (113, 273), (104, 329), (82, 336)],
        [(144, 218), (145, 280), (149, 338), (127, 350)],
    ]
    far = [
        [(127, 218), (155, 279), (175, 330), (156, 350)],
        [(127, 218), (138, 275), (158, 315), (142, 323)],
        [(127, 218), (102, 271), (98, 328), (76, 336)],
        [(127, 218), (106, 280), (86, 338), (67, 350)],
        [(127, 218), (125, 280), (123, 338), (102, 350)],
        [(127, 218), (145, 279), (162, 335), (144, 350)],
        [(127, 218), (120, 280), (115, 338), (93, 350)],
    ]
    labels = ['0 / near contact', '1 / FAR swings', '2 / FAR reaches',
              '3 / far contact', '4 / NEAR swings', '5 / NEAR reaches', '6 / planted idle']
    for row in range(2):
        for col in range(7):
            def point(p):
                return (col * 256 + (p[0] if row == 0 else 256 - p[0]), row * 420 + p[1] + 30)
            draw.text((col * 256 + 8, row * 420 + 6), labels[col], fill='white', font=font)
            draw.line([point((38, 350)), point((222, 350))], fill='#48616d', width=1)
            for chain, color in [(far[col], '#e8a252'), (near[col], '#50d6df')]:
                draw.line([point(p) for p in chain], fill=color, width=13, joint='curve')
                for p in chain[:3]:
                    x, y = point(p)
                    draw.ellipse((x - 7, y - 7, x + 7, y + 7), fill=color, outline='#14212a', width=2)
            draw.polygon([point(p) for p in [(108, 110), (151, 110), (153, 207), (128, 226), (105, 206)]], fill='#9aaab3')
            x, y = point((128, 78))
            draw.ellipse((x - 23, y - 28, x + 23, y + 25), fill='#9aaab3')
            draw.line([point((112, 123)), point((98, 172)), point((126, 176))], fill='#c6d0d7', width=11)
            draw.line([point((148, 123)), point((163, 167)), point((133, 176))], fill='#c6d0d7', width=11)
            draw.text((col * 256 + 8, row * 420 + 389), 'CYAN = NEAR / ORANGE = FAR', fill='#c3d1d7', font=small)
    image.save(OUT / 'pose-guide.png')
    print(OUT / 'pose-guide.png')


if __name__ == '__main__':
    main()
