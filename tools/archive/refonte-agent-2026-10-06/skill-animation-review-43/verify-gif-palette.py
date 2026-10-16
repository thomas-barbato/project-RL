"""Check GIF colours against the matching native frames, on effect pixels."""
import json
from pathlib import Path
from PIL import Image, ImageChops

root = Path(__file__).resolve().parent
checks = []
for skill, folder, side in [
    ('TR63-HEURT', 'HAUT-1', 0), ('TR64-TRAIT', 'HAUT-1', 1),
    ('TR63-DECHARGE', 'HAUT-2', 0), ('TR64-PULSATION', 'HAUT-2', 1),
    ('TR63-FERMETURE', 'HAUT-3', 0), ('TR64-DRAIN', 'HAUT-3', 1),
]:
    decoded, elapsed = [], 0
    with Image.open(root / f'{skill}-A02-64.gif') as gif:
        for i in range(gif.n_frames):
            gif.seek(i)
            duration = gif.info.get('duration', 0)
            decoded.append((elapsed, elapsed + duration, gif.convert('RGB').copy()))
            elapsed += duration
    for frame in [19, 21, 23, 25, 27, 47, 70]:
        timestamp = round(frame * 1000 / 24 / 10) * 10
        image = next(im for start, end, im in decoded if start <= timestamp < end)
        x = 32 + side * 616
        box = (x, 88, x + 576, 664)
        with Image.open(root / 'native' / folder / f'{frame:03}.png') as raw:
            native = raw.crop(box).convert('RGB')
        with Image.open(root / 'native' / folder / f'control-{frame:03}.png') as raw:
            control = raw.crop(box).convert('RGB')
        mask = ImageChops.difference(native, control)
        delta = ImageChops.difference(native, image)
        errors = sorted(max(e) for m, e in zip(mask.get_flattened_data(), delta.get_flattened_data()) if max(m) > 12)
        if not errors:
            continue
        mean, p95 = sum(errors) / len(errors), errors[round((len(errors) - 1) * .95)]
        assert mean < 22 and p95 < 55, (skill, frame, mean, p95)
        checks.append({'skill': skill, 'frame': frame, 'effect_pixels': len(errors), 'mean_error': round(mean, 2), 'p95_error': p95})
(root / 'gif-palette-verification.json').write_text(json.dumps({'checks': checks, 'status': 'passed'}, indent=2), encoding='utf-8')
print(json.dumps({'checked_samples': len(checks), 'status': 'passed', 'maximum_mean_error': max(c['mean_error'] for c in checks), 'maximum_p95_error': max(c['p95_error'] for c in checks)}))
