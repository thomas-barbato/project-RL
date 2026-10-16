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
| B00 à B02 | À valider | À valider | À présenter séparément |
| B03 | À valider | +10 % de vie au départ retenu ; progression à valider | À présenter séparément |

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

> Améliore votre précision avec les armes et votre esquive.

**Effet révisé proposé :** chaque point autour de 5 ajoute ou retire
2 unités de précision avec les armes et 2 unités d'esquive. Agilité 8
donne +6 à chacune ; Agilité 4 donne −2. Ce sont des valeurs de combat,
pas des points de pourcentage directement ajoutés à une probabilité.

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

> Vous repérez les dangers de plus loin et visez mieux à distance.

**Effets proposés :**

- Vision de base : 8 cases à Perception 5, puis une case supplémentaire
  tous les dix points au-dessus de 5. La distance obtenue est bornée entre
  4 et 16 cases : clamp(8 + floor((Perception − 5) / 10), 4, 16).
  Perception 4 donne 7 cases ; Perception 15 donne 9 cases ; 85 donne 16.
- Précision à distance : +1 unité par point au-dessus de 5, et −1 par
  point en dessous. Cette contribution s'ajoute à celle de l'Agilité,
  y compris lorsque la vision a atteint sa distance maximale.

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

Les altérations physiques, les dégâts reçus et la résistance aux
interruptions physiques ne reçoivent pas tous ce bonus. Les transformations
et sorts précis feront l'objet de leurs propres fiches.

## Énergie et récupération

Les compétences technologiques et magiques partagent une réserve. Le nom
« Énergie » et les barèmes suivants sont proposés pour cette réserve.

### E01 Énergie

> Sert à utiliser vos pouvoirs magiques et technologiques.

**Capacité proposée :** 30 + 3 × (le plus élevé d'Intelligence et de Volonté
− 5) + 2 × (niveau − 1), avant équipement et implants. Intelligence et
Volonté ne s'additionnent pas dans ce calcul. Un personnage avec
Intelligence 4 et Volonté 6 commence donc à 33, avant son implant.

**Régénération révisée proposée :** 3 % de l'énergie maximale par tour
hors combat, avec un minimum de 3 points ; 1 % en combat, avec un minimum
de 1 point. Arrondi supérieur, gain limité à ce qui manque. Une réserve
de 33 récupère toujours 3 ou 1 ; une réserve de 300 récupère 9 ou 3.
Consulter un menu ou attendre en temps réel ne rend rien.

Pour cette proposition, le combat dure tant qu'un ennemi hostile est
actuellement visible ou qu'une attaque du joueur ou une attaque reçue
remonte à moins de trois tours. La disparition d'un ennemi du champ de
vision ne met donc pas immédiatement fin au combat. Un ennemi caché et
inconnu ne doit pas modifier la récupération et révéler sa présence.

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
donc Frappe de rupture et Endurance de combat ; Égide occulte s'apprend ensuite.

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

**Couleur proposée, à présenter en aperçu :** cyan pour « Niveau 13 »
lorsqu'il est augmenté ; couleur neutre habituelle autrement. Le détail
des niveaux appris et ajoutés reste visible au survol ou dans la fiche.
La condition est retenue ; la palette exacte et le rendu restent à valider.

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
pour commencer avec un seul bonus de 10 %. Cet ajustement de l'implant
attend un accord distinct. Le malus d'énergie s'applique au maximum issu
des attributs et du niveau ; les bonus d'équipement s'ajoutent ensuite.
Les arrondis de réserves vont vers l'entier inférieur.

Au niveau 1, avec Vitalité 7, Intelligence 4 et Volonté 6, le passif B03
donne 41 PV maximum au lieu de 38 ; l'implant donne 29 points d'énergie
maximum au lieu de 33.
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

### B03 Endurance de combat

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
l'Arcaniste et du Métamorphe seront présentées avec la même séparation
entre texte, paramètres et limites. Les quatre familles de magie,
les drones, les cinq transformations, les implants trouvés, les portails,
les objectifs de quêtes et leurs messages auront ensuite leurs propres
fiches. Chaque actif et passif présentera sa progression jusqu'à 10 et
au-delà par l'équipement. Leurs nouveaux effets visuels devront être
montrés en situation. Les taux de loot, les rangs de bonus trouvables et
la courbe de difficulté devront être validés et testés avec cette progression.
