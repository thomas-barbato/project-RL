# Contrôles de la proposition r04 — quartier ravagé

19 octobre 2026. Candidat non activé dans la partie normale. Ces contrôles
techniques ne remplacent pas la validation visuelle du joueur.

- Six bâtiments supplémentaires, 57 placements de décors et dégâts, tous
  conditionnés à leur taille native. Les deux épaves occupent au plus 128 × 64 px.
- Les routes continues r03 sont conservées. Les nouvelles empreintes retirent
  les cases recouvertes par les bâtiments ; les gros décombres, impacts et
  épaves créent 49 obstructions basses supplémentaires. Les 718 cases libres
  sont toutes reliées au départ, et les seuils protégés restent accessibles.
- Les pixels du portail sud sont identiques à ceux de r03. Les empreintes de
  tous les fichiers de `assets/maps/prison-street-01/` sont inchangées.
- Les sources lumineuses des six nouvelles maisons suivent les lentilles
  réellement présentes dans leurs façades. Les fragments d’une même lentille
  sont fusionnés en un seul émetteur. Leurs portes condamnées ont une lumière
  rouge ; les trois portes utilisables gardent leur lumière jaune et leurs toits.
- `cargo fmt --all -- --check` : réussi.
- `cargo build --locked` : réussi.
- `cargo test --locked --bin project-rl prison_street -- --test-threads=1` :
  **8 tests réussis**, aucun échec. Le test du candidat inclut maintenant ses
  obstructions de décor dans la carte vérifiée.
- `cargo check --locked --release` : réussi.
- `git -c core.safecrlf=false diff --check` : réussi.
- Capture isolée du moteur : **17 vues** avec déplacement réel, puis retour
  réel au camp. Ouverture des deux maisons r03, masquage de leurs toits et
  échelle de cellule 64 × 64 contrôlés ; le rendu ne change pas la simulation.
- Trois animations conservées, chacune 24 images de 640 × 384 px : ventilation,
  vapeur et braises. La fabrication des GIF vérifie un changement d’image réel.

La page courante réutilisée est `artifacts/street-expansion-review-01/index.html`.
`candidate-r04.json` et `render-r04.json` conservent les preuves et empreintes.
Les vues de détail sont des captures GPU. Le plan complet est une composition
numérique des mêmes ressources avec l’éclairage, sans brouillard de visibilité.
Les planches de bâtiments montrent les sources à leur taille native, avant
le composite lumineux du moteur, avec le personnage à la même échelle.

La suite complète du projet n’a pas été relancée. Les contrôles ci-dessus
portent sur le périmètre carte, déplacement, portes, toits, reprise et rendu.
