# Catalogue SF et redimensionnement — 1 octobre 2026

Le catalogue urbain validé est disponible dans l'éditeur normal. Ce lot ajoute 48 objets SF et deux familles de murs, Laser et Caméra, avec chacune deux sources droites et quatre coins distincts. Le bouton Taille et Ctrl+R modifient les dimensions de la carte actuelle sans la recréer.

Validation du lot SF reçue le 1 octobre 2026 : intégré au catalogue normal. [Editeur_de_cartes.cmd](C:/Users/User/Desktop/project-RL/Editeur_de_cartes.cmd) ouvre directement cet éditeur. La composition SF utilise maintenant `--sf-demo` et le titre « ÉDITEUR — SALLES SF » ; l'ancien argument `--sf-review` reste compatible.

## Essayer

Double-cliquer sur [Essayer_salles_SF.cmd](C:/Users/User/Desktop/project-RL/Essayer_salles_SF.cmd). Il ouvre une installation composée avec serveurs, armurerie, chambres et laboratoire. Les mêmes images sont accessibles en ouvrant normalement l'éditeur ; les filtres Ville, Tech, Serveurs, Habitat et Armurerie permettent de parcourir le catalogue.

Les objets conservent rotation, déplacement, choix obstacle/décor, sauvegarde et annulation. Les escaliers et la trappe sont traversables par défaut dans l'essai.

## Images et prompts

Les cinq planches ont été produites avec l'outil intégré `image_gen`, en transparence. Les originaux restent conservés dans `C:/Users/User/.codex/generated_images/01a0f668-4257-7860-86fc-11b4c2f28340`. Les copies consommées par le projet sont ci-dessous ; les prompts complets sont conservés sans modification.

| Ensemble | Source enregistrée | Prompt complet |
|---|---|---|
| Serveurs et équipements techniques, 16 objets | [servers-source-v1.png](C:/Users/User/Desktop/project-RL/assets/prototypes/surface64/sf/servers-source-v1.png) | [serveurs.txt](C:/Users/User/Desktop/project-RL/artifacts/sf-redimensionnement-2026-10-01/prompts/serveurs.txt) |
| Habitat, quatre lits, quatre barils et accès souterrains, 16 objets | [habitat-source-v1.png](C:/Users/User/Desktop/project-RL/assets/prototypes/surface64/sf/habitat-source-v1.png) | [habitat.txt](C:/Users/User/Desktop/project-RL/artifacts/sf-redimensionnement-2026-10-01/prompts/habitat.txt) |
| Armurerie, établi, protection, énergie et drone, 16 objets | [armory-source-v1.png](C:/Users/User/Desktop/project-RL/assets/prototypes/surface64/sf/armory-source-v1.png) | [armurerie.txt](C:/Users/User/Desktop/project-RL/artifacts/sf-redimensionnement-2026-10-01/prompts/armurerie.txt) |
| Murs laser, 6 pièces | [walls-laser-source-v1.png](C:/Users/User/Desktop/project-RL/assets/prototypes/surface64/sf/walls-laser-source-v1.png) | [murs-laser.txt](C:/Users/User/Desktop/project-RL/artifacts/sf-redimensionnement-2026-10-01/prompts/murs-laser.txt) |
| Murs avec caméra, 6 pièces | [walls-camera-source-v1.png](C:/Users/User/Desktop/project-RL/assets/prototypes/surface64/sf/walls-camera-source-v1.png) | [murs-camera.txt](C:/Users/User/Desktop/project-RL/artifacts/sf-redimensionnement-2026-10-01/prompts/murs-camera.txt) |

La préparation du rendu découpe les planches à 64 × 64, conserve la transparence, retire de petits fragments isolés des nouveaux objets et adapte les raccords des murs à la géométrie approuvée. Les PNG sources et le lot urbain précédent sont conservés.

## Taille des cartes

Saisir une largeur et une hauteur de 1 à 256 cases, puis choisir l'un des neuf ancrages. Haut gauche conserve les coordonnées et ajoute ou retire l'espace à droite et en bas. Avec un ancrage central et une différence impaire, la case supplémentaire ou retirée est du côté droit/bas.

L'agrandissement conserve sols, peinture libre, structures, objets, orientations et propriétés. Les nouvelles cases ont un sol de terre. La réduction coupe le contenu hors du nouveau cadre ; les nombres de structures et d'objets retirés sont indiqués avant validation. Un nouvel agrandissement ne restaure pas ce contenu coupé. Ctrl+Z restaure toute la carte précédente, y compris ses dimensions, sa peinture et son départ.

Si le départ est coupé, il est replacé sur la case praticable la plus proche du bord conservé. La réduction est refusée si aucune case praticable ne reste. Les limites de peinture sont sauvegardées afin de conserver les courbes, le grain et l'échantillonnage des textures après déplacement. Les anciens fichiers JSON restent lisibles.

## Vérification

- `cargo test --locked --example map_editor --example textured_surface_preview` : 42 tests de l'éditeur et 20 tests de l'aperçu passent, aucun échec. Ils couvrent notamment les neuf ancrages, la peinture conservée pixel par pixel, les suppressions définitives après réduction, l'annulation, les données invalides, le catalogue SF, la rotation et les raccords des murs.
- `cargo build --locked --example map_editor --example textured_surface_preview` : réussi.
- `cargo check --locked --examples`, `cargo fmt --all -- --check` et `git diff --check` : réussis. Git signale uniquement des conversions LF/CRLF déjà présentes dans le chantier partagé.
- Capture native exécutée avec `map_editor.exe --capture-sf artifacts/sf-redimensionnement-2026-10-01/captures-finales` : comparaison d'une carte 16 × 10 avant/après agrandissement en 22 × 14, **655 360 pixels RGBA conservés exactement** dans la région existante.
- Sauvegarde et rechargement de la carte agrandie, réduction à 8 × 7, puis deux annulations : documents identiques aux états antérieurs.
- Captures examinées en 1360 × 840 et 960 × 540, y compris le dialogue de taille, les dimensions invalides et les nouvelles palettes. Les catalogues sont capturés à la résolution native de 64 pixels.

Les preuves finales sont dans [captures-finales](C:/Users/User/Desktop/project-RL/artifacts/sf-redimensionnement-2026-10-01/captures-finales). Les captures initiales et intermédiaires restent conservées pour comparaison.

![Installation SF à 64 pixels](C:/Users/User/Desktop/project-RL/artifacts/sf-redimensionnement-2026-10-01/captures-finales/salles-sf-native-64.png)

## Limites actuelles

Ce lot enrichit l'éditeur et son essai jouable. Les escaliers et la trappe représentent des accès souterrains, mais ne sont pas encore reliés aux étages du jeu. Les lasers bloquent comme des murs, sans dommage ni animation ; les caméras n'ont pas d'alarme. L'armurerie, les machines et les meubles n'ajoutent pas encore de services de jeu. La campagne n'importe pas encore ces cartes et ces objets.

Les sauvegardes de code avant modification se trouvent dans `avant`. Les ajouts concernent les exemples de l'éditeur/aperçu, leurs images, le lanceur SF et la documentation ; aucun commit ni push n'a été effectué.
