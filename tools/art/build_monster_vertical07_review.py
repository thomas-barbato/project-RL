"""Publish three new body builds while retaining all eighteen earlier references."""
import argparse
import build_monster_vertical03_review as previous

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-07/r01/current'
base.PREFIX='monster-vertical-07/r01/current/'
base.REVIEW='MONSTER-VERTICAL07-r01'

def main(page_only=False):
    retained={'items':previous.bank('monster-vertical-06')+previous.bank('monster-vertical-05')+previous.bank('monster-vertical-04')+previous.bank('monster-vertical-03')+previous.bank('monster-vertical-color-01')+previous.bank('monster-vertical-02')+[p for p in previous.bank('monster-vertical-01') if p['id']!='MON-144-0'],
              'labels':['16 · Cultiste armé 022 · validé','17 · Cultiste armé 024 · validé','18 · Possédé en haillons · validé','13 · Cultiste au bâton violet · validé','14 · Cultiste au bouclier · validé','15 · Cultiste au bâton orné · validé','10 · Possédé voûté · validé','11 · Hybride au bras mécanique · validé','12 · Hybride au canon · validé','07 · Petit démon ailé · validé','08 · Ailé doré · validé','09 · Ailé pâle au brassard · validé','03 · Démon ailé · rouge validé','04 · Possédé · validé','05 · Démon massif · validé','06 · Cultiste à la hache · validé','01 · Cultiste · premier lot','02 · Cultiste au fusil · premier lot']}
    base.main(page_only,labels=['19 · Possédé à la tunique ocre','20 · Démon massif aux longs bras','21 · Petit démon à crête courbe'],previous=retained,
              intro='Suite : 19 possédé à la tunique ocre, 20 démon massif aux longs bras et 21 petit démon à crête courbe. Les dix-huit sélections précédentes restent disponibles, dont le lot 16–18 validé. Vingt et un modèles proposés ; 108 autres restent à reprendre.')
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','Vérifier les appuis courts, les silhouettes basses, les bras qui restent pendants et le raccord au repos. Le dos du petit démon conserve sa courbe de tête orientée vers l’avant ; son ventre et ses yeux ne sont pas peints au dos.')
    base.PAGE.write_text(html[:start]+section+html[end:],encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(args.page_only)
