# Génération d'équipement — premier raccordement jouable

25 septembre 2026 — génération de monde 108. Équilibrage provisoire.

## Ajout du 30 septembre 2026 : lance et fusil à pompe

La génération 128 ajoute deux bases approuvées, sans tir préparé ni nouvelle armure :

- **Lance** : vraie attaque de mêlée à deux cases, dégâts réduits de moitié au contact. La Puissance et les techniques de mêlée restent applicables selon leurs conditions habituelles. Les murs bloquent l'allonge.
- **Fusil à pompe** : cône de quatre cases, une case de large au départ puis trois, prévisualisé avant confirmation. Dégâts de base 8 / 6 / 4 / 2 à une / deux / trois / quatre cases, avant armure. Un tir coûte deux munitions pour toute la zone, pas par cible. Aucun délai de récupération ajouté.

Le fusil projette une gerbe de chevrotines : grains métalliques distincts qui s'écartent selon le cône, puis petits éclats sur les cases occupées. Ce dessin ne multiplie ni les dégâts, ni les activations d'effets : une cible reçoit un seul impact direct par tir.

Ces valeurs sont provisoires. Les deux modèles sont distribués dans les caches humanoïdes de surface et de première couche, ainsi que dans les stocks blancs des marchands correspondants. Les paris de surface peuvent proposer leurs versions à bonus cachés. Les règles ordinaires d'affixes et de revente s'appliquent. Aucun robot ni animal ne reçoit ces objets par défaut ; les porteurs ennemis existants ne changent pas.

Une limite de distribution `maximum_depth: 1` évite que ces deux familles, qui n'ont pour l'instant qu'un modèle de début de partie, prennent la place d'équipement profond. Elle n'empêche ni de conserver l'objet, ni de le porter ou de le revendre plus bas. Les modèles supérieurs et leurs noms restent à proposer avant intégration.

Le profil de dégâts par distance appartient à la base, pas à un affixe : il s'applique aux dégâts directs après les bonus et avant les défenses, sans réduire les dégâts secondaires des effets. L'inventaire affiche les valeurs par distance. Les exemplaires sans bonus sont aussi disponibles au laboratoire.

Les générations 127 et antérieures retirent uniquement ces deux bases des règles et des tirages : aucune redistribution dans une partie déjà suspendue. Les sauvegardes conservent leur version.

## Arsenal approuvé du 30 septembre : génération 129

| Base | Fonctionnement initial | Coût d'une action |
|---|---|---|
| Fusil d'assaut | Trois balles de 3 dégâts, portée 7, précision -8 | 3 munitions |
| Mitrailleuse lourde | Quatre balles de 3 dégâts, portée 8, précision -25. Chaque rafale consécutive sans déplacement ajoute 8 de précision, jusqu'à +24 | 4 munitions et 8 chaleur |
| Lance-roquettes | Explosion immédiate de 10 dégâts, rayon 1, portée 7. Une seule application par cible, tireur inclus s'il est dans le souffle | 5 munitions |
| Lance-grenades | Impact de 2 dégâts sur l'occupant éventuel, puis grenade fixe au sol. Explosion de 9 dégâts, rayon 1, après deux actions suivantes. Portée 6 | 4 munitions |
| Hache de combat | 8 dégâts de base, précision -12, plafond matériel 20 | Aucun consommable |
| Marteau de guerre | 6 dégâts de base, pénétration 4, précision -8, plafond matériel 16 | Aucun consommable |

Ces valeurs sont provisoires. La Puissance affecte les impacts de mêlée suivant les règles existantes. Les rafales utilisent un impact et les défenses par balle, mais n'accordent qu'une activation réussie des effets secondaires par action. Le vol de vie additionne les dégâts directs réellement infligés puis applique son plafond une seule fois. Le tir soutenu se perd après une pause, un déplacement ou un changement d'arme. La surchauffe refuse le tir avant toute dépense.

Le lance-grenades peut viser un ennemi ou une case vide. La prévisualisation montre l'explosion future ; le compteur reste visible sur la grenade même si sa cible initiale meurt. Le délai est déterministe. Les murs bloquent la propagation. Dans cette première version, les affixes du lance-grenades se déclenchent sur le contact direct ; l'explosion différée est un payload autonome qui ne relance pas les effets de l'arme.

Deux effets d'instance supplémentaires : **de saignée** (2 dégâts par tour, 3 tours) et **de venin** (1 dégât par tour, 5 tours). Une blessure directe est nécessaire ; les cibles admises sont explicitement définies dans les statuts, sans deviner leur nature à partir du glyphe. Les machines non admises ne saignent pas et ne sont pas empoisonnées. Réappliquer un même statut rafraîchit sa durée sans additionner sa puissance ; les statuts différents restent indépendants.

Ces six bases proviennent uniquement des caches humanoïdes et des commerces correspondants, jamais automatiquement des robots ou de la faune. Les quatre modèles P1 sont distribués jusqu'à la profondeur 1 ; mitrailleuse et lance-roquettes sont P2, distribués jusqu'à la profondeur 2. Achat ordinaire sans bonus, paris masqués et revente conservent leurs règles. Leurs futurs modèles profonds restent à concevoir.

Les générations 128 et antérieures excluent uniquement ces bases, leurs deux nouveaux affixes et leurs statuts. Le cache binaire passe à `RLWS` v8 pour conserver le tir soutenu ; les caches plus anciens utilisent le journal de commandes vérifié. Aucune sauvegarde utilisateur n'est supprimée.

## Une base, plusieurs exemplaires

Une définition décrit le modèle : nom, dégâts, portée, capacité et contraintes.
Le générateur choisit une base puis produit un exemplaire blanc ou doté de
propriétés persistantes. Il ne crée aucune définition d'arme pour chaque combinaison.

Dix-huit bases du catalogue sont chargées : **six couteaux, six fusils et six
vestes**, soit une base par famille pour chacune des six couches accessibles.
Les profondeurs 6 et 7 sont réservées par l'atlas, sans descente jouable pour
l'instant ; elles utilisent le dernier palier si elles sont générées en diagnostic.

| Couche de référence | Palier | Couteau | Fusil | Veste |
|---|---|---|---|---|
| Surface | P1 | Couteau de camp | Fusil de patrouille | Veste matelassée |
| 1 | P2 | Couteau de sapeur | Fusil de guetteur | Veste de veille |
| 2 | P3 | Couteau céramique | Fusil à induction | Veste à fibres croisées |
| 3 | P4 | Couteau de chitine | Fusil de parallaxe | Veste de membranes |
| 4 | P5 | Couteau à dent vivante | Fusil à nerf tendu | Veste de peau seconde |
| 5 | P6 | Couteau du dernier seuil | Fusil de l'horizon fendu | Veste de la seconde ombre |

Les couteaux restent au contact, avec une progression de précision/pénétration.
Les fusils restent à tir unique et munitions limitées : les modèles de guet,
parallaxe et horizon privilégient la portée, avec moins de tirs disponibles et
une récupération supérieure. Les vestes utilisent l'emplacement de protection
existant (armure 1 à 6). Aucun nouveau pouvoir n'est accordé par le seul nom
« vivant », « induction » ou « ombre ». Les paramètres restent des essais.

## Tirage commun et rareté

Le moteur filtre d'abord la provenance, puis choisit une famille, un palier et
une base. Ajouter des modèles ne rend pas automatiquement leur famille plus
fréquente. La profondeur favorise les paliers élevés ; une exception supérieure
reste possible en surface. Seuls les paliers disponibles participent au tirage.

La qualité de l'exemplaire et la rareté de chaque propriété sont indépendantes.
Un exemplaire magique reçoit **1 à 3 propriétés distinctes au total**, avec au
maximum un effet spécial. Chaque propriété porte un poids relatif configurable :

| Propriété | Poids d'essai |
|---|---:|
| Puissance, Coordination, Résilience, PV maximum | 100 |
| Perception, Traitement | 80 |
| Précision, Énergie maximale | 70 |
| Pénétration, Dissipation | 35 |
| Braise | 20 |
| Décharge | 8 |

Ces nombres ne sont pas des pourcentages. À compatibilité égale, Braise possède
2,5 fois le poids de Décharge. Le pool change après chaque propriété sélectionnée :
pas de doublon et, après un effet spécial, aucun deuxième effet spécial.
Un poids nul désactive le tirage sans effacer les propriétés d'objets déjà créés.
Les paramètres des effets ne sont pas multipliés par la profondeur : Braise
applique la brûlure existante ; Décharge produit 2 dégâts électriques dans un
rayon de 1 autour de l'impact, sans toucher le porteur.

Les douze effets du catalogue de conception ont aussi des poids distincts dans
l'outil d'aperçu, mais seuls Braise et Décharge sont distribués par ce premier
raccordement. Le laboratoire exhaustif conserve ses cas imposés pour vérifier
chaque effet : leur présence n'est pas un indicateur de rareté en campagne.

## Où ces objets apparaissent

Dans les **nouvelles parties**, les régions de biome `core:human_habitat` et les
six biomes souterrains remplacent leurs tirages ordinaires de lame, lance-aiguilles
ou protection par ce générateur de source `humanoid_site`. Il s'agit de provisions
récupérables dans un lieu, pas de l'équipement des créatures qui l'occupent.
Nombre d'objets, positions et propriétaires issus des tables sont conservés.
Les biomes recherche, sécurité, réseau et corruption reçoivent désormais trois
caches et 3 à 5 tirages régionaux, comme maintenance et production ; aucun camp
ennemi supplémentaire n'est ajouté. Les villes restent sans butin procédural.
Les consommables, matériaux et le lance-flammes exceptionnel sont inchangés.
Le réglage initial produit 35 % d'exemplaires magiques, 65 % de blancs.

La nature sauvage de surface, les objets de départ et les récompenses de quête
ne changent pas. Les [marchands et paris](COMMERCE_EQUIPEMENT.md) utilisent ces bases
depuis la génération 107. Depuis la génération 108, les [Artilleurs et Soigneurs
de terrain](EQUIPEMENT_DES_ENNEMIS.md) portent une arme générée à leur création,
utilisée puis transférée à leur mort sans nouveau tirage. Les autres ennemis
restent inchangés. Avec le catalogue actuel, une source robotique, animale
ou anormale ne trouve aucun équipement admissible : aucun repli humain.

Le générateur utilise un flux RNG régional séparé. Consulter une fiche, charger
la partie ou revisiter un lieu ne change pas les propriétés. Les paris continuent
de masquer préfixes, suffixes, bonus et effets jusqu'à l'achat ; ils enchantent
le modèle annoncé avec ce même générateur, sans tirer une nouvelle base.

## Données et compatibilité

- `weapons/*.json5` et `items/*.json5` : les bases, sans affixe implicite.
- `weapon_affixes/*.json5` : suffixe, description et recettes d'effets existantes.
- `equipment_loot/*.json5` : familles, sources, paliers, accords, poids des bonus
  chiffrés et effets compatibles. `stats` commun évite de répéter la liste par base.

Le chargeur refuse les identifiants invalides, propriétés dupliquées, poids
négatifs/non entiers, paliers invalides, effets inconnus ou offensifs sur une armure.
Les poids et profils participent à l'empreinte du catalogue de butin.

Les propriétés sont enregistrées aussi dans les zones non encore visitées.
Le cache moteur utilise `RLWS` v8 ; les caches v1 à v7 utilisent le journal vérifié.
Les parties de génération 107 conservent les rencontres sans armes récupérables.
Les parties de génération 106 conservent leurs anciennes offres et leurs paris.
Les parties de génération 105 gardent leurs six bases P1/P2, leurs tirages
identiques et leur ancien monde sans les quatre nouveaux profils de caches.
Les versions 104 et antérieures excluent les dix-huit bases, les deux nouveaux
profils d'effet et le générateur. Les empreintes v105 ont été capturées avant
l'extension et sont vérifiées par un test de reprise. Aucune suspension utilisateur
n'est supprimée ou migrée en nouveau monde.

Les tests couvrent déterminisme, provenance, fréquences relatives, influence de
la profondeur, blancs sans affixe, familles indépendantes du nombre de modèles,
propriétés distinctes, dépôt/ramassage, zones différées et reprise. Le diagnostic
`--ui-cold-generated-equipment <nouveau-dossier>` montre un véritable exemplaire
généré dans l'inventaire ; `--ui-cold-deep-equipment <nouveau-dossier>` montre un
fusil P6 avec bonus et effet, sans toucher à une partie normale.
