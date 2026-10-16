"""Compare the cleaned renderer with every frame of the accepted A03 playback."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-84'
accepted = json.loads((REVIEW / 'animations.json').read_text(encoding='utf8'))
checks = json.loads((REVIEW / 'validation.json').read_text(encoding='utf8'))['checks']
captured = json.loads((REVIEW / '_capture/capture.json').read_text(encoding='utf8'))
assert len(accepted) == len(captured) == 8
results = []
for entry in captured:
    old = next(e for e in accepted if e['variant'] == entry['variant'])
    assert entry['timeline'] == old['timeline'], entry['variant']
    check = next(c for c in checks if c['variant'] == entry['variant'])
    columns = entry['native_columns']
    rows = (entry['unique_images'] + columns - 1) // columns
    maximum_delta = 0
    exact_frames = 0
    with Image.open(REVIEW / '_capture' / entry['folder'] / 'atlas.png') as image:
        image = image.convert('RGB')
        frames = []
        for index in range(entry['unique_images']):
            x, y = index % columns * 576, (rows - 1 - index // columns) * 576
            frames.append(image.crop((x, y, x + 576, y + 576)))
    atlas = Image.open(ROOT / 'artifacts/refonte-agent-2026-10-06' / old['atlas']).convert('RGB')
    compared = {}
    for frame in range(entry['frames']):
        index, previous_index = entry['indices'][frame], old['indices'][frame]
        key = (index, previous_index)
        if key not in compared:
            actual = frames[index]
            exact = hashlib.sha256(actual.tobytes()).hexdigest() == check['frame_rgb_sha256'][previous_index]
            delta = 0
            if not exact:
                x, y = previous_index % old['columns'] * 576, previous_index // old['columns'] * 576
                difference = ImageChops.difference(actual, atlas.crop((x, y, x + 576, y + 576)))
                delta = max(band[1] for band in difference.getextrema())
                # Removing the unused render-target/material initialization changes only
                # tiny antialiasing values in labels. Artwork and VFX remain exact.
                assert delta <= 3, (entry['variant'], frame, delta)
                for text_region in [(0, 430, 576, 576), (140, 175, 425, 270), (260, 315, 325, 350)]:
                    difference.paste((0, 0, 0), text_region)
                assert difference.getbbox() is None, (entry['variant'], frame, 'graphics changed')
            compared[key] = (exact, delta)
        exact, delta = compared[key]
        exact_frames += int(exact)
        maximum_delta = max(maximum_delta, delta)
    results.append({'variant': entry['variant'], 'frames': entry['frames'], 'graphics_outside_labels_identical': True,
                    'text_antialiasing_max_RGB_delta': maximum_delta, 'fully_identical_frames': exact_frames, 'timeline_identical': True})
receipt = {'scenes': len(results), 'frames': sum(c['frames'] for c in results), 'checks': results,
           'source_sha256': hashlib.sha256((ROOT / 'src/agent_skill_lab_review/fire_painted/survival_passives.rs').read_bytes()).hexdigest()}
(REVIEW / 'post-cleanup-native-verification.json').write_text(json.dumps(receipt, indent=2) + '\n', encoding='utf8')
print(f"{receipt['frames']} native frames preserve artwork, effects and outcomes; label antialiasing varies by at most 3/255.")
