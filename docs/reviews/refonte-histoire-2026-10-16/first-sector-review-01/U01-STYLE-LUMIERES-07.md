# SCENE-U01-07 / LUM-U01-07 — cohérence graphique et éclairage

18 octobre 2026. **Rendu et lumières validés, activés en génération 155.**

Le joueur répond « Valider le rendu et les lumières ». Il demande ensuite
de noter exactement le style et l’angle, et précise que les prochaines cartes
seront plus grandes. L’accord est conservé dans `visual-acceptance-r07.json` ;
la référence durable est [STYLE_VISUEL_REFERENCE.md](../../../STYLE_VISUEL_REFERENCE.md).

Le joueur demande un avis sincère sur l’écart entre le détail du décor et
celui des personnages, monstres et armes. La réponse identifie une différence
de traitement : microtextures réalistes dans le décor, masses plus simples
et contours plus marqués dans les sprites. Il choisit ensuite explicitement
« Simplifier les textures du décor, renforcer légèrement les contours ».
Il demande également que les lumières soient calculées par le moteur.
Il précise ensuite une ambiance « un peu de nuit », avec quelques sources
près des bâtiments, comme dans la première carte. La proposition conserve
une pénombre froide lisible et des points lumineux chauds localisés.

La perspective de SCENE-U01-06 reste acceptée, avec le camp comme référence
stricte. La simplification porte sur le dessin des surfaces, pas sur le plan,
les proportions, les ouvertures ou les règles de déplacement. Les quêtes,
ennemis, équipements et dessins des acteurs restent conservés.

## Dessin proposé

SCENE-U01-07-r01 réduit le grain, les rayures et les éclats minuscules.
Les surfaces sont regroupées en aplats ombrés et les contours renforcés.
La source possède un éclairage diffus neutre ; les luminaires gardent leur
position mais les nappes lumineuses peintes ne servent plus d’éclairage.

Fabrication avec imagegen intégré, source conservée sans retouche dans
`assets/source-art/first-sector-review-01/u01-style-r07/SCENE-U01-07-r01.png`.
Prompt exact : `u01-style-prompt-r07.txt`.

## Éclairage proposé

LUM-U01-07-r01 réutilise les shaders d’éclairage du camp. Quatorze sources
calculent un champ d’irradiance avec atténuation et occlusion par les murs
logiques. L’image complète du monde, y compris les acteurs, reçoit ensuite
ce champ dans le composite GPU. Le HUD reste hors du rendu éclairé.
Les ombres de murs sont calculées sur la carte statique ; ce système ne
calcule pas des ombres mobiles projetées par les corps.

Les lentilles des luminaires sont identifiées par un masque numérique.
Leur couleur lumineuse est émise dans le shader à l’allumage ; à l’extinction,
elles redeviennent sombres. Le nouveau mode d’émission est optionnel et
désactivé par défaut dans la configuration du camp pour conserver son rendu.
Les boîtiers et les supports restent des objets dessinés, sans halo peint.

Les textures de données d’éclairage sont numériques et reproductibles.
Elles ne modifient aucun pixel du dessin source. Elles sont chargées une fois,
avec une carte douce séparée des textures natives nettes ; aucun lancer de
rayons ni chargement de texture n’est effectué à chaque image du jeu.

Fabrication : `tools/reviews/build_u01_lighting_review.py`.
Contrôles : `u01-lighting-data-r07.json`, dont un récepteur éclairé devant
la porte et un récepteur bloqué derrière le mur adjacent.

La commande `--ui-cold-prison-street-lighting` présente la variante dans un
diagnostic isolé et produit des vues allumées et éteintes à pose identique.
Ce mode de revue ne réécrit ni sauvegarde utilisateur ni règle de combat.
Cette présentation reçoit ensuite la validation explicite. Les sources et
données exactes sont copiées dans `assets/maps/prison-street-01/` pour le
lancement normal. Le diagnostic conserve son choix allumé/éteint isolé.

## Vérification de l’activation

`integration-checks-r07.json` confirme le rendu normal avec lumières actives,
les sources identiques à celles acceptées et le retour au camp. Les huit
tests ciblés réussissent. Les cinq vues natives normales gardent le même
éclairage statique que la revue ; la respiration des sprites reste animée.
Six captures du camp passent également les contrôles de cadrage, de visibilité
et de rendu sans mutation après la modification du shader partagé.
Compilation debug, contrôle release, formatage et vérification du diff passent.
La suite complète n’est pas déclarée validée par ces contrôles ciblés.
