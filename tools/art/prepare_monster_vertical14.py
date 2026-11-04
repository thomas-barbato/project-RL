"""Prepare three bipeds with their exact installed frontals."""
import argparse
import json
import prepare_monster_vertical01 as base
from prepare_monster_vertical13_palette import main as prepare_palettes

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-14/r01'
base.NAMES={72:'Démon trapu à épaule rouge',73:'Démon pâle voûté',74:'Démon aux griffes dorées'}
base.REVIEW='MONSTER-VERTICAL14-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--references',action='store_true')
    if parser.parse_args().references:
        base.references(write_inventory=False)
    else:
        base.main()
        path=base.OUT/'manifest.json'
        m=json.loads(path.read_text(encoding='utf-8'))
        for number,p in enumerate(m['profiles'],40):
            p.update(name=base.NAMES[p["number"]],review_number=number,locomotion='biped',locomotor_legs=2)
        path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
        prepare_palettes(14)
