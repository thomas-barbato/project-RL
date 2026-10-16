# Cadrage complet des compétences de l'agent

6 octobre 2026. Le joueur demande de terminer le cadrage avant de reprendre
les descriptifs et de décider ce qu'il garde ou modifie. Ce document achève
les propositions de règles des neuf branches restantes et complète les
paramètres des trois arbres secondaires. Il ne crée aucun nouveau texte
roleplay, dessin, animation ou code de compétence.

Les noms, rôles et intentions déjà acceptés sont conservés. Chaque nouveau
chiffre et cas de fonctionnement demeure une proposition pour la revue
suivante. Les accords antérieurs sur des textes ou tableaux précis restent
valables ; achever ce document ne vaut pas approbation de son contenu.

## Références, rangs et attributs

Le catalogue principal contient 77 compétences dans 13 branches. Mêlée,
Survie, Drones et Affaiblissements possèdent déjà leurs lots de travail,
référencés ci-dessous. Les 51 autres compétences ont ici leurs paramètres.
DI06 Tir de couverture, ES02 Roulade et ES04 Riposte d'esquive sont retirés sans remplacement à la
demande du joueur le 7 octobre ; leurs propositions précédentes restent
dans l'historique de revue. Pas spectral conserve ses règles validées ;
aucun effet de Roulade ne lui est transféré et l'animation de Roulade est
retirée du périmètre actif.
La branche Foudre et ses six compétences FO01 à FO06 sont retirées à la
demande du joueur le 7 octobre, sous AR-FO-REMOVE01, sans remplacement ni
transfert d'effets. Les anciennes revues restent archivées ; les identifiants
ne sont pas réutilisés.
Les 14 compétences secondaires sont distinctes des trois fonctionnalités
d'aperçu et d'inspection SP00, SP01, SP02, qui ne coûtent aucun point.

Par défaut : rang effectif 1, attributs à 5, sans autre compétence ou implant.
Exemple actuel : rang appris 10, +2 d'équipement, rang effectif 12 et
attributs à 10. Un actif secondaire reste acheté une seule fois : son exemple
actuel n'est pas présenté comme douze achats. Ses passifs seuls ont des rangs.

Les attaques de comparaison utilisent D = 20 dégâts par projectile ou coup,
avant attributs. Cette valeur fictive ne change pas les armes du jeu. Les
P désigne la précision ordinaire après attributs, équipement
et conditions, avant le modificateur propre à la technique. Les
fractions sont conservées ; les dégâts sont arrondis à l'entier inférieur à la fin
de leur calcul, les soins et absorptions au supérieur. Les pourcentages
affichés sont arrondis au dixième. Un chiffre est annoncé avant défenses,
pas comme une promesse de toucher ou d'infliger des dégâts nets.

Dans les formules, n est le rang effectif et r = n − 1. Les multiplicateurs
reprennent les propositions d'attributs : M_mêlée combine Force et Agilité,
M_distance Perception et Agilité, M_magie Volonté, M_technique Intelligence.
Au profil actuel, ils valent respectivement 1,1275, 1,1275, 1,1 et 1,1.
Les coûts affichés précèdent Canalisation et les implants. La présence d'un
autre passif n'est pas supposée pour embellir le résultat d'une compétence.

## EL-STAT01 Statuts des branches Feu et Froid

Demande du 7 octobre : les compétences de la branche Feu doivent pouvoir
provoquer une brûlure, celles de la branche Froid un gel. Le joueur confirme
que les effets spécifiques déjà validés sont conservés, notamment la brûlure
de Braise, et que chaque compétence possède sa propre chance d'application.
Les chances, dégâts, durées, fréquences de test, résistances et cumuls des
compléments restent à présenter individuellement avant intégration.

La brûlure existante de Braise n'est pas remplacée par un nouveau tirage.
Les compléments de brûlure de Souffle de feu et Mur de feu sont validés
sous FE02-BURN01 et FE03-BURN01 ; leurs bases acceptées sont conservées.
Les compétences sans attaque propre
et les passifs de Froid nécessitent une règle de déclenchement distincte à
présenter ; aucun dégât ni déclenchement automatique ne leur est inventé.
Les mentions antérieures d'absence de brûlure ou de gel dans ces branches
décrivent la proposition avant cette demande et ne bloquent pas ces ajouts.

Le gel reprend l'immobilisation déjà retenue pour les mines : déplacement
bloqué, autres actions possibles. Il relève des contrôles de CTRL01 : boss
immunisés aux contrôles hors exception Gel de CTRL-EL01, chance calculée
multipliée par 0,75 contre les élites, dégâts ordinaires conservés. La brûlure ne reçoit pas implicitement ces immunités
de contrôle. Les deux statuts suivent HUD06 : barre de cible consultable
par analyse, valeurs visibles sans Éclaireur. Icônes et animations à revoir.
La demande porte sur les branches Feu et Froid ; elle n'étend pas
automatiquement ces ajouts aux armes, mines, grenades ou autres branches.

La demande et les réponses sont conservées sous
[EL-STAT01](../docs/reviews/refonte-agent-2026-10-06/elemental-status-and-emanation-redesign-decisions.json).

## EL-FREQ01 Fréquence du Gel et de la Brûlure

Le joueur demande que Gel et Brûlure soient plus fréquents, particulièrement
aux rangs élevés. Cette orientation et les valeurs affichées ci-dessous sont validées
lors du dernier accord pour continuer. Les courbes non montrées restent proposées. Elles conservent
les différences entre compétences, les effets spécifiques et CTRL01.

Sur la même cible fictive de résistance occulte 25 avec Volonté 10, FR01
retient désormais 77,8 % de Gel au rang effectif 11 (élite : 58,4 %), puis
82,5 % au rang 12 (élite : 61,9 %). Le test unique, le Gel de cinq tours,
les dégâts et les interactions réciproques avec Brûlure sont conservés.
La fiche précédente à 21,6 % / 23,4 % reste historique.

Pour la Brûlure, au rang effectif 11 puis au prochain rang 12 :

- FE02 Souffle de feu : 40 %, 80 %, 100 % selon l'exposition ; puis 43 %, 86 %, 100 %. Les jets continuent à croître par tour d'exposition, avec remise à zéro à la sortie.
- FE03 Mur de feu : 45 %, 90 %, 100 %, 100 %, 100 % ; puis 48 %, 96 %, 100 %, 100 %, 100 %. Limite d'une tentative par cible et par tour conservée.
- FE04 Émanation soudaine : 65 %, puis 68 %.
- FE05 Boule de feu : 60 %, puis 63 %.
- FE06 Combustion : 65 % de nouvelle Brûlure, puis 68 %.

Les fiches et blocs Feu acceptés restent les bases historiques de comparaison.
Ces chances affichées sont validées sous EL-FREQ01 ; leurs dégâts, durées,
coûts, recharges et autres accords restent conservés. FE01 Braise garde sa
Brûlure spécifique sans nouveau jet de réussite. Les courbes complètes,
les résistances et l'ordre des applications non encore montrés restent à revoir.
Aucun malus de contrôle n'est ajouté implicitement à la Brûlure.

Les corrections chiffrées et leurs textes de revue sont conservés sous
[EL-FREQ01](../docs/reviews/refonte-agent-2026-10-06/elemental-status-frequency-acceptance-decisions.json).

## CTRL-EL01 Boss face au Gel et à la Brûlure

Le joueur retire l'immunité générale des boss au Gel et demande un malus
plus fort sur la chance d'appliquer Gel et Brûlure. Il limite explicitement
cette exception à ces deux statuts. Les autres contrôles gardent CTRL01.
La Brûlure pouvait déjà toucher les boss ; aucun ancien accord d'immunité
aux dégâts de Feu n'est supposé. Le Poison n'a aucun malus de probabilité
lié à l'importance de la cible ; ses résistances et règles ordinaires restent
applicables. Aucun nouveau malus de Brûlure n'est donné aux élites.

Règle chiffrée validée : −50 % de la chance calculée pour un boss,
soit un multiplicateur de 0,5 appliqué une seule fois. La portée de l'exception
et le coefficient affiché sont validés lors du dernier accord pour continuer. Les dégâts et
les durées ne sont pas diminués par ce malus de chance. Le multiplicateur
boss remplace le facteur d'importance élite ; ils ne se cumulent pas.

Dans les zones persistantes, plafonner d'abord la chance issue de l'exposition
à 100 %, puis appliquer le malus boss : une zone 45 %, 90 %, 100 % donne
22,5 %, 45 %, 50 % sur le boss d'exemple. Braise conserve sa Brûlure garantie
sur une cible ordinaire ; l'application du nouveau malus donne 50 %
contre un boss et introduit ce test sur les boss uniquement.

Sur les mêmes résistances fictives que les aperçus précédents, FR01 donnerait
38,9 % de Gel au rang effectif 11 et 41,3 % au rang 12. Boule de feu donnerait
30 % de Brûlure au rang 11. Arrondir après le calcul exact et le malus ;
l'arrondi au dixième avec demi-valeur vers le haut est proposé. Les vrais
aperçus devront employer les résistances de la cible sélectionnée.
Le coefficient, les exemples affichés de FR01 au rang 11, Boule de feu et
Braise, et l'absence de malus du Poison sont validés. Les exemples non
montrés, les détails d'arrondi et les autres cas restent à revoir.

Demandes et exemples : [CTRL-EL01](../docs/reviews/refonte-agent-2026-10-06/boss-elemental-status-acceptance-decisions.json).

## DENS01 Densité des combats

Le joueur souhaite des combats denses inspirés du hack'n slash, avec beaucoup
d'ennemis différents et leurs statistiques, tout en conservant un défi mortel.
Cette orientation est retenue. Aucun nombre d'ennemis, rythme d'apparition,
comportement collectif ou nouveau réglage du moteur n'est fixé dans cette revue.
Les rôles, l'importance et les compétences de monstres seront détaillés avec
la [revue des monstres](REVUE_COMPETENCES_MONSTRES_AGENT.md).

## EL-GEL01 Gel, Brûlure et orientation des dégâts

Décisions du 7 octobre : le Gel bloque les déplacements, tout en laissant
la cible attaquer et utiliser ses autres actions. Appliquer une Brûlure à
une cible gelée supprime son Gel. La dernière révision confirme aussi
l'inverse : appliquer un Gel à une cible qui brûle supprime sa Brûlure.
Une cible brûlante peut donc être gelée ; l'immunité envisagée puis choisie
est remplacée par cette interaction réciproque. Le remplacement intervient
lorsque le nouveau statut s'applique avec succès, pas du seul fait de dégâts
élémentaires. Les durées ne se convertissent pas en dégâts supplémentaires.
L'ordre exact d'applications simultanées et les cas entre sources restent à revoir.

EL-STAT01 concerne tous les sorts de glace et de feu : Gel pour le Froid,
Brûlure pour le Feu, avec les chances distinctes et les effets spécifiques
déjà retenus. Les effets sans attaque et les passifs demandent toujours leur
propre déclenchement présenté. Aucun effet n'est ajouté aux armes ou aux
arbres secondaires sans leur revue. CTRL01 et HUD06 restent applicables.

EL-BAL01 : les sorts de Froid devront infliger légèrement moins de dégâts
que les sorts de Feu comparables pour tenir compte du contrôle. L'accord
porte sur cette orientation ; aucun pourcentage uniforme de réduction ni
barème complet n'est validé. Les valeurs seront présentées sort par sort,
avec leurs zones, durées, nombre de cibles et autres effets.

Pour FR01, le ralentissement est retiré et remplacé par un Gel de cinq
tours. Cette durée n'est pas imposée à tous les sorts de Froid. La fiche
révisée conserve une chance de Gel et présente séparément les dégâts réduits.
Les valeurs de la première fiche restent historiques et ne constituent pas
une validation de ses chiffres ou de son modèle de contrôle.

Les demandes, la réponse antérieure remplacée et la dernière correction sont
conservées sous [EL-GEL01 / EL-BAL01](../docs/reviews/refonte-agent-2026-10-06/frost-freeze-and-burning-interaction-decisions.json).

## EL-ZONE01 Exposition aux statuts des zones persistantes

Pour un sort à zone persistante capable d'appliquer un statut, le joueur
demande que le risque augmente avec les tours passés dans sa zone : gel,
brûlure ou autre statut admissible. La règle ne se limite pas aux branches
Feu et Froid et n'ajoute pas de statut à une zone qui n'en possédait pas.
Le joueur choisit une chance croissante à chaque jet : pour une base de
10 %, 10 % au premier tour, 20 % au deuxième, 30 % au troisième, etc.
La sortie de la zone remet le cumul à zéro ; une nouvelle entrée reprend
à la chance de base. Il ne s'agit pas de jets à chance constante dont on
afficherait seulement la probabilité cumulée d'avoir subi un statut.

Calcul proposé : chance d'exposition = min(100 %, chance de base × nombre
de tours d'exposition). Pour un contrôle, appliquer ensuite CTRL01 : boss
immunisés et chance multipliée par 0,75 contre les élites. Ce cumul ne
supprime pas leur immunité ou leur malus. Le moment du premier jet,
la définition d'un tour d'exposition, les zones superposées, le suivi par
lanceur et par zone, les réapplications et les résistances restent à
présenter. Aucun compteur commun à toutes les zones n'est inventé.

Une détonation unique, comme Émanation soudaine, ne devient pas une zone
persistante : son marqueur de préparation ne compte pas comme exposition
au statut. Une brûlure déjà appliquée ne constitue pas non plus une zone
au sol. Les effets spécifiques validés, CTRL01 et HUD06 sont conservés.
Les mines et grenades ne reçoivent pas de nouvelles règles implicitement ;
leurs zones persistantes feront l'objet d'une revue de compatibilité.

Demande conservée sous
[EL-ZONE01](../docs/reviews/refonte-agent-2026-10-06/elemental-status-and-emanation-redesign-decisions.json).

## Temps, disponibilité et ciblage

**Exigence d'arme confirmée :** les compétences actives d'arme de mêlée
demandent une arme de mêlée équipée et active. Les compétences actives
d'arme à distance demandent une arme à distance équipée et active.
La fiche indique le type requis si l'arme active ne convient pas, et
l'action est indisponible. Une autre arme simplement équipée ne suffit pas.

Un lancement normal prend une action et met immédiatement son effet en
place, avant les réactions ennemies. Le marquage T01 conserve son activation
sans passage de tour. Un effet explicitement différé garde son délai annoncé :
mine armée après un tour, détonation d'Émanation soudaine ou progression de l'Orbe.
Ces délais ne demandent pas
une action cachée de préparation au joueur.

Le menu des actions affiche le temps restant de recharge ; le bouton grisé
est désactivé jusqu'à la fin du délai, avec survol conservé. Le libellé proposé
« Interférence — Recharge : 3 tours » est accepté lors de la dernière réponse.
Pour les canalisations qui conservent un maintien manuel, une commande
distincte poursuit l'effet déjà lancé sans constituer une nouvelle activation
contournant sa recharge. FE02 Souffle de feu continue automatiquement à la
demande du joueur ; sa fiche révisée précise les interruptions. Cette exception
ne modifie pas les autres compétences de canalisation.

Un refus pour cible, trajet, forme, ressource, munition ou recharge illégale
ne change ni ressources, ni position, ni temps, ni tirage. Les coûts d'un
lancement exécuté restent payés sur échec ou résistance. Les compétences
qui combinent deux systèmes valident tous leurs besoins avant la première
dépense. Consulter, déplacer le curseur ou annuler un choix ne simule rien.

Portées et zones sont limitées à la carte active, aux obstacles et aux
observations réellement disponibles. Les previews présentent seulement le
terrain connu et les acteurs observés. Les zones radiales emploient la
distance euclidienne ; A02 conserve explicitement ses trois par trois cases.
La propagation sonore reste celle du moteur. Une zone peut réellement
affecter un acteur caché sans que l'interface le révèle avant l'événement.

Selon AR-HL01, les techniques DI01 à DI05 de l'arbre de tir ordinaire
ne s'appliquent pas aux armes lourdes. Les familles précises restent à déclarer.
Les techniques DI01 à DI05 utilisent une arme à projectile physique sans
explosion ni cône de plombs, notamment pistolet, PM ou fusil compatible.
DI02 enchaîne deux séquences du mode de tir actif de l'arme. Les autres
techniques définissent leur propre nombre de projectiles à partir du dégât
d'un seul projectile, sans multiplier ensuite une rafale native.
L'arme sélectionnée et ses munitions restent utilisées. Les lance-grenades,
rayons, lasers, attaques naturelles et sorts ne sont pas implicitement
convertis en balles. Chaque famille sera déclarée compatible au contenu.

## Cumuls, sources et effets prolongés

Une même compétence ne multiplie pas ses propres buffs, leurres
ou passages : un seul exemplaire, remplacé ou refusé selon sa règle. Des
sources différentes peuvent coexister. Les réductions de dégâts distinctes
se multiplient après armure et résistances ; elles ne s'additionnent pas
jusqu'à une immunité. L'absorption suit les protections précédentes, puis
Dernier sursaut conserve sa place finale. Chaque réserve d'absorption est
limitée et ne soigne pas les PV.

Dans les dix branches complétées ici, relancer un buff, un leurre ou un
couple de passages remplace sa version précédente après
validation, sans additionner les durées. Une création persistante propre
à un sort — mur, orbe, blizzard, orage, circuit ou prison — est limitée
à une par compétence et par agent ; la nouvelle remplace l'ancienne. Les
exceptions explicitement refusées restent refusées, notamment une forme
déjà active et Fusion défensive pendant son bouclier. Les mines gardent
leur capacité distincte et les quatre lots précédents leurs règles propres.

Pour des zones identiques de même élément qui se chevauchent sur une case,
la plus forte résolution périodique active est proposée, sans addition des
ticks. Une traversée et une attente sur la même zone ne donnent pas deux
résolutions du même effet dans un tour. La durée d'une zone n'est pas
réinitialisée par la mort d'une cible, sauf mention explicite de la compétence.

Les attaques secondaires automatiques — miroir, éclats, ripostes, renvois,
décharges passives — ne déclenchent pas une autre attaque secondaire. Les
bonus numériques permanents applicables restent pris en compte. Le nombre
de leurs déclenchements est limité explicitement ; menus et changements
d'équipement ne réinitialisent ni séries, ni réserves, ni recharges.

Le vol de vie se calcule sur les PV réellement retirés à un hostile, pas
sur dégâts affichés, murs, leurres ou victimes alliées. Les sources applicables
au même coup sont réunies avant un seul arrondi ; le soin proposé ne dépasse
ni 50 % des PV retirés ni les PV manquants. Une altération anti-soin peut
bloquer ce gain. Le renvoi ne renvoie pas du renvoi et ne vole pas de vie.

Les rangs de base, la géométrie et la durée des effets persistants sont
fixés au lancement. Leur auteur et leurs catégories sont conservés. Les
modificateurs conditionnels explicitement dynamiques, notamment Puissance
métamorphe, sont lus à la résolution de chaque dégât. Un retour humain ne
laisse pas ce bonus sur des zones anciennes. Les effets magiques de contrôle
utilisent puissance contre résistance occulte avec probabilités bornées
entre 5 % et 95 %. Aucun classement d'importance des monstres n'est fixé ici.

## Transformations communes

Cinq transformations temporaires, une seule forme à la fois. Chaque achat
de forme donne ses actions propres, sans points supplémentaires pour celles-ci.
Les armes ordinaires sont rangées pendant la forme ; leurs dégâts, munitions
et affixes d'attaque ne passent pas sur les attaques naturelles. Les pièces
d'armure, attributs, bonus de compétences et protections restent conservés.
La sortie retrouve l'équipement et n'offre ni soin ni recharge gratuite.

L'entrée et le retour volontaire prennent une action ; l'expiration est
automatique. Entrer dans une autre forme exige d'abord le retour humain,
et reste soumis à son coût et sa recharge. Les recharges des actions de
forme sont conservées lorsqu'on quitte puis reprend cette forme. Les PV
maximum ne changent pas du seul fait de se transformer. Les cinq dessins
existants sont conservés et les animations attendent leur revue native.

Les actions ordinaires de forme restent utilisables sous Silence, même
si elles ont un aspect magique. Ses actions spéciales sont des actifs et
peuvent être bloquées. Un sort appris indépendant d'une arme reste utilisable
si ses autres conditions sont satisfaites. Ancrage empêche les mouvements
forcés ; l'immobilisation de froid interdit les mouvements volontaires,
y compris téléportation, mais laisse attaques et actions sans déplacement.

## Accès aux compétences — décisions du 7 octobre

AR-ACCESS01 remplace les anciennes propositions de paliers : toutes les
compétences principales sont achetables dès le départ avec les points
nécessaires, sans seuil de niveau ou de points investis dans une branche.
Les lignes organisent les compétences visuellement et ne débloquent rien.
Les rangs, coûts d'apprentissage et conditions d'utilisation sont conservés.

AR-SEC01 impose l'actif associé avant les achats de ses passifs secondaires :
SF00 pour SF01 et SF02, SM00 pour SM01 à SM05, SG00 pour SG01 à SG04.
Après l'apprentissage de cet actif, ses passifs sont tous accessibles,
sans palier supplémentaire. Les interfaces SP00 à SP02 restent gratuites.

AR-HL01 prévoit un arbre Armes lourdes après la validation de toutes les
compétences du catalogue actuel. Il concernera les armes lourdes à distance
existantes et futures, exclues de l'arbre de tir ordinaire. Aucun nouveau
catalogue lourd, barème, texte de compétence ou visuel n'est créé ici.
Les détails d'organisation et de compatibilité seront revus ensuite dans
la [revue des arbres](REVUE_ARBRES_COMPETENCES_AGENT.md).

## Lots précédents conservés

| Branche | Document de travail |
| --- | --- |
| Mêlée | [Lot 03](REVUE_MELEE_AGENT_LOT_03.md) |
| Survie | [Lot 04](REVUE_SURVIE_AGENT_LOT_04.md) |
| Drones | [Lot 05](REVUE_DRONES_AGENT_LOT_05.md) et [persistance](PROPOSITION_DRONES_PERSISTANTS_AGENT.md) |
| Affaiblissements | [Lot 06](REVUE_AFFAIBLISSEMENTS_AGENT_LOT_06.md) |

Leur statut reste celui de leurs revues : les textes et tableaux effectivement
acceptés restent retenus ; Maintenance et les détails non présentés restent
des propositions. Aucune règle de ces quatre lots n'est remplacée silencieusement.

## Attaques à distance

### DI01 Tir perforant

Type : actif. Rôle retenu : Traverse plusieurs cibles alignées, avec les obstacles comme limite.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts du premier projectile | 110 % de D | 176 % de D |
| Premier impact de comparaison | 22 dégâts | 39 dégâts |
| Cibles traversées au maximum | 2 | 4 |
| Dégâts conservés par traversée | 75 % | 75 % |
| Munitions | 1 | 1 |
| Énergie | 8 points | 11 points |
| Recharge | 4 tours | 4 tours |

**Progression et calculs proposés :** Dégâts du premier projectile = 110 + 6r ; Premier impact de comparaison = floor(D × (110 + 6r)/100 × M_distance) ; Cibles traversées au maximum = min(5, 2 + floor(r/5)) ; Dégâts conservés par traversée = 75 ; Munitions = 1 ; Énergie = 8 + floor(r/3) ; Recharge = 4.

**Fonctionnement proposé :** Un projectile traverse les acteurs alignés, sans traverser un mur. Chaque acteur est testé une seule fois ; les dégâts de l'impact k sont calculés avec 0,75 puissance k avant l'arrondi. Les alliés sur la ligne peuvent être touchés. Un échec de précision sur un acteur laisse le projectile poursuivre sa trajectoire. Portée ordinaire de l'arme.

Intention visuelle conservée du catalogue : Trait fin continu et impacts successifs le long de la ligne. Rendu non créé dans cette passe.

### DI02 Feu nourri

Type : actif. Rôle retenu : enchaîne deux séquences du mode de tir actif
de l'arme contre une même cible.

**Direction acceptée le 7 octobre :** le nombre de séquences reste fixé
à deux, quel que soit le rang. Une arme en coup par coup tire deux
projectiles ; une arme tirant trois projectiles par rafale en tire six.
Le coût en munitions suit ce nombre. Les rangs améliorent les dégâts par
projectile. La direction d'une précision réduite est conservée, sans
valider ici son coefficient exact. L'ancien nombre de projectiles fixé
par la compétence et le remplacement de la rafale native sont abandonnés.

**Texte roleplay révisé validé :**

> Vous concentrez votre feu sur une seule cible et l’accablez de tirs.

**Paramètres encore proposés pour le réglage :** dégâts par projectile =
60 + 3r % de D ; impact = floor(D × (60 + 3r)/100 × M_distance) ;
précision par projectile = 90 % de P ; énergie = 10 + floor(r/3) ;
recharge = 4 tours. Les séquences valent deux ; projectiles et munitions
valent deux fois le nombre de projectiles du mode actif compatible.
La courbe complète, les arrondis et les rangs non présentés restent à revoir.
Les valeurs des rangs effectifs 11 et 12 ci-dessous sont acceptées comme
réglage provisoire lors de la validation de leur fiche le 7 octobre.

**Fiche individuelle validée :** avec une arme en coup par coup à
18–22 dégâts de base, Perception 10 et Agilité 10, le rang effectif 11
affiche deux projectiles, 18–22 dégâts par projectile (90 % de l'attaque),
une précision égale à 90 % de la précision habituelle, 13 énergie,
deux munitions et quatre tours de recharge. Au prochain rang effectif 12,
seuls les dégâts passent à 18–23 (93 %). La portée reste celle de l'arme
active. Cette comparaison et le RP sont validés sous
[DI02-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-DI02-03.json).

Chaque projectile peut toucher ou manquer sa cible. Les règles de jet et
de défense par projectile, les familles d'armes compatibles, le paiement
avant le premier tir, la mort de la cible avant la fin de l'enchaînement,
les interruptions et les interactions d'affixes restent des propositions
à préciser. Elles ne sont pas validées par l'accord sur les deux séquences.

Intention visuelle conservée du catalogue : impacts rapprochés et recul
bref répété de l'arme. Le rendu n'est pas créé ou validé dans cette passe.

### DI03 Volée

> Vous ouvrez le feu en éventail sur les ennemis qui vous font face.

Type : actif. Rôle retenu : Répartit les projectiles dans un cône pour toucher plusieurs cibles.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Cibles visées au maximum | 3 | 5 |
| Dégâts par cible | 80 % de D | 135 % de D |
| Impact de comparaison | 16 dégâts | 30 dégâts |
| Portée du cône, limitée aussi par l'arme | 3 cases | 5 cases |
| Angle du cône | 90 degrés | 90 degrés |
| Munitions par cible visée | 1 | 1 |
| Énergie | 12 points | 15 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Cibles visées au maximum = min(6, 3 + floor(r/5)) ; Dégâts par cible = 80 + 5r ; Impact de comparaison = floor(D × (80 + 5r)/100 × M_distance) ; Portée du cône, limitée aussi par l'arme = min(6, 3 + floor(r/4)) ; Angle du cône = 90 ; Munitions par cible visée = 1 ; Énergie = 12 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Une balle par hostile visible retenu dans le cône, un tir au maximum par cible. Les cibles sont prises de la plus proche à la plus éloignée, avec des lignes de tir légales. Sans cible admissible, refus sans coût. Seules les balles des tirs réellement prévus sont débitées ; leur total doit être disponible. L'aperçu présente le cône et les cibles observées retenues.

**Fiche individuelle validée le 7 octobre :** au rang effectif 11, avec
18–22 dégâts de base par projectile, six cases de portée d'arme, Perception
10 et Agilité 10 : cinq cibles maximum, un projectile par cible, 26–32
dégâts (130 %), cône de 90 degrés, portée de cinq cases, 15 énergie, une
munition par cible et six tours de recharge. Au rang effectif 12, seuls
les dégâts passent à 27–33 (135 %). Priorité aux ennemis visibles les plus
proches ; les murs bloquent les tirs. Le RP et cette comparaison sont
validés sous [DI03-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-DI03-02.json).
La courbe complète, les détails non présentés et le rendu restent à revoir.

Intention visuelle conservée du catalogue : Éventail de trajectoires séparées devant l'agent. Rendu non créé dans cette passe.

### DI04 Tir de précision

Type : actif. Remplacement de Ricochet accepté : un projectile contre une
seule cible, avec une précision améliorée et une portion d'armure ignorée.
Le bonus de chance de critique ne concerne que ce tir. Le nom, le RP,
les valeurs affichées aux rangs effectifs 11 et 12 et les conditions de
la fiche sont validés comme équilibrage provisoire. Les courbes complètes,
les cas non présentés et les règles générales des critiques restent à
revoir. Les anciens paramètres de rebond sont conservés dans les
instantanés de revue.

> Vous ajustez votre visée et cherchez la faille dans la protection de votre cible.

**Paramètres proposés :** dégâts hors critique = 100 % du projectile
ordinaire ; précision = 110 + 2r % de P ; armure ignorée =
20 + 40r/(n + 9) % ; bonus de chance de critique = 5 + 20r/(n + 9)
points de pourcentage ; énergie = 8 + floor(r/3) ; une munition ;
recharge de quatre tours ; portée ordinaire de l'arme active.
Les fractions exactes sont conservées avant les arrondis d'affichage.
Les courbes complètes restent des propositions, dont les rangs au-delà
de dix ; les limites de la chance finale relèvent des règles générales
de critique à revoir.

**Profil fictif de comparaison :** projectile à 18–22 dégâts de base,
six cases de portée, Perception 10, Agilité 10 et chance habituelle de
critique de 5 %. Ces 5 % ne sont pas une règle de base du jeu.
Au rang effectif 11 : 20–24 dégâts hors critique, précision à 130 % de P,
40 % de l'armure ignorée, bonus de critique de 15 points donnant 20 %
dans ce profil, 11 énergie et quatre tours de recharge. Au rang effectif
12, seuls la précision (132 %), l'armure ignorée (41 %) et la chance
de critique (20,5 %) changent. Le bonus critique du rang 12 vaut
15 + 10/21 points avant arrondi, soit environ 15,5 points.
La comparaison validée est conservée sous
[DI04-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-DI04-03.json).

Le bonus de critique concerne ce tir et n'est pas un buff persistant.
La cible doit être actuellement visible et le trajet direct dégagé.
AR-HL01 exclut les armes lourdes. La précision augmentée ne garantit pas
la touche. Le calcul exact du jet, les défenses, les dégâts supplémentaires
d'un critique, les cumuls, les limites et l'ordre de résolution restent
à présenter. Aucun multiplicateur de dégâts critiques n'est fixé ici.

L'ancienne intention visuelle de rebond est retirée de cette proposition.
Le nouvel effet visuel et son rendu restent à proposer séparément.

### DI05 Tir de harpon

Type : actif. Rôle retenu : Accroche une cible et la rapproche si le passage est libre.

> Votre projectile s’ancre dans la cible et la ramène vers vous d’une traction sèche.

RP, valeurs actuelles/prochain rang et conditions de la fiche individuelle
validés comme équilibrage provisoire. Les courbes et les cas non présentés
restent à revoir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts | 100 % de D | 144 % de D |
| Impact de comparaison | 20 dégâts | 32 dégâts |
| Rapprochement maximal | 1 case | 3 cases |
| Munitions | 1 | 1 |
| Énergie | 10 points | 13 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Dégâts = 100 + 4r ; Impact de comparaison = floor(D × (100 + 4r)/100 × M_distance) ; Rapprochement maximal = min(3, 1 + floor(r/5)) ; Munitions = 1 ; Énergie = 10 + floor(r/3) ; Recharge = 6.

**Fonctionnement retenu, détails non présentés à revoir :** Après un tir réussi, la traction rapproche la cible vers l'agent, jusqu'à la limite ou au premier obstacle. Aucun mur, acteur ou case interdite n'est traversé ; la cible ne finit pas sur l'agent. Une cible déclarée immobile ou ancrée garde sa position, mais peut subir les dégâts. Un échec du tir ne rapproche pas la cible. La traction relève des contrôles de CTRL01 : boss immunisés et chance calculée multipliée par 0,75 contre les élites, sans supprimer les dégâts du tir. La base conditionnelle est validée : traction garantie après touche sur une cible ordinaire mobile (100 %), 75 % contre une élite mobile. L'ordre exact des tests et la géométrie de traction restent à revoir ; aucun nouveau calcul de résistance n'est fixé.

**Comparaison individuelle validée :** arme active non lourde à 18–22 dégâts de base, portée de six cases, Perception 10 et Agilité 10. Au rang effectif 11 : 28–34 dégâts avant défenses (140 % de l'attaque), rapprochement maximal de trois cases, 13 énergie, une munition et six tours de recharge. Au prochain rang effectif 12, seuls les dégâts changent : 29–35 (144 %). L'équipement et les attributs restent identiques. La fiche sans tableau est conservée sous [DI05-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-DI05-02.json). Les règles de collision de Repousser ne sont pas étendues à cette compétence.

Intention visuelle conservée du catalogue : Fil tendu entre l'impact et l'agent, rétraction visible. Rendu non créé dans cette passe.

## Esquive

### T02 Pas spectral

Type : actif. Rôle retenu : Déplace d'une case légale et augmente brièvement l'esquive.

> Votre silhouette se trouble tandis que vous vous écartez du danger.

RP, valeurs actuelles/prochain rang et conditions de la fiche individuelle
validés comme équilibrage provisoire. Les courbes et les cas non présentés
restent à revoir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Déplacement | 1 case | 1 case |
| Bonus d'esquive | 15 unités | 48 unités |
| Durée | 2 tours | 4 tours |
| Énergie | 8 points | 11 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Déplacement = 1 ; Bonus d'esquive = 15 + 3r ; Durée = 2 + floor(r/5) ; Énergie = 8 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Reprend T02 du lot 02. Case adjacente libre et connue, mouvement ordinaire avec ses réactions. Le bonus commence avant ces réactions. Pas de mur traversé, de frappe gratuite ou d'invulnérabilité. Une nouvelle application remplace l'ancienne.

**Comparaison individuelle validée :** au rang effectif 11, déplacement d'une case, bonus de 45 points d'esquive pendant quatre tours, coût de 11 énergie et recharge de six tours. Au prochain rang effectif 12, seul le bonus d'esquive change : 48 points. L'équipement, les attributs et les autres modificateurs restent identiques. Ce bonus est ajouté à l'esquive ordinaire ; il ne représente pas des points de pourcentage de chance d'éviter une attaque. La fiche sans tableau est conservée sous [T02-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-T02-02.json). Les courbes complètes, les bornes des jets, les cas de déplacement et les cumuls non présentés restent à revoir.

Intention visuelle conservée du catalogue : Silhouette décalée puis contour léger maintenu pendant le bonus. Rendu non créé dans cette passe.

### ES03 Leurre holographique

Type : actif. Rôle retenu : Place une fausse cible temporaire qui peut détourner les attaques.

> Vous projetez un double de vous-même pour attirer les attaques loin de votre position.

RP, valeurs actuelles/prochain rang et conditions de la fiche individuelle
validés comme équilibrage provisoire. Le double doit être légèrement
transparent, selon la demande explicite du joueur. Le degré exact de
transparence et le rendu natif restent à présenter.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Durée | 3 tours | 5 tours |
| Coups nécessaires pour détruire le leurre | 1 | 1 |
| Distance de placement | 3 cases | 3 cases |
| Énergie | 8 points | 11 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Durée = 3 + floor(r/4) ; Coups nécessaires pour détruire le leurre = 1 ; Distance de placement = 3 ; Énergie = 8 + floor(r/3) ; Recharge = 6.

**Fonctionnement retenu, détails non présentés à revoir :** Un seul leurre à la fois, sur une case libre actuellement visible. Un hostile qui le voit est détourné vers le leurre si la diversion réussit ; un acte agressif de l'agent contre cet hostile lui rend la priorité. Le premier impact contre le leurre le détruit. Le leurre ne bloque pas le passage, ne fait pas de dégâts et ne donne ni XP ni loot. C'est une projection, sans invocation de drone. La diversion est validée comme contrôle relevant de CTRL01 : boss immunisés et chance calculée multipliée par 0,75 contre les élites. La base affichée est validée : 100 % pour un ennemi ordinaire voyant le leurre, 75 % contre une élite. La fréquence des tests, la perte de visibilité et le comportement après un échec restent à revoir. La transparence est une direction de rendu ; les règles de visibilité mécanique ne changent pas.

**Comparaison individuelle validée :** rang effectif 8, durée de quatre tours, un seul leurre actif détruit au premier impact, portée de placement de trois cases, coût de 10 énergie et recharge de six tours. Au prochain rang effectif 9, seule la durée change : cinq tours. L'exemple conserve l'équipement, les attributs et les autres modificateurs ; le changement de rang d'exemple sert à comparer un rang qui augmente la durée, sans réviser la courbe existante. Aux rangs effectifs 11 et 12, la durée serait identique (cinq tours) selon cette courbe ; ses rangs intermédiaires sans changement et ses valeurs non affichées restent à revoir. La fiche sans tableau est conservée sous [ES03-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-ES03-02.json). Aucun nouveau calcul de résistance, dégât ou statut transmissible par Contagion n'est ajouté.

Intention visuelle précisée par le joueur : Double légèrement transparent, avec lignes de balayage verticales. Aucun pourcentage d'opacité n'est fixé. Rendu non créé dans cette passe.

### ES05 Traversée de phase

Type : actif. Rôle retenu : Passe de l'autre côté d'un ennemi adjacent si la destination est légale.

> Vous traversez votre adversaire dans un bref déphasage et réapparaissez derrière lui.

RP, effets actuels et au prochain rang, coûts et conditions présentés validés pour ES05. Équilibrage provisoire ; courbes et cas non montrés toujours à revoir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Passage derrière un hostile adjacent | 2 cases parcourues | 2 cases parcourues |
| Bonus d'esquive | 10 unités | 32 unités |
| Durée du bonus | 2 tours | 2 tours |
| Énergie | 10 points | 13 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Passage derrière un hostile adjacent = 2 ; Bonus d'esquive = 10 + 2r ; Durée du bonus = 2 ; Énergie = 10 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Passe de l'autre côté d'un hostile adjacent, sur la case opposée connue et libre. Le passage à travers cet acteur est autorisé par ce pouvoir, mais pas à travers un mur, une porte fermée ou un coin interdit. Aucun dégât n'est infligé. Le bonus commence avant les réactions au déplacement.

**Comparaison individuelle validée :** au rang effectif 11, passage de deux cases de l'autre côté d'un ennemi adjacent visible, bonus de 30 points d'esquive pendant deux tours, coût de 13 énergie et recharge de huit tours. Au prochain rang effectif 12, seul le bonus d'esquive change : 32 points. L'équipement, les attributs et les autres modificateurs restent identiques. La destination doit être libre, connue et franchissable ; le passage à travers la cible n'autorise pas de traversée des autres obstacles. Le bonus s'ajoute à l'esquive ordinaire et s'applique aux réactions provoquées par le déplacement. La fiche sans tableau validée est conservée sous [ES05-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-ES05-02.json). Les courbes complètes, les diagonales, les grandes empreintes d'acteurs, l'ordre des réactions et les cumuls non présentés restent à revoir. Il s'agit d'un déplacement de l'agent ; aucun contrôle ni dégât n'est infligé à l'ennemi.
Intention visuelle conservée du catalogue : Silhouette fragmentée en bandes au passage de la cible. Rendu non créé dans cette passe.

### ES06 Réflexes balistiques

Type : passif. Rôle retenu : Renforce l'esquive contre les projectiles, sans renforcer toutes les défenses.

> Vous anticipez les trajectoires et vous écartez des projectiles avant l’impact.

RP, comparaison des effets actuels et au prochain rang et restriction aux projectiles soumis à un jet de précision validés pour ES06. Équilibrage provisoire ; courbes et cas non montrés toujours à revoir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Esquive supplémentaire contre projectiles | 12 unités | 45 unités |

**Progression et calculs proposés :** Esquive supplémentaire contre projectiles = 12 + 3r.

**Fonctionnement proposé :** Bonus seulement contre un projectile qui utilise un jet de précision. Il ne devient pas une réduction générale des dégâts et ne crée pas un jet d'esquive pour un sort qui n'en a pas.

**Comparaison individuelle validée :** au rang effectif 11, bonus de 42 points d'esquive contre les projectiles ; au prochain rang effectif 12, 45 points. Le bonus s'ajoute à l'esquive habituelle uniquement contre les projectiles soumis à un jet de précision. Passif sans coût ni recharge ; équipement, attributs et autres modificateurs conservés entre les deux aperçus. La fiche sans tableau validée est conservée sous [ES06-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-ES06-02.json). Les courbes complètes, la classification des projectiles physiques ou magiques, les interactions avec les attaques multiples et les autres bonus d'esquive restent à revoir. Aucun renvoi de projectile, réduction de dégâts de zone ou jet d'esquive supplémentaire pour un effet automatique n'est créé.

Intention visuelle conservée du catalogue : Courte déviation visuelle au passage d'un projectile évité. Rendu non créé dans cette passe.

## Feu

### FE01 Braise

Type : actif. Rôle retenu : Projectile étroit qui blesse puis laisse une brûlure sur la cible touchée.

> Vous projetez une braise incandescente qui s’accroche à votre cible et la consume.

Nom révisé « Braise », RP, effets actuels et au prochain rang, coût, recharge et conditions présentés validés pour FE01. Équilibrage provisoire ; courbes et cas non montrés toujours à revoir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts directs | 8 dégâts | 45 dégâts |
| Brûlure par tour | 2 dégâts | 14 dégâts |
| Durée | 3 tours | 5 tours |
| Portée | 5 cases | 5 cases |
| Énergie | 8 points | 11 points |
| Recharge | 3 tours | 3 tours |

**Progression et calculs proposés :** Dégâts directs = floor((8 + 3r) × M_magie) ; Brûlure par tour = floor((2 + 1r) × M_magie) ; Durée = 3 + floor(r/5) ; Portée = 5 ; Énergie = 8 + floor(r/3) ; Recharge = 3.

**Fonctionnement proposé :** Une seule cible hostile, ligne de vue. L'impact est immédiat ; une brûlure propre à l'agent commence au prochain tour complet. Le même auteur ne cumule pas plusieurs exemplaires de cette brûlure. Ce projectile ne traverse pas gratuitement plusieurs cibles.

**Comparaison individuelle validée :** au rang effectif 11 avec Volonté 10, dégâts directs de feu de 41, puis brûlure de 13 dégâts de feu par tour pendant cinq tours. Portée de cinq cases, coût de 11 énergie et recharge de trois tours. Au prochain rang effectif 12, dégâts directs de 45 et brûlure de 14 par tour ; les autres valeurs restent identiques. L'équipement, les attributs et les autres modificateurs sont conservés. Une seule cible ennemie visible, trajet dégagé ; la brûlure commence au prochain tour complet et plusieurs applications de l'agent ne se cumulent pas. Les dégâts sont affichés avant les défenses de la cible. La fiche sans tableau validée sous le nom « Braise » est conservée sous [FE01-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FE01-02.json). Les courbes complètes, le modèle de toucher, les collisions du projectile, l'ordre et le décompte des tours, le rafraîchissement de la brûlure et ses interactions avec les autres effets restent à revoir. Aucun bonus de dégâts de l'arme active, variance générale des sorts, contrôle ou immunité élémentaire des boss n'est ajouté. La brûlure appliquée figure dans la barre de statuts de l'analyse de cible, avec ses valeurs visibles sans Éclaireur selon HUD06 ; icône et rendu natif toujours à revoir.

Intention visuelle conservée du catalogue : Braise allongée, queue sombre et point incandescent persistant. Rendu non créé dans cette passe.

### FE02 Souffle de feu

Type : actif. Rôle révisé : Projette un cône de feu qui continue automatiquement pendant sa durée si l'agent ne se déplace pas, n'attaque pas avec ses armes et n'utilise pas une autre compétence.

> Vous déchaînez un torrent de flammes devant vous.

Nom « Souffle de feu », RP, effets actuels et au prochain rang, coût par tour, recharge et conditions de la fiche révisée validés pour FE02. Équilibrage provisoire ; courbes et cas non montrés toujours à revoir.

Complément FE02-BURN01 validé : chance croissante de brûlure dans le cône, chiffres, durée, remise à zéro à la sortie, persistance après le souffle, démarrage au prochain tour complet et non-cumul affichés acceptés. La base conserve son accord ; les cas non montrés restent à revoir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par cible et par tour | 6 dégâts | 30 dégâts |
| Chance de brûlure au premier tour d'exposition | 5 % | 16 % |
| Chance au deuxième tour d'exposition | 10 % | 32 % |
| Chance au troisième tour d'exposition | 15 % | 48 % |
| Brûlure par tour | 1 dégât | 5 dégâts |
| Durée de brûlure | 3 tours | 3 tours |
| Portée du cône | 3 cases | 5 cases |
| Angle | 90 degrés | 90 degrés |
| Durée maximale | 3 tours | 3 tours |
| Énergie par tour | 6 points | 8 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Dégâts par cible et par tour = floor((6 + 2r) × M_magie) ; Portée du cône = min(5, 3 + floor(r/5)) ; Angle = 90 ; Durée maximale = 3 ; Énergie par tour = 6 + floor(r/4) ; Recharge = 6.

**Calculs du complément proposés :** Chance de base = min(100, max(0, 5 + r)) ; Chance au tour d'exposition t = min(100, chance de base × t) ; Brûlure par tour = floor((1 + 0,4r) × M_magie) ; Durée de brûlure = 3. La sortie du cône remet la chance à sa base. La brûlure commence au prochain tour complet après application, continue après la fin du souffle et ne cumule pas ses propres applications ; réapplications et cumuls entre sorts restent à revoir.

**Fonctionnement révisé :** La continuation automatique et les interruptions présentées sont validées : déplacement, attaque avec une arme ou utilisation d'une autre compétence termine le souffle ; les dégâts reçus ne l'interrompent pas et l'énergie insuffisante y met fin. Aucune commande « Maintenir » et aucun passage de tour automatique sans action du joueur. Le décompte exact reste proposé : premier souffle au lancement, deux résolutions suivantes au fil des tours, trois résolutions au total avec leur énergie. Les autres actions ne sont pas automatiquement reclassées comme attaques ou compétences ; leurs cas particuliers restent à revoir. Les murs arrêtent le cône ; les alliés peuvent être touchés.

**Base individuelle validée :** au rang effectif 11 avec Volonté 10, 28 dégâts de feu par cible et par tour, cône de 90 degrés sur cinq cases, durée maximale de trois tours, coût de huit énergie par tour et recharge de six tours. Au prochain rang effectif 12, les dégâts de base deviennent 30 par cible et par tour. L'équipement, les attributs et les autres modificateurs restent identiques. Le souffle continue automatiquement ; déplacement, attaque avec une arme ou utilisation d'une autre compétence l'interrompt. Les dégâts reçus ne l'interrompent plus. L'énergie insuffisante met fin au souffle. Les murs arrêtent les flammes et les alliés dans le cône peuvent être touchés. Les dégâts sont affichés avant les défenses des cibles. La fiche de base validée est conservée sous [FE02-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FE02-03.json). Les courbes complètes, le départ et le décompte de recharge, les changements d'orientation, le modèle de toucher, la géométrie exacte du cône, l'ordre précis des réactions et résolutions automatiques, les tentatives d'actions refusées, l'emploi d'objets et les attaques naturelles restent à revoir. Aucun maintien manuel, passage de tour automatique, déplacement gratuit ou variance générale des sorts n'est ajouté. Les autres compétences canalisées ne reçoivent pas implicitement ces nouvelles règles.

**Complément de brûlure validé :** au rang effectif 11 avec Volonté 10, chance de 15 %, puis 30 %, puis 45 % selon les tours passés dans le cône ; brûlure de cinq dégâts de feu par tour pendant trois tours. Au prochain rang effectif 12 : chance de 16 %, puis 32 %, puis 48 %, même brûlure. Toutes les autres valeurs et conditions de la base restent conservées. La sortie réinitialise le risque, pas une brûlure déjà active. La fiche réunie validée est conservée sous [FE02-05](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FE02-05.json), avec l'accord sous [FE02-BURN01](../docs/reviews/refonte-agent-2026-10-06/fire-breath-burn-extension-acceptance-decisions.json). Premiers tests, compteur par cible et lancement, changements d'orientation, superpositions, résistance au statut, priorité entre brûlures et rafraîchissement restent à revoir. La brûlure ne reçoit pas les immunités de contrôle ; HUD06 garde ses valeurs visibles par analyse sans drone. L'indication visuelle présentée est acceptée en direction ; son rendu natif reste à présenter, sans intégration.

Intention visuelle conservée du catalogue : Langues de feu étagées devant les mains. Rendu non créé dans cette passe.

Indication visuelle de brûlure validée en direction : Petites flammes sur la cible brûlée, conservées pendant son statut même après la fin du souffle. Pixels et animation native restent à revoir.

### FE03 Mur de feu

Type : actif. Rôle retenu : Crée une ligne temporaire dangereuse à traverser.

> Vous faites jaillir un rideau de flammes qui dévore ceux qui le traversent.

RP, effets actuels et au prochain rang, coût, recharge et conditions de la fiche individuelle validés pour FE03. Équilibrage provisoire ; courbes et cas non montrés toujours à revoir.

Complément FE03-BURN01 validé : chance croissante de brûlure dans les flammes, chiffres, durée, tentative unique par cible et par tour, remise à zéro à la sortie, persistance, démarrage au prochain tour complet et non-cumul affichés acceptés. Les bases conservent leur accord ; les cas non montrés restent à revoir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de zone par tour | 5 dégâts | 29 dégâts |
| Chance de brûlure au premier tour d'exposition | 15 % | 26 % |
| Chance au deuxième tour d'exposition | 30 % | 52 % |
| Chance au troisième tour d'exposition | 45 % | 78 % |
| Chance au quatrième tour d'exposition | — (mur de 3 tours) | 100 % |
| Brûlure par tour | 1 dégât | 4 dégâts |
| Durée de brûlure | 3 tours | 3 tours |
| Longueur maximale du mur | 3 cases | 7 cases |
| Durée | 3 tours | 5 tours |
| Portée de placement | 4 cases | 4 cases |
| Murs actifs maximum | 1 | 1 |
| Énergie | 12 points | 15 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Dégâts de zone par tour = floor((5 + 2r) × M_magie) ; Longueur maximale du mur = min(7, 3 + 2 × floor(r/5)) ; Durée = 3 + floor(r/4) ; Portée de placement = 4 ; Énergie = 12 + floor(r/3) ; Recharge = 8.

**Calculs du complément proposés :** Chance de base = min(100, max(0, 15 + r)) ; Chance au tour d'exposition t = min(100, chance de base × t) ; Brûlure par tour = floor((1 + 0,3r) × M_magie) ; Durée de brûlure = 3. Au plus une tentative de brûlure par cible et par tour pour ce mur. Sortir des flammes remet le risque à sa base ; une brûlure déjà appliquée continue, démarre au prochain tour complet et ne cumule pas ses propres exemplaires. Réapplications et cumuls entre sorts restent à revoir.

**Fonctionnement proposé :** Ligne de flammes sur des cases visibles et franchissables ; les obstacles interrompent la ligne. Elle n'est pas un mur de déplacement. Une entrée ou un tour passé dans le feu blesse, au maximum une fois par acteur et par tour pour cet effet. Les alliés et l'agent peuvent être blessés.

**Base individuelle validée :** au rang effectif 11 avec Volonté 10, 27 dégâts de feu par cible et par tour, ligne d'au plus sept cases, durée de cinq tours, placement à quatre cases, coût de 15 énergie et recharge de huit tours. Au prochain rang effectif 12, les dégâts de base deviennent 29 par cible et par tour. L'équipement, les attributs et les autres modificateurs restent identiques. Les cases de placement doivent être visibles et franchissables ; les obstacles interrompent la ligne. Le mur reste traversable. Une entrée dans ses flammes ou un tour passé dedans blesse, une seule fois par cible et par tour pour cet effet, y compris l'agent et ses alliés. Un seul mur actif par agent est validé pour FE03 ; le nouveau remplace l'ancien. Cela ne valide pas implicitement la même limite pour les autres créations persistantes. Les dégâts sont affichés avant les défenses. La fiche de base validée est conservée sous [FE03-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FE03-02.json). Les courbes complètes, le choix des extrémités et la géométrie de la ligne, les premières résolutions sur un acteur déjà présent, les chevauchements de zones, l'ordre des tours et les interactions avec la vision restent à revoir. Aucun blocage de déplacement ou contrôle n'est ajouté. Rendu natif toujours à revoir, sans intégration.

**Complément de brûlure validé :** au rang effectif 11 avec Volonté 10, chance de 25 %, puis 50 %, puis 75 %, puis 100 % selon les tours passés dans les flammes ; brûlure de quatre dégâts de feu par tour pendant trois tours. Au prochain rang effectif 12 : chance de 26 %, puis 52 %, puis 78 % ; le plafond atteint au quatrième tour reste à 100 %, la brûlure est inchangée. Une seule tentative de brûlure par cible et par tour. La sortie réinitialise le risque sans supprimer la brûlure déjà active, qui commence au prochain tour complet et ne cumule pas ses propres applications. Toutes les valeurs de base restent conservées. La fiche réunie validée est conservée sous [FE03-04](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FE03-04.json), avec l'accord sous [FE03-BURN01](../docs/reviews/refonte-agent-2026-10-06/fire-wall-burn-extension-acceptance-decisions.json). Premiers tests à la création ou à l'entrée, tours d'exposition, passages successifs dans plusieurs cases de la ligne, compteur par cible et mur, réentrées, zones superposées, résistances, priorités entre brûlures et rafraîchissement restent à revoir. La brûlure ne reçoit pas les immunités de contrôle ; HUD06 conserve les valeurs de statut par analyse sans drone. L'indication visuelle présentée est acceptée en direction ; son rendu natif reste à présenter, sans intégration.

Intention visuelle conservée du catalogue : Bande de flammes verticales, interruptions aux obstacles. Rendu non créé dans cette passe.

Indication visuelle de brûlure validée en direction : Petites flammes sur la cible brûlée après sa sortie du mur, jusqu'à la fin de son statut. Pixels et animation native restent à revoir.

### FE04 Émanation soudaine

Type : actif. Rôle révisé : Marque une zone puis la fait exploser après un délai, avec des dégâts légèrement plus élevés au centre et une chance de brûlure.

> Vous tracez des fissures incandescentes d’où jaillira une gerbe de flammes.

Nom « Émanation soudaine » demandé par le joueur. RP, valeurs actuelles et au prochain rang, écart maximal de 10 % aux bords et conditions affichées dans FE04-02 validés lors de l'accord pour continuer ; courbes et cas non présentés toujours à revoir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts au centre | 14 dégâts | 63 dégâts |
| Dégâts au bord | 12 dégâts | 57 dégâts |
| Chance de brûlure par cible touchée | 10 % | 32 % |
| Brûlure par tour | 1 dégât | 7 dégâts |
| Durée de brûlure | 3 tours | 3 tours |
| Rayon | 1 case | 1 case |
| Portée | 5 cases | 5 cases |
| Délai annoncé | 1 tour complet | 1 tour complet |
| Énergie | 12 points | 15 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Puissance avant arrondi = (14 + 4r) × M_magie ; Dégâts à la distance d = floor(Puissance × (1 − 0,10 × min(1, d/R))) pour le rayon positif R ; Chance de brûlure = min(100, max(0, 10 + 2r)) ; Brûlure par tour = floor((1 + 0,5r) × M_magie) ; Durée de brûlure = 3 ; Rayon = 1 ; Portée = 5 ; Délai annoncé = 1 ; Énergie = 12 + floor(r/3) ; Recharge = 6. Un seul arrondi final par valeur : le bord n'est pas calculé depuis les dégâts du centre déjà arrondis. La pondération réduit la puissance brute de 10 % au bord ; les valeurs affichées peuvent différer légèrement de ce rapport après arrondi.

**Fonctionnement révisé proposé :** Le marqueur est posé dès le lancement. L'explosion survient après un tour complet, en laissant une réponse aux ennemis. Le rayon est arrêté par les murs. Les acteurs peuvent quitter la zone ; les alliés peuvent être touchés. La différence selon la distance concerne les dégâts directs, pas implicitement la chance ou la puissance de brûlure. Chaque cible touchée possède son propre tirage de brûlure. Les dégâts de cette brûlure commencent au prochain tour complet après l'explosion. Les applications de cette brûlure ne se cumulent pas ; la combinaison avec une brûlure d'un autre sort, sa force et le rafraîchissement de durée restent à revoir en conservant les effets spécifiques validés. Pas de destruction de terrain ni de nouvelle carte.

**Comparaison individuelle révisée validée :** au rang effectif 11 avec Volonté 10, 59 dégâts de feu au centre et 53 au bord, chance de brûlure de 30 % par cible touchée, brûlure de six dégâts de feu par tour pendant trois tours. Rayon d'une case, portée de cinq cases, délai d'un tour complet, coût de 15 énergie et recharge de six tours. Au prochain rang effectif 12 : centre 63, bord 57, chance de brûlure 32 %, brûlure sept dégâts par tour ; les autres valeurs restent identiques. L'équipement, les attributs et les autres modificateurs sont conservés. La case visée doit être visible et à portée ; la zone est marquée avant l'explosion, les occupants peuvent en sortir et les murs bloquent la propagation. Les alliés présents peuvent être touchés. La brûlure commence au prochain tour complet après l'explosion, sans cumul de ses propres applications. Les dégâts sont affichés avant les défenses. La fiche sans tableau validée est conservée sous [FE04-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FE04-03.json), avec l'accord sous [FE04](../docs/reviews/refonte-agent-2026-10-06/emanation-description-acceptance-decisions.json). Le nom intermédiaire « Explosion » est remplacé par « Émanation soudaine » avant intégration. Les courbes complètes, la pondération précise aux distances intermédiaires, la légalité du centre, la géométrie et l'occlusion, les grandes empreintes d'acteurs, l'ordre exact des tours, l'éligibilité du lanceur aux dégâts, les résistances, priorités et rafraîchissements de brûlure, les marqueurs multiples et les changements de carte restent à revoir. Aucun maintien manuel, destruction de terrain, contrôle ou variance générale des sorts n'est ajouté. Marqueur, icône de brûlure et rendu natif toujours à revoir, sans intégration.

Intention visuelle conservée du catalogue : Fissures lumineuses au sol, puis jets de feu vers le haut. Rendu non créé dans cette passe.

### FE05 Boule de feu

Type : actif. Rôle révisé proposé : Projette une boule de feu qui provoque une grande explosion à l'impact.

> Vous projetez une boule de feu qui éclate en une déflagration dévorante.

Le joueur valide le remplacement de Foyer vorace par Boule de feu, son nom, son RP, les valeurs et conditions affichées ainsi que la direction visuelle présentée. Les courbes et cas non montrés restent à revoir ; aucun rendu natif n'est accepté ni intégré. L'ancienne fiche de foyer reste dans l'historique.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de l'explosion par cible | 18 dégâts | 68 dégâts |
| Rayon de l'explosion | 3 cases | 3 cases |
| Portée | 6 cases | 6 cases |
| Chance de brûlure par cible touchée | 15 % | 37 % |
| Brûlure par tour | 1 dégât | 5 dégâts |
| Durée de brûlure | 3 tours | 3 tours |
| Énergie | 18 points | 21 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Dégâts de l'explosion = floor((18 + 4r) × M_magie) ; Rayon = 3 ; Portée = 6 ; Chance de brûlure = min(100, max(0, 15 + 2r)) ; Brûlure par tour = floor((1 + 0,4r) × M_magie) ; Durée de brûlure = 3 ; Énergie = 18 + floor(r/3) ; Recharge = 8. Un seul arrondi final par valeur. Les dégâts sont uniformes dans le rayon admissible ; la pondération centre-bord d'Émanation soudaine n'est pas transférée à cette proposition.

**Fonctionnement révisé proposé :** La boule vise une case visible à portée et explose sur cette case ou au premier obstacle ou personnage rencontré. Les murs bloquent la propagation de l'explosion et les alliés peuvent être touchés. Chaque cible touchée possède une chance de brûlure ; cette brûlure commence au prochain tour complet après l'impact et ne cumule pas ses propres applications. Aucun foyer persistant, croissance après décès, durée de zone ou limite de foyers n'est conservé. EL-ZONE01 reste applicable aux zones persistantes d'autres sorts ; le projectile et son unique explosion ne produisent pas de compteur d'exposition. La brûlure reste un statut de cible, sans immunité de boss ou malus d'élite implicites réservés aux contrôles.

**Comparaison individuelle révisée validée :** au rang effectif 11 avec Volonté 10, 63 dégâts de feu par cible, rayon de trois cases, portée de six cases, chance de brûlure de 35 % et brûlure de cinq dégâts de feu par tour pendant trois tours. Coût de 21 énergie et recharge de huit tours. Au prochain rang effectif 12 : 68 dégâts de feu et 37 % de chance de brûlure ; seules ces valeurs changent. Même équipement, mêmes attributs et autres modificateurs. Les dégâts sont affichés avant les défenses. La fiche sans tableau validée est conservée sous [FE05-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FE05-03.json), avec l'accord sous [FE05](../docs/reviews/refonte-agent-2026-10-06/fireball-description-acceptance-decisions.json) et la demande de remplacement sous [FE05-REWORK01](../docs/reviews/refonte-agent-2026-10-06/fireball-replacement-proposal-decisions.json). La fiche de Foyer vorace est conservée dans l'historique, sans accord sur ses paramètres. La fiche acceptée d'Émanation soudaine reste inchangée. Les courbes complètes, la légalité de la case visée, le chemin précis et les collisions aux coins, l'effet au contact d'un mur, la géométrie et les grandes empreintes, l'éligibilité du lanceur aux dégâts, les jets de touche, les résistances et cumuls entre brûlures, les priorités, le rafraîchissement et l'ordre des tours restent à revoir. HUD06 conserve les valeurs des statuts dans l'analyse sans drone. Aucun contrôle, destruction de terrain, variance générale des sorts ou intégration native n'est ajouté.

Intention visuelle révisée validée : Boule incandescente suivie d'une courte traînée de flammes, puis large déflagration circulaire. Direction acceptée lors de la revue ; pixels et animation native toujours à présenter, aucun rendu créé.

### FE06 Combustion

Type : actif. Rôle complété : Consume la brûlure restante d'une cible pour produire un choc et peut provoquer une nouvelle brûlure après ce choc.

> Vous concentrez les flammes qui rongent votre cible en un embrasement brutal.

RP, valeurs actuelles et au prochain rang avec la cible d'exemple, portée, coût, recharge, conditions affichées et retour conditionnel des flammes validés pour Combustion. Courbes et cas non montrés restent à revoir ; aucun rendu natif n'est accepté ni intégré.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Part de la brûlure restante convertie | 100 % | 144 % |
| Chance d'une nouvelle brûlure | 10 % | 21 % |
| Nouvelle brûlure par tour | 1 dégât | 4 dégâts |
| Durée de la nouvelle brûlure | 3 tours | 3 tours |
| Portée | 5 cases | 5 cases |
| Énergie | 6 points | 9 points |
| Recharge | 3 tours | 3 tours |

**Progression et calculs proposés :** Part de la brûlure restante convertie = 100 + 4r ; Choc = floor(dégâts de brûlure restants avant défenses × conversion / 100) ; Chance d'une nouvelle brûlure = min(100, max(0, 10 + r)) ; Nouvelle brûlure par tour = floor((1 + 0,3r) × M_magie) ; Durée de la nouvelle brûlure = 3 ; Portée = 5 ; Énergie = 6 + floor(r/3) ; Recharge = 3. Un seul arrondi final par valeur ; la Volonté déjà incluse dans les dégâts de l'ancienne brûlure n'est pas multipliée une seconde fois pour le choc.

**Fonctionnement proposé :** Consomme la brûlure encore active appartenant à l'agent sur une cible visible. Le choc est calculé sur ses dégâts restants avant défenses, puis résolu en feu contre la cible. Sans brûlure admissible, refus sans coût. Pas de consommation des flammes d'un autre auteur ou d'une zone de sol. Après suppression de l'ancienne brûlure et résolution du choc, la compétence peut appliquer sa propre nouvelle brûlure. Elle commence au prochain tour complet et ne cumule pas ses propres applications. Une nouvelle brûlure n'est pas garantie ; la gestion d'une cible morte au choc reste à revoir.

**Comparaison individuelle validée :** au rang effectif 11, conversion de 140 % de la brûlure restante. Exemple de cible sélectionnée : brûlure propre à l'agent de 13 dégâts de feu par tour, trois tours restants, soit 39 dégâts restants avant défenses ; le choc affiche 54 dégâts de feu. Chance d'une nouvelle brûlure de 20 %, nouvelle brûlure de quatre dégâts de feu par tour pendant trois tours avec Volonté 10. Portée de cinq cases, coût de neuf énergie et recharge de trois tours. Au prochain rang effectif 12 : conversion 144 %, choc 56 dégâts sur la même cible et chance de nouvelle brûlure 21 % ; les autres valeurs restent identiques. L'équipement, les attributs, les modificateurs, les dégâts par tour et les tours restants de l'ancienne brûlure sont conservés entre les aperçus. La brûlure actuelle est supprimée ; la nouvelle éventuelle commence au prochain tour complet sans cumul de ses propres exemplaires. Dégâts affichés avant défenses, recalculés selon la cible et ses tours restants. La fiche sans tableau validée est conservée sous [FE06-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FE06-02.json), avec l'accord sous [FE06](../docs/reviews/refonte-agent-2026-10-06/combustion-description-acceptance-decisions.json) et la demande sous [FE06-REBURN01](../docs/reviews/refonte-agent-2026-10-06/combustion-new-burn-direction-decisions.json). Les courbes complètes, la sélection et priorité entre plusieurs brûlures compatibles, la conservation des fractions d'un état réel, les résistances et l'ordre des modificateurs, les cibles mortes au choc, les cumuls entre sources, le rafraîchissement, les premiers et derniers ticks et les libellés sans cible admissible restent à revoir. HUD06 conserve les valeurs des statuts dans l'analyse sans drone. EL-ZONE01 n'est pas déclenché : ni le choc ni la nouvelle brûlure ne créent une zone persistante au sol. Aucun contrôle, immunité de brûlure des boss ou variance générale des sorts n'est ajouté.

Intention visuelle complétée validée : Les flammes se resserrent sur la victime puis éclatent en un flash ; elles reprennent si une nouvelle brûlure s'applique. Direction acceptée lors de la revue ; pixels et animation native restent à présenter, aucun rendu créé.

## Froid

### FR01 Aiguille de givre

Type : actif. Rôle révisé demandé : Blesse une cible et peut bloquer ses déplacements par le Gel, sans empêcher ses autres actions.

> Vous projetez une aiguille glacée qui engourdit les membres de votre cible.

FR01-REWORK01 : remplacement du ralentissement par un Gel de cinq tours demandé puis confirmé. EL-GEL01 confirme que le Gel peut remplacer une Brûlure, et qu'une Brûlure peut remplacer un Gel. La première fiche FR01-01 reste historique ; son surcoût de déplacement, sa réussite de ralentissement et son gel conditionnel de deux tours sont retirés du cadrage courant. Le RP, la fiche chiffrée révisée et le complément visuel restent à valider.

| Paramètre | Par défaut proposé | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de froid | 6 dégâts | 38 dégâts |
| Durée de gel | 5 tours | 5 tours |
| Portée | 5 cases | 5 cases |
| Puissance de contrôle | 100 unités | 165 unités |
| Facteur propre de chance de gel | 40 % | 95 % |
| Énergie | 6 points | 9 points |
| Recharge | 2 tours | 2 tours |

**Progression et calculs révisés proposés :** Dégâts de froid = floor((8 + 3r) × M_magie × 0,85) ; Durée de gel = 5 ; Portée = 5 ; Puissance de contrôle = 100 + 5r + 2(Volonté − 5) ; Énergie = 6 + floor(r/3) ; Recharge = 2. Le facteur 0,85 est une proposition pour FR01, comparée aux dégâts directs de Braise de même rang et même Volonté ; il ne fixe pas la réduction de toute la branche Froid. La brûlure de Braise n'entre pas dans cette comparaison des dégâts directs.

**Calcul de gel proposé :** Facteur de résistance = clamp(100 × max(1, puissance) / (max(1, puissance) + max(1, résistance occulte)), 5, 95). Facteur propre = min(100, max(0, 40 + 5r)). Chance finale de gel = facteur de résistance × facteur propre / 100. Un seul test final de Gel remplace les anciens tests de ralentissement puis de gel conditionnel ; EL-FREQ01 relève le facteur propre pour rendre le Gel fréquent aux rangs élevés ; les anciennes probabilités restent historiques. Pour les élites, multiplier cette chance finale par 0,75 une seule fois, avant l'affichage au dixième ; boss admissibles au Gel selon CTRL-EL01, coefficient validé de 0,5. Conserver les fractions jusqu'à l'affichage. Les courbes complètes restent à revoir ; ce modèle ne réécrit pas les autres contrôles.

**Fonctionnement révisé demandé et complété proposé :** L'impact inflige du froid. Un Gel réussi empêche la cible de se déplacer pendant cinq tours, sans empêcher ses attaques ni ses autres actions. Il supprime une Brûlure déjà présente. Une Brûlure appliquée ensuite supprime le Gel. Une cible qui brûle reste admissible au Gel ; seuls les dégâts élémentaires, sans statut appliqué, ne produisent pas ce remplacement. Les boss peuvent recevoir le Gel ; leur malus chiffré est validé sous CTRL-EL01. Les élites conservent le malus relatif de 25 %. Les dégâts suivent leurs règles ordinaires même si le Gel échoue ou si la cible y est immunisée.

**Comparaison individuelle révisée proposée :** au rang effectif 11 avec Volonté 10, 35 dégâts de froid ; Gel de cinq tours. Sur une cible fictive de résistance occulte 25, chance de Gel de 77,8 %, ou 58,4 % contre une élite de même résistance. Portée de cinq cases, coût de neuf énergie et recharge de deux tours. Au prochain rang effectif 12 : 38 dégâts et chance de Gel 82,5 % / élite 61,9 % ; les autres valeurs restent identiques. Les dégâts directs de Braise au même profil sont respectivement 41 et 45, avant sa brûlure ; FR01 est donc un peu moins puissant sur le choc direct. Même équipement, mêmes attributs, mêmes modificateurs et même résistance de cible dans l'aperçu. Une cible hostile visible à portée et une trajectoire dégagée. Dégâts avant défenses ; probabilités pour la cible fictive, sans promettre les mêmes valeurs sur tous les monstres. La fiche révisée est préparée sous [FR01-06](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FR01-06.json), avec les demandes sous [FR01-REWORK01 / EL-GEL01](../docs/reviews/refonte-agent-2026-10-06/frost-freeze-and-burning-interaction-decisions.json).

EL-FREQ01 valide les chances ordinaires et élites affichées. Le malus des boss et l'exemple au rang 11 sont validés sous CTRL-EL01 ; les autres chiffres révisés, le RP et les conditions supplémentaires de ciblage restent proposés ; la durée demandée et l'interaction réciproque sont retenues. Les courbes complètes, le modèle de toucher, les collisions, les bornes de résistance, l'ordre impact/statut et les cibles mortes, les applications simultanées, la classification des monstres, les mouvements forcés ou spéciaux, les cumuls et rafraîchissements, les premiers et derniers tours et les aperçus sur cible inconnue restent à revoir. HUD06 conserve les valeurs des statuts par analyse sans drone sans débloquer les autres statistiques. EL-ZONE01 ne s'applique pas : projectile unique sans zone persistante. Aucun recul, perforation ou variance générale des sorts n'est ajouté.

Intention visuelle conservée du catalogue : Aiguille blanche, petits cristaux sur ses points d'appui. Rendu non créé dans cette passe.

Complément visuel proposé : Si le Gel s'applique, la glace se referme autour des pieds de la cible et les flammes d'une éventuelle Brûlure s'éteignent. Cette direction et les transitions précises restent à valider ; aucun rendu créé.

**Exception boss validée pour les valeurs affichées :** sur la même cible fictive, chance de Gel 38,9 % au rang effectif 11 puis 41,3 % au rang 12 avec le coefficient 0,5 validé. L'exemple affiché au rang 11 de 38,9 % est validé ; la valeur dérivée de 41,3 % au prochain rang et son arrondi restent non montrés. Les chances ordinaires 77,8 % / 82,5 % et élites 58,4 % / 61,9 % sont validées ; le RP et les autres chiffres auparavant proposés restent à leur statut antérieur. La suppression de l'immunité de boss au Gel est demandée, sans toucher aux autres contrôles.

### FR02 Nova de givre

Type : actif. Rôle retenu : Immobilise brièvement les ennemis proches, sans supprimer leurs attaques.

> Vous libérez une onde glaciale qui se referme sur les ennemis proches.

RP, fiche chiffrée affichée et conditions présentées validés pour FR02 ; les cas non montrés restent à revoir. Le nom, le rôle et l'orientation de l'anneau givré sont conservés. Le Gel suit EL-GEL01, la fréquence EL-FREQ01, le malus élite CTRL01 et l'exception boss CTRL-EL01 validée.

| Paramètre | Par défaut proposé | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de froid par hostile | 4 dégâts | 28 dégâts |
| Durée de gel | 2 tours | 4 tours |
| Rayon autour de l'agent | 2 cases | 2 cases |
| Puissance de contrôle | 100 unités | 165 unités |
| Facteur propre de chance de gel | 30 % | 85 % |
| Énergie | 10 points | 13 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Dégâts par hostile = floor((4 + 2r) × M_magie) ; Durée de gel = 2 + floor(r/5) ; Rayon autour de l'agent = 2 ; Puissance de contrôle = 100 + 5r + 2(Volonté − 5) ; Facteur propre de chance de gel = min(100, max(0, 30 + 5r)) ; Énergie = 10 + floor(r/3) ; Recharge = 6. Les dégâts proposés restent en dessous des impacts par tour du cône de Souffle de feu au même profil (26 contre 28 puis 28 contre 30) ; cette comparaison n'égalise pas leurs zones, durées ou coûts et ne valide pas un coefficient uniforme pour le Froid.

**Calcul du Gel proposé :** Facteur de résistance = clamp(100 × max(1, puissance) / (max(1, puissance) + max(1, résistance occulte)), 5, 95). Chance finale normale = facteur de résistance × facteur propre / 100. Chance élite = chance normale × 0,75 ; chance boss = chance normale × 0,5, sans cumuler les deux malus d'importance. Un test propre par hostile atteint ; fractions conservées jusqu'à l'affichage au dixième, demi-valeurs vers le haut proposées. Les chances affichées sont validées ; les courbes complètes et les autres résistances non montrées restent à revoir.

**Fonctionnement proposé :** L'onde part de l'agent et touche les hostiles dans son rayon ; les murs arrêtent sa propagation. Chaque cible reçoit ses dégâts de froid et son propre test de Gel. Le Gel bloque ses déplacements, sans empêcher ses attaques ni ses autres actions. Un Gel appliqué supprime sa Brûlure ; une Brûlure appliquée supprime son Gel. L'échec du contrôle ne supprime pas les dégâts ordinaires. Il s'agit d'une onde unique, sans zone persistante ni cumul d'exposition EL-ZONE01. Le statut de cible dure après le passage de l'onde.

**Comparaison individuelle validée :** au rang effectif 11 avec Volonté 10, 26 dégâts de froid par cible et quatre tours de Gel. Sur une cible fictive de résistance occulte 25 : chance de Gel 69,2 %, élite de même résistance 51,9 %, boss de même résistance 34,6 %. Rayon de deux cases autour de l'agent, coût de 13 énergie et recharge de six tours. Au prochain rang effectif 12 : 28 dégâts et chances 73,8 % / élite 55,4 % / boss 36,9 % ; les autres valeurs restent identiques. Les rangs appris passent de neuf à dix pour un point, l'équipement +2, les attributs, les modificateurs et la résistance de cible restent identiques dans les calculs. Le tooltip ne décompose pas ces rangs. Les probabilités utilisent une cible d'exemple, pas une promesse universelle. Dégâts affichés avant défenses. La fiche sans tableau validée est conservée sous [FR02-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FR02-03.json), avec l'accord sous [FR02](../docs/reviews/refonte-agent-2026-10-06/frost-nova-description-acceptance-decisions.json).

Les chiffres actuels et prochains, le RP et les conditions présentées sont validés. Les courbes complètes, la géométrie exacte aux coins et l'occlusion, les grandes cibles, l'ordre dégâts/Gel et les cibles mortes, les résistances, les mouvements forcés ou spéciaux, les cumuls et rafraîchissements, les premiers et derniers tours de statut et les cas d'ennemis cachés restent à revoir. Les previews et analyses suivent les observations réelles et HUD06, sans dévoiler de cible cachée ni ses autres statistiques ; valeurs du statut consultables sans drone. Aucun déplacement forcé, étourdissement complet, condition d'arme ou variance générale des sorts n'est ajouté.

Intention visuelle conservée du catalogue : Anneau givré qui s'élargit ; glace limitée aux pieds des cibles. Direction conservée ; pixels et animation native toujours à présenter, aucun rendu créé.

Indication visuelle complémentaire validée en direction : Les flammes d'une Brûlure remplacée s'éteignent quand la glace apparaît aux pieds de la cible. Pixels et animation précise à présenter ; aucun rendu créé.

### FR03 Mur de glace

Type : actif. Rôle complété proposé : Crée un obstacle temporaire sur des cases libres et peut geler les hostiles qui le frappent au contact.

> Vous faites surgir une barrière de glace qui verrouille le passage.

Le blocage des ennemis par les blocs présents est confirmé sous FR03-BLOCK01 : ils ne peuvent pas les traverser. Le principe du Gel défensif au contact est retenu sous FR03-GEL01 après le retour positif du joueur. Le RP, la fiche chiffrée, la limite d'un mur actif et les conditions affichées du contact sont explicitement validés par « parfait, je valide ». Accord conservé dans frost-wall-description-acceptance-decisions.json ; la fiche validée FR03-03 conserve sans modification les textes et valeurs de FR03-02. Le nom, le rôle de barrière destructible et les blocs translucides du catalogue sont conservés. Le contact est la proposition de déclenchement propre à ce sort sans attaque de dégâts, conformément à EL-STAT01 ; aucune aura ni dégâts de contact ne sont ajoutés.

| Paramètre | Par défaut proposé | Actuel dans l'exemple |
| --- | --- | --- |
| PV de chaque bloc | 10 PV | 47 PV |
| Longueur maximale | 3 cases | 7 cases |
| Durée du mur | 3 tours | 5 tours |
| Portée de placement | 4 cases | 4 cases |
| Murs actifs par agent | 1 | 1 |
| Durée du gel de contact | 2 tours | 2 tours |
| Puissance de contrôle | 100 unités | 165 unités |
| Facteur propre de chance de gel | 35 % | 90 % |
| Énergie | 12 points | 15 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** PV par bloc = floor((10 + 3r) × M_magie) ; Longueur maximale = min(7, 3 + 2 × floor(r/5)) ; Durée du mur = 3 + floor(r/5) ; Portée = 4 ; Murs actifs = 1 ; Durée du Gel = 2 ; Puissance de contrôle = 100 + 5r + 2(Volonté − 5) ; Facteur propre = min(100, max(0, 35 + 5r)) ; Énergie = 12 + floor(r/3) ; Recharge = 8. Le sort conserve son rôle d'obstacle sans dégâts propres ; EL-BAL01 ne crée pas de dégâts de Froid pour les compétences sans attaque.

**Calcul du Gel proposé :** Facteur de résistance = clamp(100 × max(1, puissance) / (max(1, puissance) + max(1, résistance occulte)), 5, 95). Chance normale = facteur de résistance × facteur propre / 100. Chance élite = normale × 0,75 ; chance boss = normale × 0,5, sans cumuler les malus. Un test maximum par attaquant et par tour pour le mur de cet agent ; l'attaque doit toucher un bloc au corps à corps. Le contact peut geler même si le coup détruit le bloc. Fractions conservées jusqu'à l'affichage au dixième ; demi-valeurs vers le haut proposées. Les chances affichées et les conditions du contact présentées sont validées ; les formules internes, courbes complètes et cas non montrés restent à revoir.

**Fonctionnement proposé :** Blocs uniquement sur des cases connues, visibles, libres et franchissables. Les ennemis ne peuvent pas traverser une case occupée par un bloc du mur, règle explicitement confirmée. Les blocs arrêtent aussi les projectiles ; leur destruction séparée est validée dans la fiche individuelle. Un nouveau mur remplace le précédent, limite présentée pour FR03 sans valider les autres créations. Un hostile qui touche un bloc au corps à corps peut subir deux tours de Gel, même si ce contact brise le bloc ; une seule tentative par attaquant et par tour. Les tirs et dégâts indirects ne produisent pas ce contact de mêlée. Le Gel bloque les déplacements sans supprimer attaques ou autres actions ; un Gel appliqué supprime la Brûlure, une Brûlure appliquée supprime le Gel. CTRL01, CTRL-EL01 et HUD06 sont conservés.

**Comparaison individuelle proposée :** au rang effectif 11 avec Volonté 10, chaque bloc a 44 PV, le mur mesure au plus sept cases, dure cinq tours, se place à quatre cases, avec un seul mur actif. Gel de contact de deux tours. Sur la cible fictive de résistance occulte 25, chance de Gel 73,5 %, élite 55,1 %, boss 36,8 %. Coût 15 énergie, recharge huit tours. Au prochain rang effectif 12 : chaque bloc a 47 PV et chances 78,2 % / élite 58,6 % / boss 39,1 % ; autres valeurs inchangées. Même équipement +2, Volonté 10, autres modificateurs et résistance de cible, rang appris neuf puis dix pour un point. Le tooltip ne décompose pas les rangs. La fiche sans tableau est préparée sous [FR03-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FR03-03.json). Les chances sont celles d'une cible d'exemple, pas de toutes les futures attaques.

Le RP, les chiffres affichés, la tentative maximale, l’application sur bloc brisé, le placement, le blocage des ennemis, les interceptions et la limite/recréation sont validés. Le mur reste destructible ; les déplacements forcés ou spéciaux ne sont pas validés par cette fiche. Les courbes complètes, les extrémités et coins de la ligne, les grandes empreintes, les objets/cadavres au sol, les attaques refusées ou manquées, les types exacts de projectiles, les attaques à plusieurs coups ou blocs, les défenses des blocs, le décompte des tours, les cumuls/rafraîchissements et les effets simultanés restent à revoir. La proposition précédente de disparition sans dégâts, d'absence de butin/XP et de non-repoussement de l'occupant reste non montrée. Aucun aura de proximité, dégâts supplémentaires, étourdissement complet ou révélation de cible cachée n'est ajouté. EL-ZONE01 ne crée pas de cumul : le contact de mêlée n'est pas un séjour dans une zone de statut franchissable. Les aperçus gardent les observations réelles et les valeurs des statuts restent consultables par analyse sans drone selon HUD06 ; autres statistiques conservées à leur accès actuel.

Intention visuelle conservée du catalogue : Blocs translucides qui montent depuis le sol et se fissurent à la fin. Direction conservée ; pixels et animation native toujours à présenter, aucun rendu créé.

Complément visuel proposé : À l'application du Gel, le givre remonte aux pieds de l'attaquant et les flammes d'une Brûlure remplacée s'éteignent. Cette direction est validée ; pixels et animation native toujours à présenter, aucun rendu créé.

### FR04 Vague de froid

Type : actif. Refonte demandée sous FR04-VAGUE01 : une vague de Froid part du joueur, avance d'une case par tour dans une direction choisie et inflige des dégâts avec une chance de Gel à chaque monstre touché. Forme générale demandée : une bande large directionnelle. Le nom « Vague de froid » est explicitement validé dans la fiche complète.

> Vous déchaînez une vague glaciale qui balaie les ennemis devant vous.

La proposition précédente d'orbe fragmenté et d'éclats secondaires est remplacée sous le même identifiant FR04 à la demande du joueur. FR04-01 et les anciens aperçus restent des archives, sans ajouter une compétence. Fiche complète validée par « parfait :) » : nom,RP,chiffres affichés,dimensions,conditions du sort et direction visuelle. Accord conservé dans frost-wave-description-acceptance-decisions.json ; FR04-04 conserve les textes et valeurs de FR04-03. Aucun rendu créé ni intégration en jeu. Le placement avec aperçu de la bande de départ, largeur affichée et flèche de direction est explicitement validé sous FR04-PREV01, dans frost-wave-placement-preview-acceptance-decisions.json.

| Paramètre | Par défaut proposé | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de Froid par cible touchée | 4 dégâts | 28 dégâts |
| Avancée | 1 case par tour | 1 case par tour |
| Largeur | 3 cases | 3 cases |
| Épaisseur du front | 1 case | 1 case |
| Distance maximale | 5 cases | 5 cases |
| Préparation avant la bande voisine | 0 tour | 0 tour |
| Durée du Gel | 3 tours | 3 tours |
| Puissance de contrôle | 100 unités | 165 unités |
| Facteur propre de chance de Gel | 25 % | 80 % |
| Énergie | 14 points | 17 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** avec r = rang effectif − 1, dégâts par cible touchée = floor((4 + 2r) × M_magie) ; avancée = 1 case par tour ; largeur = 3 ; épaisseur = 1 ; distance maximale = 5 ; bande voisine appliquée à l'activation sans préparation ; Gel = 3 tours ; puissance de contrôle = 100 + 5r + 2(Volonté − 5) ; facteur propre = min(100, max(0, 25 + 5r)) ; énergie = 14 + floor(r/3) ; recharge = 8. Les valeurs actuelles et du prochain rang affichées sont validées ; les paramètres internes et courbes complètes restent à revoir. La Volonté participe aux dégâts et à la chance de Gel ; courbes complètes à revoir. Aucun coefficient universel de Froid/Feu ni variance de dégâts d'arme n'est ajouté.

**Calcul du Gel proposé :** facteur de résistance = clamp(100 × max(1, puissance) / (max(1, puissance) + max(1, résistance occulte)), 5, 95). Chance normale = facteur de résistance × facteur propre / 100 ; élite = normale × 0,75 ; boss = normale × 0,5, sans cumul de malus. Un test indépendant par hostile touché, au maximum une fois par tour pour cette vague ; un nouveau contact à un tour suivant peut infliger les dégâts et tenter le Gel à nouveau. Fractions conservées jusqu'à l'affichage au dixième, demi-valeurs vers le haut proposées.

**Fonctionnement proposé :** la bande de départ est choisie autour du joueur avant lancement ; l’aperçu se déplace avec le choix et montre sa largeur ainsi qu’une flèche de direction, règles de placement validées sous FR04-PREV01. La confirmation lance le sort depuis cette bande dans la direction correspondante et dépense l’énergie ; les ajustements de l’aperçu ne dépensent pas d’énergie. La vague garde ensuite la direction choisie, avance de façon autonome et peut traverser plusieurs monstres ; le joueur peut agir pendant son trajet. Seuls les hostiles sont affectés. Largeur proposée trois cases, front d'une case et trajet maximal cinq cases. La première bande voisine s'applique au lancement ; le délai de l'ancien orbe n'est pas repris. Les murs arrêtent les portions de vague qu'ils rencontrent ; une portion interrompue ne réapparaît pas derrière l'obstacle. La vague ne laisse pas de zone de Froid persistante au sol, ni d'éclats secondaires. Le Gel de trois tours empêche de se déplacer sans supprimer attaques ou autres actions ; un Gel appliqué supprime la Brûlure et une Brûlure appliquée supprime le Gel. CTRL01, CTRL-EL01 et HUD06 restent conservés. Un échec du Gel n'annule pas les dégâts.

**Comparaison individuelle proposée :** rang effectif 11 avec Volonté 10 : 26 dégâts de Froid par cible avant défenses ; sur la cible fictive de résistance occulte 25, Gel 64,9 %, élite 48,6 %, boss 32,4 %. Largeur trois cases, trajet cinq cases, avancée d'une case par tour, Gel trois tours, énergie 17, recharge huit tours. Au rang effectif 12 : dégâts 28 et chances 69,5 % / élite 52,1 % / boss 34,7 % ; autres valeurs inchangées. Même équipement +2, Volonté 10, autres modificateurs et résistance de cible ; rang appris neuf puis dix pour un point. Le tooltip ne décompose pas les rangs. Fiche sans tableau sous [FR04-04](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FR04-04.json). Les chances sont celles d'une cible d'exemple.

La demande du joueur valide la direction générale, le départ autour du joueur, l’avancée d’une case par tour et les dégâts avec chance de Gel lors des contacts. L’ajustement de la bande avant lancement, son aperçu indiquant largeur et direction, et la confirmation lançant le sort avec dépense d’énergie sont validés sous FR04-PREV01. Nom, RP, largeur et trajet affichés, valeurs actuelles/prochain rang, paragraphe de placement, progression autonome, contacts hostiles et traversée des monstres, limite d'un contact par cible/tour, arrêt par portion et absence de zone persistante sont validés dans la fiche complète. Les conditions internes non montrées conservent leur statut de revue. Courbes complètes, géométrie diagonale, empreintes des grandes cibles, placement initial, visibilité, échantillonnage du déplacement des acteurs et du front, ordre dégâts/Gel et cibles mortes, résistances, nombre de vagues simultanées, instant de mesure des statistiques, changement de carte, décompte des tours, cumuls/rafraîchissements et déplacements forcés ou spéciaux restent non montrés. EL-ZONE01 reste conservé pour les zones de statut persistantes et la remise à zéro à la sortie ; aucun cumul temporel n'est ajouté implicitement à ce front sans zone laissée au sol. Les statuts et leurs valeurs restent consultables sans drone selon HUD06, sans débloquer les autres statistiques de cible ni révéler d'acteur caché.

Nouvelle direction visuelle proposée : Un front de brume bleutée et de cristaux de givre avance en bande depuis le joueur ; givre aux pieds des cibles gelées et extinction des flammes d'une Brûlure remplacée. L'ancienne direction d'orbe à facettes et de jets secondaires est remplacée pour FR04 uniquement. Direction de l’effet du sort explicitement validée dans la fiche complète ; rendu natif toujours à présenter, aucun rendu créé. L’intention de l’aperçu initial montrant la largeur de la bande et une flèche de direction est validée ; positions exactes, couleurs, opacité, pixels de flèche, interactions d’entrée, terrain bloqué ou inconnu, annulation et comptabilité de temps/RNG/recharge restent non montrés. Le trajet complet de cinq cases n’a pas été demandé comme aperçu et n’est pas ajouté implicitement.

### FR05 Blizzard

Type : actif. Rôle conservé : Une zone subit des impacts de Froid répartis sur plusieurs tours. Gel par cible frappée proposé sous FR05-GEL01 avec cumul d'exposition EL-ZONE01.

> Vous déchaînez une tempête de glace qui pilonne la zone choisie.

RP, fiche chiffrée, délai initial, impacts aléatoires et conditions du Gel affichées validés provisoirement. Le joueur les trouve très hasardeux mais valide pour l’instant ; utilité à réévaluer lors des futurs essais. Accord dans blizzard-provisional-description-acceptance-decisions.json ; FR05-02 conserve les textes et valeurs de FR05-01. Le nom Blizzard et la direction générale de grêle sont conservés ; aucune intégration en jeu ni création de rendu.

| Paramètre | Par défaut proposé | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de Froid par impact | 5 dégâts | 27 dégâts |
| Impacts par tour | 2 | 4 |
| Durée | 3 tours | 5 tours |
| Rayon | 2 cases | 2 cases |
| Portée | 5 cases | 5 cases |
| Délai avant les premiers impacts | 1 tour complet | 1 tour complet |
| Durée du Gel | 3 tours | 3 tours |
| Puissance de contrôle | 100 unités | 165 unités |
| Facteur propre de chance de Gel | 30 % | 85 % |
| Énergie | 16 points | 19 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** r = rang effectif − 1 ; dégâts par impact = floor((6 + 2r) × M_magie × 0,9) ; impacts par tour = min(5, 2 + floor(r/5)) ; durée = 3 + floor(r/5) ; rayon = 2 ; portée = 5 ; premiers impacts après un tour complet ; Gel = 3 tours ; puissance de contrôle = 100 + 5r + 2(Volonté − 5) ; facteur propre = min(100, max(0, 30 + 5r)) ; énergie = 16 + floor(r/3) ; recharge = 10. Le coefficient 0,9 est proposé pour FR05 uniquement afin de garder des dégâts de Froid légèrement sous les exemples de Feu comparables ; aucun coefficient universel ni variance d'arme ajouté. La formule précédente non validée est conservée dans les archives, pas utilisée pour cette fiche.

**Gel et exposition proposés :** facteur de résistance = clamp(100 × max(1, puissance) / (max(1, puissance) + max(1, résistance occulte)), 5, 95). Chance du premier tour = facteur de résistance × facteur propre / 100. Chance du tour d'exposition n = min(100 %, chance initiale × n), puis élite × 0,75 ou boss × 0,5, sans cumul des malus. La présence consécutive compte même si aucune frappe n'a atteint la cible aux tours précédents. Le test n'a lieu que si un impact frappe la cible, maximum un test par cible frappée et par tour. La sortie remet le cumul à zéro. Fractions exactes jusqu'à l'affichage au dixième, demi-valeurs vers le haut proposées. EL-ZONE01, CTRL01 et CTRL-EL01 restent conservés ; détails d'échantillonnage proposés.

**Fonctionnement proposé :** premiers impacts au prochain tour complet. À chaque tour, quatre cases distinctes dans l'exemple sont tirées au hasard sans remplacement parmi les cases de la zone accessibles à l'effet, occupées ou non. Une case reçoit au plus un impact par tour. Les murs limitent l'aire. Les alliés peuvent être blessés et gelés. Un Gel appliqué bloque le déplacement sans supprimer attaques ou autres actions et retire la Brûlure ; une Brûlure appliquée retire le Gel. Un échec du Gel n'annule pas les dégâts. Une cible frappée a son test propre ; le cas d'un grand acteur occupant plusieurs cases reste non montré. HUD06 conserve l'accès aux valeurs des statuts sans drone, sans ouvrir les autres statistiques de cible.

**Comparaison individuelle proposée :** rang effectif 11, Volonté 10 : 25 dégâts de Froid par impact avant défenses, quatre impacts par tour, durée cinq tours, rayon deux cases, portée cinq cases, délai initial un tour complet, Gel trois tours, énergie 19, recharge dix tours. Sur la cible fictive de résistance occulte 25 : chance initiale 69,2 %, élite 51,9 %, boss 34,6 % ; dès le deuxième tour consécutif : 100 % / élite 75 % / boss 50 %. Au rang effectif 12 : dégâts 27 et chances initiales 73,8 % / élite 55,4 % / boss 36,9 % ; autres valeurs et plafonds inchangés. Même équipement +2, Volonté 10, autres modificateurs et résistance de cible ; rang appris neuf puis dix pour un point. Le tooltip ne décompose pas les rangs. Fiche sans tableau proposée sous [FR05-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FR05-02.json). Les chances sont celles d'une cible d'exemple.

RP, chiffres affichés, délai, impacts aléatoires sur cases vides possibles, effets sur alliés, test de Gel sur impact et règle d’exposition même entre impacts sont validés provisoirement, avec une réserve explicite sur leur utilité pratique. Le coefficient interne propre de dégâts et les courbes complètes non montrées restent à revoir. Courbes complètes, géométrie/visibilité/terrain bloqué, sélection et manque de cases admissibles, empreintes des grands acteurs, resampling, entrée tardive et sortie entre échantillons, origine de la durée, ordre tour/dégâts/Gel et acteurs morts, résistances, zones superposées et limites, instant de mesure des statistiques, changement de carte, décompte des tours, cumuls/rafraîchissements et déplacements forcés ou spéciaux restent non montrés. Pas d'étourdissement complet, de révélation cachée ni de modification automatique des armes ou secondaires.

Intention visuelle conservée du catalogue : Grêle oblique et petites frappes dispersées dans la zone. Complément proposé : givre aux pieds des cibles gelées et extinction des flammes d'une Brûlure remplacée. Direction visuelle affichée validée provisoirement ; rendu natif toujours à présenter, aucun rendu créé.

### FR06 Explosion cristalline

Type : actif. Refonte demandée sous FR06-DET01 : une attaque fait exploser tous les hostiles gelés visibles ; chaque source subit les dégâts ainsi que les ennemis autour d'elle, puis perd son Gel. Nom Explosion cristalline demandé et validé par le joueur. Les dégâts ne constituent pas une mort automatique.

> Vous faites voler en éclats la glace qui emprisonne vos ennemis.

Le précédent passif d'éclats arrière est remplacé sous le même identifiant FR06, à la demande du joueur. Revue de direction dans frozen-enemies-detonation-direction-decisions.json ; réponse de fonctionnement dans frozen-enemies-detonation-mechanics-decisions.json. FR06-01 reste une archive ; pas de compétence supplémentaire. Ancien déclencheur d'attaque directe, cap de quatre cibles arrière, coût nul, limite de passif et jets de Gel secondaires non repris dans le nouvel actif.

| Paramètre | Par défaut proposé | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de Froid sur chaque source gelée | 10 dégâts | 47 dégâts |
| Dégâts de Froid par explosion alentour | 5 dégâts | 23 dégâts |
| Rayon de chaque explosion | 1 case | 1 case |
| Sources initiales | Tous les hostiles gelés visibles | Tous les hostiles gelés visibles |
| Gel des sources après détonation | Consommé | Consommé |
| Énergie | 17 points | 20 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** r = rang effectif − 1 ; dégâts sur source gelée = floor((10 + 3r) × M_magie), avec M_magie = 1 + 0,02 × (Volonté − 5) ; dégâts alentour = floor(dégâts sur source gelée / 2), calculés avant les défenses des cibles ; rayon = 1 ; sources = tous les hostiles gelés visibles au lancement, sans cap numérique ; énergie = 17 + floor(r/3) ; recharge = 8. Aucun multiplicateur critique général ni variance de dégâts d'arme ajouté. Gel préalable obligatoire ; l'importance de la cible ne réduit pas les dégâts de ce sort.

**Fonctionnement proposé :** pool initial de tous les hostiles gelés réellement visibles au lancement. Chaque source provoque une explosion et prend ses dégâts directs ; les hostiles autour prennent les dégâts alentour, même s'ils ne sont pas gelés. Plusieurs explosions peuvent infliger des dégâts à la même cible ; une source peut donc prendre ses dégâts directs et les dégâts alentour des autres sources. Les murs limitent chaque explosion locale. Le Gel de chaque source est consommé après sa détonation, comme demandé. Les voisins non gelés ne deviennent pas sources et il n'y a pas de chaîne de nouvelles détonations dans ce lancement. Le nouveau sort ne provoque pas de Gel : exception FR06 acceptée explicitement à la règle de branche, autres compétences de Froid inchangées. Un boss déjà gelé est admissible selon CTRL-EL01 ; aucune nouvelle immunité de boss ni réduction de dégâts selon l'importance. Les acteurs cachés ne sont pas ajoutés au pool initial et aucune statistique cachée n'est révélée. HUD06 conserve l'accès aux valeurs des statuts et la disparition du Gel consommé. Pas d'attaque d'arme requise, pas d'aire persistante ni de cumul EL-ZONE01.

**Comparaison individuelle acceptée :** rang effectif 11, Volonté 10 : 44 dégâts de Froid sur chaque source gelée avant défenses ; 22 dégâts de Froid par explosion sur les voisins, rayon une case, tous les hostiles gelés visibles admissibles, énergie 20 et recharge huit tours. Rang effectif 12 : source 47, alentour 23 ; autres valeurs inchangées. Même équipement +2, Volonté 10 et autres modificateurs ; rang appris neuf puis dix pour un point. Le tooltip ne décompose pas les rangs. Fiche sans tableau acceptée, avec seul changement de nom, sous [FR06-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-FR06-03.json). Décision conservée dans crystalline-explosion-description-acceptance-decisions.json.

Le joueur demande le remplacement actif, toutes les sources gelées visibles, les dégâts sur sources et alentour et la disparition du Gel. RP, dégâts actuels/prochain rang, rayon un, coût/recharge, snapshot initial affiché, cumul des explosions, ciblage hostile des voisins, arrêt par murs et absence de nouveau Gel sont acceptés tels que montrés, avec le seul changement de nom demandé. Courbes complètes, union exacte des observations du joueur/drones, géométrie diagonale et grandes empreintes, sources tuées par un autre dégât avant leur propre détonation, ordre simultané des paquets/défenses et nettoyage du Gel, effets à la mort, instant de mesure des statistiques, preview/contrôles, cast sans cible ou refusé et comptabilité temps/énergie/RNG/recharge, changements de carte et interactions restent non montrés.

Nouvelle direction visuelle acceptée telle que montrée : détonations de givre autour des sources gelées ; leur couche de glace se brise et les survivants restent à leur emplacement. L'ancienne direction d'éventail arrière est remplacée pour FR06 seulement. Le rendu natif reste à présenter séparément ; aucun rendu créé.

## Énergie occulte

### A01 Trait arcanique

Type : actif. Rôle retenu : Frappe directement une cible avec de l'énergie occulte.

> Vous condensez l’énergie occulte en un trait qui frappe votre ennemi.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts directs | 8 dégâts | 45 dégâts |
| Portée | 5 cases | 5 cases |
| Énergie | 6 points | 9 points |
| Recharge | 2 tours | 2 tours |

**Progression et calculs proposés :** Dégâts directs = floor((8 + 3r) × M_magie) ; Portée = 5 ; Énergie = 6 + floor(r/3) ; Recharge = 2.

**Fonctionnement proposé :** Reprend A01 du lot 02 : hostile visible avec ligne de vue, pas de jet de précision d'arme, défenses occultes ordinaires. Le sort ne donne aucun affaiblissement supplémentaire.

Intention visuelle conservée du catalogue : Trait violet court, nœud noir au centre de l'impact. Rendu non créé dans cette passe.

**Comparaison individuelle acceptée :** rang effectif 11, Volonté 10 : 41 dégâts occultes avant défenses, portée cinq cases, énergie neuf et recharge deux tours. Rang effectif 12 : 45 dégâts ; autres valeurs inchangées. Même équipement +2, Volonté 10 et autres modificateurs ; rang appris neuf puis dix pour un point. Aucun passif de Canalisation ni implant initial supposé dans cet exemple ; le premier sort de l’Arcaniste n’est pas imposé. Fiche sans tableau validée sous [A01-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-A01-02.json). Décision conservée dans arcane-bolt-description-acceptance-decisions.json.

RP, valeurs actuelles/prochain rang, cible hostile visible sans obstacle et absence de dépendance à la précision d’arme sont acceptés tels que montrés. Courbes complètes, cumul d’équipement/passifs/implants, géométrie de ligne de vue et coins/grandes empreintes, union de vision joueur/drones, autres acteurs sur le trajet, détails des défenses occultes, instant de mesure/ordre des dégâts, preview, comptabilité des refus temps/énergie/RNG/recharge et absorptions, changements de carte et interactions restent non montrés. La proposition historique de refus sans coût du lot 02 reste en attente ; elle ne reçoit pas d’accord supplémentaire par cette fiche. Direction de trait violet court et nœud noir à l’impact acceptée telle que montrée ; rendu natif en attente.


### A02 Déflagration occulte

Type : actif. Rôle retenu : Frappe les acteurs d'une petite zone choisie, selon la fiche existante.

> Vous faites éclater un sceau et libérez l’énergie occulte qu’il renferme.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par acteur | 6 dégâts | 30 dégâts |
| Zone centrée | 3 × 3 cases | 3 × 3 cases |
| Portée | 4 cases | 4 cases |
| Énergie | 10 points | 13 points |
| Recharge | 4 tours | 4 tours |

**Progression et calculs proposés :** Dégâts par acteur = floor((6 + 2r) × M_magie) ; Zone centrée = 3 × 3 cases ; Portée = 4 ; Énergie = 10 + floor(r/3) ; Recharge = 4.

**Fonctionnement proposé :** Reprend A02 du lot 02 : case centrale visible, murs qui coupent l'effet. Agent et alliés peuvent être touchés. Le terrain n'est ni incendié ni détruit. La zone reste de trois par trois cases.

Intention visuelle conservée du catalogue : Sceau au sol puis expansion de fragments anguleux. Rendu non créé dans cette passe.

**Comparaison individuelle proposée :** rang effectif 11, Volonté 10 : 28 dégâts occultes par cible avant défenses, zone de trois par trois cases centrée sur une case visible sans obstacle, portée quatre cases, énergie 13 et recharge quatre tours. Rang effectif 12 : 30 dégâts ; autres valeurs inchangées. Même équipement +2, Volonté 10 et autres modificateurs ; rang appris neuf puis dix pour un point. Aucun passif de Canalisation ni implant initial supposé ; premier sort de l’Arcaniste non imposé. Fiche sans tableau à valider sous [A02-01](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-A02-01.json).

RP, valeurs actuelles/prochain rang, choix d’une case visible sans obstacle, aperçu des cases touchées et acteurs déjà repérés, dégâts possibles sur le joueur et ses alliés dont les drones, et arrêt aux murs sont proposés pour revue individuelle. Courbes complètes, cumul d’équipement/passifs/implants, géométrie de ligne de vue et coins/obstacles, grandes empreintes, union de vision joueur/drones, terrain inconnu et acteurs cachés face à l’aperçu, détails des défenses occultes, instant de mesure/ordre des dégâts, comptabilité des refus temps/énergie/RNG/recharge et absorptions, terrain/changements de carte et autres interactions restent non montrés. Direction de sceau au sol puis fragments anguleux conservée du catalogue ; rendu natif en attente.


### A03 Canalisation

Type : passif. Rôle retenu : Réduit les coûts magiques ; −20 % au premier rang confirmé.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Réduction des coûts magiques | 20 % | 35,7 % |

**Progression et calculs proposés :** Réduction des coûts magiques = 20 + 30r/(n + 9).

**Fonctionnement proposé :** Reprend A03 du lot 02 et son rang initial accepté. Tous les modificateurs applicables sont combinés avant un unique arrondi supérieur ; un coût positif reste au moins égal à un. Pas de réduction des munitions, du temps ou des déploiements purement technologiques. Le pas furtif gratuit au rang 10 conserve sa propre exception.

Intention visuelle conservée du catalogue : Impulsion resserrée vers les implants lors du paiement d'un sort. Rendu non créé dans cette passe.

### OC04 Prison sigillaire

Type : actif. Rôle retenu : Enferme temporairement une zone par des obstacles occultes.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV de chaque pilier | 10 PV | 35 PV |
| Durée | 2 tours | 4 tours |
| Périmètre | 8 cases autour d’un centre de 3 × 3 | 8 cases autour d’un centre de 3 × 3 |
| Portée du centre | 4 cases | 4 cases |
| Énergie | 14 points | 17 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** PV de chaque pilier = floor((10 + 2r) × M_magie) ; Durée = 2 + floor(r/5) ; Périmètre = 8 cases autour d’un centre de 3 × 3 ; Portée du centre = 4 ; Énergie = 14 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** Le centre peut contenir un acteur ; toutes les cases du périmètre doivent être visibles, libres et franchissables. Les piliers deviennent des obstacles destructibles séparément, sans XP ni loot. Aucun acteur n'est déplacé à la création. Expiration et destruction n'ajoutent pas de dégâts.

Intention visuelle conservée du catalogue : Piliers minces reliés par des glyphes, espace intérieur visible. Rendu non créé dans cette passe.

### OC05 Rayon vorace

Type : actif. Rôle retenu : Un rayon entretenu vole une part de la vie infligée à sa cible.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par résolution | 6 dégâts | 30 dégâts |
| Vie rendue sur les PV effectivement retirés | 10 % | 17,9 % |
| Résolutions maximales | 3 | 3 |
| Portée | 5 cases | 5 cases |
| Énergie par résolution | 4 points | 6 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Dégâts par résolution = floor((6 + 2r) × M_magie) ; Vie rendue sur les PV effectivement retirés = 10 + 15r/(n + 9) ; Résolutions maximales = 3 ; Portée = 5 ; Énergie par résolution = 4 + floor(r/4) ; Recharge = 6.

**Fonctionnement proposé :** Premier rayon immédiat puis deux maintiens au maximum, chacun coûtant son action et son énergie. Le vol de vie est calculé après défenses, arrondi au supérieur et plafonné aux PV manquants et aux PV retirés. Mouvement, autre action, perte de PV ou perte de cible termine le rayon. Pas de soin sur un obstacle, un leurre ou une cible ne perdant aucun PV.

Intention visuelle conservée du catalogue : Fil sombre continu, petites impulsions revenant vers l'agent. Rendu non créé dans cette passe.

### OC06 Rupture dimensionnelle

Type : actif. Rôle retenu : Relie deux cases visibles par des passages temporaires dans la même zone.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Durée | 3 tours | 5 tours |
| Distance maximale entre passages | 4 cases | 6 cases |
| Portée de chaque case depuis l'agent | 5 cases | 5 cases |
| Énergie | 16 points | 19 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Durée = 3 + floor(r/5) ; Distance maximale entre passages = min(6, 4 + floor(r/5)) ; Portée de chaque case depuis l'agent = 5 ; Énergie = 16 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** Deux cases actuellement visibles, libres et franchissables de la carte active. Entrer dans un passage mène à l'autre en un déplacement ordinaire ; une sortie bloquée refuse la traversée. Alliés et hostiles peuvent utiliser la liaison. Pas d'autre couche, de carte bonus générée ou d'échange gratuit depuis un menu. Un seul couple à la fois.

Intention visuelle conservée du catalogue : Deux fentes étroites portant le même sceau de liaison. Rendu non créé dans cette passe.

## Transformations

### M01 Mue prédatrice — Prédateur 62

Type : actif. Rôle retenu : Forme de contact mobile avec Griffe et Bond, selon la fiche existante.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Griffe ordinaire | 8 dégâts | 33 dégâts |
| Durée | 6 tours | 9 tours |
| Allonge de Griffe | 1 case | 1 case |
| Distance de Bond | 2 cases | 2 cases |
| Énergie de Bond | 4 | 4 |
| Recharge de Bond | 3 tours | 3 tours |
| Énergie | 12 points | 15 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Griffe ordinaire = floor((8 + 2r) × M_mêlée) ; Durée = 6 + floor(r/3) + min(3, floor(max(0, Volonté − 5)/20)) ; Allonge de Griffe = 1 ; Distance de Bond = 2 ; Énergie de Bond = 4 ; Recharge de Bond = 3 ; Énergie = 12 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** Reprend M01 du lot 02. Griffe : attaque naturelle physique, une action, sans énergie supplémentaire. Bond : action distincte, trajet connu et franchissable, sans frappe gratuite. L'entrée dans la forme est magique ; Bond reste naturel et n'est pas réduit par Canalisation.

Intention visuelle conservée du catalogue : Dessin 62, griffures basses et bond en silhouette compacte. Rendu non créé dans cette passe.

### TR61 Colosse biomécanique — 61

Type : actif. Rôle retenu : Forme de frappe lourde avec onde de choc et poussées.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Coup lourd ordinaire | 12 dégâts | 50 dégâts |
| Onde de choc par cible | 6 dégâts | 31 dégâts |
| Recul de l'onde | 1 case | 2 cases |
| Durée | 5 tours | 8 tours |
| Rayon de l'onde | 1 case | 1 case |
| Énergie de l'onde | 6 | 6 |
| Recharge de l'onde | 4 tours | 4 tours |
| Énergie | 14 points | 17 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Coup lourd ordinaire = floor((12 + 3r) × M_mêlée) ; Onde de choc par cible = floor((6 + 2r) × M_mêlée) ; Recul de l'onde = min(2, 1 + floor(r/5)) ; Durée = 5 + floor(r/3) + min(3, floor(max(0, Volonté − 5)/20)) ; Rayon de l'onde = 1 ; Énergie de l'onde = 6 ; Recharge de l'onde = 4 ; Énergie = 14 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** Coup lourd : attaque naturelle au contact. Onde : action physique distincte sur les hostiles adjacents, avec précision et recul sur trajet libre. Déplacements ordinaires de la forme coûtant 25 % de temps en plus, attaques conservant une action. Pas de dégâts de collision ni de hausse de PV qui soigne à l'entrée.

Intention visuelle conservée du catalogue : Dessin 61, lourdes secousses au sol autour des appuis. Rendu non créé dans cette passe.

### TR63 Cuirassé — 63

Type : actif. Rôle retenu : Forme qui encaisse et prépare une contre-attaque avec les coups reçus.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Heurt ordinaire | 6 dégâts | 31 dégâts |
| Réduction des dégâts directs | 10 % | 23,1 % |
| Réserve maximale de riposte | 10 % des PV maximum | 20,5 % des PV maximum |
| Durée | 5 tours | 8 tours |
| Dégâts de base de Décharge de plaques | 5 dégâts | 30 dégâts |
| Énergie de Décharge de plaques | 6 | 6 |
| Recharge de Décharge de plaques | 4 tours | 4 tours |
| Énergie | 14 points | 17 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Heurt ordinaire = floor((6 + 2r) × M_mêlée) ; Réduction des dégâts directs = 10 + 25r/(n + 9) ; Réserve maximale de riposte = 10 + 20r/(n + 9) ; Durée = 5 + floor(r/3) + min(3, floor(max(0, Volonté − 5)/20)) ; Dégâts de base de Décharge de plaques = floor((5 + 2r) × M_mêlée) ; Énergie de Décharge de plaques = 6 ; Recharge de Décharge de plaques = 4 ; Énergie = 14 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** La réserve reçoit les PV réellement perdus sur les coups directs d'hostiles, dans sa limite. Décharge de plaques est une action au contact : ses dégâts de base plus la réserve sont résolus contre la cible, puis la réserve est vidée même sur échec. Pas de charge sur poison, feu de sol ou dégâts alliés. La sortie de forme efface la réserve.

Intention visuelle conservée du catalogue : Dessin 63, plaques en charge puis ouverture lumineuse à la riposte. Rendu non créé dans cette passe.

### TR64 Possédé — 64

Type : actif. Rôle retenu : Forme orientée magie, lançant des attaques occultes depuis ses organes transformés.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Trait corporel ordinaire | 8 dégâts | 45 dégâts |
| Portée de Trait corporel | 4 cases | 4 cases |
| Pulsation occulte par hostile | 5 dégâts | 29 dégâts |
| Rayon de Pulsation | 2 cases | 2 cases |
| Énergie de Pulsation | 6 | 6 |
| Recharge de Pulsation | 4 tours | 4 tours |
| Durée | 4 tours | 7 tours |
| Énergie | 16 points | 19 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Trait corporel ordinaire = floor((8 + 3r) × M_magie) ; Portée de Trait corporel = 4 ; Pulsation occulte par hostile = floor((5 + 2r) × M_magie) ; Rayon de Pulsation = 2 ; Énergie de Pulsation = 6 ; Recharge de Pulsation = 4 ; Durée = 4 + floor(r/3) + min(3, floor(max(0, Volonté − 5)/20)) ; Énergie = 16 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** Trait corporel est l'attaque naturelle ordinaire de cette forme, sans coût supplémentaire, avec dégâts occultes et défenses de cible. Pulsation est une compétence active de forme, centrée sur l'agent et visant les hostiles proches. Silence laisse l'attaque ordinaire mais bloque Pulsation. La forme ne reçoit pas de munitions d'arme.

Intention visuelle conservée du catalogue : Dessin 64, motifs organiques pulsants et émissions depuis le corps. Rendu non créé dans cette passe.

### TR65 Abysse — 65

Type : actif. Rôle retenu : Forme à allonge naturelle et déplacements courts entre ombres visibles.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Allonge ordinaire | 7 dégâts | 32 dégâts |
| Portée de l'attaque naturelle | 2 cases | 2 cases |
| Portée de Pas d'ombre | 3 cases | 3 cases |
| Énergie de Pas d'ombre | 5 | 5 |
| Recharge de Pas d'ombre | 3 tours | 3 tours |
| Durée | 5 tours | 8 tours |
| Énergie | 14 points | 17 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Allonge ordinaire = floor((7 + 2r) × M_mêlée) ; Portée de l'attaque naturelle = 2 ; Portée de Pas d'ombre = 3 ; Énergie de Pas d'ombre = 5 ; Recharge de Pas d'ombre = 3 ; Durée = 5 + floor(r/3) + min(3, floor(max(0, Volonté − 5)/20)) ; Énergie = 14 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** Allonge : attaque physique naturelle, arrêtée par les obstacles, sans munition. Pas d'ombre : action magique vers une case libre actuellement visible à trois cases maximum. Pour cette proposition, une ombre est une case adjacente à un obstacle opaque connu ; aucun nouveau système d'éclairage n'est requis pour définir la destination. Pas de traversée de carte ni d'attaque gratuite.

Intention visuelle conservée du catalogue : Dessin 65, membres étirés et ombre résiduelle à l'ancienne position. Rendu non créé dans cette passe.

### M03 Puissance métamorphe

Type : passif. Rôle retenu : Augmente les dégâts sous forme ; +10 % au premier rang confirmé.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts supplémentaires sous forme | 10 % | 32 % |

**Progression et calculs proposés :** Dégâts supplémentaires sous forme = 10 + 2r.

**Fonctionnement proposé :** Reprend M03 du lot 02 : bonus seulement tant que l'agent est transformé, physique et magique, sans drones. Les effets persistants conservent leur auteur ; ce multiplicateur est lu au moment de leurs dégâts et disparaît au retour humain.

Intention visuelle conservée du catalogue : Réseau interne de la forme qui s'allume lors d'une attaque renforcée. Rendu non créé dans cette passe.

## Pouvoirs hybrides

### M02 Décharge mutagène

Type : actif. Rôle retenu : Frappe une cible ; sous forme, un impact réussi peut prolonger cette forme, avec une limite totale.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts directs | 7 dégâts | 31 dégâts |
| Tours ajoutés par impact admissible | 1 tour | 2 tours |
| Tours ajoutés au maximum par transformation | 2 tours | 4 tours |
| Portée | 3 cases | 3 cases |
| Énergie | 6 points | 9 points |
| Recharge | 2 tours | 2 tours |

**Progression et calculs proposés :** Dégâts directs = floor((7 + 2r) × M_magie) ; Tours ajoutés par impact admissible = min(2, 1 + floor(r/10)) ; Tours ajoutés au maximum par transformation = min(6, 2 + floor(r/5)) ; Portée = 3 ; Énergie = 6 + floor(r/3) ; Recharge = 2.

**Fonctionnement proposé :** Reprend les dégâts de M02 et complète la prolongation. Il faut retirer réellement des PV à un hostile pour prolonger une forme en cours. Le total d'extension appartient à cette activation de forme ; il n'est pas réinitialisé par un équipement ou un autre lancer de M02. Hors forme, le sort donne seulement ses dégâts. Pas de prolongation sur une cible invulnérable, alliée ou un leurre.

Intention visuelle conservée du catalogue : Décharge organique puis un segment de durée rallumé sur la forme. Rendu non créé dans cette passe.

### HY02 Fusion défensive

Type : actif. Rôle retenu : Sacrifie un drone de combat contrôlé pour créer une protection absorbante sur l'agent.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Absorption créée | 25 % des PV actuels du drone sacrifié | 43,3 % des PV actuels du drone sacrifié |
| Durée | 3 tours | 5 tours |
| Distance du drone | 4 cases | 4 cases |
| Énergie | 8 points | 11 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Absorption créée = 25 + 35r/(n + 9) ; Durée = 3 + floor(r/5) ; Distance du drone = 4 ; Énergie = 8 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Sacrifie D58, D59 ou D66 contrôlé, visible et à portée. D56 et D57 sont exclus ; aucun chargement de Mule n'est perdu. Absorption arrondie au supérieur, dégâts après les défenses précédentes. Un seul bouclier de Fusion ; pas de cumul ou de nouvelle Fusion pendant celui-ci. Le sacrifice ne donne ni loot ni XP ni remboursement de déploiement.

Intention visuelle conservée du catalogue : Drone se décomposant en segments qui assemblent une coque sur le joueur. Rendu non créé dans cette passe.

### HY03 Relais de sort

Type : actif. Rôle retenu : Lance un sort compatible depuis la position d'un drone contrôlé.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Distance du relais depuis l'agent | 4 cases | 6 cases |
| Dégâts du sort relayé | 100 % de ses dégâts ordinaires | 133 % de ses dégâts ordinaires |
| Surcoût du relais | 4 points | 7 points |
| Recharge propre du relais | 6 tours | 6 tours |

**Progression et calculs proposés :** Distance du relais depuis l'agent = min(8, 4 + floor(r/5)) ; Dégâts du sort relayé = 100 + 3r ; Surcoût du relais = 4 + floor(r/3) ; Recharge propre du relais = 6.

**Fonctionnement proposé :** Sorts proposés compatibles : FE01, FR01, A01, A02, FE04, FR05. Les sorts FO01 et FO04 sont exclus à la suite du retrait de leur branche sous AR-FO-REMOVE01. Le sort doit être appris et disponible. Son origine, sa portée et sa ligne de vue sont calculées depuis un drone déployé réellement observé. Le lancement paie ensemble le sort et le surcoût et engage leurs deux recharges. Canalisation n'est appliquée qu'une fois à chaque coût admissible. Le relais ne prête pas la Volonté du drone, ne transmet pas le cargo, ne contourne pas l'observation et ne fait pas apprendre le sort. Un seul lancement, une seule attribution des déclenchements passifs.

Intention visuelle conservée du catalogue : Fil de commande jusqu'au drone, émission du sort depuis son capteur. Rendu non créé dans cette passe.

### HY04 Échange de position

Type : actif. Rôle retenu : Échange les positions de l'agent et d'un drone contrôlé, si les deux destinations sont légales.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Distance d'échange | 4 cases | 6 cases |
| Énergie | 10 points | 8 points |
| Recharge | 8 tours | 6 tours |

**Progression et calculs proposés :** Distance d'échange = min(8, 4 + floor(r/5)) ; Énergie = ceil(10 − 4r/(n+9)) ; Recharge = ceil(3 + 50/(n+9)).

**Fonctionnement proposé :** Échange l'agent avec un drone déployé contrôlé et visible. Les deux destinations doivent être légales pour leurs occupants, sur la même carte. Aucun échange avec une réserve de drones, une autre couche ou un ennemi. Les dangers de destination se résolvent normalement ; le cargo reste dans son drone.

Intention visuelle conservée du catalogue : Deux silhouettes se décomposant puis se reformant sur les cases échangées. Rendu non créé dans cette passe.

### HY05 Siphon technique

Type : actif. Rôle retenu : Dissipe un renforcement ennemi compatible et récupère une part d'énergie.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Renforcements retirés au maximum | 1 | 3 |
| Énergie récupérée au total | 5 % de l'énergie maximale | 10,2 % de l'énergie maximale |
| Portée | 4 cases | 4 cases |
| Énergie | 6 points | 9 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Renforcements retirés au maximum = min(3, 1 + floor(r/5)) ; Énergie récupérée au total = 5 + 10r/(n + 9) ; Portée = 4 ; Énergie = 6 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Choisit des buffs temporaires ennemis portant explicitement la compatibilité de dissipation. Pas de traits, équipement ou passif permanent effacé. Sans effet admissible, refus sans coût. Le gain total, arrondi au supérieur, est accordé une fois par lancement, pas par buff ; il ne dépasse pas la réserve manquante. Les alliés et drones du joueur sont exclus.

Intention visuelle conservée du catalogue : Effet arraché à la cible en fragments qui rejoignent les implants. Rendu non créé dans cette passe.

### HY06 Lien de domination

Type : actif. Rôle retenu : Prend temporairement le contrôle d'un ennemi, soumis à résistance et restrictions à définir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Durée | 2 tours | 4 tours |
| Ennemis contrôlés simultanément | 1 | 1 |
| Portée | 4 cases | 4 cases |
| Puissance de contrôle | 100 unités | 175 unités |
| Énergie | 16 points | 19 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Durée = 2 + floor(r/5) + min(3, floor(max(0, Intelligence − 5)/20)) ; Ennemis contrôlés simultanément = 1 ; Portée = 4 ; Puissance de contrôle = 100 + 5r + 2(Intelligence − 5) + 2(Volonté − 5) ; Énergie = 16 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** Contrôle temporairement un hostile admissible s'il échoue à sa résistance. L'ennemi conserve ses PV, statistiques et compétences ; il devient un allié temporaire, pas un drone. Un flag explicite indomptable pourra exclure certains monstres importants, sans exclusion par nature machine/organique/démon. La fin rend la cible hostile sans soin. Sortir de carte termine le lien ; aucun captif n'est transporté. Une nouvelle domination réussie remplace l'ancienne. Le classement des monstres importants sera défini lors de leur conception.

Intention visuelle conservée du catalogue : Collier de glyphes et ligne de contrôle vers l'agent. Rendu non créé dans cette passe.

## Buffs de mêlée

### BM01 Lame incendiaire

Type : actif. Rôle retenu : Ajoute temporairement du feu aux attaques de mêlée compatibles.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Feu ajouté par coup principal réussi | 3 dégâts | 15 dégâts |
| Durée | 4 tours | 6 tours |
| Énergie | 8 points | 11 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Feu ajouté par coup principal réussi = floor((3 + 1r) × M_magie) ; Durée = 4 + floor(r/5) ; Énergie = 8 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Arme de mêlée ou attaque naturelle compatible. Le feu est résolu séparément contre les résistances ; il ne crée pas une brûlure pour Combustion. Aucun effet sur les drones, projectiles ou dégâts secondaires de riposte. Le buff ne remplace ni arme ni équipement.

Intention visuelle conservée du catalogue : Flammes attachées au tranchant ou aux griffes, sans remplacer l'arme. Rendu non créé dans cette passe.

### BM02 Faim occulte

Type : actif. Rôle retenu : Les coups de mêlée réussis rendent une part de la vie volée.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Vie rendue sur les PV retirés au contact | 5 % | 12,9 % |
| Durée | 4 tours | 6 tours |
| Énergie | 10 points | 13 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Vie rendue sur les PV retirés au contact = 5 + 15r/(n + 9) ; Durée = 4 + floor(r/5) ; Énergie = 10 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Coup principal de mêlée contre un hostile : vie volée sur les PV vraiment retirés après défenses. Les dégâts directs ajoutés à ce coup sont inclus, pas les zones ni le renvoi. Soin total arrondi une seule fois, limité aux PV manquants et aux PV retirés. Le renvoi et les attaques automatiques secondaires ne volent pas de vie.

Intention visuelle conservée du catalogue : Gouttes sombres remontant de la victime vers le bras d'attaque. Rendu non créé dans cette passe.

### BM03 Tempo offensif

Type : passif. Rôle retenu : Des coups de mêlée consécutifs gagnent un bonus limité, perdu à la rupture de la série.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Bonus par charge de série | 5 % | 16 % |
| Charges maximales | 3 | 3 |

**Progression et calculs proposés :** Bonus par charge de série = 5 + 1r ; Charges maximales = 3.

**Fonctionnement proposé :** Un coup principal de mêlée retirant des PV donne une charge pour les coups suivants. Au maximum trois charges. Un coup manqué, un déplacement ou une autre action prenant du temps rompt la série. Menu et marquage sans passage de tour ne la rompent pas. Le bonus actuel est lu sur le prochain coup ; un équipement ne crée pas de charge.

Intention visuelle conservée du catalogue : Repères successifs près de la poignée, traînée plus marquée à chaque palier. Rendu non créé dans cette passe.

### BM04 Ancrage

Type : actif. Rôle retenu : Empêche temporairement les déplacements forcés de l'agent.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Durée | 3 tours | 5 tours |
| Énergie | 6 points | 9 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Durée = 3 + floor(r/5) ; Énergie = 6 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Empêche uniquement les déplacements forcés de l'agent pendant sa durée. Il peut toujours décider de se déplacer ou d'employer ses propres mouvements magiques. Ne donne ni armure, ni réduction de dégâts, ni immunité aux attaques.

Intention visuelle conservée du catalogue : Griffes de lumière plantées sous les appuis. Rendu non créé dans cette passe.

### BM05 Sceau de vindicte

Type : actif. Rôle révisé par le joueur : Renvoie une part des dégâts
magiques directs subis vers leur auteur. BM05 reste Sceau de vindicte,
appelé « glyphe » dans la discussion ; SU03 Barrière d’épine assure le
renvoi des dégâts directs de mêlée. Le poison et tous les dégâts indirects
sont exclus des deux compétences. Ce changement d'effet ne valide pas
les chiffres et conditions supplémentaires ci-dessous.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts magiques directs renvoyés | 10 % | 23,1 % |
| Durée | 4 tours | 6 tours |
| Énergie | 10 points | 13 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Dégâts magiques directs renvoyés = 10 + 25r/(n + 9) ; Durée = 4 + floor(r/5) ; Énergie = 10 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Renvoi sur les dégâts magiques directs
effectivement subis après protections. Le poison et tous les dégâts
indirects, dont le saignement et les effets persistants, sont exclus par
décision du joueur. L'éligibilité de l'auteur, les défenses du destinataire,
les cas mortels, les attaques mixtes et les interactions avec les attaques
secondaires restent à préciser pour sa revue individuelle. Les anciennes
conditions réservant ce renvoi à un auteur au contact sont remplacées.

**Intention visuelle acceptée :** le sort ou le projectile magique repart
visiblement vers son auteur, en reprenant l'apparence de l'attaque d'origine.
La demande concerne l'animation du renvoi ; elle n'ajoute pas de blocage
des dégâts initiaux ni de nouveau jet de toucher. Les trajets, le traitement
de chaque sort et le rendu natif restent à présenter. L'éligibilité des tirs
non magiques demeure non décidée. Aucun rendu n'est créé dans cette passe.

### BM06 Protection offensive

Type : passif. Rôle retenu : Un coup de mêlée réussi produit une petite protection temporaire, avec cumul limité.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Absorption ajoutée par coup admissible | 2 % des PV maximum | 5,1 % des PV maximum |
| Contributions maximales | 3 | 3 |
| Plafond total | 15 % des PV maximum | 15 % des PV maximum |
| Durée après le dernier coup admissible | 2 tours | 2 tours |
| Contributions ajoutées par tour au maximum | 1 | 1 |

**Progression et calculs proposés :** Absorption ajoutée par coup admissible = 2 + 6r/(n + 9) ; Contributions maximales = 3 ; Plafond total = 15 ; Durée après le dernier coup admissible = 2 ; Contributions ajoutées par tour au maximum = 1.

**Fonctionnement proposé :** Un coup principal de mêlée retirant des PV à un hostile ajoute une petite réserve absorbante. Trois contributions au maximum et plafond total de 15 % des PV maximum, sans soin. Un seul ajout par tour même pour Tourbillon. Les dégâts consomment la réserve restante ; une nouvelle frappe peut la compléter dans ses limites. Les changements de maximum peuvent la plafonner mais ne la remplissent pas gratuitement.

Intention visuelle conservée du catalogue : Demi-arc protecteur assemblé devant le corps après la frappe. Rendu non créé dans cette passe.

## Buffs à distance

### T01 Marquage balistique

Type : actif. Rôle retenu : Marquage instantané ; précision × 1,15 au premier rang confirmé.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Précision supplémentaire contre la marque | 15 % de P | 37 % de P |
| Durée | 4 tours | 6 tours |
| Portée | 6 cases | 6 cases |
| Temps d'activation | 0 tour | 0 tour |
| Énergie | 6 points | 9 points |
| Recharge | 3 tours | 3 tours |

**Progression et calculs proposés :** Précision supplémentaire contre la marque = 15 + 2r ; Durée = 4 + floor(r/5) ; Portée = 6 ; Temps d'activation = 0 ; Énergie = 6 + floor(r/3) ; Recharge = 3.

**Fonctionnement proposé :** Reprend T01 du lot 02 : instantané, sans progression du temps ni réaction ennemie. Une seule marque ; prochaine attaque à distance admissible contre elle, même manquée, consomme la marque. Dans une rafale, le multiplicateur s'applique une fois à chaque jet contre la cible principale. Pas de bonus aux drones ni aux sorts sans précision. Le tir de couverture ne la consomme qu'au déclenchement de son tir, si elle est encore présente.

Intention visuelle conservée du catalogue : Viseur bref posé sur la cible, quatre traits qui se ferment. Rendu non créé dans cette passe.

### T03 Maîtrise des attaques à distance

Type : passif. Rôle retenu : +10 % de dégâts des armes et compétences à distance au premier rang confirmé.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts supplémentaires à distance | 10 % | 32 % |

**Progression et calculs proposés :** Dégâts supplémentaires à distance = 10 + 2r.

**Fonctionnement proposé :** Reprend T03 du lot 02. Permanent, avec ou sans déplacement. Concerne armes et compétences d'attaque classées à distance, dont sorts et actions de forme compatibles ; ni drones ni contact. Une compétence sans dégâts ne reçoit pas de dégâts artificiels.

Intention visuelle conservée du catalogue : Trait intérieur plus vif dans le projectile ou l'émission renforcée. Rendu non créé dans cette passe.

### BD03 Salve miroir

Type : actif. Rôle retenu : Le prochain tir compatible produit un tir secondaire vers un autre ennemi visible.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts du projectile secondaire | 50 % du premier projectile principal | 83 % du premier projectile principal |
| Durée | 3 tours | 5 tours |
| Projectiles secondaires maximaux | 1 | 1 |
| Munition supplémentaire si le second tir est possible | 1 | 1 |
| Énergie | 8 points | 11 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Dégâts du projectile secondaire = 50 + 3r ; Durée = 3 + floor(r/5) ; Projectiles secondaires maximaux = 1 ; Munition supplémentaire si le second tir est possible = 1 ; Énergie = 8 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Le prochain tir d'arme compatible peut envoyer un seul projectile secondaire vers un autre hostile visible à portée et avec ligne de tir. Une rafale n'est pas dupliquée : seul son premier projectile sert de base. Pas de duplication de grenade ou de zone. Le total de munitions du tir prévu est vérifié avant exécution. Sans second hostile admissible, le tir principal consomme le buff sans balle supplémentaire. Le projectile miroir ne génère pas de miroir ou d'autre attaque automatique.

Intention visuelle conservée du catalogue : Écho du départ de tir depuis une petite silhouette d'arme projetée. Rendu non créé dans cette passe.

### BD04 Visée stable

Type : passif. Rôle retenu : Rester immobile développe un bonus limité de précision, perdu au déplacement.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Précision par tour immobile | 5 unités | 16 unités |
| Tours cumulables | 3 | 3 |

**Progression et calculs proposés :** Précision par tour immobile = 5 + 1r ; Tours cumulables = 3.

**Fonctionnement proposé :** Un tour réellement écoulé sans changement de position ajoute un palier, jusqu'à trois. Le bonus vaut le nombre de paliers multiplié par la valeur du rang actuel. Tout changement de position, volontaire, forcé ou magique, vide les paliers. Menu, attente réelle et action instantanée n'en ajoutent pas. T03 conserve son fonctionnement indépendant.

Intention visuelle conservée du catalogue : Réticule se resserrant progressivement autour de la visée. Rendu non créé dans cette passe.

### BD05 Écran tactique

Type : actif. Rôle retenu : Prépare la déviation d'un prochain projectile hostile compatible.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts du projectile dévié | 40 % du projectile hostile | 61 % du projectile hostile |
| Durée | 3 tours | 5 tours |
| Projectiles arrêtés par activation | 1 | 1 |
| Énergie | 10 points | 13 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Dégâts du projectile dévié = 40 + 40r/(n + 9) ; Durée = 3 + floor(r/5) ; Projectiles arrêtés par activation = 1 ; Énergie = 10 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Intercepte le prochain projectile hostile compatible qui aurait touché l'agent après son jet de précision. Projectiles d'armes sans explosion seulement ; pas de rayon, zone ou sort automatique. Retour vers le tireur uniquement s'il est observé et si le trajet est légal, avec un nouveau jet normal ; sinon le projectile est neutralisé. Le retour ne génère aucun autre renvoi, miroir ou riposte. Un projectile déjà esquivé ne consomme pas l'écran.

Intention visuelle conservée du catalogue : Plan lumineux incliné et changement net de sa trajectoire à l'impact. Rendu non créé dans cette passe.

### BD06 Récupération balistique

Type : passif. Rôle retenu : Une élimination à distance récupère une petite quantité d'énergie, avec cadence limitée.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Énergie par élimination admissible | 2 % de l'énergie maximale | 4,1 % de l'énergie maximale |
| Intervalle entre gains | 3 tours | 3 tours |

**Progression et calculs proposés :** Énergie par élimination admissible = 2 + 4r/(n + 9) ; Intervalle entre gains = 3.

**Fonctionnement proposé :** Une élimination attribuée à une attaque principale à distance de l'agent récupère de l'énergie, arrondie au supérieur et limitée à la réserve manquante. Un gain au maximum toutes les trois unités de tour. Les drones, leurres, obstacles et dégâts secondaires gratuits ne donnent pas ce gain. Aucun temps ne passe pour ouvrir le menu.

Intention visuelle conservée du catalogue : Étincelle remontant vers la jauge d'énergie depuis la cible éliminée. Rendu non créé dans cette passe.

## Compléments des trois arbres secondaires

Ces quatorze compétences n'ont aucun palier. Les actifs SF00, SM00 et SG00
s'achètent une seule fois ; chacun doit être appris avant ses passifs
associés, conformément à AR-SEC01. Le rang 12 de leurs passifs signifie
dix rangs appris et deux d'équipement, pas douze achats de l'actif.

Les règles nouvelles ci-dessous complètent la [revue secondaire](REVUE_COMPETENCES_SECONDAIRES_AGENT.md)
sans approuver ses anciens textes. Ce document devient la proposition
technique actuelle pour les paramètres que cette revue laissait ouverts.

### SF00 Furtivité

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Énergie par pas sans passif | 10 | 10 |
| Temps d'activation ou de sortie | 1 action | 1 action |
| Énergie d'activation | 0 | 0 |
| Recharge | 0 | 0 |
| Facteur de portée visuelle avant SF02 | 0,75 | 0,6818 |
| Intensité des pas | 50 % de leur bruit ordinaire | 50 % de leur bruit ordinaire |

**Calculs proposés :** Énergie par pas sans passif = 10 ; Temps d'activation ou de sortie = 1 ; Énergie d'activation = 0 ; Recharge = 0 ; Facteur de portée visuelle avant SF02 = 0,75 / (1 + 0,02max(0, Agilité−5)) ; Intensité des pas = 50.

**Fonctionnement proposé :** Mode technologique. Pas réussi seulement : énergie payée avant la régénération normale du tour. Un pas furtif sans énergie est refusé sans déplacement ou perte de ressources. Attaque, sort offensif, pose de mine ou lancer de grenade termine le mode ; soins, consultation et récupération ne le terminent pas. Pas d'entretien au repos. Vision et écoute utilisent les règles réelles du moteur, pas des cercles à travers les murs ; une portée visuelle effective garde un minimum d'une case et ne supprime pas une position déjà mémorisée.

### SF01 Économie de mouvement furtif

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Énergie par pas sur la base 10 | 9 | 0 |

**Calculs proposés :** Énergie par pas sur la base 10 = ceil(10 × max(0,1−n/10)).

**Fonctionnement proposé :** Même formule que la proposition précédente, avec 10 désormais proposé comme vrai coût de base. Rang 10 et plus : zéro énergie par pas, sans minimum artificiel de un. Le coût nul ne crée pas d'énergie et ne change pas les autres coûts.

### SF02 Discrétion renforcée

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Facteur visuel supplémentaire | 0,8 | 0,0687 |

**Calculs proposés :** Facteur visuel supplémentaire = (4/5) puissance n.

**Fonctionnement proposé :** Même progression que précédemment : 80 % au rang 1, environ 10,7 % au rang 10. Agilité et le facteur de base SF00 se combinent. L'écoute ne devient pas aveugle ; les bruits de pas restent des événements réels. Bonus d'équipement proposés applicables au-delà de dix, avec le minimum d'une case de repérage.

### SM00 Pose de mine

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Explosion de comparaison | 12 dégâts | 13 dégâts |
| Énergie | 10 | 10 |
| Recharge | 1 tour | 1 tour |
| Rayon sans SM04 | 1 case | 1 case |
| Armement après la pose | 1 tour complet | 1 tour complet |
| Capacité sans SM01 | 1 mine | 1 mine |

**Calculs proposés :** Explosion de comparaison = floor((12 + 0r) × M_technique) ; Énergie = 10 ; Recharge = 1 ; Rayon sans SM04 = 1 ; Armement après la pose = 1 ; Capacité sans SM01 = 1.

**Fonctionnement proposé :** Feu par défaut, poison ou froid selon passif appris. Pose sur la case de l'agent ou une case adjacente visible et franchissable, sans autre mine ni hostile déjà présent ; objets et alliés peuvent partager la case. Déclenchement par contact hostile une fois armée, y compris si un hostile reste dessus à l'armement. L'explosion peut toucher agent et alliés. Les mines ne déclenchent pas une chaîne d'explosions entre elles. Toute pose refusée laisse la plus ancienne mine intacte.

### SM01 Réseau de mines

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Mines simultanées | 2 | 5 |

**Calculs proposés :** Mines simultanées = 1 + ceil(n/3).

**Fonctionnement proposé :** Conserve cinq mines au rang 10, cinq aussi au rang 12 ; proposition de poursuite par équipement : six au rang 13. La plus ancienne, armée ou non, est supprimée seulement après validation complète de la nouvelle pose. Pas d'explosion ou de remboursement à cette suppression.

### SM02 Mines de poison

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts supplémentaires de la variante poison | 0 % | 55 % |

**Calculs proposés :** Dégâts supplémentaires de la variante poison = 0 + 5r.

**Fonctionnement proposé :** Premier rang ouvre le choix poison ; rangs suivants renforcent explosion et nuage poison. Pas de poison périodique sans SM05. Le rang de ce passif ne modifie pas les variantes feu et glace.

### SM03 Mines de glace

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts supplémentaires de la variante glace | 0 % | 55 % |

**Calculs proposés :** Dégâts supplémentaires de la variante glace = 0 + 5r.

**Fonctionnement proposé :** Premier rang ouvre le froid ; progression identique à SM02. Le gel de zone nécessite SM05, laisse les attaques et autres actions non mobiles disponibles et ne devient pas un étourdissement général.

### SM04 Explosion étendue

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Rayon total avec base une case | 2 cases | 4 cases |

**Calculs proposés :** Rayon total avec base une case = 1 + ceil(n/4).

**Fonctionnement proposé :** Conserve le rayon supplémentaire précédent. Explosion et résidu utilisent la même aire radiale euclidienne, coupée par les murs et bornée par les limites de carte. Le rang effectif d'équipement poursuit la progression.

### SM05 Résidu persistant

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Durée du résidu | 3 tours | 14 tours |
| Feu ou poison par tour avant bonus de variante | 3 dégâts | 3 dégâts |

**Calculs proposés :** Durée du résidu = 3 + 1r ; Feu ou poison par tour avant bonus de variante = floor((3 + 0r) × M_technique).

**Fonctionnement proposé :** Durée égale à 2+n comme précédemment. Feu et poison : première résolution au prochain tour complet, une par acteur et par tour. Glace : immobilisation des hostiles dans la zone active, sans dégâts périodiques ajoutés ; leur mouvement redevient disponible après sortie forcée ou disparition de la zone. Les mines, leurs minuteries et leurs résidus sont proposés comme effacés à tout changement de carte, y compris portail bonus ; la suppression au changement de couche reste acquise. Retourner dans une carte ne recrée pas les anciennes mines.

### SG00 Lancer de grenade

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts explosifs de comparaison | 8 dégâts | 8 dégâts |
| Énergie | 8 | 8 |
| Recharge | 2 tours | 2 tours |
| Portée avant SG03 | 4 cases | 4 cases |
| Rayon de l'explosive ou aveuglante | 1 case | 1 case |

**Calculs proposés :** Dégâts explosifs de comparaison = floor((8 + 0r) × M_technique) ; Énergie = 8 ; Recharge = 2 ; Portée avant SG03 = 4 ; Rayon de l'explosive ou aveuglante = 1.

**Fonctionnement proposé :** Une action : grenade résolue à l'impact sans tour d'attente ajouté. Énergie seule, aucun objet consommé. Explosion physique cinétique, dégâts modestes et alliés exposés. Les variantes utilitaires ont zéro dégât. Toutes utilisent leur aperçu réel avant confirmation, avec un trajet et une cible actuellement visibles.

### SG01 Grenade aveuglante

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Précision conservée | 70 % | 70 % |
| Durée d'aveuglement | 2 tours | 13 tours |
| Puissance de contrôle | 100 unités | 165 unités |

**Calculs proposés :** Précision conservée = 70 ; Durée d'aveuglement = 2 + 1r ; Puissance de contrôle = 100 + 5r + 2(Intelligence − 5).

**Fonctionnement proposé :** Conserve la durée 1+n et la précision ×0,7. Le premier rang ouvre l'aveuglante. Acteurs de la zone soumis à leur résistance, alliés compris dans cette proposition. Plusieurs aveuglements ne multiplient pas entre eux la perte de précision ; la meilleure valeur active est retenue. Aucune action supprimée.

### SG02 Grenade sonore

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Intensité sonore | 8 | 30 |
| Émissions successives | 2 tours | 2 tours |

**Calculs proposés :** Intensité sonore = 8 + 2r ; Émissions successives = 2.

**Fonctionnement proposé :** Conserve 6+2n. Le centre de l'impact est la destination de recherche des monstres capables de l'entendre ; ils suivent un chemin légal. Une menace immédiate à portée d'attaque reste prioritaire. La vraie propagation avec murs et seuils d'écoute détermine les cases atteintes. Le rang 1 doit porter plus loin que le rayon un des autres grenades en terrain ouvert : ce critère fait partie des essais moteur, sans remplacer le son par un cercle ou garantir l'audition d'un monstre sourd.

### SG03 Portée de lancer

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Portée de lancer supplémentaire | 1 case | 4 cases |

**Calculs proposés :** Portée de lancer supplémentaire = 1 + floor(r/3).

**Fonctionnement proposé :** Conserve 1+floor(r/3), applicable à toutes les variantes de l'actif, pas aux armes lance-grenades. Le rayon sonore reste distinct du trajet de lancer et la cible reste actuellement visible.

### SG04 Puissance des grenades

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts supplémentaires des grenades | 10 % | 120 % |

**Calculs proposés :** Dégâts supplémentaires des grenades = 10 + 10r.

**Fonctionnement proposé :** Conserve 10n, actif secondaire et armes lance-grenades compris. Pas de dégâts artificiels pour les variantes sonore et aveuglante. La variante de l'arme conserve sa gestion de munitions et ses attributs d'attaque ordinaires.

## Interfaces et ordre de travail

SP00 : aperçu de pose ou de lancer pour tous les types. SP01 : rayon d'une
mine seule au survol ou au curseur. SP02 : sélecteur des éléments connus
d'une case partagée. Ces trois fonctionnalités gardent les demandes déjà
retenues, sans achat ni tirage ; elles ne sont pas des compétences supplémentaires.

Le panneau des buffs/debuffs, le compteur d'hostiles, Combat/Accalmie/Calme
et les recharges du menu conservent la [revue de l'interface](REVUE_INTERFACE_EFFETS_AGENT.md).
Le survol ne révèle ni mine inconnue ni acteur caché. Une recharge restant
positive interdit l'activation, sans empêcher la consultation de la fiche.

La prochaine passe présentera les descriptions une par une : texte roleplay,
effets par défaut, valeurs réelles actuelles et indications de durée/recharge.
Le joueur pourra garder ou modifier chacune. Les noms déjà retenus ne sont
pas renommés automatiquement ; toute révision conserve son identifiant.

Les compétences et animations seront ensuite intégrées après leurs accords
respectifs. Le personnage configurable de la salle de test, avec niveaux et
respécialisation illimités, vient après leur implémentation. Les monstres
auront un catalogue propre selon rôle, niveau et importance, à définir lors
de leur phase ; il n'est pas remplacé par les arbres du joueur.

Le cadrage chiffré est complet comme proposition. L'équilibrage final, les
descriptions, les illustrations, les contrôles d'interface et la validation
en partie restent des étapes séparées. Aucune compétence n'est déclarée
implémentée ou testée dans le moteur par ce document.
