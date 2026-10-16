# Équipement par main et corps de 30 pixels — 5 octobre 2026

La génération 144 utilise trois emplacements simultanés : Main gauche, Main droite et Deux mains. Les deux armes à une main constituent une configuration visible ; l'arme à deux mains constitue l'autre. Activer une configuration conserve l'autre équipée, sans tour consommé. Les attaques utilisent l'arme sélectionnée ; aucune attaque supplémentaire automatique n'est ajoutée.

Les haches et épées de mêlée utilisent Deux mains, les autres armes de mêlée une main. Les pistolets et pistolets mitrailleurs utilisent une main, les autres armes à distance Deux mains. Le contenu actuel n'a pas encore de pistolet ou de pistolet mitrailleur ; leurs dessins sont des références, et la classification de ces familles est vérifiée avec des armes synthétiques.

Les sauvegardes jusqu'à la génération 143 conservent leurs trois canaux historiques. Les agencements personnalisés des règles et des mods restent conservés. Aucun fichier de sauvegarde normal n'a été migré. La reprise vérifie les règles historiques avant de charger l'état ; les instantanés de génération 144 reçoivent l'agencement effectif de la partie pour éviter un retour inutile au rejeu.

## Dessins et taille

- 17 tenues : trois classes casquées et 14 armures de torse.
- Huit poses par tenue, soit 136 figures : sans arme, épée courte, épée géante, fusil, pistolet, deux épées, deux pistolets, combinaison épée/pistolet.
- Le dessin entier contient les mains, les bras et les armes. Les poses à une main et les combinaisons inversées sont retournées pour correspondre aux mains équipées.
- Corps mesuré à **30 pixels de hauteur dans les 136 figures**, à l'échelle native des cases de 32 pixels. Les atlas du monde utilisent un cadre de 48 pixels : ils sont dessinés sans réduction supplémentaire au-dessus et autour de la case. Les pieds gardent une ligne de contact commune. Le zoom du jeu continue à mettre ces proportions à l'échelle.
- Les armes peuvent dépasser du corps et de la case. Le dessin ne participe pas aux collisions ; la carte et la position de l'acteur continuent à les déterminer.
- L'icône de mêlée au sol utilise la nouvelle épée industrielle. Les illustrations détaillées des objets et les dessins des PNJ sont conservés.
- La référence de prise du pistolet fournie par l'utilisateur est conservée dans `reference-prise-pistolet-utilisateur.png`. La correction de cette prise et la refonte industrielle des armes de tir restent l'étape suivante demandée par l'utilisateur.

Les nouvelles sources sont générées avec imagegen intégré. Les prompts exacts sont conservés dans `PROMPTS.md` et `PROMPTS-AMBIDEXTRIE.md`. Les fusils et pistolets seuls réutilisent les sources précédemment approuvées. Le rangement en atlas effectue uniquement sélection de composante, découpe, nettoyage de l'alpha et réduction ; il ne redessine pas les armes.

## Vérifications

- `cargo test --locked --lib` : **809 réussites**, aucun échec (`tests-final-engine.log`).
- Tests finaux ciblant mains, graphismes, inventaire, scènes visuelles, arsenal, familles d'armes, reprise de protocole et instantané rapide : **45 réussites**, aucun échec (`tests-final-targeted.log`).
- Tests complets du laboratoire après adaptation de ses équipements et commandes : **39 réussites**, aucun échec (`tests-lab.log`).
- Les refus d'équipement sur une mauvaise main sont vérifiés sans mutation de l'état ni événement. Les configurations coexistantes, déplacements d'instances et reprises avec la paire ou Deux mains sont vérifiés.
- `cargo check --locked --all-targets`, compilation debug et compilation release : réussites (`check-final.log`, `build-debug-final.log`, `build-release-final.log`). Des avertissements antérieurs de code inutilisé restent présents.
- Les deux générateurs de personnages et de correspondances graphiques passent `--check` : les banques sont reproductibles.
- `git diff --check` : réussite. Format des fichiers modifiés : réussite. `cargo fmt --all --check` signale deux blocs préexistants dans `src/armor_models_tests.rs`, conservés pour ne pas reformater ce travail indépendant.

La suite client globale initiale compte 561 réussites, 61 échecs et 15 tests ignorés (`tests-client.log`). Une contre-épreuve isolée avec les sources et assets d'avant ces changements confirme **27 échecs déjà présents** (`tests-baseline-failures.log`). Ils concernent principalement des empreintes de génération, des boutiques, des reprises et les gardes de création de partie.

Une reprise ciblée de 99 tests donnait 68 réussites et 31 échecs (`tests-final-client.log`) : les 27 mêmes échecs antérieurs et quatre cas supplémentaires. Ces quatre cas ont ensuite été corrigés et vérifiés dans la série finale de 45 tests : arsenal du laboratoire, fusil à pompe dans Deux mains, emplacement du fusil de Vigie et chargement rapide d'instantané. La suite globale n'est donc pas annoncée verte ; les échecs antérieurs restent hors de cette modification.

## Captures finales et affichage

Les captures sont prises dans le client compilé, en 1280 × 800, sans agrandissement de l'image exportée :

- `player-outfits-review/cold-start.png` : 14 tenues, grande épée et combinaison mixte en portrait ; sept poses armées à leur taille native sous chaque portrait.
- `terminal-inventory-live-equipped/cold-start.png` : trois armes équipées, configuration Deux mains active, tooltip d'armure équipée.
- `terminal-inventory-live-hands/cold-start.png` : mêmes équipements, paire des mains active, arme à deux mains conservée mais inactive.
- `terminal-hud-live/cold-start.png` : personnage avec fusil à la nouvelle taille, jauges et terrain du jeu.

Les trois scènes finales d'inventaire et de jeu terminent avec un code de sortie nul. Inspection visuelle : étiquettes et connexions des mains correctes, jauges préservées, armes lisibles et corps sans réduction liée à leur longueur.

Diagnostic d'affichage à froid (`performance/timings.json`) : dix scènes, 120 images par scène, debug, 1280 × 800. Médiane CPU du monde **8,04 ms**, inventaire **10,86 ms** ; première image du monde **345,60 ms**, inventaire **25,94 ms**. Ces mesures décrivent cette exécution, sans constituer un comparatif contrôlé ni une mesure de FPS en release.

Aucun commit, push ou déploiement. Les changements indépendants déjà présents dans le projet sont conservés.
