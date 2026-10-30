"""C12 complete native engine, body-size and preserved-resource checks."""
from pathlib import Path
import json,hashlib
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
OUT=ROOT/'artifacts/vertical-walk-review-18/current/grips-C12'
ART=ROOT/'assets/source-art/vertical-walk-review-18/grips-C12'
ENGINE=OUT/'engine-r02'

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')

def main():
    native=read(ENGINE/'checks.json');assert native['review']=='VERT18-C12'
    assert native['prepared_atlases']==1104 and native['native_draws']==14352
    assert native['front_returns']==4416 and native['saved_contact_boards']==72
    assert native['same_actor_finish_every_pose'] and not native['read_decode_in_draw']
    assert not native['normal_game_enabled'] and not native['saved_game_loaded'] and not native['preferences_written']
    poses=read(ART/'conditioned/manifest.json');assert poses['prepared_families']==36 and not poses['failures']
    heights=[]
    for p in poses['profiles']:
        for name,family in p['family_poses'].items():
            values=[f['bounds'][3]-f['bounds'][1] for f in family['frames']]
            assert abs(values[12]-p['target_height'])<=2,(p['id'],name,values)
            assert max(values[:12])-min(values[:12])<=4,(p['id'],name,values)
            heights.append({'profile':p['id'],'family':name,'target':p['target_height'],'idle':values[12],'moving_range':[min(values[:12]),max(values[:12])]})
    groups=[]
    for name in ['bastion','traqueuse','arcaniste','metamorphe']:
        for family in ['ranged','melee-wide','melee-close']:
            folder=ENGINE/('camp-'+name+'-'+family);camp=read(folder/'camp-checks.json')
            assert camp['review']=='VERT18-C12' and camp['camp_material_and_irradiance']
            assert len(camp['records'])==156 and camp['front_returns']==12
            for record in camp['records']:
                assert abs(record['actor_gain']-1.48)<0.00001
                assert record['pose'] is None if record['tick']==12 else record['pose'][0]==(-2 if record['step']<2 else 2)
            for step in [0,2]:
                for tick in [0,6,12]:assert (folder/f'camp-step{step}-tick{tick:02}.png').exists()
            Image.open(folder/'camp-step0-tick06.png').crop((16,60,592,572)).save(folder/'native-crop.png')
            groups.append({'class':name,'family':family,'pose_records':156,'front_returns':12,'native_captures':6})
    preserved={}
    for bank in ['runtime-assets-C09','candidate-resources-C06','candidate-resources-C08','candidate-resources-C09','candidate-resources-C10','candidate-resources-C11','candidate-resources-C12']:
        entries=read(DOC/(bank+'.json'))['sha256']
        for path,sha in entries.items():
            if bank=='runtime-assets-C09' and path=='assets/animations/vertical-18/manifest.json' and (DOC/'runtime-assets-C12.json').exists():
                assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==read(DOC/'runtime-assets-C12.json')['sha256'][path]
                continue
            assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==sha,path
        preserved[bank]=len(entries)-int(bank=='runtime-assets-C09' and (DOC/'runtime-assets-C12.json').exists())
    report=read(DOC/'ANALYSE-C12.json');assert report['contact_checks']==12432 and not report['contact_failures']
    status='native_candidates_awaiting_visual_approval'
    if (DOC/'acceptance-C12.json').exists():
        accepted=read(DOC/'acceptance-C12.json')
        assert hashlib.sha256((DOC/accepted['candidate_resources']).read_bytes()).hexdigest()==accepted['candidate_resources_sha256']
        status='visual_accepted_integrated' if accepted['normal_game_enabled'] else 'visual_accepted_pending_integration'
    report.update(status=status,body_size_audit=heights,engine={'native_configurations':1104,'native_draws':14352,'front_returns':4416,'saved_contact_boards':72,'camp_groups':groups,'camp_pose_records':1872,'camp_front_returns':144,'normal_game_enabled':False,'saved_game_loaded':False,'preferences_written':False,'read_decode_in_draw':False},preserved_hash_banks=preserved)
    write(DOC/'ANALYSE-C12.json',report);write(DOC/'native-checks-C12.json',native)
    print(json.dumps({'review':'VERT18-C12','native_draws':14352,'two_grip_contact_checks':12432,'camp_records':1872,'preserved_hash_entries':sum(preserved.values()),'normal_game_enabled':False}))

if __name__=='__main__':main()
