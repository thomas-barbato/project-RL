# Respiration intégrée et proposition terminal — 5 octobre 2026

La respiration approuvée dans la galerie des prises est active dans le plateau.
La refonte du HUD reste une proposition réservée aux diagnostics, à juger avant
son intégration dans l'interface ordinaire.

## Respiration

- Deux poses natives, distantes d'un pixel vertical, 1,5 seconde chacune.
- Haut du corps, manches, armes et doigts solidaires ; pieds et ombre fixes.
- Joueur et six bases humanoïdes compatibles de NPC. Bestiaire non humanoïde,
  portraits, inspection, inventaire et objets au sol restent fixes.
- L'option de mouvements réduits désactive la respiration. Un déplacement par
  effet de combat a priorité sur l'idle.
- Cache borné à 192 textures au total, poses comprises. Aucun changement des
  assets sources, des IDs, statistiques, sauvegardes ou collisions.

## Proposition du plateau

La galerie [galerie-terminal.html](galerie-terminal.html) rassemble des captures
du moteur et une alternance de ses deux poses d'idle. La version servie localement
est [la galerie du terminal](http://127.0.0.1:8769/galerie-terminal.html).

0. HUD actuellement utilisé, dans la même scène isolée.
1. Proposition violet ultraviolet, 1280 × 800.
2. Proposition bleu électrique, 1280 × 800.
3. Proposition vert néon, 1280 × 800.
4. Proposition rouge néon, 1280 × 800.
5. Proposition compacte, 960 × 540.
6. Respiration intégrée, quatre configurations d'armes et 17 tenues.
7. Réduction des mouvements : mêmes personnages fixes.

Le dessin propose des modules à angles droits, coins renforcés et en-têtes à
espacement fixe. Les jauges existantes sont conservées et segmentées, ainsi que
leurs valeurs et couleurs fonctionnelles. Le phosphore des cadres, des nombres
secondaires, des raccourcis et de la minicarte suit `InterfaceTheme`. La carte
connue, ses contacts et le comptage reprennent le rendu existant ; aucune donnée
invisible n'est révélée. La progression conserve sa jauge XP. La barre de
commandes utilise les raccourcis configurés et les lignes du journal réel.

La refonte ne passe pas par le HUD normal : `terminal_console_capture.rs` est
exclu du build release et appelé uniquement par `--ui-cold-terminal-console-proposal`.
La vue compacte conserve ses propres dimensions de carte et réorganise les
modules sans réduire la taille native des sprites. Cette proposition concerne
le plateau et son HUD ; elle ne remplace pas l'inventaire ni les autres fenêtres.

## Vérification

- `cargo test --locked --bin project-rl game_art -- --test-threads=1` :
  **17 tests réussis**, pieds fixes, translation exacte des couches, timing,
  orientation des armes, contacts, intégrité des palettes et corps stables.
- `cargo test --locked --bin project-rl end_screen_app -- --test-threads=1` :
  **5 tests réussis**, navigation de mort, historique, options et nouvelle partie.
- Builds debug et release réussis. Debug sans warning ; les quatre warnings
  release déjà présents restent. Aucun warning `wall_joins`.
- Format et `git diff --check` réussis. La suite complète n'a pas été relancée.
- Les sept captures de revue confirment un état de partie identique et aucune
  création de texture après échauffement des deux poses.
- Les huit frames d'idle alternent exactement entre deux images ; les huit
  frames de réduction des mouvements sont identiques. Les six bases NPC au
  repos sont identiques aux anciennes tiles d'acteurs.
- Captures du HUD normal, inventaire et écran de mort conservées. Galerie
  contrôlée dans le navigateur : changements de thème, compact et réduction
  des mouvements, images chargées, aucune erreur JavaScript.

Mesure debug isolée `--ui-cold-performance` : CPU médian du plateau **8,36 ms**,
retour **8,39 ms**, inventaire **11,56 ms**, premier affichage **363,34 ms**.
Le précédent relevé de la galerie des prises était respectivement 8,32 / 8,32 /
11,50 / 378,12 ms. Ces chiffres sont proches et ne démontrent pas un gain.
Les dix scènes de performance confirment que le rendu ne modifie pas la simulation.

Preuves : `validation.json`, `captures/*/art-validation.json`, logs de tests,
`performance/timings.json`, `terminal-gallery-browser.png`. `before/` conserve
les fichiers tels qu'ils étaient au début de cette étape, après les prises corrigées.

`capture.ps1` reproduit les captures ; `-ProposalOnly` limite la commande aux cinq
vues du nouveau HUD. `build_gallery.py` copie ces vues dans le serveur local
existant et crée la galerie, sans modifier le pipeline d'assets.
