# Project RL — Statistiques et compétences : règles communes

Version 0.6 — 10 septembre 2026 — Cinq corrections de principe validées au point 8 ; autres barèmes d'essai conservés, validation en jeu à venir.

Ce document complète le [catalogue des compétences](PROPOSITION_COMPETENCES_v0.1.md). Il rassemble les bases confirmées puis propose une formalisation des statistiques secondaires. Il ne constitue ni un équilibrage final ni une description de systèmes déjà implémentés.

La révision 0.6 applique l'accord utilisateur « oui :) » donné aux cinq corrections de la revue finale : actions non chauffantes préservées, retardateurs laissant une réponse, audit de référence à 5 UT, fusion de Retraite méthodique dans Pas de dégagement et ouverture conditionnelle des disciplines complètes. Le suivi précis figure en section 19.4 ; les autres paramètres et arbitrages ne deviennent pas confirmés par cet accord.

## 1. Périmètre et statut

La présente étape concerne uniquement les cinq primaires, les statistiques secondaires, le matériel et les dix compétences du système actuel. À la demande de l'utilisateur, les capacités extérieures à débloquer en jeu sont reportées à beaucoup plus tard. Leur discussion antérieure est conservée dans les documents fondateurs ; aucune nouvelle règle de ces capacités n'est développée ici.

| Statut | Signification |
|---|---|
| Confirmé | Décision déjà inscrite dans le catalogue ou explicitement confirmée par l'utilisateur. |
| Proposition à valider | Formalisation présentée pour discussion ; elle ne devient pas une règle approuvée par sa seule présence dans ce document. |
| À définir / à tester | Question encore ouverte ou valeur nécessitant des essais. |

Les sections 2 et 3 rappellent les bases confirmées. Les sections 4 à 8 constituent une proposition à valider, sauf décisions explicitement identifiées comme confirmées : présence du Blindage (armure) et possibilité d'absorber totalement une attaque trop faible, sans minimum automatique de 1 dégât. La section 9 indique les décisions ouvertes. Les sections 10 et 11 décrivent les premiers barèmes physiques ; le rôle de Puissance et Résilience et l'augmentation de capacité sans réparation automatique sont retenus comme base de travail, avec chiffres à éprouver. Les principes de défense et d'interruption de la section 12 sont également retenus après accord utilisateur ; leurs paramètres et modalités détaillées restent à éprouver ou à préciser. Les fiches du catalogue restent la référence pour les techniques ; un accord de direction ne leur ajoute pas implicitement un bonus.

L'utilisateur a demandé d'enchaîner les étapes documentaires 1 à 6 et 8. Les sections 13 à 19 et l'annexe 16 du catalogue constituent cette livraison : des règles et paramètres proposés, pas une approbation implicite de leurs chiffres. Les mentions antérieures « reste à définir » retracent les étapes de discussion ; les sections nouvelles les complètent, sans changer rétroactivement leur statut. Les capacités extérieures restent reportées. **Étape 7 terminée au niveau rédactionnel :** les quinze premières formulations ont été validées individuellement en section 19.1 ; l'utilisateur a ensuite autorisé la rédaction de tous les textes restants sans validation phrase par phrase. Ce complément figure dans les [textes joueur](TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md). Cette délégation ne vaut ni validation des barèmes ni intégration au jeu.

## 2. Vocabulaire commun

| Terme | Fonction dans le système actuel |
|---|---|
| Statistique primaire | Aptitude générale du personnage, choisie à la création puis développée pendant la partie. |
| Statistique secondaire | Valeur exprimant l'efficacité ou une capacité résultant du personnage, de son matériel et, lorsque c'est pertinent, de la situation. |
| Capacité matérielle | Propriété du corps, d'un équipement ou d'un module : réserve énergétique, blindage, portée d'une arme, etc. |
| Ressource actuelle | Quantité disponible à cet instant, distincte de son maximum : PV restants, énergie disponible, munitions. |
| Compétence / discipline | L'une des dix progressions à rangs, par exemple Tir ou Intrusion. « Discipline » sert ici à la distinguer de ses techniques, sans imposer un renommage de l'interface. |
| Rang | Niveau d'investissement dans une compétence, de 0 à 5. |
| Technique | Action, posture, procédure ou comportement appris grâce à un choix de rang. |
| Amélioration de technique | Choix qui modifie une technique connue et demande celle-ci en prérequis. |

Une valeur de Précision n'est donc pas une technique de Tir. Apprendre Tir visé fournit une préparation particulière ; la Précision exprime une partie de son efficacité avec l'arme utilisée.

Terminologie validée par l'utilisateur pendant l'étape 7 : **Points de vie (PV)** pour la réserve actuelle et le maximum des personnages et créatures, organiques ou mécaniques ; **Durabilité** pour les équipements, composants et objets destructibles. Ces termes remplacent l'ancien libellé « Intégrité » lorsqu'il désignait ces réserves. Résilience reste une statistique primaire distincte ; Constitution n'est pas ajoutée. La notation de conception utilise `PV_actuels` et `PV_max`. Les valeurs, coûts, formules et règles de destruction ne changent pas ; une unité mécanique conserve ses moyens de réparation compatibles.

Ce remplacement ne concerne pas l'intégrité de la mémoire ou des données. Les identifiants existants du code, comme `integrity`, `maximum_integrity` et `IntegrityRestored`, restent inchangés à ce stade ; les exemples de code et constats techniques doivent rester fidèles au prototype. Les libellés du client ASCII seront harmonisés dans une étape d'implémentation dédiée, sans migration silencieuse des sauvegardes.

L'origine matérielle d'une valeur n'empêche pas de la présenter comme une secondaire du personnage : les pièces d'armure contribuent au Blindage calculé. Il s'agit d'une même protection, pas de deux défenses à appliquer successivement.

## 3. Bases confirmées

### 3.1. Personnage et statistiques primaires

Le personnage garde le même corps principal pendant toute la partie. Son équipement et ses améliorations évoluent. Changer d'équipement ne supprime pas ses apprentissages ; une technique connue peut devenir inutilisable si son matériel nécessaire manque.

| Primaire | Rôle déjà retenu | Limite déjà retenue |
|---|---|---|
| Puissance | Impact, exploitation des actuateurs, maîtrise du recul, déplacements forcés. | Masse, ancrage et capacités physiques du matériel restent déterminants. |
| Coordination | Précision, manipulation mécanique, déplacement contrôlé et parade. | Ne fournit pas automatiquement une accélération générale. |
| Résilience | Maintien opérationnel, résistance aux interruptions et perturbations. | Ne remplace ni le blindage ni les ressources de réparation. |
| Perception | Détection, observation, ciblage et interprétation des indices. | Ne donne pas de vision à travers les murs ; les informations essentielles restent accessibles sans spécialisation. |
| Traitement | Intrusion, procédures numériques, analyse et commandes complexes. | Ne crée pas d'énergie ou de bande passante et n'améliore pas tous les paramètres d'une attaque à la fois. |

Influence est retirée des primaires actuelles.

Règles de création confirmées : échelle de 1 à 10, valeur ordinaire de 5, somme de 28 points répartis entre les cinq primaires, chacune entre 3 et 8 à la création, plafond absolu de 10. Exemple de répartition valide, sans constituer une classe : Puissance 6, Coordination 6, Résilience 6, Perception 5, Traitement 5.

La fréquence des augmentations de primaires et le traitement détaillé des modificateurs temporaires restent à définir ; aucun dépassement du plafond de 10 n'est introduit ici.

### 3.2. Progression et accès aux techniques

Les dix compétences sont Combat rapproché, Tir, Démolition, Manœuvre, Furtivité, Reconnaissance, Ingénierie, Intrusion, Guerre électronique et Contrôle de drones.

- Chaque rang acheté donne un choix de technique ou d'amélioration éligible ; les choix antérieurs restent accessibles.
- La base actuelle limite chaque compétence à cinq choix, y compris ceux accordés au départ par une classe.
- Les fonctions ordinaires du matériel restent accessibles selon ses règles, sans apprentissage obligatoire d'une technique spécialisée.
- L'expérience et la progression du noyau appartiennent à la partie ; les classes débloquées relèvent d'une progression séparée.
- Le barème de rangs 1, 1, 2, 2, 3 points et l'hypothèse d'un point par niveau restent des paramètres d'essai, pas un équilibrage acquis.

La présence éventuelle d'un bonus numérique automatique lié au rang lui-même n'est pas tranchée ici. Il ne faut pas en ajouter un silencieusement aux bénéfices de chaque technique.

### 3.3. Défense physique confirmée

L'utilisateur demande une statistique secondaire d'armure pour la défense contre les attaques « normales ». Elle figure dans la liste sous le libellé Blindage (armure), cohérent avec les techniques existantes Brise-armure et Tir de rupture. C'est une valeur défensive du personnage, pas une nouvelle discipline à apprendre. Son calcul et la liste précise des types de dégâts couverts restent à définir.

L'utilisateur a ensuite confirmé qu'une protection suffisante peut absorber entièrement une attaque trop faible : il n'y a pas de minimum automatique de 1 dégât. L'objectif exprimé est un jeu qui ne soit ni facile ni impossible à gagner. Cet accord valide le principe d'absorption totale et la direction d'équilibrage, pas tous les coefficients de toucher ni l'ensemble du modèle chiffré proposé. Les pistes de mise en œuvre figurent en section 10.8.

## 4. Secondaires : proposition de définition

Les contributions ci-dessous décrivent des rôles, pas encore des formules. Une secondaire n'est pas nécessairement sur une échelle de 1 à 10 : unité, bornes, coefficients et arrondis devront être fixés selon ce qu'elle mesure. Un score de Précision n'est pas, à lui seul, un pourcentage de réussite contre toutes les cibles.

| Secondaire et statut particulier | Ce qu'elle exprime | Contributions envisagées | Ce qu'elle ne doit pas faire |
|---|---|---|---|
| Impact physique | Efficacité physique d'une frappe ou d'une poussée, calculée pour le geste et le matériel employés. | Puissance, actuateurs, arme ou outil, posture. | Augmenter tous les dégâts du jeu, ceux des balles ou ceux d'une grenade par simple hausse de Puissance. |
| Charge utile | Charge que le personnage peut transporter dans les conditions prévues. | Structure du corps, portage et assistance ; contribution bornée de Puissance dans les limites matérielles. | Créer des emplacements d'équipement ou permettre à une structure incapable de porter une masse de le faire sans amélioration matérielle. |
| Précision | Aptitude à atteindre une cible avec l'attaque considérée. | Coordination, précision de l'arme, préparation ; contribution ciblée de Perception. | Ignorer couvert, portée, trajectoire ou impossibilité de percevoir la cible. |
| Esquive | Aptitude à éviter certaines attaques ciblées lorsqu'un évitement est possible. | Coordination, locomotion, encombrement, état et posture. | Déplacer gratuitement le personnage ou annuler toute une zone de dégâts. |
| Blindage (armure) — présence confirmée | Protection contre les dégâts physiques des attaques qui atteignent le personnage, notamment coups et projectiles physiques. | Principalement protections du corps, armures, renforcements et modificateurs explicites. Aucune conversion automatique de Résilience en armure n'est ajoutée. | Se confondre avec l'Esquive ou protéger automatiquement contre les intrusions, la chaleur et tous les autres types de dégâts. |
| Points de vie maximaux | Nombre maximal de PV, représentant la capacité du corps à rester opérationnel malgré les dégâts. | Principalement corps et renforcements ; contribution limitée de Résilience proposée, à valider. | Remplacer le blindage, réparer automatiquement ou constituer une seconde réserve de vie du noyau. |
| Stabilité système | Résistance à une interruption ou à une perturbation du fonctionnement. | Résilience, protections et états actifs pertinents. | Réduire indistinctement tous les dégâts ou rendre toute attaque logicielle inopérante. |
| Détection | Aptitude à repérer un indice discret ou une entité dissimulée dans les conditions accessibles aux capteurs. | Perception, capteurs compatibles, signature de la cible et environnement. | Voir derrière un obstacle opaque, convertir un souvenir en position actuelle ou profiter d'un écran plus large. |
| Analyse | Qualité de l'interprétation d'une observation disponible : état, équipement, faiblesse identifiable ou fonctionnement local. | Perception, Traitement, capteurs et données disponibles. | Inventer une information absente, consulter tout l'inventaire caché ou donner automatiquement un bonus de dégâts. |
| Efficacité d'intrusion | Aptitude à franchir une défense logicielle dans une tentative d'accès autorisée par les conditions du jeu. | Traitement, programme et interface ; technique applicable explicitement décrite. | Donner une liaison inexistante, remplacer des droits manquants sans tentative ou pirater toute cible organique. |
| Défense numérique | Résistance à une tentative d'accès ou à un effet logiciel hostile compatible. | Traitement, pare-feu et protections ; contribution secondaire de Résilience à examiner. | Servir également de blindage contre toutes les décharges physiques. |
| Contrôle distant — candidat à justifier | Capacité à exploiter des consignes complexes pour les unités électroniques contrôlées. | Traitement, logiciel de commande et contrôleur. | Augmenter à lui seul le nombre de drones, les actions, la portée et la bande passante. |

« Efficacité d'intrusion » distingue la valeur calculée de la compétence Intrusion. Le libellé et son infobulle ont été validés individuellement lors de l'étape 7, en section 19.1 (texte 15).

La liste comprend désormais onze secondaires, plus le candidat Contrôle distant qui n'est pas imposé. Les techniques de Contrôle de drones, les capacités du contrôleur et la bande passante pourraient suffire. Si aucun effet distinct et utile ne justifie ce score, la proposition est de s'en passer. Un drone basique ne doit pas échouer aléatoirement à un ordre ordinaire seulement pour donner une utilité à cette valeur.

### 4.1. Distinctions importantes

- Impact physique et dégâts : un coup peut atteindre sa cible sans parvenir à la déplacer. La masse et l'ancrage interviennent dans la poussée ; la protection intervient dans les dégâts. Les deux résultats ne doivent pas être confondus.
- Esquive et Esquive préparée : la secondaire ne donne pas le déplacement de la technique. Celle-ci garde ses conditions de case libre et son coût de réaction.
- Esquive, Blindage et PV : l'Esquive intervient pour éviter certaines attaques ; le Blindage protège contre les dégâts physiques couverts lorsqu'une attaque touche ; les PV actuels subissent les dégâts restants. Une protection matérielle ne doit pas être comptée une deuxième fois sous un autre nom.
- PV et Stabilité système : les premiers décrivent la réserve de vie ; la seconde répond aux perturbations. Un personnage solide peut subir une interruption, et un personnage stable peut subir des dégâts importants.
- Détection et portée des capteurs : la portée fixe les cases potentiellement accessibles, puis les obstacles limitent la perception. L'efficacité face à une signature discrète est une autre question. L'éventuel effet numérique de Perception sur la portée reste à décider séparément.
- Détection et Analyse : repérer une présence n'implique pas connaître toutes ses propriétés. Inversement, analyser une trace ne révèle pas la position actuelle de son auteur.
- Stabilité système et Défense numérique : une attaque mixte peut avoir un volet logiciel et un volet perturbation, mais ne doit pas imposer deux jets défensifs redondants par défaut. Chaque effet doit préciser la défense pertinente.

### 4.2. Blindage : règles encore à définir

L'existence de cette secondaire et l'absorption totale possible sont confirmées. Une première proposition de réduction fixe figure en section 10 ; les modalités suivantes restent à valider ou à compléter :

- réduction fixe, proportionnelle ou autre formule, bornes et arrondis ;
- types de dégâts concernés, et traitement des dégâts mixtes ;
- ordre d'application avec la pénétration, la fragilisation et les résistances spécialisées ;
- interaction avec Brise-armure et Tir de rupture, sans double application de la même réduction de protection ;
- éventuelle répartition par zone protégée, uniquement si le système de parties du corps est retenu et implémenté.

Une valeur de Blindage n'ajoute pas implicitement un mécanisme d'usure des armures. Les résistances spécialisées, notamment thermiques ou électriques, restent à distinguer de l'armure ordinaire ; leur catalogue exact n'est pas fixé ici.

## 5. Valeurs matérielles et ressources

Proposition : conserver ces valeurs distinctes des aptitudes générales, même si une technique ou un état peut les modifier explicitement.

| Valeur | Source principale et distinction |
|---|---|
| Sources du Blindage | Corps, armures, renforcements et états pertinents ; ces contributions alimentent la secondaire Blindage de la section 4, sans seconde couche d'armure indépendante. |
| Résistances spécialisées aux dégâts | Protections, matériau et états ; leur couverture et leur combinaison avec le Blindage restent à définir par type de dégâts. |
| Énergie maximale / énergie disponible | Capacité de stockage / réserve actuelle ; Traitement ne crée pas de batterie. |
| Chaleur actuelle / seuils thermiques / dissipation | Charge thermique / limites / évacuation ; ce ne sont pas trois noms pour une même réserve. |
| Bande passante totale / occupée | Capacité de liaison / utilisation actuelle par les fonctions concernées. |
| Emplacements de modules et de programmes | Compatibilité et capacité d'installation ; à distinguer du nombre de techniques apprises. |
| Portée, munitions, dégâts de base et modes d'une arme | Profil de cette arme ; une technique doit préciser ce qu'elle modifie. |
| Portée et canaux des capteurs | Matériel et conditions de perception ; aucune dépendance à la résolution de la fenêtre. |
| Coût de déplacement et possibilités de locomotion | Corps, équipement, terrain, charge et états ; pas d'accélération universelle offerte par Coordination. |
| Nombre d'unités contrôlables | Contrôleurs, capacités disponibles et contraintes du système de drones ; pas un multiplicateur automatique de Traitement. |

Les formules de surcharge de transport, seuils thermiques, récupération des ressources et consommation de bande passante restent à définir. Cette liste n'impose pas que toutes ces jauges apparaissent dès le premier prototype.

## 6. Proposition de règles de calcul communes

### 6.1. Contributions traçables

Pour chaque action, identifier successivement :

1. Sa légalité : technique connue si nécessaire, matériel, cible compatible, perception et trajectoire.
2. Les valeurs utiles de l'acteur et du matériel, calculées pour cette action.
3. La défense ou difficulté réellement pertinente, si une incertitude doit être résolue.
4. L'effet, sa durée et ses interactions ; les règles de consommation en cas d'échec restent celles de la fiche.

Cette liste organise la description sans fixer l'ordre détaillé de tous les événements de combat. La section 10 propose un premier modèle chiffré pour les attaques physiques ordinaires.

Si Coordination contribue déjà à la Précision, une attaque ne reçoit pas en plus un deuxième bonus générique de Coordination. Une technique modifie un paramètre identifié ; elle ne multiplie pas implicitement dégâts, portée, durée et économie. Une action sans incertitude utile n'a pas besoin d'un jet supplémentaire.

### 6.2. Équipement et ressources

Les secondaires doivent être recalculables depuis leurs sources, sans empiler définitivement les bonus à chaque équipement/déséquipement. La fiche doit pouvoir distinguer la valeur propre au personnage, l'effet du matériel et les modificateurs temporaires.

Augmenter un maximum et restaurer une ressource sont deux opérations différentes. Aucun cycle d'équipement ne doit créer gratuitement des soins ou de l'énergie. La section 11.5 propose une règle pour les PV actuels lorsque leur maximum change ; elle reste à valider. Le traitement de l'énergie et des autres ressources reste à définir séparément.

Les états incompatibles, cumuls autorisés, plafonds et ordre d'application restent à compléter avant les fiches chiffrées finales. La section 12 propose des bornes et arrondis pour les résistances aux dégâts, ainsi qu'une première règle d'interruption des préparations.

## 7. Exemples de raccordement aux techniques existantes

Ces exemples expliquent les responsabilités sans ajouter de bonus ou de coût au catalogue.

| Action | Valeurs à relier | Frontière à conserver |
|---|---|---|
| Tir visé (TIR-01) | Précision de l'attaque, arme et préparation. | Le montant du bonus reste à fixer ; la préparation ne permet pas de tirer à travers un mur. |
| Repoussement | Impact physique, masse et ancrage de la cible, case d'arrivée. | Une forte Puissance ne crée pas une destination praticable. |
| Brise-armure (MEL-07) et Tir de rupture (TIR-09) | Blindage de la cible, état de fragilisation ou contournement partiel propre au tir. | Les techniques conservent leurs conditions et contreparties ; un ordre de résolution est proposé en section 10, leurs quantités et durées restent à définir. |
| Analyse de cible (REC-01) | Analyse, observation actuelle et données identifiables. | La technique n'invente pas une position hors perception et n'ajoute pas un bonus d'attaque automatique. |
| Surcharge (GEL-01) | Puissance d'émission du matériel, énergie, propagation et résistance adaptée. | Ce n'est pas automatiquement un duel entre Efficacité d'intrusion et Défense numérique : la nature physique de l'effet compte. |
| Ordre simple à un drone | Contrôleur et unité disponibles, ordre légal. | Aucun besoin d'acheter trois disciplines ni d'ajouter un jet de Contrôle distant à chaque ordre. |

## 8. Critères de vérification proposés

Ces scénarios décrivent des tests futurs ; ils n'ont pas été exécutés dans le jeu pour ce document.

1. La création accepte une répartition de somme 28 dont les valeurs sont entre 3 et 8, et refuse une somme ou une borne invalide sans modifier le personnage.
2. Une augmentation de primaire n'affecte que les valeurs et actions dont la règle la mentionne, sans double comptage.
3. Équiper puis retirer un objet rend les mêmes valeurs dérivées qu'avant, à état identique, sans gain de ressources.
4. Une Précision élevée ne rend pas valide une trajectoire bloquée ; une Détection élevée ne révèle pas une cible derrière un mur.
5. Une analyse ne révèle que les propriétés accessibles à cette observation et ne donne aucun bonus non prévu par la technique.
6. Une Esquive élevée ne transforme pas une explosion couvrant toutes les cases de repli en attaque automatiquement évitée.
7. Une attaque logicielle et une décharge physique consultent les défenses prévues par leur effet, pas une défense numérique universelle.
8. À état identique, le résultat est indépendant de l'affichage ; sauvegarde et chargement doivent conserver les sources nécessaires au même calcul.
9. À attaque physique identique, un Blindage plus élevé ne doit pas augmenter les dégâts reçus ni modifier implicitement l'Esquive. La protection n'est appliquée qu'une fois à partir des contributions matérielles.
10. Une modification de Blindage seule ne modifie pas la Défense numérique. Brise-armure et Tir de rupture ne retirent pas une protection logicielle par simple emploi du mot « armure ».

## 9. Décisions à prendre ensuite

| Repère | Question | Proposition ou prochaine étape |
|---|---|---|
| STAT-01 | Liste et rôle des secondaires. | Blindage ajouté à la demande de l'utilisateur ; examiner les autres définitions et les frontières entre Esquive, Blindage, PV, Stabilité et Défense numérique. |
| STAT-02 | Résilience et points de vie maximaux. | Section 11 : contribution limitée retenue comme base de travail ; coefficient d'essai de 5 autour de la référence 5, sur un socle matériel, à éprouver. |
| STAT-03 | Utilité d'un score de Contrôle distant. | Section 14.5 : proposition de ne pas ajouter ce score ; contrôleur, bande passante et techniques suffisent au périmètre actuel. |
| STAT-04 | Formules, unités et bornes. | Sections 10 à 16 : barèmes proposés pour les onze secondaires, ressources, temps et états ; paramètres à tester. |
| STAT-05 | Progression. | Section 17 : profil d'essai à 20 niveaux, budget initial de 2 points inclus dans les classes, 1 point par niveau gagné et cinq augmentations primaires. Ce ne sont pas des décisions finales. |
| STAT-06 | Fiches finales. | Annexe 16 du catalogue : 104 profils techniques couvrant les 94 entrées et 10 variantes, après fusion de MAN-07 dans MAN-01. Chiffres à tester ; textes joueur rédigés, quinze validations individuelles en section 19.1 puis complément sous délégation en section 19.3. |
| STAT-07 | Formule de Blindage. | Présence et absorption totale des petites attaques confirmées, sans minimum de 1 dégât ; réduction fixe et pénétration en points restent une proposition à valider. |
| STAT-08 | Chances de toucher. | Examiner la base de 70 % entre profils ordinaires, les contributions des primaires et les bornes d'essai de 5 % à 95 % ; rien n'est encore validé. |
| STAT-09 | Difficulté et possibilité de gagner. | Objectif confirmé : ni facile ni impossible à gagner. Éprouver les réponses accessibles, les obstacles obligatoires et les limites des builds défensifs ; aucun taux de victoire cible n'est fixé. |
| STAT-10 | Puissance, Impact et dégâts de mêlée. | Section 11 : rôle de Puissance limité par le matériel retenu comme base de travail ; référence 10 et coefficient 2 restent un barème d'essai. Vérifier les seuils de Blindage et les attaques multiples. |
| STAT-11 | Changement de maximum et réparation. | Augmenter le maximum sans réparation automatique retenu comme base de travail. La section 11.5 détaille le plafonnement proposé lors d'une baisse ; ergonomie et économie restent à éprouver. |
| STAT-12 | Types de dégâts et résistances. | Distinction des protections et résistances spécialisées en pourcentages retenue après accord ; plafond de 75 % comme base d'essai, autres bornes et modalités à éprouver ou préciser. |
| STAT-13 | Stabilité et interruptions. | Stabilité distincte des dégâts, interruption sans perte automatique du prochain tour et courte protection contre les répétitions retenues ; coefficients, durée exacte et chronologie restent à éprouver. |
| STAT-14 | Écart avec le prototype. | Section 12.8 : minimum de dégâts, unité de pénétration, familles de dégâts et attaques mixtes à réconcilier avant implémentation ; aucun code modifié ici. |

La discussion des capacités extérieures reste reportée. Le chapitre actuel peut progresser et être testé sans définir leur acquisition ou leur fonctionnement.

## 10. Première proposition chiffrée : combat physique

Statut : proposition chiffrée à valider, avec absorption totale et absence de minimum automatique de 1 dégât confirmées par l'utilisateur. Ce chapitre n'est pas la description du combat actuellement implémenté et n'impose pas encore ses nombres au contenu du jeu. L'accord sur l'absorption totale ne valide pas implicitement les coefficients de Précision/Esquive, leurs bornes ni toutes les modalités de pénétration.

### 10.1. Périmètre et légalité

Le modèle couvre les attaques physiques ordinaires de mêlée et les projectiles dirigés contre une cible valide, puis l'atténuation de leur composante physique. Les paramètres ont les mêmes rôles pour le joueur et pour les autres acteurs compatibles.

Avant tout jet, vérifier les conditions de l'action : matériel, ressources, portée, perception et trajectoire requises. Un mur bloquant ou une cible hors portée ne donnent jamais une chance minimale de toucher de 5 %. Une impossibilité déjà connue reste refusée sans dépense ; une attaque valide qui manque consomme son temps et les ressources prévues par son profil.

Une attaque de zone ne donne pas automatiquement un jet d'Esquive à chaque occupant. La présence dans les cases effectivement affectées, les obstacles et les éventuels déplacements de réaction déterminent l'exposition. Un éventuel jet de placement d'un projectile explosif est une question distincte. Une action certaine sur un objet inerte n'a pas besoin d'un jet uniquement pour employer cette formule.

### 10.2. Calcul proposé de Précision et d'Esquive

Toutes les valeurs de ce premier barème sont entières. Les contributions neutres de matériel et d'état valent zéro ; leurs valeurs réelles viendront des profils d'équipement et des effets.

```text
Précision_mêlée = 80 + 4 × (Coordination − 5) + modificateurs_de_précision
Précision_tir   = 80 + 4 × (Coordination − 5) + 2 × (Perception − 5)
                    + modificateurs_de_précision

Esquive = max(0, 10 + 4 × (Coordination − 5) + modificateurs_d'esquive)
```

Les modificateurs de précision regroupent les contributions explicites de l'arme, de ses réglages et des états de l'attaquant. Ceux d'esquive regroupent locomotion, encombrement et états défensifs applicables. Ils sont distincts du Blindage : une armure lourde ne réduit pas l'Esquive par simple présence d'un score élevé de Blindage ; une pénalité de charge ou de mobilité doit être explicitement définie.

Le coefficient de Perception est ici réservé au tir ordinaire ; il n'ajoute pas automatiquement un second bonus à toutes les attaques de mêlée. Coordination ne fournit ni dégâts supplémentaires ni accélération générale par ces formules. Aucun bonus de rang gratuit n'est ajouté.

Proposition de résolution :

```text
Chance_de_toucher = borner(Précision − Esquive_adverse + contexte, 5, 95)
Touche si un entier uniforme de 1 à 100 est inférieur ou égal à cette chance.
```

Le contexte contient notamment les modificateurs de préparation, de couvert partiel et de distance pertinents pour l'action. Chaque contribution apparaît une seule fois, soit dans une secondaire, soit dans le contexte. Un point de score modifie la probabilité d'un point de pourcentage avant application des bornes. Le barème de contexte reste à définir par situation.

Un seul jet résout la touche : pas de second jet générique d'Esquive après une réussite. Les réactions actives comme Esquive préparée conservent leur procédure, leur déplacement réel et leur limite commune ; elles ne sont pas un second tirage passif. La chronologie détaillée des réactions sera traitée avec leurs fiches.

Le tirage appartient à la simulation, à partir de son état aléatoire reproductible. Consulter, prévisualiser ou annuler une commande ne consomme aucun tirage.

### 10.3. Exemples de toucher

Les cas suivants sont des tirs valides, sans réaction spéciale. Sauf indication contraire : Coordination et Perception à 5, matériel neutre, aucun modificateur de contexte. La pénalité de couvert −20 n'est qu'un exemple de contexte, pas un barème de terrain adopté.

| Situation | Précision | Esquive adverse | Contexte | Chance proposée |
|---|---:|---:|---:|---:|
| Deux profils ordinaires | 80 | 10 | 0 | 70 % |
| Attaquant avec Coordination 8 | 92 | 10 | 0 | 82 % |
| Attaquant avec Perception 8 | 86 | 10 | 0 | 76 % |
| Défenseur avec Coordination 8 | 80 | 22 | 0 | 58 % |
| Attaquant avec Coordination 8, cible sous un couvert donnant −20 | 92 | 10 | −20 | 62 % |

Ces chiffres visent à rendre une bonne position et un investissement en Coordination perceptibles. Il faudra vérifier si 70 % produit trop de tours sans effet et si les bornes de 5 % et 95 % créent des échecs frustrants ou des réussites trop indulgentes dans les situations extrêmes.

### 10.4. Blindage et pénétration : réduction fixe proposée

Proposition de premier modèle global, sans répartition anatomique : le Blindage est la somme non négative des contributions explicites du corps, des protections équipées et des renforcements. Une même source n'est comptée qu'une fois. Les états de fragilisation sont traités séparément ci-dessous ; on ne les soustrait pas déjà dans cette somme.

La pénétration est exprimée en points de Blindage ignorés pour cet impact. Elle ne retire pas définitivement l'armure et n'ajoute pas de dégâts lorsque toute la protection est déjà contournée.

```text
B = Blindage avant fragilisation, entier ≥ 0
F = fragilisation déjà active, entier ≥ 0
P = pénétration applicable à cet impact, entier ≥ 0
D = dégâts physiques bruts de cet impact, entier ≥ 0

Blindage_fragilisé = max(0, B − F)
Blindage_effectif  = max(0, Blindage_fragilisé − P)
Dégâts_physiques   = max(0, D − Blindage_effectif)
```

Dans ce modèle réduit, les dégâts physiques restants sont retirés des PV actuels, sans passer sous zéro. Il n'y a pas de second jet d'armure ni de minimum automatique de 1 dégât. Une touche entièrement absorbée ne soigne pas et n'augmente pas les PV maximaux.

Les dégâts bruts viennent du profil d'attaque, avec uniquement les contributions autorisées. La section 11 propose le raccordement d'Impact physique à la mêlée ordinaire. Une distribution de dégâts ou un système universel de coups critiques restent à discuter ; Puissance n'augmente pas par défaut les dégâts d'une balle.

Couverture proposée : coups de mêlée, projectiles physiques et composantes physiques explicitement identifiées d'une explosion. Les dégâts thermiques, électriques ou logiciels ne sont pas atténués par le Blindage ordinaire par défaut. La section 12 propose le traitement des résistances et des attaques mixtes : ne pas appliquer cette soustraction au total indistinct de plusieurs types de dégâts.

### 10.5. Exemples de dégâts après une touche

Toutes les attaques du tableau ont déjà touché. Aucun effet spécial de réaction ou résistance supplémentaire n'est actif. Les valeurs de pénétration et de fragilisation illustrent la formule ; elles ne sont pas les valeurs décidées de Tir de rupture ou de Brise-armure.

| Dégâts bruts D | Blindage B | Fragilisation F | Pénétration P | Blindage effectif | Dégâts restants |
|---:|---:|---:|---:|---:|---:|
| 12 | 5 | 0 | 0 | 5 | 7 |
| 12 | 5 | 0 | 3 | 2 | 10 |
| 6 | 10 | 0 | 0 | 10 | 0 |
| 12 | 10 | 4 | 3 | 3 | 9 |
| 12 | 5 | 0 | 20 | 0 | 12 |

Proposition pour les rafales : chaque projectile est un impact distinct, avec son jet de touche et sa propre atténuation. Plusieurs composantes physiques d'un même impact sont en revanche regroupées avant la soustraction, pour ne pas payer plusieurs fois le Blindage à cause du découpage des données.

Exemple, si tous les coups touchent : trois projectiles de 6 dégâts contre un Blindage de 4 infligent 3 × 2 = 6 dégâts ; un impact unique de 18 dégâts inflige 14 dégâts. Sans Blindage, les deux totalisent 18. Cette différence favorise les petits impacts contre les cibles peu protégées et les gros impacts ou la pénétration contre les cibles blindées ; elle ne démontre pas leur équilibre en temps, précision ou consommation.

### 10.6. Techniques, retours au joueur et limites

- Brise-armure : proposer que la nouvelle fragilisation s'applique après la résolution de son propre impact, pour les attaques suivantes, sans réduction rétroactive. Les conditions de réussite, le montant et la durée restent à préciser. Pour une même famille de fragilisation, retenir la plus forte plutôt qu'additionner indéfiniment les applications ; le renouvellement de durée reste à définir.
- Tir de rupture : son contournement concerne le tir exécuté, pas une perte permanente du Blindage adverse. Sa conversion en points de pénétration reste à fixer et doit conserver le contournement partiel prévu par sa fiche. Le cas générique de pénétration 20 du tableau ne lui accorde pas une traversée totale de toute armure.
- Parade : ne pas la transformer implicitement en bonus de Blindage permanent. Sa réduction et son ordre par rapport au Blindage restent à définir avec son coût de réaction.
- Les effets secondaires ne sont pas automatiquement bloqués ou déclenchés par zéro dégât. Chaque fiche devra distinguer une condition « toucher », « infliger des dégâts » ou une autre condition explicite.
- Un profil très blindé peut absorber une arme trop faible : ce principe est confirmé. Pour respecter l'objectif de difficulté surmontable, les essais devront vérifier les réponses viables, sans imposer l'achat d'une technique particulière ni rendre une voie de progression impraticable faute d'un objet rare ; voir section 10.8.

Le joueur doit pouvoir distinguer attaque manquée, impact sans dégât observé et perte de PV constatée. Un aperçu exact de probabilité ou de dégâts n'est permis que si les informations nécessaires sont connues. Si le Blindage ou l'Esquive adverse ne sont pas connus, utiliser une indication incertaine ou une estimation fondée sur les seules observations disponibles, sans révéler une statistique cachée ni attribuer automatiquement la cause de zéro dégât à l'armure.

### 10.7. Vérifications et suite de la proposition

Vérifications arithmétiques effectuées lors de la rédaction : les dix lignes d'exemples des sections 10.3 et 10.5 ont été relues depuis ce document et recalculées. Des séries de valeurs ont aussi vérifié les bornes de probabilité, la monotonie de Précision et d'Esquive, celle du Blindage, de la fragilisation et de la pénétration, l'absence de dégâts négatifs ou supérieurs aux dégâts bruts du seul fait de la pénétration, ainsi que l'exemple de rafale face à l'impact unique. Ce sont des calculs isolés, pas des tests du moteur ni des parties jouées.

Les essais jouables restent nécessaires : fréquence des ratés, efficacité des builds défensifs, intérêt des armes légères et lourdes, coût des préparations, rencontres avec armure inconnue et accès à des solutions alternatives. La compatibilité avec le moteur n'est pas validée par un calcul de tableau.

La section 11 poursuit avec Impact, dégâts bruts de mêlée et PV ; la section 12 propose les résistances et interruptions. Restent les réactions actives détaillées, les autres états et secondaires et le budget de progression. Les capacités extérieures restent hors de cette étape.

### 10.8. Difficulté surmontable et réponses accessibles

Objectif confirmé par l'utilisateur : le jeu ne doit être ni facile ni impossible à gagner. Une arme inefficace contre un ennemi ne signifie pas que cet ennemi ou la partie doivent être impossibles à surmonter. Cela ne garantit pas la victoire de chaque personnage ni la possibilité de vaincre chaque ennemi immédiatement avec son équipement actuel.

Pistes d'équilibrage à éprouver, sans nouvelle mécanique obligatoire :

- Permettre des réponses différentes selon la situation : préparation, arme adaptée, pénétration, fragilisation, autre type de dégâts compatible, exploitation du terrain, contournement ou retraite lorsque ces possibilités existent réellement.
- Pour un obstacle obligatoire, vérifier qu'une réponse est effectivement accessible avant le blocage dans la partie générée, avec des coûts et ressources réalistes. Une solution théorique réservée à un objet rare absent ou à une seule technique non apprise ne suffit pas.
- Rendre l'inefficacité observable, sans révéler gratuitement les statistiques cachées. La difficulté doit laisser au joueur des informations exploitables pour apprendre, évaluer le risque et changer d'approche.
- Préserver l'intérêt d'un build blindé contre les attaques faibles, tout en éprouvant les menaces qui ne se résument pas à cette défense. Ne pas transformer la validation de zéro dégât en immunité générale à tous les types d'attaque.
- Distinguer un blocage produit par une mauvaise décision ou une dépense de ressources du joueur d'une impasse créée dès la génération. La possibilité de perdre fait partie du jeu ; elle ne prouve pas à elle seule que l'équilibrage est injuste.

Les essais futurs devront comparer plusieurs builds et plusieurs parties générées, observer les blocages d'objectifs, les réponses réellement disponibles et la compréhension des échecs. Aucun taux de victoire, ajustement automatique de difficulté ou garantie technique de solvabilité de toutes les seeds n'est ajouté par cette note ; ces questions demanderaient leurs propres décisions et validations.

## 11. Proposition chiffrée : Impact et PV

Statut : principes retenus comme base de travail selon le retour ci-dessous, modalités complémentaires proposées et valeurs d'essai ; pas une règle déjà programmée. Cette étape complète le barème physique avec le rôle de Puissance dans un coup de mêlée et celui de Résilience dans les PV maximaux. Elle ne change pas le corps principal, les cinq primaires, leur plafond de 10 ou les techniques existantes.

Suivi du retour utilisateur : les principes présentés dans le résumé — Puissance contribuant à la mêlée dans les limites matérielles, contribution limitée de Résilience aux PV et absence de réparation automatique lors d'une augmentation du maximum — sont retenus comme base de travail. Les nombres demeurent un barème d'essai ; cet accord ne vaut pas validation implicite de chaque détail annexe ni preuve d'équilibrage.

### 11.1. Impact physique et plafond matériel

L'Impact est un score d'efficacité physique pour un geste donné, pas un nombre de dégâts ajouté tel quel à toutes les armes. La référence proposée est 10 : Puissance 5, matériel neutre et capacité matérielle suffisante.

```text
Impact_disponible = max(0, 2 × Puissance + modificateurs_d'impact)
Impact_utilisé   = min(Impact_disponible, plafond_matériel_du_geste)
```

Les modificateurs sont les contributions entières explicites des actuateurs, de leurs réglages et des états pertinents. Le plafond matériel est un entier non négatif, exprimé dans la même unité que l'Impact ; il représente la capacité des actuateurs et du matériel qui transmettent l'effort pour ce geste. Sa valeur appartient au profil matériel, pas à une augmentation automatique de Puissance. Améliorer le matériel peut modifier sa contribution ou son plafond selon son effet déclaré.

Une Puissance élevée exploite mieux les moyens disponibles sans dépasser leurs limites. L'interface doit signaler un Impact plafonné et en indiquer la source connue. L'absence de manipulateur compatible ou d'un autre moyen requis rend le geste impossible avant le calcul : un plafond ne donne pas le droit d'utiliser un équipement incompatible.

Le terme « plafond matériel du geste » ne fixe pas ici un nouveau nombre universel pour tous les personnages. Il ne faut pas non plus modifier ou remplacer le corps principal pour employer ce modèle : équipements, actuateurs améliorés et état du même corps fournissent ses valeurs.

### 11.2. Raccordement aux dégâts de mêlée ordinaires

Pour une attaque qui utilise explicitement l'Impact, le profil fournit des dégâts physiques de référence, définis pour un Impact de 10. Pour cette première proposition, ces dégâts sont une valeur entière fixe avant les défenses, sans second tirage de dégâts ni critique générique. Une éventuelle variabilité future demanderait une règle distincte.

```text
Bonus_d'impact = Impact_utilisé − 10
Dégâts_physiques_bruts = max(0, dégâts_de_référence + Bonus_d'impact)
```

Ces dégâts entrent ensuite dans le calcul de Blindage de la section 10.4 si l'attaque a touché. Le bonus peut être négatif lorsque l'Impact est inférieur à 10. Puissance n'est pas ajoutée une seconde fois après le calcul de l'Impact et le bonus ne devient pas de la pénétration supplémentaire.

Une attaque de tir, une explosion ou une composante thermique/électrique n'utilise pas ce bonus par défaut. Une arme de mêlée à effet mixte applique cette contribution uniquement à sa composante physique compatible, sans créer de dégâts physiques si son profil n'en prévoit pas. L'attaque ordinaire sans arme doit disposer de son propre profil compatible ; elle n'emprunte pas arbitrairement les dégâts d'une arme absente.

Les techniques modifient le profil d'attaque selon leur propre fiche. Cette formule ordinaire ne définit pas encore les bonus de Frappe puissante, les réductions de Balayage ou les effets d'Écrasement. Elle ne donne pas automatiquement le plein bonus à chaque sous-effet d'une technique multiple. Un découpage technique d'un même impact en plusieurs composantes n'ajoute pas plusieurs fois le bonus.

Pour Repoussement et les autres gestes de déplacement forcé, le score d'Impact pourra contribuer à une opposition physique. La formule tenant compte de la masse et de l'ancrage reste à définir : dégâts infligés, Blindage et capacité à déplacer la cible ne sont pas interchangeables. Une touche à zéro dégât ne décide pas à elle seule du succès d'une poussée.

### 11.3. Exemples de mêlée

Hypothèses illustratives : une attaque à 12 dégâts physiques de référence, modificateur d'Impact nul, aucune pénétration ou fragilisation, aucune technique ou réaction spéciale. Chaque attaque du tableau a déjà touché. Les plafonds 20 et 14 sont des exemples de profils, pas des équipements de départ imposés.

| Puissance | Plafond matériel | Impact disponible | Impact utilisé | Bonus d'impact | Dégâts bruts | Blindage adverse | Dégâts restants |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 3 | 20 | 6 | 6 | −4 | 8 | 5 | 3 |
| 5 | 20 | 10 | 10 | 0 | 12 | 5 | 7 |
| 8 | 20 | 16 | 16 | 6 | 18 | 5 | 13 |
| 10 | 20 | 20 | 20 | 10 | 22 | 5 | 17 |
| 8 | 14 | 16 | 14 | 4 | 16 | 5 | 11 |

La différence entre 7 et 13 dégâts après Blindage montre qu'un bonus brut de 6 peut avoir un effet relatif important. Le coefficient de Puissance doit donc être testé avec les protections réellement rencontrées, les temps d'action et les autres possibilités de build ; le tableau ne prouve pas son équilibre.

### 11.4. Points de vie maximaux proposés

Les PV maximaux décrivent la réserve opérationnelle du même corps principal. Résilience apporte une marge de fonctionnement limitée ; elle n'épaissit pas le Blindage et ne remplace pas les renforcements matériels.

```text
PV_max = max(1,
    PV_de_base_du_corps
    + 5 × (Résilience − 5)
    + bonus_matériels_de_PV
    + modificateurs_d'état_du_maximum)
```

Les contributions sont entières, explicites et comptées une seule fois. Le minimum de 1 concerne seulement la capacité maximale : il ne protège pas les PV actuels contre zéro. Un personnage à zéro reste dans l'état de défaite/destruction prévu ; augmenter ensuite une capacité maximale ne le ressuscite pas.

Le profil à 100 PV de base ci-dessous sert uniquement aux calculs. Il n'est ni une valeur définitive du joueur ni un minimum imposé à tous les PNJ. Un acteur utilisant cette formule prend sa propre base matérielle ; la même gestion de réserve et de dégâts s'applique sans donner artificiellement la même endurance à toutes les créatures.

| Base du corps | Résilience | Bonus matériel | Modificateur d'état | Points de vie maximaux |
|---:|---:|---:|---:|---:|
| 100 | 3 | 0 | 0 | 90 |
| 100 | 5 | 0 | 0 | 100 |
| 100 | 8 | 0 | 0 | 115 |
| 100 | 10 | 0 | 0 | 125 |
| 100 | 5 | 20 | 0 | 120 |

Aucun gain automatique supplémentaire de PV par niveau n'est introduit par cette formule. La progression peut augmenter Résilience ou accorder une amélioration explicitement définie ; sa cadence reste dans STAT-05. La section 12 propose sa contribution à Stabilité système, distincte d'une réduction générale des dégâts.

### 11.5. Changer le maximum sans réparation gratuite

Proposition commune aux changements d'équipement, de Résilience et d'état modifiant ce maximum : conserver la réserve actuelle lorsque le maximum augmente ; la plafonner au nouveau maximum lorsqu'il diminue.

```text
PV_actuels_après = min(PV_actuels_avant, nouveau_maximum)
```

Cette opération suppose une réserve actuelle valide, non négative. Elle ne remet pas à zéro les dégâts subis et n'ajoute aucun soin automatique. Une baisse de capacité ne constitue pas en elle-même une attaque de combat donnant une riposte ou une récompense d'expérience.

| PV avant | Nouveau maximum | PV après | Conséquence |
|---|---:|---|---|
| 70 / 100 | 120 | 70 / 120 | Capacité accrue, aucune réparation. |
| 110 / 120 | 100 | 100 / 100 | Excédent non conservé au-delà du nouveau maximum. |
| 100 / 100 | 120 | 100 / 120 | Rééquiper un renforcement ne recrée pas l'excédent perdu. |
| 0 / 100 | 120 | 0 / 120 | Aucun retour à la vie. |

Les deuxième et troisième lignes peuvent former une séquence : 110/120 devient 100/100, puis 100/120, jamais 110/120 sans réparation. Il faut annoncer une perte d'excédent avant une modification volontaire dont les conséquences sont connues. Le personnage conserve ses apprentissages pendant ces changements.

Cette règle évite les soins par changement d'équipement, mais rend une augmentation de capacité moins immédiatement avantageuse pour un personnage blessé. C'est un compromis à valider : l'économie et l'accès aux réparations ordinaires devront être testés avant de le figer. La règle n'est pas étendue implicitement à l'énergie, à la chaleur ou à d'autres ressources.

### 11.6. Réparation ordinaire et limites

Une réparation est une action ou un effet explicite, pas une conséquence implicite de Résilience. Pour une réparation ordinaire d'une cible encore opérationnelle et compatible :

```text
Quantité_restaurée = min(quantité_nominale_de_réparation,
                         PV_max − PV_actuels)
```

La quantité nominale est non négative et vient du moyen de réparation utilisé, avec seulement les modificateurs autorisés. Elle ne reçoit pas automatiquement un multiplicateur de Résilience ou un bonus d'Ingénierie non décrit. Temps, ressources consommées, compatibilité et éventuelles interruptions restent ceux de l'action.

Exemple : une réparation nominale de 20 fait passer 70/100 à 90/100, ou 95/100 à 100/100 en restaurant seulement 5 points. Cette valeur de 20 n'est pas un consommable définitif. Une réparation ne dépassant pas les besoins n'implique pas un remboursement automatique de son coût. Une commande exclusivement réparatrice sans effet possible et connue comme telle reste refusée selon la règle des commandes invalides ; un objet qui a aussi un autre effet utile suit son propre profil.

La réparation ordinaire doit rester viable sans apprendre Ingénierie. Réparation ciblée conserve son rôle sur une fonction ou un composant ; elle n'est pas transformée en condition d'accès aux soins de base. Ni cette formule ni un changement de maximum ne constituent une résurrection. Le présent barème ne définit pas de régénération passive : ses éventuelles sources et conditions restent à discuter.

### 11.7. Vérifications et décisions suivantes

Vérifications arithmétiques effectuées lors de la rédaction : les 14 lignes des tableaux de cette section et les 10 lignes du chapitre 10 ont été extraites du document et recalculées. Les deux exemples de réparation ont aussi été vérifiés. Des séries de valeurs ont contrôlé l'Impact non négatif et borné par le matériel, la croissance des valeurs avec leur primaire, le minimum du maximum de PV, l'absence de gain de réserve par un cycle d'équipement et le maintien à zéro d'une réserve déjà nulle. Les dégâts ordinaires utilisent une seule fois le bonus issu de Puissance, via l'Impact. Ces vérifications portent sur les formules proposées, pas sur une implémentation du jeu.

Les essais jouables devront vérifier les seuils de Blindage, les limites matérielles lisibles, la différence entre armes légères et lourdes, la durée des combats et l'économie de réparation à plusieurs niveaux de Résilience. Les calculs seuls ne valident pas une difficulté ni l'état du moteur.

La section 12 poursuit avec les types de dégâts, leurs résistances, Stabilité système et les interruptions. Les poussées, la charge utile, les autres secondaires et le rythme de progression restent à compléter ; les capacités extérieures restent reportées.

## 12. Résistances et Stabilité système : principes retenus et barèmes d'essai

Statut : principes retenus après accord utilisateur. Sont retenus la distinction entre Blindage, résistances spécialisées, Défense numérique et Stabilité, les résistances en pourcentages avec vulnérabilités possibles, et l'interruption d'une préparation sans perte automatique du prochain tour, accompagnée d'une courte protection contre les interruptions répétées.

Le plafond de résistance de 75 % reste une base d'essai. Les autres bornes, coefficients, arrondis, regroupements de dégâts, durée précise et chronologie de la protection restent des modalités proposées à éprouver ou à préciser ; l'accord ne constitue pas une validation technique de chaque détail. Aucun de ces systèmes n'est déclaré implémenté par ce document. Les capacités extérieures à débloquer en jeu restent hors périmètre.

### 12.1. Séparer dégâts, intrusion et perturbation

| Phénomène | Conséquence | Défense pertinente dans la proposition |
|---|---|---|
| Dégât physique | Retire des PV après une touche ou une exposition valide. | Blindage, après fragilisation et pénétration. |
| Dégât thermique | Endommage matériellement la cible par chaleur. | Résistance thermique. |
| Dégât électrique | Endommage une cible compatible par décharge ou émission électrique. | Résistance électrique, pas le pare-feu. |
| Dégât chimique | Endommage une matière ou un organisme compatible, par exemple par corrosion. | Résistance chimique ; compatibilité du produit avec la cible vérifiée séparément. |
| Intrusion ou implantation logicielle | Obtient un accès ou installe un effet hostile. | Défense numérique ; sa formule propre reste à définir. |
| Perturbation d'une préparation | Risque d'interrompre une action en cours déclarée interruptible. | Stabilité système, sans réduction automatique des dégâts. |

Un même effet peut produire plusieurs phénomènes distincts : une impulsion peut endommager et perturber. Cela n'autorise pas à multiplier les jets défensifs pour un seul résultat. Une attaque logicielle ne reçoit pas automatiquement un deuxième jet de Stabilité après son jet contre la Défense numérique.

La résistance thermique réduit ici les dégâts thermiques, pas automatiquement la montée de température ni la vitesse de refroidissement. Ces grandeurs nécessitent leurs propres règles. De même, une résistance chimique ne rend pas tout gaz toxique pour toute machine : la compatibilité vient d'abord.

### 12.2. Périmètre des catégories

Proposition initiale de présentation : Physique, Thermique, Électrique et Chimique. Le groupe Physique rassemble les chocs, projectiles et composantes physiques d'explosion ; leurs différences peuvent venir du profil d'arme et de sa pénétration. Il n'ajoute pas une résistance physique en pourcentage par-dessus le même Blindage.

Les identifiants actuels du prototype `Kinetic`, `Piercing` et `Explosive` peuvent être rapprochés de ce groupe pour la conception. Le mot `Piercing` ne donne pas une pénétration automatique et `Explosive` ne transforme pas la chaleur d'une explosion en dégâts physiques. Ce rapprochement n'est pas une migration des données ni une suppression de ces identifiants.

Le code connaît aussi `Radiation` et `Corruption`. Leur existence technique ne décide pas encore leur fonctionnement de jeu : ils restent à définir ultérieurement, sans les assimiler silencieusement à une autre catégorie ni les imposer aux fiches actuelles. En particulier, Infection n'inflige pas automatiquement des dégâts `Corruption` à cause de son nom.

### 12.3. Résistances spécialisées en pourcentages

Pour chaque résistance thermique, électrique ou chimique, sommer les contributions explicites du corps, du matériel et des états, puis appliquer les bornes d'essai. Il n'y a pas de bonus automatique de Résilience à ces pourcentages ; sa contribution est déjà définie pour les PV et proposée pour la Stabilité.

```text
R_type = borner(somme_des_contributions_du_type, −50, 75)
Dégâts_du_type = arrondi_inférieur(D_bruts_du_type × (100 − R_type) / 100)
```

Les contributions sont des points de pourcentage, pas des multiplicateurs successifs : +20 et +30 donnent +50 avant plafonnement. Une même source ne doit pas apparaître deux fois. La formule suppose des dégâts bruts entiers non négatifs. Une résistance négative représente une vulnérabilité ; une valeur positive réduit les dégâts. L'arrondi n'est appliqué qu'après regroupement des composantes de même type dans un même impact.

| Dégâts bruts | Résistance | Dégâts restants |
|---:|---:|---:|
| 20 | 0 % | 20 |
| 20 | 25 % | 15 |
| 20 | 50 % | 10 |
| 20 | 75 % | 5 |
| 20 | −25 % | 25 |
| 20 | −50 % | 30 |
| 3 | 75 % | 0 |

La dernière ligne rend visible l'effet de l'arrondi : de très petits dégâts peuvent être entièrement absorbés sans immunité générale. Cette conséquence et les dégâts périodiques de faible intensité devront être testés. Aucun minimum de 1 dégât n'est réintroduit après réduction. Une immunité spécifique ou une incompatibilité d'effet ne doit pas être inventée à partir d'un empilement de bonus dépassant 75 % ; les éventuelles immunités explicites demandent une règle distincte.

La pénétration du Blindage de la section 10 reste exprimée en points d'armure. Elle ne diminue pas ces résistances en pourcentage. Aucun contournement générique de résistance spécialisée n'est ajouté ici ; une future propriété de ce genre devra nommer son unité et sa cible explicitement.

### 12.4. Attaques mixtes et ordre de calcul

Pour un impact valide :

1. Regrouper sa composante physique et ses composantes de chaque autre type, sans confondre des projectiles distincts d'une rafale.
2. Appliquer le Blindage effectif une seule fois au total physique de cet impact.
3. Appliquer séparément chaque résistance spécialisée au total du type correspondant et arrondir ce résultat à l'entier inférieur.
4. Additionner les dégâts restants, puis retirer le total des PV actuels, sans passer sous zéro.
5. Résoudre les effets secondaires uniquement selon leurs déclencheurs déclarés, sans déduire automatiquement leur présence de dégâts positifs ou nuls.

Les cas ci-dessous n'ont ni pénétration ni fragilisation, réaction spéciale ou effet logiciel.

| Physique brut | Électrique brut | Blindage | Résistance électrique | Physique restant | Électrique restant | Perte totale de PV |
|---:|---:|---:|---:|---:|---:|---:|
| 10 | 8 | 4 | 50 % | 6 | 4 | 10 |
| 4 | 8 | 10 | 50 % | 0 | 4 | 4 |
| 0 | 20 | 50 | 25 % | 0 | 15 | 15 |

Un même impact composé de 6 points `Kinetic` et 4 points `Piercing` reste 10 points physiques pour ce modèle : face à un Blindage de 4 sans pénétration, il reste 6 dégâts, pas 2. La distinction par matériaux, localisation ou résistances physiques spécialisées n'est pas implémentée implicitement par le découpage des données.

La règle des projectiles distincts de la section 10.5 reste différente : chaque projectile réellement distinct rencontre la défense. Les dégâts de chaque étape d'un effet périodique sont calculés avec l'état de la cible à cette étape, sans répéter l'attaque initiale d'implantation sauf si la fiche l'exige explicitement.

### 12.5. Stabilité système : proposition de calcul

La Stabilité mesure la résistance à une perturbation de fonctionnement précise. Ce n'est ni du Blindage supplémentaire, ni une réserve de vie, ni un score universel contre tout effet négatif.

```text
Stabilité = max(0, 50 + 5 × (Résilience − 5) + modificateurs_de_stabilité)
Chance_de_résister = borner(50 + Stabilité − Intensité_de_perturbation, 5, 95)
```

Les modificateurs de stabilité viennent des protections et états explicitement concernés. L'intensité est un score entier non négatif déclaré par l'effet, pas la perte de PV qu'il vient de provoquer. Un entier uniforme de 1 à 100 inférieur ou égal à la chance signifie que la perturbation est évitée. Les dégâts éventuels déjà résolus ne sont ni annulés ni appliqués une seconde fois.

| Résilience | Modificateur de Stabilité | Stabilité | Intensité de perturbation | Chance de résister |
|---:|---:|---:|---:|---:|
| 3 | 0 | 40 | 50 | 40 % |
| 5 | 0 | 50 | 50 | 50 % |
| 8 | 0 | 65 | 50 | 65 % |
| 8 | 20 | 85 | 50 | 85 % |
| 5 | 0 | 50 | 70 | 30 % |

Ces scores sont des exemples d'essai. Une forte Résilience ne donne pas simultanément une réduction générale des dégâts et un second bonus de résistance sur ce même jet. Le matériel peut améliorer la Stabilité sans modifier la primaire.

Le jet est passif : il ne consomme ni ne rétablit le droit à une réaction de combat. Il n'est effectué que si un effet déclaré peut réellement perturber la cible. Une cible incompatible, protégée contre cette interruption ou sans préparation concernée n'a pas besoin d'un jet fictif. Les tirages restent dans la simulation ; prévisualisations et menus n'en consomment aucun.

### 12.6. Première interruption et protection contre les répétitions

Proposition de cas initial : interrompre une préparation en cours déclarée interruptible. En cas d'échec au jet de Stabilité, cette préparation prend fin ; les coûts déjà engagés ne sont pas remboursés et les coûts non encore engagés ne sont pas prélevés. Une action déjà résolue n'est pas annulée rétroactivement.

Cette interruption n'ajoute pas un tour automatiquement perdu : le personnage pourra choisir sa prochaine action normale, notamment se déplacer ou changer de tactique. Immobilisation, neutralisation complète, panne de module et déplacement forcé nécessitent leurs propres règles ; ils ne sont pas tous simulés par cette seule interruption.

Pour une même famille d'interruption, proposer au plus un test par cible et par action ou événement environnemental identifié, même si cette action produit plusieurs composantes de dégâts ou projectiles. Cela ne limite pas artificiellement les dégâts de la rafale.

Après une interruption effectivement subie, proposer une protection temporaire contre cette famille d'interruption : elle couvre immédiatement la fin de la fenêtre en cours, puis la prochaine action normale du personnage consommant du temps et la fenêtre adverse qui la suit. Elle expire au début de l'action normale suivante qui consomme du temps. Une préparation constituée de plusieurs étapes utilise les étapes effectivement consommatrices de temps pour ce décompte.

Lire un menu, annuler ou envoyer une commande refusée ne fait pas expirer ou renouveler cette protection. Une perturbation bloquée ne prolonge pas la fenêtre. Résister au jet sans subir d'interruption ne déclenche pas cette protection. Elle ne protège ni des dégâts, ni de la chaleur, ni automatiquement d'autres familles d'effets. La protection suit la famille d'interruption, pas le type électrique ou physique de sa source, afin d'éviter un contournement par alternance de sources.

Ce mécanisme reste à valider et à éprouver avec les préparations longues et plusieurs ennemis. Il vise à laisser une possibilité d'action, pas à garantir qu'une préparation risquée aboutisse quelles que soient les décisions du joueur.

### 12.7. Raccordement aux compétences existantes

| Technique | Défenses à distinguer | Limite à conserver |
|---|---|---|
| Surcharge / Surcharge en cascade | Résistance électrique pour les dégâts ; Stabilité seulement si le profil comporte une perturbation applicable. | Une décharge n'est pas une intrusion ; aucun test de pare-feu ajouté à la réception des dégâts. |
| Surchauffe | Défense numérique à l'implantation du sabotage ; résistance thermique sur les dégâts thermiques effectivement produits. | Le refroidissement et la température restent des mécanismes distincts ; pas d'apprentissage obligatoire d'Intrusion pour sa procédure standard. |
| Infection | Défense numérique contre l'implantation ; puis défenses correspondant aux effets explicitement déclarés. | Le nom ne crée pas une nouvelle catégorie de dégâts et ne répète pas gratuitement l'implantation à chaque étape. |
| Champ de saturation | Résistance du type de dégâts déclaré par la balise ; électrique dans la proposition courante. | Zone, énergie, durée et propagation restent ceux du dispositif. |
| Implosion | Défense du sabotage initial s'il est logiciel, puis défenses propres aux composantes de la destruction locale. | Une explosion n'est pas un unique jet logiciel contre tous ses voisins ; aucun effet gravitationnel ajouté. |
| Purge | Agit sur le programme hostile actif selon sa procédure. | N'annule pas les dégâts déjà subis et ne refroidit pas automatiquement le corps. |

Ces raccordements sont des propositions de formalisation, pas une modification des fiches existantes. Les nombres des techniques et leur disponibilité dépendent toujours des systèmes réellement présents.

### 12.8. Écarts constatés avec le prototype, sans modification du code

Lecture ciblée de `src/combat/damage.rs` lors de cette rédaction :

- le prototype définit huit types : `Kinetic`, `Piercing`, `Explosive`, `Thermal`, `Electrical`, `Chemical`, `Radiation` et `Corruption` ;
- `DamageRules::default()` fixe encore `minimum_damage_after_resistance` à 1 et les bornes de résistance à −100 et +100 ;
- `resolve_damage` soustrait actuellement `DamagePacket.penetration` à une résistance en pourcentage, puis traite un paquet ; il ne s'agit pas des points de Blindage de notre proposition ;
- le traitement d'un paquet ne constitue pas à lui seul le regroupement des composantes d'un même impact décrit ci-dessus.

Ce sont des écarts entre le prototype et la conception, pas des changements effectués. Modifier seulement le minimum de dégâts ne suffirait pas à implémenter le nouveau modèle. Une étape dédiée devra définir les unités, le regroupement par impact, la compatibilité du contenu existant et les tests, avant toute modification des données ou du code. Les profils existants à résistance 100 ne doivent pas être réinterprétés silencieusement comme 75.

### 12.9. Vérifications et prochaines décisions

Vérifications arithmétiques effectuées : les 15 lignes des tableaux de cette section et les 24 lignes des chapitres 10 et 11 ont été extraites du document et recalculées. Des séries de valeurs ont vérifié les bornes et la monotonie des résistances, les vulnérabilités, le zéro dégât, la croissance de Stabilité avec Résilience et la baisse de probabilité de résistance avec l'intensité. Les exemples de regroupement physique et d'arrondi par type ont aussi été vérifiés. Ce sont des calculs isolés, pas des essais de gameplay ou des tests du moteur.

Essais de simulation à prévoir : interruption effective ou résistée, action déjà terminée, absence de préparation, répétitions par plusieurs sources, expiration de la protection après une vraie fenêtre d'action, absence de renouvellement par menus et absence de fuite d'information dans l'aperçu. Aucun de ces comportements n'est déclaré testé dans le jeu par la rédaction de ces règles.

Les informations exactes affichées restent limitées à ce qui est connu du joueur. Une résistance ou une Stabilité cachée ne doit pas être déduite gratuitement d'un aperçu exact ; les résultats observables peuvent en revanche enrichir sa compréhension.

La suite documentaire demandée figure désormais dans les sections 13 à 19 et l'annexe technique du catalogue. Les paramètres des défenses déjà discutées restent à éprouver. Aucun nouveau catalogue de capacités extérieures n'est ouvert.

## 13. Intrusion et Défense numérique — proposition

### 13.1. Un accès n'est ni une vision ni une prise de possession

Une cible logicielle doit exposer une interface compatible. Vérifier cible, droit demandé, liaison, portée, matériel et ressources avant toute tentative. Le cas de référence est une liaison locale directe, non obstruée, de portée matérielle ; la fiche peut la réduire, jamais l'allonger. Les câbles et réseaux traversant des murs restent une extension, pas une exception implicite. INT-09 utilise uniquement des nœuds connus, autorisés et individuellement joignables dans cette version.

Un accès porte un propriétaire, une cible, un ensemble de droits explicites, une origine et une expiration : lecture, commande d'une fonction, modification de registre. Un accès à une porte ne donne pas celui de tout l'étage. Une commande autorisée et déterministe ne refait pas un jet de piratage. Un détournement de tourelle modifie ses consignes, sans contrôler le corps d'un ennemi comme la capacité extérieure différée.

Opération native de référence d'un outil intrusif : obtenir une session locale sur une interface exposée, 2 UT de préparation/exécution, 8 E, réservation de 1 B pendant la tentative puis tant que la session est active, un jet à la fin ; droits limités au profil de l'interface, expiration après 6 UT ou rupture de liaison. Elle n'exige pas INT-01 ou INT-02. Une clé ou une autorisation authentique utilise sa propre procédure, sans jet hostile. Une interface n'est pas obligée d'accepter cette opération ; une issue essentielle doit alors avoir une autre solution accessible.

### 13.2. Opposition chiffrée

```text
Intrusion = max(0, 65 + 5 × (Traitement − 5) + logiciel + modificateurs)
Défense_numérique = max(0, 50 + 4 × (Traitement − 5) + pare_feu + modificateurs)
Chance_logicielle = borner(50 + Intrusion − Défense_numérique + contexte, 5, 95)
```

Un entier uniforme de 1 à 100 inférieur ou égal à Chance_logicielle signifie une réussite. Les contributions du matériel sont des scores, pas des pourcentages multiplicatifs. Pas de contribution supplémentaire de Résilience, de rang ou de Perception à ce jet : leurs autres fonctions suffisent. Pour un appareil sans primaires, son profil fournit directement sa Défense numérique ; aucun Traitement humain fictif n'est nécessaire.

| Traitement attaquant | Bonus attaquant | Traitement défenseur | Pare-feu | Intrusion | Défense numérique | Chance |
|---:|---:|---:|---:|---:|---:|---:|
| 5 | 0 | 5 | 0 | 65 | 50 | 65 |
| 8 | 0 | 5 | 0 | 80 | 50 | 80 |
| 5 | 0 | 8 | 0 | 65 | 62 | 53 |
| 5 | 0 | 5 | 20 | 65 | 70 | 45 |
| 8 | 10 | 5 | 20 | 90 | 70 | 70 |

Une implantation réussie stocke sa force logicielle pour une éventuelle Purge ; elle ne multiplie pas aussi dégâts, durée et nombre de cibles par Traitement. Purge oppose l'Intrusion du nettoyeur + 20 à cette force stockée, avec la même formule et les mêmes bornes. Un nettoyage natif accessible aux non-spécialistes prend 2 UT et 8 E, sans bonus +20 ; outils/atelier compatibles constituent également des réponses. Aucun second jet de Stabilité n'est ajouté pour le même effet logiciel.

### 13.3. Tentatives, alertes et répétitions

- Une commande impossible d'après les informations connues est refusée sans coût. Une tentative légale qui rencontre une défense ou un état caché consomme son temps et les coûts engagés, sans diagnostic gratuit de la cause cachée.
- Une tentative hostile commencée produit un événement local de sécurité. Le profil fixe sa visibilité : référence d'essai, trace enregistrée immédiatement, examen après 5 UT, transmission seulement par un dispositif/liaison existants. Le passage de 3 à 5 UT est validé comme réglage de référence, pas comme délai obligatoire de toute sécurité. Aucun témoin omniscient et aucune modification mondiale automatique des factions.
- Un échec hostile ajoute +10 de Défense numérique contre cette famille de tentatives, au maximum +20, pendant 4 UT après le dernier échec ; l'événement survit à une simple déconnexion. Un nouvel essai reste possible, mais coûte temps et énergie. Une action autorisée n'est pas pénalisée.
- Les effets logiciels limitant une fonction suivent la durée et la protection contre répétition de la section 16. Un programme déjà actif n'est pas rafraîchi par réapplication du même effet. Une Purge manquée ne rend pas un programme plus fort.
- La perte de liaison interrompt une procédure en cours et les commandes maintenues. Un programme déjà implanté suit sa propre durée, sans devenir une télécommande permanente ; Surchauffe, Infection et Implosion précisent leurs réponses locales.
- Les droits, événements suspects, délais d'audit et copies sont des données structurées. Falsifier une preuve ne supprime ni ses copies ni un témoignage. L'ordre des événements décide si une falsification est arrivée avant l'audit.

Calendrier d'audit explicite : pour une trace créée pendant les actions du cycle C, sa première phase environnementale éligible est celle de C ; un audit de délai N intervient en phase environnementale C+N−1. Pour une trace créée pendant la phase environnementale C, commencer à C+1, donc auditer à C+N. L'audit de sécurité n'est pas un retardateur annoncé de détonation : ne pas lui ajouter le cycle de grâce de la section 16.2. Référence : trace au début du cycle 1, accès natif terminé au cycle 2, falsification préparée au cycle 3 et exécutée au cycle 4, audit en fin de cycle 5. Ce cas suppose un accès réussi avec les bons droits et une preuve identifiée ; sondage, échec et observations supplémentaires consomment leur temps réel.

INT-08 utilise le droit de modification déjà acquis : cette commande n'est pas une nouvelle tentative hostile d'accès et ne crée donc pas, par la règle générique ci-dessus, une chaîne infinie de traces d'intrusion à effacer. Un audit peut constater une falsification selon les données et comportements explicitement définis par le système ; elle ne rend pas invisibles les témoins, les copies ou les alarmes déjà transmis. Aucun délai caché n'est révélé au joueur sans source autorisée. Échéances, preuves et copies persistent à la sauvegarde sans réinitialisation des délais.

## 14. Perception, charge et mouvement — proposition

### 14.1. Portée, Détection et discrétion

Ordre impératif : canal matériel disponible → portée → visibilité/obstacles → signature et occultation → connaissance autorisée. L'affichage ne participe pas au calcul. Pour ces essais, distances sur la grille mesurées en cases par distance de Chebyshev ; diagonales sans passage entre deux obstacles bloquants. La portée des capteurs reste matérielle, sans bonus automatique de Perception. Un objet clairement exposé est visible sans test de furtivité supplémentaire.

Pour une cible réellement dissimulée ou un indice discret accessible au canal :

```text
Détection = max(0, 50 + 4 × (Perception − 5) + bonus_capteur + états)
Difficulté_discrétion = 40 + 4 × (Coordination_cible − 5)
                       + occultation + bonus_discrétion_cible
Score_observation = Détection − 2 × max(0, distance − 2)
Repérage si Score_observation ≥ Difficulté_discrétion
```

Pour les objets et traces, le contenu fournit directement la difficulté, sans Coordination artificielle. Barème d'essai de l'occultation : couvert partiel +15, éclairage insuffisant +10 seulement pour le canal optique, aucun de ces bonus pour un canal auquel il ne s'applique pas. Chaque source est comptée une fois. On conserve chaque canal séparé : un camouflage optique n'efface pas un contact thermique valide.

Ce choix déterministe évite les relances de recherche au même endroit jusqu'au succès. Recalculer seulement quand une condition pertinente change : position, capteur, posture, terrain ou indice. L'écoulement du temps seul ne relance aucun dé. Un contact insuffisant ne crée pas un point rouge à sa position : il faut un indice réellement obtenu, dont l'incertitude est conservée. Le balayage visuel reste une animation ; il ne retarde pas une information tactique déjà perçue.

Exemples : Perception 5, distance 2, aucun bonus → score 50, repère une difficulté 40 mais pas 55. Perception 8, distance 4 → score 58, repère 55. Un mur bloque les trois observations quel que soit le score.

### 14.2. Analyse, traces et secrets

```text
Analyse = max(0, 50 + 3 × (Perception − 5) + 2 × (Traitement − 5)
                  + bonus_d'analyse_matériel + états)
```

Les propriétés portent un canal requis, une source d'observation et un seuil : 40 pour une propriété simple, 60 pour un détail spécialisé, 80 pour un diagnostic difficile, en valeurs d'essai. Une technique sélectionne les propriétés qu'elle sait examiner et son bonus ; elle ne crée jamais des données absentes. Les propriétés évidentes et annonces de danger ne demandent ni score ni achat. La lecture ordinaire peut établir un état grossier ; les analyses spécialisées apportent les détails accessibles. Un même résultat est conservé avec sa date et sa source ; l'analyser à nouveau sans changement n'améliore pas aléatoirement le résultat.

Les traces sont créées par le monde, pas lors de l'inspection : type, orientation, date, difficulté, auteur interne non nécessairement identifié. Profil d'essai : au maximum 3 traces par case, expiration après 12 UT, remplacement de la plus ancienne ; objets sans contact au sol ou terrains incompatibles n'en produisent pas. Le joueur ne reçoit que les traces effectivement observées, puis des souvenirs datés. Les seuils simples/difficiles de traces sont 40/60 ; les terrains peuvent les modifier explicitement.

Un secret est placé avant la recherche, avec indice, difficulté et interaction distincte. Une inspection réussie le découvre mais ne l'ouvre/désarme pas. Réinspecter des conditions identiques ne procure pas de nouveau résultat. Les seuils 50/70/90 distinguent les secrets d'essai ; un seuil élevé n'est pas admis comme unique accès obligatoire. Aucun jet ou message ne doit confirmer gratuitement une cible inconnue par validation de commande.

### 14.3. Charge utile

```text
Charge_utile_kg = max(1, min(plafond_structure_kg,
                      portage_base_kg + 2 × (Puissance − 5) + assistance_kg))
```

La masse transportée comprend inventaire, équipements portés, munitions et contenu des récipients ; chaque objet une fois, pas le corps lui-même. Les masses sont stockées en grammes entiers ; aucun arrondi par pile ne crée du portage gratuit. La taille et les emplacements restent des contraintes séparées.

| Charge réelle | Effet proposé |
|---|---|
| Jusqu'à 100 % inclus | Pas de malus de charge. |
| Plus de 100 %, jusqu'à 150 % inclus | Chaque déplacement volontaire ordinaire prend 2 UT, Esquive −10 ; pas de Charge ou de propulsion rapide. |
| Plus de 150 % | Déplacement volontaire impossible ; déposer, réparer, attendre et agir sur place restent possibles. |

Déposer un objet est une action de 1 UT ; les écrans d'inventaire ne le déplacent pas. Une baisse de capacité ne détruit ni ne dépose silencieusement les objets. Exemple : portage de base 40 kg, plafond 60 kg → Puissance 3/5/8 donne 36/40/46 kg ; 50 kg sur une capacité de 40 kg impose le palier lent. Un équipement de portage de capacité supérieure n'augmente pas artificiellement la force de poussée.

### 14.4. Poussées, ancrage et traction

Proposition déterministe après une touche requise par la technique :

```text
Résistance_déplacement = plafond(masse_totale_cible_kg / 10) + ancrage
Poussée_possible si Force_du_geste ≥ Résistance_déplacement
```

Force_du_geste utilise l'Impact plafonné du matériel, avec le seul modificateur explicitement prévu par la technique. La masse comprend corps et charge ; l'ancrage est un score entier non négatif (0 libre, +10 pour Appui stable dans les essais). Un objet fixé ou une cible incompatible n'est pas déplaçable par ce test ; la destruction préalable de sa fixation est un autre effet. Aucun second bonus de Puissance ni test de Stabilité générique.

Une poussée parcourt une case, sans permutation gratuite, traversée de mur ou poussée récursive d'une file d'acteurs. Destination occupée ou infranchissable : déplacement bloqué, dégâts éventuels du coup conservés, **pas de dégâts de collision implicites**. Cases dangereuses/franchissables résolvent leurs dangers normalement ; une fosse n'est autorisée que si les règles de chute existent. Une case inexplorée ne révèle pas son occupant dans l'aperçu. Exemple : Force 10 peut déplacer 80 kg sans ancrage (seuil 8), pas 120 kg (seuil 12), ni 80 kg avec ancrage 10 (seuil 18).

Extraction utilise la limite de traction du matériel plutôt que de forcer un allié coopératif à échouer à un jet. Corps et charge de l'allié doivent tenir dans cette limite ; les deux positions finales sont distinctes et légales. Aucun déplacement ne crée d'attaque gratuite ou de réaction déclenchée par une autre réaction.

### 14.5. Contrôle distant

Proposition : ne pas ajouter de douzième secondaire. Nombre d'unités, portée, bande passante et complexité maximale viennent du contrôleur ; les techniques définissent les routines disponibles. Un ordre ordinaire valide est certain, mais consomme du temps. Le profil d'essai d'un contrôleur autorise 2 drones et 4 B ; une unité active réserve 1 B, une routine avancée 1 B supplémentaire lorsqu'indiqué. Hors liaison, pas de nouvel ordre ni vision directe : routine déjà chargée et rapport daté au retour. Les limites ne s'appliquent pas artificiellement aux compagnons organiques.

### 14.6. Modificateurs de tir et de locomotion

Complément du contexte de Précision de la section 10 : couvert partiel interceptant la trajectoire −20 ; distance au-delà de la moitié inférieure de la portée de l'arme −5 par case supplémentaire. Un obstacle opaque bloque l'attaque, il ne donne pas seulement un malus. Ni l'éclairage ni une analyse ne donnent un second malus/bonus universel après la vérification de perception ; leurs effets doivent être explicitement prévus.

Pour une arme déclarant un recul : pénalité de Précision = −max(0, recul_du_mode − Impact_de_maintien), où l'Impact de maintien utilise les mêmes règles de Puissance et de plafond matériel pour le geste de tenue, sans augmenter les dégâts des projectiles. Un profil sans recul déclaré utilise 0. Référence d'essai : rafale à recul 10, tenue à Impact 10 → aucune pénalité supplémentaire ; tenue à Impact 6 → −4. La dispersion native de la rafale est une autre source, appliquée une fois. Les exemples isolés de toucher de la section 10 restent explicitement sans ces modificateurs.

Pour les déplacements, charge, posture lente et entrave imposent chacun un coût minimal : prendre le plus grand, pas leur produit ou leur somme (trois contraintes à 2 UT donnent toujours 2 UT par case). Un terrain fournit également son coût minimal, entier positif. Les déplacements rapides sont interdits si charge ou entrave les rendent incompatibles ; ils ne suppriment pas les dangers ni les interdictions de terrain. Le geste de Percée utilise exactement min(plafond_matériel, Impact_disponible + 4) comme force : aucun dépassement du plafond grâce au bonus de technique.

## 15. Ressources — proposition

### 15.1. Unités et profil matériel de référence

E = unité entière d'énergie, H = unité entière de chaleur, B = unité de bande passante réservée, UT = unité de temps de simulation (section 16). Le profil de laboratoire utilise 100 E maximum, 0 H initial, dissipation 5 H/UT, seuil prudent 80 H, seuil critique 100 H et 4 B. Il n'impose ni le corps final du joueur ni ces réserves à tous les PNJ. Les coûts de l'annexe sont des profils d'essai pour un matériel standard ; aucun contenu final n'est créé ici.

### 15.2. Énergie, réparation et ravitaillement

- Pas de régénération universelle d'énergie ou de PV par attente. Batterie consommable, station alimentée ou générateur consommant son carburant produisent une quantité explicite et finie. L'attente refroidit mais ne remplit pas magiquement une batterie.
- Le coût d'une étape est prélevé quand elle commence légalement. Une préparation interrompue conserve ses dépenses ; une étape non commencée ne coûte rien. Les munitions/charges sont consommées au tir ou à la pose, pas à l'ouverture du menu.
- Monter la capacité de batterie ne remplit pas la réserve ; la baisser plafonne la réserve, sans gain au rééquipement. Une batterie-objet possède sa propre charge persistante : retirer/réinstaller ne la recrée pas. Un transfert débite la source d'au moins ce qu'il crédite au destinataire.
- Énergie spéciale à zéro : modules énergivores indisponibles, mais pas de mort ou de tour automatiquement perdu. Pour ce profil, locomotion de service, manipulation simple et frappe mécanique ordinaire ne coûtent pas E ; leur alimentation de service n'est pas une batterie rechargeable exploitable. Un matériel demandant E pour sa locomotion doit fournir une solution de secours avant intégration.
- Kit de réparation ordinaire d'essai : 1 UT, un consommable, restaure jusqu'à 20 PV ; recharge ordinaire : 1 UT, une cellule contenant 25 E au plus. Ces quantités ne justifient pas un stock illimité dans une carte.
- Pièces de réparation et objets récupérés portent une provenance/quantité. Réparation, démontage et assemblage ne peuvent pas restituer plus de matière que leurs entrées et le stock initial de l'objet. Un spécialiste choisit le résultat de récupération ; il ne duplique pas ses sorties.

### 15.3. Chaleur

À chaque phase environnementale : appliquer les apports périodiques de chaleur, calculer les dégâts critiques, puis refroidir. Les apports d'une action sont déjà présents ; ils ne sont pas ajoutés une seconde fois.

```text
Dégâts_thermiques_critiques_bruts = 5 × plafond(max(0, H − 100) / 10)
H_après_refroidissement = max(0, H − dissipation_effective)
```

Les dégâts passent par la résistance thermique, mais cette résistance ne réduit pas H. Entre 80 et 100 inclus : état thermique préoccupant, sans pénalité universelle cachée. Une action volontaire ajoutant de la chaleur est refusée si elle projette H au-delà de la limite de son mode : 100 pour le mode ordinaire, 140 pour le Surcadencement compatible. La validation utilise les coûts connus de l'étape, sans anticiper une source ennemie cachée.

```text
H_projetee = max(0, H_actuelle + Apport_H_action)
Refus_thermique = (Apport_H_action > 0) ET (H_projetee > Limite_H_mode)
```

Une action à apport thermique nul ou négatif n'est pas refusée pour ce motif, même si H dépasse déjà la limite. Attendre, se déplacer sans apport de chaleur, purger ou refroidir reste possible sous réserve des autres préconditions de l'action. À H=115 : +0 ou −5 est permis du point de vue thermique ; à H=95 : +5 est permis et +6 refusé en mode ordinaire. Les dégâts thermiques, les apports périodiques et le refroidissement se résolvent toujours : ce garde-fou n'est ni une immunité ni un soin. Une hausse ennemie peut dépasser les limites ; H n'est pas plafonnée à 100 et aucun dépassement ne déclenche un stun automatique. Représentation numérique assez large, contrôle de dépassement arithmétique requis.

Exemples sans résistance, dissipation 5 : H=100 → 0 dégât puis H=95 ; H=115 → 10 dégâts puis H=110 ; H=140 → 20 dégâts puis H=135. Une source refroidissante ne restaure pas les PV déjà perdus. Un ralentissement de temps n'accélère pas le refroidissement en ouvrant des menus.

### 15.4. Bande passante et réservations

B n'est pas consommée comme une munition : elle est occupée puis libérée. Vérifier les réservations atomiquement avant de lancer une action ; pas de dépassement transitoire gratuit. Un processus libère sa réservation à sa fin, sa destruction ou son annulation. Les balises autonomes consomment leur batterie propre ; elles ne conservent un canal que si leur profil le demande.

En cas de capacité réduite : suspendre les processus selon une priorité stable choisie à l'avance (à défaut, plus récent d'abord, puis identifiant). Une unité concernée passe à sa routine locale de sécurité ; elle ne disparaît pas. Revenir à une capacité supérieure ne relance pas gratuitement des attaques ni ne recharge une balise. L'achat d'une technique ne réserve pas de B en permanence.

## 16. Temps, états, réactions et exécution — proposition

### 16.1. Horloge de référence

1 UT représente un cycle de simulation de référence : action normale du joueur, opportunités normales des autres acteurs, phase environnementale. C'est un contrat de conception à raccorder à l'ordonnanceur, pas une vitesse d'animation ni la promesse d'un nouveau système d'initiative. Un acteur lent utilise plusieurs étapes ; les autres continuent d'agir entre elles. Un ordre de groupe ne multiplie pas leurs opportunités.

Notation de l'annexe : A1 = action simple d'une UT ; P1+A1 = préparation d'une UT puis exécution lors d'une seconde action ; P2+A1 = deux étapes de préparation puis exécution ; R1 = récupération pendant la prochaine action normale, qui interdit une nouvelle attaque ou préparation offensive mais autorise déplacement, attente ou soutien. Chaque étape prend une UT et laisse agir les adversaires. Une préparation peut être abandonnée par une autre action, sans remboursement ; abandonner dans le menu seul ne donne aucun effet ni nouvelle fenêtre.

Déplacements : une case par UT par défaut. Un déplacement lent de 2 UT expose d'abord le départ puis résout l'arrivée à la seconde étape. Les techniques de propulsion peuvent explicitement traverser 2 cases dans une UT ; chaque case résout obstacles, mines et Surveillance. Une Charge avance case par case sur plusieurs UT, puis frappe lors de sa dernière étape prévue ; si le contact prévu manque, pas de poursuite automatique ou d'attaque offerte.

### 16.2. Ordre de résolution et définitions temporelles

1. Valider la commande avec les informations connues ; traiter les inconnues sans oracle de validation.
2. Pour une action normale qui démarre vraiment, expirer l'ancienne garde et rétablir au plus une réaction ; prélever le coût de l'étape, installer sa préparation ou déclarer son effet.
3. Résoudre les réactions admissibles dans l'ordre déterministe des acteurs concernés, avec priorité de protection fixée à la préparation. Une protection qui rend la cible/trajectoire impossible peut faire échouer l'attaque déjà engagée, dont les ressources restent dépensées.
4. Résoudre mouvement ou touche, dégâts, effets secondaires et décès ; une cible détruite ne reçoit pas une nouvelle commande. Les réactions n'en déclenchent pas d'autres. Un rebond, projectile ou hôte est visité selon un ordre stable, pas celui du rendu.
5. Donner leurs opportunités normales aux acteurs selon l'ordonnanceur ; puis, en phase environnementale, résoudre retardateurs et effets périodiques, dégâts thermiques, refroidissement et expirations.

Pour une durée d'effet D : expiration à la fin de la D-ième phase environnementale **strictement postérieure à la phase d'application**. Un effet créé au cours d'une phase environnementale commence son décompte à la suivante. Une application pendant les actions ordinaires peut donc agir à la phase environnementale qui suit. Les périodes utilisent les mêmes échéances, avec un identifiant pour éviter deux traitements dans une phase. Les mouvements futurs ne consomment pas une durée à la vitesse des images.

**Retardateur annoncé, convention validée distincte :** le cycle où il est armé ne compte pas. Si l'armement a lieu pendant le cycle C, au cours des actions ou de la phase environnementale, la première phase éligible est C+1 et l'événement de délai D≥1 se produit en phase environnementale C+D. Un délai 1 laisse une prochaine action ordinaire de 1 UT avant l'événement ; un délai 2 laisse deux étapes ordinaires. Une action ralentie ou une longue préparation peut ne pas tenir dans cette fenêtre, et aucune réponse n'est garantie de réussir.

Cette convention s'applique aux fusibles de charges, délais d'armement des mines, annonces d'effondrement, séquences de détonations et retard d'Implosion. Elle ne décale pas les tics d'Infection, les apports de Surchauffe, les expirations d'états, les cooldowns ou les audits de sécurité. Une commande de déclenchement distant ou d'activation manuelle reste une commande à son moment d'exécution, pas un retardateur supplémentaire non prévu.

Exemples de référence : charge posée en cycle C → détonation en fin de C+1 ; stockage compromis en C avec Implosion D=2 → détonation en fin de C+2 ; Effondrement contrôlé annoncé à la fin du montage en C → chute en fin de C+1. Pour Détonation combinée, première charge en fin de C+1 puis seconde en fin de C+2, chacune sur le terrain alors réel. Sauvegarder le type d'échéance, le cycle d'origine, le cycle dû et le fait que l'événement a été exécuté ; charger ou rouvrir un menu ne redonne aucun délai. Si une échéance est atteinte au chargement, sa phase doit être reprise exactement une fois, pas sautée ou rejouée.

Un délai de récupération CD=N interdit un nouveau lancement jusqu'à la fin de N phases environnementales éligibles après l'exécution ou l'échec de la tentative finale ; une préparation interrompue avant cette tentative n'engage pas le CD, sauf mention explicite. Les variantes partagent le CD de leur technique mère. R1 et CD ne sont pas des synonymes : l'un limite la prochaine action, l'autre une technique précise.

Pour les états appliqués lors d'une phase, la règle « strictement postérieure » évite leur expiration avant toute occasion de réponse. Les effets différés qui arrivent simultanément utilisent un ordre stable et un état actualisé ; un mur détruit ne protège pas fictivement contre l'explosion suivante.

### 16.3. Cumuls et réponses

| Famille | Règle d'essai |
|---|---|
| Préparation offensive | Une seule à la fois ; déplacement, changement de cible ou perte du contact requis annulent la visée. Aucun stockage de plusieurs bonus de préparation. |
| Garde active | Une seule posture parmi Parade, Surveillance, Interception et Esquive préparée ; expire à l'action normale suivante ou à son déclenchement. |
| Modificateur du même type | Garder le plus fort, jamais une somme indéfinie. Une nouvelle application n'allonge pas une pénalité déjà active. Sources de familles distinctes s'additionnent uniquement sur le paramètre annoncé et dans ses bornes. |
| Entrave locomotrice | Déplacement ordinaire coûte 2 UT, pas d'immobilisation totale ; dure 2 UT puis protection contre une nouvelle entrave pendant 1 UT. Charge/propulsion rapide indisponibles pendant l'effet. |
| Suppression | Précision −15 pendant 1 UT, seulement sur exposition compatible ; pas d'ordre forcé à fuir. Protection 1 UT après expiration. |
| Fonction suspendue / commande neutralisée | Une fonction nommée, 2 UT maximum dans les profils standards ; autres fonctions accessibles. Protection contre la même famille de neutralisation pendant 1 UT après expiration. Pas de désactivation de toutes les fonctions par plusieurs sources. |
| Interruption de préparation | Stabilité, une tentative par action/famille et protection de la section 12.6. Pas de tour suivant automatiquement perdu. |
| Fragilisation du Blindage | Valeur la plus forte, durée 3 UT ; pas de rafraîchissement pendant la durée active. Une nouvelle application après expiration est possible et coûte une action. |
| Programme périodique | Un même programme/famille par hôte ; pas de rafraîchissement. Purge, expiration ou destruction l'arrêtent. La provenance de la campagne est conservée pour borner la contagion. |
| Posture de discrétion | Une posture corporelle (Pas feutrés ou Profil réduit), combinable avec des fonctions distinctes mises en veille ; aucun cumul double d'un même couvert. |

Les protections locomotrices, de suppression et de fonction commencent après l'expiration ou la purge de l'effet réellement subi, et durent une phase éligible. Elles suivent la famille, non l'attaquant. Leur réapplication bloquée ne prolonge rien. Les autres dégâts restent possibles. Un profil doit déclarer sa famille ; il ne peut contourner ces règles par un simple nom différent.

Les effets multiples d'un même impact ne s'opposent pas tous à la même défense : touche physique puis effet locomoteur distinct éventuellement opposé à Stabilité ; logiciel contre Défense numérique sans second jet pour la même suspension. Toute condition supplémentaire figure dans la fiche, pas dans un bonus implicite de rang.

### 16.4. Réactions précises

- Parade : après une touche de mêlée admissible, consomme la réaction ; réduit de 50 % sa composante physique brute, arrondi inférieur, **avant** Blindage. Pas de réduction de la composante électrique. Une parade consommée est réussie même si le Blindage finit d'absorber le reste. Riposte peut alors produire une frappe ordinaire dans cette même réaction, si portée et ressources restent valides.
- Esquive préparée : avant résolution de l'attaque perçue, déplacement d'une case vers le repli choisi et encore accessible, consommation de la réaction. Le projectile engagé conserve son point visé ; aucun suivi gratuit. Une zone couvrant l'arrivée peut toujours toucher. Si la destination connue comme libre est devenue inaccessible, pas de téléportation ; aucun coût de déplacement n'est pris si aucun mouvement ne démarre.
- Surveillance : un tir natif simple, un projectile, sur le premier ennemi actuellement identifié/perçu qui entre dans les cases couvertes, après son entrée et avant sa prochaine étape. Le tireur ne sait rien d'une cible cachée ; munitions et énergie vérifiées au déclenchement.
- Interception : contre un départ volontaire depuis le contact, avant ce départ ; une frappe ne bloque pas automatiquement le mouvement. Pas de déclenchement sur poussée ou mouvement de réaction. Pas de riposte à une interception.
- Interposition : le drone peut occuper une case libre adjacente sur la trajectoire d'un projectile direct avant son impact ; il consomme sa réaction et subit le projectile selon ses défenses. Pas d'échange de places, de téléportation ou d'annulation de toute explosion.

Un réglage de priorité désigne une protection admissible ; une cible ne bénéficie pas de plusieurs interpositions en cascade sur le même projectile. Chaque acteur garde au plus une réaction entre deux actions normales, mais les réactions de plusieurs unités restent un risque d'équilibrage à mesurer.

### 16.5. Zones, champs et explosions

Disque de rayon r = cases dont la distance de Chebyshev au centre est au plus r et qui sont atteignables par les lignes d'effet prévues. Une paroi/porte fermée bloque les émissions électroniques et les explosions de référence ; le mur lui-même peut recevoir des dégâts si destructible. Pour une même explosion, calculer les cases affectées sur le terrain avant cet impact ; seule une explosion ultérieure profite d'une brèche nouvellement créée. Les cônes de référence couvrent 90 degrés selon une direction parmi huit, avec la même portée et les mêmes obstacles.

Un champ applique son impact au plus une fois par acteur et par UT, à l'entrée ou à la phase environnementale s'il y reste ; placer un champ sous un acteur compte comme une entrée. Entrées répétées par poussées dans cette même UT ne le multiplient pas. Deux champs de même famille n'infligent que le plus fort impact de cette famille dans l'UT ; leurs batteries se dépensent néanmoins. Détruire une balise arrête ses futurs impacts, sans annuler ceux déjà résolus.

Les cibles alliées/neutres sont exposées sauf filtrage matériel explicite. Aucun filtre ne devine une trahison future. Une zone visant une case connue peut affecter un occupant inconnu sans le révéler dans l'aperçu. Les dégâts et témoignages produisent uniquement les informations autorisées par les règles d'observation.

## 17. Création et progression — profil d'essai, pas classes définitives

### 17.1. Budget et augmentations

Conserver les cinq primaires, la somme initiale 28, les bornes de création 3–8 et le plafond absolu 10. Le profil d'essai commence au niveau 1 avec **2 points de compétence**. La classe peut préallouer tout ou partie de ces deux points à ses rangs/choix de départ : elle ne les ajoute pas au budget. Deux rangs 1 dans deux disciplines ou un rang 2 coûtent donc 2 points et donnent deux choix. Aucun nom ou contenu de classe n'est défini ici ; le matériel de départ compatible sera validé avec les classes.

Chaque niveau gagné du 2 au 20 donne 1 point ; aucun bonus automatique de Précision, dégâts, PV ou rang. Un point primaire est proposé aux niveaux 4, 8, 12, 16 et 20 : au plus cinq points supplémentaires, donc somme 33 au niveau 20 avant états temporaires. Les points non dépensés restent disponibles, sans franchir le plafond de 10. Les modifications temporaires bornent la primaire effective entre 1 et 10 ; elles n'ouvrent pas rétroactivement des choix de rang.

Tarif de rang conservé pour essai : 1, 1, 2, 2, 3 ; rangs achetés dans l'ordre, un choix éligible obligatoire à chaque achat. Pas de bonus numérique passif lié au rang lui-même. Pas de prérequis de primaire ajouté aux techniques actuelles. Une amélioration demande sa technique mère ; on ne peut pas dépenser le même choix sur la mère et sa variante. Les acquis de classe comptent dans les cinq choix de la discipline.

### 17.2. Courbe d'XP proposée

Pour L de 1 à 19 :

```text
XP_de_L_vers_L_plus_1 = 100 + 40 × (L − 1)
XP_cumulée_pour_L = 100 × (L − 1) + 20 × (L − 1) × (L − 2)
Budget_compétences_au_niveau_L = 2 + (L − 1)
```

| Niveau | XP cumulée | Budget total de points | Augmentations primaires gagnées |
|---:|---:|---:|---:|
| 1 | 0 | 2 | 0 |
| 5 | 640 | 6 | 1 |
| 10 | 2340 | 11 | 2 |
| 15 | 5040 | 16 | 3 |
| 20 | 8740 | 21 | 5 |

Le plafond 20 sert de scénario de fin de partie à tester, pas de verrou d'accès aux couches. Les XP au-delà peuvent être conservées sans accorder de niveau supplémentaire dans ce profil. Le moteur doit charger une courbe de contenu, pas coder définitivement cette formule. Une récompense franchissant plusieurs seuils accorde chacun une seule fois, sans réparation ni recharge gratuite.

### 17.3. Récompenses sans obligation d'extermination

Barèmes initiaux d'auteur, indépendants du niveau actuel pour ne pas récompenser l'attente avant validation : menace ordinaire 25 XP, dangereuse 60, majeure 120 ; découverte mineure 25, lieu important 100, objectif secondaire 150, objectif majeur 300. La valeur réelle et le niveau de menace sont portés par le contenu. Une différence d'au moins 3 niveaux en faveur du joueur rend une élimination triviale à 0 XP dans ce profil, cohérent avec la règle native existante. Les récompenses d'exploration/objectifs ne subissent pas cette pénalité.

Défaite, contournement, infiltration ou résolution pacifique d'un **même objectif d'obstacle** partagent une clé de récompense lorsqu'ils constituent des alternatives. Définir précisément la condition de résolution dans le contenu, pas « avoir marché près d'un ennemi ». Tuer ensuite cet ennemi ne verse pas une deuxième fois la récompense déjà rattachée à cet obstacle. Une découverte indépendante peut se cumuler si elle possède un autre objectif réel et une autre clé.

Les éliminations par alliés sont attribuées au propriétaire pertinent une seule fois ; changer d'ordre, achever une cible ou changer de carte ne recrée pas sa clé. Invocation, fabrication, réanimation reproductible, réparation/démontage de ses objets, réouverture de porte et réinfection ne produisent pas de nouvelle source d'XP. Les identifiants persistants des récompenses doivent survivre à la sauvegarde et aux revisites.

Hypothèse de calibration d'une route complète : environ 9 000 XP disponibles par objectifs, découvertes et résolutions effectivement accessibles. Comparer routes combattante, discrète et numérique sans exiger la somme de toutes les activités. Cette enveloppe est un besoin de contenu à tester, pas une promesse qu'une carte actuelle l'offre. Aucun taux de victoire ou scaling automatique des ennemis au niveau n'est introduit.

### 17.4. Choix, maîtrise et réattribution

À 21 points, deux maîtrises coûtent 18 ; il reste 3 points, par exemple pour un rang 2 et un rang 1. Les dix maîtrises coûteraient 90 : une run ne donne donc pas tout le catalogue. Le rang 5 de Reconnaissance conserve son tarif commun de 3 pour un cinquième choix antérieur ; son rapport utilité/coût reste un risque, à mesurer avant réduction particulière.

Proposition prudente pour ce premier profil : pas de réattribution en cours de run et pas d'achat supplémentaire après cinq choix. Ce choix est **à valider**, non une suppression définitive de la possibilité : la simulation doit d'abord mesurer les erreurs de build, et une réattribution limitée pourra faire l'objet d'une décision distincte. Annuler un choix avant de confirmer l'achat est sans coût ; un achat est atomique (rang, points et technique ensemble), sans possibilité d'obtenir un rang vide.

Les achats se font entre deux commandes, sans avancer le temps et sans restaurer ressources, réaction ou cooldown. Une technique compatible avec la version peut être apprise avant d'avoir son matériel ; une technique dépendant d'un système inexistant dans cette version est masquée de l'achat, ainsi que ses améliorations devenues sans prérequis disponible. Les techniques apprises ne disparaissent pas avec l'équipement. Mort : progression de run perdue ; déblocages horizontaux des classes stockés séparément.

**Disponibilité des disciplines, règle validée :** dans une version de test, différer l'ouverture d'une discipline dès qu'une suite légale de choix ne peut pas être prolongée jusqu'à cinq achats éligibles. Ne pas proposer ses premiers rangs en promettant une maîtrise impossible. Le validateur applique les dépendances de version et leurs prérequis transitifs, puis explore chaque état d'apprentissage atteignable avant le rang 5 ; aucun ne doit mener à une impasse. Compter simplement cinq fiches ou trouver un seul build valide ne suffit pas. Un plafond temporaire inférieur nécessite une décision explicite distincte ; aucun n'est ajouté ici.

La vérification inclut les acquis préalloués de chaque classe : ils consomment les rangs et choix prévus et doivent appartenir au catalogue disponible, avec une suite possible. Au chargement d'une sauvegarde, une entrée retirée comme MAN-07, une discipline devenue indisponible ou un prérequis absent demande une migration versionnée explicitement décidée, ou un refus explicite de compatibilité. Ne pas effacer de choix, réattribuer de points ou offrir des acquis de remplacement silencieusement. Les fonctions natives du matériel restent disponibles selon leurs règles même si une discipline est différée.

## 18. Contrats de données et validation de version — proposition

Les 104 profils de l'annexe 16 du catalogue sont des fiches de conception. Une fiche complète est **la ligne du profil + sa ligne qualitative existante + les règles communes explicitement héritées**. Une cellule ne décrivant aucun coût supplémentaire n'accorde pas une action gratuite. Le matériel définit ses propriétés natives ; les coefficients d'essai ne doivent pas être injectés dans les anciens objets sans migration.

### 18.1. Données minimales

| Élément | Données à conserver / vérifier |
|---|---|
| Technique | Identifiant stable, discipline, rang minimal, prérequis, type (action/amélioration/posture), dépendances de systèmes, profil/version, coûts et moments de prélèvement, portée/géométrie, cible/canal, étapes, effets, déclencheurs, familles, durées, CD et contre-mesures. |
| Matériel | Compatibilités, dégâts et composantes par impact, unités de pénétration, force/plafond, masse, énergie réelle, dissipation, réservations, munitions, paramètres de dispersion et ressources de récupération. |
| Personnage | Primaires de base, points non dépensés, rangs et identifiants des choix, corps principal stable, matériel et sources des bonus. Secondaires recalculées, sans double stockage faisant diverger les valeurs. |
| Effet actif | Source/propriétaire, cible, famille, intensité, phase d'application, type d'échéance (effet, retardateur, audit), cycle d'origine, cycle dû, marqueur d'exécution, expiration, cooldown, protection après fin, coût réservé, origine de contagion et hôtes déjà visités. |
| Temps et hasard | Phase et compteur UT, états de préparation/récupération, droit à réaction et garde, état du générateur aléatoire ; aucun tirage lié à l'affichage. |
| Connaissance | Observateur, cible/case, propriété connue, provenance, date, canal et degré de certitude ; séparée de l'état réel du monde. |
| Accès et progression | Droits, liaisons, clés d'XP déjà consommées, copies de preuves, files d'audit, masque de fonctionnalités de version, disciplines ouvertes/différées, acquis de classe et méta-déblocages hors sauvegarde de run. |
| Textes | Identifiants de localisation et statut de validation séparés des nombres ; aucune phrase proposée ne devient approuvée parce qu'une fiche technique existe. |

Le chargement refuse les rangs hors 0–5, cycles de prérequis, références retirées, durées négatives, ressources impossibles et profils sans dépendances. Les nombres intermédiaires doivent prévenir les dépassements ; les données doivent identifier leur version. Un identifiant absent ou un ancien sens de pénétration demande une erreur explicite ou une migration versionnée, jamais une substitution silencieuse.

### 18.2. Écarts à raccorder au prototype

Lecture locale du 10 septembre : `src/progression/experience.rs` utilise encore la courbe d'amorçage [10, 25, 45, 70, 100, 140, 190], un point par niveau et des règles anti-farming. La nouvelle courbe n'y est pas appliquée. `src/game/turn.rs` expose quatre phases ; cela ne prouve pas les préparations, fenêtres ou coûts multi-UT décrits ici. Le minimum de dégâts, les résistances et la pénétration présentent les écarts de la section 12.8, toujours constatés lors de cette livraison.

Les prochains travaux de code devront couvrir, dans cet ordre : données/prérequis/sauvegarde ; dégâts/secondaires ; temps et réactions ; ressources/états ; perception/indices ; terrain/dispositifs ; accès/sécurité ; drones. Chaque étape doit garder un test déterministe hors rendu. **Aucun fichier de jeu n'est modifié par cette livraison documentaire.**

## 19. Vérification, équilibrage et textes de l'étape 7

Le [rapport de vérification](RAPPORT_SYSTEME_STATISTIQUES_COMPETENCES.md) distingue contrôles documentaires/arithmétiques exécutés et scénarios de moteur encore à réaliser. Le contrôle reproductible de cette livraison est `node tools/validate_stats_docs.mjs`, sans bibliothèque tierce ni lancement du jeu.

Critères : pas de prérequis impossible ou d'entrée retirée réintroduite ; budgets cohérents ; défenses sans double comptage ; pas de ressources/XP/attaques gratuites par menu, sauvegarde, rééquipement ou ordre ; limites de perception identiques à toutes les résolutions ; réponse réellement accessible aux obstacles obligatoires. Tester aussi une version dont certaines dépendances ne sont pas disponibles : ne pas vendre une technique sans effet.

Les profils à comparer incluent mêlée mobile, tir/reconnaissance, démolition/furtivité, guerre électronique/intrusion, drones/ingénierie et généraliste. Mesurer durée d'engagement, dépenses, actions de préparation, dégâts moyens, dommages extrêmes, choix jamais utiles, mortalité contextualisée et causes de blocage. Ne pas annoncer le jeu équilibré à partir de tables ou de quelques seeds.

**Étape 7 — rédaction achevée sur délégation :** l'utilisateur a validé les quinze textes ci-dessous, puis demandé de terminer les autres sans approbation individuelle préalable. Les formulations déjà validées restent conservées à l'identique. Les textes complémentaires, leurs situations d'affichage et leurs limites d'information sont réunis dans les [textes joueur](TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md). Ils sont rédigés sous cette délégation, et non présentés comme relus un à un par l'utilisateur. Aucun texte n'est intégré au code par cette livraison ; les coefficients restent des barèmes d'essai.

### 19.1. Textes joueur validés individuellement

#### Texte 1 — Puissance

Statut : validé par l'utilisateur (« oui ca me va »), après présentation de cette formulation. Usage prévu : création de personnage et infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Renforce vos frappes physiques au corps à corps et votre capacité à repousser les adversaires. Améliore la charge que vous pouvez transporter et la maîtrise du recul, dans les limites de votre équipement.
>
> N’augmente pas les dégâts des projectiles.

#### Texte 2 — Coordination

Statut : validé par l'utilisateur (« parfait »), après présentation de cette formulation. Usage prévu : création de personnage et infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Améliore la précision de vos attaques et de vos lancers, ainsi que votre capacité à esquiver et à rester discret.
>
> N’augmente pas votre vitesse d’action.

#### Texte 3 — Résilience

Statut : validé par l'utilisateur (« oui c'est tres bien, continue. »), après présentation de cette formulation avec le vocabulaire Points de vie. Usage prévu : création de personnage et infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Augmente vos points de vie maximaux et améliore la stabilité de vos systèmes, pour mieux résister aux interruptions et à certaines perturbations.
>
> N’augmente pas votre blindage et ne restaure pas les points de vie perdus.

#### Texte 4 — Perception

Statut : validé par l'utilisateur (« oui :) »), après présentation de cette formulation. Usage prévu : création de personnage et infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Améliore votre capacité à repérer les présences dissimulées et les indices discrets, ainsi qu’à analyser vos observations. Contribue également à la précision de vos tirs.
>
> N’augmente pas la portée de vos capteurs et ne permet pas de voir à travers les murs.

#### Texte 5 — Traitement

Statut : validé par l'utilisateur (« d'accord :) »), après présentation de cette formulation. Usage prévu : création de personnage et infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Améliore l’efficacité de vos intrusions et votre résistance aux attaques logicielles. Facilite l’analyse des informations disponibles.
>
> N’augmente ni vos réserves d’énergie ni votre bande passante.

#### Texte 6 — Points de vie (PV)

Statut : validé par l'utilisateur (« oui parfait :) »), après présentation de cette formulation. Usage prévu : infobulle de la jauge de vie du personnage joueur. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Indiquent vos points de vie actuels et leur maximum. Les dégâts subis réduisent vos PV actuels. À zéro PV, votre personnage meurt et la partie prend fin.
>
> Augmenter vos PV maximaux ne restaure pas les PV perdus.

#### Texte 7 — Blindage (armure)

Statut : validé par l'utilisateur (« tres bien :) »), après présentation de cette formulation. Usage prévu : infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Réduit les dégâts physiques des coups, des projectiles et des explosions. Un blindage suffisant peut absorber entièrement un impact.
>
> Ne protège pas contre les dégâts thermiques, électriques ou chimiques, ni contre les attaques logicielles.

#### Texte 8 — Esquive

Statut : validé par l'utilisateur (« parfait :) »), après présentation de cette formulation. Usage prévu : infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Diminue les chances d’être touché par les coups et les tirs dirigés contre vous.
>
> Ne réduit pas les dégâts d’une attaque qui vous touche et ne permet pas d’éviter une explosion couvrant votre position.

#### Texte 9 — Précision

Statut : validé par l'utilisateur (« oui »), après présentation de cette formulation. Usage prévu : infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Améliore vos chances de toucher avec vos coups et vos tirs. L’esquive de la cible, la distance et les couverts peuvent réduire ces chances.
>
> N’augmente ni les dégâts ni la portée de vos armes.

#### Texte 10 — Impact physique

Statut : validé par l'utilisateur (« oui :) »), après présentation de cette formulation. Usage prévu : infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Contribue aux dégâts physiques de vos frappes au corps à corps et à la force de vos poussées. Son efficacité reste limitée par votre équipement et, pour déplacer une cible, par sa masse et son ancrage.
>
> N’augmente pas les dégâts des tirs ou des explosions.

#### Texte 11 — Charge utile

Statut : validé par l'utilisateur (« oui :) »), après présentation de cette formulation. Usage prévu : infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Indique le poids que vous pouvez transporter sans pénalité, équipement porté et contenu de l’inventaire compris.
>
> Un excès de charge ralentit vos déplacements et réduit votre esquive. Une surcharge trop importante empêche tout déplacement volontaire.

#### Texte 12 — Stabilité système

Statut : validé par l'utilisateur (« ca me va. »), après présentation de cette formulation. Usage prévu : infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Réduit le risque que vos préparations soient interrompues et aide vos systèmes à résister à certaines perturbations.
>
> Ne réduit pas les dégâts reçus et ne bloque pas les intrusions.

#### Texte 13 — Détection

Statut : validé par l'utilisateur (« tres bien :) »), après présentation de cette formulation. Usage prévu : infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Améliore votre capacité à repérer les présences dissimulées et les indices discrets à portée de vos capteurs.
>
> N’augmente pas leur portée et ne permet pas de voir à travers les murs. Repérer une présence ne révèle pas automatiquement son identité ou ses capacités.

#### Texte 14 — Analyse

Statut : validé par l'utilisateur (« ca me va »), après présentation de cette formulation. Usage prévu : infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Facilite l’interprétation de vos observations pour identifier l’état, le fonctionnement ou les faiblesses d’une cible.
>
> Les informations obtenues dépendent de vos capteurs et des données disponibles. N’augmente pas directement les dégâts de vos attaques.

#### Texte 15 — Efficacité d’intrusion

Statut : validé par l'utilisateur (« oui, c'est tres bien »), après présentation de cette formulation. Usage prévu : infobulle de la statistique. Le texte ci-dessous est conservé à l'identique ; toute modification de sa formulation devra être soumise à nouveau.

> Améliore vos chances de franchir les défenses logicielles d’une cible lors d’une intrusion ou de l’implantation d’un programme hostile.
>
> Nécessite une interface compatible et une liaison valide. N’augmente pas directement les dégâts de vos attaques.

Les quinze textes ci-dessus sont validés individuellement. Les textes restants ont ensuite été rédigés sous délégation explicite, selon le suivi de la section 19.3.

### 19.2. Décision terminologique validée

Après discussion du terme « Intégrité » et de l'alternative « Constitution », l'utilisateur a accepté (« ca me va :) ») la distinction Points de vie (PV) pour les acteurs et Durabilité pour le matériel et les objets destructibles. La documentation adopte ces termes, sans modifier les mécaniques ni le code du jeu. La formulation complète du texte de Résilience avec ce vocabulaire a ensuite été validée individuellement et figure en section 19.1.

### 19.3. Achèvement rédactionnel sur délégation

Après le texte 15, l'utilisateur a demandé : « tu peux les terminer puis on passera au (je crois) dernier point ». Cette demande remplace l'attente de validation individuelle pour les textes encore non présentés, sans autoriser de nouvelle mécanique ou de développement du jeu.

Le [complément de textes joueur](TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md) couvre Défense numérique, valeurs matérielles et ressources, apprentissage, dix disciplines, 94 techniques et 10 variantes après fusion de MAN-07, états, messages d'aide, refus et résultats observables. Les quinze textes ci-dessus ne sont pas dupliqués dans ce complément. Les chiffres affichés devront provenir du profil de règles et du matériel effectivement chargés, sans exposer de valeurs cachées.

Le dernier point du plan est l'étape 8, vérification et mise à l'épreuve. Sa première passe documentaire existe déjà ; les contrôles de couverture rédactionnelle l'étendent. Les essais jouables, l'équilibrage et le raccordement au moteur restent à réaliser séparément. Les décisions de fond encore ouvertes restent identifiées dans le rapport ; la fin de l'étape 7 ne les tranche pas.

### 19.4. Cinq corrections validées à la suite du point 8

L'utilisateur a répondu « oui :) » à la proposition des cinq corrections, après lecture de leur synthèse. Elles sont appliquées à cette révision documentaire : refus thermique limité aux apports positifs dépassant la limite du mode (§15.3) ; retardateurs annoncés sans décompte du cycle de création (§16.2) ; audit de référence à 5 UT, distinct des retardateurs (§13.3) ; maintien de consigne intégré sans achat supplémentaire à Pas de dégagement, MAN-07 conservé seulement dans l'historique du catalogue ; ouverture des disciplines conditionnée à cinq choix atteignables sur toute suite légale, avec contrôles des acquis et versions (§17.4).

La portée de cet accord reste documentaire. Aucun paramètre de dégâts, coût de rang, choix de réattribution ou capacité extérieure n'est modifié en plus. Les quinze infobulles validées individuellement sont inchangées. Le [rapport, section 10](RAPPORT_SYSTEME_STATISTIQUES_COMPETENCES.md#10-application-des-cinq-corrections-validées) conserve le bilan de l'application et des contrôles ; sa section 9 reste le constat historique ayant motivé ces corrections.

## 20. Sources et entretien

- [Catalogue des compétences](PROPOSITION_COMPETENCES_v0.1.md), sections 1 à 3 : décisions actuelles, rangs, coûts d'essai et règles transversales ; sections 4 à 8 : fiches.
- [Document de conception](../Projet_Roguelike_IA_Document_Conception_v0.2.md), sections 7 et 8 : corps principal et progression.
- [Spécification CODEX](../Projet_Roguelike_IA_Spec_CODEX_v0.2.md), section 17.5 : expérience propre à la partie et distinction avec les déblocages permanents.

Les documents fondateurs renvoient ici pour le détail en cours de rédaction. Le catalogue conserve les fiches techniques afin d'éviter deux listes concurrentes. Tout passage de « proposition » à « confirmé » doit correspondre à une décision utilisateur identifiable ; aucun code du jeu n'est modifié par cette rédaction.
