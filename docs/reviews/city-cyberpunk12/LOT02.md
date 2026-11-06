# CITY-CYBERPUNK12 — lot 02 et correction ciblée des accès

Retouche courante : **SYMBOLES-r03**, retrait des pictogrammes à trois barres
du logement, avec conservation exacte des autres pixels. Voir `SYMBOLES-r03.md`.

Le 5 novembre 2026, « oui ok on continue :) » pré-valide le premier lot
et poursuit la première ville. Son acceptation et ses sources sont figées
dans `LOT01-ACCEPTANCE-r01.json`. La nouvelle ville vient ensuite et les
deux tracés sont conservés. Les égouts restent exclus.

## Trois bâtiments, deux intérieurs

**V1-I01 · Logement** : cuisine, rangements, table, lit et chevet conservés
dans les mêmes zones. Les appareils et meubles sont repris dans une direction
domestique futuriste, sale et usée. Sol bleu-gris en résine, contrôles d'appareils
intégrés, connexions et réparations. Les murs, coins et seuil restent ceux du
vrai bâtiment ; le raccord de sol s'arrête sur la maçonnerie d'origine.

**V1-I02 · Boutique** : rayonnages, vitrines et caisse électronique autour
de l'allée centrale. Les marchandises restent reconnaissables. Sol plus chaud,
distinct de celui du logement. L'entrée et l'allée restent à leurs positions.

**V1-B04 · Entrepôt en L** : refroidissement, conduites et alimentation sur
les parties disponibles du toit, sans recouvrir la brèche ni agrandir le L.
Un seul lecteur ajouté subsiste à l'accès de cet entrepôt ; sa commande n'est
pas exposée sur le toit. Ce décor n'ajoute aucune nouvelle mécanique d'accès.

Les toits des deux intérieurs reçoivent de la ventilation/refroidissement,
sans boîtier de contrôle d'accès exposé. Leurs fissures, trémies, lucarne,
dimensions et positions sont conservées. Les vues ouvertes et fermées sont
des états d'inspection du dessin, pas une modification des règles de portes.

## Précision reçue pendant la préparation

Le joueur demande de ne pas mettre des scanners à tous les bâtiments et
de cacher la machine qui commande un accès digital/rétinien. La correction
est limitée à ces accessoires : voir `SECURITE-r02.md`. Les sources approuvées
du lot 01 restent intactes ; le garage et le bâtiment bleu reçoivent des
variantes ciblées, et les lecteurs des trois bâtiments ordinaires sont retirés.
Les nouveaux intérieurs, leurs matériaux, les routes et les autres décors sont conservés.

## Sources et retouches

Les trois études intégrées ont été dessinées avec **image_gen.imagegen intégré** :

- `assets/source-art/city-cyberpunk12/lot02/V1-I01-logement-source-r01.png` ;
- `assets/source-art/city-cyberpunk12/lot02/V1-I02-boutique-source-r01.png` ;
- `assets/source-art/city-cyberpunk12/lot02/V1-B04-entrepot-source-r01.png`.

Prompts exacts : `V1-I01-logement-PROMPT-r01.txt`,
`V1-I02-boutique-PROMPT-r01.txt` et `V1-B04-entrepot-PROMPT-r01.txt`.
Les sources, l'échelle uniforme, les emprises et les 23 pièces indépendantes
figurent dans `assets/source-art/city-cyberpunk12/lot02/manifest.json`.
`recipe.json` sépare le sol, les meubles, les équipements, les raccords et les
toitures. Une demande sur le lit ne régénère pas la cuisine ou la pièce entière.

Seul l'intérieur des cadres d'origine est composé à partir des nouvelles
études ; les maçonneries et ouvertures extérieures sont conservées. Les pixels
de sol du dessin généré sont raccordés jusqu'aux vrais murs à partir de sa
propre matière, sans dessiner de sol extérieur. Les équipements de l'entrepôt
sont sélectionnés séparément ; les autres pixels de l'étude sont écartés.

La génération est verrouillée après présentation. Les retouches suivantes
utilisent des fichiers voisins par pièce, avec conservation des autres empreintes.

## Inspection et limites

La page courante offre neuf vues, avant/après, dont deux intérieurs avec
le personnage natif 64 × 64. Le fond candidat cumule les éléments conservés
du lot 01, les corrections d'accès et le lot 02. Les anciens clichés du
premier lot restent intacts et accessibles comme historique.

`Apercu_premiere_ville.cmd` lance la revue native : Tab compare, 1–9 choisit
une vue, haut/bas déplace la caméra et Échap ferme. Les ressources sont
préparées avec le worker et le cache borné communs avant affichage. Les textures
restent créées sur le thread graphique. Aucun fichier actif de carte, collision,
visibilité, sauvegarde ou d'égouts n'est modifié.

**Lot 02 et variantes de sécurité à inspecter.** La première ville n'est
pas terminée. Les autres secteurs restent à reprendre avant la nouvelle
ville. L'éclairage dynamique, les portes, les occultations et la circulation
devront être vérifiés en jeu avant validation définitive.
