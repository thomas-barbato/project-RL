"""Lossless native repair previews; one shared HTML page, no screenshot archive."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
REVIEW=BASE/'skill-animation-review-95'

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    assert not (REVIEW/'visual-acceptance-01.json').exists()
    entries=json.loads((REVIEW/'_capture/capture.json').read_text(encoding='utf-8'))
    assert len(entries)==22
    checks=[]
    for e in entries:
        t=e['timeline'];initial=t[0]
        refused=e['id']=='DR01' and e['case'] in ['CLOISON','RAPPELE','DETRUIT','ENERGIE']
        assert all(s['blood_events']==0 for s in t)
        assert t[0]['preview']==(e['id']=='DR01' and not refused)
        if refused:
            assert all(s==initial for s in t)
        elif e['id']=='DR01':
            gain=1 if e['case']=='PLAFOND' else 46
            assert all(s['energy']==88 and s['events']==1 and s['repaired_total']==gain and not s['preview'] for s in t[12:])
            assert t[12]['hp'][0]==initial['hp'][0]+gain
            assert t[12]['hp'][1:]==initial['hp'][1:]
        else:
            assert all(s['energy']==initial['energy'] and s['cooldown']==0 for s in t)
            assert all(s['hp']==initial['hp'] and s['events']==0 for s in t[:90])
            if e['case'] in ['RAPPELE','DETRUIT','ATTENTE']:
                assert all(s['hp']==initial['hp'] and s['events']==0 for s in t)
            elif e['case']=='PLAFOND':
                assert t[-1]['hp']==[200]*3 and t[-1]['events']==3
            elif e['case']=='HOSTILE':
                assert t[90]['hp']==[84,124,54] and t[-1]['hp']==[84,124,54] and t[-1]['quiet_turns']==0
            else:
                assert t[-1]['hp']==[112,152,82] and t[-1]['events']==9
        if e['case']=='ATTENTE':assert all(s==t[12] for s in t[12:])
        for s in t:
            assert all(0<=hp<=200 for hp in s['hp'])
        count,cols=e['unique_images'],e['columns'];rows=(count+cols-1)//cols
        with Image.open(REVIEW/'_capture'/e['folder']/'atlas.ppm') as raw:
            atlas=Image.new('RGB',raw.size);hashes=[]
            for i in range(count):
                x,y=i%cols*576,(rows-1-i//cols)*576
                frame=raw.crop((x,y,x+576,y+576)).convert('RGB');atlas.paste(frame,(x,i//cols*576))
                hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
        dest=REVIEW/(e['folder']+'.webp');atlas.save(dest,'WEBP',lossless=True,method=1)
        with Image.open(dest) as packed:
            for i,h in enumerate(hashes):
                x,y=i%cols*576,i//cols*576
                assert hashlib.sha256(packed.crop((x,y,x+576,y+576)).convert('RGB').tobytes()).hexdigest()==h
            if e['label']=='HAUT':
                frames=[]
                for i in e['indices']:
                    x,y=i%cols*576,i//cols*576
                    frames.append(packed.crop((x,y,x+576,y+576)).copy())
                standalone=REVIEW/(e['id']+'-repair-blue-64.webp')
                frames[0].save(standalone,'WEBP',save_all=True,append_images=frames[1:],duration=[42 if i%3!=2 else 41 for i in range(len(frames))],loop=0,lossless=True,method=1)
                with Image.open(standalone) as animated:
                    elapsed=0
                    for i in range(animated.n_frames):
                        animated.seek(i);animated.load();n=min(e['frames']-1,(elapsed*24+500)//1000);j=e['indices'][n];x,y=j%cols*576,j//cols*576
                        assert animated.convert('RGB').tobytes()==packed.crop((x,y,x+576,y+576)).convert('RGB').tobytes()
                        elapsed+=animated.info['duration']
                    assert elapsed==7000
                board=Image.new('RGB',(1728,576))
                for col,n in enumerate([25,94,121]):board.paste(frames[n],(col*576,0))
                board.save(REVIEW/(e['id']+'-native-qa.png'))
        e['atlas']='skill-animation-review-95/'+dest.name
        checks.append({'variant':e['variant'],'path':dest.name,'sha256':sha(dest),'bytes':dest.stat().st_size,'lossless_pixels':True,'frame_sha256':hashes})
        print(e['variant'],dest.stat().st_size,flush=True)
    preservation=json.loads((REVIEW/'preservation.json').read_text(encoding='utf-8'))
    for p,h in preservation.items():assert sha(ROOT/p)==h,p
    text=json.dumps(entries,ensure_ascii=False,separators=(',',':'))
    (REVIEW/'animations.json').write_text(text,encoding='utf-8')
    template=(ROOT/'tools/art/drone_repairs_review_template.html').read_text(encoding='utf-8')
    (BASE/'apercu-animations.html').write_text(template.replace('__ENTRIES__',text),encoding='utf-8')
    proof={'method':'Native Macroquad captures and lossless per-frame comparison; no browser rendering verification.','scenes':22,'native_tile':64,'frames':168,'fps':24,'maintenance_after_three_quiet_turns':True,'maintenance_stops_on_hostile':True,'no_actions_from_clock_only_wait':True,'active_single_target_once':True,'health_capped':True,'no_revival':True,'no_repair_of_recalled_drones':True,'active_blocked_by_wall':True,'active_zero_cost_refusal_in_prototype_only':True,'no_blood':True,'previous_assets_and_pending_previews_preserved':True,'standalone_animation_ms':7000,'checks':checks}
    (REVIEW/'validation.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
    print('Atlas total MiB',round(sum(c['bytes'] for c in checks)/2**20,2))

if __name__=='__main__':main()
