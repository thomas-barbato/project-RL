"""Condition the isolated whole Hachette candidate; never repaint its pixels."""
from pathlib import Path
import colorsys
import hashlib
import json

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'assets/source-art/player-walk-review-08/AX67-r02-source.png'
DEST = ROOT / 'assets/animations/walk-08/weapons'
DOC = ROOT / 'docs/reviews/player-walk-08'


def main():
    DEST.mkdir(parents=True, exist_ok=True)
    original = Image.open(SOURCE).convert('RGBA')
    bounds = original.getchannel('A').point(lambda a: 255 if a >= 32 else 0).getbbox()
    crop = original.crop(bounds)
    scale = 44 / crop.width
    native = crop.resize((44, round(crop.height * scale)), Image.Resampling.LANCZOS)
    # Uniform whole-object reduction only; the shaft is already drawn horizontal.
    # Its centreline is y=447 in the source, attached at native canvas y=60.
    origin = [43, round(60 - (447 - bounds[1]) * scale)]
    canvas = Image.new('RGBA', (128, 128))
    canvas.paste(native, tuple(origin))
    canvas.save(DEST / 'AX67-r02.png')
    mask = Image.new('RGBA', canvas.size)
    values = []
    for r, g, b, a in canvas.get_flattened_data():
        _, light, saturation = colorsys.rgb_to_hls(r / 255, g / 255, b / 255)
        values.append((int(saturation > .13 and .14 < light < .84), 0, 0, 255) if a else (0, 0, 0, 0))
    mask.putdata(values)
    mask.save(DEST / 'AX67-r02-materials.png')
    old = Image.open(ROOT / 'assets/source-art/player-walk-review-08/AX67-reference.png').convert('RGBA')
    comparison = Image.new('RGBA', (512, 512), (40, 47, 52, 255))
    from PIL import ImageDraw
    draw = ImageDraw.Draw(comparison)
    for n, (label, image) in enumerate([('01 - Ancien manche', old), ('02 - Manche fin en bois / essai', canvas)]):
        draw.text((16, n * 256 + 12), label, fill='white')
        comparison.alpha_composite(image.resize((512, 512), Image.Resampling.NEAREST), (0, n * 256 - 128))
    comparison.convert('RGB').save(DOC / 'AX67-r02-comparison.png')
    (DOC / 'AX67-r02-conditioning.json').write_text(json.dumps({
        'review_id': 'WALK08-AX67-r02', 'status': 'pending_visual_review',
        'model': 67, 'source': str(SOURCE.relative_to(ROOT)),
        'source_sha256': hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
        'source_bounds': bounds, 'uniform_scale': scale, 'native_size': native.size,
        'origin': origin, 'anchor': [52, 60], 'support': [68, 60],
        'runtime_png': str((DEST / 'AX67-r02.png').relative_to(ROOT)),
        'normal_game_enabled': False, 'head_identity': 'distinct model67 head',
        'preserved_approved_model': 66,
    }, indent=2) + '\n', 'utf-8')


if __name__ == '__main__':
    main()
