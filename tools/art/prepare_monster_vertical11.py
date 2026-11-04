"""Two quadrupeds and a coiled creature; retain their real frontal idles."""
import argparse
import json
import prepare_monster_vertical01 as base
from prepare_monster_vertical11_palette import main as prepare_palettes

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-11/r01'
base.NAMES={63:'Quadrupède à queue',64:'Quadrupède trapu',65:'Créature au corps enroulé'}
base.REVIEW='MONSTER-VERTICAL11-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        path=base.OUT/'manifest.json';m=json.loads(path.read_text(encoding='utf-8'))
        for number,p in enumerate(m['profiles'],31):
            p['review_number']=number
            p['locomotion']='quadruped' if p['number'] in [63,64] else 'serpentine'
            p['locomotor_legs']=4 if p['number'] in [63,64] else 0
        path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
        prepare_palettes()
