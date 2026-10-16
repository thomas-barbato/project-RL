"""Lossless native capture packing with duplicate frames stored as hardlinks."""
import hashlib
import json
import os
import sys
from pathlib import Path
import time
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
REVIEW = int(sys.argv[1]) if len(sys.argv)>1 else 67
OUT = ROOT / f'artifacts/refonte-agent-2026-10-06/skill-animation-review-{REVIEW}'
SUBFOLDER = sys.argv[2] if len(sys.argv)>2 else 'native'
native = (OUT / SUBFOLDER).resolve()
assert native.is_relative_to(OUT.resolve())
known = {}
count = 0
last_report = time.monotonic()

def drain():
    global count
    for raw in native.rglob('*.rgba'):
        if raw.stat().st_size != 640 * 720 * 4:
            continue
        data = raw.read_bytes()
        if len(data) != 640 * 720 * 4:
            continue
        digest = hashlib.sha256(data).hexdigest()
        target = raw.with_suffix('.png')
        if digest in known:
            os.link(known[digest], target)
        else:
            Image.frombytes('RGBA', (640, 720), data).save(target, compress_level=1)
            known[digest] = target
        assert Image.open(target).convert('RGBA').tobytes() == data
        assert raw.resolve().is_relative_to(native)
        raw.unlink()
        count += 1

while not (native / 'capture.json').exists():
    drain()
    if time.monotonic() - last_report > 25:
        print(f'{count} native readbacks verified, {len(known)} unique images', flush=True)
        last_report = time.monotonic()
    time.sleep(0.15)
drain()
assert not list(native.rglob('*.rgba'))
(OUT/('native-encoding-verification.json' if SUBFOLDER=='native' else SUBFOLDER+'-encoding-verification.json')).write_text(json.dumps({
    'converted_frames': count,
    'unique_images': len(known),
    'pixel_identical_to_native_readback': True,
    'resampling': False,
    'duplicates': 'Hardlinks to exact byte-identical PNGs',
    'raw_readbacks_removed_only_after_pixel_verification': True
}, indent=2)+'\n', encoding='utf8')
print(f'{count} verified readbacks complete', flush=True)
