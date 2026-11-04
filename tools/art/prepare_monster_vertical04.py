"""Three possessed silhouettes, keeping their installed idle frames exact."""
import argparse
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-04/r01'
base.NAMES={6:'Possédé voûté',7:'Hybride au bras mécanique',10:'Hybride au canon'}
base.REVIEW='MONSTER-VERTICAL04-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--references',action='store_true'); args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        from prepare_monster_vertical03_palette import prepare
        prepare(base.ROOT,base.OUT,number=6,review=base.REVIEW)
