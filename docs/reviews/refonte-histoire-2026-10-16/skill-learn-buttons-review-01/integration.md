# LEARN-BTN-01 et SKILL-DEMO-01 — intégration du 18 octobre 2026

## Accords du joueur

**LEARN-BTN-01, Plaque biseautée**, accepté :

> je crois que la premiere est la plus simple et efficasse

**SKILL-DEMO-01**, exemple compact et fixe au rang 1, accepté pour toutes les
compétences actives :

> oui ca me convient, tu dois le faire pour toutes les compétences actives :)

Les précisions suivantes autorisent le rendu intégré de révision 03 : nom de la
compétence centré sans « DÉMONSTRATION » ; scène directement sous les icônes,
dans la zone centrale, sans panneau supplémentaire ; suppression d’« Utiliser » ;
« Fermer » sous les six raccourcis, dans le bas de la fenêtre de compétences.
Les citations exactes sont conservées dans `review.json`.

## Fonctionnement dans le lancement normal

- Le bouton + apprend ou améliore uniquement la compétence. Un appui achète au
  maximum un rang, avec les coûts et restrictions du moteur. Le plafond concerne
  les rangs appris ; les bonus d’équipement continuent à fonctionner.
- Le clic sur une image sélectionne gratuitement la compétence. Pour chacune des
  **68 actives**, il affiche un exemple animé de base. Les **26 passifs** ne
  montrent aucune scène.
- Le glisser-déposer d’une active apprise vers la barre reste disponible. Un
  relâchement sans déplacement n’achète aucun rang.
- « Fermer » est centré sous les six emplacements. Le bouton « Utiliser » est
  retiré. L’activation au clavier et depuis un raccourci appris reste disponible.
  La barre affichée pendant le jeu garde sa disposition et son bouton d’arme.
- La scène occupe au maximum 480 × 222 unités d’interface, dans la zone centrale.
  Seul le nombre de rangées d’icônes visibles s’adapte ; les branches et la fiche
  descriptive gardent leur hauteur. Le nom de la compétence est centré.

## Exemples et isolation

Les 68 exemples exécutent les commandes réelles, avec les rendus existants. Les
armes, cibles, blessures, brûlures, gel ou drones requis sont préparés dans une
simulation indépendante. Les protections montrent une attaque ennemie, les
transformations une action de leur forme, les invocations leur créature, les
soins une blessure initiale. Les règles chiffrées des compétences sont inchangées.

Chaque boucle repart du même état de rang 1, indépendamment du niveau appris ou
des bonus du joueur. Un seul simulateur et une surface de rendu sont réutilisés.
Les grands effets sont cadrés dans la miniature ; le monde et ses sources restent
en 64 px. Aucun nouveau dessin de personnage ou de monstre n’est ajouté.

Les points, tours, ressources, historique et générateur aléatoire de la partie
hôte restent inchangés. La simulation ne charge ni n’écrit la sauvegarde de la
partie. L’intégration est disponible dans les compilations debug et release.

## Vérifications

- 31 tests ciblés de fenêtre et barre : achat séparé, appui unique, clic gratuit,
  glisser-déposer, raccourcis, passifs, fermeture sans action, géométrie jusqu’à
  2560 × 1440 et 3840 × 2160.
- Un test moteur parcourt les 68 actives au rang 1 et leurs actions de suite,
  rejette les passifs et vérifie les exemples de mine et réanimation.
- Un test de la fenêtre normale rejoue les 68 actives, contrôle la réutilisation
  du simulateur, la restauration des boucles, l’isolation de la partie et
  l’identité de l’exemple après quatre achats de rang dans la partie hôte.
- Captures natives de toutes les actives, des deux habillages et d’un passif.
  Les animations sont vérifiées au décodage, pixel par pixel et pour leur durée.
  Le bilan et les empreintes sont dans `integration-native.json`.
- Les 16 régions de référence du bouton accepté restent documentées dans
  `button01-baseline.json` ; la précédente preuve de revue est conservée dans
  `integration-native-r02.json`.

Le passage plus large antérieur avec le filtre `skill_` donnait **210 réussites et
12 échecs** dans les anciennes propositions de flammes du laboratoire. Ces
fixtures dépendent encore de `FE02.cone_range_cells`, retiré lors du passage
accepté de Souffle de feu au couloir. Elles sont dans
`src/agent_skill_lab_review/fire_painted/obstacles/` et n’ont pas été modifiées ici.
Ce bilan ciblé ne constitue pas une réussite de la suite complète.

## Sources et captures

- Interface : `src/agent_skill_window.rs`, `src/agent_skill_bar.rs` et
  `src/skill_learn_button.rs`.
- Simulation de base : `src/game/agent_skill_demo.rs`.
- Rendu normal : `src/skill_demo.rs`.
- Capture de l’interface intégrée : `src/skill_demo_review.rs`.
- Conditionnement : `tools/art/package_skill_demo_review.py`.
- Aperçu courant : `artifacts/skill-learn-buttons-review-01/index.html`.

Aucune ressource de compilation ne dépend d’`artifacts/`. Les trames brutes sont
supprimables après vérification des animations ; les sources, accords et bilans
durables sont conservés. `artifacts/map-editor/` n’est pas concerné.

```powershell
cargo run --locked
target/visual-review/project-rl.exe --ui-cold-skill-demo artifacts/skill-learn-buttons-review-01
python tools/art/package_skill_demo_review.py
```

Compilation debug, contrôle release, formatage et `git diff --check` : réussis.
Le warning release déjà existant sur `draw_inspection_panel` inutilisé reste présent.
Les 68 animations regroupent 1 632 trames natives vérifiées.

## SKILL-DEMO-FRAME-01 — contour demandé

Le joueur demande : « tu pourrais ajouter une bordure autour de la "scene" de demonstration ? ». Un contour de 1 unité d’interface, dans la couleur d’accent de l’habillage avec une opacité de 65 %, entoure uniquement la scène. Le titre reste centré au-dessus. La disposition, les dimensions et les règles des exemples sont conservées.
Compilation et captures des deux habillages vérifiées. Les 68 animations ont été actualisées et leurs 1 632 trames vérifiées au décodage. Les trames brutes et les deux captures de comparaison ont ensuite été envoyées dans la Corbeille ; les images courantes et la preuve frame-native-01.json sont conservées.
