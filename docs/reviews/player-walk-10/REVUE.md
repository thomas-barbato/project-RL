# WALK10-PROPORTIONS-BA00-r01 — gabarit du Bastion, proposition

Le joueur signale que les personnages deviennent plus grands ou plus épais
pendant un déplacement latéral, notamment le Bastion. L'audit compare les
dessins de face effectivement utilisés avec les figures WALK09, sans arme,
à la même échelle native64. Il couvre25 tenues et100 vues latérales : repos
et passage du pied, dans les deux sens.

## Cause confirmée

Le conditionnement WALK09 prend comme hauteur cible le repos des anciennes
animations WALK08. Il ne prend pas le dessin de face comme gabarit. Le contrôle
des deux pixels de variation interne vérifie donc la stabilité du cycle,
mais pas la continuité entre face et trois quarts.

Le Bastion fait58px de face et60 à61px dans ces vues latérales. Les volumes
de tête, poitrine, bras et bottes ont également dérivé pendant le dessin.
L'écart varie selon les tenues ; il est particulièrement marqué pour le
Technomancien et la Métamorphe. Une réduction identique de toutes les tenues
ne corrigerait pas leurs proportions différentes.

Les mesures et comparaisons sont dans
`../player-walk-09/volume-audit/metrics.json` et `*-front-vs-walk.png`.
Le ratio de largeur moyenne mesure les pixels occupés dans la partie haute
de la silhouette ; il inclut les bras et les recouvrements de pose. Il ne
constitue pas une mesure physique du volume du torse.

## Premier essai proposé

Le candidat02 concerne le Bastion en tenue1 uniquement. Les14 figures entières
sont redessinées avec le physique et la tenue de sa source de face acceptée.
La tête, le buste, les gants et les bottes reprennent ces proportions. Les
figures sont conditionnées uniformément en X/Y à57–58px, avec un appui stable ;
aucun membre n'est étiré ni greffé pour simuler la marche. La démarche compacte,
les six phases et le vrai repos restent l'objectif de cette retouche.

Les deux directions sont produites séparément avec l'outil de génération
intégré. Les sources sont dans
`assets/source-art/player-walk-review-10/CLASS-00-left-proportions-r01.png`
et `CLASS-00-right-proportions-r01.png`. Les références et consignes exactes
sont dans `prompts.json`, le conditionnement dans `conditioning.json`.

`Bastion-axe-front-current-proposal.gif` montre la face,01 la version actuelle
et02 le candidat, avec la hachette67 réellement composée par le lecteur du
jeu. La rangée du haut reste à64px, le détail du bas est agrandi deux fois.
Chaque marche couvre64px en400ms, puis revient au repos. La pause et la remise
à zéro appartiennent seulement à la démonstration. Le second GIF montre
la même comparaison sans arme.

## Statut et contrôles

Proposition à examiner avant intégration. Le lecteur normal et l'exécutable
Release gardent WALK09. Seul un diagnostic de composition explicitement lancé
dans les tests charge les parties candidates ; il a passé le contrôle avec
et sans hachette, dans les deux sens et pour les14 poses.

Les sources sélectionnées, leur alpha et les consignes sont conservés.
Les autres24 tenues ne sont pas corrigées par ce premier essai. Après examen
du gabarit, chaque tenue devra prendre sa propre face comme référence et
conserver l'identité de son armure, ses prises et la démarche acceptée.
L'audit des cultistes de WALK09 reste disponible et séparé de cette retouche.

## Vérification générale demandée ensuite

Le joueur demande de vérifier tous les personnages et monstres, de face et
pendant la marche. La vérification étendue couvre les25 tenues et casques,
les471 apparences de campagne, les formes, drones et anciennes voies de dessin.
Elle confirme plusieurs écarts et distingue les changements de posture des
changements de physique. Le bilan est dans `full-audit/RAPPORT.md`, avec les
mesures, les planches natives et les limites de couverture explicites.
