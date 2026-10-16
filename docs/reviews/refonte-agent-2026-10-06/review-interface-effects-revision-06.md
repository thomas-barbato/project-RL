# Affichage des effets, des hostiles, du calme et des recharges

6 octobre 2026. Le joueur demande un affichage des hostiles repérés et de
la phase combat ou calme. Il précise que le survol du statut des hostiles
doit mettre tous les hostiles visibles en surbrillance. Ces besoins sont
retenus, avec le futur espace des buffs et debuffs. Les libellés, la règle
commune de phase et les rendus ci-dessous sont proposés avant intégration.
Le joueur demande ensuite un décompte de recharge dans le menu des actions,
avec les compétences en recharge impossibles à activer ; ce besoin est retenu
dans HUD05. Les durées et recharges restent exprimées en tours.

### HUD01 Icônes de buffs et debuffs

> Consultez les effets qui vous affectent et leur durée restante.

**Confirmé :** afficher des icônes des buffs et debuffs du personnage.
Passer la souris sur une icône montre le nom, l'effet et le temps restant
en nombre de tours. Le joueur évoque le haut à droite de la zone de jeu
comme emplacement possible, sans fixer ce placement.

**Proposition de contenu :** le tooltip emploie le nom validé de l'effet,
sa description courte et ses valeurs actuelles lorsque pertinentes,
puis la durée restante. Il doit afficher le temps réel de l'effet, sans
créer un nouveau compteur indépendant du combat. Consulter une icône
n'avance pas le temps ; les menus ne font pas décroître cette durée.

L'icône disparaîtrait à la fin de l'effet. Un état permanent ou conditionnel
ne reçoit pas un faux décompte : une indication telle que « Permanent »
ou sa condition pourrait le remplacer. Le choix des états permanents
à afficher, les cumuls et la priorité de lecture sont à définir avec
la maquette du HUD, sans obliger à afficher chaque passif appris en continu.

Les icônes, noms affichés, textes, couleurs et emplacement attendent
une revue visuelle. Ce besoin est prévu pour plus tard ; la discussion
actuelle reste centrée sur les compétences à retenir.

### HUD02 Hostiles repérés et surbrillance

**Demandes acceptées :** afficher seulement les hostiles actuellement
repérés ; survoler ce statut met tous les hostiles visibles en surbrillance.
L'accord vient de la réponse : « 1 et si on place sa souris sur le status
d'hostile repéré, ca met en surbrillance tous les hostiles visibles ».

**Libellés proposés :** « Aucun hostile repéré », « 1 hostile repéré » ou
« N hostiles repérés ». Le tooltip précise : « Hostiles actuellement visibles. »
Le nombre déduplique les acteurs vivants actuellement observés par l'agent
ou par un partage de vision réellement disponible. Il n'ajoute pas de vision
aux drones qui n'en transmettent pas. Un hostile précédemment vu et perdu
de vue sort du compteur et de la surbrillance. Les neutres et alliés sont exclus.

La portée de « zone » est la carte active et les observations disponibles,
pas le nombre total d'ennemis dans la couche. Aucun bruit non identifié ne
devient un hostile confirmé. Le compteur et le survol ne révèlent ni acteur,
position, terrain ou source cachés. Zéro signifie aucun hostile repéré.

**Rendu retenu dans l'aperçu :** cadres rouges aux coins des cases des hostiles visibles,
sur une couche distincte de leurs sprites et des icônes de perception.
Les repères cessent au départ de la souris et suivent la visibilité réelle
même pendant le survol. Le focus clavier et une activation tactile proposent
le même repérage ; ce complément d'accessibilité reste à valider.
Le survol et le focus n'avancent aucun tour et ne consomment aucun tirage.

**Ajout demandé :** un léger clignotement. La révision proposée fait varier
l'opacité des seuls cadres entre 100 % et 60 %, sur un cycle doux de 1,4 seconde,
tant que le repérage est actif. Les cadres restent toujours visibles ; les
sprites, le texte du statut et l'indicateur de phase restent fixes. L'effet
ne consomme aucun tour. Avec les mouvements réduits, les cadres restent fixes
à pleine opacité. Le joueur accepte l'animation de l'aperçu en répondant
« parfait on peut continuer . ». Le cycle de 1,4 seconde et les cadres
toujours visibles sont retenus tels que présentés ; le rendu natif reste à revoir.

### HUD03 Phase de combat et retour au calme

**Demande acceptée :** indiquer si le jeu est en combat ou au calme.
**Proposition de présentation :** deux informations proches, mais distinctes :
le statut des hostiles de HUD02 et la phase décrite ici. Leur place près des
buffs et debuffs, en haut à droite de la zone de jeu, reste une proposition.

| Phase proposée | Libellé proposé | Signal proposé | Condition proposée |
| --- | --- | --- | --- |
| Combat | Combat | Rouge, avec texte | Hostile actuellement repéré ou événement de combat ce tour |
| Retour au calme | Accalmie — N tours avant le calme | Ambre, avec décompte | Aucun hostile repéré ni événement ce tour, mais trois tours complets d'accalmie pas encore atteints |
| Calme | Calme | Vert, avec texte | Trois tours complets consécutifs d'accalmie atteints, sans menace actuellement repérée |

L'accalmie conserve les règles de récupération du combat jusqu'au passage
au calme. Son état intermédiaire explique l'attente ; il n'ajoute pas un
troisième barème d'énergie. Une absence d'hostiles visibles ne garantit
donc pas immédiatement le calme : une attaque récente ou des dégâts
persistants peuvent encore empêcher la récupération hors combat.

**Règle commune proposée :** la phase concerne l'agent et ses drones déployés.
Le compteur se remet à zéro si un hostile est actuellement repéré, si l'un
d'eux attaque, si une attaque résolue vise l'un d'eux même sans dégâts, ou
si l'un d'eux subit des dégâts directs ou persistants. Une simple décision
d'IA cachée ou la présence d'un ennemi inconnu n'est pas un événement de combat.
Sans ces événements, chaque tour complet ajoute un tour d'accalmie, jusqu'à trois.

Après les trois tours complets, le HUD annonce Calme pour le prochain tour.
Les effets réservés au calme commencent au quatrième tour consécutif si
aucun événement ni hostile repéré ne l'interrompt. L'état est recalculé après
les actions, dégâts et mises à jour de perception ; l'interface lit cette
même décision. Elle ne possède pas son propre compteur de simulation.
Menus, survols, attente en temps réel et actions instantanées ne l'avancent pas.

Le tooltip montre le motif disponible : « Hostile repéré », « Attaque reçue »,
« Attaque effectuée », « Dégâts persistants » ou le nombre de tours restants.
Il ne nomme ni ne localise une source inconnue. Plusieurs motifs peuvent
coexister ; une cause visible ou un événement effectivement subi suffit.

Au début d'une nouvelle partie sans hostile repéré, la phase proposée est
Calme. Un changement de couche remet le compteur à zéro : Accalmie avec trois
tours restants si aucun hostile n'est repéré à l'arrivée, Combat sinon.
L'action de transition ne compte pas comme un tour d'accalmie de la nouvelle carte.
Le rappel d'un drone conserve la phase commune ; il ne crée pas de soin.

### HUD04 Effets liés au calme

**Proposition de cohérence :** E01 Énergie et DP01 Maintenance des drones
utilisent la phase de HUD03. Les pourcentages proposés restent inchangés.
Maintenance répare les drones déployés uniquement lors d'un tour éligible
au calme ; elle ne possède plus cinq délais individuels cachés. Une attaque
ou des dégâts sur l'agent ou un drone interrompent la réparation commune.

Pour Maintenance, les libellés proposés sont « Réparation suspendue » en
combat, « Réparation en attente du calme » pendant l'accalmie et « Réparation disponible »
au calme. Le tooltip explique la cause, le pourcentage actuel et le fait
qu'un gain fractionnaire peut nécessiter plusieurs tours avant de rendre un PV entier.
Un drone rappelé ne répare pas. Le rang et le maximum de PV actuels, les
fractions accumulées et la limite des PV manquants conservent leur fonctionnement.

Les conditions propres aux autres compétences restent affichées par leurs
icônes : SU03 Barrière réactive garde ses quatre tours sans dégâts et peut
être prête pendant un combat. La phase globale ne réécrit pas ce délai propre.

### HUD05 Recharge des compétences dans le menu des actions

**Demandes acceptées :** pour chaque compétence encore en recharge, afficher
le nombre de tours restants avant sa prochaine utilisation dans le menu
des actions. Empêcher de cliquer pour activer cette action pendant sa recharge.
Le joueur confirme également que le décompte en tours lui convient.

**Libellé accepté :** « Interférence — Recharge : 3 tours ». La déclinaison
singulière proposée est « Interférence — Recharge : 1 tour ». Une fois la recharge terminée,
le menu retrouve son entrée normale, par exemple « Interférence ».
Le nom reste celui de la compétence validée. Le nombre correspond à son
temps de recharge actuel, et non à sa durée totale de recharge au lancement.

**Présentation retenue :** conserver l'action visible et griser son bouton
tant que sa recharge est positive. Le nom et le décompte restent lisibles.
Le survol peut toujours montrer sa fiche et ses informations ; le bouton
désactivé ne commence pas une sélection de cible et ne lance pas le pouvoir.
Les couleurs et la présentation exactes seront montrées avant intégration.

Le blocage doit aussi être respecté par les raccourcis et par les règles
du jeu : une autre voie d'entrée ne doit pas contourner la recharge.
Une tentative sur une compétence en recharge ne dépense ni énergie ni
munitions, ne fait avancer aucun tour et ne consomme aucun tirage aléatoire.
Elle ne modifie pas le temps de recharge restant.

L'interface lit le même compteur que la règle autorisant l'utilisation.
Le décompte se met à jour lorsque le temps du jeu avance. Ouvrir un menu,
survoler un bouton, cliquer sur l'action désactivée ou attendre en temps
réel ne fait pas diminuer ce délai. Si une fraction de tour reste à attendre,
l'affichage proposé arrondit au supérieur et conserve le bouton désactivé
jusqu'à la fin réelle de la recharge ; il ne présente pas un faux zéro.

Lorsque le délai atteint zéro, le blocage dû à la recharge est retiré.
Les autres conditions d'utilisation, notamment l'énergie disponible et
la validité de la cible, continuent à s'appliquer. Les compétences sans
recharge et les passifs ne reçoivent pas de faux décompte.

Les fiches distinguent les informations : « Durée » désigne le temps
pendant lequel l'effet agit ; « Recharge » indique l'attente avant une
nouvelle utilisation. Les recharges existantes conservent leurs valeurs.

## Aperçu et validation

Le prototype montre un compteur, sa surbrillance et les trois phases sur une
grille schématique. Ses lettres sont des repères, pas de nouveaux dessins
de personnages ou de monstres. Le rendu natif dans le jeu sera présenté
séparément, avec les sprites existants et leurs couches intactes.

Le joueur répond « parfait » à l'aperçu, puis demande un léger clignotement.
L'accord retient les éléments présentés : le repérage au survol, le compteur
des hostiles visibles et la phase commune Combat, Accalmie, Calme avec trois
tours d'attente, utilisée pour l'énergie et Maintenance. Les libellés et
cadres observés sont retenus dans cet aperçu schématique.

La pulsation de l'aperçu révisé est ensuite acceptée par « parfait on peut
continuer . ». Le rendu natif dans le jeu, l'emplacement définitif et les
règles non présentées, notamment l'initialisation et les transitions,
attendent leur propre validation. Les détails d'accessibilité non présentés
gardent leur statut de proposition. Cet accord ne valide pas automatiquement
les pourcentages de réparation ou le texte roleplay de DP01.

HUD05 enregistre une demande explicite d'affichage et de blocage des actions
en recharge. Le joueur accepte ensuite le libellé présenté, le bouton grisé,
le blocage et le survol en répondant « parfait tu peux continuer ». Les détails
non présentés et le rendu natif du menu restent à revoir. Le prototype des hostiles ne montre
pas encore ce menu ; aucun changement du menu du jeu n'est intégré par ce document.
