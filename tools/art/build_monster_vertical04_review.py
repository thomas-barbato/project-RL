"""Publish the possessed batch and preserve all nine earlier selections."""
import argparse
import build_monster_vertical03_review as previous

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-04/r01/current'
base.PREFIX='monster-vertical-04/r01/current/'
base.REVIEW='MONSTER-VERTICAL04-r01'

def main(page_only=False):
    retained={'items':previous.bank('monster-vertical-03')+previous.bank('monster-vertical-color-01')+previous.bank('monster-vertical-02')+[p for p in previous.bank('monster-vertical-01') if p['id']!='MON-144-0'],
              'labels':['07 · Petit démon ailé · validé','08 · Ailé doré · validé','09 · Ailé pâle au brassard · validé','03 · Démon ailé · rouge validé','04 · Possédé · validé','05 · Démon massif · validé','06 · Cultiste à la hache · validé','01 · Cultiste · premier lot','02 · Cultiste au fusil · premier lot']}
    base.main(page_only,labels=['10 · Possédé voûté','11 · Hybride au bras mécanique','12 · Hybride au canon'],previous=retained,
              intro='Suite : 10 possédé voûté, 11 hybride au bras mécanique et 12 hybride au canon. Les neuf sélections précédentes restent disponibles, dont les trois ailés 07–09 validés. Douze modèles proposés ; 117 autres restent à reprendre.')
    html=base.PAGE.read_text(encoding='utf-8'); start=html.index('<section id="monstres-vertical">'); end=html.index('</section>',start)
    section=html[start:end].replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','Vérifier les appuis, la posture voûtée, le bras mécanique et le canon sur le même côté anatomique. Le canon pointe devant le personnage, à l’opposé du spectateur dans la vue de dos.')
    base.PAGE.write_text(html[:start]+section+html[end:],encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--page-only',action='store_true'); args=parser.parse_args(); main(args.page_only)
