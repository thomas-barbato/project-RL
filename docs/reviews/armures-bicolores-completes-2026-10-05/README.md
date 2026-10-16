# Collection complète : propositions numérotées avant avis

Le 5 octobre 2026, l'utilisateur signale des tenues monocolores et des couleurs
parasites restantes. Il demande à voir toutes les propositions numérotées avant
de donner son avis, puis étend la demande à toutes les armures.

## Périmètre proposé

Les 14 armures de torse et les trois tenues de base sont couvertes. Les dessins
actifs servent de source à un renderer de recoloration isolé : `proposal.rs`.
Leurs contours, positions et alpha sont conservés. Aucun nouveau sprite,
modèle d'objet ou nom d'armure n'est ajouté au jeu. Les deux dessins image_gen
Bastion/technicien de l'étape précédente restent des propositions séparées.

Chaque tenue reçoit une zone principale et une zone secondaire : plaques des
bras/genoux, panneau frontal d'une robe ou tissu sous une veste, selon le modèle.
Les teintes des matériaux de tenue sont ramenées à trois valeurs par matériau
pour diminuer le grain. Les zones de peau et de lumière protégées restent telles
quelles. Les variantes existantes choisissent des duos ; objet et statistiques
restent identiques. L'aperçu ne change pas le tirage cosmétique ou les sauvegardes.

La proposition corrige aussi la classification qui attribuait certaines manches
cuivrées/brunes au matériau de l'arme. Les zones de casque, torse, manches et
peinture portée sont traitées séparément. La rouille voulue des armes et les
visières/visages conservent des couleurs chaudes.

## Numéros stables

| Nº | Tenue existante | Duo d'origine proposé |
|---|---|---|
| 01 | BRÈCHE | Rouge / ivoire |
| 02 | VIGIE | Bleu / ivoire |
| 03 | CREUSET | Vert / bleu nuit |
| 04 | Armure Cerbère | Ivoire / bleu |
| 05 | Tenue Cognefer | Cuivre / bleu nuit |
| 06 | Carapace Chrysalide | Violet / turquoise |
| 07 | Armure Azurite | Bleu / ivoire |
| 08 | Tenue Fantôme | Anthracite / turquoise |
| 09 | Robe de mue | Cuivre / ivoire |
| 10 | Armure Bastion | Ivoire / bleu |
| 11 | Tenue du ferrailleur | Bleu-vert / ivoire |
| 12 | Robe de la Seconde Ombre | Violet / ivoire |
| 13 | Tenue de membranes | Olive / bleu nuit |
| 14 | Combinaison Peau-Seconde | Rouge / anthracite |
| 15 | Tenue du technicien | Jaune / violet |
| 16 | Tenue d'aviateur | Bleu / ivoire |
| 17 | Tenue du nomade | Rouge / ivoire |

Les six variantes alternatives utilisent Rouge/ivoire, Bleu/ivoire,
Violet/turquoise, Turquoise/ivoire, Jaune/bleu nuit et Rose/bleu nuit.
Le nom affiché dans le jeu ne change pas dans cet essai.

## Tout voir avant avis

- `galerie.html` : galerie autonome avec les 17 numéros, les sept palettes et
  les huit poses, comparaison actuelle/proposée à taille native, zoom et 128 px.
  Ouverte dans le navigateur de Codex, servie localement sur
  `http://127.0.0.1:8769/galerie.html` pendant la session. Le fichier HTML
  contient ses images et peut être rouvert directement après arrêt du serveur.
- `propositions-01-06.png`, `propositions-07-12.png`, `propositions-13-17.png` :
  les 17 numéros, comparaison sans arme à taille native, zoom ×2 et exemples de
  pistolet, fusil, épée à deux mains et pose mixte.
- `variantes-01-06.png`, `variantes-07-12.png`, `variantes-13-17.png` : toutes les
  variantes des 17 tenues, zoom ×2.
- `fiches/01.png` à `fiches/17.png` : chaque numéro avec les sept palettes et
  toutes les huit poses en une planche. Les cellules conservent l'échelle réelle
  des dessins : le corps de 30 px apparaît à 60 px dans le zoom ×2.
- `sols-01-06-origine.png`, `sols-07-12-origine.png`, `sols-13-17-origine.png`
  et leurs versions `bleu` : captures natives Macroquad avec les fonctions
  actives de sol et de sang. Béton, métal, grille sombre et sol ensanglanté.
  La pose varie selon la ligne mais reste identique entre les deux versions.

## Vérification et limites

- 1904 variantes vérifiées : 17 tenues × 8 poses × 7 palettes × 2 tailles
  (48 px pour le cadre de jeu et 128 px pour l'inspection).
- Alpha identique à chaque source, zones protégées de peau/lumière conservées,
  deux zones de tenue visibles dans chaque variante. Le corps n'est pas agrandi.
- Sur les 272 variantes bleues, la mesure des zones de tenue hors peau, lumière
  et zones portées passe de 233 pixels orange dans l'ancien rendu à zéro dans
  la proposition. Le total additionne les deux résolutions ; il ne correspond
  pas à 233 pixels natifs uniques ni à une suppression de toute couleur chaude.
- Deux compilations isolées réussies et six captures natives produites.
- Galerie testée dans le navigateur : choix du technicien, variante bleue,
  pose pistolet ; contrôles et aperçus fonctionnels. Tab conservée pour l'avis.
- `preservation.json` : 656 fichiers actifs inchangés depuis la référence,
  comprenant code, dessins, données de contenu et packers. `git diff --check`
  réussit. Les modifications antérieures de l'utilisateur restent présentes.

Les assertions utilisent les zones explicites de la proposition, puis les
planches servent à vérifier visuellement leur placement. Elles ne constituent
pas une segmentation parfaite démontrée de chaque pixel de chaque dessin.
Les contours et détails de la source ne sont pas redessinés. La tenue Fantôme
reste volontairement sombre ; sa lisibilité sur la grille doit être appréciée
dans les captures avant avis. Les associations proposées peuvent être ajustées
par numéro, indépendamment de leur future intégration.

La suite de tests du jeu et sa compilation complète ne sont pas relancées pour
cet aperçu, qui ne modifie aucun fichier actif. Aucun commit, push ou déploiement.
L'intégration du renderer de tenue, y compris le chemin d'affichage de la
palette d'origine, attend l'avis demandé par l'utilisateur.

## Reproduire

`inspect.rs` charge les sources et l'ancien renderer copié dans `current.rs`,
ainsi que la proposition `proposal.rs`. Il produit les sorties CPU RGBA et les
assertions de `verification.json`. `build_gallery.py` les encode sans perte,
recadre et agrandit en nearest pour assembler les planches et la galerie. Il ne
peint pas de sprite. `preview.rs` affiche ces sorties avec les sols réels.

Les exécutables ont été compilés avec `rustc --edition=2024`, les bibliothèques
Macroquad, project_rl et serde_json déjà disponibles dans `target/debug/deps` ;
le preview ajoute serde pour les modules importés. Les journaux de compilation
sont `build-inspect.log` et `build-preview.log`. Le Python utilisé est le runtime
bundled `C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe`.
