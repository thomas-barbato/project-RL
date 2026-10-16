# Revue des compétences secondaires

6 octobre 2026. Le joueur ajoute des arbres secondaires de furtivité,
mines et grenades. Ils sont accessibles à toutes les classes et utilisent
les points de compétence ordinaires. Cette revue enregistre ses règles
et propose des textes et progressions ; aucune compétence de ce document
n'est encore intégrée au jeu.

Le [cadrage complet des compétences](CADRAGE_COMPETENCES_AGENT_COMPLET.md#compléments-des-trois-arbres-secondaires)
propose désormais les paramètres qui restaient ouverts dans cette revue,
notamment les coûts de base, les dégâts, les transitions de carte et les
bonus d'équipement. Il constitue leur complément technique actuel ; les
règles confirmées ci-dessous restent conservées et ses nouvelles valeurs
attendent la revue individuelle demandée par le joueur.

## Règles confirmées

- Chaque actif secondaire s'achète une seule fois, pour un point.
- Chaque passif possède dix rangs appris, à un point par rang.
- Depuis AR-SEC01, confirmé le 7 octobre, l'actif associé doit être appris
  avant d'acheter ses passifs. Tous ses passifs sont ensuite accessibles
  sans autre palier. Leur achat ne donne pas l'actif gratuitement.
- Aucun seuil de points, palier obligatoire ou minimum de douze compétences
  ne s'applique aux arbres secondaires. Tous leurs passifs restent appris
  et actifs sans emplacement à équiper, selon leurs conditions d'effet.
- Poser une mine ou lancer une grenade utilise uniquement de l'énergie.
  Ces actifs ne consomment pas un objet d'inventaire. Les armes lance-grenades
  gardent leur gestion normale des munitions.
- Les effets poison, glace, aire et nuage appartiennent aux mines. Les
  grenades suivent leur liste propre : explosive, aveuglante, sonore,
  amélioration de portée et amélioration de dégâts.
- Avant la pose d'une mine ou le lancer d'une grenade, quel que soit son
  type, le jeu montre un aperçu de sa zone d'action. La grenade sonore
  a une zone d'attraction plus grande que les autres grenades et attire
  les monstres qui l'entendent vers son centre.

Les arbres principaux sont désormais accessibles sans paliers ni seuils
de niveau ou de points investis, conformément à AR-ACCESS01. Les lignes
organisent les compétences visuellement. Les règles communes et l'arbre
Armes lourdes prévu ensuite sont décrits dans la
[revue des arbres](REVUE_ARBRES_COMPETENCES_AGENT.md).

## Lecture et validation

Les citations sont des textes proposés au joueur. Les effets explicitement
demandés ou précisés par le joueur sont indiqués comme confirmés. Les
formules, noms, coûts, durées non fixées et conditions complémentaires
sont des propositions à valider.

Dans les barèmes proposés, n désigne le rang du passif. La prise en compte
des bonus d'objets au-delà de dix et leur éventuel effet sur la limite de
mines font l'objet d'une question en cours. Les exemples aux rangs 1 et
10 n'engagent pas encore cette règle d'équipement.

Les achats de SF01 et SF02 nécessitent SF00 ; ceux de SM01 à SM05
nécessitent SM00 ; ceux de SG01 à SG04 nécessitent SG00. SG04 exige donc
l'apprentissage du lancer manuel avant son achat, même lorsque le joueur
veut renforcer une arme lance-grenades. Son effet sur ces armes est conservé.
Apprendre un type de mine ouvre ce choix ; cela n'impose pas ce type à
toutes les prochaines mines.

## Furtivité

### SF00 Furtivité

> Activez la furtivité pour vous déplacer discrètement.

**Confirmé :** actif acheté une seule fois ; permet d'activer un mode
furtif. Ce mode dépense de l'énergie à chaque pas, avec SF01 pour réduire
ce coût et SF02 pour réduire la détection.

**Proposé :** activation et désactivation prennent une action ordinaire.
L'énergie est débitée lors d'un pas réussi ; un déplacement refusé n'en
consomme pas. Le coût de base par pas, les effets de base sur la discrétion,
la réaction en cas d'énergie insuffisante et les actions qui interrompent
le mode restent à préciser. Aucun coût d'entretien pendant un menu ou une
attente n'est ajouté implicitement.

La coordination entre régénération et dépense devra être vérifiée dans
les essais chiffrés. L'Agilité est proposée comme attribut de discrétion ;
son coefficient reste à valider avec les règles de perception.

### SF01 Économie de mouvement furtif

> Vos implants ajustent votre camouflage pour limiter l’énergie consommée à chaque pas.

**Confirmé, révisé le 8 octobre sous SF01-ENERGY01 :** réduit le coût de chaque pas furtif ; au rang 10, un pas coûte un point d'énergie. Le coût ne devient jamais nul, y compris avec des rangs supplémentaires issus de l'équipement. L'ancienne règle de gratuité au rang 10 est historique.

Chaque rang appris coûte un point de compétence, avec dix rangs appris maximum et SF00 comme prérequis. Le coût d'achat reste distinct de l'énergie dépensée en marchant. Coûts intermédiaires, courbe et arrondis restent à revoir avec le coût de base proposé de SF00 ; les anciennes formules et exemples figurent dans le snapshot de la revue précédente.

Ce passif ne réduit pas automatiquement le coût d'une mine, d'une grenade ou d'un autre pouvoir. Le minimum d'une énergie concerne les pas furtifs, sans ajouter d'entretien à l'arrêt ni modifier le coût d'activation.

**Fiche individuelle validée sous SF01-ACCEPT01 :** [SF01-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-SF01-02.json). RP, coût d’une énergie aux rangs effectifs onze/douze, prérequis Furtivité, marqueur du coût minimal et direction de mise à jour dans sa fiche sont validés ; courbe intermédiaire et rendu natif restent à revoir.

### SF02 Discrétion renforcée

> Votre camouflage brouille vos contours jusqu’à rendre votre présence presque imperceptible.

**Confirmé :** réduit la zone de détection des PNJ envers le joueur furtif,
jusqu'à le rendre presque indétectable au rang 10.

**Proposition à revoir :** appliquer un facteur 0,8 puissance n à leur
portée de repérage visuel de cet agent furtif. Elle reste à environ 80 %
au rang 1 et 10,7 % au rang 10. Les obstacles et les conditions de visibilité
restent pris en compte. Un voisin immédiat peut encore repérer l'agent ;
ce n'est pas une invisibilité absolue.

Le canal visuel, le bruit des pas et les capteurs éventuels devront être
distingués avant intégration. Ce passif ne doit pas réduire arbitrairement
la perception des PNJ envers tous les autres acteurs, ni effacer leur
mémoire d'une position déjà repérée. Ces conditions sont proposées.

**Fiche individuelle validée sous SF02-ACCEPT01 :** [SF02-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-SF02-02.json). Exemple aux rangs effectifs onze/douze : distance de repérage d’une case, minimum proposé déjà atteint. Texte, distances affichées, marqueur du minimum, condition sonore et direction de mise à jour dans Furtivité sont validés ; autres profils, courbe complète et rendu natif restent à revoir.

## Mines

### SM00 Pose de mine

> Vous déposez une charge piégée qui explose au passage d’un ennemi.

**Confirmé :** actif acheté une seule fois ; mine de feu par défaut.
Elle met un tour à s'armer. Une seule mine par case. Poison et glace sont
des choix accessibles grâce à SM02 et SM03, sélectionnés avant la pose.
Poser une mine coûte de l'énergie, sans consommer un objet d'inventaire.

**Proposé :** une mine simultanée sans SM01, une action ordinaire pour
la poser sur une case libre et visible à proximité. Coût, dégâts de base,
rayon de base et portée de pose restent à proposer.

L'ordre de résolution proposé est : pose au tour T, état non armé durant
la résolution de ce tour, armement lorsque le compteur atteint T + 1.
Le déclenchement proposé est le contact d'un hostile avec sa case une
fois armée. Le cas d'un hostile déjà dessus à l'armement doit être testé.
Une mine ne reçoit pas implicitement un déclencheur à grande distance.

**Confirmé :** dépasser la capacité en posant une nouvelle mine supprime
la plus ancienne ; changer de couche supprime toutes les mines du joueur.
**Précision proposée :** cette suppression ne fait pas exploser la mine
et ne rend pas d'énergie. Toutes les mines encore posées, armées ou non,
comptent dans la capacité. Une mine explosée n'en fait plus partie.

La validation de la case, de l'énergie et du choix précède la suppression
de l'ancienne mine : une pose refusée ne retire pas une mine existante,
ne dépense rien et ne prend pas de tour. Le changement de couche doit
nettoyer également les mines non armées et leurs minuteries.

Les transitions de zone ou de portail dans une même couche, et le devenir
des zones déjà laissées par une explosion, restent à définir séparément.

**Fiche individuelle validée sous SM00-ACCEPT01 :** [SM00-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-SM00-02.json). Actif unique sans passifs, Intelligence dix : treize dégâts de feu, rayon et portée d’une case, une mine simultanée, dix énergie, recharge d’un tour et armement après un tour. RP, chiffres affichés, détails de pose, dégâts possibles aux familiers et direction visuelle sont validés. Le voyant rouge clignote lorsque la mine est armée, à la demande explicite du joueur sous SM00-BLINK01 ; cadence et rendu natif restent à revoir. Les aperçus et autres règles déjà confirmées restent conservés.

### SM01 Réseau de mines

> Vos implants coordonnent plusieurs charges pour transformer le terrain en champ de mines.

**Confirmé sous SM01-GROW01 :** cinq mines simultanées au rang dix, puis une capacité qui continue d’augmenter grâce aux rangs bonus d’équipement. Cinq n’est pas un plafond global. Une mine par case et remplacement de la plus ancienne au dépassement restent confirmés.

**Progression proposée :** capacité = 1 + ceil(n / 3), soit deux aux rangs un à trois, trois aux rangs quatre à six, quatre aux rangs sept à neuf et cinq au rang dix. Aux rangs onze/douze, cinq mines sont validées ; six au rang treize reste un résultat proposé de cette courbe, dont la cadence non montrée doit être revue. Sans le passif, la capacité reste une mine, conformément à SM00-ACCEPT01.

**Fiche individuelle validée sous SM01-ACCEPT01 :** [SM01-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-SM01-02.json). Texte, capacité affichée aux rangs onze/douze, marqueur du prochain rang et direction d’affichage dans Pose de mine sont validés. Courbe complète, cas non montrés et rendu natif restent à revoir.

### SM02 Mines de poison

> Vos mines dispersent des toxines à chaque explosion.

**Confirmé :** apprendre ce passif ouvre le type poison au moment de poser une mine. Sous SM02-POISON01, l’explosion applique également Empoisonné pendant plusieurs tours, indépendamment de SM05. L’ancienne dépendance réservant tous les dégâts périodiques à SM05 est remplacée pour ce point.

**Progression proposée :** bonus de variante de 5 × (n − 1) pour cent ; explosion sur base douze, empoisonnement par tour sur base deux, avec le multiplicateur technique et un seul arrondi par composante. Durée proposée d’Empoisonné : 3 + floor((n − 1)/5) tours. Cumul, rafraîchissement et interactions avec les zones restent à revoir.

**Nouvelle proposition individuelle révisée :** [SM02-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-SM02-03.json). Avec Intelligence dix, rang onze : bonus de cinquante pour cent et dix-neuf dégâts à l’explosion ; prochain rang douze : cinquante-cinq pour cent et vingt dégâts. Empoisonné proposé aux deux rangs : trois dégâts par tour pendant cinq tours, conservés quand la cible s’éloigne. RP, chiffres, durée précise et direction d’explosion verte attendent validation ; voyant rouge armé clignotant et règles déjà confirmées conservés. SM05 ajoute une zone persistante au sol, distincte du statut sur la cible.

**Note d’équilibrage sous SM02-BAL01 :** le joueur juge les trois dégâts par tour, soit quinze sur cinq tours avant résistances, trop faibles ou peu séduisants au rang présenté. Les valeurs sont conservées uniquement comme repères provisoires ; leur équilibrage est reporté à la définition des statistiques réelles du jeu. Sous BAL-REF01, Diablo II devient une référence demandée pour la distribution des attributs, leurs effets et l’échelle des PV et statistiques des monstres. Aucun nouveau montant ou multiplicateur n’est fixé ; cette remarque ne valide pas toute la fiche, ses autres règles ou son visuel.

### SM03 Mines de glace

> Vous pouvez choisir des dégâts de glace pour vos mines.

**Confirmé :** apprendre ce passif ouvre le type glace à la pose. Avec
SM05, la zone persistante immobilise les monstres, qui peuvent encore
attaquer et effectuer leurs autres actions.

**Progression proposée :** le premier rang ouvre le choix ; les suivants
renforcent les dégâts de froid de 5 × (n − 1) pour cent, soit +45 % au
rang 10 par rapport à cette variante au rang 1. Le choix glace seul ne
donne pas implicitement un étourdissement qui supprimerait toutes les actions.

### SM04 Explosion étendue

> Les explosions de vos mines couvrent une zone plus large.

**Confirmé :** augmente l'aire d'effet. **Progression proposée :** ajouter
ceil(n / 4) cases au rayon de base, soit une aux rangs 1 à 4, deux aux
rangs 5 à 8 et trois au rang 10. Le rayon de base reste à fixer.

La forme proposée est radiale, avec distance euclidienne et obstacles.
Le nuage de SM05 reprend cette aire. L'aperçu doit respecter les cases et
acteurs déjà connus ; les dégâts et les éventuels effets sur les alliés
devront être validés avant intégration.

### SM05 Résidu persistant

> Après leur explosion, vos mines laissent une zone qui reste active.

**Distinction sous SM02-POISON01 :** Résidu persistant laisse une zone au sol après l’explosion. Le statut Empoisonné appliqué par SM02 dure sur la cible et ne nécessite pas ce passif. Le choix feu, poison ou glace continue à déterminer le type de zone.

**Confirmé :** l'effet continue après l'explosion ; investir des points
augmente sa durée. Le type choisi pour la mine détermine la zone :

| Mine choisie | Zone persistante confirmée |
| --- | --- |
| Poison | Nuage de poison |
| Glace | Zone qui immobilise les monstres, sans supprimer leurs autres actions |
| Feu | Zone de feu |

**Durée proposée :** 2 + n tours, soit trois au rang 1 et douze au rang 10.
La zone reçoit une résolution par tour de référence, sans effet déclenché
par l'ouverture d'un menu. Pour la glace, l'immobilisation est proposée
tant que le monstre occupe une case de la zone active ; la fin de zone
libère son mouvement. Le mouvement forcé reste à définir.

Les dégâts périodiques de feu et poison, leur premier déclenchement, le
cumul entre zones, les résistances et les effets sur les alliés restent à
proposer. Une zone est distincte d'une mine encore posée. Les paramètres
seraient fixés à la pose pour éviter qu'un changement d'équipement transforme
rétroactivement toutes les mines et zones existantes ; cette convention
attend validation.

## Grenades

### SG00 Lancer de grenade

> Lancez une grenade et choisissez une variante parmi celles apprises.

**Confirmé :** actif acheté une seule fois, grenade explosive de base ;
les passifs permettent de choisir une variante. Le lancer coûte uniquement
de l'énergie. Les variantes secondaires retenues sont aveuglante et sonore.

**Proposé :** une action ordinaire, trajectoire respectant les obstacles,
cible visible et à portée. Coût, portée de base, aire, dégâts et délai de
déclenchement restent à fixer. L'interface présente le choix avant la
confirmation ; une annulation ou une cible illégale ne dépense rien.

Les variantes aveuglante et sonore sont proposées comme alternatives
utilitaires à la variante explosive. Elles ne gagnent pas implicitement
les dégâts de l'explosive. Il n'y a pas de passif poison, glace, aire ou
nuage ajouté à cet arbre.

### SG01 Grenade aveuglante

> Vous pouvez lancer une grenade qui réduit la précision des ennemis.

**Confirmé :** les monstres pris dans sa zone perdent en précision pendant
une durée qui augmente avec les points investis.

**Proposé :** le premier rang ouvre cette variante ; précision multipliée
par 0,7, pour une durée de 1 + n tours, soit deux au rang 1 et onze au
rang 10. Les chances de toucher conservent leurs bornes ordinaires.
Les murs limitent les acteurs touchés. Cette variante ne retire pas les
actions et ne provoque pas un étourdissement.

Le cumul de plusieurs aveuglements et les éventuelles résistances restent
à valider. La portée du lancer bénéficie de SG03.

### SG02 Grenade sonore

> Vous pouvez lancer une grenade qui attire les ennemis par son bruit.

**Confirmé :** le volume et la portée sonore augmentent avec les points
investis. Le premier rang ouvre cette variante.

**Barème proposé :** intensité sonore = 6 + 2n, soit huit au rang 1 et
26 au rang 10. Une émission durant deux tours est proposée. Sa portée
réelle vient de la propagation sonore du moteur, avec murs et atténuation,
et non d'un cercle qui ignorerait le terrain.

Les monstres qui l'entendent peuvent rechercher la source. La priorité
de cette recherche face à une cible déjà vue ou à un autre bruit devra
être validée ; ce pouvoir ne fait pas traverser les murs aux monstres.
La portée du lancer bénéficie de SG03 ; la portée d'écoute vient de SG02.

**Précisions confirmées par le joueur :** le point d'impact, au centre de
la zone sonore, est leur destination d'attraction. La portée sonore doit
être plus grande que les zones des autres variantes de grenade. Le barème
de volume sera ajusté pour respecter cette relation, dès le premier rang.
Les obstacles modifient la forme et la portée réelles du son ; la comparaison
des rayons de base se fait en terrain dégagé et la destination est atteinte
par un chemin franchissable.

### SG03 Portée de lancer

> Vous lancez vos grenades plus loin.

**Confirmé :** augmente la portée de lancer. **Progression proposée :**
portée supplémentaire = 1 + floor((n − 1) / 3), soit une case au rang 1
et quatre au rang 10, ajoutées à la portée de base de SG00.

Ce passif concerne les lancers de l'actif secondaire, quelle que soit
la variante. Il ne change pas implicitement la portée d'une arme lance-grenades.
La trajectoire doit rester légale et la cible dans les limites d'observation.

### SG04 Puissance des grenades

> Vos grenades et vos armes lance-grenades infligent plus de dégâts.

**Confirmé :** renforce les dégâts des grenades lancées et des armes
lance-grenades du personnage. **Progression proposée :** +10n pour cent,
soit +10 % au rang 1 et +100 % au rang 10, avant les défenses.

Ce bonus renforce les dégâts présents dans une attaque ; il ne transforme
pas une grenade sonore ou aveuglante sans dégâts en attaque explosive.
Il ne remplace pas les passifs de durée, de volume ou de portée. Les
interactions avec les bonus d'armes et d'autres compétences seront à valider.

## Aperçus des mines et grenades

### SP00 Prévisualisation des mines et grenades

> Vérifiez la zone d'action avant de confirmer.

**Confirmé :** aperçu lors du choix de pose ou de lancer, pour toutes les
variantes, avec une zone plus vaste pour l'attraction sonore. Ce n'est
pas une compétence à acheter et cet aperçu ne coûte aucun point.

**Fonctionnement proposé :** l'agent choisit le type et la case cible,
puis l'aperçu suit cette case avant la confirmation. Il montre les cases
affectées par les paramètres actuels, dont SM04 pour l'aire des mines,
SM05 pour leur zone persistante, et SG02 pour la portée sonore. Changer
le type ou les bonus applicables doit recalculer l'aperçu.

La grenade sonore montre le point d'attraction au centre et la portée
réelle de l'écoute : la propagation du moteur tient compte des murs,
portes et de l'atténuation. Le rayon de lancer augmenté par SG03 est
distinct de cette zone d'effet. Les autres grenades et mines montrent
leur zone d'explosion ou d'altération, arrêtée par les obstacles selon
leurs règles. La mine indique aussi qu'elle ne sera armée qu'après un tour.

Seuls les terrains connus et acteurs déjà observés peuvent être détaillés
dans l'aperçu. Une zone ne doit pas révéler des ennemis cachés, confirmer
un obstacle inconnu ou promettre qu'un monstre invisible entendra le son.
Le rendu doit distinguer une cible de pose ou de lancer illégale d'une
case réellement affectée. La gestion des portions non connues est à revoir
avec l'interface, sans modifier la propagation réelle après confirmation.

Déplacer le curseur, consulter cet aperçu, changer de variante ou annuler
ne prend pas de tour, ne dépense pas d'énergie, ne pose pas de mine, ne
supprime pas l'ancienne mine et ne produit aucun bruit ou effet réel.
Les couleurs, contours, marqueur de centre et formulations exactes devront
être présentés à taille native avant intégration. Cette revue ne crée pas
encore ce rendu ni un sprite de mine ou de grenade.

### SP01 Inspection d'une mine posée

> Affichez la zone d'action de cette mine.

**Confirmé :** lorsque la case contient une mine et rien d'autre à inspecter,
passer la souris dessus ou y placer le curseur du jeu affiche son rayon
d'action. Ce comportement complète l'aperçu avant pose ; il ne demande
pas de point de compétence.

**Proposé :** afficher la zone réelle de cette mine et son type, avec ses
paramètres de pose, plutôt que ceux d'une nouvelle mine calculée depuis
l'équipement actuel. Les murs et le terrain limitent la zone selon les
mêmes règles que son explosion et son éventuel nuage. Son état armé ou
en attente d'armement reste lisible.

Quand une autre cible d'inspection connue partage la case, le sélecteur
SP02 remplace l'ancienne proposition de priorité automatique au personnage
ou à l'objet. Les conditions utilisent les informations connues du joueur,
sans faire apparaître une mine inconnue ni révéler un occupant caché par
la présence ou l'absence de l'aperçu. Changer de case retire cet affichage.
Le survol et le déplacement du curseur ne prennent pas de tour, ne coûtent
pas d'énergie et n'arment ni ne déclenchent la mine. Les détails du rendu
restent à présenter avant intégration.

### SP02 Choix d'inspection sur une case partagée

> Choisissez ce que vous voulez examiner sur cette case.

**Confirmé :** si une case contient une mine et autre chose, le survol à
la souris ou au curseur du jeu ouvre un petit tooltip permettant de choisir
les informations à afficher. Le choix revient au joueur ; l'autre occupant
ne masque plus automatiquement l'inspection de la mine.

**Interaction proposée :** le tooltip liste les cibles connues de cette
case, par exemple la mine, un personnage observé et les objets visibles.
Choisir la mine affiche ses informations et son rayon d'action selon SP01.
Choisir une autre cible affiche ses informations habituelles. La sélection
est utilisable à la souris et avec les commandes de navigation et validation
du tooltip ; les touches exactes seront à revoir avec les contrôles.

Le passage entre les choix ne fait pas avancer le tour, ne dépense pas
d'énergie et ne déclenche aucune action sur la case. Le choix d'une autre
cible masque l'aire de la mine pour rendre claire l'information examinée.
Sortir de la case ou fermer l'inspection retire le tooltip et la zone.

Le contenu du sélecteur utilise seulement ce que le joueur connaît. Un
acteur caché ou une mine inconnue n'ajoute pas d'entrée et ne doit pas
modifier le comportement visible du tooltip. Une mine seule reste inspectée
directement selon SP01. La disposition, les libellés exacts, le focus et
les contrôles attendent un aperçu avant intégration.

## Attributs, observations et prochaine revue

Intelligence est proposée pour la puissance des mines et grenades issues
de ces actifs ; les armes lance-grenades conserveraient leurs contributions
d'attributs d'arme. Agilité est proposée pour la furtivité. Les coefficients
et la nature technologique ou magique des actifs restent à définir ;
une dépense d'énergie commune ne rend pas automatiquement un pouvoir magique.

L'observation sonore existante passe par `audible_incident_for` et
`sound_reaches_on_map`. Les aperçus de détection, mines, explosions et
nuages doivent utiliser les mêmes obstacles et le même temps que leurs
effets, sans révéler les acteurs cachés.

Les textes, barèmes et nouveaux rendus attendent validation. Les essais
devront notamment vérifier le pas à une énergie au rang 10 et son minimum
avec les bonus d’équipement, le refus d'achat
d'un passif avant son actif associé, l'accès libre après cet apprentissage,
l'armement au bon tour et le remplacement atomique d'une mine,
le nettoyage au changement de couche, les combinaisons de types et zones,
le gel laissant les attaques possibles, le son et les dégâts des lance-grenades.
Les aperçus et le choix d'inspection doivent aussi être vérifiés à la souris
et au curseur du jeu, avec une mine seule, une case également occupée,
plusieurs objets, une mine non armée et un changement d'équipement après
pose, sans modifier la partie. Un occupant caché ne doit pas être révélé
par le nombre de choix ni empêcher l'inspection de la mine connue.
Les illustrations et animations suivront la
[revue artistique](DIRECTION_ARTISTIQUE_TILES.md) avant intégration.
