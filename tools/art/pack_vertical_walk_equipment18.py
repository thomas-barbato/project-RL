"""Pack coherent heavy-outfit figures and independent hand landmarks for C03."""
from pathlib import Path
import json
import hashlib
import statistics
from PIL import Image, ImageDraw, ImageFont
from pack_vertical_walk_review18 import figures, opaque_box

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/source-art/vertical-walk-review-18/candidate-C03'
PREVIEW = ROOT / 'artifacts/vertical-walk-review-18/current/equipment-C03'
HINTS = {'bastion-lourd': [(47, 71), (81, 71)], 'traqueuse-lourde': [(52, 69), (76, 69)],
         'arcaniste-lourd': [(49, 68), (79, 68)], 'metamorphe-lourd': [(51, 69), (77, 69)]}


def complete_figures(image):
    try:
        return figures(image)
    except AssertionError as error:
        if not str(error).startswith('Expected four separate authored rows'):
            raise
    # A tall armor sheet can have less than ten blank rows between figures.
    # Cut only at a genuinely transparent valley near each authored row break.
    alpha = image.getchannel('A')
    scores = [sum(v >= 32 for v in alpha.crop((0, y, image.width, y+1)).tobytes()) for y in range(image.height)]
    edges = [0]
    for q in [1, 2, 3]:
        target = round(q*image.height/4)
        span = range(target-45, target+46)
        y = min(span, key=lambda y: (scores[y], abs(y-target)))
        assert scores[y] == 0, ('no transparent row boundary', q)
        edges.append(y)
    edges.append(image.height)
    cells = []
    for row in range(4):
        for col in range(4):
            cell = image.crop((round(col*image.width/4), edges[row], round((col+1)*image.width/4), edges[row+1]))
            box = opaque_box(cell)
            assert box and box[0] > 0 and box[1] > 0 and box[2] < cell.width and box[3] <= cell.height
            whole = cell.crop(box)
            padded = Image.new('RGBA', (whole.width+4, whole.height+4))
            padded.alpha_composite(whole, (2, 2))
            cells.append(padded)
    return cells


def palm(image, hint):
    x, y = hint
    points = []
    for yy in range(y-3, y+4):
        for xx in range(x-3, x+4):
            r, g, b, a = image.getpixel((xx, yy))
            if a >= 128 and max(r, g, b) < 150:
                weight = a / (1 + (xx-x)**2 + (yy-y)**2)
                points.append((xx, yy, weight))
    assert points, ('hand landmark', hint)
    total = sum(p[2] for p in points)
    return [round(sum(p[k]*p[2] for p in points)/total, 2) for k in [0, 1]]


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    PREVIEW.mkdir(parents=True, exist_ok=True)
    requests = json.loads((ROOT / 'docs/reviews/vertical-walk-18/prompts-C03.json').read_text('utf8'))
    board = Image.new('RGB', (960, len(requests['calls'])*300+20), '#1e2831')
    draw = ImageDraw.Draw(board)
    font = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Regular.ttf'), 14)
    profiles = []
    for row, req in enumerate(requests['calls']):
        source = ROOT / req.get('selected_output', req['output'])
        cells = complete_figures(Image.open(source).convert('RGBA'))
        old = Image.open(ROOT / req['references'][0]).convert('RGBA')
        old_box = opaque_box(old)
        height = old_box[3]-old_box[1]
        scale = height / statistics.median(opaque_box(im)[3]-opaque_box(im)[1] for im in cells)
        normalized = []
        for image in cells:
            reduced = image.resize((round(image.width*scale), round(image.height*scale)), Image.Resampling.LANCZOS)
            box = opaque_box(reduced)
            canvas = Image.new('RGBA', (128, 128))
            canvas.alpha_composite(reduced, (round(64-(box[0]+box[2])/2), old_box[3]-box[3]))
            normalized.append(canvas)
        sheet = Image.new('RGBA', (768, 256))
        frames = []
        for direction, indexes in [('front', [1, 2, 3, 5, 6, 7]), ('back', [9, 10, 11, 13, 14, 15])]:
            back = direction == 'back'
            for phase, n in enumerate(indexes):
                image = normalized[n]
                box = opaque_box(image)
                assert 27 <= box[0] < box[2] <= 101 and 27 <= box[1] < box[3] <= 99
                sheet.paste(image, (phase*128, back*128))
                palms = [palm(image, h) for h in HINTS[req['id']]]
                frames.append({'direction': direction, 'phase': phase, 'box': list(box), 'palms': palms})
                board.paste(image, (150+phase*128, row*300+back*144+18), image)
            draw.text((8, row*300+back*144+5), req['label'] + (' / dos' if back else ' / face'), font=font, fill='white')
        path = OUT / (req['id']+'.png'); standing = OUT / (req['id']+'-standing-original.png')
        sheet.save(path); old.save(standing)
        profiles.append({'id':req['id'],'selector':{'kind':'player','body':req['body'],'equipment':req['equipment']},
                         'file':path.relative_to(ROOT).as_posix(),'standing':standing.relative_to(ROOT).as_posix(),
                         'source':source.relative_to(ROOT).as_posix(),'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                         'source_scale_xy_uniform':scale,'original_height_px':height,'original_foot_y':old_box[3],
                         'equipment_scope':'one_handed_grips_review_only','unarmed_only':False,'frames':frames})
    board.save(PREVIEW / 'contact-native64.png')
    (OUT/'manifest.json').write_text(json.dumps({'review':'VERT18-C03','status':'review_only','normal_game_enabled':False,
        'native_tile_px':64,'canvas_px':128,'whole_authored_figures_only':True,'profiles':profiles},ensure_ascii=False,indent=2)+'\n','utf8')
    print(json.dumps({'profiles':len(profiles),'whole_moving_figures':12*len(profiles)}))


if __name__ == '__main__':
    main()
