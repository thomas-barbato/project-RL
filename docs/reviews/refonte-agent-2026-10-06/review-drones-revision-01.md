# Revue des compétences de drones

6 octobre 2026. Le joueur demande une compétence d'invocation améliorable
pour chaque drone déjà dessiné, et trois passifs communs : durée d'activité,
résistance aux dégâts en pourcentage et dégâts en pourcentage.

Cette structure est confirmée. Les noms, textes, rôles, valeurs et limites
ci-dessous sont des propositions à valider avant intégration. Les dessins
existants sont conservés à 32 × 32 ; leur acceptation visuelle ne valide
pas ces nouveaux comportements. Aucun pouvoir de cette revue n'est encore
actif dans le jeu.

## Les cinq modèles existants

| Dessin | Nom dans la banque | Proposition de rôle | Compétence |
| --- | --- | --- | --- |
| 56 | Éclaireur | Reconnaissance | D56 |
| 57 | Mule | Transport | D57 |
| 58 | Traqueur | Attaque au contact | D58 |
| 59 | Artilleur | Attaque à distance ; départ proposé de l'Opérateur | D59 / O01 |
| 66 | Gardien | Protection et attaque au contact | D66 |

Le drone Traqueur 58 est distinct de la classe Traqueur. Le dessin 60 a
été remplacé par le Gardien 66 ; il ne crée pas une sixième invocation.
Les noms de la banque restent provisoires tant que les textes de jeu
ne sont pas approuvés.

## Progression commune

Chaque modèle possède un niveau appris indépendant, de 1 à 10. Apprendre
le niveau 1 ou acheter chaque niveau suivant coûte un point de compétence.
Le premier drone d'attaque de l'Opérateur est déjà appris au niveau 1 :
neuf points supplémentaires permettent d'atteindre 10.

Les objets peuvent augmenter le niveau effectif au-delà de 10. Ces bonus
ne font pas apprendre une invocation inconnue et ne changent pas les
points dépensés. Le niveau affiché prend la couleur prévue pour les
compétences renforcées. Les cinq invocations et les trois passifs sont
accessibles à toutes les classes.

Dans les fiches, n est le niveau effectif et r = n − 1. Les tableaux
isolent les compétences avant attributs, implants et passifs. Intelligence
multiplie les PV et les dégâts des drones par 1 + 0,02 × (Intelligence − 5),
selon le barème proposé du [premier lot](REVUE_TEXTES_EFFETS_AGENT.md).
Pour la Mule, le même facteur augmente aussi la capacité de transport
proposée. Les arrondis inférieurs s'appliquent à la fin du calcul.

La puissance monte avec chaque rang : PV, dégâts pour les drones armés,
capacité pour la Mule. L'Éclaireur augmente ses PV à chaque rang et sa
vision par paliers, avec une limite de lecture de la carte. Une limite
de vision ne plafonne pas ses PV ni sa durée.

## Fonctionnement proposé à valider

- Invocation : une action, énergie commune, recharge de 6 tours propre
  à chaque compétence. Apparition sur une case adjacente libre et visible.
- Limite : deux drones actifs au total, au plus un exemplaire par modèle.
  Les rangs et Intelligence ne créent pas de drones supplémentaires.
- Les drones suivent l'agent et respectent murs, portes et cases occupées.
  Une action de drone par tour de référence ; aucun menu ni pouvoir
  instantané ne leur donne une action supplémentaire.
- Ils cessent d'être actifs à expiration de leur durée ou à zéro PV.
  Durée d'activité, PV maximum et résistance aux dégâts sont trois valeurs
  distinctes. Prolonger la durée ne soigne pas et n'empêche pas une destruction.
- Les rangs de l'invocation et de DP01 fixent les valeurs de base et la
  durée au déploiement. Intelligence, l'implant et les passifs de dégâts
  ou de résistance actuels s'appliquent ensuite. Changer le maximum de PV
  ne soigne pas gratuitement ; les PV courants sont limités au nouveau maximum.
- Les ordres visent des hostiles actuellement observés. Les drones armés
  appliquent aussi leur propre ligne de vue. Sans ordre, ils privilégient
  un hostile observé à portée, puis le plus proche, avec un ordre stable
  pour départager. Ils ne révèlent pas de cible cachée par leur ciblage.
- Un refus avant lancement ne consomme ni énergie, ni tour, ni recharge.
  La disparition d'un drone ne donne ni objet fabriqué ni expérience.
  Le suivi lors d'un changement de carte doit être défini avant intégration,
  avec restitution du vrai chargement de la Mule.

Ces limites et cette gestion du temps sont des propositions, pas des
règles déjà approuvées ou testées en partie.

### D56 Invocation de l'Éclaireur

> Déployez un drone qui explore les environs et vous transmet ce qu'il voit.

**Proposition :** drone sans attaque, spécialisé dans la reconnaissance.
PV = 16 + 3r ; vision = min(8, 4 + floor(r / 3)) cases ;
durée de base = 8 + floor(r / 3) tours ; énergie = 10 + floor(r / 3).

| Niveau effectif | PV de base | Vision | Durée de base | Énergie |
| --- | --- | --- | --- | --- |
| 1 | 16 | 4 cases | 8 tours | 10 |
| 10 | 43 | 7 cases | 11 tours | 13 |
| 12 | 49 | 7 cases | 11 tours | 13 |

L'observation vient réellement de sa position et utilise les obstacles
et le champ de vision du moteur. Elle peut faire découvrir des cases et
acteurs visibles depuis le drone ; elle ne traverse pas les murs. Les
informations perdues de vue ne sont plus actualisées. Ce partage de vision
et ses commandes devront être présentés avant intégration.

Intelligence augmente les PV. Les passifs DP01 et DP02 augmentent durée
et résistance. O03 ne crée pas d'attaque pour ce modèle.

### D57 Invocation de la Mule

> Déployez un drone qui transporte une partie de votre matériel.

**Proposition :** drone de transport sans attaque. PV = 28 + 4r ;
capacité = 40 + 8r unités de charge ; durée de base = 10 + floor(r / 3)
tours ; énergie = 10 + floor(r / 3). L'unité de charge et son raccord
à l'inventaire actuel devront être validés avec l'interface de transfert.

| Niveau effectif | PV de base | Capacité de base | Durée de base | Énergie |
| --- | --- | --- | --- | --- |
| 1 | 28 | 40 | 10 tours | 10 |
| 10 | 64 | 112 | 13 tours | 13 |
| 12 | 72 | 128 | 13 tours | 13 |

Le transfert de vrais objets se fait à proximité, avec le coût d'action
annoncé par l'interface. La Mule ne ramasse pas de butin caché et ne
duplique pas des objets. À expiration ou destruction, son vrai chargement
est déposé au sol. Un changement de carte ne doit pas détruire ce chargement.

Intelligence augmente PV et capacité. DP01 et DP02 concernent durée et
résistance. O03 ne donne pas d'attaque à la Mule.

### D58 Invocation du drone Traqueur

> Déployez un drone qui poursuit vos ennemis et les attaque au contact.

**Proposition :** drone d'attaque au contact. PV = 22 + 4r ; dégâts
cinétiques de base = 5 + 2r ; durée de base = 8 + floor(r / 3) tours ;
énergie = 12 + floor(r / 3). Attaque d'une cible adjacente, avec jet de
précision de référence 100 et défenses ordinaires. Se déplacer jusqu'à
une cible et l'attaquer sont deux actions distinctes.

| Niveau effectif | PV de base | Dégâts de base | Durée de base | Énergie |
| --- | --- | --- | --- | --- |
| 1 | 22 | 5 | 8 tours | 12 |
| 10 | 58 | 23 | 11 tours | 15 |
| 12 | 66 | 27 | 11 tours | 15 |

Intelligence augmente PV et dégâts. O03 augmente ses dégâts ; DP01 et
DP02 augmentent durée et résistance. Les passifs de dégâts personnels
du Traqueur et du Métamorphe ne renforcent pas ses attaques.

### D59 Invocation de l'Artilleur

> Déployez un drone d'attaque qui combat à vos côtés.

**Départ confirmé :** l'Opérateur reçoit une invocation de drone d'attaque
au niveau appris 1. **Affectation proposée :** le dessin Artilleur 59.
Cette fiche et O01 du [deuxième lot](REVUE_COMPETENCES_AGENT_LOT_02.md)
décrivent une seule compétence ; il n'y a pas deux invocations à acheter.

**Paramètres proposés :** PV = 20 + 4r ; dégâts cinétiques de base = 4 + r ;
durée de base = 8 + floor(r / 3) tours ; énergie = 12 + floor(r / 3).
Tir à 4 cases au plus, avec obstacles, ligne de vue et jet de précision
de référence 100. Il utilise sa réserve intégrée sans retirer les munitions
du joueur ; il ne produit pas d'objets à sa disparition.

| Niveau effectif | PV de base | Dégâts de base | Durée de base | Énergie |
| --- | --- | --- | --- | --- |
| 1 | 20 | 4 | 8 tours | 12 |
| 10 | 56 | 13 | 11 tours | 15 |
| 12 | 64 | 15 | 11 tours | 15 |

Intelligence augmente PV et dégâts. O03 augmente les dégâts ; DP01 et
DP02 augmentent durée et résistance. Au départ proposé de l'Opérateur,
Intelligence 8 et O03 niveau 1 font passer ses dégâts de 4 à 5 avant
défenses. Les tirs et l'affectation au dessin restent à présenter en situation.

### D66 Invocation du Gardien

> Déployez un drone qui vous protège et frappe les ennemis proches.

**Proposition :** drone de protection avec attaque au contact. PV = 30 + 5r ;
dégâts cinétiques de base = 3 + r ; durée de base = 8 + floor(r / 3) tours ;
énergie = 14 + floor(r / 3). Attaque adjacente avec précision de référence 100.
Protection du joueur et de ses drones observés à 2 cases au plus, avec
ligne de vue ; les murs arrêtent la protection.

Réduction des dégâts par cette protection, en pourcentage =
5 + 25r / (n + 9). Elle continue au-delà de 10 et reste inférieure à 30 %.
Deux Gardiens ne cumulent pas deux protections identiques.

| Niveau effectif | PV de base | Dégâts de base | Protection | Durée | Énergie |
| --- | --- | --- | --- | --- | --- |
| 1 | 30 | 3 | 5 % | 8 tours | 14 |
| 10 | 75 | 12 | 16,8 % | 11 tours | 17 |
| 12 | 85 | 14 | 18,1 % | 11 tours | 17 |

Intelligence augmente PV et dégâts ; le rang de l'invocation augmente la
protection. O03 renforce les attaques et DP01/DP02 durée et résistance.
La protection se multiplie avec les autres réductions de dégâts restants.
Le dessin 66 reste sans bouclier porté ; l'apparence de cette protection
devra être proposée séparément à taille native.

### DP01 Autonomie des drones

> Vos drones restent actifs plus longtemps.

**Structure confirmée :** un passif commun de durée de vie des drones.
**Interprétation proposée :** il prolonge leur durée d'activité en tours.
Il ne change pas les PV ni la résistance aux dégâts ; les deux autres
passifs traitent les dégâts et la résistance.

**Valeur proposée :** n + 1 tours supplémentaires, soit +2 au niveau 1,
+11 au niveau 10 et +13 au niveau 12. Le niveau effectif est lu au
déploiement. Modifier un bonus d'équipement ensuite ne prolonge pas un
drone déjà actif. Exemple Artilleur avec invocation et DP01 au même niveau :
10, 22 et 24 tours aux niveaux 1, 10 et 12, avant autres modificateurs.

Passif permanent, tous modèles, sans coût ni recharge. La progression
continue au-delà de 10 grâce aux objets ; une durée longue n'empêche pas
la destruction à zéro PV.

### DP02 Blindage des drones

> Vos drones subissent moins de dégâts.

**Structure confirmée :** un passif commun de résistance des drones en
pourcentage. **Valeur proposée :** réduction des dégâts =
10 + 40 × (n − 1) / (n + 9) pour cent. Elle vaut 10 % au niveau 1,
28,9 % au niveau 10 et 31,0 % au niveau 12, et reste inférieure à 50 %.

Cette réduction s'applique aux dégâts directs et persistants reçus par
tous les drones contrôlés, après les protections ordinaires. Les facteurs
de dégâts restants se multiplient avec la protection du Gardien ; les
pourcentages ne s'ajoutent pas jusqu'à une immunité. Elle ne donne pas
de PV, ne soigne pas, ne bloque pas une altération et ne prolonge pas
la durée d'activité.

Passif permanent, sans coût ni recharge. Son niveau effectif actuel est
lu au moment des dégâts ; un objet peut le renforcer au-delà de 10.

## Troisième passif commun : O03 Lien renforcé

> Vos drones infligent plus de dégâts.

L'effet initial **+25 % de dégâts de tous les drones** est confirmé.
La progression proposée reste celle d'O03 : 25 + 2 × (n − 1) pour cent,
soit +43 % au niveau 10 et +47 % au niveau 12. Il renforce les attaques
de mêlée et à distance des drones armés, sans donner d'attaque aux modèles
de reconnaissance ou de transport. Les détails sont dans le deuxième lot.
Ce renvoi ne crée pas un quatrième passif ni un second achat d'O03.

## Revue avant intégration

Les cinq invocations, les deux nouveaux passifs, leurs textes et leurs
valeurs attendent validation. Il faudra ensuite vérifier en partie le
temps, les refus sans dépense, les observations, le chargement de la Mule,
les PV sans soin gratuit, les bonus d'objets et les transitions de carte.
Les calculs de revue ne remplacent pas ces essais ni l'équilibrage.

Les nouveaux effets et commandes seront présentés en situation avant
leur intégration, selon la [direction artistique](DIRECTION_ARTISTIQUE_TILES.md).
Les images existantes ne sont pas redessinées par cette revue.
