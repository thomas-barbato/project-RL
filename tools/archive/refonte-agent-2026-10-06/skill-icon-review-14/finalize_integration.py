"""Store the completed IC14 asset, build, test and native capture validation."""
from pathlib import Path
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
NATIVE = HERE.parent / 'skill-window-native-03'


def read(path):
    return json.loads(path.read_text('utf-8-sig'))


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')


if (HERE / 'integration-validation.json').exists():
    raise RuntimeError('IC14 completion has already been recorded; keep its receipt frozen.')
checks = read(HERE / 'integration-checks.json')
approval = read(HERE / 'visual-acceptance-01.json')
captures = []
for c in read(NATIVE / 'capture-results.json'):
    image = Path(c['file'])
    assert image.is_relative_to(ROOT)
    assert c['process_exit'] == 0
    assert digest(image) == c['sha256']
    size = struct.unpack('>II', image.read_bytes()[16:24])
    assert size == (960, 540)
    captures.append({**c, 'file': image.relative_to(ROOT).as_posix(),
                     'dimensions': list(size), 'visual_inspection': 'passed'})
assert len(captures) == 8
assert {c['skill'] for c in captures} == {'D56', 'D57', 'D58', 'D66', 'DP01', 'DR01', 'DP02', 'HY07'}
record = {**checks, 'status': 'integrated', 'accepted_compacts': approval['accepted_compact'],
          'tests': {'command': 'cargo test --locked --bin project-rl agent_skill -- --nocapture', 'passed': 4, 'failed': 0},
          'validation': {'format': 'passed', 'diff_whitespace': 'passed', 'debug_build': 'passed',
                         'release_build': 'passed', 'release_warnings': '4 pre-existing warnings'},
          'captures': captures,
          'builds': {mode: {'file': f'target/{mode}/project-rl.exe', 'sha256': digest(ROOT / f'target/{mode}/project-rl.exe')}
                     for mode in ['debug', 'release']}}
write(HERE / 'integration-validation.json', record)
write(NATIVE / 'integration-validation.json', record)
page = NATIVE / 'index.html'
page.write_text(page.read_text('utf-8').replace('src="D56-960/cold-start.png"', 'src="D56-960-verified/cold-start.png"'), 'utf-8')
direction_path = HERE.parent / 'skill-icon-direction.json'
direction = read(direction_path)
direction['pending_compact_batch_review'] = 'SKILLS-IC15'
direction['pending_compact_batch_candidates'] = [
    {'number': c['number'], 'id': c['id'], 'variant': c['variant']}
    for c in read(HERE.parent / 'skill-icon-review-15/review.json')['candidates']]
write(direction_path, direction)
print(f'IC14 integrated: {len(captures)} native captures, {checks["packaged_asset_hashes_verified"]} accepted drawings verified.')
