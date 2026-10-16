from pathlib import Path
from PIL import Image, ImageChops
import difflib
import json

ROOT = Path(__file__).resolve().parent
PROJECT = ROOT.parents[1]
checks = []
for theme in ['violet', 'blue', 'green', 'red']:
    hud = json.loads((ROOT / 'captures' / f'terminal-hud-live-{theme}' / 'terminal-validation.json').read_text(encoding='utf-8'))
    inventory_path = ROOT / 'captures' / f'terminal-inventory-live-hands-equipped-{theme}'
    inv = json.loads((inventory_path / 'terminal-validation.json').read_text(encoding='utf-8'))
    assert hud['state_unchanged'] and inv['state_unchanged']
    assert hud['commands_readable']
    assert hud['theme'] == inv['theme'] == theme
    for label in ['PV', 'Énergie']:
        a = next(g for g in hud['resources'] if g['label'] == label)
        b = next(g for g in inv['resources'] if g['label'] == label)
        assert a == b, (theme, label)
    im = Image.open(inventory_path / 'cold-start.png').convert('RGB')
    assert im.getpixel((1010, 170)) == (122, 224, 150), (theme, 'PV')
    assert im.getpixel((1010, 213)) == (109, 186, 255), (theme, 'Énergie')
    checks.append(dict(theme=theme, resources_match_hud=True, native_pixels_verified=True))

diagnostics = []
for path in sorted((ROOT / 'captures').glob('*/terminal-validation.json')):
    data = json.loads(path.read_text(encoding='utf-8'))
    assert data['state_unchanged']
    assert data['commands_readable'] is not False
    assert data['hud_energy'] and data['hud_points']
    diagnostics.append(path.parent.name)

old_inv = Image.open(ROOT / 'inventory-before-font-warmup.png').convert('RGB')
new_inv = Image.open(ROOT / 'captures/terminal-inventory-live-hands-equipped-violet/cold-start.png').convert('RGB')
inventory_delta = ImageChops.difference(old_inv, new_inv)
inventory_max_delta = max(pair[1] for pair in inventory_delta.getextrema())
assert inventory_max_delta <= 1  # Sub-byte rounding of antialiased text, with unchanged geometry.
inventory_changed_pixels = sum(pixel != (0, 0, 0) for pixel in inventory_delta.get_flattened_data())

old_death = Image.open(ROOT.parent / 'terminal-console-proposition-2026-10-05/captures/ux-death/cold-start.png').convert('RGB')
new_death = Image.open(ROOT / 'captures/ux-death/cold-start.png').convert('RGB')
death_delta = ImageChops.difference(old_death, new_death)
death_max_delta = max(pair[1] for pair in death_delta.getextrema())
assert death_max_delta <= 1
death_identical = death_delta.getbbox() is None

previous = json.loads((ROOT.parent / 'terminal-console-proposition-2026-10-05/performance/timings.json').read_text())
perf = json.loads((ROOT / 'performance/timings.json').read_text())
by_scene = lambda p: {row['scene']: row for row in p['scenes']}
old, new = by_scene(previous), by_scene(perf)
performance = {name: dict(previous_median_ms=old[name]['cpu']['median_ms'], median_ms=new[name]['cpu']['median_ms'],
                         previous_first_ms=old[name]['first_cpu_ms'], first_ms=new[name]['first_cpu_ms'])
               for name in ['world', 'world-return', 'inventory']}
patch = []
for original in sorted((ROOT / 'before').glob('*.rs')):
    final = PROJECT / 'src' / original.name
    patch.extend(difflib.unified_diff(original.read_text(encoding='utf-8').splitlines(keepends=True),
        final.read_text(encoding='utf-8').splitlines(keepends=True), fromfile=f'before/{original.name}', tofile=f'src/{original.name}'))
chrome = PROJECT / 'src/terminal_chrome.rs'
patch.extend(difflib.unified_diff([], chrome.read_text(encoding='utf-8').splitlines(keepends=True),
    fromfile='/dev/null', tofile='src/terminal_chrome.rs'))
(ROOT / 'changes.patch').write_text(''.join(patch), encoding='utf-8')
report = dict(themes=checks, native_diagnostics=diagnostics,
              inventory_font_warmup_max_channel_delta=inventory_max_delta,
              inventory_font_warmup_changed_pixels=inventory_changed_pixels,
              death_capture_identical_to_approved_baseline=death_identical,
              targeted_tests=33, performance=performance, full_suite_run=False)
(ROOT / 'validation.json').write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding='utf-8')
print(json.dumps(report, ensure_ascii=False))
