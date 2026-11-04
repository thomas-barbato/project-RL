# C03 — taille des objets au sol intégrée

Le 1er novembre 2026, le joueur constate que la taille n’a pas changé en jeu.
La proposition acceptée dessinait explicitement ses icônes à 40 px, mais la
branche normale `Visual::ColoredItem` utilisait encore toute la case de 64 px.

Le rendu de carte centre désormais chaque objet dans un rectangle mesurant
40/64 de la case, dans Debug comme Release. Les couleurs et l’opacité restent
transmises au même rendu d’icône. Les dessins, l’inventaire, les fiches, la
case sélectionnable, le ramassage et l’ordre de superposition avec les corps
restent inchangés. Aucune autre proposition ANIM-POLISH01 n’est activée ici.

## Vérifications exécutées

- `cargo build --locked --bin project-rl` : réussi.
- `cargo build --locked --release --bin project-rl` : réussi, 2 min 15 s.
- `--ui-cold-ground-icons` : 28 comparaisons graphiques réussies, couvrant
  mitrailleuse lourde, lance-roquettes, armure et ressource dans les sept
  palettes générales. Les pixels du chemin normal de carte correspondent
  à la référence approuvée de 40 px. Les icônes de fiche correspondent à leur
  référence antérieure de 64 px. Les comparaisons sont faites successivement
  aux mêmes coordonnées pour éviter les différences d’échantillonnage nearest
  aux frontières de texels entre des colonnes d’écran éloignées.
- `--ui-cold-enemy-loot` : capture native réussie d’un butin réellement lâché
  par un ennemi détruit. Le couteau réduit reste dessiné au-dessus du cadavre.
- Les deux captures graphiques sont produites par le moteur Debug sans charger
  la campagne ni les préférences utilisateur. La compilation Release est
  contrôlée séparément ; aucune partie utilisateur n’est ouverte pour cet essai.
- Python, JavaScript de l’aperçu, JSON, formatage Rust des fichiers touchés et
  `git diff --check` : contrôlés. La suite complète n’a pas été relancée.

La page d’aperçu précise maintenant que la comparaison des objets au sol
montre la référence antérieure à gauche et la taille intégrée à droite.
Le lancement habituel reste `Jouer.cmd`, qui construit la version Release.
Le paquet portable historique n’est pas reconstruit dans cette reprise.

Preuves reproductibles :

- `artifacts/animation-polish-01/C03-integration/checks.json`
- `artifacts/animation-polish-01/C03-integration/ground-icons-0.png` à `-6.png`
- `artifacts/animation-polish-01/C03-enemy-loot/cold-start.png`

Ces fichiers sont des diagnostics ; le jeu ne dépend pas de leur présence.
