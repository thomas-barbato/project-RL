# SKILLS-NATIVE08 — Esquive validée

Accord explicite « ok on valide », figé dans `../skill-icon-review-27/visual-acceptance-01.json`. Les compactes 46–49 (`ES03-C01`, `ES05-C03`, `ES06-C02`, `T02-C04`) sont intégrées sans retouche. Banque : 46 compactes et 16 illustrations.

Contrôles du 11 octobre 2026 :

- `cargo test --locked agent_skill_window::tests` : trois tests réussis.
- `cargo build --locked`, `cargo fmt --all --check` et `git diff --check` ciblé : réussis.
- 502 empreintes précédentes conservées, 20 nouveaux exports et quatre masters vérifiés ; les quatre PNG actifs correspondent exactement aux exports 64 px approuvés.
- Les 94 fiches, textes, paramètres, labels et entrées précédentes du registre restent identiques.
- Quatre captures natives 960 × 540, tous les processus terminés avec succès. Icônes comparées aux sources approuvées : écart moyen de canal maximal 5,711 / 255, sous la tolérance 15 liée au filtrage GPU.
- La capture complète de Pas spectral et la planche des quatre icônes natives ont été examinées visuellement.

Preuves : `integration-validation.json`, `protected-before.json`, `tests.log`, `build.log`, `capture-results.json`, `native-icon-matches.json`, `native-selected-icons.png` et les dossiers `*-960`.

`index.html` présente les captures réelles à taille native. L’ajout des dessins n’active ni les achats ni de nouveaux effets de combat.
