# Project RL — Textes joueur : statistiques et compétences

Version rédactionnelle 2 — 10 septembre 2026 — Étape 7 complétée par les cinq corrections validées du point 8, sans intégration au jeu.

## 1. Statut, périmètre et références

Après validation individuelle des quinze premières infobulles, l'utilisateur a explicitement demandé de terminer les textes restants sans les lui soumettre un par un. Ce document est rédigé sous cette délégation : ses formulations ne sont pas présentées comme ayant chacune reçu une validation individuelle. Les quinze textes déjà approuvés restent inchangés dans les [règles communes, section 19.1](STATISTIQUES_ET_COMPETENCES.md#191-textes-joueur-validés-individuellement), sans copie concurrente ici.

Références mécaniques : [règles communes, sections 10 à 17](STATISTIQUES_ET_COMPETENCES.md) et [catalogue, fiches qualitatives et annexe 16](PROPOSITION_COMPETENCES_v0.1.md). En cas d'ambiguïté, corriger la rédaction ou demander une décision ; un texte ne doit jamais créer une mécanique. Les valeurs chiffrées restent des paramètres d'essai. Le [rapport](RAPPORT_SYSTEME_STATISTIQUES_COMPETENCES.md) distingue vérifications documentaires et futurs essais jouables.

Périmètre : statistiques, matériel utile à leur lecture, progression actuelle, dix disciplines, 94 techniques et 10 variantes, états et retours liés à ces règles. La fusion validée de Retraite méthodique dans Pas de dégagement retire un achat, sans retirer ses possibilités. Pas de nouveaux textes de quêtes, scénario, classes, factions ou capacités extérieures. Aucun nouveau nom de technique. Les termes Points de vie (PV), Durabilité et Résilience gardent leurs sens distincts ; Contrôle distant ne devient pas une nouvelle secondaire.

## 2. Convention d'affichage et limites d'information

- Les identifiants de ce document sont des clés de rédaction et de future localisation, jamais du texte à montrer tel quel au joueur. Ce Markdown n'est pas un format de contenu chargé par le moteur.
- Dans les tableaux, seules les colonnes « Libellé », « Nom », « Texte joueur » et « Limite à afficher » sont destinées au joueur. Les conditions d'affichage et les notes sont des consignes d'intégration.
- Chaque fiche associe une explication courte à sa limite. Les conditions déterminantes doivent rester visibles avant achat ou utilisation : les cacher dans une page secondaire rendrait la description trompeuse.
- Les données de fiche accompagnent le texte : type d'action, rang minimal, prérequis nommés, matériel requis, cible/canal, portée, trajectoire/zone, temps de préparation/exécution/récupération, coût initial et entretien, énergie, chaleur, bande passante occupée et durée de réservation, munitions/objets, durée d'effet, délai de réutilisation et risque pour les tiers. Afficher les champs applicables, pas une rangée de zéros.
- Ces nombres proviennent du profil actif et de l'équipement réel ; ne pas recopier les coefficients de laboratoire dans les phrases. Indiquer séparément ce qui est payé maintenant, lors de l'effet et pendant son maintien. Une réaction affiche aussi le coût qu'elle exigera au déclenchement.
- Montrer les améliorations avec le nom de leur technique mère, et une variante avec ce qu'elle remplace et son compromis. Une seule variante électronique s'applique à une utilisation ; elle n'est pas offerte avec la technique mère.
- Pour une propriété ou une défense ennemie inconnue, afficher « Non déterminé », pas zéro. N'afficher un pourcentage de réussite précis que si les données nécessaires sont connues ; sinon « Chances non déterminées ». Aucun calcul d'aperçu ne doit révéler indirectement le Blindage, la Défense numérique, un occupant caché ou un obstacle non observé.
- Un résultat distingue perception actuelle, dernier relevé daté et indice incertain. Une animation sonar, une couleur, un message, le journal ou un rapport de drone ne peuvent contourner cette limite. Un indice non localisé n'est pas dessiné à des coordonnées exactes inventées.
- Clavier et souris donnent les mêmes textes et informations. Afficher les actions remappées, sans figer une touche QWERTY ou AZERTY. Les mots, symboles et motifs accompagnent les couleurs ; aucune relation ou alerte ne repose sur la couleur seule.
- Un menu, un survol ou une annulation avant engagement ne fait pas avancer le temps. Une tentative légale peut échouer et coûter des ressources. Un refus gratuit ne décrit que ce que le personnage pouvait déjà savoir, jamais la cause cachée découverte par une sonde gratuite.
- Les accolades désignent des valeurs à fournir à l'intégration, non des exemples à afficher littéralement. Les quantités reçues/perdues sont celles du résultat réel, pas les maxima promis. Les noms et causes doivent être connus de l'observateur ; sinon utiliser la formulation générique correspondante.

## 3. Infobulles complémentaires

La première ligne complète les onze secondaires. Les autres décrivent des ressources, propriétés matérielles ou notions d'action : ce ne sont pas de nouvelles statistiques primaires.

| Identifiant | Libellé | Texte joueur |
|---|---|---|
| UI-DEFENSE-NUMERIQUE | Défense numérique | Réduit les chances qu’une intrusion ou un programme hostile franchisse vos protections logicielles. Ne protège pas contre les décharges électriques et ne supprime pas un programme déjà implanté. |
| UI-DURABILITE | Durabilité | Indique l’état d’un équipement, d’un composant ou d’un objet destructible. Les dégâts peuvent dégrader son fonctionnement ou le détruire. Restaurer sa durabilité ne restaure pas aussi les PV de son porteur. |
| UI-RESISTANCE-THERMIQUE | Résistance thermique | Réduit les dégâts thermiques reçus. Ne réduit pas votre chaleur actuelle et n’améliore pas votre refroidissement. Une valeur négative augmente les dégâts de ce type. |
| UI-RESISTANCE-ELECTRIQUE | Résistance électrique | Réduit les dégâts des décharges électriques. Ne bloque pas les intrusions ni les programmes hostiles. Une valeur négative augmente les dégâts de ce type. |
| UI-RESISTANCE-CHIMIQUE | Résistance chimique | Réduit les dégâts chimiques reçus. Ne neutralise pas automatiquement les autres effets d’une substance. Une valeur négative augmente les dégâts de ce type. |
| UI-ENERGIE | Énergie | Indique votre réserve actuelle et sa capacité maximale. Les équipements alimentés dépensent cette réserve selon leur usage. Attendre ou augmenter sa capacité ne la recharge pas automatiquement. |
| UI-ENERGIE-VIDE | Réserve d’énergie vide | Les fonctions demandant de l’énergie sont indisponibles. Les actions de service encore utilisables restent accessibles ; une batterie vide ne provoque pas à elle seule votre mort. |
| UI-CHALEUR | Chaleur | Indique la chaleur accumulée par vos systèmes. Dépasser le seuil critique peut provoquer des dégâts thermiques avant le refroidissement. La chaleur peut dépasser ce seuil : il ne représente pas une capacité maximale. |
| UI-SEUIL-ALERTE | Seuil d’alerte thermique | Signale une accumulation préoccupante de chaleur. Atteindre ce seuil ne provoque pas à lui seul de dégâts ni de pénalité cachée. |
| UI-SEUIL-CRITIQUE | Seuil critique | Au-delà de ce seuil, l’excès de chaleur provoque des dégâts thermiques. Une action chauffante dépassant la limite de son mode est refusée. Les actions qui ne chauffent pas davantage restent possibles si leurs autres conditions sont remplies. |
| UI-DISSIPATION | Dissipation | Indique la chaleur évacuée à chaque cycle de refroidissement. Certains effets peuvent la réduire. Refroidir ne restaure pas les PV déjà perdus. |
| UI-BANDE-PASSANTE | Bande passante | Indique votre capacité totale et la part occupée par les liaisons et processus actifs. Cette capacité est réservée, puis libérée à leur fin ; elle ne se dépense pas comme une batterie. Apprendre une technique ne l’occupe pas à lui seul. |
| UI-EMPLACEMENTS | Emplacements disponibles | Déterminent les modules ou programmes que votre matériel peut accueillir. Un emplacement libre doit aussi être compatible. Apprendre une technique ne crée pas de nouvel emplacement. |
| UI-PORTEE-ARME | Portée de l’arme | Fixe la distance maximale d’utilisation du mode choisi. La trajectoire, les obstacles et les conditions de ciblage restent à respecter. Une meilleure Précision ne prolonge pas cette portée. |
| UI-DEGATS | Dégâts | Décrivent les dégâts de l’attaque avant les protections de la cible. Chaque type rencontre la défense qui lui correspond. Une attaque peut toucher sans infliger de dégâts. |
| UI-PENETRATION | Pénétration physique | Ignore une partie du Blindage pour l’impact concerné. Ne détruit pas ce blindage et ne réduit pas les résistances thermiques, électriques ou chimiques. |
| UI-MUNITIONS | Munitions | Indiquent les projectiles disponibles pour l’arme. Une rafale dépense chaque projectile tiré. Une attaque manquée ne rend pas les munitions déjà utilisées. |
| UI-MODES-ARME | Modes de l’arme | Décrivent les façons de tirer ou de frapper prévues par votre équipement. Leurs coûts et leurs effets diffèrent. Une technique exigeant un mode compatible ne crée pas ce mode sur une arme qui en est dépourvue. |
| UI-PORTEE-CAPTEURS | Portée des capteurs | Fixe la distance à laquelle vos capteurs peuvent recueillir des informations. Les murs, les portes fermées et les limites de chaque canal restent applicables. Agrandir la fenêtre n’étend pas votre perception. |
| UI-CANAUX | Canaux de détection | Déterminent les types de signaux que vos capteurs peuvent percevoir. Un camouflage ou un brouillage n’affecte que les canaux concernés ; les autres peuvent rester utiles. |
| UI-SIGNATURE | Signature | Décrit les indices que votre corps et vos équipements émettent sur un canal donné. Réduire le bruit ne masque pas votre silhouette ou votre chaleur. Les observations déjà faites ne sont pas effacées. |
| UI-COUT-DEPLACEMENT | Temps de déplacement | Indique le temps nécessaire pour parcourir une case dans votre état actuel. Terrain, charge et posture peuvent ralentir le mouvement. Les autres acteurs peuvent agir pendant ce temps. |
| UI-ANCRAGE | Ancrage | Aide à résister aux déplacements imposés, avec votre masse et les limites du matériel. Ne réduit pas les dégâts d’un coup et ne remplace pas le Blindage. |
| UI-TRACTION | Capacité de traction | Limite la masse que votre matériel peut déplacer lors d’une extraction. Le corps et la charge de l’allié comptent ensemble. Un trajet praticable et des places d’arrivée libres restent nécessaires. |
| UI-DRONES | Unités contrôlables | Indique le nombre d’unités que votre contrôleur peut prendre en charge. La bande passante, les liaisons et les ressources restent nécessaires. Les ordres ne donnent pas d’actions supplémentaires aux drones. |
| UI-LIAISON | Liaison | Permet de transmettre une commande à un dispositif joignable. Portée, obstacles et brouillage peuvent la couper. Une liaison de commande ne donne pas automatiquement la vision de la cible ou de ses alentours. |
| UI-PREPARATION | Préparation | Demande du temps avant l’exécution de la technique. Les adversaires peuvent agir pendant ce délai et certaines perturbations peuvent l’interrompre. Les dépenses déjà engagées ne sont pas remboursées. |
| UI-RECUPERATION | Récupération après attaque | Empêche d’attaquer ou de préparer une nouvelle attaque pendant votre prochaine action normale. Vous pouvez encore vous déplacer, attendre ou effectuer une action de soutien autorisée. |
| UI-REUTILISATION | Délai de réutilisation | Indique quand cette technique pourra être utilisée de nouveau. Ses variantes partagent ce délai. Attendre dans un menu ne le réduit pas. |
| UI-REACTION | Réaction | Permet de déclencher une réponse préparée lorsque ses conditions sont remplies. Vous disposez d’au plus une réaction entre deux actions normales. Une réaction ne déclenche pas une autre réaction. |
| UI-GARDE | Garde préparée | Réserve une réponse jusqu’à son déclenchement ou au début de votre prochaine action normale. Vous ne pouvez maintenir qu’une garde à la fois. Les ressources nécessaires seront vérifiées au déclenchement. |
| UI-TEMPS | Unité de temps | Mesure le temps écoulé dans le monde, pas la durée d’une animation. Une action longue laisse des occasions d’agir aux autres acteurs. Consulter un menu ne fait pas avancer le temps. |
| UI-RETARDATEUR | Délai avant déclenchement | Le décompte commence après le cycle d’armement. Une unité de délai laisse une prochaine étape ordinaire pour réagir. Une action plus lente peut ne pas tenir dans cette fenêtre, et une réponse peut échouer. |

Les dégâts Radiation/Corruption, immunités non définies et détails d'usure des armures ne reçoivent pas de texte mécanique inventé. Pour un équipement, les seuils de panne/destruction affichés seront uniquement ceux que son profil définit.

## 4. Création, apprentissage et progression

Les bornes et budgets entre accolades proviennent de la configuration active. Le catalogue détaillé des classes et les modalités futures de réattribution restent hors de cette livraison.

| Identifiant | Libellé | Texte joueur |
|---|---|---|
| UI-CREATION | Répartir les statistiques | Répartissez {budget} points entre vos cinq statistiques primaires. À la création, chaque valeur doit être comprise entre {minimum} et {maximum}. Le plafond absolu est de {plafond}. |
| UI-CLASSE | Classe de départ | Choisissez parmi les classes débloquées. Chaque classe propose un équipement et des apprentissages de départ ; elle ne remplace pas vos choix de progression pendant la partie. |
| UI-CORPS | Corps principal | Vous conservez le même corps principal pendant la partie. Votre équipement et vos améliorations peuvent évoluer sans effacer les techniques apprises. |
| UI-EXPERIENCE | Expérience | Fait progresser votre personnage au cours de cette partie. Les objectifs, découvertes et obstacles résolus peuvent rapporter de l’expérience. Répéter une même résolution ne verse pas de nouvelle récompense. |
| UI-NIVEAU | Niveau | Représente votre progression dans cette partie. Les récompenses du prochain niveau sont indiquées séparément. Gagner un niveau ne restaure pas automatiquement vos PV ou votre énergie. |
| UI-POINTS-COMPETENCE | Points de compétence | Servent à acheter des rangs dans vos compétences. Chaque rang acheté comprend un choix de technique ou d’amélioration accessible. Le coût du rang suivant est indiqué avant confirmation. |
| UI-POINTS-PRIMAIRE | Points de statistique | Permettent d’augmenter vos statistiques primaires dans les limites indiquées. Augmenter une capacité maximale ne remplit pas la réserve correspondante. |
| UI-RANG | Rang de compétence | Détermine les techniques auxquelles vos prochains choix donnent accès. Les techniques des rangs précédents restent disponibles si vous ne les avez pas apprises. Monter de rang ne donne pas toutes ses techniques. |
| UI-CHOIX | Choix de technique | Chaque compétence permet au plus cinq choix, en comptant ceux de votre classe de départ. Une amélioration occupe elle aussi un choix et demande sa technique d’origine. |
| UI-MAITRISE | Maîtrise | Le dernier rang complète vos cinq choix dans cette compétence. En Reconnaissance, il permet de choisir une technique antérieure encore non apprise, sans pouvoir exclusif supplémentaire. |
| UI-AMELIORATION | Amélioration | Modifie une technique déjà apprise. Elle ne s’utilise pas seule. Consultez ce qu’elle ajoute ou remplace et les éventuels coûts supplémentaires. |
| UI-VARIANTE | Variante électronique | Transforme une technique connue et occupe un choix supplémentaire. Vous choisissez une seule variante par utilisation ; leurs effets ne s’additionnent pas. |
| UI-MATERIEL-APPRENTISSAGE | Technique et équipement | Vous pouvez apprendre une technique avant de posséder son matériel requis. Son utilisation attendra un équipement compatible. Retirer cet équipement n’efface pas l’apprentissage. |
| UI-ACTIONS-ORDINAIRES | Fonctions ordinaires | Les usages de base de votre matériel restent accessibles sans spécialisation : attaquer, employer un consommable compatible ou donner un ordre simple à un drone équipé pour cela. Les techniques ajoutent des options. |
| UI-CONFIRMATION-ACHAT | Confirmer l’apprentissage | Apprendre « {technique} » et passer au rang {rang} de {discipline} pour {cout} points ? Il vous restera {restants} points. |
| UI-REATTRIBUTION | Réattribution | La réattribution des choix n’est pas disponible dans ce mode de jeu. Vérifiez vos prérequis et le matériel nécessaire avant de confirmer un apprentissage. |
| UI-FIN-PARTIE | Progression de la partie | À la mort de votre personnage, sa progression de partie est perdue. Les déblocages permanents des classes sont conservés séparément ; ils n’accordent pas automatiquement tous les apprentissages d’une ancienne partie. |
| UI-DISCIPLINE-DIFFEREE | Compétence indisponible dans cette version | Les possibilités nécessaires à cette progression ne sont pas encore toutes disponibles dans cette version. Aucun rang ne peut y être acheté pour le moment. Les fonctions ordinaires de votre équipement restent accessibles selon leurs règles. |

UI-REATTRIBUTION est un texte conditionnel, affichable seulement si le profil joué désactive réellement la réattribution. Sa rédaction ne décide pas de supprimer cette possibilité pour le jeu final. De même, ne pas afficher un nombre de niveaux, un budget de classe ou des récompenses encore absents du contenu chargé. Une technique dépendant d'un système absent de la version reste masquée à l'achat, et non vendue sous prétexte que son texte est prêt.

Une discipline dont une suite légale ne peut pas atteindre cinq choix reste différée dans cette version. Son entrée peut rester visible comme indisponible avec UI-DISCIPLINE-DIFFEREE, mais ne propose pas d'achat ; ce statut ne dépend pas du matériel que le joueur transporte. Les classes et sauvegardes incompatibles demandent une décision de contenu ou une migration explicite, jamais une perte silencieuse de choix. Aucun plafond provisoire inférieur à cinq n'est décidé ici.

## 5. Présentation des dix disciplines

| Identifiant | Libellé | Texte joueur |
|---|---|---|
| UI-DISC-MEL | Combat rapproché | Renforcez vos options au contact : frappes préparées, parades et déplacements imposés. Votre arme, la masse adverse et l’espace disponible déterminent les manœuvres possibles. |
| UI-DISC-TIR | Tir | Préparez vos tirs, maîtrisez vos rafales et surveillez un passage. Vos techniques exploitent les modes de votre arme sans supprimer les contraintes de munitions, de portée et de visibilité. |
| UI-DISC-DEM | Démolition | Préparez des brèches, posez des mines et organisez vos mises à feu. Explosifs, matériaux et délais comptent ; vos alliés peuvent subir les conséquences d’une explosion. |
| UI-DISC-MAN | Manœuvre | Choisissez votre position sous pression : dégagement, franchissement, charge et extraction d’un allié. Votre corps doit pouvoir réaliser le mouvement et le trajet reste exposé aux dangers. |
| UI-DISC-FUR | Furtivité | Réduisez vos signatures, exploitez les couverts et préparez vos approches. Rompre le contact ne supprime pas la mémoire d’un témoin, et aucun camouflage ne masque tous les canaux à la fois. |
| UI-DISC-REC | Reconnaissance | Examinez les cibles, les traces et le terrain pour obtenir des informations utiles à vos décisions. L’analyse exploite ce que vos capteurs et vos données permettent réellement de connaître. |
| UI-DISC-ING | Ingénierie | Réparez des composants précis, préservez des pièces et adaptez vos équipements. Outils, matériaux et temps restent nécessaires ; les réparations ordinaires ne demandent pas cette spécialisation. |
| UI-DISC-INT | Intrusion | Obtenez des accès locaux, extrayez des données et détournez certaines fonctions. Chaque droit a son périmètre ; compromettre une console ne donne pas le contrôle de tout un étage. |
| UI-DISC-GEL | Guerre électronique | Employez décharges, sabotages logiciels et contre-mesures. Les émissions demandent du matériel adapté ; les programmes hostiles exigent une cible compatible et doivent franchir ses défenses numériques. |
| UI-DISC-DRN | Contrôle de drones | Programmez et coordonnez vos machines alliées. Chaque drone garde ses propres actions, capteurs et ressources ; vos commandes ne lui offrent ni déplacement instantané ni connaissance des dangers cachés. |

## 6. Textes des techniques et améliorations

Les 104 lignes suivantes reprennent exactement les identifiants et noms actifs du catalogue. Le rang minimal et les prérequis sont affichés depuis les fiches de référence, avec les valeurs détaillées prévues en section 2. Les termes « davantage », « brièvement » ou « temporairement » ne dispensent pas d'afficher le chiffre réel du profil joué.

### 6.1. Combat rapproché

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| MEL-01 | Frappe puissante | Porte une frappe de mêlée plus dévastatrice. | Demande une récupération après le coup, même si l’attaque manque sa cible. |
| MEL-02 | Frappe précise | Prépare une frappe pour améliorer vos chances de toucher. | Bouger, changer de cible ou perdre le contact annule la préparation. |
| MEL-03 | Repoussement | Porte un coup moins puissant et tente de repousser la cible d’une case. | Sa masse, son ancrage ou une destination bloquée peuvent empêcher le déplacement. Aucun dégât de collision supplémentaire. |
| MEL-04 | Parade | Prépare une garde réduisant les dégâts physiques de la prochaine attaque de mêlée qui vous touche. | Exige un équipement capable de parer et consomme votre réaction. Ne protège pas contre les tirs ou les explosions. |
| MEL-05 | Balayage | Frappe plusieurs cases voisines dans un arc, avec moins de dégâts par cible. | Exige une arme et un espace adaptés. Les alliés dans l’arc peuvent être touchés ; une récupération suit le balayage. |
| MEL-06 | Riposte | Ajoute une contre-attaque ordinaire après une parade réussie. | Demande Parade. L’attaquant doit rester à portée et les ressources de la frappe doivent être disponibles. N’accorde pas de réaction supplémentaire. |
| MEL-07 | Brise-armure | Sacrifie une partie des dégâts du coup pour fragiliser temporairement le Blindage de la cible touchée. | La fragilisation profite aux impacts suivants. Elle ne s’accumule pas et n’est pas prolongée tant qu’elle est active. |
| MEL-08 | Entrave | Frappe une fonction locomotrice identifiée pour tenter de ralentir les déplacements de la cible. | La cible peut résister. L’effet est bref et ne l’immobilise pas entièrement ; une courte protection suit sa fin. |
| MEL-09 | Écrasement | Porte une frappe particulièrement lourde contre une cible déjà entravée ou immobilisée. | L’état requis doit être présent. Le coup ne l’applique pas lui-même et demande une récupération. |
| MEL-10 | Interception | Prépare une frappe contre un adversaire qui quitte volontairement le contact. | Consomme votre réaction sans arrêter automatiquement le retrait. Un déplacement forcé ne la déclenche pas. |

### 6.2. Tir

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| TIR-01 | Tir visé | Prépare votre prochain tir simple pour améliorer sa précision contre une cible visible. | Bouger, changer de cible ou perdre la vue annule la préparation. |
| TIR-02 | Rafale contrôlée | Tire une rafale plus courte avec une dispersion réduite. | Exige un mode automatique compatible. Chaque projectile est dépensé ; la rafale ne gagne pas de dégâts par projectile. |
| TIR-03 | Tir de suppression | Tire une rafale pouvant gêner brièvement la précision d’une cible touchée et exposée. | La cible peut résister à la suppression. Ne l’oblige pas à fuir et dépense toute la rafale. |
| TIR-04 | Surveillance | Prépare un tir contre le premier ennemi perçu qui entre dans le passage désigné. | Consomme votre réaction et les ressources du tir au déclenchement. N’étend ni votre vision ni la portée de l’arme. |
| TIR-05 | Tir localisé | Vise un composant identifié pour tenter d’endommager sa fonction. | Le tir est moins précis. Les dégâts atteignent le composant, sans être appliqués une seconde fois au corps ; la destruction n’est pas garantie. |
| TIR-06 | Rafale répartie | Répartit les projectiles d’une rafale entre plusieurs cibles proches et perçues. | Ne crée aucun projectile supplémentaire. Chaque tir conserve ses contraintes de trajectoire et ses chances de toucher. |
| TIR-07 | Visée persistante | Conserve une partie du bonus de Tir visé pour vos tirs simples suivants contre la même cible. | Demande Tir visé. Bouger, perdre la vue, changer de cible ou entreprendre une autre action que tirer simplement ou attendre met fin au bonus. |
| TIR-08 | Surveillance étendue | Étend Surveillance à un secteur plus large. | Demande Surveillance. Le nombre de tirs de réaction, la portée et le champ de vision restent inchangés. |
| TIR-09 | Tir de rupture | Prépare un tir exploitant une faiblesse connue pour ignorer une partie du Blindage. | Exige une arme adaptée et une faiblesse réellement identifiée. Ne garantit pas la touche et n’ignore pas toutes les protections. |
| TIR-10 | Barrage | Maintient un feu réparti sur une petite zone, avec une possibilité de suppression des occupants touchés. | Exige de rester en place et de payer chaque étape de tir. Les couverts protègent encore et les alliés exposés risquent d’être touchés. |

### 6.3. Démolition

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| DEM-01 | Lancer ajusté | Prépare le lancer d’un explosif pour réduire sa dispersion. | Ne renforce pas l’explosion. La trajectoire doit rester valide et les occupants peuvent quitter la zone pendant la préparation. |
| DEM-02 | Désamorçage | Intervient sur le déclencheur d’un piège identifié pour le neutraliser. | Demande proximité, outils et temps. Un dispositif piégé contre la manipulation peut se déclencher en cas d’échec. |
| DEM-03 | Charge de brèche | Pose une charge destinée à endommager fortement un obstacle destructible. | Consomme une charge et laisse un délai avant explosion. La résistance réelle de l’obstacle peut empêcher l’ouverture ; les alentours sont exposés. |
| DEM-04 | Mine de proximité | Pose une mine qui s’arme après un délai puis réagit aux présences détectables par son capteur. | Sans filtre matériel adapté, un allié peut la déclencher. Elle peut être repérée, désamorcée ou détruite. |
| DEM-05 | Explosion dirigée | Configure un explosif adapté pour concentrer son effet dans une direction. | Réduit les directions couvertes, sans rendre l’explosion inoffensive pour les alliés présents dans la zone. |
| DEM-06 | Déclenchement distant | Commande la mise à feu d’un dispositif connu doté d’un récepteur compatible. | Exige une liaison valide au moment de la commande. Ne révèle pas ce qui se trouve autour du dispositif. |
| DEM-07 | Récupération de charges | Permet de récupérer la charge intacte d’un piège déjà neutralisé. | Demande Désamorçage et une intervention supplémentaire. La charge doit être récupérable ; elle remplace les pièces issues de ce même piège. |
| DEM-08 | Mise à feu séquencée | Programme plusieurs charges posées pour exploser selon des délais choisis à l’avance. | Chaque charge doit exister et être joignable pendant la programmation. Les délais ne s’adaptent pas aux mouvements ennemis. |
| DEM-09 | Effondrement contrôlé | Prépare la destruction d’un support identifié pour provoquer une chute locale de structure. | Exige un support réellement fragilisable et des charges adaptées. La zone annoncée reste dangereuse pour tous ses occupants. |
| DEM-10 | Détonation combinée | Associe deux charges : la première attaque l’obstacle, la seconde explose après un délai supplémentaire. | Consomme les deux charges. La seconde explosion ne profite que de la brèche réellement créée par la première. |

### 6.4. Manœuvre

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| MAN-01 | Pas de dégagement | Effectue un pas prudent améliorant votre Esquive contre une interception de mêlée. La consigne peut être maintenue pour les pas de retraite suivants. | Chaque pas prend son temps normal, sans cumul du bonus. Attaquer ou changer de posture rompt la consigne ; tirs, mines et dangers d’arrivée restent actifs. |
| MAN-02 | Appui stable | Renforce votre ancrage pour mieux résister aux poussées tant que vous restez en place. | Un déplacement volontaire ou forcé met fin à la posture. Ne réduit pas les dégâts reçus. |
| MAN-03 | Franchissement | Prépare le passage d’un obstacle bas ou d’un petit intervalle compatible avec votre corps. | Le passage et l’arrivée doivent être praticables. N’autorise pas à traverser un mur ou un intervalle trop large. |
| MAN-04 | Charge | Avance en ligne droite puis porte une frappe de mêlée renforcée par l’élan. | Chaque étape prend du temps et reste exposée aux dangers. Si la cible quitte le contact prévu, la charge ne la poursuit pas automatiquement. |
| MAN-05 | Esquive préparée | Prépare un déplacement de secours vers une case choisie face à une attaque perçue. | Consomme votre réaction. La case doit rester accessible ; une explosion peut aussi couvrir votre position d’arrivée. |
| MAN-06 | Poussée des propulseurs | Utilise vos propulseurs pour parcourir rapidement plusieurs cases successives. | Exige une propulsion compatible et dépense énergie et chaleur. Chaque case traversée conserve ses obstacles et dangers. |
| MAN-08 | Inertie maîtrisée | Permet de freiner volontairement une Charge et supprime sa récupération lorsqu’elle est menée à son terme. | Demande Charge et des moyens de freinage adaptés. S’arrêter prend une action ; les dépenses passées et collisions imprévues ne sont pas annulées. |
| MAN-09 | Percée | Tente de repousser un adversaire adjacent pour occuper sa place. | Sa masse, son ancrage et la case de recul doivent permettre la poussée. N’inflige pas de dégâts supplémentaires et ne traverse pas une file d’unités. |
| MAN-10 | Extraction | Aide un allié adjacent à se déplacer avec vous hors d’une position dangereuse. | Exige sa coopération ou la possibilité de le transporter, une traction suffisante et deux places d’arrivée légales. Les deux corps restent exposés aux dangers. |

### 6.5. Furtivité

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| FUR-01 | Pas feutrés | Adopte une marche plus lente pour réduire le bruit de vos déplacements. | Ne masque pas votre silhouette. Les équipements qui émettent leur propre bruit restent audibles. |
| FUR-02 | Approche couverte | Rend vos déplacements moins faciles à repérer visuellement lorsqu’un couvert vous masque partiellement. | Ne fonctionne pas à découvert et ne réduit pas les signatures des autres canaux. |
| FUR-03 | Silence des émissions | Met en veille les fonctions émettrices sélectionnées pour supprimer leurs émissions actives. | Ces fonctions restent indisponibles jusqu’à leur réactivation. Votre silhouette, votre chaleur et les souvenirs des témoins ne disparaissent pas. |
| FUR-04 | Profil réduit | Exploite la forme de votre corps pour mieux vous dissimuler derrière un couvert bas. | Ralentit les déplacements et peut interdire les armes volumineuses. Ne se cumule pas avec la posture Pas feutrés et n’aide pas à découvert. |
| FUR-05 | Embuscade | Prépare une première attaque simple plus précise et physiquement plus puissante contre une cible qui ne vous a pas localisé. | Si la cible vous localise avant l’exécution, le bonus disparaît. L’attaque conserve ses émissions normales. |
| FUR-06 | Leurre sonore | Place ou lance une source de bruit temporaire pour attirer l’attention à l’endroit choisi. | Consomme un leurre réel. Les adversaires peuvent l’ignorer, notamment s’ils sont déjà au contact d’une menace. |
| FUR-07 | Rupture de piste | Après avoir rompu la vue adverse, évite de laisser de nouvelles traces pendant une courte séquence de fuite. | Ne supprime ni vos anciennes traces ni votre dernière position connue. Une reprise de contact interrompt l’effet. |
| FUR-08 | Dissimulation des dispositifs | Camoufle visuellement un petit dispositif déjà posé. | Consomme du matériel et du temps. Ses émissions restent détectables ; déplacement ou choc dommageable peuvent rompre le camouflage. |
| FUR-09 | Camouflage actif | Active un module réduisant fortement votre signature sur le canal qu’il peut masquer. | Demande énergie et refroidissement. Une attaque ou une activation offensive énergivore y met fin ; les autres canaux peuvent toujours vous repérer. |
| FUR-10 | Neutralisation discrète | Renforce l’attaque de contact d’Embuscade contre une cible vulnérable et réduit le bruit de cette frappe. | Demande Embuscade. N’élimine pas automatiquement la cible ; un survivant ou un témoin peut donner l’alerte. |

### 6.6. Reconnaissance

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| REC-01 | Analyse de cible | Examine une cible perçue pour préciser son état, son matériel observable et les résistances ou faiblesses identifiables. | Les capteurs et les données disponibles limitent le résultat. N’accorde aucun bonus d’attaque automatique et ne révèle pas un inventaire fermé. |
| REC-02 | Lecture de traces | Examine les traces locales pour en déduire une direction et une ancienneté approximative. | Les traces doivent réellement être présentes et accessibles. Elles ne donnent pas la position actuelle de leur auteur. |
| REC-03 | Inspection minutieuse | Recherche les indices de pièges, caches, commandes dissimulées ou passages secrets dans les cases observables. | Découvrir ne signifie ni ouvrir ni désamorcer. Répéter l’inspection sans changement de conditions n’améliore pas le résultat. |
| REC-04 | Repérage des parois fragiles | Examine les parois observables pour identifier un matériau peu résistant ou une dégradation exploitable. | Ne crée pas de faiblesse et ne révèle pas ce qui se trouve derrière le mur. Un effondrement demande un support réellement adapté. |
| REC-05 | Profil de menace | Précise les possibilités offensives et défensives d’un adversaire à partir de ce qui est observable ou documenté. | Ne prédit pas sa prochaine décision et ne révèle pas ses moyens cachés sans source d’information. |
| REC-09 | Analyse multiple | Applique Analyse de cible à plusieurs cibles perçues au cours d’une même action. | Demande Analyse de cible et davantage d’énergie. Chaque cible conserve ses propres informations accessibles ; aucun ennemi caché n’est révélé. |
| REC-08 | Diagnostic énergétique | Examine la chaleur, l’alimentation et le stockage d’énergie accessibles d’une machine pour guider vos choix. | Exige un canal de diagnostic adapté. Ne cartographie pas les installations cachées et n’arme pas lui-même une Implosion. |

Les anciens identifiants REC-06, REC-07 et REC-10 restent seulement dans l'historique du catalogue ; aucun texte d'achat ne les réintroduit.

### 6.7. Ingénierie

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| ING-01 | Réparation ciblée | Restaure la durabilité d’un composant endommagé choisi. | Exige des pièces compatibles, des outils et du temps. Ne restaure pas aussi les PV du corps et ne remplace pas les réparations ordinaires. |
| ING-02 | Démontage soigneux | Prend davantage de temps pour préserver un composant survivant choisi dans une carcasse. | Le composant conserve son état réel et remplace les pièces qu’il aurait fournies. Un composant détruit ne redevient pas intact. |
| ING-03 | Diagnostic de panne | Examine une panne matérielle pour en préciser la cause et la procédure de réparation accessible. | Le diagnostic peut conclure qu’une pièce manque. Ne répare pas lui-même la panne et ne purge pas les programmes hostiles. |
| ING-04 | Réglage spécialisé | Adapte un module compatible pour privilégier l’économie d’énergie ou sa puissance de sortie. | Chaque réglage possède une contrepartie, demande du matériel et remplace celui du même emplacement. |
| ING-05 | Réparation d'urgence | Effectue une Réparation ciblée plus rapide, avec moins de durabilité restaurée pour les pièces dépensées. | Demande Réparation ciblée. Le composant doit rester réparable ; une pièce détruite n’est pas recréée. |
| ING-06 | Surcadencement | Renforce temporairement une fonction chiffrée d’un module compatible. | Augmente sa consommation et sa chaleur ; un usage trop chaud peut le dégrader. N’accélère pas toutes vos actions et n’augmente pas toutes ses propriétés. |
| ING-07 | Dérivation | Rétablit partiellement une fonction électrique endommagée en suspendant un autre sous-système compatible. | Exige un diagnostic crédible et un circuit matériel utilisable. Ne remplace pas une pièce absente et conserve le sacrifice de la fonction donneuse. |
| ING-08 | Reconditionnement | Restaure la durabilité d’un équipement récupérable dans les conditions d’un atelier. | Demande davantage de temps et des pièces. Ne dépasse pas l’état maximal réparable de l’objet. |
| ING-09 | Assemblage de terrain | Assemble un dispositif provisoire à partir d’un plan connu et de pièces disponibles. | Exige outils, composants et alimentation réels. Le dispositif garde ses propres limites et ne produit pas de ressources infinies. |
| ING-10 | Surcadencement régulé | Offre un régime de Surcadencement moins intense, consommant et chauffant moins, avec une meilleure tolérance à la dégradation. | Demande Surcadencement. La chaleur, les coûts et les risques au-delà des limites restent présents. |

### 6.8. Intrusion

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| INT-01 | Sondage d'accès | Examine une interface accessible pour préciser ses protections, les droits observables et les interventions envisageables. | Ne donne pas d’accès à lui seul. La sonde peut laisser une trace détectable par le système. |
| INT-02 | Ouverture forcée | Tente de commander l’ouverture d’un verrou électronique local sans autorisation normale. | Exige une interface compatible et peut échouer face à sa sécurité. Ne crée pas d’accès général ; une tentative engagée coûte du temps et peut laisser une trace. |
| INT-03 | Extraction de données | Récupère un lot de données identifié depuis une source sur laquelle vous disposez du droit de lecture. | La liaison doit tenir jusqu’à la fin. Les données gardent leur date et leur provenance : un journal ancien n’est pas un radar. |
| INT-04 | Usurpation locale | Présente un identifiant obtenu pour exercer temporairement ses droits sur un système déterminé. | Le justificatif doit être valide. N’invente pas de droits et ne change ni votre réputation ni la mémoire des témoins. |
| INT-05 | Détournement | Maintient temporairement une consigne simple sur un dispositif dont vous pouvez commander la fonction. | Exige un accès, une liaison et un canal disponible. Le dispositif agit à son propre rythme ; lui donner un ordre ne le fait pas tirer gratuitement. |
| INT-06 | Neutralisation de routine | Suspend brièvement une fonction automatique précise sur un système accessible. | Ne désactive pas tout l’acteur. La fonction reprend à la fin de l’effet ou à la rupture du maintien ; répéter la procédure ne prolonge pas indéfiniment sa suspension. |
| INT-07 | Porte dérobée | Installe un moyen de reprendre plus tard un accès local déjà obtenu. | Le nombre d’accès conservés est limité. Réinitialisation, inspection ou changement de couche peuvent l’invalider ; une reconnexion demande encore une liaison et du temps. |
| INT-08 | Falsification de registre | Modifie un événement suspect identifié dans un registre accessible avant que la sécurité ne l’exploite. | Ne supprime pas les copies déjà transmises, les alarmes reçues ni les souvenirs des témoins. |
| INT-09 | Détournement de sous-réseau | Coordonne temporairement plusieurs dispositifs d’un même sous-réseau connu. | Chacun doit être joignable et couvert par vos droits, dans vos limites de contrôle. Ne donne ni accès à tout l’étage ni vision supplémentaire. |
| INT-10 | Verrouillage de contrôle | Protège temporairement un dispositif détourné contre les reprises de commande ordinaires de ses contrôleurs adverses. | Nécessite le maintien de l’accès et d’un canal. Une coupure, une réinitialisation ou une intervention physique peuvent toujours y mettre fin. |

Falsification de registre et Verrouillage de contrôle sont affichables à l'achat uniquement dans une version où la sécurité consulte réellement les événements et tente réellement de reprendre les commandes. Il ne s'agit pas de mécanismes multijoueurs.

### 6.9. Guerre électronique

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| GEL-01 | Surcharge | Émet une impulsion infligeant des dégâts électriques autour de vous. Un émetteur adapté peut aussi perturber une préparation. | Exige un émetteur et une forte dépense. Les murs et portes fermées bloquent l’effet ; les alliés exposés peuvent être touchés. |
| GEL-02 | Surchauffe | Implante un sabotage augmentant la chaleur d’une machine et réduisant temporairement sa dissipation. | Doit franchir sa défense logicielle. Les dégâts dépendent du dépassement thermique ; Purge ou l’arrêt local du module compromis peuvent stopper le sabotage. |
| GEL-03 | Brouillage | Perturbe un canal de capteur ou de liaison dans la zone de votre émetteur. | Choisissez le canal affecté. Les obstacles limitent la zone, les autres canaux restent utilisables et vos alliés compatibles peuvent être gênés. |
| GEL-04 | Purge | Tente de retirer un programme hostile identifié sur vous ou sur un allié accessible. | Peut échouer et coûte une action. Ne répare pas les dégâts déjà subis et ne refroidit pas la cible. |
| GEL-05 | Surcharge en cascade | Frappe une cible puis fait rebondir la décharge électrique vers d’autres cibles proches, avec une perte de puissance. | Chaque cible doit être perçue et chaque saut non obstrué. Une même cible n’est touchée qu’une fois par la chaîne. |
| GEL-06 | Infection | Implante un sabotage infligeant des dégâts thermiques périodiques et pouvant se transmettre à quelques machines proches. | Chaque nouvel hôte peut résister. Les obstacles, la durée et le nombre d’hôtes limitent la propagation. Purge peut retirer le programme ; l’isolement limite la contagion. |
| GEL-07 | Champ de saturation | Pose une balise alimentant une zone temporaire de dégâts électriques. | Consomme la batterie de la balise et expose aussi les alliés. Les obstacles limitent le champ ; détruire la balise arrête ses effets futurs. |
| GEL-08 | Implosion | Sabote un stockage d’énergie identifié pour provoquer, après un délai annoncé, une explosion physique et thermique autour de lui. | Exige une réserve compatible suffisamment chargée et une implantation réussie. Purge ou déconnexion physique du stockage peuvent empêcher l’explosion ; aucun effet d’attraction ni élimination automatique. |

Les descriptions d'Infection et de ses variantes suivent le typage thermique direct proposé en annexe 16 : ce n'est pas une validation finale de ce choix technique. La fiche détaillée distingue ses dégâts périodiques de la hausse de jauge de Surchauffe. Couper la liaison de l'attaquant n'efface pas ces programmes déjà implantés ; leurs contre-mesures locales restent nécessaires. L'aperçu d'Implosion doit annoncer le risque pour alliés, objets et butin observables sans révéler les occupants cachés.

### 6.10. Variantes de Guerre électronique

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| GEL-V01 | Impulsion directionnelle | Oriente Surcharge en cône pour choisir plus précisément la zone exposée. | Demande Surcharge. Réduit les directions couvertes sans augmenter portée ou puissance ; les alliés présents dans le cône restent exposés. |
| GEL-V02 | Filtrage allié | Configure Surcharge pour épargner les alliés reconnus par votre matériel. | Demande Surcharge et un filtrage compatible, avec un coût accru. Les neutres et unités non identifiées ne sont pas protégés. |
| GEL-V03 | Montée accélérée | Fait monter plus rapidement la chaleur provoquée par Surchauffe. | Demande Surchauffe. Dure moins longtemps, dépense davantage d’énergie et augmente la chaleur produite par votre action. |
| GEL-V04 | Inhibition prolongée | Prolonge le sabotage de Surchauffe avec une montée thermique plus lente. | Demande Surchauffe. Laisse davantage de temps à la cible pour répondre et augmente le coût énergétique. |
| GEL-V05 | Rebond supplémentaire | Ajoute un rebond à Surcharge en cascade. | Demande Surcharge en cascade. Coûte davantage et touche plus faiblement la dernière cible, sans allonger chaque saut. |
| GEL-V06 | Décharge soutenue | Préserve davantage de puissance entre les rebonds de Surcharge en cascade. | Demande Surcharge en cascade. Augmente la consommation et la chaleur sans ajouter de cible ni de portée. |
| GEL-V07 | Infection contagieuse | Favorise la transmission d’Infection et augmente son nombre maximal d’hôtes. | Demande Infection. Inflige moins de dégâts à chaque hôte sans prolonger la durée globale de propagation. |
| GEL-V08 | Infection concentrée | Renforce les dégâts périodiques d’Infection sur sa cible initiale. | Demande Infection. Supprime la propagation et augmente la dépense sur cette seule cible. |
| GEL-V09 | Saturation persistante | Prolonge la durée du Champ de saturation avec la même batterie. | Demande Champ de saturation. Réduit les dégâts de chaque exposition ; la balise reste destructible. |
| GEL-V10 | Activation manuelle | Pose la balise du Champ de saturation inactive pour déclencher son champ au moment choisi. | Demande Champ de saturation. L’activation coûte une commande et exige une liaison valide ; la balise reste vulnérable dans l’attente et ne reçoit pas de nouvelle batterie. |

### 6.11. Contrôle de drones

| Code | Nom | Texte joueur | Limite à afficher |
|---|---|---|---|
| DRN-01 | Escorte active | Ordonne à un drone de vous escorter en respectant une distance choisie. | Le drone se déplace pendant ses propres actions et ne réagit qu’aux informations auxquelles il a accès. |
| DRN-02 | Patrouille bornée | Programme un court trajet connu et une condition d’arrêt ou de retour pour un drone. | Les obstacles nouveaux peuvent interrompre la routine. Hors liaison, le drone peut suivre la consigne déjà reçue, mais pas recevoir de nouveaux ordres. |
| DRN-03 | Leurre mobile | Envoie un drone équipé émettre un signal de diversion depuis une position choisie. | Le trajet et l’émission prennent du temps et des ressources. L’ennemi peut ignorer le leurre ou attaquer le drone. |
| DRN-04 | Collecte ciblée | Envoie un drone doté d’un manipulateur chercher un objet connu et le rapporter. | L’objet doit encore être présent et transportable. Ramassage et retour utilisent les actions du drone, sans transfert instantané vers votre inventaire. |
| DRN-05 | Tirs coordonnés | Désigne une cible perçue pour les prochaines attaques ordinaires de plusieurs drones. | Chacun doit encore pouvoir la percevoir et l’atteindre au moment de tirer. La commande ne déclenche pas de salve gratuite. |
| DRN-06 | Interposition | Ordonne à un drone de préparer la protection d’un allié en se plaçant sur la trajectoire d’un projectile. | Le drone prépare sa garde pendant son action, puis dépense sa réaction pour se déplacer. Il peut subir le projectile et ne protège pas de toute une explosion. |
| DRN-07 | Éclaireur autonome | Étend une patrouille par une reconnaissance limitée au-delà du trajet connu, suivie d’un retour et d’un rapport. | Demande Patrouille bornée, des capteurs et une réserve de retour suffisante. Le rapport est daté ; aucune vision tactique en direct n’est fournie hors de votre perception. |
| DRN-08 | Routine conditionnelle | Ajoute à une consigne une condition locale simple, comme se replier si les PV deviennent faibles ou s’arrêter devant un danger détecté. | Utilise les perceptions et actions du drone. Ne permet ni programme sans limite ni accès à l’état caché de la carte. |
| DRN-09 | Déploiement coordonné | Assigne en une procédure plusieurs positions et rôles connus à vos drones joignables. | Chaque unité se déplace ensuite à son rythme, par un trajet réel. La commande demande du temps et de la bande passante. |
| DRN-10 | Repli d'urgence | Remplace les consignes offensives de plusieurs drones par un retour temporaire vers un point connu, suivi d’une attente. | Exige une liaison et des trajets praticables. Ne téléporte pas les unités et ne garantit pas le sauvetage d’un drone encerclé. |

## 7. États et lecture du monde

Infobulles d'états observés : afficher durée restante, source et quantité seulement lorsque ces informations sont connues. L'absence d'une icône ennemie n'est pas une garantie d'absence d'effet. Les messages thermiques distinguent la jauge, son seuil d'alerte et le programme Surchauffe.

| Identifiant | Libellé | Texte joueur |
|---|---|---|
| UI-ETAT-ENTRAVE | Entravé | Vos déplacements sont ralentis et les manœuvres rapides concernées sont indisponibles. Vous pouvez encore agir ; cet effet n’est pas une immobilisation totale. |
| UI-ETAT-SUPPRESSION | Sous suppression | Votre Précision est temporairement réduite. Vous gardez le choix de vos actions ; vous n’êtes pas contraint de fuir. |
| UI-ETAT-FRAGILISATION | Blindage fragilisé | Une partie de votre Blindage est temporairement neutralisée. Une nouvelle application du même effet ne cumule pas la réduction et ne prolonge pas sa durée active. |
| UI-ETAT-FONCTION | Fonction suspendue | La fonction indiquée est temporairement indisponible. Vos autres fonctions restent utilisables selon leurs propres conditions. |
| UI-ETAT-PROTECTION | Protection temporaire | Empêche une nouvelle application de la famille d’effet indiquée pendant une courte durée. Ne protège pas contre les dégâts ou les autres familles d’effets. |
| UI-ETAT-SURCHAUFFE | Sabotage thermique actif | Un programme augmente votre chaleur et gêne la dissipation. Une purge réussie ou l’arrêt local du module compromis peut y mettre fin ; la chaleur et les dégâts déjà subis restent présents. |
| UI-ETAT-INFECTION | Infection active | Un programme provoque des dégâts périodiques. Selon sa variante, il peut tenter de se transmettre localement. Purger un hôte n’efface pas automatiquement les infections déjà transmises aux autres. |
| UI-ETAT-IMPLOSION | Stockage armé | Un stockage compromis doit exploser après le délai indiqué. Une purge ou sa déconnexion physique peut empêcher l’explosion ; les alentours du stockage sont menacés. |
| UI-ETAT-BROUILLAGE | Canal brouillé | Le canal indiqué fonctionne moins efficacement ou ne permet plus la liaison nécessaire. Les autres canaux et les observations déjà enregistrées ne sont pas effacés. |
| UI-ETAT-SURCADENCEMENT | Module surcadencé | Une sortie du module est renforcée, au prix d’une consommation et d’une chaleur accrues. Selon le régime et la température, son utilisation peut dégrader sa durabilité. |
| UI-ETAT-DERIVATION | Fonction dérivée | Une fonction endommagée reste partiellement utilisable grâce à un autre sous-système suspendu. Rétablir ce dernier met fin à cette alimentation de secours. |
| UI-ETAT-VEILLE | Fonction en veille | Cette fonction n’émet plus son signal actif, mais elle est indisponible jusqu’à sa réactivation. Les autres signatures du matériel restent présentes. |
| UI-ETAT-CAMOUFLAGE | Camouflage actif | Votre signature est réduite sur le canal indiqué, pas sur tous les capteurs. Le maintien consomme des ressources et une action offensive peut y mettre fin. |
| UI-ETAT-CHARGE-LOURDE | Charge excessive | Votre charge ralentit les déplacements et réduit votre Esquive. Les manœuvres rapides incompatibles sont indisponibles. Déposez du matériel pour alléger votre charge. |
| UI-ETAT-CHARGE-BLOQUANTE | Charge immobilisante | Votre charge empêche les déplacements volontaires. Vous pouvez encore agir sur place, notamment déposer un objet ; rien n’est abandonné automatiquement. |
| UI-ETAT-HORS-LIAISON | Hors liaison | Aucun nouvel ordre ne peut être transmis. Une routine déjà reçue peut continuer selon ses règles. La dernière position confirmée ne décrit pas nécessairement la position actuelle. |
| UI-ETAT-ACCES | Accès temporaire | Vous disposez des droits indiqués sur ce système, pour la durée et les conditions prévues. Cet accès ne s’étend pas automatiquement à ses voisins. |
| UI-ETAT-VERROU | Contrôle verrouillé | Les reprises de commande ordinaires sont temporairement bloquées dans le périmètre indiqué. La liaison, les interventions physiques et les réinitialisations restent déterminantes. |
| UI-CONTACT-ACTUEL | Présence perçue | Cette présence est actuellement repérée par un canal compatible. Son identité et ses capacités ne sont connues que si vos observations permettent de les déterminer. |
| UI-CONTACT-INCERTAIN | Indice incertain | Un indice a été recueilli, mais il ne confirme pas une position actuelle précise ni une identité. Son emplacement éventuel représente seulement la zone que l’indice permet d’estimer. |
| UI-CONTACT-SOUVENIR | Dernière observation | Information enregistrée lors d’un précédent contact. La présence ou l’état indiqué peut avoir changé depuis. |
| UI-RAPPORT-DRONE | Rapport de reconnaissance | Observations datées rapportées par un drone. Elles enrichissent votre mémoire du lieu sans maintenir une vision actuelle de ses occupants. |
| UI-SONAR | Balayage des capteurs | Représente la zone accessible à vos capteurs. Les murs et portes fermées bloquent le balayage ; son animation ne retarde pas une information déjà perçue. |
| UI-ATTITUDE | Attitude connue | Indique une relation établie par les informations disponibles : allié, neutre ou ennemi. Une attitude inconnue reste indéterminée ; un filtre matériel ne devine pas les intentions cachées. |

## 8. Libellés de fiche et informations manquantes

Les modèles courts ci-dessous s'affichent uniquement avec des valeurs connues. Les noms de techniques, statistiques et états servent aussi de libellés : ne pas leur créer des synonymes concurrents selon l'écran.

| Identifiant | Libellé | Texte joueur |
|---|---|---|
| UI-CHAMP-RANG | Rang minimal | Rang minimal : {rang} |
| UI-CHAMP-PREREQUIS | Prérequis | Requiert : {technique} |
| UI-CHAMP-MATERIEL | Matériel requis | Matériel requis : {materiel} |
| UI-CHAMP-TEMPS | Temps | Préparation : {preparation} UT ; exécution : {execution} UT |
| UI-CHAMP-RECUPERATION | Récupération | Récupération après attaque : {recuperation} UT |
| UI-CHAMP-COUT | Coût d’énergie | Énergie : {energie} — {moment} |
| UI-CHAMP-ENTRETIEN | Entretien | Énergie de maintien : {energie} par UT |
| UI-CHAMP-CHALEUR | Chaleur produite | Chaleur : +{chaleur} — {moment} |
| UI-CHAMP-CANAL | Canal occupé | Bande passante occupée : {bande} — {reservation} |
| UI-CHAMP-STOCK | Matériel consommé | Consomme : {quantite} × {objet} |
| UI-CHAMP-ZONE | Zone affectée | Zone : {geometrie} ; portée : {portee} cases |
| UI-CHAMP-DUREE | Durée | Durée : {duree} UT |
| UI-CHAMP-DELAI | Réutilisation | Réutilisable dans {delai} UT |
| UI-CHAMP-CHANCE | Chances de réussite | Chances de réussite : {chance} % |
| UI-CHAMP-VALEUR | Valeur non déterminée | Non déterminé |
| UI-CHAMP-CHANCE-INCONNUE | Chances non déterminées | Chances non déterminées |
| UI-CHAMP-SOURCE | Provenance du relevé | Source : {source} ; observation : {date} |
| UI-CHAMP-RELATION | Relation | Attitude : {attitude} |
| UI-CHAMP-MODIFICATEUR | Contribution | {source} : {valeur_signee} |

`{moment}` emploie « au démarrage », « à l’exécution » ou « au déclenchement », selon la dépense réelle ; `{reservation}` décrit « pendant la tentative », « pendant la commande » ou « pendant le maintien », selon le profil. `{geometrie}` utilise « une cible », « une case », « arc », « cône », « secteur » ou « rayon de {rayon} cases », sans promettre de contourner un obstacle. `{attitude}` emploie « Allié », « Neutre », « Ennemi » ou « Indéterminée » selon la connaissance disponible. Les phases non applicables sont omises, pas remplacées par un faux zéro. Prévoir les formes singulier/pluriel pour les noms d'objets lors de la localisation ; les abréviations UT restent invariantes.

## 9. Refus, avertissements et notifications

### 9.1. Refus avant engagement

Tous ces refus sont gratuits seulement lorsque leur condition est déjà connue. Ne pas appeler la simulation cachée pour choisir un message plus précis. Si une commande légale rencontre un changement non observable, utiliser la résolution et les messages de résultat de la section 9.3, avec les coûts effectivement engagés.

| Identifiant | Situation connue | Texte joueur |
|---|---|---|
| MSG-REFUS-TECHNIQUE | Technique non apprise. | Vous n’avez pas appris cette technique. |
| MSG-REFUS-PREREQUIS | Technique mère absente lors de l'achat. | Apprenez d’abord « {technique} ». |
| MSG-REFUS-RANG | Rang minimal non atteint pour le choix proposé. | Ce choix demande le rang {rang} en {discipline}. |
| MSG-REFUS-POINTS | Budget insuffisant pour l'achat complet. | Points insuffisants : {cout} requis, {disponibles} disponibles. |
| MSG-REFUS-CHOIX | Cinq choix déjà acquis. | Vous avez déjà effectué vos cinq choix dans cette compétence. |
| MSG-REFUS-DEJA-APPRIS | Même choix déjà acquis. | Vous avez déjà appris cette technique. |
| MSG-REFUS-PRIMAIRE | Répartition de création hors budget ou bornes. | Répartition invalide : respectez le budget et les limites indiqués pour chaque statistique. |
| MSG-REFUS-MATERIEL | Équipement requis absent ou incompatible connu. | Matériel compatible requis : {materiel}. |
| MSG-REFUS-MODE | Mode d'arme requis absent. | Cette arme ne possède pas le mode nécessaire. |
| MSG-REFUS-ENERGIE | Réserve personnelle connue insuffisante à l'étape. | Énergie insuffisante pour cette étape : {cout} requis, {disponibles} disponibles. |
| MSG-REFUS-MUNITIONS | Stock connu insuffisant pour l'étape de tir. | Munitions insuffisantes pour ce tir. |
| MSG-REFUS-OBJET | Consommable, charge ou pièces absents. | Matériel consommable manquant : {objet}. |
| MSG-REFUS-BANDE | Capacité libre insuffisante avant réservation. | Bande passante insuffisante. Suspendez un processus avant de lancer cette action. |
| MSG-REFUS-CHALEUR | Apport propre strictement positif et projection connue dépassant la limite du mode ; jamais pour +0 ou un refroidissement. | Cette action ajoute de la chaleur et dépasserait la limite autorisée : {projection} pour une limite de {limite}. |
| MSG-REFUS-RECUPERATION | Attaque interdite par récupération en cours. | Vous récupérez encore. Déplacez-vous, attendez ou choisissez une action de soutien autorisée. |
| MSG-REFUS-DELAI | Technique en délai de réutilisation. | Cette technique sera réutilisable dans {delai} UT. |
| MSG-REFUS-REACTION | Réaction déjà consommée. | Votre réaction a déjà été utilisée. |
| MSG-REFUS-CONTACT | Pas de perception actuelle requise pour cibler une entité. | Aucun contact actuel compatible avec ce ciblage. |
| MSG-REFUS-PORTEE | Cible ou case connue trop distante. | Cible hors de portée. |
| MSG-REFUS-TRAJECTOIRE | Obstacle observé bloquant le trajet. | Un obstacle connu bloque la trajectoire. |
| MSG-REFUS-DESTINATION | Destination actuellement connue inaccessible. | Cette destination est inaccessible. |
| MSG-REFUS-CHARGE | Masse transportée connue empêchant le mouvement. | Votre charge empêche ce déplacement. Déposez du matériel pour vous alléger. |
| MSG-REFUS-MANOEUVRE | Corps, posture ou état connu incompatible. | Votre état ou votre matériel ne permet pas cette manœuvre. |
| MSG-REFUS-INTERFACE | Incompatibilité de cible déjà établie. | Aucune interface compatible pour cette procédure. |
| MSG-REFUS-LIAISON | Absence de liaison établie, sans révéler sa cause cachée. | Aucune liaison valide pour transmettre cette commande. |
| MSG-REFUS-DROIT | Droits connus insuffisants. | Votre accès n’autorise pas cette fonction. |
| MSG-REFUS-IDENTIFIANT | Aucun justificatif disponible pour l'usurpation. | Aucun identifiant utilisable pour cette autorisation. |
| MSG-REFUS-PROGRAMME | Pas de programme identifié sélectionnable pour Purge. | Aucun programme hostile identifié à sélectionner. |
| MSG-REFUS-COMPOSANT | Aucun composant identifié pour la technique. | Identifiez d’abord un composant accessible. |
| MSG-REFUS-FAIBLESSE | Faiblesse requise non connue. | Aucune faiblesse exploitable identifiée pour cette action. |
| MSG-REFUS-SUPPORT | Support requis non identifié. | Aucun support adapté identifié pour cet effondrement. |
| MSG-REFUS-STOCKAGE | Réserve incompatible ou insuffisante déjà diagnostiquée. | Le stockage identifié ne permet pas d’armer cette Implosion. |
| MSG-REFUS-ATELIER | Conditions d'intervention matérielle non remplies. | Cette intervention demande un atelier ou un outil adapté. |
| MSG-REFUS-PLAN | Recette non connue. | Vous ne connaissez pas le plan nécessaire. |
| MSG-REFUS-REPARATION | État non réparable établi. | Cet élément ne peut pas être réparé par cette intervention. |
| MSG-REFUS-DRONES | Nombre d'unités ou canaux insuffisant connu. | Votre contrôleur ne peut pas prendre en charge cette unité supplémentaire. |
| MSG-REFUS-ALLIE | Coopération ou transportabilité déjà connue absente. | Cet allié ne peut pas être extrait dans ces conditions. |
| MSG-REFUS-GENERIQUE | Autre précondition légale connue non satisfaite. | Les conditions connues ne permettent pas cette action. |

MSG-REFUS-PROGRAMME ne signifie jamais « cible saine ». MSG-REFUS-CONTACT ne signifie jamais « case vide ». MSG-REFUS-LIAISON ne précise pas la présence d'un brouilleur, d'une unité ou d'un mur inconnu. Les noms de matériel demandé proviennent de la technique connue, pas d'une consultation gratuite de la cible cachée.

### 9.2. Avertissements avant une action légale

Ces avertissements décrivent les risques connus avant confirmation d'une action, sans la rendre gratuite après engagement. Les confirmations n'ajoutent aucune action au monde par elles-mêmes. L'avertissement général d'incertitude s'applique de la même façon à toutes les cases sans observation actuelle, occupées ou non.

| Identifiant | Situation connue | Texte joueur |
|---|---|---|
| MSG-ALERTE-TIERS | Zone affectant alliés ou neutres effectivement observés. | Des alliés ou des neutres connus sont dans la zone affectée. |
| MSG-ALERTE-ZONE-INCONNUE | Zone partiellement hors perception actuelle. | Cette zone n’est pas entièrement observée. L’aperçu ne garantit pas qu’elle soit vide. |
| MSG-ALERTE-THERMIQUE | Projection propre atteignant le seuil d'alerte. | Cette action vous placera en alerte thermique. |
| MSG-ALERTE-SURCADENCEMENT | Mode autorisant un dépassement avec risques connus. | Ce régime peut provoquer des dégâts thermiques et dégrader le module. Consultez la chaleur projetée avant de poursuivre. |
| MSG-ALERTE-PREPARATION | Action comportant plusieurs étapes. | La préparation prend du temps : les adversaires pourront agir avant son achèvement. |
| MSG-ALERTE-ABANDON | Préparation engagée, remplacement par une autre action. | Changer d’action abandonnera cette préparation. Les dépenses déjà engagées ne seront pas remboursées. |
| MSG-ALERTE-GARDE | Nouvelle action normale mettant fin à la garde courante. | Votre garde actuelle prendra fin au début de cette action. |
| MSG-ALERTE-DERIVATION | Sous-système donneur identifié avant intervention. | Cette dérivation suspendra « {fonction} » tant qu’elle sera maintenue. |
| MSG-ALERTE-CAPACITE | Retrait d'un module réduisant la capacité disponible. | Cette modification réduit votre capacité disponible. Vérifiez les réserves conservées et les processus qui seront suspendus. |
| MSG-ALERTE-DESAMORCAGE | Indices connus de piège contre manipulation. | Ce dispositif peut se déclencher si le désamorçage échoue. |
| MSG-ALERTE-IMPLOSION | Stockage et environs partiellement observés. | Le stockage explosera après son armement et son délai. Les unités et objets proches peuvent subir des dégâts. |
| MSG-ALERTE-TRACE | Procédure hostile dont le risque général est connu. | Cette tentative peut laisser une trace exploitable par la sécurité, même en cas d’échec. |
| MSG-ALERTE-VARIANTE | Affichage du sélecteur de profil électronique. | Une seule variante s’appliquera à cette utilisation. |
| MSG-ALERTE-MATERIEL-FUTUR | Achat légal avant obtention du matériel. | Vous pourrez apprendre cette technique, mais il vous manque actuellement le matériel nécessaire pour l’utiliser. |

### 9.3. Résultats et notifications après résolution

Les résultats ci-dessous ne sont émis que pour les événements connus de l'observateur. Une procédure engagée qui échoue conserve son temps et ses coûts réels, sans facturer une étape jamais commencée. Les compteurs de dégâts, morts, contaminations ou récupérations ne totalisent pas les événements cachés. Un journal technique de débogage éventuel doit rester séparé de l'interface du joueur.

| Identifiant | Situation observable | Texte joueur |
|---|---|---|
| MSG-RESULTAT-ECHEC | Échec résolu sans cause perceptible précise. | La tentative n’a pas abouti. |
| MSG-RESULTAT-INTERROMPU | Préparation interrompue, cause non identifiée. | Votre préparation a été interrompue. |
| MSG-RESULTAT-INTERROMPU-CONNU | Cause de l'interruption effectivement connue. | Votre préparation a été interrompue : {cause}. |
| MSG-RESULTAT-COUT | Dépenses connues d'une procédure échouée ou abandonnée. | Dépenses engagées : {depenses}. |
| MSG-RESULTAT-RATE | Attaque manquée effectivement observable. | Votre attaque manque sa cible. |
| MSG-RESULTAT-ABSORBE | Impact connu sur soi, ou absorption confirmée par observation. | Impact absorbé : aucun dégât physique subi. |
| MSG-RESULTAT-DEGATS | Dégâts reçus par le joueur ou quantité connue sur une cible observée. | {cible} : {quantite} dégâts {type}. |
| MSG-RESULTAT-POUSSEE | Déplacement bloqué constaté après un coup, cause non déterminée. | La cible n’a pas été repoussée. |
| MSG-RESULTAT-RESISTANCE | Résistance à un effet explicitement observable. | {cible} résiste à {effet}. |
| MSG-RESULTAT-REPLI | Repli de réaction n'ayant pu avoir lieu. | Le déplacement de secours n’a pas pu être effectué. |
| MSG-RESULTAT-REACTION | Réaction ne pouvant plus s'exécuter, sans dévoiler une cause cachée. | La réaction préparée n’a pas pu se déclencher. |
| MSG-RESULTAT-ACCES | Droits effectivement reçus. | Accès obtenu : {droits}. |
| MSG-RESULTAT-ACCES-PERDU | Fin d'accès constatée. | Cet accès n’est plus disponible. |
| MSG-RESULTAT-LIAISON | Liaison perdue pendant une procédure. | Liaison perdue : la procédure ne peut pas être poursuivie. |
| MSG-RESULTAT-COMMANDE | Réception d'ordre confirmée, avant exécution propre du destinataire. | Commande transmise. Son exécution dépend encore du dispositif. |
| MSG-RESULTAT-PURGE | Retrait du programme effectivement confirmé. | Programme retiré : {programme}. Les dégâts et la chaleur déjà subis restent présents. |
| MSG-RESULTAT-PURGE-ECHEC | Nettoyage tenté mais aucun retrait confirmé. | Aucun retrait de programme n’a été confirmé. |
| MSG-RESULTAT-REGISTRE | Modification locale confirmée. | Événement local modifié. Les copies déjà transmises et les témoignages ne sont pas effacés. |
| MSG-RESULTAT-DONNEES | Lot reçu avec provenance. | Données reçues : {lot}. Source : {source} ; date : {date}. |
| MSG-RESULTAT-ANALYSE | Conclusion sans détail supplémentaire accessible. | Analyse terminée. Aucun détail supplémentaire n’a pu être établi avec les informations disponibles. |
| MSG-RESULTAT-INSPECTION | Inspection n'ayant rien révélé. | Aucun nouvel indice repéré dans la zone examinée. |
| MSG-RESULTAT-DECOUVERTE | Indice ou élément réellement révélé par l'observation. | Découverte : {element}. Une intervention distincte peut être nécessaire. |
| MSG-RESULTAT-TRACE | Trace examinée, direction et ancienneté effectivement déduites. | Trace observée : {type_trace}. Direction estimée : {direction} ; ancienneté estimée : {anciennete}. |
| MSG-RESULTAT-PIEGE | Déclencheur neutralisé et résultat observable. | Piège neutralisé. |
| MSG-RESULTAT-ARMEMENT | Dispositif posé encore inactif, avec délai d'armement connu excluant le cycle de pose. | Dispositif posé. Armement dans {delai} UT. |
| MSG-RESULTAT-FUSIBLE | Charge déjà armée avec délai de détonation connu excluant le cycle d'armement. | Charge armée. Détonation dans {delai} UT. |
| MSG-RESULTAT-BALISE | Balise manuelle posée inactive. | Balise posée, inactive. Une commande sera nécessaire pour lancer le champ. |
| MSG-RESULTAT-EXPLOSION | Détonation à venir annoncée par un signe observable. | Explosion imminente : évacuez la zone signalée. |
| MSG-RESULTAT-REPARATION-PV | Réparation du corps effectivement réalisée. | {quantite} PV restaurés. |
| MSG-RESULTAT-REPARATION-OBJET | Réparation d'un composant effectivement réalisée. | {objet} : {quantite} points de durabilité restaurés. |
| MSG-RESULTAT-RECHARGE | Énergie réellement transférée. | {quantite} unités d’énergie transférées. |
| MSG-RESULTAT-RECUPERATION | Objet réellement récupéré, état connu conservé. | Objet récupéré : {objet}. État : {etat}. |
| MSG-RESULTAT-PANNE | Panne de son matériel ou panne observée. | Fonction indisponible : {fonction}. |
| MSG-RESULTAT-REFROIDISSEMENT | Chaleur du joueur revenue sous le seuil critique. | Chaleur revenue sous le seuil critique. Les dégâts déjà subis restent présents. |
| MSG-RESULTAT-ARRET-ENERGIE | Entretien impossible par réserve propre épuisée. | {processus} s’arrête faute d’énergie disponible. |
| MSG-RESULTAT-SUSPENSION | Réservation interrompue après baisse de capacité. | {processus} suspendu : capacité de contrôle insuffisante. |
| MSG-RESULTAT-DRONE-PERDU | Perte du contact, sans preuve de destruction. | Contact perdu avec {drone}. Dernière confirmation : {date}. |
| MSG-RESULTAT-DRONE-RAPPORT | Rapport réellement transmis au retour ou à la reconnexion. | Rapport reçu de {drone} : observations du {date}. |
| MSG-RESULTAT-COLLECTE | Échec rapporté par le drone seulement après contact valide. | Le drone n’a pas pu rapporter l’objet demandé. |
| MSG-RESULTAT-APPRENTISSAGE | Achat atomique complet confirmé. | Technique apprise : {technique}. {discipline} atteint le rang {rang}. |
| MSG-RESULTAT-NIVEAU | Nouveau niveau effectivement accordé. | Niveau {niveau} atteint. |
| MSG-RESULTAT-POINTS | Attribution effective, distincte du seul changement de niveau. | Points obtenus : {recompenses}. |
| MSG-RESULTAT-XP | Récompense nouvelle versée, source connue. | +{quantite} XP — {source}. |
| MSG-RESULTAT-ETAT-FINI | Expiration ou retrait observable d'un effet. | Effet terminé : {effet}. |
| MSG-RESULTAT-MORT | PV du joueur à zéro, fin de run. | Votre personnage est mort. Cette partie est terminée. |

Pour MSG-RESULTAT-DEGATS, `{type}` emploie le type connu (« physiques », « thermiques », « électriques », « chimiques ») ; si le type n'est pas établi, l'omettre sans consulter l'état caché. « Aucun nouvel indice » ne certifie pas l'absence de secret. « Contact perdu » ne signifie pas « drone détruit ». Une infection autonome hors perception ne produit ni notification d'hôte, ni compteur de victimes, ni déplacement de curseur vers cet hôte.

## 10. Vérification éditoriale et passage à l'étape 8

La rédaction couvre tous les profils du catalogue actuel, sans réintroduire les entrées retirées. Les textes validés individuellement ne sont pas réécrits. Les conditions d'affichage distinguent information certaine, mémoire datée et résultat inconnu ; les chiffres restent rattachés aux règles et au matériel.

Contrôles reproductibles : `node tools/validate_stats_docs.mjs` et `node tools/review_stats_edge_cases.mjs`. Ils couvrent les 104 descriptions actives, les noms et prérequis, les dix disciplines, tableaux et liens, puis les corrections de chronologie, chaleur et disponibilité dans des modèles isolés. Ils ne prouvent pas à eux seuls la justesse de chaque phrase ni la bonne implémentation des frontières d'information.

Les cinq corrections du point 8 sont appliquées après accord explicite. Retraite méthodique n'est plus un achat ; son identifiant reste dans l'historique du catalogue, sans texte d'achat actif ici. Les quinze infobulles validées individuellement restent inchangées. Les risques d'équilibrage encore ouverts concernent notamment la maîtrise de Reconnaissance, les petits dégâts d'Infection, les ressources, les neutralisations répétées, les réactions des drones et les routes complètes. Les essais du moteur et du rendu restent à réaliser.
