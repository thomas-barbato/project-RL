"""Native whole-figure and approval guards for models 026, 028 and 032."""
import json
import statistics
from PIL import Image
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-07/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-07/r01/current'
base.REVIEW='MONSTER-VERTICAL07-r01'
base.NUMBERS=[26,28,32]
base.RECORD='MONSTER-VERTICAL07-VALIDATION.json'

if __name__=='__main__':
    manifest=json.loads((base.SOURCE/'manifest.json').read_text(encoding='utf-8'))
    breadth=[]
    for p in manifest['profiles']:
        front=Image.open(base.ROOT/p['idle_sprite']).getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
        width=front[2]-front[0]
        for direction in ['down','up']:
            bounds=[b['bounds'] for b in p['bounds'] if b['direction']==direction]
            moving_width=statistics.median(b[2]-b[0] for b in bounds)
            ratio=moving_width/width
            # One native pixel of tolerance for integer quantisation at pack time.
            assert .80*width-1<=moving_width<=1.20*width+1,(p['id'],direction,ratio)
            breadth.append({'id':p['id'],'direction':direction,'moving_to_idle_median_width':ratio})
    base.main()
    path=base.REV/base.RECORD;report=json.loads(path.read_text(encoding='utf-8'))
    report['compact_silhouette_breadth']=breadth
    path.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
