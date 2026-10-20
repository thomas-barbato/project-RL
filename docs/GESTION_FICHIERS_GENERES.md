# Sources, validations et fichiers générés

Les dessins utilisés par le jeu appartiennent à `assets/`. Les animations de
l’agent sont dans `assets/animations/agent/`, avec les identifiants de revue
conservés. Les originaux et dessins de fabrication supplémentaires sont dans
`assets/source-art/`. Un déplacement ne change ni leurs pixels ni leur validation.

Les décisions, prompts, manifestes de fabrication et bilans conservés sont dans
`docs/reviews/`. Les références historiques à un ancien chemin `artifacts/`
peuvent être retrouvées dans le registre du nettoyage. Un ancien dessin refusé
ou remplacé reste une archive et ne devient pas une référence approuvée.

Les scripts historiques conservés sont dans `tools/archive/`. Ils documentent
la fabrication passée ; leurs anciens dossiers d’entrée et de sortie ne sont
pas une installation autonome. Avant de les réutiliser, retrouver leurs
entrées dans le registre et choisir un dossier temporaire de sortie.

`artifacts/` accueille les captures, diagnostics et aperçus reproductibles.
Après validation et intégration, conserver les sources et le bilan utile,
puis supprimer ces sorties. Réutiliser la page `apercu-animations.html` plutôt
que d’accumuler les copies. Les nouveaux fichiers temporaires sont ignorés par
Git. Les fichiers déjà suivis ne sont pas retirés de l’historique par cette règle.

Exception actuelle : `artifacts/map-editor/` contient les cartes, préférences et
récupérations de l’éditeur. Ce sont des données de travail à conserver. Ne pas
vider ce sous-dossier avec les captures.

`target/` est entièrement reproductible par Cargo. Il contient aussi les
exécutables : un nettoyage complet impose une recompilation avant de jouer.
Le nettoyage du 16 octobre conserve les exécutables principaux et supprime les
autres sorties après contrôle. Les profils de développement et de test
désactivent les informations de débogage et la compilation incrémentale pour
limiter la croissance des caches. Les contrôles d’exécution Rust restent actifs.

Le registre de cette opération est dans
`docs/maintenance/nettoyage-2026-10-16/`. Il contient les correspondances de
chemins, les empreintes des sources préservées et le résultat réel du nettoyage.
L’historique Git et les sauvegardes du joueur ne font pas partie du nettoyage.

## Camp et dialogues — rangement du 18 octobre 2026

Le camp et la bulle terminal fonctionnent dans le lancement normal. Les anciens
générateurs, exemples et lanceurs de cette phase sont rangés sous
`tools/archive/handcrafted-places-review-01/` ; les anciennes ressources de
prototype restent sous `assets/source-art/`. Les captures intermédiaires et
anciens exécutables sont envoyés dans la Corbeille, sans vidage.
L’aperçu courant est réduit aux références utiles et propositions encore en
attente. Les contrôles et correspondances sont dans
`docs/maintenance/nettoyage-2026-10-18/camp-dialogues/`. Les modules actifs de
dialogue sont `src/terminal_dialogue.rs` et `src/terminal_dialogue_app.rs`.

## Animations — nettoyage du 20 octobre 2026

Les doublons des anciennes comparaisons, les captures individuelles des 68
compétences et les exécutables de test sont retirés du projet. L’aperçu courant,
ses données, les sources, les accords et le bilan de validation sont conservés.
Quatre comparaisons natives suffisent au bilan visuel. Les anciens caches de
compilation liés à la configuration audio abandonnée sont également retirés.

Les captures et rapports automatiques sous `docs/reviews/` et
`docs/maintenance/` sont maintenant ignorés par Git ; les décisions, règles,
accords et synthèses de validation restent versionnables. Le registre local est
`docs/maintenance/nettoyage-2026-10-20/cleanup-result.json`.

La suppression groupée ayant été refusée par le contrôle automatique, les 262
fichiers sont archivés de façon réversible dans
`C:\Users\User\.codex\archives\project-RL\cleanup-2026-10-20`, avec leurs
empreintes et chemins d’origine. Cet archivage allège le projet, sans prétendre
libérer cette place sur le disque.
