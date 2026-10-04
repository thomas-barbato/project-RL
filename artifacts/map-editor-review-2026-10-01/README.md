# Premier essai de l éditeur de cartes

L'outil natif est disponible dans `examples/map_editor.rs`. Le cahier des charges et les fonctions restant à réaliser sont dans `docs/EDITEUR_CARTES.md`.

Depuis la racine du projet :

```powershell
cargo run --locked --example map_editor
```

Les images finales de cette passe sont dans `revision-2` :

- `editor-1360.png` : palette et composition ; un lit a été placé puis tourné de 90° sur le sol.
- `editor-native64.png` : même scène affichée à 64 pixels par case.
- `new-map.png` : saisie des dimensions.
- `custom-map.png` et `custom-37x19.json` : carte 37 × 19 sauvegardée puis rechargée, avec un objet tourné de 270°.
- `editor-960.png` : interface en 960 × 540, avec défilement de la palette et caméra.
- `play-test.png` : déplacements et ouverture d'une porte avec le moteur.

Les captures sont des rendus réels du binaire Macroquad. Le scénario de capture appelle les fonctions de composition et de sauvegarde ; il ne constitue pas un test automatique de chaque clic et raccourci de l'interface.

## Validation technique

- `cargo test --locked --example map_editor` : 15 tests réussis.
- `cargo test --locked --example textured_surface_preview` : 7 tests réussis ; plusieurs tests de modules sont communs aux deux exécutables.
- `cargo build --locked --example map_editor` : compilation native réussie.
- `cargo check --locked --all-targets` : réussi.
- Formatage des exemples et contrôle des différences : réussis.
- Captures natives inspectées en 1360 × 840 et en 960 × 540.

Les tests portent notamment sur les couches sol/objet, les tailles jusqu'à 256 × 256, les rotations, les collisions, les raccords des 16 configurations de murs, les portes, l'historique, la sauvegarde/relecture et la correspondance entre cases rendues et coordonnées de placement.

## Limites de ce premier essai

Le catalogue contient 10 sols, un outil de murs, deux états de porte, le départ du joueur et 10 objets. Les objets occupent une case et tournent par rotation de l'image. Le choix obstacle emploie un mur comme représentation de collision dans la scène de test, avec blocage de la vision.

Les images restent expérimentales. Les objets sur plusieurs cases, les orientations dessinées, les outils de composition plus riches, les marqueurs d'ennemis/donjons/passages et l'import en campagne restent à réaliser.

Cette passe ajoute des exemples, leurs images et leur documentation. Elle ne modifie pas le rendu de la partie normale, la génération de campagne, ses sauvegardes ou le contenu des autres changements déjà présents dans le checkout.
