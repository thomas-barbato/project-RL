# Voirie et décors natifs — 3 octobre 2026

## Changements

- Goudron sombre pour les voies et chaussées, sans réintroduire de bordures de cases.
- Pointillés centrés dans les rues larges, suivant leur axe. Ils s'interrompent
  aux carrefours et ne déduisent pas les limites à partir de cellules inconnues.
- Silhouettes urbaines et industrielles de huit pixels vues de dessus, avec
  trois tons de matière et une petite ombre restant dans la case.
- Neuf nouveaux glyphes : baril, palette chargée, ventilation, armoire électrique,
  borne incendie, borne de voirie, grille d'évacuation, bobine de câbles, paillasson.
- Retrait des T et enseignes abstraites au sol. Leurs anciens emplacements
  deviennent de la chaussée, quelques grilles, ou un paillasson devant une boutique.
- Même occupation du terrain et mêmes passages bloqués ou traversables ; la
  variété visuelle dépend des coordonnées et ne consomme aucun hasard du moteur.
- Les variantes de `Decor` sont ajoutées à la fin de l'énumération. Les anciens
  caches mémorisés sont interprétés sans remettre les signes supprimés et sans
  actualiser le terrain caché. Les noms figurent au survol et dans F1 > Symboles.

## Vérification

- `cargo fmt --check`, `git -c core.safecrlf=false diff --check` : OK.
- `cargo check --locked --all-targets` : OK.
- `cargo test --locked --bin project-rl terminal_view::` : 30 tests réussis,
  dont géométrie et confidentialité des marquages, empreintes des obstacles,
  cache binaire des anciens symboles, mémoire, caméra, cercle et effets.
- Tests supplémentaires réussis :
  `borrowed_recovery_snapshot_preserves_legacy_bytes_and_fingerprints`,
  `city_feedback_new_starts_see_elias_inside_the_randomly_placed_town`,
  `city_feedback_old_starts_and_loot_remain_unchanged`.
  Au total, 33 tests ciblés ; ce n'est pas une exécution de toute la suite.
- Compilations debug et release : OK. Deux avertissements release préexistants
  concernent `arsenal_app` et des méthodes non utilisées.
- Capture GPU `vision-circle-memory` : les pixels périphériques rejoignent
  bien le ton du sol mémorisé, sans carrés noirs.
- Captures natives relues : catalogue des 25 décors, sols, rue complète, fenêtre
  960 × 540 avec zoom 24, guide des symboles et bas du guide en petite fenêtre.
- L'exécutable release démarre sans erreur, contrôlé six secondes au menu.
  Les captures de terrain sont issues du debug.

## Fluidité

Le premier diagnostic `performance` a tourné pendant la compilation release,
avec des temps irréguliers. Le diagnostic `performance-isolee` a été relancé
après cette compilation pour lever ce doute.

Mesure debug finale : 120 images par scène, dix scènes, sans modification de
l'empreinte de simulation. Monde : CPU médian 7,71 ms, 95e percentile 11,37 ms ;
image avec présentation médiane 10,00 ms. Effet de feu : CPU médian 10,31 ms.
La première image du monde coûte 193,88 ms, comprenant notamment la préparation
de l'atlas. Ces chiffres ne constituent pas une comparaison contrôlée avec
la version précédente, ni une mesure du client release.

## Captures

- `glyphes-v1/cold-start.png` : agrandi à 64 px, puis tailles de 32 et 24 px.
- `sols-v1/cold-start.png` : goudron et exemple de pointillés.
- `rue/cold-start.png` : rue réelle dans la ville intérieure.
- `rue-960-24/cold-start.png` : même rue en petite fenêtre.
- `symboles-bas-960/cold-start.png` : neuf nouveaux décors dans l'aide.
- `memoire-v1/vision-pixels.json` : contrôle GPU du cercle.
- `performance-isolee/timings.json` : mesures finales.

Diagnostics reproductibles, chacun avec un nouveau dossier de sortie :
`--ui-cold-city-decor-palette`, `--ui-cold-floor-palette`,
`--ui-cold-road-city`, `--ui-cold-ux-symbols-bottom-small`.
