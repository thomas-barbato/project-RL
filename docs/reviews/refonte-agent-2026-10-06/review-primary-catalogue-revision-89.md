# Catalogue retenu des compétences principales

6 octobre 2026. Après présentation de la liste, le joueur répond :
« j'accepte ce que tu me propose ». Après les retraits demandés, le catalogue retient treize
branches, six compétences par branche, sauf cinq pour Attaques à distance,
quatre pour Esquive et huit pour Drones : les cinq modèles dessinés et les
trois passifs communs. Total actuel : 77 compétences. Le 7 octobre, le
joueur retire DI06 Tir de couverture, ES02 Roulade et ES04 Riposte d'esquive sans remplacement ;
leurs identifiants restent réservés à l'historique de revue. Pas spectral
conserve son nom et ses règles validées ; Roulade et son animation sont
retirées du périmètre actif.
Le joueur retire ensuite toute la branche Foudre, FO01 à FO06, sous
AR-FO-REMOVE01. Aucun remplacement ni transfert d’effets ; les revues
précédentes restent archivées et les identifiants restent réservés.

L'accord couvre les noms, les rôles, les effets décrits et les intentions
visuelles présentées, avec les chiffres explicitement affichés. Les
identifiants restent stables. Les descriptions définitives, les barèmes
non présentés et les rendus restent à valider séparément. Cet accord
n'approuve pas les paramètres supplémentaires des anciennes fiches.
Le catalogue n'est pas encore intégré au jeu.

Les futures fiches suivent la [structure descriptive demandée](REVUE_DESCRIPTIFS_COMPETENCES_AGENT.md) :
texte roleplay, effets chiffrés de référence et valeurs actuelles. Le joueur
jugera chaque description individuellement.

Le [cadrage complet des compétences](CADRAGE_COMPETENCES_AGENT_COMPLET.md)
termine maintenant les propositions de paramètres des branches restantes,
les compléments secondaires et les règles d'accès sans paliers. Le joueur
demande de finir cette conception avant de revoir les descriptifs un à un.
Ces nouveaux paramètres ne changent pas les accords de sélection ci-dessus.

## Lecture

A désigne une compétence active, P un passif appris et toujours actif
selon ses conditions. Les actifs et passifs principaux ont dix rangs
appris, à un point par rang, avec bonus d'équipement au-delà. Depuis
AR-ACCESS01, toutes les compétences principales sont achetables dès le
départ avec les points nécessaires, sans seuil de niveau ou de points
investis dans une branche. Les coûts d'utilisation et les courbes restent
à revoir individuellement. Les classes gardent un actif et un passif de
départ au rang 1 ; toutes les branches restent accessibles à tous.

Les visuels sont des intentions retenues : silhouettes, trajectoires,
formes et rythmes distincts, lisibles sur des tiles de 32 × 32. Un passif
peut avoir un signe contextuel sur un impact, une invocation ou une jauge,
sans ajouter une aura permanente à chaque personnage. Les dessins de
drones et de formes déjà acceptés restent intacts ; les nouveaux effets
seront présentés à taille native avant intégration.

## IA-BARRIERE01 Réponse des monstres aux barrières de sorts

Le joueur étend la règle OC04-IA01 aux obstacles destructibles créés par
les sorts qui bloquent la progression des monstres, notamment Mur de glace.
Le monstre utilise un passage accessible s’il permet de rejoindre sa cible.
S’il est bloqué, il attaque une partie accessible de la barrière pour ouvrir
un passage, puis reprend son déplacement. La destruction respecte les
règles propres à chaque bloc ou segment.

FR03 Mur de glace et OC04 Prison sigillaire partagent cette règle. Les
futures barrières destructibles qui bloquent les déplacements la reprennent.
Le Gel au contact de Mur de glace conserve ses conditions validées, y compris
lorsque le coup brise un bloc. Les valeurs, textes RP, visuels et autres
effets conservent leurs validations actuelles. Le choix précis du bloc,
l’évaluation des chemins et les attaques compatibles restent à définir.
Décision : spell-barrier-npc-behavior-decisions.json. Comportement à intégrer
avec les compétences.

## 1 — Mêlée

Force porte les dégâts de contact, Agilité la précision. Les techniques
ne changent pas automatiquement les statistiques d'une arme équipée.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| B01 | Frappe de rupture | A | Un coup de mêlée renforcé ; +40 % de dégâts au premier rang déjà retenu. | Trait bref et lourd, impact anguleux qui fend la lumière. |
| ME02 | Fauchage | A | Frappe plusieurs ennemis dans un arc devant l'agent. | Croissant large suivant l'arme, arrêt net derrière le joueur. |
| ME03 | Percée | A | Un coup concentré traverse une partie de l'armure de la cible. | Pointe étroite et éclats de plaques au point touché. |
| ME04 | Repousser | A | Frappe et repousse une cible ; une collision contre un mur ou un autre monstre lui ajoute 25 % des dégâts réels du coup initial. Le monstre percuté reçoit aussi 25 %, sans nouvelle réduction par l'armure ou les résistances. | Flash carré à l'impact et courte trace de recul. |
| ME05 | Charge | A | Avance en ligne jusqu'à une cible puis porte un coup au contact. | Traînée basse de poussière et front lumineux sur l'arme. |
| ME06 | Tourbillon | A | Frappe les ennemis adjacents autour de l'agent et leur inflige un saignement léger : 10 % des dégâts réels du coup par tour pendant 3 tours. | Anneau de coups complet autour du corps, sans projectile distant. |

## 2 — Survie

Vitalité garde son rôle de maximum de vie ; les protections utilisent des
mécanismes distincts : réduction, absorption, soin ou prévention d'un décès.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| B03 | Vitalité améliorée | P | Augmente les PV maximum ; +10 % au premier rang déjà retenu. | Segment supplémentaire de la jauge de vie à la mise à jour du maximum. |
| B02 | Égide occulte | A | Réduit temporairement les dégâts directs reçus ; 25 % au premier rang déjà retenu. | Coque runique fermée qui se fissure brièvement sur les impacts. |
| SU03 | Barrière d’épine | A | Renvoie une portion des dégâts directs de mêlée subis à leur auteur ; poison et dégâts indirects exclus. Les chiffres et autres conditions restent à valider. | Nouvelle intention de renvoi au contact à proposer ; l'ancienne intention d'absorption est retirée. |
| SU04 | Régénération | A | Consacre de l'énergie et plusieurs tours au rétablissement des PV. | Fils verts refermant la silhouette au rythme des soins. |
| SU05 | Purge | A | Retire automatiquement les effets néfastes compatibles les plus récents, jusqu'à un maximum qui augmente avec le rang, sans restaurer directement les PV. | Balayage blanc du haut vers le bas et fragments d'effet expulsés. |
| SU06 | Dernier sursaut | P | Un coup normalement fatal peut laisser l'agent à un PV ; disponibilité limitée à définir. | Arrêt bref du contour, pulsation du cœur puis rupture d'un sceau. |

## 3 — Drones

Intelligence renforce les drones. Les cinq invocations sont des compétences
différentes ; les trois passifs concernent l'ensemble des drones contrôlés.
Les rôles reprennent la [revue descriptive des drones](REVUE_DRONES_AGENT_LOT_05.md),
dont les valeurs et affectations restent proposées. La
[révision acceptée](PROPOSITION_DRONES_PERSISTANTS_AGENT.md) retire l'expiration
des cinq modèles et remplace Autonomie par Maintenance des drones.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| D56 | Invocation de l'Éclaireur | A | Premier drone accessible ; partage sa vision et permet de consulter le niveau, les PV, les résistances et le loot d'une cible visible par lui ou par le joueur. | Modèle 56 intact, balayage fin sortant de son capteur. |
| D57 | Invocation de la Mule | A | Déploie le drone de transport de matériel. | Modèle 57 intact, indicateur de chargement près du coffre. |
| D58 | Invocation du drone Traqueur | A | Déploie le drone qui poursuit et attaque au contact. | Modèle 58 intact, griffure basse au moment de la frappe. |
| D59 | Invocation de l'Artilleur | A | Déploie le drone de tir ; même compétence qu'O01, départ proposé de l'Opérateur. | Modèle 59 intact, flashes alternés de ses armes. |
| D66 | Invocation du Gardien | A | Déploie le drone de protection et de défense au contact. | Modèle 66 intact, segments protecteurs autour des alliés proches. |
| DP01 | Maintenance des drones | P | Répare progressivement leurs PV hors combat. | Proposition à valider : brève impulsion verte quand des PV sont effectivement réparés. |
| DP02 | Blindage amélioré | P | Réduit en pourcentage les dégâts qu'ils reçoivent. | Petites plaques lumineuses arrêtant une partie des impacts. |
| O03 | Algorithmes de combat améliorés | P | Augmente leurs dégâts ; +25 % au premier rang confirmé. | Impulsion dans leurs circuits au moment d'une attaque. |

## 4 — Affaiblissements

Ces pouvoirs techno-occultistes s'appliquent aux machines, êtres organiques
et démons. Intelligence porte leur application ; aucune branche réservée
aux seules machines n'est créée.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| O02 | Interférence | A | Diminue les dégâts directs infligés par une cible. | Parasites en créneaux autour de ses armes ou membres d'attaque. |
| AF02 | Faille d'armure | A | Réduit temporairement sa protection physique. | Fissures rouges dessinées sur son contour, sans changer son équipement. |
| AF03 | Silence | A | Bloque temporairement ses compétences actives, en laissant les attaques ordinaires. | Anneau barré qui ferme ses points d'émission. |
| AF04 | Rupture de régénération | A | Empêche temporairement ses soins et réparations. | Impulsions de soin coupées par une grille noire. |
| AF05 | Désorientation | A | Force un ennemi à attaquer la cible la plus proche, alliée ou ennemie, avec ses attaques ordinaires et ses compétences offensives ; les dégâts reçus interrompent l'effet. | Nouveau rendu à proposer ; ancien leurre abandonné. |
| AF06 | Contagion | P | À la mort d'une cible affaiblie par l'agent, transmet une version limitée du sceau à un voisin. | Sceau qui se détache et saute vers la nouvelle cible. |

## 5 — Attaques à distance

Ces techniques emploient l'arme à distance, sa précision et ses munitions.
Perception et Agilité conservent leurs contributions. Les compatibilités
avec les familles d'armes devront être explicites.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| DI01 | Tir perforant | A | Traverse plusieurs cibles alignées, avec les obstacles comme limite. | Trait fin continu et impacts successifs le long de la ligne. |
| DI02 | Feu nourri | A | Enchaîne deux séquences du mode de tir actif de l'arme contre une même cible ; les rangs améliorent les dégâts par projectile. | Impacts rapprochés et recul bref répété de l'arme. |
| DI03 | Volée | A | Répartit les projectiles dans un cône pour toucher plusieurs cibles. | Éventail de trajectoires séparées devant l'agent. |
| DI04 | Tir de précision | A | Un tir sur une cible, avec précision et chance de critique augmentées, qui ignore une partie de l'armure. | Nouvelle intention à proposer ; ancien rebond retiré. |
| DI05 | Tir de harpon | A | Accroche une cible et la rapproche si le passage est libre. | Fil tendu entre l'impact et l'agent, rétraction visible. |

## 6 — Esquive

Agilité conserve son rôle de précision et d'esquive. Le déplacement, le
leurre, la riposte et la défense contre les projectiles sont différenciés.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| T02 | Pas spectral | A | Déplace d'une case légale et augmente brièvement l'esquive. | Silhouette décalée puis contour léger maintenu pendant le bonus. |
| ES03 | Leurre holographique | A | Place une fausse cible temporaire qui peut détourner les attaques. | Double légèrement transparent, avec lignes de balayage verticales. |
| ES05 | Traversée de phase | A | Passe de l'autre côté d'un ennemi adjacent si la destination est légale. | Silhouette fragmentée en bandes au passage de la cible. |
| ES06 | Réflexes balistiques | P | Renforce l'esquive contre les projectiles, sans renforcer toutes les défenses. | Courte déviation visuelle au passage d'un projectile évité. |

## 7 — Feu

Volonté porte les sorts. La brûlure, la projection en cône, le barrage,
le retard d'explosion et les interactions avec des ennemis embrasés sont
des fonctions distinctes.

EL-STAT01 : une chance de brûlure est désormais demandée pour les compétences
de Feu, avec une chance différente par compétence. Les effets spécifiques
déjà validés, dont la brûlure de Braise, sont conservés. Les compléments
chiffrés seront revus individuellement avant intégration.

EL-FREQ01 demande une Brûlure plus fréquente aux rangs élevés. Les nouvelles
chances affichées sont validées ; les courbes non montrées restent proposées.
Les accords sur les autres effets sont conservés. Braise garde sa Brûlure spécifique sans nouveau jet.
Revue : [Fréquence des statuts](../docs/reviews/refonte-agent-2026-10-06/elemental-status-frequency-rebalance-proposal.json).

EL-ZONE01 : rester plusieurs tours dans un sort à zone persistante doit
augmenter le risque de subir son statut. Mode de cumul et sorties de zone
validés : base de 10 % donnant 10 %, 20 %, 30 % à chaque jet successif,
remise à zéro à la sortie. Une détonation unique ne devient pas une zone
persistante ; timing et zones superposées restent à revoir.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| FE01 | Braise | A | Projectile étroit qui blesse puis laisse une brûlure sur la cible touchée. | Braise allongée, queue sombre et point incandescent persistant. |
| FE02 | Souffle de feu | A | Projette un cône de feu qui continue automatiquement tant que l'agent ne bouge pas, n'attaque pas avec ses armes et n'utilise pas une autre compétence ; peut brûler les cibles avec une chance croissante selon l'exposition. | Langues de feu étagées devant les mains ; indication de brûlure sur les cibles. |
| FE03 | Mur de feu | A | Crée une ligne temporaire dangereuse à traverser ; peut brûler les cibles avec une chance croissante selon l'exposition. | Bande de flammes verticales, interruptions aux obstacles ; indication de brûlure sur les cibles. |
| FE04 | Émanation soudaine | A | Marque une zone puis la fait exploser après un délai, avec des dégâts légèrement plus élevés au centre et une chance de brûlure. | Fissures lumineuses au sol, puis jets de feu vers le haut. |
| FE05 | Boule de feu | A | Projette une boule de feu qui provoque une grande explosion à l'impact. | Boule incandescente suivie d'une courte traînée de flammes, puis large déflagration circulaire. |
| FE06 | Combustion | A | Consume la brûlure restante d'une cible pour produire un choc et peut provoquer une nouvelle brûlure après ce choc. | Flammes resserrées sur la victime puis flash concentré ; elles reprennent si une nouvelle brûlure s'applique. |

CTRL-EL01 : les boss peuvent recevoir Gel et Brûlure, avec un malus de
chance renforcé. Règle validée : −50 % de la chance finale. Le Poison garde ses règles et résistances ordinaires sans malus
lié à l'importance. Les autres contrôles conservent leur immunité de boss.
Accord : [Boss et statuts](../docs/reviews/refonte-agent-2026-10-06/boss-elemental-status-acceptance-decisions.json).

## 8 — Froid

Le Gel bloque les déplacements, sans supprimer les attaques ni les autres
actions. Appliquer une Brûlure supprime un Gel ; appliquer un Gel à une
cible qui brûle supprime sa Brûlure. La cible brûlante reste admissible au
Gel. Les cinq tours demandés pour FR01 ne sont pas une durée globale.
La zone de mine glacée garde son accord : mouvement bloqué, attaques possibles ;
sa compatibilité avec les nouveaux compléments reste à revoir.

EL-STAT01 : une chance de gel est désormais demandée dans la branche Froid,
avec une chance différente par compétence et conservation des effets
spécifiques déjà validés. Les sorts sans attaque propre et les passifs
auront leurs déclenchements présentés individuellement. Le gel suit CTRL01,
les dégâts restent applicables, et ses valeurs sont consultables sans drone
dans l'analyse de cible selon HUD06.

EL-FREQ01 demande aussi un Gel fréquent aux rangs élevés. FR01 propose
77,8 % puis 82,5 % contre la même cible fictive ; ces chiffres sont validés.
Les autres sorts gardent des chances propres et seront présentés individuellement.

EL-GEL01 et EL-BAL01 : interaction réciproque des statuts et dégâts de
Froid légèrement inférieurs aux sorts de Feu comparables. Aucun coefficient
uniforme n'est fixé ; valeurs, durées et déclenchements seront revus par sort.
Décisions : [Gel / Brûlure](../docs/reviews/refonte-agent-2026-10-06/frost-freeze-and-burning-interaction-decisions.json).

EL-ZONE01 s'applique également aux zones persistantes de froid et aux
autres sorts à zone possédant une chance de statut ; les effets spécifiques
déjà validés restent conservés.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| FR01 | Aiguille de givre | A | Blesse une cible et peut la geler pendant cinq tours ; un Gel appliqué remplace sa Brûlure et une Brûlure appliquée remplace son Gel. | Aiguille blanche et petits cristaux ; glace autour des pieds si le Gel s’applique et extinction des flammes remplacées. |
| FR02 | Nova de givre | A | Immobilise brièvement les ennemis proches, sans supprimer leurs attaques. | Anneau givré qui s'élargit ; glace limitée aux pieds des cibles. |
| FR03 | Mur de glace | A | Crée une barrière temporaire destructible que les ennemis ne peuvent pas traverser ; peut geler les hostiles qui la frappent au contact. | Blocs translucides qui montent et se fissurent ; givre aux pieds de l’attaquant gelé. |
| FR04 | Vague de froid | A | La bande de départ se place autour du joueur avant confirmation ; la vague avance dans la direction correspondante et inflige dégâts et chance de Gel aux hostiles touchés. | Aperçu de largeur et flèche de direction avant lancement ; front de brume bleutée et cristaux de givre qui avance en bande. |
| FR05 | Blizzard | A | Une zone subit des impacts de froid répartis sur plusieurs tours ; les cibles frappées peuvent être gelées, avec une chance croissante selon leur exposition. | Grêle oblique et petites frappes dispersées dans la zone ; givre aux pieds des cibles gelées. |
| FR06 | Explosion cristalline | A | Fait détoner tous les hostiles gelés visibles, les blesse ainsi que les ennemis autour, puis consomme leur Gel. | Détonations de givre autour des cibles gelées ; leur couche de glace se brise. |

## 9 — Énergie occulte

OC-NOIR01 : le joueur demande une identité de magie noire pour toute la
branche, avec sang, ombres et sceaux impies. Prison sigillaire devient un
cercle sanglant ; son RP et l’intention visuelle affichée sont validés,
avec pixels natifs et détails non présentés à revoir.
Les fiches déjà validées gardent leurs accords et leurs règles ; leurs
futures adaptations visuelles seront présentées séparément. Décision :
occult-black-magic-direction-decisions.json.

Les pouvoirs emploient Volonté. Canalisation reste un seul passif agissant
sur les pouvoirs magiques compatibles de toutes les branches ; il n'est
pas dupliqué dans Feu et Froid.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| A01 | Trait arcanique | A | Frappe directement une cible avec de l'énergie occulte. | Trait violet court, nœud noir au centre de l'impact. |
| A02 | Déflagration occulte | A | Frappe les acteurs d'une petite zone choisie, selon la fiche existante. | Sceau au sol puis expansion de fragments anguleux. |
| A03 | Canalisation | P | Réduit les coûts magiques ; −20 % au premier rang confirmé. | Impulsion resserrée vers les implants lors du paiement d'un sort. |
| OC04 | Prison sigillaire | A | Enferme temporairement une zone avec un cercle rituel destructible. | Cercle sanglant tracé au sol en huit segments de sceaux rouge sombre ; segment effacé lorsqu’il est détruit. |
| OC05 | Rayon vorace | A | Un rayon entretenu vole une part de la vie infligée à sa cible. | Fil noir veiné de rouge, pulsations rouge sombre revenant vers l’agent à chaque drain ; rupture du fil à l’arrêt. |
| OC07 | Réanimation | A | Marque un ennemi vivant ; à sa mort, le relève temporairement comme allié autonome conservant ses compétences, cinq au maximum. | Petit sceau de sang sous la cible ; brève pulsation rouge sombre à sa réanimation, puis tête de mort alliée. |

La fiche complète de Prison sigillaire est validée sous OC04 : RP du
cercle de sang, valeurs actuelles/prochain rang, placement et destruction
affichés, avec l’intention de sceaux rouge sombre et effacement du segment
brisé. Pixels natifs, courbes et cas non présentés restent à revoir.
Rayon vorace est validé sous OC05-02 : RP, chiffres, poursuite automatique
pendant l’attente et dégâts reçus sans interruption, avec fil noir veiné
de rouge, pulsations vers l’agent et rupture à l’arrêt. Pixels natifs,
courbes et cas non présentés restent à revoir. OC06 Rupture dimensionnelle
est retirée sans remplacement sous OC06-REMOVE01 ; son identifiant reste
réservé à l’historique. OC07 Réanimation est ajoutée séparément après
validation explicite de sa fiche affichée, sans réemploi de l’identifiant OC06.

Prison sigillaire — comportement des PNJ validé sous OC04-IA01 : le PNJ
utilise une brèche existante si elle permet de rejoindre sa cible. S’il est
bloqué, il attaque un segment accessible pour ouvrir un passage, puis
reprend son déplacement. Un seul segment détruit crée une sortie ; le
reste du cercle demeure en place. La sélection précise du segment et les
cas particuliers restent à définir. Comportement à intégrer avec OC04.

Les portails d’exploration vers les cartes bonus restent conservés. Le
retrait d’OC06 concerne uniquement cette ancienne compétence.

OC07 Réanimation rejoint le catalogue après « Parfait, ca me va :) » :
nom, RP, PV maximum/dégâts à 100 % puis 105 %, durée onze tours, cinq alliés,
portée cinq cases, vingt énergie et recharge huit tours. Marque jusqu’à
la mort, statistiques et compétences d’origine conservées, boss exclus,
nouvelle réanimation remplaçant le plus ancien à la limite. Sceau de sang,
pulsation rouge sombre et tête de mort sont retenus en direction ; les
courbes et cas non montrés ainsi que les pixels natifs restent à revoir.
Fiche : [OC07-02](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-OC07-02.json).
Le total retenu est de 77 compétences dans 13 branches, dont six en Énergie occulte.

Les interfaces déjà retenues restent conservées : fenêtre « Familiers »
étendue depuis celle des drones, réanimés autonomes sans commandes de
combat, bouton commun « Détruire », tête de mort pour les réanimés et
point vert pour les drones. Le contenu de la mule tombe sur sa case ;
UX-SOL-01 permet à tous les objets au sol de coexister, avec dépôts successifs
sans déplacement et tooltip des noms à la souris ou au curseur du jeu.
Voir [la revue d’interface](REVUE_INTERFACE_EFFETS_AGENT.md).

## 10 — Transformations

Les cinq formes utilisent les dessins 61 à 65 existants. Chacune est une
transformation temporaire avec ses actions propres, pas un achat distinct
pour chaque action naturelle. Le joueur conserve les statistiques de son
équipement et ses passifs. Les effets de type de dégâts des armes actives
juste avant la transformation sont transmis ; les armes rangées ne sont
pas une source. Les actions de la forme et Décharge mutagène restent
utilisables, à l’exclusion des autres actifs appris. M02 conserve son rôle
de prolongation. Ces règles sont demandées sous TRANS-LOCK01 / TRANS-EQ01 ;
leurs cas techniques non présentés restent à revoir.

Chaque forme doit proposer un intérêt propre assez fort pour justifier la
restriction d’actifs : prédation mobile et saignement pour M01, frappe
lourde et zone pour TR61, résistance et contre-attaque pour TR63, magie
organique pour TR64, allonge et mobilité pour TR65. Ces rôles conservés ne
valident pas de nouvelles valeurs chiffrées pour les quatre autres formes.

M01 Mue prédatrice : [M01-03](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-M01-03.json)
propose Griffe à 45 puis 48 dégâts, saignement à 50 puis 53 % de chance,
10 % des dégâts réels du coup initial par tour pendant trois tours,
durée neuf tours et Bond de deux cases. Le RP « Vous vous métamorphosez en une bête sauvage, animée par un instinct de prédateur. »
est explicitement validé sous M01-RP01. Les nouveaux chiffres, les
conditions et l’animation restent à valider ; l’identité M01 et le
dessin Prédateur 62 sont conservés. L’ancien passage non validé autorisant
Frappe de rupture avec Griffe reste écarté.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| M01 | Mue prédatrice — Prédateur 62 | A | Forme de contact mobile avec Griffe renforcée, chance de saignement et Bond ; valeurs proposées sous M01-02. | Dessin 62, griffures basses et bond en silhouette compacte. |
| TR61 | Colosse biomécanique — 61 | A | Forme de frappe lourde avec onde de choc et poussées. | Dessin 61, lourdes secousses au sol autour des appuis. |
| TR63 | Cuirassé — 63 | A | Forme qui encaisse et prépare une contre-attaque avec les coups reçus. | Dessin 63, plaques en charge puis ouverture lumineuse à la riposte. |
| TR64 | Possédé — 64 | A | Forme orientée magie, lançant des attaques occultes depuis ses organes transformés. | Dessin 64, motifs organiques pulsants et émissions depuis le corps. |
| TR65 | Abysse — 65 | A | Forme à allonge naturelle et déplacements courts entre ombres visibles. | Dessin 65, membres étirés et ombre résiduelle à l'ancienne position. |
| M03 | Puissance métamorphe | P | Augmente les dégâts sous forme ; +10 % au premier rang confirmé. | Réseau interne de la forme qui s'allume lors d'une attaque renforcée. |

## 11 — Pouvoirs hybrides

Ces compétences combinent des systèmes déjà appris ; elles ne donnent pas
gratuitement l'invocation ou la transformation nécessaire à leur usage.
Intelligence et Volonté y ont des rôles à détailler.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| M02 | Décharge mutagène | A | Frappe une cible ; sous forme, un impact réussi peut prolonger cette forme, avec une limite totale. | Décharge organique puis un segment de durée rallumé sur la forme. |
| HY02 | Fusion défensive | A | Sacrifie un drone de combat contrôlé pour créer une protection absorbante sur l'agent. | Drone se décomposant en segments qui assemblent une coque sur le joueur. |
| HY03 | Relais de sort | A | Lance un sort compatible depuis la position d'un drone contrôlé. | Fil de commande jusqu'au drone, émission du sort depuis son capteur. |
| HY04 | Échange de position | A | Échange les positions de l'agent et d'un drone contrôlé, si les deux destinations sont légales. | Deux silhouettes se décomposant puis se reformant sur les cases échangées. |
| HY05 | Siphon technique | A | Dissipe un renforcement ennemi compatible et récupère une part d'énergie. | Effet arraché à la cible en fragments qui rejoignent les implants. |
| HY06 | Lien de domination | A | Prend temporairement le contrôle d'un ennemi, soumis à résistance et restrictions à définir. | Collier de glyphes et ligne de contrôle vers l'agent. |

L'accord retient pour M02 les dégâts directs et la prolongation limitée
d'une forme sur un impact réussi. La fiche du deuxième lot est mise à jour
sur ce principe ; le gain de tours, sa limite et sa progression restent
à chiffrer. HY02 exclurait les drones de transport pour préserver leur vrai
chargement. Les passages, échanges et relais doivent respecter les zones,
observations et destinations autorisées.

## 12 — Buffs de mêlée

Ces pouvoirs renforcent le contact. Les passifs permanents peuvent être
conditionnels ; les buffs actifs ont une durée à définir et coûtent de
l'énergie. Ils restent disponibles aux autres classes.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| BM01 | Lame incendiaire | A | Ajoute temporairement du feu aux attaques de mêlée compatibles. | Flammes attachées au tranchant ou aux griffes, sans remplacer l'arme. |
| BM02 | Faim occulte | A | Les coups de mêlée réussis rendent une part de la vie volée. | Gouttes sombres remontant de la victime vers le bras d'attaque. |
| BM03 | Tempo offensif | P | Des coups de mêlée consécutifs gagnent un bonus limité, perdu à la rupture de la série. | Repères successifs près de la poignée, traînée plus marquée à chaque palier. |
| BM04 | Ancrage | A | Empêche temporairement les déplacements forcés de l'agent. | Griffes de lumière plantées sous les appuis. |
| BM05 | Sceau de vindicte | A | Glyphe : renvoie une part des dégâts magiques directs subis à leur auteur ; poison et dégâts indirects exclus. | Retour visible du sort ou du projectile magique vers son lanceur, avec l'apparence de l'attaque d'origine ; rendu natif à présenter. |
| BM06 | Protection offensive | P | Un coup de mêlée réussi produit une petite protection temporaire, avec cumul limité. | Demi-arc protecteur assemblé devant le corps après la frappe. |

## 13 — Buffs à distance

Le passif initial du Traqueur reste permanent et indépendant des
déplacements. Les autres bonus ci-dessous ne changent pas cet accord.

| ID | Nom proposé | Type | Effet proposé | Idée visuelle |
| --- | --- | --- | --- | --- |
| T01 | Marquage balistique | A | Marquage instantané ; précision × 1,15 au premier rang confirmé. | Viseur bref posé sur la cible, quatre traits qui se ferment. |
| T03 | Maîtrise des attaques à distance | P | +10 % de dégâts des armes et compétences à distance au premier rang confirmé. | Trait intérieur plus vif dans le projectile ou l'émission renforcée. |
| BD03 | Salve miroir | A | Le prochain tir compatible produit un tir secondaire vers un autre ennemi visible. | Écho du départ de tir depuis une petite silhouette d'arme projetée. |
| BD04 | Visée stable | P | Rester immobile développe un bonus limité de précision, perdu au déplacement. | Réticule se resserrant progressivement autour de la visée. |
| BD05 | Écran tactique | A | Prépare la déviation d'un prochain projectile hostile compatible. | Plan lumineux incliné et changement net de sa trajectoire à l'impact. |
| BD06 | Récupération balistique | P | Une élimination à distance récupère une petite quantité d'énergie, avec cadence limitée. | Étincelle remontant vers la jauge d'énergie depuis la cible éliminée. |

## Ce que cette liste laisse à décider

**Ajout du 7 octobre — contrôles :** tous les effets de contrôle suivent
[CTRL01](REVUE_AFFAIBLISSEMENTS_AGENT_LOT_06.md#ctrl01-boss-et-élites-face-aux-contrôles).
Les boss sont immunisés aux autres contrôles ; le Gel suit l'exception CTRL-EL01.
La chance de contrôle contre une élite est
multipliée par 0,75. Pour une attaque mêlant dégâts et contrôle, les dégâts
restent applicables selon leurs règles habituelles. Le classement précis
des effets et les rendus seront revus avec chaque fiche.

Le catalogue est retenu. Les paliers de déblocage sont supprimés : ses
lignes servent uniquement à organiser les compétences, toutes apprenables
avec les points nécessaires, sans exclusivité de classe ou de choix.

Selon AR-HL01, l'arbre de tir ordinaire Attaques à distance ne s'applique
pas aux armes lourdes. Un arbre Armes lourdes pour les armes lourdes à
distance existantes et futures sera préparé après la validation de toutes
les compétences actuelles. Son contenu reste à proposer ; les 77 entrées
du catalogue actuel sont conservées. Les détails sont dans la
[revue des arbres](REVUE_ARBRES_COMPETENCES_AGENT.md).

Les chiffres, coûts, durées, résistances, cumuls, déclenchements par tour,
compatibilités d'armes et d'effets seront présentés après cette sélection.
Chaque rang doit annoncer ce qu'il améliore ; les attributs conservent
leur intérêt. Les chaînes, ripostes, renvois, prolongations et absorptions
nécessitent des limites explicites, sans exécution gratuite depuis un menu.

Les formes visibles, portées et aperçus devront correspondre aux vrais
effets, respecter les obstacles et ne révéler aucune cible cachée. Les
nouveaux rendus suivent la [direction artistique](DIRECTION_ARTISTIQUE_TILES.md).
Le futur [panneau des buffs et debuffs](REVUE_INTERFACE_EFFETS_AGENT.md)
pourra afficher les états actifs du personnage, avec leur nom, leur effet
et leur durée restante au survol. Sa réalisation est prévue plus tard.

## Références utilisées pour l'inspiration

Les exemples de coups tournoyants et de charges ont été consultés dans
les [attaques du Barbare](https://classic.battle.net/diablo2exp/skills/barbarian-combatskills.shtml)
et les [techniques du Paladin](https://classic.battle.net/diablo2exp/skills/paladin-combat.shtml).
La logique de riposte au contact s'appuie notamment sur ses
[auras offensives](https://classic.battle.net/diablo2exp/skills/paladin-offense.shtml).

Les familles de projectiles, barrages, zones différées et sorts à rebonds
ont été regardées dans les pages officielles
[Feu](https://classic.battle.net/diablo2exp/skills/sorceress-fire.shtml) et
[Froid](https://classic.battle.net/diablo2exp/skills/sorceress-cold.shtml).
Les [transformations du Druide](https://classic.battle.net/diablo2exp/skills/druid-shapeshifting.shtml)
ont servi de référence de rôles distincts sous forme. Les propositions
ci-dessus adaptent ces idées au tour par tour, aux drones, aux implants
et à la magie de cet univers ; elles ne reprennent ni leur lore ni leurs chiffres.

## DENS01 Orientation des combats

Le joueur souhaite des combats denses avec de nombreux ennemis variés et
leurs statistiques, tout en conservant un défi mortel. Les quantités, rôles,
apparitions et réglages seront revus séparément.
