# Curseur Windows et lancement par Jouer.cmd

Date : 25 octobre 2026.

## Signalement et constat

Le joueur signale un curseur invisible dans le jeu, jusque dans le menu principal,
en particulier après lancement par `Jouer.cmd`.

Le déclencheur exact n'a pas été reproduit de façon systématique : une lecture
native avant correction indiquait un curseur Windows visible et une flèche valide
au-dessus du menu. Une capture du jeu seule ne permet pas de vérifier le curseur
système. Les modifications ci-dessous couvrent les faiblesses observées dans le
code ; leur résultat visuel reste à confirmer sur le lancement habituel du joueur.

## Correction

- `Jouer.cmd` lance directement le programme Release, transmet les arguments et
  conserve son code de sortie. Le lancement intermédiaire par `start` est retiré.
  Le choix de Release existait déjà avant cette correction.
- Le démarrage demande explicitement une souris visible et la flèche standard.
  La boucle de l'application réapplique la visibilité et l'icône appropriée même
  lorsque le choix logique reste identique : flèche, main ou viseur.
- Le backend Windows initialise son curseur avec la flèche système et traite
  aussi cette icône dans `WM_SETCURSOR`. Une ressource indisponible revient à la
  flèche standard.
- La restauration s'applique uniquement lorsque le pointeur est dans la zone
  cliente de la fenêtre du jeu. Les autres fenêtres et les bordures de
  redimensionnement conservent leurs pointeurs système.
- Un compteur `ShowCursor` négatif est rétabli jusqu'à zéro, sans incrémentation
  continue lorsqu'il est déjà visible. La suppression tactile/stylet est respectée.

## Vérifications effectuées

- `cargo check --locked --bin project-rl` : réussi.
- `cargo build --locked --release --bin project-rl` : réussi.
- `cargo fmt --all -- --check` et `git diff --check` : réussis.
- `C:\Python314\python.exe tools/diagnostics/test_windows_cursor.py` : trois
  tests natifs réussis. Ils exercent la fonction de production extraite sans
  modification et les vrais appels user32 : plusieurs masquages successifs,
  stabilité du compteur pendant 120 répétitions, suppression tactile et
  disponibilité des ressources flèche/main/viseur. Le compteur du thread de
  test est restauré à sa valeur initiale.
- `C:\Python314\python.exe tools/diagnostics/test_cursor_launcher.py` : le vrai
  lanceur fonctionne depuis un autre répertoire avec `--verify-assets`, sans
  fenêtre de jeu. La vérification des 573 fichiers du paquet existant réussit
  avec le code 0. Un répertoire de données inexistant produit le code 1 attendu.
  Le manifeste du paquet et les quatre fichiers JSON utilisateur sont inchangés.

Le test natif autonome évite les dépendances de développement de miniquad, qui
n'est pas membre de l'espace de travail Cargo principal. Il ne simule pas
l'affichage graphique complet du jeu. Aucun contrôle d'interface supplémentaire
n'a été effectué après l'interruption physique de l'essai précédent.

Journaux et résultats : `artifacts/cursor-visibility-01/`. Les scripts de diagnostic
sont sous `tools/diagnostics/`. Le jeu ne dépend pas de ces sorties.

SHA-256 du programme Release vérifié :
`99711ed1b5de02355edf14d22efb34fb910cb9622c24890b6ccdcda0d585314e`.

Les animations acceptées, les dessins et les règles de jeu ne sont pas modifiés
par cette correction. Aucune publication ni aucun commit n'a été effectué.
