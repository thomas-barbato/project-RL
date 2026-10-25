# WALK07-P01 — petits monstres centrés verticalement

**Accord visuel : « parfait on continue :) », le24 octobre2026.**
Les valeurs des13 cycles présentés et la règle exacte restent figées dans
`visual-acceptance-placement-01.json` et
`assets/source-art/player-walk-review-07/accepted/vertical-placement-01/rule-and-cases.json`.
Les nouveaux lots conservent cette règle et leur propre revue artistique.

Le24 octobre2026, après validation des six créatures du lot02, le joueur
demande que les monstres moins hauts que leur case soient centrés verticalement.
Les13 cycles des lots01 et02 restent acceptés et leurs182 poses exactes restent
figées. M12 demeure différé. Aucun nouveau dessin n'est généré pour ce réglage.

## Règle demandée

- La case logique reste64×64px. Mesurer la silhouette visible entière à
  l'échelle native, avec ses appendices, sans inclure une ombre séparée.
- Si la hauteur de l'enveloppe de toutes les poses est strictement inférieure
  à64px, centrer cette enveloppe verticalement sur la case. Un arrondi au pixel
  natif garde au plus0,5px d'écart pour une hauteur impaire.
- Garder un seul décalage vertical pour les14 poses et les deux orientations.
  Ne pas recentrer chaque image indépendamment : la flexion et la levée des
  pattes ne doivent pas modifier l'ancrage de toute la créature.
- Déplacer ensemble la figure entière et son ombre. Ne retoucher aucun membre,
  aucun pixel, aucune taille ni aucun dessin source accepté.
- Pour une silhouette haute de64px ou davantage, conserver l'ancrage adapté
  à sa taille. Le cadrage horizontal existant est conservé.
- Ne modifier ni occupation logique, collision, perception, tours, trajet ou
  déplacement vertical de jeu. Les400ms réglables et les vrais repos restent
  ceux des cycles acceptés.

Pour l'intégration, le décalage devra accompagner le rendu complet et son
glissement, en utilisant les métadonnées des ressources embarquées. Le jeu
ne doit pas dépendre des aperçus ou des fichiers sous `artifacts/`.

## Comparaison préparée

L'aperçu courant réutilise le lot02 accepté. « Avant » montre son ancrage
précédent en bas de case. « Après » dessine exactement le même cycle, avec
le seul décalage vertical demandé et l'ombre déplacée ensemble. Le détail×2
reprend les pixels de la scène native, avec la même grille agrandie.
Il ne compare donc pas deux dessins différents. Les deux créatures dépassant
64px dans ce lot gardent leur placement ; les quatre plus petites remontent.

Le réglage est calculé pour les13 créatures acceptées : huit silhouettes
moins hautes que64px sont centrées, dont une déjà à0,5px du centre ; cinq
grandes conservent leur ancrage. Les valeurs exactes figurent dans
`vertical-placement-01.json`. Les images et calques sources restent inchangés.

## Contrôles et statut

`verify_walk07_creatures.py` vérifie les182 poses figées, les références natives,
le centrage à0,5px près et l'absence de correction sur les grandes créatures.
Le lecteur simulé sous Node vérifie que les deux panneaux utilisent le même
dessin, que le corps et l'ombre partagent le décalage et que les six phases,
les deux directions, les commandes et la cadence restent conservées.
Les2938 sélections et300 fixtures d'équipement passent ; les601 instants
de comparaison restent alignés sur WALK05-STEP03.

Preuves : `creature-validation-lot01.json`, `creature-validation-lot02.json`,
`renderer-checks.json` et `vertical-placement-01.json`. Planche native de
comparaison : `artifacts/player-walk-review-07/vertical-placement-01-native.png`.
Ce travail prépare et montre la règle demandée. Aucune intégration au rendu
actif ni exécution réelle de ces candidats en navigateur ou en jeu n'est
revendiquée ; les contrôles JavaScript utilisent un DOM/canvas simulé.
