# Génération d'équipement

2 octobre 2026. Génération de monde 141. Équilibrage provisoire.

## Récompense aléatoire à chaque mort hostile : génération 141

Chaque ennemi hostile mort laisse un exemplaire d'équipement tiré dans le
catalogue complet : armes de toutes les familles, protections corporelles,
tête, mains et pieds. Le modèle est choisi avec la même probabilité que chaque
autre modèle, sans contrainte de famille portée, de provenance ni de profondeur.
Tous les niveaux de modèle peuvent donc apparaître dès la surface. Le dernier
modèle obtenu est exclu du tirage suivant tant qu'un autre candidat existe.

La qualité et les propriétés sont ensuite tirées séparément avec les taux
validés de 0 à 6 bonus. L'objet de récompense ne détermine pas les attaques de
l'ennemi : son équipement de combat conserve ses règles de génération 140.
Les civils, alliés, compagnons et le joueur ne fournissent pas ces récompenses.
Un robot ou un animal hostile peut en fournir, comme demandé pour les monstres.

Le tirage utilise le hasard du moteur au moment de la mort. Une mort ne peut
produire qu'une récompense, y compris sur une case déjà occupée par des objets.
Le modèle précédent et le hasard sont conservés lors des voyages et des reprises.
Ramasser, consulter ou recharger ne relance pas les propriétés de l'objet.

Les caches, stocks de boutique et paris gardent leurs règles existantes. Les
parties jusqu'à la génération 140 gardent leur ancien butin ; une nouvelle partie
est nécessaire pour obtenir ce système. Le cache RLWS v11 conserve le modèle
précédent ; les caches plus anciens sont repris par le journal vérifié.

Le test de 600 morts réelles d'ennemis portant tous le même fusil a obtenu 127
modèles dans 18 familles, les six niveaux et les sept qualités, sans répétition
immédiate. Diagnostic natif : `--ui-cold-monster-rewards <nouveau-dossier>`.

## Diversité des modèles : génération 140

Cette section décrit le contrat historique de génération 140 pour les porteurs
et le contrat encore utilisé par les caches. La récompense de mort 141 est décrite
ci-dessus.

Le tirage choisit une famille, un niveau de modèle pondéré par la profondeur,
puis une base ; les bonus sont tirés séparément. Les modèles nommés ne sont
plus limités à une seule couche : un modèle peut apparaître jusqu'à deux
couches avant sa couche habituelle, et reste ensuite admissible.

À la surface, les trois premiers modèles de chaque famille sont disponibles.
Au sein d'une famille de modèles nommés, les niveaux 1/2/3 ont des poids
7000/2200/600, soit environ 71/22/6 %. Les porteurs humanoïdes restent dans
leur famille d'arme ; leur arme réellement équipée est celle qui tombe.
Les caches humanoïdes peuvent fournir les autres familles et les armures.
Les robots et animaux n'acquièrent pas de butin humanoïde.

La répartition de 0 à 6 bonus reste inchangée. Les boutiques et paris
conservent leurs stocks de niveau local. Les sauvegardes jusqu'à la génération
139 conservent leur pool historique, leur équipement et leurs tirages.

Les sections suivantes décrivent les révisions précédentes.

## Rareté par nombre de bonus : génération 135

Répartition validée pour chaque équipement généré dans les caches et sur les
porteurs humanoïdes admissibles, indépendamment de la chance de laisser un objet :

| Bonus | Probabilité | Couleur du nom |
|---|---|---|
| 0 | 40 % | Blanc |
| 1 | 20 % | Bleu |
| 2 | 16 % | Bleu |
| 3 | 11 % | Violet |
| 4 | 7 % | Violet |
| 5 | 4 % | Doré |
| 6 | 2 % | Doré |

Le nombre est tiré avant les propriétés. Celles-ci sont toutes distinctes et
compatibles avec l'objet, avec au maximum un effet spécial d'arme, compté dans
le total. Les poids propres aux affixes ne changent pas. La profondeur conserve
son rôle sur les modèles et les valeurs, pas sur la fréquence des grandes
combinaisons. Les petites protections peuvent aussi recevoir six bonus, mais
gardent leurs valeurs réduites. Un pool de mod comportant moins de six propriétés
admissibles exclut les nombres impossibles et renormalise les poids restants.

Les paris utilisent les poids 20/16/11/7/4/2, sans résultat blanc. Leur nom ne
révèle ni couleur de rareté, ni nombre, ni affixe avant l'achat. Les achats neufs
restent blancs ; une revente conserve les propriétés exactes de l'exemplaire.
L'inventaire affiche le nombre en plus de la couleur ; les petits écrans affichent
les longues listes de bonus sur deux colonnes.

Les générations 134 et antérieures conservent leurs tirages et empreintes.
Le cache moteur passe à RLWS v9 pour les six propriétés ; les caches antérieurs
sont refusés avant décodage et la reprise utilise leur journal vérifié. Les
anciens bonus JSON et les exemplaires possédés ne sont ni supprimés ni modifiés.
Diagnostic : `--ui-cold-rarity-equipment-small` et `--ui-cold-rarity-equipment-wide`.

Le futur service d'amélioration remplacera tous les bonus par un tirage complet
de 1 à 6 bonus, jamais zéro. Il n'est pas intégré par cette étape.

Les sections suivantes décrivent les étapes historiques et leurs anciens taux.

## Tête, mains et pieds : génération 134

Trois nouveaux emplacements indépendants complètent la protection corporelle.
Le catalogue compte 78 armes et 30 protections, soit 108 bases distribuées.

| Couche | Tête | Mains | Pieds |
|---|---|---|---|
| P1 | Calotte Cognefer | Gants Cognefer | Brodequins Cognefer |
| P2 | Heaume Bastion | Gantelets Bastion | Solerets Bastion |
| P3 | Casque Cerbère | Mitaines Cerbère | Bottes Cerbère |
| P4 | Masque Chrysalide | Gantelets Chrysalide | Bottines Chrysalide |
| P5 | Capuche de mue | Gants de mue | Bottes de mue |
| P6 | Visière Fantôme | Gants Fantôme | Bottines Fantôme |

Les noms communs désignent des modèles apparentés, pas un bonus d'ensemble.
Les protections peuvent être mélangées librement. Une paire de gants ou de
chaussures constitue un seul objet, dans un seul emplacement.

Valeurs provisoires : les mains et pieds apportent chacun 1 point d'armure.
La tête apporte 1 point en P1 à P4, puis 2 en P5 et P6. Ces pièces ne pénalisent
ni l'esquive ni le déplacement. Leurs noms ne confèrent aucun pouvoir inné.
L'armure de torse conserve une progression plus forte, de 1 à 6 ou de 2 à 7.

Les exemplaires magiques ont une ou deux propriétés. Le palier de leurs bonus
suit la progression 1, 1, 2, 2, 3, 3 : les valeurs sont plus modestes en profondeur,
sans modifier les bonus des objets existants. Les probabilités relatives des
affixes restent inchangées. Aucun effet offensif d'arme n'est ajouté.
Ce budget est porté par `equipment.reduced_affixes` dans la définition de l'objet.
Le générateur et les paris restaurés depuis une sauvegarde en déduisent le même
budget, sans modifier le format des propriétés de marchand déjà sauvegardées.

Les modèles locaux sont ordinaires : caches humanoïdes, achats blancs et paris
aux propriétés cachées. Ils ne tombent pas sur les robots ou les animaux. Les
bonus d'un exemplaire vendu restent conservés au rachat. Une nouvelle pièce ne
remplace que l'objet du même emplacement. L'inventaire précise Tête, Mains ou Pieds,
emploie trois icônes distinctes et trie les pièces équipées par emplacement.
La fiche Détails / comparer montre également l'objet actuellement porté.

Noms : `content/core/locales/fr_accessory_models.json5`.
Distribution : `content/core/equipment_loot/accessory_models.json5`.
Statistiques : les 18 définitions correspondantes dans `content/core/items/`.
Les générations 133 et antérieures conservent un seul emplacement corporel et
excluent ces nouveaux modèles. Les anciens inventaires et tirages restent intacts.
Diagnostic : `--ui-cold-accessory-equipment-small` ou
`--ui-cold-accessory-equipment-wide`, suivi du dossier de capture.

## Protections renforcées validées : génération 133

Une seconde famille de protections corporelles complète les six vestes existantes.
Le catalogue distribué compte désormais 78 armes et 12 armures, soit 90 bases.
Ces six modèles occupent le même emplacement, sans créer de nouveaux slots.

| Palier | Modèle | Protection | Malus d'esquive |
|---|---|---|---|
| P1 | Brigandine Cognefer | 2 | 5 |
| P2 | Plastron Bastion | 3 | 5 |
| P3 | Armure Cerbère | 4 | 5 |
| P4 | Carapace Chrysalide | 5 | 5 |
| P5 | Manteau de mue | 6 | 5 |
| P6 | Combinaison Fantôme | 7 | 5 |

Seule la protection progresse entre modèles de cette famille. Le malus est fixe,
s'applique uniquement lorsque l'armure est équipée et ne ralentit pas les déplacements.
L'esquive affichée et les jets réels utilisent la même pénalité. Les vestes gardent
leur protection de 1 à 6 sans ce malus. Les chiffres restent provisoires.
La masse de jeu reste fixée à 2 600 g pour chaque modèle ; aucun pouvoir inné de
régénération ou de furtivité n'est déduit du nom ou de l'apparence.

Les caches humanoïdes et les stocks neufs des marchands proposent le modèle de
la couche. P6 reste utilisé au-delà. Les armures blanches sont ordinaires ; qualité,
affixes et suffixes sont tirés séparément avec les poids existants. Aucun effet
offensif d'arme n'est ajouté aux armures. Les prix suivent ceux des vestes du même
palier : le compromis porte sur la protection et l'esquive, pas sur la rareté.
Les paris cachent leurs bonus avant l'achat et les exemplaires vendus conservent
leurs propriétés. Les robots et la faune ne laissent pas tomber ces vêtements.

Noms : `content/core/locales/fr_armor_models.json5`.
Distribution : `content/core/equipment_loot/armor_models.json5`.
Définitions : `content/core/items/brigandine_cognefer.json5` et les cinq autres
modèles. Le champ `equipment.evasion_penalty` vaut zéro s'il est absent.
Les générations 132 et antérieures retirent ces nouveaux modèles et gardent
leurs empreintes et tirages. Aucun changement imposé aux anciennes parties.
Diagnostic : `--ui-cold-reinforced-armor-small` ou
`--ui-cold-reinforced-armor-wide`, suivi du dossier de capture.

## Épées validées du 30 septembre 2026 : génération 132

Six épées complètent le catalogue de campagne : 78 modèles d'armes au total,
répartis en treize familles, plus les six bases d'armure existantes.

| Palier | Modèle | Dégâts de base |
|---|---|---|
| P1 | Épée courte | 5 |
| P2 | Glaive | 7 |
| P3 | Fauchon | 9 |
| P4 | Épée longue | 11 |
| P5 | Épée bâtarde | 13 |
| P6 | Espadon | 15 |

Profil commun : une seule cible à une case, précision neutre, dégâts cinétiques,
aucune pénétration ni effet inné, un tour par attaque, sans munitions. La Puissance
continue de modifier les dégâts avec un plafond d'Impact de 14. La masse de jeu
reste fixée à 1 400 g pour tous les modèles. Seuls les dégâts de base progressent.
Les noms ne confèrent ni allonge, ni parade, ni attaque de zone supplémentaire.
Ce classement sert le jeu et ne prétend pas établir une hiérarchie historique.

Les caches humanoïdes et les marchands proposent le modèle de leur couche,
P6 restant disponible au-delà. La famille est tirée sans rareté supplémentaire.
Les bonus sont tirés séparément : mêmes statistiques et poids que les autres
armes de mêlée, avec les effets braise, décharge, saignement et poison déjà
disponibles sur les haches. Les achats neufs restent sans bonus ; les paris
masquent les propriétés jusqu'à l'achat. La revente reste possible partout.
Les robots et animaux ne reçoivent pas ces armes. Les porteurs ennemis existants
ne changent pas de famille d'arme dans ce lot.

Noms : `content/core/locales/fr_sword_models.json5`.
Distribution : `content/core/equipment_loot/sword_models.json5`.
Statistiques et animations : `content/core/weapons/melee_*.json5` et
`content/core/visuals/melee_*.json5`. Les accords des affixes suivent le genre du nom.

Les générations 131 et antérieures excluent ces six modèles et conservent leurs
catalogues, empreintes et tirages. Les anciennes parties restent inchangées.
Diagnostic d'inventaire : `--ui-cold-sword-equipment-small` ou
`--ui-cold-sword-equipment-wide`, suivi du dossier de capture.

## Mêlée validée du 30 septembre 2026 : génération 131

24 modèles de mêlée rejoignent les 48 armes à distance. Chaque famille dispose
d'une base ordinaire par couche, avec qualité et affixes tirés séparément.
Ce classement est une progression de jeu, pas une hiérarchie historique.

| Palier | Couteaux et dagues | Armes d'hast | Haches | Masses et marteaux |
|---|---|---|---|---|
| P1 | Couteau de camp | Lance | Hachette | Gourdin |
| P2 | Couteau de chasse | Épieu | Hache de guerre | Gourdin ferré |
| P3 | Poignard | Pique | Hache d'abordage | Masse d'armes |
| P4 | Dague | Vouge | Hache d'armes | Masse à ailettes |
| P5 | Stylet | Pertuisane | Hache danoise | Marteau de guerre |
| P6 | Dague à rouelles | Hallebarde | Bardiche | Bec de corbin |

Seuls les dégâts de base changent entre modèles d'une même famille :

- Couteaux et dagues : 3, 4, 5, 6, 7, 8.
- Armes d'hast : 4, 6, 8, 10, 12, 14.
- Haches : 8, 10, 12, 14, 16, 18.
- Masses et marteaux : 6, 8, 10, 12, 14, 16.

Ces chiffres restent provisoires. Les modificateurs de Puissance et les défenses
continuent de s'appliquer. Portée, précision, pénétration, masse, plafond d'Impact,
cadence et animations restent identiques au sein de chaque famille. Les armes
d'hast gardent leur allonge à deux cases et leur pénalité de dégâts au contact.
Un nom historique n'accorde pas de capacité supplémentaire : pas de lancer de
hache, de parade ou d'attaque de zone ajoutés implicitement.

Les modèles locaux sont disponibles dans les caches humanoïdes, les achats
blancs et les paris aux propriétés cachées. Les porteurs humanoïdes de couteaux
utilisent le modèle de leur couche et laissent leur exemplaire à leur mort.
Tous les modèles gardent un prix de revente même hors de leur couche d'origine.
Les bonus conservent leurs poids précédents et ne changent pas la base.

Les noms sont dans `content/core/locales/fr_melee_models.json5`, la distribution
dans `content/core/equipment_loot/melee_models.json5` et les statistiques dans
`content/core/weapons/melee_*.json5`. Les accords des bonus suivent le genre de
chaque nom (par exemple une dague précise, un stylet précis).

Les anciennes parties gardent leurs armes et tirages : la génération 130 et les
précédentes retirent les nouveaux modèles des catalogues. Les neuf anciennes
bases de mêlée restent chargées mais ne sont plus proposées par les nouveaux
tirages de génération 131. Les épées et les changements d'armures ne font pas
partie de ce lot.

## Modèles validés du 30 septembre 2026 : génération 130

Les nouvelles parties distribuent 48 modèles d'armes à distance : huit familles,
avec un modèle par palier P1 à P6 (surface = P1). Les noms ont été validés par
le joueur. Ils sont éditables dans `content/core/locales/fr_firearm_models.json5`.
Les sections précédentes et suivantes restent l'historique des prototypes.

| Famille | P1 | P2 | P3 | P4 | P5 | P6 |
|---|---|---|---|---|---|---|
| Fusil à tir unique | Veyr R-12 | Kardan R-24 | Orvek R-36 | Tervan R-48 | Serdak R-60 | Noryk R-72 |
| Fusil d'assaut | Darven A-10 | Korsan A-30 | Vektor A-60 | Brenek A-80 | Talvek A-90 | Vornek A-120 |
| Fusil à pompe | Brask P-12 | Morn P-24 | Drek P-40 | Korven P-50 | Darsk P-70 | Valdran P-90 |
| Mitrailleuse lourde | Kordal M-20 | Varkan M-40 | Draven M-80 | Torgal M-100 | Kraven M-120 | Vornak M-160 |
| Lance-roquettes | Torven LR-2 | Karvek LR-6 | Ordan LR-9 | Bravan LR-12 | Dorek LR-16 | Karsen LR-20 |
| Lance-grenades | Rovak LG-3 | Darsen LG-6 | Korven LG-9 | Merdan LG-12 | Torvik LG-16 | Valrek LG-20 |
| Lance-flammes | Varek F-10 | Torak F-30 | Karn F-60 | Derven F-80 | Korgan F-100 | Vardek F-120 |
| Fusil à énergie | Neral E-12 | Seryn E-24 | Oryx E-48 | Telvar E-60 | Nerys E-80 | Odran E-100 |

Dans chaque famille, seuls les montants des dégâts directs et, pour les lanceurs,
de l'explosion progressent. Portée, précision, pénétration, cadence, nombre de
projectiles, zone, délai, consommation et effets intrinsèques restent identiques.
Les dégâts provisoires P1 à P6 sont respectivement :

- Fusils à tir unique et à énergie : 4, 6, 8, 10, 12, 14.
- Fusils d'assaut et mitrailleuses : 3, 4, 5, 6, 7, 8 par balle.
- Fusils à pompe : 8, 11, 14, 17, 20, 23 avant décroissance avec la distance.
- Lance-roquettes : 10, 13, 16, 19, 22, 25.
- Lance-grenades : impact 2, 3, 4, 5, 6, 7 ; explosion 9, 12, 15, 18, 21, 24.
- Lance-flammes : impact 4, 6, 8, 10, 12, 14 ; brûlure et feu au sol inchangés.

La famille reste tirée uniformément parmi celles admises par la provenance.
Les bornes `minimum_depth` et `maximum_depth` sélectionnent le modèle de la couche,
sans rareté supplémentaire pour une arme blanche. P6 reste utilisé au-delà de la
dernière couche prévue. Ces bornes limitent les trouvailles et les stocks neufs,
jamais l'utilisation ni la revente d'un objet déjà obtenu.

La qualité puis les affixes restent tirés séparément : mêmes poids relatifs,
1 à 3 propriétés, au maximum un effet spécial. Aucun effet de laboratoire
supplémentaire n'est introduit par ce changement. Les magasins proposent les
huit modèles locaux sans bonus ; les paris cachent leurs propriétés avant achat.
Les porteurs humanoïdes de fusils utilisent le modèle local à tir unique.
Ni les robots ni les créatures sans équipement ne reçoivent ces armes humaines.

Les onze anciens modèles distribués restent chargés pour les inventaires et les
profils initiaux existants, mais sont exclus des nouveaux tirages en génération
130. Les générations 129 et antérieures retirent seulement les 48 nouveaux
modèles et gardent leurs catalogues, empreintes et tirages précédents. Les autres
familles de mêlée et d'armures ne sont pas modifiées dans ce lot.

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
Le cache moteur utilise `RLWS` v10 ; les caches v1 à v9 utilisent le journal vérifié.
La génération 136 ajoute les fragments instables et l'artisan d'amélioration.
La génération 135 conserve exactement ses empreintes de règles, butin et monde.
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
