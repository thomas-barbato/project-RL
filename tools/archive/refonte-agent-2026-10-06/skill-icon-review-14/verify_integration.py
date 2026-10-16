"""Verify that IC14 packaging adds only the eight accepted compact images."""
from pathlib import Path
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
RUNTIME = ROOT / 'assets/skills/agent'
IDS = {'D56', 'D57', 'D58', 'D66', 'DP01', 'DR01', 'DP02', 'HY07'}


def read(path):
    return json.loads(path.read_text('utf-8-sig'))


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    baseline = read(HERE / 'integration-baseline.json')
    ledger = read(HERE.parent / 'skill-icon-acceptance.json')
    assert ledger['accepted'] == baseline['previous_accepted']
    count = len(baseline['previous_compact_accepted'])
    assert ledger['compact_accepted'][:count] == baseline['previous_compact_accepted']
    assert {c['id'] for c in ledger['compact_accepted'][count:]} == IDS
    exports = [f for c in ledger['accepted'] + ledger['compact_accepted'] for f in c['files'].values()]
    for f in exports:
        assert digest(ROOT / f['file']) == f['sha256'], f['file']
    previous_images = [f for f in baseline['runtime_files'] if f['file'].endswith('.png')]
    for f in previous_images:
        assert digest(ROOT / f['file']) == f['sha256'], f['file']
    before, after = baseline['catalogue'], read(RUNTIME / 'catalogue.json')
    assert after['skills'] == before['skills']
    assert after['labels'] == before['labels']
    for id, old_art in before['art'].items():
        for category, value in old_art.items():
            assert after['art'][id][category] == value
    manifest = read(RUNTIME / 'manifest.json')
    for drawing in manifest['drawings']:
        assert digest(RUNTIME / drawing['file']) == drawing['sha256']
    for id in IDS:
        file = RUNTIME / after['art'][id]['compact']
        assert struct.unpack('>II', file.read_bytes()[16:24]) == (64, 64)
    old_names = {Path(f['file']).name for f in baseline['runtime_files']}
    assert {p.name for p in RUNTIME.glob('*') if p.name not in old_names} == {id + '-compact.png' for id in IDS}
    pending_ids = {'B01', 'ME04', 'SU04', 'FE05', 'FR03', 'BM02', 'ME02', 'ME03', 'ME05', 'ME06'}
    assert all('compact' not in after['art'].get(id, {}) for id in pending_ids)
    result = {
        'review_id': 'SKILLS-IC14-NATIVE01', 'accepted_compact_count': len(ledger['compact_accepted']),
        'accepted_illustration_count': len(ledger['accepted']), 'accepted_export_hashes_verified': len(exports),
        'packaged_asset_hashes_verified': len(manifest['drawings']),
        'previous_images_unchanged': len(previous_images), 'exactly_eight_new_compact_files': True,
        'native_asset_dimensions': [64, 64],
        'skills_labels_previous_art_and_approvals_unchanged': True, 'other_pending_icons_not_integrated': True,
    }
    (HERE / 'integration-checks.json').write_text(json.dumps(result, indent=2) + '\n', 'utf-8')
    print(json.dumps(result))


if __name__ == '__main__':
    main()
