"""Pack native ANIM-S37 fixtures without retaining disposable screenshots."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-80'
assert not (REVIEW / 'visual-acceptance-01.json').exists(), 'Review already accepted'
entries = json.loads((REVIEW / '_capture/capture.json').read_text(encoding='utf8'))
assert len(entries) == 14
checks = []
for e in entries:
    count = e['unique_images']
    cols = e.pop('native_columns')
    rows = (count + cols - 1) // cols
    native = Image.open(REVIEW / '_capture' / e['folder'] / 'atlas.png').convert('RGB')
    assert native.size == (576 * cols, 576 * rows)
    atlas = Image.new('RGB', native.size)
    hashes = []
    for i in range(count):
        y = (rows - 1 - i // cols) * 576
        frame = native.crop(((i % cols) * 576, y, (i % cols + 1) * 576, y + 576))
        atlas.paste(frame, ((i % cols) * 576, (i // cols) * 576))
        hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
    name = 'apercu-animations-data/' + e['variant'] + '.webp'
    output = BASE / name
    atlas.save(output, 'WEBP', lossless=True, method=2)
    with Image.open(output) as packed:
        for i, h in enumerate(hashes):
            frame = packed.crop(((i % cols) * 576, (i // cols) * 576,
                                 (i % cols + 1) * 576, (i // cols + 1) * 576)).convert('RGB')
            assert hashlib.sha256(frame.tobytes()).hexdigest() == h
    e.update(atlas=name, columns=cols)
    before, launched, after = (e['timeline'][i] for i in (0, 18, 95))
    assert before['preview'] and before['energy'] == 100 and before['turn'] == 0
    assert not launched['preview'] and all(s['damage'] == 0 for s in e['timeline'])
    if e['case'] == 'OBSTACLE':
        assert not launched['cast'] and after['energy'] == 100 and after['turn'] == 0
        assert after['cooldown'] == 0 and after['player'] == [4, 6]
    else:
        assert launched['cast'] and launched['turn'] == 1
        assert launched['energy'] == (92 if e['id'] == 'ES03' else 90)
        assert launched['remaining'] == (3 if e['id'] == 'ES03' else 2)
        assert launched['player'] == ([4, 6] if e['id'] == 'ES03' else [4, 4])
        if e['case'] == 'ATTENTE':
            assert launched == after, 'No game turn consumed by real-time idle'
        else:
            assert after['remaining'] == 0 and after['turn'] == 4
        if e['case'] == 'IMPACT' and e['id'] == 'ES03':
            assert e['timeline'][48]['impact'] and e['timeline'][48]['remaining'] == 0
        if e['case'] == 'DUREE' and e['id'] == 'ES03':
            assert e['timeline'][72]['remaining'] == 1 and not after['impact']
    checks.append(dict(variant=e['variant'],frames=e['frames'],unique_images=count,
                       pixel_equality=True,timeline=True,atlas_bytes=output.stat().st_size,
                       frame_rgb_sha256=hashes))
    print(e['variant'], count, output.stat().st_size, flush=True)
text = json.dumps(entries, ensure_ascii=False, separators=(',', ':'))
(REVIEW / 'animations.json').write_text(text, encoding='utf8')
(REVIEW / 'validation.json').write_text(json.dumps(dict(native_tile=64, scenes=14,
    lossless=True, campaign_integration=False, checks=checks), indent=2), encoding='utf8')
template = (ROOT / 'tools/art/evasion_review_template.html').read_text(encoding='utf8')
(BASE / 'apercu-animations.html').write_text(template.replace('__ENTRIES__', text), encoding='utf8')
print('Packed MiB:', round(sum(c['atlas_bytes'] for c in checks) / 2**20, 2))
