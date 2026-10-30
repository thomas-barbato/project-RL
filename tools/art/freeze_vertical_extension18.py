"""Record current C06 candidates and evidence; this is not user acceptance."""
from pathlib import Path
import json,hashlib

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18/extension-C06'
REVIEW=ROOT/'docs/reviews/vertical-walk-18'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/extension-C06'

def read(p):return json.loads(Path(p).read_text('utf8'))
def write(p,v):Path(p).write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n','utf8')

def main():
    geometry=read(PREVIEW/'monster-geometry-audit.json');modular=read(ART/'modular-checks.json');engine=read(PREVIEW/'engine/checks.json');browser=read(PREVIEW/'browser-checks.json')
    recipes=read(ART/'conditioned-weapons/recipes.json');palettes=read(ART/'conditioned-monsters/palettes/manifest.json')
    assert geometry['conditioned_models']==129 and not geometry['conditioning_errors'] and not geometry['size_flags']
    assert not recipes['checks']['contact_failures'] and not browser['errors']
    assert engine['prepared_atlases']==1591 and len(palettes['entries'])==471
    assert read(REVIEW/'preservation-C06.json')['accepted_resources_unchanged']
    focused={}
    for key in ['MON-141','MON-022']:
        d=read(PREVIEW/f'engine/recheck-{key}/checks.json');assert d['prepared_atlases']>0
        focused[key]={k:v for k,v in d.items() if k!='measurements'}
    files=set()
    for folder in ['conditioned-monsters','conditioned-weapons','references']:
        files.update(p for p in (ART/folder).rglob('*') if p.is_file())
    for kind in ['monsters','weapons']:
        d=read(REVIEW/f'prompts-C06-{kind}.json')
        for j in d['jobs']:
            files.add(ROOT/j.get('selected_output',j['output']))
            if j.get('selected_back_source'):files.add(ROOT/j['selected_back_source'])
    for key in ['front','back','opening','ownership']:files.add(ROOT/recipes['headgear'][key])
    files.update([ART/'engine-cases.json',ART/'extra-engine-cases.json',ART/'modular-checks.json'])
    hashes={p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(files)}
    write(REVIEW/'candidate-resources-C06.json',{'review':'VERT18-C06','status':'awaiting_visual_approval','normal_game_enabled':False,'user_accepted':False,'sha256':hashes})
    analysis={'review':'VERT18-C06','date':'2026-10-29','status':'native_review_ready_awaiting_visual_approval','normal_game_enabled':False,
        'scope':{'player_profiles':8,'weapon_models':113,'weapon_palette_choices':7,'one_handed_models':39,'two_handed_models':74,'headgear_choices':7,'monster_models':129,'monster_palette_pairs':471},
        'geometry':{k:v for k,v in geometry.items() if k!='models'},'modular':modular,
        'weapon_contact_checks':{'binding_pose_cases':1216*13,'visible_fist_pixel_contacts':16400,'hidden_two_handed_back_grips_reported_separately':True,'max_visible_contact_distance_px':recipes['checks']['original_color_contact_max_px'],'failures':0,'weapon_scale_changes_within_a_cycle':0},
        'engine':{k:v for k,v in engine.items() if k!='measurements'},'focused_engine_rechecks':focused,
        'browser':{'cases':len(browser['checks']),'failures':len(browser['errors']),'tested_in_app_browser':True,'proofs':['browser-weapons-proof.png','browser-monsters-proof.png']},
        'lossless_engine_contact_boards':len(read(PREVIEW/'engine/browser-captures.json')['captures']),
        'preservation':read(REVIEW/'preservation-C06.json'),
        'build_validation':{'debug_build':'passed','release_check_locked':'passed','targeted_rustfmt':'passed','git_diff_check':'passed','python_py_compile':'passed','full_test_suite_run':False},
        'resource_count':len(hashes),'resource_hashes':'candidate-resources-C06.json',
        'source_mode':'builtin_imagegen','source_location':ART.relative_to(ROOT).as_posix(),
        'not_claimed_complete':['12 current intermediate armor outfits absent from C05','5 legacy Technomancien outfits','transformations and drones','other NPCs or fallback art','normal game integration'],
        'review_document':'REVUE-C06.md','approval':None}
    timings={}
    for key in ['read_ms','decode_ms','upload_ms','board_ms_including_frame_wait']:
        values=sorted(m[key] for m in engine['measurements']);timings[key]={'median':values[len(values)//2],'p95':values[int(len(values)*.95)],'max':max(values)}
    analysis['isolated_fixture_timings_ms']=timings;analysis['timings_are_not_gameplay_fps']=True
    write(REVIEW/'ANALYSE-C06.json',analysis)
    print(json.dumps({'candidate_resources':len(hashes),'browser_checks':analysis['browser']['cases'],'status':analysis['status'],'normal_game_enabled':False}))

if __name__=='__main__':main()
