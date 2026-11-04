# MONSTER-VERTICAL-COLOR01-r01 — rouge du démon ailé

Le joueur valide le deuxième lot, avec une réserve sur 03 : « démon ailé change
de couleur quand il marche (il devient rouge plus clair) ». Les dessins de
MON-005/156/165 sont figés dans `MONSTER-VERTICAL02-ACCEPTANCE-r01.json`.
La correction de MON-144 porte sur sa couleur mobile et conserve sa démarche.

La source mobile était plus claire et plus rosée que le frontal. Les captures
avaient pourtant les mêmes paramètres d’éclairage du moteur, gain 1,48. Le
problème venait de la palette peinte des images mobiles, pas d’une nouvelle
lumière. La médiane des pixels du matériau rouge était RGB [141,50,46], contre
[118,30,25] dans le frontal conservé ; après préparation elle devient
[118,30,26]. Les tons sombres et clairs sont rapprochés de la même façon.

La correction utilise une palette déclarative appliquée une fois à l’image
préparée avant création de la texture. Les seize poses mobiles emploient la même
règle. Le moteur conserve tous les pixels alpha, les positions, les contours,
les phases, la taille et la cadence de 400 ms par pas. Les deux cellules de
repos et les matériaux hors rouge restent identiques. Aucun PNG source n’est
réécrit ou redessiné, et aucune correction n’est appliquée au décor, au joueur
ou aux autres monstres. Les ressources lourdes restent hors du dessin.

La règle et le manifeste de revue sont dans
`assets/source-art/animation-polish-01/monster-vertical-color-01/r01/` :
`red-material-palette.json` et `manifest.json`. Les PNG pointés restent ceux du
premier lot figé. Ce travail emploie le système de palette préparée dans le
code ; aucune nouvelle génération d’image n’est nécessaire.

Le retour « parfait on peut continuer » valide la correction de couleur. Les
références exactes sont figées dans `MONSTER-VERTICAL-COLOR01-ACCEPTANCE-r01.json`.
La cadence, les seize poses et le frontal restent ceux montrés. Cet accord
visuel garde son périmètre distinct de l’intégration dans la partie normale.

Le comparatif natif montre le frontal conservé, la marche précédente pâle et
la même marche harmonisée. Les cinq autres modèles restent sélectionnables
avec leurs captures précédentes. Cette correction n’active pas encore ces
nouvelles marches dans le rendu normal.

Un test sur l’atlas réel vérifie l’identité complète des alpha, du repos et
des matériaux protégés, puis compare les quantiles sombres/médians/clairs au
frontal (écart maximal de deux unités RGB). Les sources et les trois modèles
acceptés du deuxième lot sont vérifiés par empreinte. Les captures passent par
`--ui-cold-monster-vertical-color01`, sans chargement de partie sauvegardée.

Le contrôle natif compte 66 captures et deux retours au frontal exacts. Les
douze ressources du deuxième lot accepté, les douze ressources du premier lot
et les dix atlas latéraux intégrés gardent leurs empreintes. Le navigateur
charge les six modèles, lit les deux directions à 400 ms par pas et permet
de comparer chaque phase au repos, sans erreur de console. Preuve et relevé :
`artifacts/animation-polish-01/monster-vertical-color-01/r01/browser-proof.png`
et `browser-checks.json` dans le même dossier.

```powershell
python tools/art/prepare_monster_vertical_color01.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo run --locked --bin project-rl -- --ui-cold-monster-vertical-color01 artifacts/animation-polish-01/monster-vertical-color-01/r01/current
python tools/art/build_monster_vertical_color01_review.py
python tools/art/check_monster_vertical_color01.py
```
