# Une référence de bordure pour les quatre orientations

Revue du 1 octobre 2026. L'utilisateur préfère la bordure du mur orienté vers le haut et demande que les autres orientations la reprennent.

Le mur supérieur conserve ses pixels. Les murs verticaux reprennent ses deux bordures par rotation exacte de 90°, en conservant leur panneau central. Les faces opposées utilisent les rotations existantes de 180°. Les bandes de raccord des angles suivent ces profils communs. Les images sources, les orientations enregistrées et les règles de placement restent inchangées.

Avant correction, les bordures des murs verticaux étaient encore préparées depuis le bras vertical du coin. Un test comparant les couleurs des bordures après remise des quatre faces dans le même sens échouait sur la face droite. Il passe après la correction, avec égalité de tous les pixels RGBA des deux bordures sur les quatre orientations.

## Captures du rendu natif

- [Quatre orientations des murs, puis des coins](captures/wall-rotations.png).
- [Construction en gros plan à zoom ×2](captures/wall-borders-closeup.png).
- [Construction à 64 pixels par case](captures/four-corners-manual.png).
- [Raccords automatiques](captures/wall-corners.png).

## Validation et essai

- `cargo test --locked --example map_editor --example textured_surface_preview` : 24 tests de l'éditeur et 10 tests de l'essai de surface passent.
- `cargo check --locked --examples`, `cargo fmt --check`, `git diff --check` : succès.
- Construction d'un exécutable de revue et de l'essai de surface : succès.
- Revue des captures natives des quatre orientations et des bâtiments : réalisée.

L'exécutable [map_editor.exe](map_editor.exe) de ce dossier permet de tester cette version. La fenêtre utilisant encore `target/debug/examples/map_editor.exe` garde son ancien rendu tant que cet exécutable n'est pas remplacé après sa fermeture. Le PNG source ne change pas ; la correction concerne sa préparation à l'affichage dans les exemples, pas le client de campagne.
