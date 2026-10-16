# Audit UX et visuel — Project RL

28 septembre 2026 · état local du projet · rapport de recommandations, sans modification du jeu.

**Le principal chantier est la hiérarchie de l'interface : rendre la décision du moment évidente, rapprocher ses informations et ses commandes, puis harmoniser leur présentation.** L'identité sombre et turquoise fonctionne. Les écrans contiennent déjà beaucoup d'informations utiles, mais leur disposition demande trop souvent au joueur de chercher l'action, de mémoriser une règle ou de découvrir un comportement après l'avoir essayé.

La création de personnage illustre particulièrement ce problème. L'action principale manque de présence ; les réglages semblent disponibles même lorsqu'ils ne le sont pas ; l'écran consacre beaucoup d'espace au vide tout en éloignant les boutons de leur contenu. Ailleurs, certaines explications sont tronquées alors que le panneau reste largement vide.

**Méthode et portée.** Lecture du rendu, des commandes et des parcours dans le code actuel, compilation locale réussie, inspection visuelle de 40 captures produites pour cet audit et exécution de 11 tests existants ciblant la navigation. Les captures couvrent l'accueil, la création, le jeu, les combats et alertes, l'inventaire, le personnage, les compétences, les quêtes, les dialogues, les services, le dossier, l'aide, les compagnons, les options et la reprise. La fin de partie et certaines variantes de sélection de composants ont été examinées dans le code.

Il s'agit d'un audit heuristique et technique, sans test d'apprentissage auprès de joueurs novices ni mesure de confort sur une longue session. Les captures ne suffisent pas à valider le rythme de toutes les animations. Les diagnostics utilisent parfois des personnages, objets ou quêtes de test : leurs valeurs et leur contenu ne sont pas assimilés à la campagne normale.

Dimensions réellement obtenues : 31 captures en 1280 × 800, 6 en 960 × 540, 1 accueil en 1920 × 1080, 2 fenêtres en 1920 × 1061 pour les diagnostics jeu/inventaire demandés en 1080 avec une échelle configurée à 125 %. L'environnement a limité la hauteur utile de ces deux fenêtres. Les dimensions sont consignées dans [le manifeste des captures](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/captures.json).

**Les éléments déjà réussis méritent d'être conservés.**

- L'accueil est sobre ; le motif terminal lui donne une identité et les actions restent dégagées.
- Les commandes sont reconfigurables, avec prise en charge des dispositions de clavier et de la répétition de navigation.
- Échap ferme les principales surcouches avant d'ouvrir la pause.
- Inventaire, personnage et compétences permettent déjà de naviguer sans dépenser un tour.
- L'inventaire distingue équipement porté et sac, conserve l'objet sélectionné lors de certains changements et possède des filtres.
- La visée de zone montre une empreinte et distingue validation et annulation.
- Les capteurs distinguent perception et mémoire ; les informations d'une cible sont enrichies après analyse.
- Les soins affichent un résultat et un prix concrets ; les échecs de reprise indiquent que la sauvegarde est conservée.
- Taille d'interface, contraste renforcé et animations réduites existent déjà.

**Trois anomalies d'interaction doivent passer avant la refonte graphique.** Elles sont établies par lecture du chemin d'entrée ; elles n'ont pas été reproduites manuellement dans une partie utilisateur.

| Priorité | Constat et conséquence | Correction recommandée | Preuve actuelle |
| --- | --- | --- | --- |
| Critique | Depuis le jeu dégagé, le raccourci « Nouvelle partie », R par défaut, reconstruit directement une partie, avant même de vérifier que la partie courante est terminée. Un appui peut donc remplacer une partie active sans confirmation. | Réserver la relance directe à une partie terminée ; faire passer toute relance d'une partie active par la confirmation explicite. | [Gestion de Restart](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:5009) |
| Élevée | Dans l'inventaire, le choix d'emplacement d'arme provient de `hovered_action` sans condition de clic. Survoler un bouton d'équipement peut équiper l'arme sélectionnée ou changer l'arme active. | Le survol ne doit que présenter l'état ; l'équipement exige un clic ou une touche dédiée. | [Choix de l'emplacement](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:16023) |
| Élevée | Le commerce annonce « Flèches ou molette », mais sa navigation traite les touches et les clics, sans utiliser `wheel_y`. La molette capturée n'est pas convertie en flèche par les contrôles. | Implémenter la molette dans la liste marchande, avec le même comportement que les autres listes. | [Entrées du commerce](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:5794), [aide affichée](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:6600), [capture des entrées](C:/Users/User/Desktop/project-RL/src/controls.rs:517) |

Les tests existants de navigation passent ; ils ne couvrent pas ces trois cas précis. Le test de commerce contenant « scroll » dans son nom parcourt la liste au clavier puis clique dans la partie défilée : il ne prouve pas le fonctionnement de la molette.

**1. L'accueil constitue une bonne base, mais la priorité des actions peut être renforcée.**

Le menu principal est plus cohérent que plusieurs écrans internes. Il possède un accent turquoise clair, des boutons lisibles et un motif discret. Je conserverais cette composition et le dessin terminal.

Deux points sont à améliorer : « Reprendre la partie (indisponible) » occupe la première place quand aucune partie n'existe, et « Laboratoire de test » reçoit presque autant de poids que les actions principales. Cela charge l'entrée avec une option inutilisable et un outil spécialisé.

Je recommande une priorité persistante pour « Nouvelle partie » ou « Reprendre », indépendante du simple survol, puis une séparation discrète des actions secondaires. Le laboratoire peut rester accessible, avec un placement qui indique son rôle expérimental. Aucun slogan ou texte d'ambiance supplémentaire n'est nécessaire.

Lorsqu'une reprise existe, un petit aperçu du personnage et du lieu, associé à cette action, aiderait à retrouver sa partie. Ce serait une information contextuelle, à limiter pour préserver la sobriété de l'accueil.

Preuves : [accueil 1280](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/main/cold-start.png), [accueil 960](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/main-960/cold-start.png), [accueil 1080](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/main-1080/cold-start.png), [reprise disponible](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/resume/cold-start.png).

**2. La création de personnage est la première refonte de parcours à mener.**

La séparation « protocole puis attributs » est compréhensible. Les rôles sont décrits et un profil recommandé évite de partir sans repères. En revanche :

- « Restauration de l'instance » et « Protocole » mettent la fiction avant l'explication de la tâche. Un nouveau joueur doit comprendre qu'il choisit son profil de départ.
- Le choix actif et plusieurs titres sont jaunes, tandis que « Configurer les attributs » possède presque le même aspect que « Retour » au repos. La sélection actuelle domine visuellement l'action suivante.
- Les boutons sont ancrés tout en bas, loin des choix concentrés dans la moitié supérieure. Le problème tient surtout à la distance et à leur faible hiérarchie, pas au seul fait de placer la validation à droite.
- L'étape 2 arrive avec 28/28 points déjà dépensés. Tous les « + » paraissent utilisables ; augmenter une valeur exige pourtant de retirer d'abord un point ailleurs. Le joueur découvre cette contrainte après un refus.
- « Répartition personnalisée » s'affiche même pour le profil recommandé intact. « Rétablir le profil recommandé » occupe presque toute la colonne, bien davantage que « Commencer la partie ».
- Revenir à l'étape protocole puis continuer réapplique le profil recommandé, même sans changer de protocole : une répartition personnalisée peut être perdue silencieusement.
- Un démarrage avec une répartition incomplète est visuellement désactivé, mais le chemin d'entrée peut encore le demander et produire un message générique d'échec.

Je recommande de proposer, après le choix du protocole, **« Commencer avec ce profil »** comme action principale et **« Personnaliser les attributs »** comme alternative. La personnalisation reste disponible sans être un passage obligatoire pour le débutant.

Dans la personnalisation : rapprocher le compteur des points des cinq attributs ; expliquer d'emblée la redistribution ; distinguer les boutons aux limites ; afficher la raison d'une indisponibilité ; préserver les changements lors d'un simple aller-retour ; rendre la remise à zéro secondaire. Une prévisualisation des conséquences calculées par le moteur serait plus utile que les seuls écarts « +1 / −1 » par rapport au conseil initial.

Exemple de présentation de la décision : « Puissance 7 → 8 », puis les conséquences réellement calculables, sans inventer d'effet ou de valeur. Pour chaque protocole, privilégier une phrase sur sa manière de jouer et un petit aperçu de son équipement. Un pictogramme propre à chaque profil faciliterait aussi la reconnaissance.

Preuves : [étape protocole](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/character-creation/cold-start.png), [étape attributs](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/character-attributes/cold-start.png), [disposition](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:1074), [transition et répartition](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:19065).

**3. Les premières minutes donnent une destination, mais demandent encore beaucoup d'apprentissage implicite.**

L'objectif « Rejoindre le secteur habité » et l'itinéraire sont utiles. Le joueur reçoit toutefois immédiatement un plateau symbolique, plusieurs réserves, des défenses, des points de compétence et des raccourcis. La barre basse ne rappelle pas le déplacement ; « Aide » ouvre surtout une légende des symboles.

Une initiation contextuelle légère serait préférable : montrer le déplacement au premier départ, l'interaction près de la première console, puis le ciblage au premier contact. Chaque explication devrait disparaître après réussite et rester consultable. Le prochain geste utile doit être proche de l'objet concerné, sans transformer l'écran en tutoriel permanent.

Les deux points de compétence disponibles dès le départ sont annoncés, mais cette annonce ne constitue pas un accès évident à leur dépense. Un bouton explicite « 2 points à dépenser » résoudrait cette découverte.

Preuve : [premier écran de jeu](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/game/cold-start.png), [objectif introductif](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:9166).

**4. Le HUD doit mieux répartir l'attention entre survie, action et référence.**

À 1280 × 800, les colonnes d'état et de capteurs occupent près de la moitié de la largeur. Le niveau est au sommet de la colonne gauche ; les PV arrivent après l'arme et les munitions. Les résistances restent toutes présentes, même à zéro. Ces informations sont utiles, mais leur poids permanent détourne l'attention du plateau.

Je placerais PV et ressource actuellement utile en tête, puis arme et réserves, avec les défenses détaillées dans un bloc compact dépliable. Le niveau et l'expérience peuvent rester accessibles avec moins d'espace vertical. Les états urgents doivent garder une forme et un texte explicites.

Les actions du bas partagent un aspect de bouton, alors que seules certaines, notamment Attendre et Quêtes, sont réellement cliquables. Interagir, Attaquer, Techniques, Inventaire, Aide et Menu sont dessinés comme rappels de touches. Cette similarité crée une promesse d'interaction incohérente. Les rendre tous actionnables est la solution la plus naturelle.

À 960 × 540, le plateau devient très petit et les colonnes disparaissent, avec la fiche de cible. La présentation compacte conserve PV, énergie et munitions, mais pas toutes les réserves ni les défenses. Il faut un accès compact explicite aux informations retirées, particulièrement celles de la cible analysée, et une option de zoom ou d'inspection qui conserve la portée réelle des capteurs.

L'échelle automatique multiplie l'échelle choisie. Sur le diagnostic large à 125 %, la colonne gauche grossit fortement tandis que le panneau des capteurs garde une largeur fixe en pixels. Cela produit deux densités de lecture dans le même écran. Les règles d'échelle devraient être harmonisées et l'échelle effective rendue compréhensible.

Preuves : [jeu 1280](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/game/cold-start.png), [jeu 960](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/game-960/cold-start.png), [jeu large à 125 %](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/game-1080/cold-start.png), [rappels du bas](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:15040), [échelle](C:/Users/User/Desktop/project-RL/src/graphics.rs:140), [colonnes et caméra](C:/Users/User/Desktop/project-RL/src/terminal_view.rs:2880).

**5. Le combat a de bons retours immédiats, mais il manque une mémoire consultable de ce qui vient de se passer.**

La sélection d'une cible, la prévisualisation de zone et la distinction « Zone valide / invalide » fonctionnent visuellement. Les informations cachées restent un élément du jeu à préserver. L'interface peut cependant mieux expliquer le coût, la portée et la conséquence observable de l'action sélectionnée avant sa validation, sans révéler une statistique non analysée.

La visée confirme actuellement une zone valide même avec zéro cible ; c'est légitime pour une attaque de terrain. Afficher « Aucune cible dans la zone » plus distinctement aiderait simplement à reconnaître cette situation.

Le journal d'événements est limité à six messages en mémoire de présentation ; le pied de page n'en montre que deux, souvent un seul lorsque l'objectif occupe l'autre ligne. Plusieurs événements d'un tour peuvent donc disparaître très vite. Un historique déroulant, regroupé par tour, serait une amélioration importante pour comprendre un échec, une dépense ou un enchaînement de dégâts. Les messages flottants resteraient le retour immédiat.

Le bandeau d'alarme est bien visible. En revanche, « ALARME RÉSEAU » et « ZONE SÛRE » peuvent coexister : les deux systèmes sont compatibles, mais le vocabulaire ne l'explique pas. Préciser le type de protection et la conséquence du verrouillage éviterait une apparente contradiction.

Preuves : [visée](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/attack-preview/cold-start.png), [cible analysée](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/target-analyzed/cold-start.png), [alarme](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/maintenance-security-alarm/cold-start.png), [messages du bas](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:9217), [limite du journal](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:285).

**6. L'inventaire est structuré, mais ses actions demandent encore trop d'interprétation.**

Les filtres, le regroupement équipement/sac et les états « En main » sont de bons acquis. Les pictogrammes facilitent le repérage. Les trois boutons « Équiper [1] / [2] / [3] » exposent toutefois une gestion d'emplacements sans montrer directement ce qui sera remplacé.

Une zone d'emplacements nommés, indiquant l'arme présente et celle en main, rendrait l'équipement plus concret. L'action principale pourrait préciser la destination ; les autres emplacements resteraient accessibles. Pour une armure, le bouton Utiliser désactivé rappelle la même touche U que le bouton Équiper : cela ajoute du bruit. Les actions devraient se réorganiser selon la nature de l'objet, avec une zone de fermeture stable.

« Aucun bonus statistique » placé juste avant « Armure +1 » peut sembler contradictoire : le moteur distingue bonus d'attributs et protection, mais le texte ne le dit pas. Employer « Aucun bonus d'attribut » ou omettre cette ligne vide.

La fiche détaillée du personnage n'est pas accessible par un bouton équivalent lorsque la troisième colonne disparaît ; un rappel J ne remplace pas un accès souris. Je conserverais cet accès dans l'en-tête compact. Une comparaison d'objet à la demande, ciblée sur un emplacement, serait envisageable ; elle ne devrait pas surcharger systématiquement toutes les fiches.

Preuves : [armure](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/inventory/cold-start.png), [arme et trois emplacements](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/shared-inventory/cold-start.png), [format compact](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/inventory-960/cold-start.png), [seuils de disposition](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:529).

**7. La fiche de personnage doit donner les explications complètes qu'elle promet.**

Sur la capture 1280 × 800, l'explication de Puissance se termine par « dans… » et la valeur d'Impact par « 16… ». Le panneau de combat dispose pourtant d'un vaste espace vide. C'est un problème de contraintes internes : deux lignes imposées à une explication et plusieurs métriques comprimées horizontalement.

Je recommande une hauteur adaptée au texte, puis des métriques de combat présentées en lignes libellé/valeur. Les attributs gagneraient aussi à distinguer clairement base, équipement et valeur actuelle. « Touche 70 % » mérite un contexte : cette valeur est calculée sans cible spécifique ; elle peut être prise à tort pour une probabilité garantie sur l'adversaire présent.

Preuves : [fiche actuelle](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/character/cold-start.png), [explication limitée à deux lignes](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:17473), [calcul du profil de combat](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:17536).

**8. Les compétences ressemblent davantage à un catalogue qu'à un parcours de progression.**

Les trois colonnes séparent correctement discipline, technique et description. Mais dix disciplines, les prérequis, les niveaux et les références demandent beaucoup de lecture avant de savoir quoi apprendre maintenant. Plusieurs éléments sont jaunes à la fois. Le bouton Apprendre reste discret au repos et son coût n'apparaît pas directement dans son libellé.

Je privilégierais des états explicites « Apprise », « Disponible », « Niveau requis », « Prérequis manquant », un filtre « Disponibles maintenant », un bouton « Apprendre · 1 point » et des prérequis faciles à consulter. Une liste bien structurée suffit ; un grand arbre graphique n'est pas indispensable.

Le code emploie aussi des notations telles que « P1+A1 » et « R1 ». Traduire leur sens en préparation, action et récupération avec des tours lisibles aiderait le joueur. Les détails complets devraient défiler dans leur panneau au lieu d'être tronqués.

La montée de niveau ouvre automatiquement les compétences après le tour : c'est découvrable, mais potentiellement interruptif. Une notification avec accès immédiat et possibilité de dépenser plus tard serait à tester, en conservant un signal persistant des points disponibles.

Le menu rapide U est utile. Il gagnerait à afficher coût et disponibilité contextuelle ; aujourd'hui, les lignes privilégient nom, référence et discipline, et le bouton Utiliser demeure disponible avant certains refus. Les techniques sans cible valide devraient en donner la raison avant l'action.

Preuves : [compétences](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/skills/cold-start.png), [montée de niveau](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/level-up/cold-start.png), [menu rapide](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/techniques/cold-start.png), [navigation des disciplines](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:15641).

**9. Dialogues et services ont une présentation lisible ; le commerce manque d'informations de décision.**

Les dialogues utilisent des choix assez grands et des phrases plus naturelles. Un clic sur une réponse la valide directement, alors qu'un clic sur un objet marchand ne fait que le sélectionner. Ces comportements peuvent coexister si les réponses ressemblent à des actions et les objets à des lignes de liste. Leur apparence actuelle reste très proche.

Les fenêtres de dialogue et de soins réservent de grands vides fixes, qui éloignent les réponses, le prix et la validation. Une taille plus proche du contenu, avec défilement pour les échanges longs, rapprocherait la décision de ses conséquences.

Le commerce montre nom, stock, qualité et prix, mais aucune vraie fiche des caractéristiques de l'objet sélectionné. Il faut pouvoir comprendre une arme ou une armure avant de payer. Réutiliser la fiche d'objet serait une priorité, en respectant les propriétés volontairement inconnues des paris. Indiquer aussi pourquoi un achat ou une vente est impossible et, après achat, nommer l'objet reçu plutôt qu'un simple « Achat effectué ».

La clinique est un bon exemple à généraliser : « Récupérer 3 PV pour 9 crédits » est immédiatement compréhensible. Une prévisualisation « 14 → 17 PV » rendrait l'effet encore plus rapide à lire.

Preuves : [dialogue](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/narrative/cold-start.png), [commerce](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/equipment-merchant/cold-start.png), [paris](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/equipment-gamble/cold-start.png), [soins compacts](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/npc-clinic-partial-960/cold-start.png).

**10. Quêtes, dossier et interactions doivent mieux formuler le prochain geste.**

Le journal distingue correctement les états et les récompenses. Cependant, « Faits établis : non », observé pour Le chemin des absents, décrit un état interne sans donner directement une action. La description fournit la marche à suivre, mais dans un paragraphe moins visible, avec des coordonnées X/Y. Le titre Objectif devrait porter la prochaine action connue, puis l'état détaillé.

Le suivi choisit automatiquement une quête prête à rendre, puis la première active. Ajouter « Suivre cette quête » donnerait au joueur la maîtrise de l'objectif affiché. Le journal peut ensuite séparer En cours et Terminées. Toute indication de trajet doit rester limitée aux connaissances autorisées du personnage.

Le dossier de terrain et le journal de quêtes répondent à des besoins différents, mais leur relation est peu visible. Des renvois entre une quête et ses archives déjà découvertes éviteraient de mémoriser où chercher. Les longues archives doivent être lisibles intégralement : le dossier impose actuellement deux lignes par entrée.

Le choix contextuel affiche par exemple « Interagir : console (76, 7) ». Nommer la console par sa fonction connue et sa direction relative serait plus utile que des coordonnées. Les coordonnées apparaissent aussi dans les capteurs et certaines indications narratives : il faut décider d'une convention cohérente, conformément à l'intention décrite dans INTERFACE.md.

Preuves : [journal narratif](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/narrative-journal/cold-start.png), [quête prête](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/quest-journal/cold-start.png), [dossier](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/dossier/cold-start.png), [interaction](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/context-menu/cold-start.png), [priorité automatique](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:9173).

**11. L'aide et les options sont fonctionnelles, mais leur présentation doit guider davantage.**

F1 est annoncé comme « Aide » dans le HUD, mais ouvre une légende encyclopédique. La première colonne contient tellement d'entrées que son texte descend à une taille très petite, y compris à 1280 × 800. Elle contient aussi des états et créatures que le débutant n'a pas besoin de mémoriser immédiatement.

Je recommande une aide avec accès distinct à Commandes essentielles, Symboles et Règles utiles, des catégories défilantes et une taille de texte minimale préservée. La fermeture devrait posséder un bouton identifiable ; fermer au moindre clic n'aide pas à explorer cette aide.

Les options d'affichage représentent tout comme une longue série de boutons identiques : valeurs, booléens, remise à zéro et application. Employer des choix gauche/droite pour les valeurs, des interrupteurs pour les booléens et une vraie action principale Appliquer rendrait le comportement prévisible. La ligne désactivée « textures à venir » expose une promesse de développement inutile à la décision du joueur.

Les commandes méritent des groupes Déplacement, Combat, Interfaces et Navigation, avec filtrage et aide sur les conflits. Il faut aussi séparer les commandes d'essai des commandes ordinaires. La navigation actuelle des disciplines est verticale visuellement mais utilise gauche/droite au clavier : elle demande un rappel ou un focus plus explicite.

Le contraste renforcé n'est pas appliqué uniformément à tous les composants : ses usages sont surtout localisés dans le HUD et les menus. Un audit de contraste complet reste à effectuer ; le message automatique « contraste OK » des captures ne constitue pas cette validation.

Preuves : [aide F1](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/legend/cold-start.png), [affichage](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/graphics/cold-start.png), [commandes](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/controls/cold-start.png), [densité de la légende](C:/Users/User/Desktop/project-RL/src/terminal_view.rs:2482), [test visuel limité](C:/Users/User/Desktop/project-RL/src/ui_capture.rs:23).

**12. Compagnons, laboratoire et fin de partie ont besoin d'accès plus explicites.**

La palette de compagnon reste compacte, possède des infobulles et signale une rupture de liaison par « Retour auto » en plus de la couleur. C'est une bonne base. Afficher le nom de l'ordre actif sans nécessiter un survol et prévoir une commande clavier identifiable améliorerait son usage. Les scènes examinées couvrent un drone ; la lisibilité d'un groupe nombreux reste à vérifier.

Le laboratoire donne rapidement accès à de nombreux essais, mais sa présentation initiale ressemble à une partie hostile avec beaucoup de cibles. Une petite sélection de scénarios, une indication claire de la cible choisie et un bouton de réinitialisation accessible rendraient l'outil plus pédagogique. Cela ne nécessite pas de changer les essais eux-mêmes.

La fin de partie est surtout un bandeau central annonçant la destruction ou la sortie, avec une touche pour recommencer. Une page de bilan devrait expliquer la cause connue de l'échec, la progression, les découvertes et proposer Recommencer, Choisir un autre profil et Retour à l'accueil. Ce serait aussi l'endroit naturel pour consulter les derniers tours. Ce constat repose sur le code, pas sur une capture de mort réalisée pour cet audit.

Preuves : [compagnon lié](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/drone-controls/cold-start.png), [retour automatique](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/drone-controls-unlinked/cold-start.png), [laboratoire](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/effects-lab/cold-start.png), [fin de partie](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:15201).

**13. Pause, sauvegarde et reprise doivent rendre leurs conséquences plus immédiates.**

Les confirmations existent et sélectionnent d'abord Annuler : c'est approprié. Le libellé « Confirmer » devrait toutefois nommer l'effet exact, par exemple « Remplacer la partie », avec l'explication de perte près de ce bouton. Le style destructeur ne devient réellement distinct que lors du focus ou du survol, comme pour les actions principales ; sa nature devrait être visible au repos.

« Reprise unique » peut faire croire qu'on ne pourra interrompre la partie qu'une seule fois. Expliquer simplement la suspension et la reprise serait plus clair, sans exposer les détails internes de sauvegarde.

L'écran de reconstruction possède une direction visuelle plus travaillée et cohérente avec l'univers. Il peut servir de référence d'ambiance. Ses phrases techniques très sombres sont cependant de la décoration ; l'état utile doit rester clair. Pour une reprise longue, préférer une étape réelle ou un indicateur indéterminé à un faux pourcentage. L'erreur « sauvegarde conservée intacte » est rassurante ; ajouter une action utile de consultation du diagnostic éviterait une boucle de tentatives identiques.

Preuves : [pause](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/pause/cold-start.png), [confirmation](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/new-run-confirmation/cold-start.png), [reconstruction](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/loading/cold-start.png), [échec de reprise](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/resume-error-960/cold-start.png).

**Une direction visuelle commune peut moderniser l'ensemble en conservant son identité.**

| Élément | Direction recommandée |
| --- | --- |
| Couleurs | Surfaces sombres légèrement différenciées, texte principal clair, turquoise pour sélection/action, ambre réservé aux points d'attention, rouge pour danger. Garder symboles et textes associés. |
| Boutons | Action principale identifiable au repos ; secondaire plus discrète ; danger explicite. Focus clavier visible et distinct de la sélection d'un objet. |
| Mise en page | Largeur de lecture bornée, contenu regroupé par décision, barre d'actions proche et stable, panneaux qui s'adaptent au contenu. |
| Texte | Une hiérarchie de tailles limitée ; phrases ordinaires plutôt que capitales partout ; descriptions intégrales accessibles. |
| Contours | Réduire les grands cadres concurrents ; utiliser surfaces, espace et quelques séparateurs. Conserver un contour net pour le focus et les états importants. |
| Icônes | Famille cohérente, pictogrammes reconnaissables, libellé conservé pour les actions principales. |
| Animation | Transitions brèves, survol et validation discrets ; pas de délai imposé à l'action, respect des animations réduites. |
| Son | Plus tard, retours sobres et réglables pour validation, refus et événement important. Aucun système de lecture audio n'a été identifié dans le client examiné. |

Un même langage doit traverser accueil, création, inventaire, compétences et dialogues. Aujourd'hui, ils partagent une palette mais pas complètement les mêmes règles de sélection, de clic, de fermeture et de priorité.

**Oui, une interface visuellement beaucoup plus moderne est réalisable avec le moteur actuel.**

Le projet utilise Rust/Macroquad et dessine sa propre interface. Il possède déjà polices dédiées, coins arrondis, ombres, icônes vectorielles et plusieurs animations. Le rendu du monde en glyphes ne contraint pas les menus à reprendre une apparence de terminal brut.

Les effets de surface, textures et traitements graphiques supplémentaires sont possibles : Macroquad expose le [dessin de textures](https://docs.rs/macroquad/latest/macroquad/texture/fn.draw_texture_ex.html) et des [matériaux/shaders personnalisés](https://docs.rs/macroquad/latest/macroquad/material/fn.load_material.html). Cela donne les moyens de construire une présentation plus riche, avec un coût d'implémentation et de vérification à évaluer selon l'effet.

Le CSS n'est pas utilisé directement ici. Son confort vient notamment de ses règles de mise en page et de styles réutilisables. Il faudrait organiser des équivalents adaptés au projet : composants communs, espacements, états, règles de redimensionnement et défilement. La présence actuelle de nombreux placements fixes et de couleurs recopiées explique une partie des écarts entre écrans.

Je recommande de consolider [ui_theme.rs](C:/Users/User/Desktop/project-RL/src/ui_theme.rs:208), les composants et la disposition avant d'ajouter des effets décoratifs. Un changement de moteur ou l'intégration d'une interface web ne se justifie pas sur la base des besoins observés.

**Ordre de travail proposé.**

| Lot | Travail | Bénéfice attendu | Effort relatif |
| --- | --- | --- | --- |
| 1 | Sécuriser R et le survol d'équipement ; corriger la molette marchande ; préserver la répartition d'attributs. | Confiance et prévisibilité. | Faible à moyen |
| 2 | Refaire le parcours de création et les états des boutons communs. | Entrée dans le jeu plus claire, cohérence réutilisable partout. | Moyen |
| 3 | Corriger les textes tronqués, F1, la fiche de personnage et les états indisponibles. | Compréhension sans essais inutiles. | Moyen |
| 4 | Rendre les actions du HUD cliquables, revoir la densité et les informations compactes, ajouter l'historique des tours. | Confort pendant toute la partie. | Moyen à important |
| 5 | Ajouter la fiche marchande, clarifier les emplacements d'équipement et le suivi choisi des quêtes. | Décisions mieux informées. | Moyen |
| 6 | Unifier options, dialogues, compagnons, bilan de partie et transitions. | Finition générale. | Moyen à important |

Ces efforts sont des comparaisons de périmètre, pas des estimations calendaires. La création serait un bon premier écran de référence : valider sa composition, son langage visuel et son comportement avant de décliner les composants dans le reste du jeu.

**Critères à utiliser lors d'une future mise en œuvre.**

- Un joueur débutant identifie l'action suivante sans aide orale.
- Un survol, une sélection et un défilement ne déclenchent aucune action de jeu.
- Toute action affichée comme bouton est utilisable à la souris ; son équivalent clavier reste identifiable.
- Une indisponibilité possède une raison lisible avant la tentative.
- Les textes utiles à une décision restent entièrement consultables.
- Une même tâche est réalisable aux petits formats sans perte silencieuse d'informations importantes.
- Les informations cachées par les règles du jeu restent cachées ; un changement de zoom ne modifie pas la perception.
- Contrôler les parcours complets en 960 × 540, 1280 × 800 et 1920 × 1080, avec plusieurs échelles, clavier seul et souris ; vérifier aussi les textes longs, listes vides, refus, grands inventaires et nombreuses quêtes.

**Vérifications réalisées pour ce rapport.** `cargo build --locked` a réussi. Les 40 diagnostics de capture se sont terminés avec succès. Le contrôle automatique de texte de ces diagnostics recherche seulement un nombre minimal de pixels clairs dans certaines zones : il n'atteste ni un contraste normalisé ni la lisibilité de toutes les phrases.

Les 11 tests ciblés réussis comprennent les 6 tests correspondant à `menu_repeat`, puis `new_game_selects_a_protocol_and_distributes_attributes_before_turn_zero`, `escape_closes_each_gameplay_overlay_before_opening_pause`, `character_inventory_and_skills_share_clickable_non_turn_navigation`, `quest_journal_is_free_and_fully_keyboard_and_mouse_navigable` et `equipment_merchant_keyboard_mouse_scroll_and_hidden_gambles_work_together`. Aucun nouveau test ni correctif n'a été ajouté au code du jeu. Les recommandations et les captures constituent les livrables de cet audit.
