# Factions et communautés — première base de conception

Version 0.3 — 11 septembre 2026.

## 1. Statut et périmètre

L'utilisateur a approuvé la proposition des quatre premiers groupes, avec une exigence explicite : leurs caractéristiques doivent se refléter dans le jeu et rester plausibles à programmer. Ce document consigne cette base de conception. Depuis la version 0.2, le premier circuit matériel du Collectif et des Récupérateurs est jouable. Une première propriété des lots, affiliation technique, mémoire individuelle des prises observées, réaction d'alerte locale, alarme de capteur visible et intervention bornée vers un incident enregistré sont maintenant implémentées ; les factions, réputations et sociétés complètes restent documentaires.

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

**Base programmable :** objets persistants, transport borné, dépôt, règles de récupération et propriété explicite. Le déplacement d'un objet relie des stocks réels. Le commerce ultérieur utilisera ces stocks, mais demande un développement distinct ; la première preuve ne promet pas une économie complète.

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

## 5. Faisabilité : état du moteur au 11 septembre 2026

État vérifié par les tests du moteur et le parcours de diagnostic du client.

| Élément | Base réutilisable | Travail encore nécessaire |
|---|---|---|
| Déplacements et perception | Acteurs, recherche de chemin déterministe, occupation des cases et vision bloquée par les obstacles. Les agents de maintenance savent ouvrir une porte ordinaire sur leur trajet ; une porte verrouillée ou sans alimentation reste infranchissable. | Refuge, mémoire des observations et politiques de trajet propres aux futures factions. |
| Décisions des acteurs | Profils d'IA produisant attente, mouvement ou attaque. | L'IA active cible aujourd'hui le joueur ; affinités, choix de cible, entraide et combat autonome entre PNJ ne sont pas implémentés. |
| Objets | Piles au sol persistantes, prélèvement partiel, propriété éventuelle conservée au sol et dans l'inventaire, autorisations initiales de prise indépendantes de la propriété, cargaison exclusive des récupérateurs, stock de dépôt et consommation par un travail. | Acquisition ou révocation dynamique des autorisations, commerce et transferts entre zones. |
| Installations | Intégrité, capacités et dépendances déclaratives ; relais, actionneur de porte, capteur et dépôt. Une réparation rétablit les sorties réellement liées. | Dégâts provoqués par le joueur, réparations libres de tout décor, réseaux plus riches et effets autres que les capacités enregistrées. |
| Vie hors écran | Les zones visitées avancent sur les tours consommés ; patrouilles, statuts et travaux de maintenance y continuent sans produire d'information visuelle distante. | Poursuite, livraison ou migration entre zones. |
| Relations sociales et sécurité | Affiliation technique facultative, droits initiaux de prise déclaratifs, mémoire individuelle des prises non autorisées réellement vues, réaction locale d'un témoin et alarme temporaire d'un capteur opérationnel. Portée et obstacles sont réellement évalués. Les réponses déclaratives peuvent verrouiller les portes d'un actionneur après validation d'une issue sûre, accélérer une source finie et transmettre au prochain renfort la case fixe de l'incident. Celui-ci enquête sans connaître la position actuelle du joueur et peut poursuivre seulement après perception locale. Les sources et verrouillages perçus sont signalés sur la carte, dans le HUD, l'inspection et le journal ; aucun événement d'interface ne révèle une source cachée. Une installation peut toutefois déclarer explicitement une balise de navigation : elle diffuse alors seulement une direction et une distance approximatives tant qu'elle est opérationnelle et à portée. | Relations, opinions, évolution des autorisations en jeu, transmission entre acteurs ou systèmes au-delà de cette liaison locale, mobilisation multi-source et réputation. |
| Suspension | La version 9 reconstruit et vérifie installations, propriété, affiliations, souvenirs, alertes locales, alarmes installées, réponses, stocks, cargaisons, réservations, travaux et progression temporelle. Les versions 1 à 8 gardent leurs mondes historiques. | Une future modification incompatible de ces règles exigera une nouvelle version ou une migration explicite. |

Points de code : [acteurs](../src/entity/actor.rs), [décisions IA](../src/ai/behavior.rs), [résolution des commandes et tours](../src/game/game_state.rs), [zones et simulation hors écran](../src/game/expedition.rs), [suspension](../src/suspension.rs).

**Protection de la ville :** la règle hostile actuelle interdit toujours l'entrée et les dégâts dans les cases protégées. Les deux travailleurs de la preuve se déplacent par le sous-système de leur installation dans cette ville protégée ; cela ne constitue pas encore une règle générale d'affiliation. Des habitants mobiles, intrus et gardes demandent une autorisation explicite par acteur ou groupe avant de remplacer cette séparation provisoire.

## 6. Première preuve jouable implémentée

Périmètre volontairement restreint : un technicien, un récupérateur, un dépôt, un relais en panne et un régulateur de puissance placé dans la ville de test. L'actionneur de la porte de service et le capteur de sécurité dépendent du relais. Ce graphe de dépendances explicite n'est pas un réseau électrique complet.

Le récupérateur réserve puis transporte l'unique pièce jusqu'au dépôt. Le joueur peut toutefois le devancer, ramasser la pièce et la livrer lui-même par une interaction adjacente ; le transfert consomme exactement la demande et libère la réservation autonome devenue inutile. La pièce porte une propriété explicite. Le droit de prise est évalué séparément : une affiliation au propriétaire ou une autorisation déclarée par l'expédition évite tout incident sans effacer cette propriété. La preuve de base n'accorde aucun droit au joueur afin de garder le scénario d'alerte testable. Un travailleur affilié ne mémorise une prise non autorisée que si sa portée et sa ligne de vue couvrent l'action ; un acteur hors portée ou derrière un obstacle ne l'apprend pas. Le témoin configuré entre alors temporairement en alerte individuelle. S'il est visible, son glyphe conserve sa forme et reçoit un badge `!` agrandi et une double bordure. La carte entière est encadrée et un bandeau pleine largeur indique le nombre de sources et la durée restante ; l'inspection, le panneau de détection et le journal le répètent. S'il est invisible, il conserve fait et alerte sans produire de message révélateur. Le technicien s'y prépositionne, prélève cette même pièce, rejoint le relais et travaille cinq tours. Une porte ordinaire est ouverte comme une action de terrain ; une route impossible est réessayée aux tours suivants sans téléportation. Après réussite, la pièce a été consommée, le relais et le capteur sont opérationnels et la porte de service passe de « sans alimentation » à « fermée ». Le joueur ne peut pas encore donner d'ordre au collectif.

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

Les critères 1 à 10 sont couverts par les tests du module, du monde multi-zone et de la suspension : exclusivité des réservations, conservation de matière et de propriété, blocage/reprise, dépendances, déterminisme, évolution hors écran sans fuite d'événements et rejeu version 9. Les tests sociaux distinguent témoin visible, témoin caché du joueur et acteur incapable de voir l'acte ; ils vérifient aussi la durée bornée, l'absence d'alerte lors d'une prise refusée ou explicitement autorisée et la compatibilité de la version 6 sans réaction ajoutée rétroactivement. Les tests de sécurité distinguent capteur inopérant, propriétaire étranger, ligne de vue bloquée et incident détecté ; ils vérifient aussi le refus anti-enfermement, le verrouillage temporaire, son expiration hors écran et la compatibilité des versions 7 et 8. Transmission, recherche et conséquences de réputation appartiennent au jalon suivant.

## 7. Suite et décisions encore ouvertes

La première aide matérielle directe, la propriété, l'autorisation initiale, le témoignage individuel, l'alerte locale, l'alarme installée visible, le verrouillage local et une intervention bornée vers l'incident sont désormais possibles. Ordre de travail envisagé après ce circuit : définir comment gagner ou perdre une autorisation ; habitants et réactions immédiates ; transmission et signalement entre plusieurs systèmes ; mobilisation ; conséquences de réputation et interactions entre groupes. Les catalogues complets de factions seront introduits au bon jalon, sans attendre d'avoir écrit toutes les cultures du monde.

À préciser avant les implémentations concernées : noms définitifs, apparences et coutumes ; règles exactes de propriété et d'intervention du joueur ; représentation d'une ou plusieurs appartenances ; barèmes de réputation et effets de service ; modes de transmission ; traitement des conflits dans la ville protégée. Les valeurs actuelles de la preuve restent des données de prototype modifiables.

Hors première preuve : économie mondiale, démographie, faim et sommeil universels, diplomatie de conquête, migrations entre toutes les couches, dialogues générés par un modèle de langage, reconstruction libre de tout le décor et pouvoirs exclusifs de faction. Aucun de ces systèmes n'est nécessaire pour valider le premier circuit de vie locale. Leur éventuel ajout demanderait sa propre conception.

Cette annexe et son complément sur les peuplades ne définissent encore ni faction jouable, ni réputation, ni commerce. Seuls le circuit matériel et le témoignage local décrit dans la section 6 sont implémentés ; ils ne modifient pas le système de statistiques et compétences.
