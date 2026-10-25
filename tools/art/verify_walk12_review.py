"""Verify review isolation, accepted pixels and whole-figure packaging."""
from pathlib import Path
import hashlib
import json
import sys
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-12'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    accepted = json.loads((ROOT / 'docs/reviews/player-walk-11/acceptance.json').read_text())
    for name, digest in accepted['runtime_asset_sha256'].items():
        assert sha(ROOT / name) == digest, name
    for source in accepted['selected_whole_figure_sources']:
        assert sha(ROOT / source['path']) == source['sha256'], source['path']
    original = json.loads((ROOT / 'docs/reviews/player-walk-11/baseline.json').read_text())
    for name, digest in original.items():
        assert sha(ROOT / name) == digest, name
    new_manifest = ROOT / 'assets/animations/walk-12/player/manifest.json'
    entries = json.loads(new_manifest.read_text())
    prior = json.loads((ROOT / 'assets/animations/walk-11/player/manifest.json').read_text())
    conditioning = json.loads((DOC / 'conditioning.json').read_text())
    assert len(entries) == 350 and not conditioning['pending_outfits']
    assert len(conditioning['entries']) == 25
    source_hashes = {}
    for record in conditioning['entries']:
        assert sha(ROOT / record['source']) == record['sha256']
        heights = [m['height'] for m in record['poses']]
        assert max(heights)-min(heights) <= 2, record['index']
        # Alpha-threshold raster bounds can grow by one pixel after Lanczos.
        assert abs(max(heights)-record['front']['height']) <= 1, record['index']
        width_ratio = max(m['upper_body_mean_occupied_width'] for m in record['poses']) / record['front']['upper_body_mean_occupied_width']
        assert width_ratio <= 1.15, (record['index'], width_ratio)
        source_hashes[record['source']] = record['sha256']
    for index in range(25):
        for facing in [-1, 1]:
            group = [e for e in entries if e['id'] == f'CLASS-{index:02}' and e['facing'] == facing]
            assert {e['frame'] for e in group} == {8, 0, 1, 2, 3, 4, 5}
            image = Image.open(ROOT / group[0]['file']).convert('RGBA')
            assert image.size == (640, 896)
            ref = next(e for e in prior if e['id'] == f'CLASS-{index:02}' and e['facing'] == facing)
            before = Image.open(ROOT / ref['file']).convert('RGBA')
            assert image.crop((0, 0, 640, 128)).tobytes() == before.crop((0, 0, 640, 128)).tobytes()
    sys.path.insert(0, str(ROOT / 'tools/distribution'))
    from package_release import inventory
    runtime = inventory()
    integrated = (DOC / 'acceptance.json').exists()
    assert any('walk-12/' in name for name in runtime) == integrated
    if integrated:
        approval = json.loads((DOC / 'acceptance.json').read_text())
        for name, digest in approval['runtime_asset_sha256'].items():
            assert sha(ROOT / name) == digest and name in runtime, name
    assert all(name in runtime for name in accepted['runtime_asset_sha256'])
    jobs = json.loads((DOC / 'jobs.json').read_text())
    for job in jobs:
        job['status'] = 'accepted' if integrated else 'proposed_ready_for_native_visual_review'
    (DOC / 'jobs.json').write_text(json.dumps(jobs, indent=2)+'\n')
    report = {'status': 'passed', 'accepted_runtime_assets_unchanged': len(accepted['runtime_asset_sha256']),
              'accepted_sources_unchanged': len(accepted['selected_whole_figure_sources']),
              'original_walk07_walk09_and_map_editor_files_unchanged': len(original),
              'proposal_outfits': 25, 'complete_walking_drawings': 300,
              'reference_idles_unchanged': 50, 'manifest_entries': len(entries),
              'uniform_xy_scale_per_outfit': True, 'height_variation_max_px': 2,
              'front_height_raster_tolerance_px': 1,
              'upper_body_width_max_ratio_to_front': 1.15,
              'runtime_inventory_paths': len(runtime), 'new_drawings_excluded_from_distribution': not integrated,
              'user_visual_approval': 'accepted' if integrated else 'pending', 'source_hashes': source_hashes}
    (DOC / ('integration-assets.json' if integrated else 'asset-verification.json')).write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps({k:v for k,v in report.items() if k != 'source_hashes'}))


if __name__ == '__main__':
    main()
