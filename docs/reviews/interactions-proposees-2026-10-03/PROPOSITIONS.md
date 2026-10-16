# Interactions en ville et hors ville

3 octobre 2026 — propositions à lire avant intégration.

Ce lot développe cinq interactions facultatives. Leurs effets, leurs textes
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

La révision inspirée de Cogmind renforce l'information obtenue sur le terrain
et les moyens de modifier une traversée. Les observations sourcées, les
adaptations proposées et les différences avec Project RL sont séparées dans
[Inspiration Cogmind](INSPIRATION_COGMIND.md). La nouvelle piste E reste à
valider, au même titre que les quatre propositions initiales.

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

**Variante hors ville proposée après l'étude de Cogmind.** Un terminal de
secteur propose quelques requêtes utiles : obtenir un repère vers une issue,
localiser un service de maintenance ou consulter les dispositifs reliés au
réseau local. Sa liste est construite à partir des installations qu'il peut
réellement connaître ; il ne produit pas un renseignement inventé au moment
de la consultation. Il peut ne proposer qu'une partie de ces fonctions.

La lecture d'un renseignement public et une extraction non autorisée restent
deux actions différentes. Seule la seconde peut relever des règles existantes
d'intrusion, avec son coût et ses risques annoncés. Consulter ou abandonner le
menu ne tente aucune intrusion automatiquement. Un repère obtenu n'ajoute pas
le terrain environnant à la mémoire explorée et ne révèle aucun occupant.

Cette variante fait du terminal une découverte utile pendant l'expédition,
avec la possibilité de repartir dès que l'information recherchée est obtenue.
Il n'est pas nécessaire de télécharger tout son contenu ni d'accepter une quête.

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

L'inspiration Cogmind porte ici sur une installation trouvée dans le monde,
dont l'utilisation a un coût et un intérêt précis. Le choix PV/énergie et la
réserve finie sont des propositions propres à Project RL ; ils ne décrivent
pas les règles des stations de réparation de Cogmind.

## E — Poste de diversion, hors ville

**Situation.** Un poste de travail possède un signal de maintenance. Le joueur
peut le déclencher pour attirer l'attention vers ce poste pendant qu'il cherche
à franchir un autre passage. Le dispositif est physique et occupe une case
contre un mur ; son emplacement se découvre normalement.

**Intérêt.** Traverser une zone surveillée en préparant un déplacement plutôt
qu'en affrontant systématiquement ses occupants. L'effet dépend des lieux,
des obstacles, des sens et du comportement des ennemis réellement présents.

Exemple de texte :

> Déclencher le signal au poste de maintenance.
> Les ennemis qui l'entendent peuvent venir l'inspecter.

**Proposition de règles.** Une activation coûte une action normale et dépense
la charge disponible du poste. Le signal dure un nombre fini de tours annoncé
avant usage. Ses cibles sont les acteurs qui l'entendent selon l'audition et
les priorités ordinaires de leur IA, pas les ennemis de toute la carte.

Un ennemi qui voit déjà le joueur n'abandonne pas automatiquement le combat.
Un ennemi sourd ou incompatible avec ce comportement n'est pas détourné. Les
investigateurs se dirigent vers la source fixe du signal ; ils n'apprennent ni
la nouvelle position du joueur ni l'itinéraire qu'il souhaite suivre. La
diversion peut attirer un autre ennemi et rendre le passage moins favorable.

La charge et la durée persistent après un changement de région ou une reprise.
Le poste n'engendre aucun renfort ni récompense et ne se recharge pas par attente.
Lire son menu laisse le monde en pause comme les autres menus du jeu. Une
commande refusée ne dépense ni charge ni tour. Les compétences spécialisées
de diversion conservent leur contrat propre ; ce poste offre une occasion
locale liée à la géographie, pas une nouvelle technique gratuite transportable.

**À voir avant intégration.** Un ennemi qui enquête réellement sur le poste,
un ennemi gardant sa cible visible, un obstacle qui modifie la propagation du
son selon les règles du moteur, et une source terminée qui ne génère plus
de nouvelles réactions. Une enquête déjà engagée garde son déroulement normal.
Les durées, charges et paramètres sonores restent des valeurs d'essai ouvertes.

## Ordre conseillé

Commencer par **A et C** : un renseignement permet de préparer une expédition,
puis un dispositif du terrain permet d'en modifier concrètement la traversée.
Présenter ce lieu d'essai avant de le distribuer dans les cartes.

Éprouver **E** dans ce même lieu ensuite : vérifier qu'un passage peut être
franchi grâce à une diversion réelle et qu'un mauvais placement peut échouer.
La combinaison des deux dispositifs doit offrir des réponses différentes,
sans devenir une séquence de boutons obligatoire ou toujours gagnante.

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
