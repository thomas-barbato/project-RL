# Affaiblissements — descriptions à revoir, lot 06

6 octobre 2026. Six compétences du catalogue principal, sans ajout de
nouvelle compétence. Les noms, les rôles et les intentions visuelles du
catalogue conservent leur accord. Le joueur accepte les textes, tableaux et
limites présentés comme base de travail, à condition que les cinq actifs se
déclenchent dès leur lancement. Cette condition est confirmée ci-dessous.
Les formules et cas détaillés non présentés restent des propositions à revoir.
Ce document ne les intègre pas au jeu.

## Profils des deux colonnes

- **Par défaut :** compétence apprise au rang 1, Intelligence 5, sans
  équipement, implant ou autre passif qui modifie son résultat.
- **Actuel dans l'exemple :** rang appris 10, bonus d'équipement +2,
  rang effectif 12 et Intelligence 25. Les autres modificateurs sont absents.
  Il s'agit d'un personnage de comparaison, pas d'une sauvegarde du joueur.

Le rang augmenté sera affiché en cyan dans l'interface, conformément à
la demande déjà retenue. Les pourcentages sont affichés au dixième ; les
fractions exactes sont conservées dans les calculs. La possibilité d'apprendre
les compétences reste déterminée par les seuils de branche, à fixer séparément.

## Paramètres communs proposés

Les cinq actifs visent un hostile actuellement visible à cinq cases au
plus, avec une ligne de vue dégagée. Après choix de la cible et confirmation,
l'effet est résolu immédiatement, avec l'éventuelle résistance de la cible,
avant les réactions ennemies. Le lancement consomme le temps d'une action
ordinaire : les ennemis peuvent ensuite agir selon les règles du jeu.
Il ne demande aucun déplacement du joueur ni tour de préparation ou d'attente.
« Une action » désigne ce coût en temps, et non une préparation avant l'effet.
Désorientation demande également une fausse position avant confirmation ;
le détournement est appliqué immédiatement et la cible se déplace ensuite
lors de ses propres actions. Ce fonctionnement concerne les cinq actifs
de ce lot ; il ne modifie pas le marquage sans passage de tour ni le délai
d'armement d'une mine.
Une cible illégale, une recharge en cours ou une énergie insuffisante
entraîne un refus sans dépense, tour ni tirage. Une tentative légale résistée
consomme son action, son énergie et sa recharge.

Les affaiblissements fonctionnent sur les machines, les êtres organiques
et les démons. Leur puissance est opposée à la résistance occulte de la cible.
Il n'est pas créé d'immunité implicite selon sa nature ou son importance.
Les profils et compétences des monstres seront revus séparément.

Les contributions d'Intelligence reprennent les propositions S05 :

- Puissance d'application : max(1, 100 + 5 × (n − 1) + 2 × (Intelligence − 5)).
- Tours supplémentaires : min(3, floor(max(0, Intelligence − 5) / 20)).
- Chance d'application : clamp(100 × puissance / (puissance + résistance), 5, 95),
  avec une résistance d'au moins 1.

Les cinq actifs ont donc une puissance de 100 par défaut et 195 dans
l'exemple. Contre une résistance de 25, cela donne respectivement 80 % et
88,6 % de réussite. La résistance de 25 sert seulement à cette comparaison ;
les chances réelles dépendent de la cible. L'Intelligence ne multiplie pas
une seconde fois les pourcentages de réduction de dégâts ou d'armure.

Les durées indiquées incluent Intelligence. L'implant O00 ajoute son tour
de durée si équipé ; le passif A03 peut réduire une seule fois le coût
de ces pouvoirs technologiques et magiques. Ces deux effets sont exclus
des exemples. Leurs barèmes non validés conservent leur statut de proposition.

Les durées et recharges utilisent l'horloge de tours du jeu. Les menus,
survols et attentes en temps réel ne les avancent pas. Les paramètres sont
fixés lors de l'application ; retirer ensuite un objet ne réécrit pas un
effet déjà posé. Une réapplication réussie remplace les paramètres de la
même compétence, sans additionner ses versions. Des compétences différentes
peuvent coexister. Les cases, portées et lignes de vue emploient les règles
de carte existantes, sans traverser les murs ni révéler de position cachée.

**Ajout demandé le 7 octobre — HUD06 :** tout statut appliqué par une attaque
doit figurer dans la barre de buffs et debuffs de la cible, consultable par
l'action d'analyse de cible. Les valeurs de ces effets sont accessibles
même sans Éclaireur. Cette demande vaut également pour les affaiblissements
de ce lot ; les icônes et la présentation native restent à revoir dans
[HUD06](REVUE_INTERFACE_EFFETS_AGENT.md#hud06-statuts-dans-lanalyse-de-cible).

### O02 Interférence

> Vous mêlez parasites électroniques et murmures occultes. Les attaques
> de la cible perdent de leur puissance.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Réduction des dégâts directs infligés | 15 % | 33,3 % |
| Durée | 3 tours | 6 tours |
| Coût d'énergie | 8 | 11 |
| Recharge | 5 tours | 5 tours |
| Portée maximale | 5 cases | 5 cases |
| Puissance d'application | 100 | 195 |

**Progression proposée, inchangée depuis O02 du lot 02 :** réduction =
15 + 35 × (n − 1) / (n + 9) pour cent ; durée de base =
3 + floor((n − 1) / 5) ; énergie = 8 + floor((n − 1) / 3).
La réduction reste strictement inférieure à 50 %, même au-delà du rang 10.

La réduction concerne les dégâts directs produits par la cible, avant les
défenses de la victime, quel que soit le type de dégâts. Elle ne retire
aucune action, ne réduit pas les soins et ne modifie pas rétroactivement
les dégâts persistants déjà posés. Une invocation distincte ne reçoit pas
l'affaiblissement de son invocateur par ce seul lien.

**Fiche individuelle acceptée le 7 octobre :** le joueur valide le texte
roleplay et la comparaison des rangs effectifs 11 et 12 : réduction des
dégâts directs de 32,5 % puis 33,3 %, durée de cinq tours, portée de cinq
cases, coût de 11 énergie et recharge de cinq tours. L'exemple utilise
Intelligence 10 et une résistance occulte de cible de 25, soit 86,5 % puis
86,8 % de chance d'application. Il valide aussi la cible hostile visible
en ligne de vue, les trois natures de cible et l'absence de modification
des dégâts persistants déjà appliqués. Les formules et cas non affichés
gardent leur statut de proposition. La fiche est conservée sous
[O02-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-O02-02.json).

**Intention visuelle retenue :** parasites en créneaux autour des armes ou
membres d'attaque. L'animation dans le jeu reste à présenter.

### AF02 Faille d'armure

> Un sceau occulte s’insinue dans les failles de l’armure de votre cible.
> Une rouille surnaturelle la ronge et fragilise sa protection.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Réduction de l'armure physique | 20 % | 41 % |
| Durée | 3 tours | 6 tours |
| Coût d'énergie | 9 | 12 |
| Recharge | 5 tours | 5 tours |
| Portée maximale | 5 cases | 5 cases |
| Puissance d'application | 100 | 195 |

**Progression proposée :** réduction = 20 + 40 × (n − 1) / (n + 9)
pour cent ; durée de base = 3 + floor((n − 1) / 5) ;
énergie = 9 + floor((n − 1) / 3). La réduction reste strictement
inférieure à 60 %.

L'armure employée pour une attaque physique est multipliée par
1 − réduction / 100 avant sa résolution ordinaire. Il s'agit de l'armure,
y compris naturelle, et non d'une augmentation directe des dégâts reçus.
Les résistances élémentaires, la résistance occulte et les protections
qui réduisent séparément les dégâts ne sont pas abaissées. L'affaiblissement
profite à toute attaque physique contre cette cible pendant sa durée.

Si une attaque ignore déjà une part d'armure, cette part s'applique ensuite
à l'armure restante. Les deux pourcentages ne s'additionnent pas. Les
fractions sont conservées jusqu'à la résolution ordinaire de la défense.
Une cible sans armure ne reçoit pas une armure négative.

**Texte roleplay révisé et accepté le 7 octobre :** le joueur demande un
sceau qui fait rouiller l'armure, puis valide les deux phrases ci-dessus.
Cet accord porte sur le texte. Les valeurs de la fiche individuelle
(40 % puis 41 %, cinq tours, cinq cases, 12 énergie, recharge de cinq tours,
86,5 % puis 86,8 % dans l'exemple) n'ont pas été représentées avec cette
révision et gardent leur statut de proposition. La rouille évoquée ne
crée pas de dégradation permanente ni de nouveau rendu validé.
La fiche révisée est conservée sous
[AF02-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-AF02-02.json).

**Intention visuelle retenue :** fissures rouges sur le contour, sans changer
l'équipement affiché. L'animation dans le jeu reste à présenter.

### AF03 Silence

> Vous étouffez les signaux et les incantations de votre cible, la réduisant au silence.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Compétences actives | Bloquées | Bloquées |
| Durée | 2 tours | 5 tours |
| Coût d'énergie | 12 | 15 |
| Recharge | 8 tours | 8 tours |
| Portée maximale | 5 cases | 5 cases |
| Puissance d'application | 100 | 195 |

**Progression proposée :** durée de base = 2 + floor((n − 1) / 5) ;
énergie = 12 + floor((n − 1) / 3). Le rang améliore la durée et la
puissance d'application, pas le nombre d'actions retirées.

La cible ne peut pas commencer une nouvelle compétence active pendant
le silence. Elle peut se déplacer et effectuer ses attaques ordinaires.
Ses passifs restent actifs. Le pouvoir ne dissipe pas les effets déjà posés,
ne détruit pas ses invocations et n'interrompt pas rétroactivement une action
déjà commencée. Une attaque de base reste ordinaire même si son apparence
est magique ; le classement des actions de chaque monstre doit être explicite.

**Texte roleplay révisé et accepté le 7 octobre :** le joueur valide la
phrase ci-dessus. La fiche individuelle aux rangs effectifs 11 et 12
conserve ses valeurs proposées : quatre tours, cinq cases, 15 énergie,
recharge de huit tours, et 86,5 % puis 86,8 % de chance d'application dans
l'exemple. Ces valeurs et les conditions n'ont pas été représentées avec
la dernière phrase validée ; leurs accords antérieurs restent inchangés.
Ce texte ne crée pas de nouvelle règle de suppression du bruit ni de rendu
déjà validé. La fiche révisée est conservée sous
[AF03-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-AF03-02.json).

**Intention visuelle retenue :** anneau barré qui ferme les points d'émission.
L'animation dans le jeu reste à présenter.

### AF04 Rupture de régénération

> Une trame occulte entrave les chairs et les circuits. Les blessures
> restent ouvertes, les réparations n'aboutissent plus.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV rendus par soins, régénération et réparations | 0 | 0 |
| Durée | 3 tours | 6 tours |
| Coût d'énergie | 10 | 13 |
| Recharge | 6 tours | 6 tours |
| Portée maximale | 5 cases | 5 cases |
| Puissance d'application | 100 | 195 |

**Progression proposée :** durée de base = 3 + floor((n − 1) / 4) ;
énergie = 10 + floor((n − 1) / 3). Le rang améliore la durée et la
puissance d'application. Le blocage des soins est déjà complet au rang 1.

La cible ne regagne pas de PV pendant l'effet, qu'ils viennent d'elle-même,
d'un allié, d'une réparation ou d'un passif. Le pouvoir ne supprime pas
les PV existants et n'empêche pas de récupérer de l'énergie. Une augmentation
de PV maximum ne devient pas un soin. Les soins bloqués ne sont pas stockés
pour être rendus à la fin. L'effet ne définit pas les futures résurrections.

**Fiche individuelle acceptée le 7 octobre :** le joueur valide le texte
roleplay, les effets et les conditions présentés aux rangs effectifs 11
et 12 : aucun PV rendu par les soins, la régénération et les réparations,
durée de cinq tours, portée de cinq cases, coût de 13 énergie et recharge
de six tours. L'exemple utilise Intelligence 10 et une résistance occulte
de cible de 25 : 86,5 % puis 86,8 % de chance d'application. La cible est
un hostile visible en ligne de vue ; les PV déjà présents et la récupération
d'énergie restent inchangés. Les formules et autres cas non affichés
gardent leur statut de proposition. La fiche est conservée sous
[AF04-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-AF04-02.json).

**Intention visuelle retenue :** impulsions de soin coupées par une grille noire.
L'animation dans le jeu reste à présenter.

### AF05 Désorientation

> Vous laissez une fausse présence dans ses perceptions. La cible part
> à votre recherche là où vous n'êtes pas.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Destination recherchée | Fausse position choisie | Fausse position choisie |
| Durée maximale | 2 tours | 5 tours |
| Coût d'énergie | 10 | 13 |
| Recharge | 7 tours | 7 tours |
| Portée maximale de la cible | 5 cases | 5 cases |
| Distance maximale entre cible et fausse position | 3 cases | 3 cases |
| Puissance d'application | 100 | 195 |

**Progression proposée :** durée de base = 2 + floor((n − 1) / 5) ;
énergie = 10 + floor((n − 1) / 3). Le rang améliore la durée et la
puissance d'application ; il n'agrandit pas la zone de choix du leurre.

Après avoir choisi la cible, le joueur choisit une case actuellement visible,
vide et franchissable, à trois cases au plus de celle-ci. Un trajet légal
dans le terrain connu doit y mener. La confirmation forme un seul lancement.
Annuler l'un de ces choix ne consomme rien. Ce pouvoir ne crée pas une
entité attaquable et ne révèle pas un trajet ou une case cachés.

La cible donne la priorité au trajet vers cette fausse position. Arrivée
sur place, elle consacre ses actions à y rechercher l'agent jusqu'à la fin
de l'effet. Elle peut toujours attaquer un adversaire au contact ; elle
n'effectue pas ses attaques à distance pendant cette recherche. Ses passifs
et effets déjà posés continuent. Elle ne traverse pas une case bloquée
et ne reçoit ni téléportation ni déplacement gratuit.

Recevoir des dégâts met fin à l'effet, quelle qu'en soit la source.
Un trajet devenu impossible y met également fin. Les attaques au contact
restent une menace ; l'effet sert à séparer ou détourner un ennemi, et
ne le transforme pas en allié.

**Intention visuelle retenue :** écho bref de l'agent sur la fausse case et
fil brouillé vers la cible. L'animation dans le jeu reste à présenter.

### AF06 Contagion de sceau

> Les sceaux survivent un instant à leur porteur. Quand il tombe,
> l'un d'eux s'accroche à une menace proche.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Nouveaux voisins visés par mort | 1 | 1 |
| Distance maximale depuis la victime | 2 cases | 4 cases |
| Part conservée de durée, puissance et réduction chiffrée | 50 % | 71 % |
| Coût d'énergie / recharge | Aucun / aucune | Aucun / aucune |
| Propagation depuis un sceau déjà transmis | Impossible | Impossible |

**Progression proposée :** proportion conservée =
0,5 + 0,4 × (n − 1) / (n + 9) ; distance =
min(4, 2 + floor((n − 1) / 5)). La proportion progresse au-delà de 10
et reste strictement inférieure à 90 %. La portée plafonne à quatre cases.

Le passif fonctionne en permanence lorsqu'il est appris. À la mort d'un
hostile portant un sceau directement appliqué par l'agent, il tente une
seule transmission. Le coup fatal peut venir d'une autre source.
Les sceaux compatibles proposés sont Interférence, Faille d'armure,
Silence et Rupture de régénération. Désorientation n'est pas transmise :
elle dépend d'une fausse position choisie pour sa propre cible.

Si plusieurs sceaux compatibles sont présents, le plus récemment appliqué
est choisi. Le passif vise automatiquement l'hostile vivant actuellement
visible le plus proche, à portée et avec une ligne de vue depuis la victime,
qui ne porte pas déjà ce même affaiblissement. Une égalité de distance suit
l'ordre des cases, ligne puis colonne. Sans candidat, il ne se passe rien.
Le voisin peut résister ; l'échec ne lance pas une autre tentative.

La durée transmise est floor(durée restante × proportion), avec un minimum
de un tour et sans dépasser la durée restante. La puissance transmise est
max(1, floor(puissance du sceau × proportion)). Les réductions chiffrées
d'Interférence et de Faille d'armure sont multipliées par la même proportion.
Silence et Rupture gardent leur effet binaire ; seules leur durée et leur
puissance d'application sont atténuées. Les bonus d'Intelligence et d'implant
déjà inclus dans le sceau ne sont pas ajoutés une deuxième fois.

**Exemple avec une Interférence encore à sa durée complète :**

| Sceau transmis | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Durée restante avant transmission | 3 tours | 6 tours |
| Réduction des dégâts transmise | 7,5 % | 23,7 % |
| Durée transmise | 1 tour | 4 tours |
| Puissance transmise | 50 | 138 |
| Réussite contre une résistance de 25 | 66,7 % | 84,7 % |

Le sceau transmis ne peut pas transmettre à son tour. Il ne remet pas la
durée à son maximum d'origine. Un autre sceau directement appliqué ensuite
sur cette nouvelle cible peut, lui, alimenter une future transmission.
Le passif ne dépense ni action supplémentaire ni énergie et ne modifie
pas la récompense de mort ou l'attribution du coup fatal.

**Intention visuelle retenue :** sceau qui se détache et saute vers la
nouvelle cible. L'animation dans le jeu reste à présenter.

## État de la revue

Le joueur accepte le lot présenté en précisant : « Si c'est le ca sc'est
bon :) », sous la condition d'un déclenchement dès le lancement, sans
déplacement ni attente préalable. Les six textes roleplay, les tableaux
et les limites exposés dans la réponse précédente sont retenus comme base
d'équilibrage, avec cette activation immédiate confirmée.

Les formules et cas détaillés non présentés restent à revoir séparément,
notamment l'ordre de remplacement des sceaux, la sélection automatique
du voisin et les règles précises de transmission. Les animations natives
conservent leur revue distincte. L'accord ne s'étend pas aux compétences
des autres branches ou aux délais des arbres secondaires.

O02 conserve son identifiant et ses barèmes antérieurs ; le nouveau texte
et les valeurs actuelles présentés sont acceptés comme base de travail.
Ce lot contient cinq actifs et un passif, sans changer le total du catalogue.
La salle de test configurable reste prévue après les compétences et animations.
