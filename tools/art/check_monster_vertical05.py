"""Native review checks for cultist staves and shield, retaining old approvals."""
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-05/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-05/r01/current'
base.REVIEW='MONSTER-VERTICAL05-r01'
base.NUMBERS=[15,16,17]
base.RECORD='MONSTER-VERTICAL05-VALIDATION.json'

if __name__=='__main__':base.main()
