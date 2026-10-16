# Correction du joueur équipé — 4 octobre 2026

L'inventaire approuvé est conservé. Cette retouche concerne le sprite du joueur sur la carte et son portrait.

L'ancien visage peint par-dessus le corps est remplacé par trois sprites de classe dont les visages sont dessinés dans le style du corps. Les 32 protections ont été redessinées sur un personnage dans la même pose, puis séparées en parties portées. Une protection remplace la zone correspondante de la base : épaules, manches et torse pour le corps, casque frontal pour la tête, gants et bottes adaptés aux membres. Les accents de classe visibles entre les pièces restent rouges, bleus ou verts selon le personnage.

Les armes gardent leur position latérale. La poignée est placée sur un repère commun à la main droite ; les doigts sont dessinés devant elle et le canon reste à l'extérieur du torse. Le même assemblage sert aux sprites de carte et aux portraits.

## Fichiers enregistrés

- Dessins source : `C:/Users/User/Desktop/project-RL/artifacts/joueur-equipe-2026-10-04/classes-visages-v4.png`, `corps-portes.png`, `tetes-portees.png`, `mains-portees.png`, `pieds-portes.png` dans le même dossier.
- Prompts complets : [PROMPTS.md](PROMPTS.md). Dessins réalisés avec l'outil imagegen intégré, sans CLI ni API externe.
- Atlas embarqués : `C:/Users/User/Desktop/project-RL/assets/sprites/cyberpunk/player-bodies-32.png`, `player-bodies-128.png`, `player-worn-32.png`, `player-worn-128.png` dans le même dossier, ainsi que `player-worn.json`.
- Script reproductible de cadrage et séparation : `C:/Users/User/Desktop/project-RL/tools/art/build_player_worn_assets.py`.

## Vérifications

- 5 tests `game_art` réussis : les 101 armes touchent la prise, leur partie visible est connectée à celle-ci, elles restent dans la case et les armes à feu dégagent le torse. Les parties remplacées, les six gantelets et les 32 modèles dans les trois classes sont contrôlés.
- 2 tests `player_art` réussis : sélection d'apparence et commandes de direction sans altération de la partie.
- `cargo check --locked --all-targets`, compilation normale et compilation optimisée : réussis.
- Formatage et `git diff --check` : réussis.
- `python tools/art/build_player_worn_assets.py --check` : réussi.
- `python tools/art/build_cyberpunk_assets.py --check` : réussi ; les banques originales des 150 objets, 27 personnages et 101 armes sont conservées.
- Comparaison RVB de la fenêtre d'inventaire : ses 1216 × 740 pixels sont identiques à la capture approuvée. La fonction de dessin de la silhouette fil de fer est identique. Détail : [preservation.json](preservation.json).
- Captures natives du rendu corrigé, du jeu et de l'inventaire : codes de sortie 0, inspection des images effectuée. [Rendu corrigé](revue-native/cold-start.png), [jeu](jeu-natif/cold-start.png), [inventaire conservé](inventaire-preserve/cold-start.png).
- Diagnostic `--ui-cold-performance` : code de sortie 0, 120 images par scène. [Mesures complètes](performance/timings.json). Monde : première image CPU 341,85 ms, médiane 8,21 ms ; inventaire : première image CPU 26,27 ms, médiane 11,24 ms. Ces chiffres de débogage ne constituent ni une comparaison contrôlée avant/après, ni une mesure de FPS.

La suite complète du client n'a pas été relancée. Ses échecs précédemment signalés restent décrits dans `../combat-visuel-2026-10-04/RELECTURE.txt` ; les succès ciblés ci-dessus ne signifient pas que toute la suite est validée. Aucun commit ni envoi distant effectué.
