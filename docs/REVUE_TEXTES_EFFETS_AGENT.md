# Revue des textes et effets de la refonte

6 octobre 2026. Premier lot révisé : réveil, briefing, attributs, énergie,
progression, bonus de compétences dans le loot et compétences du Bastion.
Les choix demandés par le joueur sont retenus : 5 points d'attribut et
1 point de compétence par niveau, attributs sans plafond, compétences
apprises jusqu'à 10 avec un coût de 1 point par niveau, équipement pouvant
dépasser 10, départ avec le premier actif et un passif, et +10 % de vie
pour le passif initial du Bastion. Les détails ajoutés ci-dessous restent
à valider. Aucun n'est intégré au contenu jouable.

Les noms des cinq classes, leurs silhouettes et leurs orientations sont
déjà acceptés dans le [cadrage de la refonte](REFONTE_AGENT_CRYOGENISE.md).
Les chiffres ci-dessous sont des valeurs de départ à tester, pas un
équilibrage validé en partie.

L'accord du joueur sur la réponse précédente retient les exemples de
progression du Bastion présentés aux niveaux 1, 10 et 12, le transfert
du bonus de vie de l'implant vers le passif, les trois portées de bonus
de compétences et la famille de couleur cyan. Les coûts, limites,
formules nouvellement ajoutées et textes complets restent à valider.

## Comment valider

Chaque proposition possède un identifiant stable. Une validation peut
porter sur un texte, sur son effet mécanique ou sur les deux. Par exemple :
« B02 : texte accepté, protection à 20 % au lieu de 25 % ».

Les citations constituent les textes proposés au joueur. Les paramètres,
conditions et cas particuliers restent séparés. Valider ce lot n'autorise
pas les futurs sorts, les autres classes, les quêtes ou de nouveaux rendus
visuels. Les nouveaux effets visuels seront montrés en situation avant
leur intégration.

| Ensemble | Texte | Effet mécanique | Nouveau rendu visuel |
| --- | --- | --- | --- |
| N01 et N02 | À valider | Aucun effet de combat | Non proposé dans ce lot |
| S01 à S06 | À valider | À valider | Non proposé dans ce lot |
| E01 à E03 | À valider | À valider | Non proposé dans ce lot |
| P01 | À valider | Récompenses demandées retenues ; détails à valider | Non proposé dans ce lot |
| P02 et P03 | À valider | Cap, coût et bonus du loot retenus ; détails à valider | Non proposé dans ce lot |
| B00 | À valider | Transfert de vie vers le passif retenu ; autres paramètres à valider | À présenter séparément |
| B01 à B03 | À valider | Exemples aux niveaux 1, 10 et 12 retenus ; autres paramètres à valider | À présenter séparément |

## Réveil et mission

### N01 Réveil

Affichage proposé : introduction après le choix du personnage.

> La cuve s'ouvre. Vous reprenez votre souffle.
>
> Vous êtes dans une base de l'organisation qui vient de vous réveiller.
> Votre équipement vous attend. On vous demande de rejoindre le briefing.

L'identité de ceux qui ont placé l'agent en cryogénie et son passé
personnel restent à définir.

### N02 Briefing

Locuteur proposé : le responsable de mission, sans nom propre fixé.

> Nous avons des renseignements sur une offensive qui se prépare sous terre.
> Il faut l'arrêter avant qu'elle atteigne la surface.
>
> Commencez par les installations sous la base. Éliminez les premières
> menaces et cherchez d'où elles viennent. Nous avons besoin de comprendre
> ce qui se passe avant d'aller plus loin.

Le briefing annonce l'enquête et le nettoyage initial. Il ne révèle ni
l'organisation adverse, ni ses expériences sur les démons, ni leur échec.

## Attributs

Les six noms sont retenus. Leurs formulations et contributions chiffrées
ci-dessous attendent une validation. Les valeurs de départ restent fixes
par classe ; la progression permet ensuite de répartir des points libres.

Le joueur demande que les caractéristiques gardent un intérêt concret.
Les attributs et le niveau d'une compétence contribuent ensemble au
résultat : les niveaux renforcent le pouvoir, les attributs renforcent
les dégâts, la survie, la réserve ou le contrôle correspondants.

### S01 Force

> Renforce vos attaques de mêlée.

**Effet révisé proposé :** les dégâts de mêlée varient de 2 % par point de Force
autour de 5. Force 5 ne modifie pas les dégâts, Force 7 donne +4 %,
Force 4 donne −2 %, et Force 100 donne +190 %.
Le multiplicateur vaut 1 + 0,02 × (Force − 5). Il continue au-delà de 10.

Le bonus s'applique aux attaques de mêlée ordinaires et aux techniques
qui les renforcent, avant les défenses de la cible. Il n'augmente pas
automatiquement la portée, le nombre de coups ou les dégâts des sorts.

### S02 Agilité

> Améliore vos attaques physiques et votre esquive.

**Effet révisé proposé :** chaque point autour de 5 ajoute ou retire
2 unités de précision avec les armes et 2 unités d'esquive. Agilité 8
donne +6 à chacune ; Agilité 4 donne −2. Ce sont des valeurs de combat,
pas des points de pourcentage directement ajoutés à une probabilité.

**Bénéfice ajouté proposé :** bonus de dégâts physiques de 0,5 point de
pourcentage par point autour de 5, sur les attaques d'armes et les attaques
naturelles des formes. Multiplicateur : 1 + 0,005 × (Agilité − 5).
Agilité 10 donne +2,5 %, et Agilité 100 donne +47,5 %. Ce multiplicateur
continue de progresser lorsque la précision atteint une borne contre
une cible donnée. Il se combine avec Force en mêlée et Perception pour
les armes à distance. Les sorts ne reçoivent pas ce bonus de dégâts.

La précision et l'esquive continuent d'augmenter sans plafond d'attribut.
La chance de toucher les oppose selon la formule donnée dans les
conventions de résolution. Cette contribution n'accélère pas les actions.

### S03 Vitalité

> Augmente vos points de vie maximum.

**Effet proposé :** PV maximum avant bonus :
30 + 4 × (Vitalité − 5) + 2 × (niveau − 1).
Vitalité 7 donne 38 PV au niveau 1. Vitalité 100 donne 410 PV au niveau 1.
La contribution reste de 4 PV par point, sans plafond d'attribut.

La contribution aux interruptions est retirée de cette proposition.
Les règles de préparation des techniques seront revues avec les pouvoirs
concernés ; cette fiche ne supprime pas ce système du moteur existant.

Gagner des PV maximum ne rend pas les PV manquants. L'armure et la
résistance aux pouvoirs occultes restent des protections distinctes.

### S04 Perception

> Améliore vos attaques d'armes à distance et votre vision.

**Effets proposés :**

- Vision de base : 8 cases à Perception 5, puis une case supplémentaire
  tous les dix points au-dessus de 5. La distance obtenue est bornée entre
  4 et 16 cases : clamp(8 + floor((Perception − 5) / 10), 4, 16).
  Perception 4 donne 7 cases ; Perception 15 donne 9 cases ; 85 donne 16.
- Précision à distance : +1 unité par point au-dessus de 5, et −1 par
  point en dessous. Cette contribution s'ajoute à celle de l'Agilité,
  y compris lorsque la vision a atteint sa distance maximale.
- Dégâts des armes à distance : variation de 2 points de pourcentage par
  point autour de 5. Multiplicateur : 1 + 0,02 × (Perception − 5).
  Perception 10 donne +10 %, et Perception 100 donne +190 %.
  Ce bénéfice continue au-delà du palier de vision. Il s'applique aux
  tirs ordinaires et aux techniques qui les renforcent, sans augmenter
  la portée de l'arme, et ne renforce pas les sorts par ce même calcul.

Les obstacles et les conditions du lieu continuent à limiter la vision.
La portée de l'arme ne change pas. L'écoute conserve la propagation réelle
des sons ; cette proposition ne remplace pas cette propagation par un
cercle et ne révèle pas d'ennemi caché.

### S05 Intelligence

> Renforce vos drones et vos affaiblissements techno-occultistes.

**Effets proposés :**

- PV maximum et dégâts des drones : variation de 2 % par point autour
  de 5. Intelligence 8 donne +6 % ; Intelligence 4 donne −2 %.
  Le multiplicateur 1 + 0,02 × (Intelligence − 5) continue sans plafond.
- Durée des affaiblissements techno-occultistes : un tour supplémentaire
  tous les vingt points au-dessus de 5, avec au maximum trois tours
  ajoutés par cet attribut : min(3, floor(max(0, Intelligence − 5) / 20)).
  Sous 5, leur durée de base reste conservée. La durée liée au niveau de
  la compétence sera définie dans sa propre fiche.
- Puissance d'application des affaiblissements : +2 unités par point
  autour de 5, opposées à la résistance occulte de la cible. Les fiches
  précisent leur puissance de base. Cela conserve un intérêt pour le
  contrôle au-delà du palier de durée, contre toutes les natures de cibles.
- Énergie maximale : +2 par point, indépendamment de la Volonté, selon E01.

Le nombre de drones contrôlés dépendra des capacités apprises. Il ne reçoit
pas d'augmentation implicite dans ce lot. Les affaiblissements conservent
leur efficacité sur les machines, les êtres organiques et les démons.

### S06 Volonté

> Renforce vos sorts, vos transformations et votre résistance à l'occulte.

**Effets proposés :**

- Dégâts des sorts : variation de 2 % par point autour de 5, sans plafond.
  Multiplicateur : 1 + 0,02 × (Volonté − 5).
- Durée des transformations : un tour tous les vingt points au-dessus de 5, avec au
  maximum trois tours ajoutés par cet attribut. Formule identique à S05,
  avec Volonté. La durée de base de la forme reste conservée sous 5.
- Résistance aux altérations occultes : +2 unités par point au-dessus de
  5, et −2 par point sous 5. Cette valeur s'oppose à la puissance du
  pouvoir adverse, selon la formule des conventions de résolution.
- Énergie maximale : +2 par point, indépendamment de l'Intelligence.
  Les actions magiques des formes emploient aussi le multiplicateur de
  dégâts de Volonté ; leurs attaques physiques emploient Force et Agilité.

Les altérations physiques, les dégâts reçus et la résistance aux
interruptions physiques ne reçoivent pas tous ce bonus. Les transformations
et sorts précis feront l'objet de leurs propres fiches.

## Énergie et récupération

Les compétences technologiques et magiques partagent une réserve. Le nom
« Énergie » et les barèmes suivants sont proposés pour cette réserve.

### E01 Énergie

> Sert à utiliser vos pouvoirs magiques et technologiques.

**Capacité révisée proposée :**
30 + 2 × (Intelligence − 5) + 2 × (Volonté − 5) + 2 × (niveau − 1),
avant équipement et implants. Les deux attributs contribuent séparément.
Un personnage avec Intelligence 4 et Volonté 6 commence donc à 30 avant
son implant. Passer Intelligence de 4 à 5 augmente sa réserve de 2,
même si Volonté reste à 6. Ce changement évite qu'un attribut plus faible
ne fournisse aucun bénéfice de réserve.

**Régénération révisée proposée :** 3 % de l'énergie maximale par tour
hors combat, avec un minimum de 3 points ; 1 % en combat, avec un minimum
de 1 point. Arrondi supérieur, gain limité à ce qui manque. Une réserve
de 33 récupère toujours 3 ou 1 ; une réserve de 300 récupère 9 ou 3.
Consulter un menu ou attendre en temps réel ne rend rien.

**Phase révisée proposée :** la récupération utilise la phase commune
de [HUD03](REVUE_INTERFACE_EFFETS_AGENT.md#hud03-phase-de-combat-et-retour-au-calme),
partagée avec les drones déployés. Combat et accalmie utilisent le barème
de combat ; le barème hors combat commence lors d'un tour éligible au calme.
Trois tours complets sans hostile repéré, attaque effectuée ou reçue, ni
dégâts subis par l'agent ou ses drones permettent un premier tour éligible
au quatrième tour consécutif. Un ennemi caché et inconnu ne modifie pas
la récupération par sa seule présence. Ces nouvelles conditions attendent
validation ; les pourcentages proposés ci-dessus restent inchangés.

Les attaques ordinaires des armes gardent leurs propres coûts, notamment
leurs munitions. Une augmentation de réserve maximale ne la remplit pas.

### E02 Récupérer

> Prenez un moment pour récupérer de l'énergie.

**Effet révisé proposé :** consomme un tour et rend 10 % de l'énergie
maximale, avec un minimum de 6 points, en plus de la régénération normale.
Arrondi supérieur et gain limité à ce qui manque. Pour une réserve de 33,
la récupération totale reste de 7 points en combat ou 9 hors combat.
Pour une réserve de 300, elle vaut 33 ou 39. Les ennemis peuvent agir.

Si la réserve est déjà pleine, la commande est refusée sans faire avancer
le temps. Elle ne rend pas de PV.

### E03 Recharge énergétique

> Récupérez une partie de votre énergie.

**Effet révisé proposé :** un consommable rend 25 % de l'énergie maximale,
avec un minimum de 12 points, puis sa quantité diminue d'un. Arrondi
supérieur et gain limité à ce qui manque. Une réserve de 33 reçoit jusqu'à
12 ; une réserve de 300 reçoit jusqu'à 75. Son utilisation consomme un
tour ; la régénération normale de ce tour s'applique aussi.

Une réserve pleine entraîne un refus sans consommation ni tour. Si moins
de points manquent que le gain prévu, seule cette quantité est rendue et le consommable
est tout de même utilisé. Le modèle visuel du consommable reste à choisir.

## Progression

### P01 Gagner un niveau

> Vous avez gagné un niveau. Choisissez les capacités que vous voulez
> développer.

**Décision du joueur :** 5 points d'attribut libres et 1 point de
compétence par niveau gagné. Aucun plafond de conception sur les attributs.
Les valeurs de départ restent fixes par classe ; les points sont libres.
Tous les actifs et les passifs restent accessibles à toutes les classes.

**Départ demandé :** le premier actif de la classe et un passif déjà appris.
Proposition de niveau de départ : 1 pour chacun, sans point libre ajouté.
Le deuxième actif n'est pas offert. Pour le Bastion, le départ comprend
donc Frappe de rupture et Vitalité améliorée ; Égide occulte s'apprend ensuite.

**Détails proposés :** les points non dépensés peuvent être conservés.
Le niveau 1 n'accorde pas les gains d'un passage de niveau ; au niveau 10,
le personnage a gagné 45 points d'attribut et 9 points de compétence,
en plus de ses deux compétences de départ.
Les attributs restent au minimum à 1. Un passage de niveau ne rend pas
automatiquement les PV ou l'énergie manquants. L'évolution de la difficulté,
de l'expérience requise, des armes et des ennemis sera testée avec ces
nouveaux budgets ; cette fiche ne fixe pas une nouvelle courbe de campagne.

### P02 Apprendre et améliorer une compétence

> Dépensez un point pour apprendre cette compétence ou la renforcer.

**Décisions du joueur :** chaque niveau appris coûte 1 point, pour les
actifs et les passifs. Le plafond appris est 10. Le niveau effectif peut
dépasser 10 grâce aux objets équipés.

**Exception ajoutée par le joueur :** dans les
[arbres secondaires](REVUE_COMPETENCES_SECONDAIRES_AGENT.md), les actifs
s'achètent une seule fois pour un point ; leurs passifs conservent dix
rangs appris, à un point par rang. Tous ces passifs sont achetables
immédiatement, même avant l'actif associé, sans seuil de branche.
La règle d'équipement des passifs secondaires est précisée dans leur revue.

**Lecture proposée du cap :** 0 signifie non appris ; le premier achat
donne le niveau 1 ; les suivants vont de 2 à 10. Une compétence offerte
au départ est déjà à 1 et nécessite neuf achats pour atteindre 10.
Il n'y a pas dix achats supplémentaires après ce premier niveau.

**Calcul proposé :** niveau effectif = niveau appris + somme des bonus
de compétence applicables des objets équipés. Un niveau appris 10 avec
un bonus de +3 donne un niveau effectif 13. Les formules utilisent ce
13, sans le ramener à 10 et sans table limitée aux dix premiers niveaux.

Les bonus ne consomment pas de point et ne modifient pas le niveau appris.
Ils ne débloquent pas une compétence non apprise et ne satisfont pas ses
prérequis d'apprentissage. Les prérequis éventuels et niveaux d'accès
devront être indiqués dans chaque fiche, sans exclusivité de classe.
Aucune limitation à un achat par compétence et par niveau de personnage
n'est ajoutée : des points conservés peuvent être dépensés ensemble.
Un achat impossible ne dépense aucun point.

Apprendre un passif le rend actif en permanence. Son niveau effectif
évolue avec l'équipement. Chaque compétence doit annoncer ce que ses
niveaux améliorent, ses coûts et ses éventuelles bornes d'effet.

### P03 Bonus de compétences sur l'équipement

Formulations proposées pour les propriétés trouvées :

> +2 niveaux à Frappe de rupture.
>
> +1 niveau aux compétences de mêlée et de survie.
>
> +1 niveau à toutes les compétences apprises.

**Décision du joueur :** ajouter des bonus de compétences aux propriétés
trouvables dans le loot. Les objets équipés peuvent faire dépasser 10.

**Portées et valeurs proposées :**

| Portée du bonus | Valeurs par propriété |
| --- | --- |
| Une compétence précise | +1 à +3 niveaux |
| Une discipline | +1 à +2 niveaux |
| Toutes les compétences apprises | +1 niveau |

Les disciplines proposées reprennent les cinq orientations : mêlée et
survie, drones et affaiblissements, distance et esquive, magie offensive,
transformations. Elles regroupent les compétences sans les réserver à une
classe. Chaque compétence a une discipline principale pour ce calcul ;
B01, B02 et B03 appartiennent à mêlée et survie. Les quatre éléments restent
des familles de magie, sans bonus élémentaire supplémentaire dans ce lot.

Un bonus de compétence compte comme une propriété dans les 1 à 6 bonus
d'un équipement. Il peut apparaître sur une arme, une armure ou un accessoire,
quelle que soit la classe du personnage. Une même portée et une même cible
ne se répètent pas sur le même objet. Une propriété précise, une propriété
de discipline et une propriété générale peuvent se combiner ; plusieurs
objets s'additionnent. Par exemple, 10 appris +2 précis +1 discipline
+1 général donne 14 effectif.

Les bonus généraux seront moins fréquents que les bonus précis ; les
probabilités, paliers de profondeur et règles d'amélioration des objets
restent à présenter avant activation du loot. Le nombre de bonus ne
change pas automatiquement sa distribution. L'objet conserve sa famille,
son modèle et sa qualité ; le bonus ne vient pas de l'arme portée par
le monstre vaincu.

Seuls les objets équipés comptent. Déposer, vendre ou déséquiper un objet
retire ses bonus ; le niveau appris reste intact. L'affichage distingue
le niveau appris, le bonus d'équipement et le niveau effectif :

> Niveau 13 : 10 appris + 3 d'équipement.

**Affichage demandé par le joueur :** lorsqu'un bonus d'équipement augmente
la compétence, son niveau effectif apparaît dans une couleur différente.
La condition porte sur un bonus applicable strictement positif, même
avant le dépassement de 10 : 1 appris +2 d'équipement est déjà coloré.
Un objet porté dans l'inventaire ou un bonus visant une autre compétence
ne change pas cette couleur. Retirer le dernier bonus applicable rétablit
la couleur normale. Les chiffres restent lisibles sans dépendre de la couleur.

**Couleur retenue, rendu à présenter en aperçu :** cyan pour « Niveau 13 »
lorsqu'il est augmenté ; couleur neutre habituelle autrement. Le détail
des niveaux appris et ajoutés reste visible au survol ou dans la fiche.
Le choix du cyan est accepté ; sa nuance et sa lisibilité seront vérifiées
dans l'interface avant intégration.

Un passif est recalculé immédiatement ; une réduction du maximum de PV
ou d'énergie plafonne la quantité actuelle, et son augmentation ne soigne
ni ne recharge. Un pouvoir lancé conserve les paramètres calculés au
lancement jusqu'à son terme ; cela ne remonte pas les niveaux appris.
Le traitement précis des drones et des formes sera présenté dans leurs
fiches, notamment lorsqu'une transformation conserve l'équipement.

Cette inspiration retient la séparation entre points investis et niveaux
ajoutés par l'équipement, documentée par
[Blizzard dans les règles de compétences de Diablo II](https://classic.battle.net/diablo2exp/skills/basics.shtml).
Le plafond demandé pour ce jeu est 10 ; celui de Diablo II est 20.
Les portées ci-dessus adaptent les
[bonus de compétences de ses objets](https://classic.battle.net/diablo2exp/items/magic/pre.shtml)
au catalogue partagé entre toutes les classes de cette refonte.

## Bastion

### B00 Implant initial du Bastion

> Vous encaissez mieux les coups, mais disposez de moins d'énergie.

**Effets révisés proposés :** +1 point d'armure et −10 % d'énergie maximale.
Les 10 % de vie sont retirés de cet implant et donnés par le passif B03,
pour commencer avec un seul bonus de 10 %. Ce transfert est accepté.
Le malus d'énergie proposé s'applique au maximum issu
des attributs et du niveau ; les bonus d'équipement s'ajoutent ensuite.
Les arrondis de réserves vont vers l'entier inférieur.

Au niveau 1, avec Vitalité 7, Intelligence 4 et Volonté 6, le passif B03
donne 41 PV maximum au lieu de 38 ; avec la nouvelle formule E01,
l'implant proposé donne 27 points d'énergie maximum au lieu de 30.
L'armure du matériel de départ reste à fixer séparément.

Un implant trouvé remplace cette paire de traits. Il n'efface ni les
compétences ni les points d'attribut dépensés. Changer de maximum conserve
les quantités actuelles, plafonnées si nécessaire ; les échanges ne
permettent pas de se soigner ou de recharger gratuitement.

### B01 Frappe de rupture

> Portez un coup de mêlée plus puissant.

**Effet proposé :** une seule attaque de mêlée, avec +40 % de dégâts au
niveau effectif 1, puis +10 points de pourcentage par niveau supplémentaire.
Bonus en pourcentage = 40 + 10 × (n − 1), où n est le niveau effectif.
La formule continue au-delà de 10.

Elle garde la portée et les types de dégâts de l'attaque employée.
Le multiplicateur se combine avec celui de la Force avant les défenses.

**Paramètres proposés :** énergie = 6 + floor((n − 1) / 3) ; recharge de 3 tours ;
une action ordinaire. Aucun déplacement forcé, coup supplémentaire ou
affaiblissement n'est ajouté à cette première version.

| Niveau effectif | Dégâts supplémentaires | Énergie | Recharge |
| --- | --- | --- | --- |
| 1 | +40 % | 6 | 3 tours |
| 10 | +130 % | 9 | 3 tours |
| 12 | +150 % | 9 | 3 tours |

Il faut disposer d'une attaque de mêlée légale et d'une cible à portée.
Une tentative impossible ne dépense ni énergie, ni tour, ni recharge.
Une attaque légale qui manque consomme ses coûts. Le traitement sous
transformation sera fixé dans la revue des formes.

### B02 Égide occulte

> Vous subissez moins de dégâts pendant quelques tours.

**Effet proposé :** réduit les dégâts directs reçus après l'armure et
les résistances. Réduction en pourcentage =
25 + 50 × (n − 1) / (n + 9), où n est le niveau effectif.
Elle vaut 25 % au niveau 1 et continue au-delà de 10, avec des gains
progressivement plus petits ; elle reste strictement inférieure à 75 %.
Le calcul conserve sa précision ; l'affichage arrondit au dixième.

Elle s'applique à tous les types de dégâts directs. Le poison déjà actif
et les autres dégâts persistants n'en bénéficient pas dans cette version.

**Paramètres proposés :** énergie = 10 + floor((n − 1) / 3) ; durée de 4 tours ;
recharge de 8 tours ; une action ordinaire. La protection commence au
lancement et couvre les réponses ennemies pendant sa durée.

| Niveau effectif | Réduction | Énergie | Durée | Recharge |
| --- | --- | --- | --- | --- |
| 1 | 25 % | 10 | 4 tours | 8 tours |
| 10 | 48,7 % | 13 | 4 tours | 8 tours |
| 12 | 51,2 % | 13 | 4 tours | 8 tours |

Cette compétence n'est plus apprise au départ. Sa première acquisition
coûte 1 point ; ses neuf améliorations jusqu'à 10 coûtent chacune 1 point.

Deux Égides ne se cumulent pas. Relancer une Égide encore active est
refusé sans coût. Un changement d'équipement ne retire pas une protection
déjà lancée.

### B03 Vitalité améliorée

> Augmente vos points de vie maximum.

**Effet demandé au départ :** +10 % de PV maximum, à la place du bonus
de résistance aux interruptions.

**Progression proposée :** +2 points de pourcentage par niveau effectif
supplémentaire. Bonus en pourcentage = 10 + 2 × (n − 1).
Le bonus vaut +10 % au niveau 1, +28 % au niveau 10 et +32 % au niveau 12.
Il continue au-delà de 10 et s'applique aux PV maximum, pas à des soins.

L'interface propose de montrer séparément la valeur calculée :

> Vie maximum : +10 %.

Cette ligne varie avec le niveau effectif. Le texte exact reste à valider.

**Paramètres proposés :** passif permanent, sans coût d'énergie et sans
recharge. Les autres classes peuvent l'apprendre et l'améliorer. Les
bonus de vie en pourcentage s'additionnent avant leur application au
maximum de base augmenté des bonus fixes d'équipement. Le résultat est
arrondi à l'entier inférieur, une seule fois à la fin.

Avec 38 PV de base et sans autre bonus de vie, les niveaux effectifs
1, 10 et 12 donnent respectivement 41, 48 et 50 PV maximum. À la création,
le personnage commence avec son maximum initial complet. Plus tard, un
gain de niveau de ce passif ou un changement d'équipement n'accorde aucun
soin gratuit ; la vie actuelle est seulement plafonnée si le maximum baisse.

## Conventions de résolution proposées

Ces règles font partie de la validation des effets de ce lot.

- Les durées et recharges suivent le temps du jeu. Un tour de référence
  correspond au coût d'une action ordinaire ; une action plus longue
  fait écouler davantage de temps. Les menus ne comptent jamais.
- Aucun attribut n'est ramené à 10. Les probabilités restent entre 5 %
  et 95 %. Les valeurs opposées de combat ont un minimum de 1, avec
  chance de toucher = clamp(100 × précision / (précision + esquive), 5, 95).
  Les bases de référence proposées sont 100 en précision et 25 en esquive
  avant attributs, équipement et conditions. À égalité de profil, la
  référence donne donc 80 %. Les profils ennemis seront définis séparément.
  Pour une altération occulte, la même opposition compare la puissance
  du pouvoir à la résistance occulte, de base 25 avant Volonté et autres
  contributions ; la puissance propre à chaque pouvoir reste à définir.
- Les contributions d'attributs et d'implants aux dégâts s'appliquent avant
  les défenses ; les contributions de sources distinctes se multiplient.
  Les dégâts sont arrondis à l'entier inférieur à la fin de leur calcul.
  L'armure peut absorber totalement une attaque trop faible.
- Le niveau appris est enregistré séparément des bonus d'équipement.
  Toute résolution utilise le niveau effectif applicable et les formules
  continuent au-delà de 10. Les gains de dégâts ou de vie restent positifs ;
  une protection suit une courbe qui ne peut pas atteindre l'immunité.
- Les coûts modifiés sont arrondis à l'entier supérieur. Les capacités et
  bonus fixes de réserves sont arrondis à l'entier inférieur. Les gains
  proportionnels de récupération sont arrondis à l'entier supérieur,
  comme indiqué dans E01 à E03, puis limités à ce qui manque.
- Un refus avant l'exécution ne consomme ni temps, ni ressource, ni
  recharge, ni tirage aléatoire. Un échec d'une action légalement engagée
  conserve ses coûts annoncés.
- Les effets ne confirment pas une cible cachée et ne dévoilent aucune
  case à travers un obstacle par leur aperçu ou leur message de refus.

## Suite de la revue

Les descriptions et les capacités de l'Opérateur, du Traqueur, de
l'Arcaniste et du Métamorphe figurent dans le
[deuxième lot de revue](REVUE_COMPETENCES_AGENT_LOT_02.md), avec la même
séparation entre texte, paramètres et limites. Les quatre familles de magie,
les drones, les cinq transformations, les implants trouvés, les portails,
les objectifs de quêtes et leurs messages auront ensuite leurs propres
fiches. Chaque actif et passif présentera sa progression jusqu'à 10 et
au-delà par l'équipement. Leurs nouveaux effets visuels devront être
montrés en situation. Les taux de loot, les rangs de bonus trouvables et
la courbe de difficulté devront être validés et testés avec cette progression.

## S01-ATTR01 Première revue individuelle des attributs — 8 octobre 2026

La fiche reprend le coefficient de Force du brouillon, sans appliquer de
nouveau chiffre au moteur. Les six noms et les points par niveau restent
retenus. L’exemple Force dix puis onze consomme un point d’attribut ; classe,
équipement, autres attributs et compétences sont conservés. Le bonus linéaire
et sa portée, dont les attaques physiques des formes, sont soumis à l’accord
du joueur. Les composants, ordre des bonus et arrondis non montrés restent
à revoir. Aucun affichage d’arme fictive ni variance générale créé.

**S01 · Force**

*Renforce vos attaques de mêlée.*

**Effet proposé**

- Progression du bonus : **2 points de pourcentage par point de Force**.
- Référence à Force 5 : **0 %**. À Force 4 : **−2 %**.

**Exemple actuel — Force 10**

- Dégâts de mêlée et attaques physiques des transformations : **+10 %**.

**Avec 1 point supplémentaire — Force 11**

- Dégâts de mêlée et attaques physiques des transformations : **+12 %**.

Le bonus s’applique aussi aux compétences de mêlée et continue de progresser sans plafond.

Proposition : [S01-ATTR01](../docs/reviews/refonte-agent-2026-10-06/review-individual-attribute-S01-01.json).

## STAT-MODEL02 Révision proposée après comparaison avec Diablo II — 8 octobre 2026

Le joueur confirme cinq points d’attribut par niveau et l’absence de niveau
maximum, et juge trop forte la contribution de Force présentée dans S01-ATTR01.
La réponse « d’accord et que proposes tu ? » demande une nouvelle proposition ;
elle ne valide aucun coefficient de remplacement. Les anciennes propositions
restent conservées comme historique. Aucun texte ou effet n’est intégré.

Je propose **une progression linéaire, avec des bonus compatibles qui s’additionnent**, en conservant **5 points par niveau et aucun niveau maximum**.

Pour une première base d’équilibrage :

- **Force** : **+1 point de pourcentage de dégâts de mêlée par point**, y compris les compétences de mêlée et les attaques physiques des transformations.
- **Agilité** : **+2 en précision et +2 en esquive par point**. Ces valeurs sont comparées à celles de l’adversaire pour déterminer les chances de toucher et d’éviter une attaque.
- **Vitalité** : **+4 PV maximum par point**.
- **Perception** : **+1 point de pourcentage de dégâts d’armes à distance par point**, y compris les armes lourdes à distance.
- **Intelligence** : **+1 point de pourcentage aux dégâts et aux PV maximum des drones**, et **+2 énergie maximum par point**.
- **Volonté** : **+1 point de pourcentage aux dégâts des sorts**, y compris les attaques magiques des transformations, et **+2 énergie maximum par point**.

Pour les bonus en pourcentage, **5 dans l’attribut reste la référence à 0 %**.

**Le changement important concerne le cumul des dégâts.** Pour une compétence fondée sur les dégâts de l’arme, son bonus et celui de l’attribut s’additionnent.

Exemple avec une compétence qui inflige **150 % des dégâts de l’arme** :

- **Force 10** : bonus de Force **+5 %** → la compétence inflige **155 % des dégâts de l’arme**.
- Après avoir investi les cinq points du niveau, **Force 15** : bonus **+10 %** → **160 % des dégâts de l’arme**.

Dans cet exemple, le gain réel est d’environ **3,2 % de dégâts**. L’investissement reste utile, et son poids relatif diminue à mesure que les autres bonus augmentent.

Je retirerais aussi le bonus de dégâts physiques initialement proposé pour l’Agilité : avec la précision et l’esquive, elle possède déjà deux intérêts concrets. Les effets sur la vision, les contrôles et les durées de transformation seraient examinés séparément.

**Sans niveau maximum, les gains resteront illimités.** Leur équilibre dépendra donc aussi de la progression des armes et des monstres. Je partirais sur ces coefficients, puis comparerais plusieurs profils aux niveaux 10, 50, 100 et 200 lors des essais d’équilibrage. **Cette base reste une proposition à valider.**

Suivi : [STAT-MODEL02](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-02.json)
et [S01-ATTR02](../docs/reviews/refonte-agent-2026-10-06/review-individual-attribute-S01-02.json).
Les valeurs chiffrées proposées et leur cumul attendent la validation du joueur.

## STAT-MODEL03 Accord sur la base et précision sur les armes — 8 octobre 2026

Le joueur accepte la base STAT-MODEL02 et précise que la Force augmente les
dégâts des armes de mêlée, tandis que l’Agilité augmente ceux des armes à
distance en conservant ses effets sur la précision et l’esquive.

Règles chiffrées acceptées dans le périmètre présenté :

- S01 Force : un point de pourcentage par point pour les dégâts d’armes de
  mêlée et leurs compétences ; référence cinq, bonus nul à cette référence.
- S02 Agilité : un point de pourcentage par point pour les dégâts d’armes à
  distance et leurs compétences, dont les armes lourdes à distance ; deux
  unités de précision et deux unités d’esquive par point restent conservées.
- S03 Vitalité : quatre PV maximum par point.
- S05 Intelligence : un point de pourcentage par point pour les dégâts et
  les PV maximum des drones, et deux points d’énergie maximum par point.
- S06 Volonté : un point de pourcentage par point pour les dégâts des sorts,
  y compris les attaques magiques des transformations, et deux points
  d’énergie maximum par point.

La référence cinq s’applique aux contributions en pourcentage. La portée
physique des transformations déjà présentée avec la Force reste conservée ;
la répartition des composantes de dégâts et l’ordre avec les autres effets
restent à préciser. Le bonus de l’attribut et celui d’une compétence fondée
sur les dégâts de l’arme s’additionnent, dans le périmètre montré.

Cinq points d’attribut par niveau et aucun niveau maximum restent retenus.
Le joueur s’attend à des niveaux raisonnables durant les parties courantes ;
aucune borne numérique de niveau attendu n’est fixée par cette remarque.

### S04-MAPPING01 Clarification en cours

Le message ne précise pas si le bonus à distance de l’Agilité remplace celui
proposé pour la Perception ou se cumule avec lui. La première annonce du
transfert est une interprétation à confirmer. Une question a été présentée :

> Le bonus de dégâts à distance de l’Agilité remplace-t-il celui proposé pour la Perception, ou souhaites-tu que les deux se cumulent ?

Ni la suppression du bonus de Perception ni son cumul ne sont enregistrés
comme décisions acceptées en l’absence de réponse. Les effets de vision,
les probabilités exactes, les durées, les bases des jauges, les composantes
de dégâts et les autres détails non présentés conservent leur revue en cours.
Les chiffres des anciennes fiches de compétence restent des exemples
historiques : aucun recalcul de ces fiches ni effet de jeu n’est intégré ici.

Suivi : [STAT-MODEL02-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/attribute-model-acceptance-and-weapon-mapping-decisions.json),
[STAT-MODEL03](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-03.json)
et [S01-ATTR03](../docs/reviews/refonte-agent-2026-10-06/review-individual-attribute-S01-03.json).

## STAT-MODEL04 Retour accepté au bonus de Perception — 8 octobre 2026

Le joueur revient explicitement à la proposition STAT-MODEL02 :

> en faite non je vais accepter ta proposition avec perception.

S04-MAPPING01 est résolu. Le bonus de dégâts à distance demandé pour l’Agilité
au message précédent est retiré. Il ne se cumule pas avec la Perception.

La base chiffrée acceptée est désormais :

- S01 Force : un point de pourcentage par point pour les dégâts d’armes de
  mêlée et leurs compétences ; la portée physique des transformations
  présentée auparavant reste conservée.
- S02 Agilité : deux unités de précision et deux unités d’esquive par point,
  sans bonus de dégâts d’attribut.
- S03 Vitalité : quatre PV maximum par point.
- S04 Perception : un point de pourcentage par point pour les dégâts d’armes
  à distance et leurs compétences, dont les armes lourdes à distance.
- S05 Intelligence : un point de pourcentage par point pour les dégâts et
  les PV maximum des drones, et deux points d’énergie maximum par point.
- S06 Volonté : un point de pourcentage par point pour les dégâts des sorts,
  y compris les attaques magiques des transformations, et deux points
  d’énergie maximum par point.

Les contributions en pourcentage gardent cinq comme référence à bonus nul.
Le cumul additif montré, les cinq points d’attribut par niveau et l’absence
de niveau maximum restent acceptés. Les effets de vision, les bases des
jauges, les probabilités exactes, les composantes de dégâts, les autres
cumuls, les durées et les détails non montrés restent à définir ou à revoir.
Aucun texte RP supplémentaire, rendu natif ou changement du jeu n’est
validé par cette précision. Les anciens modèles restent des historiques.

Décision : [STAT-MODEL02-ACCEPT02](../docs/reviews/refonte-agent-2026-10-06/attribute-model-perception-acceptance-decisions.json).
Modèle courant : [STAT-MODEL04](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-04.json).

## P01-LEVEL01 Absence de plafond et préférence éventuelle — 8 octobre 2026

Le joueur confirme l’absence actuelle de niveau maximum. Cent est sa
préférence si un plafond devait être introduit ultérieurement ; cette
condition ne crée aucune limite active. La vie unique et les possibilités
limitées de soin motivent ce contexte. Cent peut servir de repère haut pour
les essais, sans être une estimation du niveau atteint dans une partie.
Les coefficients acceptés dans STAT-MODEL04 restent tous conservés.

Décision : [P01-LEVEL01](../docs/reviews/refonte-agent-2026-10-06/level-cap-optional100-context-decisions.json).
Modèle courant : [STAT-MODEL05](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-05.json).

## HP01-PROP01 Base et progression des PV proposées — 8 octobre 2026

Seuls les trente PV de référence et les deux PV maximum automatiques par
niveau sont nouveaux dans cette revue. Les quatre PV maximum par point de
Vitalité sont déjà acceptés. L’effet sur les PV actuels et l’ordre avec les
bonus ne sont pas fixés par ces exemples. Aucun effet n’est intégré.

On garde **aucun plafond actuellement** ; **100 reste le plafond éventuel** si nous décidons d’en instaurer un.

Passons aux **PV de base et à leur progression**. Je propose de reprendre les valeurs du brouillon :

**HP01 · Points de vie**

*Votre vitalité détermine les blessures que vous pouvez encaisser.*

**Base proposée**

- Niveau 1, Vitalité 5 : **30 PV maximum**.
- Gain automatique par niveau : **+2 PV maximum**.
- Chaque point de Vitalité : **+4 PV maximum**, déjà validé.

**Exemple actuel — Niveau 10, Vitalité 10**

- PV maximum : **68**.

**Au niveau 11**

- Sans dépenser les nouveaux points : **70 PV maximum**.
- Avec les cinq points investis en Vitalité : **90 PV maximum**.

Ces exemples sont calculés avant les bonus de PV des équipements, implants et passifs. **Les 30 PV de base et les +2 PV automatiques par niveau sont les nouvelles valeurs à valider.**

Proposition : [HP01-PROP01](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-HP01-01.json).

## HP01-ACCEPT01 Validation de la base des PV — 8 octobre 2026

Le joueur valide la fiche présentée avec « oui :) ca me va ». Le texte RP,
les trente PV maximum au niveau un avec Vitalité cinq et les deux PV maximum
automatiques par niveau après le premier sont acceptés. Les quatre PV par
point de Vitalité et les exemples soixante-huit, soixante-dix et quatre-vingt-dix
restent retenus. La validation n’ajoute aucune règle sur les PV actuels,
les soins au passage de niveau ou l’ordre avec les autres bonus.

Décision : [HP01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/base-HP-and-level-gain-acceptance-decisions.json).
Modèle courant : [STAT-MODEL06](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-06.json).

## E01-CAP01 Capacité de la réserve d’énergie proposée — 8 octobre 2026

Les deux points d’énergie maximum par point d’Intelligence et par point de
Volonté sont déjà acceptés. La référence trente et le gain automatique de
deux par niveau sont proposés séparément de la récupération, des quantités
actuelles et de l’ordre avec les autres bonus. Aucun effet n’est intégré.

Les **PV de base et leur progression sont validés**. Passons à la capacité d’énergie.

**E01 · Énergie**

*Votre réserve alimente vos pouvoirs technologiques et occultes.*

**Base proposée**

- Niveau 1, Intelligence 5 et Volonté 5 : **30 énergie maximum**.
- Gain automatique par niveau : **+2 énergie maximum**.
- Chaque point d’Intelligence : **+2 énergie maximum**, déjà validé.
- Chaque point de Volonté : **+2 énergie maximum**, déjà validé.

**Exemple actuel — Niveau 10, Intelligence 10, Volonté 10**

- Énergie maximum : **68**.

**Au niveau 11**

- Sans dépenser les nouveaux points : **70 énergie maximum**.
- Avec les cinq points répartis entre Intelligence et Volonté : **80 énergie maximum**.

Ces exemples sont calculés avant les bonus d’équipements, implants et passifs. **Les 30 points de base et les +2 automatiques par niveau sont les nouvelles valeurs à valider.**

Proposition : [E01-CAP01](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-E01-01.json).

## E01-CAP-ACCEPT01 Validation de la capacité d’énergie — 8 octobre 2026

Le joueur répond « oui je valide » à E01-CAP01. Le texte RP, les trente
points d’énergie maximum de référence au niveau un avec Intelligence et
Volonté cinq, les deux points automatiques par niveau et les exemples
soixante-huit, soixante-dix et quatre-vingts sont acceptés. Les contributions
de deux points par point d’Intelligence ou de Volonté restent conservées.
La récupération, les quantités actuelles au passage de niveau et les autres
interactions non montrées ne sont pas validées par la capacité seule.

Décision : [E01-CAP-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/energy-capacity-and-level-gain-acceptance-decisions.json).
Modèle courant : [STAT-MODEL07](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-07.json).

## E01-REG01 Récupération d’énergie proposée — 8 octobre 2026

Le repère commun Combat, Accalmie, Calme et son décompte de trois tours
sont déjà acceptés dans l’aperçu schématique. Les taux, leurs minimums,
l’arrondi et les exemples suivants sont proposés sans intégrer de changement.
Les conditions de phase et l’ordre exact des événements non montrés restent
à revoir ; cette fiche ne les fixe pas implicitement.

La **capacité d’énergie est validée**. Voici la proposition pour sa récupération.

**E01 · Récupération d’énergie**

*Votre réserve se reconstitue lentement, puis plus rapidement lorsque le calme revient.*

**Effets proposés**

- Combat et accalmie : **1 % de l’énergie maximale par tour**, minimum **1 point**.
- Calme : **3 % de l’énergie maximale par tour**, minimum **3 points**.
- Récupération arrondie à l’entier supérieur, limitée à l’énergie manquante.

**Avec une réserve maximale de 68**

- Combat et accalmie : **+1 énergie par tour**.
- Calme : **+3 énergie par tour**.

**Avec une réserve maximale de 300**

- Combat et accalmie : **+3 énergie par tour**.
- Calme : **+9 énergie par tour**.

Le retour au calme conserve le décompte de **trois tours déjà validé**. Ouvrir un menu ou attendre en temps réel ne rend pas d’énergie.

Proposition : [E01-REG01](../docs/reviews/refonte-agent-2026-10-06/review-energy-regeneration-E01-01.json).

## E01-REG-ACCEPT01 Validation de la récupération d’énergie — 8 octobre 2026

Le joueur répond « oui ca me va :) » à E01-REG01. Le texte RP, les taux
un pour cent en Combat et Accalmie et trois pour cent au Calme, leurs
minimums un et trois, l’arrondi supérieur et la limite d’énergie manquante
sont acceptés. Les exemples pour les réserves soixante-huit et trois cents,
et l’absence de récupération dans les menus ou l’attente réelle, restent
retenus. L’ordre exact des événements et les cas non montrés restent ouverts.

Décision : [E01-REG-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/energy-regeneration-acceptance-decisions.json).
Modèle courant : [STAT-MODEL08](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-08.json).

## P01-CURRENT01 Quantités actuelles lors des gains de maximum — 8 octobre 2026

Le brouillon historique P01 conservait les quantités actuelles lors de tous
les gains de maximum ; ce périmètre global n’avait pas été validé. La
proposition suivante distingue les gains permanents de niveau et de points
investis des autres sources. Elle ne modifie pas les deux décisions déjà
acceptées : Vitalité améliorée augmente le maximum sans soin, et le Colosse
conserve le pourcentage de vie. Aucun effet n’est intégré.

La **récupération d’énergie est validée**. Il reste à préciser l’effet des gains de maximum sur les quantités actuelles.

**P01 · PV et énergie lors de la progression**

**Règle proposée**

Quand un niveau ou des points d’attribut investis augmentent votre maximum, **votre quantité actuelle augmente du même montant**. Le nombre de points manquants reste identique.

**Exemple — PV**

- Avant : **40 / 68 PV**.
- Après le passage au niveau suivant : **42 / 70 PV**.
- Avec les cinq nouveaux points investis en Vitalité : **62 / 90 PV**.

**Exemple — Énergie**

- Avant : **20 / 68 énergie**.
- Après le passage au niveau suivant : **22 / 70 énergie**.
- Avec les cinq nouveaux points répartis entre Intelligence et Volonté : **32 / 80 énergie**.

Exemples avant toute autre récupération ou dépense, sans bonus de maximum.

**Équipements, implants, passifs et buffs**

- Un bonus de maximum conserve la quantité actuelle.
- Si le maximum diminue, la quantité actuelle est plafonnée au nouveau maximum.

Le **Colosse** conserve sa règle déjà validée : maintien du pourcentage de vie à la transformation et au retour.

Proposition : [P01-CURRENT01](../docs/reviews/refonte-agent-2026-10-06/review-current-reserves-on-progression-01.json).

## P01-CURRENT-ACCEPT01 Restauration complète au niveau — 8 octobre 2026

Le joueur corrige P01-CURRENT01, puis confirme la synthèse avec « oui voila :) ».
La restauration complète suit les gains du niveau et le recalcul des maximums.
Un investissement de points d’attribut ou un changement d’équipement ne
restaure rien. Les anciens exemples de conservation des points manquants
ne sont plus les règles courantes. Le traitement détaillé d’une baisse de
maximum, les buffs et les cas non montrés restent ouverts ; Vitalité améliorée
et le Colosse conservent leurs décisions déjà acceptées.

- **Montée de niveau** : les gains du niveau sont appliqués, puis les **PV et l’énergie sont entièrement restaurés jusqu’à leurs nouveaux maximums**.
- **Points investis dans les attributs** : les maximums concernés augmentent, sans restaurer les PV ou l’énergie actuels.
- **Équipement ajouté ou retiré** : les maximums sont recalculés, sans soin ni récupération d’énergie.

Décision : [P01-CURRENT-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/level-full-restoration-and-no-stat-equipment-refill-decisions.json).
Modèle courant : [STAT-MODEL09](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-09.json).

## S02-HIT01 Précision et esquive proposées — 8 octobre 2026

Les deux unités de précision et d’esquive par point d’Agilité sont déjà
acceptées. Les bases, le calcul, les bornes et le périmètre montrés ci-dessous
sont soumis à la revue. Le profil ennemi n’est pas un défaut de génération.
Aucune fiche de compétence, texte déjà accepté ou règle de jeu n’est modifié.

La règle de récupération au niveau est validée. Passons à la **précision et à l’esquive**.

**S02 · Précision et esquive**

*Votre agilité vous aide à toucher vos cibles et à éviter leurs attaques.*

**Base proposée — Agilité 5**

- Précision : **100**.
- Esquive : **25**.
- Chaque point d’Agilité : **+2 en précision et +2 en esquive**, déjà validé.

**Calcul proposé**

Chance de toucher : **précision ÷ (précision + esquive adverse) × 100**, limitée entre **5 % et 95 %**.

**Exemple actuel — Agilité 10**

Contre un adversaire ayant **100 en précision et 25 en esquive**, sans autres bonus :

- Votre précision : **110**.
- Votre esquive : **35**.
- Chance de toucher : **81,5 %**.
- Chance d’éviter son attaque : **25,9 %**.

**Avec cinq points supplémentaires — Agilité 15**

- Votre précision : **120**.
- Votre esquive : **45**.
- Chance de toucher : **82,8 %**.
- Chance d’éviter son attaque : **31 %**.

Ce calcul concerne les attaques d’armes et les attaques physiques des transformations. Les valeurs de l’adversaire sont un exemple ; les monstres auront leurs propres statistiques.

Proposition : [S02-HIT01](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-accuracy-evasion-01.json).

## S02-HIT-ACCEPT01 Validation de la précision et de l’esquive — 8 octobre 2026

Le joueur répond deux fois « ca me va :) » à la dernière fiche présentée.
Le texte RP, la référence cent en précision et vingt-cinq en esquive avec
Agilité cinq, le calcul de chance de toucher, ses bornes cinq à quatre-vingt-
quinze pour cent et le périmètre attaques d’armes et attaques physiques des
transformations sont acceptés. Les deux exemples et la contribution déjà
acceptée de deux unités par point restent retenus. Le profil d’adversaire
reste illustratif. L’annonce de la revue de vision pendant l’interruption
ne vaut pas présentation ou validation de ses nouveaux chiffres.

Décision : [S02-HIT-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/accuracy-evasion-formula-and-reference-acceptance-decisions.json).
Modèle courant : [STAT-MODEL10](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-10.json).

## S04-VISION01 Vision liée à la Perception proposée — 8 octobre 2026

Les dégâts à distance liés à la Perception sont déjà acceptés. La vision
est proposée ci-dessous, sans bonus supplémentaire de précision ou d’écoute.
La base est huit cases ; seules les tranches au-dessus de la référence
ajoutent de la portée. La baisse à sept au-dessous de cinq du vieux brouillon
n’était pas validée et n’est pas retenue dans cette nouvelle proposition.
Aucun effet ni rendu de jeu n’est intégré.

La **précision et l’esquive sont validées**. Passons à la vision liée à la Perception.

**S04 · Vision**

*Vous repérez les menaces plus loin autour de vous.*

**Effets proposés**

- Portée de base : **8 cases**.
- Bonus : **+1 case par tranche de 10 points de Perception au-dessus de 5**.
- Portée maximale : **16 cases**.

**Exemple actuel — Perception 10**

- Portée de vision : **8 cases**.
- Bonus de dégâts des armes à distance : **+5 %**, déjà validé.

**Avec cinq points supplémentaires — Perception 15**

- Portée de vision : **9 cases**.
- Bonus de dégâts des armes à distance : **+10 %**.

Les obstacles continuent à limiter la vision. La portée des armes se calcule séparément.

Au-delà du plafond de vision, la Perception continue à augmenter les dégâts à distance.

Proposition : [S04-VISION01](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-perception-vision-01.json).

## S04-VISION-FEEDBACK01 Réserve sur la croissance de vision — 8 octobre 2026

Le joueur juge que la proposition S04-VISION01 risque d’offrir trop vite une
visibilité excessive sur la carte. Cette réserve ne valide aucune solution
de remplacement. La proposition de croissance jusqu’à seize cases est
retirée de la revue courante ; son texte RP est également retiré. Le bonus
de dégâts à distance déjà accepté reste conservé. Une base fixe de huit
cases est présentée séparément, sans ajouter de bonus de détection, de
précision, de critique ou d’écoute. Aucun effet de jeu n’est modifié.

Retour : [S04-VISION-FEEDBACK01](../docs/reviews/refonte-agent-2026-10-06/perception-vision-growth-reservation-and-withdrawal-decisions.json).
Modèle courant : [STAT-MODEL11](../docs/reviews/refonte-agent-2026-10-06/review-attribute-model-11.json).

## S04-VISION02 Vision de base fixe proposée — 8 octobre 2026

Je retire donc la proposition d’augmentation de la vision par la Perception.

Je propose de conserver une **portée de vision de base fixe de 8 cases**, indépendante de cet attribut.

La **Perception garde son bonus de dégâts à distance déjà validé** :

- Perception 10 : **+5 %**.
- Perception 15 : **+10 %**.
- Progression : **+1 point de pourcentage par point de Perception**, référence 5.

Elle conserve ainsi un intérêt pour les armes à distance et les armes lourdes. **Seule la vision de base fixe à 8 cases reste à valider.**

Proposition : [S04-VISION02](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-perception-vision-02.json).
Les règles de vision partagée du drone et les autres validations restent
conservées. La portée de base proposée ne fixe pas implicitement de plafond
pour les autres effets de vision encore à revoir.

## S04-VISION-ACCEPT01 Vision de base fixe validée — 8 octobre 2026

Le joueur répond « ok :) » à la proposition de vision de base fixe à huit
cases, indépendante de la Perception. Ces deux règles sont acceptées.
Le bonus de dégâts à distance de la Perception reste conservé, ainsi que
le partage de vision des drones. La croissance de vision précédemment
proposée reste retirée. Les autres interactions et le rendu natif ne
reçoivent aucune validation implicite ; aucune intégration au jeu.

Décision : [S04-VISION-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/fixed-base-vision8-acceptance-decisions.json).
Fiche acceptée : [S04-VISION-ACCEPTED03](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-perception-vision-03.json).

## S06-RESIST01 Résistance occulte proposée — 8 octobre 2026

Les gains de dégâts des sorts et d’énergie de la Volonté restent acceptés.
Le gain de résistance occulte, sa base et son calcul ci-dessous constituent
une nouvelle proposition ; l’ancien brouillon ne vaut pas acceptation.

La **vision de base fixe à 8 cases** est validée.

**S06 · Résistance occulte — proposition**

*Votre volonté vous protège des malédictions et des entraves occultes.*

**Base — Volonté 5**

- Résistance occulte : **25**.
- Par point de Volonté : **+2 en résistance occulte**.
- Résistance minimale : **1**.

**Calcul**

Chance de subir l’effet : **puissance adverse ÷ (puissance adverse + résistance) × 100**, limitée entre **5 % et 95 %**.

**Exemple actuel — Volonté 10**

Face à un pouvoir ayant **100 en puissance**, sans autres bonus :

- Résistance occulte : **35**.
- Chance de résister : **25,9 %**.

**Avec cinq points supplémentaires — Volonté 15**

- Résistance occulte : **45**.
- Chance de résister : **31 %**.

Cette protection concerne l’application des contrôles et malédictions occultes, comme **Silence** ou **Désorientation**. Les dégâts, le Gel, la Brûlure et le Poison conservent leurs règles propres.

Le profil adverse est illustratif. Les chances des compétences déjà
acceptées, les exceptions pour les élites et les boss, ainsi que les
règles de Gel, Brûlure et Poison sont conservées. Les interactions entre
les probabilités des fiches et cette résistance seront revues séparément.
La proposition concerne la protection du joueur ; les compétences des
monstres et leurs paramètres restent à définir. Aucun nouvel effet visuel
n’est présenté et aucune règle n’est intégrée au jeu.

Proposition : [S06-RESIST01](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-occult-resistance-01.json).

## S06-RESIST-ACCEPT01 Résistance occulte validée — 8 octobre 2026

Le joueur confirme « oui ok :) ca me va » puis demande de préparer les
résistances pour les voir ensuite. La fiche S06-RESIST01 est acceptée pour
le texte RP, la résistance de base 25 avec Volonté cinq, le gain de deux
unités par point, le minimum d’une unité et la formule proposée bornée
entre cinq et quatre-vingt-quinze pour cent. Son champ reste celui des
contrôles et malédictions occultes du joueur. Les autres règles et les
exemples montrés sont conservés ; les cas non présentés ne sont pas validés.

La suite autorisée est une proposition complète des autres résistances,
avec leurs interactions. Aucun chiffre non montré et aucun effet visuel
n’est validé par délégation ; aucune intégration au jeu n’est réalisée.

Décision : [S06-RESIST-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/occult-resistance-acceptance-and-full-resistance-review-request-decisions.json).
Fiche acceptée : [S06-RESIST-ACCEPTED02](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-occult-resistance-02.json).

## RESIST-LOT01 Résistances préparées pour revue — 8 octobre 2026

La proposition rassemble R01 résistances aux dégâts, R02 armure, R03 dégâts
persistants, R04 contrôles et R05 exceptions et affichage. Les nouvelles
règles sont proposées, les validations antérieures restent conservées.
Le plafond proposé est de 75 %, avec un plancher de −50 %, des bonus
additionnés avant les malus et les limites, et les mêmes bornes pour le
joueur et les monstres. Les règles complètes et exemples présentés figurent
dans le document lié ; rien n’est intégré au moteur.

Proposition : [Résistances de la refonte](REVUE_RESISTANCES_AGENT.md).
Manifeste : [RESIST-LOT01](../docs/reviews/refonte-agent-2026-10-06/review-resistance-system-01.json).

## R03-ELEMENT01 Association élémentaire et retrait de Radiation — 9 octobre 2026

Le joueur remarque que les radiations ne correspondent à aucun contenu
jouable identifié. L’audit confirme un type technique, du texte et des
tests, sans arme, monstre ou danger trouvé infligeant ces dégâts.
L’ajout non validé est retiré de la proposition, sans supprimer le moteur.

Le joueur demande d’associer Brûlure à la résistance au feu, Empoisonné à
la résistance au poison et Gel à la résistance au froid. À la question
R03-GEL01, il choisit « Dégâts de froid et chance de Gel » : direction
acceptée, durée inchangée. La résistance occulte validée et les autres
règles antérieures restent conservées. Le calcul R03-GEL02 et ses exemples
48 % puis 24 % sur un boss sont proposés ; ils ne sont pas encore acceptés.
Aucun autre chiffre du lot précédent n’est validé par ce retour.

Décision : [R03-GEL01](../docs/reviews/refonte-agent-2026-10-06/radiation-placeholder-withdrawal-and-elemental-status-resistance-decisions.json).
Proposition actuelle : [Résistances](REVUE_RESISTANCES_AGENT.md).
Manifeste : [RESIST-LOT02](../docs/reviews/refonte-agent-2026-10-06/review-resistance-system-02.json).

## R03-GEL02-ACCEPT01 Formule élémentaire du Gel validée — 9 octobre 2026

Le joueur répond « ok on continue » à la formule et à l’exemple proposés
sous R03-GEL02. La chance avant résistance est multipliée par cent moins
la résistance au froid, puis divisée par cent. Avec quatre-vingts pour
cent avant résistance et quarante pour cent de résistance au froid, la
chance devient quarante-huit pour cent, puis vingt-quatre sur le boss
d’exemple avec le malus déjà accepté. La durée reste inchangée.

Cet accord porte sur les éléments affichés de R03-GEL02. Il ne valide pas
implicitement les plafonds du lot précédent, l’armure, les arrondis, les
cas de résistance négative ni les valeurs à recalculer des compétences.
Les associations élémentaires et les autres règles acceptées sont conservées.

Décision : [R03-GEL02-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/freeze-elemental-chance-formula-acceptance-decisions.json).

## RESIST-LOT03 Dernières règles communes proposées — 9 octobre 2026

R01, R02, les parties encore ouvertes de R03 et la direction d’affichage
R05 sont regroupées pour la suite de la revue. Aucun nouvel attribut ni
nouvel effet de compétence n’est intégré. Les valeurs et textes figurent
dans le document de résistances ; les règles présentées restent proposées.
Les cas techniques non montrés, les paramètres de loot, les profils de
monstres, les formules de rang et le rendu natif restent à revoir.

Proposition : [Résistances](REVUE_RESISTANCES_AGENT.md).
Manifeste : [RESIST-LOT03](../docs/reviews/refonte-agent-2026-10-06/review-resistance-system-03.json).

## RESIST-LOT03-ACCEPT01 Règles communes des résistances validées — 9 octobre 2026

Le joueur confirme « je valide :) ». L’accord porte sur les textes RP et
les valeurs présentées de R01 et R02, les règles affichées de R03 et la
direction d’affichage R05. Cinq familles de dégâts, base zéro, limites
moins cinquante et plus soixante-quinze pour cent, bonus additionnés avant
les malus et limites, surplus utile, armure fixe une fois par impact avec
plancher zéro, résistance courante, saignement sans seconde armure et
arrondi des dégâts ordinaires après les protections sont acceptés. Le Gel
en cas de faiblesse est plafonné à cent pour cent avant importance.
Les exemples et l’intitulé de résistance au froid montré sont acceptés.

La direction d’affichage ne vaut pas accord sur un rendu natif. Les profils
des monstres et familiers, les bonus précis d’objets, les cas de pénétration,
les calculs de rang concernés et les autres cas non montrés restent à revoir.
Aucune intégration au moteur.

Décision : [RESIST-LOT03-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/common-resistance-system-acceptance-decisions.json).
Manifeste accepté : [RESIST-LOT04](../docs/reviews/refonte-agent-2026-10-06/review-resistance-system-04.json).

## C01-CRIT01 Coups critiques proposés — 9 octobre 2026

La fiche DI04 comportait déjà un bonus de quinze points de chance de
critique au rang effectif onze ; sa base de cinq pour cent était fictive,
sa règle générale et son multiplicateur n’étaient pas fixés. La proposition
ci-dessous les soumet explicitement. Aucune fiche de compétence acceptée
n’est réécrite et aucun attribut ne reçoit de nouvelle progression.

Les règles communes des **résistances sont validées**. Passons aux critiques.

**C01 · Coups critiques — proposition**

*Vous exploitez une ouverture pour infliger une blessure dévastatrice.*

**Valeurs de base pour l’agent**

- Chance de critique : **5 %**.
- Dégâts critiques : **200 % des dégâts directs**, avant l’armure et les résistances.
- Les bonus de chance s’additionnent en points de pourcentage, avec une chance finale entre **0 % et 100 %**.

**Exemple**

Une attaque infligeant **40 dégâts physiques**, contre **12 armure** :

- Coup normal : **28 dégâts subis**.
- Coup critique : **80 avant armure → 68 dégâts subis**.

**Attaques concernées**

Mêlée, tir, armes lourdes, sorts infligeant des dégâts directs et attaques des transformations. Une attaque manquée ne peut pas devenir critique.

Pour une rafale ou une attaque de zone, chaque impact sur une cible possède son propre jet de critique.

Les dégâts persistants, les renvois et les collisions ne font pas de jet de critique supplémentaire. Un saignement ou une collision calculé sur les dégâts réels du coup profite déjà de l’augmentation du coup initial.

Le critique ne modifie pas la chance ni la durée des statuts.

**Tir de précision — rang 11**

Avec la base proposée de **5 %** et son bonus déjà validé de **15 points**, ce tir atteint **20 % de chance de critique**, sans autres bonus.

**Ces règles de critique restent à valider.**

Les profils de critique des monstres et des familiers ne sont pas fixés
par cette base de l’agent. Leurs statistiques, les détails des composantes,
les formules complètes de dégâts, la variance des armes et le rendu visuel
restent à revoir. Aucun nouveau feedback graphique n’est créé.

Proposition : [C01-CRIT01](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-critical-hits-01.json).

## C01-CRIT01-ACCEPT01 Coups critiques validés — 9 octobre 2026

Le joueur répond « ca me va :) » à la fiche C01-CRIT01. Le texte RP, la
base de cinq pour cent de l’agent, les dégâts critiques à deux cents pour
cent avant protections, les bonus de chance additionnés avec limites zéro
et cent, les attaques concernées, les jets par impact et cible, les
exclusions et les exemples présentés sont acceptés. Un critique ne change
pas la chance ni la durée des statuts. La fiche DI04 existante est conservée.

Les profils des monstres et familiers, les nouveaux bonus d’objets, les
composantes mixtes, les cas non montrés et le rendu natif restent à revoir.
Aucune intégration au moteur ni nouvelle règle d’attribut.

Décision : [C01-CRIT01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/critical-hit-rules-acceptance-decisions.json).
Manifeste accepté : [C01-CRIT02](../docs/reviews/refonte-agent-2026-10-06/review-derived-stat-critical-hits-02.json).

## D01-VAR01 Variance des armes proposée — 9 octobre 2026

Les règles des **coups critiques sont validées**. Passons aux dégâts variables des armes.

**D01 · Variance des armes — proposition**

**Sur la fiche d’une arme**

**Dégâts : 20–30**

Chaque modèle possède ses propres dégâts minimum et maximum. Cette plage est un exemple ; les valeurs seront réglées arme par arme.

**Tirage des dégâts de base**

- Chaque valeur entière de la plage a la même chance d’être tirée : ici, de **20 à 30 inclus**.
- Un tirage par coup de mêlée ou projectile. Dans une rafale, chaque projectile a son propre tirage.
- Une explosion utilise le même tirage de base pour toutes ses victimes. Les différences liées à leur position, les critiques et leurs défenses s’appliquent ensuite séparément.
- Un critique double les dégâts obtenus avant les protections ; il ne force pas le maximum de la plage.

Ces règles concernent les armes de mêlée, de tir et les armes lourdes, ainsi que leurs compétences.

**Dans la fiche d’une compétence**

Avec l’arme fictive **20–30**, une technique à **150 %** et **Force 10**, le coefficient déjà validé devient **155 %** :

**Dégâts : 31–46**

Cette plage indique les dégâts ordinaires avant les défenses de la cible. Elle se recalcule avec l’arme active et les bonus actuels. Les dégâts infligés restent des nombres entiers, arrondis après les protections.

**Le fonctionnement de cette variance reste à valider.**

Les plages propres aux modèles, les composantes mixtes, les attaques
transformées, les autres familles de sources et les détails non montrés
restent à revoir. La plage illustrative ne fixe aucun équipement réel.
Les règles existantes d’arme active et les fiches déjà acceptées restent
conservées. Aucun rendu natif ou changement du moteur.

Proposition : [D01-VAR01](../docs/reviews/refonte-agent-2026-10-06/review-weapon-damage-variance-01.json).

## D01-VAR-FEEDBACK01 Niveau de l’arme et explosions — 9 octobre 2026

Le joueur demande une plage de dégâts dépendant du niveau de l’arme,
petite au début et plus large à haut niveau, à équilibrer avec les PV des
monstres et du joueur au stade concerné. Sa dague à un–trois dégâts est
un exemple, pas une nouvelle entrée d’équipement. Aucun barème universel
ni définition technique du niveau d’arme n’est imposé.

Il demande aussi un tirage indépendant pour chaque victime d’explosion.
La proposition non acceptée d’un tirage commun est remplacée. Ces deux
directions sont explicites ; elles ne valident pas automatiquement la
distribution uniforme, les autres règles ni le texte exact de la fiche.
Les règles de critique et les anciennes fiches acceptées sont conservées.

Décision : [D01-VAR-FEEDBACK01](../docs/reviews/refonte-agent-2026-10-06/weapon-level-variance-and-per-victim-explosion-decisions.json).

## D01-VAR02 Variance corrigée pour revue — 9 octobre 2026

Oui :) **La plage de dégâts dépendra du niveau de l’arme**, et son amplitude sera équilibrée avec les PV des monstres et du joueur à ce stade de progression.

**D01 · Variance des armes — version corrigée**

- **Bas niveau :** une petite plage, comme ta dague à **1–3 dégâts**.
- **Haut niveau :** des dégâts plus élevés et un écart minimum–maximum plus grand, adapté au modèle de l’arme et aux PV attendus.
- Les plages exactes seront fixées lors de l’équilibrage des armes et des monstres.

**Explosions**

**Un tirage indépendant pour chaque victime.** Une même explosion peut donc infliger des dégâts de base différents aux ennemis touchés. Chaque cible conserve ensuite son propre jet de critique et l’application de ses défenses.

**Affichage**

La fiche de compétence continue d’afficher **les dégâts minimum–maximum avec l’arme active et les bonus actuels**.

Pour le tirage lui-même, je conserve comme proposition **la même probabilité pour chaque valeur entière de la plage**. Le niveau de l’arme détermine donc sa plage de dégâts ; les valeurs exactes restent à définir.

Proposition actuelle : [D01-VAR02](../docs/reviews/refonte-agent-2026-10-06/review-weapon-damage-variance-02.json).
Aucun code, équipement réel, animation ou contenu jouable modifié.

## D01-VAR02-ACCEPT01 Variance corrigée validée — 9 octobre 2026

Le joueur confirme « ok :) » après la version corrigée D01-VAR02. Le texte
présenté, la direction des plages dépendant du niveau et du modèle, leur
équilibrage avec les PV, le tirage indépendant par victime d’explosion,
l’affichage calculé avec l’arme active et les bonus actuels, ainsi que la
même probabilité pour chaque valeur entière de la plage sont acceptés.
La dague à un–trois reste illustrative. Aucun équipement réel ni courbe
universelle de progression n’est fixé.

Les éléments non représentés dans la version corrigée, les cas de
composantes mixtes et le rendu natif ne sont pas validés par extension.
Aucune intégration au moteur.

Décision : [D01-VAR02-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/revised-weapon-variance-acceptance-decisions.json).
Manifeste accepté : [Variance, révision 3](../docs/reviews/refonte-agent-2026-10-06/review-weapon-damage-variance-03.json).

## D02-MIX01 Dégâts physiques et élémentaires proposés — 9 octobre 2026

La **variance corrigée est validée**, y compris le tirage uniforme et le tirage indépendant pour chaque victime d’une explosion.

**D02 · Dégâts physiques et élémentaires — proposition**

Une attaque peut combiner plusieurs types de dégâts.

**Exemple de fiche, bonus déjà inclus**

- Dégâts physiques : **20–30**.
- Dégâts de froid : **5–10**.
- Dégâts totaux : **25–40**, avant les défenses et hors critique.

**Calcul proposé**

- Chaque type de dégâts possède sa plage et son propre tirage indépendant.
- Un seul jet de critique par coup sur une cible : en cas de critique, toutes ses composantes de dégâts directs sont doublées.
- L’armure s’applique une seule fois au total des dégâts physiques. Chaque résistance s’applique aux dégâts de son élément.
- Les résultats sont additionnés, puis le total est arrondi à l’entier inférieur.

**Effet des attributs**

Je propose que le bonus de **Force en mêlée**, ou de **Perception pour le tir et les armes lourdes à distance**, renforce toutes les composantes de dégâts directs de l’arme, physiques comme élémentaires. La **Volonté** renforce les sorts.

**Exemple de dégâts infligés**

Un coup tire **24 dégâts physiques + 8 dégâts de froid**. La cible possède **6 armure** et **50 % de résistance au froid** :

- Coup normal : **18 physiques + 4 froid = 22 dégâts**.
- Coup critique : **42 physiques + 8 froid = 50 dégâts**.

Les valeurs de cet exemple sont fictives. **Ces règles pour les dégâts mixtes restent à valider.**

Cette fiche propose le partage du critique, les tirages par famille, les
composantes concernées par les attributs, la protection et l’arrondi du
total. Les règles existantes conservent leur accord ; les compléments
présentés restent soumis à revue. Les sources non liées à une arme, les
attaques transformées, les effets persistants et le rendu natif ne sont
pas définis par extension. Aucun statut nouveau n’est créé par le simple
ajout d’un élément à une arme.

Proposition : [D02-MIX01](../docs/reviews/refonte-agent-2026-10-06/review-mixed-weapon-damage-01.json).

## D02-MIX01-ACCEPT01 Dégâts mixtes validés — 9 octobre 2026

Le joueur répond « oui ca me va :) » à D02-MIX01. Les plages et tirages
indépendants par type, le critique commun aux composantes d’un impact,
l’armure une seule fois sur le total physique, les résistances par élément
et l’arrondi final du total sont acceptés. Force en mêlée et Perception
pour les armes à distance renforcent les composantes directes physiques
et élémentaires de l’arme ; Volonté renforce les sorts. L’exemple de
fiche et les vingt-deux dégâts ordinaires ou cinquante critiques sont
acceptés comme illustrations, sans fixer un équipement réel.

Les cas non montrés, les autres sources et le rendu natif restent à
revoir. Les anciennes fiches et règles acceptées sont conservées.
Aucune intégration au moteur.

Décision : [D02-MIX01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/mixed-weapon-damage-acceptance-decisions.json).
Manifeste accepté : [Dégâts mixtes, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mixed-weapon-damage-02.json).

## D03-STACK01 Bonus de dégâts directs d’arme proposés — 9 octobre 2026

Les **dégâts mixtes sont validés**. Passons au cumul des bonus.

**D03 · Bonus de dégâts — proposition**

Pour les dégâts directs des armes et de leurs compétences, je propose d’additionner tous les bonus compatibles en pourcentage : **attribut, compétence utilisée, passifs, équipement et buffs**.

Une technique infligeant **150 % des dégâts de l’arme** apporte donc **+50 %** dans cette somme.

**Exemple fictif**

Arme : **20–30 dégâts**.

- Technique : **+50 %**.
- Force 10 : **+5 %**.
- Passif applicable : **+10 %**.
- Équipement : **+15 %**.
- Buff actif : **+20 %**.

**Bonus total : +100 %.**
**Dégâts actuels : 40–60**, avant les défenses et hors critique.

**Bonus fixes et bonus élémentaires**

- Les bonus fixes, comme **+5 dégâts de froid**, s’ajoutent aux dégâts de base de leur type avant l’application des pourcentages.
- Un bonus réservé à un élément, comme **+20 % de dégâts de froid**, renforce uniquement cette composante.
- Les bonus généraux compatibles renforcent toutes les composantes de l’attaque, selon les règles déjà validées.

Le critique s’applique ensuite, puis les protections de la cible. La fiche affiche la plage obtenue avec les bonus actuels ; le prochain rang conserve les mêmes autres conditions.

Les chiffres de l’exemple ne fixent aucun objet ni nouvelle compétence. **Ce cumul reste à valider.**

Le cumul additif de l’attribut et du bonus de compétence avait déjà été
accepté. Ce lot propose les sources compatibles restantes et leur ordre
pour les dégâts directs d’arme. Les cartes T03 et BM03 conservent leurs
pourcentages, leurs conditions et leur accord ; leur ordre général était
encore à revoir. Les calculs propres aux drones, transformations, sorts
et autres statistiques ne sont pas changés par cette proposition.

Proposition : [D03-STACK01](../docs/reviews/refonte-agent-2026-10-06/review-direct-weapon-bonus-stacking-01.json).

## D03-CAP01 Clarification du bonus supérieur à cent pour cent — 9 octobre 2026

Le joueur demande pourquoi le bonus de l’exemple s’arrête à cent pour
cent. La réponse précise qu’il ne s’agit pas d’un plafond et rappelle
le principe additif. Le joueur répond ensuite « ca me va :) ».

Oui :) **+100 % n’est pas un plafond**, c’était simplement le total de mon exemple. J’ai choisi ces chiffres pour montrer facilement des dégâts doublés.

Avec une arme à **20–30 dégâts** :

- Bonus total **+100 %** → **40–60 dégâts**.
- Bonus total **+250 %** → **70–105 dégâts**.
- Bonus total **+400 %** → **100–150 dégâts**.

**La proposition ne fixe donc aucun plafond général aux bonus de dégâts.**

Le choix concerne leur **addition** : chaque bonus apporte une portion supplémentaire des dégâts de base. Cela rend la progression lisible et facilite l’équilibrage quand le personnage cumule attributs, compétences, équipement et buffs, tout en permettant des dégâts très élevés.

Clarification : [D03-CAP01](../docs/reviews/refonte-agent-2026-10-06/weapon-damage-bonus-no-cap-clarification-decisions.json).

## D03-STACK01-ACCEPT01 Cumul additif et absence de plafond validés — 9 octobre 2026

L’accord après clarification porte sur le cumul des bonus directs
compatibles d’arme, les bonus fixes avant pourcentages, la restriction
par famille, la fiche calculée et l’absence de plafond général positif.
Les exemples restent fictifs. Les autres statistiques, les drones, les
transformations et les cas non présentés restent dans leur revue propre.
Les anciennes fiches et valeurs acceptées sont conservées.

Décision : [D03-STACK01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/direct-weapon-bonus-stacking-and-no-cap-acceptance-decisions.json).
Manifeste accepté : [Bonus directs, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-direct-weapon-bonus-stacking-02.json).

## D04-DOWN01 Affaiblissements des dégâts infligés proposés — 9 octobre 2026

Le **cumul additif est validé**, avec des bonus pouvant dépasser **+100 %**, sans plafond général.

**D04 · Affaiblissements des dégâts — proposition**

Un affaiblissement réduisant les dégâts infligés s’applique au résultat obtenu après les bonus et le critique éventuel, avant l’armure et les résistances de la victime.

**Exemple fictif, hors critique et avant les défenses**

- Arme : **20–30 dégâts**.
- Bonus total : **+100 %** → **40–60 dégâts**.
- Affaiblissement : **−25 % de dégâts infligés** → **30–45 dégâts**.

Le bonus reste présent : l’affaiblissement réduit les dégâts obtenus de **25 %** pendant son effet.

**Plusieurs affaiblissements distincts**

Je propose de les appliquer successivement. Avec une seconde réduction de **20 %** :

**40–60 → 30–45 → 24–36 dégâts.**

Cela représente une réduction totale de **40 %**.

Chaque réduction générale affecte toutes les composantes directes concernées. Une réduction réservée au froid affecte uniquement les dégâts de froid.

Les dégâts restent au minimum à **0**, avec l’arrondi final déjà validé. Les valeurs de cet exemple ne fixent aucun nouveau sort.

**Ce fonctionnement reste à valider.**

La portée est celle des dégâts directs d’arme infligés sous
affaiblissement. Le placement avant les défenses reprend la règle
existante d’Interférence. Les réductions défensives de la victime après
armure et résistances conservent leur cadrage distinct ; aucun ordre
précédemment accepté n’est modifié par extension. Les cas non présentés
et le rendu natif restent à revoir. Aucune intégration au moteur.

Proposition : [D04-DOWN01](../docs/reviews/refonte-agent-2026-10-06/review-outgoing-weapon-damage-reductions-01.json).

## D04-DOWN01-ACCEPT01 Affaiblissements de dégâts validés — 9 octobre 2026

Le joueur répond « ca me va :) ». L’accord porte sur la réduction relative
des dégâts obtenus après bonus et critique éventuel, avant les défenses,
le cumul successif des affaiblissements distincts, leur portée par
composante, le plancher zéro et les exemples fictifs. Une réduction
réservée au froid conserve cette restriction. Les anciens chiffres de
compétence, les règles de réapplication et les cas non présentés restent
conservés ou à revoir selon leur état antérieur.

Décision : [D04-DOWN01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/outgoing-weapon-damage-reductions-acceptance-decisions.json).
Manifeste accepté : [Affaiblissements, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-outgoing-weapon-damage-reductions-02.json).

## D05-PROTECT01 Protections en pourcentage proposées — 9 octobre 2026

Les **affaiblissements de dégâts sont validés**. Passons aux protections de la cible.

**D05 · Réduction des dégâts subis — proposition**

Une protection en pourcentage contre les dégâts directs s’applique **après l’armure et les résistances**.

**Exemple fictif**

Un coup inflige **100 dégâts physiques + 40 dégâts de froid**, avant les défenses. La cible possède **20 armure**, **50 % de résistance au froid** et une protection réduisant les dégâts directs de **25 %** :

- Après armure : **80 dégâts physiques**.
- Après résistance au froid : **20 dégâts de froid**.
- Total avant la protection : **100 dégâts**.
- Avec la protection de **25 %** : **75 dégâts subis**.

**Plusieurs protections distinctes**

Je propose de les appliquer successivement. Avec une seconde protection de **20 %** :

**100 → 75 → 60 dégâts subis.**

Les deux protections réduisent donc ensemble de **40 %** les dégâts restants après l’armure et les résistances.

Une protection ne réduit que les dégâts qu’elle couvre. Par exemple, l’Égide occulte conserve son exclusion des dégâts persistants.

Les fractions sont conservées pendant le calcul ; le total est arrondi à l’entier inférieur **après toutes les protections**. Les dégâts peuvent être réduits à **0**.

Les chiffres sont illustratifs. **Ces règles communes restent à valider.**

L’ordre proposé reprend les documents d’Égide et du cadrage général.
Les pourcentages, durées et comportements des cartes acceptées ne sont
pas réécrits. Les protections directes ne couvrent pas les dégâts
persistants par extension. Aucune absorption ni compétence de bouclier
supprimée n’est réintroduite. Les exceptions de dégâts directs sans
seconde protection, les absorptions et Dernier sursaut restent distincts.
Aucune intégration au moteur ni nouveau rendu natif.

Proposition : [D05-PROTECT01](../docs/reviews/refonte-agent-2026-10-06/review-incoming-percent-protections-01.json).

## D05-PROTECT01-ACCEPT01 Protections en pourcentage validées — 9 octobre 2026

Le joueur répond « ca me va :) » puis ajoute une directive d’affichage.
L’accord porte sur les règles présentées de D05 : protection après armure
et résistances, réductions successives distinctes selon leur couverture,
fractions conservées jusqu’à l’arrondi final et plancher zéro. L’exemple
fictif cent-quarante dégâts avant défenses, puis cent avant protections,
soixante-quinze et enfin soixante après les deux protections est accepté.
Aucun nouveau sort ni équipement créé ; les cas non montrés restent à
revoir. L’ajout visuel du joueur est consigné séparément ci-dessous.

Décision : [D05-PROTECT01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/incoming-percent-protections-acceptance-decisions.json).
Manifeste accepté : [Protections, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-incoming-percent-protections-02.json).

## HUD-DMG01-DIRECTION01 Total et détail par type demandés — 9 octobre 2026

Le joueur demande au-dessus du joueur et des monstres les dégâts totaux,
suivis entre parenthèses du détail par type avec une petite icône. Cette
organisation est une instruction explicite. Le rendu natif, les dessins
d’icônes et les détails non montrés ne sont pas approuvés par extension.
HUD08 conserve les couleurs par élément et les types explicites des logs.
L’information du bonus de Tempo offensif déjà acceptée conserve son rôle ;
sa composition avec le nouveau détail reste à présenter, sans double compte.

Décision : [HUD-DMG01-DIRECTION01](../docs/reviews/refonte-agent-2026-10-06/floating-total-and-type-icon-damage-direction-decisions.json).

## HUD-DMG01 Exemple du détail par type proposé — 9 octobre 2026

Les **protections sont validées**, et ton format d’affichage est noté pour le joueur comme pour les monstres.

**HUD-DMG01 · Dégâts affichés**

**75 (⚔ 60 · ❄ 15)**

Dans cet exemple : **75 dégâts au total**, dont **60 physiques** et **15 de froid**, après les protections.

- Le **total** apparaît en premier.
- Le détail entre parenthèses montre chaque type de dégâts avec sa **petite icône**.
- Chaque composante garde la **couleur de son type**, selon la direction déjà validée.

Je propose que les composantes affichées soient entières et que **leur somme corresponde toujours au total**, sans modifier les dégâts calculés.

Les symboles ci-dessus illustrent le format ; les icônes finales et leur lisibilité en jeu seront présentées lors du travail sur l’interface.

La cohérence des composantes affichées ne crée aucun nouvel arrondi
mécanique. L’algorithme d’attribution des entiers, les cas de décès et
d’absorption et le rendu natif restent à revoir. Aucun code ni pixel intégré.

Proposition : [HUD-DMG01](../docs/reviews/refonte-agent-2026-10-06/review-floating-damage-type-breakdown-HUD-DMG01-01.json).

## HUD-DMG01-ACCEPT01 Affichage des dégâts validé — 9 octobre 2026

Le joueur répond « parfait :) on contiue ». L’exemple soixante-quinze,
dont soixante physiques et quinze de froid après protections, la couleur
par type et les composantes entières dont la somme égale le total sont
acceptés. Les dommages calculés restent inchangés. Les symboles ne sont
pas des assets finaux ; algorithme exact et rendu natif restent à revoir.

Décision : [HUD-DMG01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/floating-damage-type-breakdown-acceptance-decisions.json).
Manifeste accepté : [HUD-DMG01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-floating-damage-type-breakdown-HUD-DMG01-02.json).

## D06-HEAL01 Soins et drain de vie proposés — 9 octobre 2026

Le **format des dégâts est validé**, avec un détail entier dont la somme correspond au total.

**D06 · Soins et drain de vie — proposition**

**Soins du joueur**

- Quantité de soin arrondie à l’entier supérieur, puis limitée aux **PV manquants** et au plafond éventuel de la compétence.
- Un plafond exprimé en pourcentage des PV maximum est arrondi à l’entier inférieur.
- Les limites déjà définies **par tour ou par utilisation** restent applicables.

**Exemples**

- Avec **130 PV maximum**, Régénération au rang 11 rend **16 PV par tour**, pour ses **12 %** déjà validés.
- Un plafond de **10 %** avec **135 PV maximum** autorise **13 PV** de soin.
- S’il ne vous manque que **4 PV**, le soin rend au maximum **4 PV**.

**Drain de vie**

Le drain se calcule sur les **PV réellement retirés**, après les protections et dans la limite des PV restants de la cible.

Une attaque inflige **40 dégâts** à un ennemi qui possède encore **12 PV** : avec un drain fictif de **25 %**, vous récupérez **3 PV**, avant vos limites de soin.

Si la cible ne perd aucun PV, le drain ne rend aucune vie. Il ne peut pas rendre plus de PV qu’il n’en a retiré.

**Attributs**

Les soins fondés sur vos PV maximum bénéficient de la Vitalité. Les drains bénéficient des attributs qui renforcent les dégâts concernés. Ces attributs ne multiplient pas une deuxième fois le pourcentage de soin ou de drain.

**Ces règles communes de soin restent à valider.**

La proposition concerne les soins du joueur. Les fiches individuelles
conservent leurs accords et leurs paramètres ; les réparations passives
de drone avec fractions gardent leur cadrage distinct. Les autres cas
non présentés, les budgets multiples et le rendu natif restent à revoir.
Aucun code ou pixel intégré.

Proposition : [D06-HEAL01](../docs/reviews/refonte-agent-2026-10-06/review-player-healing-and-drain-01.json).

## D06-HEAL01-ACCEPT01 Soins et drain de vie validés — 9 octobre 2026

Le joueur répond « oui :) je valide ». Les soins du joueur arrondis au
supérieur, plafonds en pourcentage des PV maximum arrondis à l’inférieur,
limites propres par tour ou utilisation et PV manquants sont acceptés.
Le drain utilise les PV réellement retirés après protections, sans
dépassement des PV restants de la cible et sans rendre plus de vie que
retirée. Aucun PV retiré ne donne aucun soin. Les attributs interviennent
par les PV maximum ou les dégâts concernés, sans seconde multiplication
du pourcentage de soin ou de drain. Les exemples présentés sont acceptés.

Les réparations de drone, les budgets multiples, les événements différés,
les cas non montrés et le rendu natif gardent leur état distinct.
Aucune carte individuelle, coût ou recharge n’est modifié.

Décision : [D06-HEAL01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/player-healing-and-drain-acceptance-decisions.json).
Manifeste accepté : [Soins, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-player-healing-and-drain-02.json).

## J01-MAX01 Bonus de PV et d’énergie maximum proposés — 9 octobre 2026

Les **règles de soin et de drain sont validées**.

**J01 · Bonus de PV et d’énergie maximum — proposition**

Pour les jauges ordinaires du personnage :

- La base reste celle calculée avec le **niveau et les attributs**, selon les règles validées.
- Les **bonus fixes** sont ajoutés à cette base.
- Les **bonus compatibles en pourcentage s’additionnent**, puis s’appliquent au résultat.
- Le maximum final est arrondi à l’entier inférieur.

**Exemple fictif**

- Base : **100 PV**.
- Bonus fixe : **+20 PV** → **120 PV**.
- Bonus en pourcentage : **+10 % et +15 %** → **+25 %**.

**Maximum final : 150 PV.**

Le même principe s’applique aux bonus d’énergie maximum.

**Quand le maximum change**

- Vous avez **80/100 PV** ; le maximum passe à **150** → **80/150 PV**.
- Vous avez **120/150 PV** ; le maximum redescend à **100** → **100/100 PV**.

Même règle pour l’énergie. Si vous rééquipez ensuite le bonus, la jauge ne se remplit pas.

Ces ajustements de jauge ne comptent pas comme une blessure ou une dépense d’énergie.

La **montée de niveau** conserve sa restauration complète déjà validée. Le **Colosse** conserve sa règle particulière de pourcentage de vie lors de l’entrée et de la sortie de transformation.

**Ce cumul des bonus et le plafonnement des jauges restent à valider.**

Le lot revoit l’ordre commun des bonus ordinaires et l’ajustement des
valeurs actuelles. Les formules de base, la restauration de niveau et
le pourcentage de vie du Colosse conservent leur accord. Les limites
minimales, les bonus réels d’objets et les autres cas non montrés restent
à revoir ; aucun nouveau loot, code ou rendu n’est intégré.

Proposition : [J01-MAX01](../docs/reviews/refonte-agent-2026-10-06/review-maximum-reserve-modifiers-01.json).
## J01-MAX01-ACCEPT01 Bonus de PV et d’énergie maximum validés — 9 octobre 2026

Le joueur répond « ok :) ». L’accord porte sur les bonus ordinaires de
PV et d’énergie maximum : bonus fixes avant les pourcentages compatibles
additionnés, puis arrondi final à l’inférieur. Une hausse du maximum
conserve la valeur actuelle absolue ; une baisse plafonne la valeur
actuelle au nouveau maximum. Rééquiper ne remplit pas la jauge. Ces
ajustements ne sont ni une blessure ni une dépense d’énergie.
Les exemples présentés sont acceptés. La restauration complète à la
montée de niveau et le pourcentage de vie du Colosse sont conservés.
Les valeurs d’objets, les limites minimales, les autres cas non montrés
et le rendu natif restent à revoir. Aucun code ou pixel intégré.

Décision : [J01-MAX01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/ordinary-maximum-reserve-modifiers-acceptance-decisions.json).
Manifeste accepté : [Maximums, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-maximum-reserve-modifiers-02.json).

## D07-SPELL01 Dégâts directs des sorts proposés — 9 octobre 2026

Les **règles des PV et de l’énergie maximum sont validées**.

**D07 · Dégâts directs des sorts — proposition**

Pour les impacts directs des sorts de **Feu, Froid et Occulte** :

- La compétence fournit ses dégâts de base au **rang effectif actuel**.
- Les bonus fixes compatibles s’ajoutent d’abord.
- Les bonus en pourcentage de **Volonté, passifs, équipement et buffs** s’additionnent, sans plafond général.
- Un bonus réservé au feu, par exemple, renforce uniquement les dégâts de feu.

La Volonté conserve son bonus validé : **+1 % par point au-dessus de 5**.

**Exemple fictif**

- Sort : **40 dégâts de feu** au rang actuel.
- Volonté 25 : **+20 %**.
- Passif compatible : **+25 %**.
- Équipement : **+15 %**.

**Bonus total : +60 %.**
**Dégâts actuels : 64**, avant les protections et hors critique.

Contre une cible avec **25 % de résistance au feu** : **48 dégâts subis**, ou **96 en cas de critique**, sans autre modificateur.

**Ordre du calcul**

Bonus → critique → affaiblissements des dégâts infligés → résistance de l’élément → protections compatibles de la cible → arrondi final à l’entier inférieur.

Ce bonus de dégâts ne modifie pas les chances de statut, les durées, les coûts, les portées ou les recharges.

Les chiffres sont illustratifs. **Ce cumul pour les sorts reste à valider.**

La proposition revoit explicitement le cumul compatible pour les impacts
directs des sorts de Feu, Froid et Occulte. Les valeurs de base au rang
effectif restent celles des compétences et ne sont pas ajoutées une
seconde fois comme bonus. Les valeurs de l’exemple sont fictives. Les
dégâts persistants, les drones, les transformations, les mines et grenades
conservent leurs cadrages distincts ; aucune extension de ce lot ne leur
est accordée. Les coefficients et fiches acceptés sont conservés, aucun
affixe réel, code ou rendu natif n’est créé.

Proposition : [D07-SPELL01](../docs/reviews/refonte-agent-2026-10-06/review-direct-spell-damage-bonuses-01.json).

## J01-FLAT01-CONFIRM01 Bonus fixes des objets confirmés — 9 octobre 2026

Le joueur vérifie explicitement que certains objets pourront fournir
par exemple +15 PV et +4 énergie. La réponse confirme des bonus aux
maximums, ajoutés avant les pourcentages, sans remplir les jauges à
l’équipement : cent plus quinze donne cent-quinze PV maximum ; trente
plus quatre donne trente-quatre énergie maximum. Le joueur répond
« ok parfait on continue :) ». Les exemples ne constituent pas des
valeurs d’affixes réellement générés. Aucun nouvel attribut fixe ni
catalogue de loot n’est approuvé par extension.

Décision : [J01-FLAT01-CONFIRM01](../docs/reviews/refonte-agent-2026-10-06/fixed-reserve-item-bonus-clarification-confirmation-decisions.json).

## D07-SPELL01-ACCEPT01 Dégâts directs des sorts validés — 9 octobre 2026

Après la fiche D07 et la clarification des bonus fixes ordinaires,
le joueur répond « ok parfait on continue :) ». L’accord est consigné
pour le cumul et l’ordre présentés des dégâts directs de Feu, Froid
et Occulte : base au rang effectif, bonus fixes avant le cumul additif
compatible sans plafond général, bonus élémentaires ciblés et ordre
critique, affaiblissements, résistances, protections puis arrondi final.
La Volonté garde son coefficient accepté et ce bonus de dégâts ne
modifie pas les autres paramètres des compétences. L’exemple fictif
quarante dégâts de base, soixante-quatre après bonus, puis quarante-huit
ou quatre-vingt-seize subis est accepté. Les sources et cas non montrés
restent distincts ; aucun pixel ou code intégré.

Décision : [D07-SPELL01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/direct-spell-damage-bonus-acceptance-decisions.json).
Manifeste accepté : [Sorts, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-direct-spell-damage-bonuses-02.json).

## D08-DRONE01 Bonus de dégâts des drones proposés — 9 octobre 2026

Les **bonus fixes des objets sont confirmés**, et le cumul proposé pour les sorts est validé.

**D08 · Bonus de dégâts des drones — proposition**

Pour le **Traqueur, l’Artilleur et le Gardien** :

- L’invocation fournit les dégâts de base au **rang effectif actuel**.
- Les bonus fixes compatibles s’ajoutent d’abord, dans leur type de dégâts.
- Les bonus en pourcentage d’**Intelligence, passifs, équipement et buffs dédiés aux drones** s’additionnent, sans plafond général.
- Les bonus réservés à un type renforcent uniquement les dégâts de ce type.

L’Intelligence conserve son bonus validé : **+1 % de dégâts par point au-dessus de 5**.

**Exemple fictif**

- Drone : **25 dégâts de base**.
- Intelligence 25 : **+20 %**.
- Algorithmes de combat améliorés, rang 11 : **+45 %**.
- Équipement renforçant les drones : **+15 %**.

**Bonus total : +80 %.**
**Dégâts actuels : 45**, avant les défenses de la cible.

Ce cumul additif servira à recalculer les valeurs des anciennes fiches avec le coefficient d’Intelligence validé.

Les bonus personnels de mêlée, de tir ou de sorts ne renforcent pas automatiquement les drones. L’Éclaireur et la Mule conservent leurs fonctions sans attaque.

Le bonus de dégâts ne modifie pas la vision, la capacité de transport, les coûts ou les recharges. Les PV des drones feront l’objet du calcul suivant.

Les chiffres de l’exemple ne créent aucun objet. **Ce cumul pour les drones reste à valider.**

La règle proposée concerne les drones armés et conserve le coefficient
Intelligence du modèle d’attributs accepté, les pourcentages présentés
d’Algorithmes de combat améliorés et les modèles sans attaque. Les anciens
exemples numériques restent des instantanés historiques. Le nouvel ordre
additif doit être accepté avant leur recalcul ; leurs cartes ne sont pas
réécrites. Les PV, la capacité de transport, les profils critiques,
les instants de lecture des bonus et les autres cas non montrés restent
à revoir. Aucun nouvel affixe réel, code ou rendu natif créé.

Proposition : [D08-DRONE01](../docs/reviews/refonte-agent-2026-10-06/review-direct-drone-damage-bonuses-01.json).

## D08-DRONE01-ACCEPT01 Bonus de dégâts des drones validés — 9 octobre 2026

Le joueur répond « Mais sinon oui, je valide tes recommendations. » après
avoir proposé un équipement des familiers. Le cumul additif de dégâts des
drones armés présenté est accepté : base au rang d’invocation, bonus fixes
avant les pourcentages compatibles, Intelligence au coefficient accepté,
bonus dédiés et absence de plafond général. L’exemple fictif vingt-cinq
plus quatre-vingts pour cent donne quarante-cinq dégâts avant défenses.
Les autres paramètres et modèles sans attaque restent conservés. Le
nouvel équipement de familiers est traité séparément ; le calcul d’une
arme portée n’est pas inféré de cette acceptation. Les anciennes fiches
restent des instantanés historiques sans réécriture ni code intégré.

Décision : [D08-DRONE01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/direct-drone-damage-bonus-acceptance-decisions.json).
Manifeste accepté : [Drones, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-direct-drone-damage-bonuses-02.json).

## PETS-EQ-DIRECTION01 Niveau, type et objets des familiers — 9 octobre 2026

Le joueur demande de prendre le niveau et le type de combat pour déterminer
l’équipement compatible. Les types explicitement donnés sont mêlée,
distance physique et distance magique. Les objets portés tombent au sol
à la mort et à la fin d’une résurrection. À la question sur le niveau
d’un drone pour l’équipement, il répond « Il suit le niveau du joueur » ;
le rang d’invocation et l’Intelligence restent les paramètres de puissance.
Les détails non montrés ne sont pas validés par extension.

Directions : [PETS-EQ-DIRECTION01](../docs/reviews/refonte-agent-2026-10-06/familiar-equipment-level-type-expiry-and-drone-level-decisions.json).

## PETS-EQ01 Compléments d’équipement des familiers proposés — 9 octobre 2026

Oui, équiper les familiers donnerait une utilité supplémentaire au loot et permettrait de personnaliser leur rôle.

**PETS-EQ01 · Équipement des familiers**

**Règles retenues**

- La compatibilité dépend du **niveau du familier** et de son type : **mêlée, distance physique ou distance magique**.
- Le **niveau des drones suit celui du joueur**. Le rang d’invocation et l’Intelligence continuent à renforcer leur puissance.
- À la mort d’un familier ou à la fin d’une résurrection, **les objets portés tombent au sol**.

**Compléments proposés**

- Chaque objet indique son **niveau minimum requis** et les **types de familiers compatibles**. Le familier doit satisfaire les deux conditions.
- Les ressuscités conservent le **niveau du monstre d’origine**.
- Les bonus des objets équipés s’appliquent au **familier qui les porte**, selon les effets compatibles.
- Les objets tombent **sur sa case**, avec ceux déjà présents. Une destruction volontaire les dépose aussi ; un rappel de drone conserve son équipement.
- Un bouton **« Équipement »** dans la fenêtre **« Familiers »** ouvre sa fiche. La Mule conserve en plus son rôle de transport.

L’Éclaireur et la Mule restent sans attaque. Les emplacements et familles d’objets autorisés seront définis pour chaque type.

Le calcul d’une arme équipée devra conserver l’intérêt du rang d’invocation. **Ces compléments restent à valider avant de fixer les emplacements et les formules.**

Proposition : [PETS-EQ01](../docs/reviews/refonte-agent-2026-10-06/review-familiar-equipment-PETS-EQ01-01.json).
Les critères explicites conservent leur accord ; les compléments restent
à revoir. Le calcul des PV des drones est différé le temps de ce cadrage.
Aucune intégration au moteur, aucun nouvel affixe réel ou rendu natif.

## DRONE-STAT01-DIRECTION01 Statistiques propres et niveau des drones — 9 octobre 2026

Le joueur précise que tous les drones ont le niveau du joueur, quel que
soit le nombre de points investis, et que leurs statistiques suivent les
mêmes règles que celles du joueur et des monstres. À la clarification
entre copie des attributs du joueur et profil propre, il choisit
« Attributs propres à chaque modèle ». Le niveau devient donc aussi la
base de progression statistique des drones. L’ancienne restriction au
niveau d’équipement et la base de dégâts dépendant uniquement du rang
d’invocation sont remplacées sur ces points. Les autres règles acceptées
restent conservées ; leurs instantanés historiques ne sont pas réécrits.
Les valeurs exactes par modèle, les cinq points proposés ci-dessous,
leur allocation et les effets de montée de niveau sur les jauges restent
à valider séparément. Les compléments PETS-EQ01 ne sont pas approuvés
par cette réponse. Aucun code, nouveau loot ou rendu natif intégré.

Direction : [DRONE-STAT01-DIRECTION01](../docs/reviews/refonte-agent-2026-10-06/drone-level-own-attributes-and-shared-stat-rules-decisions.json).

## DRONE-STAT01 Progression des attributs des drones proposée — 9 octobre 2026

D’accord, **chaque modèle aura ses propres attributs**, avec le même niveau que le joueur.

**DRONE-STAT01 · Statistiques des drones**

- **Niveau du drone = niveau du joueur**, quel que soit le rang de l’invocation.
- Chaque modèle possède ses propres **Force, Agilité, Vitalité, Perception, Intelligence et Volonté**.
- Ses statistiques suivent les **mêmes règles de calcul que celles du joueur et des monstres**, avec des valeurs propres à son modèle.
- Son équipement et les bonus compatibles modifient ses statistiques.
- Le rang d’invocation reste une **amélioration supplémentaire**, distincte de son niveau.

**Exemple**

Joueur niveau **20** :

- Invocation au rang **1** → drone niveau **20**.
- Invocation au rang **10** → drone toujours niveau **20**, avec les améliorations apportées par les rangs investis.

Les anciennes valeurs de PV et de dégâts devront donc être recalculées sur cette nouvelle base.

**Progression proposée**

À chaque niveau gagné, les **5 points d’attributs** du drone seraient répartis **automatiquement selon son modèle** : davantage de Force pour un drone de mêlée, de Perception pour un tireur, de Vitalité pour un protecteur.

Les répartitions exactes et les bonus des rangs seront présentés séparément. **Cette répartition automatique reste à valider.**

La progression proposée concerne les attributs des drones ; aucune
nouvelle réserve, compétence autonome, limite de drones ou attaque de
Mule/Éclaireur n’est créée. Les statistiques propres ne copient pas
l’équipement ni les attributs du joueur. Les bonus du propriétaire et
ceux du drone doivent être distingués lors de la revue des formules,
sans double application implicite. La restauration des PV et de
l’énergie à la montée de niveau du joueur n’est pas approuvée pour
les drones par extension.

Proposition : [DRONE-STAT01](../docs/reviews/refonte-agent-2026-10-06/review-drone-actor-statistics-01.json).

## DRONE-STAT01-ACCEPT01 Croissance automatique des drones validée — 9 octobre 2026

Le joueur répond « parfait :) ». Les cinq points d’attributs par niveau
répartis automatiquement selon le modèle et les orientations présentées
sont acceptés. Le niveau du drone reste celui du joueur ; le rang demeure
une amélioration distincte. Les répartitions précises n’étaient pas
présentées et restent à revoir, ainsi que les bonus de rang, les jauges
actuelles et les compléments d’équipement PETS-EQ01. Aucun code ni rendu
natif intégré.

Décision : [DRONE-STAT01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/drone-attribute-growth-acceptance-decisions.json).
Manifeste accepté : [DRONE-STAT01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-drone-actor-statistics-02.json).

## DRONE-STAT02 Profils des drones proposés — 9 octobre 2026

La **progression automatique de 5 points par niveau est validée**.

**DRONE-STAT02 · Profils d’attributs proposés**

Chaque drone commencerait avec **36 points d’attributs au niveau 1**. Les attributs non cités ci-dessous valent **5** au départ.

**Éclaireur — mobilité**

- Départ : **Agilité 9 · Vitalité 7**.
- Par niveau : **+3 Agilité · +2 Vitalité**.

**Mule — survie**

- Départ : **Agilité 7 · Vitalité 9**.
- Par niveau : **+2 Agilité · +3 Vitalité**.

**Traqueur — mêlée**

- Départ : **Force 9 · Agilité 6 · Vitalité 6**.
- Par niveau : **+3 Force · +1 Agilité · +1 Vitalité**.

**Artilleur — distance physique**

- Départ : **Perception 9 · Agilité 6 · Vitalité 6**.
- Par niveau : **+3 Perception · +1 Agilité · +1 Vitalité**.

**Gardien — protection et mêlée**

- Départ : **Force 7 · Vitalité 9**.
- Par niveau : **+1 Force · +1 Agilité · +3 Vitalité**.

Un drone invoqué tard reçoit directement les attributs correspondant au niveau du joueur.

**Exemple au niveau 20**

Avec le calcul des PV déjà retenu, avant équipement, bonus d’invocation et bonus du joueur :

- Traqueur : **Vitalité 25 → 148 PV maximum**.
- Gardien : **Vitalité 66 → 312 PV maximum**.

Ces profils conservent les fonctions des drones : l’Éclaireur et la Mule restent sans attaque. **Les valeurs de départ, les répartitions et leur application dès l’invocation restent à valider.**

Le budget de départ de trente-six points est propre à cette proposition
de drones et ne modifie pas les départs du joueur. Les cinq points de
croissance sont conservés ; leurs répartitions précises sont nouvelles.
Les exemples de PV utilisent la base commune explicitement proposée,
avant les autres bonus. Aucune attaque d’Éclaireur/Mule ni formule de
transport ou de vision n’est ajoutée. Les autres statistiques, les rangs,
l’équipement, les jauges actuelles et le rendu natif restent à revoir.
Aucun objet réel, code ou sprite intégré.

Proposition : [DRONE-STAT02](../docs/reviews/refonte-agent-2026-10-06/review-drone-attribute-profiles-01.json).

## DRONE-STAT02-ACCEPT01 Profils d’attributs des drones validés — 9 octobre 2026

Le joueur répond « ca me va :) ». Le budget de trente-six points au
niveau un, les cinq profils présentés, leurs répartitions par niveau
et l’application complète dès une invocation tardive sont acceptés.
Les exemples de niveau vingt, Traqueur à cent-quarante-huit PV et Gardien
à trois-cent-douze avant les autres bonus, sont acceptés. Les valeurs
antérieures restent des instantanés historiques. Les bonus de rang,
l’équipement, les jauges actuelles, l’origine précise des modificateurs
et le rendu natif restent à revoir. Aucun code ni sprite intégré.

Décision : [DRONE-STAT02-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/drone-attribute-profile-acceptance-decisions.json).
Manifeste accepté : [DRONE-STAT02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-drone-attribute-profiles-02.json).

## DRONE-STAT03 Attributs propres et bonus de l’agent proposés — 9 octobre 2026

Les **profils d’attributs des cinq drones sont validés**.

**DRONE-STAT03 · Attributs du drone et bonus de l’agent — proposition**

**Les attributs du drone**

- **Force** : dégâts de ses attaques de mêlée.
- **Perception** : dégâts de ses attaques physiques à distance.
- **Volonté** : dégâts de ses pouvoirs magiques.
- Ses autres caractéristiques utilisent également **ses propres attributs**, selon les règles communes.

**L’Intelligence de l’agent**

Elle conserve le bonus prévu : **+1 % de dégâts et de PV maximum des drones par point au-dessus de 5**.

L’Intelligence du drone alimenterait sa propre réserve d’énergie ; le bonus d’invocateur proviendrait uniquement de celle de l’agent.

Les bonus de dégâts compatibles du drone et de l’invocateur **s’additionnent dans le même cumul**.

Pour ses PV maximum : **base issue de son niveau et de sa Vitalité → bonus fixes → pourcentages compatibles additionnés → arrondi à l’entier inférieur**.

**Exemple isolé, sans bonus de rang d’invocation ni équipement**

- Traqueur niveau 20, **Force 66** : **+61 % de dégâts de mêlée**.
- Agent, **Intelligence 25** : **+20 %**.
- Algorithmes de combat améliorés, rang 11 : **+45 %**.

**Bonus total : +126 %.**
Une attaque de base fictive de **20 dégâts** affiche **45 dégâts**, avant les défenses.

Ses **148 PV de base** deviennent **177 PV maximum** avec le seul bonus d’Intelligence de l’agent.

**Ces origines de bonus et leur cumul restent à valider.**

La proposition distingue les attributs du porteur et le bonus dédié de
l’agent. Aucun coefficient existant n’est modifié ; leur origine et leur
cumul sont présentés explicitement. Les dégâts illustratifs sont affichés
entiers mais leurs fractions de calcul restent conservées jusqu’aux
protections et à l’arrondi mécanique final. L’ordre des bonus de PV de
drone est soumis ici, sans modifier les jauges actuelles ni leur
restauration. Aucun nouveau drone magique, attaque, objet ou rendu
natif n’est créé. Les compléments PETS-EQ01 restent à revoir.

Proposition : [DRONE-STAT03](../docs/reviews/refonte-agent-2026-10-06/review-drone-owner-and-actor-attribute-mapping-01.json).

## DRONE-STAT03-ACCEPT01 Attributs propres et bonus de l’agent validés — 9 octobre 2026

Le joueur répond « ok :) on continue :) ». Les origines présentées des
bonus sont acceptées : attributs propres du drone pour ses actions,
Intelligence de l’agent pour le renforcement des drones, sans seconde
application depuis l’Intelligence propre du drone. Les dégâts compatibles
s’additionnent ; les PV suivent base propre, bonus fixes, pourcentages
additionnés, arrondi final. Les exemples isolés à quarante-cinq dégâts
et cent-soixante-dix-sept PV sont acceptés. Les bonus exacts de rang,
l’équipement, les jauges actuelles et le rendu natif restent à revoir.
Aucun code ni sprite intégré.

Décision : [DRONE-STAT03-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/drone-owner-and-actor-attribute-mapping-acceptance-decisions.json).
Manifeste accepté : [DRONE-STAT03, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-drone-owner-and-actor-attribute-mapping-02.json).

## DRONE-STAT04 Bonus des rangs d’invocation proposés — 9 octobre 2026

Le **cumul des attributs du drone et des bonus de l’agent est validé**.

**DRONE-STAT04 · Rangs d’invocation — proposition**

Chaque compétence d’invocation de drone améliore **uniquement le modèle correspondant**.

Chaque rang après le premier apporte :

- **+10 % de PV maximum**.
- **+10 % de dégâts**, pour les drones armés.

Ces pourcentages s’ajoutent aux autres bonus compatibles. Le rang 1 apporte **+0 %** ; le rang 10, **+90 %**.

**Effet actuel — rang 11**

- Bonus de PV maximum : **+100 %**.
- Bonus de dégâts : **+100 %**, si le drone est armé.

**Prochain rang — 12 · 1 point de compétence**

- Bonus de PV maximum : **+110 %**.
- Bonus de dégâts : **+110 %**, si le drone est armé.

**Exemple avec les mêmes autres bonus**

Traqueur niveau 20 : **Force 66**, **148 PV de base** ; agent avec **Intelligence 25** ; Algorithmes de combat améliorés au **rang 11**. Attaque de base fictive : **20 dégâts**.

- Invocation rang 11 : **325 PV maximum · 65 dégâts affichés**.
- Invocation rang 12 : **340 PV maximum · 67 dégâts affichés**.

Les dégâts sont indiqués avant les défenses ; les autres bonus sont exclus de cet exemple.

Les bonus d’équipement prolongent cette progression au-delà de 10. Les effets spécifiques de reconnaissance, de transport et de protection restent traités séparément.

**Cette progression des bonus d’invocation reste à valider.**

La proposition concerne les bonus de PV et de dégâts de chaque invocation
de drone. Elle ne modifie pas le niveau, les profils propres, les limites
de familiers ou la résurrection OC07. Les autres effets spécifiques et
paramètres des cartes conservent leur cadrage. Les dégâts de l’exemple
sont fictifs et affichés entiers ; leurs fractions de calcul restent
conservées pour les défenses. Les valeurs de PV des autres anciens
instantanés seront recalculées après validation du nouveau bonus.
Les compléments PETS-EQ01 restent à revoir. Aucun nouvel objet, attaque,
code ou rendu natif intégré.

Proposition : [DRONE-STAT04](../docs/reviews/refonte-agent-2026-10-06/review-drone-invocation-rank-bonuses-01.json).

## DRONE-STAT04-ACCEPT01 Rangs d’invocation validés — 9 octobre 2026

Le joueur répond « parfait :) je valide ». Chaque rang après le premier
apporte dix points de pourcentage de PV maximum et de dégâts aux drones
armés, uniquement au modèle correspondant. Les rangs effectifs d’équipement
continuent au-delà de dix. Les exemples présentés sont acceptés : rang
onze à cent pour cent, rang douze à cent-dix pour cent et comparaison
Traqueur trois-cent-vingt-cinq PV/soixante-cinq dégâts puis trois-cent-
quarante PV/soixante-sept dégâts avec les mêmes autres conditions. Les
autres effets spécifiques et paramètres restent dans leur cadrage.
Aucun code ou rendu natif intégré.

Décision : [DRONE-STAT04-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/drone-invocation-rank-bonus-acceptance-decisions.json).
Manifeste accepté : [DRONE-STAT04, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-drone-invocation-rank-bonuses-02.json).

## PETS-EQ02 Arme confiée et attaque d’origine proposées — 9 octobre 2026

La **progression des rangs d’invocation est validée**.

**PETS-EQ02 · Armes des familiers — proposition**

**Sans arme confiée**

Le familier utilise son attaque d’origine. Pour un drone de combat, il s’agit de son arme intégrée, avec une plage de dégâts adaptée à son modèle et à son niveau.

**Avec une arme compatible équipée et active**

- La plage de dégâts de cette arme **remplace la base de son attaque ordinaire**.
- Les attributs du familier et les bonus compatibles s’appliquent ensuite **une seule fois**.
- Ses autres compétences gardent leur fonctionnement propre.
- Retirer l’arme lui rend son attaque d’origine.

Pour les drones, les bonus d’invocation, d’Intelligence de l’agent et d’Algorithmes de combat améliorés restent applicables aux attaques compatibles.

**Exemple fictif, avant les bonus**

- Attaque intégrée : **10–15 dégâts**.
- Arme confiée : **20–30 dégâts**.
- Base utilisée avec cette arme : **20–30 dégâts**.

Les dégâts de chaque type sont tirés séparément entre leur minimum et leur maximum, avec une chance égale pour chaque entier.

L’attaque d’origine fournit une base de combat ; elle ne crée pas un objet récupérable. Les armes réellement confiées au familier conservent la règle de dépôt au sol à sa mort ou à la fin d’une résurrection.

L’Éclaireur et la Mule restent sans attaque. Les emplacements, les munitions et les modes d’attaque seront définis ensuite.

**Ces règles de remplacement et de variance restent à valider.**

Le remplacement concerne la base de l’attaque ordinaire et conserve
les conditions propres des autres compétences. Les plages sont fictives
et ne définissent aucun objet réel. La variance par composante est
présentée pour les familiers ; les groupements, modes, munitions et profils
critiques restent à revoir. L’attaque d’origine ne crée aucun objet
récupérable ; les instantanés de loot et d’équipement du monstre ressuscité
restent à cadrer sans supposer une duplication. Les compléments PETS-EQ01
ne sont pas approuvés par cette proposition. Aucun code, équipement
réel, dessin natif ou animation intégré.

Proposition : [PETS-EQ02](../docs/reviews/refonte-agent-2026-10-06/review-familiar-weapon-damage-base-PETS-EQ02-01.json).


## PETS-EQ02-ACCEPT01 Armes des familiers validées — 9 octobre 2026

Le joueur répond « oui ca me va :) ». Il valide la proposition PETS-EQ02 :
l’arme compatible équipée et active remplace la base de l’attaque ordinaire,
les attributs propres et bonus compatibles s’appliquent une fois, le retrait
rend l’attaque d’origine et les autres compétences conservent leurs règles.
Les dégâts par type suivent un tirage entier uniforme indépendant entre
minimum et maximum. Les bonus d’invocation, d’Intelligence de l’agent et
O03 restent compatibles avec les attaques des drones. L’attaque native
ne crée aucun objet récupérable ; les armes réellement confiées suivent
la direction de dépôt au sol. Éclaireur et Mule restent sans attaque.
L’exemple de plages dix–quinze et vingt–trente est fictif. Les paramètres
non montrés et les compléments PETS-EQ01 restent à revoir. Aucun code,
nouvel équipement réel ou rendu natif intégré.

Décision : [PETS-EQ02-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/familiar-weapon-damage-base-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-weapon-damage-base-PETS-EQ02-02.json).

## PETS-EQ01 Compléments d’équipement repris — 9 octobre 2026

Les **règles d’armes des familiers sont validées**.

**PETS-EQ01 · Équipement des familiers — compléments proposés**

Pour compléter ce système, je propose :

- **Conditions d’équipement :** chaque objet indique son niveau minimum et les types de familiers compatibles. Le familier doit remplir les deux conditions : niveau suffisant et type compatible — mêlée, distance physique ou distance magique.
- **Niveau des ressuscités :** ils conservent le niveau du monstre d’origine. Les drones suivent le niveau du joueur, comme validé.
- **Bonus d’équipement :** les effets compatibles bénéficient au familier qui porte l’objet.
- **Récupération :** à sa mort, à la fin d’une résurrection ou lors d’une destruction volontaire, les objets portés tombent sur sa case, avec les objets déjà présents. Un drone rappelé conserve son équipement.
- **Accès :** un bouton **« Équipement »** dans la fenêtre **« Familiers »** ouvre la fiche du familier choisi. La Mule conserve aussi son rôle de transport.

Ces compléments restent à valider. Nous pourrons ensuite définir les emplacements et les familles d’objets autorisées pour chaque modèle ou type de familier.

La révision conserve l’identifiant PETS-EQ01 et reprend uniquement les
compléments précédemment proposés. Les confirmations DRONE-STAT01 à 04
et PETS-EQ02 ne valent pas accord sur ces compléments. La progression
propre des drones et les nouvelles bases d’attaque supplantent les anciennes
annotations de niveau limité à l’équipement et de dégâts limités au rang ;
les documents historiques restent conservés. Les emplacements, munitions,
coûts et temps de transfert, conditions de rappel, réglages des jauges et
rendu natif ne sont pas fixés par cette proposition. Seul le libellé d’accès
est proposé, sans nouvelle fenêtre dessinée ou intégrée.

Proposition : [PETS-EQ01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-equipment-PETS-EQ01-02.json).

## PETS-EQ01-ACCEPT01 Équipement des familiers validé — 9 octobre 2026

Le joueur répond « je valide :) » aux compléments repris dans la révision 2.
Le bouton « Équipement » de la fenêtre « Familiers » ouvre la fiche du familier
choisi. L’accord porte sur le libellé et la fonction, sans approuver le rendu
natif. Les objets exigent le niveau minimum et le type compatible, leurs bonus
compatibles bénéficient au porteur et les ressuscités gardent le niveau du
monstre d’origine. Les objets portés tombent sur la case lors de la mort,
de la fin de résurrection ou de la destruction volontaire ; un drone rappelé
conserve son équipement. La Mule garde son transport. Les emplacements,
familles autorisées, transferts et rendu natif restent à définir.

Décision : [PETS-EQ01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/familiar-equipment-complements-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ01, révision 3](../docs/reviews/refonte-agent-2026-10-06/review-familiar-equipment-PETS-EQ01-03.json).

## PETS-EQ03 Emplacements des familiers proposés — 9 octobre 2026

Les **compléments sur l’équipement des familiers sont validés**.

**PETS-EQ03 · Emplacements des familiers — proposition**

Je propose une fiche simple pour commencer.

**Traqueur, Artilleur, Gardien et monstres ressuscités**

- **1 emplacement d’arme**, compatible avec leur vocation.
- **1 emplacement d’armure de torse**, compatible avec le familier.

Une arme compatible à une ou deux mains occupe l’unique emplacement d’arme et devient son arme active.

**Éclaireur et Mule**

- **1 emplacement d’armure de torse**.

Ces deux drones conservent leur rôle sans attaque.

Les armes et armures viennent du **même loot que celui du joueur**, avec les conditions de niveau et de compatibilité validées.

Le rangement de la Mule reste distinct : son sac transporte les objets, ses emplacements servent à l’équiper.

**Ces emplacements restent à valider.**

La proposition définit uniquement les emplacements, leur portée et l’usage
de l’arme unique. Elle ne crée pas de nouveau modèle d’objet et ne modifie
pas les emplacements du joueur. Les conditions de niveau et de compatibilité
validées restent nécessaires ; les familles exactes, munitions, coût des
transferts, règles de protection et affixes compatibles restent à revoir.
Les dispositions graphiques et équipements visibles seront soumis à la revue
native de la direction artistique avant intégration. Aucun code, dessin ou
objet actif intégré.

Proposition : [PETS-EQ03](../docs/reviews/refonte-agent-2026-10-06/review-familiar-equipment-slots-PETS-EQ03-01.json).

## PETS-EQ03-ACCEPT01 Emplacements validés — 9 octobre 2026

Le joueur répond « parfait :) » à la proposition PETS-EQ03. Traqueur,
Artilleur, Gardien et monstres ressuscités possèdent un emplacement d’arme
et un d’armure de torse ; Éclaireur et Mule possèdent un emplacement d’armure
de torse. L’unique arme compatible équipée devient active, qu’elle soit
à une ou deux mains. Les objets proviennent du loot du joueur et respectent
le niveau et la compatibilité validés. Le sac de la Mule conserve une fonction
de transport distincte de ses emplacements d’équipement. Cet accord ne valide
ni familles exactes, munitions, règles de transfert ou rendu natif.
Aucun code ni dessin intégré.

Décision : [PETS-EQ03-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/familiar-equipment-slot-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ03, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-equipment-slots-PETS-EQ03-02.json).

## PETS-EQ04 Bonus d’équipement des familiers proposés — 9 octobre 2026

Les **emplacements des familiers sont validés**.

**PETS-EQ04 · Bonus d’équipement des familiers — proposition**

**Attributs et statistiques**

Les bonus compatibles d’attributs, de PV, d’énergie, d’armure, de résistances et de dégâts améliorent le familier qui porte l’objet, selon les règles communes de calcul.

Pour un drone, un bonus d’Intelligence porté augmente sa propre réserve d’énergie. Le bonus d’invocation reste lié à l’Intelligence de l’agent.

**Bonus de compétences**

- Ils augmentent le rang d’une compétence que le familier **possède déjà**, si le bonus est compatible.
- Le rang effectif peut dépasser **10** grâce à ces objets.
- Ils n’apprennent aucune nouvelle compétence et n’augmentent pas les compétences de l’agent.
- Un bonus incompatible est indiqué **« Bonus inactif »** dans la fiche de l’objet consultée pour ce familier.

**PV et énergie actuels**

Changer d’équipement modifie les maximums sans restaurer les jauges. Si un maximum baisse sous la valeur actuelle, celle-ci est ramenée au nouveau maximum.

**Exemple isolé, sans bonus en pourcentage**

- Avant : **70 / 100 PV**.
- Équipement donnant **+15 PV maximum** : **70 / 115 PV**.
- Après retrait : **70 / 100 PV**.

**Ces règles restent à valider.**

La proposition précise l’éligibilité des bonus de compétence aux pouvoirs déjà
possédés et le comportement des jauges lors d’un changement d’équipement.
Elle n’attribue aucun catalogue de compétences aux monstres et ne crée pas
de points d’apprentissage pour les familiers. L’Intelligence propre du drone
reste distincte du bonus d’invocation de l’agent. L’exemple de PV est fictif,
sans pourcentage supplémentaire, et ne génère aucun affixe concret. Les
événements de niveau, rang, invocation, rappel ou changement des attributs
de l’agent, ainsi que les procs et les auras, conservent leur statut de revue.
Les libellés et effets sont proposés ; aucun rendu natif, code ou objet intégré.

Proposition : [PETS-EQ04](../docs/reviews/refonte-agent-2026-10-06/review-familiar-equipment-bonuses-PETS-EQ04-01.json).

## PETS-EQ04-ACCEPT01 Bonus d’équipement validés — 9 octobre 2026

Le joueur répond « parfait :) » à PETS-EQ04. Les bonus compatibles bénéficient
au familier porteur selon les règles communes. L’Intelligence du drone agit
sur son énergie ; l’Intelligence de l’agent reste la source du bonus
d’invocation. Les bonus de rang s’appliquent uniquement aux compétences
compatibles déjà possédées et peuvent dépasser dix, sans nouvelle compétence
ni bonus aux compétences de l’agent. Le libellé « Bonus inactif » est accepté
pour les effets incompatibles dans le contexte du familier. L’équipement
modifie les maximums sans restaurer les jauges ; une baisse les limite au
nouveau maximum. L’exemple isolé soixante-dix/cent, soixante-dix/cent-quinze,
puis soixante-dix/cent est accepté. Les événements de niveau, rang, rappel,
invocation et changements des attributs de l’agent restent à revoir, ainsi
que les affixes exacts, procs et compétences propres aux monstres.
Aucun code, objet ou rendu natif intégré.

Décision : [PETS-EQ04-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/familiar-equipment-bonus-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ04, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-equipment-bonuses-PETS-EQ04-02.json).

## PETS-EQ05 Transferts d’équipement proposés — 9 octobre 2026

Les **bonus d’équipement des familiers sont validés**.

**PETS-EQ05 · Donner et récupérer de l’équipement — proposition**

**Conditions**

- Le familier doit être **déployé, adjacent et visible par l’agent** : dans l’une des huit cases voisines.
- Vous pouvez modifier son équipement **en combat comme au calme**.
- L’objet à lui donner doit se trouver dans **votre inventaire** et satisfaire les conditions de compatibilité.

**Coût**

- Équiper ou retirer un objet : **1 action · 0 énergie**.
- Remplacer un objet : **1 seule action · 0 énergie**.

**Remplacement**

Le nouvel objet rejoint son emplacement ; l’ancien retourne dans votre inventaire. Les bonus sont recalculés à partir du nouvel équipement, selon les règles validées.

**Si l’échange est impossible**

Si votre inventaire ne peut pas accueillir les objets après l’échange, l’action est refusée : les objets restent à leur place et aucune action n’est consommée.

Ouvrir les fiches et consulter les objets ne consomme aucune action.

**Ces conditions et coûts restent à valider.**

La proposition concerne les échanges entre l’inventaire de l’agent et les
emplacements du familier déployé. La capacité est évaluée dans l’état final
de l’échange ; le remplacement applique l’équipement final d’un seul bloc,
sans étape temporaire d’emplacement vide. Aucun délai de préparation n’est
ajouté. Les commandes de rappel et transferts de cargo de la Mule restent
séparés, avec leurs coûts et cas propres à revoir. La résolution générale
des actions des autres acteurs n’est pas redéfinie. Les cas de visibilité
et retours natifs seront examinés dans le prototype. Aucun code, nouvel
objet ou rendu natif intégré.

Proposition : [PETS-EQ05](../docs/reviews/refonte-agent-2026-10-06/review-familiar-equipment-transfers-PETS-EQ05-01.json).

## PETS-EQ05-ACCEPT01 Transferts d’équipement validés — 9 octobre 2026

Le joueur répond « ok parfait :) » à PETS-EQ05. Les échanges exigent un
familier déployé, adjacent dans les huit cases voisines et visible par
l’agent, en combat comme au calme. Le nouvel objet vient de l’inventaire
et doit être compatible. Équiper, retirer ou remplacer coûte une action
et zéro énergie ; le remplacement est un échange unique avec retour de
l’ancien objet dans l’inventaire et recalcul depuis l’équipement final.
La capacité finale doit suffire ; sinon les objets et l’action restent
inchangés. La consultation des fiches et objets ne consomme pas d’action.
Le cargo de la Mule et le rappel conservent leurs revues séparées.
Aucun code, nouvel objet ou rendu natif intégré.

Décision : [PETS-EQ05-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/familiar-equipment-transfer-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ05, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-equipment-transfers-PETS-EQ05-02.json).

## PETS-EQ06 Sac de la Mule proposé — 9 octobre 2026

Les **conditions et coûts des échanges d’équipement sont validés**.

**PETS-EQ06 · Sac de la Mule — proposition**

Dans sa fiche, un bloc **« Transport »** affiche les objets stockés et la **charge actuelle / maximale, en kg**.

**Stockage**

Tout objet transportable de votre inventaire peut y être rangé. Les conditions de niveau et de type de combat concernent l’équipement porté, pas les objets stockés.

Un objet rangé dans le sac **n’accorde aucun bonus** à la Mule ni à l’agent.

**Déposer ou reprendre un objet**

- Mule **déployée, adjacente et visible par l’agent**.
- Transfert possible **en combat comme au calme**.
- Coût : **1 action · 0 énergie**.

Si le dépôt dépasse la capacité de la Mule, ou si votre inventaire ne peut pas accueillir l’objet repris, le transfert est refusé : l’objet reste à sa place et aucune action n’est consommée.

**Règles conservées**

Le chargement reste conservé lors d’un rappel ou d’un changement de couche. À la destruction de la Mule, **tout son chargement tombe sur sa case**, avec les objets déjà présents.

**Les règles de stockage, les coûts et le bloc « Transport » restent à valider.**

La proposition complète uniquement le cargo de la Mule et les transferts
entre celui-ci et l’inventaire de l’agent. Les autres familiers ne reçoivent
pas de sac de transport. Les masses, la courbe complète de capacité et le
coefficient d’Intelligence restent à revoir ; les anciens instantanés de
PV ne sont pas réintroduits. La sélection de quantités, les transferts
de plusieurs objets, l’incidence du poids de l’armure portée et les cas
de capacité qui baisse ne sont pas fixés. Le libellé « Transport » et sa
fonction sont proposés, sans rendu natif ni code intégré.

Proposition : [PETS-EQ06](../docs/reviews/refonte-agent-2026-10-06/review-mule-cargo-transfers-PETS-EQ06-01.json).

## PETS-EQ06-ACCEPT01 Sac validé et Force demandée — 9 octobre 2026

Le joueur valide le sac, les règles de stockage, les transferts et le bloc
« Transport », puis demande d’ajouter de la Force à la Mule pour rendre
cohérente la charge maximale en kilogrammes. Cette demande fixe l’orientation
de portage, sans approuver une nouvelle répartition chiffrée ni une formule.
Le profil historique conserve Force cinq sans croissance ; la capacité
historique reposait déjà sur le rang et un exemple d’Intelligence, donc une
Force faible ne signifiait pas une capacité nulle. Le nouveau lien entre
Force propre et charge est proposé dans PETS-EQ07. Aucun profil accepté,
code ou rendu natif remplacé à ce stade.

Décision : [PETS-EQ06-ACCEPT01 / MULE-FORCE-DIRECTION01](../docs/reviews/refonte-agent-2026-10-06/mule-cargo-acceptance-and-strength-direction-decisions.json).
Manifeste accepté : [PETS-EQ06, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mule-cargo-transfers-PETS-EQ06-02.json).

## PETS-EQ07 Force et capacité de la Mule proposées — 9 octobre 2026

Le **sac de la Mule et ses transferts sont validés**. Son profil actuel garde sa Force à **5**, sans progression ; je propose de l’orienter vers le portage.

**PETS-EQ07 · Force et capacité de la Mule — proposition**

**Attributs de départ**

- **Force : 7 · Agilité : 5 · Vitalité : 9**.
- Autres attributs : **5**.

**À chaque niveau gagné**

- **+2 Force · +3 Vitalité**.

**Capacité de transport**

- Base : **40 kg**.
- Force au-dessus de 5 : **+2 kg par point**.
- Rang d’invocation après le premier : **+8 kg par rang**.

La Force utilisée est celle de la Mule, **bonus d’équipement compris**. La capacité dépend de cette Force et du rang d’invocation ; l’Intelligence de l’agent conserve ses bonus de PV et de dégâts aux drones.

**Exemples, sans bonus de Force d’équipement**

- Mule niveau 1, invocation rang 1 : **Force 7 · 44 kg**.
- Mule niveau 20, invocation rang 11 : **Force 45 · 200 kg**.
- Prochain rang — 12 · 1 point de compétence : **208 kg**.

**Ce profil et ces valeurs restent à valider.**

La proposition déplace les deux points d’Agilité initiaux et les deux points
d’Agilité par niveau vers la Force, dans les budgets déjà validés de trente-six
points au départ et cinq par niveau. La Vitalité reste neuf puis trois par
niveau. La nouvelle capacité repose sur la Force propre, ses bonus portés
et le rang effectif de l’invocation, sans multiplicateur d’Intelligence de
l’agent. Si elle est acceptée, cette courbe remplacera les anciens exemples
de capacité D57 dans la refonte ; les instantanés historiques restent conservés.
Les autres modèles, règles de PV, coûts, recharges et absence d’attaque de
la Mule ne sont pas révisés. Les capacités qui baissent, masses et combinaisons
de bonus de charge restent à revoir. Aucun code ou rendu natif intégré.

Proposition : [PETS-EQ07](../docs/reviews/refonte-agent-2026-10-06/review-mule-strength-and-carrying-PETS-EQ07-01.json).

## PETS-EQ07-ACCEPT01 Force et capacité validées — 9 octobre 2026

Le joueur répond « parfait :) on peut continuer ». Il valide Force sept,
Agilité cinq, Vitalité neuf et les autres attributs cinq au départ, puis
Force plus deux et Vitalité plus trois par niveau. La capacité de la Mule
est quarante kilogrammes plus deux par point de Force propre au-dessus
de cinq, plus huit par rang effectif d’invocation après le premier. Les
bonus de Force portés comptent ; l’Intelligence de l’agent ne multiplie plus
la capacité dans ce nouveau modèle. Les exemples quarante-quatre, deux-cents
et deux-cent-huit kilogrammes sont acceptés. Le nouveau profil et la courbe
supplantent les anciens paramètres Mule dans la refonte, via des corrections
explicites ; les sources et accords historiques restent conservés. Les autres
modèles de drones et paramètres D57 ne sont pas révisés. Aucun code ni
rendu natif intégré.

Décision : [PETS-EQ07-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/mule-strength-and-carrying-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ07, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mule-strength-and-carrying-PETS-EQ07-02.json).

## PETS-EQ08 Charge et dépassement proposés — 9 octobre 2026

La **Force et la capacité de transport de la Mule sont validées**.

**PETS-EQ08 · Charge et dépassement de capacité — proposition**

**Poids comptabilisé**

Seuls les objets rangés dans le sac comptent dans la charge affichée. Son armure portée utilise son emplacement d’équipement et ne réduit pas la capacité du sac.

**Si la capacité baisse sous la charge transportée**

- La Mule **conserve tous les objets**.
- Elle **continue de vous suivre**, sans pénalité de déplacement.
- Les nouveaux dépôts sont refusés tant qu’elle dépasse sa capacité.
- Vous pouvez reprendre des objets, même si un premier retrait ne suffit pas à revenir sous la limite, à condition que votre inventaire puisse les accueillir.

Dans le bloc **« Transport »**, la mention **« Surchargée »** accompagne la charge actuelle et la capacité maximale.

**Exemple**

Avec **180 kg** stockés, une capacité qui passe de **200 à 160 kg** donne l’affichage **180 / 160 kg · Surchargée**. Les objets restent dans le sac ; retirer **20 kg** ramène la charge à **160 / 160 kg** et retire cette mention.

Les retraits conservent le coût validé : **1 action · 0 énergie par objet**.

**Ces règles de poids et de surcharge restent à valider.**

La surcharge temporaire proposée concerne un chargement déjà présent après
une baisse de capacité. Le stockage normal continue d’exiger de respecter
la capacité finale. Les objets sont conservés sans nouvelle perte, le retrait
reste possible et la Mule continue de suivre sans ralentissement. L’exemple
est fictif et ne définit aucune malédiction, masse ou modification d’attribut.
Le retrait de vingt kilogrammes représente un ou plusieurs objets réels,
chacun selon le coût validé ; aucun transfert groupé gratuit n’est ajouté.
La mention « Surchargée » est proposée dans Transport, sans nouvel effet de
combat, barre de statut, couleur, animation ou rendu natif intégré.

Proposition : [PETS-EQ08](../docs/reviews/refonte-agent-2026-10-06/review-mule-overcapacity-PETS-EQ08-01.json).

## PETS-EQ08-ACCEPT01 Surcharge validée — 9 octobre 2026

Le joueur répond « ok oui ». La charge compte uniquement le sac, hors armure
portée. Après une baisse de capacité, la Mule conserve ses objets et son
suivi sans ralentissement ; les dépôts sont refusés et les retraits légaux
restent possibles, même si un premier retrait ne résout pas toute la surcharge.
Le bloc Transport indique « Surchargée » avec charge réelle et maximum.
À égalité, la mention disparaît ; les nouveaux objets doivent toujours tenir.
L’exemple cent-quatre-vingts/deux-cents, cent-quatre-vingts/cent-soixante, puis
cent-soixante/cent-soixante est accepté. Le coût par objet reste une action
et zéro énergie. Les masses et autres cas de capacité restent à revoir.
Aucun code ni rendu natif intégré.

Décision : [PETS-EQ08-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/mule-overcapacity-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ08, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mule-overcapacity-PETS-EQ08-02.json).

## PETS-EQ09 Réanimation et objets proposés — 9 octobre 2026

Les **règles de charge et de surcharge de la Mule sont validées**.

**PETS-EQ09 · Réanimation et objets — proposition**

**Au relèvement**

Le ressuscité conserve son profil de combat et ses compétences d’origine, avec la puissance donnée par Réanimation. Ses emplacements **Arme** et **Armure de torse** commencent vides : ses capacités d’origine ne créent pas d’objets récupérables.

Le butin tombé lors de la mort de l’ennemi reste au sol. Vous pouvez ensuite lui confier de vrais objets selon les règles d’équipement validées.

**À la fin du familier**

Son équipement confié tombe **sur sa case** :

- à sa mort ;
- à la fin de la réanimation ;
- lors d’une destruction volontaire ;
- lorsqu’un nouveau réanimé le remplace à la limite de cinq.

**Récompenses**

La mort initiale de l’ennemi accorde ses récompenses habituelles. La mort ou la disparition de sa version ressuscitée ne donne **ni XP ni nouveau butin** ; les objets confiés sont simplement restitués au sol.

**Ces règles d’objets et de récompenses restent à valider.**

La proposition définit les objets réels confiés au ressuscité et les événements
qui les restituent, y compris le remplacement du plus ancien à cinq alliés.
Elle conserve le profil et les compétences d’origine ainsi que les paramètres
validés de Réanimation. Les instantanés exacts de statistiques, anciens
modificateurs d’équipement et effets temporaires restent à revoir afin de
ne pas déduire leur retrait ou leur duplication. La première mort utilise
les récompenses habituelles ; les événements terminaux de l’allié ne créent
pas de seconde récompense dans cette proposition. Aucun objet, code, dessin
de monstre, corps ou animation intégré.

Proposition : [PETS-EQ09](../docs/reviews/refonte-agent-2026-10-06/review-raised-equipment-and-rewards-PETS-EQ09-01.json).

## PETS-EQ09-ACCEPT01 Réanimation et objets validés — 9 octobre 2026

Le joueur répond « Okay parfait :) ». Le ressuscité conserve le profil et
les compétences d’origine ajustés par Réanimation, avec ses emplacements
confiés vides. Aucun objet récupérable n’est fabriqué à partir de ses capacités
natives et le butin initial reste au sol. Les objets réellement confiés
tombent sur sa case à sa mort, à la fin de l’effet, lors d’une destruction
volontaire ou du remplacement du plus ancien à cinq alliés. La mort initiale
accorde ses récompenses normales ; les événements terminaux de l’allié ne
génèrent ni nouvelle XP ni butin, en dehors de la restitution des objets.
Les instantanés exacts des statistiques et l’ordre de puissance restent
à revoir. Aucun code, objet ou rendu natif intégré.

Décision : [PETS-EQ09-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/raised-equipment-and-reward-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ09, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-raised-equipment-and-rewards-PETS-EQ09-02.json).

## PETS-EQ10 Puissance et équipement des ressuscités proposés — 9 octobre 2026

La **gestion des objets et des récompenses des ressuscités est validée**.

**PETS-EQ10 · Puissance et équipement des ressuscités — proposition**

Pour calculer leurs PV maximum et leurs dégâts :

- On calcule les valeurs avec **leurs attributs et les bonus compatibles de leur équipement**.
- On applique ensuite **une seule fois la puissance de Réanimation**.

Une arme confiée remplace la base de leur attaque ordinaire, comme validé. Les dégâts sont calculés avant les défenses de la cible.

La puissance de Réanimation concerne **les PV maximum et les dégâts**. L’armure, les résistances, la précision, l’esquive et l’énergie gardent leur calcul habituel.

**Exemple fictif**

Après les attributs et l’équipement, avant Réanimation : **200 PV maximum · 40–60 dégâts**.

**Effet actuel — Réanimation rang 11**

- Puissance : **100 %**.
- Résultat : **200 PV maximum · 40–60 dégâts**.

**Prochain rang — 12 · 1 point de compétence**

- Puissance : **105 %**.
- Résultat : **210 PV maximum · 42–63 dégâts**.

L’exemple conserve le même équipement et les mêmes attributs.

**Cet ordre de calcul reste à valider.**

La proposition introduit une couche propre à la puissance de Réanimation,
appliquée une fois après les attributs et l’équipement compatibles. Les
coefficients cent et cent-cinq pour cent des rangs onze et douze sont déjà
validés ; leur ordre par rapport aux nouveaux objets est soumis ici. Les
valeurs fictives sont avant les défenses et ne définissent pas un monstre.
Les autres statistiques conservent leurs règles communes. Les instantanés
exactement repris de la cible d’origine, les jauges actuelles et catégories
de dégâts particulières restent à revoir. Aucun code, objet ou rendu natif
intégré.

Proposition : [PETS-EQ10](../docs/reviews/refonte-agent-2026-10-06/review-raised-power-and-equipment-PETS-EQ10-01.json).


## PETS-EQ10-ACCEPT01 Puissance avec équipement validée — 9 octobre 2026

Le joueur répond « parfait :) oui ». Les attributs propres et les bonus
compatibles d’équipement sont calculés avant l’application unique du facteur
relatif de Réanimation aux PV maximum et aux dégâts. Les autres statistiques
gardent leurs règles communes. Les coefficients déjà validés cent et cent-cinq
pour cent et l’exemple deux-cents PV/quarante à soixante dégâts, puis deux-cent-dix
PV/quarante-deux à soixante-trois dégâts, sont acceptés. Les instantanés d’origine,
jauges actuelles, courbe entière et interactions particulières restent à revoir.
Aucun code, objet ou rendu natif intégré.

Décision : [PETS-EQ10-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/raised-power-and-equipment-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ10, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-raised-power-and-equipment-PETS-EQ10-02.json).

## PETS-EQ11 Profil d’origine proposé — 9 octobre 2026

Le **calcul de puissance de Réanimation est validé**.

**PETS-EQ11 · Ce que conserve le ressuscité — proposition**

**Profil conservé**

- Son **niveau d’origine**.
- Ses **six attributs propres**, hors bonus d’équipement.
- Ses statistiques naturelles et ses compétences d’origine.
- Son attaque d’origine, disponible même sans arme confiée.

**Équipement et effets temporaires**

Les bonus des objets portés par l’ennemi ne sont pas recopiés. Seuls les objets réellement confiés au familier lui apportent des bonus d’équipement.

Ses anciens buffs et débuffs prennent fin au relèvement : il ne conserve pas, par exemple, le Poison ou le Gel qui l’affectait avant sa mort.

**Exemple fictif — Force**

- Avant sa mort : **15 propres + 5 d’équipement = 20**.
- Au relèvement, sans équipement confié : **15**.
- Avec une armure confiée donnant +3 Force : **18**.

On applique ensuite la puissance de Réanimation selon l’ordre validé.

**Ces règles de reprise du profil restent à valider.**

La proposition précise la reprise du profil permanent et des attributs propres,
hors anciens modificateurs d’objets et effets temporaires. Les variations
intrinsèques de niveau ou de génération et les compétences natives restent
conservées ; aucun nouveau profil de monstre n’est tiré. L’attaque native reste
disponible sans matérialiser une arme récupérable. Les bonus des objets confiés
sont ensuite calculés selon les règles acceptées. Le retrait des anciens bonus
d’objets et des effets temporaires est soumis ici, ces détails étant restés
ouverts dans OC07 et PETS-EQ09. L’exemple de Force est fictif. Les jauges actuelles,
recharges, instant exact de copie et catégories concrètes d’effets restent à
revoir. Aucun code, objet ou rendu natif intégré.

Proposition : [PETS-EQ11](../docs/reviews/refonte-agent-2026-10-06/review-raised-origin-profile-PETS-EQ11-01.json).


## PETS-EQ11-ACCEPT01 Profil d’origine validé — 9 octobre 2026

Le joueur répond « oui tout à fait :) ». Le ressuscité conserve son niveau,
ses six attributs propres hors équipement, ses statistiques naturelles,
ses compétences et son attaque native. Les anciens bonus d’objets et effets
temporaires ne sont pas recopiés ; les objets réellement confiés apportent
ensuite leurs bonus. L’exemple de Force quinze plus cinq avant la mort,
quinze au relèvement puis dix-huit avec une armure confiée est accepté.
Les règles d’identité des objets et l’ordre de puissance restent conservés.
Aucun code, objet ou rendu natif intégré.

Décision : [PETS-EQ11-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/raised-origin-profile-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ11, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-raised-origin-profile-PETS-EQ11-02.json).

## PETS-EQ12-DECISION01 Jauges au relèvement fixées — 9 octobre 2026

Le joueur indique directement « pv et energie rechargé au maximum. » dans
le contexte des jauges des ressuscités au relèvement. Leurs PV et leur énergie
actuels sont égaux à leurs maximums calculés à la naissance. Cette instruction
ne fixe ni nouvelle base d’énergie, ni recharge des compétences, ni règles
de rappel des drones ou de restauration après modification des bonus.
La synthèse de règle n’est pas une nouvelle fiche de compétence intégrée.
Aucun code ni rendu natif intégré.

Décision : [PETS-EQ12-DECISION01](../docs/reviews/refonte-agent-2026-10-06/raised-starting-resources-user-decisions.json).
Règle acceptée : [PETS-EQ12](../docs/reviews/refonte-agent-2026-10-06/review-raised-starting-resources-PETS-EQ12-01.json).

## PETS-EQ13 Évolution des familiers présents proposée — 9 octobre 2026

Le **profil conservé par les ressuscités est validé**. À leur relèvement, leurs **PV et leur énergie sont au maximum**, comme demandé.

**PETS-EQ13 · Évolution des familiers déjà présents — proposition**

**Drones**

- Ils suivent votre niveau, y compris lorsqu’ils sont déjà déployés.
- Quand vous gagnez un niveau, leurs **PV et leur énergie sont restaurés au maximum**, comme les vôtres.
- Leur puissance est recalculée quand le rang de leur invocation ou votre Intelligence change.

**Ressuscités**

- Ils conservent leur niveau d’origine.
- Leur puissance est recalculée quand le rang effectif de Réanimation change.
- Leur durée reste celle fixée au relèvement : ces changements ne prolongent pas leur présence.

**Changements de bonus**

Modifier un rang, un attribut ou un équipement **ne restaure pas les jauges**. Si un maximum augmente, la réserve actuelle reste identique ; s’il baisse sous cette réserve, elle est ramenée au nouveau maximum.

**Exemple fictif — hors gain de niveau**

- Avant un bonus : **70 / 100 PV**.
- Après un bonus augmentant le maximum : **70 / 130 PV**.
- Si le maximum descend ensuite à 60 : **60 / 60 PV**.

**Ces règles de mise à jour restent à valider.**

La proposition rend les bonus concernés dynamiques sur les familiers déjà
présents. Les drones suivent le niveau courant de l’agent et restaureraient
leurs jauges après un gain réel de niveau, une fois leurs statistiques mises
à jour. Les achats de rangs et changements d’attributs ou d’objets n’apporteraient
pas cette restauration. Les ressuscités garderaient leur niveau d’origine et
leur durée de naissance, tandis que leur puissance suivrait le rang effectif
courant de Réanimation. Les autres sources de soin et restauration gardent
leurs règles. Les apparitions et rappels de drones, recharges et ordre exact
des événements restent à revoir. Aucun code ou rendu natif intégré.

Proposition : [PETS-EQ13](../docs/reviews/refonte-agent-2026-10-06/review-familiar-stat-updates-PETS-EQ13-01.json).


## PETS-EQ13-ACCEPT01 Évolution des familiers validée — 9 octobre 2026

Le joueur répond « parfait :) on peut continuer ». Les drones présents suivent
le niveau de l’agent et restaurent leurs jauges après un gain réel de niveau.
Leurs bonus de rang d’invocation et d’Intelligence de l’agent sont recalculés
lorsque ces valeurs changent. Les ressuscités gardent leur niveau et durée
d’origine, tandis que leur puissance suit le rang effectif de Réanimation.
Les changements de rang, attribut ou objet ne restaurent pas les réserves :
les hausses de maximum conservent la quantité actuelle et les baisses la
plafonnent. L’exemple soixante-dix/cent puis soixante-dix/cent-trente et
soixante/soixante est accepté. La lecture historique du rang uniquement au
déploiement est remplacée pour les bonus de PV et dégâts concernés, via une
correction explicite conservant les sources historiques. Les drones rappelés,
naissances, coûts et événements restent à préciser. Aucun code ni rendu natif
intégré.

Décision : [PETS-EQ13-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/familiar-stat-updates-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ13, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-stat-updates-PETS-EQ13-02.json).

## PETS-EQ14 Déploiement et rappel des drones proposés — 9 octobre 2026

Les **mises à jour de puissance et des jauges des familiers sont validées**.

**PETS-EQ14 · Déploiement et rappel des drones — proposition**

**Drone neuf**

Lors de son premier déploiement, il possède **100 % de ses PV maximum et de son énergie maximale**.

**Rappel et redéploiement**

- Le rappel conserve **le même drone**, ses jauges, son équipement et, pour la Mule, son chargement.
- Le redéployer **ne restaure ni ses PV ni son énergie**.
- Un gain de niveau du joueur restaure aussi les jauges des drones rappelés. Les changements de bonus suivent les règles validées.

**Après destruction**

Une nouvelle invocation crée un **nouveau drone aux jauges pleines**. Les objets déposés par l’ancien restent au sol : ils ne sont pas transférés automatiquement au nouveau.

Chaque déploiement conserve le coût d’énergie et la recharge de sa compétence.

**Exemple fictif — sans autre événement entre les deux**

- Avant le rappel : **30 / 100 PV · 12 / 40 énergie**.
- Après redéploiement : **30 / 100 PV · 12 / 40 énergie**.

**Ces règles de déploiement restent à valider.**

La proposition distingue un nouvel acteur de la conservation d’un acteur
rappelé. Un drone neuf partirait avec ses jauges pleines, y compris après une
destruction puis nouvelle invocation légale. Rappel et redéploiement conservent
l’identité, les objets et les jauges sans les remplir. La restauration sur
un gain réel de niveau est proposée explicitement aussi pour les drones
rappelés. Les coûts et recharges des invocations restent conservés. L’exemple
suppose aucun autre événement entre rappel et redéploiement. Le comportement
des effets temporaires, l’énergie pendant le rappel et les commandes exactes
restent à revoir ; Maintenance reste limitée aux drones déployés. Aucun code,
objet ou rendu natif intégré.

Proposition : [PETS-EQ14](../docs/reviews/refonte-agent-2026-10-06/review-drone-deployment-resources-PETS-EQ14-01.json).


## PETS-EQ14-ACCEPT01 Déploiement et rappel validés — 9 octobre 2026

Le joueur répond « parfait on peut continuer ». Un nouveau drone part avec
ses PV et son énergie au maximum. Le rappel conserve le même acteur, ses
jauges, ses objets et le cargo de la Mule ; le redéploiement ne les restaure
pas. Les drones rappelés suivent aussi les gains réels de niveau de l’agent
avec restauration des jauges, tandis que les autres bonus suivent les règles
sans restauration. Après destruction, une nouvelle invocation légale crée
un acteur neuf aux jauges pleines, sans récupérer automatiquement les objets
laissés au sol. Les coûts et recharges de déploiement restent conservés.
L’exemple de trente/cent PV et douze/quarante énergie inchangés après rappel
et redéploiement est accepté. Les commandes exactes et effets pendant le
rappel restent à revoir. Aucun code ni rendu natif intégré.

Décision : [PETS-EQ14-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/drone-deployment-resources-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ14, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-drone-deployment-resources-PETS-EQ14-02.json).

## PETS-EQ15 Commande de rappel proposée — 9 octobre 2026

Les **règles de déploiement et de conservation des drones sont validées**.

**PETS-EQ15 · Commande de rappel — proposition**

Dans la fenêtre **Familiers**, chaque drone possède un bouton **« Rappeler »**.

**Conditions**

- Drone déployé sur votre carte.
- Sur l’une des **8 cases adjacentes** à votre personnage.
- Visible directement par votre personnage.
- Disponible en **Combat, Accalmie et Calme**.

**Coût : 0 énergie · Aucune recharge propre.**

Le rappel conserve la recharge restante de l’invocation : rappeler le drone ne permet pas de le redéployer avant sa fin.

Une fois rappelé, le drone quitte la carte et cesse d’y agir. L’Éclaireur ne fournit plus de vision.

**Bouton indisponible**

Le survol indique la raison : **« Drone trop éloigné »**, **« Drone hors de vue »** ou **« Drone déjà rappelé »**.

**Ces conditions et ces textes restent à valider.**

La proposition définit un rappel de proximité sur une des huit cases
adjacentes et directement visible par l’agent, pendant toutes les phases.
L’énergie et la recharge propre sont nulles ; la recharge d’invocation reste
conservée. Le rappel retire le drone de la carte tout en conservant son état
et ses objets. Le bouton et les motifs d’indisponibilité sont proposés en
texte, sans intégration native. Le partage actif de vision de l’Éclaireur
cesse, sans effacer la mémoire d’exploration. Les effets temporaires déjà
appliqués par un drone, les événements du rappel et le rendu restent à revoir.
Aucun code, objet ou rendu natif intégré.

Proposition : [PETS-EQ15](../docs/reviews/refonte-agent-2026-10-06/review-drone-recall-command-PETS-EQ15-01.json).


## PETS-EQ16-DIRECTION01 Ramassage de la Mule demandé — 9 octobre 2026

Le joueur demande un bouton de ramassage pour la Mule, puis un message
flottant au-dessus d’elle indiquant le nom de chaque objet ramassé. Il corrige
ensuite le périmètre : il s’agit du champ de vision du joueur, et non de la
vision propre de la Mule demandée initialement. Le bouton, ce périmètre et
le nom flottant sont des orientations explicites ; le déplacement, le temps,
les coûts, l’arrêt et le rendu exact sont proposés séparément. Cette demande
additionnelle ne valide pas les conditions de rappel PETS-EQ15, toujours en
attente. Aucun code, objet ou rendu natif intégré.

Décision : [PETS-EQ16-DIRECTION01](../docs/reviews/refonte-agent-2026-10-06/mule-loot-collection-direction-decisions.json).

## PETS-EQ16 Ramasser proposé — 9 octobre 2026

**PETS-EQ16 · Mule — Ramasser**

Dans la fenêtre **Familiers**, la Mule aura un bouton **« Ramasser »**. Elle cherchera les loots dans **le champ de vision actuel du joueur**.

À chaque objet réellement ramassé, **son nom apparaîtra au-dessus de la Mule dans un message flottant**.

**Fonctionnement proposé**

- Elle se déplace vers le loot accessible le plus proche, en restant dans votre champ de vision.
- Elle range les objets dans **son sac**, dans la limite de sa capacité. Ceux qui ne tiennent pas restent au sol.
- Les déplacements et le ramassage utilisent ses propres tours : **1 case par tour**, puis **1 tour par case pour ramasser les objets qui tiennent dans son sac**. Vous pouvez continuer à agir pendant sa collecte.
- Quand aucun loot visible et accessible ne peut être stocké, elle reprend votre suivi.
- Pendant la collecte, le bouton devient **« Arrêter »** pour interrompre l’ordre et reprendre le suivi.

**Coût proposé : 0 énergie · Aucune recharge propre.**

**Les modalités de ramassage restent à valider.**

La collecte proposée suit le champ de vision courant du joueur. La Mule se
déplace réellement sur les cases accessibles de cette zone, puis transfère
les objets qui tiennent vers son vrai cargo. Elle utilise ses propres tours
pendant que le joueur continue d’agir. Les objets trop lourds restent au sol,
le cargo ne fournit pas de bonus et aucune duplication ni télécollecte n’est
introduite. Chaque transfert réel produit le nom de l’objet au-dessus de la
Mule, comme demandé. Les priorités entre objets, commandes exactes, visibilités
partagées et rendu natif restent à revoir. Les anciens accords de capacité
et d’identité sont conservés. PETS-EQ15 reste en attente de validation.
Aucun code, objet ou rendu natif intégré.

Proposition : [PETS-EQ16](../docs/reviews/refonte-agent-2026-10-06/review-mule-loot-collection-PETS-EQ16-01.json).


## PETS-EQ16-ACCEPT01 Ramassage de la Mule validé — 9 octobre 2026

Le joueur répond « parfait on continue ». Les boutons Ramasser et Arrêter,
le butin actuellement dans le champ de vision du joueur, les trajets visibles,
le transfert réel vers le cargo dans sa capacité et le retour au suivi sont
acceptés. La Mule avance d’une case par tour et utilise un tour par case pour
les objets qui tiennent, pendant que le joueur continue d’agir. L’ordre coûte
zéro énergie et ne possède pas de recharge propre. Le nom flottant de chaque
objet ramassé est confirmé ; les pixels, polices, durée et empilement des
messages restent à revoir. Les priorités entre objets et événements de
commande restent ouverts. PETS-EQ15 n’est pas validé par cette réponse à
PETS-EQ16. Aucun code, objet ou rendu natif intégré.

Décision : [PETS-EQ16-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/mule-loot-collection-acceptance-decisions.json).
Manifeste accepté : [PETS-EQ16, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mule-loot-collection-PETS-EQ16-02.json).

## PETS-EQ17 Effets des drones rappelés proposés — 9 octobre 2026

Le **ramassage de la Mule et les messages flottants sont validés**.

**PETS-EQ17 · Effets et recharges pendant le rappel — proposition**

**Buffs et débuffs du drone**

Ils restent présents, avec leur durée restante **mise en pause**. Le Poison ou le Saignement n’infligent aucun dégât pendant le rappel ; leurs effets reprennent au redéploiement.

**PV et énergie**

Le drone rappelé ne reçoit ni soin périodique ni régénération d’énergie. La restauration au gain de niveau reste celle déjà validée.

**Recharges**

Les recharges du drone et de son invocation **continuent de diminuer avec les tours du jeu**. Le rappel ne les remet pas à zéro.

**Exemple fictif**

Avant le rappel : **buff — 3 tours · Poison — 2 tours · recharge — 4 tours**.

Après 3 tours de jeu : **buff — 3 tours · Poison — 2 tours · recharge — 1 tour**.

**Ces règles de pause des effets restent à valider.**

La proposition conserve les effets temporaires propres au drone avec une
pause des durées, dégâts et restaurations périodiques tant qu’il est rappelé.
Le rappel ne purge pas les affaiblissements ; le redéploiement les reprend.
Les recharges continuent sur les tours réels du jeu, sans progression par
menus ou temps réel. La restauration sur gain réel de niveau déjà validée
reste appliquée. Les effets donnés auparavant au joueur, à d’autres acteurs
ou aux zones ne sont pas inclus dans cette pause. Les interactions de
statistiques temporaires et l’ordre exact des événements restent à revoir.
PETS-EQ15 reste en attente. Aucun code ni rendu natif intégré.

Proposition : [PETS-EQ17](../docs/reviews/refonte-agent-2026-10-06/review-recalled-drone-effects-PETS-EQ17-01.json).


## PETS-EQ14-CLARIFY01 Sens du redéploiement — 9 octobre 2026

Le joueur demande si le redéploiement signifie détruire un drone puis le
réinvoquer. Il désigne ici le retour sur la carte du même drone vivant après
rappel, via sa compétence d’invocation et avec ses jauges et objets conservés.
Détruire puis invoquer crée un acteur neuf aux jauges pleines ; les objets
réels de l’ancien restent au sol. Cette explication reprend PETS-EQ14 et
ne modifie aucune règle. Le brouillon PETS-EQ17 ci-dessus a été préparé mais
n’a pas encore été présenté dans la réponse finale : seul le principe de
pause a été évoqué en commentaire. La question ne valide ni PETS-EQ17 ni
les conditions de rappel PETS-EQ15.

Clarification : [PETS-EQ14-CLARIFY01](../docs/reviews/refonte-agent-2026-10-06/drone-redeployment-terminology-clarification.json).


## PETS-EQ15-DIRECTION02 Repositionnement et recharge demandés — 9 octobre 2026

Le joueur questionne l’utilité du rangement par rappel et propose plutôt
une téléportation du drone sur une case adjacente pour débloquer un décor
ou un passage. Il demande ensuite une recharge pour éviter les sauvetages
répétés contre des monstres. La fonction et la présence d’une recharge sont
ses directions explicites ; le remplacement du rangement accepté dans
PETS-EQ14, le nom Repositionner, les conditions et les paramètres exacts sont
soumis dans la révision deux de PETS-EQ15. Les anciens accords sont conservés,
sans suppression silencieuse. Le brouillon PETS-EQ17 non présenté et jamais
accepté est retiré de la revue courante. Aucun code ni rendu natif intégré.

Décision : [PETS-EQ15-DIRECTION02](../docs/reviews/refonte-agent-2026-10-06/drone-repositioning-direction-decisions.json).
Brouillon retiré : [PETS-EQ17, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-recalled-drone-effects-PETS-EQ17-02.json).

## PETS-EQ15 Repositionner proposé — 9 octobre 2026

Oui, une commande pour **replacer un drone près de vous** serait plus utile pour le débloquer.

**PETS-EQ15 · Repositionner — proposition révisée**

Je propose de remplacer le rangement par rappel par un bouton **« Repositionner »** dans la fenêtre **Familiers**.

- Vous choisissez un drone déployé sur votre carte, puis une **case adjacente libre, praticable et visible**.
- Le même drone est téléporté sur cette case, même s’il est coincé ou hors de votre champ de vision.
- Il conserve **ses PV, son énergie, son équipement, son chargement, ses buffs et ses débuffs**. Ses autres recharges restent inchangées.
- Si aucune case adjacente ne convient, l’action est indisponible.

**Coût proposé : 0 énergie · Recharge : 6 tours par drone.**

Cela permettrait de le débloquer tout en limitant les sauvetages répétés en combat. Le drone resterait sur la carte ; il n’y aurait plus d’étape de rangement ni de mise en pause de ses effets.

**Ce remplacement et ses paramètres restent à valider.**

La révision propose de déplacer le même acteur uniquement, sans déplacer le
joueur, créer un drone, soigner ni effacer ses effets. La destination est
choisie parmi les cases voisines libres, praticables et visibles ; l’origine
peut être bloquée ou hors de vue sur la même carte. La recharge de six tours
est propre à chaque drone et distincte des autres recharges conservées. Le
coût nul, cette durée et sa portée par drone restent proposés. La suppression
du rangement implique une révision explicite de la partie correspondante de
PETS-EQ14, en attente d’accord. Les nouveaux drones après destruction, leurs
jauges pleines, les objets réels et toutes les statistiques restent conservés.
Échange de position HY04 reste une compétence séparée avec ses propres règles.
Les transitions, ordres en cours, choix natif de case et animation restent à
revoir. Aucun code, objet ou rendu natif intégré.

Proposition : [PETS-EQ15, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-drone-recall-command-PETS-EQ15-02.json).


## PETS-EQ15 Repositionner validé, origine hors de vue ou de portée — 9 octobre 2026

Le joueur valide la proposition par « oui ca me va :) », puis précise :
« ce bouton serait accessible aussi en cas de drone hors du champ de vision du joueur (donc hors de portée) ».

Le bouton Repositionner est accessible pour un drone vivant déployé sur la
même carte, sans limite de distance à son origine ni exigence de visibilité
ou de chemin vers lui. Le joueur choisit une case d’arrivée parmi les huit
cases adjacentes libres, praticables et visibles. Seul le drone est déplacé.
Le coût est de 0 énergie et la recharge de 6 tours, propre à chaque drone.
Ses PV, son énergie, ses objets équipés, son chargement, ses buffs, ses débuffs
et ses autres recharges sont conservés. Sans case d’arrivée valide, l’action
est indisponible.

Cette validation remplace explicitement le rangement manuel par rappel de
PETS-EQ14. Les reçus historiques restent conservés ; la création d’un nouveau
drone aux jauges pleines après destruction, les dépôts réels au sol et les
règles statistiques restent valables. PETS-EQ17 était un brouillon non accepté
et retiré : aucune pause d’effets hors carte n’est ajoutée. Échange de position
HY04 conserve ses propres règles. Les transitions de cartes, ordres en cours,
interactions avec les zones et le rendu natif restent à revoir.
Aucun code ni rendu natif intégré.

Copie effectivement présentée et validée :

**PETS-EQ15 · Repositionner — proposition révisée**

Je propose de remplacer le rangement par rappel par un bouton **« Repositionner »** dans la fenêtre **Familiers**.

- Vous choisissez un drone déployé sur votre carte, puis une **case adjacente libre, praticable et visible**.
- Le même drone est téléporté sur cette case, même s’il est coincé ou hors de votre champ de vision.
- Il conserve **ses PV, son énergie, son équipement, son chargement, ses buffs et ses débuffs**. Ses autres recharges restent inchangées.
- Si aucune case adjacente ne convient, l’action est indisponible.

**Coût proposé : 0 énergie · Recharge : 6 tours par drone.**

Cette recharge limiterait les sauvetages répétés en combat. Le drone resterait sur la carte ; il n’y aurait plus d’étape de rangement ni de mise en pause de ses effets.

**Ce remplacement et ses paramètres restent à valider.**

Le dernier avertissement de la copie ci-dessus était celui de la proposition ;
l’accord du joueur lève cette attente pour les éléments présentés. Les deux
phrases préparatoires de l’ancien manifeste qui différaient de la réponse
finale ne sont pas assimilées à du texte explicitement validé.

Décision : [PETS-EQ15-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/drone-repositioning-acceptance-decisions.json).
Fiche acceptée : [PETS-EQ15, révision 3](../docs/reviews/refonte-agent-2026-10-06/review-drone-recall-command-PETS-EQ15-03.json).


## PETS-EQ18 Familiers et changements de carte proposés — 9 octobre 2026

**PETS-EQ18 · Familiers — changement de carte**

Je propose ce fonctionnement pour les descentes, les refuges et les portails, y compris le retour d’une carte bonus :

- **Tous vos familiers vivants vous suivent automatiquement**, même s’ils sont éloignés ou hors de vue. Ils arrivent près de vous sur des cases libres et praticables.
- Ils conservent **leurs PV, leur énergie, leur équipement et leurs effets en cours**. La Mule conserve aussi tout son chargement. Le passage ne soigne pas et ne recharge pas leurs réserves.
- Les monstres relevés conservent **leur durée restante**. Les durées d’effets et les recharges continuent normalement : le passage ne les réinitialise pas.
- Les ordres de combat et de ramassage liés à l’ancienne carte sont annulés. Les familiers reprennent votre suivi.
- Si aucune disposition ne permet d’accueillir tous vos familiers, **le passage reste indisponible** et vous restez sur la carte actuelle.

**Aucune nouvelle invocation · Aucun coût d’énergie pour le transfert des familiers.**

Ces modalités restent à valider.

Cette fiche conserve la persistance des drones déjà acceptée et propose de
la préciser pour les descentes, refuges et portails, ainsi que de l’étendre
aux monstres relevés. L’origine peut être éloignée ou hors de vue ; aucun
acteur neuf n’est créé. Les réserves actuelles, objets, effets et recharges
sont conservés, ainsi que la durée restante des relevés. Les tours réels
continuent de s’écouler normalement. Le retour d’une carte bonus utilise
la même règle. Les anciens ordres de cible et de collecte sont annulés.
L’impossibilité de placer tout l’entourage bloque le transfert entier ;
aucun familier ni objet n’est supprimé pour faire de la place.

Ces précisions et extensions sont proposées, sans accord présumé par le
« ok , on continue » du joueur. Elles ne modifient pas Repositionner, les
limites de cinq drones et cinq relevés, ni les contrôles des relevés.
Le rangement manuel par rappel n’est pas réintroduit. Les compteurs de
phase, fractions de Maintenance et autres règles déjà acceptées sont
préservés dans leur périmètre ; l’ordre précis des événements, les zones
d’arrivée, cas d’expiration durant le passage et le rendu restent à revoir.
Aucun code ni rendu natif intégré.

Proposition : [PETS-EQ18, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-familiar-map-transitions-PETS-EQ18-01.json).


## PETS-EQ18 Passage validé avec arrivée progressive des familiers — 9 octobre 2026

Le joueur confirme la fiche, puis excepte explicitement la règle qui bloquait
le passage si l’entourage entier ne pouvait pas entrer. Il demande de faire
passer le joueur et de placer un familier supplémentaire à chaque déplacement
qui libère une place. Il valide ensuite la synthèse suivante par
« c'est ca, ca me semble mieux comme regle :) » :

**Le manque de place pour les familiers ne bloque plus le passage du joueur.**

Les familiers qui peuvent être placés arrivent avec lui. Les autres restent en attente : **à chaque déplacement du joueur, si une place se libère, un familier supplémentaire rejoint la carte**, jusqu’à l’arrivée de tous.

Les autres règles proposées restent validées.

Les autres blocs montrés de PETS-EQ18 restent validés : descentes, refuges,
portails et retour des cartes bonus, identité des acteurs, réserves actuelles,
objets et chargement, effets et recharges conservés, durée restante des relevés,
annulation des ordres de l’ancienne carte et reprise du suivi, aucun nouveau
coût d’invocation ou d’énergie pour le transfert. Le refus du passage faute
de place est rejeté et ne devient jamais une règle acceptée dans les reçus.
La visibilité et les actions des familiers encore sans case, leurs effets
pendant l’attente et le dépôt d’objets lors d’une mort ou expiration sont des
interactions nouvelles à revoir. Aucun code ni rendu natif intégré.

Décision : [PETS-EQ18-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/familiar-map-transition-progressive-arrival-acceptance-decisions.json).
Fiche acceptée : [PETS-EQ18, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-map-transitions-PETS-EQ18-02.json).

## PETS-EQ19 État des familiers en attente proposé — 9 octobre 2026

**PETS-EQ19 · Familiers en attente d’arrivée**

Pour les familiers qui attendent une place, je propose ceci :

- Ils restent dans la fenêtre **Familiers**, avec la mention **« En attente d’arrivée »**.
- Pendant l’attente, ils **ne combattent pas, ne ramassent rien et ne vous apportent ni vision ni protection**.
- Leurs **buffs, débuffs, recharges et durée de résurrection continuent de s’écouler** à chaque tour de jeu. Les dégâts persistants, comme le poison, continuent aussi.
- Ils ne profitent d’aucune **régénération de PV ou d’énergie** pendant l’attente.
- Si un familier meurt ou si sa résurrection expire avant son arrivée, **son équipement et son éventuel chargement tombent sur la case actuelle du joueur**.

Il conserve ainsi son état réel jusqu’à son arrivée, sans soin ni prolongation de sa résurrection.

Ces précisions restent à valider.

L’attente correspond seulement au manque de place lors d’un changement de
carte. Le rangement manuel par rappel n’est pas réintroduit. La progression
des effets et recharges pour un acteur encore sans case est présentée
explicitement ; aucun accord sur ces interactions n’est présumé à partir de
l’arrivée progressive. Le dépôt sur la case du joueur est une exception
proposée uniquement lorsqu’un familier meurt ou expire avant son arrivée.
Les objets sont réels, sans récompense nouvelle ni duplication. Le refus des
soins passifs durant cette attente ne révise pas la restauration distincte
sur un véritable gain de niveau déjà acceptée. L’ordre de la file, les boutons
disponibles, les événements simultanés et les nouvelles transitions alors
que des familiers attendent restent à revoir. Aucun code ni rendu natif intégré.

Proposition : [PETS-EQ19, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-pending-familiar-arrival-PETS-EQ19-01.json).


## PETS-EQ19 Familiers en attente validés — 9 octobre 2026

Le joueur valide la fiche par « parfait :) ». La mention En attente d’arrivée,
l’absence de combat, de collecte, de vision partagée et de protection pendant
l’attente, les durées et recharges qui continuent avec les tours de jeu,
les dégâts persistants et l’absence de régénération sont acceptés. La mort
ou l’expiration d’un familier encore sans case dépose ses objets réels sur
la case actuelle du joueur, sans création de récompense ni duplication.
La restauration lors d’un vrai gain de niveau conserve sa règle distincte.
La file, les commandes disponibles et les événements simultanés restent à
revoir, ainsi que le rendu natif. Aucun code ni rendu natif intégré.

Décision : [PETS-EQ19-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/pending-familiar-arrival-acceptance-decisions.json).
Fiche acceptée : [PETS-EQ19, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-pending-familiar-arrival-PETS-EQ19-02.json).

## PETS-EQ20 Ordres après repositionnement proposés — 9 octobre 2026

**PETS-EQ20 · Repositionner — reprise du suivi**

Je propose qu’après son repositionnement :

- **L’ordre donné au drone est annulé**, puis il reprend votre suivi. Un ordre d’attaque ne le renvoie donc pas vers sa précédente cible.
- Pour la Mule, **le ramassage s’arrête** et son bouton redevient **« Ramasser »**. Vous pouvez relancer la collecte avec un nouvel ordre.
- Les réactions automatiques de combat du drone restent inchangées.

Cela permet d’utiliser le repositionnement pour dégager un drone et interrompre sa poursuite ou sa collecte.

Ces précisions restent à valider.

Cette fiche propose uniquement le devenir des ordres explicites après un
Repositionner réussi. Le drone suit de nouveau le joueur ; la Mule ne reprend
pas sa collecte sans un nouvel ordre. Les réactions automatiques déjà prévues
ne sont pas désactivées, et aucun nouveau mode de combat n’est défini.
Un choix annulé ou sans destination ne change pas l’ordre courant. Les règles
acceptées de déplacement, de coût, de recharge, de réserves et d’objets sont
conservées, ainsi que le ramassage dans le champ de vision actuel du joueur.
L’ordre précis des actions, les effets sur la case d’arrivée et le rendu
restent à revoir. Aucun code ni rendu natif intégré.

Proposition : [PETS-EQ20, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-reposition-drone-orders-PETS-EQ20-01.json).


## PETS-EQ20 Reprise du suivi après repositionnement validée — 9 octobre 2026

Le joueur valide la fiche par « oui parfait ». Après un Repositionner réussi,
l’ordre explicite en cours est annulé et le drone reprend le suivi. La Mule
interrompt sa collecte ; son bouton revient à Ramasser et un nouvel ordre
est nécessaire pour reprendre. Les réactions automatiques de combat restent
inchangées. Le coût, la recharge, les réserves, les objets et les autres règles
déjà acceptées sont conservés. L’ordre précis des actions et le rendu natif
restent à revoir. Aucun code ni rendu natif intégré.

Décision : [PETS-EQ20-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/reposition-drone-order-acceptance-decisions.json).
Fiche acceptée : [PETS-EQ20, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-reposition-drone-orders-PETS-EQ20-02.json).

## PETS-EQ21 Boutons et limites des familiers en attente proposés — 9 octobre 2026

**PETS-EQ21 · Familiers en attente — boutons et limites**

Je propose ces règles dans la fenêtre **Familiers** :

- Vous pouvez toujours **consulter leur fiche, leur équipement et le chargement de la Mule**.
- Les actions qui demandent leur présence sur la carte sont **grisées** : ordres d’attaque, **Repositionner**, **Ramasser**, équiper ou retirer un objet et transferts d’objets. Leur tooltip indique : **« Disponible après l’arrivée du familier. »**
- Le bouton **« Destruire » reste disponible**. Ses objets équipés et son éventuel chargement tombent alors sur la case actuelle du joueur.
- Un **drone en attente réserve toujours la place de son modèle** : vous ne pouvez pas en invoquer un second exemplaire.
- Un **monstre relevé en attente compte toujours parmi les cinq relevés maximum**.
- À leur arrivée, les boutons redeviennent disponibles **selon leurs conditions habituelles**, notamment la portée et la recharge restante.

Ces précisions restent à valider.

Cette fiche propose la consultation sans transfert, la désactivation des
commandes nécessitant une case et le bouton Destruire pendant l’attente.
Le dépôt sur la case du joueur est présenté pour cette destruction volontaire.
Les familiers en attente occupent toujours leurs limites respectives : cinq
drones, un par modèle, et cinq relevés. Aucune limite commune à cinq familiers
n’est introduite. Les relevés demeurent autonomes, sans nouveaux boutons de
combat. L’arrivée conserve les autres conditions de portée, visibilité et
recharge. Le coût d’action de la destruction, la file et les événements
simultanés restent à revoir, ainsi que le rendu natif. Aucun code intégré.

Proposition : [PETS-EQ21, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-waiting-familiar-controls-PETS-EQ21-01.json).


## PETS-EQ21 Boutons validés et limite des relevés à réviser — 9 octobre 2026

Le joueur valide les boutons par « parfait oui :) », puis demande de faire
sauter la limite de cinq relevés au-dessus de dix points dans Réanimation,
pour compenser leur autonomie sans contrôle direct. La consultation, les
boutons gris et leur tooltip, la destruction volontaire en attente avec
objets sur la case du joueur, la place réservée du modèle de drone et les
conditions ordinaires après arrivée sont acceptés. Les relevés en attente
comptent dans la limite applicable, mais cinq n’est plus un plafond fixe
accepté au-dessus de dix. Le plafond de dix rangs appris reste conservé ;
la proposition suivante utilise le rang effectif avec les bonus d’équipement.

Le joueur n’a pas choisi entre un nombre illimité dès le rang onze et une
limite croissante, ni donné de taux précis. La hausse de un par rang reste
une proposition. Les anciens reçus à cinq sont conservés comme historique
avec une révision explicite de leur portée au-dessus de dix. La limite des
drones reste distincte, cinq modèles avec un exemplaire chacun. Aucun code
ni rendu natif intégré.

Décision : [PETS-EQ21-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/waiting-familiar-control-acceptance-and-raised-limit-direction-decisions.json).
Fiche acceptée : [PETS-EQ21, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-waiting-familiar-controls-PETS-EQ21-02.json).

## PETS-EQ22 Croissance du nombre de relevés proposée — 9 octobre 2026

**PETS-EQ22 · Réanimation — nombre de relevés**

Je propose **un relevé supplémentaire par rang effectif au-delà de 10**, bonus d’équipement compris :

- **Rangs 1 à 10 : 5 relevés maximum.**
- **Rang 11 : 6 relevés maximum.**
- **Rang 12 : 7 relevés maximum.**
- **Rang 13 : 8 relevés maximum**, puis la progression continue sans nouveau plafond fixe.

Les relevés **présents sur la carte ou en attente d’arrivée** comptent dans ce maximum. Ils restent autonomes.

Cette progression chiffrée reste à valider.

La formule proposée est cinq plus un par rang effectif au-dessus de dix.
Elle ne modifie pas la puissance, la durée, le coût, la recharge ni les
contrôles des relevés. Les acteurs présents et en attente comptent ensemble.
La perte d’un bonus de rang, le dépassement de capacité qui en résulterait
et les marques déjà posées lors d’un remplacement restent à revoir. Les
valeurs et leur copie restent proposées, sans approbation inférée de la
seule demande de dépasser cinq. Aucun code ni rendu natif intégré.

Proposition : [PETS-EQ22, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-raised-limit-growth-PETS-EQ22-01.json).


## PETS-EQ22 Croissance du nombre de relevés validée — 9 octobre 2026

Le joueur valide par « parfait :) ». Réanimation conserve cinq relevés maximum
jusqu’au rang effectif dix, puis gagne une place par rang supplémentaire :
six au rang onze, sept au rang douze, huit au rang treize, sans nouveau plafond
fixe. Les relevés sur la carte et en attente comptent ensemble. L’autonomie,
les effets et les autres paramètres de Réanimation sont conservés. Les pertes
de bonus et dépassements de capacité restent à revoir. Aucun code intégré.

Décision : [PETS-EQ22-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/raised-limit-growth-acceptance-decisions.json).
Fiche acceptée : [PETS-EQ22, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-raised-limit-growth-PETS-EQ22-02.json).

## GEAR-SK00 Accès aux compétences inconnues par équipement demandé — 9 octobre 2026

Le joueur demande explicitement qu’une compétence concernée par le bonus d’un
objet soit utilisable même s’il ne l’a pas débloquée lui-même. Cette règle
concerne le joueur. Elle remplace l’interprétation qui exigeait un apprentissage
préalable pour bénéficier d’un accès par équipement. Les rangs appris, les
points investis et le rang effectif restent des notions distinctes. La règle
PETS-EQ04 des familiers, qui réserve leurs bonus aux compétences compatibles
déjà possédées, n’est pas élargie automatiquement. Les conséquences détaillées
pour les passifs, pertes de bonus et achats secondaires sont proposées ensuite.
Aucun affixe, code ou rendu natif créé.

Décision directe : [GEAR-SK00](../docs/reviews/refonte-agent-2026-10-06/player-equipment-grants-unlearned-skill-decisions.json).

## GEAR-SK01 Détails des compétences accordées par équipement proposés — 9 octobre 2026

**GEAR-SK01 · Compétences accordées par les objets**

Ta règle est notée. Je propose ces précisions :

- Un objet donnant **+3 à Braise** permet de l’utiliser au **rang 3**, même sans point appris, tant que ce bonus d’équipement est actif.
- Cela s’applique aussi aux **passifs**, avec leurs conditions habituelles.
- Le bonus ne dépense aucun point et ne rend pas l’apprentissage permanent. Si vous perdez le dernier bonus sans avoir appris la compétence, **vous ne pouvez plus la lancer** ; un passif ainsi accordé cesse de fonctionner.
- Les conditions d’utilisation restent applicables : **arme adaptée, énergie et recharge**. Retirer puis remettre l’objet ne réinitialise pas la recharge.
- Pour **acheter les passifs d’un arbre secondaire avec vos points**, l’actif associé doit toujours être appris. L’accès donné par un objet permet de l’utiliser, mais ne remplace pas cet apprentissage.

Ces précisions restent à valider.

La fiche propose le rang effectif calculé depuis zéro rang appris, les passifs
accordés, la perte d’accès aux nouvelles activations en l’absence d’apprentissage,
la conservation de la recharge et le maintien du prérequis d’apprentissage
pour acheter les passifs secondaires. Elle ne rend pas un bonus temporaire
permanent et ne crée pas de point appris gratuit. Les effets déjà lancés,
transformations, marques et familiers après retrait demandent une revue
séparée, de même que les familles exactes d’affixes et leur génération.
L’affichage existant du rang effectif coloré est conservé, sans décomposition
appris plus équipement dans le tooltip. Aucun code ni rendu natif intégré.

Proposition : [GEAR-SK01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-equipment-granted-skills-GEAR-SK01-01.json).


## GEAR-SK01 Compétences accordées par équipement validées — 9 octobre 2026

Le joueur valide la fiche par « oui parfait :) ». Le rang effectif peut partir
de zéro rang appris et les passifs peuvent être accordés par équipement.
L’exemple Braise au rang trois est accepté. Aucun point appris n’est créé ;
le dernier bonus perdu bloque les nouvelles utilisations et supprime le
modificateur passif accordé. Les conditions d’arme, d’énergie et de recharge
restent valables ; retirer et remettre un objet ne réinitialise pas la recharge.
Pour acheter les passifs secondaires avec des points, l’actif associé doit
être réellement appris. Les règles des compétences possédées par les familiers
ne sont pas élargies. Le rendu et les effets déjà lancés restent à revoir.
Aucun code ni rendu natif intégré.

Décision : [GEAR-SK01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/equipment-granted-skill-acceptance-decisions.json).
Fiche acceptée : [GEAR-SK01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-equipment-granted-skills-GEAR-SK01-02.json).

## GEAR-SK02 Effets déjà créés après retrait d’un objet proposés — 9 octobre 2026

**GEAR-SK02 · Retirer l’objet après utilisation**

Je propose ces règles lorsque vous perdez un bonus de compétence :

- Les **buffs, débuffs, zones, mines et transformations déjà déclenchés** conservent leur rang de lancement et poursuivent leur durée ou leur fonctionnement normal.
- **Furtivité et Souffle de feu s’arrêtent** si le retrait de l’objet vous fait perdre tout accès à la compétence.
- Les **familiers déjà créés restent présents**. Leurs bonus d’invocation suivent votre rang actuel ; si vous perdez tout accès au sort, ils utilisent la puissance du **rang 1**, sans vous rendre la compétence utilisable.
- Les **marques de Réanimation déjà posées restent jusqu’à la mort de leur cible**. Elles peuvent encore la relever, avec le rang alors disponible et un minimum de calcul de **1** si vous n’avez plus accès au sort.
- Ces changements **ne soignent pas les familiers**, ne remplissent pas leur énergie et ne réinitialisent ni leur durée restante ni leurs recharges.

Ces précisions restent à valider.

Le maintien du rang d’un effet actif créé est proposé pour les effets ordinaires,
avec des exceptions explicites pour Furtivité et Souffle de feu lors de la perte
complète d’accès. Ce maintien ne valide pas un snapshot général de toutes les
statistiques ou défenses. Les modificateurs passifs accordés par l’objet cessent
selon GEAR-SK01. Les familiers conservent leur identité et leurs vrais objets ;
leurs bonus continuent de suivre le rang actuel, avec un plancher de calcul de
un proposé lorsque l’invocation n’est plus accessible. Ce plancher ne crée pas
un rang appris et ne rend pas le bouton d’invocation utilisable. Les marques
existantes peuvent terminer leur effet à la mort de la cible selon la même
règle de rang disponible. Les cas d’excès de mines ou de relevés après baisse
de rang restent à revoir, de même que les instants précis de calcul, les
modificateurs particuliers et le rendu. Aucun code ni rendu natif intégré.

Proposition : [GEAR-SK02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-equipment-grant-loss-effects-GEAR-SK02-01.json).


## GEAR-SK02 Conservation conditionnée à un apprentissage personnel — 9 octobre 2026

Le joueur dit : « Je suis d'accord avec ça si le joueur a déjà investis des
points de lui meme dans la compétence, s'il ne l'a pas fait alors les bonus
disparaissent tout simplement. » L’accord sur la proposition précédente est
conditionnel : il ne valide pas un maintien artificiel au rang un lorsque
la compétence provient uniquement d’un objet. Les effets ordinaires déjà
créés peuvent rester selon la proposition pour une compétence réellement
apprise ; les familiers de cette compétence suivent le rang actuel et les
marques persistent normalement. Sans rang appris, la disparition des avantages
accordés par l’objet est la direction du joueur. Le détail des effets à
retirer et du sort des familiers et objets est proposé dans la révision deux.
La version un reste un historique de proposition, jamais un accord général.
Aucun code ni rendu natif intégré.

Décision : [GEAR-SK02-DECISION01](../docs/reviews/refonte-agent-2026-10-06/equipment-grant-loss-learned-condition-decisions.json).

## GEAR-SK02 Révision deux proposée — 9 octobre 2026

**GEAR-SK02 · Retrait de l’objet — règle corrigée**

**Vous avez appris la compétence :** les effets déjà lancés continuent selon les règles proposées. Les familiers restent présents, avec une puissance recalculée selon votre rang actuel.

**Vous n’avez aucun rang appris :** si vous perdez le dernier bonus qui vous donnait accès à la compétence, ses effets en cours prennent fin : buffs, débuffs, zones, mines, transformations et marques issus de cette compétence.

**Pour les familiers obtenus uniquement grâce à l’objet**, je propose qu’ils disparaissent aussi lorsque leur invocation n’est plus accessible. Leur équipement et le chargement de la Mule sont déposés **sur leur case**, ou **sur celle du joueur s’ils attendent leur arrivée**.

Il n’y a donc **aucun maintien artificiel au rang 1** pour une compétence jamais apprise.

Cette application aux effets et aux familiers reste à valider.

Cette révision montre le retrait du dernier bonus lorsque zéro rang est
appris. Elle propose d’arrêter uniquement les effets issus de la compétence
concernée, sans purge globale des effets d’autres sources. Les familiers
créés uniquement par cet accès sont retirés, et leurs objets réels sont
rendus au sol sur leur case ou celle du joueur si l’acteur attend. Aucun
remboursement, nouvelle récompense ou explosion n’est créé par ce retrait.
Les dégâts déjà infligés restent acquis. Le maintien au rang un sans point
appris, proposé dans la version un, est retiré. Les pertes partielles de
bonus, effets de plusieurs sources, cas d’excès après baisse d’un rang appris
et l’ordre précis des événements restent à revoir. La copie et l’application
concrète proposées ci-dessus ne sont pas assimilées à une validation complète
par la seule condition du joueur. Aucun code ni rendu natif intégré.

Proposition : [GEAR-SK02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-equipment-grant-loss-effects-GEAR-SK02-02.json).


## GEAR-SK02 Retrait du dernier bonus et familiers validés — 9 octobre 2026

Le joueur répond « parfait, je valide », puis demande ce qu’il reste à définir
avant d’implémenter la refonte. Il accepte la révision deux présentée : les
effets d’une compétence réellement apprise continuent selon les règles retenues,
et la puissance des familiers suit le rang actuel. Sans rang appris, perdre le
dernier bonus accordant la compétence met fin à ses effets en cours, y compris
ses marques. Les familiers ainsi obtenus sont retirés et déposent leurs objets
réels, ainsi que le chargement de la Mule, sur leur case ou celle du joueur
s’ils attendent leur arrivée. Aucun maintien artificiel au rang un.

L’accord ne change pas les durées ou recharges et ne crée pas de remboursement,
de soin, de nouvelle récompense ou d’explosion lors du retrait. Les pertes
partielles avec un autre objet accordant encore la compétence, effets de plusieurs
sources, excès après baisse de rang et ordre précis restent à revoir. Aucun
code ni rendu natif intégré.

Décision : [GEAR-SK02-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/equipment-grant-loss-effects-acceptance-decisions.json).
Fiche acceptée : [GEAR-SK02, révision 3](../docs/reviews/refonte-agent-2026-10-06/review-equipment-grant-loss-effects-GEAR-SK02-03.json).

## IMPLEMENT-READY01 Points restant avant la refonte complète — 9 octobre 2026

Inventaire documentaire demandé par le joueur, sans nouveau canon, chiffre,
texte de jeu ou rendu approuvé. La première revue des fiches retenues est
terminée : zéro fiche de premier passage restante. Cela ne certifie pas les
courbes complètes, toutes les interactions ou les pixels natifs. Le catalogue
principal actuel comporte 79 candidats dans 14 branches ; Réparation ciblée
possède une fiche acceptée à raccorder au catalogue final.

Les fondations déjà approuvées peuvent être développées par lots. Pour intégrer
les systèmes complets concernés, il reste à consolider les courbes aux rangs
non montrés, rapprocher huit anciens reçus partiels des accords exacts sans
redemander des validations existantes, et trancher les effets cumulés, leurs
instants de fin et les baisses de rang. Les propriétés de loot donnant des
compétences, modèles réels d’armes, paramètres natifs de drones et parties
non acceptées des cinq départs de classe doivent aussi être finalisés pour
les lots qui les utilisent.

Le nouveau bestiaire, ses pouvoirs et sa densité, puis les quêtes, textes,
services et cartes de la base, des refuges et des portails restent à développer
pour la campagne complète. Leur catalogue entier ne bloque pas le premier socle
technique. Les interfaces et animations seront examinées sur prototype selon
les directions déjà acceptées. L’équilibrage progressif demeure différé et le
menu de configuration du personnage de test reste prévu après toutes les
compétences et animations, conformément au choix du joueur.

Cet inventaire distingue les fondations, chaque lot de contenu et la campagne
complète. Il ne réintroduit pas les paliers retirés, les compétences supprimées,
les anciens plafonds ou les mécanismes de drones remplacés. Aucun code ni rendu
natif intégré et aucun nouvel effet soumis ou validé par ce bilan.

Inventaire : [IMPLEMENT-READY01](../docs/reviews/refonte-agent-2026-10-06/review-implementation-readiness-01.json).


## INTERACT01 Statuts, zones et baisse de rang — 9 octobre 2026

Le joueur précise les règles en réponse à l’inventaire de préparation :
réappliquer un même statut remet son décompte depuis le début ; les effets
de zones superposées se cumulent. En cas de baisse de rang d’une compétence,
tous les éléments invoqués par celle-ci sont supprimés et leurs objets réels
sont déposés au sol. Il confirme explicitement la portée : « uniquement les
éléments invoqués par cette compétence ». Les invocations des autres
compétences restent présentes.

Cette suppression concerne une baisse de rang, même si la compétence reste
apprise ou accessible, et remplace le maintien des familiers à un rang inférieur.
Les dépôts d’objets conservent la règle de la case du familier, ou de celle du
joueur si le familier attend son arrivée. Les anciens reçus sont conservés en
historique et les nouvelles règles sont inscrites comme révisions actives.
Le traitement de la puissance lors d’une nouvelle application de statut,
les effets de plusieurs sources, l’ordre des tours, les créations non acteurs
et les bonus indirects de capacité entre compétences restent à préciser.
Aucun code ni rendu natif intégré.

## IMPLEMENT-READY01 Priorités précisées par le joueur — 9 octobre 2026

Compléter les progressions et les interactions, puis finaliser les départs
et une première mission jouable pour avancer par lots. Les interfaces et
animations sont prioritaires et doivent être conçues ensemble avec le joueur.
Le loot doit prendre Diablo 2 comme référence pour le nombre d’effets, les
types de bonus, les catégories et couleurs d’objets, avec des adaptations à
l’esthétique cyberpunk démoniaque. Cette direction ne valide encore aucun
nouveau barème de rareté, poids de tirage, famille ou affixe précis.

Le bestiaire de monstres, mini-boss et boss reste incomplet et doit être
construit progressivement. La campagne détaillée peut attendre. Le menu de
test garde son échéance précédemment choisie après compétences et animations.
Aucune valeur de progression ni aucun rendu nouveau n’est validé par ce bilan.

Décisions : [INTERACT01](../docs/reviews/refonte-agent-2026-10-06/interaction-and-priority-user-decisions.json).
Inventaire révisé : [IMPLEMENT-READY01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-implementation-readiness-02.json).


## B01-PROG01 Progression complète proposée — 9 octobre 2026

**B01-PROG01 · Frappe de rupture — progression proposée**

Les repères déjà validés sont conservés. Voici le barème complet proposé, avec une continuation au-delà de 10.

**Bonus de dégâts — rangs 1 à 10, dans l’ordre**

**+40 % · +50 % · +60 % · +70 % · +80 % · +90 % · +100 % · +110 % · +120 % · +130 %**

**Coût en énergie**

- Rangs 1–3 : **6**.
- Rangs 4–6 : **7**.
- Rangs 7–9 : **8**.
- Rangs 10–12 : **9**.

**Au-delà de 10, avec les bonus d’équipement**

- Rang 11 : **+140 %**, **9 énergie**.
- Rang 12 : **+150 %**, **9 énergie**.
- Rang 13 : **+160 %**, **10 énergie**.
- Rang 14 : **+170 %**, **10 énergie**.
- Rang 15 : **+180 %**, **10 énergie**.
- Rang 20 : **+230 %**, **12 énergie**.
- Rang 30 : **+330 %**, **15 énergie**.

Les mêmes écarts se poursuivent au-delà des rangs affichés. La recharge reste de **3 tours** et la compétence frappe **une cible à portée de l’arme**.

**Exemple de tooltip**

Arme fictive à **18–22 dégâts**, Force **10**, sans autre bonus.

*Vos implants libèrent toute leur puissance dans un coup brutal.*

**Effet actuel — rang 11**

- Bonus de dégâts : **+140 %**.
- Dégâts : **44–53**, avant les défenses et hors critique.
- Énergie : **9**.
- Recharge : **3 tours**.

**Prochain rang — 12**

- Bonus de dégâts : **+150 %**.
- Dégâts : **45–56**.

**Nécessite une arme de mêlée active.**

Cette progression et cet exemple actualisé restent à valider.

La proposition conserve les bonus déjà acceptés aux rangs un, dix et douze.
Les rangs intermédiaires, les coûts et la continuation sont présentés en
blocs numériques sans tableau. L’exemple utilise une arme fictive à dix-huit
à vingt-deux dégâts et Force dix ; les règles actuelles donnent cinq points
de bonus de Force, additionnés au bonus de la technique. Les anciens exemples
de multiplication avec Agilité sont conservés en historique et ne sont pas
réutilisés. Le prochain achat conserve l’arme, les attributs et les autres
modificateurs ; le rang appris passe de neuf à dix avec deux points de bonus
d’équipement, sans afficher cette décomposition dans le tooltip. Le plafond
appris de dix reste distinct de la continuation du rang effectif. Les calculs
vérifient les repères et les dégâts, sans constituer une validation du moteur
ou un équilibrage final. Aucun code de jeu ni rendu natif intégré.

Proposition : [B01-PROG01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-skill-progression-B01-PROG01-01.json).

### SKILL-RANK-DELEGATE01 — Progressions provisoires confiées à l'agent

Le joueur confie la définition des rangs : « je te laisse t'occuper des rangs
de compétences de toi meme pour l'instant mais le gain doit être perceptible
au fil de la progression. :) De toutes facon on équilibrera plus tard ».

Cette délégation porte sur les courbes chiffrées provisoires. Elle ne valide
pas de nouveaux textes RP, compétences, effets, liens avec les attributs ou
visuels. Les anciens reçus restent conservés ; la confirmation séparée des
rangs de B01 n'est plus un préalable à cette passe.

Les courbes couvrent **79 compétences principales**, **14 secondaires** et
le complément de réparation **DR01** déjà accepté, soit **94 compétences**.
La place définitive de DR01 dans l'arbre reste à fixer. Les actifs secondaires
restent achetables une seule fois ; leurs passifs exigent l'actif réellement
appris. Le plafond appris de dix et les bonus d'objets au-delà sont conservés.

Les dégâts, soins et bonus utilisent des gains directs ; les nombres de
familiers et les dimensions en cases progressent par paliers. Les coûts
positifs, chances et protections gardent leurs bornes. Les cinq mines au
rang dix, leur progression au-delà, le coût furtif d'une énergie au rang dix
et les limites révisées des relevés sont vérifiés.

Courbes : [SKILL-RANKS01](../docs/reviews/refonte-agent-2026-10-06/skill-rank-curves-provisional-01.json).
Contrôles : [SKILL-RANKS01-CHECK01](../docs/reviews/refonte-agent-2026-10-06/skill-rank-curves-validation-01.json).
Autorisation : [SKILL-RANK-DELEGATE01](../docs/reviews/refonte-agent-2026-10-06/skill-rank-balancing-delegation-decisions.json).

### SKILL-LEVEL01 — Puissance liée au niveau actuel, accord du joueur

Le joueur accepte de conserver l'apprentissage libre et d'ajouter une base
évolutive aux pouvoirs sans arme. Le niveau **actuel** du lanceur détermine
cette base ; le rang effectif, les attributs et l'équipement apportent leurs
contributions respectives. Le niveau auquel la compétence a été apprise
n'entre jamais dans le calcul.

Deux personnages de niveau quarante avec Braise au rang dix, les mêmes
attributs, le même équipement et les mêmes autres conditions obtiennent
exactement le même résultat. Cela reste vrai si l'un a appris Braise au niveau
deux et l'autre au niveau quarante. Le joueur confirme cette clarification :
« d'accord :) ca me semble bien ».

Les attaques d'arme utilisent leurs dégâts d'arme comme base. Les drones
conservent leurs statistiques d'acteur, les relevés celles de leur origine.
Les dégâts dérivés d'un impact réel, comme les collisions, le saignement ou
le renvoi, utilisent cet impact sans recevoir un second facteur de niveau.
Portées, durées, chances de statut, coûts et recharges ne sont pas augmentés
automatiquement par cette nouvelle règle de dégâts.

Décision : [SKILL-LEVEL01](../docs/reviews/refonte-agent-2026-10-06/current-level-skill-power-acceptance-decisions.json).

### SKILL-LEVEL01-NUM01 — Référence numérique proposée pour la première passe

Le principe précédent est accepté. Le coefficient exact ci-dessous est une
**proposition provisoire**, à distinguer de cet accord sur le principe.

La référence ajoute **10 % de la base du premier niveau par niveau gagné**.
La croissance est linéaire. Elle s'applique une fois aux composantes de dégâts
bruts concernées, avant les modificateurs compatibles et l'arrondi final.
Les bonus fixes d'objet ne sont pas multipliés par ce facteur de niveau.

**Braise — profil neutre, sans bonus, hors critique et avant défenses**

- Niveau 1, rang 1 : **8 dégâts de feu**.
- Niveau 1, rang 10 : **35 dégâts de feu**.
- Niveau 40, rang 1 : **39 dégâts de feu**.
- Niveau 40, rang 10 : **171 dégâts de feu**.

Ces valeurs illustrent la courbe provisoire ; elles ne sont ni les valeurs
définitives du jeu ni un équilibrage contre le futur bestiaire. La croissance
des PV des barrières et des soins fixes, ainsi que l'actualisation des effets
persistants déjà lancés lors d'un changement de niveau, restent à préciser.

Le prototype conserve les calculs exacts jusqu'à l'arrondi applicable. Les
dégâts destinés au joueur restent entiers. Le prochain rang conserve le niveau
actuel du personnage, l'équipement, les attributs et les autres modificateurs.

Référence : [SKILL-LEVEL01-NUM01](../docs/reviews/refonte-agent-2026-10-06/review-current-level-skill-power-01.json).

Les manifestes courants sont le modèle de statistiques **61**, le catalogue
principal **233**, le catalogue complet **208**, le compteur de revue **102**
et l'inventaire de préparation **03**. La première revue individuelle reste
terminée. Les interactions restantes, les départs de classe, le contenu du
loot et la revue native des interfaces et animations demeurent distincts.
Les contrôles portent sur la conception et l'arithmétique ; aucun code de jeu
ni rendu natif n'est intégré par cette passe.

### SKILL-LEVEL01-NUM01 — Courbe initiale acceptée

Après l'exemple au niveau quarante, le joueur demande les dégâts de Braise
au niveau onze. La réponse présente **16 dégâts de feu au rang un** et
**70 au rang dix**, sans bonus d'attributs ou d'équipement, hors critique et
avant les défenses de la cible. Le joueur répond : « d'accord, on continue ».

La première courbe ajoute donc **10 % des dégâts de base du premier niveau
par niveau gagné**, avec une croissance linéaire. Le coefficient initial et
les exemples montrés sont acceptés pour cette première version. L'équilibrage
contre les statistiques réelles des monstres reste prévu plus tard.

Les valeurs ne dépendent pas du niveau d'apprentissage. Le plafond appris
de dix, les rangs supplémentaires d'équipement et les contributions distinctes
des attributs restent ceux déjà retenus.

Décision : [SKILL-LEVEL01-NUM01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/current-level-skill-power-numeric-acceptance-decisions.json).
Profil courant : [SKILL-LEVEL01-NUM01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-current-level-skill-power-02.json).
Courbes courantes : [SKILL-RANKS01, révision 2](../docs/reviews/refonte-agent-2026-10-06/skill-rank-curves-provisional-02.json).

### INTERACT-STATUS02 — Le dernier lancement fixe la puissance

Le joueur précise : « alors ici c'est simple, quand un même statu est
réappliqué avec une puissance differente, c'est le dernier lancement qui
compte. »

Lorsqu'un même statut est réappliqué, sa nouvelle puissance remplace
l'ancienne, **même si elle est plus faible**. On ne conserve pas automatiquement
la valeur la plus forte et on n'additionne pas les puissances des réapplications.
La durée repart du début, conformément à la règle déjà acceptée.

Les conditions d'application et les résistances restent applicables. La règle
déjà acceptée du cumul des effets de zones superposées reste valable ; l'ordre
des sources simultanées et leurs réapplications reste à préciser séparément.

Décision : [INTERACT-STATUS02](../docs/reviews/refonte-agent-2026-10-06/last-status-cast-power-acceptance-decisions.json).

### INTERACT-POWER01 — Proposition : puissance des effets déjà actifs

Pour les statuts et zones de sorts déjà actifs, je propose de conserver
la puissance calculée au lancement. Gagner un niveau ou changer d'attributs
ou d'équipement modifierait les prochains lancements, sans changer cette
puissance déjà appliquée. Une nouvelle application remplacerait les valeurs
précédentes selon INTERACT-STATUS02 et ferait repartir la durée.

Les résistances et les autres défenses de la cible restent évaluées à chaque
impact ou dégât périodique auquel elles s'appliquent.

**Exemple fictif**

- Puissance d'une Brûlure déjà active : **12 dégâts par tour**, avant résistances.
- Le lanceur gagne un niveau : cette Brûlure conserve cette puissance de **12**.
- Nouvelle application réussie, calculée à **18 dégâts par tour** : elle remplace
  l'ancienne puissance et fait repartir la durée.

Les familiers conservent leurs règles de statistiques déjà acceptées. Les
suppressions d'effets ou d'éléments invoqués prévues par les règles de perte
d'accès ou de baisse de rang restent applicables. Cette proposition ne décide
pas encore de l'ordre des sources simultanées ni du traitement des marques
qui feront naître un relevé plus tard.

**Cette règle de conservation de puissance est proposée, en attente de l'accord
du joueur.** Aucun nouveau texte RP ni rendu visuel n'est présenté ici.

Proposition : [INTERACT-POWER01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-persistent-power-snapshot-INTERACT-POWER01-01.json).

Manifestes courants : statistiques **62**, catalogue principal **234**,
catalogue complet **209**, compteur de revue **103**, préparation **04**.
Les courbes de rangs n'ont pas été changées par cet accord sur le coefficient
de niveau. Les contrôles vérifient les exemples et les références de revue ;
ils ne constituent pas un test du moteur. Aucun code de jeu ni rendu natif
n'est intégré dans cette passe.

### INTERACT-POWER01 — Conservation de puissance acceptée

Le joueur répond « oui :) » à la proposition présentée pour les statuts et
zones déjà actifs. Leur puissance est conservée depuis le lancement. Les
gains de niveau et changements d'attributs ou d'équipement affectent les
prochains lancements. Une nouvelle application utilise sa nouvelle puissance
et fait repartir la durée, conformément à INTERACT-STATUS02.

Les défenses de la cible restent évaluées pour chaque dégât auquel elles
s'appliquent. Les familiers conservent leurs règles d'évolution déjà validées.
Les règles de suppression des effets ou invocations restent applicables dans
leurs cas déjà définis.

L'exemple reste fictif : une Brûlure à douze dégâts par tour avant résistances
reste à douze après un gain de niveau, puis passe à dix-huit lorsqu'une
nouvelle application réussie apporte cette puissance. Cette validation ne
fixe pas de nouvelles valeurs d'équilibrage pour Braise.

Décision : [INTERACT-POWER01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/persistent-effect-power-acceptance-decisions.json).
Fiche courante : [INTERACT-POWER01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-persistent-power-snapshot-INTERACT-POWER01-02.json).

### INTERACT-ZONE02 — Proposition : zones superposées

Je propose les précisions suivantes pour plusieurs zones touchant une cible
pendant un même tour :

- Chaque zone inflige ses propres dégâts ; toutes les contributions s'appliquent.
- Chaque zone fait son propre jet de statut, avec son propre compteur de tours
  consécutifs d'exposition. Sortir d'une zone remet à zéro son compteur.
- Un même statut reste une seule instance. Ses réapplications remplacent sa
  puissance et font repartir sa durée, selon la règle du dernier lancement.
- Les applications de statuts des zones sont traitées de la plus ancienne
  à la plus récente. Si plusieurs réussissent pendant le tour, la dernière
  réussie fixe la puissance finale. Un jet raté ne remplace pas le statut.

Les dégâts et les chances restent soumis aux protections et règles de contrôle
déjà retenues. Gel et Brûlure continuent à se supprimer mutuellement quand
l'effet opposé est appliqué avec succès.

**Exemple fictif, avant défenses**

- Zone ancienne : **20 dégâts**, puis Brûlure de **8 dégâts par tour**.
- Zone plus récente : **30 dégâts**, puis Brûlure de **5 dégâts par tour**.
- Dégâts des zones : **50** au total.
- Si les deux applications réussissent pendant ce tour : une seule Brûlure
  à **5 dégâts par tour**, avec une durée redémarrée.

Ces chiffres illustrent l'interaction ; ils ne sont pas les dégâts d'une
compétence réelle. L'ordre complet des tours, la provenance entre plusieurs
lanceurs et le cas où une zone ancienne réapplique seule un statut plus récent
restent à préciser séparément.

**Les jets indépendants, la fusion du statut et cet ordre de traitement sont
proposés, en attente de l'accord du joueur.** La validation précédente du cumul
des effets de zones est conservée.

Proposition : [INTERACT-ZONE02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-overlapping-zone-status-INTERACT-ZONE02-01.json).

Manifestes courants : statistiques **63**, catalogue principal **235**,
catalogue complet **210**, compteur de revue **104**, préparation **05**.
Les catalogues, courbes de rangs et autres décisions acceptées sont conservés.
Cette passe enregistre les accords et prépare une proposition d'interaction ;
aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-ZONE02 — Zones superposées, accord du joueur

Le joueur répond « parfait » à la proposition : les dégâts propres à chaque
zone se cumulent, chaque zone tente son statut avec son propre compteur
d'exposition, un même statut reste unique et les réapplications sont traitées
de la zone la plus ancienne à la plus récente pendant un même tour. La dernière
application réussie fixe la puissance et redémarre la durée. Un échec ne
remplace pas le statut existant.

L'exemple accepté reste fictif : vingt et trente dégâts donnent cinquante
avant les défenses ; si les deux Brûlures réussissent pendant le tour, l'ancienne
puissance de huit est remplacée par celle de cinq de la zone plus récente.
Les règles de résistances, contrôle, conservation de puissance au lancement
et remplacement réciproque de Gel et Brûlure restent applicables.

Décision : [INTERACT-ZONE02-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/overlapping-zone-status-acceptance-decisions.json).
Fiche courante : [INTERACT-ZONE02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-overlapping-zone-status-INTERACT-ZONE02-02.json).

### INTERACT-ZONE03 — Proposition : une ancienne zone réapplique seule

Je propose que la dernière réapplication réussie fixe les nouvelles valeurs,
même lorsqu'elle provient d'une zone créée plus tôt que le sort ayant posé
le statut précédent. La zone utilise la puissance qu'elle conserve depuis
son propre lancement.

**Exemple fictif, avant résistances**

- Une ancienne zone peut appliquer une Brûlure de **8 dégâts par tour**.
- Un sort lancé plus tard applique une Brûlure de **18 dégâts par tour**.
- Au tour suivant, seule l'ancienne zone réapplique la Brûlure avec succès :
  elle passe à **8 dégâts par tour** et sa durée redémarre.
- Si ce jet échoue, la Brûlure précédente conserve sa puissance de **18** et
  sa durée restante continue normalement.

L'ordre de création sert à traiter plusieurs zones pendant le même tour.
Cette proposition ne donnerait pas une priorité permanente au sort le plus
récent contre de futures réapplications réussies. Les dégâts propres aux
zones et les défenses de la cible suivent leurs règles déjà acceptées.

**Ce cas est proposé, en attente de l'accord du joueur.** Il précise une
réapplication après un sort plus récent, hors du seul exemple simultané
déjà validé. Les chiffres ne sont pas des valeurs de compétence réelle.

Proposition : [INTERACT-ZONE03, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-old-zone-status-refresh-INTERACT-ZONE03-01.json).

Manifestes courants : statistiques **64**, catalogue principal **236**,
catalogue complet **211**, compteur de revue **105**, préparation **06**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les exemples ;
aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-ZONE03 — Réapplication par une ancienne zone acceptée

Après l'explication simplifiée, le joueur répond « d'accord ca me va . ».
Une ancienne zone peut donc remplacer un statut plus puissant si elle le
réapplique avec succès. Elle emploie la puissance conservée depuis son
lancement et redémarre la durée du statut. Si son jet échoue, le statut
existant garde sa puissance et sa durée restante continue normalement.

L'exemple accepté reste fictif : une Brûlure de dix-huit dégâts par tour
devient huit si l'ancienne zone réussit une nouvelle application. Ces valeurs
ne sont pas celles d'une compétence réelle. L'ordre ancien-vers-récent entre
zones pendant un même tour reste valable, sans priorité permanente du sort
le plus récemment lancé sur les applications des tours suivants.

Décision : [INTERACT-ZONE03-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/old-zone-status-refresh-acceptance-decisions.json).
Fiche courante : [INTERACT-ZONE03, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-old-zone-status-refresh-INTERACT-ZONE03-02.json).

### INTERACT-RANK02 — Proposition : périmètre des créations invoquées

Pour appliquer la règle déjà acceptée de suppression lors d'une baisse du
rang effectif, je propose de compter comme créations invoquées :

- Les drones et les monstres relevés.
- Les mines posées et les zones de sorts présentes sur la carte.
- Les segments de Mur de glace et du cercle rituel sanglant.
- Les leurres persistants présents sur la carte.

Seules les créations du joueur provenant de la compétence active dont le
rang baisse seraient supprimées, même si cette compétence reste apprise.
Les créations des autres compétences resteraient présentes. Les objets
réellement équipés ou transportés par un familier supprimé tomberaient au
sol selon les règles déjà acceptées.

Les buffs, débuffs et transformations conserveraient leurs règles de durée,
de puissance au lancement et de perte d'accès ; ils ne seraient pas
reclassés comme créations sur la carte. Les statuts déjà appliqués par une
zone supprimée suivraient également ces règles existantes.

Un passif qui améliore une invocation ne serait pas sa compétence créatrice.
Ainsi, perdre un bonus de rang de **Mur de glace** supprimerait les murs
créés par ce sort ; perdre un rang de **Blindage amélioré** conserverait les
drones et modifierait leur protection selon leurs règles de statistiques.

**Cette classification est une proposition en attente de validation.**
Les passifs de capacité ou de variante, les marques précédant une
résurrection, les actions différées et l'ordre exact de suppression restent
à traiter séparément. Aucun nouveau texte RP, dessin ou animation n'est
présenté dans cette passe.

Proposition : [INTERACT-RANK02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-invoked-creation-scope-INTERACT-RANK02-01.json).

Manifestes courants : statistiques **65**, catalogue principal **237**,
catalogue complet **212**, compteur de revue **106**, préparation **07**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les règles
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-RANK02 — Périmètre des créations invoquées accepté

Le joueur répond « oui ca me va :) » à la classification proposée. Les
drones, monstres relevés, mines, zones, murs de glace, segments du cercle
rituel et leurres persistants entrent dans le périmètre de suppression lors
d'une baisse du rang effectif de leur compétence créatrice.

Seules les créations de cette compétence et de ce joueur disparaissent,
même si la compétence reste apprise. Les objets portés par les familiers
supprimés tombent au sol selon les règles déjà retenues. Les passifs qui
améliorent une création ne sont pas ses créateurs : une baisse de Blindage
amélioré diminue la protection des drones sans les supprimer.

Les buffs, débuffs et transformations conservent leurs règles existantes
de durée, de puissance au lancement et de perte d'accès. Les passifs de
capacité ou de variante, les marques précédant une résurrection et l'ordre
exact des événements restent à préciser séparément.

Décision : [INTERACT-RANK02-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/invoked-creation-scope-acceptance-decisions.json).
Fiche courante : [INTERACT-RANK02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-invoked-creation-scope-INTERACT-RANK02-02.json).

### INTERACT-RANK03 — Proposition : baisse de capacité de Réseau de mines

Je propose qu'une baisse du rang de **Réseau de mines**, sans baisse du rang
de **Pose de mine**, applique immédiatement la nouvelle capacité. Seules
les mines en trop disparaîtraient, en commençant par les plus anciennes,
sans explosion ni remboursement d'énergie. Les mines en attente
d'activation compteraient également dans la limite.

**Exemple avec la courbe de rangs provisoire existante**

- Neuf rangs appris et un rang d'équipement : rang effectif dix, cinq mines.
- Retrait du bonus : rang effectif neuf, quatre mines.
- Si cinq mines sont présentes, la plus ancienne disparaît ; les quatre
  autres restent présentes.

Si **Pose de mine** perd elle-même un rang, la suppression de toutes ses
créations déjà acceptée reste applicable. Le traitement du seul passif
de capacité ne constitue pas une exception à cette règle.

**La baisse de capacité, le retrait des mines en trop et le comptage des
mines non encore activées sont proposés, en attente de validation.**
Les variantes élémentaires, les nuages restant après une explosion, les
marques de résurrection et l'ordre des transactions restent hors de cette
proposition. Les limites utilisées dans l'exemple proviennent de la courbe
déjà préparée sous la délégation numérique du joueur, sans nouvel équilibrage.

Proposition : [INTERACT-RANK03, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-mine-passive-capacity-loss-INTERACT-RANK03-01.json).

Manifestes courants : statistiques **66**, catalogue principal **238**,
catalogue complet **213**, compteur de revue **107**, préparation **08**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et l'exemple de
capacité ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-RANK03 — Baisse de capacité de Réseau de mines acceptée

Le joueur répond « oui :) » à la proposition. Une baisse du rang du passif
applique immédiatement la nouvelle limite : seules les mines en trop sont
supprimées, des plus anciennes aux plus récentes, sans explosion ni
remboursement d'énergie. Les mines attendant leur activation comptent aussi
dans la limite. Une baisse du rang de Pose de mine conserve sa règle de
suppression de toutes les créations de cette compétence.

L'exemple accepté utilise la courbe provisoire existante : neuf rangs appris
et un bonus d'équipement donnent une capacité de cinq au rang effectif dix.
Retirer ce bonus ramène la capacité à quatre au rang neuf ; avec cinq mines
présentes, la plus ancienne disparaît et les quatre autres restent.

Décision : [INTERACT-RANK03-RANK04-DECISIONS](../docs/reviews/refonte-agent-2026-10-06/mine-capacity-and-variant-loss-acceptance-decisions.json).
Fiche courante : [INTERACT-RANK03, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mine-passive-capacity-loss-INTERACT-RANK03-02.json).

### INTERACT-RANK04 — Suppression des mines spéciales, instruction du joueur

Après l'annonce du cas de perte du passif permettant le poison, le froid
ou un effet persistant, le joueur précise :

> si ces mines etaient des mines de poison , de froid ou un effet persistant,
> alors la mine disparait, elle n'est pas remplacée par une mine "de base"

Cette instruction est enregistrée directement : si le passif autorisant le
type ou le résidu devient indisponible, les mines du joueur qui dépendent
de ce passif sont supprimées. Il n'y a pas de conversion en mine de feu.
Perdre Mines de poison concerne les mines de poison ; perdre Mines de glace
concerne les mines de froid ; perdre Résidu persistant concerne les mines
configurées pour laisser une zone, quel que soit leur élément.

Pose de mine reste leur compétence créatrice. Cette dépendance au passif
est distincte de l'identité du créateur et de la limite de Réseau de mines.
La baisse partielle de rang avec un passif encore disponible, les zones
déjà créées après une explosion et la perte du passif de rayon ne sont pas
déduites de cette seule instruction.

Décision : [INTERACT-RANK03-RANK04-DECISIONS](../docs/reviews/refonte-agent-2026-10-06/mine-capacity-and-variant-loss-acceptance-decisions.json).
Fiche courante : [INTERACT-RANK04, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-mine-variant-access-loss-INTERACT-RANK04-01.json).

### INTERACT-RANK05 — Proposition : zones après l'explosion d'une mine

Je propose d'étendre la suppression aux zones déjà laissées par les mines
du joueur lorsque leur passif nécessaire devient indisponible :

- Mines de poison : suppression des nuages de poison concernés.
- Mines de glace : suppression des zones de glace concernées.
- Résidu persistant : suppression de toutes les zones issues de ses mines.

Les statuts déjà appliqués par l'explosion, comme Empoisonnement ou Gel,
conserveraient leur durée restante normale et leurs règles existantes de
réapplication ou de suppression. Le Gel maintenu uniquement par une zone
supprimée prendrait fin avec celle-ci, conformément à la durée conditionnelle
de cet effet. Un ancien statut déjà remplacé ne serait pas restauré.

La suppression ne provoquerait aucune nouvelle explosion ni remboursement
d'énergie. Les zones provenant d'autres sorts ou d'autres lanceurs ne
seraient pas concernées.

**Cette application aux zones déjà créées et aux statuts est proposée, en
attente de validation.** La baisse partielle de rang des passifs encore
disponibles, le rayon, les sources de Gel superposées, les marques de
résurrection et l'ordre complet des événements restent à préciser séparément.
Aucun nouveau texte RP, nom de compétence ou rendu natif n'est créé.

Proposition : [INTERACT-RANK05, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-mine-residue-access-loss-INTERACT-RANK05-01.json).

Manifestes courants : statistiques **67**, catalogue principal **239**,
catalogue complet **214**, compteur de revue **108**, préparation **09**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-RANK05 — Zones après explosion, accord du joueur

Le joueur répond « parfait :) on continue » à la proposition. Perdre le
passif nécessaire supprime les zones concernées laissées par ses mines :
les nuages de poison pour Mines de poison, les zones de glace pour Mines de
glace et toutes les zones issues de ses mines pour Résidu persistant.

Cette suppression ne produit pas de nouvelle explosion ni remboursement
d'énergie. Les statuts Empoisonnement ou Gel appliqués par l'explosion
gardent leur durée restante et leurs règles normales ; le Gel maintenu
uniquement par une zone supprimée prend fin avec celle-ci. Un ancien statut
déjà remplacé n'est pas restauré. Les zones des autres sorts ou lanceurs
ne sont pas supprimées par cette perte de passif.

L'accord concerne la perte d'accès au passif. La baisse partielle de rang,
le passif de rayon et les sources de Gel superposées restent distincts.

Décision : [INTERACT-RANK05-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/mine-residue-access-loss-acceptance-decisions.json).
Fiche courante : [INTERACT-RANK05, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mine-residue-access-loss-INTERACT-RANK05-02.json).

### INTERACT-RANK06 — Proposition : baisse partielle des passifs de mines

Je propose d'étendre la suppression des créations concernées à toute baisse
du rang effectif de ces passifs, même lorsqu'ils restent appris ou accessibles
par l'équipement :

- Mines de poison : les mines et nuages de poison utilisant ce passif.
- Mines de glace : les mines et zones de glace utilisant ce passif.
- Résidu persistant : les mines configurées pour laisser une zone et les
  zones déjà créées avec ce passif.
- Explosion étendue : les mines et zones utilisant son rayon amélioré.

La règle proposée dépend de la baisse du rang effectif, sans exiger une
perte complète d'accès ni un changement de valeur à un palier numérique.
Les mines ou zones qui n'utilisent pas le passif concerné restent présentes,
sous réserve de leurs autres règles de suppression.

**Exemple de rang, sans nouvel équilibrage**

Mines de poison passe du rang douze au rang dix et reste disponible : les
mines et nuages de poison existants du joueur disparaîtraient. Ses mines
de feu ou de glace ne seraient pas concernées par cette seule baisse.

Il n'y aurait ni explosion, ni nouveau dégât, ni remboursement d'énergie,
ni remplacement par une mine de base. Les statuts déjà appliqués suivent
les règles acceptées de durée et de fin du Gel entretenu par une zone.

Réseau de mines conserverait son exception validée : sa nouvelle capacité
s'applique et seules les mines en trop sont supprimées, des plus anciennes
aux plus récentes. Pose de mine reste la compétence créatrice ; une baisse
de son propre rang supprime toujours toutes ses créations.

**La baisse partielle de rang et l'application à Explosion étendue restent
une proposition en attente de validation.** Les augmentations de rang et
le calcul de la puissance à la pose, les sources de Gel fusionnées, les
marques de résurrection et l'ordre exact des événements sont hors de cette
passe. Aucun nouveau texte RP, nom de compétence ou rendu natif n'est créé.

Proposition : [INTERACT-RANK06, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-mine-passive-rank-dependencies-INTERACT-RANK06-01.json).

Manifestes courants : statistiques **68**, catalogue principal **240**,
catalogue complet **215**, compteur de revue **109**, préparation **10**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-RANK06 — Baisse du rang des passifs de mines acceptée

Le joueur répond « oui :) » à la proposition. Toute baisse du rang effectif
de Mines de poison, Mines de glace, Résidu persistant ou Explosion étendue
supprime les créations du joueur qui utilisaient le passif concerné, même
s'il reste appris ou disponible par l'équipement. Le changement d'une valeur
à un palier numérique n'est pas nécessaire pour appliquer cette règle.

La suppression concerne respectivement les mines et nuages de poison, les
mines et zones de glace, les mines configurées avec un résidu et leurs zones,
ou les mines et zones utilisant le rayon amélioré. Elle ne produit aucune
explosion, nouveau dégât ou restitution d'énergie, ni conversion en mine de
base. Les créations sans cette dépendance et celles des autres lanceurs
restent soumises à leurs règles existantes.

Les statuts appliqués par l'explosion gardent leur durée ; le Gel entretenu
uniquement par une zone supprimée prend fin. Réseau de mines conserve la
suppression des seules mines en trop selon sa nouvelle capacité. Une baisse
du rang de Pose de mine supprime toujours toutes ses propres créations.

Décision : [INTERACT-RANK06-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/mine-passive-rank-dependencies-acceptance-decisions.json).
Fiche courante : [INTERACT-RANK06, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mine-passive-rank-dependencies-INTERACT-RANK06-02.json).

### INTERACT-MINE01 — Proposition : valeurs fixées au moment de la pose

Je propose qu'une mine conserve les valeurs calculées lors de sa pose
réussie : son élément choisi, sa puissance de dégâts, ses chances de statut
avant les défenses des cibles, son rayon et les effets prévus. Les paramètres
de sa future zone persistante, dont sa puissance et sa durée, seraient
également déterminés à cet instant.

Gagner un niveau, augmenter un attribut ou améliorer une compétence
renforcerait les poses suivantes. Une mine déjà posée ne serait pas
recalculée automatiquement. La zone qu'elle laisse plus tard utiliserait
les valeurs de la pose, même si le personnage a progressé avant l'explosion.
Les règles existantes de début et de durée de la zone resteraient applicables.

Les défenses, résistances et protections liées à l'importance de chaque
cible seraient celles applicables au moment où elle subit l'effet. Le
stockage porterait sur la puissance du lanceur, sans imposer un même tirage
final de dégâts ou de statut à toutes les victimes. La règle acceptée des
tirages indépendants par victime d'explosion serait conservée, sans inventer
de nouvelle amplitude de variance.

Les suppressions déjà validées lors d'une baisse de rang ou de perte de
passif resteraient prioritaires. La capacité actuelle de Réseau de mines
ne serait pas figée individuellement dans chaque mine.

**Le calcul des valeurs à la pose et leur transmission à la future zone
restent une proposition en attente de validation.** Aucun nouveau lien
d'attribut, coefficient de dégâts, texte RP, nom ou rendu natif n'est créé.
L'ordre exact d'armement et d'explosion, les sources de Gel superposées et
les marques de résurrection restent à préciser séparément.

Proposition : [INTERACT-MINE01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-mine-placement-power-INTERACT-MINE01-01.json).

Manifestes courants : statistiques **69**, catalogue principal **241**,
catalogue complet **216**, compteur de revue **110**, préparation **11**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-MINE01 — Valeurs fixées à la pose, accord du joueur

Le joueur répond « oui ca me va :) » à la proposition. Une mine conserve
les paramètres calculés lors de sa pose : élément, puissance, rayon, chances
de statut et effets prévus, y compris les valeurs de sa future zone. Gagner
un niveau ou améliorer un attribut ou une compétence modifie les poses
suivantes, sans recalculer les mines déjà présentes.

La zone créée après l'explosion utilise les valeurs fixées à la pose. Les
défenses des cibles restent celles applicables lorsqu'elles subissent l'effet.
Les règles validées de suppression en cas de baisse de rang ou de perte de
passif restent prioritaires. La capacité actuelle de Réseau de mines et
les tirages indépendants par victime ne sont pas remplacés par un résultat
unique stocké pour toutes les victimes.

Décision : [INTERACT-MINE01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/mine-placement-power-acceptance-decisions.json).
Fiche courante : [INTERACT-MINE01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mine-placement-power-INTERACT-MINE01-02.json).

### INTERACT-REANIM01 — Proposition : marques et valeurs au relèvement

Je propose d'annuler toutes les marques de Réanimation encore en attente
de ce joueur lorsque le rang effectif du sort baisse, même s'il reste
appris ou disponible. Cette annulation compléterait la suppression des
ressuscités déjà présents qui est validée. Une marque annulée ne provoquerait
pas de relèvement lorsque l'ennemi meurt, sauf si une nouvelle marque a été
posée entre-temps.

Lors d'une augmentation de rang, les marques resteraient présentes. La
puissance et la durée du nouvel allié seraient calculées avec le rang
actuel de Réanimation au moment du relèvement. Le profil naturel du monstre
serait repris à sa mort, selon les exclusions déjà acceptées : niveau,
attributs et compétences d'origine, sans ancien équipement ni effets
temporaires. Son profil ne serait pas tiré de nouveau au hasard ; aucun
nouveau facteur d'attribut ne serait ajouté.

**Exemple de rang, sans nouvel équilibrage**

Un ennemi est marqué au rang huit ; le joueur atteint le rang dix avant
sa mort : il se relèverait avec les paramètres du rang dix. Si le rang
baisse avant sa mort, la marque disparaîtrait sans produire de ressuscité.

Les règles existantes des alliés restent conservées : PV et énergie au
maximum au relèvement, puis augmentation de puissance sans remplissage
des jauges ni prolongation de durée quand le rang augmente. Une baisse
de rang supprime les ressuscités de cette compétence et leurs vrais objets
tombent au sol selon les règles validées. La limite actuelle, son augmentation
au-delà de dix et les conditions de cible déjà acceptées sont conservées.

**L'annulation des marques lors d'une baisse de rang et le calcul au
relèvement sont proposés, en attente de validation.** Les marques de
plusieurs lanceurs sur une cible, l'ordre exact des morts et remplacements,
les recharges d'origine et la première action des ressuscités restent hors
de cette passe. Aucun nouveau texte RP, nom ou rendu natif n'est créé.

Proposition : [INTERACT-REANIM01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-reanimation-mark-and-birth-INTERACT-REANIM01-01.json).

Manifestes courants : statistiques **70**, catalogue principal **242**,
catalogue complet **217**, compteur de revue **111**, préparation **12**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-REANIM01 — Marques et valeurs au relèvement acceptées

Le joueur répond « oui ok :) on valide ». Toute baisse du rang effectif
de Réanimation annule les marques de ce joueur encore en attente, en plus
de la suppression des ressuscités déjà validée. Une marque annulée ne
provoque aucun relèvement, sauf si une nouvelle marque est posée ensuite.

Lors d'une augmentation de rang, les marques restent. La puissance et la
durée du nouvel allié utilisent le rang actuel au moment du relèvement.
Son profil naturel est repris à sa mort : niveau, attributs et compétences
d'origine, sans ancien équipement ni effets temporaires. Le profil n'est
pas tiré de nouveau au hasard.

Les règles déjà acceptées restent conservées : PV et énergie au maximum
au relèvement, puis augmentation de puissance sans remplir les jauges ni
prolonger la durée lorsque le rang augmente. Les conditions de cible,
la limite actuelle, l'autonomie et les vrais objets déposés au sol lors
d'une suppression ne sont pas modifiés.

Décision : [INTERACT-REANIM01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/reanimation-mark-and-birth-acceptance-decisions.json).
Fiche courante : [INTERACT-REANIM01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-reanimation-mark-and-birth-INTERACT-REANIM01-02.json).

### INTERACT-REANIM02 — Proposition : recharges et première action

Je propose de conserver les recharges restantes des compétences d'origine
du monstre au moment de sa mort. Une compétence disponible resterait
disponible ; une compétence en recharge garderait son délai restant.
Cela concerne les compétences conservées par son profil naturel, sans
ajouter les pouvoirs de son ancien équipement ou d'un effet temporaire.

Le ressuscité apparaîtrait au relèvement, puis commencerait à agir à partir
du tour de jeu suivant. Il n'effectuerait aucune action pendant son tour
de naissance. Le choix de son action resterait autonome et soumis aux
conditions normales de ses compétences.

**Exemple fictif de recharge restante**

Une compétence a encore trois tours de recharge lorsque le monstre meurt :
elle aurait trois tours restants au moment du relèvement, puis continuerait
son décompte normal. Ces trois tours ne sont pas un nouveau délai fixé
pour une compétence particulière.

Les PV et l'énergie au maximum restent validés indépendamment de ces
recharges. La durée de présence commence au relèvement : attendre la
première action ne la prolonge pas et ne la redémarre pas.

**La conservation des recharges et la première action au tour suivant
sont proposées, en attente de validation.** Aucun délai supplémentaire
n'est proposé pour les drones. Le placement tardif ou en attente, l'ordre
exact de diminution des recharges, les marques de plusieurs lanceurs et
le choix des compétences par l'IA restent à préciser séparément. Aucun
nouveau texte RP, nom ou rendu natif n'est créé.

Proposition : [INTERACT-REANIM02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-01.json).

Manifestes courants : statistiques **71**, catalogue principal **243**,
catalogue complet **218**, compteur de revue **112**, préparation **13**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-REANIM02 — Recharges et première action acceptées

Le joueur répond « ok on continue » à la proposition. Le ressuscité
conserve les recharges restantes de ses compétences d'origine au moment
de sa mort. Une compétence disponible reste disponible ; le décompte
des autres continue normalement. Les compétences ne proviennent pas de
son ancien équipement ou d'un effet temporaire.

Il se relève à la mort de l'ennemi et commence à agir au tour de jeu
suivant son relèvement. Les PV et l'énergie au maximum restent validés.
La durée commence au relèvement, sans prolongation liée à l'attente de
sa première action. L'exemple des trois tours de recharge est fictif et
n'impose pas un nouveau délai à une compétence particulière.

Décision : [INTERACT-REANIM02-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/reanimation-cooldowns-and-first-action-acceptance-decisions.json).
Fiche courante : [INTERACT-REANIM02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-02.json).

### INTERACT-REANIM03 — Proposition : marque unique et réapplication

Je propose qu'un ennemi ne porte qu'une seule marque de Réanimation.
S'il porte déjà celle du joueur, une nouvelle tentative serait refusée
sans dépense d'énergie, tour écoulé ou modification de recharge. Sa marque
existante serait conservée. Cette exception concerne la marque de
Réanimation ; les autres statuts gardent leurs règles de réapplication.

Si un autre lanceur dispose un jour de ce pouvoir, la dernière marque
posée avec succès remplacerait la précédente et son propriétaire. Un
lancement refusé ou échoué ne remplacerait rien. À la mort de la cible,
la marque restante ne produirait qu'un seul ressuscité pour son propriétaire,
avec les paramètres de rang actuels selon la règle validée.

Une marque remplacée ne serait pas restaurée si sa remplaçante disparaît.
Le coût du lancement précédent ne serait pas remboursé. Cette règle
n'attribue pas Réanimation à un nouveau monstre et ne fixe pas les
compétences de son futur bestiaire.

Un acteur déjà créé par Réanimation ne pourrait pas recevoir de nouvelle
marque de ce sort, même s'il est hostile au lanceur. Il ne pourrait donc
pas être réanimé en chaîne. Les règles de récompenses de sa disparition
et les objets réellement confiés restent conservées.

**La marque unique, le refus d'une réapplication par le même propriétaire,
le remplacement entre lanceurs et l'exclusion des ressuscités restent
proposés, en attente de validation.** Le propriétaire qui meurt avant
sa cible, le passage entre cartes, l'ordre exact des morts et remplacements,
les textes de refus et le rendu natif restent à traiter séparément.
Aucun nouveau texte RP, nom ou message d'erreur d'interface n'est créé.

Proposition : [INTERACT-REANIM03, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-reanimation-mark-reapplications-INTERACT-REANIM03-01.json).

Manifestes courants : statistiques **72**, catalogue principal **244**,
catalogue complet **219**, compteur de revue **113**, préparation **14**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-REANIM03 — Marque unique et réapplication acceptées

Le joueur répond « parfait :) » à la proposition. Un ennemi ne porte
qu'une seule marque de Réanimation. Si elle appartient déjà au lanceur,
une nouvelle tentative est refusée sans dépense d'énergie, tour écoulé
ou modification de recharge ; la marque existante reste présente.

Si un autre lanceur dispose un jour de ce pouvoir, la dernière marque
posée avec succès remplace la précédente et son propriétaire. Une seule
réanimation est produite pour le propriétaire restant, avec son rang
actuel à la naissance selon INTERACT-REANIM01. Une ancienne marque
remplacée ne revient pas si sa remplaçante disparaît.

Un acteur déjà créé par Réanimation ne peut pas être marqué de nouveau,
même s'il est hostile au lanceur. Aucun pouvoir n'est attribué à un
nouveau monstre et aucun nouveau texte de refus n'est validé par cet accord.

Décision : [INTERACT-REANIM03-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/reanimation-mark-reapplications-acceptance-decisions.json).
Fiche courante : [INTERACT-REANIM03, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-reanimation-mark-reapplications-INTERACT-REANIM03-02.json).

### INTERACT-REANIM04 — Proposition : marques, carte et disparition du lanceur

Sur la même carte, s'éloigner ou perdre la cible de vue ne supprimerait
pas sa marque. Un changement de carte réussi annulerait les marques
en attente du lanceur sur la carte quittée : descente, refuge ou portail
démoniaque, à l'aller comme au retour. Revenir ne les restaurerait pas.

La mort ou la suppression réelle du lanceur dans la simulation annulerait
ses marques en attente. Cette règle ne concerne pas une invisibilité ou
une incapacité temporaire à lancer le sort. Seules les marques de leur
propriétaire actuel seraient supprimées ; aucune marque remplacée ne
serait restaurée et les marques d'un autre lanceur resteraient présentes.

Cette proposition concerne les marques avant la mort de leur cible.
Les familiers déjà relevés garderaient les passages entre cartes validés,
leur identité, leur équipement, leur état et leur durée restante, ainsi
que les règles d'arrivée en attente lorsqu'aucune case ne les accueille.

**L'annulation des marques au changement de carte ou à la disparition du
lanceur reste proposée, en attente de validation.** Le passage d'une cible
vers une autre carte sans son lanceur, le devenir des alliés déjà relevés
d'un éventuel lanceur non joueur après sa mort, l'ordre exact des événements
et le retour visuel restent à préciser. Aucun nouveau monstre ne reçoit
ce pouvoir et aucun nouveau texte RP ou rendu natif n'est créé.

Proposition : [INTERACT-REANIM04, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-reanimation-mark-map-and-owner-loss-INTERACT-REANIM04-01.json).

Manifestes courants : statistiques **73**, catalogue principal **245**,
catalogue complet **220**, compteur de revue **114**, préparation **15**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-REANIM04 — Marques, carte et disparition du lanceur acceptées

Le joueur répond « oui parfait » à la proposition. Sur une même carte,
s'éloigner ou perdre la cible de vue conserve sa marque de Réanimation.
Un changement de carte réussi annule les marques du lanceur sur la carte
quittée : descente, refuge ou portail démoniaque, à l'aller comme au retour.
Revenir ne les restaure pas. La mort ou la disparition définitive du
lanceur annule ses marques en attente.

La règle concerne seulement les marques de leur propriétaire actuel.
Les familiers déjà relevés gardent les règles acceptées de transfert,
leur état, leurs objets et leur durée restante. Aucun pouvoir n'est
attribué à un nouveau monstre. Les cas propres à un éventuel autre
lanceur seront traités lorsqu'une telle compétence lui sera attribuée.

Décision : [INTERACT-REANIM04-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/reanimation-mark-map-and-owner-loss-acceptance-decisions.json).
Fiche courante : [INTERACT-REANIM04, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-reanimation-mark-map-and-owner-loss-INTERACT-REANIM04-02.json).

### INTERACT-CMD01 — Proposition : coût des commandes de familiers

Je propose un coût de **1 action du joueur et 0 énergie** pour chacune
des commandes suivantes, en combat comme au calme :

- **Repositionner** : déplace le drone selon les conditions acceptées.
  La recharge reste de six tours par drone.
- **Détruire** : supprime le familier choisi. Ses objets équipés et le
  chargement éventuel de la Mule tombent sur sa case, ou sur celle du
  joueur si le familier attend son arrivée. Cette commande ne produit
  pas d'explosion et ne réinitialise pas la recharge d'invocation.
- **Ramasser** : donne l'ordre de collecte à la Mule. Ses déplacements
  et ramassages utilisent ensuite ses propres tours, comme accepté.
- **Arrêter** : interrompt la collecte de la Mule et lui fait reprendre
  le suivi du joueur.

Ouvrir la fenêtre Familiers, consulter une fiche ou préparer un choix
reste gratuit. Une commande impossible ne consommerait ni action ni
énergie et ne lancerait aucune recharge. Le coût s'appliquerait seulement
à une commande effectivement exécutée. Les conditions déjà validées
pour les familiers déployés ou en attente restent conservées.

**Les coûts de commandes et l'absence d'explosion pour Détruire restent
proposés, en attente de validation.** Les noms existants sont conservés.
Cette proposition ne définit pas les coûts des autres ordres de combat
des drones, l'ordre exact des actions des acteurs ni le rendu de la fenêtre.
Aucune nouvelle compétence, animation ou formulation d'erreur n'est créée.

Proposition : [INTERACT-CMD01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-familiar-command-action-costs-INTERACT-CMD01-01.json).

Manifestes courants : statistiques **74**, catalogue principal **246**,
catalogue complet **221**, compteur de revue **115**, préparation **16**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-CMD01 — Coûts des commandes de familiers acceptés

Le joueur répond « parfait :) » à la proposition. Repositionner, Détruire,
Ramasser et Arrêter coûtent chacune **1 action du joueur et 0 énergie**,
en combat comme au calme. Repositionner conserve sa recharge de six tours
par drone ; les trois autres commandes n'ont pas de recharge propre.

Détruire supprime le familier sans explosion ni dégâts. Son équipement
réel et le chargement éventuel de la Mule tombent sur sa case, ou sur celle
du joueur s'il attend son arrivée. La recharge de la compétence d'invocation
n'est pas réinitialisée. Ramasser lance la collecte sur les propres tours
de la Mule ; Arrêter l'interrompt et rétablit le suivi.

Consulter les fiches et préparer un choix reste gratuit. Une commande
impossible ne consomme rien et ne modifie pas les objets, les ordres ni
les recharges. Les conditions déjà acceptées restent conservées.

Décision : [INTERACT-CMD01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/familiar-command-action-costs-acceptance-decisions.json).
Fiche courante : [INTERACT-CMD01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-command-action-costs-INTERACT-CMD01-02.json).

### INTERACT-ARRIVAL01 — Proposition : priorité et cases d'arrivée

Je propose de faire arriver les familiers du plus ancien au plus récent,
selon leur invocation ou leur résurrection effective. La date de pose
d'une marque de Réanimation ne compte pas comme naissance du familier.
Il n'y aurait pas de priorité particulière pour les drones.

Les cases d'arrivée seraient les huit cases voisines du joueur, visibles,
praticables et libres de tout personnage. Chaque familier occuperait sa
propre case ; celle du joueur ne serait pas une destination. Les objets
et cadavres au sol ne bloqueraient pas à eux seuls l'arrivée et ne seraient
pas ramassés automatiquement par ce placement.

Au changement de carte, les familiers qui tiennent sur ces cases arriveraient
dans cet ordre ; les autres attendraient sans bloquer le passage. Après
chaque déplacement réellement effectué par le joueur, un seul familier
en attente arriverait si une case convient. Attendre, attaquer, consulter
un menu ou tenter un déplacement impossible ne déclencherait pas d'arrivée.

Un nouveau changement de carte emporterait tous les familiers vivants,
y compris ceux qui attendent encore, avec la même priorité. Le placement
et l'attente s'appliqueraient de nouveau sur la carte suivante. Leur état,
leurs objets et la durée restante des ressuscités garderaient les règles
déjà acceptées, sans nouvelle invocation ni restauration des réserves.

**La priorité, les huit cases voisines et le traitement d'un nouveau passage
restent proposés, en attente de validation.** Les règles acceptées de mort,
d'expiration et de dépôt d'objets pendant l'attente restent conservées.
L'ordre exact des événements du tour, les effets des dangers sur une case
d'arrivée et le rendu natif restent à préciser. Aucune action supplémentaire
n'est accordée au familier par son placement.

Proposition : [INTERACT-ARRIVAL01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-familiar-arrival-priority-INTERACT-ARRIVAL01-01.json).

Manifestes courants : statistiques **75**, catalogue principal **247**,
catalogue complet **222**, compteur de revue **116**, préparation **17**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-ARRIVAL01 — Priorité et cases d'arrivée acceptées

Le joueur répond « parfait on peut continuer :) » à la proposition.
Les familiers arrivent du plus ancien au plus récent, selon leur invocation
ou leur résurrection effective, sans priorité de type. Les huit cases
voisines du joueur doivent être visibles, praticables et libres de tout
personnage ; les objets et cadavres au sol ne bloquent pas à eux seuls.

Ceux qui tiennent arrivent au changement de carte ; les autres attendent.
Chaque déplacement réellement effectué permet ensuite l'arrivée d'un
seul familier si une case convient. Attendre ou attaquer ne déclenche pas
d'arrivée. Un nouveau passage emporte aussi ceux qui attendent encore,
avec la même priorité et les règles déjà validées pour leur état, leurs
objets, leurs recharges et la durée restante des ressuscités.

Décision : [INTERACT-ARRIVAL01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/familiar-arrival-priority-acceptance-decisions.json).
Fiche courante : [INTERACT-ARRIVAL01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-arrival-priority-INTERACT-ARRIVAL01-02.json).

### INTERACT-TIME01 — Démarrage immédiat des effets persistants demandé

Un premier brouillon du décompte préparait une proposition de durées et
recharges, avec un premier dégât périodique au tour suivant. Avant la
présentation de cette fiche, le joueur précise : « en vérité je suis pas
fan du fait qu'un effet persistant commence au tour d'apres. » puis
« je préfèrerais que ca commence directement des l'utilisation de la
compétence. »

Cette demande remplace la direction précédente : un effet persistant
nouvellement appliqué commence dès son application. Poison, Brûlure et
Saignement ont leur premier dégât immédiatement ; une zone persistante
a sa première application dès sa création. Le délai supplémentaire
précédemment prévu après l'explosion pour Résidu persistant est supprimé.

Les préparations et déclenchements différés ne sont pas anticipés : une
mine doit toujours s'armer, et l'effet persistant lié à une explosion ou
à un impact commence lorsque ce déclenchement a réellement lieu. Les
puissances, conditions, défenses et règles de dégâts aux alliés gardent
les décisions déjà validées.

Les exemples de buff et de recharge du premier brouillon ne sont ni
présentés comme proposition finale ni validés par cette demande. Leur
décompte reste ouvert. Les textes individuels modifiés devront être
présentés avant leur intégration ; aucune ancienne fiche n'est effacée.

Décision directe : [INTERACT-TIME01-DIRECTION01](../docs/reviews/refonte-agent-2026-10-06/persistent-effect-immediate-start-user-decisions.json).

### INTERACT-TIME01 — Proposition révisée : compter la première application

Je propose de compter l'application immédiate dans le nombre total annoncé,
sans seconde application automatique du même effet pendant son tour de
création. Les applications suivantes auraient lieu une fois par tour de
référence, tant que l'effet reste présent et que ses conditions s'appliquent.
Les zones indépendantes conserveraient leurs propres applications cumulées.

Exemple fictif : un poison prévu pour trois applications infligerait ses
dégâts immédiatement, puis au tour suivant et au deuxième tour suivant.
L'exemple conserve son total de dégâts ; il n'ajoute pas une application.
Il ne fixe pas une nouvelle durée ou puissance pour une compétence.

**Le nombre d'applications et l'absence d'une seconde application automatique
au même tour restent proposés, en attente de validation.** Le démarrage
immédiat est la demande explicite du joueur. La réapplication sur un statut
déjà présent, l'ordre complet du tour, l'expiration des zones et le rendu
natif restent à préciser. Les règles existantes de dernière puissance
appliquée et de durée réinitialisée restent conservées.

Proposition courante : [INTERACT-TIME01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-timer-accounting-INTERACT-TIME01-02.json).

Manifestes courants : statistiques **77**, catalogue principal **249**,
catalogue complet **224**, compteur de revue **118**, préparation **19**.
Les révisions intermédiaires **76 / 248 / 223 / 117 / 18** conservent le
brouillon préparé avant la correction du joueur ; leur convention de
durées et recharges n'est pas acceptée. Les catalogues, courbes de rangs,
attributs et autres décisions sont conservés. Les contrôles portent sur
les références de conception ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-TIME01 — Première application et total acceptés

Le joueur répond « parfait on continue » à la proposition révisée.
La première application immédiate compte dans le total annoncé. Le même
nouvel effet ne reçoit pas une deuxième application automatique pendant
son tour de création. Les applications suivantes ont lieu sur les tours
suivants, tant que l'effet reste présent et que ses conditions s'appliquent.
Les zones indépendantes gardent leurs applications propres cumulées.

L'exemple des trois applications est accepté comme explication : une
application immédiate, puis deux applications sur les deux tours suivants.
Il conserve le total des dégâts et ne fixe pas un nouveau paramètre de
compétence. Les mines et attaques différées appliquent toujours leurs
effets à l'explosion ou à l'impact. Le démarrage immédiat vient de la
demande directe déjà enregistrée sous INTERACT-TIME01-DIRECTION01.

Cet accord ne valide pas le brouillon de décompte des buffs et recharges,
la réapplication sur un statut déjà présent ni un nouvel ordre complet
des événements. Les anciennes fiches restent conservées ; leurs textes
modifiés devront être montrés avant l'intégration.

Décision : [INTERACT-TIME01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/persistent-immediate-application-count-acceptance-decisions.json).
Fiche courante : [INTERACT-TIME01, révision 3](../docs/reviews/refonte-agent-2026-10-06/review-timer-accounting-INTERACT-TIME01-03.json).

### INTERACT-TIME02 — Proposition : réappliquer un statut persistant

Je propose qu'une nouvelle attaque ou un nouveau lancement qui réussit
à réappliquer Poison, Brûlure ou Saignement remplace sa puissance,
redémarre sa durée complète et provoque immédiatement les premiers dégâts
du statut renouvelé. Cela s'appliquerait à l'impact ou à l'explosion pour
une attaque différée, sans anticiper sa préparation.

Cette application compterait comme la première de la durée renouvelée.
Le même statut ne recevrait pas ensuite une autre application automatique
pendant ce tour. Les dégâts déjà résolus ne seraient pas annulés.

Une zone déjà présente qui entretient automatiquement un statut déjà actif
renouvellerait sa durée et sa puissance selon les règles acceptées, sans
ajouter un dégât immédiat à chaque rafraîchissement. Le statut conserverait
son application périodique normale, si elle n'a pas déjà été résolue ou
remplacée par l'application immédiate du tour. Les dégâts propres à chaque
zone et leurs jets restent distincts et cumulés ; ils ne sont pas supprimés
par cette règle de décompte du statut.

Une application échouée ne modifierait pas le statut présent et ne
provoquerait pas de premier dégât de statut. Une zone appliquant un statut
qui n'était pas présent suivrait la première application immédiate acceptée.

Exemple fictif : un poison produit six dégâts par application. Une nouvelle
attaque le réapplique avec une puissance de dix ; elle produit dix dégâts
de poison immédiatement, puis les applications suivantes utilisent cette
nouvelle puissance. Ces nombres ne fixent pas les dégâts d'une compétence.

**Ces règles de réapplication restent proposées, en attente de validation.**
La dernière puissance appliquée, même plus faible, et la durée remise à
zéro restent déjà acceptées. La cadence des transferts comme Contagion,
l'ordre complet du tour, les textes individuels modifiés et le rendu natif
restent à préciser. Aucun nouveau coût ni paramètre de dégâts n'est proposé.

Proposition : [INTERACT-TIME02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-persistent-status-reapplication-INTERACT-TIME02-01.json).

Manifestes courants : statistiques **78**, catalogue principal **250**,
catalogue complet **225**, compteur de revue **119**, préparation **20**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-TIME02 — Réapplication et entretien par une zone acceptés

Le joueur répond « parfait on continue » à la proposition. Une nouvelle
attaque ou compétence réappliquant avec succès Poison, Brûlure ou Saignement
remplace la puissance, redémarre la durée complète et inflige immédiatement
la première application du statut renouvelé. Le même statut ne reçoit pas
ensuite une autre application automatique pendant ce tour.

Une zone déjà présente entretenant un statut déjà actif renouvelle sa
durée et sa puissance sans ajouter de dégât immédiat à chaque simple
rafraîchissement. Son application périodique normale reste conservée.
Les dégâts propres aux zones restent distincts et cumulés. Une application
échouée ne modifie pas le statut présent. L'exemple six vers dix est fictif
et ne fixe pas la puissance d'une compétence.

Décision : [INTERACT-TIME02-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/persistent-status-reapplication-acceptance-decisions.json).
Fiche courante : [INTERACT-TIME02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-persistent-status-reapplication-INTERACT-TIME02-02.json).

### AF06-COMPAT02 — Contagion élargie à quatre statuts

La liste historique de Contagion comprend Interférence, Faille d'armure,
Silence, Rupture de régénération et Désorientation. La vérification montre
qu'elle ne comprenait pas Poison, Brûlure ou Saignement : le point initialement
annoncé comme leur transmission par Contagion est corrigé avant proposition.

Le joueur demande : « ah oui ca ne transmet pas ces effets là ? ca me semble
anormal, qu'en penses tu ? ». L'extension à **Poison, Brûlure, Saignement et
Gel**, limitée à **un effet transmis par mort**, est présentée. Le joueur
répond : « oui on va changer ca, ca me plait pas trop . ».

La liste courante contient donc les cinq affaiblissements précédents et ces
quatre statuts supplémentaires. Le nom Contagion, le texte RP final, l'identité
AF06 et les courbes provisoires de rangs sont conservés. Cette extension
n'ajoute pas de nouvelles compétences ou zones ; elle n'approuve pas encore
les détails de sélection, de retransmission ou le cas du Gel de zone.

Décision : [AF06-COMPAT02-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/contagion-expanded-status-compatibility-user-decisions.json).
Compatibilité courante : [AF06-COMPAT02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-contagion-compatibility-AF06-COMPAT02-01.json).

### INTERACT-CONTAGION01 — Proposition : sélection et transmission

Je propose de choisir le plus récent des effets compatibles directement
appliqués par le propriétaire de Contagion, encore actif juste avant les
dégâts mortels. Un effet déjà reçu par Contagion ne serait pas sélectionnable.
Désorientation pourrait ainsi être transmise malgré son interruption par
le coup mortel ; un coup précédent l'ayant déjà supprimée la rendrait
indisponible.

La cible serait l'ennemi visible le plus proche à portée, avec un trajet
dégagé depuis la victime. Une seule tentative serait effectuée. Si elle
échoue, aucune autre cible ne serait cherchée ; s'il n'y a aucune cible
valide, aucun effet ne serait transmis.

L'effet transmis s'appliquerait immédiatement avec la durée et la puissance
prévues par Contagion, soumises aux défenses actuelles de la nouvelle cible.
Poison, Brûlure et Saignement produiraient leur premier dégât immédiatement,
compté dans le total transmis, sans autre application automatique ensuite
pendant ce tour. Un statut de même type déjà présent serait remplacé par
la puissance et la durée de la nouvelle application réussie.

Un effet reçu par Contagion ne se transmettrait pas à nouveau. Les zones
resteraient à leur emplacement : cette règle transmettrait le statut porté
par un acteur, pas la zone. Les résistances et règles propres à chaque
contrôle, notamment les exceptions validées pour Gel et Brûlure, seraient
conservées.

**Ces précisions restent proposées, en attente de validation.** La liste
des neuf statuts et un effet par mort sont déjà acceptés. Le Gel uniquement
maintenu par une zone, les liens et arrondis de rétention non encore montrés,
l'ordre exact des événements, les égalités de distance et le rendu natif
restent à préciser. Aucun nouveau chiffre, texte RP ou nom n'est proposé.

Proposition : [INTERACT-CONTAGION01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-contagion-transfer-rules-INTERACT-CONTAGION01-01.json).

Manifestes courants : statistiques **79**, catalogue principal **251**,
catalogue complet **226**, compteur de revue **120**, préparation **21**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-CONTAGION01 — Sélection et transmission acceptées

Le joueur répond « parfait on continue » à la proposition. Contagion
choisit le plus récent des effets compatibles directement appliqués par
son propriétaire, encore actif juste avant les dégâts mortels. Un effet
reçu par Contagion ne peut pas être transmis à nouveau. Désorientation
peut être sélectionnée si le coup mortel l'interrompt ; elle ne l'est plus
si un coup précédent l'avait déjà supprimée.

L'ennemi visible le plus proche à portée, avec un trajet dégagé depuis la
victime, reçoit une seule tentative. Un échec ne déclenche pas de tentative
sur une autre cible. Une transmission réussie applique immédiatement le
statut, avec sa durée et sa puissance transmises, soumises aux défenses
actuelles de la cible. Un statut identique déjà présent est remplacé.

Poison, Brûlure et Saignement produisent leur première application
immédiatement ; celle-ci compte dans le total transmis, sans autre
application automatique du même type ensuite pendant ce tour. Les zones
restent sur place. Le cas du Gel uniquement entretenu par une zone reste
à préciser, ainsi que les liens de rétention non montrés, l'ordre complet
des événements et les détails de géométrie ou d'égalité de distance.

Décision : [INTERACT-CONTAGION01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/contagion-transfer-rules-acceptance-decisions.json).
Fiche courante : [INTERACT-CONTAGION01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-contagion-transfer-rules-INTERACT-CONTAGION01-02.json).

### INTERACT-CONTAGION02 — Proposition : durée du Gel de zone transmis

Un Gel uniquement entretenu par une zone peut déjà être sélectionné grâce
à l'extension acceptée de Contagion. Il n'a cependant pas de durée autonome
sur son porteur d'origine : la sortie ou la disparition de sa zone met
normalement fin à cet entretien. Je propose donc une durée propre lors
de sa transmission à une nouvelle cible, même hors de la zone.

La durée de référence serait le temps restant de la zone source, limité
à **5 tours**. Contagion appliquerait ensuite son taux de conservation à
cette durée, arrondie à l'entier inférieur, avec au moins un tour si la
transmission réussit. Ce plafond de référence et ce lien de durée sont
nouveaux et restent à valider ; ils ne changent pas la courbe provisoire
de conservation de Contagion ni les Gels ordinaires déjà temporisés.

Exemple avec le rang effectif 11 de Contagion et sa conservation provisoire
de **80 %** : une zone ayant encore huit tours fournirait une référence
de cinq tours ; le Gel transmis durerait **4 tours**. La nouvelle cible
ne pourrait plus se déplacer, mais pourrait attaquer et utiliser ses
compétences. La disparition de la zone d'origine ne retirerait pas ce
Gel autonome. Une Brûlure réussie le supprimerait suivant la règle déjà
acceptée.

La zone ne serait ni déplacée ni copiée. Les résistances, les malus des
élites et des boss propres au Gel et l'interdiction de retransmettre un
effet reçu par Contagion seraient conservés. Les règles de sortie du Gel
entretenu sur le porteur d'origine restent inchangées.

**Cette durée plafonnée et son autonomie restent proposées, en attente
de validation.** La provenance de plusieurs Gels superposés, les autres
liens de rétention non montrés, le texte individuel modifié, le rendu natif
et l'ordre complet du tour ne sont pas validés par cette proposition.

Proposition : [INTERACT-CONTAGION02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-contagion-zone-gel-INTERACT-CONTAGION02-01.json).

Manifestes courants : statistiques **80**, catalogue principal **252**,
catalogue complet **227**, compteur de revue **121**, préparation **22**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-CONTAGION02 — Durée du Gel de zone transmis acceptée

Le joueur répond « parfait :) » à la proposition. Si le Gel sélectionné
par Contagion est uniquement entretenu par une zone, sa durée de référence
est le temps restant de cette zone, limité à cinq tours. Le taux de
conservation de Contagion réduit ensuite cette durée, arrondie à l'entier
inférieur, avec au moins un tour si la transmission réussit.

La nouvelle cible peut être gelée hors de la zone. Elle ne peut plus se
déplacer, mais peut attaquer et utiliser ses compétences. Le Gel reçu a
son propre compte à rebours : la disparition de la zone d'origine ne le
retire pas. Une Brûlure réussie le supprime. Au rang effectif 11, avec la
conservation provisoire de 80 % et une zone ayant encore huit tours, la
référence plafonnée à cinq donne quatre tours de Gel transmis.

La zone reste à son emplacement. Les résistances, les malus des élites et
des boss propres au Gel et l'interdiction de retransmettre un effet reçu
par Contagion sont conservés. La provenance de plusieurs Gels superposés
et les autres liens de rétention non montrés restent à préciser.

Décision : [INTERACT-CONTAGION02-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/contagion-zone-gel-acceptance-decisions.json).
Fiche courante : [INTERACT-CONTAGION02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-contagion-zone-gel-INTERACT-CONTAGION02-02.json).

### INTERACT-TIME03 — Proposition : décompte des buffs et recharges

Je propose que le temps écoulé pendant le lancement compte dans la durée
d'un buff ordinaire et dans la recharge de la compétence. Un buff de
trois tours agit pendant le tour de lancement, y compris les réactions
ennemies, puis pendant les deux tours suivants. Après le tour de lancement,
son icône indique donc deux tours restants.

Une recharge de trois tours lancée par une action consommant un tour
indique deux tours restants après ce lancement. Après deux autres tours
de jeu, le blocage lié à la recharge disparaît. Les autres conditions
d'utilisation continuent à s'appliquer. Une recharge encore positive
reste visible, grisée et bloquée via le bouton comme via les raccourcis,
conformément aux demandes déjà acceptées.

Les menus, survols, annulations, refus et actions sans passage du temps
ne diminuent aucun compteur. Les valeurs de trois tours servent seulement
d'exemple ; elles ne modifient aucune durée ou recharge individuelle.
Les règles validées pour les premiers dégâts persistants et leur décompte,
les délais spécifiques, les canalisations et les durées des familiers
restent conservées.

**Cette convention reste proposée, en attente de validation.** Elle reprend
uniquement le décompte des buffs et recharges du brouillon historique
INTERACT-TIME01, jamais accepté pour ces parties. Elle ne restaure pas ses
anciennes propositions de dégâts persistants au tour suivant. L'ordre
complet des acteurs, dégâts, morts et expirations, les arrondis d'affichage
des fractions de tour et le rendu natif ne sont pas validés ici.

Proposition : [INTERACT-TIME03, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-buff-cooldown-accounting-INTERACT-TIME03-01.json).

Manifestes courants : statistiques **81**, catalogue principal **253**,
catalogue complet **228**, compteur de revue **122**, préparation **23**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-TIME03 — Décompte des buffs et recharges accepté

Le joueur répond « Oui parfait :) » à la proposition. Le temps écoulé
pendant le lancement compte dans la durée du buff ordinaire et dans la
recharge de la compétence. Un buff de trois tours agit au lancement,
y compris pendant les réactions ennemies, puis pendant deux autres tours.
Une recharge de trois tours lancée par une action consommant un tour
indique deux tours restants à la fin de ce lancement.

Menus, survols, annulations, refus et actions sans passage du temps ne
diminuent aucun compteur. Une recharge positive bloque le bouton et les
raccourcis. À zéro, les autres conditions d'utilisation restent applicables.
Les exemples à trois tours illustrent le décompte ; les valeurs propres
à chaque compétence et les règles déjà validées des dégâts persistants,
délais particuliers et durées des familiers sont conservées.

Décision : [INTERACT-TIME03-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/buff-cooldown-accounting-acceptance-decisions.json).
Fiche courante : [INTERACT-TIME03, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-buff-cooldown-accounting-INTERACT-TIME03-02.json).

### INTERACT-TURN01 — Proposition : ordre de résolution d'un tour

La lecture de `complete_turn` et `resolve_ai_turn` dans
`src/game/game_state.rs` confirme que les drones sont actuellement traités
avant les ennemis. Je conserve cet ordre dans la proposition et propose
de placer les applications périodiques avant les actions de ces acteurs.
Cette observation n'est pas une validation de l'intégration de la refonte.

Pour un tour ordinaire, l'ordre proposé est le suivant :

1. L'action du joueur et ses effets immédiats, suivant les règles validées,
   notamment les premiers dégâts persistants et la création des zones.
2. Les applications dues des zones déjà actives, de la plus ancienne à la
   plus récente, puis les applications périodiques de Poison, Brûlure et
   Saignement. Une première application déjà résolue ce tour ne produit
   pas une application automatique supplémentaire du même effet ; les
   rafraîchissements automatiques et zones indépendantes conservent leurs
   règles de décompte acceptées.
3. Les familiers pouvant agir, puis les ennemis encore vivants. Le
   ressuscité attend toujours le tour suivant son relèvement pour agir.
4. La diminution des durées et recharges, l'expiration après la dernière
   période couverte et les récupérations prévues par les règles validées.
5. La mise à jour de la perception, des hostiles repérés, de la phase
   Combat/Calme et des compteurs affichés.

Chaque mort serait résolue dès les dégâts mortels, avant de poursuivre
les événements ou l'action d'un autre acteur : dépôt des objets réels,
récompenses et effets de mort applicables, dont Contagion et le relèvement.
Une cible morte ne pourrait pas exécuter son action prévue. Une
récupération ultérieure ne sauverait pas un personnage dont les PV sont
déjà tombés à zéro ; le relèvement explicite d'un ennemi marqué reste
une mécanique distincte déjà acceptée.

Exemple fictif : un ennemi possède six PV et doit subir huit dégâts de
poison ce tour. Il mourrait pendant la phase périodique, avant les actions
des familiers et des ennemis, et ne pourrait donc pas attaquer ensuite.
Ces nombres ne fixent pas les statistiques ou les dégâts d'une compétence.

**Cet ordre reste proposé, en attente de validation.** Les réactions
immédiates et délais explicites conservent leur moment propre. Aucune
action supplémentaire n'est accordée aux familiers et aucune nouvelle
vitesse ou limite générale d'actions n'est proposée. Les entrées dans les
zones, l'armement et les déclenchements des mines, les événements simultanés,
l'arrivée des familiers face aux dangers, les arrondis d'affichage et le
rendu natif restent hors de cette proposition.

Proposition : [INTERACT-TURN01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-turn-resolution-INTERACT-TURN01-01.json).

Manifestes courants : statistiques **82**, catalogue principal **254**,
catalogue complet **229**, compteur de revue **123**, préparation **24**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-TURN01 — Ordre de résolution du tour accepté

Le joueur répond « parfait oui » à la proposition. L'ordre de base est
accepté : action du joueur et effets immédiats ; applications dues des
zones de la plus ancienne à la plus récente, puis dégâts périodiques ;
familiers pouvant agir puis ennemis vivants ; durées, recharges, expirations
et récupération prévue ; actualisation de la perception et de l'interface.

Chaque mort est résolue dès les dégâts mortels, avant l'événement ou l'acteur
suivant : objets réels au sol, récompenses et effets de mort applicables,
dont Contagion et relèvement. Une cible morte ne joue plus son action.
Une récupération ultérieure ne sauve pas un personnage déjà à zéro PV.
Le ressuscité attend toujours le tour suivant son relèvement pour agir.
Les réactions immédiates, vitesses, budgets d'actions et délais propres
conservent leurs règles.

L'exemple six PV contre huit dégâts de poison illustre cet ordre et ne
fixe aucune statistique. Les détails d'entrée dans une zone, des mines,
des placements et des événements simultanés restent à préciser.

Décision : [INTERACT-TURN01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/turn-resolution-acceptance-decisions.json).
Fiche courante : [INTERACT-TURN01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-turn-resolution-INTERACT-TURN01-02.json).

### INTERACT-ZONE04 — Proposition : entrer et sortir d'une zone persistante

Je propose qu'entrer dans une zone applique son effet dès l'arrivée si
cette zone n'a pas déjà affecté la cible pendant le tour courant. La
création, l'entrée et la phase périodique partageraient une limite d'une
application de l'effet propre de chaque zone par cible et par tour.
Se déplacer à l'intérieur, ou sortir puis revenir pendant le même tour,
ne donnerait donc pas de dégâts ou de jet de statut supplémentaires de
cette même zone si son application a déjà eu lieu.

Une cible restant dans la zone pourrait recevoir la prochaine application
au tour suivant. L'entrée compterait comme la première exposition, puis
le compteur augmenterait une seule fois par tour continu dans cette zone.
La sortie remettrait à zéro uniquement le compteur de la zone quittée.
Si un retour a lieu pendant un tour déjà affecté, aucun nouveau jet ou
incrément n'aurait lieu ; la prochaine application reprendrait à la
première exposition.

Exemple illustratif : une chance de base de 10 % donne un premier jet à
10 %, puis un jet à 20 % au tour suivant si la cible reste dans la zone.
Après une sortie, le premier jet admissible suivant reprend à 10 %.
Ces valeurs ne fixent pas les paramètres d'une compétence.

La sortie arrêterait les prochaines applications de cette zone sans
annuler les dégâts déjà résolus. Poison, Brûlure et les statuts possédant
leur propre durée poursuivraient leurs timers normaux. Le Gel uniquement
entretenu par la zone cesserait à la sortie, suivant la règle déjà acceptée.
Chaque zone superposée garderait sa propre application et son compteur ;
les applications dues au même événement conserveraient l'ordre de la
plus ancienne à la plus récente.

La limite porte sur l'effet propre de la zone. Les dégâts distincts du
statut sur l'acteur conservent le décompte TIME01/TIME02 : premier statut
immédiat sans répétition automatique du même tour ; entretien d'un statut
déjà présent sans dégât de statut immédiat supplémentaire au simple
rafraîchissement. Les protections acceptées contre les dégâts des sorts
du joueur sur lui-même et ses alliés restent conservées.

**Ces règles d'entrée et de cadence restent proposées, en attente de
validation.** Les mines, les collisions avec les objets bloquants, la
provenance de plusieurs Gels de zone, la création ou l'arrivée initiale
d'acteurs et les événements de mort simultanés restent hors de cette
proposition. Aucun nouveau dégât, durée ou taux de réussite n'est proposé.

Proposition : [INTERACT-ZONE04, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-zone-entry-exit-INTERACT-ZONE04-01.json).

Manifestes courants : statistiques **83**, catalogue principal **255**,
catalogue complet **230**, compteur de revue **124**, préparation **25**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-ZONE04 — Entrée, sortie et cadence des zones acceptées

Le joueur répond « oui parfait on continue » à la proposition. L'entrée
applique l'effet de la zone immédiatement si son application sur cette
cible n'a pas déjà eu lieu pendant ce tour. Création, entrée et phase
périodique partagent une limite d'une application propre à chaque zone
et d'un jet de statut par cible et par tour, même si le jet échoue.
Un déplacement interne ou une sortie suivie d'un retour ne donne donc
pas d'application supplémentaire pendant un tour déjà affecté.

L'exposition commence au premier jet, augmente au plus une fois par tour
continu et repart à zéro à la sortie de cette zone. Les zones superposées
gardent chacune leur compteur et leur application. Les statuts ayant
leur propre durée persistent après la sortie ; le Gel uniquement entretenu
par la zone prend fin selon la règle existante. Les dégâts distincts des
statuts sur l'acteur conservent le décompte TIME01/TIME02 accepté.

Décision : [INTERACT-ZONE04-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/zone-entry-exit-acceptance-decisions.json).
Fiche courante : [INTERACT-ZONE04, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-zone-entry-exit-INTERACT-ZONE04-02.json).

### ZONE04-SCOPE01 — Correction de portée des notes de dégâts alliés

La vérification de SELF-DMG01 montre que la protection générale contre les
dégâts de ses propres sorts concerne le joueur. Les familiers et autres
alliés gardent les règles validées pour chaque compétence ; Autodestruction
exclut explicitement les alliés, tandis que la fiche SM00 conserve les
dégâts possibles aux familiers. La proposition ZONE04 affichée au joueur
n'a pas demandé de modifier ces règles de dégâts alliés.

La mention plus large dans les métadonnées et les notes de ZONE04 était
donc incorrecte. La fiche acceptée et son nouvel override sont réconciliés
avec SELF-DMG01 et les accords individuels. Cette correction documentaire
ne crée ni immunité générale des familiers ni nouvelle règle de dégâts.
La protection du joueur et tous les accords individuels sont conservés.

Réconciliation : [ZONE04-SCOPE01](../docs/reviews/refonte-agent-2026-10-06/zone-friendly-damage-metadata-reconciliation-01.json).

### INTERACT-MINE02 — Proposition : armement et déclenchement des mines

Je propose qu'une mine reste inactive pendant le tour de pose et les
réactions des acteurs de ce tour, puis s'arme à sa fin, une fois le délai
accepté d'un tour écoulé. Les menus et actions sans passage du temps ne
réduisent pas ce délai. Le voyant rouge clignotant une fois armée conserve
sa direction visuelle déjà validée, sans nouveau rendu natif.

Un ennemi passant sur une mine non armée ne la déclencherait pas. Au
moment de l'armement, un ennemi vivant encore présent sur sa case la
ferait exploser. S'il est déjà reparti, elle resterait posée et armée.
Une mine armée exploserait dès l'entrée d'un ennemi sur sa case, y compris
par déplacement forcé. Le joueur, ses familiers et les autres non-hostiles
ne la déclencheraient pas.

La mine serait consommée avant la résolution de son explosion, qui
n'aurait lieu qu'une fois. Les dégâts et statuts s'appliqueraient dès
l'explosion avec les valeurs fixées à la pose et les défenses actuelles
des victimes. Un éventuel résidu commencerait dès sa création à cette
explosion, conformément au démarrage immédiat déjà accepté, sans seconde
application automatique de création pendant le même tour.

La fiche SM00 conserve sa règle de pose sur une case visible et accessible
sans ennemi ni autre mine. Le cas proposé d'un ennemi présent à l'armement
concerne son arrivée après la pose. Les valeurs, capacités et nettoyages,
la protection générale du joueur et les règles individuelles des familiers
restent conservés.

**Ce décompte d'armement et ces précisions de déclenchement restent
proposés, en attente de validation.** Les réactions d'autres mines à une
explosion, les armements et morts simultanés, la géométrie des explosions,
les grandes empreintes d'acteurs et le rendu natif ne sont pas définis ici.

Proposition : [INTERACT-MINE02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-mine-arming-trigger-INTERACT-MINE02-01.json).

Manifestes courants : statistiques **84**, catalogue principal **256**,
catalogue complet **231**, compteur de revue **125**, préparation **26**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-MINE02 — Armement et déclenchement des mines acceptés

Le joueur répond « oui ca me va :) » à la proposition. La mine reste
inactive pendant le tour de pose et s'arme à sa fin après le délai d'un
tour. Un ennemi passant dessus avant l'armement ne la déclenche pas ;
s'il est toujours sur sa case au moment de l'armement, elle explose alors.
S'il est reparti, elle reste posée et armée.

Une mine armée explose dès l'entrée d'un ennemi sur sa case, y compris
par déplacement forcé. Le joueur et les familiers ne la déclenchent pas.
La mine est consommée une seule fois et ses dégâts, statuts et éventuel
résidu commencent dès l'explosion selon les règles validées. Le voyant
rouge clignotant une fois armée conserve sa direction acceptée.

La pose sur une case déjà occupée par un ennemi reste refusée selon SM00.
Les valeurs fixées à la pose, les limites et nettoyages, la protection
générale du joueur et les règles individuelles des familiers sont conservés.
Les événements simultanés, réactions d'autres mines à une explosion,
géométrie des explosions et rendu natif restent à préciser.

Décision : [INTERACT-MINE02-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/mine-arming-trigger-acceptance-decisions.json).
Fiche courante : [INTERACT-MINE02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-mine-arming-trigger-INTERACT-MINE02-02.json).

### INTERACT-ZONE05 — Proposition : source et durée du Gel

Je propose de traiter le Gel comme un seul statut courant, sans composants
cachés conservant plusieurs anciens Gels. La dernière application réussie
remplacerait sa puissance, sa source et sa durée ou condition de maintien.
Une tentative échouée conserverait le Gel présent sans rafraîchissement.

Un Gel uniquement entretenu par une zone dépendrait de la dernière zone
l'ayant appliqué avec succès. La sortie de cette zone ou sa disparition
mettrait fin à ce Gel, même si une autre zone de glace reste présente.
Cette autre zone devrait réussir une nouvelle application admissible
pour geler à nouveau la cible ; aucun jet supplémentaire ne serait ajouté
automatiquement au seul retrait de la source courante.

Un Gel temporaire possédant sa propre durée continuerait après une sortie
de zone. Une application ultérieure réussie par une zone pourrait cependant
le remplacer par son Gel dépendant de cette zone. Inversement, un nouveau
Gel temporaire réussi remplacerait celui lié à la zone par sa propre durée.
À la fin du Gel courant, aucun ancien Gel remplacé ne serait restauré.

Exemple : les zones A et B se superposent. A réussit à geler un monstre,
puis B réussit à son tour ; le Gel dépendrait désormais de B. Si B
disparaît, ce Gel prendrait fin et A devrait réussir une nouvelle
application pour regeler le monstre.

Une Brûlure réussie supprime toujours le Gel courant. Les Gels temporaires
reçus par Contagion conservent leur durée indépendante et leur interdiction
de retransmission. Si le Gel choisi par Contagion est uniquement entretenu
par une zone, sa durée de référence vient de la source courante, avec le
plafond et la rétention déjà acceptés sous INTERACT-CONTAGION02.

**Cette gestion de la source et de la durée reste proposée, en attente
de validation.** Les chances, résistances, malus des élites et boss,
compteurs d'exposition, cadence et ordre des zones sont conservés.
Aucun nouveau chiffre ou rendu natif n'est proposé. L'arrivée initiale
d'acteurs dans les dangers et les événements de mort simultanés restent
hors de cette proposition.

Proposition : [INTERACT-ZONE05, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-gel-source-lifetime-INTERACT-ZONE05-01.json).

Manifestes courants : statistiques **85**, catalogue principal **257**,
catalogue complet **232**, compteur de revue **126**, préparation **27**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-ZONE05 — Source et durée du Gel acceptées

Le joueur répond « oui ca me va » à la proposition. La cible conserve un
seul Gel courant. La dernière application réussie remplace sa puissance,
sa source et sa durée ou condition de maintien. Une tentative échouée ne
modifie pas ce Gel et ne rafraîchit pas son timer.

Un Gel entretenu uniquement par une zone cesse à la sortie ou au retrait
de sa source courante, même si une autre zone de glace reste présente.
Cette autre zone doit réussir une nouvelle application admissible pour
regeler la cible. Aucun jet supplémentaire ne découle du seul retrait de
la source. Un Gel doté de sa propre durée continue après une sortie de
zone ; une nouvelle application réussie peut néanmoins le remplacer.
Un ancien Gel remplacé n'est jamais restauré automatiquement.

Les interactions avec Brûlure, les durées indépendantes et l'absence de
retransmission des Gels reçus par Contagion, les chances, résistances,
malus d'importance, compteurs et ordre des zones restent conservés.
Contagion utilise la source du Gel courant lorsque celui-ci dépend d'une
zone. L'arrivée initiale des acteurs dans les dangers, les événements
simultanés et le rendu natif ne sont pas validés par cet accord.

Décision : [INTERACT-ZONE05-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/gel-source-lifetime-acceptance-decisions.json).
Fiche courante : [INTERACT-ZONE05, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-gel-source-lifetime-INTERACT-ZONE05-02.json).

### INTERACT-PETS01 — Proposition : première action et dangers à l'arrivée

Un drone nouvellement invoqué pourrait agir pendant la phase normale des
familiers du tour d'invocation, selon ses capacités et conditions normales.
Un monstre ressuscité commencerait toujours au tour suivant son relèvement,
comme déjà validé. Aucune action supplémentaire hors de cette phase ni
nouvelle règle de vitesse ou de budget d'actions n'est proposée.

Un familier existant arrivant sur une carte ou repositionné conserverait
son cycle d'action, ses délais et ses recharges. Son arrivée ne lui
accorderait pas d'action supplémentaire. Un familier en attente arriverait
après un déplacement effectif du joueur, avant les effets périodiques et
la phase des familiers, selon les règles de place et de priorité validées.

L'apparition ou l'arrivée dans une zone persistante compterait comme une
entrée si ce familier est une cible admissible. Les effets s'appliqueraient
dès l'arrivée, avec la limite validée d'une application par zone et par
cible pendant le même tour. Un repositionnement entièrement dans une même
zone suivrait les règles ordinaires des déplacements internes.

Exemple : un drone invoqué dans une zone dangereuse pour lui subirait
d'abord les effets admissibles de cette zone. S'il survit et peut agir,
il jouerait dans la phase des familiers avant les ennemis. S'il meurt à
l'arrivée, il ne jouerait pas et ses objets réels tomberaient sur sa case.

Les règles individuelles de cible et de dégâts alliés sont conservées ;
aucune immunité générale des familiers n'est créée. Les mines du joueur
ne sont pas déclenchées par ses familiers. Les ressources à la naissance,
l'état réel des familiers existants et les durées des ressuscités restent
conservés. Aucun nouveau coût, rendu natif ou interdit de placement sur
une case dangereuse n'est proposé.

**Ces précisions restent proposées, en attente de validation.** La question
du joueur sur l'adéquation de cet ordre à un roguelike mêlé au hack'n slash
ne vaut pas acceptation de cette proposition. Les coûts des autres ordres
de combat, les événements de mort simultanés et le rendu natif restent à
préciser.

Proposition : [INTERACT-PETS01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-familiar-first-action-arrival-INTERACT-PETS01-01.json).

Manifestes courants : statistiques **86**, catalogue principal **258**,
catalogue complet **233**, compteur de revue **127**, préparation **28**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-PETS01 — Première action du drone acceptée

Le joueur répond « ok on continue :) . » après la réponse sur l'adéquation
de l'ordre des phases au mélange roguelike et hack'n slash. Cette réponse
présentait explicitement la première action du drone pendant son tour
d'invocation comme une proposition restant à valider.

L'accord enregistré porte sur cette règle : un nouveau drone vivant et
admissible peut agir pendant la phase normale des familiers du tour de
son invocation. Ses capacités, conditions, vitesse et budget d'actions
ordinaires restent applicables. Les ressuscités conservent leur première
action au tour suivant leur relèvement, comme déjà validé.

Le reste de la fiche préparée PETS01 n'était pas intégralement montré dans
la réponse. Ses précisions d'arrivée et de danger ne sont pas acceptées
par déduction ; elles sont reprises dans la proposition PETS02. Les accords
antérieurs sur l'ordre des phases et l'état des familiers sont conservés.

Décision : [INTERACT-PETS01-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/drone-first-action-acceptance-decisions.json).
Fiche courante : [INTERACT-PETS01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-first-action-arrival-INTERACT-PETS01-02.json).

### INTERACT-PETS02 — Proposition : arrivée d'un familier dans une zone

Une invocation, résurrection, arrivée sur une carte ou un repositionnement
dans une nouvelle zone compterait comme une entrée. Si le familier est
une cible admissible, les effets de cette zone s'appliqueraient dès son
arrivée, avant sa première action, avec la limite validée d'une application
par zone, par cible et par tour. Un déplacement entièrement à l'intérieur
de la même zone suivrait la règle ordinaire des déplacements internes.

Si le familier mourait à l'arrivée, sa mort serait résolue avant toute
action de sa part. Ses objets réels tomberaient sur sa case selon la règle
validée, sans récompense ou objet dupliqué. Les règles de cible, résistances
et dégâts alliés propres à chaque compétence seraient conservées.

Exemple : un drone invoqué dans une zone de poison ennemie qui peut
l'affecter en subirait les effets dès son apparition. S'il survit et peut
agir, il jouerait ensuite pendant la phase normale des familiers.

Un familier en attente rejoindrait la carte après un déplacement effectif
du joueur, avant les effets périodiques et la phase des familiers, selon
les places et la priorité déjà validées. Le familier existant conserverait
son cycle d'action et ses recharges, sans action supplémentaire accordée
par l'arrivée. Son état, les ressources à la naissance, les durées des
ressuscités et l'absence de déclenchement des mines du joueur par ses
familiers restent conservés.

**Ces précisions restent proposées, en attente de validation.** Aucun
nouveau coût, chiffre de dégâts, interdit de placement sur une case
dangereuse, règle générale d'immunité alliée ou rendu natif n'est ajouté.

Proposition : [INTERACT-PETS02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-familiar-zone-arrival-INTERACT-PETS02-01.json).

Manifestes courants : statistiques **87**, catalogue principal **259**,
catalogue complet **234**, compteur de revue **128**, préparation **29**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-PETS02 — Arrivée d'un familier dans une zone acceptée

Le joueur répond « parfait :) on continue » à la proposition. Une
invocation, résurrection, arrivée sur une carte ou un repositionnement
dans une nouvelle zone compte comme une entrée. Les effets admissibles
s'appliquent dès l'arrivée, avant la première action du familier, avec la
limite validée d'une application par zone, par cible et par tour.
Un déplacement interne à une même zone conserve ses règles ordinaires.

Si le familier meurt à l'arrivée, il ne peut pas agir ; ses objets réels
et son chargement tombent sur sa case selon les accords précédents.
Les conditions de cible, les résistances et les dégâts alliés propres à
chaque compétence restent applicables. Aucune immunité générale des
familiers n'est créée.

Un familier en attente arrive après un déplacement effectif du joueur,
avant les effets périodiques et les actions des familiers. La priorité,
les places disponibles, son état réel, son cycle d'action et ses recharges
sont conservés, sans action supplémentaire due à son arrivée. Un nouveau
ressuscité garde sa première action au tour suivant son relèvement.
Les règles de ressources et les durées déjà acceptées sont conservées.

Décision : [INTERACT-PETS02-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/familiar-zone-arrival-acceptance-decisions.json).
Fiche courante : [INTERACT-PETS02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-familiar-zone-arrival-INTERACT-PETS02-02.json).

### INTERACT-CMD02 — Proposition : coût des ordres de combat des drones

Le jeu possède déjà les modes Suivre, Défensif, Agressif et Passif. Le
changement de mode actuel est sans passage du temps. Je propose un coût
de zéro tour, zéro énergie et aucune recharge propre pour changer le mode
d'un drone dans sa ligne de contrôle, ou désigner une cible à un drone
capable d'attaquer. Les conditions habituelles de présence, d'observation,
de visibilité propre au drone et d'attaque restent applicables.

L'ordre serait enregistré puis exécuté pendant la prochaine phase normale
du drone, selon ses conditions et son budget d'actions. Modifier l'ordre
ne déclencherait pas d'attaque ou de déplacement supplémentaire et ne
réinitialiserait pas les recharges. Les ordres seuls ne feraient avancer
aucun tour, timer, effet périodique, régénération ou phase ennemie.

Le joueur pourrait préparer les ordres de plusieurs drones avant sa
prochaine action. Une commande impossible préserverait l'ordre précédent,
sans dépense ni consommation de hasard. Les familiers en attente garderaient
les commandes nécessitant leur présence grisées.

Exemple : le joueur désigne le même ennemi admissible à deux drones,
puis attaque. Chacun applique son ordre lorsqu'il peut agir dans la phase
normale des familiers.

Repositionner, Détruire, Ramasser et Arrêter conservent leur coût accepté
d'une action du joueur et leurs autres conditions. Changer un mode de
combat ne mettrait pas fin à la collecte de la Mule ; son interruption
passerait par Arrêter. Les ressuscités restent autonomes. Aucun rayon
d'engagement, nouvelle compétence ou nouvelle règle de poursuite n'est
introduit ici. Le rendu natif reste à revoir séparément.

**Ces coûts et la séparation entre mode de combat et collecte restent
proposés, en attente de validation.**

Proposition : [INTERACT-CMD02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-drone-combat-order-costs-INTERACT-CMD02-01.json).

Manifestes courants : statistiques **88**, catalogue principal **260**,
catalogue complet **235**, compteur de revue **129**, préparation **30**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-CMD02 — Ordres de combat des drones acceptés

Le joueur répond « oui ca me va :) » à la proposition. Changer le mode
d'un drone ou désigner une cible à un drone capable d'attaquer coûte zéro
tour, zéro énergie et ne possède aucune recharge propre. Les modes
Suivre, Défensif, Agressif et Passif et les conditions habituelles de
présence, d'observation, de visibilité et d'attaque sont conservés.

L'ordre est enregistré puis exécuté pendant la prochaine phase normale
du drone, selon ses capacités. Aucun déplacement ou attaque supplémentaire
ni réinitialisation de recharge n'est accordé. Le joueur peut préparer
les ordres de plusieurs drones avant sa prochaine action. Une commande
impossible préserve l'ordre précédent et reste sans coût.

Repositionner, Détruire, Ramasser et Arrêter gardent leur coût validé
d'une action du joueur. Le changement de mode de combat ne met pas fin
à la collecte de la Mule : son interruption passe par Arrêter. Les
ressuscités restent autonomes. Les contrôles des familiers en attente,
les autres conditions déjà acceptées et le rendu natif à revoir restent
conservés. Aucune nouvelle portée ou priorité d'IA n'est validée ici.

Décision : [INTERACT-CMD02-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/drone-combat-order-costs-acceptance-decisions.json).
Fiche courante : [INTERACT-CMD02, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-drone-combat-order-costs-INTERACT-CMD02-02.json).

### INTERACT-CONTAGION03 — Proposition : puissance, durée et choix de cible

Je propose d'appliquer le taux courant de conservation de Contagion au
temps restant de l'effet sélectionné, aux dégâts périodiques stockés,
aux réductions chiffrées et à sa puissance d'application. Les courbes
de rangs déléguées restent conservées ; aucun nouvel équilibrage de
Poison, Brûlure ou Saignement n'est ajouté par les exemples.

La durée ou le nombre d'applications restant serait multiplié par le
taux, puis arrondi à l'entier inférieur, avec au moins un tour ou une
application si la transmission réussit, sans dépasser le temps restant
de la source. Le Gel entretenu uniquement par une zone garderait sa règle
acceptée sous CONTAGION02. Les effets binaires garderaient leur fonction,
avec durée et puissance d'application atténuées.

La puissance d'application exprimée en points serait arrondie à l'entier
inférieur, avec au moins un point ; une chance explicitement exprimée
en pourcentage conserverait sa précision. Seules les règles existantes
d'application du statut seraient utilisées, sans nouveau jet ajouté à
un effet qui n'en possède pas. Les défenses et malus d'importance de la
nouvelle cible seraient pris en compte selon ces règles.

Les bonus du lanceur déjà inclus ne seraient pas ajoutés une seconde fois.
Les dégâts ordinaires transmis avant défenses utiliseraient celles de
la nouvelle cible une fois. Les effets déjà calculés à partir de dégâts
réellement subis conserveraient leur exception d'application directe,
sans second passage dans les défenses. Les dégâts finaux resteraient
entiers ; les réductions en pourcentage garderaient leur précision.

Exemple au rang effectif 11 : conservation de 80 %. Un Poison encore
actif pour cinq applications de 40 dégâts avant résistance serait
transmis pour quatre applications de 32 dégâts avant les défenses de
la nouvelle cible. La première application serait immédiate et compterait
parmi ces quatre, comme validé. Ces nombres sont illustratifs.

La portée serait comptée depuis la victime en cases, avec une diagonale
comptant pour une case. Les murs bloqueraient la transmission directe,
sans détour de cheminement. Une égalité de distance serait départagée
par l'ordre des cases, de haut en bas puis de gauche à droite.

Les morts d'une attaque de zone suivraient l'ordre des impacts conservé
par TURN01. Chaque mort utiliserait l'état courant de la carte ; une cible
déjà morte ne serait plus admissible. Aucune nouvelle phase de mort en
lot ou nouveau tri global des victimes d'explosion n'est proposé.
Une tentative par mort, aucun repli après échec, aucun effet reçu
retransmis et les interactions de statuts déjà validées sont conservés.

**Ces liaisons de puissance et durée, arrondis et précisions de sélection
restent proposés, en attente de validation.** Le nom, le texte roleplay,
les courbes de rangs et les accords antérieurs sont conservés. Le rendu
natif et la fiche finale actualisée restent à revoir séparément.

Proposition : [INTERACT-CONTAGION03, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-contagion-retention-targeting-INTERACT-CONTAGION03-01.json).

Manifestes courants : statistiques **89**, catalogue principal **261**,
catalogue complet **236**, compteur de revue **130**, préparation **31**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception et les décisions
conservées ; aucun code de jeu ni rendu natif n'est intégré.

### INTERACT-CONTAGION03 — Puissance, durée et choix de cible acceptés

Le joueur répond « oui ca me va :) » à la proposition. Contagion conserve
une part du temps restant, des dégâts périodiques stockés, des réductions
chiffrées et de la puissance d'application. La durée est arrondie à
l'entier inférieur, avec au moins un tour ou une application lors d'une
transmission réussie, sans dépasser le temps restant de la source. Le
Gel entretenu par une zone conserve sa règle déjà validée.

Les bonus déjà inclus ne sont pas ajoutés une seconde fois. La puissance
d'application en points est arrondie à l'entier inférieur, avec un minimum
d'un point ; les pourcentages gardent leur précision. Chaque statut garde
ses contrôles d'application, les défenses de la nouvelle cible et les
exceptions validées. Les dégâts issus de dégâts réellement subis ne
repassent pas une deuxième fois dans les défenses. Les effets binaires
conservent leur fonction avec durée et puissance d'application atténuées.

La portée est comptée en cases depuis la victime, diagonales comprises
comme une case. Les murs bloquent la transmission directe. Une égalité
de distance est départagée de haut en bas puis de gauche à droite.
Les morts sont résolues dans l'ordre des impacts déjà prévu ; une cible
déjà morte n'est plus admissible. Une tentative par mort, aucun repli
après échec et aucune retransmission d'un effet reçu restent conservés.

Les interactions affichées sous READY02 sont désormais résolues. Cette
conclusion ne valide pas les anciens détails propres à une compétence
qui n'ont pas été présentés, les fiches finales actualisées, les modèles
d'équipement ou le rendu natif. Les anciennes fiches de Contagion restent
historiques ; les overrides courants et courbes déléguées font référence.

Décision : [INTERACT-CONTAGION03-ACCEPT](../docs/reviews/refonte-agent-2026-10-06/contagion-retention-targeting-acceptance-decisions.json).
Fiche courante : [INTERACT-CONTAGION03, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-contagion-retention-targeting-INTERACT-CONTAGION03-02.json).

### START-B01 — Proposition : profil de départ du Bastion

Le nom, l'orientation mêlée et survie, l'affectation de la silhouette 1,
l'actif Frappe de rupture et le passif Vitalité améliorée au rang appris
1 restent conservés. Je propose le texte de présentation suivant :

> Vous encaissez les coups et ouvrez un passage au cœur de la mêlée.

La répartition fixe reprend le profil envisagé dans la refonte : Force 7,
Agilité 5, Vitalité 7, Perception 5, Intelligence 4 et Volonté 6. Son total
est de 34 points. Les gains aux niveaux restent libres selon les règles
déjà acceptées.

L'implant initial proposerait un point d'armure supplémentaire et un
malus de 10 % d'énergie maximum. La paire pourrait être remplacée par
un implant trouvé selon l'accord existant ; les compétences apprises
resteraient acquises. Aucun nouveau nom d'implant ou affixe n'est créé.

La première compétence active conserve ses valeurs de rang 1 : +40 % de
dégâts d'arme de mêlée, coût de six énergie et recharge de trois tours,
avec une arme de mêlée active requise. Vitalité améliorée conserve ses
10 % de PV maximum au rang 1. Les courbes et effets de ces compétences
ne sont pas modifiés par cette proposition.

Au niveau 1, sans bonus d'équipement, le profil donne 38 PV de base puis
41 PV maximum après le passif et l'arrondi final. L'énergie de base est
30 puis 27 après l'implant. L'aperçu commence à 41 / 41 PV et 27 / 27
énergie. Ces valeurs ne constituent pas le profil équipé définitif.

La hache, la tenue d'assaut et les soins restent une direction de matériel.
Les modèles, valeurs, quantités et plages de dégâts réelles restent à
présenter avec l'équipement de départ. Aucun nouveau pixel, rendu de
création ou composition équipée n'est intégré ; la direction artistique
des tiles a été lue pour conserver la procédure de revue native.

**Le texte de présentation, la répartition d'attributs et les chiffres
de l'implant restent proposés, en attente de validation.** Les autres
classes, le matériel détaillé, les anciens reçus partiels et la revue
native gardent leurs étapes distinctes.

Proposition : [START-B01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-class-start-bastion-START-B01-01.json).

Manifestes courants : statistiques **90**, catalogue principal **262**,
catalogue complet **237**, compteur de revue **131**, préparation **32**.
Les catalogues, courbes de rangs, attributs et autres décisions sont conservés.
Les contrôles portent sur les références de conception, les décisions
conservées et le calcul de l'aperçu ; aucun code de jeu ni rendu natif
n'est intégré.

### START-DELEGATE01 — Bastion validé et préparation des cinq classes

Le joueur accepte le profil START-B01 et délègue la préparation des classes :

> oui ca me va :) , je te laisse faire pour les classes. si tu as des questions tu peux me les poser.

Le texte de présentation du Bastion, ses six attributs fixes, la paire
d'implant +1 armure / −10 % d'énergie maximum et l'aperçu 41 PV / 27 énergie
sont donc acceptés tels qu'affichés. Son matériel détaillé n'avait pas
encore été présenté et reste une proposition distincte.

La première réponse à la question du drone initial désignait l'Éclaireur.
Le joueur la remplace ensuite explicitement :

> pardon je change de réponse pour l'éclaireur et d'ailleurs c'est pas un super nom pour un droniste. On va changer le nom de cette classe. Met lui un drone d'attaque

Le départ préparé retient **D59, Invocation de l'Artilleur**, avec **O03,
Algorithmes de combat améliorés**, chacun au rang appris 1. La direction
« drone d'attaque » est demandée ; l'affectation de l'Artilleur est retenue
dans la préparation déléguée. La réponse précédente ne fixe plus un
départ avec l'Éclaireur. Celui-ci reste une compétence de reconnaissance
accessible, sans nouveau seuil d'apprentissage.

**CLASS-NAME-O02 — Technomancien** est le nouveau nom proposé pour
l'Opérateur. Ce nom n'est pas encore explicitement validé. La demande
est interprétée comme un changement de nom de la classe spécialisée
dans les drones, sans renommer le drone Éclaireur. Les identifiants de
revue START-O01, D59 et O03 restent conservés.

### START-ALL01 — Dossier des cinq départs préparé

Les cinq profils sont rassemblés dans un dossier sans tableau. Chacun
conserve 34 points d'attributs fixes, une compétence active et un passif
appris au rang 1. Les autres choix de départ sont Marquage balistique
pour le Traqueur, Braise pour l'Arcaniste et Mue prédatrice **M01** pour
le Métamorphe. M01 correspond au Prédateur 62 ; TR61 reste le Colosse
biomécanique et n'est pas attribué par erreur comme première forme.

Les réserves de niveau 1 sont vérifiées : Bastion 41 PV / 27 énergie,
Opérateur 30 / 38, Traqueur 30 / 30, Arcaniste 30 / 36 et Métamorphe
34 / 32. Les propositions de tenue n'ajoutent aucun bonus de PV ou
d'énergie. L'Arcaniste lance Braise pour sept énergie avec Canalisation ;
la Mue prédatrice du Métamorphe dure quatorze tours avec l'implant et
coûte quatorze énergie après le malus de coût sous forme humaine.

Le dossier propose aussi des armes et tenues communes de niveau 1,
leurs valeurs propres et des consommables de départ. Les quatre plages
de dégâts sont propres à ce matériel, sans nouvelle courbe générale
de progression des armes. Les modèles existants ne sont pas réécrits.
Les quantités de munitions et de matière relèvent encore de la première
mission. Les nouveaux textes, le matériel et le nouveau nom restent
présentés pour revue ; la délégation de préparation ne constitue pas
une approbation des compositions équipées natives.

Les silhouettes déjà acceptées sont conservées. Aucune image de
personnage, d'équipement ou de drone n'est modifiée ou intégrée ici.
L'aperçu équipé de la création sera présenté à taille native suivant la
direction artistique. Les valeurs sont des bases de conception à
équilibrer avec les premiers monstres.

Décisions : [START-DELEGATE01](../docs/reviews/refonte-agent-2026-10-06/class-start-delegation-decisions.json).
Bastion accepté : [START-B01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-class-start-bastion-START-B01-02.json).
Dossier lisible : [START-ALL01](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-01.md).
Profils structurés : [START-ALL01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-01.json).

Manifestes courants : statistiques **91**, catalogue principal **263**,
catalogue complet **238**, compteur de revue **132**, préparation **33**.
Les contrôles de conception vérifient les références, la conservation
des décisions antérieures, les identifiants de compétence et les calculs
des cinq profils. Aucun code de jeu ni rendu natif n'est intégré.

### CLASS-NAME-O02 et START-O01 — Technomancien et départ présenté acceptés

Le joueur répond « ca me va :) » à la fiche présentée. **Technomancien**
remplace donc **Opérateur** comme nom de cette classe. L'ancien nom reste
une référence historique ; START-O01 demeure son identifiant de revue.
Le drone Éclaireur n'est pas renommé.

Le texte de classe accepté est :

> Vous déployez vos drones et infiltrez les défenses ennemies par la technologie et l’occulte.

L'accord porte aussi sur le départ affiché : Invocation de l'Artilleur
D59 au rang appris 1, Algorithmes de combat améliorés O03 au rang appris
1 avec +25 % de dégâts des drones, implant donnant +15 % de PV aux drones
et +1 tour aux affaiblissements techno-occultes avec −10 % de dégâts
des armes, réserves de 30 PV et 38 énergie, pistolet et tenue technique.

Les valeurs détaillées du pistolet et de la tenue, les consommables,
les conditions d'implant non affichées et la composition équipée native
ne sont pas approuvés par le seul lien vers le dossier. La répartition
d'attributs reste préparée sous la délégation numérique des classes ;
elle n'était pas reproduite dans la dernière fiche affichée. Les autres
classes et les fiches de compétence conservent leurs reçus propres.

Décision : [START-O01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/class-start-technomancien-acceptance-decisions.json).
Fiche acceptée : [START-O01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-class-start-technomancien-START-O01-01.json).

### START-T01, START-A01 et START-M01 — Trois départs présentés pour revue

Les trois profils restants sont présentés dans ce lot, sans tableau.
Leur texte de classe, leurs attributs fixes, leurs compétences initiales,
les valeurs d'implant et les réserves apparaissent dans chaque fiche.
Les six attributs totalisent toujours 34 points ; les compétences offertes
sont toutes au rang appris 1. Les valeurs numériques sont préparées
sous la délégation de conception, pour un équilibrage ultérieur.

Le Traqueur conserve Marquage balistique et Maîtrise des attaques à
distance. Sa précision contre une cible marquée est multipliée par
1,15 au rang 1 ; aucun ajout de quinze points à la chance de toucher
n'est introduit. La marque garde ses conditions validées, notamment
l'arme à distance équipée et active, et sa consommation par la prochaine
attaque compatible contre la cible. L'implant proposé apporte +10 %
aux dégâts des armes à distance et +2 points d'esquive, avec −10 % aux
dégâts des armes de mêlée. Le départ donne 30 PV et 30 énergie.

L'Arcaniste reçoit Braise et Canalisation. Avec Volonté 8 et l'implant
proposé de +15 % aux dégâts magiques, le coup direct de Braise donne
neuf dégâts de feu avant les défenses. Canalisation réduit le coût à
sept énergie. La Brûlure garde les règles de statut et de temps déjà
acceptées. Le malus d'implant de 10 % de PV maximum donne un départ à
30 PV et 36 énergie après l'arrondi final.

Le Métamorphe reçoit Mue prédatrice M01 et Puissance métamorphe M03.
L'implant proposé ajoute deux tours aux transformations et 10 % au
coût des compétences sous forme humaine. Mue prédatrice dure ainsi
quatorze tours et coûte quatorze énergie, avec une recharge de dix
tours. Le passif offre 10 % de dégâts lorsque le personnage est
transformé. Le départ donne 34 PV et 32 énergie.

Ces résumés de création ne remplacent pas les fiches complètes des
compétences avec leurs blocs actuel et prochain rang. Aucun nouveau
statut, seuil d'apprentissage, effet visuel ou fonctionnement de
compétence n'est introduit. Le matériel est présenté comme une direction
de départ ; ses chiffres détaillés restent une revue distincte.

Propositions : [START-T01](../docs/reviews/refonte-agent-2026-10-06/review-class-start-traqueur-START-T01-01.json),
[START-A01](../docs/reviews/refonte-agent-2026-10-06/review-class-start-arcaniste-START-A01-01.json),
[START-M01](../docs/reviews/refonte-agent-2026-10-06/review-class-start-metamorphe-START-M01-01.json).
Dossier courant : [START-ALL01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-02.md).

Manifestes courants : statistiques **92**, catalogue principal **264**,
catalogue complet **239**, compteur de revue **133**, préparation **34**.
La vérification porte sur les références de conception, la portée exacte
des accords, les règles antérieures conservées et les calculs des profils.
Les trois nouvelles fiches restent proposées, sans intégration dans le jeu.

### START-THREE-ACCEPT01 — Trois départs validés

Le joueur répond « Je valide :) » aux fiches START-T01, START-A01 et
START-M01. Le texte de classe, les attributs fixes, les deux compétences
offertes au rang appris 1, les valeurs d'implant et les réserves affichées
du Traqueur, de l'Arcaniste et du Métamorphe sont acceptés tels quels.
Leurs directions de matériel sont aussi conservées : fusil et tenue
légère, couteau et tenue rituelle, couteau et tenue métamorphe.

Les valeurs des trois profils ne sont pas révisées à cette étape.
Les accords du Bastion et du Technomancien restent conservés. La
répartition d'attributs du Technomancien garde sa préparation sous la
délégation numérique des classes. Les détails du matériel, les nouveaux
descriptifs d'objets, les conditions non affichées et les compositions
natives restent distincts de cette validation des départs.

Décision : [START-THREE-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/class-three-core-starts-acceptance-decisions.json).
Fiches acceptées : [START-T01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-class-start-traqueur-START-T01-02.json),
[START-A01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-class-start-arcaniste-START-A01-02.json),
[START-M01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-class-start-metamorphe-START-M01-02.json).

### AMMO-UNIVERSAL01 — Matière des tirs supprimée, munitions communes

À la question sur les munitions, le joueur répond :

> Non en faite, on va supprimer "la matière" et mettre les mêmes munitions pour toutes les armes à distance (lourde ou non).

La refonte supprimera donc l'ancienne réserve de matière des tirs et son
identité dans les textes du jeu. Toutes les familles d'armes à distance,
y compris les armes lourdes, partageront un type de munition. Cela
ne fixe pas une consommation identique pour tous les modèles. Les
coûts, les stocks, les rechargements, les poids, les taux de loot et
l'approvisionnement des familiers et monstres restent à définir.

La réserve commune d'énergie des pouvoirs est conservée. Les compétences
de mines et de grenades gardent leurs coûts en énergie uniquement.
Une compétence de grenade n'est pas assimilée à un lance-grenades équipé.

La vérification du code distingue la matière des tirs, gérée par
`weapon_supplies.rs`, des crédits et fragments de l'artisan dans
`equipment_upgrade.rs`. La remarque précédente sur des usages de cette
même matière pour les améliorations est corrigée : ces coûts utilisent
des ressources distinctes. Aucune nouvelle monnaie ni suppression
générale des objets d'artisanat n'est décidée par cette réponse.

La direction est acceptée pour la conception. Les anciens identifiants
de matière, événements, refus, loots et libellés seront remplacés lors
de l'intégration de la refonte ; ils ne sont pas modifiés dans ce lot.

Décision : [AMMO-UNIVERSAL01](../docs/reviews/refonte-agent-2026-10-06/universal-ammunition-and-matter-removal-decisions.json).

### START-GEAR01 — Matériel initial présenté pour revue

Le lot présente les valeurs propres de quatre armes et cinq tenues
communes de niveau 1, ainsi que les réserves de départ. Les anciens
modèles ne sont pas réécrits. Les plages de dégâts sont celles de ces
armes seulement ; aucune nouvelle courbe générale de progression ou
variance de sorts n'est introduite.

La hachette proposerait 6–10 dégâts physiques, une portée d'une case
et −12 de précision. Le pistolet proposerait 3–5 dégâts, une portée
de cinq cases et +5 de précision. Le fusil de patrouille proposerait
3–5 dégâts, une portée de sept cases et +5 de précision. Le couteau
proposerait 2–4 dégâts, une portée d'une case et +10 de précision.
Ces chiffres sont les valeurs de base avant les attributs et autres
modificateurs applicables.

Les tenues proposeraient trois points d'armure et cinq points d'esquive
en moins pour le Bastion, deux points d'armure pour le Technomancien,
un pour le Traqueur, un pour l'Arcaniste et deux pour le Métamorphe.
Leurs réserves de PV et d'énergie restent celles validées.

L'aperçu de Frappe de rupture est calculé avec la hachette active,
Force 7 et son bonus de rang 1. Les bonus de dégâts s'ajoutent selon
la règle existante : +40 % de la compétence et +2 % de Force. La plage
affichée est donc 8–14 dégâts physiques avant les défenses et les
coups critiques. La tenue et l'implant initial donnent quatre points
d'armure au total. L'arrondi final est conservé.

Le Technomancien et le Traqueur recevraient chacun quarante munitions
communes. Le pistolet et le fusil initiaux en consommeraient une par
projectile. Les consommations des armes lourdes et les profils de
chargeur restent distincts de cette proposition de stock initial.

Chaque classe recevrait deux Doses de soin restaurant chacune douze
PV et une Cellule d'énergie restaurant douze énergie, dans la limite
du maximum. Ces objets sont consommés à l'utilisation. Leurs noms,
valeurs et le texte de remise d'équipement sont proposés ; aucun
descriptif RP individuel d'objet ni nouvelle icône n'est inventé ici.
L'aperçu équipé de chaque classe sera montré à taille native avant
l'intégration, suivant la direction artistique lue pour ce lot.

Proposition : [START-GEAR01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-starting-equipment-START-GEAR01-01.json).
Dossier courant : [START-ALL01, révision 3](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-03.md).

Manifestes courants : statistiques **93**, catalogue principal **265**,
catalogue complet **240**, compteur de revue **134**, préparation **35**.
Les contrôles vérifient les références de conception, la conservation
des accords, les profils non modifiés et le calcul de l'aperçu de
Frappe de rupture. Le matériel reste proposé, sans code de jeu ou
nouveau rendu intégré.

### START-GEAR01-ACCEPT01 — Matériel de départ validé

Le joueur répond « ok :) on peut continuer » au lot START-GEAR01.
Les noms et les valeurs affichés des quatre armes, des cinq tenues,
des stocks et des consommables sont acceptés. Le texte de remise du
matériel, l'aperçu de Frappe de rupture à 8–14 dégâts et les quatre
points d'armure du Bastion sont également conservés tels qu'affichés.

Le Technomancien et le Traqueur commencent donc avec quarante munitions
communes ; leur pistolet et leur fusil initiaux consomment une munition
par projectile. Les cinq classes reçoivent deux Doses de soin restaurant
chacune douze PV et une Cellule d'énergie restaurant douze énergie,
dans la limite du maximum. Les compétences de mines et de grenades
conservent leurs coûts en énergie déjà validés.

La validation ne porte pas sur des poids, des limites de piles,
des règles de chargeur ou de rechargement, l'approvisionnement des
autres acteurs, les statistiques d'armes lourdes, des descriptifs RP
individuels d'objets ni leurs icônes. Les conditions d'implant non
affichées conservent leur revue distincte. Aucun chiffre n'est modifié
pour cet accord.

Décision : [START-GEAR01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/starting-equipment-acceptance-decisions.json).
Fiche acceptée : [START-GEAR01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-starting-equipment-START-GEAR01-02.json).

### START-VIS01 — Cinq compositions équipées proposées

La direction artistique a été lue. Les corps natifs de 32 × 32 déjà
acceptés sont assemblés avec les manches, les armes et les doigts du
rendu existant. Aucune couche n'est dessinée ou recolorée. Le canevas
transparent de 48 × 48 conserve un pixel pour un pixel et permet le
débordement des armes sans agrandir le personnage.

Les numéros de revue restent stables :

1. Bastion, corps 9 et famille visuelle Hache pour la hachette de départ.
2. Technomancien, corps 2 et famille Pistolet.
3. Traqueur, corps 1 et famille Fusil de précision pour le fusil de patrouille existant.
4. Arcaniste, corps 11 et famille Couteau.
5. Métamorphe, corps 5 et famille Couteau.

Ces familles désignent les calques existants et ne modifient pas la
classification mécanique des armes. Les prises et les poses sont
celles du catalogue actuel ; aucune nouvelle obligation d'utilisation
à une ou deux mains n'est introduite par cette planche.

La planche compare les silhouettes sans arme avec les compositions
équipées, montre la taille native et un zoom entier ×3, puis les
fonds sombre, clair et texturé. Une scène native distincte assemble
les cinq personnages et des icônes d'armes existantes sur plusieurs
sols du jeu. Il s'agit d'un montage de revue, sans capture d'une
nouvelle création de personnage jouable.

Les contrôles portent sur l'alpha, les sept points de prise, l'absence
de nouveaux fragments détachés et la conservation des pixels du corps
hors des découpes explicites de mains et du masque d'occlusion des
calques. Les fichiers sources restent identiques. Les cinq nouveaux
assemblages sont proposés pour validation visuelle ; les interfaces,
animations et nouvelles icônes de consommables restent distinctes.

Proposition : [START-VIS01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-01.json).
Planche : [cinq classes équipées](../assets/source-art/refonte-agent-2026-10-06/classes-equipees-START-VIS01-review-01.png).
Situation native : [sols du jeu](../assets/source-art/refonte-agent-2026-10-06/classes-equipees-START-VIS01-scene-native-01.png).

### HEAVY-FAMILY01 — Séries d'armes lourdes demandées

Le joueur demande plusieurs séries par famille d'armes lourdes,
adaptées aux personnages : Lance-flammes, Lance-roquettes, Gatling,
Lance-plasma et Lance-grenades. Les lance-flammes déjà présents
serviront de base. Les modèles existants de lance-roquettes et de
lance-grenades sont conservés comme références ; les mitrailleuses
et armes à énergie existantes ne deviennent pas automatiquement
des nouveaux modèles de Gatling ou de lance-plasma.

La demande fixe ce travail de catalogue et d'adaptation des poses
et des prises. Elle ne fixe pas le nombre de modèles, leurs niveaux,
dégâts, effets, poids, prérequis, consommations, taux de loot ni leurs
descriptifs individuels. Elle ne supprime aucune autre famille
existante. Les nouvelles séries suivront la revue de candidats
numérotés à taille native avant intégration. Les armes lourdes
conservent la direction acceptée des munitions communes.

Décision : [HEAVY-FAMILY01](../docs/reviews/refonte-agent-2026-10-06/heavy-weapon-family-series-decisions.json).
Dossier courant : [START-ALL01, révision 4](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-04.json).

Manifestes courants : statistiques **94**, catalogue principal **266**,
catalogue complet **241**, compteur de revue **135**, préparation **36**.
Les contrôles de ce lot vérifient 1 783 références de contenu et de
fichiers, la conservation des profils et des règles acceptées ainsi
que l'assemblage des calques. Aucun code de jeu ni asset actif n'est
modifié. Les nouveaux aperçus équipés restent proposés.

### START-VIS01-FEEDBACK01 — Prises et lisibilité à corriger

Le joueur refuse la prise du pistolet du Technomancien et signale
un avant-bras supplémentaire ainsi qu'une forme claire parasite
sur les armes de mêlée à une main. Il confirme que le problème de
main et de manche touche également le Technomancien. Il trouve
ensuite le Traqueur et son fusil trop détaillés pour la taille
native. « Son fils » est compris comme « son fusil » dans le
contexte de cette planche, sans introduire un autre personnage.

Les contrôles précédents de contact entre les calques ne suffisaient
pas à établir une anatomie ni une matière cohérentes. Le montage
ajoutait des manches et doigts génériques clairs après une découpe
rectangulaire de la main du corps d'origine.

La révision 2 propose de conserver intégralement le bras et le gant
noir d'origine pour les candidats 2, 4 et 5. La poignée de l'arme
est déplacée dans ce gant ; l'arme est dessinée avant le corps, qui
recouvre sa poignée. Aucun nouveau bras ou doigt n'est ajouté. Les
pixels opaques du corps d'origine sont identiques et les contacts
entre les trois poignées et les gants ont été vérifiés. Cette règle
de montage est un prototype ; ses autres familles, côtés et groupes
seront revus avant tout changement du rendu actif.

Le joueur conteste aussi l'association systématique des couleurs
d'armure et d'arme. Leur indépendance, déjà définie dans la direction
artistique, est conservée. Aucune obligation d'assortiment n'est
créée par les choix de palettes des anciennes sources.

Décision : [START-VIS01-FEEDBACK01](../docs/reviews/refonte-agent-2026-10-06/equipped-class-visual-feedback-decisions-01.json).
Prototype des prises : [START-VIS01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-02.json).

### START-VIS01 — Révision 3, Traqueur simplifié et palettes indépendantes

La compétence imagegen a été utilisée pour préparer une nouvelle
proposition de Traqueur et de fusil, à partir des références locales
inspectées. Les générations, leurs consignes et les versions
intermédiaires sont conservées dans le dossier de la refonte.
Le corps utilise de grandes masses bleues et ivoire ; le fusil est
acier et anthracite. Les gants sont sombres et la zone claire du
poignet a été retirée dans la dernière génération.

La pose équipée est conditionnée pour la revue avec un échantillonnage
nearest, un alpha solide et douze couleurs opaques sans dithering.
Le canevas final reste de 48 × 48 pour le débordement de l'arme ;
la référence humaine reste une case native de 32 × 32. Les pixels
de l'ancienne silhouette du Traqueur ne sont pas déclarés conservés :
il s'agit d'une nouvelle proposition simplifiée à juger à taille native.

Cette pose générée complète ne remplace pas le système de calques.
Après l'accord visuel, il faudra préparer et vérifier le corps fixe,
les bras, les prises et l'arme séparés avant son utilisation dans le
rendu actif et les changements d'équipement. La réduction de palette
et les contrôles de fichier ne constituent pas une validation artistique.

La planche courante réunit les corrections des candidats 2, 4 et 5
et la nouvelle proposition 3, avec comparaison avant/après, vues
natives, zoom entier et plusieurs fonds. Le 1 reste inchangé, sans
validation implicite. Une scène native séparée montre leur lecture
sur les sols du jeu. Les chiffres des classes, les compétences,
les implants, le matériel initial validé et la demande de séries
d'armes lourdes restent inchangés. Les corrections visuelles sont
proposées, sans intégration automatique.

Proposition courante et sources : [START-VIS01, révision 3](../docs/reviews/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-03.json).
Planche : [prises et lisibilité](../assets/source-art/refonte-agent-2026-10-06/classes-equipees-START-VIS01-review-03.png).
Situation native : [sols du jeu](../assets/source-art/refonte-agent-2026-10-06/classes-equipees-START-VIS01-scene-native-03.png).
Dossier de classe : [START-ALL01, révision 5](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-05.json).

Manifestes courants : statistiques **95**, catalogue principal **267**,
catalogue complet **242**, compteur de revue **136**, préparation **37**.
Les contrôles vérifient 1 824 références de contenu et de fichiers,
la conservation des règles et chiffres acceptés, l'alpha et les
paramètres des aperçus natifs. Aucun code de jeu ni asset actif
n'est modifié par cette revue.

### START-VIS01-ACCEPT01 — Accord sur les aperçus précédents

« Je valide » accepte les quatre candidats modifiés de la révision 3 :
Technomancien, Traqueur, Arcaniste et Métamorphe. Le joueur suspend ensuite
la suite par « attends », puis précise que le pistolet doit pointer vers
le bas avec ce bras. L'accord sur le gant et le bras d'origine demeure ;
l'orientation vers le haut est remplacée par cette nouvelle demande.
Les silhouettes, prises et couleurs déjà acceptées des autres candidats
restent valables. Le Bastion inchangé et les combinaisons d'armes non
présentées ne reçoivent pas de validation implicite.

Décision : [START-VIS01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/equipped-class-visual-acceptance-decisions-01.json).

### START-VIS01-FEEDBACK02 — Pistolet vers le bas et robe rouge vif

Le candidat 2 conserve intégralement son bras et son gant noir. Le calque
du pistolet est orienté vers le bas puis sa poignée rejoint le gant.
Le candidat 4 conserve sa silhouette et sa prise de couteau ; seul le
matériau principal de sa robe change de palette. Les parties ivoire,
les pixels protégés et la lame gardent leurs couleurs.

La première robe rouge, révision 4, est jugée insuffisamment vive :
« Le rouge n'est pas assez petant ». La révision 5 présente un rouge
plus saturé. « Beaucoup mieux oui » valide cette nouvelle couleur pour
l'Arcaniste. Cet accord porte sur la robe ; l'orientation corrigée du
pistolet attend encore une validation visuelle distincte.

Ces deux montages utilisent les calques, les matériaux et les points
d'attache existants. Les corps restent natifs 32 × 32 dans un canevas
48 × 48 pour les armes. La revue montre la comparaison, un zoom entier,
des vues 1:1, plusieurs fonds et une scène sur les sols du jeu. Le bras
du Technomancien ne change pas ; aucun avant-bras ou doigt n'est ajouté.
La palette rouge vif est conservée comme proposition acceptée, sans
remplacement de la banque active dans cette étape de conception.

Demande et réserves : [START-VIS01-FEEDBACK02](../docs/reviews/refonte-agent-2026-10-06/pistol-red-robe-visual-feedback-decisions-01.json).
Robe rouge vif validée : [START-VIS01-4-ACCEPT02](../docs/reviews/refonte-agent-2026-10-06/red-robe-visual-acceptance-decisions-01.json).
Proposition et sources : [START-VIS01, révision 5](../docs/reviews/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-05.json).
Planche : [pistolet vers le bas et robe rouge vif](../assets/source-art/refonte-agent-2026-10-06/classes-equipees-START-VIS01-review-05.png).
Situation native : [sols du jeu](../assets/source-art/refonte-agent-2026-10-06/classes-equipees-START-VIS01-scene-native-05.png).

### WEAPON-VIS-COVERAGE01 — Adapter toutes les armes aux nouvelles classes

Le catalogue actuel contient 113 modèles, tous reliés à 18 dessins de
familles d'armes portées, sans ID manquant. L'adaptation aux nouveaux corps
est possible en réutilisant ces familles et en vérifiant leurs prises,
les mains actives, les deux côtés et l'ambidextrie. La couverture des ID
ne signifie pas que toutes ces nouvelles compositions ont été montrées
ou validées. Elle ne signifie pas non plus 113 dessins individuels.
La demande déjà acceptée de séries lance-flammes, lance-roquettes,
Gatling, lance-plasma et lance-grenades est conservée.

Contrôle du catalogue : [WEAPON-VIS-COVERAGE01](../docs/reviews/refonte-agent-2026-10-06/weapon-held-family-coverage-2026-10-09.json).
Dossier de classe : [START-ALL01, révision 6](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-06.json).

Manifestes courants : statistiques **96**, catalogue principal **268**,
catalogue complet **243**, compteur de revue **137**, préparation **38**.
Les contrôles vérifient 1 836 références, les règles et chiffres conservés,
les compositions et le catalogue actuel. Aucun code de jeu ni asset actif
n'est modifié ; la capture du futur rendu intégré reste à faire.

### START-VIS01-FEEDBACK03 — Pistolet dans l'axe du bras

Le joueur précise que le canon doit suivre la direction du bras et de la
main. La version verticale reste un recours acceptable si cet alignement
n'est pas réalisable. Il autorise aussi le débordement de l'arme autour
de la case, comme pour les armes à deux mains. La référence du corps
reste une tile native 32 × 32 ; sa taille et ses collisions ne changent pas.

La révision 6 propose uniquement une nouvelle orientation du pistolet du
candidat 2. Le calque existant est incliné autour de sa poignée dans la
direction mesurée de l'avant-bras, vers le bas et l'extérieur. La poignée
rejoint le gant original. Le bras et le gant ne sont pas redessinés ; aucun
doigt ni avant-bras supplémentaire n'est ajouté. Le débordement tient
dans le canevas 48 × 48 sans couper l'arme. La rotation utilise un
échantillonnage nearest et ne crée aucune nouvelle couleur.

Les autres candidats et la robe rouge vif validée sont conservés. La
comparaison numérotée montre la version verticale et le nouvel aperçu,
en zoom entier et à taille native sur plusieurs fonds. L'orientation
diagonale est soumise à la revue, sans intégration automatique.

Demande : [START-VIS01-FEEDBACK03](../docs/reviews/refonte-agent-2026-10-06/pistol-arm-axis-visual-feedback-decisions-01.json).
Proposition : [START-VIS01, révision 6](../docs/reviews/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-06.json).
Planche : [pistolet dans l'axe du bras](../assets/source-art/refonte-agent-2026-10-06/classes-equipees-START-VIS01-review-06.png).
Scène native : [sols du jeu](../assets/source-art/refonte-agent-2026-10-06/classes-equipees-START-VIS01-scene-native-06.png).
Dossier de classe : [START-ALL01, révision 7](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-07.json).

Manifestes courants : statistiques **97**, catalogue principal **269**,
catalogue complet **244**, compteur de revue **138**, préparation **39**.
Les contrôles vérifient 1 806 références, la conservation des décisions
acceptées, le contact de la poignée, l'absence de pixels coupés et les
pixels du corps inchangés. Aucun code de jeu ni asset actif n'est modifié.

### WEAPON-SIL01 — Toutes les familles sur les cinq silhouettes

« Oui c'est mieux » valide l'orientation diagonale du pistolet proposée en
révision 6. Le joueur demande ensuite son passage en gris clair et confirme
l'adaptation de toutes les armes aux silhouettes. Il exclut le rose des
armes et armures ; les variantes existantes passeront à un **gris neutre**,
selon sa réponse explicite. Leurs modèles, statistiques et bonus sont conservés.
Le violet reste une palette distincte. L'accord sur ces directions ne vaut
pas acceptation des nouvelles prises ou des nouvelles palettes concrètes.

La revue couvre les **113 modèles actuels**, reliés aux **18 familles**,
sur les **cinq silhouettes**. Elle propose **375 configurations** : sans
arme, mains inversées pour chaque arme à une main, prises à deux mains,
et toutes les associations de deux armes compatibles. Les identifiants
de candidat conservent le numéro de classe et le numéro de famille.
Les futures armes reprennent ces règles d'attache, avec une revue des
nouvelles formes lorsqu'elles sont créées.

Les corps restent natifs 32 × 32 ; le canevas 64 × 64 accueille les
débordements. Le bras et le gant du Technomancien sont conservés.
Les avant-bras à deux mains viennent des couches existantes et suivent
les matériaux de chaque tenue, indépendamment de la palette de l'arme.
Le Traqueur utilise une couche sans arme préparée depuis la source disponible :
elle et ses nouvelles prises sont proposées à la revue, sans revendiquer
une identité de pixels avec son ancien prototype complet au fusil.
La robe rouge vif de l'Arcaniste et la composition au couteau du Métamorphe
restent identiques, hors marge transparente du canevas.

La galerie permet de changer les deux mains et le fond, avec une vue native
et des zooms entiers. Trois planches natives montrent toutes les familles.
La comparaison gris clair conserve exactement la géométrie du pistolet
accepté. Le remplacement rose/gris dispose de son propre aperçu.
Le correctif de palette est préparé **sans être appliqué** : le même
indice de variante sera conservé et les anciennes valeurs JSON `Pink`
seront relues comme `Gray`. Aucun objet n'est supprimé par ce changement.

Décisions : [WEAPON-SIL01-DIRECTION01](../docs/reviews/refonte-agent-2026-10-06/weapon-gray-and-family-adaptation-decisions-01.json).
Banque : [WEAPON-SIL01, révision 3](../docs/reviews/refonte-agent-2026-10-06/weapon-silhouette-review-03/review-weapon-silhouettes-WEAPON-SIL01-03.json).
Présentation : [planches, couleurs et galerie](../docs/reviews/refonte-agent-2026-10-06/weapon-silhouette-review-03/weapon-silhouette-review-presentation-02.json).
Galerie : [prises et combinaisons](../artifacts/refonte-agent-2026-10-06/weapon-silhouette-review-03/weapon-silhouette-gallery-02.html).
Départs : [START-VIS01, révision 7](../docs/reviews/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-07.json).
Situation native : [sols et objets du jeu](../assets/source-art/refonte-agent-2026-10-06/classes-equipees-START-VIS01-scene-native-07.png).
Dossier de classe : [START-ALL01, révision 8](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-08.json).

Manifestes courants : statistiques **98**, catalogue principal **270**,
catalogue complet **245**, compteur de revue **139**, préparation **40**.
Les contrôles vérifient 2 237 références, l'alpha des 375 configurations,
la couverture du catalogue et la conservation des règles acceptées.
La galerie est testée dans Chromium sur ordinateur et petit écran ;
aucune erreur JavaScript ni débordement horizontal n'a été détecté.
Le correctif de palette passe le contrôle d'applicabilité sans être appliqué.
Il s'agit d'aperçus de revue : aucun code Rust ni asset actif n'est modifié,
et la validation du rendu intégré reste à effectuer après acceptation.

### WEAPON-SIL01-ACCEPT01 — Validation et intégration graphique

« Oui je valide » répond explicitement à la revue des nouvelles prises,
des rendus gris et de la base du Traqueur. L'accord couvre les 375
configurations de la galerie, les 18 familles sur les cinq silhouettes,
le pistolet gris clair et le gris neutre qui remplace la variante rose.
Les acceptations précédentes demeurent ; aucune autre mécanique n'est ajoutée.

Les couches natives sont installées dans une banque dédiée, avec les cartes
de matériaux et les prises propres aux silhouettes. Le rendu actif utilise
ces couches pour les indices de tenue correspondants. Les autres corps
conservent leur composition. Les armes visibles suivent toujours le groupe
actif et gardent leur famille, indépendamment de la tenue. Les matériaux
par défaut du pistolet et du fusil de précision sont aussi repris par leurs
illustrations au sol et dans l'inventaire.

Le correctif de palette est appliqué : `Pink` est remplacé par `Gray`
au même indice binaire, avec une compatibilité de lecture JSON. Le nombre
de variantes et les tirages cosmétiques sont conservés. Aucun modèle,
bonus, attribut, compétence, ressource de combat ou collision ne change.
Il s'agit de l'intégration graphique ; le catalogue des classes jouables
et les autres systèmes de la refonte restent distincts.

Les 375 compositions du moteur passent une comparaison pixel par pixel
avec la banque acceptée. Les 23 tests graphiques et neuf tests ciblés de
couleurs et de persistance passent. Les captures du rendu actif vérifient
les cinq silhouettes, les mains inversées, l'ambidextrie, les armes lourdes,
le gris neutre, la respiration et une scène du plateau. Les contrôles de
capture confirment une partie inchangée et aucun ajout de texture après
l'échauffement du cache. Les builds debug et release et le formatage passent.

Accord : [WEAPON-SIL01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/weapon-silhouette-visual-acceptance-01.json).
Banque active : [couches et matériaux](../assets/sprites/cyberpunk/agent-rigs/manifest.json).
Captures et contrôles : [validation](../docs/reviews/weapon-integration-2026-10-09/validation.json).
Départs : [START-ALL01, révision 9](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-09.json).

Manifestes courants : statistiques **99**, catalogue principal **271**,
catalogue complet **246**, compteur de revue **140**, préparation **41**.
Les textes, chiffres, interactions et décomptes de compétences acceptés
restent inchangés. Les dossiers courants distinguent les ressources
graphiques actives du reste de la refonte encore à implémenter.

### START-IMPLANT01 — Portée des implants de départ, proposition

Après l'intégration des couches d'armes acceptées, la revue reprend les
conditions d'application des implants initiaux. Les cinq paires de bonus
et malus de START-ALL01, révision 9, sont conservées exactement.

La proposition précise les armes lourdes, les effets persistants des sorts,
les familiers, les durées au lancement et le calcul des malus relatifs.
Le surcoût du Métamorphe est proposé sur les activations humaines, en
conservant le coût validé du déplacement furtif. Les montants d'exemple
illustrent les calculs ; ils ne créent ni modèle d'arme ni affixe de loot.

Ces conditions restent à valider pour les éléments présentés dans la
conversation. Aucun nouveau nom, texte RP, icône ou rendu d'implant n'est
proposé. Aucun code de gameplay n'est modifié. Les manifestes 99 / 271 /
246 / 140 / 41 et leurs références historiques restent inchangés.

Proposition : [START-IMPLANT01, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-initial-implants-START-IMPLANT01-01.json).

### START-IMPLANT01-ACCEPT01 et AMMO02 — Implants validés, munitions en revue

« Parfait :) » valide les précisions présentées pour les cinq implants :
armes lourdes dans la portée des effets d'arme, affaiblissements prolongés
au lancement, dégâts persistants des sorts sans double application,
surcoût des activations humaines sans modifier le déplacement furtif,
malus relatifs et conservation des compétences et ressources lors du
remplacement. Les douze valeurs de bonus et malus déjà validées demeurent.
Les notes techniques non présentées ne créent aucun autre effet approuvé.

AMMO02 prépare la réserve commune du personnage et la consommation des
attaques immédiates. Les 40 munitions du Technomancien et du Traqueur et
le coût de 1 munition par projectile de leurs armes de départ sont déjà
acceptés par START-GEAR01. La proposition distingue les rafales, le coût
par projectile des armes lourdes à définir avec leurs modèles, les
explosions et le refus intégral d'une attaque faute de munitions. Les
chargeurs, textes d'affichage et description de l'objet proposés restent
à valider. Le poids, les quantités de loot, les réserves des autres acteurs,
les tirs différés et l'icône restent séparés.

Accord : [START-IMPLANT01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/initial-implants-acceptance-decisions-01.json).
Règles retenues : [START-IMPLANT01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-initial-implants-START-IMPLANT01-02.json).
Proposition : [AMMO02, révision 1](../docs/reviews/refonte-agent-2026-10-06/review-universal-ammunition-AMMO02-01.json).
Départs : [START-ALL01, révision 10](../docs/reviews/refonte-agent-2026-10-06/review-class-starts-START-ALL01-10.json).

Manifestes courants : statistiques **100**, catalogue principal **272**,
catalogue complet **247**, compteur de revue **141**, préparation **42**.
Le contrôle vérifie 1 837 références. Les profils de classe, compétences,
validations précédentes et couches graphiques restent inchangés. Aucun
code de gameplay n'est modifié par cette étape de revue.

### AMMO02-MSG01 — Message de refus raccourci

Le joueur demande exactement « Munitions insuffisantes ». Ce texte
remplace le message détaillant les quantités requises et disponibles.
La correction porte uniquement sur ce message ; les autres textes et
les règles d'AMMO02 restent en revue. Aucun code de gameplay n'est modifié.

Proposition courante pour AMMO02 : [révision 2](../docs/reviews/refonte-agent-2026-10-06/review-universal-ammunition-AMMO02-02.json).
Cette révision remplace la première pour le message de refus uniquement ;
les manifestes historiques et leurs références restent conservés.

### AMMO-BOX01 et EQUIP-EXPAND01 — Boîte équipée et nouvelles séries

Le joueur remplace la gestion des quantités de munitions par une boîte de
munitions équipée dans un emplacement dédié de la silhouette. Sa présence
équipée permet l'usage des armes à distance, armes lourdes comprises. Ces
boîtes doivent être vendues et trouvées dans le loot, avec des versions
magiques. La synthèse est acceptée explicitement. Cette direction remplace
AMMO-UNIVERSAL01 et AMMO02 : les stocks initiaux de 40 munitions et les coûts
par projectile historiques ne doivent pas être implémentés pour la refonte.
Les autres équipements de départ et les coûts en énergie validés demeurent.
Les compétences d'armes à distance ordinaires doivent infliger légèrement
moins de dégâts que les compétences de mêlée comparables ; les compétences
d'armes lourdes sont exclues de cet ajustement. Les valeurs restent à revoir.

« Parfait :) » accepte les directions visuelles du focaliseur occulte à une
main, de la hampe de canalisation à deux mains et du bouclier métallique à
une main, avec plusieurs variétés comparables aux séries existantes. Aucun
nouveau pixel, texte RP de modèle ou statistique d'objet n'est accepté par
ce seul accord. La demande confirme aussi les cinq familles lourdes :
lance-flammes, lance-roquettes, Gatlings, lance-plasma et lance-grenades.

L'inventaire de production prend six modèles par série, comme les pistolets,
PM et séries de lanceurs déjà présents. Huit séries représentent 48 modèles
à créer ou à revoir : 18 pour les focaliseurs, hampes et boucliers, 30 pour
les cinq familles lourdes. Les trois armes de base supplémentaires existantes
sont conservées, ainsi que les IDs des 18 modèles de série de lanceurs.
Les Gatlings et lance-plasma ne renomment pas les mitrailleuses et fusils à
énergie existants. Les boîtes constituent un lot séparé, sans nombre de
modèles encore fixé. Aucun nouveau dessin n'est produit dans cet inventaire.

La revue devra montrer les icônes et objets tenus, avec les cinq corps fixes,
les prises gauche/droite, les deux mains et les combinaisons arme/bouclier,
à taille native 32 pixels et zoom entier. Les modèles doivent présenter des
formes reconnaissables et conserver des points d'attache cohérents.

Direction : [AMMO-BOX01](../docs/reviews/refonte-agent-2026-10-06/equipped-ammunition-box-direction-decisions-01.json).
Familles : [EQUIP-EXPAND01](../docs/reviews/refonte-agent-2026-10-06/caster-shield-heavy-expansion-directions-01.json).
Inventaire de production : [EQUIP-SERIES01](../docs/reviews/refonte-agent-2026-10-06/review-equipment-series-EQUIP-SERIES01-01.json).

Manifestes courants : statistiques **101**, catalogue principal **273**,
catalogue complet **248**, compteur de revue **142**, préparation **43**,
départs **11**. Les contrôles vérifient 1 868 références et la conservation
des profils de classe, compétences, validations antérieures et 113 modèles
existants. Les nouvelles directions s'appliquent par priorité explicite sur
les anciens dossiers ; aucun code de gameplay ou asset actif n'est modifié.

### HEAVY-POSE01 — Armes lourdes tenues à l'horizontale

Le joueur accepte le plan de six modèles pour chacune des huit séries,
puis précise que les poses des armes lourdes s'affichent principalement
à l'horizontale. Cette direction s'applique aux lance-flammes, lance-roquettes,
Gatlings, lance-plasma et lance-grenades. Les deux prises et les avant-bras
doivent être adaptés aux cinq silhouettes, avec des corps et proportions fixes.
Les directions des focaliseurs, hampes et boucliers ne changent pas.

L'orientation est acceptée ; les pixels, angles exacts et points d'attache
concrets devront encore être présentés en candidats numérotés à taille native.
Le rendu actif des armes existantes n'est pas modifié dans cette correction.

Direction : [HEAVY-POSE01](../docs/reviews/refonte-agent-2026-10-06/heavy-horizontal-pose-directions-01.json).
Inventaire courant : [EQUIP-SERIES01, révision 2](../docs/reviews/refonte-agent-2026-10-06/review-equipment-series-EQUIP-SERIES01-02.json).
Cette révision précise l'orientation des familles lourdes et remplace la
révision 1 pour la production des candidats. Les références historiques
des manifestes sont conservées.

### SHIELD-POSE01 — Bouclier à plat, centré sur la main

Le joueur demande une face de bouclier affichée à plat, dont le centre est
posé sur la main qui le porte. Le bouclier doit masquer cette main et une
partie de l'avant-bras. Sa face est donc dessinée devant ces parties du
personnage ; aucune main ou couche de doigts ne repasse devant le bouclier.
Les points de main des cinq silhouettes servent de références, avec une
vérification du centrage concret sur les futurs candidats natifs.

Cette direction concerne les six boucliers. Les poses horizontales des
armes lourdes et les autres directions de famille demeurent. Aucun dessin,
statistique ou comportement de défense n'est intégré par cette précision.

Direction : [SHIELD-POSE01](../docs/reviews/refonte-agent-2026-10-06/shield-flat-hand-centered-direction-01.json).
Inventaire courant : [EQUIP-SERIES01, révision 3](../docs/reviews/refonte-agent-2026-10-06/review-equipment-series-EQUIP-SERIES01-03.json).
Cette révision devient la référence pour produire les candidats de boucliers
avec la face devant la main. Les références précédentes sont conservées.

### SHIELD-VIS01 — Six candidats visuels de boucliers

Après l'accord sur la direction de pose, six dessins numérotés **EQ-S01 à
EQ-S06** sont préparés pour revue. Leurs sources imagegen sont conservées avec
les prompts ; les couches tenues et icônes sont conditionnées sur alpha binaire,
avec filtrage nearest et palette sans dithering. Les formes et couleurs concrètes
restent en attente d'une validation explicite.

La planche montre les cinq corps fixes à taille native et zoom entier. La galerie
permet de choisir la main du bouclier, le personnage, l'arme de l'autre main et le
fond. Les 60 placements de bouclier sont composés en 180 configurations : autre
main vide, couteau ou pistolet. Le centre est posé sur le point de main existant ;
la face couvre la main et une partie de l'avant-bras, sans doigts redessinés
par-dessus. Aucun pixel extérieur au masque du bouclier ne change.

Ces prototypes restent dans `artifacts/`. Ils ne créent pas encore de nouveaux
objets actifs, noms d'objets ou règles de défense. La validation de pose antérieure
est conservée séparément de celle des pixels, toujours à obtenir.

Revue : [SHIELD-VIS01, révision 1](../docs/reviews/refonte-agent-2026-10-06/shield-review-01/review-shields-SHIELD-VIS01-01.json).
Planche : [six candidats sur les cinq personnages](../artifacts/refonte-agent-2026-10-06/shield-review-01/shields-native-and-zoom.png).
Galerie : [boucliers, main et arme au choix](../artifacts/refonte-agent-2026-10-06/shield-review-01/shield-gallery-01.html).

### SHIELD-VIS01, révision 2 — Corrections industrielles des 04 et 06

Le joueur confirme que les boucliers sont bien placés, puis demande de revoir
**EQ-S04** et **EQ-S06**, jugés trop médiévaux. Le placement concret est donc
accepté séparément des dessins. Les pointes et croisillons de ces deux références
sont remplacés dans un nouveau prototype par des plaques et renforts industriels :
bord inférieur plat et traverse pour le 04, plaque rectangulaire et module latéral
pour le 06. Ces deux rendus modifiés attendent une nouvelle validation visuelle.

Les quatre autres références gardent exactement leurs pixels. Les contrôles
vérifient 120 compositions inchangées et conservent les points de main de toutes
les compositions. Les corps restent fixes et aucun pixel extérieur aux masques
des boucliers ne change. Les prototypes précédents restent disponibles ; aucun
asset ou objet actif n'est intégré.

Revue courante : [SHIELD-VIS01, révision 2](../docs/reviews/refonte-agent-2026-10-06/shield-review-02/review-shields-SHIELD-VIS01-02.json).
Retour du joueur : [portée du retour et corrections](../artifacts/refonte-agent-2026-10-06/shield-review-02/feedback-and-revision-02.json).
Planche des corrections : [EQ-S04 et EQ-S06](../assets/source-art/refonte-agent-2026-10-06/shield-review-02/shields-04-06-revised-native.png).
Galerie révisée : [six références, personnage et main au choix](../artifacts/refonte-agent-2026-10-06/shield-review-02/shield-gallery-02.html).

### SHIELD-VIS01, révision 4 — Construction du 06 et contours natifs

Le joueur demande de reconstruire toute la partie dorée du 06 dans le style
du reste du bouclier, et précise qu'une simple recoloration ne suffit pas.
Il signale aussi les contours peu visibles du bouclier cuivré et demande une
vérification des six dessins à petite taille. Le module latéral et sa grande
attache du 06 sont retirés : la plaque anthracite et le cadre métallique se
prolongent sur le côté droit. Cette nouvelle construction attend une validation
visuelle ; l'ancienne partie dorée ne reste pas sous une nouvelle couleur.

Les six références sont observées sur les cinq silhouettes natives et sur fond
sombre, clair et chargé. Les contours des 02 et 04 sont renforcés dans leurs
sources imagegen pour survivre à la réduction. Tous leurs pixels de bordure
natifs sont désormais sombres, avec les pièces d'acier à l'intérieur du contour.
Les 01, 03 et 05 restent identifiables et conservent exactement leurs pixels.
Les 90 compositions qui les concernent sont inchangées. Le centrage et les
corps fixes sont préservés dans les 180 compositions.

Une planche de diagnostic compare des cases de 30 et 32 pixels sur les trois
fonds. Le format actif reste 32 pixels : l'échelle de 30 pixels sert seulement
à éprouver la lisibilité. La révision 3 garde le premier essai de correction ;
la révision 4 utilise les contours renforcés présentés dans la galerie courante.
Les prototypes précédents restent conservés. Aucun asset actif ou effet de
défense n'est intégré par cette revue.

Revue courante : [SHIELD-VIS01, révision 4](../docs/reviews/refonte-agent-2026-10-06/shield-review-04/review-shields-SHIELD-VIS01-04.json).
Retour et correction : [portée des modifications](../artifacts/refonte-agent-2026-10-06/shield-review-04/feedback-and-revision-04.json).
Vérification native : [audit des six références](../artifacts/refonte-agent-2026-10-06/shield-review-04/native-readability-audit-04.json).
Planche : [comparaison à 30 et 32 pixels](../assets/source-art/refonte-agent-2026-10-06/shield-review-04/shields-readability-30-32.png).
Galerie : [six références révisées](../artifacts/refonte-agent-2026-10-06/shield-review-04/shield-gallery-04.html).

### SHIELD-VIS01, révision 7 — Cadre inférieur du EQ-S02

Le joueur signale que la bordure argentée du bas du 02 est trop fine et peu
lisible, et que la construction diffère trop entre haut et bas. Le nouveau
prototype élargit la bande métallique inférieure, son bord bas et les deux
raccords verticaux sous la traverse. La source conserve les panneaux olive et
la silhouette industrielle. L'aperçu compare le résultat à 30 et 32 pixels,
sur trois fonds, ainsi qu'à la révision 4 précédemment présentée.

Les révisions 5 et 6 conservent les essais intermédiaires. La 6 n'est pas retenue
pour la présentation finale : son contour et sa traverse se lisaient moins bien
après réduction. La révision 7 est le candidat courant, en attente d'accord sur
les pixels modifiés. Le placement déjà accepté reste centré sur la main.

Les cinq autres boucliers et leurs 150 compositions restent exactement
identiques à la révision 4. Les 180 compositions conservent leurs points de
main et leurs corps fixes. Aucun asset actif ou effet mécanique n'est intégré.

Revue : [SHIELD-VIS01, révision 7](../docs/reviews/refonte-agent-2026-10-06/shield-review-07/review-shields-SHIELD-VIS01-07.json).
Comparaison : [EQ-S02 avant et après, taille native](../assets/source-art/refonte-agent-2026-10-06/shield-review-07/EQ-S02-before-after-native.png).
Diagnostic : [EQ-S02 à 30 et 32 pixels](../assets/source-art/refonte-agent-2026-10-06/shield-review-07/EQ-S02-readability-30-32.png).
Galerie : [six références, correction du EQ-S02](../artifacts/refonte-agent-2026-10-06/shield-review-07/shield-gallery-07.html).

### SHIELD-VIS01-ACCEPT01 — Acceptation des boucliers courants

« Ok je valide on continue » accepte la galerie courante de révision 7 : six
boucliers et leurs placements, avec le cadre inférieur corrigé du EQ-S02.
L'accord sur les pixels est conservé séparément des noms et statistiques des
nouveaux objets, qui restent à définir. Les anciennes révisions restent intactes.

Accord : [SHIELD-VIS01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/shield-visual-acceptance-01.json).
La production se poursuit avec les six focaliseurs à une main EQ-F01 à EQ-F06.

### FOCAL-VIS01 — Six focaliseurs à une main

Les candidats **EQ-F01 à EQ-F06** sont dessinés selon la direction acceptée :
poignée courte, tête fourchue et cœur contrasté, avec des formes distinctes.
Les sources et prompts imagegen intégré sont conservés. La réduction utilise
une palette issue des sources, sans dithering, et un alpha binaire ; la tenue
du porteur ne recolore pas le focaliseur.

Chaque modèle est posé sur les cinq corps fixes, dans chaque main. Sa tête
suit le prolongement du vecteur coude-main propre au côté concerné, sans
nouveau bras ni nouvelle main. Les aperçus couvrent l'autre main vide, un
couteau, un pistolet, le même focaliseur ou chacun des six boucliers validés :
600 compositions, 60 prises principales. Les personnages restent natifs
32 × 32, avec des comparaisons sur trois fonds et un diagnostic à 30 pixels.

Ces nouveaux pixels et prises concrètes attendent la validation du joueur.
Cette revue ne définit pas les noms RP, statistiques ou règles d'attaque des
nouveaux objets. Elle n'impose pas de focaliseur pour pouvoir lancer un sort.
Les sources demeurent des prototypes dans `artifacts/`.

Revue : [FOCAL-VIS01, révision 1](../docs/reviews/refonte-agent-2026-10-06/focalizer-review-01/review-focalizers-FOCAL-VIS01-01.json).
Planche : [six focaliseurs sur les cinq silhouettes](../artifacts/refonte-agent-2026-10-06/focalizer-review-01/focalizers-native-and-zoom.png).
Prises : [les deux mains à taille native](../assets/source-art/refonte-agent-2026-10-06/focalizer-review-01/focalizers-both-hands-native.png).
Galerie : [main, personnage et bouclier au choix](../artifacts/refonte-agent-2026-10-06/focalizer-review-01/focalizer-gallery-01.html).

### HEAVY-MG01 — Mitrailleuses lourdes, dont gatlings

Le joueur élargit la série auparavant intitulée « Gatlings » à une famille de
**mitrailleuses lourdes**, qui pourra inclure des gatlings parmi ses modèles.
Le plan conserve six candidats et la pose horizontale à deux prises. Les IDs
de revue EQ-GA01 à EQ-GA06 sont conservés pour ne pas casser les références déjà
émises ; ces IDs techniques ne constituent pas des noms d'objets en jeu.

Cette direction ne remplace pas silencieusement les IDs, statistiques ou dessins
acceptés des mitrailleuses déjà existantes. Les prochains modèles concrets et
leurs éventuelles correspondances avec le contenu actif restent à présenter.

Direction : [HEAVY-MG01](../docs/reviews/refonte-agent-2026-10-06/heavy-machinegun-family-direction-01.json).
Inventaire courant : [EQUIP-SERIES01, révision 4](../docs/reviews/refonte-agent-2026-10-06/review-equipment-series-EQUIP-SERIES01-04.json).

### FOCAL-VIS01-ACCEPT01 — Six focaliseurs validés

Le joueur valide les modèles présentés dans `focalizer-gallery-01.html` :
« j'aime bien ces models, je les valides ». Cet accord porte sur les six
dessins EQ-F01 à EQ-F06 et les prises concrètes montrées sur les cinq
silhouettes, avec les combinaisons à une main de la révision 1.

La banque et ses empreintes restent inchangées ; un enregistrement distinct
conserve cet accord. Les noms d'objets, textes RP et statistiques restent à
définir séparément. La production se poursuit avec six hampes de canalisation
à deux mains EQ-H01 à EQ-H06, selon la direction déjà acceptée.

Accord : [FOCAL-VIS01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/focalizer-visual-acceptance-01.json).
Inventaire : [EQUIP-SERIES01, révision 5](../docs/reviews/refonte-agent-2026-10-06/review-equipment-series-EQUIP-SERIES01-05.json).

### SHAFT-VIS01 — Six hampes de canalisation à deux mains

Les candidats EQ-H01 à EQ-H06 suivent la direction acceptée : un manche
métallique continu, une tête industrielle compacte et deux prises distinctes.
Les six sources ont été dessinées avec imagegen intégré. Leur conditionnement
utilise une palette issue des sources, un alpha binaire et des transformations
nearest, sans redessin des personnages ou de leurs bras.

Les 30 compositions réutilisent exactement les bras et doigts de la pose à
deux mains déjà validée de la famille 2. Les points de prise sont (32, 38) et
(35, 34) dans un canevas de 64 pixels contenant le corps natif 32 × 32. La
palette de chaque arme reste indépendante de celle du porteur. Aucun asset
actif, nom d'objet, texte RP ou effet mécanique n'est ajouté.

Ces six nouveaux dessins et leurs prises concrètes restent à valider. La
galerie propose les cinq personnages et trois fonds. Une planche native, un
diagnostic à 30/32 pixels et six captures de navigateur sont conservés. Les
contrôles vérifient 205 références, les deux contacts de main par composition,
la continuité des sources et l'identité des couches de bras/doigts réutilisées.
Les captures du navigateur sont des preuves de présentation ; leur échelle
physique dépend du fournisseur. Les planches d'art conservent l'échelle native.

Revue : [SHAFT-VIS01, révision 1](../docs/reviews/refonte-agent-2026-10-06/shaft-review-01/review-shafts-SHAFT-VIS01-01.json).
Planche : [six hampes sur les cinq silhouettes](../artifacts/refonte-agent-2026-10-06/shaft-review-01/shafts-native-and-zoom.png).
Galerie : [personnage et fond au choix](../artifacts/refonte-agent-2026-10-06/shaft-review-01/shaft-gallery-01.html).
Prompts : [sources imagegen intégré](../docs/reviews/refonte-agent-2026-10-06/shaft-review-01/prompts.json).

### SHAFT-VIS01-ACCEPT01 — Six hampes validées ; essai BD demandé

Le joueur valide les six ajouts présentés dans `shaft-gallery-01.html` :
« okay bon Je valide ces ajouts ». Un accord distinct conserve les dessins
EQ-H01 à EQ-H06 et leurs prises concrètes à deux mains. La banque émise reste
inchangée. Les noms d'objets, textes RP et statistiques restent à définir.

Le joueur demande ensuite une planche des personnages et armes dans un style
bande dessinée. Il souhaite tester cette direction et ne passera à ce style
partout que s'il lui plaît. BD-STYLE01 est donc une étude visuelle distincte,
à présenter en grand et à l'échelle des tiles ; elle ne migre pas les assets
actifs et ne remplace pas les validations des séries précédentes.

Accord : [SHAFT-VIS01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/shaft-visual-acceptance-01.json).
Inventaire : [EQUIP-SERIES01, révision 6](../docs/reviews/refonte-agent-2026-10-06/review-equipment-series-EQUIP-SERIES01-06.json).

### BD-STYLE01 — Planche d'essai bande dessinée

Cinq études équipées sont proposées : BD-C01 Bastion avec lame et bouclier,
BD-C02 Technomancien avec pistolet, BD-C03 Traqueur avec fusil de précision,
BD-C04 Arcaniste avec hampe de canalisation et BD-C05 Métamorphe avec couteau
et focaliseur. Les classes et couleurs de référence guident les dessins,
avec un rendu BD : contours encrés, aplats et ombres simples. Ces nouveaux
pixels attendent l'avis du joueur.

Les cinq sources transparentes sont dessinées avec imagegen intégré et
conservées avec leurs prompts. La planche présente les personnages en grand,
des détails de leurs prises et un comparatif actuel/BD à 30 et 32 pixels.
Les petites vues sont une première réduction LANCZOS des sources, avec
antialiasing et sans limitation de palette. Leur échelle utilise des repères
sur le corps, afin de conserver les débordements des armes. Le Technomancien
est réfléchi horizontalement en entier pour retrouver le pistolet côté droit
de l'image ; la source brute demeure conservée.

La galerie compare les cinq candidats sur trois fonds. Les dimensions CSS des
vues natives et du zoom ×2 sont vérifiées, ainsi que 53 références de fichiers
et les dix variantes natives sans rognage de l'équipement. Les captures du
navigateur conservent leur échelle de fournisseur et sont assemblées sans
redimensionnement. La planche exportée reste disponible séparément.

Il s'agit de dessins complets équipés destinés à juger la direction ; ils ne
constituent pas encore une banque de couches corps/bras/armes prête au moteur.
Aucun asset actif ni effet mécanique n'est modifié. Le passage global en BD
reste conditionné à l'avis du joueur sur cet essai.

Revue : [BD-STYLE01, révision 1](../docs/reviews/refonte-agent-2026-10-06/comic-style-review-01/review-BD-STYLE01-01.json).
Planche : [cinq personnages, équipements et comparaison native](../artifacts/refonte-agent-2026-10-06/comic-style-review-01/comic-style-board-01.png).
Galerie : [vues BD, style actuel et fonds au choix](../artifacts/refonte-agent-2026-10-06/comic-style-review-01/comic-style-gallery-01.html).
Prompts : [cinq dessins imagegen intégré](../docs/reviews/refonte-agent-2026-10-06/comic-style-review-01/prompts.json).


### BD-STYLE01-RULES02 — qualité acceptée, gabarit et prises conservés

Le joueur accepte la qualité du dessin BD, mais demande des personnages plus présents dans les 32 px et rappelle que les armes peuvent déborder. Sa clarification confirme que les règles validées de création et de placement de l’équipement restent applicables : seul le style de dessin change. L’échelle ne doit jamais dépendre de l’encombrement des armes. Les corps fixes, couches séparées, prises et orientations validées restent la référence.

Les illustrations complètes de la première planche ne constituent pas une banque modulaire vérifiée. Les nouvelles études ayant modifié les proportions sont conservées comme essais, sans intégration ni validation de leurs géométries. Le souhait de différencier casques et tenues est noté ; aucune nouvelle identité concrète n’est acceptée par cet accord sur la qualité. Le prochain rendu doit être présenté à taille native avec contrôle des couches et des points d’attache.

Trace et audit : [BD-STYLE01-RULES02](../docs/reviews/refonte-agent-2026-10-06/comic-style-review-02/rules-and-feedback-02.json).

### BD-CATALOGUE01 — catalogue BD par modèle, sans validation ni intégration

Le joueur précise « pour l'instant je ne valide rien » et demande de refaire
toutes les armes et armures avant de les associer aux personnages, en grand et
à 32 px. Il choisit explicitement **un dessin distinct par modèle d'objet**.
La précédente possibilité d'un dessin tenu commun par famille ne définit donc
pas le nombre de dessins de cet essai. La banque active du jeu reste inchangée.

Le périmètre de revue compte 178 candidats : 113 modèles d'armes actuels,
32 objets d'armure, trois tenues de base, six boucliers, six focaliseurs,
six hampes, six nouvelles mitrailleuses lourdes comprenant des gatlings et
six lance-plasma. Les séries existantes de lance-flammes, lance-roquettes et
lance-grenades sont associées à leurs IDs actuels, sans doublonner les modèles.
BD-W001 à BD-W113 et BD-A001 à BD-A032 gardent leurs identités de contenu.
Les nouvelles séries EQ conservent leurs numéros de revue et n'ajoutent ni
noms définitifs ni statistiques.

Chaque dessin est généré séparément avec imagegen intégré. Les tenues sont
des corps non armés ; les armes, boucliers, casques, gants et bottes sont des
sources séparées. Les compositions de revue réutilisent les corps fixes,
points de main et de coude, poses à deux mains et doigts de premier plan.
Le corps utilise une hauteur de 32 px déterminée indépendamment des armes.
Les boucliers restent de face, centrés sur la main, devant celle-ci.

Le joueur signale ensuite que les armes à distance ordinaires sont trop
grosses par rapport aux personnages. Les compositions des pistolets,
pistolets-mitrailleurs et fusils ordinaires sont réduites ; les prises restent
en contact et les armes lourdes conservent leur gabarit plus imposant.
Cette correction de proportion fait partie des candidats à examiner.

Tous ces nouveaux pixels restent **en attente de validation artistique**.
Les contrôles de couverture, d'empreintes et d'attache ne constituent pas une
acceptation du dessin, de la lisibilité ou d'une migration globale du style.
Aucun texte de gameplay, effet mécanique, équipement actif ou sauvegarde
n'est modifié par cette revue.

Galerie : [BD-CATALOGUE01](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-01/index.html).
Catalogue : [178 candidats et identités conservées](../docs/reviews/refonte-agent-2026-10-06/comic-catalogue-review-01/catalogue.json).
Prompts réellement utilisés : [génération intégrée par modèle](../docs/reviews/refonte-agent-2026-10-06/comic-catalogue-review-01/prompts-used.json).
Contrôles : [couverture, provenance et banque active inchangée](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-01/coverage-and-provenance.json).

### BD-REPAIR02 — rejet du montage du catalogue et reprise des couches

Le joueur rejette le rendu composé de BD-CATALOGUE01 : textures d'armures
dégradées, mains formant des carrés noirs, armes masquées par les tenues et
proportions d'armes incorrectes. La galerie précédente demeure conservée comme
comparaison ; aucun de ses candidats ne reçoit une validation par ce retour.

L'audit trouve quatre erreurs de composition : les ombres sombres de l'ancien
corps pixelisé sont réinjectées dans le nouveau dessin ; les mains de l'ancien
atlas sont agrandies sur des manches BD ; certaines armes à une main passent
derrière le corps entier ; les dimensions des fusils dépendent de prises
devinées sur les sources plutôt que de leur gabarit et de leurs vraies poignées.

Sept exemples BD-REPAIR02-01 à 07 reprennent trois tenues et quatre modèles
d'équipement depuis leurs sources intactes, sans nouvelle génération d'image.
Les régions de bras et de gants viennent du dessin de chaque tenue. Les points
de main, coude et poignée sont mesurés sur ces sources ; les aplats ne sont
plus découpés par l'ancienne silhouette pixelisée. La largeur et la hauteur
du porteur sont recalées sur les proportions de sa référence existante.
Les gabarits d'arme sont explicites et indépendants de l'écartement des prises.
Une arme reste devant le torse, avec le gant au-dessus de la poignée ; le
bouclier recouvre la main et l'avant-bras.

Ces exemples restent des prototypes à examiner, en grand et à taille native,
sur terrains et fonds clair/sombre. La pose des doigts est issue des gants
existants de ces sources ; elle ne constitue pas une nouvelle prise dessinée
et validée. Les autres modèles du catalogue n'ont pas encore reçu ce recalage
individuel. Aucun de ces essais ne remplace la banque active du jeu.

Galerie : [comparaison BD-REPAIR02](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-02/index.html).
Planche : [avant et après, grand et natif](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-02/before-after-board.png).
Mesures : [points d'attache par source](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-02/source-registration.json).
Statut et limites : [revue des sept exemples](../docs/reviews/refonte-agent-2026-10-06/comic-catalogue-review-02/review.json).

### BD-STYLE01-REFERENCE03 — retour à la première planche et affichage ×2 autorisé

Le joueur signale encore des aplats noirs, des armes derrière les tenues et des
placements incorrects. Il rappelle que sa demande porte sur le redessin de
l'existant, avec le seul changement du style. Les montages de BD-CATALOGUE01 et
les corrections de BD-REPAIR02 ne sont donc pas une référence acceptée pour la
reprise, malgré leurs contrôles techniques.

Il fournit la première planche BD-STYLE01 et sa source du Métamorphe, puis
précise : « les version \"BD zoom x2\" sont les meilleurs ». La référence de
qualité est celle des cinq dessins BD-C01 à BD-C05 de `comic-style-review-01`,
avec leurs contours encrés, aplats et ombres simples. Les compositions doivent
conserver les poses, proportions, points d'attache et placements de référence.

Le joueur autorise ensuite le passage à des cases affichées en **64 px**, si
nécessaire pour conserver le rendu « BD zoom ×2 ». La galerie confirme qu'il
s'agit des mêmes dessins réduits pour 32 px, agrandis par deux en nearest : le
canevas de 64 px couvrant deux cases est affiché sur 128 px. Cette autorisation
concerne l'échelle d'affichage ; elle ne modifie pas les cases logiques ni les
règles de jeu et ne valide pas les nouveaux modèles du catalogue.

Cette mise à jour consigne la référence et l'autorisation. Elle n'intègre aucun
dessin et ne change pas encore les réglages actifs du jeu.

Trace : [BD-STYLE01-REFERENCE03](../docs/reviews/refonte-agent-2026-10-06/comic-style-reference-03.json).

### BD64-REFERENCE04 — aperçu fidèle et taille 64 px disponible

Après « ok vas-y », les cinq réductions originales de BD-STYLE01 sont reprises
sans redessin ni nouveau montage. Chaque pixel RGBA est répété exactement en
un bloc de 2 × 2 pixels, afin de retrouver le rendu « BD zoom ×2 ». Les prises,
poses, contours, ombres et proportions sont ceux de la première planche.
Les armes restent présentes dans leurs illustrations de référence ; aucun
ancien bras ou masque pixelisé n'est ajouté.

Une galerie montre le personnage choisi sur le sol du jeu et sur fonds clair
et sombre. L'affichage se compare en 32 et 64 px. Le panneau conserve une
échelle de un pixel de canevas pour un pixel CSS : un panneau étroit affiche
moins de cases, sans réduire le personnage. La planche séparée montre les cinq
sources en grand et aux deux tailles ; une scène rassemble les cinq vues en 64 px.

Le réglage « Taille des cases » du jeu accepte désormais **64 px**, avec
conservation des autres réglages et du choix par défaut existant. Les contrôles
de lecture/enregistrement et de coordonnées souris sont étendus à cette taille.
Quinze tests ciblés distincts et la compilation de développement réussissent.
Les mesures de navigateur vérifient le chargement des dix petites vues, leurs
dimensions CSS et l'absence de débordement horizontal dans le panneau étroit.

Le contrôle des 81 920 pixels agrandis et des quinze références originales
confirme une transformation limitée à l'échelle ×2. Les huit fichiers de la
banque active de personnages restent identiques. Cet aperçu artistique ne
constitue pas une capture de gameplay ni une correction du catalogue complet.
Les illustrations équipées ne remplacent pas les futures couches modulaires.

Galerie : [BD64-REFERENCE04](../artifacts/refonte-agent-2026-10-06/comic64-reference-review-04/index.html).
Planche : [dessins d'origine, vues 32 et 64 px](../artifacts/refonte-agent-2026-10-06/comic64-reference-review-04/reference-32-64-board.png).
Trace : [sources et limites de l'aperçu](../docs/reviews/refonte-agent-2026-10-06/comic64-reference-review-04/review.json).

### BD-CATALOGUE03 — catalogue complet et corrections des prises

Le catalogue de revue comprend 178 dessins distincts : 137 armes, 6 boucliers,
17 tenues de torse et 18 pièces de tête, mains ou pieds. Les armes et boucliers
sont composés sur les cinq tenues canoniques ; les 17 tenues sont aussi
présentées individuellement. Les autres pièces ont leur illustration propre
dans l'inventaire. Cela représente 822 compositions, sans modification des
identités de contenu ni des règles de jeu.

Le joueur confirme que le torse définit toute la tenue, casque compris. Les
pièces de tête, mains et pieds conservent leurs slots et leurs effets sans
remplacer une partie de cette tenue sur le personnage. Les poses reprennent
seulement les bras nécessaires depuis les dessins sources ; elles conservent
le corps et le casque de référence. Les armes restent devant le torse et les
doigts recouvrent leur poignée.

Après le signalement d'armes inversées, la direction des armes à feu est
corrigée par réflexion horizontale, afin de garder leurs organes de visée
au-dessus et leurs chargeurs au-dessous. Les armes de mêlée à deux mains
sont réduites ; la longueur réelle des armes d'hast est mesurée le long de
leur diagonale pour éviter la lance démesurée et coupée montrée par le joueur.

Les prises BD-GRIP01 (Bastion) et BD-GRIP04 (Arcaniste) sont de nouveaux
candidats dessinés pour les pistolets et pistolets-mitrailleurs. Le bras est
orienté vers l'extérieur et le canon suit son axe. Douze modèles sont repris
sur ces deux tenues. Les autres parties du personnage restent celles de la
référence. Ces prises sont à examiner ; la demande de correction ne vaut
pas acceptation du nouveau rendu.

Les contrôles vérifient les 822 agrandissements exacts de 32 vers 64 px,
l'absence de découpe au bord des compositions, 90 compositions dont les
pièces hors torse laissent le corps intact et 17 comparaisons de casque
entre poses. Les quinze références BD-STYLE01 et les huit fichiers actifs
de la banque du jeu restent identiques. Le navigateur confirme le chargement
des aperçus et leur affichage sans réduction CSS ni débordement horizontal.
Ces vérifications techniques ne remplacent pas la revue artistique.

Galerie : [BD-CATALOGUE03](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-03/index.html).
Catalogue : [identités et compositions](../docs/reviews/refonte-agent-2026-10-06/comic-catalogue-review-03/catalogue.json).
Contrôles : [validation](../docs/reviews/refonte-agent-2026-10-06/comic-catalogue-review-03/validation.json).
Prises : [sources et prompts](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-03/sidearm-pose-provenance.json).
Captures : [Bastion avec PM](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-03/audit/browser-bastion-smg.png),
[Arcaniste avec PM](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-03/audit/browser-arcaniste-smg.png).

### BD-FRONT01 — comparaison du Bastion en vue de face

Le joueur rappelle que les personnages d'origine sont de face et demande
pourquoi les essais BD ont une orientation de trois quarts. Un Bastion
frontal est préparé avec la même armure, sans arme, avant une éventuelle
reprise des autres tenues. Sa hauteur reste de 29 pixels natifs dans une
case logique de 32 pixels. La comparaison présente l'ancien dessin et le
candidat en grand, en 32 px et en agrandissement exact à 64 px.

Ce candidat n'est ni validé ni étendu aux autres personnages. Tous les
nouveaux dessins de BD-CATALOGUE03 et BD-FRONT01 restent des ressources de
revue ; aucun ne remplace les assets actifs du jeu.

Comparaison : [BD-FRONT01](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-03/front-bastion.html).
Source et prompt : [provenance frontale](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-03/front-pose-provenance.json).
Capture : [comparaison aux tailles de jeu](../artifacts/refonte-agent-2026-10-06/comic-catalogue-review-03/audit/browser-front-bastion-native.png).

### BD-FLAT01 — gabarit frontal partagé, puis correction des prises

Le joueur choisit des personnages strictement de face et un même gabarit de
bras pour toutes les tenues. Le premier prototype présente le Bastion ivoire
et la robe rouge, avec 16 configurations et les 12 modèles de pistolets/PM
individuels : 56 compositions. Les armes utilisent exactement les mêmes
couches et transformations sur les deux tenues. Le torse conserve son dessin
complet, casque et bottes compris ; seuls les membres de la pose changent.

Le joueur indique que le reste fonctionne bien, mais demande une correction
de la mêlée à deux mains : main gauche vers le haut, main droite vers le bas.
Il demande aussi que la partie de l'arme tenue disparaisse sous les doigts.
Ces directions sont enregistrées ; les nouveaux dessins de la révision 2
restent des candidats à examiner.

Deux corrections de mains sont dessinées avec l'outil imagegen intégré.
Le compositeur reprend uniquement leurs gants et raccords de poignets, sous
des masques explicites. Le reste des sources de bras reste celui de la
révision 1. Les silhouettes de gants sont placées devant les poignées : aucun
carré noir ni ancien masque pixelisé ne vient remplacer une main.

Les 56 agrandissements de 32 vers 64 px sont exacts. Les 16 configurations
gardent leurs placements d'armes et le gabarit de la révision 1 ; les deux
vues sans arme restent identiques. Le contrôle vérifie 52 recouvrements de
prise et l'absence de composition coupée. La région de tête haute résolution
et la région native du casque restent identiques ; la colonne native 36,
ligne 23, appartient à l'épaule et peut recevoir une contribution Lanczos
d'une unité depuis le gant relevé. Les huit fichiers actifs restent identiques.

Cette étape corrige le prototype de revue. Les autres tenues, les nouvelles
robes et l'identité visuelle du Métamorphe restent à traiter après l'accord
sur ce gabarit. Aucun nouveau dessin BD ne remplace les assets actifs.

Galerie : [BD-FLAT01, révision 2](../artifacts/refonte-agent-2026-10-06/flat-rig-review-02/index.html).
Contrôles : [validation des prises](../docs/reviews/refonte-agent-2026-10-06/flat-rig-review-02/validation.json).
Sources et prompts : [corrections locales des mains](../docs/reviews/refonte-agent-2026-10-06/flat-rig-review-02/grip-corrections-prompts.json).
Captures de la galerie : [prise de mêlée à deux mains](../artifacts/refonte-agent-2026-10-06/flat-rig-review-02/audit/browser-melee-grips-native.png),
[poignée du pistolet](../artifacts/refonte-agent-2026-10-06/flat-rig-review-02/audit/browser-pistol-grip-native.png).

### ARMOR-FLAT01 — catalogue complet, variantes et propositions d'armures

Le joueur accepte `BD-FLAT01`, révision 2, et demande son extension à toutes
les armures. Il souhaite aussi plusieurs nouvelles propositions de tenues de
mage et de combat, ainsi qu'une refonte de la tenue du Métamorphe. Les armures
existantes doivent être montrées et numérotées pour qu'il puisse désigner
celles à supprimer. Aucune suppression n'est demandée par identifiant à ce stade.

La revue présente 17 tenues actuelles (`AR-01` à `AR-17`), six nouveaux dessins
(`AR-N01` à `AR-N06`) et 18 illustrations de pièces séparées (`AR-P01` à
`AR-P18`). Les six propositions comprennent deux tenues de mage, deux de combat
et deux alternatives à `AR-03`, la tenue actuelle du Métamorphe. Leurs libellés
sont des repères de revue, pas des noms de nouveaux objets validés en jeu.
Les correspondances avec les anciens numéros et identifiants de contenu sont
conservées dans le catalogue.

Chaque tenue complète a trois palettes proposées. Le joueur précise ensuite
qu'il ne souhaite pas de variantes pour les casques, gants et bottes, puisque
ces pièces séparées ne sont pas affichées sur le plateau. Elles gardent leur
unique illustration d'inventaire. Les tenues sont présentées dans
les 16 configurations du gabarit accepté, en illustration agrandie, à taille
native 32 px et en agrandissement exact 64 px. Le torse continue à définir
l'apparence entière du personnage ; les casques, gants et bottes séparés
gardent leur illustration d'inventaire, sans changer ce corps.

Le joueur ajoute une direction future : les armures auront des prérequis de
statistiques inspirés de Diablo II. Les attributs concernés et leurs valeurs
restent à définir. Une couleur différente ne modifie pas ces prérequis.

**Accord acquis :** gabarit frontal de révision 2, extension à toutes les
tenues, plusieurs variantes de couleur par modèle et préparation des nouveaux
candidats. **À examiner :** adaptations concrètes, nouvelles propositions et
palettes. **À définir :** liste des modèles à retirer et valeurs des prérequis.
Ce lot prépare la revue et ne remplace aucun dessin dans le jeu actif.

Galerie : [ARMOR-FLAT01](../artifacts/refonte-agent-2026-10-06/armor-flat-review-01/index.html).
Catalogue : [modèles et identifiants](../docs/reviews/refonte-agent-2026-10-06/armor-flat-review-01/review.json).
Prompts : [dessins imagegen intégrés](../artifacts/refonte-agent-2026-10-06/armor-flat-review-01/generation-specs.json).
Contrôles : [composition et palettes](../docs/reviews/refonte-agent-2026-10-06/armor-flat-review-01/validation.json).
Capture de revue : [Métamorphe, palettes et tailles natives](../artifacts/refonte-agent-2026-10-06/armor-flat-review-01/audit/browser-metamorphe-native.png).

### ARMOR-FLAT02 — diversité des silhouettes et cinq départs légers

Le joueur apprécie la qualité du Manteau de mutation `AR-N06`, mais demande
de vraies différences entre les armures, notamment leurs casques. L'accord
sur les articulations communes reste acquis ; il ne justifie pas d'utiliser
une même forme de tête, de plastron, de pantalon ou de bottes partout.

Le joueur a aussi demandé cinq familles de progression : légère de magicien,
légère, moyenne, lourde et très lourde. Les cinq classes doivent commencer
avec un équipement plus modeste et léger. Les anciennes silhouettes avancées
peuvent être conservées pour des armures ultérieures.

**Retraits désignés :** `AR-02`, `AR-04`, `AR-13`. **Révisions particulières :**
casque de `AR-08`, palettes de `AR-10` et `AR-11`, suppression ou refonte
complète de `AR-12`. Les anciens numéros ne sont pas réattribués.

La nouvelle revue propose cinq départs légers `AR-N07` à `AR-N11` et trois
armures de progression `AR-N12` à `AR-N14`. `AR-N06` est conservé comme
référence de qualité, sans redessin. Les nouvelles tenues sont examinables
avec les mêmes armes, poses et trois palettes, en 32 px et 64 px.

**Accord acquis :** liberté de varier les silhouettes et les matières,
progression d'armures et départs légers. **À examiner :** chacun des huit
dessins, ses palettes et son libellé. L'appréciation de la qualité de `AR-N06`
n'intègre pas ses règles ou ses palettes. Les contenus actifs restent inchangés.

Galerie : [ARMOR-FLAT02](../artifacts/refonte-agent-2026-10-06/armor-flat-review-02/index.html).
Prompts : [huit dessins imagegen](../artifacts/refonte-agent-2026-10-06/armor-flat-review-02/generation-specs.json).

Contrôles de cette revue : 432 agrandissements exacts, 702 recouvrements
de prise vérifiés, aucun canevas coupé, régions de tête et de bas du corps
fixes entre poses, mêmes transformations d'armes et mêmes articulations.
Les 48 compositions de `AR-N06` restent identiques à la revue précédente.
Les huit ressources actives protégées restent identiques. Cela vérifie la
composition proposée ; aucune intégration ni validation de gameplay n'est faite.

La sélection des manches amples de `AR-N10` emploie des polygones source
explicitement adaptés au tissu pendant le conditionnement. Elle ne déplace
ni les articulations ni les armes. Les rampes des tissus sombres sont déclarées
par source afin de produire trois variantes réellement distinctes.

Contrôles : [composition](../docs/reviews/refonte-agent-2026-10-06/armor-flat-review-02/validation.json),
[galerie](../docs/reviews/refonte-agent-2026-10-06/armor-flat-review-02/audit/browser-validation.json).
Captures : [ensemble des silhouettes](../artifacts/refonte-agent-2026-10-06/armor-flat-review-02/audit/browser-overview.png),
[robe et prise à deux mains, tailles de jeu](../artifacts/refonte-agent-2026-10-06/armor-flat-review-02/audit/browser-robe-native.png).

### ARMOR-FLAT02-ACCEPT01 — huit tenues acceptées

Le 10 octobre 2026, « Je les valides et on continue :) » accepte les dessins
`AR-N07` à `AR-N14`, leurs trois palettes et les compositions montrées avec
le gabarit frontal. Les sources, palettes et aperçus acceptés sont figés par
empreintes dans [l'accord visuel](../docs/reviews/refonte-agent-2026-10-06/armor-flat-review-02/visual-acceptance-01.json).
Les noms définitifs et les statistiques ne sont pas déduits de cet accord.
`AR-N06` reste la référence appréciée déjà enregistrée. Les quatre corrections
du catalogue précédent seront soumises dans `ARMOR-FLAT03`.

### ARMOR-FLAT03 — corrections et artefacts de l'armure très lourde

**À examiner :** casque cuivré uniforme de `AR-08`, référence pétrole/acier
de `AR-10`, remplacement cyan/rouge par acier bleuté/ivoire de `AR-11`,
refonte futuriste complète de `AR-12`. Les noms et ID de ces quatre objets
restent ceux du catalogue. Trois palettes et seize configurations sont montrées.

Le joueur signale ensuite des artefacts jaunes sur l'armure très lourde
`AR-N14`. Les ombres jaunes étaient partiellement exclues de la recoloration.
La correction conserve son dessin, la référence jaune, les articulations,
les prises et les régions neutres ; seules les variantes peintes sont corrigées.
L'accord antérieur sur sa silhouette reste valable. Son rendu corrigé est
présenté dans cette revue ; les autres fichiers acceptés restent figés.

Les edits de casque et propositions de couleurs et de technicien sont issus
de l'outil imagegen intégré. Les palettes utilisent leurs cartes de matériaux
éditables ; la géométrie d'AR-10 et AR-11 est conservée exactement. L'extraction
du casque AR-08 conserve les vêtements précédents. Les images restent dans
la banque de revue, avant intégration active.

Galerie : [ARMOR-FLAT03](../artifacts/refonte-agent-2026-10-06/armor-flat-review-03/index.html).
Prompts : [dessins et couleurs](../artifacts/refonte-agent-2026-10-06/armor-flat-review-03/generation-specs.json).
Correction : [affectation du jaune](../artifacts/refonte-agent-2026-10-06/armor-flat-review-03/palette-artifact-correction.json).
Contrôles : [composition](../docs/reviews/refonte-agent-2026-10-06/armor-flat-review-03/validation.json).

Contrôle de la galerie : cinq fiches, recherche par ID, configuration lourde
et tailles CSS exactes vérifiées, aucune image chargée introuvable. La capture
[armure très lourde corrigée](../artifacts/refonte-agent-2026-10-06/armor-flat-review-03/audit/browser-very-heavy-native.png)
montre la grande vue et les trois palettes en 32 et 64 px ;
[prise avec arme lourde](../artifacts/refonte-agent-2026-10-06/armor-flat-review-03/audit/browser-very-heavy-weapon.png)
montre les mains et le placement horizontal. Le contrôle de composition couvre
240 agrandissements exacts, 390 recouvrements de prise et aucune coupure.
