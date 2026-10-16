"""Verify native taunt frames, package losslessly, replace the shared review page."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-94'
CAPTURE = REVIEW / '_capture'

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    assert not (REVIEW / 'visual-acceptance-01.json').exists()
    review_status = json.loads((REVIEW / 'review.json').read_text(encoding='utf-8'))
    assert not review_status.get('visual_acceptance'), 'Taunt already accepted; do not reopen its review.'
    entries = json.loads((CAPTURE / 'capture.json').read_text(encoding='utf-8'))
    assert len(entries) == 8
    baseline_path = REVIEW / 'gardien-size01/baseline.json'
    baseline = json.loads(baseline_path.read_text(encoding='utf-8')) if baseline_path.exists() else None
    if baseline:
        expected = {e['variant']: e['simulation_sha256'] for e in baseline['scenes']}
        for e in entries:
            data = {k: e[k] for k in ['variant', 'case', 'direction', 'frames', 'fps', 'cast_frame', 'turn_frames', 'timeline']}
            assert hashlib.sha256(json.dumps(data, sort_keys=True, separators=(',', ':')).encode()).hexdigest() == expected[e['variant']]
    checks = []
    for e in entries:
        timeline = e['timeline']
        assert timeline[0]['preview'] and not timeline[0]['launched']
        assert all(not s['preview'] for s in timeline[12:])
        assert all(s['damage'] == 0 for s in timeline)
        assert all(s['icons'] == sum(r > 0 for r in s['remaining']) for s in timeline)
        assert all(len(s['sources']) == 3 and len(s['remaining']) == 3 for s in timeline)
        expected = [0, None, 0] if e['case'] == 'CLOISON' else [0, 1, 0] if e['case'] == 'DEUX_GARDIENS' else [0, 0, 0]
        assert timeline[12]['sources'] == expected
        if e['case'] == 'ATTENTE':
            assert all(s == timeline[12] for s in timeline[12:])
        else:
            assert timeline[54]['turn'] == 1 and timeline[96]['turn'] == 2
            assert timeline[-1]['icons'] == 0
        if e['case'] == 'RAPPEL':
            assert timeline[54]['recalled'] and timeline[54]['icons'] == 0
        count, cols = e['unique_images'], e['columns']
        rows = (count + cols - 1) // cols
        with Image.open(CAPTURE / e['label'] / 'atlas.ppm') as source:
            atlas = Image.new('RGB', source.size)
            hashes = []
            for i in range(count):
                x, y = i % cols * 576, (rows - 1 - i // cols) * 576
                frame = source.crop((x, y, x + 576, y + 576)).convert('RGB')
                atlas.paste(frame, (x, i // cols * 576))
                hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
        destination = REVIEW / (e['label'] + '.webp')
        atlas.save(destination, 'WEBP', lossless=True, method=1)
        with Image.open(destination) as packed:
            for i, h in enumerate(hashes):
                x, y = i % cols * 576, i // cols * 576
                assert hashlib.sha256(packed.crop((x, y, x + 576, y + 576)).convert('RGB').tobytes()).hexdigest() == h
            if e['label'] == 'HAUT':
                frames = []
                for i in e['indices']:
                    x, y = i % cols * 576, i // cols * 576
                    frames.append(packed.crop((x, y, x + 576, y + 576)).copy())
                frames[0].save(REVIEW / 'provocation-gardien-64.webp', 'WEBP', save_all=True,
                    append_images=frames[1:], duration=[42 if i % 3 != 2 else 41 for i in range(len(frames))],
                    loop=0, lossless=True, method=1, minimize_size=False)
                # A compact native contact sheet for pixel-level visual inspection.
                board = Image.new('RGB', (576 * 3, 576), '#111619')
                for col, f in enumerate([16, 36, 105]):
                    board.paste(frames[f], (col * 576, 0))
                board.save(REVIEW / 'native-keyframes.png')
        e['atlas'] = 'skill-animation-review-94/' + destination.name
        checks.append({'variant': e['variant'], 'lossless_pixels': True, 'sha256': digest(destination),
                       'bytes': destination.stat().st_size, 'frame_sha256': hashes})
        print(e['variant'], destination.stat().st_size, flush=True)

    icon_path = CAPTURE / 'D66-TAUNT-I01-native32.png'
    if baseline:
        assert digest(REVIEW / icon_path.name) == baseline['status_icon_sha256']
    with Image.open(icon_path) as source:
        icon = source.convert('RGBA')
        if baseline:
            with Image.open(REVIEW / icon_path.name) as previous_icon:
                assert icon.tobytes() == previous_icon.convert('RGBA').tobytes()
        assert icon.size == (32, 32)
        assert all(icon.getpixel(p)[3] == 0 for p in [(0, 0), (31, 0), (0, 31), (31, 31)])
        assert 0 < sum(a > 0 for a in icon.getchannel('A').get_flattened_data()) < 32 * 32 // 2
        assert icon.getbbox() is not None
        icon.save(REVIEW / icon_path.name)
        icon_bounds = icon.getbbox()
    text = json.dumps(entries, ensure_ascii=False, separators=(',', ':'))
    (REVIEW / 'animations.json').write_text(text, encoding='utf-8')
    template = (ROOT / 'tools/art/guardian_taunt_review_template.html').read_text(encoding='utf-8')
    (BASE / 'apercu-animations.html').write_text(template.replace('__ENTRIES__', text), encoding='utf-8')
    elapsed = 0
    decoded_frames = 0
    top = next(e for e in entries if e['label'] == 'HAUT')
    with Image.open(REVIEW / 'provocation-gardien-64.webp') as animated, Image.open(REVIEW / 'HAUT.webp') as packed:
        for i in range(animated.n_frames):
            animated.seek(i)
            animated.load()
            sample = min(top['frames'] - 1, (elapsed * top['fps'] + 500) // 1000)
            idx = top['indices'][sample]
            x, y = idx % top['columns'] * 576, idx // top['columns'] * 576
            assert animated.convert('RGB').tobytes() == packed.crop((x, y, x + 576, y + 576)).convert('RGB').tobytes()
            elapsed += animated.info['duration']
            decoded_frames += 1
    assert elapsed == 5000
    proof = {'method': 'Native Macroquad capture; lossless per-frame packaging. Browser rendering not inspected.',
             'scenes': 8, 'native_tile': 64, 'frames': 120, 'fps': 24, 'preview_removed_on_cast': True,
             'wall_excludes_target': True, 'no_stacking': True, 'status_survives_clock_only_wait': True,
             'status_ends_on_turn_or_recall': True, 'taunt_damage': 0, 'icon_rgba_size': [32, 32],
             'icon_alpha_bounds': icon_bounds, 'icon_sha256': digest(REVIEW / icon_path.name),
             'standalone_animation_ms': elapsed, 'standalone_decoded_frames': decoded_frames,
             'standalone_animation_matches_native_pixels': True, 'checks': checks}
    if baseline:
        size_review = json.loads((REVIEW / 'gardien-size01/review.json').read_text(encoding='utf-8'))
        candidate = REVIEW / 'gardien-size01/D66-BODY02-SIZE01-native128.png'
        assert digest(candidate) == size_review['candidate_sha256']
        proof.update(body_size_candidate='D66-SIZE01', body_height=64, body_width=80,
                     previous_simulation_timelines_unchanged=True, status_icon_unchanged=True)
    (REVIEW / 'validation.json').write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8')
    print('Compact atlas total MiB', round(sum(c['bytes'] for c in checks) / 2**20, 2))

if __name__ == '__main__':
    main()
