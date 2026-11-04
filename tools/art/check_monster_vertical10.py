"""Whole bipeds, retained native fronts and generation provenance."""
import hashlib
import json
import statistics
from pathlib import Path
from PIL import Image
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-10/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-10/r01/current'
base.REVIEW='MONSTER-VERTICAL10-r01'
base.NUMBERS=[60,61,62]
base.RECORD='MONSTER-VERTICAL10-VALIDATION.json'

if __name__=='__main__':
    m=json.loads((base.SOURCE/'manifest.json').read_text(encoding='utf-8'));breadth=[]
    prompts=json.loads((base.REV/'MONSTER-VERTICAL10-PROMPTS.json').read_text(encoding='utf-8'))
    generations=json.loads((base.REV/'MONSTER-VERTICAL10-GENERATIONS.json').read_text(encoding='utf-8'))
    assert len(prompts['jobs'])==len(generations['outputs'])==8
    assert sum(o['selected'] for o in generations['outputs'])==6
    for job in prompts['jobs']:
        for file,expected in zip(job['paths'],job['input_sha256'],strict=True):
            assert hashlib.sha256(Path(file).read_bytes()).hexdigest()==expected
    for output in generations['outputs']:
        assert hashlib.sha256((base.ROOT/output['source_file']).read_bytes()).hexdigest()==output['sha256']
    assert {o['source_file'] for o in generations['outputs'] if o['selected']}=={s for p in m['profiles'] for s in p['sources']}
    for p in m['profiles']:
        assert p['locomotion']=='biped' and p['locomotor_legs']==2
        box=Image.open(base.ROOT/p['idle_sprite']).getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox();width=box[2]-box[0]
        for direction in ['down','up']:
            bounds=[b['bounds'] for b in p['bounds'] if b['direction']==direction]
            moving_width=statistics.median(b[2]-b[0] for b in bounds)
            assert .8*width-1<=moving_width<=1.2*width+1,(p['id'],direction,moving_width,width)
            breadth.append({'id':p['id'],'direction':direction,'moving_to_idle_median_width':moving_width/width})
    base.main()
    path=base.REV/base.RECORD;report=json.loads(path.read_text(encoding='utf-8'))
    report['silhouette_breadth']=breadth
    report['generation_provenance_verified']={'selected_outputs':6,'discarded_narrow_sheets':2,'hashed_inputs':16}
    if 'browser_validation' in report:
        b=report['browser_validation']
        assert len({p['id'] for p in b['checked_models']})==30
        assert len(b['manual_stops'])==6 and len(b['automatic_models'])==3
        quad=[p for p in b['checked_models'] if p['id']=='MON-032-0']
        assert len(quad)==1 and 'monster-vertical-21-quad' in quad[0]['animation']
    path.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
