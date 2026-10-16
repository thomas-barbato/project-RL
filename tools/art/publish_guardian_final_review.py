"""Publish the remaining taunt review using the existing native frames unchanged."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
ORIGINAL = BASE / 'skill-animation-review-94'
REVIEW = BASE / 'skill-animation-review-103'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def main():
    assert not (ORIGINAL / 'visual-acceptance-01.json').exists(), 'Taunt already accepted; do not reopen.'
    assert not (REVIEW / 'visual-acceptance-01.json').exists(), 'Final review already accepted.'
    entries = read(ORIGINAL / 'animations.json')
    validation = read(ORIGINAL / 'validation.json')
    assert len(entries) == validation['scenes'] == 8
    assert validation['body_height'] == 64 and validation['body_width'] == 80
    expected = {c['variant']: c for c in validation['checks']}
    checks = []
    for e in entries:
        image_path = BASE / e['atlas']
        check = expected[e['variant']]
        assert digest(image_path) == check['sha256'], image_path
        with Image.open(image_path) as atlas:
            for i, frame_hash in enumerate(check['frame_sha256']):
                x, y = i % e['columns'] * 576, i // e['columns'] * 576
                frame = atlas.crop((x, y, x + 576, y + 576)).convert('RGB')
                assert hashlib.sha256(frame.tobytes()).hexdigest() == frame_hash
        assert len(e['timeline']) == len(e['indices']) == e['frames'] == 120
        assert e['fps'] == 24
        states, state_indices, lookup = [], [], {}
        for state in e['timeline']:
            key = json.dumps(state, sort_keys=True, separators=(',', ':'))
            if key not in lookup:
                lookup[key] = len(states)
                states.append(state)
            state_indices.append(lookup[key])
        assert [states[i] for i in state_indices] == e['timeline']
        checks.append({'variant': e['variant'], 'atlas': e['atlas'], 'sha256': check['sha256'],
                       'native_frame_hashes_verified': len(check['frame_sha256']),
                       'timeline_sha256': hashlib.sha256(json.dumps(e['timeline'], sort_keys=True).encode()).hexdigest()})
        e['states'], e['state_indices'] = states, state_indices
        del e['timeline']
    icon = ORIGINAL / 'D66-TAUNT-I01-native32.png'
    assert digest(icon) == validation['icon_sha256']
    preserved = read(BASE / 'skill-animation-review-102/preservation.json')
    # Preserve the accepted revised Contagion and its receipt as well as all prior references.
    for name in ['visual-acceptance-01.json', 'AF06-A02-organic-native-64.webp',
                 'AF06-TENTACLE02-source.png', 'review.json']:
        path = BASE / 'skill-animation-review-102' / name
        preserved[str(path.relative_to(ROOT)).replace('\\', '/')] = digest(path)
    for rel, sha in preserved.items():
        assert digest(ROOT / rel) == sha, rel
    template = (ROOT / 'tools/art/guardian_final_review_template.html').read_text(encoding='utf-8')
    html = template.replace('__ENTRIES__', json.dumps(entries, ensure_ascii=False, separators=(',', ':')))
    assert '__ENTRIES__' not in html
    REVIEW.mkdir(exist_ok=True)
    write(REVIEW / 'preservation.json', preserved)
    write(REVIEW / 'review.json', {
        'date': '2026-10-15', 'review_id': 'D66-TAUNT-FINAL01',
        'status': 'taunt_visuals_pending',
        'original_review': '../skill-animation-review-94/review.json',
        'candidates': [
            {'number': 101, 'id': 'D66', 'variant': 'D66-TAUNT-A01', 'title': 'Provocation du Gardien'},
            {'number': 103, 'id': 'D66', 'variant': 'D66-TAUNT-I01', 'title': 'Marqueur de provocation'}],
        'body_and_size_already_accepted': '../skill-animation-review-94/gardien-size01/visual-acceptance-01.json',
        'native_tile': 64, 'native_scenes': 8, 'directions': 4,
        'scope': 'Existing taunt impulse and marker only. Native pixels and original timelines preserved; no body reapproval.',
        'separate_pending': ['O03-A01 visual', 'B03 mechanics', 'Shared drone capacity and taunt numbers'],
        'campaign_integration': False,
        'source': 'src/agent_skill_lab_review/fire_painted/guardian_taunt.rs',
        'source_sha256': digest(ROOT / 'src/agent_skill_lab_review/fire_painted/guardian_taunt.rs'),
        'status_icon': {'path': 'skill-animation-review-94/' + icon.name, 'sha256': digest(icon)},
        'standalone_animation': {'path': 'skill-animation-review-94/provocation-gardien-64.webp',
                                 'sha256': digest(ORIGINAL / 'provocation-gardien-64.webp')},
        'preview': '../apercu-animations.html',
    })
    write(REVIEW / 'validation.json', {
        'method': 'Existing native Macroquad frame hashes and complete timelines checked; no browser rendering claim.',
        'scenes': checks, 'native_pixels_unchanged': True, 'mechanical_timelines_unchanged': True,
        'preserved_files': len(preserved), 'new_capture_files': 0, 'new_animation_exports': 0,
    })
    (BASE / 'apercu-animations.html').write_text(html, encoding='utf-8')
    print(json.dumps({'scenes': len(entries), 'page_bytes': len(html.encode()), 'preserved_files': len(preserved),
                      'new_capture_files': 0, 'native_pixels_unchanged': True}))


if __name__ == '__main__':
    main()
