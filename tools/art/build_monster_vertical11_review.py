"""Review two grounded quadrupeds and one slither, retaining all prior models."""
import argparse
import json
import build_monster_vertical28_arms_review as previous
from build_monster_vertical03_review import bank

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-11/r01/current'
base.PREFIX='monster-vertical-11/r01/current/'
base.REVIEW='MONSTER-VERTICAL11-r01'


def main(page_only=False):
    retained=previous.retained_bank()
    retained['items']=bank('monster-vertical-28-weapon')+retained['items']
    retained['labels']=['28 · Démon rouge élancé · lame, bras et couleurs validés']+retained['labels']
    assert len(retained['items'])==len(retained['labels'])==30
    path=base.OUT/'checks.json';data=json.loads(path.read_text(encoding='utf-8'))
    for item in data['items']:
        if item['id'] in ['MON-063-0','MON-064-0']:
            item['scope_note']=item['name']+' · quatre pattes porteuses, corps bas et pas courts · frontal conservé au repos.'
            item['phase_names']=['contact avant gauche','charge gauche','reprise arrière droite','passage avant droit','contact avant droit','charge droite','reprise arrière gauche','passage avant gauche','Repos frontal']
        else:
            item['scope_note']=item['name']+' · corps serpentin sans jambes, ondulation des anneaux au sol · bras libres et repos enroulé conservé.'
            item['phase_names']=['appui de la spire gauche','onde vers l’avant','extension courte','spire arrière rassemblée','appui de la spire droite','onde opposée','extension opposée','retour de la spire','Repos frontal']
    path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    base.main(page_only,labels=['31 · Quadrupède à queue','32 · Quadrupède trapu','33 · Créature au corps enroulé'],previous=retained,
              intro='Suite : 31 quadrupède à queue, 32 quadrupède trapu et 33 créature au corps enroulé. Quatre appuis pour les deux premiers ; ondulation sans jambes pour le troisième. Le n°28 conserve sa lame, ses bras et ses couleurs validés. Trente-trois modèles proposés ; 96 autres restent à reprendre.')
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('deux appuis alternés sur 800 ms','un cycle de 800 ms adapté aux appuis de chaque morphologie')
    section=section.replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','Vérifier les quatre pattes porteuses, la posture basse, le côté de la queue et de la plaque rouge. Pour le n°33, vérifier la continuité du corps enroulé : aucune jambe et aucun redressement. Comparer les couleurs et le retour au frontal conservé.')
    base.PAGE.write_text(previous.label_comparison_columns(html[:start]+section+html[end:]),encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(args.page_only)
