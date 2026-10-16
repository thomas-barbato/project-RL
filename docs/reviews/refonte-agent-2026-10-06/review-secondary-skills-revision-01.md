# Revue des compétences secondaires

6 octobre 2026. Le joueur ajoute des arbres secondaires de furtivité,
mines et grenades. Ils sont accessibles à toutes les classes et utilisent
les points de compétence ordinaires. Cette revue enregistre ses règles
et propose des textes et progressions ; aucune compétence de ce document
n'est encore intégrée au jeu.

## Règles confirmées

- Chaque actif secondaire s'achète une seule fois, pour un point.
- Chaque passif possède dix rangs appris, à un point par rang.
- Tous les passifs sont achetables immédiatement, même avant l'actif
  associé. Leur achat ne donne pas l'actif gratuitement.
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

Les arbres principaux conservent leurs paliers et leur minimum de quatre
lignes de trois compétences par branche spécialisée. Ils sont décrits
dans la [revue des arbres principaux](REVUE_ARBRES_COMPETENCES_AGENT.md).

## Lecture et validation

Les citations sont des textes proposés au joueur. Les effets explicitement
demandés ou précisés par le joueur sont indiqués comme confirmés. Les
formules, noms, coûts, durées non fixées et conditions complémentaires
sont des propositions à valider.

Dans les barèmes proposés, n désigne le rang du passif. La prise en compte
des bonus d'objets au-delà de dix et leur éventuel effet sur la limite de
mines font l'objet d'une question en cours. Les exemples aux rangs 1 et
10 n'engagent pas encore cette règle d'équipement.

Les passifs apprenables avant leur actif peuvent rester sans effet tant
que leur usage n'est pas disponible. Le passif de dégâts des grenades
peut déjà renforcer une arme lance-grenades sans apprendre le lancer manuel.
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

> Vos déplacements furtifs consomment moins d'énergie.

**Confirmé :** réduit le coût de chaque pas furtif ; ce coût est nul au
rang 10. **Progression proposée :** coût final =
ceil(coût de base × max(0, 1 − n / 10)), après les autres modificateurs.
Le coût ne devient jamais négatif. À titre d'exemple, un coût de base de
10 donnerait 9 au rang 1, 5 au rang 5 et 0 au rang 10 ; ce nombre 10 est
un exemple de calcul, pas le coût de base retenu.

Le minimum d'énergie réservé aux pouvoirs à coût positif ne doit pas
ramener à un point le pas rendu gratuit au rang 10. Ce passif ne réduit
pas automatiquement le coût d'une mine, d'une grenade ou d'un autre pouvoir.

### SF02 Discrétion renforcée

> Les ennemis doivent être plus proches pour vous repérer en furtivité.

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

## Mines

### SM00 Pose de mine

> Posez une mine et choisissez son type de dégâts parmi ceux appris.

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

### SM01 Réseau de mines

> Vous pouvez maintenir davantage de mines au sol.

**Confirmé :** augmente le nombre de mines simultanées, jusqu'à cinq au
rang 10 ; une mine par case ; remplacer la plus ancienne au dépassement.

**Progression proposée :** capacité = 1 + ceil(n / 3), soit deux aux rangs
1 à 3, trois aux rangs 4 à 6, quatre aux rangs 7 à 9 et cinq au rang 10.
Sans le passif, la capacité proposée reste une mine. La capacité au-delà
du rang 10 par équipement reste à trancher.

### SM02 Mines de poison

> Vous pouvez choisir des dégâts de poison pour vos mines.

**Confirmé :** apprendre ce passif ouvre le type poison au moment de poser
une mine. **Progression proposée :** ce premier rang ouvre le choix ; les
rangs suivants renforcent ses dégâts de poison de 5 × (n − 1) pour cent,
soit +45 % au rang 10 par rapport à cette variante au rang 1.

Un effet persistant de poison est apporté par SM05. Sa durée ne vient
pas automatiquement du choix du type de dégâts.

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

Un personnage observé ou un objet visible sur cette case garde la priorité
d'inspection. Les conditions utilisent les informations connues du joueur,
sans faire apparaître une mine inconnue ni révéler un occupant caché par
la présence ou l'absence de l'aperçu. Changer de case retire cet affichage.
Le survol et le déplacement du curseur ne prennent pas de tour, ne coûtent
pas d'énergie et n'arment ni ne déclenchent la mine. Les détails du rendu
restent à présenter avant intégration.

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
devront notamment vérifier le pas gratuit au rang 10, les passifs appris
avant l'actif, l'armement au bon tour, le remplacement atomique d'une mine,
le nettoyage au changement de couche, les combinaisons de types et zones,
le gel laissant les attaques possibles, le son et les dégâts des lance-grenades.
Les deux modes d'aperçu doivent aussi être vérifiés à la souris et au
curseur du jeu, avec une mine seule, une case également occupée, une mine
non armée et un changement d'équipement après pose, sans modifier la partie.
Les illustrations et animations suivront la
[revue artistique](DIRECTION_ARTISTIQUE_TILES.md) avant intégration.
