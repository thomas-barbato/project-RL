# Raccords de murs et limites des sols

Correction du premier éditeur natif à la suite de la revue visuelle du 1er octobre 2026.

- Les marges transparentes des murs et portes montrent désormais le terrain situé de leur côté. Un sol intérieur peint sur la case d'un mur ne dépasse plus à l'extérieur.
- Les sols enregistrés restent inchangés : retirer un mur retrouve le sol peint sous celui-ci. Le seuil des portes conserve ce sol.
- La préparation de la pièce en L conserve son dessin entier. Les deux bras ne sont plus copiés dans des rectangles qui se chevauchent et écrasent le raccord.
- Les PNG sources, la campagne et les fichiers de cartes existants restent inchangés.

## Validation

`cargo test --locked --example map_editor` : 17 tests passent, dont les nouveaux contrôles de terrain aux quatre côtés, aux quatre coins, au bord de carte et contre une structure diagonale. Les contrôles des 16 configurations de murs, des rotations et des portes restent valides.

Compilation de `map_editor` et vérification de tous les exemples réussies. Captures produites par l'exécutable natif avec `--capture artifacts/corrections-murs-2026-10-01/captures`.

- `captures/wall-corners.png` : deux bâtiments à 64 pixels par case ; les sols sont volontairement peints jusque sous tous les murs pour vérifier leurs limites.
- `captures/wall-corners-open-doors.png` : même composition, portes ouvertes.
- `captures/editor-native64.png` : démonstration meublée corrigée.
- Les autres captures couvrent les sols, la création de carte, la sauvegarde/relecture, l'affichage 960 × 540 et l'essai jouable.

La correction porte sur la préparation et le rendu des images existantes. Les raccords en T et en croix utilisent toujours les assemblages du premier prototype et demandent une revue artistique distincte.
