# Descriptions et chiffres des compétences de drones

6 octobre 2026. Les cinq invocations et trois passifs communs reprennent
les noms et rôles du [catalogue retenu](PROPOSITION_COMPETENCES_PRINCIPALES_AGENT.md).
Le joueur choisit explicitement cinq drones actifs simultanément, avec
un seul exemplaire de chaque modèle. Les nouveaux textes et chiffres
ci-dessous sont proposés pour une validation distincte, avant intégration.

Les modèles 56, 57, 58, 59 et 66 restent ceux déjà dessinés. D59 et O01
désignent une seule compétence, et O03 un seul passif de dégâts. Le
dessin 60 ne crée pas de modèle supplémentaire.

## Référence et exemple actuel

La référence utilise chaque compétence au rang effectif 1, Intelligence 5,
sans les autres passifs, implants ou bonus. La colonne actuelle illustre
dix rangs appris et deux rangs d'équipement pour chaque compétence du lot,
soit un rang effectif de 12, avec Intelligence 10 et les trois passifs au
rang 12. Elle ne décrit pas une sauvegarde du joueur.

Le barème proposé d'Intelligence ajoute 2 % par point autour de 5 aux
PV et dégâts des drones, ainsi qu'à la capacité de la Mule. Intelligence 10
donne donc +10 %. Les dégâts actuels combinent ce facteur et les +47 %
de Lien renforcé, soit un multiplicateur de 1,617. Les durées incluent
les treize tours ajoutés par Autonomie. Aucun autre modificateur n'est présent.

Les PV et dégâts sont arrondis à l'entier inférieur à la fin du calcul.
Les dégâts indiqués précèdent les défenses et ne garantissent pas une touche.
Les réductions affichées sont arrondies au dixième, avec leur précision
complète conservée pour le calcul. La capacité de transport est proposée
en kilogrammes, avec stockage en grammes entiers comme les masses existantes.
Le rang renforcé sera distingué en cyan selon l'accord précédent.

## Fonctionnement commun

**Limite acceptée :** cinq drones actifs au maximum, un exemplaire par modèle.
Les cinq modèles peuvent donc être déployés ensemble. Aucun point de compétence
supplémentaire n'est exigé pour ouvrir des emplacements de drones.

**Fonctionnement proposé :** chaque invocation dépense uniquement l'énergie
commune, prend une action ordinaire et possède une recharge de six tours.
Le drone apparaît sur une case adjacente libre, franchissable et visible.
Une invocation déjà présente ou un emplacement illégal entraîne un refus
sans dépense, tour ni recharge. Aucun objet d'inventaire n'est consommé.

Chaque drone agit une fois par tour de référence, selon son rôle ; le
déplacement et l'attaque au contact sont deux actions distinctes. Les menus
et les pouvoirs instantanés ne lui accordent pas d'action supplémentaire.
Les obstacles et la visibilité limitent ses mouvements, attaques et protections.

Les rangs d'invocation et les bonus de durée sont lus au déploiement.
Les modificateurs actuels d'Intelligence, de résistance et de dégâts s'appliquent
ensuite. Un changement de maximum de PV conserve les PV disponibles, plafonnés
si nécessaire, sans soin gratuit. Un drone disparaît à zéro PV ou à expiration.

Le joueur peut désigner une cible hostile observée. Les drones armés doivent
aussi pouvoir la voir depuis leur propre position. Sans ordre, ils choisissent
un hostile observé à portée, puis le plus proche selon un départage stable.
La reconnaissance de l'Éclaireur transmet ses observations réelles ; un autre
drone n'acquiert pas implicitement ce partage de vision.

Le rappel et les transitions de carte restent à définir avec les commandes.
Le chargement réel de la Mule devra toujours être conservé, restitué ou déposé
sur une case légale ; une disparition ne fabrique ni objet ni récompense.

### D56 Invocation de l'Éclaireur

> Votre éclaireur balaie les environs et vous transmet ce qu'il voit.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV maximum | 16 | 53 |
| Vision depuis le drone | 4 cases | 7 cases |
| Durée d'activité | 8 tours | 24 tours |
| Coût d'énergie | 10 | 13 |
| Recharge | 6 tours | 6 tours |
| Attaque | Aucune | Aucune |

**Progression proposée :** PV = 16 + 3 × (n − 1) ; vision =
min(8, 4 + floor((n − 1) / 3)) ; durée avant Autonomie =
8 + floor((n − 1) / 3) ; énergie = 10 + floor((n − 1) / 3).
Intelligence renforce les PV. La vision plafonne à huit cases ; les PV
et la durée continuent au-delà de ce plafond.

Les observations partent de sa position et respectent les murs et le champ
de vision. Le drone peut révéler ce qu'il voit réellement, puis cesse de
mettre à jour ce qu'il perd de vue. Lien renforcé ne lui donne pas d'attaque.

**Intention visuelle retenue :** modèle 56 et balayage de capteur.
Le nouveau rendu demeure à présenter.

### D57 Invocation de la Mule

> Votre mule mécanique porte le matériel qui alourdit vos pas.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV maximum | 28 | 79 |
| Charge transportable | 40 kg | 140,8 kg |
| Durée d'activité | 10 tours | 26 tours |
| Coût d'énergie | 10 | 13 |
| Recharge | 6 tours | 6 tours |
| Attaque | Aucune | Aucune |

**Progression proposée :** PV = 28 + 4 × (n − 1) ; capacité =
40 + 8 × (n − 1) kg avant Intelligence ; durée avant Autonomie =
10 + floor((n − 1) / 3) ; énergie = 10 + floor((n − 1) / 3).
La capacité conserve la formule numérique de la première revue, avec
le kilogramme proposé comme unité explicite. Intelligence multiplie
les PV et la capacité ; le calcul de capacité se fait en grammes entiers.

Le transport repose sur de vrais objets transférés à proximité, avec un
coût de transfert à fixer. Taille, emplacements, masses absentes des anciens
objets et éventuelle surcharge devront être définis avec l'inventaire.
À destruction ou expiration, le chargement est déposé au sol sur une case
légale. Il ne doit être ni dupliqué ni perdu lors d'une transition.
Lien renforcé ne donne pas d'attaque à la Mule.

**Intention visuelle retenue :** modèle 57 et indication de chargement.
Le nouveau rendu demeure à présenter.

### D58 Invocation du drone Traqueur

> Votre traqueur mécanique poursuit sa proie et la frappe au contact.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV maximum | 22 | 72 |
| Dégâts par attaque, avant défenses | 5 | 43 |
| Portée d'attaque | 1 case | 1 case |
| Durée d'activité | 8 tours | 24 tours |
| Coût d'énergie | 12 | 15 |
| Recharge | 6 tours | 6 tours |

**Progression proposée :** PV = 22 + 4 × (n − 1) ; dégâts cinétiques
de base = 5 + 2 × (n − 1) ; durée avant Autonomie =
8 + floor((n − 1) / 3) ; énergie = 12 + floor((n − 1) / 3).
Intelligence renforce PV et dégâts ; Lien renforcé multiplie les dégâts.

Il attaque une cible adjacente avec la précision de référence 100 et les
défenses ordinaires. Rejoindre une cible consomme son déplacement avant
de pouvoir l'attaquer lors d'une action suivante. Les bonus de dégâts
personnels du Traqueur ou du Métamorphe ne renforcent pas le drone.

**Intention visuelle retenue :** modèle 58 et griffures basses.
Le nouveau rendu demeure à présenter.

### D59 Invocation de l'Artilleur

> Votre artilleur verrouille une cible et ouvre le feu à vos côtés.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV maximum | 20 | 70 |
| Dégâts par tir, avant défenses | 4 | 24 |
| Portée de tir | 4 cases | 4 cases |
| Durée d'activité | 8 tours | 24 tours |
| Coût d'énergie | 12 | 15 |
| Recharge | 6 tours | 6 tours |

**Progression proposée :** PV = 20 + 4 × (n − 1) ; dégâts cinétiques
de base = 4 + (n − 1) ; durée avant Autonomie =
8 + floor((n − 1) / 3) ; énergie = 12 + floor((n − 1) / 3).
Intelligence renforce PV et dégâts ; Lien renforcé multiplie les dégâts.

Le tir utilise la ligne de vue du drone et une précision de référence 100.
Sa réserve intégrée évite de consommer les munitions du joueur. D59 et O01
restent le même achat. L'Artilleur demeure l'affectation proposée de
l'invocation d'attaque initiale de l'Opérateur ; elle ne crée pas une
seconde compétence ni un cadeau supplémentaire.

**Intention visuelle retenue :** modèle 59 et flashes alternés des armes.
Le nouveau rendu demeure à présenter.

### D66 Invocation du Gardien

> Votre gardien forme un rempart mobile et frappe les ennemis qui s'approchent.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV maximum | 30 | 93 |
| Dégâts au contact, avant défenses | 3 | 22 |
| Protection des alliés proches | 5 % de réduction | 18,1 % de réduction |
| Rayon de protection | 2 cases | 2 cases |
| Durée d'activité | 8 tours | 24 tours |
| Coût d'énergie | 14 | 17 |
| Recharge | 6 tours | 6 tours |

**Progression proposée :** PV = 30 + 5 × (n − 1) ; dégâts cinétiques
de base = 3 + (n − 1) ; protection = 5 + 25 × (n − 1) / (n + 9)
pour cent ; durée avant Autonomie = 8 + floor((n − 1) / 3) ;
énergie = 14 + floor((n − 1) / 3). La protection reste strictement
inférieure à 30 % et progresse au-delà de 10.

Le Gardien attaque au contact avec une précision de référence 100.
Sa protection réduit les dégâts directs et persistants du joueur et
de ses drones proches, y compris lui-même, avec ligne de vue et obstacles.
Elle cesse de couvrir un bénéficiaire hors de portée ou hors de vue.
Elle multiplie les dégâts restants après les autres protections ; elle
ne s'additionne pas jusqu'à produire une immunité.

**Intention visuelle retenue :** modèle 66 et segments protecteurs.
Le dessin conserve son équipement existant ; le nouveau rendu demeure à présenter.

### DP01 Autonomie des drones

> Vos réserves d'alimentation maintiennent les drones en activité plus longtemps.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Durée supplémentaire pour chaque drone | +2 tours | +13 tours |
| Coût et recharge | Aucun | Aucun |
| Fonctionnement | Passif permanent | Passif permanent |

**Progression proposée :** durée supplémentaire = n + 1 tours.
Le rang effectif est lu au déploiement. Les rangs de l'invocation et
d'Autonomie sont indépendants : les améliorer augmente deux composantes
différentes de la durée totale. Modifier le bonus ensuite ne prolonge
pas un drone déjà actif.

Le passif concerne les cinq modèles. Il ne donne pas de PV, ne répare
pas les dégâts et n'empêche pas une destruction avant la fin du délai.

**Intention visuelle retenue :** indication de durée au déploiement.
Le nouveau rendu demeure à présenter.

### DP02 Blindage des drones

> Des renforts protègent vos drones et limitent les dégâts qu'ils subissent.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Réduction des dégâts reçus | 10 % | 31,0 % |
| Coût et recharge | Aucun | Aucun |
| Fonctionnement | Passif permanent | Passif permanent |

**Progression proposée :** réduction = 10 + 40 × (n − 1) / (n + 9)
pour cent. Elle reste strictement inférieure à 50 %, même au-delà de 10.
Le rang effectif actuel s'applique aux dégâts directs et persistants
reçus par tous les drones contrôlés, après leurs protections ordinaires.

Blindage ne soigne pas, n'ajoute pas de PV, ne prolonge pas la durée
et ne retire pas les altérations. Avec le Gardien, les deux facteurs de
dégâts restants se multiplient. Aux rangs 12 du présent exemple, leur
réduction combinée sur un drone couvert vaut environ 43,4 %.

**Intention visuelle retenue :** plaques lumineuses aux impacts.
Le nouveau rendu demeure à présenter.

### O03 Lien renforcé

> Votre liaison de combat coordonne les drones pour rendre leurs frappes plus destructrices.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Bonus de dégâts des drones armés | +25 % | +47 % |
| Coût et recharge | Aucun | Aucun |
| Fonctionnement | Passif permanent | Passif permanent |

**Effet initial déjà accepté :** +25 % de dégâts pour tous les drones armés.
**Progression proposée :** bonus = 25 + 2 × (n − 1) pour cent, soit +43 %
au rang 10 et +47 % au rang 12. Les gains continuent au-delà de 10.
Le bonus se combine avec Intelligence avant les défenses, avec un seul
arrondi final des dégâts.

Le rang du passif et Intelligence sont lus au moment de l'attaque.
Il concerne le Traqueur, l'Artilleur et le Gardien. Il ne crée pas
d'attaque pour l'Éclaireur ou la Mule, et ne renforce pas le personnage.
O03 n'est pas un quatrième passif distinct de celui du deuxième lot.

**Intention visuelle retenue :** impulsion dans les circuits à l'attaque.
Le nouveau rendu demeure à présenter.

## Revue et essais futurs

Les textes roleplay, chiffres, courbes, unités, commandes et conditions
nouvelles attendent validation. Les dessins existants restent intacts ;
les animations et autres rendus nouveaux suivront la
[direction artistique](DIRECTION_ARTISTIQUE_TILES.md) avant intégration.

Les futurs essais devront couvrir les cinq drones ensemble, la limite
par modèle, les bonus au-delà de dix, les durées et les actions réelles,
les observations, les protections combinées et le vrai chargement de la Mule.
Le [menu de test](REVUE_PERSONNAGE_TEST_AGENT.md) reste prévu après
l'implémentation de toutes les compétences et animations.
