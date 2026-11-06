# DRONE-MOTION01-r02 — repos de face

Le joueur trouve la vue de trois quarts étrange pour un drone au repos.
La Mule et le Traqueur reprennent une orientation frontale, centrée sur
leur axe. Leurs deux vues de repos sont visibles côte à côte dans l'aperçu.
La nouvelle pose sert aussi au départ et à l'arrivée. La hauteur native
et l'ancrage au sol reprennent ceux de leur dessin actuel.

Le repos frontal de la Mule provient de la case en bas à gauche de
`D57-directions.png`. Le Traqueur utilise la même case de
`D58-reference-orientations.png`, où ses quatre pattes sont en appui.
Ces planches sont conservées dans `assets/source-art/animation-polish-01/drone-motion-01/`.
L'Éclaireur, l'Artilleur et le Gardien disposent déjà d'un repos frontal.

Le joueur valide l'aperçu r02 par « je valide :) ». Le rendu normal utilise
les cinq atlas natifs dans `assets/animations/drone-motion-01/`. Ils
reprennent les poses et les phases de l'aperçu. Une case dure 400 ms avec
le réglage par défaut. Le portrait reprend la pose de repos de l'atlas.

Le diagnostic `--ui-cold-agent-drone-playable DOSSIER --motion` contrôle
un déplacement réel de chaque drone dans chacune des quatre directions.
Il vérifie le retour au repos, le mouvement réduit et l'utilisation de
l'atlas préparé avant le dessin.
