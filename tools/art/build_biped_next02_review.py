"""Three new silhouettes at the accepted 400ms cadence, plus frozen MON-144."""
import argparse
import re
import build_biped_lateral01_review as base


def configure():
    base.OUT=base.ROOT/'artifacts/animation-polish-01/biped-lateral-02/r01/current'
    section=base.SECTION
    start=section.index('<p>Premier lot :');end=section.index('</p>',start)+4
    section=section[:start]+'''<p><strong>01 Possédé · 02 Cultiste · 03 Démon massif :</strong> nouvelles propositions sur les vrais dessins du cycle joueur. <strong>04 Démon ailé :</strong> animation déjà validée, conservée comme référence.</p>'''+section[end:]
    section=section.replace('01 · Bipèdes : appuis et port d’arme','Bipèdes : suite du cycle joueur à 400 ms')
    section=section.replace('Nouveaux dessins à examiner : huit poses, 650 ms par pas, puis vrai repos frontal. Le reste du bestiaire et les autres configurations d’armes restent à reprendre.','Cadence retenue : 400 ms par pas, 800 ms par cycle. Le joueur et le monstre sont synchronisés. Les trois nouveaux modèles restent à examiner avant intégration.')
    section=section.replace('Créature ou port d’arme','Monstre à comparer')
    section=section.replace('<option value="650">650 ms par pas</option><option value="800">800 ms par pas</option><option value="400">400 ms · ancien rythme</option>','<option value="400">400 ms par pas · retenu</option><option value="650">650 ms par pas</option><option value="800">800 ms par pas</option>')
    section=section.replace('<option value="walk">Marche actuelle / nouveau pas</option><option value="idle">Repos actuel / nouveau pas</option>','<option value="idle" selected>Joueur / monstre même cycle</option><option value="walk">Face actuelle / nouvelle marche</option>')
    section=section.replace('Marche actuelle au même rythme · nouveau cycle. Taille native, cases de 64 px ; mêmes décor et éclairage.','Joueur de référence · monstre adapté. Taille native, cases de 64 px ; même décor et lumière du camp, traitement habituel de chaque acteur.')
    section=section.replace('Bipède au repos, marche actuelle et nouvelle démarche','Joueur et monstre sur la même phase de marche')
    section=section.replace('style="max-width:480px"','style="width:fit-content;max-width:100%"')
    section=section.replace('Le Bastion utilise le dessin partagé du fusil d’assaut et des bras dessinés pour les deux prises. Cet essai couvre sa tenue lourde et cette arme uniquement. Il ne valide pas encore toutes les armes ni toutes les classes.','Référence : Bastion lourd sans arme, cycle joueur conservé. Les identités et gabarits des monstres viennent de leur frontal actuel. Ce lot ne valide pas encore leurs prises d’armes, leurs autres palettes ni leur intégration au jeu.')
    section=section.replace('biped-lateral-01/r01/','biped-lateral-02/r01/')
    base.SECTION=section
    base.SCRIPT=base.SCRIPT.replace('biped-lateral-01/r01/','biped-lateral-02/r01/').replace('BIPED-LATERAL01-r01','BIPED-LATERAL02-r01')
    base.SCRIPT=base.SCRIPT.replace("'Créature '+item.id+' · palette d’origine · huit poses adaptées à sa morphologie.'","'Créature '+item.id+' · '+(item.accepted?'animation validée':'nouvelle proposition')+' · 400 ms par pas.'")
    base.SCRIPT=base.SCRIPT.replace('Repos actuel · marche actuelle au même rythme · nouveau cycle. Taille native, cases de 64 px.','Joueur de référence · frontal actuel · cycle adapté. Taille native, cases de 64 px.')
    base.SCRIPT=base.SCRIPT.replace('Repos actuel · nouveau cycle. Taille native, cases de 64 px.','Joueur de référence · monstre adapté. Taille native, cases de 64 px.')
    base.SCRIPT=base.SCRIPT.replace('Marche actuelle au même rythme · nouveau cycle. Taille native, cases de 64 px.','Frontal actuel · nouvelle marche. Taille native, cases de 64 px.')


def main(page_only=False):
    if (base.ROOT/'artifacts/animation-polish-01/biped-lateral-03/r01/current/checks.json').exists():
        from build_biped_armed03_review import main as current
        return current(page_only=page_only)
    configure()
    base.main(page_only=page_only,labels=['01 · Possédé · candidat','02 · Cultiste · candidat','03 · Démon massif · candidat','04 · Démon ailé · validé'],headers=['Joueur référence','Frontal actuel','Cycle joueur'],step_ms=400)
    html=base.PAGE.read_text(encoding='utf-8')
    html=re.sub(r'<title>.*?</title>','<title>Project RL · Bipèdes à 400 ms · BIPED-LATERAL02-r01</title>',html,count=1)
    html=re.sub(r'(<div class="tag">).*?(</div>)',r'\g<1>BIPED-LATERAL02-r01 · TROIS NOUVEAUX BIPÈDES · 400 MS\g<2>',html,count=1)
    html=html.replace('Le cycle du joueur sert de référence directe au démon ailé ; les autres revues restent disponibles plus bas.','Trois nouveaux bipèdes reprennent le cycle joueur à 400 ms par pas ; le démon ailé validé reste comparable.')
    base.PAGE.write_text(html,encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(page_only=args.page_only)
