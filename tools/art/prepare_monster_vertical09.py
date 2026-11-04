"""Three low four-support creatures, retaining their exact installed frontal idles."""
import argparse
import json
import prepare_monster_vertical01 as base
from prepare_monster_vertical03_palette import prepare as prepare_ivory

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-09/r01'
base.NAMES={35:'Créature pâle aux griffes',37:'Petit quadrupède pâle',38:'Créature à gueule orangée'}
base.REVIEW='MONSTER-VERTICAL09-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        path=base.OUT/'manifest.json';m=json.loads(path.read_text(encoding='utf-8'))
        for number,p in enumerate(m['profiles'],25):
            p['review_number']=number;p['locomotion']='quadruped';p['locomotor_legs']=4
            p['phase_names']=['Contact A','Appuis bas A','Avancée des pattes B','Reprise des appuis B','Contact B','Appuis bas B','Avancée des pattes A','Reprise des appuis A','Repos frontal']
        path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
        prepare_ivory(base.ROOT,base.OUT,35,base.REVIEW)
