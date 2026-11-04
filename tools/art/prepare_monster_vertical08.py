"""Three unarmed bipeds, with original proportions and installed frontal idles."""
import argparse
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-08/r01'
base.NAMES={8:'Créature rouge à crête osseuse',34:'Petit démon orangé',36:'Créature bleue effilée'}
base.REVIEW='MONSTER-VERTICAL08-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        from prepare_monster_vertical08_balance import prepare
        prepare(base.ROOT,base.OUT)
