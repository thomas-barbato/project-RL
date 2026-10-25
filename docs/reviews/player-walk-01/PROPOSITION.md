# PLAYER-WALK-01 — trois prototypes de déplacement

Demande du joueur, 23 octobre 2026 : voir les trois pistes d'animation pour
les comparer avant choix. Le lot prépare des aperçus, sans intégration au jeu.

- **WALK-01** : posture frontale existante, animation discrète des appuis.
- **WALK-02** : marche gauche/droite, deux orientations de trois quarts.
- **WALK-03** : marche dans huit directions, avec profils et vues de dos.

Référence commune : Bastion cuivre lourd BA-CU01-P04-r01, natif 64 × 64.
Les trois versions partagent parcours, cadence, sol et échelle. La galerie
présente une vue native et un agrandissement entier ×2, avec pause et avance
image par image. Les sols schématiques clair/sombre servent à la comparaison ;
ils ne sont pas de nouvelles ressources de décor proposées à l'intégration.

Précision du joueur : les déplacements sont case par case. Chaque déplacement
de la revue est donc limité à une case, y compris en diagonale. La position
arrivée est alignée sur la grille. Le bouton « Avancer d’une case » termine
un seul pas puis s'arrête. La lecture automatique marque 520 ms d'arrêt entre
les commandes de démonstration ; ce délai n'est pas une proposition de règle
ou de blocage des commandes dans le jeu. Huit poses de repos dédiées rendent
ces arrêts visibles, avec les deux pieds au sol.

Les poses tournées sont de nouveaux dessins de revue, non acceptés. Le premier
essai conserve les pixels de la référence et les compose à l'affichage.
Le lot est sans arme pour isoler la lecture des appuis. La compatibilité des
prises, des autres tenues et de leurs équipements reste à produire après le
choix d'une direction. La galerie n'est pas une capture du moteur.

La cadence actuelle de glissement, vérifiée dans TileGlide::SECONDS, est de
120 ms. Elle est la valeur initiale de la revue. Les transitions 200 ms et le ralenti 400 ms sont des réglages de revue,
sans changement des commandes, des tours ou de la simulation.

Source de la galerie : tools/art/player_walk_review_01.html.
Conditionnement reproductible : tools/art/build_player_walk_review_01.py.
Sorties temporaires : artifacts/player-walk-review-01/.
Sources des dessins supplémentaires : assets/source-art/player-walk-review-01/.

## Sources et fabrication

Les nouveaux dessins sont produits avec l'outil **imagegen intégré**, sans
appel CLI/API distinct, puis conditionnés sans modifier les sources acceptées.

- [48 poses de marche](../../../assets/source-art/player-walk-review-01/bastion-walk-eight-directions-source.png)
- [8 poses de repos](../../../assets/source-art/player-walk-review-01/bastion-idle-eight-directions-source.png)
- [Prompt de marche](prompt.txt)
- [Prompt de repos](prompt-idle.txt)
- [Bilan du conditionnement](packaging.json)

## Vérification effectuée

Le 23 octobre, la galerie est exécutée dans le navigateur, dans le cadre de
prévisualisation du skill visualize. Les trois scènes, les zooms entiers,
la pause, l'avance image par image et le changement clair/sombre fonctionnent.
Le bouton « Avancer d’une case » est essayé : le déplacement diagonal observé
rejoint la case adjacente, puis reste arrêté, sans commande supplémentaire.
Les personnages ne sortent pas des scènes et leurs positions de repos sont
alignées sur la grille de 64 px.

Le conditionnement vérifie 48 cellules non vides et distinctes, une échelle
commune aux poses de marche, un fond alpha et une toile native de 64 × 64.
Huit poses de repos supplémentaires sont incluses. La syntaxe du JavaScript
est vérifiée par Node. Aucun module du jeu n'est modifié et aucune exécution
dans le moteur n'est revendiquée pour ce lot.
