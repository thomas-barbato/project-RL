# Prises d’armes corrigées — 5 octobre 2026

Corrections intégrées dans le pipeline natif actif : 18 dessins de famille
pour 113 modèles, sur 17 tenues de joueur et 6 bases humanoïdes de NPC.
Les 23 images de corps et leurs cartes de matériaux restent identiques aux
fichiers sauvegardés dans `before/`. IDs, catégories, équipement actif,
statistiques et collision ne sont pas modifiés.

## Défauts corrigés

- Des points/barres détachés dans la planche d’armes entraient dans le cadrage
  et réduisaient le vrai dessin. La silhouette principale est isolée avant
  réduction native ; chacune des 18 armes forme un composant connecté.
- L’ancien découpage de manches/doigts par fenêtres et neutralité de couleur
  conservait des morceaux de poignée, chargeur et acier. La nouvelle source
  de mains ne contient aucune arme ; les masques de doigts sont explicites.
- Les prises sont déclarées par famille, avec une main de support indépendante
  pour les armes à deux mains. La main arrière du fusil d’assaut saisit la
  poignée, et les autres familles utilisent leurs propres points de contact.
- Les manches rejoignent les coudes fixes par une transformation nearest qui
  conserve leur épaisseur. La pose pistolet conserve son coude d’origine.
- Couteau, hache et grande épée présentent leur tranchant vers l’extérieur ;
  l’inversion de côté retourne les couches portées, jamais le corps fixe.
- Les icônes au sol et les portraits utilisent les mêmes silhouettes nettoyées.

## Sources et prompts

Deux éditions avec l’outil **imagegen intégré**, fond transparent :

1. `grips-clean-source.png` : retrait des quatre armes de la source de prises,
   conservation/réparation des gants fermés et des manches. Prompt exact dans
   [grips-prompt.txt](grips-prompt.txt).
2. `greatsword-outward-source.png` : déplacement du biseau coupant sur le côté
   extérieur, avec la même inclinaison. Prompt exact dans
   [greatsword-prompt.txt](greatsword-prompt.txt).

La hache utilise la source extérieure déjà approuvée. Le couteau est retourné
horizontalement lors du conditionnement. Aucun corps n’a été régénéré.
`tools/art/build_crawl_assets.py` extrait, transforme et conditionne les sources
de façon déterministe ; il ne peint pas de nouvelles formes.

## Revue et validation

- [galerie-prises.html](galerie-prises.html) : 10 captures réelles du moteur,
  numérotées ; familles avec zoom et vue 1:1, 575 prises sur toutes les bases,
  49 associations d’ambidextrie, plateau, inventaire, survol et écran de mort.
- `audit.py` : planches de diagnostic des mêmes couches natives. Les 575 prises
  et 245 paires supplémentaires sur cinq corps ont été inspectées.
- `cargo test --locked --bin project-rl game_art::` : **15 réussis**, dont contact
  doigt/poignée, orientation des tranchants, absence de fragments flottants,
  corps fixe et géométrie des palettes. Le test des compositions parcourt
  **575 prises + 1 127 paires** (49 × 23 bases).
- `cargo test --locked --lib weapon_hands` : **3 réussis**, groupes équipés
  conservés et refus atomiques.
- Builds debug et release réussis. Debug sans warning ; release conserve les
  quatre warnings déjà présents avant cette correction. `wall_joins` ne revient
  pas dans les warnings.
- `cargo fmt --all -- --check`, `git diff --check` et le conditionnement
  `--check` réussis. Les entrées de contenu, contacts et bornes des corps
  du manifeste sont inchangés.
- Les six captures de revue confirment `state_unchanged_after_render` et
  `no_new_textures_after_warmup`. Cache borné à 192 textures, comme avant.
- Mesure debug isolée : médiane CPU du plateau **8,32 ms**, retour **8,32 ms**,
  inventaire **11,50 ms** ; premier affichage **378,12 ms**. Voir
  `performance/timings.json`. Ce relevé ne prouve pas un gain de performance.

La suite complète du projet n’a pas été relancée pour cette modification
de rendu. Les succès ci-dessus concernent les contrôles ciblés.

## Proposition d’animation idle

L’onglet **11** montre un essai web séparé utilisant les couches actives.
Le buste oscille entre deux positions distantes d’un pixel, une boucle de
trois secondes. Pieds fixes, mains et arme solidaires du buste ; les 17 tenues
et 18 familles sont sélectionnables. Contrôlé dans le navigateur à taille
native et zoom ×4, sans erreur JavaScript. Capture : `idle-browser.png`.

Cet idle **n’est pas intégré au jeu**. L’intégration future peut rester purement
visuelle, sans tour, modification des collisions ou de la sauvegarde, avec
respect de l’option de réduction des animations. Ce prototype sert à juger
le mouvement avant de l’activer dans le rendu normal.
