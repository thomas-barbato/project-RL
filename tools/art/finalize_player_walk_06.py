"""Record completed WALK06 checks and retain only useful capture evidence."""
import hashlib,json,shutil
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-06'
ART=ROOT/'artifacts/player-walk-review-06/engine'
def read(path):return json.loads(path.read_text('utf-8'))
def write(path,data):path.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n','utf-8')

runtime=read(ART/'checks.json');pixels=read(DOC/'engine-pixel-checks.json')
assert runtime['status']=='passed' and runtime['accepted_rifle_model']==2
assert pixels['native_framebuffer_comparisons']==100 and pixels['minimum_opaque_pixel_match']==1
evidence=DOC/'engine';evidence.mkdir(exist_ok=True)
for filename in ['camp-step-200ms.png','camp-rest-400ms.png','native-tenue-1-sombre.png','native-tenue-1-clair.png']:
    shutil.copyfile(ART/filename,evidence/filename)
exe=ROOT/'target/release/project-rl.exe'
report={'review_id':'WALK06','status':'integrated','date':'2026-10-23',
    'scope':'25 class armor silhouettes, assault rifle model2, Original palettes, no extra equipped helmet',
    'acceptance':'acceptance.json','runtime_manifest':'assets/animations/walk-06/manifest.json',
    'rejected_walk04_enabled':False,'vertical_rendering_changed':False,
    'default_duration_ms':400,'duration_choices_ms':[120,200,400],
    'review_pause_added_to_gameplay':False,'release_build':'cargo build --locked --release --bin project-rl',
    'release_build_passed':True,'release_sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),
    'targeted_walk_renderer_tests':{'passed':6,'failed':0},
    'broader_walk_filter':{'passed':18,'failed':1,'ignored':1,
        'failure':'ascii_app::world_context::tests::world_context_attack_without_ammo_never_walks',
        'assertion':'app.world_context.is_some() at src/world_context.rs:418; context-menu opening before drawing',
        'reproduced_in_isolation':True,'baseline_checkout_not_built':True,
        'note':'No input/context-menu/simulation code changed by WALK06; failure is reported separately, not counted as a passing suite.'},
    'rustfmt_touched_modules':True,'git_diff_check':True,
    'portable_inventory_contains_all_six_walk06_files':True,
    'runtime_capture':runtime,'pixel_checks':'engine-pixel-checks.json',
    'native_pixel_comparisons':100,'minimum_opaque_pixel_match':1.0,
    'user_settings_or_campaign_written':False,
    'texture_cache_budget_bytes':8*1024*1024,'maximum_five_atlases_rgba_bytes':5*448*640*4,
    'next_requested_work':['other weapons, hand configurations and helmets','game monsters'],
    'next_lots_require_visual_review':True}
write(DOC/'integration-checks.json',report)
for path in [DOC/'packaging.json',DOC/'compatibility-contract.json']:
    data=read(path);data['runtime_integration' if path.name=='packaging.json' else 'runtime_integrated']=True
    data['integrated_scope']=report['scope'];write(path,data)
path=ROOT/'docs/reviews/player-walk-05/adaptation-coverage.json';data=read(path)
data['partial_runtime_integration']='WALK06: 25 class/rifle configurations only'
for entry in data['entries']:
    if entry['id'].startswith('CLASS-'):entry['integrated_configuration']=report['scope']
write(path,data)

# These exact, reproducible framebuffer frames were inspected before cleanup.
# Keep native comparisons, the current review and accepted sources.
removed=[]
for filename in [f'walk-live-{i:02}.png' for i in range(30)]:
    target=ART/filename
    if not target.exists():continue
    if target.is_symlink() or not target.resolve().is_relative_to(ART.resolve()):
        raise RuntimeError(f'Unsafe generated capture path: {target}')
    target.unlink();removed.append(filename)
write(DOC/'capture-cleanup.json',{'removed_reproducible_frames':removed,'sources_preserved':True,
    'native_engine_comparisons_preserved':True,'current_review_preserved':True,'map_editor_touched':False})
print(json.dumps({'status':'integrated','native_comparisons':100,'walk_tests_passed':6,
    'broader_test_failure_reported':True,'redundant_capture_frames_removed':len(removed)},indent=2))
