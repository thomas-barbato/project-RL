# Cadrage complet des compétences de l'agent

6 octobre 2026. Le joueur demande de terminer le cadrage avant de reprendre
les descriptifs et de décider ce qu'il garde ou modifie. Ce document achève
les propositions de règles des dix branches restantes et complète les
paramètres des trois arbres secondaires. Il ne crée aucun nouveau texte
roleplay, dessin, animation ou code de compétence.

Les noms, rôles et intentions déjà acceptés sont conservés. Chaque nouveau
chiffre et cas de fonctionnement demeure une proposition pour la revue
suivante. Les accords antérieurs sur des textes ou tableaux précis restent
valables ; achever ce document ne vaut pas approbation de son contenu.

## Références, rangs et attributs

Le catalogue principal contient 86 compétences dans 14 branches. Mêlée,
Survie, Drones et Affaiblissements possèdent déjà leurs lots de travail,
référencés ci-dessous. Les 60 autres compétences ont ici leurs paramètres.
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

## Temps, disponibilité et ciblage

Un lancement normal prend une action et met immédiatement son effet en
place, avant les réactions ennemies. Le marquage T01 conserve son activation
sans passage de tour. Un effet explicitement différé garde son délai annoncé :
mine armée après un tour, explosion d'Éruption, progression de l'Orbe ou
tir de réaction après mise en surveillance. Ces délais ne demandent pas
une action cachée de préparation au joueur.

Le menu des actions affiche le temps restant de recharge ; le bouton grisé
est désactivé jusqu'à la fin du délai, avec survol conservé. Le libellé proposé
« Interférence — Recharge : 3 tours » est accepté lors de la dernière réponse.
Une commande distincte de maintien poursuit une canalisation déjà lancée :
elle n'est pas une nouvelle activation contournant sa recharge. Cette
présentation de maintien sera revue avec les descriptifs et l'interface.

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

Les techniques DI01 à DI06 utilisent une arme à projectile physique sans
explosion ni cône de plombs, notamment pistolet, PM ou fusil compatible.
Elles définissent leur propre nombre de projectiles à partir du dégât d'un
seul projectile, et ne multiplient pas une rafale native une seconde fois.
L'arme sélectionnée et ses munitions restent utilisées. Les lance-grenades,
rayons, lasers, attaques naturelles et sorts ne sont pas implicitement
convertis en balles. Chaque famille sera déclarée compatible au contenu.

## Cumuls, sources et effets prolongés

Une même compétence ne multiplie pas ses propres buffs, leurres, surveillances
ou passages : un seul exemplaire, remplacé ou refusé selon sa règle. Des
sources différentes peuvent coexister. Les réductions de dégâts distinctes
se multiplient après armure et résistances ; elles ne s'additionnent pas
jusqu'à une immunité. L'absorption suit les protections précédentes, puis
Dernier sursaut conserve sa place finale. Chaque réserve d'absorption est
limitée et ne soigne pas les PV.

Dans les dix branches complétées ici, relancer un buff, un leurre, une
surveillance ou un couple de passages remplace sa version précédente après
validation, sans additionner les durées. Une création persistante propre
à un sort — mur, foyer, orbe, blizzard, orage, circuit ou prison — est limitée
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

## Proposition de paliers pour les quatorze branches

Pour six compétences, proposition de deux paliers de trois : entrée à zéro
rang appris et second palier à cinq rangs appris dans la branche. Pour les
huit drones, troisième palier de deux compétences à dix rangs appris.
Cette nouvelle grille adapte le catalogue accepté ; elle reste à valider,
sans rétablir automatiquement les anciennes grilles retirées.

Tous les choix restent apprenables. Seuls les rangs appris comptent, y
compris les rangs de départ offerts ; les bonus d'objets ne débloquent pas
les paliers. Un seuil ouvre un achat, il ne donne aucune compétence gratuite.
Les branches sont indépendantes et toutes les classes peuvent les développer.


| Branche | Palier 0 | Palier 5 | Palier 10 |
| --- | --- | --- | --- |
| Mêlée | B01, ME02, ME03 | ME04, ME05, ME06 | — |
| Survie | B03, B02, SU03 | SU04, SU05, SU06 | — |
| Drones | D59, D57, O03 | D56, D58, DP01 | D66, DP02 |
| Affaiblissements | O02, AF02, AF03 | AF04, AF05, AF06 | — |
| Attaques à distance | DI01, DI02, DI05 | DI03, DI04, DI06 | — |
| Esquive | T02, ES02, ES03 | ES04, ES05, ES06 | — |
| Feu | FE01, FE02, FE06 | FE03, FE04, FE05 | — |
| Froid | FR01, FR02, FR03 | FR04, FR05, FR06 | — |
| Foudre | FO01, FO02, FO03 | FO04, FO05, FO06 | — |
| Énergie occulte | A01, A02, A03 | OC04, OC05, OC06 | — |
| Transformations | M01, TR61, M03 | TR63, TR64, TR65 | — |
| Pouvoirs hybrides | M02, HY02, HY04 | HY03, HY05, HY06 | — |
| Buffs de mêlée | BM01, BM03, BM04 | BM02, BM05, BM06 | — |
| Buffs à distance | T01, T03, BD04 | BD03, BD05, BD06 | — |

Cette grille conserve les compétences initiales connues dans le premier palier.
Le choix final du premier sort et de la première forme à la création sera
traité avec cette interface ; aucune affectation supplémentaire n'est imposée ici.

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

### DI02 Rafale concentrée

Type : actif. Rôle retenu : Tire plusieurs projectiles successifs sur une même cible.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Projectiles sur une cible | 3 | 5 |
| Dégâts par projectile | 60 % de D | 93 % de D |
| Impact de comparaison | 12 dégâts | 20 dégâts |
| Précision par projectile | 90 % de P | 90 % de P |
| Munitions | 3 | 5 |
| Énergie | 10 points | 13 points |
| Recharge | 4 tours | 4 tours |

**Progression et calculs proposés :** Projectiles sur une cible = min(6, 3 + floor(r/4)) ; Dégâts par projectile = 60 + 3r ; Impact de comparaison = floor(D × (60 + 3r)/100 × M_distance) ; Précision par projectile = 90 ; Munitions = min(6, 3 + floor(r/4)) ; Énergie = 10 + floor(r/3) ; Recharge = 4.

**Fonctionnement proposé :** Toute la rafale est payée avant son premier projectile. Chaque projectile possède son jet de précision et ses défenses. Elle remplace la rafale ordinaire d'un PM ; elle ne multiplie pas ses trois projectiles par le nombre affiché. La cible principale est choisie avant confirmation. Portée de l'arme.

Intention visuelle conservée du catalogue : Impacts rapprochés et recul bref répété de l'arme. Rendu non créé dans cette passe.

### DI03 Volée

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

Intention visuelle conservée du catalogue : Éventail de trajectoires séparées devant l'agent. Rendu non créé dans cette passe.

### DI04 Ricochet

Type : actif. Rôle retenu : Utilise un rebond sur une surface connue pour changer la direction du tir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts après rebond | 90 % de D | 156 % de D |
| Impact de comparaison | 18 dégâts | 35 dégâts |
| Rebonds | 1 | 1 |
| Précision après rebond | 80 % de P | 80 % de P |
| Munitions | 1 | 1 |
| Énergie | 8 points | 11 points |
| Recharge | 4 tours | 4 tours |

**Progression et calculs proposés :** Dégâts après rebond = 90 + 6r ; Impact de comparaison = floor(D × (90 + 6r)/100 × M_distance) ; Rebonds = 1 ; Précision après rebond = 80 ; Munitions = 1 ; Énergie = 8 + floor(r/3) ; Recharge = 4.

**Fonctionnement proposé :** Choix d'une face de mur connue et visible, puis d'une cible actuellement observée. Les deux segments sont libres, sauf la surface du rebond, et leur longueur cumulée respecte la portée de l'arme. Un seul rebond ; pas de gain de portée ni de cible cachée révélée. L'aperçu montre les deux segments avant confirmation.

Intention visuelle conservée du catalogue : Trajectoire angulaire et étincelle au point de rebond. Rendu non créé dans cette passe.

### DI05 Tir de harpon

Type : actif. Rôle retenu : Accroche une cible et la rapproche si le passage est libre.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts | 100 % de D | 144 % de D |
| Impact de comparaison | 20 dégâts | 32 dégâts |
| Rapprochement maximal | 1 case | 3 cases |
| Munitions | 1 | 1 |
| Énergie | 10 points | 13 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Dégâts = 100 + 4r ; Impact de comparaison = floor(D × (100 + 4r)/100 × M_distance) ; Rapprochement maximal = min(3, 1 + floor(r/5)) ; Munitions = 1 ; Énergie = 10 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Un tir réussi rapproche la cible vers l'agent, jusqu'à la limite ou au premier obstacle. Aucun mur, acteur ou case interdite n'est traversé ; la cible ne finit pas sur l'agent. Une cible déclarée immobile ou ancrée garde sa position, mais peut subir les dégâts. Un échec du tir ne rapproche pas la cible.

Intention visuelle conservée du catalogue : Fil tendu entre l'impact et l'agent, rétraction visible. Rendu non créé dans cette passe.

### DI06 Tir de couverture

Type : actif. Rôle retenu : Prépare un tir de réaction contre un hostile entrant dans un secteur choisi.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts du tir de réaction | 100 % de D | 155 % de D |
| Impact de comparaison | 20 dégâts | 34 dégâts |
| Durée | 3 tours | 5 tours |
| Angle du secteur | 60 degrés | 60 degrés |
| Tirs par activation | 1 | 1 |
| Munitions payées à l'activation | 1 | 1 |
| Énergie | 8 points | 11 points |
| Recharge | 5 tours | 5 tours |

**Progression et calculs proposés :** Dégâts du tir de réaction = 100 + 5r ; Impact de comparaison = floor(D × (100 + 5r)/100 × M_distance) ; Durée = 3 + floor(r/5) ; Angle du secteur = 60 ; Tirs par activation = 1 ; Munitions payées à l'activation = 1 ; Énergie = 8 + floor(r/3) ; Recharge = 5.

**Fonctionnement proposé :** L'action met immédiatement la surveillance en place ; le tir a lieu plus tard, au premier hostile visible entrant dans le secteur à portée de l'arme. La munition et l'énergie sont payées à l'activation, même si aucun tir ne se déclenche. Mouvement, changement d'arme, expiration ou premier tir terminent la surveillance. La réaction ne donne pas une action supplémentaire à l'ennemi et ne lance pas une nouvelle surveillance.

Intention visuelle conservée du catalogue : Secteur de visée au sol puis trait déclenché au franchissement. Rendu non créé dans cette passe.

## Esquive

### T02 Pas spectral

Type : actif. Rôle retenu : Déplace d'une case légale et augmente brièvement l'esquive.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Déplacement | 1 case | 1 case |
| Bonus d'esquive | 15 unités | 48 unités |
| Durée | 2 tours | 4 tours |
| Énergie | 8 points | 11 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Déplacement = 1 ; Bonus d'esquive = 15 + 3r ; Durée = 2 + floor(r/5) ; Énergie = 8 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Reprend T02 du lot 02. Case adjacente libre et connue, mouvement ordinaire avec ses réactions. Le bonus commence avant ces réactions. Pas de mur traversé, de frappe gratuite ou d'invulnérabilité. Une nouvelle application remplace l'ancienne.

Intention visuelle conservée du catalogue : Silhouette décalée puis contour léger maintenu pendant le bonus. Rendu non créé dans cette passe.

### ES02 Roulade

Type : actif. Rôle retenu : Parcourt deux cases franchissables, sans attaquer gratuitement.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Déplacement maximal | 2 cases | 2 cases |
| Bonus d'esquive jusqu'au prochain tour | 10 unités | 32 unités |
| Durée du bonus | 1 tour | 1 tour |
| Énergie | 6 points | 9 points |
| Recharge | 4 tours | 4 tours |

**Progression et calculs proposés :** Déplacement maximal = 2 ; Bonus d'esquive jusqu'au prochain tour = 10 + 2r ; Durée du bonus = 1 ; Énergie = 6 + floor(r/3) ; Recharge = 4.

**Fonctionnement proposé :** Deux cases au maximum en ligne droite, toutes connues, libres et franchissables. Chaque case garde ses réactions et dangers. Le bonus commence avant le trajet. Aucun mur, acteur ou coin interdit traversé et aucune attaque ajoutée.

Intention visuelle conservée du catalogue : Mouvement bas en deux étapes et trace courte au sol. Rendu non créé dans cette passe.

### ES03 Leurre holographique

Type : actif. Rôle retenu : Place une fausse cible temporaire qui peut détourner les attaques.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Durée | 3 tours | 5 tours |
| Coups nécessaires pour détruire le leurre | 1 | 1 |
| Distance de placement | 3 cases | 3 cases |
| Énergie | 8 points | 11 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Durée = 3 + floor(r/4) ; Coups nécessaires pour détruire le leurre = 1 ; Distance de placement = 3 ; Énergie = 8 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Un seul leurre à la fois, sur une case libre actuellement visible. Un hostile qui le voit peut le prendre comme cible de diversion ; un acte agressif de l'agent contre cet hostile lui rend la priorité. Le premier impact contre le leurre le détruit. Le leurre ne bloque pas le passage, ne fait pas de dégâts et ne donne ni XP ni loot. C'est un effet de perception, sans invocation de drone.

Intention visuelle conservée du catalogue : Double translucide avec lignes de balayage verticales. Rendu non créé dans cette passe.

### ES04 Riposte d'esquive

Type : passif. Rôle retenu : Après une attaque esquivée, ouvre une riposte limitée si l'adversaire est à portée.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de riposte | 60 % d'une attaque de mêlée | 104 % d'une attaque de mêlée |
| Dégâts de comparaison | 12 dégâts | 23 dégâts |
| Intervalle entre ripostes | 2 tours | 2 tours |

**Progression et calculs proposés :** Dégâts de riposte = 60 + 4r ; Dégâts de comparaison = floor(20 × (60 + 4r)/100 × M_mêlée) ; Intervalle entre ripostes = 2.

**Fonctionnement proposé :** Après une attaque réellement esquivée, une riposte est possible uniquement si l'attaquant est visible et à portée de l'arme de mêlée ou de l'attaque naturelle compatible. Elle fait un jet de précision normal, sans énergie ni action manuelle supplémentaire. Un coup absorbé par l'armure n'est pas une esquive. La riposte ne déclenche ni nouvelle riposte ni autre attaque automatique.

Intention visuelle conservée du catalogue : Bref chevron retourné vers l'attaquant puis éclat de riposte. Rendu non créé dans cette passe.

### ES05 Traversée de phase

Type : actif. Rôle retenu : Passe de l'autre côté d'un ennemi adjacent si la destination est légale.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Passage derrière un hostile adjacent | 2 cases parcourues | 2 cases parcourues |
| Bonus d'esquive | 10 unités | 32 unités |
| Durée du bonus | 2 tours | 2 tours |
| Énergie | 10 points | 13 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Passage derrière un hostile adjacent = 2 ; Bonus d'esquive = 10 + 2r ; Durée du bonus = 2 ; Énergie = 10 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Passe de l'autre côté d'un hostile adjacent, sur la case opposée connue et libre. Le passage à travers cet acteur est autorisé par ce pouvoir, mais pas à travers un mur, une porte fermée ou un coin interdit. Aucun dégât n'est infligé. Le bonus commence avant les réactions au déplacement.

Intention visuelle conservée du catalogue : Silhouette fragmentée en bandes au passage de la cible. Rendu non créé dans cette passe.

### ES06 Réflexes balistiques

Type : passif. Rôle retenu : Renforce l'esquive contre les projectiles, sans renforcer toutes les défenses.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Esquive supplémentaire contre projectiles | 12 unités | 45 unités |

**Progression et calculs proposés :** Esquive supplémentaire contre projectiles = 12 + 3r.

**Fonctionnement proposé :** Bonus seulement contre un projectile qui utilise un jet de précision. Il ne devient pas une réduction générale des dégâts et ne crée pas un jet d'esquive pour un sort qui n'en a pas.

Intention visuelle conservée du catalogue : Courte déviation visuelle au passage d'un projectile évité. Rendu non créé dans cette passe.

## Feu

### FE01 Braise perçante

Type : actif. Rôle retenu : Projectile étroit qui blesse puis laisse une brûlure sur la cible touchée.

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

Intention visuelle conservée du catalogue : Braise allongée, queue sombre et point incandescent persistant. Rendu non créé dans cette passe.

### FE02 Souffle de fournaise

Type : actif. Rôle retenu : Projette du feu en cône ; maintenir le souffle coûte du temps et de l'énergie.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par maintien | 6 dégâts | 30 dégâts |
| Portée du cône | 3 cases | 5 cases |
| Angle | 90 degrés | 90 degrés |
| Résolutions maximales | 3 | 3 |
| Énergie par résolution | 6 points | 8 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Dégâts par maintien = floor((6 + 2r) × M_magie) ; Portée du cône = min(5, 3 + floor(r/5)) ; Angle = 90 ; Résolutions maximales = 3 ; Énergie par résolution = 6 + floor(r/4) ; Recharge = 6.

**Fonctionnement proposé :** Le lancement produit le premier souffle immédiatement. Deux maintiens au maximum, chacun prenant une action et son énergie. Le joueur peut arrêter ; mouvement, autre action ou perte de PV termine la canalisation. La recharge ne bloque pas la commande distincte de maintien. Les murs arrêtent le cône ; les alliés peuvent être touchés.

Intention visuelle conservée du catalogue : Langues de feu étagées devant les mains. Rendu non créé dans cette passe.

### FE03 Mur de feu

Type : actif. Rôle retenu : Crée une ligne temporaire dangereuse à traverser.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de zone par tour | 5 dégâts | 29 dégâts |
| Longueur maximale du mur | 3 cases | 7 cases |
| Durée | 3 tours | 5 tours |
| Portée de placement | 4 cases | 4 cases |
| Énergie | 12 points | 15 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Dégâts de zone par tour = floor((5 + 2r) × M_magie) ; Longueur maximale du mur = min(7, 3 + 2 × floor(r/5)) ; Durée = 3 + floor(r/4) ; Portée de placement = 4 ; Énergie = 12 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Ligne de flammes sur des cases visibles et franchissables ; les obstacles interrompent la ligne. Elle n'est pas un mur de déplacement. Une entrée ou un tour passé dans le feu blesse, au maximum une fois par acteur et par tour pour cet effet. Les alliés et l'agent peuvent être blessés.

Intention visuelle conservée du catalogue : Bande de flammes verticales, interruptions aux obstacles. Rendu non créé dans cette passe.

### FE04 Éruption

Type : actif. Rôle retenu : Marque une zone puis la fait exploser avec un délai annoncé.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de l'explosion | 14 dégâts | 63 dégâts |
| Rayon | 1 case | 1 case |
| Portée | 5 cases | 5 cases |
| Délai annoncé | 1 tour complet | 1 tour complet |
| Énergie | 12 points | 15 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Dégâts de l'explosion = floor((14 + 4r) × M_magie) ; Rayon = 1 ; Portée = 5 ; Délai annoncé = 1 ; Énergie = 12 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Le marqueur est posé dès le lancement. L'explosion survient après un tour complet, en laissant une réponse aux ennemis. Le rayon est arrêté par les murs. Les acteurs peuvent quitter la zone ; les alliés peuvent être touchés. Pas de destruction de terrain ni de nouvelle carte.

Intention visuelle conservée du catalogue : Fissures lumineuses au sol, puis jets de feu vers le haut. Rendu non créé dans cette passe.

### FE05 Foyer vorace

Type : actif. Rôle retenu : Une zone de braises s'étend de façon limitée lorsqu'une victime y meurt.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par tour | 4 dégâts | 28 dégâts |
| Rayon initial | 1 case | 1 case |
| Rayon maximal après des morts dans le foyer | 2 cases | 3 cases |
| Durée | 4 tours | 6 tours |
| Portée | 4 cases | 4 cases |
| Énergie | 14 points | 17 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Dégâts par tour = floor((4 + 2r) × M_magie) ; Rayon initial = 1 ; Rayon maximal après des morts dans le foyer = min(3, 2 + floor(r/10)) ; Durée = 4 + floor(r/4) ; Portée = 4 ; Énergie = 14 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** À chaque mort hostile dans le foyer, son rayon gagne une case jusqu'à sa limite. Un décès n'est compté qu'une fois. L'expansion n'allonge pas sa durée et respecte les murs. Les flammes blessent aussi les alliés ; une seule résolution par acteur et par tour.

Intention visuelle conservée du catalogue : Noyau de braises gagnant de nouvelles cases par pulsations. Rendu non créé dans cette passe.

### FE06 Combustion

Type : actif. Rôle retenu : Consume la brûlure restante d'une cible pour produire un choc immédiat.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Part de la brûlure restante convertie | 100 % | 144 % |
| Portée | 5 cases | 5 cases |
| Énergie | 6 points | 9 points |
| Recharge | 3 tours | 3 tours |

**Progression et calculs proposés :** Part de la brûlure restante convertie = 100 + 4r ; Portée = 5 ; Énergie = 6 + floor(r/3) ; Recharge = 3.

**Fonctionnement proposé :** Consomme la brûlure encore active appartenant à l'agent sur une cible visible. Le choc est calculé sur ses dégâts restants avant défenses, puis résolu en feu contre la cible. La Volonté déjà incluse dans la brûlure n'est pas multipliée une seconde fois. Sans brûlure admissible, refus sans coût. Pas de consommation des flammes d'un autre auteur ou d'une zone de sol.

Intention visuelle conservée du catalogue : Feu resserré sur la victime puis flash concentré, fin des flammes persistantes. Rendu non créé dans cette passe.

## Froid

### FR01 Aiguille de givre

Type : actif. Rôle retenu : Blesse une cible et ralentit ses déplacements.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts directs | 7 dégâts | 44 dégâts |
| Surcoût du déplacement | 30 % | 45,7 % |
| Durée | 3 tours | 5 tours |
| Portée | 5 cases | 5 cases |
| Puissance de contrôle | 100 unités | 165 unités |
| Énergie | 6 points | 9 points |
| Recharge | 2 tours | 2 tours |

**Progression et calculs proposés :** Dégâts directs = floor((7 + 3r) × M_magie) ; Surcoût du déplacement = 30 + 30r/(n + 9) ; Durée = 3 + floor(r/5) ; Portée = 5 ; Puissance de contrôle = 100 + 5r + 2(Volonté − 5) ; Énergie = 6 + floor(r/3) ; Recharge = 2.

**Fonctionnement proposé :** L'impact blesse immédiatement. Le ralentissement, soumis à résistance, augmente le coût en temps du mouvement, pas celui de toutes les attaques. Il ne réduit pas la précision et ne supprime aucune action.

Intention visuelle conservée du catalogue : Aiguille blanche, petits cristaux sur ses points d'appui. Rendu non créé dans cette passe.

### FR02 Nova de givre

Type : actif. Rôle retenu : Immobilise brièvement les ennemis proches, sans supprimer leurs attaques.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par hostile | 4 dégâts | 28 dégâts |
| Durée | 2 tours | 4 tours |
| Rayon autour de l'agent | 2 cases | 2 cases |
| Puissance de contrôle | 100 unités | 165 unités |
| Énergie | 10 points | 13 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Dégâts par hostile = floor((4 + 2r) × M_magie) ; Durée = 2 + floor(r/5) ; Rayon autour de l'agent = 2 ; Puissance de contrôle = 100 + 5r + 2(Volonté − 5) ; Énergie = 10 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Vise les hostiles autour de l'agent, avec obstacles. Le gel bloque leurs déplacements volontaires s'il passe la résistance ; les attaques et autres actions restent possibles. Les déplacements forcés peuvent sortir une cible de sa position. Les téléportations restent des actions de déplacement et sont bloquées pendant ce gel.

Intention visuelle conservée du catalogue : Anneau givré qui s'élargit ; glace limitée aux pieds des cibles. Rendu non créé dans cette passe.

### FR03 Mur de glace

Type : actif. Rôle retenu : Crée un obstacle temporaire sur des cases libres.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV de chaque bloc | 10 PV | 47 PV |
| Longueur | 3 cases | 7 cases |
| Durée | 3 tours | 5 tours |
| Portée | 4 cases | 4 cases |
| Énergie | 12 points | 15 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** PV de chaque bloc = floor((10 + 3r) × M_magie) ; Longueur = min(7, 3 + 2 × floor(r/5)) ; Durée = 3 + floor(r/5) ; Portée = 4 ; Énergie = 12 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Blocs uniquement sur des cases connues, visibles, libres et franchissables. Ils bloquent déplacements et projectiles comme obstacles temporaires ; chaque bloc peut être détruit séparément, sans loot ni XP. Aucun acteur n'est repoussé pour faire place. La disparition ne cause pas de dégâts.

Intention visuelle conservée du catalogue : Blocs translucides qui montent depuis le sol et se fissurent à la fin. Rendu non créé dans cette passe.

### FR04 Orbe fragmenté

Type : actif. Rôle retenu : Un orbe progresse en libérant des éclats dans plusieurs directions.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par éclat | 4 dégâts | 28 dégâts |
| Progression de l'orbe | 1 case par tour | 1 case par tour |
| Étapes d'émission | 3 | 3 |
| Directions d'éclats par étape | 4 | 4 |
| Portée des éclats | 1 case | 3 cases |
| Énergie | 14 points | 17 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Dégâts par éclat = floor((4 + 2r) × M_magie) ; Progression de l'orbe = 1 ; Étapes d'émission = 3 ; Directions d'éclats par étape = 4 ; Portée des éclats = min(3, 1 + floor(r/5)) ; Énergie = 14 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Trajet initial de trois cases connues et visibles, en ligne droite. L'orbe progresse d'une case par tour puis émet dans les quatre directions cardinales ; il disparaît contre un mur ou après trois émissions. Les éclats sont arrêtés par murs et premier acteur rencontré. Un acteur ne reçoit qu'un éclat par émission. Les alliés peuvent être touchés. Aucun gel ajouté implicitement.

Intention visuelle conservée du catalogue : Boule lente à facettes et jets d'éclats détachés. Rendu non créé dans cette passe.

### FR05 Blizzard

Type : actif. Rôle retenu : Une zone subit des impacts de froid répartis sur plusieurs tours.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par impact | 6 dégâts | 30 dégâts |
| Impacts par tour | 2 | 4 |
| Durée | 3 tours | 5 tours |
| Rayon | 2 cases | 2 cases |
| Portée | 5 cases | 5 cases |
| Énergie | 16 points | 19 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Dégâts par impact = floor((6 + 2r) × M_magie) ; Impacts par tour = min(5, 2 + floor(r/5)) ; Durée = 3 + floor(r/5) ; Rayon = 2 ; Portée = 5 ; Énergie = 16 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** À chaque tour, les impacts sont répartis sans remplacement parmi les cases de la zone accessibles à l'effet. Une case reçoit au maximum un impact ce tour. L'effet commence au prochain tour complet et peut blesser les alliés. Les murs arrêtent l'aire ; ce sort ne donne pas automatiquement une immobilisation.

Intention visuelle conservée du catalogue : Grêle oblique et petites frappes dispersées dans la zone. Rendu non créé dans cette passe.

### FR06 Fragilité cristalline

Type : passif. Rôle retenu : Frapper un ennemi immobilisé par le froid projette des éclats vers des ennemis derrière lui.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par éclat | 3 dégâts | 15 dégâts |
| Cibles secondaires maximales | 2 | 4 |
| Allonge derrière la victime | 1 case | 3 cases |
| Déclenchements maximaux par tour | 1 | 1 |

**Progression et calculs proposés :** Dégâts par éclat = floor((3 + 1r) × M_magie) ; Cibles secondaires maximales = min(4, 2 + floor(r/5)) ; Allonge derrière la victime = min(3, 1 + floor(r/5)) ; Déclenchements maximaux par tour = 1.

**Fonctionnement proposé :** Un coup principal de l'agent contre un hostile immobilisé par le froid projette des éclats vers des hostiles visibles derrière lui. Le gel peut provenir d'une mine glacée. Les drones ne déclenchent pas ce passif. Les éclats ne redéclenchent pas Fragilité, ne libèrent pas automatiquement la cible et n'ajoutent pas de gel.

Intention visuelle conservée du catalogue : Éventail court jaillissant de la cible plutôt que du lanceur. Rendu non créé dans cette passe.

## Foudre

### FO01 Chaîne conductrice

Type : actif. Rôle retenu : L'éclair saute entre plusieurs ennemis proches, sans rebond illimité.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts du premier arc | 10 dégâts | 47 dégâts |
| Cibles maximales | 3 | 5 |
| Dégâts conservés par saut | 80 % | 80 % |
| Distance maximale d'un saut | 2 cases | 2 cases |
| Portée initiale | 5 cases | 5 cases |
| Énergie | 10 points | 13 points |
| Recharge | 4 tours | 4 tours |

**Progression et calculs proposés :** Dégâts du premier arc = floor((10 + 3r) × M_magie) ; Cibles maximales = min(6, 3 + floor(r/5)) ; Dégâts conservés par saut = 80 ; Distance maximale d'un saut = 2 ; Portée initiale = 5 ; Énergie = 10 + floor(r/3) ; Recharge = 4.

**Fonctionnement proposé :** Chaque saut prend le hostile visible admissible le plus proche, avec ligne de vue depuis la cible précédente. Pas de cible visitée deux fois. Les dégâts du saut k conservent 0,8 puissance k de la valeur de départ. Sans autre cible admissible, la chaîne s'arrête.

Intention visuelle conservée du catalogue : Arcs brisés reliant les cibles dans l'ordre des sauts. Rendu non créé dans cette passe.

### FO02 Champ statique

Type : actif. Rôle retenu : Retire une part des PV actuels des ennemis proches, avec un plancher de vie à définir.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Part des PV actuels avant résistances | 20 % | 36,4 % |
| Plancher des PV après ce seul effet | 50 % des PV maximum | 50 % des PV maximum |
| Rayon | 2 cases | 2 cases |
| Énergie | 12 points | 15 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Part des PV actuels avant résistances = (20 + 25r/(n+9)) × M_magie ; Plancher des PV après ce seul effet = 50 ; Rayon = 2 ; Énergie = 12 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Affecte les hostiles proches ; dégâts de foudre limités pour ne pas faire passer la cible sous la moitié de ses PV maximum. Une cible déjà sous ce plancher ne perd rien. Les résistances diminuent encore la perte. Le plancher concerne ce champ, pas un autre effet distinct déclenché pendant le tour. Le calcul ne peut pas donner un soin.

Intention visuelle conservée du catalogue : Cercle de décharge très bref et flash intérieur sur les victimes. Rendu non créé dans cette passe.

### FO03 Champ répulsif

Type : actif. Rôle retenu : Repousse les ennemis proches vers des cases libres.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Recul maximal | 1 case | 3 cases |
| Rayon | 2 cases | 2 cases |
| Puissance de contrôle | 100 unités | 165 unités |
| Énergie | 10 points | 13 points |
| Recharge | 6 tours | 6 tours |

**Progression et calculs proposés :** Recul maximal = min(3, 1 + floor(r/5)) ; Rayon = 2 ; Puissance de contrôle = 100 + 5r + 2(Volonté − 5) ; Énergie = 10 + floor(r/3) ; Recharge = 6.

**Fonctionnement proposé :** Repousse les hostiles proches si le contrôle passe leur résistance. Trajets radiaux libres, sans traversée d'acteur ni dégât de collision ajouté. Le sort ne donne pas de dégâts directs. Une cible ancrée ne bouge pas. Les dangers rencontrés conservent leurs règles.

Intention visuelle conservée du catalogue : Onde électrique sortante, étincelles sous les déplacements. Rendu non créé dans cette passe.

### FO04 Orage fixé

Type : actif. Rôle retenu : Une zone choisie reçoit des éclairs périodiques visant un ennemi à la fois.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par éclair | 10 dégâts | 47 dégâts |
| Durée | 3 tours | 5 tours |
| Rayon | 2 cases | 2 cases |
| Cibles par tour | 1 | 1 |
| Portée du centre | 5 cases | 5 cases |
| Énergie | 16 points | 19 points |
| Recharge | 10 tours | 10 tours |

**Progression et calculs proposés :** Dégâts par éclair = floor((10 + 3r) × M_magie) ; Durée = 3 + floor(r/4) ; Rayon = 2 ; Cibles par tour = 1 ; Portée du centre = 5 ; Énergie = 16 + floor(r/3) ; Recharge = 10.

**Fonctionnement proposé :** Un éclair par tour complet contre un hostile actuellement observé dans la zone, le plus proche du centre à égalité départagée par case. Sans hostile admissible, aucun éclair. Pas de tir sur un acteur caché, pas de nouvelle charge passive pour chaque frappe et pas de dégât aux alliés.

Intention visuelle conservée du catalogue : Nœud orageux au-dessus du sol et frappes verticales espacées. Rendu non créé dans cette passe.

### FO05 Circuit fermé

Type : actif. Rôle retenu : Relie deux ancrages par une ligne électrique qui blesse au franchissement.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts au franchissement | 6 dégâts | 30 dégâts |
| Durée | 3 tours | 5 tours |
| Distance maximale entre ancrages | 3 cases | 5 cases |
| Portée de chaque ancrage depuis l'agent | 5 cases | 5 cases |
| Énergie | 12 points | 15 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Dégâts au franchissement = floor((6 + 2r) × M_magie) ; Durée = 3 + floor(r/5) ; Distance maximale entre ancrages = min(6, 3 + floor(r/4)) ; Portée de chaque ancrage depuis l'agent = 5 ; Énergie = 12 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Deux cases libres et visibles reliées sans mur. La ligne n'est pas un obstacle. La franchir blesse au maximum une fois par acteur et par tour ; rester immobile dessus ne crée pas une nouvelle traversée. Alliés et agent peuvent être touchés. Les déplacements forcés restent des franchissements.

Intention visuelle conservée du catalogue : Deux bornes runiques et fil électrique tremblant entre elles. Rendu non créé dans cette passe.

### FO06 Accumulation électrique

Type : passif. Rôle retenu : Les sorts électriques chargent l'agent ; un seuil ouvre une décharge supplémentaire limitée.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts de la décharge | 4 dégâts | 28 dégâts |
| Sorts requis | 3 | 3 |
| Rayon de décharge autour de l'agent | 1 case | 1 case |
| Expiration sans nouveau sort électrique | 5 tours | 5 tours |

**Progression et calculs proposés :** Dégâts de la décharge = floor((4 + 2r) × M_magie) ; Sorts requis = 3 ; Rayon de décharge autour de l'agent = 1 ; Expiration sans nouveau sort électrique = 5.

**Fonctionnement proposé :** Un lancement légal d'un sort électrique offensif appris donne une charge, pas chaque cible, rebond ou tick. À trois charges, une décharge unique touche les hostiles proches puis vide les charges. Cette décharge ne produit aucune charge ni attaque automatique dérivée. Les charges expirent sans nouveau sort et au changement de carte.

Intention visuelle conservée du catalogue : Charges tournant autour des implants puis libération en étoile. Rendu non créé dans cette passe.

## Énergie occulte

### A01 Trait arcanique

Type : actif. Rôle retenu : Frappe directement une cible avec de l'énergie occulte.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts directs | 8 dégâts | 45 dégâts |
| Portée | 5 cases | 5 cases |
| Énergie | 6 points | 9 points |
| Recharge | 2 tours | 2 tours |

**Progression et calculs proposés :** Dégâts directs = floor((8 + 3r) × M_magie) ; Portée = 5 ; Énergie = 6 + floor(r/3) ; Recharge = 2.

**Fonctionnement proposé :** Reprend A01 du lot 02 : hostile visible avec ligne de vue, pas de jet de précision d'arme, défenses occultes ordinaires. Le sort ne donne aucun affaiblissement supplémentaire.

Intention visuelle conservée du catalogue : Trait violet court, nœud noir au centre de l'impact. Rendu non créé dans cette passe.

### A02 Déflagration occulte

Type : actif. Rôle retenu : Frappe les acteurs d'une petite zone choisie, selon la fiche existante.

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

**Fonctionnement proposé :** Sorts proposés compatibles : FE01, FR01, FO01, A01, A02, FE04, FR05, FO04. Le sort doit être appris et disponible. Son origine, sa portée et sa ligne de vue sont calculées depuis un drone déployé réellement observé. Le lancement paie ensemble le sort et le surcoût et engage leurs deux recharges. Canalisation n'est appliquée qu'une fois à chaque coût admissible. Le relais ne prête pas la Volonté du drone, ne transmet pas le cargo, ne contourne pas l'observation et ne fait pas apprendre le sort. Un seul lancement, une seule attribution des déclenchements passifs.

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

Type : actif. Rôle retenu : Renvoie une part des dégâts reçus au contact vers leur auteur.

| Paramètre | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts au contact renvoyés | 10 % | 23,1 % |
| Durée | 4 tours | 6 tours |
| Énergie | 10 points | 13 points |
| Recharge | 8 tours | 8 tours |

**Progression et calculs proposés :** Dégâts au contact renvoyés = 10 + 25r/(n + 9) ; Durée = 4 + floor(r/5) ; Énergie = 10 + floor(r/3) ; Recharge = 8.

**Fonctionnement proposé :** Renvoi occulte sur les PV effectivement perdus après protections lors d'une attaque de mêlée hostile. L'auteur doit encore être une cible admissible au contact. Le renvoi respecte sa résistance, ne vole pas de vie, ne charge pas les séries et ne déclenche pas une nouvelle riposte ou un autre renvoi. Aucun renvoi de poison ou de zone.

Intention visuelle conservée du catalogue : Pointes lumineuses sur l'armure et petites épines repartant à l'impact. Rendu non créé dans cette passe.

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

Ces quatorze compétences conservent les achats libres. Les actifs SF00,
SM00 et SG00 s'achètent une seule fois. Le rang 12 de leurs passifs signifie
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
