# Poses des armes du joueur — 4 octobre 2026

Les 61 fusils et lanceurs du catalogue se tiennent désormais à deux mains devant le torse. La transformation de chaque illustration rejoint la poignée et le support avant ; les doigts sont dessinés au-dessus. Les 40 armes de mêlée conservent leur position latérale et leur transformation précédente.

La pose des pistolets et revolvers est préparée séparément : coude plié, avant-bras levé et canon dirigé vers le haut à côté du visage. Le catalogue actuel ne contient aucun pistolet. L'écran de contrôle utilise une référence graphique isolée, disponible uniquement dans le client de diagnostic, sans ajouter une arme aux objets, au butin ou aux sauvegardes. Les catégories du catalogue graphique choisissent la pose ; la portée d'attaque ne décide pas du nombre de mains.

Les manches des 14 protections de torse et les six modèles de gants ont des dessins adaptés aux deux nouvelles poses. Les visages, jambes, casques et bottes proviennent des dessins précédents. Les trois couleurs de classe sont conservées. Les quatre banques de la pose latérale sont identiques octet pour octet aux versions précédentes. L'inventaire approuvé et la fonction de silhouette fil de fer restent identiques.

## Fichiers enregistrés

Dessins réalisés avec l'outil **imagegen intégré**. Les prompts complets sont dans [PROMPTS.md](C:/Users/User/Desktop/project-RL/docs/reviews/joueur-tir-deux-mains-2026-10-04/PROMPTS.md).

Sources dans `C:/Users/User/Desktop/project-RL/artifacts/joueur-tir-deux-mains-2026-10-04/` :

- `classes-deux-mains.png`, `classes-main-levee.png` : deux poses des trois classes.
- `corps-two-handed.png`, `corps-raised.png` : 14 torses et manches dans chaque pose.
- `gants-deux-mains.png` : six paires de gants à deux mains ; `gants-tir.png` fournit les six versions à bras levé, dans ses deux dernières rangées. Sa première proposition à deux mains est remplacée par la planche dédiée.
- `pistolet-reference.png` : illustration de contrôle de la pose à une main.

Atlas embarqués dans `C:/Users/User/Desktop/project-RL/assets/sprites/cyberpunk/` :

- `player-two-handed-bodies-32.png`, `player-two-handed-bodies-128.png`, `player-two-handed-worn-32.png`, `player-two-handed-worn-128.png`.
- `player-raised-bodies-32.png`, `player-raised-bodies-128.png`, `player-raised-worn-32.png`, `player-raised-worn-128.png`.
- `player-reference-pistol-128.png` : référence du diagnostic, exclue de la compilation optimisée.
- `player-worn.json` : sources et repères des trois poses.

Le script [build_player_worn_assets.py](C:/Users/User/Desktop/project-RL/tools/art/build_player_worn_assets.py) reproduit le cadrage, la séparation anatomique, les couleurs et les atlas. [build_cyberpunk_assets.py](C:/Users/User/Desktop/project-RL/tools/art/build_cyberpunk_assets.py) génère les métadonnées de prise des armes sans modifier leurs dessins originaux.

## Vérifications

- Sept tests `game_art` réussis : contact des 101 modèles avec leurs mains, continuité du dessin entre les deux prises des fusils, absence de débordement du canon, direction du pistolet de référence, gants adaptés aux prises et conservation des visages/jambes.
- Deux tests `player_art` réussis : choix de l'arme active et commandes directionnelles sans modification de la partie.
- Classification vérifiée pour les catégories de pistolets/revolvers, fusils et armes de mêlée.
- `cargo check --locked --all-targets`, compilations debug et optimisée : réussis.
- `cargo fmt --all --check`, `git diff --check` et les deux scripts d'assets avec `--check` : réussis.
- Trois captures natives : codes de sortie 0. [Comparaison des poses](C:/Users/User/Desktop/project-RL/artifacts/joueur-tir-deux-mains-2026-10-04/combat-review/cold-start.png), [jeu réel](C:/Users/User/Desktop/project-RL/artifacts/joueur-tir-deux-mains-2026-10-04/terminal-hud-live/cold-start.png), [inventaire conservé](C:/Users/User/Desktop/project-RL/artifacts/joueur-tir-deux-mains-2026-10-04/terminal-inventory-live-equipped/cold-start.png). Les images ont été inspectées, dont la comparaison après correction de l'alignement des poignets.
- L'inventaire approuvé est identique en RVB sur ses 1216 × 740 pixels. Les banques latérales, leur transformation d'armes et le dessin fil de fer sont préservés. [Preuve](C:/Users/User/Desktop/project-RL/artifacts/joueur-tir-deux-mains-2026-10-04/preservation.json).
- Diagnostic `--ui-cold-performance` terminé avec code 0, après les compilations, 120 images par scène et empreinte de simulation inchangée. Monde : première image CPU 368,36 ms, médiane 8,59 ms ; inventaire : première image CPU 25,81 ms, médiane 11,60 ms. [Mesures complètes](C:/Users/User/Desktop/project-RL/artifacts/joueur-tir-deux-mains-2026-10-04/performance/timings.json). Ces chiffres de débogage ne constituent ni une comparaison contrôlée avant/après, ni une mesure de FPS.

La suite complète du client n'a pas été relancée ; ses échecs précédemment signalés restent décrits dans `C:/Users/User/Desktop/project-RL/docs/reviews/combat-visuel-2026-10-04/RELECTURE.txt`. Les succès ciblés ci-dessus ne valident pas toute la suite. Aucun commit ni envoi distant effectué.
