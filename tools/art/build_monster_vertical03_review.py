"""Third native gait batch; retain all earlier references and accepted red palette."""
import argparse
import json
import build_monster_vertical01_review as base

base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-03/r01/current'
base.PREFIX='monster-vertical-03/r01/current/'
base.REVIEW='MONSTER-VERTICAL03-r01'

def bank(folder):
    data=json.loads((base.ROOT/f'artifacts/animation-polish-01/{folder}/r01/current/checks.json').read_text(encoding='utf-8'))
    return [{**p,'capture_base':f'{folder}/r01/current/'} for p in data['items']]

def main(page_only=False):
    previous={'items':bank('monster-vertical-color-01')+bank('monster-vertical-02')+[p for p in bank('monster-vertical-01') if p['id']!='MON-144-0'],
              'labels':['03 · Démon ailé · rouge validé','04 · Possédé · validé','05 · Démon massif · validé','06 · Cultiste à la hache · validé','01 · Cultiste · premier lot','02 · Cultiste au fusil · premier lot']}
    base.main(page_only,labels=['07 · Petit démon ailé','08 · Ailé doré','09 · Ailé pâle au brassard'],previous=previous,
              intro='Suite : 07 petit démon ailé, 08 ailé doré et 09 ailé pâle au brassard. Les six modèles précédents restent sélectionnables, dont le démon rouge corrigé et validé. Neuf modèles proposés ; 120 autres restent à reprendre.')
    html=base.PAGE.read_text(encoding='utf-8'); start=html.index('<section id="monstres-vertical">'); end=html.index('</section>',start)
    section=html[start:end].replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','Vérifier les appuis au sol, les attaches des ailes, les couleurs entre repos et marche et le brassard unique sur le même avant-bras anatomique.')
    base.PAGE.write_text(html[:start]+section+html[end:],encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--page-only',action='store_true'); args=parser.parse_args(); main(args.page_only)
