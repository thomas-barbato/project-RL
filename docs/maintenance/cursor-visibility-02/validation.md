# Curseur visible avec Jouer.cmd — 31 octobre 2026

Le joueur signale un pointeur invisible dans le jeu et sur le menu principal,
mais visible ailleurs. Il précise utiliser `Jouer.cmd`.

Le lanceur du projet exécutait déjà directement la version Release depuis
le dossier du projet. La précédente restauration native du 25 octobre est
toujours présente. Le diagnostic de cette reprise constate une flèche valide,
`CURSOR_SHOWING` actif et un compteur de visibilité nul lorsque le jeu est au
premier plan. Le déclencheur exact de l’invisibilité rapportée n’est donc pas
reproduit de façon déterministe. Il ne faut pas présenter ce constat comme une
cause prouvée dans le script ou dans le pilote graphique.

## Correction du lancement habituel

`Jouer.cmd` active désormais `PROJECT_RL_SOFTWARE_CURSOR=1` dans son environnement
local. Le jeu prépare trois images à partir des ressources Windows : flèche,
main et viseur, avec leurs dimensions et points actifs. Il les dessine en dernier,
dans les coordonnées de la fenêtre, sur le menu, les dialogues, les interfaces,
la carte et l’écran d’erreur de démarrage. L’affichage ne dépend plus de la
présentation matérielle du curseur natif pour ce lancement.

La position vient du pointeur Windows actuel, converti dans le client actif,
pour conserver son alignement dès le démarrage et après Alt+Tab. Le curseur
dessiné est absent lorsque l’application n’est pas au premier plan, hors de
la zone cliente, ou lors de la suppression tactile signalée par Windows.
Le pointeur natif du jeu est masqué pour éviter un doublon ; les autres
applications gardent leur gestion habituelle.

Les trois textures sont préparées sur le thread graphique avant affichage,
conservées dans un cache fixe et ne sont pas recréées pendant le dessin.
Si la préparation échoue, le rendu natif antérieur est conservé. Le mode reste
désactivable avec la variable à `0` ; sans cette activation, les autres points
d’entrée gardent le curseur natif. Aucune option joueur supplémentaire, dessin
d’armure, règle de jeu ni sauvegarde n’est modifié.

L’instrumentation temporaire du backend Windows a été retirée. Ce correctif
n’ajoute aucune modification au fournisseur miniquad. La dépendance Windows
directe `winapi` réutilise la version 0.3.9 déjà verrouillée.

## Contrôles

- Test natif de `software_cursor` : extraction réelle des trois ressources,
  pixels visibles, transparence et points actifs dans les limites — réussi.
- `tools/diagnostics/test_windows_cursor.py` : les trois contrôles natifs de la
  restauration antérieure passent.
- `cargo check --locked --bin project-rl` : réussi.
- `cargo build --locked --release --bin project-rl` : réussi avant essai graphique ;
  une dernière reconstruction inclut la lecture de la position native courante.
- Le vrai `Jouer.cmd --verify-assets` fonctionne depuis un autre dossier :
  code 0 sur le paquet existant et code 1 attendu sur des données absentes.
  Le manifeste du paquet et les quatre fichiers JSON utilisateur sont inchangés.
- Le vrai `Jouer.cmd --laboratoire-competences` lance le laboratoire indépendant :
  le curseur est visible sur la carte. Aucune partie de campagne n’est chargée.

Les captures et journaux de cette reprise sont dans
`artifacts/cursor-visibility-01/`. La suite complète n’a pas été relancée.

Les ressources natives sont copiées avec
[DrawIconEx](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawiconex),
qui applique le masque ou l’alpha de l’icône Windows. Le compteur de visibilité
reste géré par le backend existant selon
[ShowCursor](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcursor).
