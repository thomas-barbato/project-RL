"""Three bipeds, preserving all thirty-nine previous selections."""
import argparse
import json
import build_monster_vertical28_arms_review as previous
from build_monster_vertical03_review import bank

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-14/r01/current'
base.PREFIX='monster-vertical-14/r01/current/'
base.REVIEW='MONSTER-VERTICAL14-r01'


def retained_bank():
    retained=previous.retained_bank()
    retained['items']=bank('monster-vertical-13')+bank('monster-vertical-12')+bank('monster-vertical-11')+bank('monster-vertical-28-weapon')+retained['items']
    retained['labels']=['37 · Créature rouge à six pattes · validé','38 · Créature aux avant-bras recourbés · validé','39 · Arachnide · validé','34 · Créature aux bras ailés · validé','35 · Rampant sombre à bouche ronde · validé','36 · Démon massif à carapace · validé','31 · Quadrupède à queue · validé','32 · Quadrupède trapu · validé','33 · Créature au corps enroulé · validé','28 · Démon rouge élancé · lame, bras et couleurs validés']+retained['labels']
    assert len(retained['items'])==len(retained['labels'])==39
    assert len({p['id'] for p in retained['items']})==39
    return retained


def main(page_only=False, rear=False):
    if rear:
        base.PREFIX='monster-vertical-14/r02/current/'
        base.OUT=base.PAGE.parent/base.PREFIX
        base.REVIEW='MONSTER-VERTICAL14-r02'
    retained=retained_bank()
    path=base.OUT/'checks.json';data=json.loads(path.read_text(encoding='utf-8'))
    for item in data['items']:
        item['scope_note']=item['name']+' · deux appuis alternés, bras portés bas et frontal conservé à l’arrêt.'
        if rear and item['id']=='MON-074-0':
            item['scope_note']='N°42 · Le dos est redessiné pendant toute la marche vers le haut. Marche vers le bas et frontal à l’arrêt conservés.'
        item['phase_names']=['Contact A','Appui A','Passage du pied B','Reprise du pied B','Contact B','Appui B','Passage du pied A','Reprise du pied A','Repos frontal']
    path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    base.main(page_only,labels=['40 · Démon trapu à épaule rouge','41 · Démon pâle voûté','42 · Démon aux griffes dorées'],previous=retained,
              intro=('Correction du n°42 : la marche vers le haut montre désormais son dos. Les n°40 et 41 restent inchangés.' if rear else 'Suite : trois bipèdes aux silhouettes distinctes, du démon trapu aux grandes griffes dorées. Les n°37–39 sont validés. Quarante-deux modèles proposés ; 87 autres restent à reprendre.'))
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('deux appuis alternés sur 800 ms','un cycle de 800 ms adapté aux appuis de chaque morphologie')
    section=section.replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','Vérifier les pas courts, la carrure, le côté des parties asymétriques et le raccord au frontal. Les grandes griffes du n°42 suivent ses avant-bras.')
    if rear:
        section=section.replace('<option value="MON-074-0">','<option value="MON-074-0" selected>').replace('<option value="up">','<option value="up" selected>')
    base.PAGE.write_text(previous.label_comparison_columns(html[:start]+section+html[end:]),encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');parser.add_argument('--rear',action='store_true');args=parser.parse_args();main(args.page_only,args.rear)
