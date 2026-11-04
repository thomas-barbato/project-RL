"""Prepare fixed material channel lookups; never repaint the arm-posture PNGs."""
import copy
import hashlib
import json
from pathlib import Path
from PIL import Image
from prepare_monster_vertical01 import verify_frozen_assets

ROOT=Path(__file__).resolve().parents[2]
SOURCE=ROOT/'assets/source-art/animation-polish-01/monster-vertical-28-arms/r01'
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-28-color/r01'
REV=ROOT/'docs/reviews/animation-polish-01'
REVIEW='MONSTER-VERTICAL28-COLOR01-r01'
MATERIAL_NAME='rust_materials'

def material(c):
    r,g,b,a=c
    if not a:return 0
    if r>40 and .12*r<=g<.70*r and .06*r<=b<.60*r:return 1
    if r>25 and r>=g>=b:return 2
    return 0

def quant(values,rank=round):
    return [[sorted(c[k] for c in values)[rank(q*(len(values)-1))] for k in range(3)] for q in [.1,.5,.9]]

def channel_lookup(source,target,rank=round):
    # Fixed monotone material curves shared by all eight poses of a direction.
    # The key shadow, middle and highlight ranks stay tied to the retained idle.
    qs=[0,.02,.1,.25,.5,.75,.9,.98,1];channels=[]
    for k in range(3):
        a=sorted(c[k] for c in source);b=sorted(c[k] for c in target);knots={0:0,255:255}
        for q in qs:knots[a[rank(q*(len(a)-1))]]=b[rank(q*(len(b)-1))]
        # Give the three tested principal ranks priority if a coarse source ties.
        for q in [.1,.5,.9]:knots[a[rank(q*(len(a)-1))]]=b[rank(q*(len(b)-1))]
        points=sorted(knots.items());lut=[]
        for v in range(256):
            lo,hi=next((lo,hi) for lo,hi in zip(points,points[1:]) if lo[0]<=v<=hi[0])
            lut.append(round(lo[1]+(hi[1]-lo[1])*(v-lo[0])/max(1,hi[0]-lo[0])))
        assert all(x<=y for x,y in zip(lut,lut[1:])),('Non-monotone material curve',k)
        channels.append(lut)
    return [list(c) for c in zip(*channels)]

def main():
    frozen=verify_frozen_assets();OUT.mkdir(parents=True,exist_ok=True)
    original=json.loads((SOURCE/'manifest.json').read_text(encoding='utf-8'));p=copy.deepcopy(original['profiles'][0])
    atlas=Image.open(ROOT/p['atlas']).convert('RGBA');front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
    luts=[];audit=[]
    for row,direction in enumerate(['down','up']):
        pixels=list(atlas.crop((0,row*128,1280,(row+1)*128)).get_flattened_data())
        for kind,label in [(1,'rust_flesh'),(2,'dark_materials')]:
            a=[c for c in pixels if c[3]>=224 and material(c)==kind]
            b=[c for c in front.get_flattened_data() if c[3]>=224 and material(c)==kind]
            lut=channel_lookup(a,b);luts.append(lut)
            prepared=[[lut[c[k]][k] for k in range(3)] for c in a]
            before,target,after=quant(a),quant(b),quant(prepared)
            assert max(abs(x-y) for aa,bb in zip(after,target) for x,y in zip(aa,bb))<=2
            audit.append({'direction':direction,'material':label,'moving_pixels':len(a),'idle_pixels':len(b),'before':before,'retained_idle':target,'after':after})
    rule={'review':REVIEW,'model':60,'variant':0,'material':MATERIAL_NAME,'material_luts':luts,'source_atlas':p['atlas'],'source_sha256':hashlib.sha256((ROOT/p['atlas']).read_bytes()).hexdigest(),'reference':p['idle_sprite'],'reference_sha256':hashlib.sha256((ROOT/p['idle_sprite']).read_bytes()).hexdigest(),'method':'Fixed per-direction material channel curves shared by eight poses; rust flesh and dark materials matched to retained frontal shadow/middle/highlight ranks; no per-phase gain','png_files_modified':False,'normal_game_enabled':False}
    palette=OUT/'MON-060-material-palette.json';palette.write_text(json.dumps(rule,indent=2)+'\n',encoding='utf-8')
    p.pop('previous_atlas',None);p.pop('previous_atlas_sha256',None)
    p.update({'material_palette':palette.relative_to(ROOT).as_posix(),'comparison':'previous_review_colors'})
    m={**original,'review':REVIEW,'frozen_source_hashes_verified':frozen,'profiles':[p]}
    (OUT/'manifest.json').write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    report={'review':REVIEW,'unchanged_shape_sources':[{'file':f,'sha256':hashlib.sha256((ROOT/f).read_bytes()).hexdigest()} for f in [p['atlas'],p['idle_sprite'],*p['sources']]],'materials':audit,'png_files_modified':False,'world_gain_unchanged':1.48,'normal_game_enabled':False}
    (REV/(REVIEW.removesuffix('-r01')+'-MATERIALS.json')).write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(audit))

if __name__=='__main__':main()
