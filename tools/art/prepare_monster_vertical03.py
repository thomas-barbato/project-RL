"""Three further winged ground walkers, retaining their installed frontals."""
import argparse
import prepare_monster_vertical01 as base

base.OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-03/r01'
base.NAMES={39:'Petit démon ailé',146:'Ailé doré',151:'Ailé pâle au brassard'}
base.REVIEW='MONSTER-VERTICAL03-r01'
base.ALIGN_SUPPORT_ROOT=True

if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--references',action='store_true'); args=parser.parse_args()
    if args.references:base.references(write_inventory=False)
    else:
        base.main()
        from prepare_monster_vertical03_palette import prepare
        prepare(base.ROOT,base.OUT)
