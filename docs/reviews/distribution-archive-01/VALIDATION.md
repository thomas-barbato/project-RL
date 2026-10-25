# DIST-PACK-01 — Archive de distribution — 25 octobre 2026

Les versions distribuées utilisent désormais `data.rlpak`, une archive ZIP
standard, lue directement par le jeu. Les ressources externes, les catalogues
et les mods livrés sont regroupés ; les fichiers déjà incorporés à l’exécutable
y restent. Les sources de développement, dessins, règles, textes et sauvegardes
ne sont pas modifiés par cette opération.

La préparation conserve l’inventaire explicite, les alias de PNG identiques et
la recompression PNG sans perte. Les entrées ZIP sont compressées uniquement
quand cela réduit leur taille. Le manifeste de distribution passe en version 2 ;
le lecteur et l’outil de reconstruction restent compatibles avec les anciens
paquets en fichiers séparés.

Le lanceur `Construire_distribution.cmd` vérifie maintenant l’exécutable avec
`--verify-assets` dans le dossier de préparation avant de publier le paquet.
Une erreur d’entrée laisse l’ancien paquet intact. Les fichiers utilisateur ou
les fichiers gérés modifiés continuent à bloquer la réutilisation d’une sortie.
Les mods supplémentaires restent dans le dossier externe `mods/`.

## Vérifications effectuées

- `cargo test --locked --lib data_paths::` : **6 tests réussis**. Lecture des
  archives et alias, limite de taille, développement et ancien format, données
  corrompues, inventaire incorrect et absence de repli sur des fichiers séparés.
- Le test de catalogues compare l’intégralité du `LoadedContent` avec celui
  chargé depuis le dépôt : mêmes identités, règles, textes, ordre des paquets et
  catalogues. Il couvre aussi les mods supplémentaires en fichiers séparés.
- `cargo test --locked --lib content::` : **55 tests réussis**.
- Tests Python de distribution : **5 tests réussis**. Migration d’un ancien
  paquet, contenu des PNG, alias, reproductibilité, ajout de ressources,
  protection des fichiers utilisateur et conservation du paquet précédent
  lorsqu’une nouvelle entrée est invalide.
- `cargo fmt --all -- --check`, `git diff --check`, compilation Release et
  préparation réelle avec `python -B tools/distribution/package_release.py
  --build` : réussis.
- `cargo check --locked --tests --bin project-rl` : réussi, sans avertissement.
- Une copie temporaire hors du dépôt, lancée depuis un autre répertoire de
  travail, a réussi `--verify-assets` puis `--smoke-assets` dans le moteur natif.
  Elle contenait seulement l’exécutable, l’archive, le manifeste, le lanceur et
  un dossier de mods vide. Aucune copie externe de `assets/` ou `content/`.
- Contrôle natif : **1 758 entrées**, **1 368 PNG décodés et envoyés au GPU**,
  **7 atmosphères**, menu, catalogues et son de dialogue chargés. Le partage de
  texture d’animation est vérifié. Les préférences de ce contrôle utilisent un
  profil temporaire, indépendant des préférences et sauvegardes du joueur.
- Sur cette copie, le retrait de l’archive puis sa troncature sont tous deux
  rejetés par `--verify-assets`. La copie temporaire est retirée après contrôle.

Les détails et empreintes du paquet testé sont dans `validation.json`.

## Mesures du paquet actuel

- 1 813 chemins logiques, dont 55 alias.
- Entrées après dédoublonnage et recompression PNG : **105 888 970 octets**.
- Archive : **99 392 004 octets**, soit **94,8 Mio** ; l’empaquetage ZIP réduit
  de **6,1 %** la taille des entrées après ces optimisations.
- Aucun changement de résolution, compression avec perte ou chiffrement.

Ce contrôle valide le chargement et le paquet portable. Il ne constitue pas
une mesure des performances en partie ni une nouvelle validation visuelle des
dessins. Les banques incorporées à l’exécutable ne sont pas migrées dans cette
étape. Le guide de maintenance est `docs/DISTRIBUTION.md`.
