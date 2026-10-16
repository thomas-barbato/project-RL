# Interactions en ville et hors ville

3 octobre 2026 — propositions à lire avant intégration.

Ce lot développe quatre interactions facultatives. Leurs effets, leurs textes
et leurs coûts restent à approuver. Aucun changement du jeu, de sa génération
ou des sauvegardes n'est réalisé par ce dossier.

## Intention

Donner au joueur des raisons d'observer les lieux et d'agir sur eux : préparer
une expédition, comprendre son équipement, choisir une traversée et décider
où utiliser une ressource rare. Ces interactions accompagnent la fuite de la
simulation ; elles ne demandent pas de terminer une série de quêtes.

Les commerces, soins, améliorations, caches sous alarme, consoles de portes et
récupérations sur les carcasses existent déjà. Le lot s'appuie sur ces systèmes
et apporte des situations complémentaires. Les habitants et les noms établis
restent les références ; les intitulés suivants décrivent des fonctions.

## A — Renseignements locaux, en ville

**Situation.** Avant de partir, le joueur demande à un habitant ou consulte un
terminal ce qu'on sait des environs. Il choisit une destination locale, puis
obtient un indice durable dans son dossier de terrain.

**Intérêt.** Décider quel lieu explorer avec les ressources et l'équipement
actuels. L'information décrit un passage et un danger concret plutôt qu'une
destination de quête obligatoire.

Exemple de texte, à adapter aux faits réellement générés :

> Le passage des ateliers est court, mais il traverse une halle à découvert.
> Les galeries permettent de contourner cette halle. Le trajet est plus long.

**Proposition de règles.** Une première indication gratuite dans la ville de
départ. Une information provient d'un lieu et d'un état réellement présents
dans cette partie ; elle porte une date d'observation si son contenu peut
vieillir. La consultation suivante relit le même renseignement. La direction
peut être indiquée sans révéler le plan, les occupants cachés ou le butin.

Les services ordinaires restent utilisables sans cette consultation. Un
renseignement ne garantit pas une route sûre et ne connaît pas les actions
récentes d'ennemis hors de portée du témoin ou du terminal.

**À voir avant intégration.** Dialogue court, indication reçue, inscription
dans le dossier et un parcours où deux renseignements conduisent à des choix
différents. Les prix d'éventuelles informations supplémentaires restent ouverts.

## B — Banc d'essai, en ville

**Situation.** Dans un atelier, le joueur choisit deux armes de son inventaire
et une cible d'essai dont les protections sont annoncées.

**Intérêt.** Comprendre pourquoi une attaque est absorbée, comparer deux armes
et apprécier leur consommation avant de s'engager dans un vrai combat.

Exemple de résultat :

> Cette protection absorbe les tirs de la première arme.
> La seconde la traverse, mais utilise davantage de réserve.

**Proposition de règles.** Utiliser les formules de combat réelles sur un état
d'essai séparé. Montrer les résultats des attaques testées et leur coût simulé,
en distinguant un échantillon d'une garantie. N'afficher un intervalle exact
que si le moteur le fournit réellement. Les protections du mannequin sont
publiques ; les caractéristiques des ennemis de la partie restent cachées.

L'essai ne dépense ni munitions, ni énergie, ni durabilité de la partie. Il
n'accorde ni expérience ni objets et n'avance aucun tour du monde. Sa graine
reste séparée du hasard de la partie. Les effets complexes non pris en charge
dans la première version sont signalés avant l'essai.

**À voir avant intégration.** Comparaison de deux armes possédées, contre deux
protections annoncées, avec au moins un impact absorbé et un impact utile.
Cet atelier n'est pas un service d'amélioration supplémentaire.

## C — Alimentation d'un passage, hors ville

**Situation.** Une commande physique accessible depuis le chemin ordinaire
alimente une petite branche de l'installation. Cette branche fournit un
raccourci motorisé et le capteur qui surveille sa traversée.

**Intérêt.** Choisir entre une traversée courte sous surveillance et un détour
moins exposé aux réponses de cette installation. L'information sur les deux
effets est visible avant l'action.

Exemple de choix :

> Alimenter la branche : ouvrir le raccourci et activer son capteur.
> Couper la branche : fermer le raccourci et désactiver son capteur.

**Proposition de règles.** La commande agit sur cette branche seulement et
consomme une action normale. Le changement reste réversible depuis le poste.
Un détour praticable existe dans les deux états ; il n'est pas nécessaire de
posséder une technique d'intrusion pour utiliser la commande physique.

Le capteur obéit à sa portée, ses obstacles et ses réponses locales déclarées.
Son activation n'envoie aucune alerte à elle seule et ne donne pas la position
du joueur aux ennemis. Couper la branche arrête ses nouvelles détections ;
les alertes et renforts déjà engagés continuent selon leurs règles ordinaires.
L'absence de capteur ne garantit pas l'absence d'ennemis.

La fermeture attend si la case de porte est occupée. Le poste reste accessible
des deux côtés ou par le détour. La branche ne commande ni un service urbain
ni le seul accès à la prochaine couche. Son état persiste dans la partie.

**Présentation proposée.** Un terminal d'une case installé contre un mur,
porte existante et capteur identifié à l'inspection. Les états possèdent des
libellés ; les voyants peuvent accompagner cette information. Aucun grand
accessoire dans le passage. L'aperçu interactif illustre les conséquences,
sans proposer un nouveau dessin du jeu.

**À voir avant intégration.** Un petit lieu d'essai, les deux états, une
détection réellement évitée, une alerte déjà engagée qui reste valable et une
fermeture différée par un occupant. Aucune valeur de portée ou de dégâts n'est
fixée par cette proposition.

## D — Station de maintenance limitée, hors ville

**Situation.** Une station abandonnée dispose d'une réserve finie. Le joueur
peut employer une charge pour restaurer ses PV ou son énergie.

**Intérêt.** Arbitrer entre survivre au prochain combat et conserver les moyens
d'utiliser ses capacités. La découverte devient une décision de ressources,
pas seulement un coffre à ouvrir.

Exemple de texte :

> Il reste une charge. Réparer le module ou recharger son énergie ?
> La station fait du bruit pendant son utilisation.

**Proposition de règles.** Le bénéfice, les charges restantes et le bruit sont
annoncés avant validation. Une utilisation consomme une action et une charge
en même temps qu'elle applique le résultat. Choisir un bénéfice déjà au maximum
est refusé sans coût. La station ne régénère pas sa réserve en quittant le lieu,
en attendant ou en reprenant une sauvegarde.

Le bruit utilise l'audition normale : un ennemi présent peut enquêter s'il
l'entend. Aucun ennemi supplémentaire n'apparaît pour punir l'utilisation.
Les valeurs de soin, de recharge, de bruit et le nombre initial de charges
restent des paramètres d'essai à mesurer avec plusieurs personnages.

Cette réserve prend une part du budget de ressources du lieu. Son implantation
ne garantit pas une recharge à chaque région et ne conditionne pas la descente.
Elle est accessible depuis le sol adjacent ; son dessin reste celui d'un petit
appareil en vue de dessus, à soumettre séparément si un nouvel accessoire est
nécessaire.

**À voir avant intégration.** Deux besoins concurrents, une utilisation sous
menace audible et une station épuisée qui conserve son état après reprise.

## Ordre conseillé

Commencer par **A et C** : un renseignement permet de préparer une expédition,
puis un dispositif du terrain permet d'en modifier concrètement la traversée.
Présenter ce lieu d'essai avant de le distribuer dans les cartes.

Tester **B** ensuite pour aider à comprendre les combats et les impacts
absorbés. Introduire **D** après les essais de combat : sa réserve peut rendre
une couche trop facile si elle s'ajoute sans contrepartie au butin existant.

Une consigne n'est pas retenue dans ce premier lot : les passages sans retour
réduisent son intérêt, alors que les retours pour gérer le sac peuvent devenir
une routine. Cette décision de proposition ne supprime aucun objet du jeu.

## Appuis vérifiés dans le projet

- `src/facility/mod.rs` : relais d'alimentation, dépendances d'installations,
  terminaux de données, capteurs, réponses locales et réparation persistante.
- `src/intrusion.rs` : commandes d'appareils, droits d'accès, traces et contrôle
  borné. Une commande physique de branche exige cependant un contrat propre.
- `src/engineering.rs` : récupération de composants persistante et réglages
  de modules ; ces fonctions ne sont pas présentées ici comme des nouveautés.
- `docs/MONDE_EXPLORATION_ET_PROGRESSION.md` : exploration libre, départ vers
  une couche irréversible, quêtes locales facultatives et sauvegardes historiques.
- `docs/PREMIERE_COUCHE_JOUABLE.md` : deux itinéraires, annexes et raccourcis
  déjà présents ; la proposition ajoute des choix situés dans les lieux.

La lecture du code vérifie ces appuis, pas la jouabilité des interactions
proposées. Leur intégration exigera des essais moteur, des parcours sur plusieurs
graines, des captures natives et la conservation des anciennes sauvegardes.
