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

## Assets — optimisation du 21 octobre 2026

Le paquet joueur est construit par `Construire_distribution.cmd` à partir de
`tools/distribution/runtime-assets.json`. Il exclut les sources de fabrication,
les revues, les captures et les caches. Voir `docs/DISTRIBUTION.md`.

Après validation, les captures ordinaires, les diagnostics et les programmes
de test devenus inutiles sont à retirer ; ils n’ont pas à former une nouvelle
archive. Conserver le bilan utile, les sources, les décisions et les données
de travail de l’éditeur. Le cache Debug peut être nettoyé avec Cargo après les
contrôles. Le nettoyage actuel conserve le Release et le paquet autonome.

Les sorties jetables précédemment archivées le 20 octobre sont elles aussi
retirées. Les suppressions définitives étant bloquées par le contrôle automatique,
ces fichiers passent par la Corbeille ; le bilan distingue la réduction du
projet de l’espace effectivement libéré sur le disque.

## Classes, lisibilité et armes — nettoyage du 22 octobre 2026

Les cinq classes, les aides de lisibilité choisies et les six dessins d’armes
validés sont intégrés. Les anciens aperçus et les exports bruts de cette phase
sont retirés après reconstruction du Release. L’aperçu courant reste dans
`artifacts/weapon-readability-review-01/` ; les anciennes pages donnent accès
à cet aperçu et aux bilans conservés. Les sources et les banques du jeu restent
dans `assets/`, les comparaisons natives utiles dans `docs/reviews/`.

Les empreintes RGBA des 3 100 compositions de référence remplacent leurs
anciennes planches de diagnostic. Pour relancer le contrôle, réexporter les
compositions actuelles avec `class_armor_native_diagnostic`, puis utiliser
`tools/art/verify_weapon_readability_integration_03.py`. Le contrôle ne dépend
plus des anciennes planches supprimées. Les captures du moteur utiles à
l’aperçu courant restent conservées.

Le cache Debug est nettoyé avec `cargo clean --profile dev` ; le Release et
`Jouer.cmd` restent disponibles. Les anciennes sorties passent par la Corbeille,
sans vidage. Les données de `artifacts/map-editor/` sont contrôlées et conservées.
Registre : `docs/maintenance/nettoyage-2026-10-22/armes/`.

## Marche des créatures — rangement du24 octobre2026

Après correction de M100/M104 et intégration des130 cycles de base, les anciens
aperçus de marche, exports intermédiaires et paquet de diagnostic sont retirés
du projet vers la Corbeille. Les sources restent sous `assets/source-art/`,
les atlas utilisés par le jeu sous `assets/animations/walk-07/`, et quatre
captures utiles du moteur avec les accords et bilans sous
`docs/reviews/player-walk-07/integration/`.

Les compositions et données de revue d'armes de joueur encore utiles sont
conservées dans `artifacts/player-walk-review-07/recipe-final/` et les entrées
joueur de `preview-data/`. Les données de `artifacts/map-editor/` gardent leurs
empreintes. La compilation Release reste disponible. Le bilan distingue les
octets retirés du projet de l'espace disque, non libéré par un envoi dans la
Corbeille : `docs/reviews/player-walk-07/integration/cleanup-result.json`.

## Marche et casques — nettoyage du25 octobre2026

À la demande du joueur, les anciens exports, atlas de diagnostic, comparatifs
intermédiaires et programmes de contrôle sont retirés après validation du Release.
Les captures répétées de la rue et les anciens catalogues de prises sont réduits
à quelques vues finales représentatives. Les sources graphiques et les banques
utilisées par le jeu restent dans `assets/`, avec leurs empreintes conservées.

La revue des casques conserve un seul aperçu d'ensemble des six modèles sur les
cinq classes. Les commandes de régénération remplacent les liens vers les grandes
planches et GIF individuels retirés. Les accords, règles, bilans et propositions
encore en attente restent disponibles dans `docs/reviews/` et `assets/source-art/`.

Les données de `artifacts/map-editor/`, la référence documentaire d'itemisation,
l'aperçu d'animations courant et la proposition de trajet souris sont conservés.
Les anciens exports passent dans la Corbeille, sans vidage. Le cache Debug est
supprimé avec `cargo clean --profile dev` ; `Jouer.cmd` conserve son Release prêt
à lancer. Le prochain lancement Debug ou d'un outil Debug recompilera son cache.

Périmètre, empreintes et contrôles : `docs/maintenance/nettoyage-2026-10-25/`.
