# Étude Crawl et propositions natives

5 octobre 2026. Proposition avant avis ; aucun asset actif du jeu remplacé.

Définition pérenne : `../../docs/DIRECTION_ARTISTIQUE_TILES.md`.
Le README du projet et `../../AGENTS.md` renvoient à cette définition.

## Contenu

- `reference-crawl.png` : tiles sources étudiées, natives et agrandies sans lissage.
- `comparaison-32.png` : état actif, référence Crawl et nouveau candidat.
- `armures-numerote.png` : 17 tenues, numérotation conservée.
- `armes-numerote.png` : 16 représentations de familles, pas 16 nouveaux objets.
- `bestiaire-numerote.png` : 23 références de bestiaire/NPC existantes et une
  proposition de zombie conservée comme proposition.
- `poses-01-06.png`, `poses-07-12.png`, `poses-13-17.png` : huit configurations
  d'armes pour la même base de chaque tenue.
- `sols-*.png` et `scene-native.png` : captures Macroquad isolées, utilisant les
  fonctions de sol et de sang actives ; elles ne sont pas des captures d'une
  modification intégrée à la simulation.
- `galerie.html` : galerie autonome avec images incorporées, catégories,
  sélection de tenue, corps seul et couches d'armes/prises seules.
- La copie `../armures-bicolores-completes-2026-10-05/galerie-crawl.html` est
  servie à `http://127.0.0.1:8769/galerie-crawl.html`. Les anciennes galeries restent intactes.

## Fabrication

Le mode intégré du skill imagegen a servi à créer les dessins originaux. Les
sources `*-source.png`, les prompts `prompt-*.txt` et la version initiale du
bestiaire sont conservés. Les tiles de Crawl sont des références de style et
d'échelle ; aucun de leurs pixels n'est intégré dans les nouveaux corps/armes.

`package.py` découpe les sources, recadre les silhouettes, fixe l'échelle native
par catégorie, applique un seuil d'alpha 128, un rééchantillonnage nearest et un
encodage indexé sans dithering (24 couleurs pour corps/monstre, 16 pour arme/prise).
Les composantes de la planche bestiaire sont extraites avant cadrage pour éviter
le tronquage et les morceaux de cellules voisines. Les petites composantes
parasites sont filtrées ; les formes disjointes intentionnelles restent à contrôler
visuellement. Cette procédure de découpe est propre à cette planche proposée,
pas une règle universelle d'import de tous les monstres futurs.

`compose.rs` est un prototype Rust isolé : référence fixe de corps, remplacement
déclaré des zones de main/avant-bras, couche d'arme avec prises dessinées, miroir
de cette couche pour la main gauche. Le canevas de 48 pixels porte une case
logique de 32 pixels avec huit pixels de marge de chaque côté. Le corps occupe
30 pixels de haut ; il ne rétrécit pas pour les grandes armes.

`build_review.py` encode les planches, atlas composés et galerie. `native_preview.rs`
affiche les compositions sans lissage sur les fonctions de sols du projet.

## Vérification et limites

- 57 tiles natives : dimensions, alpha binaire, contenu non vide et budgets de palette.
- 184 compositions : 17 tenues × huit configurations, six NPC × huit configurations.
- Référence du corps identique hors remplacement déclaré des mains/avant-bras et
  pixels masqués par les couches de premier plan ; `composition-check.json`.
- Compilation/exécution du compositeur Rust et du programme Macroquad isolés.
- Contrôle visuel des planches et captures natives ; galerie vérifiée dans le navigateur.
- 656 fichiers actifs comparés à leur empreinte de début : aucun changement.

L'intégration active et les variantes cosmétiques du nouvel ensemble ne sont pas
effectuées. Les masques de remplacement des avant-bras restent communs et les
raccords doivent être acceptés visuellement avant de devenir un pipeline actif.
Une future intégration devra déclarer les matériaux des variantes et adapter
les clés de cache, les orientations et les points d'attache du rendu actif.
Les nouveaux modèles dessinés ne changent ni contenu, ni statistiques, ni collisions.
Les contrôles de cet aperçu ne constituent pas une exécution de la suite complète du jeu.

## Reproduire

Ne pas relancer `prepare_reference.py` pour vérifier la préservation : il crée
une nouvelle empreinte de départ. Conserver `baseline.json` lors d'une itération.

1. `package.py` avec Python + Pillow.
2. Compiler/exécuter `compose.rs` avec Macroquad, en édition Rust 2024.
3. `build_review.py` avec Python + Pillow.
4. Compiler/exécuter `native_preview.rs` avec Macroquad, project_rl, serde et serde_json.
5. Relancer `build_review.py` pour incorporer les nouvelles captures à la galerie.

Les programmes utilisent les dépendances Rust déjà compilées dans `target/debug/deps`.
Ils écrivent uniquement dans ce dossier d'aperçu et ajoutent une nouvelle page au
serveur de revue précédent.
