# Refonte du contexte et du personnage

Décisions du 6 octobre 2026. Le nouveau contexte est celui d'un agent humain
augmenté réveillé de cryogénie, dans un univers cyberpunk sombre confronté à une
menace démoniaque. La refonte concerne la campagne, les cinq classes de départ,
les compétences actives et passives, les statistiques et la création du
personnage. Les décisions de conception ci-dessous ne sont pas encore des
fonctionnalités intégrées au jeu.

## Validation des textes et effets

Consigne du joueur du 6 octobre 2026 : « tous les textes / effets devront
etre validés par moi meme ». Chaque texte destiné au joueur et chaque effet
mécanique ou visuel nouveau ou modifié doit être présenté avant intégration.
Le texte, les chiffres et le rendu visuel reçoivent des validations
distinctes. Un accord sur les silhouettes ou les noms de classes ne valide
pas de nouvelles descriptions, des paramètres ou des animations.

Les propositions sont regroupées dans la
[revue des textes et effets](REVUE_TEXTES_EFFETS_AGENT.md), avec des
identifiants stables. Les formulations et effets non approuvés restent
hors du contenu jouable. Les éléments déjà acceptés gardent leur accord ;
une modification de leur formulation ou de leurs paramètres exige une
nouvelle revue pour la partie modifiée.

La [revue des compétences des autres classes](REVUE_COMPETENCES_AGENT_LOT_02.md)
complète ce premier lot. L'accord « ok » du joueur après le tableau du
Bastion retient les propositions présentées dans cette réponse : les
exemples aux niveaux 1, 10 et 12, le déplacement du bonus de vie de
l'implant vers le passif et le cyan pour un niveau augmenté. Il ne donne
pas d'accord aux nouveaux pouvoirs ou aux paramètres non présentés.

Les [arbres de compétences](REVUE_ARBRES_COMPETENCES_AGENT.md) font aussi
partie de la refonte, à la demande du joueur. La proposition réunit cinq
arbres partagés, accessibles à toutes les classes, avec des branches pour
les drones, les éléments magiques et les transformations. Le joueur a choisi
un déblocage par seuil de points investis dans chaque branche. La première
grille de quatre lignes de trois compétences est maintenant en réexamen :
le joueur retenait alors quatorze branches et jugeait que douze compétences chacune
pourraient être trop nombreuses. Il demande d'abord une liste de compétences
différentes, chacune avec une idée d'effet visuel. Les options d'un palier
restent toutes apprenables avec des points, sans choix exclusif.
Les seuils seront définis après le catalogue selon le nombre de compétences
de chaque branche ; les anciens seuils proposés de 5 et 10 points sont
retirés. Le joueur accepte ensuite le
[catalogue présenté](PROPOSITION_COMPETENCES_PRINCIPALES_AGENT.md) : six
compétences par branche et huit pour Drones, soit 86 au total initialement. Les noms,
rôles, effets décrits et intentions visuelles sont retenus ; les
descriptions définitives, barèmes, paliers et rendus restent à valider.
L'accord ne s'étend pas aux paramètres supplémentaires des anciennes fiches ;
les bonus d'objets ne compteraient pas pour les seuils.

Chaque future fiche de compétence doit comporter un texte descriptif
roleplay, les effets chiffrés de référence et les valeurs actuelles.
Le joueur précise qu'il jugera chaque description individuellement.
La [revue du format](REVUE_DESCRIPTIFS_COMPETENCES_AGENT.md) conserve cette
demande et distingue les valeurs de référence du résultat du personnage.

Le premier lot de mêlée présenté est accepté comme base de travail.
Le joueur souhaite faire les équilibrages progressivement, plus tard.
Les révisions conserveront les validations distinctes et les identifiants.
Après l'implémentation de toutes les compétences et animations, un
[menu du personnage de test](REVUE_PERSONNAGE_TEST_AGENT.md) devra permettre
de se respécialiser et de gagner des niveaux à volonté dans la salle de test.
Le joueur choisit un menu de configuration ; ce travail est explicitement différé.

Les monstres auront leurs propres compétences. Le nombre utilisable
dépendra de leur rôle, de leur niveau et de leur importance. Le sens et
les catégories de cette importance seront définis plus tard ; aucun
nombre, barème ou comportement n'est déduit automatiquement de ce critère.
La [revue des compétences de monstres](REVUE_COMPETENCES_MONSTRES_AGENT.md)
conserve ces décisions, sans attribuer de pouvoirs nouveaux à un monstre.

Les [arbres secondaires](REVUE_COMPETENCES_SECONDAIRES_AGENT.md) ajoutent
furtivité, mines et grenades avec une progression distincte : chaque actif
s'achète une seule fois, chaque passif possède dix rangs appris, et tous
les passifs sont achetables immédiatement sans exiger l'actif associé.
Ils utilisent les mêmes points de compétence et ne suivent pas la grille
des branches principales. Les mines et grenades de ces actifs dépensent
uniquement de l'énergie. Leurs fiches séparent les règles du joueur des
textes et barèmes proposés.
Avant toute pose de mine ou tout lancer de grenade, le joueur demande
un aperçu de la zone d'action pour le type choisi. La grenade sonore
doit attirer vers le centre de sa zone les monstres qui l'entendent,
avec une portée plus grande que les autres variantes de grenade.
Une mine déjà posée affiche également sa zone d'action lorsque la souris
ou le curseur du jeu survole sa case et qu'il n'y a rien d'autre à inspecter.
Si la mine partage sa case avec autre chose à inspecter, le même survol
ouvre un petit tooltip de choix. Le joueur peut sélectionner la mine pour
afficher son rayon ou choisir les informations d'un autre élément connu.
Cette règle remplace la proposition de priorité automatique à l'autre occupant.

Pour plus tard, le joueur demande un
[espace d'icônes de buffs et debuffs](REVUE_INTERFACE_EFFETS_AGENT.md)
du personnage. Le survol montre le nom, l'effet et le temps restant en
tours. Le haut à droite de la zone de jeu est un emplacement proposé ;
la maquette et les icônes restent à présenter avant intégration.

## Contexte retenu

Une organisation secrète réveille l'agent et l'envoie nettoyer les zones
souterraines. Ses renseignements évoquent un plan visant à faire remonter une
menace et envahir le monde de la surface.

Le joueur commence par traquer de petites menaces. Son enquête révèle
progressivement une autre organisation qui a tenté d'assujettir des démons
grâce à la technologie. Il découvre ensuite que cette tentative a échoué.
Sa mission se poursuit dans les profondeurs, jusqu'à essayer de détruire la
racine du mal.

La découverte de l'organisation adverse et de son échec appartient à la
progression de l'histoire. Le briefing initial ne révèle pas d'emblée ces
informations. L'organisation qui emploie l'agent et celle qui a mené les
expériences sont deux rôles distincts ; aucun lien secret, mensonge ou
retournement entre elles n'est encore défini.

Le cadre reste futuriste, industriel et cyberpunk sombre. La présence
démoniaque s'ajoute à cette identité. La magie est réelle et la technologie
peut la canaliser sans l'expliquer entièrement. La manière dont l'agent
acquiert cet accès, la nature exacte de la racine du mal et les noms des
organisations restent à choisir.

Le lore comprend plusieurs dimensions démoniaques. Pendant l'exploration
des souterrains, des portails permettent de rejoindre des cartes bonus
situées dans ces dimensions. Ces cartes contiennent des monstres plus
coriaces et offrent de meilleurs gains d'expérience et d'équipement.
Elles ajoutent des détours facultatifs à la descente. Le retour, la fréquence
des portails et la manière dont les dimensions interviennent dans la quête
principale restent à définir.

## Cinq orientations de départ

Les classes donnent un aspect, des compétences, des statistiques et de
l'équipement de départ. Elles orientent le début de partie sans obliger le
joueur à conserver le même style de jeu. Les cinq noms et les orientations
ont été acceptés le 6 octobre 2026. Les pouvoirs précis restent à détailler.

| Classe | Orientation demandée |
| --- | --- |
| 1 | Combat de mêlée et magie de renforcement de mêlée, avec une priorité à la survie. |
| 2 | Invocation de drones et affaiblissements techno-occultistes contre toutes les natures d'ennemis. |
| 3 | Combat à distance et magie de renforcement à distance, avec une priorité aux dégâts et à l'esquive. |
| 4 | Magie de dégâts. |
| 5 | Magie et transformations, avec un rôle polyvalent. |

Toutes les compétences actives et passives sont accessibles à toutes les
classes. Chaque classe possède néanmoins un bonus et un malus qui lui
donnent une identité au départ. Ces traits sont remplaçables plus tard.
Le remplacement se fait par des implants trouvés. Leurs effets restent à
définir ; ils ne créent pas de compétences exclusives. Le remplacement doit
traiter ensemble le bonus et sa contrepartie, sans réinitialiser les
compétences apprises.

L'apprentissage et l'amélioration des compétences passent entièrement par
les points gagnés aux niveaux. Tous les passifs appris fonctionnent en
permanence, sans sélection d'un nombre limité à équiper. Les compétences de
départ sont accordées par la classe ; le catalogue accessible ensuite est
commun. La dernière décision accorde un actif, le premier de la classe,
et un passif au départ, tous deux au niveau appris 1. Le second actif
doit être appris avec les points de compétence.
Les barèmes de l'ancien personnage ne constituent pas automatiquement les
barèmes du nouvel agent.

Chaque niveau gagné donne 5 points d'attribut à répartir librement et
1 point de compétence. Les attributs n'ont pas de plafond de conception.
Apprendre une compétence au niveau 1 ou augmenter son niveau appris de 1
coûte toujours 1 point. Le plafond appris est 10, pour les actifs comme
pour les passifs. Le niveau 1 gratuit d'une compétence de départ compte
dans ces dix niveaux : neuf achats supplémentaires permettent d'atteindre 10.
Cette lecture retient le « cap de 10 » demandé par le joueur.

Le niveau effectif ajoute au niveau appris les bonus des objets équipés.
Il peut dépasser 10 ; les formules doivent continuer au-delà de ce niveau.
Des bonus de compétences entrent dans les propriétés trouvables du loot.
Leurs portées, valeurs et probabilités restent à valider dans la revue.
Les bonus ne constituent pas une nouvelle méthode d'apprentissage.
Le niveau effectif affiché doit avoir une couleur différente lorsqu'il
est augmenté par un bonus. La couleur et son aperçu restent à valider ;
le détail des niveaux appris et ajoutés doit aussi rester lisible. La
famille de couleur cyan est retenue ; son rendu exact reste à présenter.

## Cinq profils retenus

Les noms et les orientations des bonus et malus ont été acceptés le
6 octobre 2026. Les valeurs suivantes constituent une première proposition
d'équilibrage à tester. Chaque profil répartit 34 points ; l'ordre des
valeurs est Force, Agilité, Vitalité,
Perception, Intelligence et Volonté. Les points acquis aux niveaux restent
libres, quelle que soit cette répartition de départ.

| Numéro et nom proposé | Attributs de départ | Bonus de l'implant initial | Malus de l'implant initial | Matériel envisagé |
| --- | --- | --- | --- | --- |
| 1 Bastion | 7, 5, 7, 5, 4, 6 | 1 point d'armure supplémentaire ; proposition révisée, le bonus de vie passe au passif. | Réserve de pouvoirs réduite de 10 %. | Hache, tenue d'assaut et soins. |
| 2 Opérateur | 4, 5, 5, 6, 8, 6 | PV des drones augmentés de 15 % et affaiblissements prolongés d'un tour. | Dégâts des armes réduits de 10 %. | Pistolet, tenue technique et soins. |
| 3 Traqueur | 4, 8, 5, 7, 4, 6 | Dégâts des armes à distance augmentés de 10 % et 2 points d'esquive supplémentaires. | Dégâts des armes de mêlée réduits de 10 %. | Fusil, couteau, tenue légère et soins. |
| 4 Arcaniste | 4, 5, 6, 6, 5, 8 | Dégâts des sorts augmentés de 15 %. | PV maximum réduits de 10 %. | Couteau, tenue de canalisation et soins. |
| 5 Métamorphe | 6, 6, 6, 5, 4, 7 | Transformations prolongées de deux tours. | Coût des pouvoirs augmenté de 10 % sous forme humaine. | Lame courte, combinaison biomécanique et soins. |

Un nouvel implant remplace la paire actuelle. La classe initiale ne bloque
ni l'apprentissage des autres disciplines, ni le port d'un autre
équipement. Les changements de maximum de PV ou de réserve devront préserver
les quantités effectivement disponibles, sans permettre de les remplir par
des échanges répétés d'implants.

Les matériels ci-dessus indiquent un rôle et non un modèle déjà attribué.
Une tenue de départ qui reprend l'aspect d'une armure existante ne reçoit pas
automatiquement la puissance ou la rareté de cette armure.

Le joueur demande que le passif initial du Bastion donne 10 % de vie en
plus à la place de la résistance aux interruptions. La proposition retire
donc les 10 % de vie de l'implant pour ne pas les compter deux fois au
départ. Ce déplacement et les exemples de progression à +10 %, +28 % et
+32 % aux niveaux effectifs 1, 10 et 12 sont retenus après l'accord du
joueur. Les autres paramètres de l'implant et les détails non présentés
restent à valider. Le passif appris reste acquis lorsqu'un implant est remplacé.

## Départ des cinq classes

Le départ avec un actif et un passif appris par classe remplace le départ
avec deux actifs proposé précédemment, en plus du bonus et du malus de
classe. Le départ comprend exactement un niveau appris dans une compétence
active et un niveau dans un passif. Les dernières corrections du joueur
figurent ensemble dans ce tableau ; le Bastion demeure la classe de mêlée.
Les noms et les autres paramètres du catalogue restent à détailler.
Les mêmes compétences pourront être apprises par les autres personnages avec
leurs points de niveau.

| Classe | Un actif appris au niveau 1 | Un passif appris au niveau 1 |
| --- | --- | --- |
| Bastion — mêlée et survie | Frappe de rupture : coup de mêlée renforcé, +40 % de dégâts au niveau effectif 1. | Vitalité améliorée : +10 % de vie maximum. |
| Opérateur — drones et affaiblissements | Une invocation de drone d'attaque ; affectation proposée à l'Artilleur, tile 59. | Lien renforcé : +25 % de dégâts pour tous les drones. |
| Traqueur — distance et esquive | Marquage balistique instantané : précision multipliée par 1,15. | +10 % de dégâts des armes et compétences d'attaque à distance, en permanence. |
| Arcaniste — magie offensive | Une compétence de magie de dégâts. Le choix exact au départ reste à préciser. | Canalisation : coût d'énergie des pouvoirs magiques réduit de 20 %. |
| Métamorphe — magie et transformations | Une compétence de transformation. Le choix exact au départ reste à préciser. | +10 % de dégâts lorsque le personnage est transformé. |

Le choix du marquage a été précisé par le joueur : le bonus augmente la
valeur de précision de 15 %, et n'ajoute pas 15 points à la chance de toucher.
La création avec un sort ou une forme au choix, ou avec un choix fixé par
classe, fait l'objet d'une question en cours. Le catalogue de sorts doit
couvrir feu, froid et occulte ; celui des formes doit reprendre
les cinq dessins disponibles. Trait arcanique et Mue prédatrice restent
des exemples de compétences, sans être imposés par défaut.

Égide occulte, Interférence, Pas spectral, Déflagration occulte et Décharge
mutagène s'apprennent ensuite avec les points de compétence. Le passif de
distance ne dépend plus d'un déplacement. Le passif du Métamorphe ne réduit
plus les dégâts reçus ; son bonus dépend de la transformation en cours.

Les chiffres, portées, coûts, durées et conditions d'application seront
définis avec les statistiques et les ressources. Une transformation ou un
drone dessiné ne reçoit pas de comportement implicite par son apparence.

Les cinq drones existants — Éclaireur 56, Mule 57, Traqueur 58, Artilleur 59
et Gardien 66 — reçoivent chacun une compétence d'invocation améliorable,
conformément à la demande du joueur. Les cinq modèles n'ont plus de délai
d'expiration ; ils gardent leurs PV et peuvent être détruits. Les trois
passifs communs sont DP01 Maintenance des drones, qui répare progressivement
leurs PV hors combat, DP02 Blindage des drones, qui réduit les dégâts reçus
en pourcentage, et O03 Lien renforcé, qui augmente leurs dégâts en pourcentage.
O03 est déjà retenu pour le départ de l'Opérateur ; il n'est pas acheté deux fois.
Le joueur retient cinq drones actifs simultanément, avec un seul exemplaire
de chaque modèle. Le déploiement utilise l'énergie commune, sans entretien
par tour. Rappel et changement de couche conservent PV et chargement, sans
soin gratuit. La Mule détruite dépose son matériel au sol. Ces décisions
sont enregistrées dans la [révision de persistance](PROPOSITION_DRONES_PERSISTANTS_AGENT.md).
Les valeurs détaillées et les descriptifs sont présentés
dans la [revue descriptive des drones](REVUE_DRONES_AGENT_LOT_05.md).
La [première revue](REVUE_DRONES_AGENT.md) conserve l'historique des anciennes
durées, remplacées par cette décision. Les nouveaux chiffres, coûts et interfaces
des commandes, ainsi que les conditions de réparation, restent à valider.

## Six statistiques retenues

Les attributs sont Force, Agilité, Vitalité, Perception, Intelligence et
Volonté. Intelligence porte le versant technique, les drones et le hacking.
Volonté porte la magie, la résistance occulte et les transformations.
Cette séparation permet de distinguer les deux orientations tout en gardant
leur combinaison accessible.

La répartition de départ est fixe pour chaque classe. Le joueur gagne
ensuite 5 points d'attribut à répartir librement par niveau, sans plafond
de conception. Le rôle chiffré de chaque attribut et le budget initial
restent à valider. Des probabilités ou des distances peuvent être bornées
sans plafonner l'attribut qui les influence ; les autres contributions de
ce même attribut continuent de progresser. La création devra expliquer leurs effets
avec des phrases courtes et montrer les valeurs de combat qui en résultent.

Nouvelle exigence du joueur : « les caracteristiques du personnage devront
avoir un interet aussi ». Chaque attribut doit améliorer des résultats
réels du personnage et interagir avec les compétences appropriées.
Un palier de vision, une durée plafonnée ou une probabilité limitée ne
doit pas supprimer tous les bénéfices de l'attribut à forte valeur.
Cela n'oblige pas chaque attribut à renforcer tous les styles de combat.

Les ajustements proposés ajoutent des dégâts physiques à l'Agilité, des
dégâts d'armes à distance à la Perception et 2 points d'énergie maximale
par point d'Intelligence ou de Volonté indépendamment. Les compétences
emploient leur niveau effectif et les attributs associés ; un niveau
de compétence ne remplace pas les attributs. Ces nouvelles formules
restent à valider.

## Ressource commune et pouvoirs

Le système doit couvrir la survie au contact, la précision et l'esquive, les
drones et l'affaiblissement, la magie offensive et les transformations.
Les attributs doivent permettre des personnages hybrides sans rendre une
seule statistique indispensable à tous les styles.

Les compétences technologiques et magiques utilisent une réserve commune.
Les munitions des armes restent gérées séparément. Le nom de cette réserve,
sa capacité et les coûts restent à définir, de même que la place de la
chaleur et du contrôle des drones.

La réserve se régénère au fil des tours, plus lentement en combat. Une action
de récupération consomme un tour et permet de la reconstituer plus vite.
Des consommables apportent une récupération d'urgence. Ces mécanismes
suivent le temps du jeu ; attendre devant un menu n'apporte aucun gain.

Le joueur accepte la notion de calme à condition de rendre l'état visible
dans l'UX. Il demande d'indiquer les hostiles actuellement repérés et la
phase combat ou calme. Survoler le statut des hostiles met tous les hostiles
visibles en surbrillance. La [revue du HUD](REVUE_INTERFACE_EFFETS_AGENT.md)
propose un retour au calme avec décompte et une phase commune à l'agent
et aux drones pour l'énergie et Maintenance. Les libellés, les conditions
chiffrées et le rendu de cette proposition attendent une revue distincte.

Les transformations adoptent plusieurs formes spécialisées pendant quelques
tours. Elles remplacent l'apparence et les attaques ordinaires par celles de
la forme. Les statistiques de l'équipement et tous les passifs appris restent
actifs. Les effets de type de dégâts des armes actives juste avant la
transformation sont transmis aux attaques transformées, selon les effets
réels des objets ; les armes rangées ne servent pas de source. Les actions
propres à la forme et Décharge mutagène restent utilisables ; les autres
actifs appris sont indisponibles. Décharge mutagène conserve son rôle de
prolongation. L'équipement se retrouve au retour à la forme humaine.
Chaque forme doit offrir un intérêt assez fort pour cette restriction ;
M01 fait l'objet d'une proposition renforcée avec saignement. Les durées,
valeurs précises, cumuls, autres effets d'arme et rendus non présentés
restent à valider. Décisions : TRANS-LOCK01 / TRANS-EQ01 / TRANS-PWR01.

Les affaiblissements techno-occultistes peuvent atteindre toutes les natures
de cibles : machines, êtres organiques et démons. Leur usage ne dépend pas de
la présence d'un implant ou d'un système piratable. Les effets précis et les
éventuelles résistances restent à définir.

La magie offensive couvre trois familles : feu, froid et énergie occulte.
La branche Énergie occulte doit évoquer la magie noire : sang, ombres et
sceaux impies, selon la direction OC-NOIR01 demandée le 7 octobre. Prison
sigillaire utilise un cercle sanglant dans sa proposition révisée ; les
textes et visuels exacts modifiés restent soumis à leur revue, avec les
effets déjà validés conservés.
Le joueur retire toute la branche Foudre le 7 octobre, sous AR-FO-REMOVE01,
avec ses six compétences, sans remplacement ni transfert de leurs effets.
Le catalogue actuel compte 77 compétences principales dans 13 branches.
OC06 Rupture dimensionnelle est retirée sans remplacement sous OC06-REMOVE01 ;
les portails d’exploration vers les dimensions démoniaques restent conservés.
OC07 Réanimation est ajoutée séparément après validation de sa fiche
le 7 octobre : marque un ennemi vivant puis le relève temporairement
comme allié autonome à sa mort, avec ses statistiques et compétences
d’origine et une puissance réglée par le rang. Les boss sont exclus et
le nouveau remplace le plus ancien si cinq réanimés sont déjà présents.
Le RP, les valeurs et l’intention visuelle affichés sont validés ; les
courbes complètes, cas non montrés et pixels natifs restent à revoir.
Les anciennes revues et leurs accords restent archivés ; les identifiants
FO01 à FO06 ne sont pas réutilisés. Chacune pourra proposer des attaques directes, des zones et des
effets persistants. Les sorts précis et leurs résistances restent à définir.

## Aperçu du personnage

La création affiche le personnage correspondant à la classe sélectionnée,
avec son équipement initial. L'aperçu doit reprendre les ressources et les
couches utilisées sur le plateau afin que le joueur voie le personnage avec
lequel il commencera réellement.

Les nouvelles silhouettes, tenues ou compositions suivent la procédure de
[direction artistique des tiles](DIRECTION_ARTISTIQUE_TILES.md) : candidats
numérotés à taille native de 32 × 32, zoom entier et revue en situation avant
leur intégration. Les dessins déjà acceptés peuvent servir lorsqu'ils
conviennent au personnage défini. La classe décrit son départ ; l'évolution
de son équipement doit ensuite se refléter dans son apparence.

La [première planche de tenues](../assets/source-art/refonte-agent-2026-10-06/classes-tenues-review.png)
présente cinq affectations proposées à partir de dessins déjà acceptés.
Chaque candidat apparaît à 32 × 32 puis en zoom entier ×4, sans arme.
L'affectation des cinq silhouettes aux classes a été acceptée le
6 octobre 2026 ; la planche ne constitue pas l'écran de création intégré.
Les pixels des dessins sources sont
conservés à l'identique. Le
[manifeste des candidats](../docs/reviews/refonte-agent-2026-10-06/class-candidates.json)
conserve leurs indices et leurs références.

## Remplacement de la fiction précédente

L'ancienne histoire est remplacée par ce contexte. Le protagoniste IA, les
protocoles de restauration et l'objectif d'évasion cessent de définir la
nouvelle campagne. Les dialogues, quêtes, descriptions et documents qui
dépendent de ces prémisses devront être retirés ou réécrits.

Le départ se déroule dans une nouvelle base de l'organisation. La descente
propose ensuite des refuges successifs dans les profondeurs. Les services
nécessaires au nouvel agent doivent y recevoir un rôle cohérent. La base
initiale n'est pas conçue comme un QG auquel on revient entre chaque mission.
Les habitants, les services disponibles et les règles de déplacement entre
ces refuges restent à préciser.

Les systèmes de quêtes, de génération, de combat, d'équipement et de
perception servent de socle à la refonte. Leur présence n'impose pas de
conserver l'identité ou la motivation des anciens personnages.

Le joueur accepte que la refonte rende les anciennes sauvegardes
incompatibles. Leur contenu narratif historique n'a donc pas à rester
jouable. Cette décision autorise le retrait des anciennes définitions ;
elle ne demande pas de supprimer les fichiers de sauvegarde personnels.

## Ordre de mise en place

1. Fixer les choix de fond : pouvoirs, progression, lieu de départ,
   statistiques et anciennes sauvegardes.
2. Définir les cinq profils et un premier catalogue d'actifs et de passifs
   avec des effets lisibles, des coûts et des limites explicites.
3. Remplacer le contexte jouable et construire les premiers objectifs de
   mission sans révéler les étapes ultérieures de l'end'aquête.
4. Raccorder les compétences initiales, les statistiques et les ressources
   au moteur, puis tester des personnages spécialisés et hybrides.
5. Présenter les apparences de départ, intégrer les candidats acceptés et
   vérifier l'aperçu de création avec le rendu en partie.
6. Retirer ou réécrire les références restantes à l'ancienne fiction et
   vérifier les liens documentaires, les sauvegardes selon le choix retenu,
   les tests, la compilation et les captures du parcours de création.

## Raccord à préparer dans le moteur

L'inspection du moteur le 6 octobre montre encore cinq attributs stockés
sur 8 bits et un plafond absolu par défaut de 10 dans
`src/stats/primary.rs`. Le raccord devra ajouter Volonté, élargir les
valeurs et les calculs, et retirer le plafond de conception sans remplacer
celui-ci par une saturation à 255. Les totaux et sauvegardes doivent suivre.
Les bornes de création sont distinctes de la progression libre.

`src/skills/progression.rs` stocke actuellement des techniques apprises,
sans niveaux par technique. Il faudra séparer niveau appris, bonus
d'équipement et niveau effectif, puis raccorder les formules aux actions
réelles, aux passifs, aux prérequis et aux sauvegardes. Les propriétés
de loot devront conserver la cible du bonus de compétence par instance.
La fiche du personnage et l'aperçu avant dépense devront montrer les
effets obtenus avec les mêmes calculs que le combat.
