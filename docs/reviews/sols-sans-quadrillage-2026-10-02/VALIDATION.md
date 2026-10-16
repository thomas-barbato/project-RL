# Sols sans quadrillage — 2 octobre 2026

Les bordures haut/gauche des cases et le point répété dans leur angle ont été
retirés du rendu des sols. Neuf matériaux utilisent un atlas natif de 768 × 768
pixels produit une fois, avec des motifs continus à travers les cases. Le métal,
la chaussée, les voies, les caillebotis, le béton, le gravier, l'herbe, la friche
et la boue gardent des teintes sombres et des détails propres au matériau.

L'atlas est fixé aux coordonnées du monde, indépendant du générateur aléatoire
du jeu et du zoom. Les fonds sous les objets utilisent également leur surface.
La grille logique, les sauvegardes, le gameplay et les effets ne sont pas modifiés.

## Vérifications

- `cargo fmt --check` et `git -c core.safecrlf=false diff --check` : OK.
- `cargo check --locked --all-targets` : OK.
- `cargo test --locked --bin project-rl terminal_view::` : 25 tests réussis,
  couvrant notamment les effets, la mémoire, le cercle et les coordonnées de vue.
- Compilation debug et release : OK. Deux avertissements release préexistants
  concernent `arsenal_app` et des méthodes non utilisées.
- Captures natives relues : ville, sous-sol, catalogue des neuf sols, zooms 24,
  32, 40 et 48, fenêtre de 960 × 540.
- Diagnostics GPU `vision-circle`, `vision-circle-memory` et
  `vision-circle-memory-small` : OK, sans carrés noirs dans le terrain mémorisé.
  Les pixels de contrôle sont comparés à leur propre grain de sol, avec la même
  tolérance de deux niveaux RGB qu'avant ; deux cases voisines ne sont plus
  supposées avoir une couleur uniforme identique.
- Diagnostic natif de performance : 120 images pour chacun des dix écrans et
  scènes, sans changement de l'empreinte de simulation. Mesures debug : scène
  de jeu 10,91 ms de CPU médian, 13,24 ms au 95e percentile ; première image
  286,19 ms, comprenant l'initialisation de l'atlas. Ce diagnostic ne fournit
  pas de comparaison de performance avec la version précédente.
- L'exécutable release démarre sans erreur ; contrôle de lancement de six
  secondes sur le menu principal. Les captures de terrain proviennent du debug.

## Captures

- `avant-ville/cold-start.png` : ancien quadrillage visible.
- `ville-v1/cold-start.png` : ville avec les sols définitifs.
- `sous-sol-v1/cold-start.png` : sous-sol avec les sols définitifs.
- `catalogue/cold-start.png` : neuf surfaces, dessinées par le rendu du jeu.
- `memoire/cold-start.png`, `inconnu/cold-start.png`,
  `memoire-960/cold-start.png` : vision circulaire.
- `sous-sol-24/cold-start.png`, `ville-40/cold-start.png`,
  `ville-48-960/cold-start.png` : zooms et petite fenêtre.

Le dossier `memoire-v1` conserve la capture intermédiaire qui a révélé que
l'ancien contrôle comparait les pixels de deux sols désormais différents.
Le contrôle final utilise la couleur attendue de chaque pixel et réussit.

Pour régénérer le catalogue :
`target/debug/project-rl.exe --ui-cold-floor-palette <nouveau-dossier>`.
Les diagnostics habituels acceptent les suffixes `-cell24`, `-cell40` et
`-cell48`, éventuellement suivis de `-small`, sans modifier les réglages du joueur.
