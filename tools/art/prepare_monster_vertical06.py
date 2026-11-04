"""Two armed cultists and a possessed walker, preserving installed idles."""
import argparse
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-06/r01'
base.NAMES={22:'Cultiste armé 022',24:'Cultiste armé 024',25:'Possédé en haillons'}
base.REVIEW='MONSTER-VERTICAL06-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        from prepare_monster_vertical03_palette import prepare
        prepare(base.ROOT,base.OUT,number=25,review=base.REVIEW)
