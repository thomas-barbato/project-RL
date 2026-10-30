"""C11 native proof and unchanged-resource audit, without accepting the art."""
from pathlib import Path
import json,hashlib
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
OUT=ROOT/'artifacts/vertical-walk-review-18/current/knife-contact-C11'

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')

def main():
    native=read(OUT/'engine/checks.json');assert native['review']=='VERT18-C11'
    assert native['prepared_atlases']==192 and native['native_draws']==2496 and native['front_returns']==768
    assert native['saved_contact_boards']==72 and native['same_actor_finish_every_pose']
    assert not native['read_decode_in_draw'] and not native['normal_game_enabled']
    assert not native['saved_game_loaded'] and not native['preferences_written']
    groups=[]
    for group in ['bastion','traqueuse','arcaniste','metamorphe']:
        folder=OUT/'engine'/('camp-'+group);check=read(folder/'camp-checks.json')
        assert check['review']=='VERT18-C11' and check['camp_material_and_irradiance']
        assert check['front_returns']==12 and len(check['records'])==156
        assert all(abs(v['actor_gain']-1.48)<0.00001 for v in check['records'])
        for step in [0,2]:
            for tick in [0,6,12]:assert (folder/f'camp-step{step}-tick{tick:02}.png').exists()
        Image.open(folder/'camp-step0-tick06.png').crop((16,60,592,572)).save(folder/'native-crop.png')
        groups.append({'class':group,'pose_records':156,'front_returns':12,'native_captures':6})
    preserved={}
    for bank in ['runtime-assets-C09','candidate-resources-C06','candidate-resources-C08','candidate-resources-C09','candidate-resources-C10']:
        entries=read(DOC/(bank+'.json'))['sha256']
        for path,sha in entries.items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==sha,path
        preserved[bank]=len(entries)
    report=read(DOC/'ANALYSE-C11.json');assert report['hilt_finger_contact_checks']==3744 and not report['hilt_finger_contact_failures']
    report.update(status='native_candidates_awaiting_visual_review',engine={'native_configurations':192,'native_draws':2496,'front_returns':768,'saved_contact_boards':72,'camp_groups':groups,'camp_pose_records':624,'camp_front_returns':48,'normal_game_enabled':False,'saved_game_loaded':False,'preferences_written':False},preserved_hash_banks=preserved)
    write(DOC/'ANALYSE-C11.json',report);write(DOC/'native-checks-C11.json',native)
    print(json.dumps({'review':'VERT18-C11','native_draws':2496,'hilt_finger_contacts':3744,'camp_records':624,'preserved_files':sum(preserved.values()),'normal_game_enabled':False}))

if __name__=='__main__':main()
