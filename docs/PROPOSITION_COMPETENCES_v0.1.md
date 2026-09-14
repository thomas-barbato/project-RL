# Project RL — Proposition complète des compétences

Version 0.1 révisée, annexe technique 0.6 — 10 septembre 2026 — Cinq corrections du point 8 validées et appliquées ; autres barèmes d'essai conservés.

**Décision postérieure du 13 septembre 2026 :** les rangs de compétence sont retirés. Une technique est accessible selon son niveau minimal, ses éventuels seuils d'attributs et les techniques déjà apprises. Les nombres de 1 à 5 conservés dans les tableaux historiques deviennent des niveaux minimaux. La courbe de prix des cinq premiers apprentissages reste à tester et son dernier tarif s'applique aux suivants, sans plafond de techniques apprises.

Ce document rassemble les compétences discutées et les simplifications approuvées après relecture. Il décrit un objectif de conception ; il ne signifie pas que ces systèmes sont déjà programmés. Les dépendances techniques et les réglages encore ouverts sont identifiés avant consolidation des documents fondateurs.

Le catalogue couvre cinq catégories et dix compétences : 94 entrées principales de techniques ou d'améliorations, complétées par 10 variantes électroniques. Reconnaissance comprend sept entrées ; Manœuvre en comprend neuf après fusion de Retraite méthodique dans Pas de dégagement. Pour une première lecture, les sections 4 à 8 contiennent les fiches, la section 11 donne le jugement critique et la section 13 distingue les directions retenues des réglages à tester. Un personnage ne reçoit jamais automatiquement tout ce catalogue.

Suite demandée par l'utilisateur : l'annexe 16 complète chacune des 104 entrées actives par un profil technique d'essai, en lien avec les sections 13 à 19 des [règles communes](STATISTIQUES_ET_COMPETENCES.md). Les anciennes mentions de paramètres à définir sont complétées par cette annexe, sans transformer tous les chiffres proposés en décisions validées. Après l'étape 7, l'accord explicite sur les cinq corrections du point 8 autorise les changements ciblés de la révision 0.6 : historique en section 15.7. Les [textes joueur](TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md) suivent ces corrections ; aucune intégration au jeu n'est effectuée.

Après les explications du 10 septembre, l'utilisateur a autorisé l'application des simplifications. REC-07 est fusionnée dans REC-01, REC-06 est mise en attente et REC-10 est retirée de la première version. Les reformulations de Reconnaissance et d'Intrusion sont appliquées dans les fiches. La section 15 conserve les raisons de ces décisions ; les identifiants retirés restent réservés dans l'historique, sans achat ni prérequis actif. Les coûts chiffrés restent à tester.

## 1. Périmètre et statut des décisions

### Déjà validé dans la conversation

- Cinq statistiques primaires : Puissance, Coordination, Résilience, Perception et Traitement. Influence est retirée.
- Le corps principal reste le même pendant toute la partie ; son équipement et ses améliorations évoluent. Les aptitudes du noyau IA restent acquises lors d'un changement d'équipement. Les capacités matérielles proviennent du corps, des modules et des équipements.
- Échelle primaire de 1 à 10, valeur ordinaire de 5, budget de création de 28 points, valeurs de création entre 3 et 8, plafond absolu de 10.
- Chaque technique ou amélioration s'apprend séparément ; son accès dépend de conditions explicites, jamais du nombre de choix déjà effectués dans la discipline.
- Les connaissances acquises sont conservées lors d'un changement d'équipement, mais leur utilisation peut demander un équipement compatible. La prise de contrôle temporaire d'un ennemi est une piste facultative hors des arbres, décrite en section 15.6, et non un remplacement du corps principal.
- Expérience et progression du noyau propres à la partie ; classes de départ débloquées séparément et favorisant la variété des builds.
- Progressions de Combat rapproché, Tir et Guerre électronique discutées et acceptées. Leurs noms et leurs effets centraux sont repris ci-dessous.
- Une seule réaction de combat entre deux actions normales du personnage, commune aux techniques concernées.
- Vision et sonar bloqués par les murs et les portes fermées ; distinction entre perception actuelle, souvenir et information incertaine. L'écran et les animations ne changent pas l'information tactique disponible.

### Orientations retenues après relecture

- Organisation en dix compétences : Reconnaissance regroupe l'ancienne Reconnaissance et Analyse tactique ; Ingénierie regroupe Maintenance et Ingénierie modulaire.
- Catalogue de Démolition, Manœuvre, Furtivité, Ingénierie et Contrôle de drones retenu comme direction de travail ; Reconnaissance et Intrusion révisées dans ce document.
- Préparations, prérequis, interruptions et contre-mesures conservés comme base des essais, avec la limite commune des réactions.
- Variantes électroniques et leurs compromis conservés ; chiffres, niveaux minimaux et seuils d'attributs susceptibles d'ajustement après essais.
- Drones hors perception limités aux routines et rapports datés ; jeu solo signifiant un seul joueur humain, avec PNJ alliés, drones ou familiers/pets possibles.
- Traces et secrets localisés ; repérage des parois fragiles ; analyses regroupées ; sécurité active nécessaire à la falsification des registres et à la protection d'un contrôle piraté.
- Aucun maximum de techniques apprises par discipline. La réattribution et les extensions de liaison restent à définir séparément.

Les dégâts, probabilités, portées, réserves, durées et courbes d'expérience restent à équilibrer. Les noms des nouvelles techniques peuvent tous être changés. Aucune classe nouvelle n'est déclarée définitive par ce rapport.

### Répartition générale

| Catégorie | Compétences | Décisions principales |
|---|---|---|
| Combat | Combat rapproché, Tir, Démolition | Choisir l'engagement, la cible, le terrain à transformer et les ressources à dépenser. |
| Mobilité et discrétion | Manœuvre, Furtivité | Se placer, se retirer, réduire sa signature et choisir quand être détecté. |
| Perception | Reconnaissance | Obtenir et interpréter une information utile avant d'agir. |
| Ingénierie | Ingénierie | Entretenir, adapter et exploiter son matériel avec des ressources limitées. |
| Systèmes numériques | Intrusion, Guerre électronique, Contrôle de drones | Obtenir des accès, attaquer les systèmes et coordonner des unités matérielles. |

## 2. Progression commune

### 2.1. Conditions d'accès, technique, amélioration et matériel

Une technique est une action, une posture, une procédure ou un comportement appris. Une amélioration modifie une technique connue et demande cette technique en prérequis. Le matériel fournit les moyens physiques ou logiciels de l'exécuter.

Le joueur peut apprendre toute entrée dont il remplit les conditions : niveau du personnage, valeurs d'attributs et techniques prérequises. Toutes ne demandent pas les trois types de conditions. Une entrée non répétable ne peut être achetée qu'une fois ; aucune entrée de ce catalogue ne donne un empilement illimité de bonus.

Exemple : une technique avancée de Tir peut demander le niveau 3 et Rafale contrôlée, tandis qu'une autre peut demander le niveau 3 et une Coordination minimale. Le nombre de techniques déjà apprises ne remplace aucune de ces conditions.

Les classes peuvent accorder des techniques au départ, mais ces acquis ne créent ni palier implicite ni plafond. La réattribution reste une décision séparée.

La découverte d'un équipement ou d'un programme spécialisé apporte des fonctions natives et des variantes matérielles. Elle ne vaut pas automatiquement achat d'une connaissance du noyau. Inversement, apprendre une technique doit suffire à en connaître la procédure standard : il faut éviter de réclamer aussi un livre rarissime pour chaque action de base. Une fonction native d'un objet reste utilisable selon les règles de cet objet, même si une technique peut ensuite l'améliorer.

### 2.2. Coûts d'apprentissage à tester

| Apprentissage dans la discipline | Repère historique | Coût proposé | Coût cumulé | Nombre cumulé de choix |
|---|---|---:|---:|---:|
| 0 | Aucun | 0 | 0 | 0 |
| 1 | Initié | 1 | 1 | 1 |
| 2 | Opérationnel | 1 | 2 | 2 |
| 3 | Spécialiste | 2 | 4 | 3 |
| 4 | Expert | 2 | 6 | 4 |
| 5 | Maîtrise | 3 | 9 | 5 |

La proposition initiale d'un point de compétence par niveau et les tarifs ci-dessus restent provisoires. Avec ce barème, trois apprentissages dans deux disciplines coûtent 8 points ; cinq apprentissages dans une discipline coûtent 9 points. Il faut confronter ce rythme au nombre de niveaux réellement atteints pendant une partie ordinaire. À partir du sixième apprentissage d'une même discipline, le coût reste actuellement fixé à 3 points.

Les compétences progressent par dépense des points gagnés avec l'expérience. Répéter un tir, une réparation ou un piratage ne fait pas monter directement la compétence. Les récompenses d'exploration, de quête, d'infiltration et d'objectifs doivent permettre de progresser sans rendre l'élimination systématique obligatoire.

### 2.3. Statistiques primaires et limites matérielles

| Primaire | Contributions privilégiées | Limites à préserver |
|---|---|---|
| Puissance | Impact, exploitation des actuateurs, recul, déplacements forcés. | La masse, l'ancrage et les capacités du châssis restent déterminants. |
| Coordination | Précision, manipulation mécanique, déplacement contrôlé, parade. | Ne donne pas automatiquement une accélération générale ni une furtivité parfaite. |
| Résilience | Maintien opérationnel, résistance aux interruptions et aux perturbations. | Ne remplace pas tous les blindages et pare-feu ; ne crée pas de ressources de réparation. |
| Perception | Détection, observation, ciblage et interprétation des indices. | Ne révèle pas les positions actuelles derrière les murs et ne masque pas les informations essentielles aux personnages ordinaires. |
| Traitement | Intrusion, procédures numériques, analyse et commandes complexes. | Ne multiplie pas simultanément dégâts, cibles, portée et économie de toutes les techniques ; ne crée ni énergie ni bande passante. |

Les compétences donnent accès à des possibilités ; les statistiques secondaires expriment leur efficacité dans un équipement et une situation donnés. Ce rapport n'impose pas de seuils primaires chiffrés supplémentaires pour apprendre les techniques. Les actions physiques restent exécutables seulement par un corps compatible.

Le document complémentaire [Statistiques et compétences : règles communes](STATISTIQUES_ET_COMPETENCES.md) rassemble les bases confirmées et une proposition de définition des secondaires, de leurs sources et de leurs limites. Les propositions y restent à valider ; elles ne modifient pas implicitement les fiches ci-dessous. La poursuite actuelle de la documentation exclut les capacités extérieures à débloquer en jeu, reportées à beaucoup plus tard.

Ajout demandé par l'utilisateur : le Blindage (armure) apparaît explicitement comme secondaire de défense physique, distincte de l'Esquive et de la Défense numérique. Ses contributions matérielles alimentent cette valeur unique. La section 10 du document complémentaire propose un premier calcul de Précision/Esquive et une réduction fixe par le Blindage avec pénétration ; ces formules ne sont pas encore validées. Brise-armure et Tir de rupture conservent leurs fiches actuelles ; leurs raccordements au calcul sont des propositions distinctes.

Confirmation ultérieure : une armure suffisante peut absorber totalement une attaque trop faible, sans minimum automatique de 1 dégât. L'objectif est un jeu ni facile ni impossible à gagner ; les réponses accessibles et les obstacles obligatoires devront être éprouvés. Cet accord ne valide pas tous les coefficients du barème proposé. Les sections 3.3 et 10.8 du document complémentaire précisent la portée de cette décision.

La section 11 du document complémentaire poursuit avec Impact physique, dégâts de mêlée ordinaires, Points de vie maximaux et changements de réserve. Le rôle de Puissance limité par le matériel, la contribution limitée de Résilience et l'absence de réparation automatique à l'augmentation du maximum sont retenus comme base de travail après le retour utilisateur ; les valeurs restent à éprouver. Les fiches de mêlée et d'Ingénierie ci-dessous ne sont pas chiffrées ou modifiées implicitement par cet accord.

La section 12 formalise ensuite les résistances thermique, électrique et chimique, les attaques mixtes, Stabilité système et l'interruption de préparations. Après accord utilisateur, la distinction des protections, les résistances en pourcentages et l'interruption sans perte automatique du prochain tour avec une courte protection contre les répétitions sont retenues comme direction. Le plafond de résistance de 75 % reste une base d'essai ; les autres chiffres et modalités détaillées restent à éprouver ou préciser. La section relève aussi les écarts avec le calcul du prototype, sans modifier le code ni les fiches de Guerre électronique.

## 3. Règles transversales retenues pour les essais

### 3.1. Temps, préparations et réactions

- Consulter la fiche, examiner une trajectoire prévue ou préparer une commande dans l'interface ne fait pas avancer la simulation.
- Une action exécutée a un coût en temps. Une tentative valide qui échoue peut consommer son temps et ses ressources ; une commande impossible pour une raison déjà connue est refusée avant dépense.
- Un échec ne doit pas servir à tester gratuitement l'existence d'une cible cachée. Les tentatives sur une case sans information fiable ne confirment pas implicitement une présence.
- Viser, armer un piège, poser une balise, réparer et commander activement une unité sont des actions de jeu. Les manipulations d'interface seules n'en sont pas.
- Une préparation longue est découpée en étapes annoncées et interruptibles. Le joueur doit comprendre le temps investi et le risque d'interruption.
- Pendant une préparation, `Attendre` investit une UT supplémentaire dans la commande déjà mémorisée et déclenche automatiquement son exécution à la fin. La même commande peut également être répétée. Se déplacer, attaquer autrement ou entreprendre une autre action acceptée annule explicitement la préparation ; l'interface affiche cette conséquence et propose un bouton de continuation.
- Un déplacement sur plusieurs cases résout les cases successivement : collision, pièges, danger et surveillance. Il ne traverse pas gratuitement un couloir couvert.
- Les techniques décrites comme lentes ou exigeant une récupération nécessiteront une représentation du temps adaptée. Dans le premier moteur à phases simples, elles peuvent être représentées par des étapes ou un état de récupération explicite, avant toute éventuelle initiative plus fine.

La limite acceptée est une réaction de combat par personnage entre deux actions normales. Règle retenue pour les essais : le droit à réaction revient au début d'une nouvelle action normale qui consomme effectivement du temps, y compris Attendre. Ouvrir un menu, choisir une cible, annuler ou envoyer une commande refusée ne le rétablit pas.

Une réaction ne rétablit pas ce droit, ne déclenche pas une autre réaction et consomme les ressources de l'action produite. Une posture de Surveillance ou de Parade expire au début de la prochaine action normale de son propriétaire, ou après son déclenchement. Une riposte préparée appartient à cette fenêtre ; elle n'ouvre pas un tour gratuit.

Pour les drones, PNJ et autres compagnons, la base des essais applique la même limite à chaque acteur. Le nombre d'unités reste un paramètre d'équilibrage majeur ; l'énergie et la bande passante s'appliquent aux unités électroniques concernées, sans être imposées aux familiers par cette règle. Les destructions environnementales peuvent se propager selon leurs propres règles, avec un nombre fini d'objets et sans récursion infinie.

### 3.1 bis. Compétences intrinsèques et équipement

Décision retenue : une compétence apprise fournit toujours son effet de base. Une charge, une mine, un leurre, une balise, un drone ou une réparation créés par une technique ne demandent donc ni consommable, ni outil, ni châssis préalable dans l'inventaire. Leur coût porte sur le temps, l'énergie, la chaleur, la bande passante, la recharge et le nombre d'instances actives. La bande passante d'un dispositif persistant reste réservée jusqu'à sa disparition, sa récupération ou sa destruction.

Les objets correspondants restent dans le monde pour la fabrication, le commerce et les améliorations futures : ils pourront renforcer, spécialiser ou transformer l'effet, jamais autoriser seuls l'usage d'une compétence déjà apprise. Un drone acheté ou construit sera ainsi une variante persistante et personnalisable ; `DRN-01` garantit néanmoins un drone utilitaire de base. Cette règle remplace les mentions plus anciennes de matériau obligatoire ou d'objet consommé dans les tableaux de ce document.

### 3.2. Perception, ciblage et information

- Une attaque ciblant une entité exige une perception actuelle compatible avec son mécanisme. Un souvenir ne suffit pas à suivre une cible mobile.
- Un explosif ou une balise peut viser une case connue sans garantir ce qui s'y trouve maintenant, si sa trajectoire est valide. La prévisualisation n'y dessine pas les occupants cachés.
- Les émissions électroniques locales et leurs rebonds respectent les murs et portes fermées selon la direction déjà retenue. Surcharge en cascade et Infection ne créent pas de propagation automatique à travers ces obstacles.
- La propagation d'un effet et la perception sont deux règles distinctes. Un mur détruit peut ouvrir un passage à une phase ultérieure d'une explosion, sans révéler des ennemis au-delà du champ réellement accessible.
- Un journal, un plan récupéré ou une trace donne une information datée, partielle ou incertaine. Il ne devient pas une position ennemie actualisée en permanence.
- Les sons indirects ou signaux incertains suivent leurs règles dédiées et restent présentés comme des indices ; ils ne se transforment pas en silhouettes exactes.
- L'identité évidente, l'attitude connue, les dangers visibles et l'annonce de préparation d'une attaque restent accessibles sans Reconnaissance spécialisée.

Pour les commandes distantes, cette version prend une liaison directe non obstruée comme cas de référence. Une extension par câble ou réseau reconnu devra définir explicitement ses conditions et ses retours d'information. Une commande adressée à un dispositif connu ne doit jamais donner, à elle seule, la position d'un inconnu derrière un mur.

### 3.3. Coûts, risques et réponses possibles

Chaque technique cherche un compromis principal : temps, exposition, énergie/chaleur, munitions, composants, préparation ou canal de contrôle occupé. Toutes ne cumulent pas tous ces coûts.

- Une capacité de zone indique ses cases affectées connues et son risque pour les alliés. L'identification ami/ennemi n'est jamais une connaissance magique des intentions.
- Les dégâts électroniques physiques et les attaques logicielles ont des conditions et des défenses distinctes. Les libellés définitifs des résistances restent à arrêter.
- Une créature simulée purement organique n'est pas automatiquement piratable parce que le monde est une simulation. La compatibilité est une propriété de la cible. Les pouvoirs d'anomalie des couches profondes pourront être définis séparément.
- Une immobilisation, une entrave ou une neutralisation doit laisser une réponse possible : durée bornée, résistance, coût, rupture de liaison ou protection temporaire contre la répétition. La règle précise sera commune aux capacités concernées.
- Réparer puis démonter ne multiplie pas les ressources. Fabriquer et détruire ses propres unités ne crée pas une source d'XP. Réouvrir une porte ou réinfecter la même cible ne constitue pas un nouvel objectif récompensé.
- Les actes contre des alliés ou des neutres peuvent être attribués selon les témoins, les preuves et la mémoire des PNJ. Une capacité ne modifie pas arbitrairement une réputation globale.

### 3.4. Lecture des fiches

Les codes tels que MEL-01 sont des repères de relecture, pas des noms à afficher au joueur. La colonne Niveau indique le niveau minimal du personnage. Les prérequis écrits sont obligatoires dans la proposition ; les synergies ne le sont pas. Une amélioration indiquant une technique par son code demande de l'avoir apprise.

Les choix acceptés sont conservés dans leur compétence, sous leur forme révisée lorsqu'une simplification a été approuvée. Les précisions opérationnelles constituent la base des essais ; leur paramétrage reste à éprouver.

Une entrée dépendant d'un système absent de la version jouable n'est pas proposée à l'achat tant que ce système n'existe pas. Cela concerne notamment le diagnostic énergétique, les composants ciblables et les deux techniques d'Intrusion liées à une sécurité active. Cette disponibilité de version est distincte d'un matériel compatible que le joueur peut obtenir plus tard dans une partie.

## 4. Catégorie Combat

### 4.1. Combat rapproché

Statut : progression et noms validés ; précisions de résolution retenues comme base des essais.

Rôle : frapper fort, frapper précisément, contrôler le contact. Primaires principales : Puissance et Coordination ; Résilience peut aider à supporter une perturbation pendant une préparation. Matériel : arme de mêlée, manipulateur ou corps adapté. Sans technique apprise, une attaque ordinaire reste possible.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| MEL-01 | 1 | Frappe puissante | Renforce une attaque de mêlée, avec une récupération plus longue. | Arme/actuateurs adaptés ; davantage de temps laissé aux adversaires. Une cible peut exploiter la récupération. |
| MEL-02 | 1 | Frappe précise | Prépare une attaque pour améliorer les chances de toucher une cible difficile. | Préparation sur une cible perçue ; perte du contact ou changement de cible fait perdre la préparation. |
| MEL-03 | 2 | Repoussement | Inflige peu de dégâts et tente de déplacer la cible d'une case. | Comparaison de force utile, masse et ancrage ; destination légale. Un obstacle bloque le déplacement ; une collision dommageable reste conditionnelle. |
| MEL-04 | 2 | Parade | Consacre une action à réduire les dégâts de la prochaine attaque de mêlée pendant la fenêtre de garde. | Équipement capable de parer ; la garde ne protège pas automatiquement des explosions ou des programmes. L'adversaire peut attendre, reculer ou utiliser une autre attaque. |
| MEL-05 | 3 | Balayage | Frappe plusieurs cases adjacentes dans un arc, avec des dégâts réduits par cible. | Arme et espace adaptés ; risque pour les alliés et consommation accrue. Les obstacles coupent les trajectoires. |
| MEL-06 | 3 | Riposte | Améliore MEL-04 : après une parade réussie, produit une contre-attaque si l'ennemi reste à portée. | Demande Parade et utilise le droit de réaction. Aucun enchaînement de ripostes entre acteurs. |
| MEL-07 | 4 | Brise-armure | Sacrifie une partie des dégâts immédiats pour fragiliser temporairement le blindage. | Cible blindée et arme adaptée ; effet borné, sans cumul infini. Une cible sans blindage offre peu d'intérêt. |
| MEL-08 | 4 | Entrave | Frappe une fonction locomotrice pour gêner brièvement le déplacement. | Cible mobile compatible et résistance applicable ; pas de blocage permanent par répétition. |
| MEL-09 | 5 | Écrasement | Exploite une immobilisation ou une entrave pour infliger une frappe particulièrement lourde. | La condition peut venir d'un allié, du terrain ou de MEL-08 ; cette dernière n'est pas un prérequis d'apprentissage. Récupération et matériel limitent l'emploi. |
| MEL-10 | 5 | Interception | Prépare une garde permettant de frapper un adversaire qui quitte le contact. | Utilise une réaction ; n'annule pas automatiquement le mouvement. Proposition : se déclenche sur un retrait volontaire, pas sur un Repoussement allié. |

Frontière : les charges, franchissements et déplacements volontaires appartiennent à Manœuvre. Le déplacement imposé par un coup appartient au combat rapproché.

Exemple : Repoussement peut envoyer un adversaire dans un Champ de saturation. La case d'arrivée et la propagation restent légales ; ce déplacement forcé ne déclenche pas une Interception supplémentaire.

### 4.2. Tir

Statut : progression et noms validés ; précisions de résolution retenues comme base des essais.

Rôle : précision, rafales et contrôle d'un passage. Primaire principale : Coordination ; Perception intervient dans le ciblage et l'exploitation des indices. Puissance peut aider certains équipements lourds sans devenir une exigence universelle. Sans technique apprise, le tir ordinaire et les modes natifs de l'arme restent disponibles.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| TIR-01 | 1 | Tir visé | Consacre une action à préparer le prochain tir sur une cible visible pour améliorer sa précision. | Bouger, changer de cible ou perdre le contact annule la préparation. La cible peut chercher un couvert. |
| TIR-02 | 1 | Rafale contrôlée | Limite les projectiles d'une rafale pour réduire la dispersion. | Arme dotée d'un mode compatible ; moins de volume de feu. Ne crée pas de mode automatique sur une arme à un coup. |
| TIR-03 | 2 | Tir de suppression | Gêne brièvement la précision d'une cible exposée par un feu soutenu. | Dépense accrue ; conditions d'exposition et de compatibilité. Ne force pas une fuite ; certains châssis résistent mieux à la gêne. |
| TIR-04 | 2 | Surveillance | Réserve un tir contre un ennemi qui s'expose dans un passage désigné avant la prochaine action normale. | Préparation d'une action, cible alors perçue, trajectoire valide et réaction disponible. Les ressources du tir sont consommées au déclenchement. |
| TIR-05 | 3 | Tir localisé | Vise un élément identifié pour tenter d'endommager une fonction particulière. | Pénalité de précision ; exige une simulation des composants ou parties du corps. Une touche n'est pas une destruction automatique. |
| TIR-06 | 3 | Rafale répartie | Répartit les projectiles disponibles entre plusieurs cibles proches et visibles. | Chaque trajectoire est validée ; le total reste celui de la rafale. Le joueur renonce à concentrer tous les tirs sur une cible. |
| TIR-07 | 4 | Visée persistante | Améliore TIR-01 : conserve une partie de la préparation après le tir sur la même cible. | Demande Tir visé ; rester immobile et conserver le contact. Le bonus ne croît pas indéfiniment. |
| TIR-08 | 4 | Surveillance étendue | Améliore TIR-04 : couvre un secteur plus large. | Demande Surveillance ; aucune augmentation du champ de vision, de la portée physique ou du nombre de réactions. |
| TIR-09 | 5 | Tir de rupture | Prépare un tir exploitant une faiblesse identifiée pour contourner une partie du blindage. | Arme adaptée et faiblesse connue par observation, équipement, renseignement ou allié. Ne traverse pas toute protection et n'impose pas une seconde compétence obligatoire. |
| TIR-10 | 5 | Barrage | Maintient un feu sur une petite zone, avec dégâts répartis et suppression. | Arme soutenant ce tir, forte consommation et immobilisation pendant l'action. Les couverts et le risque allié restent applicables. |

Frontière : les attaques de zone natives d'une arme restent disponibles. Démolition apporte des procédés de mise à feu et de destruction ; elle n'est pas automatiquement ajoutée à tous les jets d'un lanceur explosif. Les règles doivent éviter de compter deux fois la même maîtrise.

Exemple : Surveillance contrôle l'entrée d'une salle. Un ennemi peut rester à couvert, emprunter un autre accès ou provoquer le déclenchement avec une unité moins précieuse.

### 4.3. Démolition

Statut : direction retenue après relecture ; chiffres à tester et effondrements conditionnés à leur système dédié.

Rôle : explosifs, mines, brèches et préparation du terrain. Coordination aide à placer ou lancer ; Perception à lire une structure ; Traitement à programmer un dispositif. Aucun attribut unique ne pilote toutes ces opérations. Sans technique apprise, utiliser une grenade ou un explosif simple selon son mode natif reste possible.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| DEM-01 | 1 | Lancer ajusté | Prépare un lancer d'explosif pour réduire l'écart à la case visée. | Consomme du temps de préparation ; trajectoire et explosif réels. Une cible mobile peut quitter la zone. |
| DEM-02 | 1 | Désamorçage | Neutralise le déclencheur d'un piège identifié après une intervention. | Outils, proximité et temps ; conditions d'échec annoncées. Pas de tests gratuits répétés pour découvrir une mine cachée. |
| DEM-03 | 2 | Charge de brèche | Place une charge adaptée pour ouvrir un mur, une porte ou un couvert destructible. | Consomme une charge ; matériau et résistance comptent. La mise à feu crée du bruit, des débris et parfois un danger de l'autre côté. |
| DEM-04 | 2 | Mine de proximité | Installe et règle un dispositif déclenché par une présence compatible avec son capteur. | Mine consommée à la pose ; délai d'armement lisible. Un filtre d'identification exige le matériel correspondant et n'accède pas aux intentions cachées. |
| DEM-05 | 3 | Explosion dirigée | Configure un explosif compatible pour concentrer son effet dans une direction. | Forme de charge et préparation adaptées ; couverture réduite dans les autres directions. Ne transforme pas toute grenade en explosion sélective. |
| DEM-06 | 3 | Déclenchement distant | Déclenche un dispositif connu doté d'un récepteur compatible. | Liaison valide et action de commande ; brouillage ou rupture de liaison peuvent l'empêcher. Ne révèle pas les occupants autour du dispositif. |
| DEM-07 | 4 | Récupération de charges | Améliore DEM-02 pour récupérer un piège neutralisé sans sacrifier nécessairement son mécanisme. | Demande Désamorçage ; outil et temps. Une charge dépensée ne réapparaît pas et les pièces récupérées restent finies. |
| DEM-08 | 4 | Mise à feu séquencée | Programme plusieurs charges pour se déclencher selon des délais fixés à l'avance. | Consomme chaque charge et le temps de programmation. Le programme n'adapte pas ses choix à des ennemis non détectés. |
| DEM-09 | 5 | Effondrement contrôlé | Exploite un support identifié pour provoquer une chute locale de structure. | Demande un système de supports et une zone réellement fragilisable. Préparation importante, zone de danger annoncée, pas d'effondrement arbitraire du niveau. |
| DEM-10 | 5 | Détonation combinée | Couple deux charges compatibles : la première ouvre ou affaiblit un obstacle, la seconde agit après cette transformation. | Deux charges, montage et délais ; la seconde propagation est recalculée sur le terrain effectivement modifié. L'adversaire peut interrompre la préparation. |

Frontière : Ingénierie produit ou répare un mécanisme ; Démolition choisit comment le poser et le faire exploser. Furtivité peut le dissimuler ; Reconnaissance peut identifier les matériaux. Ces synergies n'imposent pas trois achats pour poser une mine ordinaire.

Exemple : ouvrir une issue latérale avec une Charge de brèche peut éviter une salle défendue. Le coût est matériel et sonore, et le passage peut aussi être emprunté par les ennemis.

## 5. Catégorie Mobilité et discrétion

### 5.1. Manœuvre

Statut : direction retenue après relecture ; temps, coûts et déplacements à éprouver en jeu.

Rôle : se déplacer sous pression, franchir un obstacle et choisir sa position. Coordination est centrale ; Puissance, masse, ancrage et propulsion interviennent selon le geste. Résilience aide au maintien opérationnel sous perturbation, sans rendre le déplacement invulnérable. Sans technique apprise, déplacement, retraite et franchissements ordinaires autorisés par le corps restent possibles.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| MAN-01 | 1 | Pas de dégagement | Effectue un pas prudent réduisant la vulnérabilité à une interception de mêlée lors du retrait. La consigne peut être maintenue pour les pas de retraite suivants sans achat supplémentaire. | Chaque pas consomme son action et exige une destination accessible. Attaquer ou changer de posture rompt la consigne, sans cumul du bonus. Ne neutralise pas les zones dangereuses ni les tirs couvrant l'arrivée. |
| MAN-02 | 1 | Appui stable | Prépare un ancrage temporaire pour mieux résister aux poussées et déséquilibres. | Sol et corps compatibles ; rester en place. Consomme du temps et ne protège pas des autres dégâts. |
| MAN-03 | 2 | Franchissement | Traverse un obstacle bas, un débris ou un petit intervalle compatible avec le châssis. | Départ, passage et arrivée vérifiés ; coût en temps/énergie. Aucun passage à travers un mur ou une fosse trop large. |
| MAN-04 | 2 | Charge | Avance en ligne puis effectue une attaque ordinaire bénéficiant de l'élan. | Cible perçue, ligne praticable et distance suffisante ; coût global supérieur à une action simple. Chaque case peut déclencher un danger ou une surveillance. |
| MAN-05 | 3 | Esquive préparée | Prépare un déplacement de secours face à une attaque perçue compatible. | Case de repli choisie à la préparation et encore libre au déclenchement ; utilise la réaction commune. L'attaque ou sa zone peut aussi atteindre cette case. |
| MAN-06 | 3 | Poussée des propulseurs | Augmente ponctuellement la distance parcourue par une manœuvre. | Propulsion compatible, énergie et chaleur ; déplacement résolu case par case, sans bonus permanent à toutes les actions. |
| MAN-08 | 4 | Inertie maîtrisée | Améliore MAN-04 : permet d'arrêter volontairement une charge avant le contact ou de mieux supporter sa récupération. | Demande Charge et les moyens de freiner ; l'énergie déjà dépensée ne revient pas. Une collision inattendue conserve ses conséquences. |
| MAN-09 | 5 | Percée | Force un passage à travers la position d'un adversaire déplaçable en le repoussant vers une case valide. | Masse, force et espace nécessaires ; coût important. Ne traverse pas plusieurs corps, un mur ou une unité ancrée sans résolution physique. |
| MAN-10 | 5 | Extraction | Aide un allié adjacent à quitter avec soi une position dangereuse. | Allié coopératif ou transportable, capacité de traction et trajet praticable ; déplacement coûteux. Ne donne pas une attaque gratuite à l'unité déplacée. |

Frontière : cette compétence déplace les corps. Combat rapproché décide du coup et des techniques martiales ; Furtivité traite la signature. Une charge peut employer l'arme équipée sans obliger à apprendre une technique de mêlée supplémentaire.

Exemple : Esquive préparée peut éviter un projectile si une case sûre existe. Face à une explosion couvrant aussi cette case, elle ne constitue pas une immunité. Le même personnage renonce alors à sa réaction de Surveillance pendant la fenêtre concernée.

### 5.2. Furtivité

Statut : direction retenue après relecture ; détection, signatures et comportements de recherche à éprouver en jeu.

Rôle : choisir ses engagements, exploiter les couverts et rompre une poursuite. Coordination et les signatures du châssis dominent ; les équipements définissent les émissions, la chaleur et les possibilités de camouflage. Sans technique apprise, profiter d'un mur, d'un couvert ou de l'absence de témoin fonctionne déjà.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| FUR-01 | 1 | Pas feutrés | Réduit le bruit des déplacements en adoptant une marche plus lente. | Corps et surface comptent ; baisse de vitesse de déplacement. Un capteur visuel peut toujours détecter le personnage. |
| FUR-02 | 1 | Approche couverte | Exploite mieux les occultations partielles pendant un déplacement proche d'un couvert. | Nécessite un couvert réel ; aucun avantage équivalent en terrain ouvert. Les angles observés par les ennemis restent déterminants. |
| FUR-03 | 2 | Silence des émissions | Réduit la signature active en mettant temporairement en veille certains équipements émetteurs. | Les fonctions mises en veille deviennent indisponibles. Ne supprime ni la silhouette, ni la chaleur, ni la perception passive encore disponible. |
| FUR-04 | 2 | Profil réduit | Adopte une posture exploitant la silhouette du châssis pour mieux se dissimuler derrière des couverts bas. | Châssis compatible, déplacement plus lent et restrictions sur certains équipements. Les capteurs adaptés restent efficaces. |
| FUR-05 | 3 | Embuscade | Prépare une première attaque favorisée contre une cible qui n'a pas localisé le personnage. | Préparation, cible perçue et condition de non-détection ; l'attaque produit ses signatures normales. Aucun bonus maintenu pendant tout le combat. |
| FUR-06 | 3 | Leurre sonore | Place ou lance une source de bruit pour provoquer une investigation. | Objet ou dispositif réel et trajectoire valide ; le son suit sa propagation. Un ennemi déjà en contact peut ignorer le leurre. |
| FUR-07 | 4 | Rupture de piste | Après avoir rompu la ligne de vue, réduit les indices produits pendant un court déplacement de fuite. | Demande d'abord de quitter la perception adverse. Les ennemis conservent la dernière position connue et continuent à chercher. |
| FUR-08 | 4 | Dissimulation des dispositifs | Camoufle visuellement une mine, une balise ou un petit équipement posé. | Matériel et temps ; ne supprime pas ses émissions actives. Un inspecteur, un choc ou un capteur adapté peut le repérer. |
| FUR-09 | 5 | Camouflage actif | Pilote un module de camouflage pour réduire fortement une signature déterminée pendant une durée limitée. | Module spécialisé, énergie et chaleur ; le canal masqué est explicite. Attaquer ou surcharger certains équipements peut interrompre le camouflage ; les autres capteurs gardent leur rôle. |
| FUR-10 | 5 | Neutralisation discrète | Améliore FUR-05 : exécute une attaque de contact préparée visant à neutraliser une cible vulnérable avec une faible signature. | Demande Embuscade ; compatibilité, vulnérabilité et dégâts nécessaires. Pas d'élimination automatique d'une cible intacte ; survivants et témoins peuvent alerter. |

Frontière : Guerre électronique brouille ou trompe activement des systèmes adverses. Furtivité réduit les signatures et exploite le contexte. Intrusion obtient des droits ou falsifie des données. Réduire une émission ne signifie pas effacer la mémoire d'un témoin.

Exemple : un Leurre sonore attire une patrouille dans une autre pièce. Le personnage contourne sa dernière position connue, puis doit encore éviter la caméra qui observe le passage. Le leurre n'a pas déplacé la connaissance de tous les ennemis de la carte.

## 6. Catégorie Perception

### 6.1. Reconnaissance

Statut : catalogue simplifié après accord, incluant la fusion avec Analyse tactique. Sept techniques sont retenues ; l'historique des trois entrées sorties du catalogue figure en section 6.2.

Rôle : transformer des observations en décisions, avant ou pendant un engagement. Perception est principale ; Traitement, capteurs et bases de données interviennent dans l'interprétation. Sans technique apprise, le joueur voit les informations évidentes : cible perçue, attitude connue, équipement clairement visible et annonce compréhensible des dangers.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| REC-01 | 1 | Analyse de cible | Consacre une action à examiner une entité perçue : état, matériel observable et résistances ou vulnérabilités identifiables. Ces informations aident le personnage ou ses alliés à choisir une action adaptée. | Capteurs et données disponibles déterminent ce qui peut être identifié. Aucun bonus d'attaque automatique, inventaire fermé révélé ou seconde analyse obligatoire pour obtenir les résistances accessibles. |
| REC-02 | 1 | Lecture de traces | Détecte ou interprète les indices locaux laissés par certains déplacements : empreintes, chenilles, huile, direction et ancienneté approximative. | Traces réellement enregistrées selon le corps et le terrain, avec durée et nombre par case bornés. Les indices évidents restent accessibles à tous ; aucune position actuelle garantie de leur auteur ni mise à jour visible hors perception. |
| REC-03 | 2 | Inspection minutieuse | Recherche dans les cases observables des pièges, caches, commandes dissimulées ou passages secrets réellement présents dans le niveau. | Temps d'inspection et indices accessibles ; découverte distincte de l'ouverture ou du désamorçage. Moyens alternatifs pour les accès essentiels ; aucune relance gratuite à conditions identiques. |
| REC-04 | 2 | Repérage des parois fragiles | Identifie les matériaux, cloisons, portes ou portions de mur observables présentant une résistance plus faible ou un état dégradé. | Temps d'examen et propriétés réelles du décor. N'exige aucun calcul de stabilité du bâtiment ; les supports et effondrements restent une extension distincte. |
| REC-05 | 3 | Profil de menace | Analyse les moyens offensifs et défensifs observables d'un adversaire pour préciser ses possibilités. | Demande des observations ou des données crédibles ; n'annonce pas la prochaine décision de l'IA comme une certitude. |
| REC-09 | 3 | Analyse multiple | Améliore REC-01 : applique une même action d'analyse à plusieurs cibles simultanément perçues, avec un nombre limité. | Demande Analyse de cible ; mêmes restrictions de connaissance pour chaque cible. Économise des actions d'analyse sans révéler les ennemis cachés ni supposer des équipements identiques. |
| REC-08 | 4 | Diagnostic énergétique | Examine l'alimentation, la chaleur, les propriétés conductrices connues ou le stockage d'énergie compatible d'une machine perçue pour guider l'emploi de Surchauffe, Implosion ou d'autres outils. | Capteurs adaptés et états effectivement simulés. Aucun relevé des installations cachées ni simulation continue du réseau exigée ; l'entrée devient disponible avec les systèmes qu'elle analyse. |

Les identifiants sont conservés ; les lignes suivent le niveau minimal plutôt que l'ordre des anciens codes. Les informations physiques et résistances relèvent d'Analyse de cible ; Profil de menace précise les possibilités de combat observables ; Diagnostic énergétique traite les états d'énergie et de chaleur. L'analyse donne de l'information, pas un bonus abstrait supplémentaire à activer avant le tir.

| Niveau du personnage | Nouvelles possibilités | Exemple de choix à ce niveau |
|---|---|---|
| 1 | Analyse de cible, Lecture de traces. | Une technique accessible. |
| 2 | Inspection minutieuse, Repérage des parois fragiles. | Une technique accessible, y compris parmi celles ouvertes au niveau 1. |
| 3 | Profil de menace, Analyse multiple. | Une technique accessible ; Analyse multiple demande REC-01. |
| 4 | Diagnostic énergétique, une fois ses états disponibles. | Une technique accessible parmi celles ouvertes jusqu'au niveau 4. |
| 5 | Aucune technique exclusive supplémentaire. | Les techniques antérieures encore inconnues restent apprenables. |

Ainsi, un spécialiste peut choisir Analyse de cible, Lecture de traces, Analyse multiple, Diagnostic énergétique puis Inspection minutieuse. Si le diagnostic énergétique n'est pas encore disponible dans une version du jeu, les autres entrées restent accessibles selon leurs propres conditions. Le coût croissant de la spécialisation reste à éprouver puisqu'il complète le répertoire au lieu d'ouvrir artificiellement un pouvoir de maîtrise.

Frontière : l'inspection de base et les informations nécessaires à une mort compréhensible ne sont pas des récompenses de haut niveau. Une connaissance détaillée crée un avantage préparé. Reconnaissance n'exécute ni le piratage ni la démolition qu'elle peut aider à choisir.

Exemple : Repérage des parois fragiles révèle qu'une cloison corrodée est moins résistante que les murs renforcés voisins. Le personnage peut y préparer une brèche pour contourner une salle défendue. Le repérage décrit la cloison existante et ne fait pas apparaître une faiblesse sur commande.

Point de vigilance : cette compétence doit rester utile après plusieurs parties, lorsque le joueur connaît déjà les espèces. Les équipements variables, états locaux, traces et structures propres à la carte doivent fournir une information nouvelle. Elle ne doit pas se réduire à acheter l'accès à un bestiaire que le joueur connaît de mémoire.

### 6.2. Historique des entrées de Reconnaissance sorties du catalogue

Ces identifiants sont réservés pour retrouver les commentaires précédents. Ils ne sont plus proposés à l'apprentissage, ne comptent pas dans les 94 entrées principales actuelles et ne sont plus des prérequis actifs.

| Ancien code | Statut | Décision appliquée |
|---|---|---|
| REC-06 | Différée | Discrimination des signaux reste en attente d'un système de faux contacts réellement utile et éprouvé. Aucun remplacement ajouté au catalogue. |
| REC-07 | Fusionnée | Analyse de faille est intégrée à REC-01, Analyse de cible. Aucun second achat ni action d'analyse supplémentaire pour cette ancienne entrée. |
| REC-10 | Retirée de la première version | Exploitation tactique est supprimée sous sa forme de bonus préparé, redondante avec l'information obtenue et les techniques d'attaque. Aucun remplacement ajouté. |

## 7. Catégorie Ingénierie

### 7.1. Ingénierie

Statut : direction retenue après relecture, incluant la fusion de Maintenance et Ingénierie modulaire ; économie des réparations à éprouver en jeu.

Rôle : préserver et adapter les moyens matériels. Coordination compte dans la manipulation ; Traitement dans le diagnostic et les procédures. La quantité de matière, les outils et le lieu d'intervention restent déterminants. Sans technique apprise, employer un consommable de réparation, installer un module compatible et effectuer une récupération ordinaire restent possibles.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| ING-01 | 1 | Réparation ciblée | Oriente une intervention vers une fonction ou un composant endommagé précis. | Pièces compatibles, outils et temps ; ne restaure pas gratuitement la durabilité. La réparation ordinaire doit rester viable sans cet achat. |
| ING-02 | 1 | Démontage soigneux | Privilégie la conservation d'un composant choisi lors de la récupération d'une carcasse. | Intervention plus longue ; choix entre pièce conservée et ressources extraites du même objet. Les dommages subis limitent le résultat. |
| ING-03 | 2 | Diagnostic de panne | Identifie la cause d'un dysfonctionnement matériel et la procédure nécessaire pour le traiter. | Outils et examen ; peut révéler un besoin de pièce plutôt qu'une solution immédiate. Une infection logicielle relève de la purge ou de la sécurité numérique. |
| ING-04 | 2 | Réglage spécialisé | Modifie un paramètre d'un module dans les tolérances prévues, avec une contrepartie. | Atelier ou kit adapté ; un réglage par emplacement de modification. Exemples : économie contre puissance, stabilité contre cadence. Aucun cumul de réglages identiques. |
| ING-05 | 3 | Réparation d'urgence | Améliore ING-01 pour rétablir plus vite une fonction critique en pleine situation dangereuse. | Demande Réparation ciblée ; rendement matériel inférieur ou réparation partielle. L'intervention consomme du temps et peut être interrompue. |
| ING-06 | 3 | Surcadencement | Augmente temporairement la sortie d'un module compatible. | Énergie, chaleur et risque de dégradation annoncés ; bornes matérielles. Ne relève pas le plafond primaire de 10 et n'accélère pas toutes les actions du personnage. |
| ING-07 | 4 | Dérivation | Maintient une fonction endommagée en redirigeant une ressource depuis un autre sous-système réel. | Compatibilité et diagnostic obtenu par une compétence, un outil ou un atelier ; la fonction sacrifiée se dégrade ou s'arrête. Ne remplace pas une pièce physiquement absente par une ressource imaginaire. |
| ING-08 | 4 | Reconditionnement | Restaure durablement un équipement récupérable dans de meilleures conditions qu'une réparation de terrain. | Atelier, pièces et durée importante ; limites de l'objet. Ne garantit pas qu'un composant détruit soit réparable. |
| ING-09 | 5 | Assemblage de terrain | Assemble un dispositif provisoire à partir d'un plan connu et de pièces disponibles. | Plan, composants et outils ; le catalogue des objets constructibles est borné. Un drone ou une tourelle assemblée garde ses coûts de contrôle et sa durée de vie. |
| ING-10 | 5 | Surcadencement régulé | Améliore ING-06 pour choisir un régime renforcé plus stable, au prix d'une partie du gain maximal. | Demande Surcadencement ; chaleur et consommation persistent. Aucun fonctionnement renforcé infini sans coût ni immunité aux surcharges ennemies. |

Frontière : les explosifs sont mis en œuvre par Démolition ; la commande avancée des unités relève de Contrôle de drones ; les intrusions et infections restent numériques. Les outils ordinaires ne sont pas rendus inutilisables par l'absence d'Ingénierie.

Exemple : Dérivation coupe temporairement un capteur secondaire pour alimenter une propulsion endommagée. Le personnage gagne une possibilité de fuite et accepte une perte d'information réelle pendant ce fonctionnement dégradé.

Point de vigilance : si tous les builds prennent Réparation ciblée ou Démontage soigneux au premier niveau pour survivre économiquement, la compétence devient une taxe. Il faudra alors améliorer les solutions ordinaires ou rendre les avantages plus situationnels, plutôt que réduire artificiellement l'accès aux réparations.

## 8. Catégorie Systèmes numériques

### 8.1. Intrusion

Statut : direction retenue après relecture ; falsification et verrouillage précisés et conditionnés à des comportements de sécurité actifs.

Rôle : obtenir des accès, récupérer des informations et détourner les fonctions d'un système. Traitement domine ; interfaces, programmes, droits déjà obtenus et sécurité de la cible déterminent les possibilités. Sans technique apprise, utiliser une console autorisée, une clé ou une fonction native d'un outil reste possible.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| INT-01 | 1 | Sondage d'accès | Examine les interfaces et protections accessibles d'un système pour préciser les actions envisageables. | Interface compatible et temps ; certains systèmes détectent la sonde. Ne révèle pas tout le réseau ni tous les mots de passe. |
| INT-02 | 1 | Ouverture forcée | Tente de commander un verrou électronique local sans autorisation normale. | Accès technique et sécurité franchissable ; temps, risque de trace et d'alarme. Un verrou purement mécanique exige une autre solution. |
| INT-03 | 2 | Extraction de données | Récupère des plans, journaux ou informations accessibles depuis un système compromis. | Temps d'extraction et accès valable. Les informations portent leur date et leur provenance ; un journal ancien n'est pas un radar actuel. |
| INT-04 | 2 | Usurpation locale | Utilise un identifiant obtenu pour présenter une autorisation crédible à un système déterminé. | Identifiant valide ou données suffisantes, interface et durée limitée. Ne change pas une réputation ni la mémoire des témoins. |
| INT-05 | 3 | Détournement | Modifie temporairement une consigne simple d'un dispositif : orientation, éclairage, ouverture ou cible autorisée. | Accès obtenu, canal de contrôle et action de commande. Une tourelle conserve son propre rythme et ses ressources ; pas de tir gratuit à chaque ordre. |
| INT-06 | 3 | Neutralisation de routine | Suspend brièvement une fonction automatique précise d'un système compromis. | Fonction identifiable et accès ; durée bornée. Ne met pas automatiquement un acteur entier hors jeu à chaque répétition. |
| INT-07 | 4 | Porte dérobée | Conserve un accès discret à un système déjà compromis pour faciliter une intervention ultérieure. | Accès initial, temps d'installation et capacité limitée d'accès maintenus. Une inspection, une réinitialisation ou un changement de couche peut invalider cet accès. |
| INT-08 | 4 | Falsification de registre | Modifie un événement suspect enregistré sur un système compromis avant qu'une inspection ou transmission de sécurité puisse l'exploiter. | Nécessite une sécurité qui consulte réellement ces événements ; sinon l'entrée est reportée. Temps, accès et données identifiées ; alarmes déjà transmises, copies et souvenirs des témoins subsistent. |
| INT-09 | 5 | Détournement de sous-réseau | Coordonne un ensemble limité de dispositifs effectivement rattachés à un point d'accès compromis. | Topologie connue, canaux et droits suffisants ; préparation importante. Ne prend pas le contrôle de toutes les unités d'un étage et ne donne pas de vision supplémentaire. |
| INT-10 | 5 | Verrouillage de contrôle | Empêche temporairement des contrôleurs adverses de refermer une porte ouverte, réactiver une caméra neutralisée ou reprendre une tourelle détournée dans le périmètre piraté. | Nécessite des adversaires qui cherchent effectivement à reprendre le contrôle ; sinon l'entrée est reportée. Durée et périmètre bornés, accès et canal maintenus ; coupure, intervention physique ou réinitialisation peuvent y mettre fin. |

Frontière : Intrusion obtient et transforme les accès. Guerre électronique produit les attaques, infections et contre-mesures de combat. Contrôle de drones améliore la conduite de plusieurs unités. Une prise de contrôle simple ne demande pas forcément les trois compétences ; une flotte durable exige ses limites matérielles propres.

Exemple : un pirate détourne l'éclairage d'un couloir pour faciliter son passage ou celui d'un PNJ allié. Avant l'inspection du terminal par la sécurité, il falsifie l'accès suspect enregistré. Une alarme déjà reçue ailleurs et le souvenir d'un gardien témoin restent des preuves distinctes.

INT-08 et INT-10 restent dans le catalogue cible, avec leurs dépendances explicites. Elles ne sont pas proposées à l'achat dans une version dépourvue des comportements de sécurité correspondants. La sécurité exploite des événements structurés et des règles locales ; aucun modèle de langage n'a à interpréter un journal en texte libre.

Point de vigilance : un échec valide consomme son temps et peut laisser une trace. La trace reste locale ou transmise par un système réel ; ce n'est pas une connaissance immédiate de tous les ennemis. Une porte ne verse pas de l'XP à chaque réouverture.

### 8.2. Guerre électronique

Statut : progression, effets centraux et six noms offensifs validés. Brouillage et Purge font partie de la progression acceptée. Les variantes constituent la base retenue ; coûts et conditions d'apprentissage restent à éprouver.

Rôle : dégâts électroniques, attaques logicielles, propagation, contrôle de zone et contre-mesures. Traitement améliore l'exploitation des procédures ; Perception aide au ciblage. La puissance d'émission, les réserves et la dissipation viennent du matériel. Un programme offensif logiciel exige une cible compatible ; une décharge physique exige un émetteur.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| GEL-01 | 1 | Surcharge | Produit une impulsion de dégâts électroniques autour du personnage, avec une perturbation éventuelle selon l'équipement. | Émetteur adapté, courte portée et forte dépense ; murs et portes fermées bloquent l'émission. Les alliés exposés peuvent être touchés. |
| GEL-02 | 1 | Surchauffe | Sabote la gestion thermique d'une machine : la température augmente, puis des dégâts surviennent si les limites sont dépassées. | Cible compatible et attaque logicielle réussie ; pas d'achat obligatoire en Intrusion pour sa procédure standard. Refroidissement, purge ou rupture du processus peuvent répondre. |
| GEL-03 | 2 | Brouillage | Perturbe temporairement un type de capteur ou de liaison dans une zone accessible à l'émetteur. | Matériel et énergie ; canal visé explicite. Les capteurs non concernés et les observations déjà faites restent utiles. |
| GEL-04 | 2 | Purge | Tente d'éliminer ou d'isoler un programme hostile actif sur soi ou un système allié accessible. | Temps et outils numériques ; ne restaure pas les dégâts matériels déjà subis. Protection de base et autres solutions existent pour les non-spécialistes. |
| GEL-05 | 3 | Surcharge en cascade | Frappe une cible puis fait rebondir la décharge vers un nombre limité de cibles proches, avec perte de puissance. | Émetteur adapté, cibles perçues et trajet non obstrué à chaque saut. Une cible ne peut pas être touchée indéfiniment par retour de chaîne. |
| GEL-06 | 3 | Infection | Introduit un sabotage progressif pouvant se transmettre à quelques machines proches et compatibles. | Attaque initiale, délais et durée/propagation bornés ; isolement et purge possibles. La propagation locale ne révèle pas les positions cachées et ne franchit pas automatiquement les murs. |
| GEL-07 | 4 | Champ de saturation | Pose une balise créant une zone persistante de dégâts électroniques. | Balise consommée ou déployée depuis une réserve matérielle, temps de pose, énergie et durée limitée. La balise peut être détruite et la zone évitée. |
| GEL-08 | 5 | Implosion | Compromet le stockage d'énergie d'une cible adaptée pour déclencher la destruction locale prévue, avec dégâts autour de la cible. | Stockage réel compatible, préparation et signe avant-coureur. La cible peut être isolée, purgée ou éloignée ; le butin et les alliés peuvent subir les conséquences. |

Implosion conserve l'effet de destruction locale accepté avant le changement de nom. Le nom n'ajoute pas à lui seul une attraction gravitationnelle des unités voisines.

#### Variantes de techniques

Chaque variante ci-dessous représente un apprentissage supplémentaire, demande sa technique et n'est pas acquise automatiquement. Les noms et niveaux minimaux sont conservés comme base de travail. Par défaut, une seule variante d'une même technique est appliquée à une exécution ; acheter les deux demande deux choix et permet de choisir le profil utilisé.

| Code | Niveau | Variante | Prérequis | Transformation et compromis |
|---|---:|---|---|---|
| GEL-V01 | 2 | Impulsion directionnelle | GEL-01 | Oriente Surcharge en cône ; couvre moins de directions pour mieux choisir la zone exposée. Ne rajoute pas gratuitement de portée. |
| GEL-V02 | 2 | Filtrage allié | GEL-01 | Améliore le filtrage pour épargner les alliés identifiés par le matériel ; complexité et coût accrus, sans lecture des intentions inconnues. |
| GEL-V03 | 2 | Montée accélérée | GEL-02 | Accélère la montée thermique de Surchauffe, avec une sollicitation plus forte ou une durée plus courte. |
| GEL-V04 | 2 | Inhibition prolongée | GEL-02 | Prolonge le sabotage thermique, avec une montée initiale moins forte et davantage de temps laissé à une purge. |
| GEL-V05 | 4 | Rebond supplémentaire | GEL-05 | Permet un rebond de plus, au prix d'une dépense supplémentaire et d'une puissance plus faible en fin de chaîne. |
| GEL-V06 | 4 | Décharge soutenue | GEL-05 | Conserve davantage de puissance entre les rebonds, avec un coût énergétique supérieur. Les limites de cibles demeurent. |
| GEL-V07 | 4 | Infection contagieuse | GEL-06 | Favorise la transmission, au prix de dégâts plus faibles par hôte. Durée et nombre de transmissions restent plafonnés. |
| GEL-V08 | 4 | Infection concentrée | GEL-06 | Renforce le sabotage de la cible initiale en réduisant ou supprimant sa propagation. |
| GEL-V09 | 5 | Saturation persistante | GEL-07 | Prolonge le champ, avec une dépense totale supérieure ou une intensité réduite. La balise reste destructible. |
| GEL-V10 | 5 | Activation manuelle | GEL-07 | Permet de poser la balise inactive puis de lancer le champ au moment choisi ; exige une commande, une liaison valide et laisse la balise vulnérable avant activation. |

Guerre électronique dispose ici de davantage d'options décrites que les autres compétences, car ses variantes avaient déjà été discutées. Des variantes supplémentaires pour les autres compétences pourront être conçues après avis ; leur nombre ne doit pas être gonflé pour une simple symétrie.

Exemple : Surchauffe menace un adversaire utilisant une arme énergivore ; celui-ci peut réduire sa cadence pour refroidir. Surcharge en cascade récompense un regroupement, tandis qu'un Champ de saturation prépare une zone où Repoussement devient intéressant.

### 8.3. Contrôle de drones

Statut : direction retenue après relecture ; routines, nombre d'unités et limites de perception à éprouver en jeu.

Rôle : obtenir des comportements utiles de machines alliées sans créer une multitude d'actions gratuites. Traitement intervient dans la complexité des ordres ; les contrôleurs, canaux et réserves limitent le nombre et l'activité des unités. Sans technique apprise, un drone possédé et équipé de son contrôleur accepte des ordres simples tels que suivre, attendre, attaquer une cible connue ou revenir par un trajet autorisé.

| Code | Niveau | Technique | Fonctionnement | Conditions, coût et réponse possible |
|---|---:|---|---|---|
| DRN-01 | 1 | Drone spectral | Manifeste un drone utilitaire de base sur une case adjacente libre. Sa doctrine initiale est de suivre le joueur et de tenter d'attaquer la cible que celui-ci vient d'attaquer. | 10 E, +3 H, 1 B réservée par le drone, recharge 3 ; une seule manifestation active. À batterie vide, la manifestation se dissipe, libère sa bande passante et redevient disponible après la recharge ordinaire. Le drone possède ses propres actions, capteurs, composants et position : son tir d'assistance exige donc perception, portée et ligne de tir. |
| DRN-02 | 1 | Patrouille bornée | Programme un trajet court dans une zone connue, avec une condition d'arrêt ou de retour. | Temps de programmation et navigation compatible ; obstacles nouveaux ou danger peuvent interrompre la routine. |
| DRN-03 | 2 | Leurre mobile | Coordonne un drone équipé pour produire une signature attirant l'attention depuis une position choisie. | Dispositif de leurre, énergie et exposition de l'unité ; l'ennemi peut l'ignorer ou la détruire. |
| DRN-04 | 2 | Collecte ciblée | Envoie une unité dotée d'un manipulateur récupérer un objet connu et le rapporter. | Capacité de charge, trajet et ordres ; l'objet peut avoir disparu. Aucun inventaire distant instantané ni déplacement gratuit des ressources. |
| DRN-05 | 3 | Tirs coordonnés | Désigne une cible observée pour que plusieurs unités compatibles concentrent leurs prochaines attaques ordinaires. | Canaux, ordres, lignes de tir et ressources propres ; ne déclenche pas une salve gratuite à chaque nouvelle désignation. |
| DRN-06 | 3 | Interposition | Programme un drone de protection pour se placer sur une trajectoire et tenter de protéger un allié. | Châssis, position de départ et trajet adaptés ; consomme la réaction du drone, qui peut subir l'attaque. Ne téléporte pas l'unité et ne bloque pas nécessairement une zone entière. |
| DRN-07 | 4 | Éclaireur autonome | Améliore DRN-02 : permet une reconnaissance bornée au-delà du dernier point connu, avec retour et rapport daté. | Demande Patrouille bornée, capteurs et réserve suffisante. Proposition prudente : pas de vision tactique en direct hors perception du noyau ; voir la règle dédiée ci-dessous. |
| DRN-08 | 4 | Routine conditionnelle | Ajoute une condition locale à une consigne : se replier si endommagé, s'arrêter devant un danger détecté, protéger un allié proche. | Capacité de programmation et une règle simple bornée. Utilise les perceptions du drone ; pas de programme infini ni d'accès à l'état caché de la carte. |
| DRN-09 | 5 | Déploiement coordonné | Répartit les unités contrôlées entre plusieurs positions et rôles connus par une commande de groupe. | Ordre coûteux et bande passante ; chaque unité se déplace lors de ses actions normales. Aucune formation instantanée au travers des murs. |
| DRN-10 | 5 | Repli d'urgence | Ordonne une retraite de groupe priorisant la conservation des unités et du matériel. | Liaison et trajets valides ; les unités renoncent à leurs prochains comportements offensifs pour revenir. Aucun rappel ou sauvetage automatique d'un drone encerclé. |

#### Perception et commande des drones retenues pour les essais

Un drone possède sa propre perception pour agir localement. L'interface du joueur ne reçoit pas automatiquement la somme des champs de vision de tous les drones : cela élargirait fortement l'information tactique et changerait la règle actuelle du projet.

Chaque drone est une entité physique placée sur sa propre case du monde : il se déplace, occupe l'espace, peut bloquer ou être bloqué, subir des attaques et être détruit. Le mode terminal lui attribue provisoirement un glyphe propre ; le futur mode graphique remplacera ce rendu par l'image du drone sur cette même case, sans changer sa simulation ni le transformer en simple bonus attaché au joueur.

Pour cette version, un drone hors liaison poursuit seulement une routine déjà donnée. Une doctrine de compagnon peut rejoindre la dernière position confirmée de son contrôleur, puis y attendre ; elle ne reçoit jamais sa position distante actuelle. Un rapport d'exploration devient un souvenir daté lorsqu'il peut être transmis ou rapporté ; il ne maintient pas une icône d'ennemi actualisée à travers un mur. La position affichée d'un drone hors contact reste sa dernière position confirmée, clairement distinguée d'une position actuelle.

Une future vue déportée ou un pilotage direct pourrait constituer un choix de conception distinct, avec ses limites propres. Cette possibilité reste ouverte à l'avis de l'utilisateur ; elle n'est pas introduite implicitement par Éclaireur autonome.

Les ordres actifs consomment du temps. Une routine ne donne pas un nouvel acte à chaque ouverture de menu. Le nombre de drones dépend de la bande passante, des limites actives et des châssis persistants éventuels ; l'énergie, les liaisons et les réactions par unité doivent être équilibrées ensemble.

La doctrine de base d'une nouvelle unité contrôlée est une escorte rapprochée : elle suit son propriétaire avec ses déplacements ordinaires et, si celui-ci attaque explicitement une cible pendant son action, elle tente à son tour une attaque ordinaire contre cette cible. Le drone ne triche pas sur sa perception, sa portée, sa ligne de tir, son énergie ni sa précision. `DRN-05` reste distinct : il permet de désigner une cible sans devoir soi-même l'attaquer et de coordonner plusieurs unités compatibles.

La palette flottante commune aux alliés permet de remplacer cette consigne sans dépenser de tour : **Suivre** assiste seulement la cible attaquée par le joueur, **Défensif** n'engage que les menaces arrivées près du groupe, **Agressif** recherche et poursuit une menace perçue dans une laisse bornée, et **Passif** suit sans attaquer. Ses boutons à icônes expliquent leur doctrine au survol et signalent en rouge une liaison indisponible. La commande s'applique actuellement au groupe contrôlé ; une sélection individuelle pourra être ajoutée sans changer le contrat des comportements. Ces doctrines sont des comportements ordinaires de compagnon, pas des techniques à acheter ; une technique avancée peut ensuite les remplacer par un ordre plus précis.

#### Obtention des drones

Trois voies d'obtention doivent coexister : acheter un châssis assemblé dans un magasin compatible, en construire un à partir d'un plan, de composants et d'outils, ou manifester l'unité utilitaire de base avec `DRN-01`. Un drone acheté ou construit persiste jusqu'à sa destruction ou sa perte et pourra recevoir des spécialisations matérielles. Le drone manifesté reste borné par sa recharge, sa limite active, sa réserve énergétique et la bande passante occupée ; sa réserve vide provoque sa dissipation explicite et libère la possibilité d'une nouvelle manifestation après recharge. Quelle que soit sa provenance, le drone obtenu devient une entité physique sur sa propre case et reste soumis aux mêmes règles de liaison, énergie, actions, dégâts et destruction.

Le drone de test automatiquement placé près du joueur a été retiré des nouvelles parties avec la génération 54. Les suspensions plus anciennes le conservent uniquement pour garantir leur rejeu historique.

Frontière : Ingénierie peut fabriquer une unité ; une technique dédiée peut en appeler ou déployer une selon ses coûts ; Intrusion peut obtenir le contrôle initial d'une machine compatible ; Contrôle de drones améliore les consignes et la coordination. Acheter et utiliser un drone basique ne demande pas d'investir dans ces trois disciplines.

Exemple : un drone leurre attire une sentinelle tandis qu'un récupérateur rapporte un objet connu. Le joueur doit protéger les unités, disposer des canaux nécessaires et accepter que leurs déplacements prennent du temps et puissent échouer.

## 9. Interactions entre catégories

Les combinaisons ci-dessous illustrent des possibilités. Elles ne créent pas de bonus cachés simplement parce que deux compétences figurent sur la fiche.

| Combinaison | Situation créée | Limite qui préserve le choix tactique |
|---|---|---|
| Repoussement + Champ de saturation | Déplacer une cible dans une zone déjà dangereuse. | Masse et destination valides ; coût de la balise, risque allié et nombre de déclenchements par phase borné. |
| Entrave + Écrasement | Préparer une frappe lourde sur une cible momentanément gênée. | Deux investissements/actions ou coopération ; résistance et sortie possible de l'entrave. |
| Reconnaissance + Tir localisé | Repérer un capteur ou une propulsion utile à neutraliser. | Cible perçue, coût d'observation et simulation réelle de la fonction. |
| Charge de brèche + Surveillance | Ouvrir une ligne de tir puis couvrir le nouveau passage. | Préparation audible, réaction limitée ; les ennemis utilisent aussi l'ouverture. |
| Leurre sonore + Mine de proximité | Attirer une investigation vers un dispositif posé. | Le comportement adverse n'est pas garanti ; la mine peut être détectée ou frapper un tiers. |
| Brouillage + Approche couverte | Affaiblir un canal de détection et profiter d'un couvert réel. | Autres capteurs, émissions du brouilleur et mémoire des témoins restent actifs. |
| Surchauffe + adversaire énergivore | Forcer un arbitrage entre puissance de feu et refroidissement. | Cible compatible, purge et gestion thermique ; ne fonctionne pas sur tous les organismes. |
| Diagnostic énergétique + Implosion | Identifier un stockage d'énergie compatible sur une cible perçue et préparer son sabotage. | États effectivement simulés, observation, compatibilité et préparation ; pas d'explosion de tous les objets électriques. |
| Démontage soigneux + Réglage spécialisé | Transformer une récupération précise en adaptation matérielle. | Pièces finies, temps et contrepartie du réglage. |
| Détournement + Falsification de registre | Utiliser un système puis empêcher l'exploitation ultérieure d'un accès suspect local. | Inspection ou transmission de sécurité réelle ; accès identifié, témoins, copies et alertes déjà transmises conservent leurs conséquences. |
| Éclaireur autonome + Démolition | Choisir une route préparée à partir d'un rapport antérieur. | Rapport daté ; patrouilles et obstacles peuvent avoir changé. |
| Interposition + Repli d'urgence | Couvrir la retraite d'unités précieuses. | Réaction et corps du protecteur exposés ; aucune extraction instantanée. |

## 10. Exemples de builds et coût de spécialisation

Ces exemples décrivent des répartitions, pas de nouvelles classes verrouillées. Ils supposent le tarif provisoire et aucun apprentissage gratuit hors des techniques explicitement accordées par la classe.

| Profil | Répartition | Coût | Manière de jouer et faiblesse |
|---|---|---:|---|
| Combattant mobile | Combat rapproché 3 + Manœuvre 3 | 8 points | Choisit ses contacts et exploite les déplacements ; dépend du trajet, de la masse et de l'exposition. |
| Tireur observateur | Tir 3 + Reconnaissance 3 | 8 points | Investit dans l'information et les engagements préparés ; moins efficace si constamment obligé de bouger. |
| Démolisseur discret | Démolition 3 + Furtivité 3 | 8 points | Prépare mines, leurres et brèches ; dépend de ses consommables et peut provoquer des conséquences locales importantes. |
| Saboteur électronique | Guerre électronique 3 + Intrusion 3 | 8 points | Combine accès et attaques de systèmes ; doit prévoir une réponse aux cibles incompatibles et aux coupures. |
| Opérateur de drones | Contrôle de drones 3 + Ingénierie 3 | 8 points | Coordonne et préserve du matériel ; plus exposé aux pertes, au brouillage et au manque de pièces. |
| Spécialiste confirmé | Cinq techniques d'une compétence + deux d'une autre | 11 points | Obtient cinq choix dans sa spécialité et deux en soutien ; leurs niveaux, attributs et prérequis doivent rester satisfaits individuellement. |
| Généraliste | Deux techniques dans quatre compétences | 8 points | Dispose de huit choix variés, sous réserve des conditions propres à chacun. |

Chaque équipement important doit pouvoir être exploité à un niveau ordinaire par un corps compatible. Les techniques renforcent les orientations sans rendre inutiles toutes les trouvailles hors spécialisation. Une classe de départ peut garantir le matériel minimal de son concept, tandis que les trouvailles pendant la partie permettent de bifurquer.

## 11. Rapport critique

### 11.1. Ce qui me paraît solide

1. Les dix compétences ont des situations reconnaissables : contact, ligne de tir, destruction, placement, détection, connaissance, matériel, accès, attaque numérique et coordination.
2. Le personnage conserve son corps principal et ses apprentissages pendant que les équipements et améliorations permettent l'adaptation.
3. Les spécialisations physiques ont de véritables outils de contrôle et d'environnement ; la guerre électronique possède des dégâts directs et de zone dès sa progression.
4. Les fonctions ordinaires restent accessibles. Les compétences de soutien n'ont pas à être achetées pour utiliser un simple objet ou comprendre une menace visible.
5. Les préparations et conséquences locales peuvent créer des rencontres différentes sur des cartes procédurales : couloir surveillé, accès ouvert, mine déplacée, machine compromise ou drone perdu.
6. Les réactions partagées donnent un prix aux builds défensifs hybrides. Un personnage ne cumule pas automatiquement toutes les contre-attaques connues.
7. Les informations techniques restent liées à une observation, un accès ou un objet. Cela permet des surprises sans rendre l'interface mensongère ou omnisciente.

### 11.2. Risques et corrections recommandées

| Risque | Pourquoi il compte | Recommandation |
|---|---|---|
| Traitement trop dominant | Intervient dans plusieurs compétences numériques et dans une partie de l'Ingénierie. | Fixer des contributions secondaires étroites ; réserver puissance, nombre d'unités, portée et réserves aux matériels concernés. Ne pas faire dépendre tous les apprentissages de Traitement. |
| Ingénierie obligatoire | Une économie équilibrée autour du rendement du spécialiste pénaliserait tous les autres builds. | Équilibrer les réparations et récupérations ordinaires sans cette compétence ; privilégier des choix matériels particuliers et des réparations de fonctions. |
| Reconnaissance surtout utile au débutant | Un joueur expérimenté pourrait connaître les fiches ennemies et ne plus l'acheter. | Donner des informations locales et variables, des failles observées, des traces et des préparations utiles ; préserver l'accès gratuit aux avertissements essentiels. |
| Drone supplémentaire égal à plusieurs actions gratuites | Chaque unité peut ajouter dégâts, réactions et reconnaissance. | Contrôler le nombre, la bande passante, les réserves et les ordres ; tester la réaction par unité. Pas de salve additionnelle au simple envoi d'un ordre. |
| Défense par attente toujours optimale | Surveillance, Parade ou camouflage pourraient permettre de neutraliser tout danger sans avancer. | Diversifier les réponses adverses : détour, couverture, leurre, tir indirect adapté, destruction d'équipement ou refus d'approcher. Les coûts et ressources doivent porter une partie du risque. |
| Compétences de préparation trop lentes | Si chaque ennemi exige plusieurs actions de menus et d'analyse, le rythme devient laborieux. | Interface commune, raccourcis, préparations visibles et peu nombreuses ; certaines informations acquises restent mémorisées sans devoir relancer la même action. |
| Attaques électroniques universelles | Si tout est piratable dans une simulation, la plupart des autres armes perdent leur rôle. | Compatibilités et résistances explicites ; solutions matérielles alternatives ; anomalies profondes conçues séparément. |
| Verrouillages répétés | Entrave, neutralisation ou purge/offensive répétées peuvent supprimer toute réponse. | Une politique commune de durée, de résistance et de protection contre le maintien permanent. Éviter une exception différente et invisible pour chaque boss. |
| Doubles ou triples achats obligatoires | Ciblage, piratage et appareil pourraient chacun demander une compétence pour une action banale. | Procédure standard utilisable avec la compétence principale et son matériel ; les autres compétences apportent de la préparation ou de l'efficacité. |
| Blocage du build par l'équipement trouvé | Un spécialiste peut perdre son seul matériel compatible. | Garantir plusieurs familles d'outils compatibles et des fonctions ordinaires utiles ; prévoir une disponibilité raisonnable du matériel d'entrée, sans garantir chaque objet rare. |
| Saturation des options | Un grand catalogue peut produire des erreurs de lecture irréversibles. | Afficher les prérequis, les dépendances et l'effet concret avant achat ; discuter séparément d'une éventuelle réattribution limitée. |
| Coût d'une forte spécialisation en Reconnaissance | Les achats tardifs coûtent davantage sans ouvrir automatiquement de pouvoir exclusif. | Tester l'intérêt de ces choix à leur tarif actuel ; conserver une liste courte plutôt qu'ajouter une capacité uniquement pour remplir un palier artificiel. |
| Guerres de statistiques obligatoires | La même action pourrait compter la primaire, deux secondaires et plusieurs compétences redondantes. | Écrire une chaîne de calcul courte par action ; un rôle distinct par contribution et une prévisualisation intelligible. |

### 11.3. Limites propres au catalogue actuel

Combat rapproché et Tir sont les plus faciles à juger avec une petite arène. Guerre électronique a une identité claire, mais demande des règles distinctes pour émission physique, infection et gestion thermique. Démolition est convaincante si le terrain peut être transformé ; Effondrement contrôlé est coûteux à produire et peut être reporté.

Ingénierie et Reconnaissance ont besoin d'essais plus attentifs : leur intérêt doit provenir des situations et du matériel, sans devenir des passages obligés. Contrôle de drones est le domaine le plus sensible pour le nombre d'actions et les limites de perception. Il faut le valider avec très peu d'unités avant d'augmenter l'échelle.

La révision de Reconnaissance regroupe les renseignements d'une cible et conserve des traces, secrets et parois fragiles définis dans le monde. L'interprétation des faux contacts reste différée. Le diagnostic énergétique attend les états qu'il lit, et Intrusion attend des règles de sécurité observables pour proposer ses actions sur les registres et la reprise de contrôle.

La liste complète sert à examiner la cohérence du jeu. Elle ne constitue pas une obligation d'implémenter toutes les entrées dans la première version jouable. Les techniques avancées supposent souvent des systèmes encore à construire.

## 12. Faisabilité et ordre de mise à l'épreuve

### 12.1. État local consulté

Une lecture ciblée a été faite le 10 septembre 2026 dans les documents fondateurs, la documentation moteur et les points d'entrée de progression, d'acteur et de commande. Cette lecture confirme une base de simulation à commandes, des attaques, des capacités, des statuts, un inventaire et une progression d'XP avec points de compétence. Elle ne constitue pas un audit exhaustif du dépôt ni une validation de toutes les mécaniques ci-dessus.

L'acteur consulté expose notamment une réserve de PV (`integrity` dans le prototype), des résistances, des attaques, des capacités et des statuts. Le ciblage de composants, les préparations et réactions décrites ici demanderont une extension des contrats correspondants. Ce rapport n'implémente aucun achat de technique et ne prouve aucune capacité jouable nouvelle.

### 12.2. Dépendances à construire ou à vérifier

| Famille de règles | Techniques qui en dépendent | Premier essai utile |
|---|---|---|
| Catalogue d'apprentissages, prérequis et sauvegarde | Toutes les compétences. | Choisir une technique, refuser un niveau, un attribut ou un prérequis absent, conserver le choix après changement d'équipement et chargement. |
| Temps de préparation, récupération et réaction | Parade, Riposte, Tir visé, Surveillance, Esquive préparée. | Deux acteurs, une préparation et une interruption ; une seule réaction entre actions normales. |
| Mouvement forcé et collisions | Repoussement, Charge, Percée, Extraction. | Petite salle avec obstacle, unité lourde, case dangereuse et trajet bloqué. |
| Projectiles, précision et modes de tir | Tir, certains explosifs et drones. | Trajectoires multiples, couvert, consommation réelle de munitions et absence de cibles cachées dans l'aperçu. |
| Énergie, chaleur et résistances numériques | Surcharge, Surchauffe, Surcadencement, camouflage. | Un coût matériel et une réponse adverse lisibles, sans dépendance à la vitesse d'animation. |
| Dispositifs, retardateurs et terrain destructible | Mines, brèches, balises, détonations préparées. | Une charge ouvre une paroi ; la propagation suivante voit le terrain modifié. |
| Composants et dommages de fonctions | Tir localisé, Réparation ciblée, certaines entraves. | Endommager puis réparer une fonction réelle sans confondre les PV du personnage et la durabilité du matériel. |
| Perception, indices et états d'alerte | Furtivité, Analyse de cible et Analyse multiple. | Deux observateurs avec informations différentes ; cible perçue, résistances identifiables, perte de vue et absence de bonus d'attaque inventé par l'analyse. |
| Traces locales et secrets du terrain | Lecture de traces, Inspection minutieuse, Repérage des parois fragiles. | Traces bornées et datées, cache réellement placée avant inspection et cloison dont les propriétés existent avant le repérage ; aucune dépendance aux effondrements. |
| Accès, journaux et topologie locale | Intrusion. | Une porte, un terminal et une consultation différée d'événements suspects ; tentative ennemie de reprise de contrôle et conséquences locales observables. |
| Contrôle, routines et économie d'unités | Drones. | Une unité et un contrôleur, puis deux unités ; perte de liaison, retour, réaction et dépense de ressources. |
| Supports structurels | Effondrement contrôlé. | Une structure locale explicitement destructible ; pas d'effondrement arbitraire ni de dépendance pour terminer la carte. |

Ordre recommandé pour les premiers essais : progression et prérequis, puis un petit ensemble mêlée/tir/zone partageant les mêmes règles de temps, puis terrain et dispositifs, ensuite perception et intrusion, enfin coordination de plusieurs unités. Les contenus spécialisés peuvent être ajoutés lorsque leurs règles communes sont éprouvées.

### 12.3. Scénarios de vérification futurs

Ces scénarios sont des critères pour le développement futur ; ils n'ont pas été exécutés comme tests de gameplay dans ce travail documentaire.

1. Un personnage sans compétence utilise une arme récupérée, une réparation ordinaire et un module compatible.
2. Un personnage conserve son corps principal et ses apprentissages après changement d'équipement ; une technique sans matériel compatible devient indisponible avec une explication claire.
3. Un apprentissage ne donne qu'une entrée ; une amélioration sans parent est refusée et ne consomme pas de points.
4. Parade, Interception, Surveillance et Esquive préparée ne contournent pas la réaction commune. Une commande annulée ou un menu n'en restitue pas une.
5. Une unité repoussée traverse la résolution prévue de la case, sans déclencher une infinité de réactions ou de dégâts de champ pendant une seule phase.
6. Une animation accélérée, un autre mode de fenêtre ou un écran plus large ne changent ni les résultats ni les cibles connues.
7. Un mur bloque vision et émissions concernées ; une explosion qui ouvre ce mur met à jour le terrain selon l'ordre des phases, sans divulguer le reste de la carte.
8. Une rafale répartie conserve son total de projectiles. Un ordre de drone ne produit pas un tir supplémentaire hors de son économie normale.
9. Une infection cesse ou se propage selon ses bornes ; une machine isolée et une cible purement organique reçoivent les résultats appropriés.
10. Une réparation, un démontage, une réouverture de porte et la destruction d'un drone fabriqué ne forment aucune boucle de ressources ou d'XP infinies.
11. Les témoins conservent leurs souvenirs après falsification d'un registre local. Les attaques de zone contre des tiers peuvent avoir les conséquences attribuables prévues.
12. Un drone hors contact ne fournit pas des positions ennemies en direct. Son rapport devient une information datée et sa routine ne consulte pas des données cachées.
13. Les issues essentielles d'une carte restent accessibles par les moyens prévus ; aucune technique rare unique n'est rendue indispensable par hasard sans alternative conçue.
14. Une alerte, un coût, une incompatibilité ou une zone dangereuse possède une indication textuelle ou symbolique et fonctionne au clavier comme à la souris, sans dépendre de la couleur seule.
15. Chaque technique de Reconnaissance proposée possède un parcours d'accès valide ; Analyse multiple exige Analyse de cible. Aucun achat ni prérequis ne pointe vers une entrée fusionnée, différée ou retirée.
16. Les traces expirent selon les règles locales et n'actualisent pas une case hors perception. Une inspection découvre un secret déjà placé sans en créer un et sans ouvrir automatiquement le passage correspondant.
17. Une version sans audit des registres, reprise de contrôle adverse ou états énergétiques n'offre pas les techniques qui en dépendent. Leur activation future apporte un effet observable, tout en préservant témoins, alertes déjà transmises et limites de connaissance.

## 13. Suivi des décisions et essais restants

Les recommandations de direction ont été acceptées. Le tableau conserve leurs repères de discussion et indique ce qui reste à définir ou à mesurer ; un tarif ou une durée de test ne devient pas une valeur définitive par cet accord.

| Repère | Décision | Base retenue | Suite à prévoir |
|---|---|---|---|
| AVIS-01 | Organisation générale. | Dix compétences ; fusions de Reconnaissance et Ingénierie, et sept entrées dans Reconnaissance révisée. | Éprouver leur utilité et leurs différences dans des situations jouables. |
| AVIS-02 | Nombre maximal d'apprentissages. | Aucun plafond par discipline. | Éprouver la courbe de prix ; traiter la réattribution comme une décision séparée. |
| AVIS-03 | Information donnée par les drones. | Routines autonomes et rapports datés ; aucune vue déportée en direct hors perception du noyau par défaut. | Toute extension de vision déportée ferait l'objet d'une décision distincte. |
| AVIS-04 | Liaisons à travers un obstacle. | Liaison directe comme base ; un ordre distant n'accorde pas de vision supplémentaire. | Définir séparément les éventuels câbles et réseaux autorisés. |
| AVIS-05 | Réactions des unités alliées. | Même limite par acteur. | Tester le nombre d'unités et leur économie avant d'augmenter la taille du groupe. |
| AVIS-06 | Ciblage des fonctions. | Tir localisé et réparation de composants dans la direction cible. | Proposer ces techniques seulement quand les fonctions correspondantes existent. |
| AVIS-07 | Ambition de la destruction. | Brèches, dispositifs et propriétés des parois en priorité. | Effondrements structuraux comme extension avec son propre système. |
| AVIS-08 | Variantes et spécialisation. | Une variante électronique appliquée à la fois. | Vérifier les compromis et la valeur de chaque choix. |
| AVIS-09 | Budget d'une partie. | Base d'essai à 1 point par niveau et coût croissant des choix successifs. | Comparer avec la durée réelle des parties ; examiner notamment les achats tardifs de Reconnaissance. |

Pour commenter, les codes permettent de viser une entrée précise : par exemple « FUR-09 trop puissant », « renommer DEM-03 » ou « AVIS-03 : je préfère un autre fonctionnement ». Un retour sur l'intérêt des situations et la clarté des noms sera plus utile à ce stade que le choix d'un pourcentage de dégâts définitif.

## 14. Références et portée du travail

- Conversation de conception : décisions sur les primaires, secondaires, conditions d'apprentissage et trois premières compétences. Source prioritaire pour ces décisions récentes.
- [Document de conception v0.2](C:/Users/User/Desktop/project-RL/Projet_Roguelike_IA_Document_Conception_v0.2.md), sections 5 à 8 : monde, perception, combat, incarnation et progression.
- [Spécification CODEX v0.2](C:/Users/User/Desktop/project-RL/Projet_Roguelike_IA_Spec_CODEX_v0.2.md), sections 8, 14 à 17.5 : temps, propagation, combat et expérience.
- [Documentation du moteur](C:/Users/User/Desktop/project-RL/docs/MOTEUR.md), consultée pour les contrats de simulation et l'état décrit de la progression et des effets.

Le catalogue est une proposition originale pour Project RL. Les inspirations de genre proviennent de la direction déjà discutée ; le rapport n'attribue pas ces techniques ni leurs règles à Cogmind ou Caves of Qud.

Livrable initial de ce travail : le présent rapport Markdown, sans modification du code. La clarification ultérieure sur le corps principal et le contrôle temporaire est également reportée dans les documents fondateurs (section 15.6). La vérification porte sur la structure, les références internes et la cohérence documentaire, pas sur un équilibrage prouvé en partie.

## 15. Décisions appliquées après le retour utilisateur du 10 septembre

### 15.1. Portée de l'accord et jeu solo

L'utilisateur a accepté les recommandations générales puis autorisé l'application des simplifications expliquées. Les fiches de Reconnaissance et d'Intrusion sont désormais révisées, leurs prérequis et exemples sont mis à jour, et les chiffres restent des paramètres d'essai. Cet accord porte sur la conception ; les systèmes décrits ne sont pas rendus jouables par la rédaction de ce document.

« Jeu solo » signifie un seul joueur humain. Le personnage peut avoir des PNJ alliés, des drones ou des familiers/pets. Les capacités de soutien gardent leur place ; les règles d'apprivoisement ou de commande propres aux familiers restent à définir. La bande passante des drones n'est pas imposée aux compagnons organiques.

Les mots « groupe », « allié » et « commandes adverses » désignent les acteurs de ce monde et leurs contrôleurs. Les simplifications portent sur les doublons et les dépendances, pas sur l'absence de coéquipiers humains.

### 15.2. Correspondance des entrées après simplification

| Repère conservé | Décision appliquée | Effet et condition |
|---|---|---|
| REC-01 | Renommée Analyse de cible et enrichie par la fusion de REC-07 ; niveau 1. | Une même analyse peut renseigner l'état, l'équipement observable et les résistances identifiables d'une cible perçue. |
| REC-02 | Lecture de traces conservée ; niveau 1. | Indices de déplacement réellement présents, locaux et datés ; certains visibles à tous, d'autres à détecter ou interpréter. |
| REC-03 | Inspection minutieuse précisée ; niveau 2. | Recherche de pièges, caches, commandes dissimulées et passages secrets déjà placés dans le monde ; découverte distincte de leur activation. |
| REC-04 | Renommée Repérage des parois fragiles ; niveau 2. | Matériau, résistance et état de cloisons, portes et parois observables. Aucun calcul de stabilité du bâtiment entier requis. |
| REC-05 | Profil de menace conservé ; niveau 3. | Possibilités de combat observables, distinctes des propriétés matérielles identifiées par REC-01. |
| REC-06 | Différée ; hors catalogue d'apprentissage. | Discrimination des signaux sera réexaminée seulement si les faux contacts ont un rôle éprouvé. |
| REC-07 | Fusionnée ; hors catalogue d'apprentissage. | L'ancienne Analyse de faille ne demande plus de second achat ni d'action d'analyse supplémentaire ; son information rejoint REC-01. |
| REC-08 | Renommée Diagnostic énergétique ; niveau 4. | États énergétiques, thermiques et propriétés réellement simulés de machines perçues ; disponibilité avec ces systèmes. |
| REC-09 | Renommée Analyse multiple et avancée au niveau 3. | Amélioration de REC-01 appliquée à plusieurs cibles perçues en une action ; limites d'information conservées pour chacune. |
| REC-10 | Retirée de la première version ; hors catalogue d'apprentissage. | L'ancien bonus préparé d'Exploitation tactique faisait doublon avec l'analyse et les techniques d'attaque. Aucun remplacement ajouté. |
| INT-08 | Falsification de registre précisée ; niveau 4. | Agit sur une preuve locale avant une inspection ou transmission effective de sécurité ; indisponible sans cette boucle. |
| INT-10 | Renommée Verrouillage de contrôle ; niveau 5. | Empêche temporairement la reprise d'un dispositif piraté par ses contrôleurs adverses ; indisponible si aucun comportement adverse ne tente cette reprise. |

Les identifiants conservés permettent de retrouver les retours précédents. Cette révision historique avait ramené le catalogue à 95 entrées principales et 10 variantes ; les trois anciennes entrées de Reconnaissance ont leur statut propre en section 6.2. La fusion ultérieure de MAN-07, suivie en section 15.7, porte le total actuel à 94 entrées principales et 10 variantes.

### 15.3. Périmètre technique retenu pour Reconnaissance

Les traces associent à une case un type, une direction, une ancienneté et une durée de conservation. Leur création dépend du corps et du terrain ; leur conservation est bornée. Les cases hors perception ne montrent pas leurs nouveaux indices au joueur et une trace n'est pas la position actuelle de son auteur. Étendre le suivi de traces aux comportements ennemis reste un travail distinct.

Les secrets existent dans le contenu avant inspection. La découverte met à jour la connaissance du personnage ; l'ouverture d'un passage ou le désamorçage d'un piège suit ensuite ses propres règles. Les accès essentiels disposent de moyens ordinaires ou alternatifs prévus par la conception.

Le repérage des parois lit des propriétés définies sur le terrain : matériau, résistance, fissure, corrosion ou renforcement. L'exemple retenu est une cloison fragilisée permettant un détour. L'ancien exemple de pilier menaçant une travée est remplacé ; les effondrements restent conditionnés au système dédié de Démolition.

Les résistances par type de dégâts sont représentées dans le moteur consulté par `ResistanceProfile`. La découverte de ces informations par une compétence, les états thermiques et le diagnostic du stockage d'énergie nécessitent leurs propres règles. Leur évocation ici ne démontre pas leur fonctionnement actuel en jeu.

Reconnaissance conserve sept techniques accessibles selon leurs conditions individuelles, sans palier abstrait à remplir. Le coût de ses achats tardifs et le nombre de cibles de l'analyse multiple seront éprouvés en jeu.

### 15.4. Sécurité active : conditions d'Intrusion retenues

Pour INT-08, un accès suspect peut être enregistré sur un appareil compromis puis examiné plus tard par un terminal, un processus de sécurité ou un PNJ. Falsifier cet événement avant consultation peut éviter que cette preuve déclenche l'investigation prévue. Cela ne supprime ni une alarme déjà transmise, ni une copie déjà reçue, ni la mémoire d'un témoin.

La boucle minimale utilise des événements structurés et une règle locale d'inspection ou de transmission. Une indication telle qu'un contrôle en attente rend la possibilité d'action compréhensible. Aucun modèle de langage n'a à lire un journal en texte libre. Une version sans cette boucle ne propose pas Falsification de registre à l'achat.

Pour INT-10, un contrôleur adverse peut tenter de refermer une porte ouverte par le joueur, de réactiver une caméra ou de récupérer une tourelle détournée. Verrouillage de contrôle bloque temporairement ces commandes habituelles dans le périmètre piraté. Accès, durée et canal restent limités ; coupure, intervention physique ou réinitialisation peuvent permettre une réponse.

Une version sans reprise de contrôle adverse ne propose pas cette technique à l'achat. Les alarmes, renforts, fermetures de portes et piratages déjà envisagés dans la conception fournissent le contexte, mais les comportements précis doivent être construits et vérifiés.

### 15.5. Portée de la révision

Les modifications portent sur le catalogue, les statuts des décisions, les dépendances, les exemples et les critères de vérification documentaire. Aucun remplacement n'a été ajouté pour les trois entrées de Reconnaissance sorties du catalogue. Les noms offensifs de Guerre électronique et les autres orientations acceptées sont conservés.

Les prochains essais doivent vérifier que l'analyse fournit une information utile, que les traces et secrets existent effectivement, et que les techniques de sécurité ont une conséquence observable. La révision des fiches de Reconnaissance et d'Intrusion ne modifie ni les documents fondateurs, ni le code du jeu, ni les règles propres aux familiers. La clarification ultérieure ci-dessous est, elle, synchronisée avec les documents fondateurs.

### 15.6. Corps principal fixe et capacité indépendante envisagée

Statut de suivi : le corps fixe et sa vulnérabilité pendant le contrôle sont confirmés ; la conception des capacités extérieures est reportée à beaucoup plus tard à la demande de l'utilisateur. Cette section conserve la piste discutée sans en faire un préalable au système actuel.

Clarification utilisateur du 10 septembre 2026 : le corps principal du personnage reste le même pendant toute la run ; seuls son équipement et ses améliorations évoluent. Les anciennes mentions du changement de corps comme mécanisme courant étaient trop affirmatives et sont corrigées. Les connaissances restent acquises pendant la partie lorsque l'équipement change.

L'utilisateur propose de prévoir une capacité facultative à débloquer en jeu, hors des arbres, permettant de contrôler temporairement un ennemi. Pendant l'effet, le corps principal reste présent à son emplacement, « éteint » et inactif. Le retour normal se fait vers ce même corps ; les interruptions et destructions restent à définir. Ce n'est ni un remplacement permanent du corps ni une résurrection.

L'utilisateur a confirmé que le corps principal reste vulnérable et peut subir des dégâts pendant cet effet. Le mettre à l'abri est un choix tactique ; l'extinction n'accorde pas de protection automatique. Une éventuelle interruption sur dégâts et les conséquences de sa destruction restent à décider.

La capacité n'est pas ajoutée à Intrusion, Guerre électronique ou Contrôle de drones et ne modifie pas leurs catalogues. Elle relève de la future famille des capacités indépendantes. Son déblocage peut être distinct d'un achat : nom, mode d'obtention, coût d'apprentissage et éventuelle limite de capacités indépendantes restent ouverts.

La section 7.2 du document de conception recense les décisions restantes : cibles, durée et coûts, liaison, retour volontaire, conséquences des dégâts et de la destruction du corps principal, mort de l'ennemi contrôlé, statistiques et actions disponibles, perception, expérience et factions. Aucune invulnérabilité ou vision combinée n'est accordée implicitement. Cette capacité ne doit pas dépendre d'un champ de vision passant à travers les murs.

Cette clarification est synchronisée avec les sections 7 et 8.4 du document de conception et la section 17.6 de la spécification CODEX. Le catalogue des techniques existantes et le code du jeu ne sont pas modifiés.

### 15.7. Corrections du point 8 validées : catalogue actif et historique

Après la revue finale, l'utilisateur a accepté les cinq corrections de principe (« oui :) »). Leur application est définie en sections 13.3, 15.3, 16.2 et 17.4 des règles communes. L'audit de référence passe à 5 UT ; les retardateurs annoncés excluent leur cycle d'armement ; les actions non chauffantes ne sont pas bloquées par le seul dépassement thermique ; une discipline incomplète reste différée jusqu'à garantir cinq choix légaux sur toute suite d'apprentissage.

Retraite méthodique est fusionnée dans Pas de dégagement : neuf entrées de Manœuvre restent actives, sans modifier leur prix relatif. La consigne maintenue n'exécute pas une suite de mouvements instantanés : chaque pas conserve son temps, ses dangers, sa validation et ses fenêtres adverses.

| Ancien code | Statut | Décision appliquée |
|---|---|---|
| MAN-07 | Fusionnée, identifiant réservé | Retraite méthodique n'est plus un achat. Son maintien de consigne appartient à MAN-01, Pas de dégagement, sans bonus d'Esquive ni économie d'action supplémentaires. |

Aucun prérequis actif ne pointe vers MAN-07. Une sauvegarde qui posséderait cet ancien choix nécessite une migration explicitement décidée ou un refus de compatibilité ; aucun remboursement ou remplacement implicite n'est défini. Les barèmes de dégâts, Infection, le tarif de Reconnaissance et la réattribution restent dans leur statut antérieur.

## 16. Annexe technique complète — profils d'essai 0.6

### 16.1. Héritage obligatoire et matériel de référence

Statut : profils de travail, avec cinq corrections de principe validées au point 8 ; autres valeurs à tester, aucune intégration au moteur. Les **94 entrées principales et 10 variantes** ont chacune une ligne ci-dessous. Chaque fiche comprend sa ligne qualitative des sections 4–8, sa ligne technique et les règles communes suivantes. Les trois anciens identifiants de Reconnaissance et MAN-07 ne sont pas réintroduits ; les noms et prérequis des entrées encore actives restent inchangés.

- Toutes les règles de [Statistiques et compétences](STATISTIQUES_ET_COMPETENCES.md), notamment sections 10–17, s'appliquent : légalité, perception, défenses, temps, coûts engagés, réactions, réservations, non-cumul et progression.
- A1, P1+A1, R1, E/H/B et UT sont définis dans les règles communes. « Arme » signifie les coûts natifs de l'attaque, puis les suppléments indiqués ; un montant autonome sans « Arme » donne le coût total de la technique pour le matériel de référence. Les variantes indiquent ce qu'elles remplacent ou ajoutent au coût de leur mère.
- Sauf mention : une cible ; durée instantanée ; CD 0 ; portée égale au minimum de celle indiquée et de la limite matérielle ; aucune pénétration ajoutée ; pas de jet hors celui de la touche ordinaire ou de l'opposition explicitement nommée. Les procédures sur un objet coopératif sont certaines une fois leurs préconditions remplies. Une amélioration passive ne consomme pas d'action à l'achat et ne déclenche rien seule.
- Les prérequis d'apprentissage sont exactement ceux des lignes qualitatives : niveau, attributs et techniques déjà apprises. Une dépendance à composants, supports, audits, reprise de contrôle, énergie, traces ou routines rend l'entrée indisponible à l'achat si le système manque. Ce n'est pas une exigence de compétence supplémentaire.
- Retirer une entrée de la version retire aussi ses améliorations sans prérequis disponible. Une discipline est ouverte lorsqu'elle offre un parcours d'apprentissage cohérent ; sinon son ouverture est différée, sans palier vide ni plafond temporaire inventé. Les acquis de classe et sauvegardes passent ce contrôle selon les règles communes §17.4.
- Chaleur volontaire : le refus thermique concerne uniquement un apport positif dépassant la limite du mode (§15.3). Retardateurs annoncés : exclure le cycle d'armement ; les tics, cooldowns et audits gardent leurs calendriers distincts (§§13.3 et 16.2).
- Action invalide connue : aucun coût. Tentative légale manquée : coûts engagés conservés. Préparation interrompue : aucun effet final ni coût d'étape future ; objets déjà posés restent dans le monde. Le CD éventuel suit la section 16.2 des règles communes. Un effet actif du même type n'est pas renouvelé indéfiniment.
- Sauf prélèvement explicitement situé dans une interaction de fabrication ou de commerce : E payé à la première étape, B réservée dès la première étape jusqu'à la fin du processus concerné et H ajoutée à l'exécution de l'effet énergivore. Une compétence ne consomme aucun objet pour produire son effet de base. Un entretien par UT est payé avant son effet périodique ; faute de réserve, l'effet s'arrête. La chaleur de maintien suit chaque paiement réel. Les dépenses de l'arme restent celles de son tir ou de sa frappe effectifs.
- Les multiplications des dégâts physiques se font après calcul de l'Impact autorisé, puis arrondi inférieur, avant Parade/Blindage. Le bonus d'Impact n'est pas ajouté une seconde fois à chaque composante. Un tir n'utilise pas l'Impact.
- Les paramètres des armes restent matériels. Référence pour les exemples : mêlée 12 dégâts physiques à Impact 10 ; frappe de service sans arme 5 ; projectile simple 10, portée 6, une munition ; rafale native 3 projectiles avec −15 de Précision chacun ; tir simple sans cette dispersion. Un profil énergétique remplace ses munitions par son coût propre, il n'est pas rendu gratuit.
- Émetteur électrique de référence : puissance 16, portée 4. Explosif de référence : 24 physiques + 12 thermiques, rayon 2 ; mine : 20 physiques, rayon 1 ; charge de brèche : 60 physiques à l'obstacle ciblé, 12 physiques sur les autres cases du rayon 1. Ce sont des profils de laboratoire, pas de nouveaux objets ajoutés au jeu.
- Les tirs lancés sur une case connue, explosifs et zones ne donnent pas l'identité des occupants cachés. Les dégâts de zone atteignent les tiers compatibles ; les émissions électroniques locales respectent les obstacles. Une analyse ou un ordre ne modifie pas la carte perceptible.
- Défauts de cumul : une préparation offensive, une garde ; même modificateur non additionné ; même pénalité non rafraîchie ; une variante électronique choisie par exécution. Les techniques de tir préparé ne s'empilent pas avec une autre préparation offensive, par exemple Embuscade ou Tir de rupture.

Les lignes utilisent des codes et du vocabulaire de conception, pas des descriptions à copier dans le jeu. Le [rapport de vérification](RAPPORT_SYSTEME_STATISTIQUES_COMPETENCES.md) précise la portée des contrôles et les essais encore nécessaires.

### 16.2. Combat rapproché — 10 profils

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| MEL-01 | A1 + R1 | Arme | Contact | Dégâts physiques ×1,5 ; touche ordinaire. | Récupération engagée même si raté ; esquive, armure ou distance. |
| MEL-02 | P1+A1 | Arme à la frappe | Contact, cible suivie | +20 Précision pour une frappe. | Bouger, perdre le contact ou changer de cible annule ; pas de cumul avec une autre visée. |
| MEL-03 | A1 | Arme | Contact, poussée 1 case | Physiques ×0,5 ; si touche, poussée avec Force=Impact, même à zéro dégât. | Masse/ancrage ou destination invalide bloquent la poussée ; aucun dégât de collision ajouté. |
| MEL-04 | A1 de garde | 0 à la garde ; 2 E au déclenchement | Soi, une attaque de mêlée | Réaction : −50 % physiques bruts avant Blindage, selon règles communes. | Expire à l'action suivante ; détourner, attendre ou employer un autre type d'attaque. |
| MEL-05 | A1 + R1 | Arme pour une frappe + 4 E | Jusqu'à 3 cases adjacentes contiguës en arc | Un jet par occupant ; physiques ×0,7 par cible, alliés inclus. | Espace réel requis ; couvert, armure et dispersion des adversaires. |
| MEL-06 | Amélioration de MEL-04 | Parade + Arme de riposte | Attaquant encore au contact | Une frappe ordinaire après parade, dans la même réaction. | Pas de réaction supplémentaire ; absence de portée/ressources supprime la riposte seulement. |
| MEL-07 | A1 | Arme + 3 E | Contact | Physiques ×0,6 ; si touche, fragilisation 4 après l'impact pendant 3 UT. | Même à zéro dégât, pas d'effet sur cible sans Blindage ; pas de cumul/rafraîchissement actif. |
| MEL-08 | A1 | Arme + 3 E ; CD 2 | Contact, locomotion identifiée | Physiques ×0,5 ; si touche, Stabilité contre intensité 60 ; échec défensif : entrave 2 UT. | Cible incompatible ou fonction absente : pas d'entrave ; protection 1 UT après fin. |
| MEL-09 | A1 + R1 | Arme + 5 E | Contact, cible entravée/immobilisée | Physiques ×1,8 ; touche ordinaire. | Précondition vérifiée à l'engagement ; aucune entrave gratuite ni prérequis MEL-08 ajouté. |
| MEL-10 | A1 de garde | Arme au déclenchement | Une cible quittant le contact | Une frappe ordinaire de réaction avant retrait volontaire. | Mouvement non annulé automatiquement ; poussée/réaction ne déclenche pas l'interception. |

### 16.3. Tir — 10 profils

Pour les modes continus, chaque étape paie ses projectiles. Des munitions insuffisantes connues refusent l'étape avant tir. Un changement caché de cible/terrain après engagement suit la résolution réelle, sans rembourser les tirs déjà partis.

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| TIR-01 | P1+A1 | Arme au tir | Portée arme, une cible | +20 Précision au prochain tir natif simple. | Mouvement, changement de cible ou perte de vue annulent. |
| TIR-02 | A1 | 2 projectiles natifs | Portée arme, une cible | Rafale limitée à 2 ; dispersion −5 par projectile au lieu de −15 du profil natif. | Arme automatique compatible ; moins de projectiles contre cibles peu protégées. |
| TIR-03 | A1 | 3 projectiles natifs | Portée arme, une cible exposée | Dispersion −15 ; si au moins une touche, Stabilité contre 60 ; suppression 1 UT sur échec défensif. | Un seul test de suppression par action ; pas de suppression sur simple présence cachée. |
| TIR-04 | A1 de garde | Un projectile natif au déclenchement | Une ligne de 3 cases désignées, dans la portée/vision | Un tir simple de réaction sur première cible ennemie perçue qui entre. | Pas de tir hors perception ; détour, couvert, attente ou leurre. |
| TIR-05 | A1 | Arme | Portée arme, composant identifié | −20 Précision ; dégâts ordinaires assignés au composant, pas simultanément au corps. | Système de composants requis ; réduction/défaillance selon ses seuils réels, pas destruction garantie. |
| TIR-06 | A1 | 3 projectiles natifs | Jusqu'à 3 cibles perçues, distantes entre elles d'au plus 3 cases | Répartir exactement 3 projectiles, au moins 1 par cible ; dispersion −15. | Chaque trajectoire et défense résolues ; aucun tir supplémentaire. |
| TIR-07 | Amélioration de TIR-01 | Ceux du tir | Même cible | Après le tir visé, conserve +10 Précision pour les tirs simples suivants. | Fin dès mouvement, perte de vue, autre cible ou action autre que tir simple/attente ; ne croît jamais. |
| TIR-08 | Amélioration de TIR-04 | Ceux de Surveillance | Secteur de 90 degrés dans portée/vision | Remplace les 3 cases par le secteur ; un seul tir et une seule réaction. | Aucune vision ni portée supplémentaire ; mêmes réponses que Surveillance. |
| TIR-09 | P1+A1 | Arme + 4 E au tir | Portée arme, faiblesse connue | +4 pénétration physique pour ce tir, bornée à la moitié inférieure du Blindage encore présent après fragilisation. | Pas de bonus de toucher ; couvert/mouvement interrompent la préparation ; aucune réduction permanente. |
| TIR-10 | Deux étapes A1 | 3 projectiles par étape | 3 cases adjacentes connues, dans portée | Une balle par case à chaque étape ; jet ordinaire −15 contre occupant réellement exposé ; suppression comme TIR-03. | Immobilité entre étapes ; interruption annule la suite, pas les tirs passés ; alliés exposés. |

Le plafonnement de TIR-09 porte sur sa pénétration **supplémentaire** ; la propriété native de l'arme conserve son unité et son effet. Exemple B=5, aucune fragilisation : cette technique apporte +2, pas +4 ni ignorance totale. Le calcul ne révèle pas B dans un aperçu si cette valeur est inconnue.

### 16.4. Démolition — 10 profils

Lancer natif de référence : viser une case connue à 4 cases maximum, trajectoire libre. Si le profil possède une dispersion, chance de placement exact = borner(70 + 4 × (Coordination−5), 5, 95) ; sinon placement certain. Sur échec, dévier d'une case dans une direction choisie par le RNG, puis arrêter le projectile à la dernière case franchissable de sa trajectoire réelle. Pas de second jet d'esquive de zone. Les explosifs et récepteurs n'apparaissent pas par apprentissage.

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| DEM-01 | P1+A1 | Un explosif au lancer | Lancer natif, case connue | +20 à la chance de placement exact si dispersion native ; pas de bonus de dégâts. | Préparation interrompue avant lancer : objet conservé ; cible peut quitter la zone. |
| DEM-02 | P1+A1 | Outil ; 2 E à l'intervention finale | Piège identifié adjacent | Intervention certaine si déclencheur accessible et non armé contre manipulation ; sinon chance=borner(50+Analyse−difficulté_piège,5,95). | Échec final déclenche le piège armé contre manipulation ; mécanique annoncée par les indices connus, jamais un test gratuit de présence. |
| DEM-03 | P1+A1 | Une charge de brèche à la pose finale | Obstacle adjacent destructible | Charge armée : détonation après 1 UT, cycle de pose exclu ; 60 physiques sur cible, 12 physiques dans rayon 1. | Préparation interrompue : pas de charge posée ; une étape ordinaire de réponse après pose, sans garantir qu'un long désamorçage ou retrait ralenti tienne dans ce délai ; résistance réelle. |
| DEM-04 | P1+A1 | Une mine à la pose finale | Case adjacente, déclencheur rayon 1 | Armement après 1 UT, cycle de pose exclu ; capteur compatible ; explosion 20 physiques rayon 1. | Détection/désamorçage ; tout contact compatible peut déclencher une fois armée sauf filtre matériel explicite. |
| DEM-05 | P1+A1 | Un explosif configurable | Lancer/pose natifs, cône 90 degrés rayon natif | Même intensité dans le cône, aucune émission dans les autres directions. | Direction fixée avant armement ; matériel non configurable inéligible, alliés dans cône exposés. |
| DEM-06 | A1 | 2 E ; 1 B pendant commande | Un dispositif connu, liaison directe ≤6 | Déclenchement à la fin de la commande, propagation native. | Brouillage, obstacle ou récepteur coupé : pas de mise à feu ; l'échec ne renseigne pas ses voisins. |
| DEM-07 | Amélioration de DEM-02, puis A1 de récupération | 0 E supplémentaire ; outil | Piège adjacent déjà neutralisé | Conserve la charge intacte récupérable, au lieu du lot de pièces issu du même piège. | Piège marqué récupéré, stock fini ; mécanisme détruit ne devient pas intact. |
| DEM-08 | P1+A1 de programmation | 4 E ; 1 B pendant programmation | Jusqu'à 3 charges posées connues, liaison ≤6 | Fixer un délai de 1 à 3 UT par charge ; départ à l'installation effective du programme, cycle d'installation exclu. | Charges et pose payées séparément ; rupture pendant programmation annule les ordres non installés ; ne recrée pas une charge déjà déclenchée. |
| DEM-09 | P2+A1 | 2 charges structurelles au montage final | Support identifié adjacent ; zone structurelle ≤9 cases | À la fin du montage, annonce puis délai 1 UT, cycle d'annonce exclu ; 40 physiques par case autorisée par le support, puis débris prévus. | Système de supports obligatoire ; évacuation ou intervention si elle tient dans le délai, interruption du montage ; pas de tout-étage. |
| DEM-10 | P2+A1 | 2 charges compatibles au montage final | Obstacle adjacent ; deux impacts rayon 1 | Montage en C : première charge de brèche en fin de C+1, seconde explosive en fin de C+2 ; propagation recalculée. | Deux événements distincts ; réponse possible selon temps disponible et trajet ; sauvegarde sans redémarrage des délais ni double détonation. |

### 16.5. Manœuvre — 9 profils

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| MAN-01 | A1 de déplacement | 0 E | Une case accessible | +20 Esquive uniquement contre une interception de ce retrait ; maintien de la consigne inclus sans achat supplémentaire. | Chaque pas coûte son action ; attaque/autre posture rompt la consigne, aucun cumul du bonus. Ni interception ni tirs/mines/dangers supprimés ; charge lourde garde son temps. |
| MAN-02 | A1 de posture | 0 E | Soi | Ancrage +10 tant qu'immobile ; ne remplace pas la garde/réaction. | Déplacement volontaire ou forcé met fin ; n'atténue aucun dégât. |
| MAN-03 | P1+A1 | 4 E au franchissement | Obstacle bas ou intervalle d'une case, arrivée au plus à 2 cases | Franchissement certain avec corps compatible et trajet légal ; arrivée résolue normalement. | Nouvelle obstruction interrompt le franchissement ; pas de passage diagonal fermé ou de fosse arbitraire. |
| MAN-04 | Avance de 2–3 UT + frappe A1 + R1 | 2 E par case avancée puis Arme | Ligne droite ; cible initialement perçue | Une case par UT, puis frappe au contact, physiques ×1,25 ; la dernière avance laisse sa fenêtre adverse. | Obstacles/dangers interrompent la route ; cible partie : pas de suivi/attaque gratuite, R1 seulement si frappe exécutée. |
| MAN-05 | A1 de garde | 3 E si déplacement déclenché | Une case de repli adjacente choisie | Réaction d'esquive réelle, selon règles communes ; pas de second jet passif. | Repli occupé/inaccessible : pas de déplacement ; zone peut couvrir les deux cases. |
| MAN-06 | A1 | 8 E, +10 H ; CD 2 | Deux cases successives légales | Déplacement propulsé de deux cases ; chaque danger et Surveillance est résolu. | Propulseur, charge normale et sortie requises ; arrêt sur obstacle, coûts déjà engagés perdus. |
| MAN-08 | Amélioration de MAN-04 | Ceux des étapes faites | Charge en cours | Peut remplacer la prochaine avance par un arrêt A1 ; supprime R1 après une charge menée à son terme. | Freinage compatible ; collisions imprévues non annulées, énergie non rendue. |
| MAN-09 | P1+A1 + R1 | 10 E | Adversaire adjacent, une case de poussée | Force=Impact+4 dans plafond matériel ; si poussée possible, occupe l'ancienne case de la cible ; aucun dégât ajouté. | Masse/ancrage, fixation ou destination bloquent ; aucun passage à travers une file d'unités. |
| MAN-10 | P1+A1 | 6 E | Un allié adjacent, déplacement commun d'une case | Traction matérielle suffisante ; mouvement vers deux cases finales distinctes et légales. | Coopération/transportabilité requise ; dangers pour les deux, pas d'attaque ni action offerte à l'allié. |

### 16.6. Furtivité — 10 profils

Les modificateurs ci-dessous concernent une signature/canal, pas une invisibilité globale. Les bruits de référence utilisent des événements de force entière : déplacement 10, tir mécanique 30, explosion 50 ; propagation décroissante par case selon le terrain, bloquée par paroi fermée dans le profil local actuel. Les sons n'offrent pas de coordonnées exactes sans perception compatible. Seul le sous-système auditif est concerné par une diminution de bruit.

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| FUR-01 | Posture choisie avec le prochain déplacement | 0 E ; déplacement 2 UT | Soi | Bruit de déplacement −10, minimum 0 ; pas de réduction optique. | Quitter la posture avec une autre commande normale ; équipements bruyants restent audibles. |
| FUR-02 | Passif conditionnel | 0 | Soi | +10 difficulté de repérage optique lors d'un déplacement sous couvert partiel réel. | Aucun effet à découvert ou contre un canal non optique. |
| FUR-03 | A1 activation/désactivation | 0 E | Équipements émetteurs sélectionnés | Émission des fonctions mises en veille à zéro ; fonctions actives associées indisponibles jusqu'à réactivation. | Visuel, chaleur et mémoire inchangés ; aucun redémarrage gratuit lors d'une attaque. |
| FUR-04 | A1 de posture | 0 E ; déplacement 2 UT | Soi, couvert bas | +15 difficulté de repérage optique avec corps et couvert compatibles. | Exclut Pas feutrés ; armes volumineuses déclarées incompatibles indisponibles ; aucun bonus à découvert. |
| FUR-05 | P1+A1 | Arme | Cible perçue n'ayant pas localisé l'attaquant | +20 Précision, physiques ×1,25 pour une première attaque simple. | Détection avant exécution annule le bonus ; l'attaque reste possible comme attaque ordinaire avec ses coûts. |
| FUR-06 | A1 | Un leurre sonore consommable | Lancer 4 cases, bruit pendant 3 UT | Source de bruit 30 dans la case posée, à chaque phase éligible. | Ne garantit pas une investigation ; ennemi en contact peut ignorer ; destruction arrête la source. |
| FUR-07 | A1 de déplacement puis jusqu'à 2 pas suivants | 3 E à l'activation ; CD 4 | Soi, après rupture de vue | Aucune nouvelle trace de déplacement pendant ces 3 pas, à réaliser en 3 UT maximum. | Ne supprime pas traces passées/dernière position connue ; reprise de contact annule. |
| FUR-08 | P1+A1 | Un lot de camouflage | Petit dispositif adjacent posé | Difficulté de repérage optique +20 jusqu'à déplacement, choc dommageable ou retrait du camouflage. | Émissions actives intactes ; inspection/capteurs compatibles peuvent découvrir. |
| FUR-09 | A1, maintien 3 UT maximum | 10 E activation puis 4 E/UT ; +4 H/UT ; CD 4 | Soi, un canal prévu par module | +30 difficulté de repérage du seul canal masqué. | Fin sur attaque, activation offensive énergivore, énergie insuffisante ou expiration ; autres canaux inchangés. |
| FUR-10 | Amélioration de FUR-05 | Arme + 4 E | Contact, cible vulnérable identifiée | Remplace le ×1,25 par ×1,5 physique ; bruit de cette frappe −20, minimum 0. | Pas de mise à mort automatique : survie et témoins permettent l'alerte ; une cible intacte peut résister. |

### 16.7. Reconnaissance — 7 profils

Une lecture déterministe conserve sa conclusion et sa provenance. Les bonus permettent d'atteindre des seuils accessibles, pas de franchir les obstacles. Les résultats physiques utiles à Tir de rupture peuvent aussi provenir d'observations natives ou d'un outil : REC-01 n'est pas un péage obligatoire.

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| REC-01 | A1 | 0 E | Une cible perçue à portée de capteur | Analyse +15 pour propriétés physiques, état, équipements observables et résistances accessibles. | Seuil non atteint : observation partielle ; aucune donnée inaccessible créée ; pas de bonus de combat. |
| REC-02 | A1 | 0 E | Traces observables dans rayon 3 | Détection +10 puis Analyse +15 pour direction/ancienneté accessibles. | Traces réellement présentes, au plus 3 par case, durée 12 UT ; pas de poursuite en direct. |
| REC-03 | P1+A1 | 0 E | Cases observables de rayon 3 | Détection +20 contre seuil des secrets ; découverte seule. | Aucun jet renouvelé ; ouverture et désamorçage séparés, accès essentiel alternatif. |
| REC-04 | A1 | 0 E | Jusqu'à 3 cases de paroi observées adjacentes entre elles | Analyse +15 sur matériau, résistance ou dégradation réelle. | Ne produit ni faiblesse nouvelle ni connaissance de supports absents. |
| REC-05 | A1 | 0 E | Une cible perçue | Analyse +20 sur capacités offensives/défensives observables et leurs contraintes. | Ne lit pas le prochain choix de l'IA, les poches ou un programme caché sans source. |
| REC-09 | Amélioration de REC-01 | 2 E | Jusqu'à 3 cibles simultanément perçues | Une seule A1 d'analyse, même +15 sur chacune. | Même connaissance requise pour chaque cible ; pas d'hypothèse d'équipement identique. |
| REC-08 | A1 | 2 E | Une machine perçue, canal de diagnostic adapté | Analyse +20 sur chaleur, alimentation et stockage réellement inspectables. | États absents : technique indisponible à l'achat ; aucune cartographie automatique du réseau. |

### 16.8. Ingénierie — 10 profils

Un composant possède sa propre durabilité lorsque le système existe. Le réparer ne soigne pas une deuxième fois le corps ; sa destruction ou son retrait n'efface pas les apprentissages. Outils, pièces et atelier ne sont pas créés par la technique. Les recettes référencées ci-dessous constituent un petit profil d'essai fermé, pas le futur catalogue complet de fabrication.

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| ING-01 | P1+A1 | Un lot de pièces de réparation à l'étape finale | Soi ou composant allié adjacent | Restaure jusqu'à 20 points de durabilité du composant choisi, sans réparer le corps en plus. | Pièce compatible et composant réparable ; interruption conserve le coût déjà engagé, système requis. |
| ING-02 | P2+A1 | Outil ; pas de création de matière | Une carcasse adjacente | Récupère un composant survivant choisi ; retire ce composant du rendement en pièces. | État réel préservé, carcasse vidée de sa sortie ; aucune recréation d'un composant détruit. |
| ING-03 | A1 | Outil, 2 E | Soi ou module adjacent | Analyse +20 sur cause matérielle et procédure accessible. | Un besoin de pièce peut rester non résolu ; ne purge pas un logiciel. |
| ING-04 | P2+A1 | Un lot de réglage, atelier ou kit | Un module compatible accessible | Un réglage : économie (coût E ×0,8, sortie ×0,9) ou puissance (sortie ×1,1, coût E ×1,25). | Arrondir coût E supérieur, sortie inférieure ; un emplacement, remplace ancien réglage sans cumul ni récupération du lot. |
| ING-05 | Amélioration de ING-01, A1 | Un lot de pièces | Un composant compatible accessible | Restaure 10 au lieu de 20, en une action. | Rendement inférieur, pas de résurrection du composant détruit ; soins ordinaires du corps toujours accessibles. |
| ING-06 | A1 activation, 3 UT | 5 E activation ; coût E d'usage ×1,5, chaleur d'usage +8 | Un module avec sortie chiffrée compatible | Sortie ×1,25 ; autorise H projetée jusqu'à 140 ; chaque usage à H>100 après apport retire 2 points de durabilité au module. | Choix de sortie déclaré (dégâts ou rendement), pas temps/portée/cibles ; arrêt possible A1, dégradation réelle non réparée à la fin. |
| ING-07 | P1+A1 | 4 E ; sous-système donneur suspendu | Deux modules du même corps | Rétablit une fonction électrique dégradée à 50 % de sa sortie, jusqu'au rétablissement du donneur ou retrait. | Diagnostic crédible et chemin matériel réel ; pièce absente/incompatible exclue ; sacrifice d'une autre fonction. |
| ING-08 | P4+A1 | Atelier, 2 lots de pièces | Un équipement récupérable | Restaure jusqu'à 50 points de durabilité propre, sans dépasser l'état maximal réparable du profil. | Temps d'atelier, composants irrécupérables exclus ; démontage ultérieur ne rend pas les pièces ajoutées. |
| ING-09 | P2+A1 | Plan, outil, 2 lots de pièces + batterie 20 E | Une case adjacente libre | Recette d'essai : balise-leurre 10 points de durabilité, 20 E, 2 E/UT, bruit 30 ; cesse à épuisement. | Consommations réelles, pas d'XP à sa destruction ; récupération au plus des entrées restantes, jamais batterie rechargée. |
| ING-10 | Amélioration de ING-06 | Activation 5 E ; coût E ×1,25, chaleur +4 par usage | Même module | Remplace sortie ×1,25 par ×1,15 ; pas de dégradation du surcadencement tant que H≤120. | Au-delà de 120, même perte de 2 au module ; seuil volontaire 140 conservé, dégâts thermiques du corps inchangés. |

La « sortie » d'un module est un paramètre identifié par son profil, jamais tous ses nombres à la fois. Un outil sans coût E natif ne reçoit pas gratuitement les modes ING-04/06 : il doit offrir un profil alimenté compatible. Les configurations de référence ne font pas de l'Ingénierie une obligation de restaurer les PV du personnage.

### 16.9. Intrusion — 10 profils

Les actions sur un accès acquis ne refont pas de jet hostile sauf contestation explicitement indiquée. Le maintien 1 B d'une session est celui de la section 13, pas une seconde réservation cachée à chaque commande ; les suppléments sont nommés. Les actions de commande ne font pas tirer la tourelle hors de son tour. Les sessions, preuves et droits n'accordent aucune position ennemie hors perception.

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| INT-01 | A1 | 2 E | Une interface connue, liaison ≤4 | Analyse +15 sur interface/droits/protections observables ; pas d'accès accordé. | Sonde produit la trace prévue par le profil ; pas de mot de passe caché révélé. |
| INT-02 | P1+A1 | 6 E à la tentative finale, 1 B pendant procédure | Un verrou électronique local, liaison ≤4 | Un jet logiciel ; réussite commande son ouverture une fois, sans droit global. | Échec : temps/coûts/trace et durcissement ; clé, autre accès ou brèche restent des réponses possibles. |
| INT-03 | P1+A1 | 4 E | Une source avec droit de lecture, liaison ≤4 | Extrait un lot de données identifié, avec source et date persistantes. | Rupture avant fin : pas de lot partiel exploitable dans ce profil ; copie ne crée pas de nouvelles XP. |
| INT-04 | A1 | 4 E | Un système, identifiant obtenu, liaison ≤4 | Autorisation locale limitée à ses droits pour 4 UT ou jusqu'à invalidation du justificatif. | Vérification des justificatifs déterministe ; pas de jet pour « fabriquer » des droits inconnus ; témoins intacts. |
| INT-05 | A1 | 4 E, 1 B de contrôle en plus de la session | Un dispositif avec droit de commande, liaison ≤4 | Une consigne simple maintenue jusqu'à 3 UT ou rupture. | Le contrôleur adverse peut reprendre si non verrouillé ; le dispositif agit à son propre rythme. |
| INT-06 | A1 | 6 E ; CD 3 | Une routine nommée sur système accessible ≤4 | Suspend cette fonction 2 UT, sans second jet si le droit suffit. | Pas de neutralisation globale ; reprise après fin, protection de famille 1 UT ; rupture met fin au maintien. |
| INT-07 | P1+A1 | 8 E installation ; un emplacement d'accès dormant | Un système déjà compromis ≤4 | Garde un justificatif de reprise locale ; maximum 2 portes dérobées, fin au changement de couche. | Réinitialisation/audit peut supprimer ; reconnexion A1/2 E, liaison et droits toujours vérifiés, sans vision offerte. |
| INT-08 | P1+A1 | 6 E | Un événement local identifié, droit de modification ≤4 | Modifie cette preuve avant examen ; audit de référence 5 UT depuis création de la trace, calendrier §13.3. | Droit acquis : pas de nouvelle tentative d'accès ni trace d'intrusion récursive ; copies/témoins/audits antérieurs subsistent ; sécurité active requise. |
| INT-09 | P2+A1 | 12 E, 1 B de contrôle par dispositif | Jusqu'à 3 dispositifs d'un même sous-réseau connu, chacun joignable ≤4 | Une commande de groupe, droits réellement obtenus pour chaque nœud ; maintien 3 UT. | Hors capacité/liaison/droits, aucun contrôle magique ; version locale sans traversée de mur ; pas d'accès hérité d'un simple nom de réseau. |
| INT-10 | A1 | 8 E, +1 B pour le verrou | Un dispositif déjà détourné ≤4 | Bloque 3 UT les reprises de commande ordinaires de son contrôleur ; pas une nouvelle attaque. | Rupture, réinitialisation ou action physique restent efficaces ; indisponible sans reprises adverses ; CD 4. |

### 16.10. Guerre électronique — 8 profils principaux

Les dégâts des émissions viennent du matériel. Les programmes offensifs GEL-02/06/08 comprennent leur procédure d'implantation standard et ne demandent pas l'achat d'Intrusion. Chaque implantation hostile teste Défense numérique une fois ; les conséquences périodiques n'ajoutent pas le même test à chaque tic. Les défenses physiques ou thermiques restent applicables à leurs dégâts propres.

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| GEL-01 | A1 | 20 E, +15 H ; CD 2 | Disque rayon 2 autour de soi, source exclue | 16 électriques par cible compatible exposée ; si émetteur perturbateur, interruption contre intensité 55. | Murs/portes bloquent ; alliés touchés, résistance électrique et espacement ; Stabilité seulement si préparation concernée. |
| GEL-02 | A1 | 12 E, +4 H, 1 B pendant tentative ; CD 3 | Une machine compatible ≤4 | Jet logiciel ; succès : +15 H et dissipation réduite de 3 (minimum 0) à chaque phase, pendant 3 UT. | Purge ou arrêt local du module thermique compromis met fin ; aucune chaleur si cible incompatible ; pas de liaison nécessaire après implantation. |
| GEL-03 | A1, maintien 3 UT maximum | 6 E activation puis 3 E/UT ; +2 H/UT ; 1 B | Disque rayon 2, un canal | Capteur choisi : Détection −20 ; ou liaison : puissance de liaison −20 contre seuil matériel, pas les deux. | Effet sans jet logiciel, obstacles et compatibilité ; autres canaux utilisables, quitter zone/détruire brouilleur ; source émet elle-même. |
| GEL-04 | A1 | 6 E | Soi ou allié joignable ≤2 | Un jet logiciel de nettoyage, bonus +20 contre force du programme stockée ; retire un programme nommé en cas de succès. | Échec coûte l'action ; ne soigne ni ne refroidit ; procédure native de nettoyage/atelier disponible sans cette technique. |
| GEL-05 | A1 | 22 E, +15 H ; CD 3 | Première cible perçue ≤4, jusqu'à 2 rebonds de ≤2 cases | 16, 12 puis 9 électriques ; une visite par cible, chaque trajet non obstrué ; ordre choisi parmi cibles perçues. | Chaîne s'arrête si saut invalide, pas de remboursement ; espacement/isolation et résistance électrique. |
| GEL-06 | A1 | 16 E, +6 H ; 1 B à l'implantation ; CD 4 | Une machine perçue ≤4, propagation locale ≤2 | Jet logiciel ; sabotage local : 4 thermiques bruts par UT, 3 tics max, sans hausse automatique de la jauge H ; transmission détaillée ci-dessous. | Purge/isolation ou coupure de l'interface compromise ; résistance thermique sur chaque tic, aucun dégât « Corruption » ajouté. |
| GEL-07 | P1+A1 | Une balise avec batterie propre de 30 E | Pose adjacente ; disque rayon 2 | Balise 20 points de durabilité ; 8 électriques par exposition admissible, 3 UT ; dépense 10 E de balise/UT. | Pas d'E du porteur après pose ; obstacle/sortie/destruction ; alliés exposés ; même famille de champs non multipliée dans une UT. |
| GEL-08 | P1+A1 d'implantation puis retard 2 UT | 24 E, +10 H ; 1 B pendant implantation ; CD 6 | Stockage identifié compatible ≤4 ; rayon 1 à la détonation | Jet logiciel ; arme un stockage contenant au moins 20 E, réservé sans duplication ; cycle d'implantation exclu du retard ; 30 physiques + 20 thermiques, source comprise. | Signe avant-coureur obligatoire ; deux étapes ordinaires de réponse, nettoyage natif possible si ses conditions sont remplies mais succès non garanti ; purge/déconnexion/déplacement, ni mort automatique ni gravité. |

Infection : une campagne possède un identifiant, l'attaquant, sa force logicielle initiale, un maximum de **3 hôtes**, un ensemble d'hôtes déjà tentés et une expiration absolue 6 UT après l'implantation initiale. Après son premier tic, chaque hôte peut tenter **une seule** transmission sur un voisin compatible réellement accessible de rayon 2, sans mur/porte fermé ; un jet logiciel par nouvel hôte contre sa défense actuelle. Choix par distance puis identifiant stable, sans préférer une faiblesse cachée. Une cible tentée n'est pas retentée par la même campagne ; à défaut de candidat lors de cette unique occasion, pas de recherche répétée infinie. Chaque hôte reçoit au plus 3 tics et jamais au-delà de la fin globale. Les transmissions échouées n'augmentent pas le nombre d'hôtes réussis, mais restent enregistrées.

Les transmissions autonomes peuvent toucher un voisin hors perception du joueur ; elles n'en révèlent ni l'identité ni la position dans le rendu. Toute transmission suit la ligne d'effet locale et ne dépend pas d'une vue à travers un mur. Il s'agit d'une propagation, pas d'une nouvelle commande ciblant une entité cachée. L'identifiant de campagne et ses bornes subsistent après purge du premier hôte ; pas de retour au même hôte pour redémarrer la chaîne.

Implosion : les 20 E sont prélevés et placés dans le stockage armé lors de l'implantation réussie ; si purge/annulation, ils sont dissipés et non restitués. Un stockage absent, vide ou incompatible ne devient pas explosif. Si le stockage est détruit avant le délai, annuler le programme avant de résoudre l'éventuel effet natif de destruction du stockage, une seule fois. Déplacer la cible déplace le centre futur, sans donner un suivi visuel hors perception.

### 16.11. Guerre électronique — 10 variantes

Chacune coûte **un apprentissage** supplémentaire et demande sa mère, selon la table qualitative. Une seule variante par exécution ; toutes les restrictions de la mère restent présentes. Les profils ci-dessous ne sont pas dix nouveaux pouvoirs autonomes.

| Code | Mère | Temps | Coûts modifiés | Transformation chiffrée | Contrepartie / fin |
|---|---|---|---|---|---|
| GEL-V01 | GEL-01 | Identique | Identiques | Remplace disque par cône de 90 degrés, rayon 2, mêmes 16 électriques. | Couverture réduite ; aucun gain de portée ou de puissance. |
| GEL-V02 | GEL-01 | Identique | 26 E au lieu de 20 ; 1 B pendant émission | Exclut les alliés reconnus par identifiant matériel valide au moment de l'émission. | Non identifiés/neutres non protégés ; le filtre n'accède pas aux intentions. |
| GEL-V03 | GEL-02 | Identique | 16 E, +6 H | +25 H par tic, dissipation −3, durée 2 UT. | Pic supérieur, durée réduite, mêmes défenses et purge. |
| GEL-V04 | GEL-02 | Identique | 14 E, +4 H | +10 H par tic, dissipation −3, durée 5 UT. | Montée lente, davantage de temps pour répondre. |
| GEL-V05 | GEL-05 | Identique | 28 E, +18 H | Un quatrième hôte ; dégâts 16, 12, 9, 6. | Un rebond seulement en plus, mêmes distances, visibilité et unicité. |
| GEL-V06 | GEL-05 | Identique | 30 E, +20 H | Trois hôtes maximum ; dégâts 16, 14, 12. | Coût énergétique/thermique supérieur, pas de portée supplémentaire. |
| GEL-V07 | GEL-06 | Identique | 22 E, +8 H | Maximum 4 hôtes ; deux tentatives de transmission distinctes par hôte à son occasion unique ; 2 thermiques bruts/tic. | Force stockée identique ; durée globale 6 UT et 3 tics par hôte inchangées. |
| GEL-V08 | GEL-06 | Identique | 18 E, +8 H | Un seul hôte, aucune propagation ; 8 thermiques bruts/tic, 3 tics. | Forte dépense sur une cible, purge identique, pas de contagion. |
| GEL-V09 | GEL-07 | Identique | Même batterie 30 E, 6 E/UT | Durée 5 UT, dégâts 5 électriques par exposition. | Intensité réduite ; total théorique 25 contre 24 pour la mère, mais immobilisation de zone plus longue. |
| GEL-V10 | GEL-07 | Pose identique puis A1 de commande | +2 E et 1 B pendant activation | Pose inactive ; activation par liaison directe ≤6 ; ensuite 3 UT comme mère. | Balise vulnérable avant activation ; aucune nouvelle batterie au réarmement, activation unique. |

Le typage thermique d'Infection est une proposition de formalisation du sabotage matériel progressif, pas un nouveau nom de dégâts ni une décision finale. Il la distingue de Surchauffe, qui agit sur la jauge thermique et la dissipation : Infection inflige directement de petits dégâts périodiques après résistance. Les variantes conservent ce mécanisme ; leur intérêt face à l'arrondi et aux fortes résistances demande un essai ciblé.

### 16.12. Contrôle de drones — 10 profils

Un ordre transmis coûte l'action du donneur. Son exécution consomme les actions et ressources normales des drones. Profil de liaison d'essai : puissance 50, difficulté 40 ; liaison disponible si puissance après brouillage ≥40 et si portée/obstacles autorisent la liaison. Un drone hors liaison conserve ses 1 B réservés tant que le propriétaire ne le libère pas par une action de contrôle ; cela évite de dépasser le nombre d'unités par coupures volontaires. Les équipements restent persistants et les unités libérées ne reçoivent plus d'ordres avant réacquisition d'un emplacement.

| Code | Temps / type | Coûts | Portée / cibles | Effet chiffré proposé | Échec, fin et contre-mesure |
|---|---|---|---|---|---|
| DRN-01 | A1 | 10 E, +3 H, 1 B réservée ; CD 3 | Une case adjacente libre ; une manifestation active | Manifeste le drone utilitaire de base avec une escorte à distance 1 et un tir d'assistance sur la cible attaquée par le joueur. | Déplacement et attaque utilisent ses actions propres ; perception, portée, ligne de tir, énergie et précision normales. À 0 énergie, il se dissipe, libère B et peut être manifesté de nouveau après le CD. |
| DRN-02 | P1+A1 | 3 E, 1 B unité | Un drone, trajet connu de 6 points maximum | Routine de patrouille ; arrêt ou retour fixé à la préparation. | Obstacle nouveau : arrêter/revenir suivant règle, pas de navigation omnisciente ; routine continue hors liaison. |
| DRN-03 | A1 de commande | 2 E donneur ; dispositif drone 3 E/UT, maximum 3 UT | Un drone avec leurre, destination connue ≤6 à la commande | Émet bruit/signature 30 à la position atteinte ; pas de déplacement instantané. | L'ennemi choisit sa réponse ; drone exposé, arrêt si énergie insuffisante. |
| DRN-04 | A1 de commande | 2 E, 1 B unité | Un objet connu, trajet autorisé | Ramassage A1 du drone puis retour ; masse et capacité réelles. | Objet disparu : rapport daté au prochain contact ; aucune téléportation vers l'inventaire joueur. |
| DRN-05 | A1 de commande | 4 E ; +1 B pendant transmission | Jusqu'à 2 drones en liaison ≤6, une cible perçue | Désigne la même cible pour leur prochaine attaque ordinaire ; aucune attaque immédiate. | Chacun exige perception/trajectoire/ressources au moment d'agir ; pas de suivi d'une cible cachée. |
| DRN-06 | A1 de commande | 2 E donneur ; 3 E du drone au déclenchement | Un drone de protection et un allié proche | Le drone prépare sa garde à sa prochaine action normale ; une interposition de réaction selon règles communes. | Case/trajectoire libre, perception locale et réaction requises ; ni téléportation ni absorption de toute zone. |
| DRN-07 | Amélioration de DRN-02 | 4 E au lieu de 3 ; +1 B réservé pour routine | Un drone équipé | Autorise jusqu'à 6 cases nouvelles au-delà du trajet connu, puis retour ; réserve énergétique de retour contrôlée. | Arrêt devant obstacle/danger détecté ; aucune carte live, seulement rapport daté à la reconnexion. |
| DRN-08 | P1+A1 de programmation | 3 E ; +1 B pendant routine | Un drone compatible | Une condition et une action locale : PV sous 40 %, énergie sous 25 % ou danger perçu → arrêt/repli/protection. | Pas de boucles imbriquées, d'information globale ni d'action en plus ; un seul tel supplément B si déjà facturé par DRN-07. |
| DRN-09 | P1+A1 de commande | 6 E ; +1 B pendant transmission | Jusqu'à 2 drones joignables, deux positions/rôles connus | Assigne deux consignes en une procédure ; chacun rejoint sa position lors de ses actions normales. | Routes réelles, unités non joignables non commandées ; pas de formation instantanée. |
| DRN-10 | A1 de commande | 5 E ; +1 B pendant transmission ; CD 3 | Jusqu'à 2 drones joignables ≤6 | Remplace les consignes offensives par retour vers un point connu pendant 3 UT, puis attente. | Drones encerclés non sauvés automatiquement ; tirs ennemis et coûts de déplacement conservés. |

### 16.13. Relecture et validations restantes

Les coefficients sont centralisables en contenu lors de l'implémentation ; le texte de cette annexe n'est pas un format à parser directement dans le jeu. Le contrôle documentaire vérifie sa couverture et ses références. Le rapport séparé compare les budgets et quelques résultats arithmétiques, puis énumère les scénarios jouables à exécuter : cela ne prouve ni l'équilibrage du catalogue ni la présence des systèmes dans le prototype.

Points sensibles restant à éprouver : récupération et préparation de mêlée, coût des achats tardifs de Reconnaissance, pression thermique de Surchauffe, petits dégâts périodiques d'Infection, rendement de fabrication et multiplication des réactions par les drones. Le seul gain ergonomique de MAN-07 est désormais fusionné dans MAN-01 par accord utilisateur ; son ancienne faiblesse ne constitue plus un achat à équilibrer.

**Étape 7 terminée au niveau rédactionnel :** cette annexe reste la référence technique, pas le texte d'interface. Les quinze validations individuelles figurent en section 19.1 des [règles communes](STATISTIQUES_ET_COMPETENCES.md) ; la délégation explicite autorisant la rédaction du reste est suivie en section 19.3. Les [textes joueur](TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md) fournissent les descriptions, infobulles, refus et notifications complémentaires. Ni les coefficients ni les capacités extérieures ne sont modifiés par cette rédaction ; aucun texte n'est intégré automatiquement au jeu.
