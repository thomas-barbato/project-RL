"""Second whole-figure vertical batch, preserving the first batch and all fronts."""
import argparse
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-02/r01'
base.NAMES={5:'Possédé',156:'Démon massif',165:'Cultiste à la hache'}
base.REVIEW='MONSTER-VERTICAL02-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--references',action='store_true'); args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:base.main()
