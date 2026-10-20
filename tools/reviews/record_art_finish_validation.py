"""Preserve the completed integration's measured validation receipts."""
from pathlib import Path
import json
import shutil

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01'
OUT = ROOT / 'artifacts/art-quality-audit-01'


def main():
    checks = json.loads((DOC / 'integration-checks.json').read_text('utf-8'))
    assert checks['status'] == 'accepted_and_integrated'
    assert not any(checks['native_panels'].values())
    assert not any(checks['scene_changed_pixels'].values())
    timings = json.loads((OUT / 'finish01-performance/timings.json').read_text('utf-8'))
    assert len(timings['scenes']) == 14
    assert all(row.get('host_unchanged', True) for row in timings['scenes'])
    shutil.copyfile(OUT / 'finish01-performance/timings.json', DOC / 'performance-debug.json')
    data = dict(review='ART-FINISH-01', status='accepted_and_integrated', date='2026-10-19',
                commands=[
                    dict(command='cargo test --locked --bin project-rl game_art:: -- --test-threads=1', passed=43, failed=0),
                    dict(command='cargo build --locked', result='passed'),
                    dict(command='cargo check --locked --release', result='passed; compilation check, not a linked release executable'),
                    dict(command='cargo fmt --all -- --check', result='passed'),
                    dict(command='git diff --check', result='passed'),
                    dict(command='--ui-cold-art-finish-integration + verify_art_finish_integration.py', result='26 samples and 2 scene viewports exactly equal to accepted reference'),
                    dict(command='--ui-cold-performance', result='14 scenes completed; no host-state mutation'),
                ],
                visual_inspection=['native board', 'integrated house and street at native scale'],
                packaged_exports=30, unchanged_previous_asset_files=48,
                runtime_dependency='embedded assets/sprites/art-finish-01 only',
                full_suite_run=False, browser_validation='Static local links verified; no browser automation',
                performance=dict(report='performance-debug.json', comparative_baseline=False,
                    note='Debug diagnostic only; first-use costs remain visible, no speedup claim',
                    world_first_cpu_ms=timings['scenes'][0]['first_cpu_ms']),
                cleanup='integration-cleanup.json: two redundant reference captures removed; sources, map-editor work and accepted references retained')
    (DOC / 'integration-validation.json').write_text(json.dumps(data, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print('Recorded: 43 graphics tests, debug build, release check, exact GPU comparisons and 14 diagnostic scenes')


if __name__ == '__main__':
    main()
