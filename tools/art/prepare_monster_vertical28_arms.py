"""Repose review28 arms while retaining its existing frontal idle and gait."""
import argparse
import hashlib
import json
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-28-arms/r01'
base.NAMES={60:'Démon rouge élancé · bras relâchés'}
base.REVIEW='MONSTER-VERTICAL28-ARMS01-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        path=base.OUT/'manifest.json';m=json.loads(path.read_text(encoding='utf-8'))
        previous=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-10/r01/MON-060-atlas.png'
        p=m['profiles'][0];p.update({'review_number':28,'locomotion':'biped','locomotor_legs':2,'previous_atlas':previous.relative_to(base.ROOT).as_posix(),'previous_atlas_sha256':hashlib.sha256(previous.read_bytes()).hexdigest()})
        path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
