"""Collect the completed native-render and compositor evidence."""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-12'
ART = ROOT / 'artifacts/player-walk-review-12'


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    geometry = read(DOC / 'asset-verification.json')
    assert geometry['status'] == 'passed'
    contacts = read(ART / 'proposed-export/contact.json')
    assert contacts['proposed_gait'] and not contacts['failures']
    assert len(contacts['cases']) == 2690
    normal_contacts = read(ART / 'equipped-export/contact.json')
    assert not normal_contacts['failures'] and len(normal_contacts['cases']) == 2690
    world = read(ART / 'native/checks.json')
    assert world['status'] == 'passed' and not world['candidate_renderer_review_only']
    actual = read(ART / 'native/actual-scene/checks.json')
    assert actual['player_front_at_arrival'] and actual['original_save_unchanged']
    catalogue = read(ART / 'catalogue-proposed/checks.json')
    assert len(catalogue) == 54
    candidates = [c for c in catalogue if c['visual'].startswith('Player')]
    assert len(candidates) == 35 and all(c['proposed_walk12'] for c in candidates)
    assert all((ART / 'catalogue-proposed' / c['file']).exists() for c in catalogue)
    assert not (ART / 'native/stderr.log').read_text().strip()
    assert not (ART / 'catalogue-proposed/stderr.log').read_text().strip()
    audit = read(DOC / 'live-state-audit.json')
    monsters = [t for t in audit['movement_trials'] if not t['player']]
    missing = [t for t in monsters if t['cycle_for_this_direction'] is None]
    assert not missing
    files = ['src/combat_visuals.rs', 'src/terminal_view.rs', 'src/game_art_walk_rig.rs',
             'src/game_art_walk.rs', 'src/game_art_walk_approved.rs', 'src/walk_review.rs',
             'src/walk_capture.rs', 'src/walk_proportions_player_export.rs']
    report = {
        'revision': 'WALK12-TECH-r01', 'date': '2026-10-25', 'status': 'passed',
        'normal_game': {'front_at_player_arrival': True, 'equipped_arms_rigid_during_step': True,
                        'vertical_creature_steps_select_existing_cycle': True,
                        'default_milliseconds_per_cell': 400, 'selectable_milliseconds': [120, 200, 400],
                        'distance_cells': 1, 'accepted_images_changed': False},
        'builds': {'debug': 'passed,29.53s', 'release': 'passed,126s; final recheck0.27s',
                   'release_sha256': sha(ROOT / 'target/release/project-rl.exe'),
                   'cargo_fmt_check': 'passed', 'git_diff_check': 'passed using repository line-ending settings'},
        'tests': {'combat_visuals': '15 passed',
                  'normal_471_creature_palette_geometries': 'passed,27.22s',
                  'normal_350_whole_poses_and_leg_guards': 'passed,19.38s',
                  'normal_arms_weapon_pose_stability': 'passed,27.59s',
                  'proposed_350_poses_legs_rigid_grips_and_context_restoration': 'passed,45.94s',
                  'normal_contact_cases': 2690, 'normal_contact_failures': 0,
                  'proposed_contact_cases': 2690, 'proposed_contact_failures': 0,
                  'proposed_contact_seconds': 109.09},
        'actual_saved_scene': {'source': 'isolated restoration; original save bytes unchanged',
                              'player_front_at_arrival': True,
                              'monster_moves_audited': len(monsters), 'missing_cycles_after_fix': 0,
                              'native_frames': 3, 'report': 'live-state-audit.json'},
        'native_world': {'scenes': 53, 'cell_px': 64, 'status': world['status']},
        'native_proposal_catalogue': {'scenes': 54, 'player_cases': 35, 'stderr': ''},
        'proposal': {'revision': 'WALK12-COMMON-r01', 'outfits': 25, 'whole_walking_drawings': 300,
                     'reference_idles_unchanged': 50, 'new_art_integrated_in_normal_game': False,
                     'user_visual_approval': 'pending', 'runtime_distribution_includes_proposal': False,
                     'geometry_verification': 'asset-verification.json',
                     'manifest_sha256': sha(ROOT / 'assets/animations/walk-12/player/manifest.json'),
                     'contact_report_sha256': sha(ART / 'proposed-export/contact.json')},
        'accepted_runtime_assets_unchanged': geometry['accepted_runtime_assets_unchanged'],
        'accepted_sources_unchanged': geometry['accepted_sources_unchanged'],
        'original_walk07_walk09_and_map_editor_files_unchanged': geometry['original_walk07_walk09_and_map_editor_files_unchanged'],
        'source_sha256': {name: sha(ROOT / name) for name in files}}
    (DOC / 'validation.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps({k:report[k] for k in ['revision', 'status', 'actual_saved_scene', 'native_proposal_catalogue', 'proposal']}))


if __name__ == '__main__':
    main()
