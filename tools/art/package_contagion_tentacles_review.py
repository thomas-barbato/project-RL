"""Compare AF06-A02 with accepted A01; mechanics and reference pixels preserved."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
REVIEW=BASE/'skill-animation-review-102'

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(name,data):(REVIEW/name).write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
def compact(entry):
    states=[];known={};indices=[]
    timeline=entry.pop('timeline')
    for state in timeline:
        key=json.dumps(state,sort_keys=True)
        if key not in known:known[key]=len(states);states.append(state)
        indices.append(known[key])
    assert [states[i] for i in indices]==timeline
    entry.update(states=states,state_indices=indices)

assert not (REVIEW/'visual-acceptance-01.json').exists()
references=[e for e in json.loads((BASE/'skill-animation-review-90/animations.json').read_text()) if e['id']=='AF06']
entries=json.loads((REVIEW/'_capture/capture.json').read_text())
assert len(entries)==11
checks=[]
wall_cells=[(x,y) for y,line in enumerate('#########\n#.......#\n#.......#\n#....#..#\n#...#...#\n#.......#\n#.......#\n#.......#\n#########'.splitlines()) for x,c in enumerate(line) if c=='#']
for e in entries:
    old=next(p for p in references if (p['case'],p['direction'])==(e['case'],e['direction']))
    assert e['timeline']==old['timeline'],e['variant']
    cols=e.pop('native_columns');count=e['unique_images'];rows=(count+cols-1)//cols
    with Image.open(REVIEW/'_capture'/e['folder']/'atlas.png') as raw:
        atlas=Image.new('RGB',raw.size);hashes=[]
        for i in range(count):
            x,y=i%cols*576,(rows-1-i//cols)*576
            frame=raw.crop((x,y,x+576,y+576)).convert('RGB');atlas.paste(frame,(x,i//cols*576))
            hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
    dest=REVIEW/(e['variant']+'.webp');atlas.save(dest,'WEBP',lossless=True,method=1)
    label_edges=[]
    with Image.open(dest) as packed,Image.open(BASE/old['atlas']) as before:
        for i,h in enumerate(hashes):
            x,y=i%cols*576,i//cols*576
            assert hashlib.sha256(packed.crop((x,y,x+576,y+576)).convert('RGB').tobytes()).hexdigest()==h
        for n in range(e['frames']):
            i=e['indices'][n];x,y=i%cols*576,i//cols*576;new=packed.crop((x,y,x+576,y+576)).convert('RGB')
            i=old['indices'][n];x,y=i%old['columns']*576,i//old['columns']*576;prior=before.crop((x,y,x+576,y+576)).convert('RGB')
            if e['case']=='MUR':
                for x,y in wall_cells:
                    box=(x*64,y*64,x*64+64,y*64+64)
                    assert new.crop(box).tobytes()==prior.crop(box).tobytes(),(e['variant'],n,x,y)
            if n>=34 and new.tobytes()!=prior.tobytes():
                from PIL import ImageChops
                box=ImageChops.difference(new,prior).getbbox()
                # Three captured damage-label edge rows differ by a few pixels.
                # Do not alter native frames to hide the observed rasterization.
                assert e['case']=='DEGATS' and n in [49,50,51] and box[0]>=237 and box[2]<=244 and box[1]>=100 and box[3]<=108,(e['variant'],n,box)
                label_edges.append({'frame':n,'bbox':box})
        if e['case']=='GROUPE' and e['direction']==2:
            frames=[]
            for i in e['indices']:
                x,y=i%cols*576,i//cols*576;frames.append(packed.crop((x,y,x+576,y+576)).convert('RGB'))
            target=REVIEW/'AF06-A02-organic-native-64.webp';durations=[1000*(n+1)//24-1000*n//24 for n in range(len(frames))]
            frames[0].save(target,'WEBP',save_all=True,append_images=frames[1:],duration=durations,loop=0,lossless=True,method=1)
            with Image.open(target) as animation:
                elapsed=0
                for n in range(animation.n_frames):
                    animation.seek(n);animation.load();index=min(111,(elapsed*24+500)//1000)
                    assert animation.convert('RGB').tobytes()==frames[index].tobytes()
                    elapsed+=animation.info['duration']
                assert elapsed==sum(durations)
            board=Image.new('RGB',(1728,576))
            for col,n in enumerate([16,19,24]):board.paste(frames[n],(col*576,0))
            board.save(REVIEW/'tentacles-native-qa.png')
    e.update(atlas='skill-animation-review-102/'+dest.name,columns=cols,comparison_variant='tentacles')
    checks.append({'variant':e['variant'],'path':dest.name,'sha256':sha(dest),'lossless_pixels':True,'mechanical_timeline_identical':True,'wall_pixels_unchanged':e['case']=='MUR','post_animation_pixels_identical':not label_edges,'prepared_damage_label_edge_differences':label_edges,'bytes':dest.stat().st_size})
    compact(e)
    print(e['variant'],dest.stat().st_size,flush=True)
for old in references: old['comparison_variant']='reference';compact(old)
entries=references+entries
preserved=json.loads((REVIEW/'preservation.json').read_text())
for path,h in preserved.items():assert sha(ROOT/path)==h,path
text=json.dumps(entries,ensure_ascii=False,separators=(',',':'));(REVIEW/'animations.json').write_text(text,encoding='utf-8')
template=(ROOT/'tools/art/contagion_tentacles_review_template.html').read_text(encoding='utf-8')
(BASE/'apercu-animations.html').write_text(template.replace('__ENTRIES__',text),encoding='utf-8')
write('validation.json',{'method':'Native Macroquad frames; lossless encoded pixels, complete mechanical timeline equality with accepted A01, and exact wall/post-animation pixel checks. No browser rendering claim.','new_scenes':11,'comparison_scenes':22,'native_tile':64,'fps':24,'frames':112,'directions':4,'preserved_files':len(preserved),'checks':checks})
write('progress-audit.json',{'AF05_A01':'accepted','AF06_A01':'accepted','AF06_A02':'only requested visual revision awaiting review','mechanics_unchanged':True,'campaign_integration':False})
raw_files=sorted(p for p in (REVIEW/'_capture').rglob('*') if p.is_file())
write('cleanup-plan.json',{'files':[{'path':p.relative_to(REVIEW/'_capture').as_posix(),'sha256':sha(p),'bytes':p.stat().st_size} for p in raw_files],'bytes':sum(p.stat().st_size for p in raw_files)})
print('New playback bytes',sum(c['bytes'] for c in checks))
