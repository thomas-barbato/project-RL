# Essai du pinceau rond — 1 octobre 2026

La carte conserve sa grille de simulation. Une couche de peinture au pixel natif permet de tracer des routes courbes et des zones irrégulières indépendamment des limites des cases.

## Essayer

L'exécutable habituel `target/debug/examples/map_editor.exe` est recompilé. Dans Sols, sélectionner Rond puis une texture sèche. Clic gauche pour peindre, clic droit pour gommer la peinture, boutons −/+ ou Maj + molette pour changer la largeur. Ctrl+Z annule un geste complet ; Ctrl+Y le rétablit.

Pour ouvrir directement la composition montrée :

```powershell
& .\target\debug\examples\map_editor.exe --map artifacts/pinceau-rond-2026-10-01/captures-validees/carte-pinceau-rond.json
```

## Comportement

- Diamètre de 16 à 384 pixels natifs, indépendant du zoom. Une case correspond à 64 pixels natifs.
- Traits continus entre positions successives du pointeur, sans trous dus à un déplacement rapide. Quitter la carte interrompt le raccord du trait.
- Transitions courtes et grain stable sur les bords ; les matériaux successifs se composent.
- Gomme limitée à la couche peinte ; sols de base, objets et structures restent présents.
- Sauvegarde des matériaux, diamètres et points dans le champ facultatif `paint`. Les anciennes cartes sans ce champ se chargent encore.
- Peinture affichée dans l'éditeur et le test jouable, y compris le traitement des marges des murs.
- Terrains de simulation, collisions, déplacements, portes et visibilité conservés. L'eau utilise le placement par cases et masque la peinture libre.
- Cache GPU des seules tuiles visibles, invalidation des nouveaux segments du trait, mémoire bornée à 1024 entrées.

Les PNG sources et les murs précédemment approuvés ne sont pas retouchés. Cet essai utilise les textures existantes ; il n'ajoute pas de nouveaux décors. La répétition de l'herbe et du gravier reste visible, surtout au grossissement ×2. Le résultat valide le pinceau, pas encore le niveau artistique final de la carte.

## Validation

- `cargo test --locked --example map_editor --example textured_surface_preview` : 30 tests de l'éditeur et 14 tests de l'aperçu réussis.
- Vérifications de continuité entre cases, superposition et gomme, invalidation du cache, ancien format, rejet des données invalides, annulation/rétablissement et export avec terrains de jeu inchangés.
- `cargo check --locked --examples`, `cargo build --locked --example map_editor`, `cargo fmt --all -- --check` et `git diff --check` réussis.
- Exécutable natif lancé avec `--capture-brush` : six captures valides, sortie normale. Sauvegarde/rechargement puis annulation de la gomme contrôlés par égalité exacte du document dans ce parcours.
- `captures-validees/chemin-courbe.png` : composition à 1360 × 840.
- `captures-validees/chemin-courbe-grille.png` : même composition avec la grille.
- `captures-validees/gomme-ronde.png` : gomme sur le chemin.
- `captures-validees/chemin-detail.png` : grossissement à 128 pixels par case.
- `captures-validees/pinceau-960.png` et `captures-validees/chemin-test.png` : palette et mode jouable à 960 × 540.

Ces captures et tests vérifient le parcours de l'application et ses données ; une manipulation manuelle du pinceau avec la souris n'a pas été automatisée. L'import dans la campagne et la diversité des images restent des étapes distinctes.

`avant/` conserve les sources de l'éditeur avant cet essai. `captures/` et `captures-final/` conservent les premiers passages de capture ; `captures-validees/` correspond à la version finale recompilée.
