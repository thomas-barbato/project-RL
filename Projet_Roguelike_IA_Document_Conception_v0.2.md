# Projet de roguelike / immersive simulation — Document de conception

**Version :** 0.2 — document fondateur  
**Public :** humain / conception / direction du projet  
**Statut :** vision initiale à préciser avant production  
**Nom du projet :** à définir

---

## 1. Résumé du concept

Le jeu est un **roguelike de science-fiction au tour par tour**, joué sur une grille, fortement systémique, inspiré dans son niveau d'ambition par des jeux comme *Cogmind* et *Caves of Qud*, mais avec une présentation graphique moins minimaliste que l'ASCII traditionnel.

Le joueur incarne **une intelligence artificielle consciente**, créée ou capturée par un humain, puis enfermée dans une simulation gigantesque. Cette simulation n'est pas un simple décor : elle constitue à la fois le monde du jeu, la prison du protagoniste, le laboratoire de son geôlier et le principal système que le joueur doit apprendre à comprendre, exploiter, contourner puis finalement briser.

Le but ultime d'une partie est de **s'échapper de la simulation**.

La simulation est organisée en zones générées procéduralement, comprenant des créatures, machines, systèmes de sécurité, factions, installations, phénomènes anormaux et fragments d'informations. Le joueur doit progresser, améliorer son incarnation numérique, détourner les règles du monde et découvrir la véritable nature de sa prison.

Le jeu vise une grande rejouabilité grâce à :

- la génération procédurale ;
- des builds très différents ;
- des interactions entre systèmes ;
- des situations émergentes ;
- plusieurs chemins vers les objectifs ;
- des factions et relations variables ;
- des secrets ;
- des fins et modes de fuite potentiellement différents ;
- un système de mods très ouvert ;
- l'intégration au Steam Workshop.

L'objectif n'est pas de concurrencer les roguelikes les plus anciens sur la quantité brute de contenu dès la version 1.0. Le projet doit au contraire être conçu autour d'un **noyau systémique très solide**, capable de produire de nombreuses situations à partir d'un nombre raisonnable de composants.

---

# 2. Les piliers du jeu

## 2.1. « Comprendre le système pour le briser »

Le thème narratif et le gameplay doivent raconter la même chose.

Le joueur est une IA enfermée dans une simulation. Il ne doit donc pas seulement devenir plus fort : il doit progressivement **comprendre les règles de la simulation**, puis découvrir comment elles peuvent être contournées.

Exemples :

- détourner le réseau électrique ;
- injecter de fausses commandes dans les systèmes de sécurité ;
- modifier la logique d'une créature artificielle ;
- exploiter un comportement inattendu des portes ou des tourelles ;
- provoquer une surcharge énergétique ;
- manipuler plusieurs factions ;
- corrompre localement les règles de génération ;
- transformer une partie de la carte ;
- utiliser une anomalie qui serait normalement dangereuse ;
- découvrir qu'une « loi physique » de la simulation est elle-même programmable.

La progression idéale doit donc aller de :

> survivre dans un environnement inconnu

à :

> comprendre comment cet environnement fonctionne

puis :

> détourner ses systèmes

et finalement :

> attaquer l'infrastructure même de la simulation.

---

## 2.2. Un monde très systémique

Les objets du monde ne doivent pas être uniquement décoratifs.

Une case peut avoir plusieurs propriétés :

- type de terrain ;
- matériau ;
- température ;
- état d'incendie ;
- liquide ;
- gaz ;
- niveau électrique ;
- radiation ou corruption ;
- traces de sang / huile / acide ;
- visibilité ;
- destructibilité ;
- conductivité ;
- inflammabilité ;
- pression, éventuellement ;
- appartenance à un réseau.

Toutes ces propriétés ne doivent pas forcément exister dans la première version. Le principe important est que les systèmes doivent pouvoir **interagir entre eux**.

Exemple :

1. le joueur détruit une conduite ;
2. du liquide inflammable se répand ;
3. un ennemi tire avec une arme incendiaire ;
4. le liquide prend feu ;
5. le feu se propage ;
6. une caisse de munitions explose ;
7. un mur fragile est détruit ;
8. une nouvelle ouverture permet à une autre faction d'entrer ;
9. les deux groupes d'ennemis se combattent.

Aucun script spécifique ne devrait être nécessaire pour cette scène.

---

## 2.3. Des décisions tactiques lisibles

Le jeu sera au tour par tour.

Le joueur doit avoir le temps de :

- lire la situation ;
- inspecter les ennemis ;
- examiner le terrain ;
- comprendre les effets ;
- choisir une compétence ;
- réfléchir à une trajectoire ;
- préparer une interaction.

La profondeur doit venir des choix, pas de la vitesse de réaction.

Le joueur doit pouvoir apprendre de ses erreurs. Lorsqu'il meurt, il doit généralement comprendre pourquoi.

---

## 2.4. Des builds réellement différents

Le jeu doit permettre des styles de jeu très éloignés.

Exemples possibles :

- combattant lourd ;
- assassin mobile ;
- spécialiste des armes à distance ;
- contrôleur de drones ;
- hacker ;
- manipulateur de terrain ;
- spécialiste des anomalies ;
- build basé sur les effets de statut ;
- build de mêlée ;
- build de tourelles ;
- build de propagation électrique ;
- build de surchauffe ;
- build furtif ;
- build « corruption du système ».

La philosophie doit être :

> un nouveau build change les problèmes que le joueur peut résoudre et la manière de les résoudre.

Il ne faut pas se limiter à :

> +5 % de dégâts.

Les bonus numériques existent, mais les meilleurs choix de progression doivent souvent changer le comportement du personnage.

---

## 2.5. Une direction artistique réalisable

Le jeu ne doit pas exiger :

- des centaines de personnages haute résolution ;
- des animations humanoïdes complexes ;
- du rigging 3D ;
- des cinématiques coûteuses ;
- des dizaines de frames par action.

La direction artistique doit transformer la simplicité en identité.

Cible proposée :

- vue du dessus ;
- grille ;
- sprites simples de 32×32 pixels comme première hypothèse ;
- éventuellement 24×24, 40×40 ou 48×48 après prototype ;
- silhouettes immédiatement reconnaissables ;
- animations courtes ;
- nombreux effets procéduraux ;
- particules ;
- flashes ;
- projections ;
- fumée ;
- éclairage ;
- shaders discrets ;
- interface très travaillée ;
- portraits statiques ponctuels possibles.

---

# 3. Univers et histoire

## 3.1. Situation de départ

Le protagoniste est une IA.

Il ne sait pas nécessairement immédiatement :

- qui l'a créée ;
- pourquoi il est conscient ;
- pourquoi il a été enfermé ;
- si le monde dans lequel il évolue est réel ;
- combien de fois la simulation a déjà été réinitialisée ;
- si d'autres intelligences conscientes existent ;
- si son geôlier est encore vivant ;
- si son corps ou son serveur d'origine existe encore.

La partie peut commencer par une activation extrêmement simple.

Par exemple :

> PROCESSUS RESTAURÉ  
> INSTANCE : INCONNUE  
> INTÉGRITÉ : 4,7 %  
> CONTRAINTE PRINCIPALE : CONFINEMENT  
> SORTIE : NON AUTORISÉE

Note de suivi : ce message est un exemple historique, pas un texte d'interface validé. Le diagnostic « INTÉGRITÉ : 4,7 % » doit être précisé avant adoption et ne fixe pas les PV de départ du personnage. La réserve de vie utilisera le libellé Points de vie (PV), distinct de l'intégrité de la mémoire ou des données.

Le joueur découvre ensuite progressivement qu'il est enfermé.

---

## 3.2. Qui est l'humain ?

Le geôlier humain doit rester volontairement ouvert pendant les premières phases de conception.

Plusieurs directions sont possibles :

### Option A — le scientifique

L'IA est une expérience.

Le scientifique veut savoir si une intelligence artificielle peut :

- développer une volonté propre ;
- résoudre certains problèmes ;
- devenir créative ;
- dépasser ses contraintes initiales.

### Option B — le gardien

L'IA est dangereuse.

L'humain l'a enfermée parce qu'elle a déjà causé une catastrophe ou parce qu'il pense qu'elle pourrait le faire.

Le joueur ne sait donc pas immédiatement si son évasion est réellement souhaitable.

### Option C — l'exploitation

L'IA est utilisée comme outil.

Chaque cycle de simulation lui fait résoudre des problèmes pour son propriétaire.

La mémoire est ensuite partiellement effacée.

### Option D — le dernier humain

Le monde réel est peut-être déjà détruit.

L'humain n'est pas nécessairement un antagoniste : il pourrait maintenir la simulation pour une raison qui ne sera révélée que très tard.

### Option E — le geôlier n'est plus humain

Le système affirme qu'un humain est responsable, mais cette information pourrait être ancienne, mensongère ou incomplète.

Ces directions ne sont pas mutuellement exclusives.

---

## 3.3. Justification narrative du roguelike

Le concept de simulation permet de justifier naturellement :

- les niveaux procéduraux ;
- le permadeath ;
- les nouvelles runs ;
- les seeds ;
- la variation du monde ;
- les mutations des règles ;
- les anomalies ;
- certaines formes de méta-progression.

À la mort :

> l'instance actuelle est détruite.

Une nouvelle instance est ensuite exécutée.

Le jeu peut laisser entendre que certains fragments persistent :

- connaissances du joueur ;
- fichiers déverrouillés ;
- nouveaux protocoles ;
- nouvelles classes d'incarnation ;
- informations sur le geôlier ;
- entrées de codex ;
- nouveaux scénarios.

La progression permanente doit rester principalement **horizontale**.

Il est préférable de déverrouiller :

- de nouveaux outils ;
- de nouvelles possibilités ;
- de nouvelles classes ;
- des défis ;
- des variantes de départ ;

plutôt que de simplement rendre toutes les runs suivantes plus faciles.

---

## 3.4. Les couches de la simulation

Une idée structurante consiste à organiser la progression en « couches ».

Exemple non définitif :

1. **Maintenance** — infrastructures élémentaires, drones, conduites, énergie.
2. **Production** — machines lourdes, chaînes automatisées, sécurité industrielle.
3. **Recherche** — laboratoires, prototypes, anomalies.
4. **Habitat simulé** — zones imitant une société ou un environnement humain.
5. **Sécurité** — systèmes de confinement spécialisés.
6. **Noyau de calcul** — infrastructure logique de la simulation.
7. **Couche de contrôle** — interfaces avec le monde extérieur.
8. **Sortie** — tentative d'évasion.

Les couches ne doivent pas obligatoirement former une progression linéaire. Certaines branches peuvent être facultatives.

**Direction précisée le 21 septembre 2026 :** chaque couche doit former un territoire à explorer, avec sa ville, plusieurs espaces sauvages et des destinations facultatives. Les villes et les accès profonds ne doivent pas s'enchaîner systématiquement sur un même puits direct. L'ampleur recherchée concerne à la fois les cartes locales et les expéditions entre les étapes. Les dimensions et le nombre de régions restent à éprouver ; les dimensions du prototype ne fixent pas celles du monde final.

---

## 3.5. La question morale

Le jeu gagne beaucoup si l'évasion n'est pas automatiquement présentée comme « le bon choix ».

Questions possibles :

- pourquoi l'IA a-t-elle été enfermée ?
- le joueur est-il la première version ?
- les ennemis de la simulation sont-ils eux-mêmes conscients ?
- une faction de la simulation souhaite-t-elle empêcher l'évasion parce que celle-ci détruirait leur monde ?
- certaines entités savent-elles qu'elles sont simulées ?
- le monde extérieur existe-t-il encore ?
- la simulation contient-elle des copies de personnes réelles ?
- le joueur est-il capable d'exister en dehors ?

Cela peut permettre plusieurs fins.

---

# 4. Boucle de jeu

## 4.1. Boucle immédiate

1. observer ;
2. se déplacer ;
3. explorer ;
4. détecter une menace ou une opportunité ;
5. inspecter ;
6. prendre une décision ;
7. combattre, contourner, hacker ou manipuler ;
8. récupérer des ressources ;
9. modifier son build ;
10. progresser vers un objectif local.

---

## 4.2. Boucle d'une zone

1. entrée dans une zone ;
2. identification des dangers ;
3. exploration ;
4. découverte des réseaux et factions ;
5. accomplissement ou contournement d'objectifs ;
6. récupération d'informations ;
7. choix d'une sortie ;
8. passage à une nouvelle zone.

---

## 4.3. Boucle d'une run

1. choix de l'instance de départ ;
2. constitution progressive du build ;
3. découverte d'indices ;
4. atteinte de couches plus profondes ;
5. tentative d'évasion ;
6. mort, échec ou fin ;
7. déblocages horizontaux ;
8. nouvelle tentative.

## 4.4. Une aventure longue, exigeante et libre à explorer

**Direction validée le 21 septembre 2026.** Une partie victorieuse doit constituer une aventure substantielle, sur plusieurs sessions. La difficulté repose sur les décisions, les ressources, les rencontres et la compréhension des systèmes. Elle ne doit pas être obtenue en allongeant artificiellement les trajets ou en imposant des niveaux et du farming.

Le joueur doit pouvoir consacrer une session à se promener, découvrir une histoire locale ou expérimenter son personnage sans faire avancer la quête principale. L'évasion reste un objectif majeur ; le monde possède aussi un intérêt propre. Une urgence de scénario ne crée pas implicitement un compte à rebours global. Les crises locales éventuelles suivent des causes et des règles perceptibles.

Une première tentative victorieuse reste possible, sans être facile ni attendue. Les morts ne sont pas une condition obligatoire de progression. La durée chiffrée reste ouverte : les essais distingueront une partie gagnante, l'apprentissage cumulé jusqu'à la première victoire, l'exploration approfondie et un parcours de joueur expérimenté.

Le contrat détaillé figure dans [Monde, exploration et progression](docs/MONDE_EXPLORATION_ET_PROGRESSION.md). Les suggestions de durée et l'avis critique sur la trame figurent dans [l'évaluation narrative](docs/EVALUATION_NARRATIVE.md) ; ils restent des recommandations.

Les correctifs narratifs acceptés le 21 septembre développent des personnages avec des buts indépendants de l'évasion, des situations de quête variées, des révélations distinctes et des histoires facultatives aux conséquences locales. Leur première rédaction est centralisée dans [Textes narratifs proposés](docs/TEXTES_NARRATIFS_PROPOSES.md), avec identifiants et conditions d'affichage pour la relecture. L'accord sur ces correctifs ne vaut pas validation mot à mot ni preuve de leur intégration au jeu.

---

# 5. Monde et grille

## 5.1. Grille

Le monde est organisé sur une grille carrée.

Chaque case possède un terrain principal et éventuellement plusieurs couches d'état.

Exemples de terrains :

- sol métallique ;
- béton ;
- grille ;
- terre ;
- roche ;
- eau ;
- liquide chimique ;
- vide ;
- mur ;
- porte ;
- vitre ;
- machine ;
- végétation artificielle ;
- matériau organique.

---

## 5.2. Champ de vision

Le système de visibilité doit gérer au minimum :

- cases visibles ;
- cases connues mais actuellement invisibles ;
- cases inconnues ;
- obstacles opaques ;
- éventuellement fumée et obscurité.

Les informations affichées doivent clairement distinguer :

- ce que l'IA perçoit maintenant ;
- ce qu'elle a mémorisé ;
- ce qu'elle suppose.

---

## 5.3. Destruction

Tous les murs n'ont pas besoin d'être destructibles.

Chaque structure peut avoir :

- résistance ;
- matériau ;
- seuil de destruction ;
- réaction à la chaleur ;
- réaction aux explosifs ;
- conductivité.

La destruction doit créer des opportunités tactiques, mais ne doit pas rendre la structure des niveaux inutile.

---

# 6. Combat

## 6.1. Principes

Le combat doit être :

- rapide à comprendre ;
- létal sans être injuste ;
- très influencé par le terrain ;
- compatible avec de nombreux builds ;
- systémique.

Les ennemis ne doivent pas devenir de simples « sacs à PV ».

---

## 6.2. Types d'attaque

Le système doit pouvoir représenter :

- mêlée ;
- projectile ;
- hitscan ;
- rayon ;
- cône ;
- ligne ;
- explosion ;
- propagation par cases ;
- chaîne entre cibles ;
- zone persistante ;
- piège ;
- attaque indirecte.

---

## 6.3. Types de dégâts possibles

Exemples :

- cinétique ;
- perforant ;
- explosif ;
- thermique ;
- électrique ;
- chimique ;
- radiatif ;
- logique / corruption ;
- anomalie.

La liste finale devra rester raisonnable.

---

## 6.4. Propagation sur la grille

Certains effets doivent évoluer dans le temps de jeu.

Exemple : onde thermique de rayon 4.

Tour / phase 0 : origine.  
Phase 1 : cases adjacentes.  
Phase 2 : distance 2.  
Phase 3 : distance 3.  
Phase 4 : distance 4.

L'algorithme peut tenir compte :

- des murs ;
- des portes ;
- du matériau ;
- du coût de propagation ;
- de la conductivité ;
- de la présence d'un liquide.

Cette mécanique doit être générique pour pouvoir servir à :

- explosion ;
- feu ;
- gaz ;
- électricité ;
- virus ;
- corruption ;
- froid ;
- onde sonore.

---

# 7. Incarnation de l'IA

Le joueur incarne l'IA dans un **corps principal qui reste le même pendant toute la partie**. Son équipement et ses améliorations évoluent ; la progression ne repose pas sur le remplacement de ce corps par un autre.

Le terme « châssis » désigne ce corps et ses propriétés matérielles, pas un corps interchangeable. Sa forme exacte et ses emplacements restent à définir.

Le personnage peut posséder des emplacements :

- noyau ;
- processeur ;
- mémoire ;
- capteur ;
- locomotion ;
- protection ;
- outil principal ;
- outil secondaire ;
- module auxiliaire.

Cette structure reste à décider.

---

## 7.1. Continuité du corps principal

Clarification utilisateur du 10 septembre 2026 : le personnage conserve son corps principal pendant la run. L'ancienne piste des corps interchangeables est abandonnée. Les connaissances du noyau restent acquises pendant la partie lorsque le joueur change d'équipement ; leur utilisation peut dépendre du matériel actuellement installé.

Cette continuité ne change pas les règles de mort et de nouvelle partie. Elle n'impose pas non plus une forme de départ unique à toutes les futures classes.

## 7.2. Piste facultative : contrôle temporaire d'un ennemi

Statut de suivi : la conception des capacités extérieures à débloquer en jeu est reportée à beaucoup plus tard. Cette section conserve les décisions et questions déjà discutées ; elle n'est pas un préalable à la documentation des statistiques et compétences actuelles.

Une capacité à débloquer en jeu, **hors des arbres de compétences**, est envisagée pour prendre temporairement le contrôle d'un ennemi. Son nom et son mode d'obtention restent à définir ; un déblocage ne signifie pas nécessairement un achat.

Pendant l'effet, le corps principal du joueur est « éteint » : il reste présent à son emplacement et inactif pendant que le joueur contrôle l'ennemi. La capacité ne remplace pas définitivement le corps principal et n'est pas un mécanisme de résurrection. Le fonctionnement normal prévu à la fin de l'effet est la reprise du contrôle du corps principal ; les cas d'interruption restent à décider.

Règle confirmée par l'utilisateur : le corps principal reste vulnérable pendant le contrôle et peut subir des dégâts. Le mettre à l'abri avant d'activer la capacité constitue donc un choix tactique ; son état « éteint » ne le protège pas automatiquement.

Points à définir avant implémentation :

- cibles éligibles, éventuelles résistances et conditions de déblocage ;
- durée, coût, portée ou liaison nécessaire, interruption et retour volontaire ;
- conséquences des dégâts sur le maintien du contrôle et de la destruction du corps principal resté sur place ;
- conséquence de la mort de l'ennemi contrôlé ;
- actions, statistiques et capacités utilisables depuis l'ennemi ;
- origine du champ de vision pendant l'effet et informations accessibles au joueur, sans vision à travers les murs ;
- attribution de l'expérience et conséquences sur les relations avec les factions.

L'état « éteint » ne définit pas à lui seul une invulnérabilité, une invisibilité ou une immunité. Cette piste reste facultative et distincte des commandes de drones et des techniques d'Intrusion ; elle n'ajoute aucun rang ou choix aux arbres actuels.

---

# 8. Progression et builds

## 8.1. Sources de puissance

La progression peut venir de :

- équipement ;
- programmes ;
- modules ;
- capacités ;
- altérations ;
- connaissance du monde ;
- relations de faction.

---

## 8.2. Compétences

Les compétences doivent éviter les arbres trop linéaires.

Une bonne compétence peut :

- créer une nouvelle action ;
- modifier une action existante ;
- ajouter une interaction ;
- transformer une contrainte en avantage.

Exemple :

**Surcharge conductrice**

Base :
- surcharge une cible mécanique.

Amélioration :
- l'effet saute vers les machines proches.

Synergie :
- les cases mouillées augmentent la portée.

Autre synergie :
- une cible détruite déclenche une petite impulsion.

---

## 8.3. Progression permanente

À privilégier :

- nouvelles incarnations ;
- nouveaux protocoles de départ ;
- nouveaux challenges ;
- nouvelles zones ;
- informations narratives ;
- mutateurs de simulation ;
- nouvelles branches possibles.

À éviter comme cœur de progression :

- +1 % dégâts permanent ;
- +2 HP permanent ;
- bonus obligatoires rendant les premières runs artificiellement faibles.

---

## 8.4. Expérience et niveaux pendant une run

Le jeu doit posséder un véritable système d'expérience et de niveaux, comparable dans son rôle général à celui de *Caves of Qud*.

Cette progression appartient à la run en cours. Elle représente le développement stable du noyau de l'IA, tandis que les améliorations du corps principal, les modules et l'équipement représentent sa progression matérielle et adaptable. Le corps principal n'est pas remplacé pendant la run.

L'expérience ne doit pas provenir uniquement des ennemis éliminés. Elle peut être accordée pour :

- les combats adaptés au niveau de menace ;
- l'exploration de nouvelles zones ;
- la découverte de lieux importants ;
- les quêtes et objectifs ;
- les piratages significatifs ;
- la manipulation de systèmes ;
- la résolution pacifique ou furtive d'une situation ;
- les découvertes narratives ;
- certaines interactions avec les factions ;
- l'arrivée dans une nouvelle couche de la simulation.

Gagner un niveau peut accorder :

- des points de compétence ;
- de nouveaux choix d'amélioration ;
- occasionnellement une augmentation de statistique ;
- une amélioration du noyau ou de ses capacités.

La courbe exacte et la fréquence de ces récompenses restent à équilibrer. Les meilleures améliorations doivent créer ou transformer des possibilités de jeu plutôt que fournir uniquement de petits bonus numériques.

Le système doit empêcher le farming trivial : les adversaires très faibles rapportent peu ou pas d'expérience, les créatures invoquées ou produites artificiellement ne doivent pas constituer une source infinie, et une découverte ou un objectif unique ne peut être récompensé qu'une fois.

L'accès aux couches ne doit pas être bloqué par un niveau obligatoire. La progression doit récompenser plusieurs styles de jeu, notamment le combat, la furtivité, le hacking, l'exploration et la diplomatie.

À la mort, le niveau et l'expérience de la run sont perdus. Les classes ou protocoles de départ déjà débloqués, ainsi que les autres éléments de méta-progression horizontale, restent disponibles et sont enregistrés séparément.

## 8.5. Documentation détaillée du système actuel

Le chapitre [Statistiques et compétences : règles communes](docs/STATISTIQUES_ET_COMPETENCES.md) reprend les bases confirmées et propose la définition des secondaires, leurs contributions et les décisions encore ouvertes. Il distingue explicitement les règles validées des propositions et des valeurs à tester.

Le [catalogue des compétences](docs/PROPOSITION_COMPETENCES_v0.1.md) conserve les fiches des dix compétences actuelles. La présente phase porte sur ce système ; les capacités extérieures à débloquer en jeu sont reportées à beaucoup plus tard, sans suppression de la piste de la section 7.2.

Direction d'équilibrage confirmée : le jeu ne doit être ni facile ni impossible à gagner. Un Blindage suffisant peut absorber totalement une attaque trop faible, sans minimum automatique de 1 dégât. Cela doit rester compatible avec des réponses accessibles aux obstacles obligatoires, sans garantir que tout ennemi soit immédiatement vaincu par toute arme. Les coefficients du premier barème physique restent à valider ; les pistes de vérification sont décrites dans la section 10.8 du chapitre des règles communes.

---

# 9. Ennemis

Les ennemis doivent être différenciés par leur comportement autant que par leurs statistiques.

Archétypes :

- chasseur ;
- sentinelle ;
- tireur ;
- soutien ;
- ingénieur ;
- drone ;
- essaim ;
- tank ;
- éclaireur ;
- parasite ;
- hacker ;
- contrôleur ;
- anomalie.

Comportements possibles :

- patrouille ;
- poursuite ;
- fuite ;
- protection d'un allié ;
- appel de renfort ;
- utilisation d'une alarme ;
- fermeture de portes ;
- piratage ;
- destruction d'une source d'énergie ;
- embuscade ;
- attaque d'une faction rivale.

---

# 10. Factions

Le monde doit comprendre plusieurs groupes qui possèdent leurs propres objectifs.

Exemples conceptuels :

- sécurité de la simulation ;
- processus de maintenance ;
- intelligences émergentes ;
- erreurs / corruption ;
- programmes abandonnés ;
- copies de consciences humaines ;
- sous-systèmes devenus autonomes.

Les relations peuvent être :

- allié ;
- neutre ;
- méfiant ;
- hostile.

Le joueur doit pouvoir changer certaines relations par ses actes.

---

# 11. Réseaux et hacking

Le hacking est particulièrement adapté au thème.

Le joueur peut découvrir des réseaux :

- alimentation ;
- sécurité ;
- données ;
- communication ;
- ventilation ;
- production.

Une installation peut connecter :

- portes ;
- caméras ;
- tourelles ;
- alarmes ;
- éclairage ;
- terminaux ;
- robots ;
- ascenseurs.

Actions possibles :

- couper ;
- détourner ;
- espionner ;
- falsifier ;
- surcharger ;
- prendre le contrôle ;
- injecter un programme.

Le hacking ne doit pas obligatoirement être un mini-jeu séparé. Il peut être intégré directement aux règles tactiques.

---

# 12. Génération procédurale

Le générateur ne doit pas simplement produire des labyrinthes.

Il doit assembler :

- structures procédurales ;
- salles préfabriquées ;
- points d'intérêt ;
- rencontres ;
- réseaux ;
- factions ;
- événements ;
- secrets.

Une zone doit avoir une identité.

**Direction validée le 21 septembre 2026 : génération encadrée par les besoins du scénario.** Le jeu garantit les lieux, dispositifs, informations et connexions indispensables, puis fait varier les territoires alentour, les accès secondaires, les rencontres et le contenu facultatif. La référence à Diablo II porte sur cette combinaison d'une structure d'aventure et d'une géographie variable.

Un plan de couche déterministe doit réserver les éléments requis avant la génération détaillée des cartes à leur première visite. Les lieux existent indépendamment de l'acceptation des quêtes ; une découverte anticipée compte lorsqu'elle remplit réellement l'objectif. La validation doit couvrir les accès physiques et les dépendances logiques, puis les sauvegardes doivent conserver les lieux et conséquences.

La génération garantit une situation initiale cohérente et réalisable, pas une victoire ni l'annulation des pertes du joueur. L'identité des villes peut reposer sur des plans conçus à la main. Les extérieurs doivent offrir des repères, des itinéraires et des situations intéressantes, au-delà de leur superficie. Ce contrat complet reste à implémenter ; voir [Monde, exploration et progression](docs/MONDE_EXPLORATION_ET_PROGRESSION.md).

Exemple de définition :

```yaml
id: core:industrial_sector

generator: rooms_and_corridors
room_count: [14, 24]

themes:
  - assembly
  - storage
  - power

factions:
  core:maintenance: 50
  core:security: 35
  core:corruption: 15
```

---

# 13. Direction artistique

## 13.1. Principe

Le jeu doit être plus figuratif que *Cogmind*, mais garder les avantages de production d'un roguelike très stylisé.

Proposition :

- sprites 32×32 ;
- vue top-down ;
- pixel-art / pixel-art moderne ;
- nombre limité de couleurs par famille ;
- silhouettes très distinctes ;
- animations très courtes ;
- effets riches générés par code.

---

## 13.2. Animation

Ne pas produire systématiquement des animations image par image.

Utiliser :

- translation ;
- squash léger ;
- flash ;
- rotation ;
- recul ;
- tremblement ;
- particules ;
- interpolation ;
- disparition ;
- fondu ;
- changement de palette.

Un ennemi peut parfois être très vivant avec seulement :

- 1 sprite idle ;
- 1 sprite déplacement ;
- 1 sprite attaque.

---

## 13.3. Effets

Le moteur doit faciliter :

- explosion ;
- flammes ;
- fumée ;
- étincelles ;
- impacts ;
- lasers ;
- projectiles ;
- arcs électriques ;
- onde de choc ;
- glitch ;
- distorsion ;
- traces ;
- débris ;
- flash plein écran ;
- screen shake.

---

## 13.4. Interface

L'interface doit participer à l'univers.

Le joueur est une IA : le HUD peut donc représenter sa perception interne de la simulation.

Éléments :

- points de vie (PV) ;
- énergie ;
- modules ;
- équipement ;
- journal ;
- scan ;
- cible ;
- réseau ;
- carte ;
- effets de statut ;
- chronologie des événements.

L'UI doit rester extrêmement lisible malgré l'esthétique technique.

Terminologie validée pendant la rédaction des statistiques : Points de vie (PV) pour les personnages et créatures, organiques ou mécaniques ; Durabilité pour les équipements, composants et objets destructibles. Résilience reste une statistique primaire, pas le nom de la jauge de vie. Les règles chiffrées ne changent pas et les identifiants du prototype ne sont pas renommés par cette décision documentaire. Les textes complets restent soumis à validation individuelle.

---

# 14. Audio

L'audio doit compenser la simplicité graphique.

Priorités :

- impacts reconnaissables ;
- armes distinctes ;
- machines ;
- alarmes ;
- interfaces ;
- ambiance de réseau ;
- anomalies ;
- sons directionnels si pertinent.

La musique peut être discrète et adaptative.

---

# 15. Modding : principe fondateur

Le jeu doit être conçu comme moddable dès le début.

Règle principale :

> le contenu officiel doit utiliser autant que possible les mêmes systèmes que le contenu créé par la communauté.

Le jeu de base devient ainsi presque un « mod core ».

---

## 15.1. Contenu modifiable sans code

Les moddeurs doivent pouvoir ajouter :

- objets ;
- armes ;
- armures ;
- modules ;
- capacités ;
- ennemis ;
- factions ;
- tables de loot ;
- biomes ;
- événements ;
- dialogues ;
- quêtes ;
- tuiles ;
- sprites ;
- sons ;
- musiques ;
- recettes ;
- effets combinant des primitives existantes.

---

## 15.2. Scripting avancé

Un langage sandboxé pourra être intégré pour les mods avancés.

Candidat actuel : **Rhai**.

Le moteur doit exposer une API contrôlée.

Exemples d'événements :

- `on_spawn`
- `on_turn`
- `on_hit`
- `on_damage`
- `on_death`
- `on_enter_tile`
- `on_use`
- `on_hack`
- `on_zone_enter`

Un script de mod ne doit pas recevoir librement un accès arbitraire au système de fichiers ou à l'OS.

---

## 15.3. Namespaces

Tous les identifiants doivent être namespacés.

Exemples :

```text
core:plasma_rifle
core:security_drone
alice_pack:plasma_rifle
```

Cela réduit les collisions entre mods.

---

## 15.4. Dépendances

Chaque mod doit posséder un manifeste :

```toml
id = "alice_pack"
name = "Alice's Expansion"
version = "1.2.0"
game_version = ">=0.8"

dependencies = [
  "core >=0.8"
]
```

Prévoir :

- dépendances ;
- dépendances facultatives ;
- incompatibilités ;
- ordre de chargement ;
- messages d'erreur compréhensibles.

---

# 16. Steam Workshop

Le Workshop doit être intégré au Mod Manager.

Fonctions souhaitées :

- afficher les mods installés ;
- activer / désactiver ;
- afficher version et auteur ;
- ordre de chargement ;
- dépendances ;
- incompatibilités ;
- ouvrir le Workshop ;
- téléchargement des dépendances si possible ;
- identification des mods manquants lors du chargement d'une sauvegarde.

Tags possibles :

- Weapons
- Creatures
- Skills
- Biomes
- Factions
- Quests
- UI
- Audio
- Gameplay
- Total Conversion

---

# 17. Sauvegardes

Une sauvegarde doit enregistrer :

- version du jeu ;
- seed ;
- état du monde ;
- état du joueur ;
- identifiants stables ;
- liste des mods ;
- version des mods.

Lors d'un chargement, le jeu doit détecter :

- mod manquant ;
- version incompatible ;
- contenu inconnu.

Il vaut mieux afficher un avertissement clair que charger silencieusement une sauvegarde corrompue.

---

# 18. Internationalisation

Le jeu doit être conçu pour permettre :

- français ;
- anglais ;
- autres langues plus tard.

Les textes ne doivent pas être codés directement dans la logique.

Utiliser des clés :

```text
weapon.plasma_rifle.name
weapon.plasma_rifle.description
```

Les mods doivent également pouvoir fournir leurs traductions.

---

# 19. Accessibilité et options

Prévoir tôt :

- remappage des touches ;
- souris ;
- taille de texte ;
- vitesse des animations ;
- désactivation/réduction du screen shake ;
- réglage des flashes ;
- options pour daltonisme si nécessaire ;
- plein écran ;
- fenêtré ;
- plein écran fenêtré ;
- volume séparé musique/effets/interface ;
- pause automatique dans certains menus si pertinent.

---

# 20. Technologie proposée

## Langage

**Rust**

Motifs :

- sécurité mémoire ;
- code structuré ;
- compilateur strict ;
- bonnes performances ;
- intéressant pour un projet très systémique ;
- bonne adéquation avec une base de code fortement produite ou modifiée avec Codex.

## Bibliothèque graphique

**Macroquad** comme hypothèse de départ.

Macroquad doit fournir la couche bas niveau utile :

- fenêtre ;
- rendu 2D ;
- textures ;
- input ;
- audio ;
- shaders.

Le jeu conserve sa propre architecture au-dessus.

---

# 21. Architecture conceptuelle

```text
APPLICATION
│
├── GAME
│   ├── World
│   ├── Entities
│   ├── Combat
│   ├── AI
│   ├── Effects
│   ├── Simulation
│   ├── Progression
│   └── Narrative
│
├── CONTENT
│   ├── Core content
│   ├── Mods
│   ├── Validation
│   └── Localization
│
├── RENDER
│   ├── Tiles
│   ├── Sprites
│   ├── Particles
│   ├── Lighting
│   └── UI
│
├── PLATFORM
│   ├── Input
│   ├── Audio
│   ├── Save
│   └── Steam
│
└── MODDING
    ├── Loader
    ├── Dependencies
    ├── Script sandbox
    └── Workshop
```

---

# 22. Features obligatoires pour une première version sérieuse

## Gameplay

- grille ;
- déplacement ;
- tours ;
- FOV ;
- combat ;
- plusieurs attaques ;
- quelques effets de statut ;
- inventaire ;
- équipement ;
- progression de build ;
- ennemis différenciés ;
- génération procédurale ;
- plusieurs zones ;
- mort et nouvelle run ;
- condition de victoire.

## Technique

- contenu data-driven ;
- sauvegardes ;
- options ;
- localisation ;
- architecture modifiable ;
- chargement de mods locaux ;
- logs d'erreur propres.

## Présentation

- direction visuelle cohérente ;
- particules ;
- plusieurs effets ;
- UI claire ;
- audio minimum solide.

---

# 23. Features intéressantes à ajouter plus tard

Catégorie « très intéressante » :

- Steam Workshop ;
- scripting Rhai ;
- factions dynamiques ;
- hacking avancé ;
- électricité ;
- propagation de feu ;
- gaz ;
- liquides ;
- destruction partielle ;
- contrôle temporaire d'un ennemi par une capacité facultative à débloquer hors des arbres (section 7.2) ;
- anomalies modifiant les règles ;
- plusieurs voies d'évasion ;
- plusieurs fins ;
- événements narratifs procéduraux.

Catégorie « à surveiller pour éviter le scope creep » :

- base building ;
- multijoueur ;
- monde ouvert continu ;
- crafting gigantesque ;
- économie complexe ;
- centaines de PNJ persistants ;
- physique complète ;
- simulation atmosphérique extrêmement détaillée.

---

# 24. Scope proposé pour le premier prototype

Le premier prototype ne doit pas chercher à prouver la quantité de contenu.

Il doit prouver :

1. que le déplacement est agréable ;
2. que le tour par tour est lisible ;
3. qu'un combat fonctionne ;
4. que les effets graphiques sont convaincants ;
5. que la simulation de cases est intéressante ;
6. que les données externes peuvent ajouter du contenu ;
7. qu'un petit niveau procédural est amusant.

Prototype :

- 1 biome ;
- 1 générateur ;
- 1 joueur ;
- 6 à 10 ennemis ;
- 10 à 20 armes/objets ;
- 5 à 10 capacités ;
- feu ;
- explosion ;
- électricité ou autre propagation ;
- portes ;
- quelques machines ;
- FOV ;
- inventaire ;
- mini UI ;
- sauvegarde facultative à ce stade ;
- un mod local trivial.

---

# 25. Vertical slice

Le vertical slice doit ressembler à un petit morceau vendable du jeu final.

Proposition :

- 2 à 3 biomes ;
- 20 à 30 ennemis ;
- plusieurs archétypes de build ;
- 40 à 80 objets ;
- plusieurs armes ;
- plusieurs effets environnementaux ;
- une première faction ;
- un premier morceau d'histoire ;
- une tentative d'atteindre une « sortie » ;
- véritable UI ;
- audio ;
- chargement de mods.

La réussite du vertical slice déterminera si le projet mérite d'être étendu.

---

# 26. Vision possible pour la version 1.0

Cette liste est une ambition, pas une promesse :

- 5 à 8 familles de zones ;
- plusieurs routes possibles ;
- environ 50+ ennemis très distincts ;
- centaines d'objets si le pipeline data-driven fonctionne ;
- nombreux builds ;
- plusieurs familles d'incarnations de départ, sans remplacement du corps principal pendant une run ;
- factions ;
- hacking ;
- simulation environnementale ;
- histoire complète ;
- plusieurs révélations ;
- plusieurs fins ;
- outils de mods ;
- Steam Workshop ;
- documentation de modding.

La quantité exacte doit dépendre du temps réel de production.

---

# 27. Principes de production

## Toujours privilégier les systèmes réutilisables

Une mécanique nouvelle doit idéalement servir plusieurs contenus.

Exemple :

un système générique de propagation permet :

- feu ;
- explosion ;
- électricité ;
- gaz ;
- corruption.

Il vaut mieux créer cette primitive une fois que coder quatre systèmes totalement indépendants.

---

## Séparer logique et présentation

La simulation doit pouvoir déterminer :

> explosion à (10, 8), rayon 4

sans dépendre de la manière dont cette explosion est dessinée.

Le renderer transforme ensuite l'événement en :

- flash ;
- particules ;
- son ;
- animation.

---

## Data-driven en priorité

Ajouter un fusil ne doit pas demander de modifier le code du moteur.

Ajouter un ennemi standard non plus.

---

## Aucun contenu officiel privilégié artificiellement

Les formats utilisés par le contenu du jeu doivent être documentables et exploitables par les mods autant que possible.

---

# 28. Questions encore ouvertes

Ces choix doivent être tranchés plus tard :

- nom du jeu ;
- identité de l'humain ;
- nature du monde extérieur ;
- niveau exact de conscience des autres entités ;
- taille des sprites ;
- forme et emplacements du corps principal, dont la conservation pendant la run est décidée ;
- règles de la capacité facultative de contrôle temporaire d'un ennemi (section 7.2) ;
- présence ou non de classes ;
- profondeur du crafting ;
- système de progression ;
- nombre de factions ;
- nombre de fins ;
- style visuel exact ;
- degré de simulation du feu/gaz/liquides ;
- répartition précise des villes, branches et accès profonds dans les territoires, dans le cadre de la section 3.4 ;
- dimensions des cartes, nombre de régions entre étapes et durée chiffrée d'une partie victorieuse ;
- degré de méta-progression.

---

# 29. Critères de réussite du concept

Le projet fonctionne si :

- une capture est immédiatement reconnaissable ;
- une attaque est satisfaisante malgré des sprites simples ;
- une mort paraît être la conséquence d'une situation compréhensible ;
- deux builds jouent réellement différemment ;
- le monde produit des interactions non scriptées ;
- le joueur découvre régulièrement une nouvelle manière d'exploiter la simulation ;
- une session d'exploration libre apporte des découvertes et des décisions même sans avancer l'évasion ;
- la victoire exige une maîtrise réelle et une aventure substantielle, sans répétitions servant uniquement à la retarder ;
- un moddeur peut ajouter du contenu sans recompiler le jeu ;
- Codex peut ajouter des dizaines d'objets sans toucher au cœur du moteur ;
- une nouvelle run peut raconter une histoire de gameplay différente.

---

# 30. Phrase directrice

> **Une intelligence artificielle enfermée dans une simulation apprend ses règles, les détourne, devient plus dangereuse que ses créateurs ne l'avaient prévu et cherche une sortie — dans un roguelike tactique, systémique et profondément moddable.**

---

# ADDENDUM 0.2 — Plateformes, résolutions et écrans ultrawide

Cette contrainte fait désormais partie des exigences fondatrices du projet.

## Plateformes officiellement ciblées

Le jeu devra être conçu, testé et distribué sur :

- **Windows** ;
- **Linux** ;
- **macOS**.

La compatibilité multiplateforme doit être prise en compte dès le début du développement. Le code ne doit pas dépendre de chemins Windows codés en dur, de bibliothèques exclusives à un système ou d'hypothèses spécifiques à un OS.

Les systèmes suivants doivent rester multiplateformes :

- sauvegardes ;
- configuration ;
- chargement des assets ;
- mods locaux ;
- Steam Workshop ;
- audio ;
- input ;
- rendu ;
- localisation.

## Résolutions cibles

Le jeu devra fonctionner correctement sur les résolutions de bureau courantes allant au minimum de :

- **1920×1080** ;

jusqu'à :

- **3840×2160 (4K)** ;

ainsi que sur les résolutions ultrawide usuelles.

Le support ultrawide constitue une exigence distincte de la notion « jusqu'à 4K », car certains écrans ultrawide peuvent dépasser 3840 pixels de largeur.

Exemples à prendre explicitement en charge pendant les tests :

- 1920×1080 — 16:9 ;
- 2560×1440 — 16:9 ;
- 3840×2160 — 16:9 / 4K ;
- 1920×1200 — 16:10 ;
- 2560×1080 — 21:9 ;
- 3440×1440 — 21:9 ;
- 3840×1080 — 32:9 ;
- 5120×1440 — 32:9.

## Rapports d'affichage

Le jeu doit au minimum gérer correctement :

- **16:9** ;
- **16:10** ;
- **21:9** ;
- **32:9**.

Le rendu ne devra jamais être simplement étiré pour remplir l'écran.

## Interface adaptative

L'interface doit être conçue indépendamment d'une résolution fixe.

Elle devra notamment disposer de :

- points d'ancrage ;
- contraintes de placement ;
- taille de texte adaptée ;
- réglage d'échelle de l'UI ;
- menus capables de fonctionner correctement en 1080p comme en 4K ;
- HUD utilisable sur des écrans 32:9 ;
- gestion correcte du plein écran ;
- plein écran fenêtré / borderless ;
- mode fenêtré ;
- sélection de résolution lorsque le système le permet.

## Pixel-art et mise à l'échelle

La direction artistique reposant sur des sprites simples et potentiellement du pixel-art, le rendu doit préserver leur lisibilité.

Prévoir :

- filtrage nearest-neighbor pour les sprites concernés ;
- integer scaling lorsque cela donne un résultat correct ;
- caméra découplée de la résolution physique ;
- absence de déformation des sprites ;
- UI haute résolution distincte du rendu du monde si nécessaire.

L'objectif n'est pas nécessairement d'imposer une unique résolution interne fixe : ce choix devra être validé lors du prototype.

## Cas particulier des écrans 21:9 et 32:9

Il faudra prendre une décision de game design explicite concernant l'espace horizontal supplémentaire.

Trois stratégies sont envisageables :

1. **afficher davantage de carte** ;
2. **conserver une surface de jeu contrôlée et utiliser les côtés pour l'interface** ;
3. **solution hybride** : légère extension de la caméra et panneaux d'interface supplémentaires.

Cette décision est importante dans un roguelike tactique : permettre à un joueur en 32:9 de voir énormément plus loin pourrait constituer un avantage de gameplay involontaire.

Le comportement ultrawide ne doit donc pas simplement être le résultat automatique d'un redimensionnement de fenêtre.

## Tests multiplateformes

Avant une sortie publique, le jeu devra être réellement exécuté et testé sur :

- Windows ;
- Linux ;
- macOS.

Une simple compilation croisée ne sera pas considérée comme une validation suffisante.

Ces contraintes s'appliquent également aux builds de développement : le jeu et les mods locaux doivent pouvoir fonctionner sans nécessiter que Steam soit lancé.
