# Raccord des mains et proportions du joueur — 4 octobre 2026

Les mains rejoignent maintenant les ouvertures des manches de chaque protection de torse, dans les deux poses de tir. Les positions sont relevées sur les dessins des 14 modèles ; les deux mains sont déplacées séparément et les prises du fusil suivent leurs nouveaux points de contact. La pose au pistolet a aussi un avant-bras baissé plus court : son poignet est remonté d'environ deux pixels à l'échelle du personnage, vers la hanche.

Le torse est dessiné en continu derrière les mains. Les paumes passent derrière l'arme et les doigts devant sa poignée. Les fragments de chemise rouge sont retirés des calques de mains, ce qui laisse le fusil visible. Le pistolet est tourné vers le haut sans retournement en miroir, avec sa détente vers le corps. Le cadrage des fusils exclut les fragments isolés présents dans certaines illustrations sources.

L'inventaire et sa silhouette fil de fer approuvés sont conservés. Les quatre banques de la pose latérale et la transformation des armes de mêlée sont identiques à celles d'avant ces corrections. Les dessins des PNJ, règles de combat, butin et sauvegardes restent inchangés. Le catalogue comporte toujours 61 fusils/lanceurs et 40 armes de mêlée ; le pistolet de contrôle est une référence du diagnostic, sans ajout au contenu.

## Dessins et fichiers enregistrés

Les trois planches ont été éditées avec **imagegen intégré**, avec fond transparent. Les [prompts complets](C:/Users/User/Desktop/project-RL/docs/reviews/joueur-calques-corriges-2026-10-04/PROMPTS.md) sont conservés avec leurs sources :

- [Torses à deux mains sans mains peintes](C:/Users/User/Desktop/project-RL/artifacts/joueur-calques-corriges-2026-10-04/corps-two-handed-sans-mains.png).
- [Torses avec le bras levé sans mains peintes](C:/Users/User/Desktop/project-RL/artifacts/joueur-calques-corriges-2026-10-04/corps-raised-sans-mains.png).
- [Avant-bras baissé raccourci](C:/Users/User/Desktop/project-RL/artifacts/joueur-calques-corriges-2026-10-04/corps-raised-avant-bras-court.png). Seule cette région est extraite de la dernière planche ; le torse, l'épaule et le bras levé précédents sont repris sans modification.

Les atlas 32/128 pixels des deux poses sont dans `C:/Users/User/Desktop/project-RL/assets/sprites/cyberpunk/`, avec [player-grips.rs](C:/Users/User/Desktop/project-RL/assets/sprites/cyberpunk/player-grips.rs) et [player-worn.json](C:/Users/User/Desktop/project-RL/assets/sprites/cyberpunk/player-worn.json). Le script [build_player_worn_assets.py](C:/Users/User/Desktop/project-RL/tools/art/build_player_worn_assets.py) reproduit le cadrage, les séparations de calques et les points de raccord. [game_art_worn.rs](C:/Users/User/Desktop/project-RL/src/game_art_worn.rs) assemble les calques dans le client.

## Vérifications finales

- Onze tests `game_art` réussis, dont le raccord aux ouvertures réelles des manches, la couverture de ces ouvertures par les six modèles de gants et les mains sans gants des trois classes, les deux prises des fusils, les limites du sprite pour les 61 fusils et 14 torses, le sens du pistolet et l'absence de fragments rouges dans les doigts.
- Deux tests `player_art` réussis : choix de l'arme active et commandes de déplacement sans modification de la partie.
- `cargo check --locked --all-targets`, compilation debug et compilation optimisée : réussis sur la version corrigée.
- `cargo fmt --all --check`, `git diff --check`, scripts `build_player_worn_assets.py --check` et `build_cyberpunk_assets.py --check` : réussis.
- Quatre captures natives finales : codes de sortie 0. Les images ont été inspectées : [14 torses, trois classes et six gants](C:/Users/User/Desktop/project-RL/artifacts/joueur-calques-corriges-2026-10-04/player-cuffs-review/cold-start.png), [comparaison des poses](C:/Users/User/Desktop/project-RL/artifacts/joueur-calques-corriges-2026-10-04/combat-review/cold-start.png), [rendu en jeu](C:/Users/User/Desktop/project-RL/artifacts/joueur-calques-corriges-2026-10-04/terminal-hud-live/cold-start.png), [inventaire](C:/Users/User/Desktop/project-RL/artifacts/joueur-calques-corriges-2026-10-04/terminal-inventory-live-equipped/cold-start.png).
- La fenêtre d'inventaire est identique en RVB sur ses 1216 × 740 pixels à la version approuvée. Banques latérales identiques octet pour octet ; fonctions de silhouette fil de fer et de placement des armes latérales inchangées. [Preuve de conservation](C:/Users/User/Desktop/project-RL/artifacts/joueur-calques-corriges-2026-10-04/preservation.json).
- Diagnostic `--ui-cold-performance` terminé avec code 0 après les compilations, 120 images par scène. Monde : première image CPU 357,06 ms, médiane 9,15 ms ; inventaire : première image CPU 29,63 ms, médiane 12,24 ms. [Mesures complètes](C:/Users/User/Desktop/project-RL/artifacts/joueur-calques-corriges-2026-10-04/performance/timings.json). Ce diagnostic de débogage ne constitue ni une comparaison contrôlée avant/après, ni une mesure de FPS.

La suite complète du client n'a pas été relancée. Les échecs déjà signalés sont détaillés dans `C:/Users/User/Desktop/project-RL/docs/reviews/combat-visuel-2026-10-04/RELECTURE.txt` ; les succès ciblés ne valident pas toute la suite. Aucun commit ni envoi distant effectué.
