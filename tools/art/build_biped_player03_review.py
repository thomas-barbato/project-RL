"""Show the approved player next to the corresponding monster pose, in lockstep."""
import argparse
import re
import build_biped_lateral01_review as base


def main(page_only=False):
    if (base.ROOT/'artifacts/animation-polish-01/biped-lateral-02/r01/current/checks.json').exists():
        from build_biped_next02_review import main as current
        return current(page_only=page_only)
    base.OUT=base.ROOT/'artifacts/animation-polish-01/biped-lateral-01/r03/current'
    section=base.SECTION
    start=section.index('<p>Premier lot :');end=section.index('</p>',start)+4
    section=section[:start]+'''<p><strong>04 · Démon ailé sur le cycle du Bastion.</strong> La référence joueur et le nouveau monstre avancent sur la même phase, au même rythme et sur le même trajet. Les poses ont été dessinées à partir des vrais dessins du joueur.</p>'''+section[end:]
    section=section.replace('01 · Bipèdes : appuis et port d’arme','04 · Même démarche que le joueur')
    section=section.replace('Nouveaux dessins à examiner : huit poses, 650 ms par pas, puis vrai repos frontal. Le reste du bestiaire et les autres configurations d’armes restent à reprendre.','Nouvelle proposition : Bastion de référence et démon synchronisés, huit poses, 650 ms par pas. La carrure et les ailes restent celles du monstre. Les autres monstres et les armes attendent cette revue.')
    section=section.replace('Créature ou port d’arme','Créature en revue')
    section=section.replace('Nouvelle proposition : Bastion de référence et démon synchronisés, huit poses, 650 ms par pas. La carrure et les ailes restent celles du monstre. Les autres monstres et les armes attendent cette revue.','Animation du démon validée : 400 ms par pas, soit 800 ms par cycle. Le Bastion de référence reste synchronisé. Les autres monstres et les prises d’armes restent à décliner et à examiner.')
    section=section.replace('<option value="650">650 ms par pas</option><option value="800">800 ms par pas</option><option value="400">400 ms · ancien rythme</option>','<option value="400">400 ms par pas · retenu</option><option value="650">650 ms par pas</option><option value="800">800 ms par pas</option>')
    section=section.replace('<option value="walk">Marche actuelle / nouveau pas</option><option value="idle">Repos actuel / nouveau pas</option>','<option value="idle" selected>Joueur / démon même cycle</option><option value="walk">Ancien démon / nouveau</option>')
    section=section.replace('Marche actuelle au même rythme · nouveau cycle. Taille native, cases de 64 px ; mêmes décor et éclairage.','Joueur de référence · démon adapté. Taille native, cases de 64 px ; même décor et lumière du camp, traitement habituel de chaque acteur.')
    section=section.replace('Bipède au repos, marche actuelle et nouvelle démarche','Bastion de référence et démon sur la même phase de marche')
    section=section.replace('style="max-width:480px"','style="width:fit-content;max-width:100%"')
    section=section.replace('Le Bastion utilise le dessin partagé du fusil d’assaut et des bras dessinés pour les deux prises. Cet essai couvre sa tenue lourde et cette arme uniquement. Il ne valide pas encore toutes les armes ni toutes les classes.','Référence : Bastion en tenue lourde, sans arme, CLASS-LATERAL02-r02. MON-144-0 est validé en palette d’origine, sans arme. La cadence retenue pour la suite du chantier est de 400 ms par pas. L’intégration au jeu et les prises d’armes restent des étapes distinctes.')
    section=section.replace('biped-lateral-01/r01/','biped-lateral-01/r03/').replace('current/MON-005-0/native.webp','current/MON-144-0/native.webp')
    base.SECTION=section
    base.SCRIPT=base.SCRIPT.replace('biped-lateral-01/r01/','biped-lateral-01/r03/').replace('BIPED-LATERAL01-r01','BIPED-LATERAL01-r03')
    base.SCRIPT=base.SCRIPT.replace('Repos actuel · marche actuelle au même rythme · nouveau cycle. Taille native, cases de 64 px.','Bastion de référence · ancien démon r02 · nouveau démon r03. Taille native, cases de 64 px.')
    base.SCRIPT=base.SCRIPT.replace('Repos actuel · nouveau cycle. Taille native, cases de 64 px.','Joueur de référence · démon adapté. Taille native, cases de 64 px.')
    base.SCRIPT=base.SCRIPT.replace('Marche actuelle au même rythme · nouveau cycle. Taille native, cases de 64 px.','Ancien démon r02 · nouvelle proposition. Taille native, cases de 64 px.')
    base.main(page_only=page_only,labels=['04 · Démon ailé · validé'],headers=['Bastion référence','Ancien démon r02','Démon validé'],step_ms=400)
    html=base.PAGE.read_text(encoding='utf-8')
    html=re.sub(r'<title>.*?</title>','<title>Project RL · Cycle joueur et démon · BIPED-LATERAL01-r03</title>',html,count=1)
    html=re.sub(r'(<div class="tag">).*?(</div>)',r'\g<1>BIPED-LATERAL01-r03 · CYCLE JOUEUR DE RÉFÉRENCE\g<2>',html,count=1)
    html=re.sub(r'<p>(?:Reprise du démon ailé après le refus du premier lot\. Les trois classes restent comparables plus bas\.|Premier lot de monstres bipèdes et essai de port du fusil ; les trois classes restent comparables plus bas\.)</p>','<p>Le cycle du joueur sert de référence directe au démon ailé ; les autres revues restent disponibles plus bas.</p>',html,count=1)
    html=html.replace('Reprise du démon ailé après le refus du premier lot. Les trois classes restent comparables plus bas.','Le cycle du joueur sert de référence directe au démon ailé ; les autres revues restent disponibles plus bas.')
    base.PAGE.write_text(html,encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(page_only=args.page_only)
