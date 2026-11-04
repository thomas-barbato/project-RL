"""Three cultists with carried staves or shield, preserving their real idle."""
import argparse
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-05/r01'
base.NAMES={15:'Cultiste au bâton violet',16:'Cultiste au bouclier',17:'Cultiste au bâton orné'}
base.REVIEW='MONSTER-VERTICAL05-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:base.main()
