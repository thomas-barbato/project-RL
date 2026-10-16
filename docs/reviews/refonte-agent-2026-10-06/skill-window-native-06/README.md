# SKILLS-NATIVE06 — cinq compactes de tir validées

Accord visuel : `../skill-icon-review-20/visual-acceptance-01.json`.
Variantes intégrées sans retouche : DI01-C03, DI02-C02, DI03-C03,
DI04-C03 et DI07-C02 (35–38 et 40). La banque compte 37 compactes
et 16 illustrations ; les 94 fiches et leurs paramètres restent identiques.

Contrôles effectués le 11 octobre 2026 :

- `cargo test --locked agent_skill_window::tests` : 3 tests réussis.
- `cargo build --locked` : réussi.
- `cargo fmt --all --check` et `git diff --check` : réussis.
- 289 empreintes précédentes vérifiées et préservées ; 25 exports validés vérifiés.
- Cinq captures du jeu en 960 × 540 ; processus terminés avec succès.
- Cinq icônes natives 64 px comparées aux exports acceptés : toutes conformes,
  écart moyen maximal de canal 7,306 / 255 (tolérance 15 pour le filtrage GPU).

Preuves : `capture-results.json`, `native-icon-matches.json`,
`integration-validation.json`, `native-selected-icons.png` et les dossiers
`DI01-960` à `DI07-960`. La capture se limite aux cinq identifiants concernés.
Les effets de combat et les achats de la refonte ne sont pas activés par ce travail.
