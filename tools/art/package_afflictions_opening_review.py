"""ANIM-S41 lossless playback and turn-lifetime verification, isolated review only."""
import hashlib
import json
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-86'
assert not (REVIEW / 'visual-acceptance-01.json').exists()
entries = json.loads((REVIEW / '_capture/capture.json').read_text(encoding='utf-8'))
assert len(entries) == 16
checks = []
for e in entries:
    before, applied, last = (e['timeline'][n] for n in (0, 18, 111))
    cost = 8 if e['id'] == 'O02' else 9
    assert before['energy'] == 100 and before['remaining'] == 0
    assert all(t['hp'] == 100 and t['damage'] == 0 for t in e['timeline'])
    assert applied['preview'] is False
    if e['case'] == 'MUR':
        assert all(t['energy'] == 100 and t['cooldown'] == t['turn'] == t['remaining'] == 0 and not t['cast'] for t in e['timeline'])
    else:
        assert applied['energy'] == last['energy'] == 100 - cost
        assert applied['cooldown'] == 5
        assert applied['cast']
        assert applied['applied'] == (e['case'] != 'RESISTE')
        assert applied['icon_visible'] == (e['case'] != 'RESISTE')
        if e['case'] == 'ATTENTE':
            assert all(t == applied for t in e['timeline'][18:])
            assert last['remaining'] == 3 and last['turn'] == 0
        else:
            for n, remaining in [(18, 3), (50, 2), (74, 1), (100, 0)]:
                t = e['timeline'][n]
                assert t['remaining'] == (0 if e['case'] == 'RESISTE' else remaining)
                assert t['icon_visible'] == (e['case'] != 'RESISTE' and remaining > 0)
            assert last['turn'] == 3 and last['cooldown'] == 2
    cols = e.pop('native_columns')
    count = e['unique_images']
    rows = (count + cols - 1) // cols
    with Image.open(REVIEW / '_capture' / e['folder'] / 'atlas.png') as source:
        source = source.convert('RGB')
        atlas = Image.new('RGB', source.size)
        hashes = []
        for i in range(count):
            x, y = i % cols * 576, (rows - 1 - i // cols) * 576
            frame = source.crop((x, y, x + 576, y + 576))
            atlas.paste(frame, (x, i // cols * 576))
            hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
    name = 'apercu-animations-data/' + e['variant'] + '.webp'
    atlas.save(BASE / name, 'WEBP', lossless=True, method=1)
    with Image.open(BASE / name) as packed:
        for i, expected in enumerate(hashes):
            x, y = i % cols * 576, i // cols * 576
            assert hashlib.sha256(packed.crop((x, y, x + 576, y + 576)).convert('RGB').tobytes()).hexdigest() == expected
    e.update(atlas=name, columns=cols)
    checks.append({'variant': e['variant'], 'frames': e['frames'], 'unique_images': count,
                   'frame_rgb_sha256': hashes, 'pixel_equality': True, 'timeline': True,
                   'atlas_bytes': (BASE / name).stat().st_size})
    print(e['variant'], count, (BASE / name).stat().st_size, flush=True)
for p in json.loads((REVIEW / 'previous-review-preservation.json').read_text(encoding='utf-8')):
    assert hashlib.sha256((ROOT / p['path']).read_bytes()).hexdigest() == p['sha256']
text = json.dumps(entries, ensure_ascii=False, separators=(',', ':'))
(REVIEW / 'animations.json').write_text(text, encoding='utf-8')
(REVIEW / 'validation.json').write_text(json.dumps({'native_tile': 64, 'scenes': 16, 'lossless': True,
    'previous_review_and_art_preserved': True, 'checks': checks}, indent=2) + '\n', encoding='utf-8')
template = (ROOT / 'tools/art/afflictions_opening_review_template.html').read_text(encoding='utf-8')
(BASE / 'apercu-animations.html').write_text(template.replace('__ENTRIES__', text), encoding='utf-8')
print('Playback MiB:', round(sum(c['atlas_bytes'] for c in checks) / 2**20, 2))
