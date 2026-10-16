# Intégration graphique du 4 octobre 2026

L’aperçu validé est intégré au client Rust/Macroquad. La correction demandée pendant l’intégration reprend les décors précédents et les enrichit à l’échelle de 32 pixels.

**Retour visuel ultérieur : l’utilisateur a rejeté ces décors.** Les personnages et objets restent validés. La comparaison des premières planches dans `artifacts/decors-reference-32-2026-10-04` a ensuite été retenue, avec la contrainte de conserver l’architecture et les placements du jeu. Son intégration est décrite dans [la validation des décors](../decors-integration-2026-10-04/VALIDATION.md). Les résultats ci-dessous décrivent le premier rendu technique, désormais remplacé pour les matières urbaines.

## Rendu intégré

- Cases de 32 pixels par défaut ; les préférences de zoom explicitement enregistrées restent respectées. Sur une petite fenêtre, la caméra garde la taille choisie et montre une portion plus petite de la carte.
- Personnages, PNJ et bestiaire de face avec une petite ombre de contact calculée à partir des pieds de chaque silhouette.
- 16 icônes de catégories au sol pour les 150 modèles du catalogue chargé : 61 armes à feu, 40 armes de mêlée, 32 pièces d’armure et 17 autres objets. Les armures partagent une seule icône.
- Illustrations spécifiques à chaque modèle dans les listes d’inventaire, les détails et la cible. Les modèles inconnus des mods conservent le dessin de secours.
- Cible illustrée en bas à droite, en survolant une case visible ou en sélectionnant une cible. Le panneau garde les informations de santé et d’analyse.
- Sols continus avec les couleurs et matières précédentes : asphalte, béton fissuré, métal usé, grilles. Détails ajoutés à l’échelle du pixel natif.
- Murs raccordés conservés et enrichis. Portes, serveurs, consoles, caisses, ventilations et réservoirs reprennent les silhouettes précédentes dans une banque native de 32 pixels, avec de petites fixations et rainures supplémentaires.

Les trois variantes complètes validées de chaque classe sont sélectionnées selon la classe et l’arme active. Elles ne représentent pas encore le port exact de chaque modèle d’arme et d’armure par des calques interchangeables. Les modèles individuels sont tous présents dans l’inventaire et la cible. Aucun zombie ni nouvel ennemi n’est ajouté aux règles du jeu.

## Captures du client

- [Plateau, objets, personnages et décors](cyberpunk-decors-final/cold-start.png), 1280 × 800.
- [Portrait et informations d’une cible analysée](cyberpunk-actor-final/cold-start.png), 1280 × 800.
- [Ville de la partie normale](ville-decors-32/cold-start.png), 1280 × 800.
- [Plateau dans une petite fenêtre](cyberpunk-small-decors-final/cold-start.png), 960 × 540.
- [Inventaire dans une petite fenêtre](cyberpunk-inventory-small-decors-final/cold-start.png), 960 × 540.
- [Illustration d’armure dans l’inventaire](cyberpunk-armour-decors-final/cold-start.png), 1280 × 800.
- [Matériaux de sols](floor-palette-final/cold-start.png) et [murs](floor-palette-walls-final/cold-start.png), rendus par le client.

Les captures `cyberpunk*` exportent `art-validation.json`. Les octets de l’état moteur sont identiques avant et après les trois images de vérification. Les scènes de diagnostic ont leur propre partie et n’écrivent pas la sauvegarde du joueur.

## Vérifications

- `cargo fmt --all --check`, `cargo check --locked`, `git diff --check` : OK.
- `cargo build --locked --bin project-rl` et `cargo build --locked --release --bin project-rl` : OK. L’exécutable release utilisé par les lanceurs existants est recompilé.
- `python tools/art/build_cyberpunk_assets.py --check` : reproduction identique des banques et du catalogue.
- Suite moteur : 806 tests réussis lors de l’exécution générale.
- [Tests graphiques ciblés](graphics-tests.log) : 56 réussis, notamment la couverture des 150 modèles, l’absence de fuite de butin caché, les raccords mémorisés, les portes, le zoom, le pointeur, les ombres et l’espace de l’en-tête d’inventaire.
- [Tests de création avec un profil isolé](creation-tests.log) : 3 réussis. Leur premier échec venait d’une sauvegarde déjà présente dans le profil utilisé par la suite générale.

La première exécution générale du client a produit 574 réussites, 29 échecs et 15 tests ignorés. Les deux échecs introduits dans le diagnostic de sprites et la géométrie de l’en-tête d’inventaire ont été corrigés et passent dans la suite ciblée. Trois autres cas passent avec un profil isolé. Les 24 échecs restants ont été reproduits sur l’archive de `HEAD`, sans modifications graphiques : 23 dans [la comparaison du client](baseline-tests.log), et le cas de survie générée dans [sa comparaison séparée](baseline-survival.log). Ils concernent notamment les anciens fingerprints, les commerces, les trajets et les reprises. La suite générale n’est donc pas entièrement verte ; ces problèmes existants restent hors du changement graphique.

## Mesure native

`--ui-cold-performance` a terminé ses 10 scènes de 120 images sans modification de la simulation. [Mesures debug locales](performance-final/timings.json) : monde, médiane CPU 12,76 ms et p95 13,92 ms ; inventaire, médiane 16,04 ms et p95 20,05 ms. La première image du monde inclut la génération ponctuelle des textures de sol (477 ms). Ces valeurs décrivent cette mesure locale et ne constituent pas une comparaison avant/après ni une mesure release.

## Sources

Les dessins originaux de l’aperçu sont préservés. Aucun asset de Crawl ou de Jupiter Hell n’est embarqué. Les quatre banques actives sont intégrées à l’exécutable ; le serveur d’aperçu et Python ne sont pas nécessaires pour jouer. La banque de terrain simplifié de l’ancien aperçu est conservée comme référence et n’est plus utilisée par le client.

Aucun commit ni push n’a été effectué.
