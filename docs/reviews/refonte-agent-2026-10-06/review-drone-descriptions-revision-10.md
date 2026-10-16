# Descriptions et chiffres des compétences de drones

6 octobre 2026. Les cinq invocations et trois passifs communs reprennent
les noms et rôles du [catalogue retenu](PROPOSITION_COMPETENCES_PRINCIPALES_AGENT.md).
Le joueur choisit explicitement cinq drones actifs simultanément, avec
un seul exemplaire de chaque modèle. Les nouveaux textes et chiffres
ci-dessous sont proposés pour une validation distincte, avant intégration.
La [révision acceptée](PROPOSITION_DRONES_PERSISTANTS_AGENT.md) retire le délai
d'expiration des cinq modèles et remplace Autonomie par Maintenance des drones.

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
de Lien renforcé, soit un multiplicateur de 1,617. Maintenance au rang 12
propose une réparation de 7,5 % des PV maximum par tour éligible.
Aucun autre modificateur n'est présent ; les cinq durées sont illimitées.

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

**Règles acceptées :** chaque déploiement utilise l'énergie commune, sans
entretien par tour. Les cinq modèles n'expirent plus ; leurs PV restent
la limite de survie. Rappel et changement de couche conservent les PV et
le chargement, sans soin gratuit. La Mule détruite dépose son matériel au sol.

**Fonctionnement proposé :** chaque invocation prend une action ordinaire
et possède une recharge de six tours.
Le drone apparaît sur une case adjacente libre, franchissable et visible.
Une invocation déjà présente ou un emplacement illégal entraîne un refus
sans dépense, tour ni recharge. Aucun objet d'inventaire n'est consommé.

Chaque drone agit une fois par tour de référence, selon son rôle ; le
déplacement et l'attaque au contact sont deux actions distinctes. Les menus
et les pouvoirs instantanés ne lui accordent pas d'action supplémentaire.
Les obstacles et la visibilité limitent ses mouvements, attaques et protections.

Les rangs d'invocation sont lus au déploiement. Les modificateurs actuels
d'Intelligence, de réparation, de résistance et de dégâts s'appliquent
ensuite. Un changement de maximum de PV conserve les PV disponibles, plafonnés
si nécessaire, sans soin gratuit. Un drone est détruit à zéro PV.

Le joueur peut désigner une cible hostile observée. Les drones armés doivent
aussi pouvoir la voir depuis leur propre position. Sans ordre, ils choisissent
un hostile observé à portée, puis le plus proche selon un départage stable.
La reconnaissance de l'Éclaireur transmet ses observations réelles ; un autre
drone n'acquiert pas implicitement ce partage de vision.

Le coût et l'interface des commandes de rappel et de transfert restent à
définir. Le rappel conserve le drone et son vrai chargement pour un prochain
déploiement. Un changement de couche les conserve aussi. Une destruction
dépose le chargement sur une case légale ; elle ne fabrique ni objet ni récompense.

### D56 Invocation de l'Éclaireur

> Votre éclaireur balaie les environs et vous transmet ce qu'il voit.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV maximum | 16 | 53 |
| Vision depuis le drone | 4 cases | 7 cases |
| Durée d'activité | Illimitée | Illimitée |
| Coût d'énergie | 10 | 13 |
| Recharge | 6 tours | 6 tours |
| Attaque | Aucune | Aucune |

**Progression proposée :** PV = 16 + 3 × (n − 1) ; vision =
min(8, 4 + floor((n − 1) / 3)) ; énergie = 10 + floor((n − 1) / 3).
Intelligence renforce les PV. La vision plafonne à huit cases ; les PV
continuent au-delà de ce plafond.

Les observations partent de sa position et respectent les murs et le champ
de vision. Le drone peut révéler ce qu'il voit réellement, puis cesse de
mettre à jour ce qu'il perd de vue. Lien renforcé ne lui donne pas d'attaque.

Le joueur valide la fiche individuelle aux rangs 11 et 12, avec Intelligence
10 : 50 puis 53 PV, vision de sept cases, déploiement de 13 énergie et
recharge de six tours. Il demande ensuite que l'Éclaireur devienne le
premier drone accessible et permette une observation détaillée des cibles.
L'articulation avec l'invocation d'attaque de départ précédemment proposée
pour l'Opérateur reste à régler dans la progression et les classes.

**Ajout à la fiche :** observation du niveau, des PV, des résistances et
du loot de la cible. La cible doit être visible par le joueur **ou** par
l'Éclaireur ; une seule de ces deux visions suffit. Cet effet ne change
pas au rang suivant et n'est donc pas répété dans son aperçu.

Sans l'Éclaireur, la fenêtre d'inspection reste accessible avec le nom,
l'image, les coordonnées et un texte de présentation, ainsi qu'un
avertissement d'accès limité. Le style demandé est celui d'un terminal
informatique rétrofuturiste. Les textes exacts et le rendu de cette fenêtre
restent à présenter. Les coûts et le temps de l'observation, la disponibilité
requise du drone, le cache des informations et la définition précise du
loot consultable restent à préciser.

La [fiche révisée D56](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-D56-04.json)
présente le nouvel effet pour validation de sa rédaction.

**Description fonctionnelle demandée par le joueur :**

> Déploie un drone de reconnaissance qui partage sa vision avec vous. Il vous permet aussi d'avoir plus d'information durant l'action d'analyse de cible.

**Intention visuelle retenue :** modèle 56 et balayage de capteur.
Le nouveau rendu demeure à présenter.

### D57 Invocation de la Mule

> Votre mule mécanique porte le matériel qui alourdit vos pas.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV maximum | 28 | 79 |
| Charge transportable | 40 kg | 140,8 kg |
| Durée d'activité | Illimitée | Illimitée |
| Coût d'énergie | 10 | 13 |
| Recharge | 6 tours | 6 tours |
| Attaque | Aucune | Aucune |

**Progression proposée :** PV = 28 + 4 × (n − 1) ; capacité =
40 + 8 × (n − 1) kg avant Intelligence ; énergie = 10 + floor((n − 1) / 3).
La capacité conserve la formule numérique de la première revue, avec
le kilogramme proposé comme unité explicite. Intelligence multiplie
les PV et la capacité ; le calcul de capacité se fait en grammes entiers.

Le transport repose sur de vrais objets transférés à proximité, avec un
coût de transfert à fixer. Taille, emplacements, masses absentes des anciens
objets et éventuelle surcharge devront être définis avec l'inventaire.
À la destruction, le chargement est déposé au sol sur une case légale.
Rappel et transition conservent les objets réels, sans transfert automatique
vers l'inventaire du joueur. Ils ne doivent être ni dupliqués ni perdus.
Lien renforcé ne donne pas d'attaque à la Mule.

Le joueur valide [la fiche individuelle D57](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-D57-02.json) :
texte roleplay et fonctionnel, 74 puis 79 PV aux rangs 11 et 12, capacité
de 132 puis 140,8 kg avec Intelligence 10 conservée, coût de 13 énergie
et recharge de six tours. Une seule Mule, sans attaque, conserve son
chargement lors du rappel et des changements de couche ; il tombe au sol
à sa destruction. Les formules complètes, les règles d'inventaire non
montrées et le rendu restent à revoir.

**Intention visuelle retenue :** modèle 57 et indication de chargement.
Le nouveau rendu demeure à présenter.

### D58 Invocation du drone Traqueur

> Votre traqueur mécanique poursuit sa proie et la frappe au contact.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV maximum | 22 | 72 |
| Dégâts par attaque, avant défenses | 5 | 43 |
| Portée d'attaque | 1 case | 1 case |
| Durée d'activité | Illimitée | Illimitée |
| Coût d'énergie | 12 | 15 |
| Recharge | 6 tours | 6 tours |

**Progression proposée :** PV = 22 + 4 × (n − 1) ; dégâts cinétiques
de base = 5 + 2 × (n − 1) ; énergie = 12 + floor((n − 1) / 3).
Intelligence renforce PV et dégâts ; Lien renforcé multiplie les dégâts.

Il attaque une cible adjacente avec la précision de référence 100 et les
défenses ordinaires. Rejoindre une cible consomme son déplacement avant
de pouvoir l'attaquer lors d'une action suivante. Les bonus de dégâts
personnels du Traqueur ou du Métamorphe ne renforcent pas le drone.

Le joueur valide [la fiche individuelle D58](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-D58-02.json) :
texte roleplay et fonctionnel, 68 puis 72 PV et 40 puis 43 dégâts cinétiques
aux rangs 11 et 12, avec Intelligence 10 et Lien renforcé à +47 % conservés.
La portée de contact est d'une case, le coût de 15 énergie et la recharge
de six tours. Un seul drone Traqueur peut être déployé ; ses dégâts sont
affichés avant les défenses de la cible. Les formules complètes, les règles
non montrées et le rendu restent à revoir.

**Intention visuelle retenue :** modèle 58 et griffures basses.
Le nouveau rendu demeure à présenter.

### D59 Invocation de l'Artilleur

> Votre artilleur verrouille une cible et ouvre le feu à vos côtés.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV maximum | 20 | 70 |
| Dégâts par tir, avant défenses | 4 | 24 |
| Portée de tir | 4 cases | 4 cases |
| Durée d'activité | Illimitée | Illimitée |
| Coût d'énergie | 12 | 15 |
| Recharge | 6 tours | 6 tours |

**Progression proposée :** PV = 20 + 4 × (n − 1) ; dégâts cinétiques
de base = 4 + (n − 1) ; énergie = 12 + floor((n − 1) / 3).
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
| Durée d'activité | Illimitée | Illimitée |
| Coût d'énergie | 14 | 17 |
| Recharge | 6 tours | 6 tours |

**Progression proposée :** PV = 30 + 5 × (n − 1) ; dégâts cinétiques
de base = 3 + (n − 1) ; protection = 5 + 25 × (n − 1) / (n + 9)
pour cent ; énergie = 14 + floor((n − 1) / 3). La protection reste strictement
inférieure à 30 % et progresse au-delà de 10.

Le Gardien attaque au contact avec une précision de référence 100.
Sa protection réduit les dégâts directs et persistants du joueur et
de ses drones proches, y compris lui-même, avec ligne de vue et obstacles.
Elle cesse de couvrir un bénéficiaire hors de portée ou hors de vue.
Elle multiplie les dégâts restants après les autres protections ; elle
ne s'additionne pas jusqu'à produire une immunité.

Le joueur valide [la fiche individuelle D66](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-D66-02.json) :
texte roleplay et fonctionnel, 88 puis 93 PV, 21 puis 22 dégâts cinétiques
et 17,5 puis 18,1 % de protection aux rangs 11 et 12. La portée de contact
est d'une case, le rayon de protection de deux cases, le coût de 17 énergie
et la recharge de six tours. Un seul Gardien ; sa protection couvre le joueur
et ses drones, y compris lui-même, avec ligne de vue, pour les dégâts directs
et persistants. La ligne détaillant les rangs appris et le bonus d'équipement
est retirée du tooltip conformément à CD07. Les formules complètes, les
règles non montrées et le rendu restent à revoir.

**Intention visuelle retenue :** modèle 66 et segments protecteurs.
Le dessin conserve son équipement existant ; le nouveau rendu demeure à présenter.

### DP01 Maintenance des drones

> Vos drones réparent leurs dommages lorsque les combats leur laissent un répit.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV réparés par tour éligible | 2 % des PV maximum | 7,5 % des PV maximum |
| Accalmie nécessaire avant réparation | 3 tours complets | 3 tours complets |
| Premier tour éligible | Au quatrième tour d'accalmie | Au quatrième tour d'accalmie |
| Coût et recharge | Aucun | Aucun |
| Fonctionnement | Passif permanent | Passif permanent |

**Nom et rôle acceptés :** réparation progressive des cinq modèles hors combat.
**Progression proposée :** réparation = 2 + 0,5 × (n − 1) pour cent
des PV maximum par tour éligible, soit 6,5 % au rang 10 et 7,5 % au rang 12.
Les bonus d'équipement continuent à augmenter cette valeur au-delà de 10.
Le rang du passif et le maximum de PV actuels sont lus au moment de réparer.
Intelligence augmente donc la quantité réparée en renforçant les PV maximum,
sans second multiplicateur d'Intelligence sur le pourcentage de réparation.

**Accalmie révisée proposée :** les drones utilisent la phase commune de
[HUD03](REVUE_INTERFACE_EFFETS_AGENT.md#hud03-phase-de-combat-et-retour-au-calme),
affichée pour l'agent et les drones déployés. Un hostile actuellement repéré,
une attaque effectuée ou reçue, ou des dégâts directs ou persistants sur
l'un d'eux remettent le compteur commun à zéro. Trois tours complets sans
ces événements permettent une réparation au quatrième tour consécutif,
puis aux tours calmes suivants. Les anciennes attentes par drone sont
remplacées par cette proposition commune, dont les conditions restent à valider.

La réparation est calculée après toutes les actions et tous les dégâts du
tour. Elle restaure uniquement les PV manquants, sans résurrection ni retrait
d'altérations. Les fractions de PV s'accumulent entre réparations ; seuls
les PV entiers sont ajoutés. Atteindre le maximum efface le reliquat pour
ne pas stocker une réserve de soins. Exemple : un Artilleur de 70 PV maximum
avec Maintenance au rang 12 accumule 5,25 PV par tour éligible ; quatre tours
éligibles consécutifs rendent 5, 5, 5 et 6 PV s'il lui manque assez de vie.

Un drone rappelé ne se répare pas. Le rappel conserve la phase commune ;
le changement de couche remet son compteur à zéro. Les deux conservent
les PV et la fraction déjà accumulée. Les menus et actions instantanées
ne font pas avancer le compteur. Le coût et le moment du rappel restent
à définir avec les commandes. Le HUD indique réparation suspendue,
attente avant réparation ou réparation disponible, selon la phase réelle.

**Nouvelle intention visuelle proposée :** brève impulsion verte autour
du drone quand des PV entiers sont effectivement réparés, sans redessiner
son sprite. Cette intention et son rendu restent à valider ; l'ancien
décompte d'Autonomie est abandonné.

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

Blindage ne soigne pas, n'ajoute pas de PV et ne retire pas les altérations.
Avec le Gardien, les deux facteurs de
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

## DR01 Soin actif d'un robot — ajout demandé

Le joueur demande deux compétences distinctes : Maintenance des drones
reste le passif de réparation au calme décrit précédemment, et un nouvel
actif soigne un robot pendant le combat. Cette décision remplace la
suggestion intermédiaire d'une Maintenance en combat interrompue par
les attaques du joueur.

L'actif reçoit l'identifiant de revue stable DR01. **Nom proposé :**
Réparation ciblée. Sa fiche chiffrée, ses cibles autorisées, sa portée,
son coût, sa recharge et son rendu restent à présenter et à valider.
Son emplacement est proposé dans Drones, sans modifier silencieusement
les paliers ni les identifiants déjà retenus. Le rôle demandé ne valide
pas ces détails ni le texte définitif du nouvel actif.

La [décision de séparation](../docs/reviews/refonte-agent-2026-10-06/robot-active-heal-and-passive-maintenance-decisions.json)
conserve la demande du joueur. Les chiffres et le tooltip individuels
de DP01 restent à revoir ; aucun soin n'est intégré au jeu à ce stade.

## Revue et essais futurs

Les textes roleplay, chiffres, courbes, unités, commandes et conditions
nouvelles attendent validation. Les dessins existants restent intacts ;
les animations et autres rendus nouveaux suivront la
[direction artistique](DIRECTION_ARTISTIQUE_TILES.md) avant intégration.

Les futurs essais devront couvrir les cinq drones ensemble, la limite
par modèle, les bonus au-delà de dix, la persistance et les actions réelles,
les observations, les protections combinées et le vrai chargement de la Mule.
Pour Maintenance, vérifier l'accalmie, les fractions de PV, les dégâts
persistants, le rappel, les transitions et l'absence de soins gratuits.
Le [menu de test](REVUE_PERSONNAGE_TEST_AGENT.md) reste prévu après
l'implémentation de toutes les compétences et animations.
