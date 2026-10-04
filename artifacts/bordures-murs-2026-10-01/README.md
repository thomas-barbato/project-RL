# Bordures des murs alignées sur les angles

Correction locale du 1 octobre 2026. L'utilisateur approuve l'emboîtement du lot v4 et préfère la bordure de ses angles.

## Diagnostic

Les murs droits étaient recadrés indépendamment des coins. Le rognage du noyau opaque supprimait leur contour sombre avant l'étirement à la bande commune de 48 pixels. La planche source contient aussi des bandes claires différentes entre ses murs droits et ses coins.

Sur une coupe du rendu avant correction, le premier pixel de la bordure horizontale valait RGBA `[188, 188, 191, 255]`, contre `[29, 29, 31, 255]` sur la partie droite du coin. Le mur comportait également un second liseré clair à la ligne 18, absent de la coupe du coin. Les tests de continuité géométrique passaient, mais ne détectaient pas cette différence de dessin.

## Correction

La préparation reprend les profils de bordure extérieur et intérieur des bras droits du coin haut gauche pour les murs horizontaux et verticaux. Le contour sombre, le biseau gris et leur épaisseur suivent le coin de référence. Les panneaux centraux des murs sont conservés. Les autres coins restent des pièces entières, avec les bandes de raccord habituelles. Les sources PNG et les données des cartes ne sont pas modifiées.

Une encoche transparente ponctuelle sur la silhouette générée est prolongée avec la couleur opaque immédiatement voisine. Cela conserve la largeur des raccords sans ajouter de trou.

## Revue native

- [Construction complète à 64 pixels par case](captures-final/four-corners-manual.png).
- [Gros plan à zoom ×2](captures-final/wall-borders-closeup.png).
- [Raccords automatiques](captures-final/wall-corners.png).
- [Démo meublée](captures-final/editor-1360.png).

Les dossiers `captures` et `captures-v2` conservent les essais intermédiaires. La version revue se trouve dans `captures-final`.

## Validation

- `cargo test --locked --example map_editor --example textured_surface_preview` : 23 tests de l'éditeur et 9 tests de l'essai de surface passent.
- Construction de l'exécutable de revue : succès. Le fichier `map_editor.exe` de ce dossier permet de tester pendant que l'ancien éditeur reste ouvert.
- Captures natives de l'éditeur et revue des bordures : réalisées.

Le remplacement de `target/debug/examples/map_editor.exe` dépend de la fermeture de sa fenêtre : Windows verrouille l'exécutable en cours d'utilisation. Les images restent réservées à l'éditeur et à l'essai de surface ; l'apparence en campagne reste soumise à validation.
