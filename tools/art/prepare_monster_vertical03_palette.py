"""Prepare a shared ivory-material lookup; keep every source PNG unchanged."""
import hashlib
import json
import statistics
from bisect import bisect_left,bisect_right
from PIL import Image

def skin(rgb):
    r,g,b=rgb
    return r>51 and r>=g>=b and .08<(r-b)/r<.55 and (g-b)>.36*(r-b)

def prepare(root,folder,number=151,review='MONSTER-VERTICAL03-r01'):
    path=folder/'manifest.json'; manifest=json.loads(path.read_text(encoding='utf-8'))
    p=next(p for p in manifest['profiles'] if p['number']==number)
    source=Image.open(root/p['atlas']).convert('RGBA').crop((0,0,1280,256)); front=Image.open(root/p['idle_sprite']).convert('RGBA')
    a=[c[:3] for c in source.get_flattened_data() if c[3]>=224 and skin(c[:3])]
    b=[c[:3] for c in front.get_flattened_data() if c[3]>=224 and skin(c[:3])]
    ar=sorted(c[0] for c in a); br=sorted(c[0] for c in b); rows=[]
    for r in range(256):
        q=(bisect_left(ar,r)+bisect_right(ar,r))/(2*len(ar)); rr=br[round(q*(len(br)-1))]
        aa=[c for c in a if abs(c[0]-r)<=10] or [min(a,key=lambda c:abs(c[0]-r))]
        bb=[c for c in b if abs(c[0]-rr)<=10] or [min(b,key=lambda c:abs(c[0]-rr))]
        gains=[statistics.median(c[k] for c in bb)/max(1,statistics.median(c[k] for c in aa)) for k in [1,2]]
        rows.append([rr,*[round(min(1.5,max(.2,g)),6) for g in gains]])
    if number in (6,25,35):
        # These poses redistribute exposed highlights. Fit the shared
        # channel lookup to the retained skin, without per-frame adjustments.
        for channel in [1,2]:
            target=sorted(c[channel] for c in b)
            for _ in range(6):
                actual=sorted(min(255,round(c[channel]*rows[c[0]][channel])) for c in a)
                points=[(actual[round(q*(len(actual)-1))],target[round(q*(len(target)-1))]) for q in [.1,.5,.9]]
                if max(abs(x-y) for x,y in points)<=2:break
                knots=[(0,0),*points,(255,255)]
                for r,row in enumerate(rows):
                    group=[c[channel] for c in a if c[0]==r]
                    if not group:continue
                    value=min(255,statistics.median(group)*row[channel])
                    low,high=next((lo,hi) for lo,hi in zip(knots,knots[1:]) if lo[0]<=value<=hi[0])
                    mapped=low[1]+(high[1]-low[1])*(value-low[0])/max(1,high[0]-low[0])
                    row[channel]=round(min(1.5,max(.2,row[channel]*mapped/max(1,value))),6)
    rule={'review':review,'model':number,'variant':0,'material':'warm_ivory','red_min':50,'other_channel_max_ratio':.48,'red_lut':rows,
          'source_atlas':p['atlas'],'source_sha256':hashlib.sha256((root/p['atlas']).read_bytes()).hexdigest(),'reference':p['idle_sprite'],
          'reference_sha256':hashlib.sha256((root/p['idle_sprite']).read_bytes()).hexdigest(),'normal_game_enabled':False,
          'method':'Whole-cycle warm-ivory CDF and conditional green/blue; retained frontal reference; no phase-dependent gain; no PNG edit'}
    target=folder/f'MON-{number:03}-ivory-palette.json'; target.write_text(json.dumps(rule,indent=2)+'\n',encoding='utf-8')
    p['material_palette']=target.relative_to(root).as_posix()
    path.write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')

if __name__=='__main__':
    from pathlib import Path
    root=Path(__file__).resolve().parents[2]; prepare(root,root/'assets/source-art/animation-polish-01/monster-vertical-03/r01')
