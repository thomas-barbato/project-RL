"""Condition the generated GL01 drawing without repainting or moving its grips."""
from pathlib import Path
from PIL import Image
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'assets/source-art/animation-polish-01/grenade-launcher-r01'

def main():
    original = SOURCE / 'GL01-r01-generated.png'
    image = Image.open(original).convert('RGBA')
    # Exclude only the generated near-transparent noise around the main object.
    box = image.getchannel('A').point(lambda a: 255 if a >= 16 else 0).getbbox()
    part = image.crop(box).convert('RGBa').resize((44, 18), Image.Resampling.LANCZOS).convert('RGBA')
    native = Image.new('RGBA', (128, 128))
    native.paste(part, (44, 53))
    path = SOURCE / 'GL01-r01-native.png'
    native.save(path)
    data = {'review': 'ANIM-POLISH01-C05-GL01-r01', 'normal_game_enabled': False,
            'generated_crop': box, 'native_bounds': native.getbbox(),
            'rear_grip': [59, 68], 'front_grip': [77, 68],
            'conditioning': 'alpha-aware crop and premultiplied downsampling only; no repainting',
            'sha256': {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in [original, path]}}
    (ROOT / 'docs/reviews/animation-polish-01/GL01-SOURCE.json').write_text(json.dumps(data, indent=2)+'\n', 'utf8')
    print(json.dumps(data))

if __name__ == '__main__':
    main()
