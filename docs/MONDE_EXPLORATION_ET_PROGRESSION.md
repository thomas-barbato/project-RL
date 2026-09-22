# Monde, exploration et progression

**Mise à jour :** 22 septembre 2026.

**Statut :** direction de conception validée par l'utilisateur ; mise en œuvre encore partielle. Les valeurs d'essai et les exemples signalés comme propositions ne sont pas des paramètres approuvés.

## 1. Expérience recherchée

Le jeu doit proposer une aventure longue, exigeante et rejouable. Le joueur doit pouvoir consacrer des sessions à explorer, découvrir des lieux, faire évoluer son personnage et expérimenter les systèmes sans avancer immédiatement la quête principale. L'évasion demeure l'objectif de victoire ; le monde doit aussi donner envie d'être parcouru pour lui-même.

Deux références éclairent cette direction :

- **Diablo II :** une structure d'aventure et des lieux nécessaires aux quêtes, intégrés dans des territoires dont une partie de la géographie est générée. La présence du contenu indispensable est garantie par la construction du monde.
- **Caves of Qud :** l'intérêt propre de l'exploration et de l'expérimentation, dans un monde combinant contenu écrit et génération procédurale.

Ces références ne fixent ni une quantité de contenu comparable, ni leurs règles de combat, de retour en ville ou de renouvellement des ennemis. Project RL conserve son tour par tour, son corps principal durable, ses règles de perception et sa persistance pendant une run.

## 2. Deux échelles à développer ensemble

La demande porte à la fois sur la taille des cartes extérieures et sur l'étendue du territoire parcouru entre les villes et les profondeurs.

| Échelle | Direction retenue |
|---|---|
| Carte locale | De grands extérieurs dotés de reliefs, d'obstacles, de repères et de lieux distincts. La taille peut varier selon le rôle du lieu. |
| Couche | Un territoire comprenant une ville propre à cette couche, plusieurs zones sauvages, des destinations et des branches facultatives. |
| Progression verticale | Des expéditions entre les étapes importantes. Les arrivées, villes et accès profonds ne doivent pas former systématiquement un même puits urbain direct. |
| Exploration libre | Des régions et des lieux intéressants en dehors du trajet nécessaire à la victoire, avec des boucles et des retours possibles. |

Une ville sert de refuge et de point d'ancrage. Elle conserve son identité et peut garder un plan conçu à la main. Son rôle dans le territoire, les chemins alentour et les accès doivent être pensés à l'échelle de la couche.

« Sauvage » désigne ici l'espace hors des villes : friches, forêts, ruines, infrastructures, installations abandonnées, territoires habités ou milieux anormaux. Cela ne signifie ni une nature uniforme, ni une hostilité obligatoire de chaque occupant.

Une grande carte doit comporter une géographie utile : passages, abris, points d'observation, obstacles contournables et destinations. Les espaces calmes peuvent donner du relief au rythme. La densité d'ennemis ne doit pas être multipliée mécaniquement par la surface.

## 3. Génération encadrée par le scénario

Le scénario déclare ses éléments indispensables et leurs dépendances. Le monde réserve leur place, puis produit la géographie et les rencontres compatibles avec ces obligations.

| Élément | Garantie | Variation possible |
|---|---|---|
| Ville | Présence de l'ancre urbaine prévue pour la couche et accès aux services ordinaires selon leur politique déclarée. | Implantation dans le territoire et chemins alentour ; plan propre à la ville. |
| Lieu principal | Présence de chaque lieu nécessaire au scénario choisi. | Position, disposition locale, accès secondaires et défenses autorisées. |
| Information ou dispositif critique | Au moins un moyen cohérent d'obtenir l'information ou d'accomplir l'état nécessaire. | Supports alternatifs, circonstances de découverte et moyens de résolution. |
| Branche principale | Présence de chaque branche exigée par le scénario. | Difficultés, ressources et rencontres dans les limites de son contrat. |
| Contenu secondaire | Respect des contraintes du contenu effectivement sélectionné. | Présence, emplacement, variantes, récompenses et habitants. |

Dans la proposition actuelle de La Porte Zéro, Ville et Jardin doivent tous deux exister, même si une seule de ces routes suffit. Cette obligation reste propre à ce scénario proposé ; la direction de génération validée ne transforme pas ses personnages et révélations en canon définitif.

Les lieux importants existent indépendamment de l'acceptation d'une quête. Accepter une mission ne fait pas apparaître rétroactivement sa destination dans un secteur déjà exploré. Une découverte anticipée doit être reconnue lorsqu'elle satisfait réellement l'objectif. Une commande explicite de relevé peut toutefois demander une nouvelle observation : ce contrat doit être annoncé, sans imposer de refaire une action déjà suffisante.

La garantie de génération porte sur les conditions initiales : connexions, prérequis et moyens de progression. Elle ne garantit ni victoire, ni résolution facile avec les ressources restantes du personnage. Les pertes et les actes du joueur conservent leurs conséquences ; les solutions de secours reposent sur des supports prévus dans le monde, sans réapparition arbitraire d'un objectif détruit.

## 4. Organisation technique visée

Le moteur possède déjà des graines déterministes, des régions générées à la première visite, des passages persistants et une validation de navigation. L'étape suivante est un **plan de couche soumis aux contraintes du scénario**. Ce contrat général reste à implémenter.

1. Déclarer les lieux, rôles, connexions et dépendances nécessaires au scénario choisi.
2. Construire un plan déterministe à partir de la graine, avec des identités stables pour les lieux garantis, des branches et des contraintes de distance configurables.
3. Réserver ces lieux et leurs passages avant la matérialisation des cartes ; le plan léger n'exige pas de générer tout l'atlas.
4. Matérialiser les cartes détaillées à la visite, puis placer les éléments réservés, les rencontres et le butin compatibles.
5. Valider indépendamment les accès physiques et les dépendances logiques. Un objet requis ne peut pas être seulement accessible derrière la porte qu'il ouvre.
6. Conserver le plan et ses états dans le contrat déterministe de suspension. Une reprise ou un retour ne redistribue aucun lieu ni récompense.

Le schéma de données exact reste à concevoir. Il devra réutiliser les définitions de monde, passages, installations et quêtes existantes. Les flux du terrain, des lieux, des rencontres et du butin restent séparés autant que nécessaire pour éviter qu'un changement de récompense ne déplace les étapes principales.

Une validation de connectivité ne prouve pas l'équilibrage. Les essais doivent aussi vérifier que les principales familles de personnages disposent de solutions réalistes, avec des coûts et des risques différents. Toutes les solutions n'ont pas à exister dans chaque rencontre.

## 5. Exploration libre et histoires locales

La quête principale donne une direction à long terme. Le joueur doit pouvoir la laisser en attente pour explorer ou expérimenter, sans accepter des missions uniquement pour avoir le droit de parcourir le monde.

Les lieux secondaires doivent apporter au moins un intérêt identifiable : outil inhabituel, interaction à découvrir, histoire locale, relation, raccourci, danger singulier ou ressource utile. Leur seul rôle ne peut pas être de fournir l'expérience manquante avant une quête obligatoire.

Les habitants ont des besoins, des liens et des désaccords qui ne concernent pas tous l'évasion. Une quête locale peut modifier une installation, un accès ou une aide sans passer par une simulation sociale complète. Le report des factions et de la réputation reste applicable ; ces systèmes demanderont toujours leur conception propre.

Une intention narrative urgente ne doit pas créer implicitement un compte à rebours global pénalisant toute promenade. Les alertes et crises locales peuvent continuer à évoluer selon des causes visibles. Une mission chronométrée éventuelle doit annoncer son déclencheur, son délai en tours et ses conséquences. L'évolution des situations déjà actives respecte le contrat de simulation des zones visitées ; cela ne demande pas de simuler tout l'atlas inexploré.

## 6. Longueur et difficulté

La campagne complète vise une partie victorieuse substantielle, pouvant s'étendre sur plusieurs sessions, et une victoire exigeante. La durée chiffrée, le taux de victoire cible et les budgets de contenu ne sont pas encore fixés.

Il faut mesurer séparément :

- la durée d'une partie victorieuse suivie normalement ;
- le temps cumulé et les tentatives nécessaires à une première victoire ;
- une partie consacrée à l'exploration et aux expérimentations ;
- le parcours d'un joueur expérimenté qui utilise des raccourcis connus.

Les neuf jalons de la trame ne définissent ni neuf petites tâches, ni neuf cartes, ni une durée de campagne. Chaque grand jalon doit préciser son territoire, ses situations, ses préparatifs et ses conséquences avant qu'on puisse en estimer la durée.

La difficulté doit venir notamment des rencontres, de la gestion des ressources, du positionnement, des choix d'itinéraire et de la compréhension des systèmes. Les possibilités de fuite et de renoncement ont une place réelle. Une solution accessible sans talent obligatoire peut demander une préparation difficile ou un détour dangereux.

Le joueur peut gagner dès sa première tentative s'il réussit effectivement les épreuves. Aucun nombre de morts ni bonus permanent de puissance ne doit être nécessaire. Cette possibilité ne signifie pas qu'une première victoire soit facile ou attendue.

Une longue exploration doit offrir des occasions raisonnables de maintenir son style de jeu : réparation, énergie, équipement et moyens ordinaires de secours sont à équilibrer. Elle ne doit ni épuiser définitivement les compétences essentielles, ni rendre un unique talent de soutien obligatoire.

La durée ne doit pas être produite par des trajets vides, des dialogues à répéter, une multiplication de clés équivalentes, des niveaux obligatoires, des ennemis artificiellement gonflés ou du farming sans risque. Explorer doit améliorer les options du personnage tout en conservant des défis ; un ajustement automatique de tous les ennemis au niveau du joueur n'est pas adopté par ce document.

Les retours connus devront rester confortables. Voyage automatique interrompu par un danger perçu et raccourcis ouverts par les actes du joueur sont des pistes à concevoir. Ils doivent respecter le temps simulé et les informations disponibles. Aucun portail gratuit ni voyage instantané universel n'est décidé ici.

## 7. État actuel et valeurs encore à éprouver

Au 22 septembre 2026, les données locales définissent :

- une carte de départ de 192 × 128 cases, ville et friches comprises ;
- des régions de surface ordinaires de 128 × 80 cases dans les nouvelles parties, et des régions profondes ordinaires de 96 × 64 ;
- depuis la version 93, deux enceintes de surface par région, trois caches et deux camps, dont un seul terminal nécessaire au relevé ; le plafond des groupes de rencontre gagne un tirage, sans doubler la présence initiale ;
- cinq villes profondes aux couches 1 à 5, placées sur le même puits ;
- un atlas très étendu, dont la superficie adressable ne représente pas une quantité équivalente de contenu conçu ;
- des premières quêtes locales de démonstration, sans campagne complète d'évasion.

Cette organisation est un état de prototype. L'essai de taille et de présence des sites ne représente pas encore les expéditions sauvages, les lieux narratifs garantis et la durée visés par le présent document. Les parties suspendues en version 91 ou antérieure conservent leur taille de 96 × 64 à la surface ; celles de version 92 gardent aussi leurs anciens reliefs, sites et tirages de butin. Les villes et les profondeurs ne changent pas. Le second site donne une destination facultative, mais les enceintes emploient encore une seule famille de plan. Il faut éprouver la durée des trajets et diversifier les types de lieux et de décisions avant de retenir cette formule.

Les tailles de **160 × 112** pour un extérieur courant et **192 × 128** pour certains grands lieux ont été proposées comme essais. Elles ne constituent ni des valeurs approuvées, ni des dimensions uniformes à appliquer. Le nombre de régions entre étapes et la répartition des difficultés restent également à éprouver.

Une première preuve pourrait porter sur une couche, sa ville, plusieurs régions extérieures, des lieux garantis, deux itinéraires et quelques destinations facultatives. Sa taille limitée servirait à tester la conception ; elle ne fixerait pas la durée du jeu final.

## 8. Critères de validation futurs

- Les lieux indispensables existent sur les graines testées et leurs dépendances sont réalisables.
- Changer l'ordre des visites ne déplace pas les lieux garantis ; reprendre une partie conserve les mêmes faits.
- Plusieurs personnages et itinéraires peuvent atteindre le prochain jalon sans talent universel obligatoire.
- Deux graines changent les décisions et les approches, au-delà du déplacement des objets.
- Une session d'exploration sans progression de la quête principale apporte découvertes et décisions intéressantes.
- Une route principale offre une progression soutenue ; les branches facultatives ne deviennent pas une taxe de niveau ou de collecte.
- Le suivi des essais distingue temps de décision, découverte, déplacement et répétition, avec les ressources et causes de mort.
- Un joueur expérimenté peut gagner du temps par sa maîtrise ; les trajets et dialogues déjà compris ne servent pas de durée minimale artificielle.

## 9. Documents et références

- [Document fondateur](../Projet_Roguelike_IA_Document_Conception_v0.2.md), sections 4 et 12.
- [Spécification technique](../Projet_Roguelike_IA_Spec_CODEX_v0.2.md), sections 41 et 43.
- [Trame narrative proposée](../Trame%20narrative%20et%20qu%C3%AAte%20principale%20%E2%80%94%20Projet%20Roguelike%20IA.md), sections 6 et 10.
- [État de la carte jouable](VILLE_ET_EXTERIEUR.md) et [moteur](MOTEUR.md).
- [Évaluation narrative du 21 septembre](EVALUATION_NARRATIVE.md), avis critique et propositions distinctes des décisions validées.
- [Diablo II — carte officielle de l'acte I](https://classic.battle.net/diablo2exp/maps/act1.shtml) et [principes de variation des cartes](https://classic.battle.net/diablo2exp/basics/) : lieux de quête associés aux zones et coexistence de plans variables et fixes.
- [Caves of Qud — présentation officielle](https://cavesofqud.com/) : monde mêlant narration écrite, génération et interactions systémiques. Cette référence n'est pas un engagement à reproduire son volume de contenu.
