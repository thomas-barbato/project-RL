"""Check original blade visibility, handedness, connected grip and material scope."""
import hashlib
import json
from pathlib import Path
from PIL import Image
import check_monster_vertical01 as base
from prepare_monster_vertical28_color import quant
from prepare_monster_vertical28_weapon import steel,armed_material

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-28-weapon/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-28-weapon/r01/current'
base.REVIEW='MONSTER-VERTICAL28-WEAPON01-r01'
base.NUMBERS=[60]
base.RECORD='MONSTER-VERTICAL28-WEAPON01-VALIDATION.json'

def largest_component(image):
    remaining={(x,y) for y in range(128) for x in range(160) if image.getpixel((x,y))[3]>=96};groups=[]
    while remaining:
        seed=remaining.pop();group={seed};queue=[seed]
        while queue:
            x,y=queue.pop()
            for dy in [-1,0,1]:
                for dx in [-1,0,1]:
                    q=(x+dx,y+dy)
                    if q in remaining:remaining.remove(q);group.add(q);queue.append(q)
        groups.append(group)
    return max(groups,key=len)

if __name__=='__main__':
    m=json.loads((base.SOURCE/'manifest.json').read_text(encoding='utf-8'));p=m['profiles'][0]
    assert p['weapon']['anatomical_hand']=='right' and p['weapon']['image_side']=={'down':'left','up':'right'}
    for key in ['previous_atlas','previous_material_palette']:
        assert hashlib.sha256((base.ROOT/p[key]).read_bytes()).hexdigest()==p[key+'_sha256']
    rule=json.loads((base.ROOT/p['material_palette']).read_text(encoding='utf-8'))
    assert (rule['model'],rule['variant'],rule['material'])==(60,0,'rust_materials_armed')
    assert len(rule['material_luts'])==4 and all(len(t)==256 for t in rule['material_luts'])
    for file,key in [('source_atlas','source_sha256'),('reference','reference_sha256')]:assert hashlib.sha256((base.ROOT/rule[file]).read_bytes()).hexdigest()==rule[key]
    prompts=json.loads((base.REV/'MONSTER-VERTICAL28-WEAPON01-PROMPTS.json').read_text(encoding='utf-8'));generations=json.loads((base.REV/'MONSTER-VERTICAL28-WEAPON01-GENERATIONS.json').read_text(encoding='utf-8'))
    assert len(prompts['jobs'])==len(generations['outputs'])==2
    for job in prompts['jobs']:
        for file,expected in zip(job['paths'],job['input_sha256'],strict=True):assert hashlib.sha256(Path(file).read_bytes()).hexdigest()==expected
    assert {o['source_file'] for o in generations['outputs']}==set(p['sources'])
    for o in generations['outputs']:assert o['selected'] and hashlib.sha256((base.ROOT/o['source_file']).read_bytes()).hexdigest()==o['sha256']
    atlas=Image.open(base.ROOT/p['atlas']).convert('RGBA');front=Image.open(base.ROOT/p['idle_sprite']).convert('RGBA');visibility=[]
    reference_core=sum(1 for y in range(88,128) for x in range(76) if front.getpixel((x,y))[3]>=224 and steel(front.getpixel((x,y))))
    for row,direction in enumerate(['down','up']):
        pixels=list(atlas.crop((0,row*128,1280,(row+1)*128)).get_flattened_data())
        for kind in [1,2]:
            source=[c for c in pixels if c[3]>=224 and armed_material(c)==kind];target=[c for c in front.get_flattened_data() if c[3]>=224 and armed_material(c)==kind];lut=rule['material_luts'][row*2+kind-1]
            prepared=[[lut[c[k]][k] for k in range(3)] for c in source]
            assert max(abs(x-y) for a,b in zip(quant(prepared),quant(target)) for x,y in zip(a,b))<=2
        for phase in range(8):
            im=atlas.crop((phase*160,row*128,(phase+1)*160,(row+1)*128));px=im.load();main=largest_component(im)
            blade={(x,y) for y in range(88,128) for x in range(160) if px[x,y][3]>=224 and steel(px[x,y]) and (x<76 if direction=='down' else x>84)}
            assert len(blade)>=max(3,round(reference_core*.5)),('Blade not readable beside hand',direction,phase,len(blade))
            assert blade<=main,('Blade disconnected from figure',direction,phase)
            visibility.append({'direction':direction,'phase':phase,'steel_core_pixels':len(blade),'correct_side':p['weapon']['image_side'][direction],'connected_to_figure':True})
    captures=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'));item=captures['items'][0]
    assert captures['monster_gain']==1.48 and item['geometry_and_alpha_unchanged'] and item['palette_changed_pixels']>3000
    assert item['previous_material_palette']==p['previous_material_palette'] and item['headers']==['Frontal conservé','Arme absente','Arme rétablie']
    base.main()
    path=base.REV/base.RECORD;r=json.loads(path.read_text(encoding='utf-8'));r['weapon_visibility']=visibility;r['retained_frontal_steel_core_pixels']=reference_core
    r['generation_provenance_verified']={'selected_outputs':2,'hashed_inputs':4}
    if 'browser_validation' in r:
        b=r['browser_validation'];assert len({p['id'] for p in b['checked_models']})==30
        assert len(b['manual_stops'])==2 and b['automatic_both_directions']
        revised=[p for p in b['checked_models'] if p['id']=='MON-060-0'];assert len(revised)==1 and 'monster-vertical-28-weapon' in revised[0]['animation']
        assert b['weapon_comparison_caption']=='Frontal conservé · Arme absente · Arme rétablie. Taille native, cases de 64 px.'
    path.write_text(json.dumps(r,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
