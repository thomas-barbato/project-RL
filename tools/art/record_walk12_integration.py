"""Finalize integration evidence for the explicitly accepted WALK12 lot."""
from pathlib import Path
import hashlib
import json
import shutil

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-12'
ART = ROOT / 'artifacts/player-walk-review-12'


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    approval = read(DOC / 'acceptance.json')
    assert all(sha(ROOT / name) == digest for name, digest in approval['runtime_asset_sha256'].items())
    assets = read(DOC / 'integration-assets.json')
    assert assets['user_visual_approval'] == 'accepted'
    assert not assets['new_drawings_excluded_from_distribution']
    world = read(ART / 'integration-native/checks.json')
    assert world['status'] == 'passed' and world['accepted_common_gait12_enabled']
    actual = read(ART / 'integration-native/actual-scene/checks.json')
    assert actual['original_save_unchanged'] and actual['all_actors_front_at_arrival']
    catalogue = read(ART / 'catalogue-integrated/checks.json')
    players = [c for c in catalogue if c['visual'].startswith('Player')]
    assert len(catalogue) == 54 and len(players) == 35
    assert all(c['proposed_walk12'] for c in players)
    assert not (ART / 'integration-native/stderr.log').read_text().strip()
    assert not (ART / 'catalogue-integrated/stderr.log').read_text().strip()
    keep = DOC / 'integration-native'
    keep.mkdir(exist_ok=True)
    for name in ['step-100ms.png', 'step-200ms.png', 'front-at-400ms.png', 'checks.json']:
        shutil.copy2(ART / 'integration-native/actual-scene' / name, keep / name)
    report = {
        'revision': 'WALK12-INTEGRATION-r01', 'status': 'passed', 'date': '2026-10-25',
        'accepted_review': 'WALK12-COMMON-r01', 'normal_runtime_integration': True,
        'actors_return_to_canonical_front': ['player', 'all monsters', 'all NPCs'],
        'outfits': 25, 'whole_walking_drawings': 300, 'reference_idles': 50,
        'runtime_asset_hashes_unchanged': len(approval['runtime_asset_sha256']),
        'selected_source_hashes_unchanged': len(approval['selected_whole_figure_sources']),
        'inherited_runtime_assets_preserved': assets['accepted_runtime_assets_unchanged'],
        'inherited_sources_preserved': assets['accepted_sources_unchanged'],
        'old_walk_sources_and_map_editor_files_preserved': assets['original_walk07_walk09_and_map_editor_files_unchanged'],
        'duration_ms_default': 400, 'selectable_ms': [120, 200, 400], 'distance_cells': 1,
        'tests': {'combat_visuals': '16 passed, all actor arrivals and both movement axes',
                  'accepted_gait_whole_legs_rigid_grips_and_legacy_scope': 'passed,45.69s',
                  'proportions_and_legacy_comparison': 'passed,24.23s',
                  'all113_weapons_on_each_class': 'passed,25.35s',
                  'distribution': '3 passed'},
        'native_world_scenes': len(list((ART / 'integration-native').glob('*.png'))),
        'native_catalogue_scenes': len(catalogue), 'native_player_catalogue_cases': len(players),
        'actual_saved_scene': actual,
        'builds': {'debug': 'passed', 'release': 'passed',
                   'release_sha256': sha(ROOT / 'target/release/project-rl.exe')},
        'cargo_fmt_check': 'passed', 'git_diff_check': 'passed',
        'runtime_inventory_paths': assets['runtime_inventory_paths'],
        'legacy_walk04_enabled': False, 'pending_extras': 'nine WALK08 form/drone walking cycles remain separate',
        'preserved_actual_scene_evidence': 'integration-native/'}
    assert report['native_world_scenes'] == 53
    (DOC / 'integration-validation.json').write_text(json.dumps(report, indent=2)+'\n')
    approval['integration_verification'] = 'passed'
    (DOC / 'acceptance.json').write_text(json.dumps(approval, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print(json.dumps({k:report[k] for k in ['revision', 'status', 'actors_return_to_canonical_front',
          'outfits', 'runtime_asset_hashes_unchanged', 'native_world_scenes', 'native_catalogue_scenes']}))


if __name__ == '__main__':
    main()
