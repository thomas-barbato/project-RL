# Cahier des charges pour créer les cartes

**Date :** 1 octobre 2026. **Statut :** première proposition et essai local ; les fonctions restantes et l'intégration à la campagne restent à examiner avec l'utilisateur.

L'éditeur doit permettre de composer et de contrôler les décors de Project RL : dimensions, sols, murs, objets, orientations et propriétés de jeu. Son premier objectif est de rendre visibles et corrigeables les erreurs de placement et de raccord. Le premier catalogue utilise les images produites pour l'essai de surface. Leur présence dans cet outil ne vaut pas validation artistique pour la campagne.

## Direction du premier essai

Les lieux et leur décoration peuvent être conçus à la main. L'incertitude entre les parties se concentre sur les rencontres, les caves ou donjons et les accès vers les zones suivantes. Le contenu variable doit respecter des emplacements ou des règles préparés dans l'éditeur.

Cette direction doit être évaluée sur plusieurs parties avant de remplacer la génération actuelle. Des extérieurs fixes deviennent familiers ; varier seulement les ennemis ou la sortie ne renouvelle pas toute leur géographie. Plusieurs cartes, variantes et donjons seront nécessaires si l'exploration doit rester surprenante.

Le moteur utilise déjà des coordonnées en cases. Les images de référence sont préparées en tuiles de 64 × 64 pixels ; la taille affichée dépend du zoom. Une carte de 40 × 25 signifie 40 × 25 cases, indépendamment de la taille de la fenêtre.

## Fonctions de création et de navigation

| Fonction | Exigence de l'éditeur complet | Premier essai |
|---|---|---|
| Nouvelle carte | Choisir largeur, hauteur, sol initial, nom, thème et rôle du lieu. | Dimensions de 1 à 256 cases par côté ; sol de terre et départ initial. Cette limite est propre à l'outil d'essai. |
| Redimensionnement | Agrandir ou réduire avec choix du point d'ancrage et annonce des éléments supprimés. | Taille ou Ctrl+R : de 1 à 256 cases, neuf ancrages, compte des éléments coupés et annulation complète. |
| Navigation | Zoom, déplacement de la vue, recentrage, coordonnées et grille. | Zoom, caméra, grille facultative masquée par défaut et sélection à la souris. |
| Catalogue | Vignettes, catégories, recherche, filtres par thème et orientations disponibles. | Sols, cinq familles de murs et objets ; filtres Tout, Ville, Tech, Serveurs, Habitat et Armurerie. Recherche à ajouter. |
| Peinture | Pinceau, ligne, rectangle, remplissage et variantes contrôlées. | Placement et peinture par cases ; outils de formes et remplissage à ajouter. |
| Sélection | Sélection simple et multiple, déplacement, duplication et suppression. | Sélection simple, déplacement, copie, rotation et suppression. |
| Historique | Annuler et rétablir les opérations de composition. | Historique borné, conservant aussi les changements de dimensions. |
| Fichiers | Nouveau, ouvrir, enregistrer, enregistrer sous, indication des modifications et protection à la fermeture. | JSON local, chemin modifiable, état modifié et protection à la fermeture. |
| Essai jouable | Tester la carte avec les déplacements, interactions et perception du jeu ; revenir sans altérer la composition. | Déplacements, portes et vision du moteur ; retour à la composition. |

## Sols objets et structures

Un objet se place au-dessus d'un sol. Tourner, déplacer ou supprimer cet objet ne doit jamais remplacer le sol sous lui. Peindre un autre sol ne doit pas effacer le mobilier.

Les données devront distinguer :

- Le terrain et ses propriétés : sol, eau, obstacles et autres terrains du jeu.
- Son apparence : matériau, texture, variante et raccord aux matériaux voisins.
- Les structures : murs, portes, passages et installations.
- Les objets : mobilier, équipements et éléments naturels ou décoratifs.
- Les marques de composition : départ du joueur, zones de rencontres, accès de donjons et passages candidats.

L'éditeur complet devra afficher, masquer et verrouiller les catégories de données pour permettre une modification précise. L'essai sépare déjà sols, structures et objets, avec un objet ou une structure au maximum par case. La superposition de plusieurs petits objets et les calques verrouillables restent à concevoir.

Les murs calculent leur silhouette selon les voisins : éléments isolés, extrémités, segments, coins, jonctions en T et croisements. Une rotation manuelle ne doit pas casser ce calcul. Les portes ont une orientation et un état initial ; leurs raccords devront être contrôlés lors de la validation.

## Rotation et dimensions des objets

La première version propose les orientations 0°, 90°, 180° et 270°, sur la grille. La rotation s'applique avant placement ou à un élément sélectionné.

Le catalogue complet devra déclarer pour chaque élément :

- Son identifiant stable, son nom et sa catégorie.
- Ses dimensions et son point d'ancrage.
- Les cases occupées dans chaque orientation.
- Les orientations autorisées.
- Le mode d'affichage : rotation de l'image ou images distinctes selon l'orientation.
- Ses propriétés de passage, de vision et d'interaction.

Un meuble de 2 × 1 cases tourné de 90° occupe 1 × 2 cases. L'image, l'aperçu de placement et les cases de collision doivent se transformer ensemble. La rotation doit être refusée ou signalée si cette empreinte sort de la carte ou chevauche un obstacle incompatible.

Certains objets avec une façade visible ou un éclairage marqué nécessiteront des variantes artistiques plutôt qu'une simple rotation du PNG. Le catalogue devra permettre ce choix. Dans le premier lot, les objets occupent une case et tournent par rotation de l'image ; leurs orientations restent à valider visuellement.

## Propriétés et interactions du jeu

Le décor ne doit pas déterminer les règles du jeu par sa seule couleur ou son image. Une texture de métal et une texture de béton peuvent représenter un même terrain traversable ; l'eau profonde conserve sa propre règle de déplacement.

| Élément | Données nécessaires dans l'éditeur complet |
|---|---|
| Objet décoratif | Image, orientation, empreinte et ordre d'affichage. |
| Obstacle | Cases bloquées et comportement de vision indépendant du passage. |
| Porte | Orientation, état initial, accès et éventuel dispositif de commande. |
| Installation | Identifiant de contenu existant et paramètres d'interaction compatibles avec le moteur. |
| Habitants et services | Rôle, identité, zone protégée et liens avec les données du jeu. |
| Objets et ressources | Référence de contenu, propriété éventuelle et règle de placement ou de renouvellement. |
| Objectif | Identité stable et liens avec les quêtes ou faits du monde. |
| Passage | Identité de destination, point d'arrivée, retour et conditions d'ouverture. |

**Limite de l'essai :** le choix « obstacle » utilise une représentation de mur dans la carte de test et bloque aussi la vision. Cela éprouve les déplacements, mais ne constitue pas le modèle final du mobilier. L'indépendance entre passage et vision et les interactions de mobilier devront être intégrées avec les règles du moteur.

Le joueur reste représenté par `@` dans l'essai. Les personnages, animations, effets de combat et services de campagne ne sont pas intégrés à cet outil initial.

## Contenu aléatoire encadré

### Rencontres

L'éditeur complet devra permettre de définir des zones de placement et des emplacements exclus, puis d'associer des profils du bestiaire existant. Les règles préciseront populations autorisées, quantités ou budgets, distance des arrivées, contraintes de terrain et accès à préserver.

Les tirages doivent modifier des situations de jeu : groupes, positions ou patrouilles cohérentes. Ils ne doivent pas placer arbitrairement des ennemis dans les services d'une ville, sur un objectif ou à l'intérieur d'un obstacle. Les chiffres d'équilibrage restent ceux du contenu approuvé ; l'éditeur ne devra pas les remplacer silencieusement.

### Caves et donjons

Les accès seront placés ou choisis parmi des emplacements candidats préparés. Les intérieurs pourront être générés, notamment par assemblage de salles ou de morceaux de carte conçus dans l'éditeur. Le choix exact de cette méthode reste à arrêter.

Chaque morceau devra porter ses raccords, entrées, sorties, rôle et contraintes de placement. Tourner un morceau doit transformer sa géométrie, son mobilier, ses marqueurs et ses connexions ensemble. Les accès nécessaires doivent rester utilisables après l'assemblage.

### Passage vers la zone suivante

La proposition est de tirer un accès parmi plusieurs emplacements préparés, avec des chemins et des enjeux distincts. Il devra rester cohérent avec le lieu et accessible selon les conditions de progression prévues. Déplacer une sortie de quelques cases sans modifier le trajet ne suffit pas à garantir une variation intéressante.

La même graine doit reproduire le même monde initial. Un retour dans une zone ou une reprise de sauvegarde doit retrouver ses placements et les conséquences des actions du joueur. Les échanges entre zones, les accès de retour et les dépendances de quête doivent être résolus avant la validation d'une carte de campagne.

**Ces trois catégories de variation sont prévues dans le cahier des charges ; elles ne sont pas encore exécutées par le premier éditeur.**

## Catalogue artistique

Le catalogue doit grandir par lots utilisables. Chaque lot est examiné à la taille réelle, dans une composition, et dans les orientations nécessaires. La bibliothèque de l'outil peut contenir des éléments expérimentaux ; la bibliothèque utilisée par la campagne doit identifier les éléments validés.

| Lot | Contenu |
|---|---|
| Premier essai de surface | 10 sols, murs et raccords, portes ouvertes et fermées, 10 meubles ou équipements. |
| Extérieur de surface à produire | Rochers, arbustes, touffes et roseaux, débris, ruines et transitions de terrain. |
| Variantes à produire | Sols moins répétitifs, états de mobilier, orientations dessinées lorsque nécessaire. |
| Autres couches | Catalogues propres à leurs lieux et à leur progression visuelle ; aucun thème profond n'est déduit automatiquement du lot de surface. |

Les sols doivent éviter les joints parasites et les motifs trop visibles. Les objets doivent posséder une transparence correcte et une échelle cohérente. Les murs et portes doivent couvrir toutes les configurations autorisées. Les ombres et la perspective doivent rester cohérentes dans une composition réelle.

Les sols sont des matériaux continus, sans cadre autour de chaque case. Les plaques encadrées ne constituent pas le sol standard. La grille de composition est un repère facultatif, masqué par défaut et activable avec G ; elle ne doit pas être intégrée aux images.

Le premier lot provient de planches générées, préparées pour un affichage de 64 × 64 pixels. Il ne s'agit pas d'un catalogue entièrement dessiné à la main en pixel art. Les raccords géométriques sont vérifiés, mais la finition artistique reste à examiner.

## Données sauvegarde et intégration

Le format de l'éditeur doit être versionné et séparer la carte de ses images. À terme, il devra référencer les éléments du catalogue par des identifiants stables et conserver les dimensions, terrains, instances, orientations, empreintes, propriétés, marqueurs et connexions.

Le JSON du premier essai contient la version, les dimensions, le départ, les sols, les structures et les objets. Les indices d'images sont ceux du premier lot ; ils devront être migrés vers des identifiants de catalogue avant une bibliothèque extensible. Les fichiers sauvegardés ne sont pas encore des cartes importables dans la campagne.

La grille de simulation reste conservée. La carte visuelle peut être peinte librement, sans suivre les limites des cases. Le champ facultatif `paint` conserve des traits au pixel natif, leur matériau et leur diamètre ; son absence dans les anciennes cartes signifie simplement qu'elles ne contiennent pas de peinture libre.

L'intégration devra réutiliser les données de zones et les primitives du moteur. Elle devra préserver les suspensions existantes et maintenir la séparation entre présentation, simulation et génération. Une composition ne devient pas un contenu de campagne par sa seule sauvegarde dans l'éditeur.

## Contrôles attendus

La validation devra distinguer les défauts de dessin des défauts de jeu et donner leurs coordonnées ou leurs éléments concernés :

- Images absentes, dimensions incohérentes, orientation indisponible et raccords manquants.
- Instances hors carte, superpositions incompatibles et empreintes incorrectes après rotation.
- Départ inutilisable, accès essentiels inaccessibles et passage incohérent avec sa destination.
- Portes ou objectifs dont les prérequis ne peuvent être atteints.
- Emplacements de rencontres invalides et contenu indispensable absent après tirage.
- Identifiants inconnus, données invalides ou version de fichier incompatible.

Les contrôles de navigation présents dans le jeu sont à réutiliser pour cette intégration. Une carte connectée n'est pas nécessairement une carte intéressante ou équilibrée ; les essais jouables et la revue visuelle restent nécessaires.

## Parcours de validation

1. Créer une carte rectangulaire avec les dimensions saisies.
2. Peindre une zone de béton, placer un lit dessus et vérifier que les deux données sont conservées.
3. Tourner ce lit, le déplacer puis le supprimer ; le béton doit rester à sa position.
4. Refaire ces opérations avec Annuler et Rétablir.
5. Construire des murs avec coins, extrémités et jonctions ; vérifier leurs raccords.
6. Placer une porte dans une orientation adaptée, passer en test, l'ouvrir et la traverser.
7. Sauvegarder, rouvrir et retrouver dimensions, orientations, propriétés et composition.
8. Tenter une donnée invalide ; la carte ouverte doit rester intacte.
9. Vérifier les coordonnées de placement et les commandes en 960 × 540 et en grand format.
10. Lors de l'intégration du contenu variable, comparer plusieurs graines et vérifier la stabilité après sauvegarde et retour.

## Suite de réalisation

Le premier essai sert à juger les gestes de composition et le premier lot graphique. La prochaine étape sera d'ajuster cette interface et ses images après utilisation.

L'éditeur complet demandera ensuite le catalogue extensible, les empreintes sur plusieurs cases, les outils de sélection et peinture plus riches, les propriétés de jeu, puis les marqueurs du contenu variable. L'import en campagne et la génération des donjons viendront après la définition et la validation de ces données.

## Utilisation du premier essai

Depuis la racine du projet :

Double-cliquer sur `Editeur_de_cartes.cmd` pour ouvrir l'éditeur habituel avec les catalogues urbain et SF validés. Le lanceur `Essayer_salles_SF.cmd` ouvre directement la composition SF.

```powershell
cargo run --locked --example map_editor
```

Pour ouvrir un fichier de l'outil :

```powershell
cargo run --locked --example map_editor -- --map artifacts/map-editor/maps/carte.json
```

La palette contient les sols, les murs, les portes, le départ du joueur et les objets. Le bouton Nouvelle permet de saisir les dimensions. Le bouton Démo restaure la scène meublée ; cette opération est annulable.

### Pinceau rond pour le sol

Dans Sols, choisir Case pour modifier le terrain case par case, ou Rond pour dessiner sur le sol avec la texture sélectionnée, y compris Eau peu profonde et Eau profonde. Le pinceau rond travaille à la résolution native des images, indépendamment du zoom. Il relie les positions successives du pointeur : un mouvement rapide ne laisse pas de trous entre les coups de pinceau. Les bords reçoivent une transition courte et un grain stable adapté au pixel art.

Les boutons − et +, ou Maj + molette sur la carte, règlent le diamètre entre 0,25 et 6 cases, par pas de 0,25 case. Le clic gauche peint ; le clic droit gomme uniquement cette peinture, en révélant le sol de base et en conservant murs et objets. Ctrl+Z annule le trait complet ; Ctrl+Y le rétablit. Les traits restent présents en test jouable et après sauvegarde et rechargement.

La peinture libre accepte tous les sols et les deux profondeurs d'eau. Pour l'essai jouable, le matériau dominant au centre d'une case détermine son terrain : sol, eau peu profonde ou eau profonde. Les contours visuels restent libres ; les déplacements restent sur la grille. La gomme restitue le terrain de base. Les murs, portes et objets bloquants conservent leur priorité sur ce terrain. L'éditeur refuse un trait qui rendrait le départ du joueur inaccessible par de l'eau profonde.

Aux marges des murs, seul le matériau du sol de base est repris depuis la case voisine. La peinture est ensuite dessinée une seule fois à ses coordonnées dans le monde. Elle ne doit pas être copiée dans les bordures d'une autre case. Les captures de régression pour les quatre côtés d'un bâtiment, un coin et le bord de carte sont dans `artifacts/pinceau-eau-murs-2026-10-01`.

L'essai du pinceau, sa carte sauvegardée et les captures à 1360 × 840 et 960 × 540 sont dans `artifacts/pinceau-rond-2026-10-01`. Il valide les outils de peinture ; la diversité des textures et le catalogue de décors doivent encore être enrichis pour atteindre le rendu artistique souhaité.

La palette Murs propose quatre coins distincts : haut gauche, haut droit, bas gauche et bas droit. Choisir un coin et cliquer sur une case d'angle remplace son mur droit par cette pièce, même si les murs voisins sont placés manuellement. Chaque coin possède sa propre image ; R choisit l'image du coin suivant. L'outil Mur et raccords utilise également ces quatre images pour les angles automatiques. Aux bords transparents des structures, le rendu reprend le sol voisin de chaque côté : un sol peint sous le mur ne se prolonge plus à l'extérieur. Ses données restent conservées pour retrouver ce sol en retirant le mur ; le seuil d'une porte conserve également son sol.

Avec l'outil Mur, R tourne le prochain mur et fixe son orientation. En sélection, R tourne le mur ou choisit l'orientation suivante du coin existant et conserve sa forme, même si des voisins sont ajoutés ou retirés. A ou un nouveau choix de Mur et raccords rétablit les raccords automatiques. Le mode auto ou manuel est indiqué sous la palette. Les cartes enregistrées avant cette option conservent leurs raccords automatiques ; les orientations manuelles de coins enregistrées dans l'essai précédent sont traduites vers la nouvelle image correspondante.

Le lot actuel dessine ensemble les murs horizontaux, verticaux et les quatre coins. Sur un contour rectangulaire, les murs automatiques suivent la face de leur coin de raccord ; le bord clair reste ainsi sur le pourtour. En placement manuel, les quatre faces du mur droit correspondent au haut, à la droite, au bas et à la gauche avec les rotations 0°, 90°, 180° et 270°. Les anciens choix manuels restent conservés et peuvent être ajustés avec R. Ce nouveau lot reste soumis à validation visuelle ; les portes et les autres jonctions doivent encore être harmonisées.

| Commande | Action |
|---|---|
| Clic en placement | Placer le choix de la palette. |
| Clic droit | Retirer l'objet ou la structure ; conserver le sol. |
| Sélection ou V | Sélectionner puis glisser un élément pour le déplacer. |
| R | Tourner l'élément sélectionné ou le prochain élément à placer. |
| A avec un mur | Revenir aux raccords automatiques. |
| B | Modifier le caractère obstacle ou décor de l'objet sélectionné ou à placer. |
| Suppr | Supprimer l'élément sélectionné. |
| P | Copier le choix sous le pointeur dans la palette de placement. |
| Ctrl+C et Ctrl+V | Copier la sélection puis placer une copie sous le pointeur. |
| Molette sur la carte | Changer le zoom. |
| Sols > Rond, clic gauche | Peindre librement le sol ou l'eau. |
| Sols > Rond, clic droit | Gommer la peinture libre. |
| Maj + molette avec le pinceau rond | Régler la largeur du pinceau. |
| G | Afficher ou masquer la grille de composition. |
| Clic milieu ou flèches | Déplacer la caméra en composition. |
| Molette sur la palette | Faire défiler les éléments. |
| Ctrl+Z et Ctrl+Y | Annuler et rétablir. |
| Ctrl+N | Ouvrir Nouvelle carte. |
| Taille ou Ctrl+R | Redimensionner la carte actuelle avec choix d'ancrage. |
| Sauver dans la palette | Choisir le chemin du fichier à enregistrer. |
| Ctrl+S | Enregistrer au chemin courant. |
| Charger ou Ctrl+O | Choisir le chemin du fichier à ouvrir. |
| Tester ou Tab | Passer entre composition et essai jouable. |
| Flèches ou ZQSD en test | Déplacer le joueur. |
| E en test | Actionner une porte voisine. |
| F en test | Afficher la vision du moteur. |
| 1, 2, 3 | Afficher les cases à 32, 48 ou 64 pixels. |

Les cartes de cet outil et les images expérimentales restent séparées de la partie normale. Les captures et le relevé de validation sont conservés dans `artifacts/map-editor-review-2026-10-01`.

## Lot urbain disponible

`Essayer_ville_surface.cmd` ouvre le catalogue urbain et une rue composée avec une épicerie, un logement, une clinique et un atelier. En ligne de commande : `cargo run --locked --example map_editor -- --urban-review`. Le bouton Démo restaure cette rue dans ce mode ; Annuler conserve son comportement.

Le lot propose 12 nouveaux sols et 48 nouveaux objets, dont 16 équipements contemporains ou futuristes. Dans Murs, les boutons Métal, Brique et Crépi sélectionnent la famille des murs droits et des quatre coins. Les portes utilisent encore les cadres métalliques existants. Les sols urbains acceptent la pose par case et le pinceau rond ; les objets conservent rotation, déplacement, choix obstacle/décor et sauvegarde.

Les fichiers précédents conservent leurs indices ; l'absence de famille de mur signifie Métal. Le lot urbain validé est maintenant disponible dans l'éditeur normal. Les équipements ajoutés sont des images et des obstacles de test : distributeurs, robot et machines ne fournissent pas encore de services ou d'animations.

Les sources générées se trouvent dans `assets/prototypes/surface64/urban-review`. Les prompts, captures à 64 pixels et preuves de validation du lot sont dans `artifacts/ville-surface-lot2-2026-10-01`. La validation artistique reste à faire avant adoption dans la campagne.

## Éléments SF

Le lot SF a été validé pour l'éditeur le 1 octobre 2026. Le catalogue normal contient 48 nouveaux objets SF et deux familles de murs : Laser et Caméra, chacune avec les quatre coins. Les filtres Serveurs, Habitat et Armurerie regroupent les nouvelles images. La caméra intégrée fait partie du mur et conserve ses raccords ; les caméras isolées restent disponibles comme objets.

`Essayer_salles_SF.cmd` ouvre une installation composée avec salle de serveurs, armurerie, chambres et laboratoire (`--sf-demo` ; l'ancien argument `--sf-review` reste accepté). Le catalogue est aussi accessible en ouvrant normalement l'éditeur. Les sources sont dans `assets/prototypes/surface64/sf`, les prompts et les captures dans `artifacts/sf-redimensionnement-2026-10-01`.

Les escaliers et la trappe sont accessibles par défaut en test. Ce sont des accès visuels : leur destination souterraine n'est pas encore reliée à un étage du jeu. Les lasers bloquent comme des murs dans ce prototype ; les caméras ne déclenchent pas d'alarme. L'armurerie n'ajoute pas d'armes utilisables, de services ou d'animations à la campagne.

## Modifier les dimensions

Cliquer sur Taille ou utiliser Ctrl+R, saisir largeur et hauteur, puis choisir l'ancrage. Haut gauche conserve les coordonnées des placements et ajoute ou retire les cases à droite et en bas. Les autres ancrages déplacent ensemble le contenu conservé. Pour une différence impaire, l'ancrage central place la case supplémentaire ou retirée du côté droit/bas.

L'agrandissement ajoute des cases de terre. Les sols, peintures, murs, portes et objets déjà présents sont conservés. La réduction retire les placements hors du nouveau cadre et découpe la peinture à ce cadre. Un agrandissement ultérieur ajoute de l'espace neuf ; il ne fait pas réapparaître les zones coupées. Ctrl+Z restaure entièrement la carte précédente, y compris ces zones et le départ du joueur.

La fenêtre indique le nombre de murs/portes et d'objets hors limites. Si le départ est coupé, il est replacé sur la case accessible la plus proche du bord conservé. Une réduction qui ne laisserait aucune case accessible est refusée. Le format JSON conserve les limites de peinture et le déplacement de l'ancien fond ; les fichiers antérieurs restent lisibles.
