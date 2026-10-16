# Terminal et inventaire intégrés

Le terminal validé est actif dans le jeu lancé avec `cargo run`. L'inventaire
reprend ses cadres angulaires, ses modules et la couleur choisie dans les options.
Sa silhouette, les emplacements d'équipement, l'activation des configurations
d'armes et les détails au survol gardent leur fonctionnement.

Les PV, l'énergie et la chaleur de l'inventaire utilisent les mêmes données et
couleurs que le HUD : PV verts ou rouges en danger, énergie bleue ou rouge à zéro,
chaleur selon ses seuils. L'énergie réservée garde son hachurage violet. L'expérience
utilise la couleur du thème et la courbe de progression de la partie courante.
Le fond opaque de l'inventaire supprime les cadres du plateau en transparence.

## Implémentation

- `src/terminal_chrome.rs` fournit les cadres, titres, modules, jauges segmentées
  et boutons communs, toujours à partir de `UiTheme`.
- Le renderer normal du plateau dessine le terminal directement. L'ancienne
  surcouche de proposition et les cadres verts fixes sont supprimés.
- L'inspection conserve la cible réellement survolée, les contacts perçus et
  les informations de vision/bruit. Les zones cliquables des commandes et des
  ressources suivent leurs nouveaux rectangles.
- Les titres préparent leurs lettres ensemble. Les tailles de police utilisées
  dans l'inventaire sont préparées au démarrage, pour éviter les chargements
  répétés d'atlas pendant sa première ouverture.
- Les sprites et les règles de jeu ne sont pas modifiés par cette étape.

## Vérifications

- 33 tests ciblés réussis : 5 inventaire, 18 terminal/caméra/perception,
  5 écran de mort, 1 bouton Attendre, 1 journal et 3 tests HUD.
- Compilation debug réussie sans avertissement. Compilation release réussie
  avec les quatre avertissements déjà présents (arsenal, anciens chemins de
  reprise, variante `Visual::Item` et `draw_item`).
- `cargo fmt --all` et `git diff --check` réussis.
- 15 captures dans le moteur : quatre thèmes, fenêtres compactes, UI à 150 %,
  1920 × 1080, objet du sac, équipement survolé et mort.
- Les 14 diagnostics du terminal/inventaire gardent les octets de la partie
  identiques avant/après dessin. Les commandes du HUD sont effectivement
  rasterisées dans les captures, avec leurs zones cliquables correspondantes.
- Les pixels des barres PV/énergie sont vérifiés dans les quatre thèmes :
  `(122, 224, 150)` et `(109, 186, 255)`.
- La capture de mort est identique à la précédente capture validée.
- Le préchauffage des polices garde la même géométrie et les mêmes couleurs :
  seuls cinq pixels d'anticrénelage du texte diffèrent d'une unité RGB.
- La suite complète n'a pas été relancée.

## Mesure de fluidité

Diagnostic debug habituel, 1280 × 800, 120 images par scène, partie inchangée.
L'ouverture initiale de l'inventaire est mesurée à **24,58 ms**, contre 27,58 ms
dans le diagnostic précédent. Avant la préparation des polices, elle atteignait
plus de 200 ms. Le rendu régulier est mesuré à 12,12 ms pour l'inventaire
(précédemment 11,56 ms), et le retour au plateau à 8,35 ms (précédemment 8,39 ms).
Ces mesures debug restent sensibles à la charge de la machine.

Les détails sont dans `validation.json`, `performance/timings.json`, les journaux
de tests et `changes.patch`. `before/` préserve les fichiers concernés avant
cette étape, sans effacer les modifications antérieures de la branche.

## Galerie

[Captures numérotées](http://127.0.0.1:8769/galerie-terminal.html), avec vue à taille
réelle. L'ancienne proposition est conservée dans `galerie-terminal-proposition.html`.
`capture.ps1`, `audit.py` et `build_gallery.py` permettent de reproduire les contrôles.
Pour une nouvelle série sans écraser la capture de mort, utiliser
`capture.ps1 -CaptureDirectory captures-nouveau`.

Aucun commit ni push.
