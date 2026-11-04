"""Check direct previous/new arm review and the partial lot10 acceptance."""
import hashlib
import json
import statistics
from pathlib import Path
from PIL import Image
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-28-arms/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-28-arms/r01/current'
base.REVIEW='MONSTER-VERTICAL28-ARMS01-r01'
base.NUMBERS=[60]
base.RECORD='MONSTER-VERTICAL28-ARMS01-VALIDATION.json'

if __name__=='__main__':
    acceptance=json.loads((base.REV/'MONSTER-VERTICAL10-ACCEPTANCE-r01.json').read_text(encoding='utf-8'))
    assert acceptance['models']==[61,62] and acceptance['rejected_models']==[60]
    m=json.loads((base.SOURCE/'manifest.json').read_text(encoding='utf-8'));p=m['profiles'][0]
    assert p['locomotion']=='biped' and p['locomotor_legs']==2 and p['review_number']==28
    assert hashlib.sha256((base.ROOT/p['previous_atlas']).read_bytes()).hexdigest()==p['previous_atlas_sha256']
    prompts=json.loads((base.REV/'MONSTER-VERTICAL28-ARMS01-PROMPTS.json').read_text(encoding='utf-8'))
    generations=json.loads((base.REV/'MONSTER-VERTICAL28-ARMS01-GENERATIONS.json').read_text(encoding='utf-8'))
    assert len(prompts['jobs'])==len(generations['outputs'])==2
    for job in prompts['jobs']:
        for file,expected in zip(job['paths'],job['input_sha256'],strict=True):
            assert hashlib.sha256(Path(file).read_bytes()).hexdigest()==expected
    assert {o['source_file'] for o in generations['outputs']}==set(p['sources'])
    for output in generations['outputs']:
        assert output['selected'] and hashlib.sha256((base.ROOT/output['source_file']).read_bytes()).hexdigest()==output['sha256']
    item=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'))['items'][0]
    assert item['previous_atlas']==p['previous_atlas']
    assert item['comparison']=='previous review / arm posture revision'
    assert item['headers']==['Frontal conservé','Bras précédents','Bras relâchés']
    old=Image.open(base.ROOT/p['previous_atlas']);widths=[]
    for row,direction in enumerate(['down','up']):
        previous=[]
        for phase in range(8):
            bb=old.crop((phase*160,row*128,(phase+1)*160,(row+1)*128)).getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
            previous.append(bb[2]-bb[0])
        bounds=[b['bounds'] for b in p['bounds'] if b['direction']==direction]
        assert abs(statistics.median(b[3]-b[1] for b in bounds)-p['reference_height'])<=1
        before=statistics.median(previous);after=statistics.median(b[2]-b[0] for b in bounds)
        assert after<before,('Arms remain flared',direction,before,after)
        widths.append({'direction':direction,'previous_width':before,'revised_width':after,'height_retained':p['reference_height']})
    base.main()
    path=base.REV/base.RECORD;r=json.loads(path.read_text(encoding='utf-8'));r['arm_posture_silhouette']=widths
    r['generation_provenance_verified']={'selected_outputs':2,'hashed_inputs':4}
    if 'browser_validation' in r:
        b=r['browser_validation'];assert len({p['id'] for p in b['checked_models']})==30
        assert len(b['manual_stops'])==2 and b['automatic_both_directions']
        revised=[p for p in b['checked_models'] if p['id']=='MON-060-0'];assert len(revised)==1 and 'monster-vertical-28-arms' in revised[0]['animation']
        assert b['arm_comparison_caption']=='Frontal conservé · Bras précédents · Bras relâchés. Taille native, cases de 64 px.'
    path.write_text(json.dumps(r,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
