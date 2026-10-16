# Revue animée des compétences de l’agent

**15 octobre 2026 · AGENT-CADENCE01 validé :** les sorts, techniques physiques
offensives et invocations du catalogue sont réutilisables dès l’action suivante.
Les anciennes recharges des exemples et captures ci-dessous sont historiques.
Les durées des effets et la cadence autonome des drones restent conservées.
Les lignes de recharge sont retirées des fiches descriptives, y compris du
bloc Prochain rang. La nouvelle règle ne valide ni ne crée un nouveau dessin.
Le premier lot jouable A01 est suivi dans `INTEGRATION_COMPETENCES_AGENT.md`.

11 octobre 2026. Le joueur demande de voir toutes les animations avant de
les accepter, avec des validations par lots après chaque aperçu. La procédure
reprend celle des icônes : candidats numérotés, variantes identifiables, rendu
animé à taille native, puis accord explicite avant intégration au combat.

Une icône déjà acceptée ne valide pas l’animation de sa compétence. Les noms,
textes et règles chiffrées déjà acceptés gardent leurs identifiants. Les passifs
qui modifient un rendu seront montrés avec l’action ou l’événement concerné.

## ANIM-S01 — premier lot en attente de revue

01 `ME02-A01` **Fauchage** : un arc de lame court traverse les trois cases
devant l’agent ; les trois cibles reçoivent de petits éclats d’impact.

02 `FE01-A01` **Braise** : projectile incandescent avec traînée sombre,
impact bref, point incandescent et quelques braises sur la cible brûlée.

03 `FR01-A01` **Aiguille de givre** : projectile étroit de glace,
éclats à l’impact, givre autour des pieds de la cible gelée.

Chaque scène existe en 32 et 64 px natifs. La galerie propose pause, reprise,
relecture et vitesse réduite. Les GIF et les séquences PNG viennent du rendu
Macroquad du prototype ; les scènes sont extraites sans redimensionnement.
Les acteurs, leurs armes, les décors et les icônes acceptés sont conservés.

Le Gel est volontairement déclenché pour examiner son apparence : ce choix
ne remplace pas sa probabilité acceptée. Le nom de la compétence flotte
brièvement au-dessus de l’agent, selon la règle déjà retenue. Les indicateurs
de Brûlure et Gel et les trois nouveaux rendus attendent leur accord visuel.

## LAB-S01 — interface de laboratoire en attente de revue séparée

Un point d’entrée debug isolé présente les trois animations. Il prépare le
choix de compétence, les rangs appris et les bonus d’objet, le niveau et les
six attributs, la respécialisation, les recharges libres et l’avance d’un tour.
Les textes, commandes et disposition de cette interface sont proposés.

Ce prototype n’utilise pas `GameState` et ne lance pas de combat : les réglages
de niveau et d’attributs ne calculent pas encore les dégâts. Les seules courbes
affichées sont les coûts, recharges et durées provenant du catalogue existant.
Relire une animation ou modifier sa vitesse n’avance aucun tour et ne remet
pas ses temporisations à zéro. L’avance d’un tour seule décrémente les timers.

Entrée debug : `--apercu-laboratoire-competences`. Capture reproductible :
`--ui-cold-agent-skill-animations <nouveau dossier>`. Le lancement habituel,
la salle de test existante, la campagne et les sauvegardes sont inchangés.

Preuves et lecteur animé :
`artifacts/refonte-agent-2026-10-06/skill-animation-review-01/`.
Les validations de `ME02-A01`, `FE01-A01`, `FR01-A01` et de `LAB-S01` sont
distinctes et restent en attente. L’intégration des effets au moteur suivra
les accords ; aucune approbation n’est déduite du choix de cette procédure.

## ANIM-S01 — révision 02, intensité et priorité au 64 px

Après l’aperçu A01, le joueur demande des animations « beaucoup plus marquées »
et « qu’il en mette pleins les yeux ». Il précise de les concevoir pour des
cases de 64 px, avec disparition du 32 px à terme. Les animations A01 sont
à reprendre ; elles ne sont pas validées. Leurs captures et sources restent
conservées dans la première revue pour comparaison.

Les numéros 01, 02 et 03 sont conservés avec les variantes `ME02-A02`,
`FE01-A02` et `FR01-A02`. Le croissant de Fauchage est ample et lumineux,
les trois impacts projettent des éclats. Braise laisse une longue traînée de
feu et une gerbe à l’impact, suivies de flammes autour de la cible. L’aiguille
de givre laisse un sillage, éclate en fragments et forme des cristaux aux pieds.

Les halos et gerbes d’impact ne créent pas de nouvelle zone de dégâts.
Ces effets plus présents sont des propositions, avec particules déterministes
sans consommation du hasard du jeu. Les règles, sprites et paramètres acceptés
ne sont pas modifiés. Le second aperçu utilise uniquement le 64 px natif ;
cela ne supprime pas encore le réglage 32 px du jeu courant.

Preuves et galerie locale, avec lecture, pause et ralentissement :
`artifacts/refonte-agent-2026-10-06/skill-animation-review-02/`.
Cette galerie fonctionne avec ses images locales, sans serveur. Les trois
variantes A02 et la proposition de laboratoire attendent leur revue.

## ANIM-S01 — trois animations A02 acceptées ; effets de sang à préciser

Le joueur répond « c’est très bien, n’oublie pas aussi les effets de sang sur
les blessures ». Cet accord valide les trois animations présentées :
01 `ME02-A02` Fauchage, 02 `FE01-A02` Braise et 03 `FR01-A02` Aiguille de givre.
Leurs GIF exacts et leurs empreintes sont enregistrés dans
`skill-animation-review-02/visual-acceptance-01.json`. L’interface de laboratoire
`LAB-S01` attend toujours une revue séparée. Les animations restent dans le
prototype ; cet accord visuel ne raccorde pas les compétences au combat.

La demande d’effets de sang concerne la présentation des blessures, sans
ajouter automatiquement le statut mécanique Saignement. Les machines restent
sans sang, selon la direction artistique existante. Tout nouveau rendu ou
modification sera montré en mouvement avant intégration.

Le joueur demande ensuite un avis sur l’intérêt du sang pour les sorts de
feu et de glace. La proposition est de réserver le sang aux blessures
physiques et aux pouvoirs occultes qui le justifient, et de donner au feu
et au froid leurs propres effets élémentaires. Pour une attaque mêlant un
impact physique et un élément, les deux rendus peuvent être associés.
Cette répartition est en discussion : la question ne vaut pas validation
de toutes ses applications ou de nouveaux dessins.

## ANIM-BLOOD01 — répartition des effets de sang validée

Le joueur répond « parfait :) » à la proposition précédente. La règle
artistique est acceptée : les blessures physiques (mêlée, balles, harpons)
produisent du sang sur les êtres de chair. Le feu utilise des flammes,
de la fumée et des braises ; le froid utilise du givre et des éclats de glace.
Les pouvoirs occultes peuvent produire du sang lorsqu’ils déchirent la chair
ou drainent la vie. Une arme combinant impact physique et dégâts élémentaires
peut associer les deux rendus. Les machines ne produisent pas de sang.

Cet accord porte sur la répartition artistique, sans ajout automatique du
statut Saignement. Les nouveaux rendus restent soumis à la revue animée par
lots avant leur intégration. Accord enregistré dans
`docs/reviews/refonte-agent-2026-10-06/skill-animation-review-02/blood-policy-acceptance-01.json`.

## ANIM-S02 — mêlée et collisions, en attente de revue

Le deuxième lot poursuit la revue en 64 px natifs avec les noms et effets
existants. Les animations A02 acceptées du premier lot sont conservées.

04 `B01-A01` **Frappe de rupture** : frappe ample en acier lumineux, impact
franc et projection de sang sur une cible de chair.

05 `ME03-A01` **Percée** : frappe étroite et dirigée, éclats de plaques et
blessure sanglante. Les éclats servent l’impact ; ils ne créent pas de nouvel
affaiblissement d’armure.

06 `ME04-A01-MUR` **Repousser, collision contre un mur** : la cible glisse
sur les cases libres et s’arrête avant la cloison. La collision montre un
second impact, des fragments métalliques et du sang provenant de la cible.

07 `ME04-A01-CIBLE` **Repousser, collision contre un monstre** : le recul
s’arrête avant le second monstre ; la collision touche visuellement les
deux créatures. Le petit mouvement de réaction du monstre percuté est un
bump de présentation, sans déplacement supplémentaire de case.

Les deux exemples de recul utilisent le rang effectif 11 : l’obstacle
interrompt le maximum de trois cases après deux cases libres. Les marques
au sol et les projections illustrent les blessures selon `ANIM-BLOOD01`,
sans ajouter le statut Saignement ou de règle de persistance sauvegardée.

Les quatre rendus attendent une validation explicite. Le laboratoire peut
rejouer ce lot avec `--apercu-laboratoire-competences --lot-melee`. L’avance
d’un tour règle les recharges ; la lecture des mouvements est indépendante.
Les dégâts réels restent à raccorder au moteur de combat.

La vue de laboratoire `LAB-S02`, à sélection fixe pour ce lot, est également
un prototype. Sa disposition et ses commandes attendent une revue séparée ;
l’accord sur les quatre animations ne validera pas cette interface.

Captures natives, GIF, lecteur local avec pause et ralentissement :
`artifacts/refonte-agent-2026-10-06/skill-animation-review-03/`.

## ANIM-S02 — les quatre aperçus acceptés

Le joueur répond « tres bien :) » à la présentation des GIF 04 à 07.
L’accord visuel couvre `B01-A01`, `ME03-A01`, `ME04-A01-MUR` et
`ME04-A01-CIBLE`, y compris les impacts, le recul, les collisions et le sang
montrés. Les fichiers exacts et leurs empreintes sont enregistrés dans
`skill-animation-review-03/visual-acceptance-01.json`. Les règles de dégâts
restent celles du catalogue accepté. `LAB-S02` attend sa revue séparée.

## ANIM-S03 — Charge et Tourbillon proposés

08 `ME05-A01` **Charge** : ruée avec silhouettes de passage, poussière basse
et éclat sur la lame. L’agent finit sur la case libre adjacente à une cible
visible située quatre cases plus loin, puis porte un seul coup. L’impact
et la blessure montrent le sang de la cible de chair, sans recul forcé.

09 `ME06-A01` **Tourbillon** : un croissant de lame parcourt les huit voisins
de l’agent. Chaque impact produit des éclats et une blessure sanglante.
Le saignement de trois tours déjà accepté est illustré par des gouttes
persistantes. L’agent reste sur sa case et les monstres ne sont pas repoussés.

Le décalage visuel des impacts suit le passage de la lame ; il ne répartit
pas l’action sur plusieurs tours. La lecture reste indépendante des timers
de recharge et de statut. Le texte « Saignement · 3 tours » est un indicateur
du prototype, sans simulation des dégâts réels.

Les deux rendus attendent leur accord. La vue de laboratoire `LAB-S03`,
accessible avec `--apercu-laboratoire-competences --lot-assaut`, est proposée
séparément. Sprites, icônes et paramètres du catalogue restent conservés.

Captures, séquences PNG natives, GIF et lecteur local :
`artifacts/refonte-agent-2026-10-06/skill-animation-review-04/`.


### ANIM-S03 — Charge et Tourbillon acceptés

Le joueur répond « je valide oui :) » aux GIF 08 `ME05-A01` Charge et
09 `ME06-A01` Tourbillon. L’accord couvre les deux rendus exacts, y compris
les blessures et le saignement montrés. Empreintes dans
`skill-animation-review-04/visual-acceptance-01.json`. La vue `LAB-S03`
reste proposée séparément ; les règles chiffrées et le combat restent inchangés.

## ANIM-S04 — premiers sorts de Feu proposés

10 `FE02-A01` **Souffle de feu** : torrent turbulent en cône de 90°,
portée de cinq cases dans cet exemple de rang effectif 11. Le cône est
présent dès l'activation ; son écoulement ne constitue pas une progression
des dégâts d'une case à l'autre. Les flammes gardent leurs cœurs clairs,
leurs bords orange, de la fumée et des braises. Pas de sang.

11 `FE03-A01` **Mur de feu** : rideau de flammes sur cinq cases, traversable.
Une cible de la scène quitte le rideau et conserve les petites flammes de
la brûlure montrée. Ce trajet est un mouvement scénarisé du prototype,
sans nouvelle règle de déplacement ou de choix de trajet des monstres.

12 `FE04-A01` **Émanation soudaine** : fissures incandescentes sur la zone
de rayon un, puis gerbe ascendante, flash, braises et fumée. Le montage
annonce « Préparation · 1 tour », puis « +1 tour · explosion ». Le passage
visuel à 0,85 seconde représente un événement de tour scénarisé, jamais un
délai de jeu en secondes. Dans le laboratoire, l'explosion attend la
commande d'avance d'un tour. Elle ne laisse pas de nouvelle zone de feu.

Les brûlures sont des applications réussies choisies pour la présentation ;
le prototype ne tire aucune chance et ne calcule aucun dégât. Les durées
et recharges du catalogue restent exprimées en tours. Rejouer, ralentir
ou regarder l'animation ne les décrémente pas. L'exemple n'ajoute aucun
recul, destruction du terrain ou dommage aux alliés.

Ces trois rendus attendent un accord visuel avant intégration au combat.
La vue `LAB-S04`, avec `--apercu-laboratoire-competences --lot-feu`, est
proposée séparément. Sprites, icônes, règles et GIF déjà acceptés sont
protégés par empreintes.

Sources, captures finales natives et lecteur local avec pause :
`artifacts/refonte-agent-2026-10-06/skill-animation-review-05/`.

## ANIM-STYLE01 — proposition de flammes dessinées

Le joueur trouve les flammes procédurales d'ANIM-S04 éloignées du style
des personnages. Il demande un aperçu dessiné avant de décider d'une règle
artistique commune à toutes les animations. Cette règle reste à établir
après son avis ; aucun nouveau style n'est déclaré accepté ici.

Les numéros sont conservés : 10 `FE02-A02` Souffle de feu,
11 `FE03-A02` Mur de feu et 12 `FE04-A02` Émanation soudaine.
Les flammes utilisent deux planches transparentes de huit dessins,
générées avec Imagegen intégré. Le personnage en robe rouge accepté sert
de référence de contours et de masses de couleur, sans retouche du personnage.
Les planches sont découpées et alignées par leur point d'origine, avec
échelle constante ; aucune interpolation ou déformation des dessins.
Les contours restent irréguliers et les masses orange, jaunes et ivoire
portent l'effet. Les rubans, cônes de dégradé et halos mathématiques de la
proposition précédente ne servent pas à dessiner ces flammes.

La scène conserve ses cases de 64 px, ses cibles, ses positions et ses
effets prévus : mur traversable, brûlure après la sortie et préparation
d'un tour avant Émanation. La lecture utilise douze images dessinées par
seconde ; ce rythme ne devient ni vitesse de jeu ni délai d'action.
Les empreintes protègent les acteurs, icônes et GIF précédents.

Cette proposition reste isolée du combat. Les trois rendus et la vue
`LAB-STYLE01` attendent leur revue. Les sources, prompts exacts, exports,
montages animés et lecteur local sont dans
`artifacts/refonte-agent-2026-10-06/skill-animation-review-06/`.

## ANIM-OCC01 — Souffle de feu dans les couloirs et contre les obstacles

Le joueur demande un test animé dans un couloir ou avec un obstacle devant
l'agent. Cette revue conserve les dessins de flammes d'ANIM-STYLE01 et le
numéro 10 de FE02, avec trois variantes de situation :

- `FE02-A03-COULOIR` : couloir d'une case débouchant sur une pièce. Les cibles
  A, B et C sont exposées ; D est protégé par un angle fermé.
- `FE02-A03-CLOISON` : cloison continue devant l'agent. A est exposé ;
  B et C restent derrière la cloison et ne portent pas de flammes de brûlure.
- `FE02-A03-OBSTACLE` : bloc dans le cône. A et B sont exposés sur les côtés ;
  C est protégé derrière le bloc.

Le prototype calcule un cône discret vers la droite, de profondeur cinq
cases et d'angle 90° à ce rang d'exemple. Les cases doivent être libres
et avoir une ligne de vue depuis l'agent, avec la fonction existante
`has_line_of_sight` et les angles fermés bloquants. Cette discrétisation
précise est proposée pour revue, sans modification du catalogue ou du combat.
Les murs qui arrêtent FE02 restent une règle déjà définie.

Une seule liste de cases alimente l'aperçu de ciblage, le masque GPU du cône
et le choix des cibles montrées brûlées. Le dessin source est inchangé.
Le masque suit les cases ; un fondu intérieur d'environ deux pixels
adoucit les coupes sans dessiner de flamme sur une case exclue. La case
de l'agent accueille uniquement l'origine du dessin et ne devient pas
une case de dégâts. Aucun renforcement des dégâts ni détour des flammes
autour des murs n'est ajouté.

Le début de la boucle montre l'aperçu de ciblage, puis son lancement.
Les 0,7 seconde de présentation de cet aperçu ne deviennent pas un délai
de préparation du sort. La lecture ne fait avancer aucun tour. Les cibles
protégées restent volontairement visibles dans cette scène de comparaison ;
aucune révélation d'ennemi caché n'est ajoutée au jeu. Les brûlures montrées
sont des réussites scénarisées, sans jet aléatoire ni calcul de dégâts.

Les trois rendus, les textes du prototype et la vue de laboratoire
`LAB-OCC01` attendent leur revue. Lancement isolé avec
`--apercu-laboratoire-competences --lot-souffle-obstacles`.
La règle de style commune à toutes les animations reste à confirmer.
Sources, captures, contrôles de masquage et lecteur local :
`artifacts/refonte-agent-2026-10-06/skill-animation-review-07/`.

## ANIM-OCC02 — aperçu précis puis courant visuel continu

Le joueur explique que les frontières de cases coupent les flammes et
empêchent un résultat naturel près des obstacles. Il demande un aperçu de
la zone touchée avant le lancement, puis une animation continue qui épouse
le décor, comme un courant qui s'étale. Il confirme la synthèse de ce
principe par « c'est ca :) ». Les nouveaux rendus attendent leur propre avis.

Le numéro 10 de FE02 et les trois situations sont conservés :
`FE02-A04-COULOIR`, `FE02-A04-CLOISON` et `FE02-A04-OBSTACLE`.
L'aperçu de ciblage, les cases touchées et les cibles exposées/protégées
restent ceux du prototype précédent. L'animation n'utilise plus leur union
de carrés comme masque de découpe. Des noyaux arrondis qui se chevauchent
forment un champ visuel continu, guidé par la zone de gameplay. Leurs
franges peuvent dépasser une frontière de case libre. Des tests de segment
à l'échelle du pixel les empêchent de traverser les rectangles physiques
des murs. Le champ reste vide au centre des cases des cibles protégées.
Un premier essai à ombres triangulaires a été conservé en brouillon ;
le rendu soumis utilise ces courants arrondis sur les côtés des obstacles.

Le dessin original des flammes reste intact. Au rendu, une légère
déformation du courant près des surfaces suit leurs normales et varie
pendant la lecture. Un fondu intérieur adoucit le contact avec les murs ;
leurs pixels solides excluent toujours les flammes. Cette méthode propose
un courant stylisé, sans prétendre simuler toute la physique d'un liquide.
Les fragments décoratifs n'accordent pas de dégâts supplémentaires,
de nouvelle portée ni de propagation de gameplay autour d'une cloison.

Les scènes restent visibles en entier pour la comparaison. Les cibles
protégées n'obtiennent pas de brûlure. Les 0,7 seconde de ciblage au début
du montage ne constituent pas un délai du sort. Aucun tour n'avance pendant
la lecture et aucun nouveau calcul de combat n'est intégré.

La revue propose les trois rendus, leurs textes de présentation et la vue
`LAB-OCC02`, lancée avec
`--apercu-laboratoire-competences --lot-souffle-continu`.
La comparaison animée conserve la version précédente à gauche et montre
le nouveau courant à droite. Les assets et artefacts antérieurs restent
protégés par empreintes. Captures, sources et lecteur local :
`artifacts/refonte-agent-2026-10-06/skill-animation-review-08/`.

### ANIM-OCC02 — conservé comme solution de secours

Le joueur trouve que le résultat ne fonctionne pas suffisamment et demande
de le garder de côté comme solution de secours. Les trois rendus A04,
la comparaison animée, les captures et le code du laboratoire sont conservés
sans retouche. Cette conservation ne vaut pas validation du rendu ni
autorisation d'intégration au combat. Statut actualisé dans `review.json` ;
empreintes des GIF et sources dans `fallback-retention-01.json`.
Le choix entre une nouvelle construction modulaire, un changement de style
et une superposition plus libre reste à discuter avec le joueur.

## ANIM-PART01 — essai de souffle composé de petites flammes

Le joueur autorise le test de la construction modulaire proposée, puis
précise que les animations doivent fonctionner vers la droite et la gauche,
même lorsqu'une cible se trouve derrière le personnage. La pose frontale
ne limite donc pas la direction de visée. Ce principe de direction concerne
les compétences ; il ne donne pas d'accord sur les nouveaux dessins ou rendus.

Le numéro 10 de FE02 est conservé, avec six situations A05 :
`FE02-A05-COULOIR-DROITE`, `FE02-A05-COULOIR-GAUCHE`,
`FE02-A05-CLOISON-DROITE`, `FE02-A05-CLOISON-GAUCHE`,
`FE02-A05-OBSTACLE-DROITE` et `FE02-A05-OBSTACLE-GAUCHE`.
Les cartes, cibles et cases touchées sont les mêmes que précédemment ;
pour la gauche, leurs positions sont réfléchies horizontalement. Les corps,
équipements et poses frontales restent inchangés, sans miroir du personnage.

Une nouvelle planche Imagegen intégré propose huit dessins d'une petite
flamme horizontale complète. La robe rouge approuvée sert de référence de
style et l'ancienne flamme de référence de palette. Les frames sont découpées,
alignées et réduites à 64 px, avec alpha intact. Les flammes se déplacent et
s'orientent sur des trajets visuels courts, mis en cache et arrondis aux
jonctions. Des variations de phase et de position décalent leurs mouvements.
Il n'y a plus de cône complet découpé, de masque de cases ni de déformation
du grand dessin. Une enveloppe circulaire conservatrice couvre tous les
pixels du petit sprite, pour le garder entier et hors des murs solides.

Les trajets suivent l'empreinte du prototype pour composer le courant ;
ils ne changent pas la propagation du sort, ses dégâts, sa durée, sa portée,
ses chances de statut ou les cibles protégées. La population de flammes
est déjà répartie sur le courant lors du lancement : leur déplacement est
une animation, sans délai d'arrivée des dégâts ni avance de tour. Le feu
reste sans sang. ANIM-OCC02 reste intact comme solution de secours.

Dans la vue `LAB-PART01`, le joueur peut choisir la situation et la direction,
garder l'aperçu affiché, puis cliquer sur Lancer. Les montages présentent
0,7 seconde de ciblage avant le lancement pour la revue uniquement.
Les six rendus, la nouvelle petite flamme, les textes du prototype et cette
interface attendent leur propre validation. Entrée isolée :
`--apercu-laboratoire-competences --lot-souffle-particules`.

Masters, prompts exacts, sources, captures et lecteur local :
`artifacts/refonte-agent-2026-10-06/skill-animation-review-09/`.

### ANIM-PART01 — essai rejeté, sources conservées

Le joueur rejette le rendu modulaire et demande de revenir à une grande
animation superposée au plateau. ANIM-PART01 reste archivé avec ses sources
et captures ; son rejet n'annule pas les animations précédemment acceptées.
ANIM-OCC02 demeure conservé comme solution de secours.

## ANIM-OVER01 — grande animation, murs au premier plan et quatre directions

Le joueur choisit une grande animation au-dessus du jeu, un aperçu de zone
plus visible et clignotant, puis étend la visée aux quatre directions.
Il accepte de tester un masquage limité aux surfaces des murs : les sols
libres derrière un obstacle ne sont pas découpés par une ombre de cet obstacle.
Cet accord autorise le prototype ; son nouveau rendu reste à valider.

Le numéro 10 de FE02 est conservé. Les douze vues A06 combinent COULOIR,
CLOISON et OBSTACLE avec DROITE, GAUCHE, HAUT et BAS. Le grand dessin et les
petites flammes utilisent les atlases originaux d'ANIM-STYLE01, sans retouche,
déformation ou nouvelle génération. Le dessin complet tourne avec la visée ;
les personnages restent debout et de face. Le terrain solide est redessiné
devant le feu, puis les personnages passent devant toutes les flammes.

La liste des cases touchées et les cibles exposées/protégées sont conservées,
avec leurs rotations cardinales. Une flamme décorative peut rester visible
sur un sol protégé derrière un mur : elle ne donne aucun dégât ni statut.
Les contours cyan, leur fond sombre et leur ligne intérieure claire indiquent
les cases réellement touchées. Leur luminosité alterne une fois par seconde
entre une phase forte et une phase atténuée, sans disparaître. L'aperçu reste
visible pendant le souffle ; les corps ne sont pas teintés par son remplissage.

La scène de revue garde 0,7 seconde de ciblage avant le lancement, sans créer
un délai de préparation de FE02. La lecture ne fait avancer aucun tour.
Les réussites de Brûlure sont scénarisées pour les cibles exposées ; les
cibles protégées ne reçoivent pas les petites flammes de statut.

Les rendus A06 et la vue `LAB-OVER01` attendent leur validation séparée.
Entrée de laboratoire :
`--apercu-laboratoire-competences --lot-souffle-superpose`.
Sources, captures natives de 64 px, montages animés et lecteur local :
`artifacts/refonte-agent-2026-10-06/skill-animation-review-10/`.
Aucun effet de combat de la campagne n'est ajouté par cet essai.

### ANIM-BURN01 — petites flammes de Brûlure acceptées

Le joueur désigne explicitement les petites flammes existantes sur les
personnages comme animation du statut temporaire Brûlure, puis demande de
les faire passer derrière les personnages. Leur forme est acceptée et cette
instruction de couche est appliquée dans ANIM-OVER01. Elles restent verticales
quelle que soit la direction du souffle. L'accord ne change ni dégâts,
ni chance d'application, ni durée du statut, et ne valide pas le grand souffle
A06. Source de la fonction et empreintes des dessins dans
`skill-animation-review-10/burn-visual-acceptance-01.json`.
Le raccordement au statut de la campagne reste à réaliser après les revues.

## ANIM-OVER02 — accord sur le style, corrections des obstacles et du ciblage

Le joueur valide la direction d'ANIM-OVER01 avec trois corrections : les
obstacles doivent couper l'animation à leur emplacement, l'aperçu doit
disparaître dès la validation de l'attaque, et Brûlure doit rester visible
sur tout acteur affecté, joueur ou monstre. Il confirme ensuite la coupure
par les obstacles et demande un test dans un couloir long. Ces instructions
remplacent la permission de laisser des flammes décoratives derrière une
cloison et le maintien de l'aperçu pendant le souffle. Les dessins acceptés,
les quatre directions et la couche derrière les personnages sont conservés.
Accord partiel et références exactes dans
`skill-animation-review-11/visual-acceptance-01.json`.

Le numéro 10 de FE02 reste stable. A07 reprend les trois situations dans
les quatre directions et ajoute un couloir long ouvert, puis barré par une
cloison, vers la droite et la gauche. Chaque couloir comprend treize colonnes
dont onze cases libres dans sa version ouverte. La portée de cinq cases au
rang d'exemple reste inchangée. Les quatre directions de ces couloirs sont
également disponibles dans le laboratoire natif.

Le dessin original reste entier là où il est visible, sans déformation ni
nouvelle génération. Des segments reliant son point d'émission à chaque
pixel déterminent la première rencontre avec les rectangles physiques des
murs. La partie bloquée s'arrête au mur et ne réapparaît pas derrière lui ;
les parties qui passent à côté peuvent continuer. Ce masque géométrique
ne découpe pas le dessin selon les frontières des cases de gameplay et ne
change ni la liste des cases touchées ni les règles de propagation du sort.

L'aperçu cyan clignotant ne paraît qu'avant le lancement. Remplissage et
contours disparaissent dès sa confirmation. Les petites flammes de Brûlure
conservent leur dessin accepté et passent derrière les acteurs exposés.
Le montage garde un moment sans grand souffle pour lire ce statut, puis
montre sa fin. `B01 / BRULURE-A02-AGENT-MONSTRE` présente aussi le statut sur
l'agent et sur un monstre, sans créer de dégâts alliés.

Les durées de montage — ciblage jusqu'à 0,7 seconde, souffle jusqu'à
1,8 seconde et Brûlure jusqu'à 2,7 secondes — servent uniquement à la revue.
Elles ne remplacent pas les durées en tours et ne font avancer aucun tour.
Ces rendus corrigés et `LAB-OVER02` sont proposés dans
`artifacts/refonte-agent-2026-10-06/skill-animation-review-11/`.
Entrée isolée : `--apercu-laboratoire-competences --lot-souffle-arrete`.
Les acteurs, icônes, paramètres, captures antérieures et essais de secours
restent conservés ; le combat de la campagne n'est pas modifié.

Le joueur précise que Brûlure doit être réellement sur l'acteur touché.
La première capture montrait que la robe cachait presque toutes les petites
flammes arrière. La correction conserve ces flammes principales derrière
le corps et propose deux petites langues du même dessin sur sa silhouette,
à hauteur du ventre et des jambes. Tous ces points d'attache partent de la
position de l'acteur ; ils ne sont pas des feux de terrain. Ce complément
de premier plan est inclus dans le rendu de statut A02 à examiner.
Les premières captures restent archivées dans `native-draft-01/`.

La capture finale utilise une cible de rendu native fixe de 1024 × 1024.
Cette cible empêche une réduction passagère de la fenêtre de capture de
produire des images de 1 × 1. Elle ne redimensionne pas les cases ni les
dessins et ne modifie pas le rendu du laboratoire interactif.

## ANIM-OVER03 — retrouver le dessin à la sortie du couloir

Le joueur constate que la coupure A07 forme un cône géométrique trop strict
quand le couloir mène à une pièce. Il préfère retrouver les formes dessinées,
même avec un léger débordement des frontières de cases libres. La correction
porte seulement sur ce masque visuel ; les dessins, la zone de gameplay,
l'arrêt aux obstacles, la disparition de l'aperçu et Brûlure sont conservés.

Le numéro 10 de FE02 reste stable. Les quatre directions de COULOIR sont
proposées sous `FE02-A08-COULOIR-DROITE`, GAUCHE, HAUT et BAS. La revue place
A07 à gauche et A08 à droite à la même phase, en cases natives de 64 px.
La comparaison et les quatre vues utilisent le même lecteur avec pause
et ralenti. A07 et toutes les captures précédentes restent archivées.

L'essai repère une ouverture libre au bout du couloir étroit qui contient
l'émetteur. Cette ouverture doit être accessible depuis celui-ci sans traverser
de mur. À partir de son entrée, un second point de visibilité permet au dessin
original de reprendre sa largeur sur le sol de la pièce. Il ne déforme pas
les flammes et ne prolonge pas les murs latéraux du couloir en triangles dans
la pièce. Les murs physiques rencontrés depuis l'ouverture continuent à couper
le dessin. Les quatre autres situations de revue — cloison, obstacle isolé,
couloir long et couloir long barré — conservent exactement leurs pixels de masque.

Ce point supplémentaire ne donne ni portée, ni dégâts, ni nouveau chemin de
propagation au sort. Il concerne l'habillage de la zone, qui peut dépasser
une frontière de case libre. Les paramètres du catalogue et les listes de
cases touchées restent identiques. Les phases de ciblage et de Brûlure sont
également comparées aux captures A07 sans changement attendu.

Les rendus A08 et `LAB-OVER03` restent à examiner dans
`artifacts/refonte-agent-2026-10-06/skill-animation-review-12/`.
Entrée native : `--apercu-laboratoire-competences --lot-souffle-ouverture`.
La revue ne modifie pas le combat de la campagne.

### ANIM-OVER03 — validation de Souffle de feu et des règles communes

Le joueur répond « cette animation est validée :) on gardera les regles pour
les prochaines animations :) ». Le rendu 10 / FE02-A08 dans les quatre
directions est accepté, avec les formes peintes retrouvées après l'ouverture,
les arrêts sur les obstacles, l'aperçu retiré au lancement et la Brûlure
montrée sur les corps affectés. La comparaison A07/A08 sert de référence
exacte ; A07 conserve son rôle de version antérieure.

L'accord et les empreintes du rendu accepté figurent dans
`skill-animation-review-12/visual-acceptance-01.json`. Le manifeste de cette
revue passe à `visually_approved`. Les règles communes sont inscrites dans
`DIRECTION_ARTISTIQUE_TILES.md`, section « ANIM-OVER03 — rendu accepté et règles
des prochaines animations ». Les nouveaux lots resteront présentés avant
leur intégration. Cet accord ne modifie pas les chiffres de combat et ne
valide pas automatiquement les autres animations ou la vue `LAB-OVER03`.

## ANIM-S05 — Mur de feu et Émanation soudaine

Le joueur demande de poursuivre après l'accord sur Souffle de feu et les
règles communes. Le lot reprend les numéros 11 / FE03 et 12 / FE04, avec
deux situations chacun : `FE03-A03-LIBRE`, `FE03-A03-CLOISON`,
`FE04-A03-LIBRE` et `FE04-A03-CLOISON`. Les dessins de flammes sont ceux du
master peint existant ; les personnages, icônes et animations acceptés ne
sont pas retouchés. Les vues utilisent des cases natives de 64 px.

Mur de feu forme un rideau par chevauchement des dessins, espacés d'une
demi-case à l'affichage. Sa longueur au rang d'exemple 11 suit les sept
cases du catalogue ; la cloison interrompt la ligne. Cette densité graphique
n'ajoute pas de cases de dégâts. Le monstre A traverse le mur au premier
tour scénarisé et rejoint une case libre. Ses petites flammes de Brûlure
suivent sa silhouette pendant le déplacement puis après la sortie. Les
autres cibles présentes dans le rideau continuent à brûler après sa fin.
Le mur reste traversable et ne crée pas de contrôle de déplacement.

Émanation conserve l'empreinte en croix d'une case de rayon utilisée dans
la revue précédente. L'aperçu cyan disparaît dès le lancement ; les fissures
restent comme marqueur du sort pendant sa préparation. Un événement explicite
« +1 tour » provoque l'éruption. Une gerbe centrale plus grande et des gerbes
latérales jaillissent derrière les corps. Cette accentuation visuelle ne
modifie pas l'écart de dégâts entre le centre et les bords. Brûlure reste
visible sur les cibles après la disparition des grandes flammes.

Dans les scènes de cloison, les flammes sont exclues des surfaces solides
et du sol libre séparé de la zone par la cloison complète. Le dessin conserve
ses formes dans les espaces libres de la pièce, sans découpe par les
frontières des cases de dégâts. Les cibles C de FE03 et F de FE04 derrière
la cloison demeurent protégées. Les brûlures montrées sont des réussites
scénarisées pour la revue : aucun calcul de dégâts ou tirage aléatoire,
aucune modification des résistances ou des chances n'est ajouté.

Les montages durent cinq secondes. Le lancement survient à 0,7 seconde ;
les événements de tours sont scénarisés à 1,35, 1,90, 2,40, 2,85, 3,30,
3,80 et 4,30 secondes. Ce temps de présentation ne remplace pas les tours.
Dans `LAB-S05`, seuls les clics sur « +1 tour » ou Espace avancent ces compteurs.
La lecture, le replay, la pause et le ralenti n'avancent pas les tours et
ne rendent pas une nouvelle activation disponible avant sa recharge.
Le laboratoire propose également les quatre directions et un rang effectif
d'aperçu ; il ne configure pas le personnage de test de la campagne.

Les quatre rendus et la vue `LAB-S05` attendent leur validation séparée dans
`artifacts/refonte-agent-2026-10-06/skill-animation-review-13/`.
Entrée native : `--apercu-laboratoire-competences --lot-feu-zones`.
La géométrie utilisée reste une fixture de revue, sans ajout au combat,
à la progression, aux sauvegardes ou aux descriptions du catalogue.

## ANIM-S05-CADENCE01 — comparaison du mouvement des flammes

Sur la revue 13, le joueur répond « ca me semble tres bien :) meme si
peut-être que l'animation des flammes est un peu rapide ? ». Le retour sur
le rendu est favorable ; une cadence révisée reste à présenter et à valider.
La revue 14 conserve les numéros 11 / FE03 et 12 / FE04, avec les variantes
`FE03-A04-LIBRE`, `FE03-A04-CLOISON`, `FE04-A04-LIBRE` et
`FE04-A04-CLOISON`. Elle compare le mouvement actuel à un mouvement des
flammes ralenti d'un tiers : huit dessins par seconde au lieu de douze,
sur les mêmes huit images sources. Brûlure suit cette cadence également.

Seule la lecture des images de flammes change. La montée et la disparition
des gerbes, les déplacements, les fissures de préparation, le clignotement
du ciblage et les événements de tours restent synchronisés avec la revue
13. Les durées, recharges, portées, obstacles, acteurs et dessins ne sont
pas modifiés. Le code conserve la cadence précédente par défaut et propose
l'essai isolé via `--apercu-laboratoire-competences --lot-feu-cadence`.
Souffle de feu et toutes les revues antérieures restent conservés.

Comparaisons animées côte à côte à taille native dans
`artifacts/refonte-agent-2026-10-06/skill-animation-review-14/`.
La nouvelle cadence attend l'accord du joueur avant adoption.

### Validation de la cadence et correction de présence par tours

Le joueur répond « d'accord je valide :) », puis précise que l'animation
doit durer tant que le tour concerné n'est pas passé. Le dessin et la cadence
ralentie restent acceptés, avec empreintes des rendus dans la revue 14,
`visual-acceptance-01.json`. La fin automatique en secondes d'Émanation
doit être corrigée. Les anciennes mentions de proposition restent l'historique.

La règle `ANIM-TEMPS01` maintient l'animation pendant l'attente du joueur,
jusqu'au passage du tour prévu. La présence visuelle n'ajoute pas de nouvelle
application de dégâts. `12 / FE04-A05-TOUR`, dans la revue 15, compare le
rendu précédent au maintien animé pendant près de trois secondes sans
nouveau tour. Le deuxième passage de tour retire l'éruption ; la Brûlure
reste ensuite jusqu'à la fin de ses propres tours. Le lecteur de revue
propose des tours manuels pour pouvoir attendre indéfiniment.

Entrée de laboratoire isolée :
`--apercu-laboratoire-competences --lot-feu-tours`. Un test vérifie aussi
qu'attendre 600 secondes ne termine pas le rendu ni ses statuts, et qu'un
replay ne restaure pas une éruption déjà expirée par passage de tour.
Le nouveau maintien visuel attend sa revue ; les chiffres et le combat
de la campagne ne sont pas modifiés.

### Validation de FE04-A05 et ANIM-TEMPS01

Le joueur répond « parfait je valide et on continue :) » sur la revue 15.
Le maintien du dessin jusqu'au passage du tour, sa cadence ralentie et la
continuité de Brûlure montrés dans l'aperçu sont acceptés. Les empreintes et
l'accord figurent dans `skill-animation-review-15/visual-acceptance-01.json`.
Le prochain lot reprend 13 / FE05 Boule de feu et 14 / FE06 Combustion.

## ANIM-S06 — Boule de feu et Combustion

La revue 16 propose `13 / FE05-A01` et `14 / FE06-A01` à taille native
64 px. La boule incandescente et la déflagration sont de nouveaux dessins
produits avec imagegen ; leur prompt exact et la planche source transparente
figurent dans le dossier de revue. Le conditionnement recadre et redimensionne
proportionnellement les sprites, sans peinture ni recoloration. Les images
de flammes précédemment acceptées restent utilisées pour Brûlure et le
resserrement de Combustion. Les acteurs et leurs poses sont conservés.

Boule de feu montre un projectile dans la direction visée, puis une explosion
ample autour de l'impact. La scène de cloison montre un impact sur le sol libre
juste devant le mur. Le décor et le sol de la pièce séparée restent sans effet.
La forme des flammes conserve ses contours sur le sol libre, sans découpage
sur les frontières de cases de dégâts. Le rayon circulaire de trois cases
est une fixture d'aperçu ; la résolution finale des collisions et de la zone
en combat n'est pas implémentée par cette revue.

Combustion resserre les flammes sur la seule cible brûlante, puis montre un
embrasement concentré. La Brûlure initiale est consommée dans la fixture.
Un cas préparé montre une nouvelle Brûlure après le choc ; le cas de cloison
montre son absence. Ces cas ne définissent pas de nouveau tirage aléatoire.
Les impacts continuent à s'animer pendant l'attente et disparaissent au
passage du tour suivant. Les petites flammes de statut restent ensuite
jusqu'à la fin de leurs propres tours. Aucun nouveau dégât n'est calculé
ou répété par le maintien du dessin.

Les quatre vues natives sont droite, gauche, haut et bas ; le bas montre
la cloison. Les corps restent de face. Le lecteur de revue possède des tours
manuels, une pause et une sélection de scène. Le laboratoire natif ajoute
les conditions de lancement, les recharges et le cas de nouvelle Brûlure.
Lancer Combustion reste indisponible sans Brûlure sur la cible.

Entrée isolée : `--apercu-laboratoire-competences --lot-feu-final`.
Les deux rendus sont proposés dans
`artifacts/refonte-agent-2026-10-06/skill-animation-review-16/` et restent
à valider avant intégration. Les mécaniques, les textes de compétences,
les paramètres et le combat de la campagne restent conservés.

### ANIM-S06-A02 — la boule explose à la collision visible

Pendant la préparation du lot, le joueur demande que le projectile parte
réellement du lanceur et explose sur la cible qu'il touche. Il autorise
également des dessins de flammes propres à chaque sort, tant que la qualité
et le style restent cohérents. Les sources de la revue 16 restent conservées.

La revue 17 propose `13 / FE05-A02`, avec le même numéro stable. Le trajet
visuel dure 0,65 seconde pour mieux distinguer le déplacement, puis l'impact
reste animé jusqu'au passage du tour. Le projectile est joué une seule fois :
la boucle d'attente ne contient que l'explosion. La collision détermine le
centre de l'explosion et l'empreinte d'aperçu. Sur le trajet libre, A reçoit
le projectile. Dans la scène d'interception, D placé avant A est touché et
devient le centre. Face à la cloison, la boule éclate à la face du mur, avec
les flammes exclues du décor et de la pièce séparée. Les figures de portée
et rayon du catalogue restent conservées ; ces collisions restent une
fixture visuelle avant intégration au combat.

`14 / FE06-A01` Combustion accompagne la revue avec le même embrasement
ciblé et les cas préparés de nouvelle Brûlure. Le laboratoire permet le
trajet direct ou l'interception. Les tests vérifient le premier monstre
touché, le centre de l'explosion, les coupures de cloison et les compteurs
indépendants du temps d'attente. Les deux compétences attendent leur accord
visuel dans `skill-animation-review-17/`.

### ANIM-S06-A04 — impacts allégés et extinction automatique

Le joueur trouve l'explosion et Combustion trop envahissantes, et la boule
trop grosse. Il invite à utiliser davantage la transparence, puis rappelle
que Boule de feu n'a pas de zone persistante : son animation doit se terminer
une fois le projectile et le choc joués. `ANIM-TEMPS02` distingue ces impacts
instantanés des effets persistants pilotés par les tours. Le maintien accepté
de `FE04-A05-TOUR` demeure inchangé.

La revue 18 conserve le brouillon natif de la réduction visuelle, encore
maintenu par tours, et ses sources exactes. Elle est remplacée avant présentation
par la revue 19, `13 / FE05-A04` et `14 / FE06-A03`, avec le même dessin source.
Le projectile est ramené de 128 à 96 px ; l'explosion de 512 à 352 px, avec
une opacité de 52 %. Combustion utilise seulement deux petites flammes à
65 % d'opacité autour de la cible, sans explosion circulaire.

Le trajet dure 0,65 seconde et l'explosion 0,55 seconde. Le choc de Combustion
dure 0,60 seconde. Leur dernière fraction de seconde s'estompe progressivement.
Ces durées sont uniquement celles de l'animation et n'ajoutent aucun tour
de préparation, aucune nouvelle zone de dégâts ni aucun soin. Attendre
ne répète pas les dégâts et ne réduit pas les recharges. Une nouvelle Brûlure
éventuelle reste sur le corps jusqu'à l'expiration de ses trois tours de revue.

Le lecteur joue le lancement une seule fois, puis boucle uniquement les flammes
de statut pendant l'attente. Les tours se passent manuellement. Les montages
natif/GIF passent explicitement leurs tours à 3, 4 et 5 secondes pour montrer
l'expiration de Brûlure. Les scènes testent droite, gauche, haut, bas devant
une cloison, et un monstre interposé. Les rendus sont proposés avant validation
dans `artifacts/refonte-agent-2026-10-06/skill-animation-review-19/` ; le combat
de la campagne demeure une étape distincte.

### ANIM-S06-A05 — explosion dessinée en étapes

Le joueur précise que la transparence doit servir le rendu, sans transformer
tout le sort en effet translucide. Ce qu'il veut voir à l'impact est une
explosion ; la boule éclatée du dessin précédent ne le convainc pas.

La revue 20 propose `13 / FE05-A05`, à partir d'une nouvelle planche imagegen
de six dessins. Ils se jouent chronologiquement, avec un flash, un éclatement
irrégulier, des flammes détachées puis des braises éparses. La fin ne revient
pas au dessin du début. Le corps de l'impact est plus marqué ; la transparence
accompagne surtout sa dissipation. Le projectile de 96 px est conservé,
comme les 0,65 seconde de trajet et les 0,55 seconde de choc. Combustion
`14 / FE06-A03` conserve son assemblage ciblé proposé dans la revue 19.
Les deux rendus restent à valider dans `skill-animation-review-20/`.

Pour accélérer les captures de cette revue isolée, `--capture-raw` exporte
les pixels natifs avec la même inversion verticale que Macroquad. Le
conditionnement les encode en PNG sans perte et vérifie l'identité de chaque
pixel avant de retirer uniquement ce fichier temporaire brut. Ce chemin
d'export ne modifie aucun dessin, filtre, masque, paramètre ni règle de jeu.

### Validation du lot et FE05-A06 — trajet plus rapide

Le joueur répond « je valide :) », puis « je valide mais j'aimerais qu'on
accelere legerement le mouvement de la boule de feu (avant qu'elle n'explose) ».
La revue 20 est acceptée pour ses dessins et rendus `FE05-A05` et `FE06-A03`.
La vitesse du projectile fait l'objet d'une révision distincte. L'accord et
les empreintes sont enregistrés dans sa `visual-acceptance-01.json`.

`13 / FE05-A06`, revue 21, conserve les images exactes et ramène le trajet
de 0,65 à 0,50 seconde. Le choc reste de 0,55 seconde ; Combustion reste
de 0,60 seconde et Brûlure conserve sa cadence. Les durées, recharges,
empreintes et règles de combat ne sont pas modifiées. Le nouveau contact
apparaît à l'image 29 au lieu de 33 dans la capture à 24 images par seconde ;
l'impact termine à l'image 43 au lieu de 46. Le délai modifié reste purement
visuel. La galerie compare l'ancien trajet et le nouveau dans les mêmes
scènes, avec lancement simultané. Cet ajustement seul reste à examiner.

### FE05-A07 — nouvelle accélération du projectile

Sur la revue 21, le joueur demande « tu peux encore accelerer :) ».
La revue 22 compare 0,50 seconde à gauche et 0,35 seconde à droite, avec
un lancement simultané. `13 / FE05-A07` conserve exactement les dessins,
la durée de 0,55 seconde de l'explosion, les coupures par obstacles et les
compteurs de statut/recharge. Le contact apparaît à l'image 26 et le choc
termine à l'image 39 dans la capture à 24 images par seconde. Cet ajustement
de vitesse reste à examiner ; la validation artistique du lot 20 demeure.

Le joueur précise ensuite que les sorts et les autres compétences actives
doivent s'activer rapidement dans ce mélange de roguelike et hack'n slash.
`ANIM-RAPIDE01` fixe la direction des prochains lots : lancements vifs et
impacts courts, tout en gardant les zones/statuts animés pendant leurs tours.
Les préparations mécaniques déjà prévues restent distinctes de ces cadences.

### FE05-A07 — vitesse finale acceptée

Le joueur répond « ok je valide :) » sur la revue 22. Le trajet à 0,35 seconde
est accepté, avec le même choc de 0,55 seconde et les dessins précédemment
validés. L'accord et les empreintes de l'aperçu sont conservés dans
`skill-animation-review-22/visual-acceptance-01.json`. Les accords de la revue
20 pour Boule de feu et Combustion demeurent. La campagne et les paramètres
de combat ne sont pas modifiés par cet enregistrement.

## ANIM-S19 — Décharge mutagène et Autodestruction

Après les cinq transformations et leurs actions propres validées, la revue
48 propose `47 / M02-A01` et `48 / HY07-A01`. Deux planches imagegen
fournissent la décharge organique verte et son impact bref ; les corps
acceptés sont réutilisés, dont le Possédé BODY05 aux muscles rouges et
tendons chair. L'explosion de drone réutilise le dessin de feu accepté.

La galerie et le laboratoire natif montrent quatre directions, les murs,
les cinq formes pour Décharge mutagène, ses cas sans prolongation, les
trois drones de combat et les refus d'Autodestruction. Les victimes font
la réaction commune ; l'autre drone et le joueur restent indemnes. Aucun
statut de Brûlure, Poison ou Saignement n'est ajouté implicitement.

Les effets s'éteignent après leurs impacts brefs ; les recharges et durées
ne diminuent qu'avec les tours manuels. Les aperçus clignotent avant la
confirmation et disparaissent au lancement. Les mécaniques affichées
reprennent les fiches acceptées, dans des cas préparés sans calcul réel
de dégâts, PV, énergie, IA, loot ou sauvegarde. Les deux compositions
attendent la revue visuelle du joueur. Entrée native :
`--apercu-laboratoire-competences --lot-hybrides`.

Le joueur donne la priorité à la fin des compétences et animations avant
l'adaptation 64 px des monstres et PNJ existants et la création d'un très
grand nombre de nouveaux monstres. Les acteurs ennemis et drones encore
en 32 px agrandis sont signalés comme temporaires dans la revue 48.

### ANIM-S19 — validation des deux animations

Le joueur répond « je valide » le 13 octobre 2026. Les compositions
`47 / M02-A01` et `48 / HY07-A01` sont acceptées avec les directions,
formes et cas présentés. La preuve est conservée dans la revue 48,
`visual-acceptance-01.json`. Les dessins et aperçus sont figés ; les
règles conservent leurs validations propres. Aucun raccordement au
combat de la campagne n'est réalisé par cet accord.

## ANIM-S20 — Échange de position et Lien de domination

La revue 49 propose `49 / HY04-A01` et `50 / HY06-A01`. Échange de
position fait disparaître brièvement l'agent et le drone dans des
fragments sombres avant de les reformer sur leurs cases échangées.
Domination affiche un symbole occulte violet au-dessus de la victime
pendant ses quatre tours, avec la réaction commune à l'effet négatif.
Les cas préparés montrent la cargaison de la Mule conservée, les
refus, le maintien du lien après des dégâts, son expiration et le
remplacement d'une ancienne cible. La liste de cibles préparée du
drone respecte l'allié dominé, sans modifier l'IA de la campagne.

Les deux animations restent à examiner. Entrée native :
`--apercu-laboratoire-competences --lot-echange-domination`.
Ces prototypes ne calculent ni dégâts, ressources, chances, loot ou
sauvegardes et ne fixent pas le coût d'un contrôle raté, les diagonales
ou le franchissement d'obstacles intermédiaires lors d'un échange.

### ANIM-S20 — validation des deux compositions

Le joueur répond « parfait je valide :) » le 13 octobre 2026.
`49 / HY04-A01` Échange de position et `50 / HY06-A01` Lien de domination
sont acceptés avec les animations et cas présentés. L'accord et les
empreintes des fichiers figurent dans la revue 49,
`visual-acceptance-01.json`. Les règles conservent leurs validations
propres et aucun combat ou système d'IA de campagne n'est modifié.

## ANIM-S21 — Lame incendiaire et Faim vampirique

La revue 50 propose `51 / BM01-A01` et `52 / BM02-A01`. Lame incendiaire
garde l'arme sans nouvelle surcouche de flammes : le total de dégâts est
suivi de ses composantes physiques et feu, avec leur petit symbole et leur
couleur. Aucun statut Brûlure n'est ajouté par le buff.

Faim vampirique fait apparaître deux mâchoires de vampire qui se referment
brièvement sur la cible à chaque frappe réussie préparée. Le soin flotte
au-dessus de l'agent. Les deux scènes gardent le sang des blessures physiques
et la réaction commune de la victime, sans déplacer sa case logique.
Les icônes compactes validées occupent la barre des buffs ; leur survol
montre le nom, l'effet et le nombre de tours restants.

La galerie et le laboratoire présentent quatre directions, une seconde
frappe, l'attente sans tour, l'expiration, le mur, la cible cachée et les
refus de lancement en Silence, avec une arme à distance ou en transformation.
Les valeurs de frappe 20, total 34 et soin 8 sont des exemples préparés,
sans nouvelle statistique d'arme, variance, défense ou calcul d'équilibrage.
Les valeurs de rang 11 des fiches BM01-02 et BM02-03 restent inchangées.
Les cas de soin nul, miss, absorption, vie pleine et plafond atteint
restent à préciser séparément. L'ordre de résolution des actions n'est
pas défini par la chronologie de démonstration.

Les compositions restent à valider. Entrée native :
`--apercu-laboratoire-competences --lot-buffs-melee`.
Aucun raccordement au combat ou aux sauvegardes de campagne.

### ANIM-S21 — animations validées et accès navigateur corrigé

Le joueur répond « ok ca me va :) je valide » le 13 octobre 2026.
Les compositions `51 / BM01-A01` et `52 / BM02-A01` sont acceptées.
La preuve et les empreintes sont conservées dans la revue 50,
`visual-acceptance-01.json`. Les règles et cas encore non montrés
gardent leur statut ; aucun raccordement au combat de campagne.

Le joueur signale ensuite que le lien du laboratoire affiche le texte
du script PowerShell. Un lien vers `cote-a-cote.html` ouvre désormais
les deux animations dans le navigateur. La comparaison s'adapte à la
fenêtre et propose la taille native et le plein écran. Les fichiers
d'animation sont conservés à l'identique. Cette correction de navigation
reste distincte de l'accord artistique.

Pour les prochaines galeries, proposer un véritable aperçu HTML côte à
côte ; ne pas présenter le lien d'un script .ps1 comme un aperçu qui
s'exécute depuis le navigateur. Le laboratoire natif reste un outil
local à lancer avec PowerShell. Le générateur appelle
`tools/art/add_browser_animation_comparison.py` après l'écriture de
la galerie pour produire cet accès directement.

## ANIM-S22 — Tempo offensif et Sceau de vindicte

La revue 51 propose `53 / BM03-A01` et `54 / BM05-A01`. Tempo offensif
gagne automatiquement ses charges après les frappes principales préparées.
La barre de buffs indique les charges et le bonus au survol, sans timer
artificiel. Les nombres affichent le total et la part du bonus déjà incluse,
avec une petite indication du type physique. Les coups manqués, le mouvement
et une autre action rompent la série ; un menu sans tour la conserve.

Sceau de vindicte réutilise le projectile occulte accepté et son violet
franc. Il touche l'agent, puis repart vers son auteur avec le même dessin
et la même taille. Les deux victimes gardent la réaction commune. La magie
ne produit pas de sang ; le coup de mêlée exclu conserve sa blessure physique.
Le buff reste dans la barre jusqu'à ses six tours. Le poison, les dégâts
indirects et la mêlée ne produisent aucun retour ; le dernier exemple montre
une attaque après expiration, toujours subie, sans renvoi.

Les deux panneaux sont des scènes indépendantes avec quatre directions,
attente sans action, murs, cible cachée, arme active, Silence et survol.
Les chiffres de frappe et renvoi sont préparés pour la lecture et ne créent
aucun barème de monstres, armes, variance, défense ou arrondi. Les cas mixtes,
multi-coups, absorption, coups mortels, défenses du destinataire, récursion
et compatibilité transformée de Tempo restent à discuter séparément.

La galerie ouvre directement sa comparaison HTML ; laboratoire natif :
`--apercu-laboratoire-competences --lot-tempo-vindicte`.
Les deux compositions sont validées visuellement le 13 octobre 2026.
Aucun raccordement aux combats ou sauvegardes de campagne.

### ANIM-S22 — validation visuelle

Le 13 octobre 2026, « je valide :) » accepte Tempo offensif
`53 / BM03-A01` et Sceau de vindicte `54 / BM05-A01`, présentés dans
`skill-animation-review-51/cote-a-cote.html`. Les compositions et cas
montrés deviennent les références visuelles pour l'intégration.
L'accord et les empreintes des fichiers sont consignés dans
`skill-animation-review-51/visual-acceptance-01.json` ; le dossier
`native-glyph-draft` reste un brouillon non validé.

Les règles chiffrées déjà acceptées conservent leur statut. Les nombres
préparés dans l'aperçu ne constituent pas un nouvel équilibrage ; les
interactions encore signalées dans la revue restent à définir. Cette
validation n'intègre aucun effet au combat de campagne.

## ANIM-S23 — Visée stable et Récupération d’énergie

La revue 52 propose `55 / BD04-A01` Visée stable et `56 / BD06-A01`
Récupération d’énergie. Les deux passifs réutilisent leurs icônes acceptées.
Le viseur rouge se resserre au fil des trois cumuls et la barre de buffs
montre leur nombre ; son survol affiche le bonus de précision, sans timer.
Un déplacement volontaire, forcé ou magique supprime les cumuls. Un menu
sans tour les conserve ; une attente réelle devant l'aperçu ne les augmente pas.

Une courte étincelle violette rejoint la jauge d'énergie depuis la victime
vaincue. Le gain chiffré apparaît près de la jauge. Cet élément d'interface
ne représente pas un projectile blessant et rejoint la jauge hors du masque
des dégâts. Le passif ne crée pas un nouveau buff temporisé.
Les blessures physiques gardent le sang et la réaction commune, y compris
lors du passage au cadavre. Le cas de renvoi exclu ne produit pas de sang.

Les quatre directions, attente sans action, mur, cible cachée, survol,
menu sans tour, énergie pleine ou presque pleine et sources exclues sont
présentés dans `skill-animation-review-52/cote-a-cote.html`. Les éliminations
de familiers, renvois, ripostes et leurres sont préparées sans simuler leurs
attaques ; elles ne déclenchent pas de récupération.

Les valeurs acceptées des fiches BD04-02 et BD06-04 restent conservées :
précision +15 % par cumul, maximum +45 %, récupération de 22 % de l'énergie
maximale et intervalle de trois tours. Les exemples 100 énergie maximale,
40 → 62 → 84 et 95 → 100, avec une frappe de 20, restent des fixtures visuelles.
Ils ne fixent aucune statistique d'arme, variance, chance de toucher ou arrondi.
Chaque action prépare une nouvelle victime pour illustrer l'intervalle.
L'ordre réel de résolution, les éliminations multiples ou persistantes,
les familles d'armes admissibles et le comportement à énergie pleine
gardent leur revue mécanique distincte.

Les deux rendus sont validés visuellement le 13 octobre 2026. Entrée native :
`--apercu-laboratoire-competences --lot-visee-energie`.
Aucun effet de combat ni sauvegarde de campagne n'est modifié.

### ANIM-S23 — validation visuelle

Le 13 octobre 2026, « je valide :) » accepte Visée stable
`55 / BD04-A01` et Récupération d’énergie `56 / BD06-A01`, présentés dans
`skill-animation-review-52/cote-a-cote.html`. Les compositions et cas montrés
deviennent les références visuelles pour l'intégration. L'accord et les
empreintes sont conservés dans `skill-animation-review-52/visual-acceptance-01.json`.
Les dossiers `native-feedback-draft` et `native-menu-draft` restent exclus.

Les règles chiffrées conservent leurs validations propres. Les chiffres
préparés et les interactions encore signalées dans la revue ne reçoivent
pas une nouvelle validation mécanique. Aucun effet n'est raccordé au combat
de campagne par cet accord artistique.

## ANIM-S24 — Marquage et maîtrise à distance

La revue 53 propose `57 / T01-A01` Marquage balistique et `58 / T03-A01`
Maîtrise des attaques à distance. Les deux icônes acceptées sont conservées.
Les traits rouges du viseur se resserrent sur la cible de Marquage ; son
nom apparaît brièvement au-dessus de l'agent. L'icône du buff montre les
six tours de la marque et le bonus au survol. Une bande d'analyse préparée
montre également ce statut sur la cible, sans condition de drone.
La cible reçoit le mouvement d'aller-retour commun dès l'application
de la marque, y compris dans le cas où le tir suivant manque.

Le tir suivant consomme la marque, même sur un coup manqué. La rafale
préparée garde son bonus de précision pour ses trois projectiles. Une
attente réelle devant l'aperçu ne fait avancer aucun tour. Le cas
d'expiration attend six actions préparées avant de tirer sans marque.
Le remplacement, après la recharge, transfère la marque à une autre cible
déjà visible et n'affiche pas deux marques simultanées. La marque conserve
sa règle particulière déjà acceptée de lancement sans passage de tour.

La maîtrise conserve les projectiles physiques et leur ajoute un cœur
plus vif. Le total préparé `28 (+8)` inclut le bonus et s'accompagne de sa
composante physique avec un petit symbole. Le bonus permanent ne crée
pas d'icône de buff temporisé. Les attaques préparées de mêlée et de drone
affichent leur total de 20 sans bonus. Les impacts réutilisent le sang
et la réaction commune de la victime ; une cible manquée ne réagit pas.

La comparaison HTML montre quatre directions, attente sans action, murs,
cible cachée, survol, refus de marquage en mêlée, Silence ou transformation,
tir manqué, rafale, expiration, remplacement et tir d'un drone. La bande
d'analyse est un aperçu de statut, pas une nouvelle fenêtre d'analyse complète.
Les dessins d'acteurs, poses et armes restent intacts.

Les fiches acceptées T01-02 et T03-02 conservent leurs valeurs : précision
×1,35, durée maximale six tours, portée six cases, coût neuf énergie,
recharge trois tours ; dégâts à distance +40 %. Les frappes de 20, total
de maîtrise 28 et rafale de 60 sont des nombres préparés, sans nouvelle
variance d'arme, chance de toucher, statistiques ou défense de monstre.
Les sorts, transformations, armes lourdes, rafales sur plusieurs cibles
et calculs réels gardent leurs contrats et revues propres.

Les rendus attendent l'accord visuel du joueur. Entrée native :
`--apercu-laboratoire-competences --lot-marquage-maitrise`.
Aucun effet n'est raccordé au combat ou aux sauvegardes de campagne.

### Avancement contrôlé après ANIM-S23

Le catalogue contient 94 compétences, dont 35 ont une validation d'animation
explicite. Quatre autres ont déjà un aperçu mais attendent leur revue complète :
Nova de givre, Vague de froid, Trait arcanique et Déflagration occulte.
Il reste donc 55 compétences à traiter visuellement (32 actives, 23 passives),
avant la préparation d'ANIM-S24. Ce nombre n'est pas celui des animations
distinctes : des passifs modifient les valeurs ou les effets existants.
Les numéros de galerie incluent aussi les actions de transformation,
le Gel et des variantes de collision. Trace : `skill-animation-review-53/progress-audit.json`.

### ANIM-S24-A02 — Icône au-dessus de la cible marquée

Le 14 octobre 2026, le joueur demande que Marquage balistique laisse une
icône au-dessus de la tête de sa cible. La revue 54 propose `57 / T01-A02` :
l'icône compacte déjà acceptée est affichée en 24 px dans un petit fond sombre,
au-dessus de la cible. Elle suit la réaction visuelle de cette dernière,
reste présente pendant toute la marque et disparaît à sa consommation ou
expiration. Lors du remplacement, seule la nouvelle cible porte l'icône.

Le viseur, les indicateurs de durée et les chiffres conservent leur fonctionnement.
L'icône est une surcouche de statut et peut couvrir le décor situé au-dessus
de la tête ; elle ne représente ni une case affectée ni un effet blessant.
La revue 53 est conservée. `58 / T03-A01` est repris sans changement de dessin
ou d'animation. La demande valide la direction de l'ajout ; son rendu natif
reste à examiner dans `skill-animation-review-54/cote-a-cote.html`.
Aucun effet de campagne n'est intégré et aucune validation de T03 n'est déduite.


## ANIM-S27 — Déflagration amplifiée et Drain d’impact

La revue 57 propose **63 / HL05-A01** et **64 / HL06-A01**, dans une
comparaison HTML animée et dans le laboratoire natif. Les cases font 64 px.
Les deux compositions attendent une validation visuelle explicite.

Déflagration amplifiée reprend le projectile et l’explosion peinte acceptés.
Au rang 11, le rayon d’exemple passe de une à quatre cases, sans multiplier
les dégâts. Les murs coupent le dessin et ses effets ; une arme sans explosion
conserve son tir ordinaire. Ce passif ne crée ni buff temporaire ni lancement
séparé, et ne modifie pas la zone de ciblage de Déluge ou le rayon de contrôle
d’Impact assommant.

Drain d’impact reprend l’impact habituel, le sang physique et la réaction
commune des cibles blessées. Une brève impulsion verte entoure le joueur et
le gain « +X PV » apparaît au moment du soin. Le retour vient uniquement des
PV réellement retirés à la cible principale, avec le plafond par utilisation
et la limite des PV manquants déjà acceptés. L’impulsion se termine d’elle-même ;
aucun statut temporisé n’est ajouté à la barre de buffs.

Les scènes préparées montrent les quatre directions, une cloison, l’attente
sans action, les armes exclues ou sans explosion, les refus et les limites de
soin. Le cas à retardement attend une action avant l’explosion : ce délai d’un
tour sert d’exemple, sans fixer celui des futures familles d’armes. Le cas des
PV pleins montre un retour nul et ne tranche pas l’autorisation réelle du
lancement à vie pleine. Les montants, protections et acteurs restent préparés ;
aucune courbe réelle, règle d’arrondi ou priorité de résolution n’est ajoutée.

Entrée native : `--apercu-laboratoire-competences --lot-armes-lourdes-finale`.
Comparaison : `skill-animation-review-57/cote-a-cote.html`.
Les aperçus acceptés de la revue 56 restent intacts. Les monstres temporaires
seront adaptés après les animations. Aucun effet n’est raccordé au combat de
campagne par ce prototype.


## ANIM-S28 — Furtivité et Économie de mouvement furtif

La revue 58 propose **65 / SF00-A01** et **66 / SF01-A01**, en cases natives
64 px. Les nouvelles compositions attendent leur validation visuelle.

Le camouflage réutilise les couches exactes du personnage et de son arme.
Un bref balayage bleu acier accompagne leur passage à une légère transparence.
L’apparence furtive et l’icône SF00 restent présentes entre les actions, sans
durée artificielle. La transition visuelle ne retarde pas l’état du mode.
Le passif SF01 modifie le coût affiché et les dépenses des pas ; il ne crée
aucune activation supplémentaire ni seconde icône de buff temporisé.

Les deux panneaux comparent onze énergie par pas sans SF01 à une énergie au
rang dix. Les cas préparés montrent aussi le rang neuf et le minimum avec
un rang effectif douze. À zéro énergie, le pas demandé retire le camouflage
et est effectué normalement, conformément à la règle acceptée.

Les quatre directions gardent les personnages de face. L’attente, le menu,
un pas bloqué, un soin fourni et une action offensive préparée montrent la
conservation ou la levée du mode. Le soin de cinq PV est un événement de revue,
pas une nouvelle compétence ; l’action offensive lève le mode sans exécuter
une attaque. Aucun bruit, cône de détection, effacement de mémoire ou nouvelle
IA n’est simulé. Le survol reprend uniquement l’exemple accepté « Distance de
repérage : 2 cases ». SF02 n’est pas ajouté implicitement à ces profils.

Le cas d’une réserve positive inférieure au coût, celui de zéro énergie devant
un obstacle et l’ordre réel des dépenses, tours et régénération restent à
traiter séparément. Les compteurs sont préparés pour la revue. Les modèles
acceptés et les galeries précédentes restent intacts.

Entrée native : `--apercu-laboratoire-competences --lot-furtivite-economie`.
Comparaison : `skill-animation-review-58/cote-a-cote.html`.
Aucun effet n’est raccordé au combat ou à la perception de la campagne.


## ANIM-S29 — Attaque sournoise et Discrétion renforcée

La revue 59 propose **65 / SF00-A02**, révision du rendu de Furtivité, et
**67 / SF02-A01** Discrétion renforcée. Le camouflage accepté dans la revue 58
reste intact : mêmes couches, balayage bleu acier, opacité et retour à
l’apparence normale. SF01 conserve sa propre référence acceptée.

Le bonus de cinq pour cent par rang a été accepté sous SF00-SNEAK01-ACCEPT01.
L’aperçu de rang dix prend une base préparée de vingt dégâts : l’attaque
lancée en furtivité affiche trente, dont dix de bonus. Le mode et son icône
prennent fin au lancement ; le projectile conserve ce bonus jusqu’à l’impact.
L’attaque suivante à découvert affiche vingt. Une réactivation préparée de
Furtivité permet ensuite de montrer une nouvelle attaque renforcée.
Les rangs onze et douze montrent respectivement trente-et-un et trente-deux.
Ces nombres ne définissent aucune nouvelle statistique ou variance réelle.

Les tirs et frappes de mêlée reprennent les impacts physiques et le sang
acceptés. L’exemple de sort direct de feu reprend les dessins de projectile
et d’explosion acceptés, sans sang. Il illustre seulement un impact magique :
la zone, la Brûlure, le coût et la recharge complets de Boule de feu ne sont
pas simulés par cette scène. Tous les impacts déclenchent la réaction commune
sur la cible, sans déplacement logique de sa case. Les nombres montrent le
total, le bonus et la composante physique ou de feu avec son petit symbole.

Discrétion renforcée, au rang effectif onze, reprend l’exemple accepté d’une
case de repérage, contre deux sans ce passif. Le libellé est « Distance de
repérage ». La valeur est mise à jour dans le survol de l’icône SF00 ; le passif
ne crée ni activation distincte ni seconde icône de buff temporisé.
Les deux profils de revue possèdent Économie au rang dix : un pas préparé
coûte donc une énergie. Le rang de Furtivité et son bonus restent distincts
du rang de Discrétion renforcée et de sa distance.

La comparaison montre les quatre directions, attente sans tour, mêlée, sort,
attaque à découvert, rangs supplémentaires, déplacement avant l’attaque et
cloison. La scène bloquée rend l’attaque indisponible dans le prototype ; elle
ne tranche pas la règle générale des attaques refusées. Aucun calcul d’IA,
écoute, visibilité, mémoire ou réaction ennemie n’est ajouté. Les rafales,
zones, canalisations, dégâts différés, critiques et ordres de modificateurs
restent à raccorder au résolveur réel. Les compositions nouvelles attendent
leur validation ; aucune nouvelle règle chiffrée n’est déduite des exemples.

Entrée native : `--apercu-laboratoire-competences --lot-furtivite-sournoise-discretion`.
Comparaison : `skill-animation-review-59/cote-a-cote.html`.
Le dossier `native-label-draft` reste un brouillon non présenté, exclu de
l’accord à venir. Les monstres provisoires seront adaptés après les animations.
Aucun effet n’est intégré au combat de campagne par cette galerie.

## SM05-ANIM05 — Résidu persistant accepté en revue 69

Le 14 octobre 2026, « ok parfait je valide » accepte le rendu corrigé de
la revue 69 en cases natives 64 px : sol enflammé, gaz désynchronisé entre
les cases et surface continue de neige, givre et glace. Aucun fond coloré
uniforme n’est ajouté ; la zone carrée respecte les obstacles.
Le Gel sur les victimes reprend exactement la référence GEL-A05 de revue 28.

Les identifiants sont `73 / SM05-A03-FEU`, `73 / SM05-A03-POISON` et
`73 / SM05-A05-FROID`, en terrain ouvert et contre une cloison.
L’explosion précède le marqueur persistant ; les tours explicites et
l’expiration gardent leurs règles. Les valeurs de dégâts préparées restent
des exemples et ne définissent aucune nouvelle statistique.

Preuve : `skill-animation-review-69/visual-acceptance-01.json`.
Les propositions précédentes et les autres candidats conservent leur
statut. SM04 et l’intégration au combat restent à traiter séparément.

## SM04-ANIM01 — Explosion étendue proposée en revue 71

`72 / SM04-A01` compare, dans les trois éléments, le rayon de base d’une
case avec le rayon de quatre cases déjà défini au rang onze. Les zones
restent carrées et les murs coupent l’explosion et le résidu. Les dessins
acceptés de la revue 69 et GEL-A05 sont repris ; l’aperçu agrandi et le
survol natif restent à valider.

Galerie : `skill-animation-review-71/cote-a-cote.html`.
Laboratoire : `--apercu-laboratoire-competences --lot-explosion-etendue`.
L’agrandissement des autres compétences sera proposé séparément, en tenant
compte de la demande de conserver des zones d’impact courtes en mêlée.

## ZONE-GROWTH01 / ZONE-VISION01 — Portées et cercle de vision

Le 14 octobre 2026, le joueur valide une progression de la portée avec le
rang de compétence, par paliers perceptibles. Elle concerne toutes les
compétences magiques et d'attaque à distance disposant d'une portée, ainsi
que les compétences de mêlée permettant de rejoindre une cible ou
d'échanger sa position. Pour les zones, portée de lancement et surface
doivent progresser de façon cohérente. Les zones d'impact de mêlée restent
courtes, même lorsqu'une charge ou un échange gagne en distance.

Le carré de 3x3 sans effet autour du joueur est abandonné. Le placement
reste choisi par le joueur, le décor coupe les zones et la prévisualisation
montre les cases réellement affectées. Le contournement d'un obstacle
isolé par le prototype de mines reste à corriger et à présenter.

Le joueur demande aussi d'agrandir le cercle visible pour accompagner ces
portées. La base précédemment acceptée était fixe à huit cases. Le nouveau
rayon et son éventuel mode de progression restent à définir et à présenter ;
aucune croissance automatique par rang de sort ou attribut n'est déduite.
La revue devra conserver le cercle, les obstacles qui limitent la vision,
l'intérêt de l'exploration et vérifier le cadrage en cases de 64 px.

Les nouvelles valeurs chiffrées et le rendu restent à valider. Ce relevé
de décisions ne modifie pas le moteur, le contenu de campagne ou les sauvegardes,
et ne vaut pas acceptation visuelle de la revue 71.

Relevé : `zone-growth-melee-limit-discussion-01.json`.

Le même jour, « oui ca me va :) » accepte le principe d'un agrandissement
mesuré du cercle visible pour accompagner les nouvelles portées, en conservant
le blocage par les obstacles et l'intérêt de l'exploration. Le nouveau rayon
et le cadrage en 64 px devront être prévisualisés. Cette réponse n'approuve
aucun rayon chiffré, aucun rendu nouveau ni aucune croissance automatique
à chaque rang de compétence. L'intégration au moteur reste à venir.

## ZONE-GROWTH02 / ZONE-VISION02 — Mise en place et laboratoire

Le joueur demande ensuite « parfait on met tout ca en place puis on continue :) ».
La délégation SKILL-RANK-DELEGATE01 permet de régler les paliers numériques
sans une validation de chaque rang. Le catalogue de la fenêtre U reçoit
les progressions provisoires de portée et de surface : 46 compétences,
62 paramètres spatiaux. Les valeurs au rang un sont conservées ; les paliers
continuent avec les bonus d'équipement au-delà de dix rangs appris.

Les portées progressent d'une case par palier de trois rangs. Les largeurs
et longueurs de mur progressent de deux cases par palier pour conserver
leurs dimensions impaires. Les tirs liés à l'arme gagnent une portée
supplémentaire propre à la compétence, sans changer les tirs ordinaires.
Le cercle rituel conserve ses huit segments au premier rang puis étend
son périmètre avec son rayon. Les mines et grenades conservent leurs actifs
achetables une fois et leurs passifs dédiés ; leurs courbes déjà acceptées
ne sont pas remplacées. Dégâts, coûts, durées, recharges et statuts sont conservés.

La géométrie partagée coupe les zones aux murs et aux obstacles isolés,
avec protection des cases derrière l'obstacle et des coins fermés. Les
formes rondes, carrées, directionnelles et les périmètres sont distincts ;
les couloirs coupent aussi les vagues voie par voie. L'explosion des mines,
son résidu et son survol utilisent la même emprise limitée par les obstacles.
La mêlée conserve ses impacts courts. Les distances de Charge, du Bond et
d'Échange de position progressent indépendamment de ces impacts.

Le nouveau laboratoire de portée montre un candidat de vision fixe à
douze cases face à la référence de huit. Ce nombre et son rendu restent
soumis à la revue ; la vision de campagne n'est pas modifiée. La carte
conserve le brouillard, les obstacles et les acteurs masqués. Le ciblage
est libre, avec refus des centres hors portée, masqués ou bloqués. Les
cases sont en 64 px réels avec déplacement de la caméra et vue d'ensemble.
Les portées d'armes utilisent une arme d'exemple de six cases dans ce test.

Galerie interactive : `skill-spatial-review-01/index.html`.
Relevé : `skill-spatial-review-01/progressions.json`.
Laboratoire natif : `--apercu-laboratoire-competences --lot-portees-zones-vision`.
Les descriptions comparent le rang actuel au prochain rang appris en
gardant le bonus d'équipement. À dix rangs appris, elles indiquent le plafond.
Ce lot teste la portée et l'emprise ; il ne remplace pas les animations
acceptées et ne raccorde pas à lui seul les nouveaux effets au combat.

## ZONE-VISION02 — Cercle de douze cases validé et intégré

Le 14 octobre 2026, « ok ca me semble bien :) on peut continuer » valide
le rayon fixe de douze cases et le cercle montré dans la revue spatiale.
Les nouvelles parties et le laboratoire courant l'utilisent à partir de
la génération 147. La vision ordinaire des PNJ conserve huit cases ; leurs
profils particuliers restent inchangés. Les murs et le cercle continuent
de limiter la visibilité. Aucun gain automatique par rang n'est ajouté.

Les anciennes parties, jusqu'à la génération 146, conservent leurs règles
de vision et leurs empreintes de sauvegarde. Aucun fichier de sauvegarde
du joueur n'est modifié. Deux tests vérifient le rayon actuel, la détection
ordinaire des PNJ et la restauration d'une suspension de génération 146.
Le contrôle de compilation verrouillé de toutes les cibles réussit.

Accord : `skill-spatial-review-01/vision-acceptance-01.json`.
Cette validation ne vaut pas acceptation de l'animation SM04-A01 en revue 71.

## ANIM-S33 — Grenade explosive et portée de lancer en revue 72

Le nouveau lot propose `74 / SG00-A01 · Lancer de grenade` et
`77 / SG03-A01 · Portée de lancer`. Une petite grenade métallique voyage
vers la cible, puis produit une brève explosion de fragments et de fumée.
Les blessures physiques reprennent le sang accepté et le mouvement d'impact.
La grenade reste un effet éphémère : aucune zone persistante n'est ajoutée.

Le passif augmente seulement la distance disponible : quatre cases sans
le passif et huit avec son rang effectif onze. Il utilise le même projectile,
la même explosion et les mêmes dégâts. Les chiffres de démonstration restent
préparés : cibles à cent PV, impact physique de vingt-quatre, coût de huit
énergie et recharge de deux tours. Ils ne constituent pas un nouvel équilibrage.

La galerie présente les quatre directions et quatre scènes : deux groupes
à des distances différentes, explosion au bord d'une cloison, trajet bloqué
et cible près du joueur. La prévisualisation clignote avant confirmation,
puis disparaît ; les obstacles coupent le dessin et le joueur ne reçoit pas
ses propres dégâts. Un refus ne consomme ni énergie ni tour. Attendre ne
fait pas progresser les compteurs ; le bouton Tour suivant passe une action
préparée. Le bouton Rejouer réinitialise la démonstration.

Galerie animée réelle : `skill-animation-review-72/cote-a-cote.html`.
Laboratoire : `--apercu-laboratoire-competences --lot-grenades-portee`.
Les deux sources transparentes sont produites avec imagegen intégré ;
les prompts sont conservés dans `skill-animation-review-72/generation.json`.
Les 32 vues proviennent du rendu natif à cases de 64 px. La galerie est
vérifiée dans le navigateur et ses douze échantillons de canvas sont
identiques aux captures natives. Les murs sont vérifiés sur chaque vue.

Ce lot attend la validation visuelle du joueur ; ses effets ne sont pas
intégrés au combat de campagne. Il reste 36 compétences sans aperçu animé.
Les autres variantes de grenade seront présentées dans les lots suivants.

## AOE-FALLOFF01 — Carré de grenade et dégâts dégressifs proposés

Le joueur demande que la grenade couvre un carré de 3×3 et que les dégâts
baissent en s'éloignant de l'épicentre. Il souhaite examiner cette règle
pour les attaques magiques de zone, afin de conserver l'intérêt des attaques
physiques de mêlée et de tir. Le nouveau prototype de revue 73 présente
`74 / SG00-A02` et `77 / SG03-A02` ; la revue 72 est conservée.

Le profil proposé conserve les dégâts pleins au centre et la moitié sur
le bord. Dans le carré de 3×3, les huit voisins, coins compris, prennent
donc les mêmes dégâts réduits : vingt-quatre au centre et douze autour
dans cette démonstration. Le passif de portée ne change pas ces nombres.
Dans les carrés plus grands, les couches intermédiaires diminuent
progressivement entre le centre et le bord. Le rayon de référence reste
celui de la zone complète, même si les murs retirent des cases.

Les dégâts sont entiers, arrondis à l'inférieur avant les défenses.
L'aperçu expose cette proposition numérique sans modifier les règles
de combat, les contrôles, le catalogue ou les sauvegardes. Les dessins
existants sont repris ; seul l'impact du prototype couvre davantage le
carré demandé. Les murs continuent à couper le dessin et le joueur est
protégé de ses propres dégâts. La prévisualisation disparaît au lancer.

Le schéma interactif compare les dégâts de chaque couche pour des carrés
de 3×3 à 11×11. La dégressivité réduit les dégâts de chaque cible éloignée,
mais ne limite pas leur nombre : avec cent dégâts au centre, un carré
11×11 rempli d'ennemis cumule encore 7 700 dégâts, contre 12 100 uniformes.
L'équilibrage doit donc aussi considérer les dégâts de base, le coût,
la recharge et la durée. Les souffles, vagues, dégâts persistants et
effets de contrôle devront avoir une application adaptée, examinée
avant de généraliser une règle de centre à leur fonctionnement.

Galerie : `skill-animation-review-73/cote-a-cote.html`.
Proposition chiffrée : `skill-animation-review-73/proposal.json`.
Laboratoire : `--apercu-laboratoire-competences --lot-grenades-degressives`.
Les nombres 100 % / 50 % et le nouveau rendu attendent la revue du joueur.
Ce lot révise deux aperçus déjà montrés : le nombre restant demeure 36.

## AOE-FALLOFF01 — Validation et règle commune

« parfait :) on le fait alors :) » valide le carré de grenade de 3×3,
le rendu SG00-A02 / SG03-A02 et les dégâts de 100 % au centre à 50 % sur
les cases les plus éloignées. L'accord et les empreintes des 4 024 captures
et GIFs présentés sont conservés dans
`skill-animation-review-73/visual-and-numeric-acceptance-01.json`.

Le résolveur partagé de `skills::area_damage` applique une seule réduction
avant les défenses. Le carré utilise ses couronnes de cases, le cercle la
distance euclidienne et le losange la distance de sa forme. Le calcul reste
entièrement entier, y compris pour les cercles. Un mur retire les cases
atteintes sans renormaliser le rayon ni renforcer celles qui restent.
Chaque composante de dégâts conserve son type et sa pénétration.

Dix applications du catalogue sont raccordées à ce profil : Lancer de
grenade, explosion initiale des mines, Déflagration occulte, Émanation
soudaine, Boule de feu, Nova de givre, impacts de Blizzard, Explosion
cristalline, Autodestruction et Pulsation occulte du Possédé. Pour cette
dernière, le Trait corporel et le Drain organique ne sont pas modifiés.
Les anciennes réductions d'Émanation soudaine et d'Explosion cristalline
sont remplacées par le profil spatial, sans les multiplier une seconde fois.

La fenêtre U distingue dégâts au centre et dégâts en bordure. Le laboratoire
de grenade utilise désormais le carré validé par défaut ; l'option
`--historique-grenades-uniformes` conserve le profil antérieur pour comparaison.
Les explosions des mines utilisent aussi le calcul commun. Les chances,
durées et dégâts des statuts et des résidus ne sont pas modifiés par cette étape.
L'adaptation des cônes, vagues et dégâts persistants reste distincte.

Le laboratoire spatial affiche les pourcentages conservés par case. Les
captures antérieures restent archivées. Le raccordement des nouvelles
actions au combat réel reste celui prévu après les revues d'animations ;
aucune action ancienne ni sauvegarde n'est convertie implicitement.

## ANIM-S34 — Grenade aveuglante, Leurre et Puissance des grenades

Candidats 75 / SG01-A01, 76 / SG02-A01 et 78 / SG04-A01 ; nouveaux
rendus à valider dans `skill-animation-review-74/cote-a-cote.html`.
Le lancer explosif SG00-A02 sert uniquement de référence déjà validée.
Les dessins cylindriques blanc cassé et bleu des deux variantes, ainsi
que le projectile explosif et son atlas, sont réutilisés sans réécriture.

SG01 présente un bref éclat blanc, une réaction d'impact et un œil barré
avec les tours restants sur les cibles affectées. Les applications sont
préparées sur des mannequins ordinaires : aucun nouveau jet de résistance,
ni aucune immunité du joueur à son propre aveuglement ne sont définis ici.
Le rang effectif 11 montre douze tours et une précision réduite de 30 %.

SG02 montre la grenade bleue, des ondes ambrées et deux tours d'émission.
Le rang 1 est choisi dans cet aperçu pour rendre les limites du champ
sonore visibles sur une petite carte. La propagation et l'atténuation
viennent de `StealthRules::sound_field_on_map`, sans cercle de substitution.
Les mannequins attirés se rapprochent lors des tours suivants et continuent
après la fin du bruit. Les priorités de combat, élites et occupations du
point d'impact restent hors de cette simulation de revue.

SG04 conserve le dessin, la zone et le rythme de l'explosion validée.
Le rang effectif 11 compare 24 / 12 dégâts sans passif à 50 / 25 avec
le bonus de 110 %, dans cet exemple entier avant défenses. Aucun lancement
de buff et aucun dégât des variantes utilitaires ne sont ajoutés.

Les trois scènes existent dans quatre directions en taille native de
64 px. La prévisualisation disparaît après confirmation ; un lancer bloqué
ne consomme rien. Les durées suivent les tours et non l'attente réelle.
Le laboratoire est accessible avec `--lot-grenades-utilitaires` et demeure
séparé de la campagne. Trois nouvelles compétences sont présentées,
laissant 33 animations encore à montrer ; cet ajout ne vaut pas validation.

## ANIM-S34-REV02 — Grenades compactes et œil barré

Le joueur trouve les grenades du plateau trop grandes et détaillées et
demande un véritable œil barré pour le statut d'aveuglement. La revue 75
propose SG01-A02, SG02-A02 et SG04-A02, avec un projectile SG00-A03 dans
la comparaison sans passif. Les numéros de compétences sont conservés.

Une planche RGBA issue d'Imagegen intégré remplace les dessins du plateau :
corps en aplats, ombres simples, pas de textures ni de micro-détails.
Les deux cylindres blanc cassé et bleu et la fragmentation gris/rouge
gardent leurs identités ; leurs silhouettes font environ 18 pixels de haut
sur les cases de 64 pixels. Les icônes des compétences restent intactes.
Le symbole d'aveuglement est un œil en amande avec une pupille visible
et une barre rouge diagonale contrastée ; les tours sont affichés à côté.

Seuls les dessins sont repris. Les 48 chronologies de compteurs et d'effets
doivent rester identiques à la revue 74. Les images non affectées sont
réutilisées à l'identique, avec leur provenance. Les anciennes galeries
et sources sont conservées. Nouvelle galerie :
`skill-animation-review-75/cote-a-cote.html`, en attente de validation.
`--historique-grenades-detaillees` conserve l'ancien rendu du laboratoire.
Les règles, durées, dégâts, ondes et explosions ne changent pas ; le nombre
d'animations encore à présenter reste 33. Aucune intégration en campagne.

## ANIM-S34-REV02-ACCEPT01 — Validation du lot corrigé

« parfait je valide » confirme les rendus de la revue 75 : Grenade
aveuglante SG01-A02, Leurre SG02-A02 et Puissance des grenades SG04-A02,
ainsi que le projectile simplifié SG00-A03 utilisé dans la comparaison.
Les trois grenades du plateau gardent leurs couleurs et silhouettes
distinctes, en aplats simples, avec une hauteur d'environ 18 pixels.
Le marqueur d'aveuglement devient l'œil en amande avec pupille visible,
barre rouge contrastée et tours restants à côté.

L'accord et les empreintes des images affichées et de leurs sources natives
sont conservés dans `skill-animation-review-75/visual-acceptance-01.json`.
Les anciens rendus restent archivés. Les icônes de l'arbre, règles, dégâts,
durées et probabilités ne sont pas modifiés. Les valeurs préparées ne
constituent pas une nouvelle décision d'équilibrage, et l'éligibilité du
joueur à son propre aveuglement demeure distincte. Le raccordement en
campagne garde son calendrier prévu après les revues. Il reste 33 animations
à présenter ; cette validation ne crée aucun nouveau candidat.

## Aperçus réutilisables — 14 octobre 2026

Le joueur demande de réutiliser la même page après validation d'un lot,
pour éviter l'accumulation des galeries. L'adresse courante devient
`artifacts/refonte-agent-2026-10-06/apercu-animations.html`.
`python tools/art/publish_current_animation_review.py NUMERO` remplace cette
page à partir du lot choisi, sans copier les images, scripts ou GIF et sans
modifier le lot validé. Utiliser cette adresse pour présenter les prochains lots.

Les captures temporaires devront également être réutilisées pour les futurs
lots, plutôt que conservées systématiquement à chaque essai. Conserver les
dessins sources, le code, les paramètres et les décisions de validation,
avec un aperçu final compact. Une page unique ne réduit pas à elle seule
le poids des images déjà accumulées. Les anciens fichiers sont traités dans
un plan de nettoyage distinct, présenté au joueur avant suppression.

## ANIM-S35 — Égide occulte et Barrière d’épine proposées

Reprise du 14 octobre 2026 : candidats `79 / B02-A01` et `80 / SU03-A01`.
La page `artifacts/refonte-agent-2026-10-06/apercu-animations.html` est remplacée
directement ; ses atlas compacts sont dans `apercu-animations-data/`. Les dessins
sources et paramètres sont conservés dans `skill-animation-review-76/`.
Les captures de travail sont retirées après conditionnement et vérification.

L’Égide montre un sceau violet franc qui apparaît rapidement au lancement et
réagit brièvement aux impacts directs. La Barrière montre une couronne
d’épines brunes à pointes claires, déployée au lancement puis projetée vers
l’extérieur lors du contact de mêlée. Ces deux nouveaux dessins viennent
d’Imagegen intégré, avec fond transparent et centre libre pour conserver
la lisibilité du personnage. Leur animation reste à valider.

Les effets persistants sont indiqués dans la barre d’état avec l’icône
acceptée et les tours restants. Aucune coque permanente ne masque le corps.
La réaction commune d’aller-retour concerne le joueur touché et l’attaquant
qui reçoit les épines. Les blessures physiques reprennent le sang déjà
accepté ; l’exemple de dégât persistant n’en projette pas.

Les valeurs proviennent du catalogue actuel au rang effectif 11 : Égide
réduit les dégâts directs de 55 %, pendant quatre tours, pour treize énergie
et huit tours de recharge ; Barrière renvoie 40 % des dégâts de mêlée reçus,
pendant cinq tours, pour quinze énergie et dix tours de recharge. Les exemples
préparés de vingt dégâts donnent respectivement neuf dégâts subis ou huit
renvoyés. Le contact de dix dégâts persistants n’est ni réduit ni renvoyé.
Ces entiers ne définissent aucune nouvelle variance, statistique de monstre,
défense ou règle générale d’arrondi et ne modifient pas les courbes.

Seize vues natives de 64 px couvrent les quatre directions de contact,
le tir direct, les dégâts persistants, l’attente sans tour et le lancement
refusé sous Silence. La confirmation retire la preview. L’attente réelle ne
réduit aucune durée ; les tours explicites font expirer les buffs. Un second
coup après expiration montre l’absence de protection ou de riposte.

Laboratoire isolé : `--apercu-laboratoire-competences --lot-protections-survie`.
L’aperçu navigateur utilise les mêmes pixels natifs, assemblés sans perte.
Le compte passe de 33 à 31 compétences encore à présenter. Il ne s’agit pas
d’une validation ni d’une intégration en campagne. Les précédents candidats,
icônes, personnages et règles conservent leur statut.

### ANIM-S35-ACCEPT01 — validation et sang des épines

Le joueur valide explicitement les candidats 79 / B02-A01 et 80 / SU03-A01.
Les dégâts renvoyés par Barrière d’épine doivent produire le sang physique
déjà accepté sur la victime vivante, monstre ou joueur ; les machines gardent
leur règle sans sang. Le prototype affiche déjà ce sang sur l’attaquant blessé.
L’accord et les empreintes figurent dans
`skill-animation-review-76/visual-acceptance-01.json`.

Le catalogue apprend désormais les rangs mais ses nouveaux effets restent à
raccorder au moteur, au bouton Utiliser et au menu clic droit. Après cette
explication, le joueur choisit explicitement de terminer les aperçus d’animations
avant cette intégration jouable. Ne pas activer les anciennes techniques sous
les noms du nouveau catalogue.

## ANIM-S36 — Régénération et Purge proposées

Candidats `81 / SU04-A01` et `82 / SU05-A01`, préparés le 15 octobre 2026
dans le laboratoire isolé `--lot-soins-survie`. La même page
`artifacts/refonte-agent-2026-10-06/apercu-animations.html` présente le lot.
Les sources PNG transparentes, prompts et paramètres sont conservés dans
`skill-animation-review-77/` ; le rendu natif reste à valider.

Régénération montre des fils verts lumineux autour du corps et de courts
points de suture. Le personnage conserve son dessin. L'effet reste animé
tant que la canalisation préparée existe, sans soin supplémentaire lié au
temps réel. Chaque maintien explicite produit son soin et sa dépense.
La barre d'état indique les maintiens restants ; une blessure physique
interrompt le soin avec l'aller-retour et le sang communs.

Purge balaie la silhouette du haut vers le bas avec un trait blanc ivoire
et expulse des fragments d'altération. Le poison et la brûlure préparés
disparaissent avec leurs marqueurs ; le dessin de brûlure accepté est placé
derrière le personnage. La scène de réapplication fait revenir le poison,
avec la réaction commune à un effet négatif, sans représenter d'immunité
ni rendre de vie. Les marqueurs réutilisent la goutte de SM02 et le feu de
FE06 ; aucun fichier d'icône du catalogue n'est remplacé.

Les valeurs proviennent du catalogue actuel au rang effectif onze : trois
soins au maximum, quatorze pour cent des PV maximum et six énergie par soin,
recharge huit pour Régénération ; trois altérations au maximum, une énergie
et recharge six pour Purge. Avec cent PV maximum et quarante au départ,
l'exemple montre 54, 68 puis 82 PV ; Purge conserve quarante PV. Ce sont des
compteurs préparés et non des statistiques nouvelles ou une règle d'arrondi.
Les dégâts périodiques du poison et de la brûlure ne sont pas simulés.
Deux altérations compatibles évitent de décider une égalité de priorité,
une interaction avec Silence ou les malus d'équipement.

Douze scènes couvrent le déroulement complet, l'attente sans tour, puis
quatre origines de blessure ou de nouvelle altération. La preview disparaît
au lancement. Le navigateur reprend les pixels du laboratoire sans perte,
avec pause, ralenti et mode manuel. Les captures de travail sont supprimées
après vérification ; les dessins, le code et les atlas compacts sont gardés.
Vingt-neuf compétences restent ensuite à présenter. Ce lot ne raccorde pas
encore les effets au combat, conformément à la priorité choisie.

### ANIM-S36-REV02 — Purge sur le modèle du soin

Le 15 octobre, le joueur apprécie l'effet de soin 81 / SU04-A01 et demande
de conserver cette direction. Il refuse 82 / SU05-A01 et demande un effet
semblable au soin, d'une autre couleur, accompagné d'un texte au-dessus de
la tête. L'accord ne porte pas sur de nouveaux chiffres ou règles.
Le retour est conservé dans `skill-animation-review-77/visual-feedback-01.json`.

Le candidat 82 / SU05-A02 remplace le balayage blanc et les icônes expulsées
par une impulsion de fils blanc doré autour du corps, avec le même mouvement
que les fils verts de Régénération. Le texte « Purge », centré au-dessus de
la tête, est plus grand et contrasté. L'effet bref retire les altérations
préparées sans donner de soin, durée de canalisation ou immunité.
La palette est une proposition ; le nouveau rendu reste à valider.

La source PNG vient d'une variante Imagegen du dessin de soin. La source
verte et ses six aperçus compacts restent identiques. Une nouvelle capture
native de référence compare les 112 images du soin aux images précédentes.
Les autres captures ne refont que Purge ; les compteurs sont comparés à
ceux de la revue 77. Les sources, prompts et contrôles sont conservés dans
`skill-animation-review-78/`, la page réutilisable reste `apercu-animations.html`.
Il reste 29 compétences à présenter ; cette révision n'en ajoute aucune.

### ANIM-S36-REV03 — validation et Purge présentée seule

Le joueur valide les fils blanc doré et le texte de Purge, avec une demande
explicite de retirer les flammes derrière le joueur et les autres effets
de démonstration. Cet accord est conservé dans
`skill-animation-review-78/visual-acceptance-01.json`.

Le nettoyage 82 / SU05-A03 retire la couche de brûlure préparée avant le
lancement, ainsi que l'injection ultérieure d'un nouveau poison dans cet
aperçu. Les statuts à nettoyer restent indiqués dans la barre d'état avant
Purge. Le joueur ne montre que le dessin de son corps, les fils dorés et
« Purge » lors de l'activation. La réapplication n'est plus provoquée dans
la comparaison ; ceci ne crée aucune immunité en jeu.

Le dessin doré, sa pulsation et son texte restent identiques. Les 112 images
de référence du soin sont comparées à la revue précédente, et les images
de la pulsation dorée aux images validées de la revue 78. Le renderer commun
de brûlure, les règles de combat et les autres compétences sont conservés.
La page réutilisable est actualisée ; les contrôles sont dans
`skill-animation-review-79/`. Aucun nouveau candidat n'est ajouté.

Le joueur valide ensuite l'aperçu nettoyé et demande de poursuivre en
nettoyant les fichiers de travail entre les lots. L'accord final est dans
`skill-animation-review-79/visual-acceptance-01.json`. Douze atlas compacts
des versions refusée SU05-A01 et remplacée SU05-A02 sont supprimés,
soit 4,37 Mio ; sources, code, contrôles et accords sont conservés.

## ANIM-S37 — Leurre holographique et Traversée de phase proposés

Candidats `83 / ES03-A01` et `84 / ES05-A01`, préparés le 15 octobre 2026.
Le laboratoire isolé est accessible avec `--lot-esquive`. L'aperçu réutilise
`artifacts/refonte-agent-2026-10-06/apercu-animations.html` ; les données de
revue et les contrôles restent dans `skill-animation-review-80/`.
Les deux rendus attendent l'accord visuel du joueur.

Le Leurre holographique compose le dessin porté existant dans un canevas
transparent de 128 pixels pour garder ses débordements, avec le corps prévu
pour une case native de 64 pixels. Il montre une copie bleutée translucide
animée par de fines bandes verticales. La copie ne perd pas son arme et
aucun dessin de tenue n'est remplacé. Un premier impact préparé disperse
le double sans sang ; la scène de durée le conserve pendant ses trois tours.
Une diversion réussie préparée déclenche l'aller-retour commun sur l'ennemi
ordinaire, sans lui infliger de dégâts. Aucun algorithme d'IA, nouveau jet
de diversion ou règle de priorité n'est ajouté.

La Traversée de phase fragmente brièvement la silhouette en bandes
horizontales avec des rémanences violettes. Le mouvement rejoint la case
derrière l'ennemi adjacent en 0,24 seconde ; le corps conserve sa pose
frontale et son équipement. Le mouvement ne blesse ni ne contrôle l'ennemi.
Le marqueur du bonus d'esquive reste deux tours puis disparaît.

Les exemples utilisent cette fois le rang effectif un du catalogue actuel :
le leurre dure trois tours, coûte huit énergie et recharge en six tours,
avec une portée de placement de trois cases. La traversée parcourt deux
cases, apporte dix points d'esquive pour deux tours, coûte dix énergie et
recharge en huit tours. La scène ne change pas les progressions de portée
acceptées à d'autres rangs. Un mur sur la case d'arrivée interdit le
lancement sans tour, énergie ou recharge dépensés.

Quatorze scènes montrent les quatre directions, la fin de durée, l'attente
sans nouveau tour et le refus devant un mur. La preview disparaît dès la
confirmation. Le navigateur rejoue les pixels natifs sans perte, avec
pause, ralenti et mode manuel. Les captures et preuves de travail sont
retirées après les contrôles ; les atlas compacts de ce lot occupent
environ 3,77 Mio. Aucun effet de campagne n'est raccordé par cet aperçu.
Vingt-sept compétences restent ensuite à présenter, en plus de l'accord
encore attendu sur ces deux candidats.

### ANIM-S37-ACCEPT01 — validation visuelle

Le joueur répond « top :) » après l'aperçu du lot 80. Les rendus
`83 / ES03-A01` et `84 / ES05-A01` sont validés ; l'accord est conservé
dans `skill-animation-review-80/visual-acceptance-01.json`. Cette validation
ne modifie pas les règles du catalogue et ne raccorde pas les effets à la
campagne. Les captures temporaires ont déjà été supprimées. Vingt-sept
compétences restent à présenter.

## ANIM-S38 — Réflexes balistiques et Pas spectral proposés

Candidats `85 / ES06-A01` et `86 / T02-A01`, préparés le 15 octobre 2026.
Le lot termine les aperçus de la branche Esquive. Il utilise le laboratoire
isolé `--lot-pas-reflexes` et la même page `apercu-animations.html` ; les
contrôles sont conservés dans `skill-animation-review-81/`. Les deux
rendus restent à valider par le joueur.

Réflexes balistiques montre un résultat d'esquive préparé : un projectile
passe près du personnage, qui s'écarte brièvement sur le côté puis retrouve
sa position exacte. Une rémanence bleutée et le texte « Esquive » accompagnent
le mouvement. Le passif conserve son bonus permanent au score contre les
projectiles ; il ne crée ni aura, ni buff temporisé, ni coût ou recharge.
La scène de tir réussi montre qu'un bonus ne garantit pas l'évitement.
Les deux résultats sont préparés pour la revue, sans nouveau jet de
précision, pourcentage d'esquive ou règle de projectile automatique.

Pas spectral rejoint une case libre en 0,18 seconde, avec de courtes
silhouettes rémanentes. Un contour bleu clair reste autour du personnage
pendant le bonus, sans remplacer sa tenue, son arme ni sa pose frontale.
Il disparaît à l'expiration mesurée en tours, même si le temps réel continue
à s'écouler. Dans l'exemple touché, le joueur conserve les dégâts, le sang
physique et la réaction d'aller-retour communs, malgré son bonus d'esquive.
Les douze dégâts de cet exemple ne constituent pas un équilibrage.

Le rang effectif un reprend le catalogue actuel : douze points d'esquive
contre les projectiles pour le passif ; un déplacement d'une case, quinze
points d'esquive pendant deux tours, huit énergie et six tours de recharge
pour Pas spectral. Les courbes de portée acceptées restent conservées.
Un mur sur la destination bloque le Pas spectral sans tour, énergie ou
recharge dépensés. Le projectile de l'autre scène s'arrête contre ce mur,
sans blessure ni mouvement d'esquive déclenché derrière l'obstacle.

Quatorze scènes couvrent les quatre directions, un tir qui touche,
l'attente sans nouveau tour et les obstacles. La preview disparaît à la
confirmation. Les contrôles vérifient les pixels natifs de 64 pixels,
les durées explicites et le mode manuel. La page est réutilisée et les
captures temporaires sont supprimées après vérification. Aucun effet
de campagne n'est raccordé par ce lot. Vingt-cinq compétences restent
ensuite à présenter, en plus des deux rendus proposés ici.

### ANIM-S38-ACCEPT01 — validation et rappel de la portée

Le joueur accepte les deux rendus et demande de confirmer que Pas spectral
permet de parcourir plus d'une case. Le catalogue actuel et ZONE-GROWTH02
confirment une portée d'une, deux, trois, quatre puis cinq cases aux rangs
effectifs un, quatre, sept, dix et treize. L'aperçu du lot utilisait le rang
un ; il ne remplace pas la progression déjà acceptée. L'accord visuel et
ce rappel sont conservés dans `skill-animation-review-81/visual-acceptance-01.json`.
Aucun nouvel équilibrage ni raccordement au combat n'est ajouté.

### SPECTRAL-RANGE01 — distance doublée et arrivée au choix

Le joueur demande ensuite de doubler la portée de Pas spectral et de lui
laisser choisir une distance plus courte. Cette demande remplace les
anciennes valeurs de portée : deux, quatre, six, huit et dix cases maximum
aux rangs effectifs un, quatre, sept, dix et treize. Le joueur choisit sa
case d'arrivée dans cette limite ; atteindre la distance maximale n'est
pas obligatoire. La destination et le trajet doivent rester légaux.
Les murs et obstacles bloquent toujours le passage.

Le catalogue et son conditionnement conservent cette révision, décrite dans
`spectral-step-range-and-choice-decisions.json` et la fiche `T02-03`.
La fenêtre affiche « Déplacement maximum ». Le laboratoire spatial accepte
les destinations plus proches, refuse une destination trop éloignée, le
déplacement nul et les trajets bloqués. Le dessin validé n'est pas changé :
l'animation du lot 81 reste un exemple de distance choisie d'une case,
avec un maximum de deux au rang un. Les autres paramètres, l'énergie par
utilisation et la recharge sont conservés. Le raccordement des effets au
combat reste prévu après la fin des aperçus d'animations.

## ANIM-S39 — Vitalité améliorée et Dernier sursaut proposés

Candidats `87 / B03-A01` et `88 / SU06-A01`, préparés le 15 octobre 2026
dans le laboratoire isolé `--lot-passifs-survie`. Les deux rendus restent
à valider. La page `apercu-animations.html` est réutilisée et les contrôles
restent dans `skill-animation-review-82/`.

Vitalité améliorée étend la jauge de vie sur une échelle fixe, avec un
segment supplémentaire souligné et une pulsation brève de l'icône acceptée
dans l'interface. Le corps et les armes conservent leurs dessins. Aucun
soin, aura corporelle ou buff temporisé n'est créé pour ce passif permanent.
Le démarrage de l'exemple prépare une mise à jour de statistiques, pas une
nouvelle commande d'activation ou un achat de rang simulé.

Dernier sursaut montre un bref contour doré au sauvetage, une pulsation de
son cœur compact déjà accepté à côté du personnage et le nom au-dessus
de lui. Les effets s'éteignent après le déclenchement. Le coup produit la
réaction commune et le sang physique ; le total affiché indique les PV
réellement perdus. L'icône de disponibilité indique « Prêt », puis la
recharge, sans simuler une durée d'invulnérabilité ou un buff protecteur.

Les valeurs viennent du catalogue au rang effectif onze : Vitalité apporte
cinquante pour cent de PV maximum. Dans l'exemple de base à cent, 60/100
devient 60/150 sans soin. Dernier sursaut laisse un PV et recharge en
soixante tours. Un coup préparé à trente sur les vingt PV initiaux enlève
donc dix-neuf PV lors du sauvetage. Ce n'est pas une nouvelle formule de
dégâts ou un changement de sa réduction. Les passifs ne dépensent aucune
énergie dans ces exemples.

Seize scènes couvrent quatre directions de coup, l'attente sans nouveau
tour, un dégât de poison persistant, une recharge déjà en cours et un
deuxième coup. Le poison ne déclenche pas le sauvetage ; la recharge et le
second coup peuvent aussi laisser le personnage mort. L'exemple en recharge
commence avec trois tours restants préparés ; cette valeur ne remplace pas
la recharge du catalogue. Un acteur à zéro PV est atténué et marqué « Mort »,
sans proposer un nouveau cadavre ni une animation de mort définitive.

Les boutons et compteurs restent ceux d'un aperçu préparé, sans simulation
de priorité entre plusieurs protections ou de poison complet. Le temps
réel ne soigne pas et ne réduit pas la recharge. Le lecteur reprend sans
perte les pixels natifs de 64 pixels, avec pause, ralenti et étapes manuelles.
Les captures de travail sont retirées après vérification. Aucun effet de
campagne n'est raccordé. Vingt-trois compétences restent ensuite à présenter,
en plus des deux rendus proposés ici. SPECTRAL-RANGE01 reste conservé.

## ANIM-S40-ACCEPT01 — passifs de puissance

Le joueur valide les candidats 89 / A03-A01, Canalisation, et
90 / M03-A01, Puissance métamorphe. L’accord visuel est conservé dans
`skill-animation-review-85/visual-acceptance-01.json`. Les valeurs préparées
illustrent les paramètres du catalogue ; elles ne créent ni variance d’arme,
ni nouveau déclenchement passif, ni intégration au combat. Vitalité améliorée
reste à redéfinir séparément. Vingt et une compétences restent à présenter.

## ANIM-S41 — Interférence et Faille d’armure proposées

Candidats 91 / O02-A01 et 92 / AF02-A01. Le laboratoire isolé
`--lot-affaiblissements-ouverture` montre l’application d’un affaiblissement
sur un monstre ou sur le joueur, sans dégâts ni sang à cette application.
La réaction d’aller-retour commune accompagne l’application réussie.
Un bref impact occulte reprend l’atlas déjà validé. Interférence conserve
des arcs électroniques discontinus autour de la victime ; Faille d’armure
dessine des fractures de rouille sur sa protection, sans remplacer sa tenue.
Leur icône existante apparaît au-dessus de la cible, avec les tours restants.
Ces marqueurs et les effets corporels disparaissent à l’expiration.

Les exemples au rang un reprennent les valeurs du catalogue : trois tours,
cinq tours de recharge, huit énergie et quinze pour cent de réduction des
dégâts directs pour Interférence ; neuf énergie et vingt pour cent de réduction
d’armure physique pour Faille d’armure. La durée est préparée sans modificateur
de cible. Les scènes ne calculent pas encore de coups sous affaiblissement.
Le résultat résisté est préparé sans nouveau jet ; les dépenses qui l’illustrent
ne décident pas les règles encore en discussion. L’obstacle refuse le lancement.

Seize scènes couvrent les quatre directions, le joueur touché, l’attente,
le mur et la résistance. L’attente réelle conserve les tours ; seules les
étapes explicites du lecteur font avancer la durée et la recharge. La zone
de prévisualisation disparaît au lancement. La page d’aperçu existante est
réécrite et les captures temporaires sont supprimées après vérification.
Les icônes approuvées restent intactes. Aucun raccordement au combat de
campagne n’est effectué. Les deux rendus restent à valider ; dix-neuf autres
compétences restent ensuite à présenter, plus Vitalité améliorée en réflexion.


## AFFLICTIONS-ZONE01 — révision de toute la branche en malédictions de zone

Le joueur demande le 15 octobre de reprendre la branche Affaiblissements
comme des malédictions de zone. Cette instruction concerne Interférence,
Faille d’armure, Silence, Rupture de régénération et Désorientation, ainsi
que le réexamen du passif Contagion. Elle ne vaut pas acceptation des
animations ponctuelles ANIM-S41 ni de nouveaux chiffres non présentés.

La proposition complète est enregistrée dans
`afflictions-area-revision-01.json`. Les actifs appliquent un effet aux
hostiles dans une zone carrée choisie ; l’effet suit ensuite chaque victime,
sans laisser de zone persistante au sol. Les surfaces proposées sont 3 × 3,
5 × 5, 7 × 7, 9 × 9 puis 11 × 11 aux rangs effectifs 1, 4, 7, 10 et 13.
La portée croissante actuelle et la découpe par les obstacles sont conservées.
Chaque cible conserve sa résistance et ses immunités. Les dégâts dégressifs
ne sont pas étendus implicitement aux affaiblissements sans dégâts.

Contagion est proposé comme une transmission de groupe autour du mort,
avec son rayon actuel de deux à quatre cases, ses coefficients affaiblis
et l’interdiction de retransmission. Cette proposition détaillée reste à
valider. Les autres effets, coûts et durées gardent leur référence actuelle.
Les nouveaux textes et aperçus natifs restent à revoir par lots. Aucun
changement du catalogue actif, du combat ou des sauvegardes n’est effectué.


### AFFLICTIONS-ZONE01-ACCEPT01 — fonctionnement de zone accepté

Le joueur valide la proposition commune le 15 octobre : cinq malédictions
actives de zone, effets attachés aux victimes, résistances individuelles,
carrés de 3/5/7/9/11 cases aux rangs effectifs 1/4/7/10/13 et obstacles
protecteurs. Contagion transmet un effet affaibli autour du mort, dans un
carré de 5 à 9 cases et sans retransmission en chaîne. L’accord est enregistré
dans `afflictions-area-acceptance-01.json` ; il ne valide pas les nouveaux dessins.

Le catalogue reçoit les largeurs et les métadonnées des six compétences.
Un correctif de conditionnement conserve ces décisions lors d’un réexport.
Les autres paramètres et les compétences hors de cette branche sont conservés.
Le laboratoire spatial garde la limite de neuf cases de Contagion, y compris
avec des bonus de rang élevés. Les effets de campagne restent à raccorder
après les aperçus. La reformulation finale des textes reste à revoir.

## ANIM-S41-REV02 — premiers aperçus de malédictions de zone

Les candidats 91/O02-A02 et 92/AF02-A02 remplacent les aperçus ponctuels.
Dix-huit scènes montrent les quatre directions, plusieurs ennemis, un allié
épargné, une résistance individuelle, les obstacles et le joueur touché.
Une application visuelle de groupe est suivie de marqueurs individuels avec
la réaction commune. Ces dessins restent à valider. La carte complète est
visible en 64 px natifs ; les compteurs sont placés sous la vue pour ne plus
masquer les acteurs. Le lanceur peut choisir le centre par clic dans le
laboratoire natif `--lot-affaiblissements-zone`.

Les exemples de déplacement font sortir une victime au premier tour : elle
conserve son affaiblissement. Un autre ennemi entre au deuxième tour et ne
reçoit rien de la zone passée. Les trois tours sont explicites ; l’attente
réelle ne réduit pas la durée. La preview utilise les cellules légales de la
géométrie partagée et disparaît à la confirmation. La décoration déborde
sur le sol libre mais s’arrête aux murs et dans leur ombre. Le lancement ne
produit ni dégâts ni sang ; les résultats de résistance sont préparés.

La même page HTML est réutilisée et les captures temporaires sont retirées
après vérification. Les anciennes lectures compactes à cible unique deviennent
un cache remplaçable ; les sources des effets et les icônes restent conservées.

## ANIM-S41-REV03 — marqueurs discrets proposés

À la demande du joueur, les deux malédictions sont revues ensemble :
91/O02-A03 Interférence et 92/AF02-A03 Faille d’armure. L’essai retire les
arcs, fissures, éclats et flashes d’application. Il conserve seulement une
icône de 24 px au-dessus de chaque victime, sans rectangle de fond, avec
un compteur de durée de 14 px. La réaction commune aux effets négatifs reste
présente. Ces rendus sont proposés pour revue, sans validation anticipée.

Les PNG déjà validés de l’arbre sont conservés à l’identique ; un matériau
propre aux marqueurs rend leur fond presque noir transparent. Un fin contour
suit le dessin pour le distinguer du sol. Les règles de zone et les durées
restent celles du lot précédent ; les dix-huit chronologies sont comparées
intégralement. La même page d’aperçu est réutilisée et les captures remplacées
sont nettoyées après vérification. Les effets de campagne restent à raccorder.

### ANIM-S41-REV03-ACCEPT01 — marqueurs discrets acceptés

Le 15 octobre, « ok je valide :) » confirme les deux rendus de la revue 88 :
91/O02-A03 Interférence et 92/AF02-A03 Faille d’armure. Les petites icônes
sans fond au-dessus des victimes et leurs compteurs de durée sont retenus,
sans animation sur le corps ni flash de lancement. La réaction commune aux
effets négatifs reste conservée. L’accord et les empreintes des dix-huit
scènes sont consignés dans `skill-animation-review-88/visual-acceptance-01.json`.
Les règles de zone déjà acceptées et le calendrier d’intégration au combat
restent conservés. Les quatre autres compétences de la branche attendent
encore leurs aperçus ; Vitalité améliorée reste à discuter séparément.

## ANIM-S42 — Silence et Rupture de régénération proposées

La revue 89 présente 93/AF03-A01 Silence et 94/AF04-A01 Rupture de
régénération. Elle réutilise la présentation validée sous ANIM-S41-REV03 :
icône de 24 px sans fond au-dessus de la victime, compteur de 14 px,
réaction commune aux effets négatifs, sans animation sur le corps. Les
PNG de l’arbre restent conservés à l’identique : visage bâillonné et soin barré.
Les deux nouveaux rendus et le texte de présentation restent à valider.

Les exemples de rang 1 reprennent le catalogue courant : Silence dure deux
tours, coûte douze énergie et possède huit tours de recharge ; Rupture dure
trois tours, coûte dix énergie et possède six tours de recharge. Le carré de
trois cases de côté et la portée de cinq cases suivent AFFLICTIONS-ZONE01.
Le laboratoire montre les quatre directions, un groupe et un allié épargné,
une résistance préparée, les obstacles, le joueur touché, l’attente et les
mouvements hors/dans l’ancienne zone. Les effets restent attachés aux acteurs,
les nouveaux entrants restent épargnés et la preview disparaît au lancement.

Le scénario est un aperçu de rendu et de durée. Il ne simule pas les décisions
de l’IA ni des commandes de soin, et ne raccorde pas encore les compétences
au combat de campagne. Le même moteur de revue sert aux deux premiers lots
de malédictions, via `--lot-affaiblissements-zone` et
`--lot-silence-regeneration`, sans copier ses règles et son rendu. La page
`apercu-animations.html` est réutilisée ; les captures temporaires sont
retirées après vérification des lectures compactes.

### ANIM-S42-ACCEPT01 — Silence et Rupture de régénération acceptées

Le 15 octobre, « ok je valide :) » confirme les deux rendus de la revue 89 :
93/AF03-A01 Silence et 94/AF04-A01 Rupture de régénération. Les marqueurs
sans fond de 24 px, leurs compteurs de 14 px et la réaction commune sont
retenus pour les scènes présentées. Les durées distinctes du catalogue,
deux et trois tours au rang 1, restent conservées ; aucune nouvelle formule
ni intégration au combat n’est déduite de cet accord. Le reçu et les empreintes
des dix-huit scènes figurent dans `skill-animation-review-89/visual-acceptance-01.json`.
Le nettoyage du lot est terminé. Désorientation et Contagion restent à présenter,
ainsi que les quinze autres animations non montrées. Vitalité améliorée reste
à discuter séparément.

## ANIM-S43 — Désorientation et Contagion proposées

La revue 90 propose 95/AF05-A01 Désorientation et 96/AF06-A01 Contagion.
Les marqueurs individuels gardent la présentation acceptée : 24 px sans
fond, compteur de 14 px et réaction commune aux effets négatifs. Les icônes
existantes du catalogue restent conservées à l’identique. Désorientation
utilise sa durée maximale de deux tours au rang 1 ; ses dix énergie et sept
tours de recharge sont conservés. La scène « Dégâts » met fin à son effet
sur une victime qui reçoit dix dégâts physiques préparés. Une victime immunisée
est également présentée, sans nouveau calcul de résistance.

Contagion reste passive, sans coût ni recharge ni zone à choisir par le joueur.
La victime au centre porte une Rupture directement appliquée, préparée avec six
tours restants selon le rang sept du catalogue. Sa mort transmet cet effet aux
ennemis éligibles du carré de cinq cases de côté. Au rang un de Contagion,
la conservation de cinquante pour cent donne trois tours transmis. Le blocage
binaire des soins reste entier. Les icônes de Rupture rejoignent rapidement
les nouveaux porteurs ; l’icône de Contagion signale brièvement le déclenchement
au-dessus du mort. Ce mouvement de transmission et ces nouveaux rendus restent
à valider. Les corps existants du bestiaire sont réutilisés.

Vingt-deux scènes couvrent les quatre directions, le joueur touché, les alliés,
les obstacles, les résultats individuels préparés, l’attente, les mouvements,
l’interruption par dégâts et la mort d’un porteur reçu. Dans ce dernier cas,
un ennemi proche du second mort mais hors de la zone du premier reste épargné :
il n’y a pas de retransmission. Les neuf statuts compatibles déjà acceptés sont
conservés ; cette revue illustre seulement la Rupture, sans restreindre la liste.

Le laboratoire reste un prototype de rendu et d’événements préparés, sans
raccord aux actions de campagne ni modification des sauvegardes, du catalogue
ou des formules de résistance. La même page HTML est remplacée et les captures
intermédiaires sont supprimées après vérification de sa lecture compacte.

## ANIM-S44 — Éclaireur et Mule proposés

La revue 91 propose 97/D56-A01 Invocation de l’Éclaireur et 98/D57-A01
Invocation de la Mule. Un bref effet technique lumineux accompagne l’arrivée
sur une case adjacente libre ; il s’éteint, puis le drone reste présent.
La carte est affichée en cases de 64 px. Les modèles actuels 56 et 57 du
bestiaire sont réutilisés, sans redessiner leurs sources 32 px. Cet aperçu
ne valide donc pas de nouveaux modèles 64 px. Le rendu d’arrivée et le
message flottant restent à valider.

Les exemples de rang 1 gardent dix énergie et six tours de recharge.
Les refus sur une case bloquée ou occupée, avec neuf énergie ou lorsqu’un
même modèle est déjà déployé, ne consomment ni énergie ni recharge dans
le scénario préparé. Les deux drones restent non combattants. La scène
d’attente garde le drone et sa recharge sans progression de tour.

La Mule reçoit un ordre de collecte préparé après son déploiement. Elle
avance d’une case au premier tour, puis transfère le même couteau de camp
du sol vers son chargement au deuxième. Son nom apparaît au-dessus d’elle
une seule fois ; aucun objet supplémentaire n’est créé. L’Éclaireur ne
ramasse rien. Dans la scène de visibilité, les lignes de vue de la carte et
le rayon actuel de l’Éclaireur révèlent l’objet à gauche. La vision privée
de la Mule ne permet pas de le collecter à droite, conformément à PETS-EQ16.

Le coût de l’analyse, les statistiques complètes, les phases de première
action et l’IA de campagne ne sont pas définis par ces situations préparées.
Les fenêtres de familiers et l’inventaire réel restent à raccorder après
les aperçus. Le moteur des malédictions précédentes reste inchangé ; la
validation visuelle de la revue 90 n’est pas déduite de la demande de continuer
après l’explication sur l’absence de combat simulé. Sa lecture compacte est
conservée. La page active reste `apercu-animations.html` et les captures
temporaires du nouveau lot sont nettoyées après vérification.

### ANIM-S44-ACCEPT01 — Éclaireur et Mule acceptés

Le 15 octobre, « tres bien je valide :) » confirme les deux rendus de la
revue 91 : 97/D56-A01 Invocation de l’Éclaireur et 98/D57-A01 Invocation
de la Mule. L’arrivée technique, la présence persistante, la révélation
par vision partagée, le déplacement puis le ramassage avec le nom flottant
sont retenus dans les scènes présentées. Les modèles existants sont conservés ;
aucun nouveau dessin de corps 64 px n’est validé par cette revue. Les coûts
actuels et les règles déjà acceptées restent conservés. Le reçu et les
empreintes des vingt-deux scènes sont consignés dans
`skill-animation-review-91/visual-acceptance-01.json`. L’intégration au combat,
à l’inventaire et à l’IA reste prévue après les aperçus. Le nettoyage du lot
est terminé. La suite proposée est le drone Traqueur et l’Artilleur.


## ANIM-S45 — Drone Traqueur et Artilleur proposés

La revue 92 présente 99/D58-A01 Invocation du drone Traqueur et
100/D59-A01 Invocation de l’Artilleur. L’arrivée technique reprend le rendu
accepté en revue 91. Le Traqueur effectue un bref mouvement vers sa victime
et une frappe tranchante au contact ; l’Artilleur projette une munition rapide
avec un éclair de bouche et un impact. Les cibles vivantes reçoivent le sang
physique déjà défini et le petit aller-retour commun. Les machines reçoivent
des étincelles et le même recul, sans sang. Ces deux nouvelles compositions
d’attaque restent à valider.

La phase ordinaire des familiers du tour d’invocation est illustrée : le
Traqueur commence à approcher, l’Artilleur peut tirer. Aucun budget d’action
supplémentaire n’est créé. Les actions suivantes sont préparées par tour ;
l’attente en secondes n’ajoute ni déplacement ni attaque et ne diminue pas
la recharge. Une cloison infranchissable empêche l’approche et les tirs.
Les refus avec onze énergie ou un modèle déjà présent restent atomiques.
Le coût d’invocation au rang 1 conserve douze énergie et six tours de recharge.

Vingt scènes montrent quatre directions, une cible au contact, une cloison,
une machine, l’attente et les deux refus. Les dégâts 18/12, les 100 PV de
la cible et la route rectiligne sont des situations préparées pour observer
le rendu : ils ne définissent pas de nouvelles statistiques, armes, variance,
priorités d’IA, vitesses, coûts d’énergie des drones ou phases de campagne.
Les corps 58/59 existants restent des sources 32 px affichées dans les cases
de 64 px, en attendant la reprise du bestiaire prévue après les animations.
Le joueur et les monstres restent de face dans les quatre directions.

La même page `apercu-animations.html` est remplacée. Les sources graphiques,
le catalogue, les rendus déjà acceptés et les lectures compactes encore en
attente restent préservés. Les captures et journaux intermédiaires sont
supprimés après les vérifications natives et de lecture. Le laboratoire
isolé est accessible par `--lot-drones-combat`, sans intégration en campagne.


### ANIM-S45-BODY02 — reprise demandée pour l’Artilleur

Pendant la préparation du lot, le joueur signale que l’Artilleur n’a pas la
même qualité que les autres personnages. Son ancien dessin 32 px agrandi
reste effectivement utilisé par le prototype. La présentation finale des
animations 99/100 est donc différée pour proposer D59-BODY02 : un corps
redessiné dans le style BD, exporté directement pour une case 64 px.
Cette demande ne valide ni le nouveau modèle natif, ni son éventuelle épave,
ni les orientations des canons ou les nouvelles animations. Les données
de combat et la banque active restent inchangées. Les trois tests et les
vingt situations natives préparées ont passé leurs contrôles mécaniques ;
les captures temporaires du corps à remplacer sont supprimées, avec leur
reçu de nettoyage, plutôt que de publier un aperçu déjà signalé comme raté.


Le dessin source D59-BODY02 est produit par imagegen intégré. Le joueur
répond « beaucoup mieux » et demande de reprendre l’icône correspondante.
La même source RGBA fournit donc le modèle natif 64 px et D59-C05, cadré
pour les vues compactes 64/32 px ; aucune silhouette différente n’est générée
pour la miniature. Le grand dessin reçoit cet accueil favorable, tandis que
les exports natifs et la nouvelle icône restent à examiner. La page active
présente maintenant cette revue statique avec une composition à côté du
joueur actuel. Les 276 Mio de captures intermédiaires ont été retirés ;
les assets, la documentation, le code et les petits reçus restent conservés.


### ANIM-S45-A02 — Artilleur à chenilles et reprise de l’aperçu animé

Le joueur préfère des chenilles aux roues, puis demande de voir l’animation.
L’édition imagegen D59-BODY03 conserve le haut du corps, ses deux canons,
les volumes métalliques et les panneaux rouges de BODY02. Deux bandes de
chenilles lisibles remplacent les pneus. L’export 64 px est effectué directement
à partir de cette source RGBA, avec les mêmes pieds et la même largeur de
cadrage que BODY02. L’icône D59-C06 utilise ce même dessin, aux deux tailles
compactes, sans créer un second modèle différent.

Le retour « je valide :) » arrive pendant cette préparation et confirme la
direction demandée. Il ne vaut pas accord sur les animations non encore montrées.
La revue 92 reprend donc 99/D58-A01 et 100/D59-A02 : vingt-deux scènes natives
montrent déploiement, approche/contact, tir dans les quatre directions, cloison,
machine sans sang, attente sans nouveau tour et refus atomiques. Une scène
supplémentaire remplace une attaque par un déplacement préparé au tour suivant
l’invocation, sans ajouter d’action. Les mouvements et trajectoires de revue
ne fixent pas de nouvelle priorité d’IA ou de règle de vitesse.

Le nouveau modèle de l’Artilleur et son ombre séparée sont chargés uniquement
dans le laboratoire. La banque active 59, les règles et les sauvegardes restent
inchangées. Le Traqueur conserve provisoirement son ancien corps. Les impacts
physiques, le sang vivant et le recul commun gardent leur traitement ; les
chiffres 18/12 et les 100 PV de cible restent des valeurs de démonstration.
La lecture contient six secondes natives pour laisser disparaître les derniers
effets avant de reboucler. La même page HTML reste la vue de revue actuelle.


### ANIM-S45-PAL01 — palette du Traqueur choisie par le joueur

Le joueur hésite sur les anciennes couleurs violet/cyan. Il choisit ensuite
« Graphite et vert militaire (recommandé) », avec des lames en acier clair.
La variante D58-PAL01 applique cette palette uniquement à l’affichage dans
le laboratoire, au moyen d’un matériau GPU. Les pixels du dessin original,
sa forme et ses points d’appui sont conservés. Il ne s’agit donc pas encore
d’un redessin 64 px du corps 58. La miniature existante reçoit exactement la
même transformation de palette dans l’aperçu ; les fichiers actifs restent
intacts. Le contrôle natif compare les 4 096 pixels de la composition 64 px :
son masque alpha occupé reste identique, tandis que 464 pixels changent de RGB.

L’animation devient 99/D58-A02. Les mêmes situations préparées sont recapturées
pour ce seul drone ; les onze lectures natives de 100/D59-A02 Artilleur à
chenilles sont conservées à l’identique. Le choix valide la direction de
palette, sans déduire une acceptation des rendus animés non encore examinés.
Les captures temporaires sont nettoyées une fois la lecture compacte vérifiée.


La revue animée 92 est publiée sur la page réutilisée : 99/D58-A02 et
100/D59-A02. Les quatre tests ciblés passent. Les vingt-deux situations
possèdent une lecture compacte vérifiée pixel par pixel ; les onze chronologies
du Traqueur restent identiques et les onze fichiers de l’Artilleur sont conservés
à l’octet près. Deux lectures animées autonomes de six secondes reprennent
chacune 144 images natives, sans perte de pixels. Le contrôleur HTML est testé
hors navigateur avec huit scènes, quatre directions et progression manuelle ;
aucune automatisation de navigateur ni validation de rendu HTML n’est déduite
de ce contrôle. Les captures et journaux temporaires sont nettoyés. Le rendu
animé reste à valider ; la banque et le combat de campagne restent inchangés.

### ANIM-S45-ACCEPT01 — Traqueur et Artilleur acceptés

Le joueur répond « parfait :) » après la publication de la revue animée 92.
La validation porte sur 99/D58-A02 et 100/D59-A02, leurs vingt-deux scènes
présentées et les icônes assorties. Le Traqueur conserve la palette graphite
et vert militaire D58-PAL01 ; sa forme provient toujours du dessin 32 px
existant. L’Artilleur conserve le corps natif 64 px à chenilles D59-BODY03
et l’icône D59-C06, dérivée du même dessin aux tailles 64 et 32 px.

Les déplacements préparés, frappes, tirs, impacts, recul, sang sur les cibles
vivantes, impacts secs sur les machines et blocages par les murs présentés
sont acceptés visuellement. Les dégâts 18/12 et les 100 PV des cibles restent
des exemples de revue. Les mécaniques déjà validées restent applicables ;
cet accord ne définit pas de nouvel équilibrage ou d’IA et ne constitue pas
l’intégration jouable en campagne. Les onze compétences non encore montrées,
la validation visuelle de la revue 90 et les règles de B03 restent séparées.

Le reçu `skill-animation-review-92/visual-acceptance-01.json` conserve les
empreintes des vingt-deux lectures, des dessins, des icônes et des deux
animations autonomes. Les captures et journaux temporaires ont déjà été
nettoyés ; les sources et les reçus sont conservés.

### ANIM-S46 — Gardien et Algorithmes de combat améliorés

Le joueur demande de poursuivre après ANIM-S45-ACCEPT01. Le lot suivant
présente 101/D66-A01 et 102/O03-A01 dans le laboratoire isolé. Le Gardien
reçoit des segments protecteurs près des corps couverts et une réaction
lumineuse à l’impact. Algorithmes reçoit une courte impulsion des circuits
au moment d’une attaque de drone armé. Ces nouveaux rendus restent proposés.

Au rang 1, les valeurs déjà définies sont conservées : D66 coûte 14 énergie,
recharge en six tours et protège à deux cases avec une réduction de 5 % ;
O03 apporte +25 % aux drones armés, sans coût ni recharge. Les scènes montrent
le joueur, le Gardien lui-même et des drones alliés protégés, ainsi que la
perte de couverture hors de portée ou derrière une cloison. Elles ne décident
pas d’une nouvelle convention de distance diagonale ou d’ordre des défenses.

Les attaques préparées reprennent l’aller-retour commun et les impacts
physiques : sang sur une cible vivante, étincelles sur une machine. Le bonus
ne crée pas d’attaque pour l’Éclaireur ou la Mule. Le passif est déjà appris
dans son exemple ; le bouton de revue ne devient pas une activation en jeu.
L’attente sans nouveau tour n’ajoute aucune attaque ou dépense.

Les dégâts 12/20/25/19 et les réserves 160/120 PV sont des exemples de lecture,
non des statistiques nouvelles du bestiaire. Le corps 66 reste celui de la
banque existante 32 px ; sa résolution sera traitée lors du travail prévu sur
le bestiaire. Le corps 64 px de l’Artilleur et la palette validée du Traqueur
restent conservés. Les dessins actifs, le catalogue, les sauvegardes et les
combats de campagne ne sont pas modifiés par ce prototype.

La revue 93 utilise la page `apercu-animations.html` existante. Vingt-six scènes
préparées couvrent dix situations et les quatre directions du cas principal.
Neuf compétences restent à montrer après ce lot ; B03 et l’accord visuel de
la revue 90 demeurent séparés. Les captures temporaires doivent être nettoyées
après vérification des lectures compactes ; les sources et reçus sont conservés.

### ANIM-S46-BODY02 / D58-PAL02 — redessin demandé avant les animations

Pendant la préparation, le joueur refuse le dernier modèle de drone montré
et demande de le redessiner. Le Gardien 101 reçoit donc le candidat D66-BODY02,
produit avec imagegen intégré en prenant l’Artilleur validé comme référence
de style. Sa silhouette bipède, ses gros avant-bras et ses plaques orange et
acier rendent son rôle plus lisible. La source RGBA est exportée directement
pour une case de 64 px, sans passer par 32 px. Le modèle cadré mesure 60 × 48 px,
pieds au niveau 62. L’icône D66-C03 provient du même dessin. Le nouveau corps,
son cadrage natif, cette icône et leurs éventuels effets restent à valider.

Le joueur demande aussi de rendre les pattes du Traqueur blanc os, comme sa
tête. D58-PAL02 reprend la méthode de palette du rendu isolé D58-PAL01 et
change uniquement le matériau précédemment cyan des pattes. Le contrôle natif
compare 4 096 pixels : 168 pixels changent, tous dans cette matière ; la tête,
les autres couleurs et le masque alpha occupé restent identiques. L’icône
assortie reçoit la même palette. Le dessin demeure celui de la source 32 px ;
cette demande n’est pas interprétée comme un redessin de sa forme.

La page réutilisée présente ces deux propositions, avant/après à taille native,
ainsi que leur composition sur le terrain avec le joueur et l’Artilleur. Le lot
animé D66/O03 reste en préparation : ni ces animations ni ces nouveaux dessins
ne sont déclarés acceptés. Onze compétences restent donc à montrer en animation,
en comptant D66/O03. Les premières captures de l’ancien Gardien, puis les
intermédiaires de contrôle, sont supprimés après conservation des sources,
exports de revue et petits reçus. Aucun fichier graphique actif n’est remplacé.

### ANIM-S46-BODY-ACCEPT01 — dessins du Gardien et du Traqueur acceptés

Le joueur répond « oui c’est beaucoup mieux :D » à la comparaison des deux
propositions. Le corps D66-BODY02, ses exports et l’icône assortie D66-C03 sont
acceptés, ainsi que les pattes blanc os D58-PAL02 du Traqueur. Le reçu
`skill-animation-review-93/body-palette-acceptance-01.json` conserve les
empreintes des fichiers et des compositions natives montrées. Cet accord
porte sur les dessins ; les effets animés D66/O03 restent à présenter. Le
prototype reprend ces versions sans modifier le catalogue ou la campagne.

### ANIM-S46-A01 — aperçu animé publié avec les dessins validés

La revue 93 reprend 101/D66-A01 et 102/O03-A01 avec le nouveau corps du Gardien
accepté, l’Artilleur à chenilles et les pattes blanc os du Traqueur. Vingt-six
scènes natives de six secondes montrent le déploiement, la protection persistante
et sa réaction à un impact, puis les circuits renforcés au moment d’une attaque.
La perte de protection par distance ou cloison, les machines sans sang,
l’attente sans tour, les refus et les drones de soutien sans attaque figurent
parmi les dix situations. Les quatre directions restent proposées.

La scène de cloison place maintenant la Mule et l’ennemi sur des cases distinctes.
Cette correction de fixture conserve les vingt-quatre autres lectures à l’octet
près. Les trois tests ciblés passent ; les vingt-six atlas compacts sont vérifiés
sans perte de pixels. Les deux lectures animées autonomes vérifient chacune
144 images natives. Le contrôleur HTML passe ses contrôles hors navigateur sur
dix situations et quatre directions ; ce contrôle ne vaut pas rendu HTML observé.

Les captures brutes, les doublons de chronologie, les captures de contrôle et
les journaux temporaires sont retirés après vérification. Soixante-six anciens
atlas d’aperçus explicitement validés sont également supprimés ; les dessins,
le code, les règles, les petits reçus et les aperçus non acceptés de la revue 90
sont conservés. Les deux nouveaux effets animés restent à valider. Neuf
compétences restent à montrer après ce lot ; B03 demeure séparé.

### DRONE-ARMY01 / D66-TAUNT01 — changement de rôle et armée validés

Le joueur accepte la recommandation d’une armée de drones avec plusieurs
exemplaires par modèle, un Éclaireur unique et une capacité totale progressant
avec l’investissement dans Drones. Il accepte aussi la provocation automatique
de zone du Gardien en remplacement de sa réduction de dégâts, ainsi que les
ordres de groupe et la libération du passage. Le reçu
`drone-army-and-guardian-taunt-acceptance-01.json` remplace explicitement les
anciennes limites correspondantes. Les dessins déjà acceptés sont conservés.

[La révision de conception](ARMEE_DE_DRONES_ET_PROVOCATION.md) distingue cet
accord des chiffres proposés : capacité initiale 4, +1 tous les cinq rangs
appris ; provocation 5 × 5, deux tours et recharge interne quatre tours,
remplaçant l’attaque du Gardien pour cette phase. Ces nouvelles valeurs,
modalités et nouveaux effets visuels restent à examiner. Aucun ancien bouclier
ou rendu O03 n’est déclaré accepté par cet accord de direction.

La page réutilisée présente ces propositions et un calculateur de capacité.
L’animation O03 précédemment montrée reste accessible à sa revue. Aucun nouveau
lot de captures ou cache volumineux n’est créé à cette étape. Les animations de
provocation et les neuf compétences non encore montrées restent à préparer.

### D66-TAUNT-MULTI01-ACCEPT01 — provocations simultanées validées

Le joueur répond « ok parfait :) » aux règles présentées : conserver une
provocation existante sans remplacement ni rafraîchissement ; choisir le Gardien
admissible le plus proche pour une cible libre, avec un départage stable ; garder
un seul effet et une seule icône ; terminer l’effet si le Gardien est détruit ou
rappelé. Chaque monstre est traité séparément. Le reçu
`guardian-simultaneous-taunts-acceptance-01.json` distingue cet accord des valeurs,
du rendu et des détails d’implémentation encore à examiner.

### ANIM-S47 — provocation du Gardien et marqueur proposés

Le joueur demande un visuel de provocation et une petite icône au-dessus des
monstres affectés. La revue 94 propose 101 / `D66-TAUNT-A01` : une impulsion
cuivrée brève, derrière les corps et arrêtée aux cloisons. Le candidat 103 /
`D66-TAUNT-I01` est un symbole rouge-orangé sur fond transparent, d’environ
24 px dans un canevas de 32 px, attaché à la cible dans la scène en 64 px.
Les monstres gardent une seule icône et font la réaction commune à l’application
de l’effet négatif. Aucune blessure ni projection de sang n’est ajoutée.

Huit scènes couvrent quatre orientations, une cloison, deux Gardiens, un rappel
et l’attente sans tour. L’impulsion cesse rapidement ; le marqueur persiste avec
le statut, puis disparaît au rappel ou à la fin des tours de l’essai. Deux tours
et un rayon de deux cases restent des valeurs de démonstration non acceptées.
La métrique et le départage sont des choix de fixture, sans nouvelle politique
de campagne. Les ennemis de ces scènes sont fixes et ne simulent pas leur combat.

Le laboratoire possède une entrée native `--lot-provocation-gardien`. La page
réutilisée permet la démonstration automatique ou des tours manuels, avec lecture
à taille native et sans nouvelle arborescence de galeries HTML. Les atlas et la
lecture animée autonome sont conservés sans perte ; les captures brutes et les
images de contrôle sont retirées après vérification. Les deux tests ciblés du
prototype passent. Le contrôleur est vérifié hors navigateur ; aucun rendu HTML
observé n’est déclaré. Les deux candidats restent à valider ; O03 garde sa revue
visuelle séparée. Le catalogue actif, le combat et les sauvegardes sont inchangés.

### D66-SIZE01 — Gardien à la hauteur d’une case, proposé

À la demande du joueur, le candidat 104 augmente la hauteur visible du Gardien
de 48 à 64 px. Le même dessin frontal et les mêmes proportions donnent un corps
de 80 × 64 px : les bras dépassent de 8 px de chaque côté de la case logique.
La source haute définition D66-BODY02 est réduite directement dans un canevas
transparent de 128 × 128 px. La source et l’ancienne tile acceptée sont intactes.

La revue 94 et la même page d’aperçu sont réutilisées. Une comparaison numérotée
à taille native sur trois sols accompagne les scènes animées. Les chronologies
de simulation et l’icône de provocation sont comparées à leurs empreintes avant
ce changement de taille. Seuls le corps du Gardien dans le prototype et son
ombre de contact sont agrandis. Le rendu de provocation garde sa revue propre ;
la nouvelle taille reste à valider avant intégration dans la campagne.

### D66-SIZE01-ACCEPT01 — nouvelle taille du Gardien validée

Le joueur répond « parfait :) » à l’aperçu mis à jour. Le candidat 104 /
`D66-SIZE01` est accepté : corps de 64 px de haut, proportions conservées et
léger débordement latéral. Le reçu
`skill-animation-review-94/gardien-size01/visual-acceptance-01.json` fige l’export
de 80 × 64 px sur canevas transparent de 128 px et la comparaison native montrée.
Il remplace la hauteur précédente de 48 px pour les prochains aperçus du Gardien.
L’animation `D66-TAUNT-A01`, le marqueur `D66-TAUNT-I01`, les chiffres proposés
et le rendu O03 restent séparés et ne sont pas approuvés par cette réponse.
Aucun changement au combat ou aux ressources graphiques actives de la campagne.

### ANIM-S48 — Maintenance et Réparation ciblée bleues, proposées

Le joueur choisit le bleu pour la réparation des drones. Il précise que Blindage
amélioré, passif permanent, ne nécessite aucune animation : seule une petite
icône est prévue dans la future fenêtre des compagnons. Le reçu
`drone-passive-visual-policy-decisions.json` conserve ce périmètre et ne valide
pas de nouvelle valeur chiffrée de blindage.

La revue 95 propose 105 / `DP01-A01` Maintenance des drones et 106 / `DR01-A01`
Réparation ciblée. Le premier produit une brève réparation bleue sur les drones
déployés lorsque des PV sont effectivement rendus, après trois tours complets de
calme. Le second envoie une impulsion bleue rapide vers un seul robot allié, puis
montre un éclat de réparation plus marqué. Le trajet est seulement visuel : les
PV s’appliquent dès l’action. Les valeurs de démonstration proviennent des
profils déjà présentés ; les 200 PV maximum et les blessures initiales sont des
fixtures, sans nouvel équilibrage ni nouvelle courbe de portée.

Vingt-deux scènes natives couvrent les quatre orientations, l’interruption par
un hostile, les PV presque pleins, une cloison, le rappel, la destruction,
l’énergie insuffisante et l’attente sans tour. Le Gardien reprend sa taille
64 px validée, l’Artilleur ses chenilles et le Traqueur ses pattes blanc os.
Les trois tests ciblés passent ; la page commune affiche les deux rendus à taille
native avec démonstration automatique et six tours manuels. Les captures brutes
et les images de contrôle sont retirées après vérification des lectures sans
perte. Aucun rendu HTML observé n’est déclaré : le contrôleur est vérifié hors
navigateur. Les deux nouveaux rendus restent à valider.

Six compétences de Tir restent à préparer en animation : DI01, DI02, DI03,
DI04, DI05 et DI07. DP02 ne compte plus parmi les animations à créer. Le rendu
de provocation D66, son marqueur, O03, la revue 90 et la réflexion sur B03
conservent leurs statuts séparés ; aucun accord n’est déduit de « on continue ».
Le catalogue, les ressources actives et la campagne ne changent pas à cette étape.

### ANIM-S48-ACCEPT01 — réparations bleues acceptées

Le joueur répond « parfait c'est tres bien :) » aux deux rendus montrés.
105 / `DP01-A01` et 106 / `DR01-A01` sont acceptés, avec leurs scènes natives
de réparation et de refus préparées. Le reçu
`skill-animation-review-95/visual-acceptance-01.json` fige les vingt-deux atlas
et les deux lectures animées autonomes. L’accord reste visuel : les PV d’exemple,
les détails mécaniques non montrés et l’intégration à la campagne sont séparés.
Les autres rendus encore en attente ne sont pas validés par cette réponse.

### D58-RES64-01 — Traqueur redessiné, proposé avant reprise des animations

Le joueur remarque que le Traqueur est plus pixelisé que les autres drones.
La vérification des fichiers confirme que D58-PAL02 conserve intégralement des
blocs uniformes de 2 × 2 px issus du dessin 32 px ; ses deux dernières révisions
changeaient les couleurs. Le joueur autorise ensuite un véritable redessin.

Le candidat 107 / `D58-BODY01` reprend l’identité du quadrupède : tête allongée
blanc os, dos voûté vert militaire, châssis graphite et quatre pattes blindées
blanc os. Sa nouvelle source est dessinée avec imagegen intégré, puis réduite
directement pour une case logique de 64 px. La hauteur visible reste de 50 px ;
la largeur de 68 px dépasse de 2 px par côté, dans un canevas transparent de
128 px. Le corps n’est pas un agrandissement d’une tile 32 px.

La revue 96 et la page commune présentent une comparaison numérotée sur trois
sols, un zoom ×4 sans lissage et deux scènes natives de 576 px, avant/après,
avec le Gardien et l’Artilleur acceptés. Seule la zone du Traqueur et de son
ombre de contact diffère entre ces deux scènes. Le rendu reste à valider.
Les banques du jeu, l’icône, l’épave et les animations acceptées restent intacts.
Les deux captures intermédiaires et les journaux sont retirés ; les compositions
courantes sont conservées pour la revue. Les vingt-deux anciens atlas de cache
des réparations validées sont retirés après contrôle des empreintes, en gardant
leurs deux lectures animées complètes et leurs sources/reçus. Aucun nouvel
effet mécanique ou nouvel acteur n’est introduit.

### D58-RES64-01-ACCEPT01 — nouveau Traqueur validé et comparaisons nettoyées

Le joueur répond « tres bien, je valide :) » au candidat 107 / D58-BODY01.
La source, l’export direct sur canevas de 128 px pour une case de 64 px, la
palette et la composition native montrée sont acceptés. Le reçu
`skill-animation-review-96/visual-acceptance-01.json` en conserve les empreintes.
Le nouveau dessin sert de référence aux prochains aperçus, sans changer de
règle de jeu ni valider une nouvelle icône, une épave ou un effet animé.

À sa demande de nettoyage, la page commune conserve le dessin validé et la
composition actuelle ; les anciennes images de comparaison, le zoom de revue
et la scène avant sont retirés après vérification des empreintes. Les caches
Python de `tools/art/__pycache__` sont supprimés sans toucher à leurs scripts.
Les sources, exports validés, animations complètes acceptées, documents et code
restent conservés. Les captures temporaires natives avaient déjà été retirées.

### D56-RES64-01 — Éclaireur redessiné, candidat 108 à valider

Après comparaison, l’Éclaireur utilisait encore sa tile 32 px agrandie. Le joueur
autorise sa reprise : « ok on va l’améliorer :) ». Le candidat 108 / D56-BODY01
conserve la coque ivoire, le capteur bleu, le châssis graphite et les deux petits
propulseurs latéraux du drone flottant. Une nouvelle source imagegen est réduite
directement à 56 × 32 px, sur canevas transparent de 128 px pour une case de 64 px,
sans étirement. Son rôle de reconnaissance, ses identifiants et ses règles restent
inchangés. Cette autorisation de travail ne valide pas encore le nouveau dessin.

La revue 97 et la page commune présentent la taille native et deux compositions
natives avant/après avec les trois autres drones validés. Le contrôle limite les
différences à l’Éclaireur et son ombre. Le diagnostic `--lot-eclaireur-64` est
isolé de la campagne. Les banques actives, l’icône, l’épave et les animations
déjà acceptées restent conservées ; aucun nouvel effet n’est validé à cette étape.

La compilation debug et le formatage passent. Les deux captures de revue
conservent leurs pixels RGB natifs ; les doublons bruts et les deux journaux
vides sont retirés après contrôle, soit 628 464 octets. La même page HTML est
réécrite pour cette revue, sans ajouter une nouvelle galerie autonome.

### D56-RES64-02 — réduction de l’Éclaireur revue après refus du premier rendu

Le joueur juge le candidat 108 encore trop pixelisé. La première réduction
56 × 32 px est refusée sur ce critère ; la présence d’une source détaillée
ne suffit pas à prouver une qualité visuelle équivalente aux autres drones.
La variante `D56-BODY01-SIZE02` est réduite directement depuis la même source
à 70 × 40 px, sans agrandir l’export précédent. Elle est 25 % plus grande dans
chaque dimension et propose 3 px de débordement par côté. La silhouette, la
palette, la pose et la hauteur de flottement restent identiques. Ce nouvel
encombrement est une proposition à valider, pas un changement de collision.

La page commune compare le premier rendu réellement montré au nouveau rendu,
avec les trois drones déjà acceptés. Le contrôle natif limite les différences
au corps de l’Éclaireur ; la campagne, les banques et les animations acceptées
ne changent pas. Les deux dessins ne sont pas déclarés équivalents en qualité
sur la seule base du nombre de pixels : le nouveau résultat attend la revue.

### D56-RES64-02-ACCEPT01 et D57-LIMIT01-ACCEPT01

Le joueur valide l’Éclaireur révisé 108 / D56-BODY01-SIZE02 : source, corps
70 × 40 px et débordement de 3 px par côté. Le reçu de la revue 97 fige leurs
empreintes. Ce modèle devient la référence des prochains aperçus ; la campagne,
l’icône et l’épave gardent leur calendrier distinct d’intégration.

Il précise en même temps qu’une seule Mule peut être active. Cette règle,
enregistrée dans `mule-single-active-acceptance-01.json`, remplace la partie de
l’ancien accord qui autorisait plusieurs Mules. L’Éclaireur reste lui aussi
unique ; Traqueurs, Artilleurs et Gardiens conservent le principe d’exemplaires
multiples dans la capacité commune. Les chiffres de cette capacité et les
autres rendus en attente ne sont pas validés par cet accord. La suite reprend
les six animations de Tir encore jamais montrées.

### ANIM-S49 — Tir perforant et Feu nourri, premier lot de Tir proposé

109 / `DI01-A01` présente un projectile rapide traversant des cibles alignées,
avec des impacts successifs, le sang physique accepté sur les êtres vivants
et la réaction commune d’aller-retour. La cloison arrête le projectile à sa
face proche. Les quatre traversées de l’exemple de rang 11 conservent 75 %
des dégâts à chaque passage ; les valeurs 38, 28, 21 et 16 sont des impacts
préparés. L’allié dans la ligne et l’échec préparé gardent leurs cas distincts.

110 / `DI02-A01` montre les deux séquences natives acceptées : deux tirs avec
une arme en coup par coup, six projectiles avec une rafale de trois. Les tirs
restent concentrés sur une seule cible, avec recul bref et impacts rapprochés.
Une seule valeur de dégâts cumulés reste lisible au-dessus de chaque victime.
Les machines produisent des étincelles sans sang ; aucun statut Saignement
n’est ajouté. La prévisualisation clignote avant confirmation, puis disparaît.

La revue 98 utilise le laboratoire natif `--lot-tirs-ouverture` et la même
page HTML pour les deux animations côte à côte, en lecture automatique ou
manuelle. Les quatre directions et les cas cloison, machine, échec, allié,
limite, rafale, munitions, énergie, arme active incompatible et attente sont
préparés. Les PV 240, les coûts de fiche et les impacts servent de repères ;
les probabilités, distributions, décès précoces et détails non validés de
résolution ne sont pas fixés par ces exemples. L’attente et les tours suivants
ne rejouent pas l’attaque. La campagne, les banques et les sauvegardes restent
inchangées ; ces deux rendus attendent leur propre accord visuel.

DI03, DI04, DI05 et DI07 restent les quatre animations de Tir encore jamais
montrées. Les rendus antérieurs encore en attente gardent leur revue distincte.

Les trois tests ciblés du prototype, la compilation debug et le formatage
passent. Vingt-huit scènes natives sont capturées à 30 images/seconde ; leurs
pixels sont vérifiés après encodage sans perte. Les commandes automatiques,
manuelles, la pause et les onze situations sont contrôlées hors navigateur,
sans déclarer une inspection du rendu HTML. La page conserve des canevas de
576 px pour des cases de 64 px. Les états répétés sont regroupés dans les
données de lecture. Les deux passages de captures brutes, les images de
contrôle et les comparaisons de l’Éclaireur désormais validé sont retirés ;
le lot conserve environ 3,6 Mo de lectures, données, reçus et contrôles. La
source et les exports validés de l’Éclaireur restent intacts, ainsi que les
vingt-sept fichiers protégés du contrôle de préservation.

### ANIM-S49 — validation des deux animations de Tir

Le joueur valide 109 / `DI01-A01` et 110 / `DI02-A01` par « je valide :) ».
Le reçu `skill-animation-review-98/visual-acceptance-01.json` fige le code,
les deux lectures animées et les empreintes des vingt-huit scènes contrôlées.
L'accord concerne les rendus présentés, sans ajouter de formule de critique,
de variance, de décès anticipé ni d'intégration à la campagne. Les autres
rendus et chiffres en attente conservent leur validation séparée.

### ANIM-S50 — Volée et Tir de précision, deuxième lot de Tir proposé

111 / `DI03-A01` propose un éventail de trajectoires rapides, avec un
projectile pour chacun des cinq ennemis retenus au rang effectif 11. Les
cibles proches sont prioritaires ; les ennemis hors du cône ou au-delà de
la limite restent intacts. La cloison isolée masque les lignes centrales,
sans empêcher les tirs légaux sur ses côtés. L'aperçu utilise la géométrie
commune du cône à 90° et la croissance spatiale acceptée : arme d'exemple
à neuf cases, profondeur du cône limitée à six cases. Ces valeurs remplacent
l'ancien exemple de portée de la fiche, sans réviser les autres règles.

112 / `DI04-A01` propose un viseur rouge très bref sur une seule cible,
suivi d'un projectile tendu et d'un impact physique. Le viseur disparaît ;
il ne constitue ni une préparation ni un marquage persistant. La précision
améliorée ne garantit pas le succès : un échec préparé reste visible dans
sa situation dédiée. Les armes lourdes restent exclues par la fiche acceptée.

Les impacts 29 et 22 sont des exemples fixes, le second non critique. Aucun
nouveau multiplicateur critique, jet aléatoire, départage général ou effet
de Saignement n'est ajouté. La Volée physique n'hérite pas de la dégressivité
des zones magiques. Le sang accepté reste réservé aux êtres vivants et la
réaction commune accompagne chaque blessure ; les machines ont des étincelles.
La prévisualisation clignotante disparaît dès confirmation. Les refus gardent
intacts les ressources et le tour ; l'attente ne rejoue jamais les tirs.

La revue 99 utilise le laboratoire `--lot-volee-precision` et la page partagée
`apercu-animations.html`, avec les deux scènes côte à côte et des cases de
64 px. Ces deux rendus attendent leur propre validation visuelle. DI05 et
DI07 restent les deux animations de Tir encore jamais montrées. La campagne
et l'adaptation ultérieure du bestiaire conservent leurs étapes séparées.

Les trois tests ciblés, la compilation debug, le formatage et le contrôle
d'espaces passent. Vingt-six scènes natives de 120 images sont vérifiées
après encodage sans perte ; les quatre orientations, dix situations, la
pause et les six tours manuels sont contrôlés hors navigateur. Les planches
natives de contrôle ont été inspectées puis supprimées. Les trente-trois
fichiers protégés, dont le code et les lectures acceptés du lot précédent,
gardent leurs empreintes. Le nettoyage retire 147 986 913 octets de captures
brutes, 662 235 octets de planches/journaux et 3 130 984 octets d'ancien cache
d'atlas accepté. La revue 99 conserve environ 3,4 Mo de lectures compactes,
données et reçus ; les deux animations complètes du lot 98 restent disponibles.

### ANIM-S50 — validation de Volée et Tir de précision

Le joueur valide 111 / `DI03-A01` et 112 / `DI04-A01` par « ok j'aime ! je valide ».
Le reçu `skill-animation-review-99/visual-acceptance-01.json` conserve le code,
les deux lectures complètes et les empreintes des vingt-six scènes natives.
Cette acceptation porte sur les rendus présentés. Les autres chiffres, jets,
départages, cas de décès et intégrations de campagne gardent leur portée séparée.

### ANIM-S51 — Tir de harpon et Tir vampirique proposés

113 / `DI05-A01` présente un projectile métallique à barbes et son câble,
suivi d'une traction sèche de trois cases dans l'alignement préparé. L'impact
emploie le sang physique accepté et la réaction commune d'aller-retour. Une
cible ancrée ou l'élite dont la résistance est préparée garde sa position,
sans perdre les dégâts du tir. Le cas d'échec ne s'ancre pas et ne déplace
personne. Aucun statut persistant ni bonus de collision n'est ajouté.

114 / `DI07-A01` conserve le tir physique de l'arme puis montre une brève
impulsion verte derrière l'agent, avec « +X PV » au-dessus de sa tête. Le
dessin reprend la source de soin acceptée en revue 77, sans ajouter de drain
ou de résidu. Au rang effectif 11 présenté, le soin représente 50 % des PV
réellement retirés à la cible principale, limité à 10 % des PV maximum et
aux PV manquants. Les exemples ont 100 PV maximum : l'impact préparé de 36
rend 10 PV après plafond, celui de 12 rend 6 PV ; à 98/100, seuls 2 PV reviennent.
Ces nombres pairs évitent de trancher l'arrondi général et les PV sont des
repères de scène, sans fixer les défenses ou statistiques du jeu.

Les tirs sont brefs et la prévisualisation clignotante disparaît à la
confirmation. Les obstacles masquent les tirs ; le helper de traction vérifie
aussi les obstacles, cases occupées et la case de l'agent dans l'alignement
préparé. Les diagonales, interceptions, machines, surdégâts, boucliers et
conditions de lancement à pleine vie gardent leurs décisions séparées.
La résistance d'élite montrée est un résultat préparé ; elle ne remplace
pas la chance conditionnelle de traction de 75 % déjà acceptée.

La revue 100 et `--lot-harpon-vampirique` présentent les deux animations en
64 px, dans quatre directions, avec huit tours de contrôle de recharge.
Attendre ne répète ni les dégâts, ni la traction, ni le soin. Ces deux rendus
attendent leur validation visuelle dans `apercu-animations.html`. Toutes les
animations de Tir auront alors été montrées ; les rendus et chiffres plus
anciens encore en attente ne sont pas validés par cette progression. Le
raccordement au combat et l'adaptation du bestiaire restent des étapes distinctes.

Les trois tests ciblés, la compilation debug, le formatage et le contrôle
d'espaces passent. Les vingt-huit scènes natives et leurs 120 images à 30 Hz
sont vérifiées après encodage sans perte. Le contrôleur automatique et manuel,
la pause, les quatre orientations et les onze situations sont contrôlés hors
navigateur, sans déclarer une inspection du rendu HTML. Les planches natives
du harpon, du soin et des cas limites ont été inspectées puis retirées.
Les quarante fichiers protégés conservent leurs empreintes. Le nettoyage retire
163 506 000 octets de captures brutes, 692 824 octets de planches/journaux et
2 941 882 octets du cache d'atlas accepté du lot 99. La revue 100 conserve
environ 3,1 Mo ; le code et les deux lectures complètes acceptées du lot 99
restent disponibles avec leurs reçus.

### ANIM-S51-ACCEPT01 — Tir de harpon et Tir vampirique validés

Le joueur valide 113 / `DI05-A01` et 114 / `DI07-A01` par « parfait :) je valide ! ».
Le reçu `skill-animation-review-100/visual-acceptance-01.json` fige le code,
les deux lectures complètes et les empreintes des vingt-huit scènes natives.
L'accord concerne ces rendus et les situations préparées montrées, sans créer
de nouvelle règle d'arrondi, de défense, de trajet diagonal ou d'intégration.
Les six animations de Tir DI01, DI02, DI03, DI04, DI05 et DI07 sont maintenant
validées visuellement. Leur intégration jouable reste une étape distincte.

### ANIM-S43-REVIEW02 — clôture de la validation des Affaiblissements

La reprise de revue 101 a interprété trop strictement l'accord déjà donné
sur AF05 et AF06. Le joueur le rappelle explicitement : « je comprend pas ,
on l'a déjà validé ca non ? ». Le suivi est corrigé par
`skill-animation-review-101/visual-acceptance-01.json` : 95 / AF05-A01 et
96 / AF06-A01 sont déjà validés. Les tests de décisions et d'attaques des
monstres restent prévus lors de l'intégration jouable ; cette étape distincte
ne remet pas en cause la validation des images et marqueurs. Les autres
accords sur les corps, tailles et rôles des drones conservent leur portée.

La revue 101 a remis inutilement 95 / `AF05-A01` Désorientation et
96 / `AF06-A01` Contagion dans la page partagée. Leurs rendus d'origine
restent conservés avec leur accord désormais correctement consigné.
Les vingt-deux atlas existants sont comparés aux empreintes de chaque image
de la revue 90. Le code source conserve exactement son empreinte. Les états
sont regroupés pour réduire les données de lecture ; leur reconstruction
reproduit intégralement les chronologies d'origine. Aucun nouvel effet ou
chiffre n'est proposé par cette reprise. Les interactions de combat entre
monstres gardent leur test jouable ultérieur, déjà annoncé au joueur.

Les huit situations, les quatre directions, les tours manuels, la pause,
l'interruption par dégâts et l'absence de retransmission sont contrôlés hors
navigateur. Les planches natives ont été inspectées puis retirées. La revue
réutilise 1 930 054 octets d'atlas existants ; ses nouvelles lectures complètes
et données compactes occupent environ 335 Ko. Les soixante et onze fichiers
protégés gardent leurs empreintes. Le nettoyage retire 2 709 752 octets de
cache accepté du lot Tir et 293 170 octets de planches de contrôle. Le code,
les sources, les lectures complètes et les reçus acceptés sont conservés.

### AF06-A02 — transmission organique demandée

Après ce rappel, le joueur demande de retoucher uniquement Contagion : une
sorte de tentacule doit toucher chaque cible effectivement concernée, avec
un rendu glauque. La version A01 reste acceptée et conservée ; Désorientation
n'est pas remise à valider. La revue 102 prépare donc uniquement AF06-A02.

Le nouveau dessin est une matière organique peinte avec imagegen intégré,
chair blafarde, plis sombres, ventouses et veines olivâtres. Des rubans courbes
la déforment du cadavre vers les destinataires : poussée rapide, bref contact,
puis rétraction. Les icônes de l'effet transmis restent ensuite au-dessus des
victimes. Le masque du décor coupe les tentacules sur les obstacles. Les
alliés, victimes hors zone et cibles qui résistent ne reçoivent pas de tentacule.

Les règles, durées, coûts, résistances et absence de propagation en chaîne
restent ceux de la version acceptée. L'animation ne crée ni blessure ni dégât
supplémentaire. Le laboratoire isolé utilise `--lot-contagion-tentacules`.
La révision visuelle A02 est maintenant validée ; la campagne est inchangée.

Les six tests ciblés, la compilation debug, le formatage et le contrôle
d'espaces passent. Onze scènes natives couvrent les quatre orientations et
huit situations ; leur chronologie mécanique est identique à A01. La page
compare la référence acceptée et A02, avec déclenchement de la mort, trois
tours, attente, pause et lecture automatique contrôlés hors navigateur.
Les images sont vérifiées après encodage sans perte. Tous les pixels de murs
restent identiques à la référence dans la scène d'obstacles. Après disparition
des tentacules, seuls trois bords d'une ligne du nombre de dégâts préparés
diffèrent de cinq à six pixels ; les deux captures ont été inspectées et ces
différences de rasterisation sont conservées dans le reçu, sans retoucher
les images natives. Les autres pixels après l'effet sont identiques.

La texture source transparente et le prompt imagegen restent dans la revue 102.
Les captures brutes et planches de contrôle sont supprimées après vérification :
90 581 576 et 322 939 octets retirés. La revue conserve environ 3,7 Mo ; les
soixante-quinze fichiers protégés gardent leurs empreintes. Le code et les
dessins d'origine acceptés ne sont pas modifiés.

### AF06-A02-ACCEPT01 — Contagion organique validée

Le joueur valide explicitement la révision par « je valide , c'est tres bien :D ».
AF06-A02 devient la référence visuelle de Contagion : tentacules organiques
du cadavre vers chaque destinataire réel, contact puis rétraction, marqueurs
persistants et découpe par les obstacles. Les règles déjà acceptées restent
inchangées ; cet accord concerne le rendu et ne constitue pas une intégration
jouable dans la campagne.

Le reçu `skill-animation-review-102/visual-acceptance-01.json` consigne cet
accord et les empreintes du code de revue, de la texture source, du prompt,
des onze atlas natifs et de la lecture animée complète. Les accords précédents
sur AF05-A01 et AF06-A01 sont conservés. Aucun autre candidat en attente ne
reçoit de validation implicite. Les données encore utilisées par l'aperçu sont
conservées ; les captures intermédiaires ont déjà été nettoyées.

### D66-TAUNT-FINAL01 — contrôle final de la provocation

Après validation de Contagion AF06-A02, le joueur demande de continuer.
La page partagée présente les deux éléments de provocation dont le reçu visuel
reste ouvert : 101 / D66-TAUNT-A01 et 103 / D66-TAUNT-I01. Le dessin et la hauteur
du Gardien restent validés ; aucune nouvelle revue de son corps n'est demandée.

La revue 103 réutilise les huit atlas natifs de la revue 94 sans modifier leurs
pixels, leurs identifiants ni leurs chronologies. Les deux fenêtres permettent
de comparer la zone ouverte et deux Gardiens, ou de choisir une cloison, un rappel
et l'attente sans tour. La prévisualisation disparaît à la confirmation ;
l'impulsion est brève, tandis que les marqueurs suivent les tours et le rappel.
Les cinq situations, les quatre directions de la scène ouverte, la pause et
les tours manuels sont vérifiés hors navigateur. Les scènes avec attente gardent
leurs statuts même quand l'autre scène passe un tour.

Les empreintes de chaque image native et les états reconstruits sont contrôlés.
Une planche native est inspectée puis supprimée. Aucune nouvelle capture du jeu
ni aucun export animé n'est créé. Les atlas de Contagion A02 désormais inutilisés
sont retirés d'après le reçu accepté ; sa texture source, son code, son animation
complète et son accord restent conservés. La revue ne modifie ni la campagne,
ni les chiffres proposés de provocation/capacité. O03 et B03 gardent leur suivi
séparé. Les reçus et contrôles sont dans `skill-animation-review-103/`.

### D66-TAUNT-FINAL01-ACCEPT01 — provocation et marqueur validés

Le joueur répond « ca semble fonctionner correctement :) » à l'aperçu final.
Les candidats 101 / D66-TAUNT-A01 et 103 / D66-TAUNT-I01 sont donc validés pour
leur rendu montré : impulsion cuivrée, marqueur unique transparent, retrait de
la prévisualisation, expiration et rappel. Le dessin et la taille déjà acceptés
du Gardien restent conservés.

Le reçu `skill-animation-review-103/visual-acceptance-01.json` fige cet accord,
les huit atlas natifs, le marqueur, la lecture animée complète et le code du
laboratoire. Il ne valide pas les chiffres de portée, de durée ou de capacité,
ni une intégration jouable encore à réaliser. O03-A01 garde sa revue visuelle
séparée ; B03 conserve sa réflexion mécanique en attente. Les images encore
utilisées par la page restent disponibles ; aucun nouveau cache n'est créé.

### O03-FINAL01 — Algorithmes de combat améliorés

Le joueur demande de poursuivre après l'accord visuel de la provocation du
Gardien. La revue 104 reprend le candidat stable 102 / O03-A01, dont le retour
visuel d'attaque restait à examiner. Les circuits bleus de l'essai précédent
restent brefs ; les dessins natifs acceptés du Gardien, du Traqueur et de
l'Éclaireur remplacent seulement leurs anciens exports dans ce laboratoire.
L'Artilleur conserve ses chenilles acceptées. Aucun corps n'est remis à valider.

Algorithmes reste un passif permanent sans coût ni recharge. Le bouton de
l'aperçu sert à observer une attaque, pas à activer le passif en campagne.
Le bonus accepté au rang 1 est de 25 % ; le coup préparé de 20 devient 25 avant
les défenses. Ces dégâts et les 160 PV de la cible sont des exemples. L'impulsion
s'éteint rapidement après chaque attaque ; une attente sans tour ne crée aucun
coup. L'Éclaireur et la Mule restent sans attaque. Les cloisons bloquent les
attaques et les machines produisent des étincelles, sans sang.

Le laboratoire isolé `--lot-algorithmes-combat` et la page réutilisée montrent
seize scènes natives de 144 images, avec les trois drones armés dans les quatre
directions et les cas de cloison, de cible mécanique, de soutien et d'attente.
Les chronologies mécaniques sont exactement identiques à la revue 93 ; son code
historique est intact. Les deux tests ciblés, la compilation debug et le formatage
passent. Le contrôleur est vérifié hors navigateur : deux scènes simultanées,
trois tours manuels, pause, rejeu et attente indépendante. Aucun rendu HTML
observé ni succès de la suite complète n'est revendiqué.

Tous les pixels des atlas encodés sans perte sont comparés aux captures natives.
Les trois lectures animées complètes sont vérifiées pour leurs 144 images et
six secondes. La planche de contrôle native est inspectée puis retirée avec les
captures brutes et les journaux. Les atlas de provocation désormais inutilisés
sont également supprimés ; son code, son marqueur, sa lecture complète et son
accord restent conservés. Les détails chiffrés de capacité/provocation et B03
restent séparés ; aucune intégration au combat ou modification de sauvegarde
n'est ajoutée. Le rendu O03 présenté attend son accord propre.

### O03-NO-CIRCUIT01 — aucun effet supplémentaire pour le passif

Le joueur répond « euh le circuit affiché n'est pas necessaire :) » à la revue
104. Le circuit est retiré, sans nouvelle animation de remplacement. Algorithmes
reste un bonus passif permanent ; ses dégâts, les dessins acceptés et les
animations ordinaires d'attaque/impact sont conservés. Cette décision clôt la
proposition de circuit ; elle n'en constitue pas une validation visuelle.

La fonction de dessin et son appel sont supprimés de l'entrée finale O03 et de
l'ancien laboratoire Gardien/Algorithmes, pour éviter de retrouver l'effet dans
une entrée historique. Le reçu `skill-animation-review-104/no-circuit-decision.json`
conserve le retour exact et les empreintes avant/après de ces retraits. Les
circuits proposés ne doivent pas être republiés. La page partagée présente la
décision sans animation supplémentaire. Les atlas et lectures de l'effet retiré
sont nettoyés, avec un manifeste de suppression ; les sources de drones, icônes,
règles, code et reçus restent conservés. Aucun autre passif ou rendu accepté
n'est modifié. Le raccordement au combat reste à effectuer ; B03 et les chiffres
de capacité/provocation gardent leur suivi séparé.
