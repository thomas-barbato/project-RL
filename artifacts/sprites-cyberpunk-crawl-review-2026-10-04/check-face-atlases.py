"""Validate the active source rectangles; do not change any raster image."""
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parent
frames = json.loads((ROOT / 'face-bestiary-frames.json').read_text(encoding='utf8'))
assert len(frames['frames']) == 24
assert [frame['index'] for frame in frames['frames']] == list(range(24))
checks = []
for file, rectangles in [
    (frames['atlas'], frames['frames']),
    ('classes-face-v1.png', [dict(index=row*3+col, x=col*418, y=row*418, width=418, height=418) for row in range(3) for col in range(3)])
]:
    image = Image.open(ROOT / file)
    assert image.mode == 'RGBA' and image.getchannel('A').getextrema()[0] == 0
    cells = []
    for frame in rectangles:
        x, y, width, height = (frame[key] for key in ('x','y','width','height'))
        assert 0 <= x < x + width <= image.width and 0 <= y < y + height <= image.height
        alpha = image.getchannel('A').crop((x,y,x+width,y+height))
        bounds = alpha.point(lambda value: 255 if value > 128 else 0).getbbox()
        assert bounds and 0 < bounds[0] < bounds[2] < width and 0 < bounds[1] < bounds[3] < height, (file,frame['index'],bounds)
        cells.append(dict(index=frame['index'], source_rectangle=[x,y,width,height], opaque_bounds=bounds))
    checks.append(dict(atlas=file, size=list(image.size), nonempty_frames=len(cells), opaque_border_contacts=0, frames=cells))
report = dict(atlases=checks, note='Source rectangle and alpha validation, not artistic approval; large generated atlases rendered at 32px.')
(ROOT / 'face-atlas-validation.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n', encoding='utf8')
print(json.dumps([(atlas['atlas'],atlas['nonempty_frames'],atlas['opaque_border_contacts']) for atlas in checks]))
