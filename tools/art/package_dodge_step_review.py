"""Package ANIM-S38 native pixels, then discard capture intermediates after QA."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-81'
assert not (REVIEW / 'visual-acceptance-01.json').exists(), 'Review already accepted'
entries = json.loads((REVIEW / '_capture/capture.json').read_text(encoding='utf8'))
assert len(entries) == 14
checks = []
old_checks = {}
if (REVIEW / 'validation.json').exists():
    old_checks = {c['variant']: c for c in json.loads((REVIEW / 'validation.json').read_text(encoding='utf8'))['checks']}
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
    previous = old_checks.get(e['variant'])
    if not (output.exists() and previous and previous['frame_rgb_sha256'] == hashes):
        atlas.save(output, 'WEBP', lossless=True, method=2)
    with Image.open(output) as packed:
        for i, h in enumerate(hashes):
            frame = packed.crop(((i % cols) * 576, (i // cols) * 576,
                                 (i % cols + 1) * 576, (i // cols + 1) * 576)).convert('RGB')
            assert hashlib.sha256(frame.tobytes()).hexdigest() == h
    e.update(atlas=name, columns=cols)
    before, launched, after = (e['timeline'][i] for i in (0, 18, 95))
    assert before['preview'] and before['energy'] == 100 and before['turn'] == 0
    assert not launched['preview']
    if e['id'] == 'ES06':
        assert all(s['energy'] == 100 and s['cooldown'] == 0 and s['remaining'] == 0 and
                   s['dodge'] == 12 and not s['permanent_aura'] and s['player'] == [4, 5]
                   for s in e['timeline'])
        assert after['hp'] == (88 if e['case'] == 'TOUCHE' else 100)
        assert not after['shot'] if e['case'] == 'ATTENTE' else after['shot']
    elif e['case'] == 'OBSTACLE':
        assert not after['begun'] and after['energy'] == 100 and after['turn'] == 0
        assert after['cooldown'] == 0 and after['player'] == [4, 5]
    else:
        assert launched['begun'] and launched['turn'] == 1
        assert launched['energy'] == 92 and launched['remaining'] == 2
        assert launched['dodge'] == 15 and launched['player'] == [4, 4]
        assert after['hp'] == (88 if e['case'] == 'TOUCHE' else 100)
        if e['case'] == 'ATTENTE':
            assert launched == after, 'Clock time must not consume turns'
        else:
            assert after['remaining'] == 0 and after['dodge'] == 0
    checks.append(dict(variant=e['variant'],frames=e['frames'],unique_images=count,
                       pixel_equality=True,timeline=True,atlas_bytes=output.stat().st_size,
                       frame_rgb_sha256=hashes))
    print(e['variant'], count, output.stat().st_size, flush=True)
text = json.dumps(entries, ensure_ascii=False, separators=(',', ':'))
(REVIEW / 'animations.json').write_text(text, encoding='utf8')
(REVIEW / 'validation.json').write_text(json.dumps(dict(native_tile=64, scenes=14,
    lossless=True, campaign_integration=False, prepared_outcomes=True,
    no_guaranteed_dodge_rule=True, checks=checks), indent=2), encoding='utf8')
template = (ROOT / 'tools/art/dodge_step_review_template.html').read_text(encoding='utf8')
(BASE / 'apercu-animations.html').write_text(template.replace('__ENTRIES__', text), encoding='utf8')
print('Packed MiB:', round(sum(c['atlas_bytes'] for c in checks) / 2**20, 2))
