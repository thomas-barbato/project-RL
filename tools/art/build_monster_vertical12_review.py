"""Wing-arms, a four-legged crawler and a heavy biped; retain accepted lot11."""
import argparse
import json
import build_monster_vertical28_arms_review as previous
from build_monster_vertical03_review import bank

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-12/r01/current'
base.PREFIX='monster-vertical-12/r01/current/'
base.REVIEW='MONSTER-VERTICAL12-r01'


def main(page_only=False):
    retained=previous.retained_bank()
    retained['items']=bank('monster-vertical-11')+bank('monster-vertical-28-weapon')+retained['items']
    retained['labels']=['31 · Quadrupède à queue · validé','32 · Quadrupède trapu · validé','33 · Créature au corps enroulé · validé','28 · Démon rouge élancé · lame, bras et couleurs validés']+retained['labels']
    assert len(retained['items'])==len(retained['labels'])==33
    assert len({p['id'] for p in retained['items']})==33
    path=base.OUT/'checks.json';data=json.loads(path.read_text(encoding='utf-8'))
    for item in data['items']:
        if item['id']=='MON-067-0':
            item['scope_note']='35 · quatre courtes pattes porteuses, corps bas et sombre · bouche ronde devant uniquement · frontal conservé au repos.'
            item['phase_names']=['contact avant gauche','charge gauche','reprise arrière droite','passage avant droit','contact avant droit','charge droite','reprise arrière gauche','passage avant gauche','Repos frontal']
        else:
            item['scope_note']='34 · deux pieds au sol, ailes portées par les bras, sans bras supplémentaires ni envol.' if item['id']=='MON-066-0' else '36 · deux appuis lourds alternés, pas courts, carapace rigide et bras relâchés · les grandes griffes restent attachées aux mains.'
            item['phase_names']=['Contact A','Appui A','Passage du pied B','Reprise du pied B','Contact B','Appui B','Passage du pied A','Reprise du pied A','Repos frontal']
    path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    base.main(page_only,labels=['34 · Créature aux bras ailés','35 · Rampant sombre à bouche ronde','36 · Démon massif à carapace'],previous=retained,
              intro='Suite : 34 créature aux bras ailés, 35 rampant sombre à bouche ronde et 36 démon massif à carapace. Deux pieds pour les créatures 34 et 36 ; quatre pattes pour le rampant. Les n°31–33 sont validés. Trente-six modèles proposés ; 93 autres restent à reprendre.')
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('deux appuis alternés sur 800 ms','un cycle de 800 ms adapté aux appuis de chaque morphologie')
    section=section.replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','Vérifier les deux bras ailés sans bras ajoutés, les quatre pattes du rampant et sa bouche visible de face uniquement. Le démon massif garde ses bras abaissés et sa carapace attachée au torse. Comparer les couleurs et le retour au frontal conservé.')
    base.PAGE.write_text(previous.label_comparison_columns(html[:start]+section+html[end:]),encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(args.page_only)
