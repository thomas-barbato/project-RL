# Surface urbaine et intérieur mobile

2 octobre 2026 · génération 140.

La surface vise une mégapole : des quartiers bâtis, quelques grandes voies,
des rues secondaires, des ruelles et des cours intérieures. La friche n'est
plus la direction principale. Le mode texturé et l'enrichissement de l'éditeur
restent en pause pendant ce travail sur la carte jouable.

## Intérieur et extérieur

L'**intérieur** est la ville dessinée avec les quêtes et les services. Son plan
reste conçu à la main. L'**extérieur** est le reste de la métropole, généré
selon des règles urbaines.

À partir de la génération 138, l'intérieur et le prologue de recyclage sont
placés ensemble près d'un bord choisi parmi nord, est, sud et ouest. Leur
position le long de ce bord varie aussi. Des symétries du plan permettent de
garder l'intérieur contre le bord et son accès vers l'extérieur. Les consoles,
portes, matériaux de quête, habitants, marchands et objectifs suivent ce
placement. Depuis la génération 140, le joueur commence **dans l'intérieur**,
près d'Elias, visible dès le premier écran. Le prologue de recyclage reste
présent dans le monde mais n'est plus le point de départ.

L'intérieur accueille cinq habitants mobiles, en plus du soigneur qui suit
sa routine. Les contacts de quête et le marchand restent à leur poste.
L'itinéraire indique les coordonnées de l'entrée pour les anciens départs,
puis celles d'Elias avant la première quête. Le suivi du retour à un donneur
de quête fonctionne aussi à l'intérieur de la grande carte.

Les décors des boutiques extérieures ne fournissent pas de services : les
libellés au survol et la légende F1 précisent leurs fonctions et obstacles.
Une proposition de rendu terminal industriel est isolée dans le diagnostic
`--ui-cold-city-visual-proposal` ; elle attend validation avant intégration.

Une nouvelle partie choisit une nouvelle graine lors de l'entrée dans la
création du personnage. Changer de protocole ou revenir à l'étape précédente
ne change pas cette graine. La graine et la révision de génération déterminent
la carte : une reprise restitue exactement le même monde.

## Règles de l'extérieur

La carte de départ mesure 192 × 128. La subdivision des quartiers produit
quelques avenues de cinq cases, des rues de deux ou trois cases et des ruelles
d'une case entre les façades. Les dimensions et la disposition des bâtiments
varient. Les grands bâtiments peuvent avoir plusieurs pièces ou une cour
traversée par un passage. Les façades disposent généralement de deux entrées,
avec des portes ordinaires à ouvrir.

Les consoles inactives, conteneurs, serveurs et piliers sont de vrais obstacles
pour le mouvement et la vue. Ils ne deviennent pas automatiquement des
machines utilisables ou destructibles. Les chaussées se distinguent des sols
intérieurs à l'affichage.

Les régions ordinaires de surface reçoivent aussi des quartiers. Les sites,
rencontres, caches, terminaux, installations et accès existants sont conservés.
La validation vérifie les circulations et les accès aux commandes murales.
Les villes régionales déjà dessinées gardent leur plan. Les souterrains ne sont
pas redessinés par cette étape.

Depuis la génération 139, certains îlots deviennent des parcs, boutiques,
supérettes, cafés ou immeubles résidentiels. Douze glyphes terminaux ajoutent
arbres de parc, bancs, jardinières, éclairages, poubelles, comptoirs, étagères,
présentoirs, distributeurs, tables, enseignes et repères de transport. Le
mobilier est placé hors des accès et des ruelles étroites. Les commerces sont
des lieux de décor ; seuls les marchands déjà fonctionnels proposent un service.

Les accès aux régions restent sur leurs bords respectifs ; si le groupe de
départ occupe un accès, celui-ci est décalé sur le même bord. Le passage du
secteur industriel reste à son emplacement extérieur actuel.

## Essayer et sauvegarder

Lancer `Essayer_expedition.cmd`, puis **Nouvel essai d'expédition**. Les nouvelles
parties normales utilisent également la génération 139. Reprendre une partie
antérieure conserve son ancienne carte et son ancien emplacement de départ.
La génération 137 garde ses quartiers urbains avec le départ original ; la
génération 138 garde le placement mobile sans ces nouveaux décors.

Le format des sauvegardes reste inchangé. L'essai et la partie normale gardent
leurs fichiers distincts. Les aperçus natifs et les contrôles de cette étape se
trouvent dans `artifacts/interieur-aleatoire-2026-10-02`. Le plan complet est un
diagnostic hors jeu ; la partie conserve la perception et la mémoire normales
du personnage.

Le nombre d'ennemis et les quantités de butin ne sont pas augmentés. Les lignes
de vue et les distances ont changé : les tests d'accès et de reprise ne
constituent pas une validation de l'équilibrage ou du rythme de l'expédition.

## Référence

Les articles officiels de Cogmind servent de référence pour séparer structure
et peuplement, sans reprendre ses cartes ou ses assets :
[Map Composition](https://www.gridsagegames.com/blog/2015/05/map-composition/) et
[Tunneling Algorithm](https://www.gridsagegames.com/blog/2014/06/mapgen-tunneling-algorithm/).
