# Factions et communautés — première base de conception

Version 0.6 — 20 septembre 2026.

## 1. Statut et périmètre

L'utilisateur a approuvé la proposition des quatre premiers groupes, avec une exigence explicite : leurs caractéristiques doivent se refléter dans le jeu et rester plausibles à programmer. Ce document consigne cette base de conception. Depuis la version 0.2, le premier circuit matériel du Collectif et des Récupérateurs est jouable. Une première propriété des lots, affiliation technique, mémoire individuelle des prises observées, réaction d'alerte locale, alarme de capteur visible et intervention bornée vers un incident enregistré sont maintenant implémentées. Une disposition de combat déclarative distingue aussi allié, neutre et hostile pour le ciblage autonome des compagnons ; les factions, réputations et sociétés complètes restent documentaires.

Les noms des groupes restent provisoires. Leur identité, leurs besoins et les comportements décrits ci-dessous sont la direction retenue. Les quantités, durées, seuils de réputation, lieux exacts, personnages et formats techniques restent à définir. Les pistes d'implémentation ne sont pas présentées comme des systèmes déjà disponibles.

Complément soumis à validation : [six peuplades candidates](PROPOSITION_PEUPLADES_v0.1.md). Leurs noms, corps et coutumes sont des propositions, pas des choix déjà approuvés. Ces peuplades ne remplacent pas les quatre groupes ci-dessous : origine culturelle, affiliation et métier restent distincts.

Références : [document de conception](../Projet_Roguelike_IA_Document_Conception_v0.2.md), [trame narrative, notamment le monde habité](../Trame%20narrative%20et%20qu%C3%AAte%20principale%20%E2%80%94%20Projet%20Roguelike%20IA.md), [architecture actuelle](MOTEUR.md) et [ville de test](VILLE_ET_EXTERIEUR.md). Les documents fondateurs et le scénario ne sont pas modifiés par cette annexe. La ville du prototype reste un terrain de test, pas une identification définitive avec une ville du récit.

## 2. Principes retenus

- Distinguer origine ou forme du corps, communauté, affiliation et métier. Une apparence ne définit pas automatiquement un camp ; un métier peut être exercé au service de plusieurs organisations.
- Donner à chaque groupe un besoin matériel, des lieux utiles, des occupations observables et des relations avec les autres. Le joueur n'est pas la raison d'être de tous les habitants.
- Faire découler les réactions des faits connus : observation directe, témoignage ou transmission par un moyen existant. Un dégât dont l'auteur est inconnu n'est pas automatiquement imputé au joueur.
- Distinguer opinion personnelle, réputation auprès d'un groupe et réaction immédiate à une menace. Sauver un technicien ne rend pas automatiquement toute son organisation alliée.
- Faire progresser l'étrangeté des corps, habitats, cultures et valeurs avec celle de l'environnement : plus le joueur descend, moins le monde paraît humain. C'est une dominante, pas une obligation d'un peuple par étage ; des enclaves familières peuvent subsister. Préserver des communautés jusque dans certaines couches profondes : étrangeté et hostilité ne sont pas synonymes. Leurs coutumes précises restent à valider.
- Ne pas créer d'avantage à l'attente réelle, au survol ou à l'ouverture de menus. L'activité suit le temps du jeu ; son affichage reste soumis aux limites de perception.
- Préférer des routines limitées dont les conséquences sont réelles à des promesses de simulation totale. Une histoire culturelle peut être écrite sans simuler toute une civilisation, mais une réparation annoncée comme mécanique doit changer l'état de l'installation.

## 3. Les quatre premiers groupes

### 3.1. Les Habitants du Quartier

**Nature :** communauté locale attachée à son habitat. Dans les premières couches, ses habitudes doivent encore évoquer une société humaine familière. Aucune espèce unique ni croyance uniforme n'est imposée à tous ses membres.

**Priorité :** préserver logements, services et sécurité quotidienne. Son attitude envers l'évasion dépend des conséquences pour ses habitants, pas d'une obligation de soutenir le protagoniste.

**Présence dans le jeu :** certains habitants circulent entre quelques lieux connus ; d'autres tiennent un poste. Un témoin d'un danger interrompt son occupation, cherche un refuge ou prévient un garde. Une installation défaillante affecte un passage ou un service qui en dépend réellement. Des répliques courtes évoquent les événements connus.

**Base programmable :** lieux d'activité, occupation actuelle, événement perçu et réaction prioritaire. Il n'est pas nécessaire de simuler d'emblée famille, alimentation, sommeil ou emploi du temps complet pour chaque personne. Les dialogues contextuels exigent des conditions de déclenchement ; ils ne peuvent révéler un fait inconnu de leur locuteur.

**Preuve attendue :** lors d'un incident près de la place, les témoins réagissent ; les habitants qui n'ont reçu aucune information poursuivent leur activité. Un service fermé pour une panne ne reste pas utilisable par son interface.

### 3.2. Le Collectif de maintenance

**Nature :** organisation qui entretient des équipements utilisés par plusieurs communautés. Ses membres valorisent la continuité du service et revendiquent leur autonomie. Elle ne rassemble pas automatiquement tous les techniciens du monde.

**Priorité :** conserver les installations utiles, l'accès aux interventions et les moyens de les accomplir. Elle dépend notamment de pièces fournies par les récupérateurs.

**Présence dans le jeu :** un technicien prend en charge un travail connu, rejoint un dépôt, prélève les pièces nécessaires, se rend sur place et répare. La réparation prend du temps et consomme des ressources. Sans pièce ou trajet accessible, le travail attend ; une menace peut interrompre l'intervention et provoquer un repli.

**Base programmable :** travaux déclarés, installations à états explicites, stock et attribution des interventions. Le premier système vise quelques appareils prévus pour être réparables, pas tous les éléments du décor. Une panne doit être connue par une observation, un signalement ou un système de suivi défini ; aucun diagnostic omniscient n'est accordé implicitement.

**Preuve attendue :** deux techniciens ne prélèvent pas la même dernière pièce et ne terminent pas deux fois le même travail. Une réparation achevée rétablit la fonction concernée et diminue effectivement le stock.

### 3.3. Les Récupérateurs des friches

**Nature :** groupes vivant de matériel abandonné et de sa remise en circulation. Leurs échanges rendent service aux quartiers et ateliers ; leurs méthodes et leurs revendications sur les ressources peuvent créer des tensions.

**Priorité :** préserver leurs réserves et leurs accès aux zones de récupération. Un récupérateur isolé n'est pas nécessairement membre de cette organisation.

**Présence dans le jeu :** un membre parcourt un petit secteur ou un itinéraire, récupère des objets admissibles dans la limite de sa capacité, puis les apporte à un dépôt. Il évite une menace plutôt que de combattre systématiquement. Une prise constatée sur du matériel appartenant au groupe peut susciter une réaction.

**Base programmable :** objets persistants, transport borné, dépôt, règles de récupération et propriété explicite. Le déplacement d'un objet relie des stocks réels. Le premier commerce local possède désormais ses propres fonds, stocks blancs par profondeur, exemplaires revendus et paris déterministes ; il reste distinct du stock de maintenance et ne prétend pas encore simuler une économie mondiale.

**Preuve attendue :** un composant ramassé disparaît du sol, reste transporté jusqu'à son dépôt et n'est ni dupliqué ni recréé par un changement de zone. Un objet possédé n'est pas assimilé automatiquement à un déchet récupérable.

### 3.4. La Sécurité du système

**Nature :** autorité organisée, pas nécessairement un peuple. Elle surveille des installations, contrôle des accès et traite les incidents dans les limites des unités et liaisons dont elle dispose.

**Priorité :** appliquer ses règles de sécurité et préserver les équipements sous sa responsabilité. Elle peut protéger une installation utile aux habitants tout en restreignant leurs déplacements ou les activités des récupérateurs.

**Présence dans le jeu :** patrouilles entre points de passage, observation d'incidents, signalement et recherche locale d'une cible identifiée. Après perte de contact, les unités ne connaissent pas sa position actuelle. Une intervention mobilise des unités existantes ou des renforts dont l'origine est définie.

**Base programmable :** états de patrouille, observation, signalement, recherche et retour au poste. Le signalement exige un trajet vers un poste ou une liaison disponible selon un système effectivement implémenté ; ni radio universelle ni communication à travers les murs ne sont ajoutées implicitement. La recherche après perte de vue et le combat entre PNJ demandent de nouveaux comportements.

**Preuve attendue :** empêcher une transmission empêche la diffusion correspondante. Un incident non attribué ne dégrade pas automatiquement la réputation du joueur. Les renforts ne sont pas matérialisés arbitrairement à côté de lui.

## 4. Interdépendances visibles

Le Quartier utilise les services entretenus par le Collectif. Le Collectif a besoin des pièces des Récupérateurs. La Sécurité protège certains équipements, mais peut restreindre les itinéraires qui permettent leur approvisionnement.

Situation de référence : une restriction d'accès retarde une livraison ; une réparation reste en attente ; un service demeure indisponible. Le joueur peut aider à livrer la pièce ou rétablir l'accès, contourner la difficulté ou profiter de l'indisponibilité du service. Ces possibilités doivent passer par les règles des objets, accès et installations, pas par un dialogue qui réinitialise gratuitement le problème.

Cette relation de dépendance n'impose pas encore de score diplomatique, de guerre ou de pénurie permanente. Le besoin commun fournit des situations ; les valeurs précises et les réactions restent à calibrer.

## 5. Faisabilité : état du moteur au 20 septembre 2026

État vérifié par les tests du moteur et le parcours de diagnostic du client.

| Élément | Base réutilisable | Travail encore nécessaire |
|---|---|---|
| Déplacements et perception | Acteurs, recherche de chemin déterministe, occupation des cases et vision bloquée par les obstacles. Les agents de maintenance savent ouvrir une porte ordinaire sur leur trajet ; une porte verrouillée ou sans alimentation reste infranchissable. | Refuge, mémoire des observations et politiques de trajet propres aux futures factions. |
| Décisions des acteurs | Profils d'IA produisant attente, mouvement ou attaque. Une relation de combat immédiate, distincte de l'affiliation, empêche les compagnons autonomes de prendre un acteur neutre ou allié pour une menace. | L'IA hostile active cible aujourd'hui le joueur ; résolution dynamique des relations, choix de cible général, entraide et combat autonome entre PNJ ne sont pas implémentés. |
| Objets | Piles au sol persistantes, prélèvement partiel, propriété éventuelle conservée au sol et dans l'inventaire, autorisations initiales de prise indépendantes de la propriété et acquisition permanente de ce droit par effet de quête pour un propriétaire précis, cargaison exclusive des récupérateurs, stock de dépôt et consommation par un travail. Le marchand conserve séparément fonds, stocks et instances revendues. | Révocation explicitement conçue des autorisations, renouvellement marchand et transferts commerciaux entre zones. |
| Installations | Intégrité, capacités et dépendances déclaratives ; relais, actionneur de porte, capteur et dépôt. Une réparation rétablit les sorties réellement liées. | Dégâts provoqués par le joueur, réparations libres de tout décor, réseaux plus riches et effets autres que les capacités enregistrées. |
| Vie hors écran | Les zones visitées avancent sur les tours consommés ; patrouilles, statuts et travaux de maintenance y continuent sans produire d'information visuelle distante. | Poursuite, livraison ou migration entre zones. |
| Dialogue, services et quêtes | Un PNJ adjacent et visible expose une interaction en lecture seule : rôle localisé, réplique dérivée de son état connu, éventuelle alerte individuelle, services et quêtes séparés. Consulter ou fermer cette vue ne consomme aucun tour. Le technicien accepte le matériau demandé par le véritable ordre de réparation. La marchande propose achat blanc, revente exacte et paris cachés sans dépendre d'un exploit, d'une alerte ou de cette réparation. Le soigneur suit une courte routine poste/pause mais rend ses soins payants partout où le joueur le rejoint, sans prérequis de quête ou de maintenance. La fondation de quête attache plusieurs contrats ordonnés au même acteur, valide leurs prérequis sans cycle, suit quatre états et applique atomiquement crédits, expérience et objets. Ses objectifs typés couvrent la livraison, l'exploration, la consultation d'un enregistrement et la défaite d'acteurs étiquetés. Un groupe de choix affiche plusieurs approches au clavier comme à la souris ; l'acceptation de l'une exclut réellement les autres, puis sa réussite débloque la suite correspondante. L'habitant de la ville porte la première branche sobre, sans perdre sa routine ni devenir un service. Le journal dérive sa progression de l'état réel. Des badges `?` et `!` signalent respectivement l'offre et le rapport quand le PNJ est visible. | Identités individuelles définitives, arbres de conversation indépendants des offres de quête, conséquences sur le monde autres que le déblocage, partage du crédit avec les alliés, réputation marchande, renouvellement des stocks et privilèges supplémentaires. |
| Relations sociales et sécurité | Affiliation technique facultative, droits initiaux de prise déclaratifs, acquisition permanente d'un droit par quête, mémoire individuelle des prises non autorisées réellement vues, réaction locale d'un témoin et alarme temporaire d'un capteur opérationnel. Portée et obstacles sont réellement évalués. Un témoin peut signaler le fait à un unique acteur déclaré ou, par une seconde liaison explicite, à un capteur opérationnel du même propriétaire. Le destinataire humain conserve une mémoire reçue distincte et peut enquêter physiquement sur la case fixe ; le système installé peut activer ses réponses locales. Les réponses déclaratives verrouillent une porte après validation d'une issue sûre ou accélèrent une source finie en transmettant au prochain renfort la case de l'incident. Celui-ci enquête sans connaître la position actuelle du joueur et peut poursuivre seulement après perception locale. Les sources et verrouillages perçus sont signalés sur la carte, dans le HUD, l'inspection et le journal ; aucun événement d'interface ne révèle une source cachée. La lecture tactique apprise peut visualiser les champs de tous les PNJ actuellement visibles, y compris les services sans profil social spécialisé, mais n'ajoute aucun acteur caché. | Relations, opinions, révocation éventuelle des autorisations, messagers physiques, transmission entre plusieurs systèmes ou zones, mobilisation multi-source et réputation. |
| Suspension | La version 9 reconstruit et vérifie installations, propriété, affiliations, souvenirs, alertes locales, alarmes installées, réponses, stocks, cargaisons, réservations, travaux et progression temporelle. Les versions 1 à 8 gardent leurs mondes historiques. | Une future modification incompatible de ces règles exigera une nouvelle version ou une migration explicite. |

Points de code : [acteurs](../src/entity/actor.rs), [décisions IA](../src/ai/behavior.rs), [résolution des commandes et tours](../src/game/game_state.rs), [zones et simulation hors écran](../src/game/expedition.rs), [suspension](../src/suspension.rs).

**Protection de la ville :** la règle hostile actuelle interdit toujours l'entrée et les dégâts dans les cases protégées. Les deux travailleurs de la preuve se déplacent par le sous-système de leur installation et le premier habitant par une routine locale explicitement enregistrée ; cela ne constitue pas encore une règle générale d'affiliation. Intrus et gardes demandent une autorisation explicite par acteur ou groupe avant de remplacer cette séparation provisoire.

## 6. Première preuve jouable implémentée

Périmètre volontairement restreint : un technicien, un récupérateur, un dépôt, un relais en panne et un régulateur de puissance placé dans la ville de test. L'actionneur de la porte de service et le capteur de sécurité dépendent du relais. Ce graphe de dépendances explicite n'est pas un réseau électrique complet.

Le récupérateur réserve puis transporte l'unique pièce jusqu'au dépôt. Le joueur peut toutefois le devancer, ramasser la pièce et la livrer lui-même au dépôt ou la confier directement au technicien par son panneau de service ; les deux chemins alimentent le même ordre, consomment exactement la demande et libèrent la réservation autonome devenue inutile. Parler ne passe pas le temps, contrairement à la confirmation du transfert. La pièce porte une propriété explicite. Le droit de prise est évalué séparément : une affiliation au propriétaire ou une autorisation déclarée par l'expédition évite tout incident sans effacer cette propriété. La preuve de base n'accorde aucun droit au joueur afin de garder le scénario d'alerte testable. Un travailleur affilié ne mémorise une prise non autorisée que si sa portée et sa ligne de vue couvrent l'action ; un acteur hors portée ou derrière un obstacle ne l'apprend pas. Le témoin configuré entre alors temporairement en alerte individuelle. Le récupérateur peut en outre signaler directement ce fait au technicien si leur portée et leur ligne de vue courantes le permettent ; le technicien conserve alors une mémoire reçue distincte et peut entrer en alerte, sans relayer le message. S'ils sont visibles, leurs glyphes conservent leur forme et reçoivent les marques d'alerte existantes ; le journal nomme le signalement reçu sans ajouter de code couleur indispensable. Un acteur caché conserve son état sans produire de message révélateur. Le technicien s'y prépositionne, prélève cette même pièce, rejoint le relais et travaille cinq tours. Une porte ordinaire est ouverte comme une action de terrain ; une route impossible est réessayée aux tours suivants sans téléportation. Après réussite, la pièce a été consommée, le relais et le capteur sont opérationnels et la porte de service passe de « sans alimentation » à « fermée ». Le joueur ne peut pas encore donner d'ordre au collectif.

La première tranche de vie urbaine ajoute séparément un soigneur, une clinique lisible sur la carte et une routine bornée entre poste et pause. Cette activité continue sur les tours consommés, y compris hors écran, mais ne conditionne pas le service : le joueur peut demander un soin lorsqu'il rejoint le PNJ. Le prix dépend seulement des PV effectivement restaurés ; fonds, santé, caisse et temps sont modifiés atomiquement, tandis qu'un refus ne change rien. Ce service neutre reste indépendant du circuit de maintenance, des alertes locales et des futurs noms de faction.

La tranche suivante ajoute un habitant sans commerce, soin ni ordre de réparation. Il alterne entre les abords des logements et la place par la même primitive bornée de déplacement entre deux points que le soigneur. Sa réplique décrit sa phase réelle — présence, trajet ou retour. Rôle, routine, services et quêtes restent des données distinctes : ses demandes ajoutées en version 65 ne fabriquent aucun service et ne suspendent jamais son circuit.

Le socle suivant ajoute ce canal de quête séparé. Quatre fixtures déterministes prouvent livraison, exploration, consultation d'archive et combat étiqueté par les actions réelles du moteur. Une cinquième prouve qu'un prérequis reste invisible avant achèvement, puis verse objet et expérience sans paiement partiel. Une sixième présente deux approches, accepte celle choisie au clavier ou à la souris et retire l'autre branche. La version 66 ajoute un état du monde borné et nommé : une quête peut le produire, une autre l'exiger, le joueur voit son résumé et le donneur peut adapter sa réplique. La version 67 ajoute un effet matériel déclaré, prévisualisé puis appliqué dans la même transaction que la remise ; la version 68 étend cette primitive aux installations et la version 69 aux droits permanents de prise pour un propriétaire précis. Une fixture dédiée accomplit le contrat, ramasse ensuite un lot attribué devant un témoin affilié et vérifie l'absence de souvenir comme d'alerte ; le lot conserve son propriétaire dans l'inventaire. La ville réutilise les primitives précédentes : l'habitant propose soit un relevé des abords, soit la consultation du registre, puis une vérification propre au fait local réellement établi. La branche des relevés peut ouvrir l'accès arrière du dépôt ; celle des archives remplace réellement le registre incomplet par une correction à relire sur le terminal. Le nouveau droit n'est pas encore attaché artificiellement à cette branche urbaine : il attend un contrat et un propriétaire narrativement appropriés. Ni l'entrée principale ni les services ordinaires ne sont conditionnés par ces demandes. Les formulations et récompenses restent des données de prototype et pourront suivre les futurs personnages définitifs.

### Critères d'acceptation

1. La livraison et la réparation peuvent aboutir sans qu'une quête donnée au joueur soit nécessaire pour les lancer.
2. Les objets ont un seul emplacement ou détenteur à chaque étape ; leurs transferts et leur consommation ne les dupliquent pas.
3. Plusieurs travailleurs ne réservent pas la même ressource ou le même travail de façon incompatible. Une interruption libère ou conserve explicitement ses réservations, sans blocage permanent.
4. Un manque de pièces ou un trajet bloqué suspend l'activité de façon compréhensible ; une occasion de reprise existe quand la situation change. La politique exacte de reprise appartient au contrat de la future implémentation.
5. L'installation change réellement de fonctionnement après réparation ; une simple animation ou ligne de journal ne suffit pas.
6. Une zone visitée continue d'avancer pendant les tours consommés ailleurs, selon le contrat existant. Aucune zone non visitée n'est rendue active et aucun mouvement interzone de PNJ n'est ajouté implicitement.
7. L'évolution hors perception n'actualise pas les informations du joueur. Au retour, l'observation révèle le nouvel état ; les menus ne font pas avancer les travaux.
8. La suspension et la reprise conservent stocks, états des travaux, détenteurs et progression temporelle sans nouvelle livraison ni réparation gratuite. Un changement de règles incompatible exige une décision de version ou migration explicite.
9. Une même configuration, seed et suite d'actions produit le même résultat. Les routines doivent avoir des recherches bornées ; les performances seront mesurées, pas déduites du seul fait que le projet utilise Rust.
10. Toute alerte ou alarme perceptible doit avoir une représentation explicite en jeu, au minimum textuelle et cartographique. Elle ne peut pas exister uniquement comme état interne ou dépendre uniquement d'une couleur.

Les critères 1 à 10 sont couverts par les tests du module, du monde multi-zone et de la suspension : exclusivité des réservations, conservation de matière et de propriété, blocage/reprise, dépendances, déterminisme, évolution hors écran sans fuite d'événements et rejeu version 9. Les tests sociaux distinguent témoin visible, témoin caché du joueur et acteur incapable de voir l'acte ; ils vérifient aussi la durée bornée, l'absence d'alerte lors d'une prise refusée ou explicitement autorisée et la compatibilité de la version 6 sans réaction ajoutée rétroactivement. La preuve v69 vérifie l'acquisition du droit par quête, la v70 le destinataire unique et l'absence de retransmission, et la v71 le déplacement vers l'incident enregistré plutôt que vers le joueur. La preuve v72 vérifie qu'un capteur ne reçoit le fait que par une liaison réelle depuis son témoin, qu'une dépendance hors service coupe cette liaison, qu'un même incident ne double pas l'alarme et qu'une mobilisation accélère exactement une source finie dont l'intervenant enquête sur la case enregistrée. Elle vérifie aussi la compatibilité v71 sans liaison ajoutée rétroactivement. Les messagers physiques, la transmission entre plusieurs systèmes ou zones et la réputation restent des jalons séparés.

## 7. Jalon différé — factions et réputation

Décision du 20 septembre 2026 : la réputation, les relations chiffrées entre groupes et leurs effets sur prix, accès ou quêtes sont explicitement repoussés. Ils ne constituent plus la prochaine tranche du moteur et aucune version de génération ne leur est réservée. Le sujet ne sera rouvert qu'après définition d'un ensemble de groupes suffisamment nombreux et distincts, de leurs intérêts contradictoires et d'au moins une situation jouable où ces relations produisent un choix réel plutôt qu'une simple jauge.

La première aide matérielle directe, la propriété, l'autorisation initiale ou gagnée par quête, le témoignage individuel, l'alerte locale, le signalement direct à un destinataire, l'enquête physique de ce destinataire, la liaison vers un système installé, l'alarme visible, le verrouillage local et la mobilisation d'une ressource finie vers l'incident restent disponibles sans impliquer une réputation. Le droit v69 demeure permanent et limité à un propriétaire ; sa révocation n'est pas déduite d'un score inexistant. Les liaisons v70 et v72 restent locales, explicites et à un seul saut ; la réponse v71 et l'intervention mobilisée ne connaissent que la case enregistrée.

Les invariants déjà établis pour cette prochaine tranche sont les suivants :

- la réputation d'un groupe, l'opinion d'un individu, l'alerte immédiate et la relation de combat restent quatre états distincts ;
- aucun score global ne peut apprendre un incident sans une chaîne de connaissance réelle et attribuée ;
- une variation doit viser un groupe stable et conserver sa cause consultable, plutôt que modifier silencieusement tous les PNJ ;
- les services ordinaires neutres ne deviennent pas conditionnels par défaut. Toute remise, surtaxe, restriction ou faveur devra être une politique explicite du service concerné ;
- les valeurs seront bornées et les seuils déclaratifs. Un seuil ne devra pas encoder à lui seul une hostilité de combat permanente ;
- l'interface devra nommer le groupe, le sens du changement et sa cause sans dépendre uniquement d'une couleur.

Lorsque ce jalon sera rouvert, resteront à décider avant le code : l'échelle numérique ou les paliers nommés, la persistance ou l'érosion, les faits positifs et négatifs admissibles, la portée géographique d'un groupe, le traitement des preuves contradictoires, et les premiers effets concrets sur dialogues, prix, accès ou quêtes.

À préciser avant les implémentations concernées : noms définitifs, apparences et coutumes ; règles exactes de propriété et d'intervention du joueur ; représentation d'une ou plusieurs appartenances ; barèmes de réputation et effets de service ; modes de transmission ; traitement des conflits dans la ville protégée. Les valeurs actuelles de la preuve restent des données de prototype modifiables.

Hors première preuve : économie mondiale, démographie, faim et sommeil universels, diplomatie de conquête, migrations entre toutes les couches, dialogues générés par un modèle de langage, reconstruction libre de tout le décor et pouvoirs exclusifs de faction. Aucun de ces systèmes n'est nécessaire pour valider le premier circuit de vie locale. Leur éventuel ajout demanderait sa propre conception.

Cette annexe et son complément sur les peuplades ne définissent encore ni faction jouable ni valeur de réputation. Le circuit matériel, la chaîne locale de connaissance et de mobilisation décrite dans la section 6, le premier commerce local indépendant et la première disposition de combat déclarative sont implémentés ; ils ne constituent pas encore une économie ou une société complète.
