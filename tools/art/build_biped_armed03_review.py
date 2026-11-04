"""Publish three new bipeds, two with rigid two-handed weapon carries."""
import argparse
import re
import build_biped_next02_review as previous


def configure():
    previous.configure()
    base = previous.base
    base.OUT = base.ROOT/'artifacts/animation-polish-01/biped-lateral-03/r01/current'
    base.SECTION = base.SECTION.replace('Bipèdes : suite du cycle joueur à 400 ms', 'Bipèdes : ailes et armes à deux mains')
    base.SECTION = base.SECTION.replace('<strong>01 Possédé · 02 Cultiste · 03 Démon massif :</strong> nouvelles propositions sur les vrais dessins du cycle joueur. <strong>04 Démon ailé :</strong> animation déjà validée, conservée comme référence.', '<strong>01 Ailé d’obsidienne · 02 Cultiste au fusil · 03 Cultiste à la hache :</strong> nouvelles propositions sur le cycle joueur. <strong>04–07 :</strong> les quatre monstres déjà validés restent comparables sans modification de leurs dessins.')
    base.SECTION = base.SECTION.replace('Référence : Bastion lourd sans arme, cycle joueur conservé. Les identités et gabarits des monstres viennent de leur frontal actuel. Ce lot ne valide pas encore leurs prises d’armes, leurs autres palettes ni leur intégration au jeu.', 'Référence de démarche : Bastion lourd, cycle conservé. Sur 02 et 03, les bras portent l’arme d’origine à deux mains pendant le passage des pieds. Examiner les deux prises et la rigidité du fusil ou du manche à chaque phase. L’ailé conserve ses deux ailes et sa queue. Nouvelles silhouettes et prises restent en revue ; les autres palettes et l’intégration au jeu sont distinctes.')
    base.SECTION = base.SECTION.replace('biped-lateral-02/r01/', 'biped-lateral-03/r01/').replace('current/MON-005-0/native.webp', 'current/MON-145-0/native.webp')
    base.SCRIPT = base.SCRIPT.replace('biped-lateral-02/r01/', 'biped-lateral-03/r01/').replace('BIPED-LATERAL02-r01', 'BIPED-LATERAL03-r01')
    return base


def main(page_only=False):
    if (previous.base.ROOT/'artifacts/animation-polish-01/biped-lateral-03/r02/current/checks.json').exists():
        from build_biped_hoods_r02_review import main as current
        return current(page_only=page_only)
    base = configure()
    labels = ['01 · Ailé d’obsidienne · candidat', '02 · Cultiste au fusil · candidat', '03 · Cultiste à la hache · candidat', '04 · Possédé · validé', '05 · Cultiste · validé', '06 · Démon massif · validé', '07 · Démon ailé · validé']
    base.main(page_only=page_only, labels=labels, headers=['Joueur référence', 'Frontal actuel', 'Cycle joueur'], step_ms=400)
    html = base.PAGE.read_text(encoding='utf-8')
    html = re.sub(r'<title>.*?</title>', '<title>Project RL · Bipèdes et armes · BIPED-LATERAL03-r01</title>', html, count=1)
    html = re.sub(r'(<div class="tag">).*?(</div>)', r'\g<1>BIPED-LATERAL03-r01 · AILES ET ARMES À DEUX MAINS · 400 MS\g<2>', html, count=1)
    html = html.replace('Trois nouveaux bipèdes reprennent le cycle joueur à 400 ms par pas ; le démon ailé validé reste comparable.', 'Un nouvel ailé et deux cultistes armés reprennent le cycle joueur à 400 ms par pas ; les quatre modèles validés restent comparables.')
    base.PAGE.write_text(html, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--page-only', action='store_true'); args = parser.parse_args()
    main(page_only=args.page_only)
