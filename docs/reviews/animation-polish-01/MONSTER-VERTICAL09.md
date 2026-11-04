# MONSTER-VERTICAL09-r01 — trois petites créatures à quatre appuis

Le 3 novembre 2026, « je valide :) on continue » accepte la correction
quadrupède du n°21, MON-032-0. Ses quatre ressources sont figées dans
`MONSTER-VERTICAL21-QUAD01-ACCEPTANCE-r01.json`. Son ancien essai bipède
reste rejeté et archivé ; cet accord ne le rétablit pas.

Le neuvième lot propose les variantes 0 de :

25. MON-035-0 : créature pâle aux griffes, plis rouges et ouverture frontale.
26. MON-037-0 : petit quadrupède pâle, dos sombre et petite ouverture rouge.
27. MON-038-0 : créature à gueule orangée, dos bleu-gris et pattes avant pâles.

Ce sont des libellés de revue. L’examen des références confirme quatre
membres porteurs pour chacun : deux antérieurs et deux postérieurs.
La marche doit garder le corps près du sol et les pattes avant porteuses,
sans bras pendants, redressement, torse humain, membres supplémentaires
ou appendice copié du guide. Le guide de mouvement est le quadrupède 21
validé ; il ne sert pas de référence d’identité.

Les dessins actuels mesurent respectivement 48 × 31, 37 × 25 et 51 × 33 px.
Conserver ces silhouettes larges et basses, leurs matières et les différences
de gabarit. De dos, les ouvertures frontales sont occultées par les corps ;
elles ne deviennent pas une seconde bouche. MON-038 garde ses coins relevés
et ses membres avant et arrière de tailles différentes, sans visage ajouté
dans la cavité. Les guides arrière actuels servent à vérifier cette orientation.

Huit figures entières par direction, 400 ms par avancée et 800 ms par cycle.
Appuis alternés entre les quatre pattes, pas courts et faible oscillation.
Une échelle uniforme par direction et une translation du cycle alignent
l’appui moyen, sans zoom par phase, greffe de pattes ni déformation de bandes.
Le repos conserve exactement le frontal VERT18, son ancrage et son ombre.
Les phases du lecteur utilisent les groupes d’appuis propres aux quadrupèdes.

Sources et atlas :
`assets/source-art/animation-polish-01/monster-vertical-09/r01/manifest.json`.
Six planches dessinées avec l’outil intégré image_gen en mode édition avec
conservation d’identité. Prompts et références exacts dans
`MONSTER-VERTICAL09-PROMPTS.json`. Le diagnostic
`MONSTER-VERTICAL09-COLORS.json` lit les matériaux visibles par direction,
sans modifier les PNG ni remplacer la revue visuelle native.

La première capture native montrait une chair trop claire sur MON-035.
Une palette de préparation commune à tout son cycle rapproche les valeurs
de la chair du frontal conservé. Elle ne change ni les planches sources,
ni l’alpha, ni les autres matériaux, ni les deux repos. Le fichier
`MON-035-ivory-palette.json` est limité à ce candidat dans le lecteur de revue.
Le diagnostic de couleurs ci-dessus reste celui des planches brutes.

La préparation préserve 146 empreintes antérieures, dont les quatre fichiers
de l’essai bipède du n°21 rejeté et les quatre fichiers de sa correction
quadrupède acceptée. Les dix atlas latéraux intégrés restent inchangés.
Préparer les ressources avant affichage et créer les textures dans le thread
du contexte graphique, sans lecture lourde dans le dessin.

Statut lors de la proposition : candidats 25–27 à examiner avant intégration. Les
24 références précédentes restent disponibles, avec une seule entrée du
n°21 à quatre appuis. Vingt-sept modèles proposés au total ; 102 autres
restent à reprendre, puis les variantes restantes à contrôler. Les captures
ne chargent aucune partie ni préférence et n’activent pas ces cycles dans
la partie normale. Les identifiants, rôles et règles de jeu restent les mêmes.

```powershell
python tools/art/prepare_monster_vertical09.py --references
python tools/art/prepare_monster_vertical09.py
python tools/art/audit_monster_vertical09_colors.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo run --locked --bin project-rl -- --ui-cold-monster-vertical09 artifacts/animation-polish-01/monster-vertical-09/r01/current
python tools/art/build_monster_vertical09_review.py
python tools/art/check_monster_vertical09.py
```

Vérification du 3 novembre 2026 : compilation verrouillée réussie, six tests
de palette réussis, format Rust et `git diff --check` réussis. Les tests de
MON-035 vérifient les quantiles de chair, les autres matériaux, chaque alpha
et les repos contre les ressources réelles. Sa palette prépare 2 668 pixels
de marche sans toucher aux sources.

Le contrôle natif valide 48 figures de marche distinctes, six cellules de
repos exactes, 198 captures et trois animations de 2 400 ms comprenant les
deux directions et les arrêts. Les largeurs médianes restent comprises entre
0,95 et 1,03 fois le frontal conservé. Les six sorties de génération et leurs
douze références d’entrée sont vérifiées par empreinte.

Exécution du lecteur dans le navigateur : 27 sélections uniques chargent
chacune 66 captures ; les trois nouveaux modèles passent automatiquement
du bas au haut à 400 ms. Six arrêts manuels, les libellés distincts de pattes
et de pieds, les 129 fiches d’inventaire et le canevas natif 576 × 320 sont
contrôlés. Aucune erreur JavaScript n’est relevée. Une seule entrée du n°21
charge la correction quadrupède validée. Preuves et détail dans
`artifacts/animation-polish-01/monster-vertical-09/r01/browser-checks.json`,
`browser-proof.png` et `browser-back-proof.png` ; bilan durable dans
`MONSTER-VERTICAL09-VALIDATION.json`. Ces vérifications ne constituent pas
une intégration en partie normale ni une validation visuelle du joueur.

Acceptation du 3 novembre 2026 : « je valide :) », puis « et on continue »,
valide les n°25–27, leur démarche quadrupède à 400 ms et la palette préparée
de MON-035. Les treize ressources sont figées dans
`MONSTER-VERTICAL09-ACCEPTANCE-r01.json`. La suite est le lot 10 ; cette
acceptation visuelle n’active pas les nouveaux cycles en partie normale.
