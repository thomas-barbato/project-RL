"""Independent audit of the delivered native review assets and untouched game files."""
from pathlib import Path
from PIL import Image
import hashlib, json

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[1]
manifest = json.loads((OUT / 'manifest.json').read_text(encoding='utf-8'))
assert [e['number'] for e in manifest] == list(range(1, 56))
atlas = Image.open(ROOT / 'assets/sprites/cyberpunk/actors-32.png').convert('RGBA')
checks = []
for entry in manifest:
    number, side = entry['number'], entry['tile_size']
    for kind in ('living', 'corpse'):
        tile = Image.open(OUT / entry[kind]).convert('RGBA')
        assert tile.size == (side, side), (number, kind, tile.size)
        pixels = list(tile.get_flattened_data())
        visible = [p for p in pixels if p[3]]
        assert visible, (number, kind, 'empty')
        assert {p[3] for p in pixels} <= {0, 255}, (number, kind, 'alpha')
        colors = len({p[:3] for p in visible})
        assert colors <= 24, (number, kind, colors)
        bounds = tile.getbbox()
        if number <= 32 or kind == 'corpse':
            assert 0 < bounds[0] and bounds[2] < side and 0 < bounds[1] and bounds[3] < side, (number, kind, 'margin', bounds)
        checks.append(dict(number=number, kind=kind, size=side, colors=colors, bounds=bounds))
    if number >= 33:
        index = entry['actor_index']
        expected = atlas.crop((index % 16 * 32, index // 16 * 32, index % 16 * 32 + 32, index // 16 * 32 + 32))
        actual = Image.open(OUT / entry['living']).convert('RGBA')
        assert expected.tobytes() == actual.tobytes(), (number, 'existing living changed')

baseline = json.loads((OUT / 'baseline.json').read_text(encoding='utf-8'))
changed = [path for path, digest in baseline.items() if hashlib.sha256((ROOT / path).read_bytes()).hexdigest() != digest]
assert not changed, changed
report = dict(references=55, native_tiles=110, existing_living_tiles_identical=23, binary_alpha=True,
              maximum_palette=24, active_files_checked=len(baseline), active_files_changed=changed, checks=checks)
(OUT / 'audit.json').write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding='utf-8')
print(json.dumps({key: value for key, value in report.items() if key != 'checks'}, ensure_ascii=False))
