"""Cultist review, with twelve previous selections retained as references."""
import argparse
import build_monster_vertical03_review as previous

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-05/r01/current'
base.PREFIX='monster-vertical-05/r01/current/'
base.REVIEW='MONSTER-VERTICAL05-r01'

def main(page_only=False):
    retained={'items':previous.bank('monster-vertical-04')+previous.bank('monster-vertical-03')+previous.bank('monster-vertical-color-01')+previous.bank('monster-vertical-02')+[p for p in previous.bank('monster-vertical-01') if p['id']!='MON-144-0'],
              'labels':['10 · Possédé voûté · validé','11 · Hybride au bras mécanique · validé','12 · Hybride au canon · validé','07 · Petit démon ailé · validé','08 · Ailé doré · validé','09 · Ailé pâle au brassard · validé','03 · Démon ailé · rouge validé','04 · Possédé · validé','05 · Démon massif · validé','06 · Cultiste à la hache · validé','01 · Cultiste · premier lot','02 · Cultiste au fusil · premier lot']}
    base.main(page_only,labels=['13 · Cultiste au bâton violet','14 · Cultiste au bouclier','15 · Cultiste au bâton orné'],previous=retained,
              intro='Suite : 13 cultiste au bâton violet, 14 cultiste au bouclier et 15 cultiste au bâton orné. Les douze sélections précédentes restent disponibles, dont les possédés et hybrides 10–12 validés. Quinze modèles proposés ; 114 autres restent à reprendre.')
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','Vérifier les appuis sous les robes, les prises sur les bâtons, le bouclier et la lame sur leur même côté anatomique. De dos, la capuche masque le visage et le revers du bouclier est visible.')
    base.PAGE.write_text(html[:start]+section+html[end:],encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(args.page_only)
