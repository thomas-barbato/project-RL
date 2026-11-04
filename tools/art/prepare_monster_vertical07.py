"""Compact hybrid, heavy demon and small crested biped; retain installed idles."""
import argparse
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-07/r01'
base.NAMES={26:'Possédé à la tunique ocre',28:'Démon massif aux longs bras',32:'Petit démon à crête courbe'}
base.REVIEW='MONSTER-VERTICAL07-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:base.main()
