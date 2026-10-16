"""One inline lossless native preview, derived from the current captured atlas."""
from pathlib import Path
import hashlib
import json
import sys
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-93'
entries = json.loads((REVIEW/'animations.json').read_text(encoding='utf-8'))
skill=sys.argv[1] if len(sys.argv)>1 else 'D66'
entry = next(e for e in entries if e['id']==skill and e['case']=='COMBAT' and e['direction']==2)
assert entry['frames'] == 144 and entry['fps'] == 24
durations = [round((i+1)*1000/24)-round(i*1000/24) for i in range(144)]
with Image.open(BASE/entry['atlas']) as atlas:
    frames = []
    for i in entry['indices']:
        x,y = i%entry['columns']*576,i//entry['columns']*576
        frames.append(atlas.crop((x,y,x+576,y+576)).convert('RGB'))
output = REVIEW/('gardien-protection-64.webp' if skill=='D66' else 'algorithmes-combat-64.webp')
frames[0].save(output, 'WEBP', save_all=True, append_images=frames[1:], duration=durations,
    loop=0, lossless=True, method=1)

# Encoder may coalesce identical frames. Compare every decoded span to its
# original native frame timestamps, rather than asserting a fixed frame count.
starts = [0]
for duration in durations:
    starts.append(starts[-1]+duration)
checked = 0
with Image.open(output) as animation:
    total = 0
    for i in range(animation.n_frames):
        animation.seek(i); animation.load()
        start = animation.info['timestamp']; end = start+animation.info['duration']
        data = animation.convert('RGB').tobytes()
        for f in range(144):
            if starts[f]>=start and starts[f]<end:
                assert frames[f].tobytes()==data
                checked += 1
        total = end
    assert checked==144 and total==6000 and animation.size==(576,576)
receipt = {'source_variant':entry['variant'],'native_tile':64,'canvas':[576,576],
    'duration_ms':6000,'native_frames_checked':checked,'lossless_native_pixel_equality':True,
    'sha256':hashlib.sha256(output.read_bytes()).hexdigest(),'bytes':output.stat().st_size}
(REVIEW/('inline-'+skill+'-verification.json')).write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8')
print(receipt)
