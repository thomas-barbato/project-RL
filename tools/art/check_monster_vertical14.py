"""Check three bipeds, their reference silhouettes and their material palettes."""
import hashlib
import json
import statistics
from pathlib import Path
from PIL import Image
import check_monster_vertical01 as base
from prepare_monster_vertical13_palette import material,quant

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-14/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-14/r01/current'
base.REVIEW='MONSTER-VERTICAL14-r01'
base.NUMBERS=[72,73,74]
base.RECORD='MONSTER-VERTICAL14-VALIDATION.json'


if __name__=='__main__':
    m=json.loads((base.SOURCE/'manifest.json').read_text(encoding='utf-8'));breadth=[]
    prompts=json.loads((base.REV/'MONSTER-VERTICAL14-PROMPTS.json').read_text(encoding='utf-8'))
    generations=json.loads((base.REV/'MONSTER-VERTICAL14-GENERATIONS.json').read_text(encoding='utf-8'))
    correction=json.loads((base.REV/'MONSTER-VERTICAL14-BREADTH-PROMPTS.json').read_text(encoding='utf-8'))
    prompts['jobs'].extend(correction['jobs'])
    assert len(prompts['jobs'])==len(generations['outputs'])==7
    assert sum(o['selected'] for o in generations['outputs'])==6
    for job in prompts['jobs']:
        for file,expected in zip(job['paths'],job['input_sha256'],strict=True):assert hashlib.sha256(Path(file).read_bytes()).hexdigest()==expected
    for output in generations['outputs']:assert hashlib.sha256((base.ROOT/output['source_file']).read_bytes()).hexdigest()==output['sha256']
    assert {o['source_file'] for o in generations['outputs'] if o['selected']}=={s for p in m['profiles'] for s in p['sources']}
    for p in m['profiles']:
        expected=('biped',2)
        assert (p['locomotion'],p['locomotor_legs'])==expected
        rule=json.loads((base.ROOT/p['material_palette']).read_text(encoding='utf-8'))
        assert (rule['model'],rule['variant'],rule['material'])==(p['number'],0,'arthropod_materials')
        assert len(rule['material_luts'])==6 and all(len(t)==256 for t in rule['material_luts'])
        assert hashlib.sha256((base.ROOT/rule['source_atlas']).read_bytes()).hexdigest()==rule['source_sha256']
        atlas=Image.open(base.ROOT/p['atlas']).convert('RGBA')
        for row,reference in enumerate(rule['references']):
            assert reference['direction']==['down','up'][row]
            assert hashlib.sha256((base.ROOT/reference['file']).read_bytes()).hexdigest()==reference['sha256']
            target=Image.open(base.ROOT/reference['file']).convert('RGBA')
            box=target.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
            expected_width=(box[2]-box[0])*p['reference_height']/(box[3]-box[1])
            moving_width=statistics.median(b['bounds'][2]-b['bounds'][0] for b in p['bounds'] if b['direction']==reference['direction'])
            assert .8*expected_width-1<=moving_width<=1.2*expected_width+1,(p['number'],reference['direction'],moving_width,expected_width)
            breadth.append({'id':p['id'],'direction':reference['direction'],'median_width_to_installed_view':moving_width/expected_width})
            source=atlas.crop((0,row*128,1280,(row+1)*128))
            for kind in [1,2,3]:
                a=[c for c in source.get_flattened_data() if c[3]>=224 and material(c)==kind]
                b=[c for c in target.get_flattened_data() if c[3]>=224 and material(c)==kind]
                lut=rule['material_luts'][row*3+kind-1];actual=quant([[lut[c[k]][k] for k in range(3)] for c in a])
                assert max(abs(x-y) for aa,bb in zip(actual,quant(b)) for x,y in zip(aa,bb))<=2
    base.main()
    captures=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'))
    assert all(len(item['phase_names'])==9 and item['phase_names'][8]=='Repos frontal' for item in captures['items'])
    path=base.REV/base.RECORD;report=json.loads(path.read_text(encoding='utf-8'))
    assert report['accepted_weapon28_assets_preserved']==5 and report['accepted_lot10_assets_preserved']==8 and report['accepted_lot11_assets_preserved']==18 and report['accepted_lot12_assets_preserved']==18 and report['accepted_lot13_assets_preserved']==18
    report['silhouette_breadth']=breadth
    report['anatomy_scope']={'MON-072-0':'stocky biped with asymmetric red shoulder mass','MON-073-0':'hunched biped with asymmetric dark forearm','MON-074-0':'lean biped with two large attached golden claws'}
    report['generation_provenance_verified']={'selected_outputs':6,'discarded_initial_sheets':1,'hashed_inputs':14}
    report['material_reference_policy']='Actual installed view per direction; one fixed palette per material for all eight poses; no per-phase gain; Rust pixels/alpha/rest test.'
    if 'browser_validation' in report:
        b=report['browser_validation'];assert len({p['id'] for p in b['checked_models']})==42
        assert len(b['manual_stops'])==6 and len(b['automatic_models'])==3
        assert all(p['phase']=='8' and p['text'].endswith('Repos frontal') for p in b['manual_stops'])
        assert {p['id'] for p in b['automatic_models']}=={'MON-072-0','MON-073-0','MON-074-0'}
        assert all(p['direction']=='both' and p['step_ms']==400 and any(s.startswith('Bas') for s in p['observed']) and any(s.startswith('Haut') for s in p['observed']) for p in b['automatic_models'])
        assert b['canvas']['width']==b['canvas']['css_width']==576 and b['canvas']['height']==320
        assert b['final_selection']['playing']=='Pause' and b['final_selection']['step_ms']==400
        armed=[p for p in b['checked_models'] if p['id']=='MON-060-0']
        assert len(armed)==1 and 'monster-vertical-28-weapon' in armed[0]['animation']
        retained=[p for p in b['checked_models'] if p['id'] in {'MON-069-0','MON-070-0','MON-071-0'}]
        assert len(retained)==3 and all('monster-vertical-13' in p['animation'] for p in retained)
    path.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
