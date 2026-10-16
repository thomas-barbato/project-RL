# Propositions à relire — aucune intégration validée

Les textes ci-dessous sont des propositions. Les modifications de cette passe dans l'aide F1 ont été retirées. Les numéros 01 à 42 restent ceux du document précédent ; 04 à 06 sont remplacés ici, et les entrées manquantes commencent à 43. Les autres textes restent consultables dans [la version précédente](../RELECTURE_STATISTIQUES.md).

Les nombres des exemples correspondent au contenu standard actuel ; une future intégration devra reprendre les valeurs de la partie. Il ne s'agit pas de nouveaux réglages d'équilibrage. Les entrées 62 et 63 signalent des mécaniques prévues mais non appliquées : elles sont à discuter séparément des descriptions de statistiques actives.

### 04. Bande passante (B)

Limite les drones et les effets que vous pouvez maintenir en même temps. La jauge indique la part **occupée** : **2 / 4** signifie que 2 B sont utilisés et qu'il reste 2 B libres. Une nouvelle action demandant 3 B est alors impossible.

**Drone spectral** occupe 1 B tant que le drone existe. Ce point redevient disponible lorsque le drone est détruit ou disparaît à court de batterie. **Brouillage** occupe également 1 B pendant son maintien ; il le libère quand l'effet se termine ou que vous ne pouvez plus payer son entretien en énergie.

Une jauge pleine empêche les nouvelles réservations, mais n'arrête pas les effets déjà actifs. Attendre ne libère pas une place encore utilisée. Certaines actions, comme l'implantation d'un programme hostile, ne réservent leur bande passante que pendant la tentative.

### 05. Chaleur (H) et dissipation

La chaleur s'accumule lorsque vous utilisez une action qui en produit. Un coût de **5 H** ajoute 5 points à votre jauge. La **dissipation** est la quantité de chaleur évacuée à la fin de chaque tour.

Par exemple, **Surcharge** produit 15 H. En partant de zéro, avec une dissipation de 4, il reste **11 H à la fin du tour**, si aucun autre effet ne produit de chaleur.

Pour refroidir, laissez passer des tours en produisant moins de chaleur que vous n'en dissipez. Attendre fait aussi agir les ennemis ; laisser un menu ouvert ne refroidit rien. Une chaleur élevée peut user un module surcadencé ou empêcher son utilisation, selon les limites décrites ci-dessous.

### 06. Alerte thermique et seuil critique

La jauge devient **ambre à 40 H**, puis **rouge à 80 H**. Dans la version actuelle, ces seuils servent d'avertissement : ils ne provoquent à eux seuls ni perte de PV, ni ralentissement, ni blocage des actions. La chaleur peut continuer à monter au-delà de 80.

Avec **Surcadencement**, chaque utilisation du module ajoute **8 H**. Si cette utilisation fait dépasser **100 H**, le module perd **2 points de durabilité**. Si l'attaque ferait dépasser **140 H**, elle est refusée. Ces vérifications ont lieu **avant le refroidissement** : la dissipation de fin de tour ne permet donc pas d'éviter l'usure de cette utilisation.

Le programme hostile **Surchauffe** fonctionne autrement : il chauffe le système électronique de sa cible et peut lui infliger des dégâts thermiques. Cette chaleur appartient à la cible ; ce n'est pas la jauge personnelle décrite ici.

## Statistiques secondaires manquantes

### 43. Points de vie maximaux et Vitalité

Le second nombre de la jauge PV indique votre maximum : à **18 / 30**, il vous reste 18 PV et vous pouvez en récupérer 12. Ce maximum dépend du corps, de la Résilience, des niveaux et des bonus applicables.

Le bonus **Vitalité** d'un équipement augmente ce maximum tant que l'objet est équipé, sans vous soigner. Avec +10 PV maximaux, **18 / 30 devient 18 / 40**.

### 44. Détection

Améliore votre capacité à repérer les présences dissimulées et les indices discrets à portée de vos capteurs. N’augmente pas leur portée et ne permet pas de voir à travers les murs. Repérer une présence ne révèle pas automatiquement son identité ou ses capacités.

La Perception contribue à ce score. Par exemple, une recherche de dispositif caché compare votre Détection à sa difficulté de dissimulation. Un bonus de Détection aide à réussir cette recherche ; il n'ajoute pas de cases à sa portée.

### 45. Analyse

Facilite l’interprétation de vos observations pour identifier l’état, le fonctionnement ou les faiblesses d’une cible. Les informations obtenues dépendent de vos capteurs et des données disponibles. N’augmente pas directement les dégâts de vos attaques.

La Perception et le Traitement contribuent à ce score. Par exemple, un diagnostic énergétique utilise l'Analyse pour examiner les réserves et le fonctionnement accessibles d'une machine. Voir la machine et comprendre son fonctionnement sont deux choses différentes.

### 46. Efficacité d'intrusion

Améliore vos chances de franchir les défenses logicielles d’une cible lors d’une intrusion ou de l’implantation d’un programme hostile. Nécessite une interface compatible et une liaison valide. N’augmente pas directement les dégâts de vos attaques.

Le Traitement augmente votre score d'intrusion. Il est opposé à la Défense numérique de la cible, avec les bonus de la tentative. Un score de 65 n'est donc pas une chance de réussite de 65 %. Une tentative autorisée peut échouer et consommer son coût.

### 47. Discrétion optique et dissimulation

Rend votre silhouette plus difficile à repérer lorsqu'un effet de dissimulation est actif. La Coordination, les couverts et les bonus de camouflage augmentent la difficulté opposée à la Détection de l'observateur. Si sa Détection atteint cette difficulté, il vous repère.

Cela ne supprime pas vos bruits et ne vous cache pas automatiquement quand vous marchez à découvert.

## Capacités, matériel et signaux à détailler

### 48. Capacité énergétique

Quantité maximale d'énergie que votre réserve peut contenir. Un bonus de +10 augmente ce plafond de 10, sans ajouter 10 E à la réserve actuelle : **6 / 20 devient 6 / 30**. Retirer un bonus peut abaisser le plafond et faire perdre l'énergie qui le dépasse.

### 49. Régénération d'énergie

Quantité d'énergie rendue à chaque tour terminé, sans dépasser la capacité : **1 E par tour dans les nouvelles parties actuelles**. Une valeur de zéro signifie qu'attendre ne rend aucune énergie.

Un effet maintenu peut dépenser de l'énergie pendant le même tour. Si son entretien dépasse la régénération, votre réserve continue de baisser.

### 50. Capacité de bande passante

Nombre total de B que vous pouvez réserver simultanément. La part libre est égale à cette capacité moins la part occupée. Augmenter le Traitement ne donne pas de B supplémentaires.

La limite propre à une technique reste applicable : avoir de la bande passante libre ne permet pas de dépasser son nombre maximal de drones ou de dispositifs actifs.

### 51. Dissipation thermique

Quantité de chaleur retirée à la fin de chaque tour, sans descendre sous zéro. Avec une dissipation de 4, un bonus d'équipement de +2 fait retirer **6 H par tour**.

Ce bonus accélère le refroidissement futur ; il ne retire pas de chaleur au moment où vous équipez l'objet. La Résistance thermique réduit des dégâts reçus et ne refroidit pas cette jauge.

### 52. Pénétration de résistance

Réduit la résistance du type indiqué pour cet impact, en points de pourcentage. Par exemple, **10 points de pénétration thermique** font passer une résistance thermique de **30 % à 20 %** lors du calcul des dégâts.

Ce bonus ne modifie pas la pénétration d'armure et n'affaiblit pas durablement la cible.

### 53. Masse portée et réduction de masse

Poids total des objets transportés, y compris les objets équipés et tous les exemplaires des piles. Il s'ajoute à la masse du corps pour les poussées et les extractions.

Une réduction de masse sur un objet ne réduit que le poids de cet objet : **−20 % transforme 5 kg en 4 kg**. Elle ne libère pas de place d'inventaire.

### 54. Ancrage

Résistance supplémentaire aux poussées, en plus du poids du corps et de sa charge. Un adversaire mieux ancré est plus difficile à repousser.

La force de la poussée doit aussi respecter les limites de l'arme ou du dispositif ; réussir à infliger des dégâts ne garantit pas de déplacer la cible.

### 55. Capacité de traction

Poids total maximal que votre dispositif peut déplacer lors d'une extraction d'allié. Le corps de l'allié et tout ce qu'il porte comptent ensemble. Si cette masse dépasse la capacité, l'extraction est refusée.

Une capacité suffisante ne permet pas de traverser un mur ou d'arriver sur une case occupée.

### 56. Limite d'effets ou d'unités actifs

Nombre maximal de manifestations d'une même technique que vous pouvez maintenir à la fois. Avec une limite de 1, un second déploiement est refusé tant que le premier existe, même s'il reste de l'énergie et de la bande passante.

Ce nombre n'est pas le nombre de techniques que vous pouvez apprendre.

### 57. Portée et puissance de liaison

La portée est la distance maximale à laquelle un drone peut recevoir vos ordres. À l'intérieur de cette portée, le signal doit encore être assez puissant : la distance, les obstacles et le brouillage l'affaiblissent.

Une liaison coupée peut empêcher un nouvel ordre sans détruire le drone ni libérer sa bande passante. Les capteurs du drone ont leur propre portée.

### 58. Batterie du drone

Réserve d'énergie propre au drone. Ses déplacements et les fonctions qui la sollicitent la consomment ; la régénération du joueur ne la recharge pas. Une commande peut aussi coûter de l'énergie au joueur : sa fiche distingue les deux coûts.

Dans les nouvelles parties actuelles, un drone manifesté dont la batterie atteint zéro disparaît et rend sa bande passante.

### 59. Capacité de manipulation du drone

Poids maximal que le manipulateur du drone peut prendre en charge. Une collecte vérifie le poids de la pile visée ; une pile trop lourde est refusée. L'action exige également l'énergie nécessaire et le drone doit pouvoir accueillir la collecte.

### 60. Intensité sonore

Force d'un bruit ou d'un leurre sonore. Le signal s'affaiblit en parcourant les cases et davantage en traversant des obstacles. Une intensité plus élevée peut donc être entendue de plus loin ; ce nombre n'est pas directement une portée en cases.

Entendre un bruit ne donne pas la vision de sa source.

### 61. Unité de temps (UT)

Durée d'une action dans le monde. Une action de 2 UT laisse avancer le monde davantage qu'une action de 1 UT : les adversaires, les effets et les délais peuvent évoluer pendant ce temps.

P1 désigne une préparation de 1 UT ; A1 une action de 1 UT ; R1 une récupération de 1 UT. Lire un menu ne consomme aucune UT.

## Mécaniques prévues mais non appliquées — formulation à discuter

Ces deux entrées ne doivent pas promettre au joueur des effets absents. La description actuelle de Puissance les mentionne déjà : elle devra faire l'objet d'une correction soumise à relecture si ces mécaniques restent absentes.

### 62. Charge utile

La Charge utile doit indiquer le poids transportable sans pénalité. Cette limite n'est pas encore appliquée dans la version actuelle : porter trop lourd ne réduit pas votre esquive et ne ralentit ni ne bloque vos déplacements.

L'inventaire est limité par ses places disponibles. Le poids transporté intervient déjà dans les poussées et les extractions.

### 63. Maîtrise du recul

Désigne la capacité prévue à contrôler le recul d'une arme. Aucun score distinct ni bonus de recul lié à la Puissance n'est actuellement appliqué aux tirs.

Pour améliorer vos chances de toucher aujourd'hui, consultez la Précision, les attributs qui y contribuent et les bonus de vos techniques.
