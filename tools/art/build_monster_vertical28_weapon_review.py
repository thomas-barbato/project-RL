"""Restore the original blade, retaining relaxed arms and corrected skin colors."""
import argparse
import json
import build_monster_vertical28_arms_review as previous

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-28-weapon/r01/current'
base.PREFIX='monster-vertical-28-weapon/r01/current/'
base.REVIEW='MONSTER-VERTICAL28-WEAPON01-r01'

def main(page_only=False):
    retained=previous.retained_bank()
    path=base.OUT/'checks.json';data=json.loads(path.read_text(encoding='utf-8'))
    item=data['items'][0];assert item['id']=='MON-060-0'
    item['scope_note']='28 · lame d’origine dans la main droite : à gauche de face, à droite de dos · bras relâchés et couleurs harmonisées · 29 et 30 validés.'
    path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    base.main(page_only,labels=['28 · Démon rouge élancé · arme rétablie'],previous=retained,
              intro='N°28 : la lame d’origine est rétablie dans la main droite pendant la marche. Les bras restent relâchés et les couleurs sont harmonisées avec le repos. Comparer le frontal, la version sans arme et la reprise armée. Les n°29 et 30 restent validés ; trente modèles restent sélectionnables.')
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('deux appuis alternés sur 800 ms','un cycle de 800 ms adapté aux appuis de chaque morphologie')
    section=section.replace('Nouveaux dessins en revue.','N°28 : arme rétablie en revue.')
    section=section.replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','N°28 : vérifier la lame tenue dans la main droite sur chaque phase. Elle se trouve à gauche de l’image de face, à droite de dos, pointe vers le bas. La main gauche reste libre. Vérifier la prise, la visibilité de l’acier et le raccord au repos.')
    base.PAGE.write_text(previous.label_comparison_columns(html[:start]+section+html[end:]),encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(args.page_only)
