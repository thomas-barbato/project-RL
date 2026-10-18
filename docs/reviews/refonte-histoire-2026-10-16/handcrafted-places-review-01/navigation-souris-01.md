# WALK-VIS-01 — Déplacement à la souris et aperçu A*

Règles acceptées le 18 octobre 2026 ; rendu E corrigé accepté et intégré sous
l'identifiant WALK-VIS-03-E le même jour.

- Une compétence sélectionnée garde la priorité : clic gauche pour confirmer,
  clic droit ou Échap pour annuler.
- Sans compétence, le clic gauche sur un ennemi garde l'attaque ordinaire ;
  les interactions existantes avec les PNJ et le décor sont conservées.
- Sur le sol libre adjacent, un clic avance d'une case et consomme un tour.
- Pour une destination plus éloignée, un premier clic prépare un trajet A*
  gratuitement. Un second clic sur la même destination confirme ce trajet.
- Cliquer une autre destination remplace l'aperçu, sans confirmer le premier.
- Le clic droit ou Échap efface l'aperçu sans déplacement, dépense ni menu
  contextuel. Le clic droit peut aussi interrompre le déplacement commencé.
- Le trajet confirmé passe par les commandes de déplacement normales : un
  tour par case. L'arrêt existant sur blessure, obstacle ou nouvel ennemi
  visible est réutilisé.
- Le clavier et sa touche contextuelle configurable restent disponibles.

Le déplacement libre au clic prend la priorité sur l'ancien amorçage automatique
du tir de zone en cliquant le sol. Les armes de zone restent utilisables par
leur visée explicite existante ; une visée déjà ouverte se confirme au clic.
Cette priorité est maintenant activée dans la partie normale.

## Dessin proposé

Un tracé bleu holographique fin, souligné de sombre, relie les centres des cases.
Des chevrons donnent le sens ; chaque pas possède un petit cercle numéroté.
La destination porte la dalle biseautée E et ses deux petits repères circulaires,
avec un intérieur dégagé : le chemin s'arrête avant la dernière case.
Le numéro final donne le nombre
de cases à parcourir. Le trait démarre hors de la silhouette du joueur.
Le sol et les personnages gardent leurs dessins et leur échelle.

Le trajet passe par l'A* déterministe existant, sur les cases visibles,
praticables, libres de personnage et sans danger déjà identifié. Il ne montre
aucun terrain caché. Il est recalculé à la confirmation ; s'il a changé, le
nouveau trajet est présenté au lieu de démarrer un autre itinéraire en silence.

## Revue et intégration

Les commandes et le rendu intégrés sont dans `src/mouse_navigation.rs` et
`src/ascii_app.rs`. Aucun dessin de `artifacts/` n'est chargé par le jeu.
`src/walk_preview_review.rs` ne contient plus qu'un essai et des contrôles natifs
du même code de production ; l'ancien prototype est archivé.
L'essai interactif se lance avec `cargo run -- --apercu-deplacement-souris`.
La capture native utilise `--ui-cold-walk-preview <dossier>`.
La galerie courante est `artifacts/walk-preview-review-01/index.html`.
Les images ont des cases de 64 px sans agrandissement.

Les contrôles sont consignés dans `navigation-souris-01-native.json`.
