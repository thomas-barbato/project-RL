# Éditeur de cartes : outils de composition

Version locale du 1 octobre 2026, intégrée à `target/debug/examples/map_editor.exe`.

## Essayer

Lancer `Editeur_de_cartes.cmd` à la racine du projet. `Essayer_outils_editeur.cmd` ouvre une carte de démonstration indépendante. Les fichiers de cartes existants restent lisibles.

- Catalogue / F2 : recherche sans accents, grandes vignettes, favoris et 24 éléments récents.
- Ligne, Rectangle, Remplir : composition en une opération annulable. Les murs du rectangle se raccordent dans l'aperçu. Les placements incompatibles sont refusés entièrement.
- Sélection : glisser une zone. Ctrl+C copie les calques visibles et déverrouillés ; Ctrl+V active le collage ; R tourne ; Suppr retire. Déplacer permet de glisser depuis la sélection. Une seule case permet d'inspecter un élément.
- Ensembles / F7 : nommer et conserver une sélection, puis la replacer, éventuellement tournée, dans une autre carte.
- Calques / F3 : visibilité et verrouillage des sols, murs, meubles, décorations et marqueurs.
- Propriétés / F4 : empreinte de 1 à 8 cases par dimension, rotation, décalages natifs, taille du dessin, passage et vision indépendants. Le mode décoration permet plusieurs objets non bloquants dans une case. Un mur ou une porte sélectionné possède son propre panneau.
- Vérifier / F5 : zones inaccessibles, portes isolées, marqueurs sans accès ou destination, surcouches de déplacement, vision et accès. Cliquer une remarque recentre la vue.
- Ennemis, Cave, Sortie : zones candidates avec profil ou destination. Même graine reproduit le tirage ; Graine suivante le change. Les cases choisies sont accessibles, distinctes et éloignées du départ d'au moins une case.
- Cartes / F6 : bibliothèque avec aperçu texturé ; cinq versions précédentes conservées lors des enregistrements.
- Miniature et Centrer : retrouver une zone de la carte. La récupération automatique conserve les modifications toutes les 30 secondes hors d'un trait en cours et à la fermeture sans enregistrement.

Le guide détaillé et le cahier des charges mis à jour se trouvent dans `docs/EDITEUR_CARTES.md`.

## Images ajoutées

28 vues utilisées pour sept objets : chaise, table, console opérateur et lits bleu, rouge, gris et orange. Chaque objet possède quatre vues distinctes ; le rendu ne couche plus leur façade par une rotation du PNG.

Génération avec l'outil d'images intégré à Codex, mode **built-in**, fond transparent. Les prompts sont conservés dans `prompts/meubles.txt` et `prompts/lits.txt`. Sources copiées sans modification :

- `assets/prototypes/surface64/directions/furniture-four-directions-v1.png` : lignes table, serveurs, chaise et console. Les quatre vues de serveurs ne sont pas utilisées ; leur perspective n'était pas assez cohérente.
- `assets/prototypes/surface64/directions/beds-four-directions-v1.png` : quatre couleurs et quatre directions de tête de lit.

La préparation du catalogue extrait et réduit les sprites à 64 pixels. Les sources précédentes sont conservées. Les autres objets utilisent encore leurs images existantes et la rotation du PNG.

## Vérification

- `cargo test --locked --example map_editor --example textured_surface_preview` : **52 + 25 tests passés**.
- `cargo test --locked --lib world::` : **73 tests passés**, dont déplacements, champs de vision, propagation, génération et compatibilité JSON/bincode de Map.
- `cargo fmt --all -- --check` et `cargo check --locked --all-targets` : passés.
- `git diff --check` : passé.
- Compilation des deux exécutables d'essai : réussie.
- Capture native avec `--capture-workbench` : panneaux vérifiés à 1360 × 840 et 960 × 540, miniature texturée et ensembles, déplacement de groupe puis annulation exacte, refus atomique d'un collage incompatible, calque verrouillé, aperçu des raccords, tirage reproductible, versions et récupération.
- Capture complémentaire `--capture-viewport`, dans `verification-vue` : un meuble agrandi et décalé reste visible lorsque son ancrage sort de la vue ; panneau des portes avec le texte final.

Empreinte SHA256 de l'exécutable livré : `F9FFCB70626A9EF8B6A0E9B627031540787ECF3FEF0FBCDEB75B4B5F529F5D6C`.

Les captures de référence sont dans **validation-finale**. La carte `maps/atelier.json` et l'ensemble `ensembles/salle-technique.json` dans ce dossier servent uniquement à la vérification. Le rendu `carte-native.png` représente une scène technique de démonstration, pas un plan définitif de ville.

## Limites actuelles

Les nouvelles possibilités sont celles de l'éditeur autonome. Les profils et destinations des marqueurs sont enregistrés comme références libres ; les cartes n'alimentent pas encore la génération de la campagne. L'aperçu ne fait pas apparaître des ennemis jouables, ne génère pas l'intérieur des caves et ne déclenche pas les transitions. Les machines, lasers, caméras, escaliers et armurerie gardent les limites de comportement décrites dans le cahier des charges.

Les obstacles de mobilier sont reconstruits depuis le document de l'éditeur. Leur surcouche dans Map est transitoire et exclue de sa sérialisation : les dispositions JSON et bincode des sauvegardes de campagne sont conservées. Les fichiers de l'éditeur gardent explicitement empreintes et propriétés.

Les tests et captures couvrent la logique de composition et le rendu natif. Ils ne constituent pas un parcours automatisé de chaque combinaison de clics et raccourcis. La suite complète du jeu n'a pas été relancée dans cette tâche.
