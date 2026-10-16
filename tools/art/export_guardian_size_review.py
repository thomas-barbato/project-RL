"""Direct-source scale candidate; preserve the accepted drawing and game rules."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-94'
OUT = REVIEW / 'gardien-size01'
SOURCE = BASE / 'skill-animation-review-93/gardien-body02/D66-BODY02-source.png'
PREVIOUS = BASE / 'skill-animation-review-93/gardien-body02/D66-BODY02-native64.png'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    assert not (REVIEW / 'visual-acceptance-01.json').exists()
    assert not (OUT / 'visual-acceptance-01.json').exists(), 'Accepted size candidate: preserve its exported assets.'
    assert sha(SOURCE) == 'e07cb6872d63531cd79f5fb2f5d65563f56e212a7945caea49568fc3fe50286d'
    assert sha(PREVIOUS) == '343ac378ea0e34beb7892f2df833e0547fe9adf8ecb59e79ca526dbf0bc70ad0'
    OUT.mkdir(exist_ok=True)
    source = Image.open(SOURCE).convert('RGBA')
    bounds = source.getchannel('A').point(lambda a: 255 if a >= 8 else 0).getbbox()
    crop = source.crop(bounds)
    height = 64
    width = round(crop.width * height / crop.height)
    body = crop.resize((width, height), Image.Resampling.LANCZOS)
    result = Image.new('RGBA', (128, 128))
    result.alpha_composite(body, ((128 - width) // 2, 32))
    destination = OUT / 'D66-BODY02-SIZE01-native128.png'
    result.save(destination)
    old = Image.open(PREVIOUS).convert('RGBA')
    # Numbered native-size comparison on three grounds, with logical tile borders.
    board = Image.new('RGB', (520, 220), '#111619')
    draw = ImageDraw.Draw(board)
    draw.text((16, 10), 'Gardien / cases 64 px / taille native', fill='#e7ded2')
    for x, title, background in [(18, 'Sol sombre', '#252a2b'), (186, 'Sol clair', '#bbb8ab'), (354, 'Sol texture', '#555348')]:
        draw.text((x, 30), title, fill='#d8cabb')
        for y, text, image in [(58, 'Avant / 48 px', old), (142, '104 / 64 px', result)]:
            draw.rectangle((x, y, x + 143, y + 63), fill=background)
            if title == 'Sol texture':
                for yy in range(y, y + 64, 7):
                    for xx in range(x, x + 144, 9):
                        draw.rectangle((xx, yy, xx + 2, yy + 1), fill='#777568')
            draw.rectangle((x + 40, y, x + 103, y + 63), outline='#93846b')
            px, py = x + 72 - image.width // 2, y + 32 - image.height // 2
            board.paste(image, (px, py), image)
            draw.text((x, y - 14), text, fill='#e7ded2')
    comparison = OUT / 'D66-SIZE01-comparison-native.png'
    board.save(comparison)
    bounds_out = result.getchannel('A').point(lambda a: 255 if a >= 8 else 0).getbbox()
    assert bounds_out[3] - bounds_out[1] == 64
    assert bounds_out[2] - bounds_out[0] == 80
    assert result.getpixel((0, 0))[3] == 0
    proof = {
        'date': '2026-10-15', 'review_id': 'D66-SIZE01', 'number': 104,
        'status': 'visual_proposal_not_accepted', 'source': str(SOURCE.relative_to(ROOT)),
        'source_sha256': sha(SOURCE), 'source_alpha8_bounds': bounds,
        'previous': str(PREVIOUS.relative_to(ROOT)), 'previous_sha256': sha(PREVIOUS),
        'previous_body_size': [60, 48], 'candidate_body_size': [80, 64],
        'candidate_canvas': [128, 128], 'candidate_alpha8_bounds': bounds_out,
        'horizontal_overhang_per_side': 8, 'logical_tile': [64, 64],
        'method': 'Uniform direct high-resolution source reduction, not enlargement of a 32px or 64px tile.',
        'candidate_sha256': sha(destination), 'comparison_sha256': sha(comparison),
        'drawing_palette_pose_preserved': True, 'collision_or_stats_changed': False,
        'campaign_integration': False
    }
    (OUT / 'review.json').write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({'body': [width, height], 'canvas': [128, 128], 'bounds': bounds_out}))

if __name__ == '__main__':
    main()
