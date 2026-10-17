# Interface et fenêtre des familiers — revue corrigée

16 octobre 2026. Revue courante HUD-06 et UX-PETS-05. Le joueur retient les deux
habillages, Métal sombre par défaut et Terminal PC au choix dans les options.

## Correction du périmètre

Le joueur précise : « il manque enormement de choses non ? […] d'alleger
visuellement moi ». La demande porte sur le dessin de l'interface. Les premières
propositions HUD-02A et HUD-02B réduisaient trop le contenu visible ; elles ne sont
pas retenues. Un affichage de variante dans l'aperçu ne remplace pas ce retour.

Le joueur ajoute ensuite deux précisions :

- La fenêtre des drones et réanimés peut être ouverte par un bouton avec icône.
- Le bas de l'écran mérite aussi d'être allégé et clarifié.

La révision conserve les informations et fonctions actuelles, en travaillant
les cadres, accents, titres, espacements et regroupements. Elle ne déplace pas
les défenses ni la progression dans une fenêtre cachée.

## Preuves et limites de l'audit

L'audit initial a examiné l'exploration et le laboratoire avec trois drones,
en utilisant l'exécutable debug existant et des parties de diagnostic isolées.
Les captures conservent l'ancien monde du prototype :

- artifacts/interface-review/current-1280/cold-start.png, 1280 × 720.
- artifacts/interface-review/current-1920/cold-start.png, 1920 × 1080.
- artifacts/interface-review/current-drones/drone-5.png, 1280 × 800.
- current-drones/drone-checks.json : trois drones, capacité quatre, visées sans
  coût et munitions du joueur inchangées.

Le code draw_companion_bar montre les PV et la batterie du premier compagnon
et exige un profil de drone. Il ne fournit pas la liste commune déjà prévue.
Les sept modules et les dix boutons en pied d'écran donnent beaucoup de poids
aux cadres. Ces constats justifient le travail visuel et la fenêtre des familiers ;
ils ne justifient pas de retirer des informations.

Les recommandations initiales visant à cacher défenses, progression ou accès
sont remplacées. Aucun audit complet de tous les écrans d'inventaire,
compétences, commerce, création et réglages n'est annoncé.

## HUD-03 — Contenu conservé

La maquette courante conserve les groupes du HUD :

- État du personnage : PV, énergie libre et réservée, munitions et compagnons.
- Arme active : nom et coût en munitions.
- Défenses : armure, esquive, stabilité, défense numérique et toutes les
  résistances affichées, y compris zéro et non applicable.
- Progression : niveau, expérience et points disponibles, avec accès K.
- Capteurs : carte connue, comptes d'hostiles, neutres et objets, lecture tactique.
- Inspection : portrait, nom, distance, PV, armure, résistances connues et effets
  dans l'exemple. L'affichage réel reste soumis aux informations autorisées.
- Objectif et accès correspondant.
- Effets actifs du personnage, avec détails et durées au survol dans HUD-04.
- Coordonnées, secteur et messages récents. Le compteur global de tours est
  retiré sur demande explicite du joueur dans HUD-04.

Les alertes, l'itinéraire lorsqu'il est actif, les variantes de compétences,
les états d'interaction et les avertissements de visée restent à préserver dans
la réalisation native. Leur absence de l'image fixe du laboratoire ne permet
pas de les supprimer. Les réattributions de touches conservent leur fonctionnement.

La modification proposée est graphique : surfaces plus calmes, titres discrets,
séparations fines et accents réservés aux ressources ou aux états utiles. Les
numéros de modules et coins décoratifs ne portent aucune donnée de jeu.
Les grands blocs restent à leur place générale.

## HUD-03-BAS — Pied d'écran clarifié

Tous les dix accès du pied d'écran sont présents :

- Actions de jeu : Attendre, Interagir, Attaquer et Techniques.
- Fenêtres et outils : Quêtes, Inventaire, Examiner, Historique, Aide et Menu.

Les six emplacements rapides de compétences et le changement d'arme restent
distincts de ces commandes générales. L'accès aux compétences et aux points
reste dans la progression. Les messages occupent leur propre zone sous les
commandes, avec accès permanent à l'historique.

La maquette remplace la rangée de dix grands cadres équivalents par deux
groupes lisibles, avec raccourcis visibles et états de survol/focus. Aucun accès
n'est regroupé derrière un nouveau sous-menu. Cette présentation ne modifie
ni l'attente, ni l'exécution d'une attaque ou compétence, ni le coût des actions.

## HUD-04 — Corrections demandées et habillage proposé

Le joueur préfère HUD-03 et demande les ajustements suivants. Ces demandes
précises remplacent les points correspondants de HUD-03 ; elles ne remettent
pas en cause le maintien des autres informations et commandes.

**HUD-04-CENTRE — Position demandée.** Les six emplacements de compétences et
le bouton de changement d'arme sont centrés sous le plateau, dans la largeur
de la fenêtre centrale. Les panneaux latéraux ne déterminent pas ce centrage.
Les quatre actions générales, les six accès aux fenêtres et les messages
gardent leurs groupes distincts en dessous.
Le joueur rappelle expressément de conserver le changement d'arme : son bouton
est placé juste à gauche des six emplacements, avec l'infobulle « Changer d'arme ».
Ce bouton ne sert ni à recharger ni à changer de série de compétences.

**HUD-04-LIEU — Informations demandées.** Le nombre de tours écoulés disparaît.
Les coordonnées sont placées immédiatement après le nom du lieu dans l'en-tête,
selon la préférence exprimée par le joueur. Les durées des effets, recharges
et durées des réanimés restent disponibles ; elles ne sont pas des compteurs
globaux de tours. L'ancienne ligne sous le plateau n'est plus nécessaire.

**HUD-04-EFFETS — Présentation demandée.** Les effets actifs sont représentés
par leurs icônes seules, sans nom ni durée en texte permanent. Une colonne
verticale près du bord gauche du plateau est proposée. Le nom et la durée
apparaissent au survol ou au focus clavier ; les informations d'effet prévues
par la revue existante restent requises dans l'infobulle native.

Le joueur demande un clignotement lorsqu'il reste exactement un tour. Le rendu
proposé est une pulsation lente de 1,3 seconde, sans disparition complète, avec
un liseré clair. Ces paramètres de dessin restent à examiner. La préférence de
réduction des animations conserve un liseré fixe. Dans la réalisation, l'état
est calculé depuis la durée réelle du moteur, actualisé après chaque action :
aucun décompte en secondes, aucune durée fictive pour un effet permanent,
aucune avance de simulation liée au survol ou à une fenêtre ouverte.

L'exemple montre Brûlure à trois tours et Barrière d'épines à un tour pour
comparer un effet stable et un effet qui clignote. Ces valeurs de démonstration
ne changent ni les durées ni les règles de ces effets dans le jeu.

**HUD-04-MATIERE — Rendu proposé.** Le joueur demande si des textures peuvent
donner davantage l'impression d'un jeu. L'aperçu propose du métal sombre à grain
fin, quelques traces d'usure, des bords légèrement biseautés et des séparations
gravées. La texture reste discrète derrière les textes ; les ressources et
alertes gardent les accents les plus visibles. Aucun nouveau portrait, monstre,
objet porté ou décor n'est créé par ce travail.

Le grain de revue est produit localement de manière déterministe. Le prototype
ne charge aucune nouvelle ressource du jeu. La matière, son intensité, les
icônes et le rythme de clignotement constituent une proposition visuelle à
examiner ; le principe d'un habillage texturé n'en vaut pas validation détaillée.

## HUD-04B — Agencement retenu et version sauvegardée

Le joueur dit aimer l'agencement et demande de le garder exactement, en retirant
le skill-dock : la barre de compétences et son bouton de changement d'arme
suffisent. Le fond pleine largeur et son conteneur sont retirés ; la barre
reste seule, centrée sous le plateau. Les panneaux, modules, commandes,
informations, effets verticaux et accès aux familiers gardent leur place.

Le joueur demande de sauvegarder cette version avant d'essayer une autre
ambiance. La référence métallique corrigée est conservée séparément :

- interface-metal-hud-04b.source.html : source de revue avec image incorporée.
- interface-metal-hud-04b.html : version consultable hors de l'aperçu courant.
- INTERFACE_SAUVEGARDE_04B.json : empreintes et périmètre conservé.

Le contraste de séparation de 15 % choisi dans l'aperçu est repris comme valeur
de départ. L'ouverture de Familiers pendant l'examen ne remplace pas la décision
d'une fenêtre fermée au lancement et accessible par bouton. La sauvegarde est
un point de retour de conception, sans commit ni modification du client du jeu.
L'accord sur l'agencement ne tranche pas le choix final de matière.

## HUD-05-TERMINAL — Essai vieux terminal PC

Le joueur demande un second habillage pour comparer avant de choisir. L'essai
conserve le même agencement que HUD-04B : il n'ajoute ni bandeau sous les
compétences, ni module, ni sous-menu, ni compteur de tours.

Le rendu proposé emploie des caractères monospace, des teintes vert phosphore,
des titres entre crochets, des jauges segmentées, des bordures droites et une
trame horizontale fixe et discrète. Il remplace les reliefs métalliques du
premier habillage. La trame peut être coupée dans les commandes de revue ;
aucune option de jeu n'est ajoutée ici. Le clignotement à un tour garde le
principe déjà demandé ; aucune instabilité ou oscillation d'écran n'est ajoutée.

Le terrain, la carte, les personnages et le portrait partagent la même image
source et les mêmes recadrages. Ils ne reçoivent pas de filtre monochrome.
Les données, noms et interactions de démonstration restent identiques.

L'aperçu courant permet de passer entre « Terminal PC » et « Métal sombre ».
Les interactions restent locales à chaque variante. Le style terminal est une
proposition de rendu, sans intégration et sans validation présumée. Les textes
des effets et leurs règles n'ont pas changé.

**Décision suivante — HUD-06 :** le joueur retient les deux rendus avec choix
dans les options et le premier habillage comme défaut. L'hésitation consignée
dans HUD-04B et HUD-05-TERMINAL est donc résolue. Le fichier métallique sauvegardé
et son manifeste restent le point de retour historique ; ils ne sont pas réécrits.

## HUD-06 — Deux habillages et vraies icônes

Le choix demandé est Métal sombre au premier lancement, puis Métal sombre ou
Terminal PC selon la préférence enregistrée. Le style modifie l'habillage,
sans changer l'agencement, les icônes de contenu, le terrain, les acteurs ni
les règles. Le joueur demande explicitement les vrais dessins de compétences
et de statuts, à la place des symboles génériques de l'aperçu.

La maquette courante montre le parcours Menu, Options, Affichage, puis le choix
« Habillage de l’interface », avec « Métal sombre » et « Terminal PC ». Le
nouveau libellé « Habillage de l’interface » est proposé ici avant intégration.
Appliquer retient le choix dans l'état de cet aperçu ; Annuler restaure le style
précédent. Le menu ne constitue pas une reproduction de tous les réglages du jeu.

L'audit du raccordement retrouve les préférences natives dans src/graphics.rs,
leurs lignes dans src/pause_menu.rs et leur traitement dans src/ascii_app.rs.
La couleur d'accent existante est un réglage distinct : un futur champ de style
doit conserver les autres réglages et charger les anciens fichiers avec le
défaut Métal sombre. Le prototype ne lit ni n'écrit le fichier graphics.json.
Le raccordement au client natif reste à réaliser après cette revue des ajouts.

Les dessins incorporés proviennent, sans retouche, de assets/skills/agent/ et
du catalogue chargé par agent_skill_window::art. La provenance et les empreintes
sont dans INTERFACE_ICONES_06.json :

- D58, D59 et D66 pour les trois compétences de drones.
- O02 pour Interférence, selon l'association de statut déjà utilisée par le jeu.
- SU03 pour Barrière d’épine, en réutilisant le dessin compact de cette compétence.
- DP02, O03 et AF02 pour les exemples d'effets sur les familiers.

Les images gardent leurs couleurs dans les deux habillages. Les sources sont
64 × 64 ; leur affichage compact suit la place de la barre ou de la liste.
Le statut d'exemple Interférence remplace Brûlure dans la scène de revue afin
de montrer un dessin de statut déjà lié dans le client. Cela ne réduit pas
la liste des effets à prendre en charge et ne modifie aucune compétence.

## UX-PETS-04 — Ouverture par bouton et icône

Le principe d'ouverture par bouton avec icône est demandé par le joueur.
Dans HUD-03 à HUD-05, le bouton « Familiers » se trouve près du compteur de
compagnons. Cet emplacement est révisé par UX-PETS-05 ci-dessous.
La fenêtre est fermée au chargement. Le bouton l'ouvre et peut la refermer ;
elle possède aussi une fermeture, un repli et une poignée de déplacement.
La position et l'icône précises restent une proposition de rendu.

UX-PETS-01 et UX-PETS-02 gardent leur organisation acceptée :

- Fenêtre commune, déplaçable, repliable, défilante ; une ligne par familier.
- Nom, identité et PV actuels/maximaux de chacun.
- Drones : comportement, liaison et batterie dans le détail.
- Réanimés temporaires : durée restante et autonomie ; pas de sélecteur de
  comportement de combat ni de faux temps d'expiration pour les drones.
- Ordres des drones, individuels ou communs. L'accès F3 aux comportements est
  conservé dans la proposition ; il n'est pas réaffecté à l'ouverture de la fenêtre.
- Capacités séparées : la révision Armée de drones remplace l'ancien plafond
  fixe de cinq drones ; aucun plafond commun drones/réanimés n'est ajouté.
- « Détruire » pour les deux types, avec confirmation proposée.

Les règles de réanimation, ordres, destruction, coût, temps, absence de soin
au rappel et chargement de la Mule ne sont pas redéfinies. Leur intégration
suivra les décisions déjà acceptées et les conditions réellement disponibles.
Ouvrir ou fermer une fenêtre ne doit pas faire avancer la simulation.

## UX-PETS-05 — Accès près de Techniques, fenêtre agrandie et effets

Le joueur propose de placer le bouton Familiers à côté de Techniques. La revue
montre ce regroupement dans l'ordre Attendre, Interagir, Attaquer, Techniques,
Familiers. Le compteur des compagnons reste dans le panneau du personnage,
avec les deux capacités séparées et l'accès F3 aux comportements. Le bouton
d'ouverture n'est pas dupliqué. Aucun raccourci nouveau n'est inventé.

Le joueur demande ensuite une fenêtre plus grande pour la lisibilité et pour
y afficher les buffs, passifs ou non, debuffs et timers des familiers. La
proposition passe de 280 à 400 pixels logiques de large, avec une liste visible
de 330 pixels de haut, des identités de 40 pixels, des PV chiffrés et leur jauge.
Les dimensions et la mise en page sont montrées avant intégration.

Sous chaque familier, une rangée d'icônes montre les effets qui le concernent.
Au survol ou au focus : nom, effet actuel et durée réelle lorsqu'elle existe.
Un bonus passif applicable est inclus sans lui inventer une expiration.
Les compétences passives apprises qui n'affectent pas ce familier ne constituent
pas des effets à lui attribuer. Un buff activé et une condition temporaire
gardent leur propre minuterie. La durée d'existence d'un réanimé reste distincte
de celle de ses effets. Les icônes à un tour restant suivent le clignotement
demandé pour les statuts du personnage ; les permanents ne clignotent pas.
Un effet temporaire déclenché par un passif conserve aussi sa durée réelle :
passif et permanent ne sont pas synonymes.

La scène de démonstration montre Blindage amélioré (DP02) et Algorithmes de
combat améliorés (O03) sur les drones, un affaiblissement Interférence à un tour
sur l'Artilleur et Faille d'armure à deux tours sur un réanimé. DP02 et O03 sont
déjà des bonus des drones dans le moteur. Ces exemples n'ajoutent ni statut,
ni nouvelle cible autorisée, ni règle de propagation : la version native devra
lire uniquement les effets et bonus réellement applicables à chaque acteur.

Le déplacement, le repli, le défilement et les ordres conservés dans UX-PETS-04
restent présents. La liste longue ne réduit pas les lignes jusqu'à les rendre
illisibles. Les détails et ordres du familier sélectionné restent sous la liste.

## Données et interactions de démonstration

Le terrain, la carte et le portrait proviennent du dessin déjà présent dans
une capture du laboratoire. Le recadrage retire les anciens coins de cadre.
Les pixels du terrain ne sont pas agrandis au-delà de leur taille d'origine ;
ce recadrage ne change ni le champ de vision ni les règles du jeu.

Les cinq familiers, les valeurs d'inspection, les durées, PV et effets affichés
illustrent les champs. Ils ne constituent pas une sauvegarde ni une proposition
d'équilibrage. HUD-06 réutilise les vrais dessins de compétences et d'effets.
Les symboles d'outils de fenêtre et d'identité des réanimés restent des symboles
de revue ; aucun nouveau dessin de personnage n'est créé.

L'ouverture, la fermeture, le repli, le déplacement, la sélection et les modes
de drones peuvent être examinés dans l'aperçu. Les autres boutons montrent
leur place et leur fonction ; ils ne reproduisent pas les écrans complets ni
ne commandent le moteur. Les tours et durées n'avancent pas. La suppression
après confirmation ne retire qu'une entrée fictive de la liste.

Les nouveaux libellés se limitent à la présentation de la fenêtre et des groupes :
« Familiers », « Ouvrir/Fermer la fenêtre Familiers », « Fenêtres », « Replier »,
« Déplier », « Ordre des drones », « Ordres individuels », « Autonome »,
« Batterie », « Détruire [nom] ? » et « Annuler ». Les intitulés de statistiques,
compétences, effets et commandes existantes sont conservés.
HUD-06 propose en plus « Habillage de l’interface » ; les deux noms de styles
et les commandes Menu, Options, Affichage, Appliquer et Annuler sont montrés
dans le parcours d'options de la maquette.

## Résolutions, revue et réalisation

Priorités : Full HD 1920 × 1080, QHD 2560 × 1440 et 4K 3840 × 2160, avec une
échelle confortable ; 1280 × 720 secondaire. Aucun chantier mobile ou tablette.
L'aperçu adapte son échelle en grand format ; il ne modifie aucun réglage du jeu.

Les deux habillages et le défaut Métal sombre sont retenus par le joueur.
La place du bouton Familiers, la fenêtre élargie avec effets et le nouveau
libellé du réglage sont présentés dans HUD-06 / UX-PETS-05 avant intégration,
conformément à AGENTS.md. Aucun nouvel effet mécanique n'est proposé.

Cette passe touche seulement l'aperçu et la documentation. La réalisation native,
les captures du nouveau client dans les résolutions cibles, la validation des
listes longues, des interactions en combat, de la visée, des refus sans coût
et des reprises de sauvegarde restent à faire après revue du rendu retenu.

Le registre INTERFACE_CONTROLES.json décrit l'ancienne revue HUD-02 ; le contrôle
de HUD-03 est consigné dans INTERFACE_CONTROLES_03.json et celui de HUD-04 dans
INTERFACE_CONTROLES_04.json. La comparaison courante est consignée dans
INTERFACE_CONTROLES_05.json ; les ajouts courants dans INTERFACE_CONTROLES_06.json.
