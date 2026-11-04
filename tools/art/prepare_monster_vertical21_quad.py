"""Reopen review 21 with four supports, preserving its installed frontal idle."""
import argparse
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-21-quad/r01'
base.NAMES={32:'Créature à crête courbe · quatre appuis'}
base.REVIEW='MONSTER-VERTICAL21-QUAD01-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        import json
        path=base.OUT/'manifest.json';manifest=json.loads(path.read_text(encoding='utf-8'))
        p=manifest['profiles'][0];p['review_number']=21;p['locomotion']='quadruped';p['locomotor_legs']=4
        p['phase_names']=['Contact A','Appuis bas A','Avancée des pattes B','Reprise des appuis B','Contact B','Appuis bas B','Avancée des pattes A','Reprise des appuis A','Repos frontal']
        manifest['anatomy_correction']='MONSTER-VERTICAL07-AMENDMENT-r02.json'
        path.write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
