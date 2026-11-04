"""Restore the original right-hand blade and keep corrected walking materials."""
import argparse
import hashlib
import json
import prepare_monster_vertical01 as base
import prepare_monster_vertical28_color as colors

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-28-weapon/r01'
base.NAMES={60:'Démon rouge élancé · lame rétablie'}
base.REVIEW='MONSTER-VERTICAL28-WEAPON01-r01'
base.ALIGN_SUPPORT_ROOT=True

def steel(c):
    r,g,b,a=c;high=max(r,g,b);low=min(r,g,b)
    return a!=0 and high>65 and (100*(high-low)<18*high or (r<=g and r<=b and 100*(high-low)<35*high))

original_material=colors.material
def armed_material(c):return 0 if steel(c) else original_material(c)

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        path=base.OUT/'manifest.json';m=json.loads(path.read_text(encoding='utf-8'));p=m['profiles'][0]
        p.update({'review_number':28,'locomotion':'biped','locomotor_legs':2,'weapon':{'identity':'Original short steel blade','anatomical_hand':'right','image_side':{'down':'left','up':'right'}}})
        path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
        colors.SOURCE=base.OUT;colors.OUT=base.OUT;colors.REVIEW=base.REVIEW;colors.MATERIAL_NAME='rust_materials_armed';colors.material=armed_material
        colors.main()
        m=json.loads(path.read_text(encoding='utf-8'));p=m['profiles'][0]
        p.update({'previous_atlas':'assets/source-art/animation-polish-01/monster-vertical-28-arms/r01/MON-060-atlas.png','previous_material_palette':'assets/source-art/animation-polish-01/monster-vertical-28-color/r01/MON-060-material-palette.json','comparison':'original_weapon_restored'})
        for key in ['previous_atlas','previous_material_palette']:
            p[key+'_sha256']=hashlib.sha256((base.ROOT/p[key]).read_bytes()).hexdigest()
        path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
