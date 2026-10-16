# Ville de surface : lot urbain à valider

Essai local du 1 octobre 2026. Le lot peut être essayé dans l'éditeur ; sa présence ne constitue pas une adoption artistique dans la campagne.

## Essayer

Double-cliquer sur `C:\Users\User\Desktop\project-RL\Essayer_ville_surface.cmd`, ou lancer `cargo run --locked --example map_editor -- --urban-review` depuis le projet.

Ce mode ouvre une rue de 26 × 16 cases avec épicerie, logement, clinique, atelier et petit marché. Les éléments ajoutés se trouvent après les anciens dans les palettes Sols et Objets. La molette fait défiler les listes. Murs propose les familles Métal, Brique et Crépi. R tourne, B choisit obstacle ou décor, V sélectionne, P copie le choix sous le pointeur. Les sols urbains acceptent le pinceau rond. Le bouton Démo restaure cette rue et cette opération est annulable.

## Lot et sources

Images produites avec l'outil image_gen intégré. Les originaux restent dans le dossier generated_images de Codex. Les copies du projet sont dans `assets/prototypes/surface64/urban-review`. La préparation existante en Rust découpe les atlas, les ramène à 64 × 64 pixels, normalise les raccords et réserve une marge transparente de trois pixels aux nouveaux objets.

| Lot | Nombre | Source | Prompt exact |
|---|---:|---|---|
| Sols urbains | 12 | floors-source-v1.png | prompts/sols.txt |
| Mobilier domestique, commerces et soins | 16 | interiors-source-v1.png | prompts/mobilier.txt |
| Voirie, marché et stockage | 16 | street-source-v1.png | prompts/voirie.txt |
| Équipements contemporains et futuristes | 16 | technology-source-v1.png | prompts/contemporain-futur.txt |
| Murs en brique : deux droits, quatre coins | 6 | walls-brick-source-v1.png | prompts/murs-brique.txt |
| Murs en crépi : deux droits, quatre coins | 6 | walls-plaster-source-v1.png | prompts/murs-crepi.txt |

Les sources précédemment approuvées sont conservées. Les indices des anciens sols et objets restent identiques. Une famille de mur absente dans un ancien JSON conserve le métal. Les nouvelles cartes conservent sols, peinture libre, objets, rotations, familles des murs et collisions lors de l'enregistrement.

## Captures retenues

`captures-finales-v2` contient les 12 PNG retenus et la carte JSON de l'essai. Les autres dossiers de captures documentent les étapes intermédiaires.

- `ville-native-64.png` : carte complète, 1664 × 1024, chaque case rendue à 64 pixels par le moteur graphique réel.
- `sols-64.png` : chaque matériau répété sur quatre cases pour examiner les raccords.
- `mobilier-64.png`, `voirie-64.png`, `technologie-64.png` : objets à leur taille native.
- `editeur-vue-ensemble.png`, `boutique-murs-brique-64.png`, `logement-murs-crepi-64.png` : palette et scène dans la fenêtre native.
- `palette-mobilier.png`, `palette-voirie.png`, `palette-technologie.png`, `test-ville.png` : navigation dans le catalogue et essai jouable.

## Vérification

- `cargo test --locked --example map_editor --example textured_surface_preview` : 36 + 16 tests réussis.
- Après le dernier ajustement de marge des sprites, le test des deux atlas urbains a été relancé dans les deux exécutables : 2 réussites.
- `cargo build --locked --example map_editor --example textured_surface_preview`, `cargo check --locked --examples`, `cargo fmt --all -- --check` et `git diff --check` réussis.
- Vérification des 16 masques de raccord pour chaque famille de mur, des profils communs entre murs et coins, de la transparence des objets et de l'opacité des sols.
- Vérification du catalogue, du pinceau rond urbain, de la rotation et de la copie des nouveaux murs, de l'historique, de la lecture JSON et du rejet des indices invalides.
- La rue se sauvegarde et se recharge, et les quatre bâtiments sont accessibles à pied depuis le départ.
- Captures natives examinées : murs et coins, taille réelle des assets, palette, débordement entre la table de soins et l'armoire médicale corrigé, libellés de catalogue lisibles.

## Limites artistiques et de jeu

Il s'agit d'une rue d'essai pour examiner les assets. La composition d'une ville complète, les enseignes, les grandes installations et les objets sur plusieurs cases restent à développer. Les objets tournent actuellement par rotation de l'image ; des vues dessinées par orientation seront nécessaires pour certains meubles à façade visible.

Les équipements modernes sont des images plaçables, sans nouveaux services ni animations. Un robot placé reste immobile. Les portes utilisent les cadres métalliques existants, quelle que soit la famille de maçonnerie. Le modèle d'obstacle est celui de l'éditeur initial. L'intégration à la campagne attend l'examen du lot.
