"""Native checks for three quadrupeds, including their original low proportions."""
import json
import hashlib
import statistics
from pathlib import Path
from PIL import Image
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-09/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-09/r01/current'
base.REVIEW='MONSTER-VERTICAL09-r01'
base.NUMBERS=[35,37,38]
base.RECORD='MONSTER-VERTICAL09-VALIDATION.json'

if __name__=='__main__':
    m=json.loads((base.SOURCE/'manifest.json').read_text(encoding='utf-8'));breadth=[]
    prompts=json.loads((base.REV/'MONSTER-VERTICAL09-PROMPTS.json').read_text(encoding='utf-8'))
    generations=json.loads((base.REV/'MONSTER-VERTICAL09-GENERATIONS.json').read_text(encoding='utf-8'))
    assert len(prompts['jobs'])==len(generations['outputs'])==6
    for job in prompts['jobs']:
        for file,expected in zip(job['paths'],job['input_sha256'],strict=True):
            assert hashlib.sha256(Path(file).read_bytes()).hexdigest()==expected
    for output in generations['outputs']:
        assert output['selected']
        assert hashlib.sha256((base.ROOT/output['source_file']).read_bytes()).hexdigest()==output['sha256']
    for p in m['profiles']:
        assert p['locomotion']=='quadruped' and p['locomotor_legs']==4
        box=Image.open(base.ROOT/p['idle_sprite']).getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox();width=box[2]-box[0]
        for direction in ['down','up']:
            bounds=[b['bounds'] for b in p['bounds'] if b['direction']==direction]
            moving_width=statistics.median(b[2]-b[0] for b in bounds)
            assert .8*width-1<=moving_width<=1.2*width+1,(p['id'],direction,moving_width,width)
            breadth.append({'id':p['id'],'direction':direction,'moving_to_idle_median_width':moving_width/width})
    pale=next(p for p in m['profiles'] if p['number']==35)
    palette=json.loads((base.ROOT/pale['material_palette']).read_text(encoding='utf-8'))
    assert (palette['model'],palette['variant'],palette['material'])==(35,0,'warm_ivory')
    assert len(palette['red_lut'])==256 and not palette['normal_game_enabled']
    for file_key,hash_key in [('source_atlas','source_sha256'),('reference','reference_sha256')]:
        assert hashlib.sha256((base.ROOT/palette[file_key]).read_bytes()).hexdigest()==palette[hash_key]
    base.main()
    path=base.REV/base.RECORD;report=json.loads(path.read_text(encoding='utf-8'));report['low_silhouette_breadth']=breadth
    captures=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'))
    prepared=next(p for p in captures['items'] if p['id']=='MON-035-0')
    assert prepared['geometry_and_alpha_unchanged'] and prepared['palette_changed_pixels']>1000
    report['generation_provenance_verified']={'selected_outputs':6,'hashed_inputs':12}
    report['prepared_material_palette']=pale['material_palette']
    if 'browser_validation' in report:
        browser=report['browser_validation']
        assert len({p['id'] for p in browser['checked_models']})==27
        assert len(browser['manual_stops'])==6 and len(browser['automatic_models'])==3
        quad=[p for p in browser['checked_models'] if p['id']=='MON-032-0']
        assert len(quad)==1 and 'monster-vertical-21-quad' in quad[0]['animation']
    path.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
