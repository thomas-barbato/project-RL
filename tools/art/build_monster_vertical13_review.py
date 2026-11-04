"""Three arthropod anatomies, preserving all thirty-six previous selections."""
import argparse
import json
import build_monster_vertical28_arms_review as previous
from build_monster_vertical03_review import bank

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-13/r01/current'
base.PREFIX='monster-vertical-13/r01/current/'
base.REVIEW='MONSTER-VERTICAL13-r01'


def main(page_only=False):
    retained=previous.retained_bank()
    retained['items']=bank('monster-vertical-12')+bank('monster-vertical-11')+bank('monster-vertical-28-weapon')+retained['items']
    retained['labels']=['34 · Créature aux bras ailés · validé','35 · Rampant sombre à bouche ronde · validé','36 · Démon massif à carapace · validé','31 · Quadrupède à queue · validé','32 · Quadrupède trapu · validé','33 · Créature au corps enroulé · validé','28 · Démon rouge élancé · lame, bras et couleurs validés']+retained['labels']
    assert len(retained['items'])==len(retained['labels'])==36
    assert len({p['id'] for p in retained['items']})==36
    path=base.OUT/'checks.json';data=json.loads(path.read_text(encoding='utf-8'))
    for item in data['items']:
        if item['id']=='MON-069-0':
            item['scope_note']='37 · six pattes en trois paires, deux trépieds alternés · petit corps rouge bas et tête attachée au thorax.'
            item['phase_names']=['Contact du trépied A','Charge sur A','Passage bas du trépied B','Repose de B','Contact du trépied B','Charge sur B','Passage bas du trépied A','Repose de A','Repos frontal']
        elif item['id']=='MON-070-0':
            item['scope_note']='38 · quatre pattes porteuses et deux avant-bras recourbés distincts · thorax dressé stable, faux non porteuses.'
            item['phase_names']=['Contact avant gauche','Charge gauche','Reprise arrière droite','Passage avant droit','Contact avant droit','Charge droite','Reprise arrière gauche','Passage avant gauche','Repos frontal']
        else:
            item['scope_note']='39 · huit pattes en quatre paires, groupes de quatre appuis alternés · gros abdomen rouge et sombre conservé.'
            item['phase_names']=['Contact du groupe A','Charge sur A','Passage bas du groupe B','Repose de B','Contact du groupe B','Charge sur B','Passage bas du groupe A','Repose de A','Repos frontal']
    path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    base.main(page_only,labels=['37 · Créature rouge à six pattes','38 · Créature aux avant-bras recourbés','39 · Arachnide'],previous=retained,
              intro='Suite : 37 créature rouge à six pattes, 38 créature à quatre pattes et deux avant-bras recourbés, 39 arachnide à huit pattes. Chaque morphologie garde ses appuis. Les n°34–36 sont validés. Trente-neuf modèles proposés ; 90 autres restent à reprendre.')
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('deux appuis alternés sur 800 ms','un cycle de 800 ms adapté aux appuis de chaque morphologie')
    section=section.replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','Vérifier les six pattes du n°37, les quatre pattes et les deux bras en faux du n°38, et les huit pattes du n°39. Les corps gardent leur hauteur et leur largeur ; aucune patte ne se détache. Comparer les matériaux, le dos et le retour au frontal conservé.')
    base.PAGE.write_text(previous.label_comparison_columns(html[:start]+section+html[end:]),encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(args.page_only)
