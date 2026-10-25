# PLAYER-WALK-02 — poses équipées et direction retenue

Le 23 octobre 2026, le joueur préfère la piste WALK-02 gauche/droite, et demande
de voir les armes équipées. Il accepte le principe suivant : les déplacements
strictement verticaux gardent le personnage de face. Le précédent prototype
gardait encore la dernière orientation latérale ; ce comportement est corrigé
dans cette nouvelle revue. Sur les diagonales, la composante horizontale
sélectionne gauche ou droite. Chaque commande reste limitée à une case.

Le joueur préfère une transition visuelle de **400 ms** et demande de garder
le choix. La nouvelle revue démarre donc à 400 ms et conserve 120 et 200 ms.
L'accord porte sur cette direction et cette préférence de durée ; il ne vaut
pas acceptation des nouveaux dessins équipés ou autorisation d'intégration.
Lors d'un futur raccordement, la durée restera visuelle : une nouvelle commande
ne devra pas attendre la fin du pas et repartira de la position affichée,
comme le glissement actuellement accepté. Les tours et positions logiques
resteront appliqués par la simulation existante.

Les candidats WA02-01 à WA02-05 montrent pistolet, marteau à une main, fusil,
hache à deux mains et deux pistolets. Le Bastion cuivre lourd reste la référence.
Les nouvelles prises sont dessinées avec imagegen intégré à partir des trois
vues retenues et des armes existantes. Il s'agit de poses de revue, pas encore
des couches d'équipement raccordées au moteur. Le bas du corps animé réutilise
la première revue. Les autres classes, toutes les armes du catalogue et les
palettes ne sont pas validées par ces cinq exemples.

Le jeu n'est pas modifié. Les sources acceptées, l'ancienne galerie et sa
publication fournie à l'identique sont conservées.

Sources : assets/source-art/player-walk-review-02/.
Prompts : prompts/ dans cette revue.
Conditionnement : tools/art/build_player_walk_review_02.py.
Galerie : tools/art/player_walk_review_02.html.

Les cinq dessins source sont dans :

- [WA02-01 — pistolet](../../../assets/source-art/player-walk-review-02/WA02-01-pistol-source.png), [prompt](prompts/pistol.txt).
- [WA02-02 — marteau](../../../assets/source-art/player-walk-review-02/WA02-02-hammer-source.png), [prompt](prompts/hammer.txt).
- [WA02-03 — fusil](../../../assets/source-art/player-walk-review-02/WA02-03-rifle-source.png), [prompt](prompts/rifle.txt).
- [WA02-04 — hache](../../../assets/source-art/player-walk-review-02/WA02-04-axe-source.png), [prompt](prompts/axe.txt).
- [WA02-05 — deux pistolets](../../../assets/source-art/player-walk-review-02/WA02-05-dual-source.png), [prompt](prompts/dual.txt).

## Vérification de la revue

La galerie est exécutée dans le navigateur le 23 octobre. La sélection des armes,
la pause, l'avance d'une case, l'examen des images et les sols clair/sombre sont
contrôlés. Les deux scènes montrent simultanément les déplacements latéraux et
verticaux ; la seconde conserve la vue de face. Le sélecteur démarre à 400 ms
et garde les deux durées alternatives. La syntaxe JavaScript est contrôlée.

Les sources nouvellement dessinées proposent des prises à cette échelle ;
elles ne sont pas des exports du compositeur du jeu. La comparaison ne valide
donc pas le raccordement de toutes les armes, mains, armures et palettes.
Le contrôle d'intégration dans le moteur restera distinct de cette revue.

## Publication de l'aperçu

Cette revue est publiée dans le même projet Sites privé :
https://player-walk-rl-23-oct.braveiris11.chatgpt.seuite/armes.html

Déploiement confirmé `succeeded`, version
`appgprj_6adb2f2dc3348191af0c2dce5eb37cb8~appgver_e5ef4ab2cd148191b53304c6e210ace4`.
La page d'origine `index.html` fournie par le joueur est conservée octet pour
octet ; la nouvelle page garde un iframe en bac à sable et sa CSP.
