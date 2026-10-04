# Cahier des charges pour créer les cartes

**Date :** 2 octobre 2026. **Statut :** outils de composition intégrés à l'éditeur local ; le raccordement des cartes, rencontres et accès à la campagne reste à réaliser.

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
| Navigation | Zoom, déplacement de la vue, recentrage, coordonnées et grille. | Zoom à la molette autour du pointeur, barres horizontale et verticale, déplacement par glissement, mini-carte et grille facultative. |
| Catalogue | Vignettes, catégories, recherche, filtres par thème et orientations disponibles. | Grandes vignettes, recherche sans accents, favoris, 24 choix récents, catégories et filtres de thèmes. |
| Peinture | Pinceau, ligne, rectangle, remplissage et variantes contrôlées. | Pinceau par case ou rond, ligne, rectangle, remplissage borné par les murs, aperçu de la forme et refus des placements incompatibles. |
| Sélection | Sélection simple et multiple, déplacement, duplication et suppression. | Sélection rectangulaire, déplacement, copie, rotation et suppression de groupes ; ensembles enregistrables et réutilisables. |
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

L'éditeur affiche, masque et verrouille cinq calques : sols, murs, mobilier, décorations et marqueurs. Les outils de composition et les sélections agissent sur les calques visibles et déverrouillés. Les meubles possèdent une empreinte sur la grille ; plusieurs petites décorations non bloquantes peuvent se superposer à une case ou à un meuble. Le bouton Objet superposé suivant permet d'inspecter les éléments d'une pile.

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

Les objets peuvent occuper de 1 à 8 cases par dimension. La table, la chaise, la console opérateur et les lits bleu, rouge, gris et orange disposent de quatre images distinctes : la façade reste debout dans chaque direction. Les autres objets utilisent encore une rotation du PNG. Les dimensions des anciens placements restent conservées ; les nouveaux lits colorés occupent par défaut 1 × 2 cases, la table et la console 2 × 1.

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

Le mobilier possède deux réglages indépendants : Bloque le déplacement et Bloque la vision. Ils sont appliqués aux cases de son empreinte dans l'essai jouable. Les anciens fichiers sans réglage de vision conservent leur comportement antérieur : un obstacle bloque aussi la vue. Les interactions des machines, commerces et meubles restent à raccorder au contenu du jeu.

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
cargo run --release --locked --example map_editor
```

Pour ouvrir un fichier de l'outil :

```powershell
cargo run --release --locked --example map_editor -- --map artifacts/map-editor/maps/carte.json
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
| Sélection ou V | Glisser pour sélectionner une zone ; une seule case suffit pour inspecter un élément. |
| Déplacer | Glisser depuis l'intérieur de la sélection pour déplacer les calques modifiables du groupe. |
| R | Tourner le groupe sélectionné, l'ensemble à coller ou le prochain élément à placer. Propriétés permet de tourner un élément seul. |
| A avec un mur | Revenir aux raccords automatiques. |
| B | Modifier le caractère obstacle ou décor de l'objet sélectionné ou à placer. |
| Suppr | Supprimer l'élément sélectionné. |
| P | Copier le choix sous le pointeur dans la palette de placement. |
| Ctrl+C et Ctrl+V | Copier les calques modifiables de la sélection puis activer l'aperçu de collage ; cliquer pour placer. R tourne l'ensemble. |
| F2 ou Ctrl+F | Ouvrir le catalogue ; Ctrl+F active la recherche. |
| F3 | Afficher et verrouiller les calques. |
| F4 | Propriétés du meuble, mur, porte ou marqueur sélectionné. |
| Sauver, F5 ou Ctrl+S | Enregistrer directement au chemin courant. |
| Ouvrir, F6 ou Ctrl+O | Ouvrir la bibliothèque des cartes enregistrées, leurs aperçus et versions précédentes. |
| F7 | Enregistrer la sélection comme ensemble ou charger un ensemble à placer. |
| F8 | Vérification, surcouches de déplacement, vision et accès, aperçu des tirages. |
| Animer ou F9 | Arrêter ou reprendre les animations décoratives. |
| Molette sur la carte | Changer le zoom autour du pointeur, de 8 à 128 pixels par case. |
| Sols > Rond, clic gauche | Peindre librement le sol ou l'eau. |
| Sols > Rond, clic droit | Gommer la peinture libre. |
| Maj + molette avec le pinceau rond | Régler la largeur du pinceau. |
| G | Afficher ou masquer la grille de composition. |
| Glisser avec le bouton milieu, ou Espace + clic gauche | Déplacer la caméra sans peindre. |
| Barres horizontale et verticale | Glisser le curseur ou cliquer dans la barre pour parcourir la carte. |
| Flèches en composition | Déplacer la caméra ; Maj accélère le déplacement. |
| Molette sur la palette | Faire défiler les éléments. |
| Ctrl+Z et Ctrl+Y | Annuler et rétablir. |
| Ctrl+N | Ouvrir Nouvelle carte. |
| Taille ou Ctrl+R | Redimensionner la carte actuelle avec choix d'ancrage. |
| Ctrl+Maj+S | Choisir un autre chemin pour enregistrer la carte. |
| Cartes > Autre chemin… | Choisir le chemin d'une carte hors du dossier affiché. |
| Flèches puis Entrée dans Cartes | Sélectionner puis ouvrir un fichier. |
| Tester ou Tab | Passer entre composition et essai jouable. |
| Flèches ou ZQSD en test | Déplacer le joueur. |
| E en test | Actionner une porte voisine. |
| F en test | Afficher la vision du moteur. |
| 1, 2, 3 | Afficher les cases à 32, 48 ou 64 pixels. |

Les cartes de cet outil et les images expérimentales restent séparées de la partie normale. Les captures et le relevé de validation sont conservés dans `artifacts/map-editor-review-2026-10-01`.

## Lot urbain disponible

`Essayer_ville_surface.cmd` ouvre le catalogue urbain et une rue composée avec une épicerie, un logement, une clinique et un atelier. En ligne de commande : `cargo run --locked --example map_editor -- --urban-review`. Le bouton Démo restaure cette rue dans ce mode ; Annuler conserve son comportement.

Le lot propose 12 nouveaux sols et 48 nouveaux objets, dont 16 équipements contemporains ou futuristes. Dans Murs, les boutons Métal, Brique et Crépi sélectionnent la famille des murs droits et des quatre coins. Les portes utilisent encore les cadres métalliques existants. Les sols urbains acceptent la pose par case et le pinceau rond ; les objets conservent rotation, déplacement, choix obstacle/décor et sauvegarde.

Les fichiers précédents conservent leurs indices ; l'absence de famille de mur signifie Métal. Le lot urbain validé est maintenant disponible dans l'éditeur normal. Les équipements ajoutés sont des images et des obstacles de test ; leurs services restent à raccorder au jeu. Certains voyants possèdent désormais une animation décorative.

Les sources générées se trouvent dans `assets/prototypes/surface64/urban-review`. Les prompts, captures à 64 pixels et preuves de validation du lot sont dans `artifacts/ville-surface-lot2-2026-10-01`. La validation artistique reste à faire avant adoption dans la campagne.

## Éléments SF

Le lot SF a été validé pour l'éditeur le 1 octobre 2026. Le catalogue normal contient 48 nouveaux objets SF et deux familles de murs : Laser et Caméra, chacune avec les quatre coins. Les filtres Serveurs, Habitat et Armurerie regroupent les nouvelles images. La caméra intégrée fait partie du mur et conserve ses raccords ; les caméras isolées restent disponibles comme objets.

`Essayer_salles_SF.cmd` ouvre une installation composée avec salle de serveurs, armurerie, chambres et laboratoire (`--sf-demo` ; l'ancien argument `--sf-review` reste accepté). Le catalogue est aussi accessible en ouvrant normalement l'éditeur. Les sources sont dans `assets/prototypes/surface64/sf`, les prompts et les captures dans `artifacts/sf-redimensionnement-2026-10-01`.

Les escaliers et la trappe sont accessibles par défaut en test. Ce sont des accès visuels : leur destination souterraine n'est pas encore reliée à un étage du jeu. Les lasers bloquent comme des murs dans ce prototype ; les caméras ne déclenchent pas d'alarme. L'armurerie n'ajoute pas d'armes utilisables, de services ou d'animations à la campagne.

## Modifier les dimensions

Cliquer sur Taille ou utiliser Ctrl+R, saisir largeur et hauteur, puis choisir l'ancrage. Haut gauche conserve les coordonnées des placements et ajoute ou retire les cases à droite et en bas. Les autres ancrages déplacent ensemble le contenu conservé. Pour une différence impaire, l'ancrage central place la case supplémentaire ou retirée du côté droit/bas.

L'agrandissement ajoute des cases de terre. Les sols, peintures, murs, portes et objets déjà présents sont conservés. La réduction retire les placements hors du nouveau cadre et découpe la peinture à ce cadre. Un agrandissement ultérieur ajoute de l'espace neuf ; il ne fait pas réapparaître les zones coupées. Ctrl+Z restaure entièrement la carte précédente, y compris ces zones et le départ du joueur.

La fenêtre indique le nombre de murs/portes et d'objets hors limites. Si le départ est coupé, il est replacé sur la case accessible la plus proche du bord conservé. Une réduction qui ne laisserait aucune case accessible est refusée. Le format JSON conserve les limites de peinture et le déplacement de l'ancien fond ; les fichiers antérieurs restent lisibles.

## Outils de composition disponibles

Ligne et Rectangle affichent la proposition avant le relâchement de la souris, avec les raccords des murs. Le rectangle remplit les sols et ne trace que le périmètre pour les murs et portes. Remplir remplace le sol de base dans la zone de même matériau ; il s'arrête aux murs et retire la peinture libre dans les cases remplacées. Une opération complète correspond à une annulation. Un placement incompatible est refusé entièrement.

Une sélection inclut les meubles et marqueurs entièrement contenus dans son rectangle. Un meuble coupé par la sélection reste en place. Masquer ou verrouiller un calque permet, par exemple, de copier le mobilier sans copier le sol. Déplacer et tourner transforment également les peintures courbes et le départ du joueur s'il est sélectionné. Copier ou réutiliser un ensemble ne duplique pas le départ.

Propriétés règle largeur, hauteur, rotation, décalages de −31 à +31 pixels natifs et taille du dessin de 25 à 200 %. Les décalages et cette taille modifient l'apparence ; la collision conserve l'empreinte définie par les dimensions. Les murs proposent leur famille et le retour aux raccords automatiques. Une porte peut être ouverte, fermée, verrouillée ou sans courant.

La miniature en haut à droite permet de recentrer la vue par un clic. Centrer ramène la vue sur la sélection ou sur la carte complète. Les grandes cartes se parcourent avec le zoom, les flèches et le clic milieu.

Les lanceurs utilisent désormais `target/release/examples/map_editor.exe`, compilé avec `cargo build --release --locked --example map_editor`. Le cache de peinture conserve toutes les cases visibles et une marge, puis évince les cases les moins récemment utilisées. La miniature reste en cache tant que le document et ses dimensions d'affichage ne changent pas. Ces corrections permettent de parcourir la grande carte sans recalculer ses sols en boucle ; les données de la carte et les collisions ne changent pas. Le diagnostic natif `--capture-performance DOSSIER_NEUF`, accompagné de `--map CHEMIN`, mesure les vues de la grande carte de test, le défilement, le mode Tester, le placement et le pinceau, sans enregistrer le document.

## Contrôles et contenu variable

Vérifier signale les régions sans accès depuis le départ, les portes isolées et les marqueurs sans case accessible ou sans destination. Le calcul considère une porte fermée comme ouvrable ; une porte verrouillée ou sans courant reste bloquée. Les surcouches montrent les obstacles de déplacement, les obstacles de vision ou les cases hors de la région accessible. Cliquer une remarque recentre la vue.

Les outils Ennemis, Cave et Sortie créent des zones candidates. Propriétés conserve un nom, un profil ou une destination, et une quantité pour les ennemis. Le tirage choisit les positions accessibles, exclut le départ et évite les superpositions. Graine suivante change le résultat ; Même graine le reproduit. Un accès de cave et une sortie sont choisis parmi leurs zones candidates, tandis que chaque zone d'ennemis fournit sa quantité disponible.

Ces marqueurs et leur aperçu préparent les cartes. Ils ne génèrent pas encore les intérieurs des donjons et ne déclenchent pas les changements de zone ou les rencontres de la campagne. Les profils et destinations sont pour l'instant des références libres, sans validation contre le contenu du jeu.

## Récupération et bibliothèque locale

Toutes les 30 secondes, une carte modifiée est conservée dans un fichier de récupération distinct, en dehors d'un trait en cours. Une fermeture sans enregistrement conserve également cette récupération. Au prochain lancement normal, Reprendre restaure la session ; Archiver et ignorer conserve le fichier sous un autre nom. Ctrl+S reste l'enregistrement explicite de la carte.

Chaque enregistrement conserve jusqu'à cinq versions précédentes dans le dossier `.versions` à côté de la carte. Cartes > Versions permet d'en restaurer une ; cette restauration est annulable et doit être enregistrée pour être conservée. Le lancement normal rouvre la dernière carte existante utilisée. Les favoris, choix récents et états des calques sont conservés séparément.

Les ensembles se trouvent dans `artifacts/map-editor/ensembles`, les cartes habituelles dans `artifacts/map-editor/maps` et la récupération dans `artifacts/map-editor/recovery`. Les captures et preuves de la version actuelle sont dans `artifacts/editeur-outils-complets-2026-10-01`.

## Grande carte de test du 2 octobre

`Essayer_grande_carte.cmd` ouvre directement `artifacts/map-editor/maps/grande-ville-et-nature-2026-10-02.json`, près du départ dans la ville. Cette carte de 160 × 112 cases comprend 17 bâtiments urbains, six bâtiments dans le hameau, un marché, deux entrées de donjon, une rivière, deux ponts, un lac, des bois et des sentiers. Les bâtiments sont meublés ; leurs fonctions sont pour l'instant visuelles. Le décor enregistré est fixe et entièrement modifiable avec les outils habituels.

Depuis une fenêtre déjà ouverte, cliquer sur Ouvrir ou appuyer sur F6, sélectionner `grande-ville-et-nature-2026-10-02`, puis cliquer sur Ouvrir ou appuyer sur Entrée. Si la bibliothèque affiche un autre dossier, utiliser Autre chemin… et saisir `C:/Users/User/Desktop/project-RL/artifacts/map-editor/maps/grande-ville-et-nature-2026-10-02.json`. L'ouverture charge les sols, peintures, murs, objets et marqueurs du document ; F5 ou Ctrl+S enregistre ensuite les modifications.

La catégorie Objets > Nature ajoute 16 éléments : arbres, buissons, rochers, roseaux, tronc, herbes, souche, deux entrées de grotte, affleurement et ruine. Les petits rochers, roseaux, herbes et entrées de grotte sont traversables par défaut. Les entrées de cette carte ont un marqueur Cave avec une référence de destination ; la transition et l'intérieur restent à raccorder au jeu.

Les fichiers de composition et les captures sont dans `artifacts/grande-carte-test-2026-10-02`. `tools/build_large_editor_map.py` construit un document JSON de l'éditeur, avec la graine de composition fixe 20261002 ; il refuse d'écraser une carte existante. La validation et les captures utilisent le moteur natif : `map_editor.exe --map CHEMIN --capture-landscape DOSSIER_NEUF`. Le lanceur utilise `--center-on-spawn` pour ouvrir la vue près du départ sans déplacer le joueur.

## Navigation, chargement et animations du 2 octobre

Les commandes Sauver et Ouvrir restent directement accessibles en haut de la fenêtre. Une nouvelle carte reçoit un nom de fichier disponible, comme `nouvelle-carte-2.json`, pour que son premier F5 conserve les cartes déjà enregistrées. Ctrl+Maj+S permet de choisir un nom. La bibliothèque sélectionne la carte courante à son ouverture ; les flèches, Entrée et les boutons permettent de parcourir puis d'ouvrir les fichiers. Un fichier invalide est refusé sans remplacer la composition ni son chemin.

Les barres de défilement restent aux bords droit et inférieur du canevas. Le zoom garde le point sous le pointeur, dans les limites de la carte. La mini-carte indique le rectangle de la vue ; cliquer dessus permet de se déplacer directement. La palette conserve son propre défilement à la molette.

L'aperçu des fichiers se construit progressivement, avec un indicateur d'avancement et une résolution adaptée à sa taille affichée. Il possède son propre cache pour conserver celui de la carte en cours. La préparation des nouvelles tuiles peintes du canevas dispose d'un budget de quatre millisecondes par image : le sol de base reste visible jusqu'à préparation de la peinture. Ce budget ne représente pas le temps total d'une image. L'éviction du cache utilise un index ordonné, sans recherche linéaire dans toutes les tuiles. Les captures attendent la fin de la préparation.

F9 ou Animer active les reflets de l'eau peu profonde et profonde, la pulsation des lasers et de certains voyants, ainsi qu'un léger mouvement des arbres et buissons. Ces effets restent décoratifs : ils ne modifient ni les collisions, ni le document enregistré. Les caméras ont un voyant animé ; leur tête ne pivote pas encore. `Essayer_animations.cmd` ouvre une scène d'essai modifiable. Si le matériel graphique refuse le shader, le rendu statique reste utilisable.

Trois familles approuvées sont disponibles dans Murs : Béton (indice 5), Pierre (6) et Grillage (7). Chacune comprend les segments, les quatre coins et les raccords usuels. Les indices 0 à 4 restent conservés. Le grillage bloque le déplacement mais laisse passer la vision ; les portes conservent leurs cadres métalliques. `Essayer_nouveaux_murs.cmd` ouvre les trois enclos d'essai. Les images générées et leur provenance se trouvent dans `assets/prototypes/surface64/barriers-review`.

Le signalement de fermeture au chargement correspond à un événement Windows AppHangB1. L'aperçu auparavant calculé en une fois constituait une cause probable du blocage ; aucune trace de panique n'a été retrouvée. Trois ouvertures consécutives passent avec cette version, ainsi que la sauvegarde/relecture exacte et le refus d'un fichier invalide. Le lanceur conserve désormais les erreurs dans `artifacts/map-editor/derniere-erreur.log`. Les mesures, captures natives et limites de validation sont dans `artifacts/editeur-ux-2026-10-02/README.md`.
