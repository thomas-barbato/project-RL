# Grande carte de test — 2 octobre 2026

Carte locale, fixe, entièrement modifiable dans l'éditeur : **160 × 112 cases**, soit 17 920 cases. Ouvrir `Essayer_grande_carte.cmd` à la racine du projet. La vue commence près du départ dans la ville ; molette pour zoomer, clic milieu ou flèches pour parcourir, miniature pour rejoindre un lieu.

Fichier : `artifacts/map-editor/maps/grande-ville-et-nature-2026-10-02.json`.

## Composition

- Ville : 17 bâtiments meublés, dont épicerie, clinique, centre de données, armurerie, atelier, restaurant, auberge, huit logements et deux entrepôts ; marché et mobilier de rue.
- Hameau : quatre maisons, un atelier, une réserve, place de terre et petit marché.
- Nature : bois, prairies sèches, buissons, arbres morts, rochers, rivière, deux ponts, lac, roseaux et chemins courbes.
- Deux accès distincts : grotte envahie au nord-est (139,21), ancienne carrière au sud-ouest (29,96).
- Départ en ville (41,32), deux zones candidates d'ennemis et une zone de sortie au sud-est. Les références `test/...` sont des métadonnées, sans destination de campagne implémentée.

La carte comporte 934 structures, 1 518 objets et 104 traits de peinture. `composition.json` décrit les bâtiments et les repères. Les fonctions des commerces et des installations sont visuelles ; ni les habitants, ni les rencontres, ni les transitions ou intérieurs de donjon ne sont raccordés à la campagne.

## Validation

Le fichier passe par le chargeur de l'éditeur et `Scene::from_document`, qui construit la carte et les collisions du jeu. La conversion JSON aller-retour conserve exactement le document. Le contrôle de tous les sols accessibles, portes et marqueurs donne **15 307 cases accessibles et zéro problème** ; les portes sont initialement ouvertes. Le contrôle utilise les déplacements cardinaux, donc les accès ne dépendent pas d'une diagonale entre deux obstacles.

`cargo test --locked --example map_editor` : **52 tests réussis**, notamment chargement, sauvegarde, murs, rotations, collisions, pinceau, eau, empreintes et catalogue. `cargo fmt --all -- --check`, `cargo check --locked --examples` et `git diff --check` réussissent. La suite complète du jeu n'a pas été exécutée pour cette carte.

Les dix PNG dans `apercus/` proviennent du rendu natif : vue complète, ville, centre-ville, hameau, grotte, carrière, rivière, lac, éditeur et catalogue Nature. Ils ne sont pas des maquettes générées. Les deux premiers brouillons avaient des passages bloqués par du mobilier et une mauvaise rive ; ils sont archivés dans `avant/`, et les anciennes captures `verification-*` ne représentent pas le résultat final.

La carte a été composée avec les données de l'éditeur par `tools/build_large_editor_map.py`, puis chargée et contrôlée dans l'éditeur natif ; elle n'a pas été dessinée case par case à la souris. La graine 20261002 stabilise la composition du fichier et ne régénère pas le décor pendant une partie. Le script refuse d'écraser un fichier existant.

Reproduire les captures, avec un nouveau dossier de destination :

```powershell
./target/debug/examples/map_editor.exe --map artifacts/map-editor/maps/grande-ville-et-nature-2026-10-02.json --capture-landscape DOSSIER_NEUF
```

## Nouveau lot Nature

Seize objets ont été ajoutés à Objets > Nature, aux indices stables 104 à 119. Les anciens indices sont conservés. Source : `assets/prototypes/surface64/nature/nature-source-v1.png`, atlas 4 × 4 avec transparence réelle, produit avec l'outil imagegen intégré. Le moteur extrait les cellules en 64 × 64, avec un bord transparent comme les autres objets. Le dessin et la collision sont séparés ; les arbres agrandis dans cette carte conservent une empreinte d'une case. Les deux grottes utilisent trois cases par côté et restent traversables.

La source générée reste également conservée dans `C:/Users/User/.codex/generated_images/01a0f668-4257-7860-86fc-11b4c2f28340/exec-430f1753-b4ea-4548-82a0-66bec1ae3333.png`. Aucun asset antérieur n'a été remplacé. Le prompt exact est dans `prompt-nature.txt`. Ce lot et cette carte sont des éléments de test à examiner, sans validation artistique pour la campagne.
