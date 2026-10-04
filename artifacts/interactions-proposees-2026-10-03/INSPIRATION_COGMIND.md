# Cogmind : appuis pour les interactions de Project RL

3 octobre 2026 — étude de conception et révision de propositions.

Les observations ci-dessous proviennent du manuel officiel et des articles
du développeur. Les articles décrivent leurs versions historiques et leurs
choix de conception ; ils ne constituent pas un inventaire de la version
actuelle. Le manuel en texte consulté affiche **Beta 17.1**. Le PDF ancien
affiche Beta 7 et n'est pas retenu comme référence des règles actuelles.

Cette étude ne prétend pas analyser la vidéo YouTube ou avoir joué une partie.
Elle n'active aucune des interactions proposées dans Project RL.

## Ce que Cogmind apporte à cette réflexion

### Une découverte utile pendant la traversée

Le développeur décrit les renseignements de terminaux comme des avantages
stratégiques facultatifs : accès, machines, réserves et autres informations
sur les lieux. Ils donnent une raison de s'écarter momentanément du trajet.
[Map Intel, article du développeur, 2015](https://www.gridsagegames.com/blog/2015/02/map-intel-information-warfare-revisited/).

**Adaptation proposée.** Étendre A aux terminaux rencontrés hors ville : une
issue, un service utile ou les installations d'un réseau local. Chaque donnée
vient d'une source déclarée. Le joueur peut prendre l'information qui lui
manque et poursuivre, sans faire de ce terminal une liste à terminer.

### Décider quand s'arrêter

Le manuel distingue accès, détection et traçage ; fermer puis rouvrir une
session tracée ne réinitialise pas sa progression. Il décrit aussi les
stations de réparation de pièces et les unités de recyclage de composants.
[Manuel officiel, sections Hacking et Machines](https://www.gridsagegames.com/cogmind/manual.txt).

**Adaptation proposée.** Afficher le bénéfice demandé et les conséquences
avant une commande. A peut distinguer une donnée publique d'une extraction
qui relève de l'intrusion existante. C annonce les deux effets du changement
d'alimentation. D conserve une réserve finie et un bruit déclaré. Les essais
ne doivent pas permettre de réinitialiser les coûts en ouvrant un menu ou en
revenant dans la région.

Les ressources finies de D et son choix PV/énergie sont notre proposition.
Ils ne sont pas attribués aux stations de réparation de Cogmind.

### Agir sur les réponses du lieu

Un article du développeur montre notamment des commandes capables de
détourner la destination de groupes issus d'une garnison. L'intérêt vient
du changement de situation sur la carte.
[Unauthorized Hacks, article du développeur, 2015](https://www.gridsagegames.com/blog/2015/11/unauthorized-hacks/).

**Adaptation proposée.** Ajouter E, un poste de diversion à usage limité.
Il émet un bruit à un endroit fixe ; les acteurs capables de l'entendre
réagissent selon leur IA. Ce mécanisme physique se prête à un premier essai
local, sans définir une nouvelle autorité contrôlant tous les ennemis.
La correspondance avec Cogmind est l'action sur une traversée ; le signal
sonore et ses règles sont propres à notre proposition.

### Un détour qui change les choix de la partie

Le développeur explique que l'aide d'une branche peut faciliter certaines
situations en rendant d'autres plus difficiles. Il précise qu'un coût de
trajet ou de combat peut déjà constituer une contrepartie suffisante.
[Level Design and Shaping a Cogmind Experience, 2019](https://www.gridsagegames.com/blog/2019/02/level-design-shaping-cogmind-experience/).

**Adaptation proposée.** Placer quelques possibilités utiles dans des annexes
et sur des itinéraires distincts. Une information, une charge ou un poste
de diversion doit modifier une décision réelle. Évaluer l'effort pour l'atteindre
et son usage, plutôt que distribuer le même bonus supplémentaire dans chaque
région. Les conséquences doivent être annoncées avant un engagement sans retour.

### Des opérations rapides à comprendre

En retravaillant la fabrication, le développeur a retiré des étapes qu'il
jugeait fastidieuses, notamment les contraintes de conteneur et de niveau de
machine. Son objectif était de rendre l'usage occasionnel intéressant.
[Fabrication, article du développeur, 2016](https://www.gridsagegames.com/blog/2016/05/fabrication/).

**Adaptation proposée.** Chaque interaction commence avec quelques choix
compréhensibles, accessibles au clavier et à la souris. Une variante de
piratage s'appuie sur les techniques du jeu ; aucun mini-jeu obligatoire
ou suite de menus supplémentaires ne découle de cette étude.

## Premier lieu d'essai conseillé

1. Un renseignement reçu en ville décrit une halle surveillée et son détour.
2. Sur le terrain, un terminal peut fournir un repère vers une issue ou une
   station de maintenance réellement présente.
3. La traversée possède une commande d'alimentation accessible et, dans la
   seconde variante d'essai, un poste de diversion à usage limité.
4. Le joueur choisit de traverser, de contourner ou de préparer une diversion.
   Il peut aussi renoncer à utiliser les machines. Les services et la progression
   restent accessibles sans accepter une mission.
5. Si une réserve de maintenance est présente, l'utiliser constitue une dépense
   finie du budget de ressources du lieu, avec une menace audible à considérer.
6. Le joueur rejoint la prochaine couche selon les passages sans retour déjà
   validés. Une visite de tous les dispositifs n'est pas exigée.

La première comparaison porte sur trois approches : traversée directe,
contournement et diversion. Mesurer ressources dépensées, dégâts subis,
position des ennemis et lisibilité des conséquences sur plusieurs graines.
Ces mesures ne remplaceront pas un essai humain de la difficulté.

## Ce qui reste propre à Project RL

La descente irréversible, le corps durable, l'expérience, les compétences,
l'équipement et la liberté d'explorer dans la couche actuelle suivent les
décisions déjà prises pour le projet. Les machines restent de petits appareils
en vue de dessus, avec des couleurs de fonction indépendantes du thème.

Cette révision ne propose ni fabrication, ni réseau de recyclage automatique,
ni nouvelle faction, ni système d'alarme global. Le banc d'essai B conserve
son intérêt pédagogique en ville, après les interactions de terrain A, C et E.
Les conditions de la victoire finale et l'introduction restent à définir
séparément. Le lot complet et ses exemples demeurent soumis à validation.
