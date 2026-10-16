# Éditeur : chargement, navigation, performances et nouveaux décors

Version locale du 2 octobre 2026, compilée en release. Lancer `Essayer_grande_carte.cmd` pour ouvrir la carte existante. `Essayer_nouveaux_murs.cmd` et `Essayer_animations.cmd` ouvrent des scènes d'essai modifiables.

## Utilisation

- **F5 / Sauver** : enregistrer directement ; Ctrl+Maj+S pour choisir un autre fichier.
- **F6 / Ouvrir** : bibliothèque des cartes ; flèches puis Entrée, ou bouton Ouvrir. Autre chemin… permet de choisir un autre dossier.
- **Molette sur la carte** : zoom autour du pointeur. Maj + molette conserve le réglage du pinceau rond.
- **Barres X/Y**, bouton milieu glissé ou **Espace + clic gauche glissé** : parcourir la carte. La mini-carte indique la zone visible.
- **F8** : vérification de la carte. **F9 / Animer** : arrêter ou reprendre les animations.

L'interface affiche les commandes principales en haut, le zoom, le nom du fichier et des boutons actifs plus lisibles. Elle a été inspectée en rendu natif à 1360 × 840 et 960 × 640. Les fichiers invalides sont refusés sans remplacer la composition. Une nouvelle carte reçoit un chemin disponible pour préserver les cartes déjà enregistrées.

## Chargement et performances mesurés

Le signalement de fermeture correspond à un événement Windows **AppHangB1**, sans trace de panique retrouvée. Le calcul synchrone de l'aperçu complet constituait une cause probable du blocage. L'aperçu est désormais préparé sur plusieurs images, à une résolution adaptée à son affichage et dans un cache indépendant. Le canevas prépare progressivement ses tuiles peintes avec un budget de 4 ms par image. Le sol de base est visible pendant cette préparation. Les évictions du cache ne parcourent plus toutes les entrées.

Mesures locales du dernier exécutable, sur la carte de **160 × 112 cases**, dans `validation-livraison/validation.json` :

| Essai | Ouverture des données et de la scène | Images pour l'aperçu | CPU moyen pendant l'aperçu | Image animée chaude, présentation comprise |
|---|---:|---:|---:|---:|
| 1 | 16,20 ms | 80 | 4,47 ms | 10,85 ms en moyenne |
| 2 | 19,04 ms | 80 | 4,25 ms | 12,61 ms en moyenne |
| 3 | 16,54 ms | 80 | 4,11 ms | 12,42 ms en moyenne |

L'aperçu précédent utilisait 400 à 420 images dans `validation-finale/validation.json`. Ces comptes ne sont pas une durée en secondes. Le premier passage présente encore une pointe CPU de 94,48 ms ; les deux suivants culminent à 28,29 et 25,42 ms. L'ouverture des données ne comprend pas la préparation progressive de toutes les peintures visibles. Le budget de 4 ms porte sur cette préparation, pas sur toute l'image. Ces mesures concernent cette machine et ces vues ; elles ne garantissent pas les performances de toutes les cartes.

## Animations

Reflets de l'eau peu profonde et profonde, pulsation des lasers et de certains voyants, mouvement léger des arbres et buissons. Les voyants des caméras sont animés ; leur tête ne pivote pas. Les effets sont décoratifs et ne modifient pas les règles, le hasard ou le document enregistré.

Deux captures à des temps contrôlés vérifient des changements distincts dans les zones eau, laser, voyants et arbres. Les images chaudes animées ne recalculent **aucune** tuile peinte. Un matériel refusant le shader conserve le rendu statique.

## Images ajoutées

Les trois familles ont été approuvées par l'utilisateur et intégrées au catalogue normal : Béton, Pierre et Grillage. Les quatre coins sont disponibles pour chacune. Le grillage conserve ses mailles transparentes ; il bloque le déplacement, tout en laissant passer la vision. Les anciens indices de murs restent identiques et les portes conservent leurs cadres métalliques.

Images produites avec **imagegen intégré**. Sources sauvegardées :

- `assets/prototypes/surface64/barriers-review/walls-concrete-source-v1.png`
- `assets/prototypes/surface64/barriers-review/walls-stone-source-v1.png`
- `assets/prototypes/surface64/barriers-review/walls-fence-source-v1.png`

Les [prompts exacts](prompts-textures.md) comprennent le premier essai de béton et sa correction. Seule la correction est chargée. Aucun fichier image de la grande carte n'a été remplacé.

## Vérification et captures

- **61 tests** de `map_editor` et **30 tests** de `textured_surface_preview` passent.
- Compilation verrouillée des exemples, formatage Rust et contrôle des espaces du diff.
- Trois ouvertures via la même fonction que le bouton de la bibliothèque ; vérification exacte du document chargé.
- Sauvegarde directe puis relecture JSON identique dans `validation-livraison/sauvegarde-verifiee.json`.
- Refus d'un fichier incomplet en conservant scène et chemin.
- Tests des limites de caméra, des barres, du point d'ancrage du zoom, de la peinture et des raccords des murs.
- Captures et assertions natives des animations. Aucun essai automatisé des gestes physiques de souris ou du clavier n'est revendiqué.

La grande carte source est conservée à l'identique : SHA-256 `B39D3B3B98F76B3958B394E6A0538EA1FB858CD854A2B528815CA0B8779D150C`.

Captures finales :

- [Éditeur à 960 pixels](validation-livraison/editeur-960.png)
- [Bibliothèque à 960 pixels](validation-livraison/bibliotheque-960.png)
- [Trois familles de murs](murs-valides/nouveaux-murs.png)
- [Béton](murs-valides/beton.png), [pierre](murs-valides/pierre.png), [grillage](murs-valides/grillage.png)
- [Animations, temps A](validation-livraison/animations-a.png) et [temps B](validation-livraison/animations-b.png)

Les sources sauvegardées dans `avant/` permettent de comparer les corrections avec le début de cette intervention. Les tests portent sur l'éditeur et son rendu partagé ; aucune validation complète de la campagne n'est revendiquée. Le lanceur écrit les erreurs dans `artifacts/map-editor/derniere-erreur.log` et les laisse consultables après un arrêt en erreur.
