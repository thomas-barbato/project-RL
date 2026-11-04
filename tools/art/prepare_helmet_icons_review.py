"""Condition generated helmet drawings for native64 review; preserve raw sources."""
from pathlib import Path
import hashlib
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'assets/source-art/animation-polish-01/helmet-icons-r01'
REVIEW = ROOT / 'docs/reviews/animation-polish-01'


def prepare():
    records = []
    for number in range(1, 4):
        label = f'H03-I{number:02}'
        raw_path = SOURCE / f'{label}-generated.png'
        raw = Image.open(raw_path).convert('RGBA')
        alpha = raw.getchannel('A')
        assert alpha.getextrema()[0] == 0, f'Missing transparent margin: {raw_path}'
        bounds = alpha.point(lambda a: 255 if a >= 16 else 0).getbbox()
        assert bounds
        crop = raw.crop(bounds)
        factor = 54 / max(crop.size)
        size = tuple(max(1, round(v * factor)) for v in crop.size)
        # Premultiplied-alpha reconstruction is asset conditioning, not repainting.
        native = crop.convert('RGBa').resize(size, Image.Resampling.LANCZOS).convert('RGBA')
        canvas = Image.new('RGBA', (64, 64))
        origin = tuple((64 - v) // 2 for v in size)
        canvas.paste(native, origin)
        target = SOURCE / f'{label}-native64.png'
        canvas.save(target)
        records.append({'id':label,'raw':raw_path.relative_to(ROOT).as_posix(),
                        'native':target.relative_to(ROOT).as_posix(),
                        'raw_size':list(raw.size),'source_alpha16_bounds':list(bounds),
                        'native_size':[64,64],'conditioned_object_size':list(size),
                        'native_origin':list(origin),'raw_sha256':hashlib.sha256(raw_path.read_bytes()).hexdigest(),
                        'native_sha256':hashlib.sha256(target.read_bytes()).hexdigest()})
    (REVIEW/'HELM03-SOURCES.json').write_text(json.dumps({
        'review':'ANIM-POLISH01-HELM03-ICONS-r01','mode':'builtin_image_gen',
        'conditioning':'alpha>=16 crop, aspect-preserving premultiplied Lanczos to max54px, centered on native64; no repainting',
        'icons':records},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    return records


if __name__ == '__main__':
    print(json.dumps({'prepared':len(prepare()),'source':str(SOURCE)}))
