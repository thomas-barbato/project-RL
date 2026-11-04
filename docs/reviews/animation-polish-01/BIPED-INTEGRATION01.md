# BIPED-INTEGRATION01 — dix cycles intégrés

Le joueur autorise « tu peux integrer :) » après la validation des dix modèles
MON-005, 014, 039, 144, 145, 146, 151, 156, 164 et 165. L'intégration reprend
uniquement la variante 0 et la palette d'origine montrées. Les autres monstres,
variantes de couleur, déplacements haut/bas, classes et équipements du joueur
gardent leurs chemins existants.

Les huit dessins mobiles par direction sont repris pixel pour pixel depuis
les sources acceptées. La pose 8 installée utilise le vrai frontal montré au
repos : frontal conservé explicite lorsqu'il existe, sinon frontal VERT18 du
jeu utilisé par la revue. La neuvième pose générée et non montrée n'est jamais
substituée à ce frontal. Le fusil et la hache des cultistes conservent leurs
deux prises ; le brassard du démon pâle conserve son côté anatomique.

Les ressources du jeu sont dans `assets/animations/biped-lateral-01/`, avec
un manifeste indépendant des sources de travail et des captures. Leur
provenance et leurs empreintes sont dans `BIPED-INTEGRATION01-ASSETS.json`.
Les accords visuels historiques restent figés et distincts de cet accord
d'intégration.

Le rendu normal utilise quatre phases pondérées par pas, puis quatre sur le
pas opposé. Le déplacement d'une case dure 400 ms ; le cycle complet couvre
deux pas, 800 ms. L'arrêt utilise le frontal réel, sans pause de démonstration
ajoutée au jeu. Le mode de mouvement réduit utilise également le frontal.
Le réglage personnel était encore à 200 ms : seul `walk_duration_ms` est passé
à 400, conformément à la cadence validée ; les autres préférences sont gardées.
Les autres vitesses restent disponibles dans les réglages existants.

Les images sont décodées sur le préparateur commun avant le premier affichage,
avec anticipation des acteurs proches. Les textures utilisent le cache borné
commun et sont créées sur le thread graphique. Le dessin ne lit ni ne décode
les PNG. Les identités, règles, sauvegardes et états de zone restent indépendants
de ce cache de présentation.

Vérifications natives : 160 poses mobiles identiques aux sources acceptées,
38 empreintes de sources figées conservées, 660 captures identiques entre
pose validée et rendu normal, 120 vues verticales inchangées. Les 20 arrivées
des deux directions utilisent le même frontal. Aucun chargement, décodage ou
envoi de texture n'a lieu pendant ces dessins ; le retour à chaud attend 0 ms.
La capture utilise les vrais glissements de case de CombatMotion et vérifie
les 64 px parcourus par pas de 400 ms, avec caméra centrée sur l'acteur.

Les tests ciblés des bipèdes et les 19 tests de déplacement passent.
Les compilations Debug et Release, rustfmt ciblé, compilation Python et
inventaire portable sont vérifiés. Les détails de lancement, distribution et
préservation des données personnelles figurent dans le bilan d'intégration.

Preuves locales : `artifacts/animation-polish-01/biped-integration-01/`.
Capture : `--ui-cold-biped-integration OUTPUT`, puis
`tools/art/check_biped_integration.py`. Les captures ne sont pas des dépendances
du jeu.

Les deux lanceurs réels, celui du projet et celui de la distribution portable,
passent `--verify-assets` depuis un autre répertoire. Une racine de données
absente échoue comme prévu. Ces diagnostics n'ont modifié aucune sauvegarde.
Pendant le lancement graphique normal ultérieur, `city-test-run.json` est
actualisé ; cette version est conservée sans restauration. Les autres
sauvegardes sont identiques. Le réglage de cadence est la seule préférence
modifiée. Le client quitte normalement avec le code 0. Windows a refusé
l'activation de sa fenêtre pour la capture du menu : son apparence n'est pas
confirmée. La capture occultée a été écartée et ne sert pas de preuve du menu.
