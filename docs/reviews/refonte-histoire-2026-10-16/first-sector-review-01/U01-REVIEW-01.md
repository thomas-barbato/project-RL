# U01 — abords de la prison, première revue détaillée

18 octobre 2026. **Plan et nouveaux décors candidats, non intégrés au jeu.**

La revue suivante [U01-REVIEW-02](U01-REVIEW-02.md) réduit la voiture à 2 × 1 cases,
ajoute la proposition de flammes animées et présente les bâtiments. Cette page
conserve les dimensions et limites de la première proposition.

L'agencement du chapitre ACT1-LAYOUT-01-r01 est accepté. La présente revue
précise sa zone U01 ; cet accord général ne valide pas ses nouvelles cases,
dimensions ou illustrations. Les intitulés restent des repères de conception.

Deux éléments sont présentés séparément :

- **MAP-U01-01-r01** : plan fixe de 40 × 28 cases, circulation et obstacles.
- **DEC-U01-01-r01** : kit de seize éléments de terrain de 64 × 64 px et
  voiture DEC-U01-17-r02 sur un canevas de 192 × 128 px, proposée sur 3 × 2 cases.

## Lecture du plan

Le joueur arrive par l'ouest depuis la sortie du camp accepté. Une chaussée
de huit cases de large, bordée de trottoirs de deux cases, conduit vers le
quartier résidentiel à l'est. Au nord, un accès de quatre cases mène au futur
refuge local. Un renfoncement au sud donne une possibilité de détour et de
repli sans nouveau passage de campagne ou objectif obligatoire.

Les blocs de barricade, la voiture et quelques débris interrompent les lignes
droites sans barrer la rue. Une circulation continue de trois cases de large
reste possible autour des obstacles sur le trajet principal. Ces dimensions
constituent une proposition à éprouver en combat, pas un standard appliqué
silencieusement à toutes les cartes suivantes.

Le trajet cardinal montré depuis le point de revue jusqu'au raccord U02 fait
38 pas. Les 532 cases praticables sont reliées ; le point de retour P00 et
l'approche F00 sont accessibles. Ce sont des contrôles de la grille Python,
sans simulation des tours, des monstres, de la visibilité ou des familiers.

Le raccord de camp prévu vise son point existant (37,8). Aucun pixel ni élément
du camp validé n'a été déplacé. Le déclenchement réciproque, les cases d'arrivée
et la fermeture technique hors dessin exigée par le moteur devront être
vérifiés lors du raccordement. Ni F00 ni U02 ne sont des cartes créées ici.

## Portée des nouveaux dessins

Le générateur d'images intégré fournit l'atlas urbain en utilisant le camp
accepté comme référence de style. L'atlas original est conservé sans retouche.
Les rectangles de découpe écartent ses séparateurs noirs ; chaque élément
est exporté en 64 × 64 px avec sa découpe et son empreinte dans le manifeste.

Les candidats 01–08 concernent l'asphalte, les trottoirs, les marquages et
les bordures. Les candidats 09–12 sont des limites de façade orientées.
Les candidats 13–16 concernent les blocs, débris, drainage et seuil ouvert.
Le kit ne prétend pas fournir déjà tous les raccords de façade de la ville.
L'assemblage emploie des bandes issues des dessins orientés sans tourner les
façades. Seuls les deux sols d'asphalte utilisent des réflexions de texture
locales pour atténuer la répétition, sans toucher aux sources ni au hasard du
jeu. Les bandes et leurs raccords font partie du rendu soumis à examen.

La voiture 17 r02 reprend le premier essai avec un traitement simplifié et
une vue relevée. Sa source transparente reste intacte ; son export est contenu
dans un canevas de 192 × 128 px sans déformer ses proportions. Son emprise de
3 × 2 cases et le blocage représenté restent des propositions de cette revue.
L'essai r01 n'est ni accepté ni employé dans l'assemblage.

Le joueur accepté sert seulement de référence de taille. Son dessin de 128 px
avec débordement est composé à l'échelle 1, avec son ancrage (64,96), comme
dans le camp ; il n'est pas réduit à un carré de 64 px. Son corps visible
conserve ses pixels et sa hauteur d'environ 64 px.

**Les masses sombres des bâtiments sont encore schématiques.** Leurs toits,
étages, fenêtres, enseignes et intérieurs ne constituent pas des dessins
proposés ici. La vue native permet de juger les matériaux de rue, leurs
premiers raccords et l'échelle des objets ; elle ne présente pas une ville
achevée. Les portes ne sont pas remplacées par des lumières et les ouvertures
du plan restent dégagées.

## Fichiers et reproduction

Source du plan : `u01-layout-r01.json`. Sources des dessins :
`assets/source-art/first-sector-review-01/u01-r01/`.
Prompts exacts : `u01-decor-prompt-r01.txt`, `u01-car-prompt-r01.txt` et
`u01-car-prompt-r02.txt`. Fabrication : outil **image_gen intégré**, puis
découpe et export Pillow ; aucun appel API ou modèle CLI supplémentaire.

```text
python -X utf8 tools/reviews/build_act1_city_layout.py
python -X utf8 tools/reviews/build_u01_city_review.py --revision r01
```

La page courante `artifacts/first-sector-review-01/index.html` est réutilisée.
Elle présente le plan, deux vues natives de rue, la planche de candidats et
l'assemblage complet de 2560 × 1792 px. Ses images de jeu gardent leur taille
native et se parcourent par défilement. La vue d'ensemble du plan possède
sa propre échelle de schéma et ne réduit aucun dessin du joueur.

`u01-layout-checks-r01.json` conserve les contrôles de circulation et les
empreintes des sorties. `u01-decor-manifest-r01.json` conserve les sources
et exports des dessins. `u01-review-inspection-r01.json` conserve l'examen
visuel et la vérification des liens de la page sur ces sorties. Le jeu,
ses sauvegardes et `artifacts/map-editor/`
ne sont pas modifiés. Aucun nouvel ennemi, dialogue, butin, pouvoir, condition
de quête, coût de déplacement ou éclairage animé n'est intégré par cette revue.

La prochaine étape consiste à examiner le plan, l'échelle et ce premier kit,
puis compléter les façades urbaines et vérifier leur rendu avec les collisions
dans le prototype moteur. Les accords sur ces parties seront enregistrés
séparément ; la géométrie du chapitre déjà acceptée reste inchangée.
