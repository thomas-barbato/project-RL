"""Next three existing anatomies, keeping their installed frontal idle exact."""
import argparse
import json
import prepare_monster_vertical01 as base
import prepare_monster_vertical11_palette as palette

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-12/r01'
base.NAMES={66:'Créature aux bras ailés',67:'Rampant sombre à bouche ronde',68:'Démon massif 068'}
base.REVIEW='MONSTER-VERTICAL12-r01'
base.ALIGN_SUPPORT_ROOT=True
palette.OUT=base.OUT
palette.REVIEW=base.REVIEW

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        path=base.OUT/'manifest.json';m=json.loads(path.read_text(encoding='utf-8'))
        for number,p in enumerate(m['profiles'],34):
            p['review_number']=number
            p['locomotion']='quadruped' if p['number']==67 else 'biped'
            p['locomotor_legs']=4 if p['number']==67 else 2
            if p['number']==66:p['wing_arms']=True
        path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
        palette.main()
