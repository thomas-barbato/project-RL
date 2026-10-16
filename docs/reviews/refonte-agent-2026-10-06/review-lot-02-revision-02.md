# Revue des compétences des quatre autres classes

6 octobre 2026. Ce lot propose les implants initiaux, les compétences de
départ et une deuxième compétence à apprendre pour l'Opérateur, le
Traqueur, l'Arcaniste et le Métamorphe. Les noms de classes et leurs
orientations sont acceptés. Les textes et effets de ce lot attendent
l'accord du joueur ; ils ne sont pas actifs dans le jeu.

Chaque personnage commence avec l'actif 01 et le passif 03 de sa classe,
au niveau appris 1. L'actif 02 s'apprend avec un point. Tous ces pouvoirs
sont accessibles aux autres classes. Les implants donnent une paire de
traits remplaçable par un implant trouvé ; les compétences restent acquises.

## Lecture des fiches

Les citations sont les textes proposés à l'écran. Les paramètres restent
séparés. Le symbole n désigne le niveau effectif : niveau appris, de 1
à 10, plus bonus des objets équipés. Les valeurs à 12 illustrent le
dépassement par l'équipement. Les coûts indiqués sont ceux avant autres
modificateurs ; l'arrondi supérieur est appliqué à la fin du coût total.

Les formules d'attributs sont celles du
[premier lot révisé](REVUE_TEXTES_EFFETS_AGENT.md). Les tableaux isolent
le niveau de compétence ; attributs, implants et autres passifs s'appliquent
ensuite. Chaque fiche précise les attributs concernés. Les dégâts sont
arrondis une seule fois à la fin et passent par les défenses de la cible.
Les portées de ciblage de ce lot utilisent la distance euclidienne.
La mêlée et les déplacements suivent leurs règles ordinaires ; la zone
de 3 × 3 cases d'A02 est définie explicitement et ne change pas cette règle.

| Fiches | Texte | Effets mécaniques | Nouveaux rendus |
| --- | --- | --- | --- |
| O00 à O03 — Opérateur | À valider | À valider | À présenter en situation |
| T00 à T03 — Traqueur | À valider | À valider | À présenter en situation |
| A00 à A03 — Arcaniste | À valider | À valider | À présenter en situation |
| M00 à M03 — Métamorphe | À valider | À valider | À présenter en situation |

## Opérateur

### O00 Implant initial de l'Opérateur

> Vos drones sont plus solides et vos affaiblissements durent plus longtemps.
> Vos armes infligent moins de dégâts.

**Proposition :** +15 % de PV maximum aux drones contrôlés, +1 tour aux
affaiblissements techno-occultistes infligés, −10 % de dégâts des armes.
Le malus s'applique aux armes de mêlée et à distance et aux techniques
qui les renforcent. Il ne réduit pas les dégâts d'un drone, d'un sort
ou des attaques naturelles d'une forme.

Le bonus de PV du drone s'applique après Intelligence ; les éventuels
autres bonus de PV en pourcentage s'ajoutent à ces 15 % avant application.
Il ne rend pas de PV aux drones déjà blessés. Remplacer l'implant recalcule
ce bonus de maximum sans soin gratuit. Le tour de durée est inclus au
lancement d'un affaiblissement ; remplacer l'implant ne prolonge pas un
effet déjà actif. Aucun bonus de nombre de drones n'est ajouté.

### O01 Déploiement d'un drone

> Déployez un drone qui combat à vos côtés pendant quelques tours.

**Proposition :** fait apparaître un drone de tir allié sur une case
libre adjacente visible. Un seul drone issu de cette compétence peut
être actif par personnage. Le drone suit l'agent et agit une fois par
tour de référence ; aucune action gratuite n'est produite dans un menu.
Il disparaît lorsque sa durée s'achève ou que ses PV tombent à zéro.

| Niveau effectif | PV de base | Dégâts de base par tir | Durée de base | Énergie |
| --- | --- | --- | --- | --- |
| 1 | 20 | 4 | 8 tours | 12 |
| 10 | 56 | 13 | 11 tours | 15 |
| 12 | 64 | 15 | 11 tours | 15 |

Formules : PV = 20 + 4 × (n − 1) ; dégâts = 4 + (n − 1) ;
durée = 8 + floor((n − 1) / 3) ; énergie = 12 + floor((n − 1) / 3).
Recharge : 6 tours. Déploiement : une action ordinaire. Portée du tir :
4 cases, avec obstacles et ligne de vue. Dégâts cinétiques.

**Attributs :** Intelligence multiplie PV et dégâts. Le passif O03
multiplie aussi les dégâts ; le bonus de l'implant O00 concerne les PV.
La précision de référence du drone est 100 avant conditions de tir,
opposée à l'esquive adverse selon la règle du premier lot.
Son maximum de vie est recalculé si Intelligence ou un bonus d'implant
change, sans soin gratuit. Sa durée et ses valeurs de base liées au niveau
du déploiement restent celles de son apparition.

Le joueur peut désigner une cible hostile actuellement visible. Sans
ordre, le drone vise un hostile visible par l'agent et dans sa propre
ligne de vue, en donnant priorité à la cible la plus proche ; à égalité,
un ordre stable départage les cibles. Il ne révèle pas de cible cachée.
Son déplacement respecte les obstacles et les cases occupées.

Il emploie sa propre réserve intégrée de tir pendant cette durée, sans
retirer les munitions du joueur. Aucun objet ni expérience n'est accordé
pour sa disparition. Un déploiement impossible, une énergie insuffisante
ou un drone déjà actif entraîne un refus sans coût, tour ni recharge.
Le modèle parmi les drones déjà dessinés reste à affecter et à montrer.

### O02 Interférence

> Réduisez les dégâts infligés par une cible.

**Proposition :** affaiblissement techno-occultiste sur une cible hostile
visible à 5 cases au plus. Il fonctionne contre les machines, les êtres
organiques et les démons. Il réduit les dégâts directs que cette cible
inflige ; il ne retire pas ses actions et ne réduit pas ses soins.

Réduction en pourcentage = 15 + 35 × (n − 1) / (n + 9).
La courbe progresse au-delà de 10 et reste strictement sous 50 %.
Durée de base = 3 + floor((n − 1) / 5).
Énergie = 8 + floor((n − 1) / 3) ; recharge de 5 tours ; une action.

| Niveau effectif | Réduction | Durée de base | Énergie |
| --- | --- | --- | --- |
| 1 | 15 % | 3 tours | 8 |
| 10 | 31,6 % | 4 tours | 11 |
| 12 | 33,3 % | 5 tours | 11 |

**Attributs :** Intelligence ajoute sa contribution de durée et
2 × (Intelligence − 5) à une puissance d'application de base
100 + 5 × (n − 1). Cette puissance s'oppose à la résistance occulte de
la cible. Une tentative légale résistée consomme l'action, l'énergie
et la recharge. Un refus avant lancement n'en consomme aucune.

L'implant O00 ajoute son tour de durée. Deux Interférences ne s'additionnent
pas : une application réussie remplace la précédente par les nouveaux
paramètres. Elle ne prolonge pas indéfiniment une ancienne version plus forte.
Les dégâts persistants déjà posés ne sont pas modifiés rétroactivement.
Cette compétence possède les deux caractères technologique et magique ;
le passif A03 peut réduire son coût, une seule fois.

### O03 Lien renforcé

> Vos drones infligent plus de dégâts.

**Proposition :** passif permanent, sans coût ni recharge.
Bonus de dégâts des drones en pourcentage = 25 + 2 × (n − 1).
Il vaut +25 % au niveau 1, +43 % au niveau 10 et +47 % au niveau 12.
Le multiplicateur se combine avec Intelligence avant les défenses.
Au départ de l'Opérateur, avec Intelligence 8 et un drone au niveau 1,
ses dégâts passent de floor(4 × 1,06) = 4 à
floor(4 × 1,06 × 1,25) = 5, avant les défenses de la cible.

Il concerne les drones contrôlés par le personnage, y compris les futurs
modèles dont les fiches confirmeront ce type. Il ne renforce pas un allié
humain ni le personnage transformé. Le niveau du passif et Intelligence
sont lus au moment d'un tir ; le niveau de déploiement du drone et sa
durée sont ceux calculés à son apparition. Retirer un bonus d'équipement
sur ce passif modifie donc ses prochains tirs.

## Traqueur

### T00 Implant initial du Traqueur

> Vos armes à distance sont plus efficaces et vous esquivez mieux.
> Vos armes de mêlée infligent moins de dégâts.

**Proposition :** +10 % de dégâts des armes à distance et +2 unités
d'esquive ; −10 % de dégâts des armes de mêlée. Les techniques qui
renforcent ces attaques reçoivent les mêmes modificateurs. Les sorts,
drones et attaques naturelles des formes ne reçoivent pas ces dégâts
supplémentaires ou ce malus. Le bonus d'esquive s'ajoute à Agilité.
Le changement d'implant ne modifie pas les compétences apprises.

### T01 Marquage balistique

> Renforcez votre prochain tir contre une cible.

**Proposition :** marque un hostile visible à 6 cases au plus. La prochaine
attaque ordinaire d'une arme à distance du joueur contre cette cible
reçoit un bonus de dégâts de 40 + 10 × (n − 1) pour cent, avant défenses.
La marque dure 4 + floor((n − 1) / 5) tours et concerne une seule cible.

| Niveau effectif | Dégâts supplémentaires | Durée de la marque | Énergie |
| --- | --- | --- | --- |
| 1 | +40 % | 4 tours | 6 |
| 10 | +130 % | 5 tours | 9 |
| 12 | +150 % | 6 tours | 9 |

Énergie = 6 + floor((n − 1) / 3) ; recharge de 3 tours ; marquer prend
une action ordinaire. Le tir vient ensuite et consomme ses propres
munitions et son temps. La marque ne change pas la portée de l'arme.
Le marquage est magique et son coût peut être réduit par A03 ; le tir
qui l'utilise reste une attaque d'arme.

**Attributs :** le tir emploie Perception pour les dégâts à distance et
la précision, et Agilité pour la précision et son petit bonus de dégâts
d'armes. Le bonus de marquage se multiplie avec ces contributions.
Les drones et les sorts ne consomment ni ne bénéficient de la marque.

Un tir légal contre la cible consomme la marque même s'il manque. Un
tir impossible ne la consomme pas. Tirer sur une autre cible ne l'utilise
pas, mais sa durée continue. Un nouveau marquage remplace l'ancien.
Une rafale ne multiplie pas le bonus par le nombre de projectiles : il
renforce les dégâts directs de l'attaque sur la cible principale une fois,
sans renforcer les dégâts de zone sur les autres acteurs.

### T02 Pas spectral

> Déplacez-vous d'une case et esquivez mieux pendant quelques tours.

**Proposition :** déplacement vers une case adjacente libre et connue,
suivant les règles ordinaires de déplacement, puis bonus temporaire
d'esquive de 15 + 3 × (n − 1) unités. Il ne traverse pas un mur ou un
obstacle et ne rend pas invulnérable. Les réactions au déplacement restent
possibles ; le bonus commence avant celles-ci.

| Niveau effectif | Esquive supplémentaire | Durée | Énergie |
| --- | --- | --- | --- |
| 1 | +15 | 2 tours | 8 |
| 10 | +42 | 3 tours | 11 |
| 12 | +48 | 4 tours | 11 |

Durée = 2 + floor((n − 1) / 5) ; énergie = 8 + floor((n − 1) / 3) ;
recharge de 6 tours ; une action ordinaire. Agilité et l'équipement
apportent leur esquive habituelle en plus de ce bonus. Les chances restent
calculées contre la précision de l'attaquant, avec les bornes ordinaires.

Une case illégale ou une ressource insuffisante entraîne un refus sans
temps ni coût. Deux protections issues de ce pouvoir ne s'additionnent
pas ; un nouveau lancement remplace la précédente. Ce déplacement est
magique et peut bénéficier d'A03. Il déclenche la condition de T03.

### T03 Tir mobile

> Vos tirs sont plus puissants juste après un déplacement.

**Proposition :** passif permanent. Lorsqu'un tir ordinaire d'une arme
suit immédiatement un déplacement du joueur, ses dégâts sur la cible
principale gagnent 10 + 2 × (n − 1) pour cent : +10 % au niveau 1,
+28 % au niveau 10 et +32 % au niveau 12.

Le déplacement normal et T02 ouvrent cette possibilité. Une autre action,
une attente ou un autre tir la termine ; consulter un menu ne compte pas
comme action. Le premier tir légal consomme cette possibilité même s'il
manque. Un tir impossible ne la consomme pas. Un changement de carte
termine cette possibilité. Un mouvement imposé par un ennemi ne l'ouvre pas.

Le passif ne coûte pas d'énergie, ne crée pas de commande et n'a pas de
recharge. Il se multiplie avec Perception, Agilité, le marquage actif et
les autres bonus applicables à l'attaque. Les sorts et les drones sont exclus.

## Arcaniste

### A00 Implant initial de l'Arcaniste

> Vos sorts infligent plus de dégâts, mais vous avez moins de vie.

**Proposition :** +15 % de dégâts des pouvoirs magiques ; −10 % de PV maximum.
Le multiplicateur de dégâts s'applique aux sorts directs, aux zones et
aux actions magiques des formes. Il ne renforce pas les dégâts d'une arme
ordinaire ni ceux d'un drone. Une technique renforçant une arme reste
une attaque d'arme pour ce calcul.

Le malus de vie s'ajoute aux autres pourcentages de maximum de vie, puis
s'applique au maximum de base augmenté des bonus fixes, avec un arrondi
inférieur final. Le remplacement par un implant trouvé ne soigne pas.

### A01 Trait arcanique

> Frappez une cible avec un trait d'énergie occulte.

**Proposition :** dégâts occultes directs sur un hostile visible à 5 cases
au plus, avec ligne de vue. Dégâts de base = 8 + 3 × (n − 1).
Énergie = 6 + floor((n − 1) / 3) ; recharge de 2 tours ; une action.

| Niveau effectif | Dégâts de base | Énergie avant passifs |
| --- | --- | --- |
| 1 | 8 | 6 |
| 10 | 35 | 9 |
| 12 | 41 | 9 |

**Attributs :** Volonté multiplie les dégâts ; Intelligence et Volonté
contribuent séparément à l'énergie disponible. A00 peut renforcer les
dégâts ; A03 peut réduire le coût. Ce sort atteint une cible légalement
visée sans jet de précision d'arme ; ses résistances et défenses aux
dégâts occultes s'appliquent. Il n'inflige pas d'affaiblissement supplémentaire.

Une cible hors de portée ou non visible entraîne un refus sans temps,
énergie ni recharge. Un lancement légal absorbé par les défenses conserve
ses coûts. L'aspect du trait sera présenté avant son intégration.

### A02 Déflagration occulte

> Faites exploser l'énergie occulte dans une petite zone.

**Proposition :** choisit une case visible à 4 cases au plus, avec ligne
de vue. Inflige 6 + 2 × (n − 1) dégâts occultes de base dans la zone
de 3 × 3 cases centrée sur elle. Les murs arrêtent l'effet vers les cases
séparées du centre par un obstacle. Le rayon ne grandit pas avec le niveau.

| Niveau effectif | Dégâts de base par acteur | Énergie avant passifs |
| --- | --- | --- |
| 1 | 6 | 10 |
| 10 | 24 | 13 |
| 12 | 28 | 13 |

Énergie = 10 + floor((n − 1) / 3) ; recharge de 4 tours ; une action.
Volonté et A00 renforcent les dégâts ; A03 peut réduire le coût.
Les résistances de chaque acteur s'appliquent séparément.

L'explosion peut toucher le joueur et ses alliés, drones compris. L'aperçu
montre les cases concernées et les acteurs déjà observés ; il ne révèle
pas les acteurs cachés. La zone ne met pas le terrain en feu et ne détruit
pas un mur dans cette version. Feu, froid et foudre auront des sorts
distincts dans le prochain catalogue.

### A03 Canalisation

> Vos pouvoirs magiques consomment moins d'énergie.

**Proposition :** passif permanent. Réduction du coût en pourcentage =
20 + 30 × (n − 1) / (n + 9). Elle vaut 20 % au niveau 1, 34,2 % au
niveau 10 et 35,7 % au niveau 12 ; elle progresse au-delà de 10 et reste
strictement inférieure à 50 %.

Le coût de base est multiplié par (1 − réduction / 100) et par les autres
modificateurs, puis arrondi au supérieur une seule fois. Une action ayant
un coût positif conserve un minimum de 1 point d'énergie. Au niveau 1,
un sort coûtant 6 de base coûte donc 5, avant autre modificateur.

Ce passif concerne les sorts, protections, mouvements magiques,
transformations et compétences hybrides portant le caractère magique.
Il ne réduit pas le coût d'un déploiement purement technologique, les
munitions d'une arme ou le temps d'une action. Il ne nécessite ni posture,
ni attente sur place, ni équipement dans un emplacement de passif.

## Métamorphe

### M00 Implant initial du Métamorphe

> Vos transformations durent plus longtemps.
> Vos pouvoirs coûtent plus d'énergie sous forme humaine.

**Proposition :** +2 tours à la durée d'une transformation temporaire ;
+10 % de coût d'énergie des compétences sous forme humaine. Le malus
concerne les pouvoirs technologiques et magiques ; les tirs ordinaires,
leurs munitions et la récupération d'énergie ne reçoivent pas ce malus.

Le coût d'entrée dans une forme est payé alors que le personnage est
humain et reçoit donc le malus. Une fois transformé, les autres pouvoirs
n'en reçoivent plus. Le bonus de durée est calculé au lancement ; remplacer
l'implant ne prolonge pas une forme déjà active. A03 peut réduire le coût
des pouvoirs magiques après combinaison des modificateurs, avant l'arrondi.

### M01 Mue prédatrice

> Prenez une forme de prédateur pour combattre au contact et bondir.

**Proposition :** transformation temporaire remplaçant l'apparence et
les attaques ordinaires par deux actions de forme : Griffe et Bond.
Les armes ordinaires ne peuvent pas être utilisées pendant la forme.
Les compétences apprises qui ne nécessitent pas une arme restent
utilisables ; celles nécessitant une arme sont refusées sans coût.

| Niveau effectif | Dégâts de base de Griffe | Durée de base | Énergie avant traits |
| --- | --- | --- | --- |
| 1 | 8 | 6 tours | 12 |
| 10 | 26 | 9 tours | 15 |
| 12 | 30 | 9 tours | 15 |

Formules : dégâts de Griffe = 8 + 2 × (n − 1) ;
durée = 6 + floor((n − 1) / 3) ; énergie = 12 + floor((n − 1) / 3).
Recharge : 10 tours. L'entrée dans la forme prend une action ordinaire.
Volonté ajoute sa contribution de durée ; M00 ajoute deux tours.
Les niveaux élevés renforcent toujours Griffe, même entre deux paliers
de durée. Les coûts continuent au-delà de 10.

**Griffe :** une attaque naturelle de mêlée adjacente, cinétique, une
action ordinaire, sans coût d'énergie supplémentaire ni recharge propre.
Force et Agilité renforcent ses dégâts ; Agilité contribue à la précision.
Frappe de rupture peut utiliser Griffe si elle est apprise, avec son propre
niveau effectif et ses coûts. Une attaque légale peut manquer.

**Bond :** déplacement de deux cases au plus en ligne droite, une action,
4 points d'énergie et une recharge de 3 tours. Chaque case traversée doit
être connue, libre et franchissable selon les règles ordinaires. Aucun
mur, porte fermée, acteur ou coin interdit n'est traversé. Les réactions
au déplacement restent possibles. Le bond n'inflige pas une attaque gratuite
et ne franchit pas de changement de carte. Une destination illégale
entraîne un refus sans coût ni temps. Bond est une action naturelle de
forme ; A03 ne réduit pas son coût. La transformation elle-même est magique.

**Équipement proposé pendant la forme :** objets conservés dans leurs
emplacements ; bonus d'attribut, bonus de compétences et protections des
armures toujours actifs. Les dégâts, munitions et affixes d'attaque propres
à une arme rangée ne renforcent pas Griffe. L'équipement et son apparence
habituelle se retrouvent au retour. Les PV maximum ne changent pas du seul
fait d'entrer dans cette forme ; aucun soin ou recharge n'est accordé.

Les passifs appris restent actifs. La forme utilise le niveau effectif
calculé lors de son entrée pour ses paramètres de base ; les attributs et
passifs actuels s'appliquent au moment des actions. Sa durée couvre les
réponses ennemies après l'activation. La fin est automatique à expiration ;
un retour volontaire prend une action ordinaire sans énergie. Une entrée
alors que cette forme est déjà active est refusée sans coût. Revenir à la
forme humaine ne remet pas les recharges à zéro.

L'affectation à l'un des dessins de transformation existants et le rendu
des deux actions seront présentés en situation avant intégration.

### M02 Décharge mutagène

> Frappez une cible proche avec une décharge d'énergie occulte.

**Proposition :** dégâts occultes directs sur un hostile visible à 3 cases
au plus, avec ligne de vue. Dégâts de base = 7 + 2 × (n − 1).
Énergie = 6 + floor((n − 1) / 3) ; recharge de 2 tours ; une action.

| Niveau effectif | Dégâts de base | Énergie avant traits |
| --- | --- | --- |
| 1 | 7 | 6 |
| 10 | 25 | 9 |
| 12 | 29 | 9 |

Volonté renforce les dégâts ; A00 peut aussi les renforcer et A03 réduire
le coût. Le sort peut être utilisé sous forme humaine ou transformée.
M00 ajoute son malus de coût seulement sous forme humaine.
Comme A01, une cible légalement visée est atteinte sans jet d'arme, puis
ses défenses s'appliquent. Cette première version n'ajoute ni zone,
ni poison, ni mutation permanente à la cible.

### M03 Adaptation

> Vous subissez moins de dégâts directs.

**Proposition :** passif permanent. Réduction des dégâts directs en
pourcentage = 5 + 15 × (n − 1) / (n + 9).
Elle vaut 5 % au niveau 1, 12,1 % au niveau 10 et 12,9 % au niveau 12.
Elle continue au-delà de 10 et reste strictement inférieure à 20 %.

Le passif agit sous forme humaine et transformée, après armure et
résistances. Il ne coûte pas d'énergie et n'a pas de recharge. Il ne
réduit pas les dégâts persistants déjà actifs et ne raccourcit pas une
altération. Avec Égide occulte, les deux facteurs de dégâts restants
se multiplient ; leurs pourcentages ne s'additionnent pas jusqu'à 100 %.
Vitalité continue d'augmenter la vie maximum indépendamment de ce passif.

## Vérifications nécessaires avant intégration

Les essais chiffrés isoleront les niveaux de compétence et les attributs,
puis testeront leurs combinaisons avec les implants et les objets. Un bonus
d'équipement doit modifier le niveau effectif et l'affichage cyan sans
changer les points appris. Les coûts réels, refus, durées, changements de
maximum et formes devront respecter ces mêmes règles en partie.

Les distances utilisent la grille et les règles de déplacement retenues
par le moteur ; les tirs et sorts exigent les lignes de vue annoncées.
Les observations, aperçus et messages respectent le terrain et les acteurs
connus. Les nouveaux rendus devront suivre la revue à taille native de la
[direction artistique](DIRECTION_ARTISTIQUE_TILES.md). Ce document ne
crée pas de sprite, de nouvelle espèce hostile ou de modèle de loot.
