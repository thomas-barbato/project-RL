"""A single low-step demon replaces the rejected batch in the active preview."""
import argparse
import re
import build_biped_lateral01_review as base


def main(page_only=False):
    if (base.ROOT/'artifacts/animation-polish-01/biped-lateral-01/r03/current/checks.json').exists():
        from build_biped_player03_review import main as current
        return current(page_only=page_only)
    base.OUT=base.ROOT/'artifacts/animation-polish-01/biped-lateral-01/r02/current'
    section=base.SECTION
    start=section.index('<p>Premier lot:') if '<p>Premier lot:' in section else section.index('<p>Premier lot :')
    end=section.index('</p>',start)+4
    section=section[:start]+'''<p><strong>04 · Démon ailé au sol, reprise ciblée.</strong> Le premier lot est refusé : jambes qui pédalent ou montent trop, corps artificiel. Les autres monstres attendent cette reprise.</p>'''+section[end:]
    section=section.replace('01 · Bipèdes : appuis et port d’arme','04 · Démon ailé : pas courts et rasants')
    section=section.replace('Nouveaux dessins à examiner : huit poses, 650 ms par pas, puis vrai repos frontal. Le reste du bestiaire et les autres configurations d’armes restent à reprendre.','Nouvel essai : pieds plus près du sol, faible amplitude, torse et ailes moins mobiles. Même cadence de 650 ms par pas ; trajet de comparaison raccourci à 12 px par cycle.')
    section=section.replace('Créature ou port d’arme','Créature en reprise')
    section=section.replace('Marche actuelle / nouveau pas','Essai refusé / pas rasants').replace('Repos actuel / nouveau pas','Repos actuel / pas rasants')
    section=section.replace('Marche actuelle au même rythme · nouveau cycle.','Essai refusé · pas rasants, au même rythme.')
    section=section.replace('Le Bastion utilise le dessin partagé du fusil d’assaut et des bras dessinés pour les deux prises. Cet essai couvre sa tenue lourde et cette arme uniquement. Il ne valide pas encore toutes les armes ni toutes les classes.','Ce nouvel essai concerne uniquement MON-144-0, palette d’origine, sans arme. Le port des armes reste à examiner après la démarche ; l’ancien essai du Bastion ne vaut pas validation.')
    section=section.replace('biped-lateral-01/r01/','biped-lateral-01/r02/')
    section=section.replace('current/MON-005-0/native.webp','current/MON-144-0/native.webp')
    base.SECTION=section
    base.SCRIPT=base.SCRIPT.replace('biped-lateral-01/r01/','biped-lateral-01/r02/').replace('BIPED-LATERAL01-r01','BIPED-LATERAL01-r02')
    base.SCRIPT=base.SCRIPT.replace('Marche actuelle au même rythme · nouveau cycle.','Essai refusé · pas rasants, au même rythme.')
    base.main(page_only=page_only,labels=['04 · Démon ailé · pas rasants'],headers=['Repos actuel','Essai refusé r01','Pas rasants r02'])
    html=base.PAGE.read_text(encoding='utf-8')
    html=re.sub(r'<title>.*?</title>','<title>Project RL · Démon ailé · BIPED-LATERAL01-r02</title>',html,count=1)
    html=re.sub(r'(<div class="tag">).*?(</div>)',r'\g<1>BIPED-LATERAL01-r02 · REPRISE DU DÉMON AILÉ\g<2>',html,count=1)
    html=html.replace('Premier lot de monstres bipèdes et essai de port du fusil ; les trois classes restent comparables plus bas.','Reprise du démon ailé après le refus du premier lot. Les trois classes restent comparables plus bas.')
    base.PAGE.write_text(html,encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(page_only=args.page_only)
