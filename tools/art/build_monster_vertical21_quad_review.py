"""Replace rejected review 21 in the selector; keep all other twenty-three models."""
import argparse
import json
import build_monster_vertical03_review as previous

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-21-quad/r01/current'
base.PREFIX='monster-vertical-21-quad/r01/current/'
base.REVIEW='MONSTER-VERTICAL21-QUAD01-r01'

def main(page_only=False):
    retained={'items':previous.bank('monster-vertical-08')+[p for p in previous.bank('monster-vertical-07') if p['id']!='MON-032-0']+previous.bank('monster-vertical-06')+previous.bank('monster-vertical-05')+previous.bank('monster-vertical-04')+previous.bank('monster-vertical-03')+previous.bank('monster-vertical-color-01')+previous.bank('monster-vertical-02')+[p for p in previous.bank('monster-vertical-01') if p['id']!='MON-144-0'],
              'labels':['22 · Créature rouge à crête osseuse · validé','23 · Petit démon orangé · validé','24 · Créature bleue effilée · validé','19 · Possédé à la tunique ocre · validé','20 · Démon massif aux longs bras · validé','16 · Cultiste armé 022 · validé','17 · Cultiste armé 024 · validé','18 · Possédé en haillons · validé','13 · Cultiste au bâton violet · validé','14 · Cultiste au bouclier · validé','15 · Cultiste au bâton orné · validé','10 · Possédé voûté · validé','11 · Hybride au bras mécanique · validé','12 · Hybride au canon · validé','07 · Petit démon ailé · validé','08 · Ailé doré · validé','09 · Ailé pâle au brassard · validé','03 · Démon ailé · rouge validé','04 · Possédé · validé','05 · Démon massif · validé','06 · Cultiste à la hache · validé','01 · Cultiste · premier lot','02 · Cultiste au fusil · premier lot']}
    assert len(retained['items'])==len(retained['labels'])==23
    check_path=base.OUT/'checks.json';data=json.loads(check_path.read_text(encoding='utf-8'))
    profile=json.loads((base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-21-quad/r01/manifest.json').read_text(encoding='utf-8'))['profiles'][0]
    data['items'][0]['phase_names']=profile['phase_names'];data['items'][0]['scope_note']='21 · quatre appuis · corps bas, pattes avant porteuses · ancien essai bipède retiré.'
    check_path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    base.main(page_only,labels=['21 · Crête courbe · quatre appuis · reprise'],previous=retained,
              intro='Correction du n°21 (MON-032) : déplacement bas sur ses quatre appuis. Son ancien essai bipède est retiré. Les n°22–24 sont validés ; les vingt-trois autres sélections restent disponibles. Vingt-quatre modèles proposés ; 105 autres restent à reprendre.')
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('deux appuis alternés sur 800 ms','un cycle de 800 ms adapté aux appuis de chaque morphologie')
    section=section.replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','N°21 : vérifier le corps bas et les quatre pattes porteuses. Il ne doit plus se redresser ni laisser pendre ses membres avant. Le frontal reste identique à l’arrêt.')
    base.PAGE.write_text(html[:start]+section+html[end:],encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(args.page_only)
