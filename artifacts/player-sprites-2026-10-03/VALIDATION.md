# Joueur debout — intégration du 3 octobre 2026

## Résultat

- Le joueur utilise la direction approuvée : silhouette humaine debout de trois-quarts, armure ivoire/grise, visière cyan et pieds clairs.
- La pose suit l'arme de l'emplacement actif : mains vides, mêlée ou distance. Elle utilise le type réel d'attaque, sans déduire ce type de la portée ni du numéro d'emplacement.
- Les mouvements horizontaux retournent le sprite, au clavier comme dans les commandes de marche à la souris. Les mouvements verticaux gardent le dernier côté ; aucune rotation ne couche le personnage.
- L'atlas de 72 × 44 pixels contient trois poses à 20 pixels et trois poses à 24 pixels. Il est préparé et chargé au démarrage, puis réutilisé pendant le rendu. La préparation conserve les proportions et aligne les pieds.
- Les couleurs du personnage restent propres au sprite. Les thèmes d'interface ne le recolorent pas.
- Les captures ont révélé que l'indicateur de quête d'un NPC voisin recouvrait les jambes du joueur. Quand une case occupée se trouve au-dessus, l'indicateur devient compact et reste dans la case du NPC ; les marqueurs dégagés conservent leur présentation flottante.
- Les dessins des autres acteurs et des décors restent ceux du rendu existant. Le changement d'apparence ne modifie pas les règles, les ressources, la progression, les commandes ni les structures sérialisées.

## Vérification

- `cargo check --locked --all-targets` : réussi.
- `cargo build --locked` : réussi. L'avertissement préexistant sur `TerminalView::wall_joins` demeure.
- `cargo fmt --all -- --check` et `git diff --check` sur les trois sources modifiées : réussis.
- `cargo test --locked --bin project-rl terminal_view::` : **36 tests réussis**, dont le contrôle des pieds/visières à 20 et 24 pixels et l'absence de chevauchement des marqueurs sur les six zooms.
- `cargo test --locked --bin project-rl player_art` : **2 tests réussis**, couvrant les armes actives, le cas sans arme, les vraies entrées clavier et les commandes de marche à la souris. La sélection de posture et les mises à jour graphiques seules conservent l'état moteur et l'historique.
- Les tests `borrowed_recovery_snapshot_preserves_legacy_bytes_and_fingerprints` et `same_build_snapshot_restores_without_executing_the_journal` : **2 tests réussis**.
- Total : **40 tests distincts réussis**. La suite complète du client n'a pas été relancée ; ses échecs antérieurs sont consignés dans `../neon-integration-2026-10-03/VALIDATION.md`.
- Le PNG source est identique à la sortie imagegen : SHA-256 `2910345D2E4C39F898D817B2CEA7921476EE46E03371FA12D6E4E2BDA0891CC8`.

## Captures natives

- `gallery/cold-start.png` : les trois poses dans les deux sens, agrandissement de l'atlas 24 pixels à 96 pixels, puis tailles réelles 24 et 20 pixels.
- `final/` : huit captures de la matrice mêlée/distance × gauche/droite × 20/24 pixels, dans les thèmes bleu et violet. Tous les diagnostics de présence de texte passent. Ces captures précèdent la correction du marqueur de quête.
- `verified/melee-right-20/cold-start.png` et `verified/ranged-left-24/cold-start.png` : captures finales après cette correction, vérifiées visuellement. La silhouette entière et ses pieds restent visibles près d'Elias ; les diagnostics de présence de texte passent.

`before/` contient les trois sources dans leur état au début de cette intervention.
`integration.patch` compare uniquement cette intervention à ces copies et inclut
le nouveau module, le PNG et sa note de provenance. Les changements antérieurs de
l'espace de travail ne sont pas inclus dans ce patch. Aucun commit ni push.
