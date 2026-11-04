"""Run the shared native checks against the next batch and retained first lot."""
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-02/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-02/r01/current'
base.REVIEW='MONSTER-VERTICAL02-r01'
base.NUMBERS=[5,156,165]
base.RECORD='MONSTER-VERTICAL02-VALIDATION.json'

if __name__=='__main__':base.main()
