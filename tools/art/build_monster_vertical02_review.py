"""Next batch, with the positively reviewed first three preserved in the viewer."""
import argparse
import json
import build_monster_vertical01_review as base

base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-02/r01/current'
base.PREFIX='monster-vertical-02/r01/current/'
base.REVIEW='MONSTER-VERTICAL02-r01'

def main(page_only=False):
    old=json.loads((base.ROOT/'artifacts/animation-polish-01/monster-vertical-01/r01/current/checks.json').read_text(encoding='utf-8'))
    previous={'items':[{**p,'capture_base':'monster-vertical-01/r01/current/'} for p in old['items']],
              'labels':['01 · Cultiste · premier lot','02 · Cultiste au fusil · premier lot','03 · Démon ailé · premier lot']}
    base.main(page_only,labels=['04 · Possédé','05 · Démon massif','06 · Cultiste à la hache'],previous=previous,
              intro='Suite : 04 possédé, 05 démon massif et 06 cultiste à la hache. Le premier lot 01–03 reste sélectionnable comme référence après votre retour positif. Six modèles proposés ; 123 autres restent à reprendre.')

if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--page-only',action='store_true'); args=parser.parse_args(); main(args.page_only)
