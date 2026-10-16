# Correction du pinceau : eau et marques près des murs

## Causes et correction

Les textures Eau peu profonde et Eau profonde étaient exclues du mode rond : sélection du pinceau limitée aux indices 0 à 7, validation des fichiers limitée à 8 matériaux, cache des textures limité à ces mêmes 8 matériaux, et peinture masquée sur les cases d'eau. Ces restrictions sont retirées. Les dix matériaux de sol utilisent le même trait libre, la même gomme et le même format de sauvegarde.

Le rendu des murs copiait un morceau de la peinture d'une case voisine dans ses marges. Un petit trait placé en bas d'une case intérieure pouvait ainsi apparaître aussi sous le mur du haut, à environ une case de distance. Le sol de base conserve son traitement aux marges des structures, mais la peinture est maintenant dessinée une seule fois à ses coordonnées dans le monde, après ces marges. Les données sauvegardées ne contenaient pas les marques parasites : les cartes existantes n'ont pas besoin d'être repeintes.

Pour le test jouable et le rechargement, le matériau visuellement dominant au centre de chaque case détermine le terrain de simulation. L'eau peu profonde reste traversable, l'eau profonde bloque le passage. La gomme révèle à nouveau le terrain de base. Murs, portes et objets bloquants gardent leur priorité. Un nouveau trait qui couvrirait le départ d'eau profonde est refusé sans perdre le trait déjà valide.

L'exécutable habituel `target/debug/examples/map_editor.exe` est recompilé. Les PNG des textures, des murs et des coins ne sont pas modifiés. La campagne n'est pas modifiée.

## Reproduction et validation

- Avant correction : `--capture-brush-fixes` a reproduit le défaut. Le trait intérieur modifiait 266 pixels, dont 103 hors du rayon du pinceau. Les images sont dans `reproduction/` ; l'assertion du diagnostic a échoué comme attendu.
- Après correction : six comparaisons natives entre captures avant/après, à 1360 × 840, sur le mur du haut vu depuis l'intérieur, le bord extérieur de la carte, les murs droit/gauche/bas et un coin. Zéro pixel modifié hors du rayon du pinceau dans les six cas.
- `cargo test --locked --example map_editor --example textured_surface_preview` : 33 tests de l'éditeur et 15 tests de l'aperçu réussis. Les nouveaux tests vérifient les deux profondeurs, le rendu, les terrains et le passage après export/rechargement, la gomme, l'annulation/rétablissement et la protection du départ.
- `cargo check --locked --examples`, `cargo build --locked --example map_editor`, `cargo fmt --all -- --check`, `git diff --check` réussis.
- La composition de l'eau est sauvegardée, chargée et comparée exactement à son document d'origine dans le diagnostic natif. Elle contient 13 cases peu profondes et 12 cases profondes pour le test jouable.
- Les captures finales comprennent les six comparaisons de localisation et l'eau dans l'éditeur et le mode test. La manipulation manuelle de la souris n'a pas été automatisée.

`avant/` conserve les sources avant cette correction. `raccords-corriges/` et `captures-validees/` conservent les passages intermédiaires ; `captures-finales/` correspond à la version finale recompilée.

Pour essayer la carte montrée :

```powershell
& .\target\debug\examples\map_editor.exe --map artifacts/pinceau-eau-murs-2026-10-01/captures-finales/eau-pinceau-rond.json
```
