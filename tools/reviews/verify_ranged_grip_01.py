"""Lay out unmodified renderer exports for native-size and enlarged QA."""
from pathlib import Path
import sys
from PIL import Image, ImageDraw
import json

ROOT = Path(__file__).resolve().parents[2]
phase = sys.argv[1]
folder = ROOT / 'artifacts/ranged-grip-fix-01' / phase
sheet = Image.new('RGB', (1160, 850), '#3e4444')
draw = ImageDraw.Draw(sheet)
for row, outfit in enumerate([200, 201, 202, 203, 204]):
    for col, (color, model) in enumerate((c, m) for c in ['original', 'blue'] for m in ['none', '101', '38', '15']):
        x, y = col * 145, row * 170
        image = Image.open(folder / f'{outfit}-{model}-{color}.png').convert('RGBA').transpose(Image.Transpose.FLIP_TOP_BOTTOM)
        sheet.paste(image, (x + 8, y + 27), image)
        draw.text((x + 5, y + 6), f'{outfit} / {model} / {color}', fill='#f0d9a7')
sheet.save(folder / 'native.png')

detail = Image.new('RGB', (1024, 1080), '#3e4444')
draw = ImageDraw.Draw(detail)
for row, outfit in enumerate([200, 201, 202]):
    for col, model in enumerate(['none', '38']):
        image = Image.open(folder / f'{outfit}-{model}-original.png').convert('RGBA').transpose(Image.Transpose.FLIP_TOP_BOTTOM)
        image = image.crop((20, 27, 106, 100)).resize((344, 292), Image.Resampling.NEAREST)
        x, y = col * 512, row * 360
        detail.paste(image, (x + 75, y + 45), image)
        draw.text((x + 30, y + 15), f'{outfit} / {model} - diagnostic x4', fill='#f0d9a7')
detail.save(folder / 'detail.png')
if phase == 'after':
    before = folder.parent / 'before'
    records = []
    for path in sorted(before.glob('[0-9]*.png')):
        old = Image.open(path).convert('RGBA').transpose(Image.Transpose.FLIP_TOP_BOTTOM)
        new = Image.open(folder / path.name).convert('RGBA').transpose(Image.Transpose.FLIP_TOP_BOTTOM)
        model = path.stem.split('-')[1]
        fixed = all(old.crop(box).tobytes() == new.crop(box).tobytes()
                    for box in [(0, 0, 128, 44), (0, 80, 128, 128)])
        identical = old.tobytes() == new.tobytes()
        assert fixed, path.name
        assert identical == (model in ['none', '101']), path.name
        def fragments(image):
            return sum(image.getpixel((x, y))[3] > 0 for y in range(68, 78) for x in range(88, 128))
        record = dict(image=path.name, fixed_head_and_legs=fixed, identical=identical)
        if model == '38':
            record.update(phantom_pixels_before=fragments(old), phantom_pixels_after=fragments(new))
            assert record['phantom_pixels_before'] > 0 and record['phantom_pixels_after'] == 0, path.name
        records.append(record)
    proof = Image.new('RGB', (320, 330), '#3e4444')
    draw = ImageDraw.Draw(proof)
    for row, (outfit, label) in enumerate([(200, 'GRIP-01 / Bastion'), (202, 'GRIP-02 / Traqueur')]):
        y = row * 165
        draw.text((12, y + 4), label, fill='#f0d9a7')
        for col, (directory, heading) in enumerate([(before, 'Avant'), (folder, 'Apres')]):
            x = col * 160
            draw.text((x + 15, y + 22), heading, fill='white')
            image = Image.open(directory / f'{outfit}-38-original.png').convert('RGBA').transpose(Image.Transpose.FLIP_TOP_BOTTOM)
            proof.paste(image, (x + 12, y + 36), image)
    proof.save(folder.parent / 'comparison-native.png')
    (folder.parent / 'pixel-checks.json').write_text(json.dumps(dict(
        review='GRIP-FIX-01', native_cell=64, modified_source_assets=False,
        samples=records, unchanged=sum(r['identical'] for r in records),
        corrected=sum(not r['identical'] for r in records)), indent=2) + '\n', encoding='utf-8')
print(folder / 'native.png')
print(folder / 'detail.png')
