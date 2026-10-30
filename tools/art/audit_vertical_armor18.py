"""Verify the C10 isolated engine outputs and preserve their review evidence."""
from pathlib import Path
import hashlib,json
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
OUT=ROOT/'artifacts/vertical-walk-review-18/current/armor-C10'

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')

def main():
    native=read(OUT/'engine/checks.json')
    assert native['review']=='VERT18-C10'
    assert native['prepared_atlases']==420 and native['native_draws']==5460
    assert native['front_returns']==1680 and native['saved_contact_boards']==36
    assert native['same_actor_finish_every_pose'] and not native['read_decode_in_draw']
    assert not native['normal_game_enabled'] and not native['saved_game_loaded'] and not native['preferences_written']
    cases=read(ROOT/'assets/source-art/vertical-walk-review-18/armor-C10/engine-cases.json')
    for case in cases:
        if case['save']:assert (OUT/'engine'/(case['id']+'.png')).exists(),case['id']
    groups=[]
    for group in ['bastion','traqueuse','arcaniste','metamorphe']:
        folder=OUT/'engine'/('camp-'+group);camp=read(folder/'camp-checks.json')
        assert camp['review']=='VERT18-C10' and camp['native_tile_px']==64
        assert camp['camp_material_and_irradiance'] and camp['moving_and_idle_actor_gain']==1.48
        assert camp['front_returns']==12 and len(camp['records'])==156
        for record in camp['records']:
            assert abs(record['actor_gain']-1.48)<0.00001
            assert record['pose'] is None if record['tick']==12 else record['pose'][0]==(-2 if record['step']<2 else 2)
        captures=[]
        for step in [0,2]:
            for tick in [0,6,12]:
                file=folder/f'camp-step{step}-tick{tick:02}.png';im=Image.open(file);assert im.size==(960,760)
                captures.append(file.relative_to(ROOT).as_posix())
        # Same pixels and native scale, removing only the fixture chrome.
        original=Image.open(folder/'camp-step0-tick06.png').convert('RGB')
        original.crop((16,60,592,572)).save(folder/'native-crop.png')
        groups.append({'class':group,'full_native_captures':captures,'moving_idle_returns':camp['front_returns']})
    preserved={}
    for bank in ['runtime-assets-C09','candidate-resources-C06','candidate-resources-C08','candidate-resources-C09']:
        entries=read(DOC/(bank+'.json'))['sha256']
        for path,digest in entries.items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest,path
        preserved[bank]=len(entries)
    report=read(DOC/'ANALYSE-C10.json')
    report.update(status='native_candidates_awaiting_visual_approval',engine={'command':'RL_VERTICAL_REVIEW_CASES=assets/source-art/vertical-walk-review-18/armor-C10/engine-cases.json target/debug/project-rl.exe --ui-cold-vertical-extension artifacts/vertical-walk-review-18/current/armor-C10/engine','prepared_atlases':420,'native_draws':5460,'front_returns':1680,'saved_native_contact_boards':36,'camp_groups':groups,'camp_pose_records':624,'camp_front_returns':48,'same_actor_finish_and_exposure':True,'read_decode_in_draw':False,'normal_game_enabled':False,'saved_game_loaded':False,'preferences_written':False},preserved_hash_banks=preserved,validation={'cargo_build_locked':'passed','python_compile':'passed','git_diff_check':'passed (existing LF/CRLF notices only)','full_test_suite_run':False})
    write(DOC/'ANALYSE-C10.json',report)
    write(DOC/'native-checks-C10.json',native)
    print(json.dumps({'review':'VERT18-C10','native_poses':5460,'camp_pose_records':624,'preserved_hashes':sum(preserved.values()),'normal_game_enabled':False}))

if __name__=='__main__':main()
