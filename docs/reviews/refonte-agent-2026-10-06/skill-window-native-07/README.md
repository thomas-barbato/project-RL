# SKILLS-NATIVE07 — Survie validée

Accord : `../skill-icon-review-23/visual-acceptance-01.json`.
Les cinq compactes 41–45 sont intégrées sans retouche : B02-C03, SU03-C02,
SU05-C02, SU06-C01, BD06-C01. Banque : 42 compactes, 16 illustrations.
Les 94 fiches, leurs textes et leurs paramètres restent identiques.

Contrôles du 11 octobre 2026 :

- `cargo test --locked agent_skill_window::tests` : 3 tests réussis.
  Le premier lancement avait conservé l’ancien compte attendu de 37 dessins ;
  après mise à jour à 42, les trois tests passent.
- `cargo build --locked`, `cargo fmt --all --check` et `git diff --check`
  sur les fichiers concernés : réussis.
- 319 empreintes précédentes et 25 nouveaux exports vérifiés.
- Cinq captures réelles 960 × 540, processus terminés avec succès.
- Icônes natives comparées aux cinq exports acceptés : toutes conformes,
  écart moyen de canal maximal 11,64 / 255 (tolérance 15 pour le filtrage GPU).

Preuves : `integration-validation.json`, `capture-results.json`,
`native-icon-matches.json`, `native-selected-icons.png` et les dossiers `*-960`.
Les effets de combat et les achats ne sont pas activés par l’ajout de ces images.
