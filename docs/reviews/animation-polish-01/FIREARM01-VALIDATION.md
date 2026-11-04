# FIREARM01-r01 — taille et lisibilité des armes de tir

Le joueur signale des fusils peu lisibles sur le personnage et des armes de tir
à deux mains encore trop grandes. L'inspection confirme que C02-A était toujours
limité à l'ancien aperçu et excluait les fusils. FIREARM01 présente un nouveau
candidat numéroté avant toute activation dans le jeu normal.

Le candidat 01 conserve les sources et les points de prise. Il réduit les
prolongements situés hors du rectangle des deux prises, plus fortement pour
les armes lourdes. Les pixels sources sont échantillonnés sans interpolation
pendant cette préparation ; le compositeur normal conserve son échelle et
ses rotations. Les métaux neutres reçoivent un contraste modéré et une arête
supérieure, sans changer l'alpha, les panneaux colorés ou le bois. L'ombre de
contact reste limitée au corps et respecte les couches avant/arrière.
Les parties masquées par le torse de dos restent masquées.

Les dimensions ci-dessous concernent le dessin source, avant son placement
sur les classes ; elles ne sont pas des tailles finales d'affichage :

- Fusil d'assaut W002 : 46 × 17 → 41 × 17 px.
- Lance-flammes W017 : 54 × 27 → 40 × 19 px.
- Lance-grenades W023 : 52 × 24 → 42 × 18 px.
- Mitrailleuse W031 : 54 × 25 → 38 × 20 px.
- Lance-roquettes W051 : 62 × 19 → 42 × 14 px.

## Vérification exécutée

- `cargo check --locked --bin project-rl` et construction Debug : réussis.
- `cargo check --locked --release --bin project-rl` : réussi. Les branches
  du candidat et son entrée de diagnostic sont réservées au Debug.
- `--ui-cold-firearm-polish .../firearms-r01/current` : réussi, sans charger
  une campagne ni écrire les préférences du joueur.
- 427 contrôles de source : 61 modèles × sept couleurs. Les coordonnées de
  prise et l'alpha de leur rectangle sont conservés, les dimensions opaques
  n'augmentent pas et la source normale est identique après chaque essai.
- 1 120 planches natives : huit armes sur quatre classes en armure lourde,
  quatre orientations et sept phases. Les quatre autres niveaux d'armure
  sont aussi comparés sur la Traqueuse pour W002 et W031.
- 200 comparaisons de pixels confirment que la colonne « actuel » reproduit
  le vrai rendu normal au repos et en marche dans les quatre directions.
- Chaque paire conserve la même scène du camp, la même position et le même
  instant de lumière. Aucun pixel du décor hors de la zone du joueur ne change.
- Les 40 contrôles de retour au repos après la marche sont identiques.
- L'audit du matériau confirme LIGHT01 / Discret des deux côtés.
- La publication n'annote que la bande vide des légendes. Les pixels des
  figures et du décor restent identiques aux captures du moteur.
- Dans le navigateur : Traqueuse très lourde avec fusil, de dos en marche
  ralentie ; Arcaniste avec mitrailleuse de côté ; retour au repos et phase
  manuelle du fusil. Les sélections, le chargement préalable du cycle et
  la lecture passent, sans avertissement ni erreur console.

Les détails sont dans `firearms-r01/current/checks.json`, sous l'aperçu courant.
Le jeu normal et son exécutable Release conservent le rendu précédemment
intégré, dont LIGHT01, C03 et HELM06. L'anatomie des prises et le nouveau gabarit
restent à juger visuellement ; les contrôles de points de prise ne valent pas
acceptation visuelle. La suite complète n'a pas été relancée.
