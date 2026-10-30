"""C11-BACK-R02 proof, with byte-exact preservation of approved fronts."""
from pathlib import Path
import json,hashlib
from PIL import Image,ImageDraw,ImageFont
from finalize_knife_grip18 import cell

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
BASE=ROOT/'assets/source-art/vertical-walk-review-18/knife-contact-C11'
OUT=BASE/'back-r02'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/knife-contact-C11'
ENGINE=PREVIEW/'engine-back-r02'

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')

def main():
    native=read(ENGINE/'checks.json');assert native['review']=='VERT18-C11-BACK-R02'
    assert native['prepared_atlases']==192 and native['native_draws']==2496
    assert native['front_returns']==768 and native['saved_contact_boards']==72
    assert native['same_actor_finish_every_pose'] and not native['read_decode_in_draw']
    assert not native['normal_game_enabled'] and not native['saved_game_loaded'] and not native['preferences_written']
    catalogue=read(OUT/'browser-catalogue.json');fronts=0
    board=Image.new('RGB',(960,696),'#1e2831');draw=ImageDraw.Draw(board);font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),13)
    for i,p in enumerate(catalogue['profiles']):
        for entry in p['cases']:
            old=Image.open(BASE/'composites'/f'{p["id"]}-{entry["loadout"]}-H{entry["head"]}-B.png').convert('RGBA');new=Image.open(OUT/'composites'/(entry['case_B']+'.png')).convert('RGBA')
            for n in [0,1,2,3,4,5,12]:assert cell(old,n).tobytes()==cell(new,n).tobytes();fronts+=1
            if entry['loadout']=='pistol-knife' and entry['head']==0:
                x=i%4*240;y=i//4*232;draw.text((x+4,y+4),p['label'],font=font,fill='white')
                for row,n in enumerate([6,9]):
                    for col,sheet in enumerate([old,new]):
                        im=cell(sheet,n).crop((64,88,192,168));board.paste(im,(x-8+col*120,y+25+row*92),im)
                draw.text((x+4,y+210),'Ancien dos / dos repris',font=font,fill='#f4c785')
    assert fronts==1176;board.save(PREVIEW/'C11-back-r02-native64.png')
    for group in ['bastion','traqueuse','arcaniste','metamorphe']:
        folder=ENGINE/('camp-'+group);camp=read(folder/'camp-checks.json')
        assert camp['review']=='VERT18-C11-BACK-R02' and camp['front_returns']==12 and len(camp['records'])==156
        assert camp['camp_material_and_irradiance'] and all(abs(v['actor_gain']-1.48)<0.00001 for v in camp['records'])
        for step in [0,2]:
            for tick in [0,6,12]:assert (folder/f'camp-step{step}-tick{tick:02}.png').exists()
        Image.open(folder/'camp-step0-tick06.png').crop((16,60,592,572)).save(folder/'native-crop.png')
    banks={}
    for name in ['runtime-assets-C09','candidate-resources-C06','candidate-resources-C08','candidate-resources-C09','candidate-resources-C10','candidate-resources-C11','candidate-resources-C11-back-r02']:
        values=read(DOC/(name+'.json'))['sha256']
        for path,sha in values.items():
            if name=='runtime-assets-C09' and path=='assets/animations/vertical-18/manifest.json' and (DOC/'runtime-assets-C12.json').exists():
                assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==read(DOC/'runtime-assets-C12.json')['sha256'][path]
                continue
            assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==sha,path
        banks[name]=len(values)-int(name=='runtime-assets-C09' and (DOC/'runtime-assets-C12.json').exists())
    status='native_rear_candidates_awaiting_visual_approval'
    if (DOC/'acceptance-C11-back-r02.json').exists():
        accepted=read(DOC/'acceptance-C11-back-r02.json')
        assert hashlib.sha256((DOC/accepted['candidate_resources']).read_bytes()).hexdigest()==accepted['candidate_resources_sha256']
        status='visual_accepted_integrated' if accepted['normal_game_enabled'] else 'visual_accepted_not_installed'
    report=read(DOC/'ANALYSE-C11-back-r02.json');report.update(status=status,verified_front_exact_copies=fronts,engine={'native_configurations':192,'native_draws':2496,'front_returns':768,'camp_pose_records':624,'camp_front_returns':48,'same_actor_finish':True,'normal_game_enabled':False,'saved_game_loaded':False,'preferences_written':False},preserved_hash_banks=banks)
    write(DOC/'ANALYSE-C11-back-r02.json',report);write(DOC/'native-checks-C11-back-r02.json',native)
    print(json.dumps({'front_and_idle_preserved':fronts,'native_draws':2496,'rear_body':'whole authored forward-carry figures','normal_game_enabled':False}))

if __name__=='__main__':main()
