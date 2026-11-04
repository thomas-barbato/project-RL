"""Color-only correction: stable geometry, alpha, idle and accepted other models."""
import hashlib
import json
import check_monster_vertical01 as base
from prepare_monster_vertical28_color import material,quant
from PIL import Image

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-28-color/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-28-color/r01/current'
base.REVIEW='MONSTER-VERTICAL28-COLOR01-r01'
base.NUMBERS=[60]
base.RECORD='MONSTER-VERTICAL28-COLOR01-VALIDATION.json'

if __name__=='__main__':
    m=json.loads((base.SOURCE/'manifest.json').read_text(encoding='utf-8'));p=m['profiles'][0]
    rule=json.loads((base.ROOT/p['material_palette']).read_text(encoding='utf-8'))
    assert (rule['model'],rule['variant'],rule['material'])==(60,0,'rust_materials')
    assert len(rule['material_luts'])==4 and all(len(lut)==256 for lut in rule['material_luts'])
    for file,key in [('source_atlas','source_sha256'),('reference','reference_sha256')]:
        assert hashlib.sha256((base.ROOT/rule[file]).read_bytes()).hexdigest()==rule[key]
    baseline=json.loads((base.REV/'MONSTER-VERTICAL28-COLOR01-MATERIALS.json').read_text(encoding='utf-8'))
    for a in baseline['unchanged_shape_sources']:
        assert hashlib.sha256((base.ROOT/a['file']).read_bytes()).hexdigest()==a['sha256']
    captures=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'));item=captures['items'][0]
    assert item['geometry_and_alpha_unchanged'] and item['palette_changed_pixels']>4000
    assert item['headers']==['Frontal conservé','Couleurs précédentes','Couleurs harmonisées']
    assert captures['monster_gain']==1.48
    atlas=Image.open(base.ROOT/p['atlas']).convert('RGBA');front=Image.open(base.ROOT/p['idle_sprite']).convert('RGBA')
    for row in range(2):
        pixels=list(atlas.crop((0,row*128,1280,(row+1)*128)).get_flattened_data())
        for kind in [1,2]:
            source=[c for c in pixels if c[3]>=224 and material(c)==kind];target=[c for c in front.get_flattened_data() if c[3]>=224 and material(c)==kind]
            lut=rule['material_luts'][row*2+kind-1];prepared=[[lut[c[k]][k] for k in range(3)] for c in source]
            assert max(abs(x-y) for a,b in zip(quant(prepared),quant(target)) for x,y in zip(a,b))<=2
    base.main()
    path=base.REV/base.RECORD;r=json.loads(path.read_text(encoding='utf-8'))
    r['shape_sources_preserved']=baseline['unchanged_shape_sources'];r['material_alignment']=baseline['materials'];r['world_gain']=1.48
    if 'browser_validation' in r:
        b=r['browser_validation'];assert len({p['id'] for p in b['checked_models']})==30
        assert len(b['manual_stops'])==2 and b['automatic_both_directions']
        revised=[p for p in b['checked_models'] if p['id']=='MON-060-0'];assert len(revised)==1 and 'monster-vertical-28-color' in revised[0]['animation']
        assert b['color_comparison_caption']=='Frontal conservé · Couleurs précédentes · Couleurs harmonisées. Taille native, cases de 64 px.'
    path.write_text(json.dumps(r,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
