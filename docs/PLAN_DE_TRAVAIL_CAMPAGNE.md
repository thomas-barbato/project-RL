# Plan de travail de la nouvelle campagne

17 octobre 2026 — ordre de travail accepté dans la conversation. Identifiant
de suivi : **PLAN-CAMPAGNE-02**. Ce plan organise la réalisation ; il n'autorise
aucune nouvelle valeur de dégâts, règle de voyage, compétence ou appellation.
Ces propositions suivront leur revue habituelle avant intégration.

Références : [histoire courante](HISTOIRE_CAMPAGNE.md),
[monde et progression](MONDE_EXPLORATION_ET_PROGRESSION.md),
[décisions de refonte](reviews/refonte-histoire-2026-10-16/CADRAGE.md),
[intégration graphique du bestiaire](reviews/refonte-histoire-2026-10-16/bestiary-integration-06/README.md).

## Ordre recommandé

Étudier la construction du monde, concevoir notre propre génération, puis
répartir les monstres est cohérent. Deux ajustements réduisent les reprises :

- Vérifier très tôt une petite boucle complète : exploration, combat avec
  quelques compétences représentatives, XP, récompense et retour au camp.
  Le nombre de cartes reste limité tant que cette boucle n'est pas satisfaisante.
- Entretenir la stabilité, la lisibilité et une première difficulté pendant
  toute la réalisation. La passe finale d'équilibrage viendra après les boss,
  lorsqu'on pourra évaluer une campagne complète.

Le premier jalon jouable comprend un secteur représentatif. Il permet de
mesurer les distances, les lignes de tir, le nombre d'adversaires engagés et
le rythme de progression avant d'étendre le contenu.

## PM00 — Terminer le socle actuel

**But :** disposer d'une base identifiable avant de changer le monde.

Consolider le catalogue, les modèles refusés et les sources préservées ;
distinguer les dessins utilisables des pouvoirs encore à définir. Conserver
les identités et variantes après chargement, ainsi que les corps et le butin.
Les apparences raccordées aux anciens rôles constituent un raccord provisoire :
elles ne fixent pas le peuplement de la future campagne.

Établir le bilan des tests de l'application, en particulier les empreintes
historiques et les reprises de partie. Prioriser les régressions qui touchent
les sauvegardes ou empêchent de tester la nouvelle campagne. Les captures et
fichiers temporaires inutiles ne sont retirés qu'après préservation et contrôle.

**Livrable :** état de référence, décisions artistiques retrouvables, liste
des problèmes techniques restant ouverts et registre de rangement.

**Critère de sortie :** le fonctionnement retenu peut être reproduit et les
limites connues sont documentées ; les défauts critiques de reprise sont traités
avant de s'appuyer dessus pour la nouvelle génération.

## PM01 — Étudier Diablo II et Project Diablo 2

**But :** comprendre les choix qui servent l'exploration et les combats.

Séparer les observations de Diablo II des modifications propres à Project
Diablo 2. Examiner les données locales disponibles et les sources primaires
pertinentes ; distinguer faits vérifiables, observations en jeu et hypothèses.
L'étude portera sur :

- L'organisation des actes, zones, sorties, refuges et raccourcis.
- Les éléments fixes et variables d'une carte, l'assemblage du terrain et la
  reproductibilité par graine.
- Les chemins principaux, embranchements, impasses utiles et points remarquables.
- La relation entre salles, obstacles, lignes de vue et possibilités de combat.
- Les familles disponibles par zone, les groupes, les élites et les points
  d'apparition particuliers.
- Le lien entre parcours, renouvellement des ennemis, récompenses et progression.

Les références en temps réel devront ensuite être adaptées à notre règle
**un tour = une action**. L'étude n'appliquera aucune modification au jeu.

**Livrable :** analyse illustrée, éléments utiles à reprendre comme méthode,
limites pour notre jeu et questions de conception à trancher.

**Critère de sortie :** comprendre les principes qui nous intéressent et pouvoir
justifier nos propres choix de génération.

## PM02 — Définir la structure de notre monde

**But :** fixer l'articulation de la campagne avant d'écrire le générateur.

Proposer l'organisation des actes, secteurs et cartes en partant du récit.
Le découpage en six actes du manuscrit demeure une base proposée ; un acte
peut contenir plusieurs cartes et ne correspond pas automatiquement à une
ancienne profondeur du prototype.

Préciser le rôle des camps, les accès entre secteurs, les revisites, la mémoire
des cartes et leur éventuelle régénération. Le principe déjà retenu reste le
voyage rapide entre camps débloqués ; déblocage, accès à la commande, conditions
et coûts éventuels seront présentés séparément. Conserver les excursions
facultatives et leur retour dans la réflexion sur les connexions.

Choisir la place des morceaux de carte dessinés à la main et des parties
générées. Définir les objectifs de circulation, les espaces requis par les
combats et les points narratifs indispensables. Les décors définitifs pourront
être produits après vérification de ces besoins.

**Livrable :** schéma de campagne et cahier de génération, avec décisions
validées et questions encore ouvertes clairement distinguées.

**Critère de sortie :** chaque type de carte a un rôle, des entrées et sorties
compréhensibles et une place dans la progression.

## PM03 — Prototyper le premier générateur

**But :** produire une zone intéressante à parcourir, avec des décors simples.

Construire le terrain et ses connexions sur les cases carrées de 64 px.
Ajouter un outil de lecture des graines et des points d'intérêt, en réutilisant
l'éditeur et les diagnostics existants lorsque cela convient. Préparer les
emplacements des accès, objectifs, groupes et récompenses.

Vérifier la connectivité, l'accessibilité des objectifs, les chemins de retour,
les placements légaux et la reproduction exacte d'une graine. Mesurer les
distances, l'espace disponible et les impasses plutôt que de fixer d'emblée
un nombre arbitraire de grandes cartes.

**Livrable :** premier type de zone reproductible et ensemble de graines de test.

**Critère de sortie :** aucune graine testée ne bloque la progression ; la carte
offre des choix de trajet et des espaces de combat lisibles.

## PM04 — Vérifier les rencontres et quelques compétences

**But :** vérifier que le terrain convient réellement aux combats.

Placer un petit échantillon représentatif du bestiaire : contact, tir, soutien
ou lancement de sort et menace plus résistante. Proposer leurs compétences
minimales et leurs chiffres avant intégration. Tester les approches de mêlée,
à distance et magiques du joueur sur ces rencontres.

Définir comment une famille occupe une zone, comment les groupes se forment,
ce qui provoque leur engagement et comment le joueur lit leur danger. Vérifier
les lignes de tir, les renforts éventuels et les possibilités de repli. Les
variantes de couleur devront correspondre à leurs capacités une fois celles-ci
validées. Le bonus « Rapide » reste exclu.

**Livrable :** première bibliothèque de rencontres et besoins concrets du terrain.

**Critère de sortie :** les combats ont des différences tactiques compréhensibles
et ne reposent pas seulement sur une augmentation des points de vie.

## PM05 — Poser une première progression et une première difficulté

**But :** rendre la petite zone jouable avec une progression mesurable.

Proposer les premiers gains d'XP, besoins de niveau et récompenses, puis les
tester sur le parcours réel. Relier la puissance au monstre et au niveau de zone,
conformément au principe déjà accepté. Examiner ensemble PV, dégâts, défenses,
composition des groupes, ressources du joueur et équipement obtenu.

Vérifier la place des compétences, les consommables et le rythme d'amélioration
de l'équipement. Réutiliser les systèmes existants lorsqu'ils conviennent.
Présenter les modifications chiffrées avant de les activer. Les noms, les
apparences et les valeurs mécaniques gardent leurs validations distinctes.

**Livrable :** première courbe chiffrée et compte rendu de sessions courtes.

**Critère de sortie :** progression perceptible, combats compréhensibles et
ressources permettant de terminer le parcours avec les approches testées.

## PM06 — Boucler un premier secteur jouable

**But :** obtenir une petite expérience complète avant d'étendre le monde.

Relier le départ, le camp, une exploration générée, des rencontres, une
récompense et la sortie du secteur. Installer les services essentiels et un
fil de mission court. Vérifier le voyage entre camps sur des destinations de
test débloquées, avec les seules règles qui auront été acceptées.

Le premier secteur sert aussi à préciser les besoins de décors, de PNJ et de
dialogues. La réalisation complète des boss de secteur reste une étape dédiée ;
leur emplacement et leurs besoins d'espace peuvent déjà être réservés.

**Livrable :** première tranche jouable de bout en bout.

**Critère de sortie :** on peut créer une partie, jouer le parcours, progresser,
revenir au camp, sauvegarder et reprendre sans assistance technique.

## PM07 — Étendre le monde et répartir le bestiaire

**But :** développer les autres secteurs avec une identité propre.

Ajouter les cartes progressivement à partir du socle vérifié. Pour chaque
secteur, travailler ensemble terrain, décors, ambiance, camp, PNJ et objectifs.
Garder les quêtes nécessaires au rythme et au récit ; aucun quota par acte.

Répartir les familles : cultistes humains au début, possédés et conversions
intermédiaires, puis démons plus nombreux et plus répugnants. Les hybrides
mécaniques, insectoïdes, esprits, morts-vivants et autres silhouettes enrichissent
les rencontres selon le lieu. Prévoir des troupes ordinaires en quantité et une
place distincte pour les élites.

Vérifier la diversité des compositions, les tailles des créatures, leurs ombres
et leur lisibilité dans les décors réels. Le catalogue fournit des dessins ;
il ne fixe ni un quota de monstres par secteur ni leur sélection définitive.

**Livrable :** secteurs jouables, tables de rencontres et décors cohérents.

**Critère de sortie :** chaque secteur se distingue par sa géographie et ses
rencontres tout en respectant la progression d'ensemble.

## PM08 — Compléter les compétences et bonus des monstres

**But :** donner à chaque rôle les comportements nécessaires.

Compléter les sorts, invocations, attaques spéciales, états et contres, notamment
ceux des nécromanciens scientifiques et démoniaques. Présenter textes, effets,
chiffres, durées et signaux visuels avant leur activation.

Terminer le système de mots-clés fixes et aléatoires, sans doublon sur un monstre.
Leur puissance dépendra de leur porteur et de sa zone selon les règles retenues.
Vérifier les combinaisons, les plafonds et les exceptions ; conserver la lecture
dans Examiner. Proposer les noms de base et qualificatifs élémentaires
individuellement lorsque nécessaire. Cultistes humains et possédés gardent
leurs appellations communes déjà acceptées.

**Livrable :** fiches de compétences et de bonus validées, comportements testés.

**Critère de sortie :** les groupes utilisent leurs capacités de façon lisible,
avec des réponses possibles et des états correctement persistés.

## PM09 — Créer les boss de secteur

**But :** donner à chaque secteur un véritable chef et un combat propre.

Concevoir silhouette, taille, rôle narratif, arène, séquences d'attaque, signaux
d'anticipation, éventuelles phases et récompense. Un gros boss par secteur,
avec une taille propre pouvant dépasser largement 64 px. Les créatures déjà
classées comme élites ne deviennent pas automatiquement ces chefs.

Prototyper un boss à la fois et ajuster son arène si ses mécaniques l'exigent.
Vérifier les différentes approches du joueur, les invocations, les collisions,
les lignes de vue et les situations de blocage.

**Livrable :** combat et arène validés pour chaque boss retenu.

**Critère de sortie :** la victoire dépend d'une compréhension du combat, avec
une difficulté cohérente pour le niveau et l'équipement attendus.

## PM10 — Équilibrer la campagne complète

**But :** régler le rythme lorsque cartes, compétences et boss fonctionnent ensemble.

Rejouer les parcours, mesurer XP, temps de progression, dégâts subis, morts,
consommation de ressources, butin et écarts entre configurations du joueur.
Étudier aussi les revisites, le voyage rapide, les zones facultatives et les
possibilités de répétition des rencontres. Ajuster les règles après revue.

**Livrable :** bilan de progression et corrections chiffrées argumentées.

**Critère de sortie :** aucun passage obligatoire ne repose sur une configuration
particulière non annoncée ; les gains et risques restent cohérents sur le parcours.

## PM11 — Finitions visuelles, interface et performances

**But :** rendre la campagne agréable à lire et réactive.

Affiner les animations, impacts, états, corps, ombres, transitions et décors.
Vérifier les deux thèmes d'interface, les fenêtres de familiers, les infobulles
et les résolutions 1080p, 1440p et 4K. Corriger les textes après validation.
Profiler les lenteurs et les cartes chargées ; préserver les actions de jeu et
les sauvegardes pendant les optimisations.

**Livrable :** passe de lisibilité, cohérence graphique et performances mesurées.

**Critère de sortie :** informations lisibles, actions réactives et rendu cohérent
sur les scènes et résolutions représentatives.

## PM12 — Produire et intégrer les sons

**But :** construire l'identité sonore sur des combats déjà stabilisés.

Préparer auparavant les points de déclenchement utiles et, si nécessaire, des
sons provisoires pour tester un signal. Produire ensuite ambiances, attaques,
impacts, sorts, créatures, boss, interface et musique. Régler volumes, répétition
et priorités quand plusieurs événements surviennent ensemble.

Vérifier la cohérence des signaux avec les événements et états réellement perçus.
La propagation mécanique du bruit existante doit rester indépendante du volume
d'écoute choisi dans les options.

**Livrable :** bibliothèque sonore et mixage intégrés avec leurs réglages.

**Critère de sortie :** les sons soutiennent l'ambiance et la compréhension des
combats sans devenir fatigants ni divulguer des informations cachées.

## PM13 — Validation finale et version d'essai

**But :** vérifier une expérience complète reproductible.

Rejouer la campagne sur plusieurs graines et configurations. Vérifier création,
voyage, quêtes, boss, mort, victoire, sauvegardes, chargements et réglages.
Contrôler les performances, les résolutions visées, les ressources réellement
embarquées et l'absence de dépendances aux dossiers temporaires. Garder une
liste de défauts avec leur impact avant de préparer une version de test.

**Livrable :** version candidate et bilan de vérification.

**Critère de sortie :** parcours réalisable, défauts bloquants corrigés et limites
connues explicitement consignées. Publication et déploiement restent des actions
distinctes à demander.

## Manière d'avancer

Pour chaque phase : examiner l'existant, préparer une proposition concrète,
faire valider les nouveaux éléments concernés, réaliser un petit ensemble
cohérent, tester puis conserver les décisions utiles. Une validation passée
n'est pas redemandée si son contenu reste identique.

Le chemin principal est **PM01 → PM02 → PM03 → PM04/PM05 → PM06 → PM07/PM08 →
PM09 → PM10 → PM11 → PM12 → PM13**. PM00 accompagne le départ ; sauvegardes,
tests, lisibilité et premières mesures de difficulté sont suivis à chaque phase.
Les contenus indépendants peuvent être préparés entre deux validations, sans
étendre toute la campagne avant la réussite du premier secteur.
