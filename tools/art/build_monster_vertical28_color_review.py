"""Show retained idle and identical moving poses before/after material balance."""
import argparse
import json
import build_monster_vertical28_arms_review as previous

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-28-color/r01/current'
base.PREFIX='monster-vertical-28-color/r01/current/'
base.REVIEW='MONSTER-VERTICAL28-COLOR01-r01'

def main(page_only=False):
    retained=previous.retained_bank()
    path=base.OUT/'checks.json';data=json.loads(path.read_text(encoding='utf-8'))
    item=data['items'][0];assert item['id']=='MON-060-0'
    item['scope_note']='28 · couleurs harmonisées avec le repos · mêmes poses et bras relâchés · 29 et 30 validés.'
    path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    base.main(page_only,labels=['28 · Démon rouge élancé · couleurs harmonisées'],previous=retained,
              intro='N°28 : harmonisation des couleurs de marche avec le repos. Les poses et les bras relâchés sont conservés. Comparer le frontal d’origine, les mêmes poses avant correction et leur palette harmonisée. Les n°29 et 30 restent validés ; trente modèles restent sélectionnables.')
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('deux appuis alternés sur 800 ms','un cycle de 800 ms adapté aux appuis de chaque morphologie')
    section=section.replace('Nouveaux dessins en revue.','N°28 : correction de couleur en revue.')
    section=section.replace('Chaque nouvelle figure est dessinée entière. Les autres morphologies suivront leur locomotion propre.','Les mêmes dessins sont conservés ; seule leur palette change.')
    section=section.replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','N°28 : vérifier la continuité des couleurs entre marche et arrêt. Les couleurs de chair et des parties sombres sont rapprochées du frontal ; les poses, contours, transparence et bras restent identiques entre les deux colonnes mobiles. Même éclairage du moteur, à 400 ms.')
    base.PAGE.write_text(previous.label_comparison_columns(html[:start]+section+html[end:]),encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(args.page_only)
