"""Six supports, curved forelimbs and eight supports; retain canonical idles."""
import argparse
import json
import prepare_monster_vertical01 as base
from prepare_monster_vertical13_palette import main as prepare_palettes

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-13/r01'
base.NAMES={69:'Créature rouge à six pattes',70:'Créature aux avant-bras recourbés',71:'Arachnide'}
base.REVIEW='MONSTER-VERTICAL13-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        path=base.OUT/'manifest.json';m=json.loads(path.read_text(encoding='utf-8'))
        for number,p in enumerate(m['profiles'],37):
            p['review_number']=number
            p['locomotion']={69:'hexapod',70:'quadruped',71:'arachnid'}[p['number']]
            p['locomotor_legs']={69:6,70:4,71:8}[p['number']]
        path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
        prepare_palettes()
