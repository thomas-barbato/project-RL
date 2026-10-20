# Contrôles techniques de la proposition r03

19 octobre 2026. Le rendu reste candidat : ces contrôles ne remplacent pas
la validation visuelle du joueur.

- `build_street_unified_review.py` : géométrie reliée, 886 cases praticables,
  trois bâtiments visitables, bord logique fermé, largeur de l’avenue vérifiée
  à cinq hauteurs. Le portail sud conserve exactement les pixels d’origine.
- `cargo fmt --all -- --check` : réussi.
- `cargo build --locked` : réussi.
- `cargo test --locked --bin project-rl prison_street -- --test-threads=1` :
  **8 tests réussis, aucun échec**. Les tests concernent notamment les trajets,
  les portes, les toits, la compatibilité de reprise et le nouveau plan relié.
  Ils ne constituent pas un passage de la suite complète du projet.
- `cargo check --locked --release` : réussi.
- `git -c core.safecrlf=false diff --check` : réussi.
- `--ui-cold-street-expansion` : **14 vues du moteur**, avec déplacement réel
  vers chaque point, et retour réel au camp. Le rectangle du personnage reste
  de 64 × 64 px. Les ouvertures des nouvelles maisons et le masquage des toits
  sont contrôlés. Les rendus ne modifient pas l’empreinte de la simulation.
- Trois clips de 24 images, 640 × 384 px sans agrandissement : ventilation,
  vapeur et braises. Le conditionnement vérifie que les images changent.
- Les empreintes des fichiers de la carte active sont identiques à celles
  enregistrées avant fabrication. Le mode normal conserve la rue approuvée.

L’aperçu réutilise `artifacts/street-expansion-review-01/index.html`.
`render-r03.json` contient les résultats du moteur ; `candidate-r03.json`
contient les empreintes et le bilan géométrique. Les trois gros plans des
jonctions sont des captures GPU, tandis que le plan complet est une
composition numérique des mêmes ressources, sans le brouillard de visibilité.
