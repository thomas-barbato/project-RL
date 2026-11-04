"""Three unarmed bipeds; preserve their current shapes and frontal idles."""
import argparse
import json
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-10/r01'
base.NAMES={60:'Démon rouge élancé',61:'Démon cornu aux sabots',62:'Démon aux plaques osseuses'}
base.REVIEW='MONSTER-VERTICAL10-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        path=base.OUT/'manifest.json';m=json.loads(path.read_text(encoding='utf-8'))
        for number,p in enumerate(m['profiles'],28):
            p['review_number']=number;p['locomotion']='biped';p['locomotor_legs']=2
        path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
