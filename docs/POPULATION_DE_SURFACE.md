# Première population de surface

24 septembre 2026 — première tranche jouable, génération 100.

Mise à jour du 25 septembre : la génération 109 augmente la densité de surface.
Les paragraphes sur les ajouts sans hausse des effectifs décrivent les anciennes
générations ; voir le bilan ci-dessous pour les nouvelles parties.

## Densité actuelle — génération 109

- **Carte de départ (192 × 128)** : les dix adversaires historiques restent en
  place, auxquels s'ajoutent **20 à 24 humanoïdes équipés**. La faune existante
  est conservée séparément. Les Artilleurs et Soigneurs ne sont donc plus limités
  aux régions suivantes ; leurs armes réelles sont récupérables à leur mort.
- **Régions habitées (128 × 80)** : 12 à 16 tirages de rencontre, contre 3 à 6,
  en plus des 1 à 3 groupes de population existants.
- **Étendues sauvages (128 × 80)** : 14 à 18 tirages de rencontre, contre 4 à 7,
  en plus des 2 à 4 groupes existants. Un groupe peut contenir plusieurs acteurs ;
  ces chiffres ne comptent ni la faune ni les renforts.
- Les poids des Artilleurs et Soigneurs passent à 160 et 40. Les autres poids,
  caractéristiques de combat, niveaux, armures, faune et budgets de renforts ne
  changent pas. Ce sont des réglages d'essai, pas un équilibre définitif.

Le placement `spread_groups` laisse au moins 12 cases (distance de Chebyshev)
entre les acteurs de groupes différents et 16 cases autour des arrivées. Les
cases protégées, occupées ou inaccessibles depuis les accès sont exclues. Un
groupe peut rester compact ; les groupes déjà installés sont pris en compte.
Les premiers tirages visent les points d'intérêt existants, puis un échantillon
aléatoire favorise les espaces moins occupés. Dans les friches initiales, les
abords des ruines et axes de traversée ont des points d'ancrage dédiés ; une
marge supplémentaire écarte les nouveaux adversaires de la ville. Le terrain
et les règles de protection réelles ne sont pas modifiés.

La population est créée une seule fois : aucun ennemi ajouté derrière le joueur,
ni réapparition au retour. Les camps gardent leur renouvellement limité existant.
Les parties de génération 108 et antérieure gardent exactement leurs anciens
effectifs, tirages et positions, y compris après une reprise par journal. Une
**nouvelle partie** est nécessaire pour essayer ces changements.

Cette tranche concerne la densité, pas l'intégration des 36 entrées du bestiaire.
La prochaine tranche reste la validation et l'intégration des espèces profondes,
de leurs comportements et de leurs butins propres ; recherche, sécurité, réseau
et corruption ne sont pas peuplés par cette modification.

## Direction et périmètre

Le monde ne doit pas être rempli de robots. Un rôle de combat ne définit pas la
nature de son occupant : habitants humanoïdes, créatures et machines pourront
coexister. Direction confirmée : la surface est surtout peuplée d'habitants
humanoïdes, avec peu de faune. Plus on descend, plus la faune devient étrange ;
des robots sont également présents, sans devenir la population par défaut.
Les peuples, espèces et proportions exactes restent à définir ; aucun nom de
peuple n'est rendu canonique ici.
Les deux nouveaux adversaires de cette tranche sont humanoïdes et ne déclarent
pas de système électronique. Les anciens profils de prototype ne sont pas
reclassés rétroactivement. Les noms désignent leurs fonctions, pas leurs peuples.

Décision confirmée : les nouveaux PNJ extérieurs sont protégés dans cette
première version. Aucun objectif indispensable ne dépend d'eux. La réputation
reste reportée ; les commerces et soins existants gardent leur accès ordinaire.

## Rencontres ajoutées

- **Opérateur du tri** : sur la place, près du dépôt. Il parle de son travail
  et indique l'abri de l'éclaireuse. Il ne donne ni quête ni récompense.
- **Éclaireuse de la lisière** : dans un abri juste après la porte est, au sud
  de la route. Elle parle des groupes rencontrés dans les friches et donne des
  conseils tactiques. L'abri utilise la protection réelle des terrains ; son
  occupante reste à son poste et ne bloque pas l'accès. La protection n'est pas
  une simple promesse de dialogue.
- **Artilleur** (`a`) : rencontré dans les régions de surface générées. Une
  action annonce la case visée, la suivante tire sur cette même case. Un
  déplacement peut éviter le tir ; un obstacle opaque ajouté coupe celui-ci,
  et déplacer le tireur rompt sa visée. Une longue action du joueur peut
  cependant laisser passer plusieurs occasions ennemies.
- **Soigneur de terrain** (`s`) : rejoint un allié blessé réellement perçu,
  puis lui rend jusqu'à 3 PV au contact. Chaque soin consomme une des trois
  fournitures de son état persistant, sans produire d'objet ou d'XP. Il ne se
  soigne pas lui-même, n'aide ni le joueur ni les neutres, et ne connaît pas
  les blessés cachés. Il peut combattre lorsqu'il n'a pas de soin à effectuer.

Les deux profils ennemis complètent les tables de rencontres des deux biomes
de surface sans relever le nombre de groupes. Leur présence varie avec les
tirages et leurs emplacements ; ils ne remplacent pas les dix adversaires de
la carte initiale. Les profondeurs gardent leurs populations antérieures.
Les PV, fournitures, dégâts et poids sont des valeurs d'essai, pas un équilibre
définitif. Le soin restaure des PV par une primitive de soutien commune ; une
taxonomie biologique et des incompatibilités de soin ne sont pas simulées ici.

## Lisibilité et persistance

Le tir est signalé par un message, un état textuel et une croix encadrée sur
la case visée. La marque n'est visible que si le tireur et cette case sont
actuellement perçus. Elle ne dépend pas de F2. Les deux adversaires ont des
glyphes, des descriptions, des contacts CAPTEURS et des entrées de légende.
Les dialogues reprennent la navigation clavier/souris existante.

Visée, fournitures et conversations sont conservées à la suspension. Les tirs
déjà engagés et les soins progressent aussi dans les zones visitées hors écran,
sans communiquer les événements locaux au joueur. Les reprises 99 et
antérieures retirent les nouveaux contacts et profils, conservent leur ancienne
carte et leur table de tirage, puis vérifient le rejeu habituel.

Les diagnostics isolés `--ui-cold-surface-enemies` et
`--ui-cold-surface-scout` permettent de contrôler le rendu sans modifier une
partie du joueur. Cette tranche n'ajoute pas encore de faune, de migrations,
de nouvelle faction ou de nouvelle branche de campagne.

## Suite — génération 101

La [première faune de surface](FAUNE_DE_SURFACE.md) ajoute trois espèces
provisoires, classées par familles, niveaux de référence et habitats. Elle
complète les rencontres humanoïdes sans les remplacer. Le soigneur de terrain
ne soigne pas ces animaux. Une reprise de génération 100 conserve sa population
sans faune ; aucune ancienne carte n'est repeuplée automatiquement.

La génération 102 ajoute un quatrième animal, le Grignoteur de gravats : fuite
au contact perçu, défense seulement s'il est acculé. Les poids de familles,
le budget et le nombre de groupes restent identiques. Une reprise 101 conserve
ses trois espèces et ses tirages précédents.

La génération 103 ajoute le Brise-os, rare et solitaire, uniquement dans les
étendues sauvages. Sa morsure annoncée vise une case fixe, puis laisse deux
occasions de récupération immobile. Ni les friches initiales ni la population
humanoïde ne changent. Les reprises 102 conservent leurs anciennes tables.
