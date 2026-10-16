"""Keep O03 mechanics identical to the earlier review; show current accepted bodies."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-104'
CAPTURE = REVIEW / '_capture'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def main():
    assert not (REVIEW / 'visual-acceptance-01.json').exists()
    assert read(REVIEW / 'review.json').get('status') != 'no_dedicated_animation_required', 'O03 circuit removed at the player request; do not republish the rejected effect.'
    entries = read(CAPTURE / 'capture.json')
    assert len(entries) == 16 and all(e['id'] == 'O03' for e in entries)
    reference = [e for e in read(BASE / 'skill-animation-review-93/animations.json') if e['id'] == 'O03']
    checks = []
    qa = Image.new('RGB', (576 * 3, 576 * 3))
    for e in entries:
        old = next((r for r in reference if r['case'] == e['case'] and r['direction'] == e['direction']),
                   next(r for r in reference if r['case'] == e['case']))
        assert e['timeline'] == old['timeline'], e['variant']
        for state in e['timeline']:
            assert state['energy'] == 100 and state['cooldown'] == 0 and state['deployments'] == 0
            assert state['passive_bonus_percent'] == 25 and not state['preview']
        count, cols = e['unique_images'], e.pop('native_columns')
        rows = (count + cols - 1) // cols
        with Image.open(CAPTURE / e['folder'] / 'atlas.ppm') as raw:
            packed = Image.new('RGB', raw.size)
            hashes = []
            for i in range(count):
                x, y = i % cols * 576, (rows - 1 - i // cols) * 576
                frame = raw.crop((x, y, x + 576, y + 576)).convert('RGB')
                hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
                packed.paste(frame, (x, i // cols * 576))
        destination = REVIEW / (e['folder'] + '.webp')
        packed.save(destination, 'WEBP', lossless=True, method=1)
        with Image.open(destination) as encoded:
            for i, digest in enumerate(hashes):
                x, y = i % cols * 576, i // cols * 576
                assert hashlib.sha256(encoded.crop((x, y, x + 576, y + 576)).convert('RGB').tobytes()).hexdigest() == digest
            if e['folder'] in ['O03-HAUT', 'O03-TRAQUEUR', 'O03-ARTILLEUR']:
                frames = []
                for i in e['indices']:
                    x, y = i % cols * 576, i // cols * 576
                    frames.append(encoded.crop((x, y, x + 576, y + 576)).copy())
                standalone = REVIEW / (e['folder'] + '-native-64.webp')
                frames[0].save(standalone, 'WEBP', save_all=True, append_images=frames[1:],
                               duration=[42 if i % 3 != 2 else 41 for i in range(len(frames))],
                               loop=0, lossless=True, method=1, minimize_size=False)
                row = ['O03-HAUT', 'O03-TRAQUEUR', 'O03-ARTILLEUR'].index(e['folder'])
                for col, index in enumerate([14, 17, 36]):
                    qa.paste(frames[index], (col * 576, row * 576))
        states, indices, lookup = [], [], {}
        for state in e.pop('timeline'):
            key = json.dumps(state, sort_keys=True, separators=(',', ':'))
            if key not in lookup:
                lookup[key] = len(states)
                states.append(state)
            indices.append(lookup[key])
        e.update(columns=cols, atlas='skill-animation-review-104/' + destination.name,
                 states=states, state_indices=indices, cast_frame=12, turn_frames=[48, 72, 96])
        checks.append({'variant': e['variant'], 'path': destination.name, 'sha256': sha(destination),
                       'bytes': destination.stat().st_size, 'lossless_pixels': True,
                       'mechanical_timeline_identical_to_previous_review': True,
                       'frame_sha256': hashes})
        print(e['variant'], destination.stat().st_size, flush=True)
    for rel, digest in read(REVIEW / 'preservation.json').items():
        assert sha(ROOT / rel) == digest, rel
    qa.save(REVIEW / '_native-qa.png')
    write(REVIEW / 'animations.json', entries)
    write(REVIEW / 'validation.json', {
        'method': 'Native Macroquad capture; lossless per-frame verification and exact O03 mechanical timeline equality with review93.',
        'native_tile': 64, 'scenes': 16, 'frames': 144, 'fps': 24, 'directions_for_each_armed_model': 4,
        'active_catalogue_unchanged': True, 'historical_review_source_unchanged': True, 'checks': checks,
    })
    template = (ROOT / 'tools/art/combat_algorithms_review_template.html').read_text(encoding='utf-8')
    html = template.replace('__ENTRIES__', json.dumps(entries, ensure_ascii=False, separators=(',', ':')))
    assert '__ENTRIES__' not in html
    (BASE / 'apercu-animations.html').write_text(html, encoding='utf-8')


if __name__ == '__main__':
    main()
