# SKILLS-NATIVE09 — Feu validé

Le 11 octobre 2026, le joueur valide IC28 : « oui j'aime , je valide :) ».
Les cinq compactes sont intégrées exactement :

- 50 — FE02-C01 — Souffle de feu
- 51 — FE03-C01 — Mur de feu
- 52 — FE04-C01 — Émanation soudaine
- 17 — FE05-C03 — Boule de feu
- 53 — FE06-C01 — Combustion

Accord figé dans `../skill-icon-review-28/visual-acceptance-01.json`.
Braise et l'illustration existante de Boule de feu sont préservées.
La banque active compte 51 compactes et 16 illustrations, soit 67 dessins.

## Vérifications

- 530 empreintes de fichiers préexistants sont inchangées.
- Les 25 exports et les cinq masters approuvés correspondent à leurs empreintes.
- Les cinq PNG actifs correspondent exactement aux exports approuvés de 64 px.
- Les 94 fiches du catalogue, leurs textes, paramètres et règles sont conservés.
- `cargo test --locked agent_skill_window::tests` : trois tests réussis.
- `cargo build --locked`, `cargo fmt --all --check` et `git diff --check` réussissent.
- Les cinq captures natives sortent avec le code 0, en 960 × 540 px.
- Les icônes relevées dans ces captures correspondent aux sources ; l'écart moyen
  maximal par canal est 7,946 / 255, sous la tolérance de 15 liée au filtrage GPU.
- L'aperçu du navigateur affiche la capture à sa taille réelle, sans débordement,
  dans le viewport courant de 1108 × 884 px.

L'ajout d'une compacte à FE05 enrichit sa référence d'illustration existante.
Le premier audit comparait le dictionnaire entier et refusait cet ajout attendu.
`finalize_verification.py` vérifie séparément chaque catégorie et confirme
la conservation de l'illustration. Aucun dessin n'a été retouché.

Les compétences non raccordées au combat restent indisponibles ; cette
intégration concerne les dessins approuvés et n'active aucun achat ou effet.

## Preuves locales

`integration-validation.json`, `protected-before.json`, `tests.log`, `build.log`,
`capture-results.json`, `native-icon-matches.json`, `native-selected-icons.png`,
`browser-validation.json` et `browser-proof.png`.

`index.html?skill=FE06` présente les captures réelles du jeu.
