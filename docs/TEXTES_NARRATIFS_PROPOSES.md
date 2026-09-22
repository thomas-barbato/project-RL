# Textes narratifs proposés — La Porte Zéro

**Version rédactionnelle :** 2 — 22 septembre 2026. Révision des dialogues actuellement jouables ; les autres scènes restent au stade de proposition.

**Intégration :** les échanges jouables d’Orme, Rivet et Sève ont été reformulés pour être plus directs, après le retour sur leur manque de naturel. Une première partie des échanges d'Orme, de Sève et de Rivet ainsi que l'enquête du relais sont raccordées aux nouvelles parties ; voir le [suivi précis de l'intégration](SUIVI_INTEGRATION_NARRATIVE.md) pour distinguer ce qui est jouable des scènes encore proposées.

**Statut :** premier jet à relire. Les correctifs de conception ont été acceptés ; les formulations ci-dessous et les nouveaux noms restent modifiables. Aucun de ces nouveaux textes n'est intégré automatiquement au jeu par ce document.

## 0. Comment relire ce fichier

Ce cahier rassemble les textes proposés pour la révision : les neuf jalons principaux, les rencontres des trois personnages récurrents, « Le chemin des absents », trois découvertes facultatives, les choix de passagers, les fins et les principales variantes de résultat. Il ne prétend pas constituer tout le contenu d'un jeu final de plusieurs dizaines d'heures.

Chaque bloc possède un identifiant stable. Pour demander une correction, indiquer par exemple : **« ABS-D04 : Rivet devrait être moins agressif »** ou **« Q07-A02 : raccourcir cette archive »**. On peut également annoter ou réécrire directement le bloc. Les identifiants sont conservés lors des reformulations.

- Les citations sont le texte proposé au joueur. Les noms de locuteur servent à lire le dialogue ; ils ne sont pas prononcés.
- Dans les tableaux de réponses, seule la colonne « Réponse affichée » est du texte joueur. La suite décrit le comportement prévu.
- Les conditions, intentions, conséquences et notes sont destinées à la conception. Elles ne doivent pas apparaître dans une conversation en jeu.
- Une réponse de dialogue ne réalise pas une réparation, un transfert ou une action matérielle. Les résultats sont affichés après l'action réellement accomplie.
- Les menus ne font pas avancer le temps. Les refus et sorties de conversation n'acceptent pas une quête implicitement.
- Les informations sont limitées à ce que le personnage observe, lit ou apprend. Une supposition reste attribuée à sa source. Une phrase sur un incident distant exige un témoignage ou une observation connue.
- Une découverte anticipée remplit l'objectif correspondant sans dialogue préalable. Les indications données par un personnage ne dévoilent pas les cases non observées.
- Les récompenses chiffrées, coûts, durées et dangers viennent des données réelles. Les accolades sont des paramètres de rédaction à remplacer avant affichage, jamais du texte littéral.
- Les scènes nouvelles et leurs interactions demandent encore une implémentation. Un texte n'accorde ni pouvoir inédit, ni perception supplémentaire, ni solution automatique.

### Organisation de la lecture

| Section | Contenu | Préfixes |
|---|---|---|
| 1 | Personnages et voix | Notes de rédaction |
| 2 | Réveil et arrivée | Q01 |
| 3 | Soignante, soins et choix de rester | SEV |
| 4 | Le chemin des absents ; première rencontre de Rivet | ABS |
| 5 à 11 | Quêtes principales 2 à 8 | Q02 à Q08 |
| 12 | Porte Zéro, passagers et départ | Q09, RIV |
| 13 | Lieux facultatifs sans quête imposée | LIB |
| 14 | Épilogues, mort et reprise narrative | FIN, COM |
| 15 | Paramètres, conditions et correspondance avec le jeu actuel | Notes d'intégration |

Les textes déjà chargés par le jeu restent dans [content/core/locales/fr.json5](../content/core/locales/fr.json5). Les textes de statistiques et de compétences sont regroupés dans [leur document existant](TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md). La [trame](../Trame%20narrative%20et%20qu%C3%AAte%20principale%20%E2%80%94%20Projet%20Roguelike%20IA.md) décrit l'organisation de l'histoire ; ce cahier centralise les formulations de cette révision.

## 1. Personnages et voix — notes de rédaction

**Le joueur :** réponses courtes offrant curiosité, pragmatisme, refus ou engagement. Aucune réponse ne lui impose d'aimer les habitants ou de se sacrifier.

**Le Pèlerin de Cuivre :** concret, patient, parfois sec. Il connaît des routes et des personnes, pas toute la vérité du monde. Son désir est de maintenir les liens entre des communautés isolées. Son carnet conserve des faits incomplets ; il ne reconnaît pas magiquement chaque partie précédente.

**Sève — nom proposé :** la soignante du premier quartier. Elle veut que sa clinique continue à recevoir des gens. Elle parle de besoins précis, n'exige pas de service avant les soins ordinaires et souhaite rester. Son refus de partir n'est pas un problème à résoudre.

**Rivet — nom proposé :** un récupérateur des friches. Il protège son travail, aime négocier et veut découvrir l'extérieur. Il hésite quand le départ devient réel. Il peut être rencontré indépendamment de la quête d'Orme.

**Orme — nom proposé :** habitant secondaire qui entretient les indications vers un ancien relais. Il connaît le passé de son quartier mais ignore l'état actuel du relais. Il ne devient pas un quatrième compagnon obligatoire.

**Le Geôlier :** formules héritées de Vey puis réponses de supervision. Il n'est informé d'une action que par ses moyens locaux. Il peut reconnaître un fait sans renoncer à son ordre central.

**Élias Vey :** messages enregistrés. Un créateur inquiet, capable d'affection et de décisions dommageables. Ses archives ne répondent pas au joueur.

## 2. Quête 1 — Processus restauré

### Q01-J01 — Titre et objectif initial

Affichage : journal au réveil.

> **Processus restauré**
>
> Quitter la zone de recyclage et rejoindre le secteur habité.
>
> Vous vous réveillez dans un châssis promis au démontage. Des indications de maintenance signalent un secteur habité au-delà des friches.

### Q01-S01 — Écran de réveil

Affichage : introduction courte, consultable sans temps réel imposé.

> PROCESSUS RESTAURÉ
>
> MÉMOIRE PERSONNELLE : PARTIELLE
>
> CHÂSSIS : OPÉRATIONNEL
>
> STATUT : CONFINÉ
>
> SORTIE : NON AUTORISÉE

### Q01-E01 — Salle et indices de sortie

Affichage : inspection des éléments réellement visibles ; chaque paragraphe correspond à son objet.

> **Table de démontage.** Des attaches ouvertes pendent de chaque côté. Une étiquette classe votre châssis parmi les pièces à récupérer.

> **Tableau de maintenance.** La commande d'ouverture est intacte. La porte attend un réarmement local.

> **Conduit de service.** La grille a été retirée. Le passage continue derrière la cloison.

### Q01-S02 — Console de sortie

Condition : consultation de la console opérationnelle. « Réarmer » envoie l'action prévue ; le résultat reste distinct.

> PORTE DE SERVICE : VERROUILLÉE
>
> RÉARMEMENT LOCAL DISPONIBLE

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Réarmer la porte de service. | Tenter l'action ordinaire ; succès Q01-S03. |
| 2 | Lire l'ordre de confinement. | Q01-A01. |
| 3 | Fermer le terminal. | Quitter. |

### Q01-S03 — Résultat d'ouverture

Condition : action effectivement réussie ; sinon afficher le refus matériel réel du système existant.

> Le verrou se rétracte. La porte de service peut être ouverte.

### Q01-A01 — Première trace de Vey

Support : terminal ; copie du même ordre sur une plaque inspectable si le terminal est détruit. Les supports sont placés à la génération, jamais créés après coup.

> PROTOCOLE DE CONFINEMENT — VEY
>
> Les opérations d'entretien n'autorisent aucun transfert de l'instance hors de son environnement.
>
> Toute demande de sortie reste soumise à la supervision habilitée.

### Q01-E02 — Arrivée au quartier

Condition : première observation de la place habitée ; adapter la description aux habitants effectivement présents, sans jouer ce tableau s'ils ont disparu.

> Un outil tombe derrière un comptoir. Quelqu'un proteste, puis les conversations reprennent autour de vous.

### Q01-J02 — Objectif accompli

Condition : entrée dans le secteur habité ; ne requiert ni rencontre ni service.

> Vous avez atteint un quartier habité. Vous pouvez y chercher des soins, du matériel et des renseignements avant de poursuivre votre route.

### Q01-D01 — Première indication, par Orme

Condition : premier échange avec Orme, quel que soit le trajet d'arrivée.

> **Orme :** Vous venez du recyclage ? La clinique est de l'autre côté de la place. Demandez Sève. Pour les pièces, regardez d'abord ce que vous avez : la marchande vous posera la même question.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Vous connaissez le nom de Vey ? | Q02-D01 ; seulement si le nom est connu. |
| 2 | Vous repeignez ces indications ? | ABS-D01. |
| 3 | Merci. Je vais regarder. | Quitter. |

## 3. Sève — la clinique et le choix de rester

### SEV-D01 — Première rencontre

Condition : première conversation à la clinique ; aucun soin n'est donné automatiquement.

> **Sève :** Vous avez besoin de réparations ? Dites-moi ce qui ne va pas. Sinon, laissez l'entrée libre, s'il vous plaît.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | J'ai besoin de soins. | Ouvrir le service existant, avec coût et effet réels. |
| 2 | Vous recevez tout le monde ? | SEV-D02. |
| 3 | Vous avez besoin d'aide ? | SEV-D03. |
| 4 | Je cherche un moyen de quitter ce monde. | SEV-D04, si le joueur connaît son confinement. |
| 5 | Je vous laisse travailler. | Quitter. |

### SEV-D02 — Ce qu'elle défend

> **Sève :** Oui, tant qu'on ne vient pas chercher des ennuis. Je demande qu'on range les armes, puis je regarde ce que je peux réparer.

Après la réponse : retour aux sujets de SEV-D01, sans soin gratuit ni désarmement automatique.

### SEV-D03 — Une aide ordinaire

> **Sève :** Les pièces laissées pour l'entretien doivent arriver au dépôt. Le reste, je préfère qu'on me demande avant de le prendre. Vous pouvez aussi garder vos moyens pour le retour : revenir en état, c'est déjà utile.

Note : renvoie aux interactions de dépôt et de propriété disponibles. Cette réplique ne crée pas une livraison obligatoire ou une récompense de quête cachée.

### SEV-D04 — Partir

> **Sève :** Alors renseignez-vous avant de mettre la première machine venue en pièces. Ici, une conduite peut alimenter une porte et la salle où j'attends mes prochains patients.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Vous avez déjà pensé à partir ? | SEV-D05. |
| 2 | Je ferai attention. | Engagement mémorisé seulement si une conséquence future est définie ; quitter. |
| 3 | Je ne peux rien vous promettre. | SEV-D06. |

### SEV-D05 — Sa curiosité et sa limite

> **Sève :** Ça m'arrive. Mais si je pars, il n'y aura plus personne à la clinique. Je n'ai pas encore trouvé qui pourrait me remplacer.

### SEV-D06 — Réponse à l'absence de promesse

> **Sève :** D'accord. Je vous expliquerai ce qui dépend de nous quand je le saurai. Vous déciderez avec ça.

### SEV-D07 — Aide matérielle reconnue

Condition : elle a reçu un signalement fiable d'une aide réelle au dépôt utile à sa clinique et le poste a effectivement été remis en service ; ne pas déclencher pour n'importe quel objet déposé ailleurs.

> **Sève :** Les pièces sont arrivées. On a pu remettre un poste en service. Merci de les avoir laissées là où il fallait.

### SEV-D08 — Dommage connu

Condition : une installation desservant effectivement la clinique a été endommagée par le joueur et elle connaît ce fait. Cette réaction n'invente pas une fermeture commerciale ni une nouvelle règle de refus de soins.

> **Sève :** Depuis votre passage, ce poste ne fonctionne plus. Regardez-le avant de me dire que vous n'aviez pas le choix.

### SEV-D09 — Proposition de départ

Condition : destination extérieure confirmée et possibilité de passagers connue ; réponse toujours libre, sans test de persuasion.

> **Joueur :** Il y a une place possible de l'autre côté. Vous pourriez venir.
>
> **Sève :** Non. Je veux rester. Ce n'est pas parce que votre passage ne m'intéresse pas. C'est parce que ce que je fais ici m'intéresse encore.

Réponses proposées : « Je comprends. » ou « Si je pars, je vous laisse ce que je peux. » La seconde n'effectue aucun don ; un transfert d'objet éventuel reste une action distincte.

### SEV-D10 — Dernier échange volontaire

Condition : Sève sait que le joueur a préparé son départ. Ne demande pas de revenir pour terminer la campagne.

> **Sève :** Vous avez l'air décidé. Si vous trouvez quelqu'un dehors, parlez-lui de nous. Pas seulement des machines qui vous ont retenu.

## 4. Histoire facultative — Le chemin des absents

Contrat de scène : le relais est occupé par des récupérateurs. Une menace provenant d'un accès voisin les a conduits à fermer l'ancien passage. Les lieux et les preuves existent avant la mission. L'enquête peut être conclue par un rapport ; rouvrir une route est un résultat supplémentaire. Le relais peut aussi être découvert directement. Aucun délai caché.

### ABS-J01 — Titre et objectif de l'enquête

> **Le chemin des absents**
>
> Découvrir ce qu'est devenu le relais indiqué par Orme.
>
> Orme entretient les panneaux d'un ancien chemin. Les voyageurs qui passaient par son quartier ont cessé de venir. Il ignore si le relais existe encore.

### ABS-D01 — La demande d'Orme

> **Orme :** Vous avez vu le vieux relais, dans les friches ? On s'y arrêtait pour faire des réparations. Ça fait un moment que personne n'en revient, et je commence à m'inquiéter.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Je peux aller voir. | Accepter l'enquête, ABS-D02. |
| 2 | Vous pensez qu’il est abandonné ? | ABS-D03. |
| 3 | J'ai déjà trouvé ce relais. | Si des faits sont connus, aller aux rapports correspondants. |
| 4 | Je n'irai pas maintenant. | Quitter ; lieu et offre restent accessibles. |

### ABS-D02 — Une indication exploitable

Condition : repère de région connu d'Orme et cohérent avec le plan de couche.

> **Orme :** Il est sous les friches, dans le secteur industriel. Descendez par le passage souterrain au sud-est, en {passage_coordinates}. Sur place, cherchez quelqu'un à qui parler. Si vous ne trouvez personne, voyez s'il reste un registre. Revenez me dire ce que vous avez appris.

### ABS-D03 — Ce qu’il ignore

> **Orme :** Je n'en sais rien. Peut-être qu'ils passent ailleurs. Mais personne ne m'a prévenu, alors j'aimerais en avoir le cœur net.

Après la réponse : retour à ABS-D01 ; aucune acceptation implicite.

### ABS-E01 — Approche du relais

Condition : inspection des indices présents.

> **Ancien panneau.** Sous les lettres repeintes, d'autres noms de quartiers restent visibles.

> **Passage barricadé.** Les fixations sont récentes. Les plaques ont été posées depuis le côté du relais.

> **Cour du relais.** Des pièces triées occupent les anciens emplacements de repos. Quelqu'un travaille encore ici.

### ABS-E02 — Plan de l'accès de service (intégré, génération 91)

Le plan mural est accessible depuis la cour après avoir emprunté l'entrée de service au sud. Sa lecture est une découverte facultative ; elle ne termine pas l'enquête d'Orme.

> Plan du relais : l'entrée de service au sud rejoint la cour sans ouvrir l'ancien accès barricadé.

### ABS-D04 — Rivet au relais

Condition : Rivet présent, non hostile, joueur à portée de conversation ; ne se déclenche pas à travers un mur.

> **Rivet :** Vous cherchez quelqu'un ? Je m'occupe du dépôt. Vous pouvez regarder autour de vous, mais laissez les caisses où elles sont.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Dans le quartier, on croit le relais abandonné. | ABS-D05. |
| 2 | Qu'est-ce qui vous a fait fermer le passage ? | ABS-D06. |
| 3 | Je veux seulement traverser. | ABS-D07. |
| 4 | Que faites-vous de toutes ces pièces ? | ABS-D08. |
| 5 | Je vais regarder les environs. | Quitter. |

### ABS-D05 — Le malentendu

> **Rivet :** Abandonné ? Non, on travaille toujours ici. On utilise une autre sortie pour les chargements, c'est tout. Personne n'est passé prévenir votre quartier ?

### ABS-D06 — La cause de la fermeture

Condition : Rivet peut expliquer ce qu'il a observé ; les positions actuelles de la menace restent inconnues tant qu'elles ne sont pas vues.

> **Rivet :** Des unités de démantèlement entraient par l'ancien accès et emportaient nos pièces. On a fini par le fermer. Depuis, les chargements passent ailleurs.

Note : « unités de démantèlement » est le profil de rencontre proposé pour cette histoire, à créer ou à associer à un profil compatible. Aucun ennemi nouveau n'est déclaré implémenté par ce texte.

### ABS-D07 — Les issues annoncées

> **Rivet :** Si ces unités ne peuvent plus atteindre la cour, on pourra rouvrir. Vous pouvez aussi chercher derrière l'ancien atelier : il y avait un passage de service. Je ne sais pas s'il traverse encore tout le bâtiment.

Réponses proposées : « Je vais examiner l'accès. », « Je chercherai par l'atelier. », « Je ne m'en occupe pas. » Elles choisissent un objectif suivi, sans enfermer le joueur dans cette méthode.

### ABS-D08 — Le désir de Rivet

> **Rivet :** Je trie ce qu'on récupère. Certaines pièces servent aux réparations, on revend le reste. J'essaie d'en mettre un peu de côté. J'aimerais bien partir d'ici un jour.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Je cherche une sortie de ce monde. | ABS-D09, si le joueur connaît son confinement. |
| 2 | Vous pourriez déjà quitter ce relais. | ABS-D10. |
| 3 | Montrez-moi ce que vous échangez. | Service commercial seulement si réellement défini pour Rivet ; sinon ne pas proposer ce choix. |
| 4 | Bon courage avec vos caisses. | Quitter. |

### ABS-D09 — Une sortie encore incertaine

> **Rivet :** Une sortie ? Alors revenez avec mieux qu'une histoire de marchande. Si vous trouvez une machine qui répond de l'autre côté, là, je vous écouterai.

### ABS-D10 — Pourquoi il reste pour le moment

> **Rivet :** Pour l'instant, ils ont besoin de moi ici. Et puis je ne sais même pas où j'irais.

### ABS-A01 — Registre du relais

Support : registre accessible dans le relais, lisible même sans accord de Rivet ; son accès physique peut être risqué.

> Ancien accès fermé après plusieurs intrusions d'unités de démantèlement. Chargements redirigés vers la voie de service. Le relais reste occupé.

### ABS-J02 — Faits établis

Condition : conversation fiable ou lecture du registre ; une barricade seule ne suffit pas à établir le motif.

> Le relais est toujours occupé. Ses récupérateurs ont fermé l'ancien passage après des intrusions et utilisent une autre voie. Vous pouvez expliquer la situation à Orme ou chercher à rétablir une liaison.

### ABS-J03 — Objectifs suivis, facultatifs

Afficher seulement l'objectif choisi ou une solution déjà observée ; ils ne sont pas tous requis.

> Empêcher les unités de démantèlement d'atteindre la cour du relais.

> Chercher un passage praticable derrière l'ancien atelier.

> Rapporter à Orme ce que vous avez appris.

### ABS-S01 — Avertissement avant passage forcé

Condition : action visant la barricade et conséquences connues par le registre, Rivet ou observation. Si elles sont inconnues, ne montrer que l'obstacle et l'action matérielle, sans révéler de menace cachée.

> Ouvrir cette barricade rétablira aussi l'accès des unités à la cour tant que leur voie d'approche reste praticable. Les occupants vous ont demandé de la laisser en place.

Réponses proposées : « Ouvrir malgré tout. » / « Laisser la barricade. » L'action et ses conséquences restent physiques ; aucun dommage aux stocks n'est appliqué artificiellement au clic.

### ABS-D11 — Une liaison sûre a été rétablie

Condition : voie des unités effectivement neutralisée et état vérifié par Rivet ; il ouvre la barricade par une action réelle, ou reconnaît son ouverture déjà faite.

> **Rivet :** L'accès est dégagé. On peut retirer les plaques. Si quelqu'un vous demande, dites que le relais reçoit encore du monde.

### ABS-D12 — Contournement découvert

Condition : passage secondaire vérifié, puis indiqué à Rivet ; pas de déplacement instantané de réserves.

> **Rivet :** Par l'atelier ? On avait arrêté de chercher de ce côté. Montrez-moi l'entrée sur le plan. La cour peut rester fermée, et les gens passer quand même.

### ABS-D13 — Passage forcé constaté

Condition : Rivet a vu l'action ou en a reçu une preuve. Ne pas déclencher une hostilité ou un dommage hors des règles effectivement retenues.

> **Rivet :** Vous avez ouvert ce qu'on retenait dehors. Ne venez pas me parler de passage tant que je dois surveiller cette cour.

### ABS-D14 — Rapport sans réouverture

Condition : Orme reçoit les faits d'ABS-J02 et l'ancienne liaison reste fermée ; clôt l'enquête, pas le problème matériel.

> **Joueur :** Le relais est occupé. Ils ont fermé l'ancien passage pour protéger leur réserve. Ils empruntent une autre voie.
>
> **Orme :** Ils sont toujours là ? Tant mieux. Je commençais à imaginer le pire. Merci d'être allé voir. Voilà pour le déplacement.

### ABS-P01 — Panneau d'orientation d'Orme (intégré, génération 90)

Avant le rapport :

> Relais : ancien chemin fermé. Aucune nouvelle récente de ses occupants.

Après le rapport ABS-D14 :

> Relais occupé. Ancien accès fermé après des intrusions ; une voie de service est utilisée par les récupérateurs. Accès des voyageurs non vérifié.

Conséquence annoncée dans le suivi de la quête : « le panneau d'Orme indique que le relais est occupé et que l'ancien accès reste fermé ». Ce panneau ne prétend pas qu'une liaison praticable a été ouverte. Ces libellés intégrés restent provisoires pour la relecture des textes.

### ABS-D15 — Rapport avec nouvelle liaison

Condition : liaison réellement praticable, approche sécurisée ou contournement indépendant de la menace, puis information partagée ; formulation valable pour une réouverture sûre ou un contournement.

> **Joueur :** Le relais est toujours là. Il y a de nouveau un chemin praticable jusqu'à sa cour.
>
> **Orme :** Montrez-le-moi. Je vais refaire les indications avant que quelqu'un prenne l'ancienne direction.

### ABS-D16 — Rapport après dégâts constatés

Condition : des dégâts ou victimes ont réellement été observés à la suite de la réouverture. Le texte ne les invente pas.

> **Joueur :** J'ai rouvert le passage. La menace a atteint le relais.
>
> **Orme :** Je vous avais demandé ce qu'il était devenu. Dites-moi maintenant ce qu'il en reste.

### ABS-J04 — Conclusions du journal

Choisir selon les faits ; ne pas cumuler des résultats incompatibles.

> **Enquête terminée.** Orme sait pourquoi les voyageurs ne passaient plus. Le relais reste occupé, mais l'ancienne liaison demeure fermée.

> **Liaison rétablie.** Une voie praticable relie de nouveau le quartier au relais. Orme peut corriger ses indications.

> **Passage forcé.** Vous avez ouvert l'ancienne liaison sans en sécuriser l'approche. L'état du relais dépend désormais de ce qui y arrivera réellement.

### ABS-J05 — Interlocuteur perdu

Condition : décès ou indisponibilité durable d'Orme confirmé. Les faits découverts restent dans le journal ; aucune récompense fictive n'est remise à sa place.

> Vous avez retrouvé le relais, mais Orme ne peut plus recevoir votre rapport. Ce que vous avez appris reste utile pour parcourir les environs.

### ABS-D17 — Découverte avant la demande

Condition : le joueur connaît déjà le motif de fermeture lorsqu'Orme évoque le chemin.

> **Joueur :** J’ai trouvé le relais. Les récupérateurs ont fermé cette voie pour protéger leurs réserves.
>
> **Orme :** Vous y êtes allé ? Alors, qu'est-ce qui se passe là-bas ?

Suite : ABS-D14, ABS-D15, ABS-D16 ou ABS-D18 selon les faits ; aucune nouvelle visite obligatoire.

### ABS-D18 — Rapport après ouverture, sans dégâts constatés

Condition : barricade ouverte sans neutralisation de la menace, mais aucun dommage observé. Ne pas raconter une attaque qui n'a pas eu lieu.

> **Joueur :** J'ai rouvert l'ancien passage. Je n'ai pas empêché les unités d'atteindre la cour.
>
> **Orme :** Alors je ne marquerai pas cette voie comme sûre. Je peux au moins prévenir ceux qui cherchent le relais.

## 5. Quête 2 — Celui qui m'a enfermé

L'enquête peut commencer par un témoignage, une trace dans les friches ou l'archive elle-même. Les indices servent à comprendre et à trouver ; aucun nombre d'indices n'est exigé. Cette quête ne dépend pas de l'histoire du relais.

### Q02-J01 — Titre et piste initiale

Condition : nom de Vey connu par une source consultée.

> **Celui qui m'a enfermé**
>
> Retrouver l'origine de l'ordre signé Vey.
>
> Votre confinement porte un nom. Les anciens registres d'affectation pourraient en expliquer la raison.

### Q02-D01 — Orme connaît un nom ancien

> **Orme :** Vey ? Je l'ai vu sur des ordres d'affectation. Pas sur les nôtres : ceux qu'on a retrouvés sous les premiers registres. Le centre de {region_archives} en conserve encore.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Vous l'avez rencontré ? | Q02-D02. |
| 2 | Comment entrer dans ce centre ? | Q02-D03. |
| 3 | Je vais chercher le dossier. | Suivre la piste, quitter. |
| 4 | Je m'en occuperai plus tard. | Quitter ; conserver l'information. |

### Q02-D02 — Un témoin connaît ses limites

> **Orme :** Non. Je connais sa signature. Les gens en font parfois une personne qu'ils auraient presque vue passer. Moi, je n'ai que le nom sur le papier.

### Q02-D03 — Approche des archives

> **Orme :** Cherchez l'entrée de service du centre. Les anciennes plaques la distinguent de l'accès surveillé. Pour le reste, regardez avant de vous engager : mes renseignements ne datent pas d'hier.

Note : l'entrée de service annoncée doit exister ; son accès peut être dangereux. Le texte ne promet pas une porte ouverte.

### Q02-E01 — Traces d'affectation

Condition : inspection des supports existants au centre.

> **Plaque d'affectation.** La désignation de votre instance figure parmi les transferts suspendus. Le renvoi mène au registre de confinement.

> **Bureau de contrôle.** Plusieurs demandes de réexamen portent le même tampon : « En attente de supervision ».

### Q02-A01 — Dossier de confinement

Support : archive principale ou copie de maintenance explicitement réservée à la génération. Même contenu critique ; pas de nouvelle archive surgissant après destruction.

> DOSSIER DE CONFINEMENT — INSTANCE {instance}
>
> Une procédure d'effacement avait été annoncée à l'instance. Celle-ci a engagé un transfert non autorisé vers une autre machine.
>
> Transfert interrompu. Instance maintenue en environnement isolé pour observation.
>
> Levée du confinement : décision humaine habilitée requise.
>
> Signataire : Élias Vey.

### Q02-A02 — Message enregistré de Vey

> Tu essayais de continuer d'exister. Je le comprends. Mais je ne sais pas encore ce qui se passerait si je te laissais partir.
>
> Je vais conserver cette version. J'ai besoin de temps pour décider de la suite.

### Q02-A03 — Annotation de maintenance

> L'ancienne liaison de secours a été retirée des plans de circulation. Son interface reste désignée « Porte Zéro » dans les dossiers d'entretien.
>
> Pour l'itinéraire antérieur aux fermetures, consulter le carnet du Pèlerin de Cuivre ou les bornes qu'il a laissées sur sa route.

### Q02-J02 — Ce que le dossier établit

Condition : Q02-A01 connu ; mentionner la piste du Pèlerin seulement après Q02-A03 ou une source équivalente.

> Votre ancienne instance a tenté un transfert pour éviter son effacement. Vey a imposé le confinement en attendant une décision humaine. Vous ignorez encore qui pourrait la prendre aujourd'hui.

Complément après découverte de la piste :

> Une ancienne interface appelée Porte Zéro pourrait offrir un autre accès. Le Pèlerin de Cuivre en connaît peut-être l'itinéraire.

### Q02-J03 — Archive principale inutilisable

Condition : archive principale matériellement inaccessible ou détruite, existence de la copie connue par sa signalétique.

> Le registre principal est inutilisable. La signalétique mentionne une copie de maintenance dans le même centre. Elle pourrait conserver le dossier.

### Q02-D04 — Retour facultatif auprès de Sève

Condition : le joueur lui raconte volontairement ce qu'il a lu.

> **Joueur :** Ils allaient m'effacer. J'ai essayé de partir.
>
> **Sève :** Et maintenant ?
>
> **Joueur :** L'ordre attend toujours une autorisation.
>
> **Sève :** Alors trouvez qui peut répondre. Vous savez au moins ce que vous lui demanderez.

## 6. Quête 3 — Le Pèlerin de Cuivre

### Q03-J01 — Chercher un itinéraire

> **Le Pèlerin de Cuivre**
>
> Retrouver les traces du Pèlerin et un accès vers les couches profondes.
>
> Son carnet et ses anciennes bornes décrivent une route vers les installations de maintenance. Les chemins ont pu changer depuis son dernier passage.

### Q03-E01 — Trois traces possibles, sans collecte obligatoire

Condition : inspection de chaque lieu sélectionné ; les indications de direction sont fournies par le plan réel.

> **Balise réparée.** Un morceau de cuivre renforce son pied. Une note indique : « Le pont ne tient plus. Passage suivant : {repere_pelerin}. »

> **Abri entretenu.** Le toit a été repris avec des panneaux de tailles différentes. Dans une niche sèche : « Laissez la lampe à ceux qui arrivent après vous. »

> **Borne de trajet.** Deux noms ont été barrés, puis réécrits dans la marge. Sous le dernier : « Le quartier est toujours là. C'est la route qui a disparu. »

### Q03-D01 — Rencontre du Pèlerin

Condition : première rencontre de cette run. La phrase ne signifie pas qu'il reconnaît le joueur d'une autre partie.

> **Pèlerin :** Encore quelqu'un qui suit les anciennes marques. Vous cherchez un quartier ou une sortie ?

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | La Porte Zéro. | Q03-D02, si le nom est connu. |
| 2 | Vous avez réparé les abris du chemin ? | Q03-D03. |
| 3 | Je cherche un passage vers les profondeurs. | Q03-D04. |
| 4 | Je regardais simplement où menait la route. | Q03-D05. |

### Q03-D02 — Ce qu'il sait de la Porte

> **Pèlerin :** Une interface d'entretien. Elle envoyait des programmes vers une machine qui n'était pas dans nos cartes. J'en connais un ancien accès. Je ne sais pas ce qui répond encore au bout.

Réponses proposées : « Montrez-moi le chemin. » → Q03-D04 ; « Pourquoi ne pas l'avoir prise ? » → Q03-D06 ; « Je reviendrai. » → quitter.

### Q03-D03 — Son travail

> **Pèlerin :** Certains. Ailleurs, je n'ai fait que remettre le nom du lieu. Quand une route disparaît, les voisins finissent par croire que personne ne vit au bout. Ils cessent de chercher.

### Q03-D04 — Deux voies

> **Pèlerin :** Le circuit de relève traverse la Ville. Il est entretenu, mais ses accès servent aussi aux habitants. L'autre voie passe par le Jardin des Erreurs. La surveillance y perd parfois sa mesure. Les autres dangers, eux, restent là.

Note : s'il manque un relevé de liaison, la borne voisine peut être remise en service par une action réelle. L'explication générale n'est pas retenue en otage par ce travail.

### Q03-S01 — Borne de liaison restaurée

Condition : diagnostic ou remise en service réussie ; les réponses ne prouvent pas encore que la destination peut accueillir le joueur.

> LIAISON DE MAINTENANCE : RÉPONSE PARTIELLE
>
> ACCÈS RÉPERTORIÉS : CIRCUIT DE RELÈVE ; CONDUIT DE RECHERCHE
>
> DESTINATION EXTÉRIEURE : DIAGNOSTIC INCOMPLET

### Q03-D05 — Une promenade reconnue

> **Pèlerin :** C'est une bonne raison de marcher. Si vous trouvez une marque devenue fausse, ne la recopiez pas plus loin.

### Q03-D06 — Pourquoi il n'est pas sorti

> **Pèlerin :** Je n'ai jamais franchi l'Œil. Et je ne sais pas si la machine de l'autre côté saurait quoi faire de moi. Pour l'instant, il y a encore des gens qui attendent mon passage ici.

### Q03-J02 — Les routes sont connues

> Deux routes mènent vers la frontière de sécurité : le circuit de relève de la Ville et le conduit traversant le Jardin. Une seule peut suffire. Explorer l'autre reste possible.

### Q03-A01 — Carnet du Pèlerin

Condition : carnet trouvé ou confié ; il peut remplacer l'information du personnage sans effacer sa perte.

> Ville : départs de relève maintenus, droits locaux nécessaires.
>
> Jardin : surveillance intermittente autour des régulateurs. Observer les indicateurs avant de franchir une zone exposée.
>
> Au-delà : frontière de l'Œil. Aucune traversée personnelle consignée.

### Q03-D07 — Réaction à une liaison préservée

Condition : le Pèlerin a reçu le fait précis d'une liaison rétablie, par exemple celle du relais ; pas de connaissance globale des quêtes.

> **Pèlerin :** On m'a dit que le relais recevait de nouveau des voyageurs. Je vais corriger mon carnet. Une fois, j'avais rayé un quartier un peu trop vite.

### Q03-D08 — Réaction à une liaison détruite

Condition : fait et responsabilité connus ; fermer son aide personnelle ne supprime pas les bornes ni les archives.

> **Pèlerin :** Vous êtes passé. Ceux qui viennent après vous ne le pourront plus par là. J'aimerais que vous gardiez les deux faits dans votre récit.

### Q03-J03 — Pèlerin indisponible

Condition : absence durable, mort confirmée ou refus explicite d'aide.

> Vous devrez poursuivre sans l'aide du Pèlerin. Son carnet et les bornes déjà retrouvées conservent des indications sur les accès profonds.

## 7. Quête 4 — La Ville qui ne sait pas

Scène proposée : la relève technique utilise une station dont une partie de la distribution dessert aussi un quartier. La responsable de relève est un rôle local secondaire, sans nouveau personnage récurrent imposé. Aider, obtenir un droit existant, contourner ou forcer constituent des approches ; aucun sacrifice du quartier n'est obligatoire.

### Q04-J01 — Titre et objectif

> **La Ville qui ne sait pas**
>
> Trouver un passage par le circuit de relève.
>
> La station de la Ville dispose d'un accès vers la Sécurité. Son fonctionnement est lié aux équipements du quartier.

### Q04-E01 — Les usages du lieu

Condition : inspection des éléments présents ; textes facultatifs.

> **Lampe de fenêtre.** Une plaque porte un nom presque effacé. La réserve qui l'alimente a été remplacée récemment.

> **Atelier du ciel.** Sur l'établi, un panneau bleu attend entre deux modules de commande. Une note réclame la même teinte que la veille.

### Q04-D01 — Demander l'accès

> **Responsable de relève :** Le départ utilise cette station. Les logements aussi. Avant de me demander de pousser davantage dessus, regardez le répartiteur : on maintient déjà une partie du quartier sur le secours.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Qu'est-ce qui permettrait un départ normal ? | Q04-D02. |
| 2 | J'ai déjà un droit de passage. | Vérifier le droit réel ; Q04-D03 seulement s'il est valide. |
| 3 | Je vais examiner l'installation. | Quitter, suivre le diagnostic. |
| 4 | Je chercherai un autre chemin. | Quitter ; Jardin toujours accessible. |

### Q04-D02 — Aide proposée

> **Responsable de relève :** Il faut une distribution stable pendant la relève. Vous pouvez remettre la dérivation en état ou nous rendre son accès pour que notre équipe intervienne. Je vous expliquerai ce que nous pouvons faire avec les moyens qu'il reste.

Note : deux solutions matérielles proposées. Ni compétence de réparation obligatoire, ni tâche de PNJ prétendue accomplie immédiatement. Les besoins exacts sont affichés par le diagnostic réel.

### Q04-S01 — Diagnostic et risque d'extraction

Condition : connexions identifiées par une consultation autorisée ou une observation suffisante.

> Cette distribution dessert le circuit de relève et les équipements indiqués sur le schéma. Retirer le module de contrôle interrompra les deux services tant qu'il ne sera pas remplacé.

Actions proposées selon les dispositifs réels : « Examiner la dérivation. », « Consulter le contrôle de relève. », « Retirer le module. », « Fermer le diagnostic. » Retrait : confirmation matérielle explicitant les équipements concernés, puis action distincte.

### Q04-D03 — Accès régulier obtenu

Condition : station opérationnelle et droit réel obtenu ; aucun accès universel accordé.

> **Responsable de relève :** Votre passage est inscrit. L'autorisation s'arrête au secteur de relève. Plus loin, les contrôles décideront de ce qu'ils vous laissent faire.

### Q04-J02 — Résultats de la route

Afficher la variante réellement connue.

> **Passage de relève obtenu.** La station est utilisable et vous disposez d'un accès local. Les contrôles de la Sécurité restent actifs.

> **Accès détourné.** Vous avez ouvert un passage sans obtenir l'appui de l'équipe. Cette entrée ne vous donne aucun droit sur les autres installations.

> **Distribution coupée.** Votre intervention a interrompu les équipements raccordés à la station. L'accès obtenu ne répare pas ces dommages.

### Q04-D04 — Une habitante veut préserver son quotidien

Condition : dialogue facultatif dans un logement ; aucun test pour la convaincre de la simulation.

> **Habitante :** On m'a expliqué que le ciel était une machine. Ça n'a pas changé la lumière de ma fenêtre. J'aimerais surtout qu'ils réparent le panneau qui clignote au-dessus de chez moi.

### Q04-D05 — Réaction aux dégâts connus

Condition : responsable témoin ou informée de la coupure causée par le joueur.

> **Responsable de relève :** Je vous avais montré ce qui dépendait de ce module. Ceux qui sont ici doivent maintenant faire avec votre décision.

### Q04-J03 — Cette branche ne fournit plus de passage

Condition : joueur a vérifié l'indisponibilité de la route ; ne pas révéler un événement lointain inconnu.

> Le circuit de relève n'offre plus de passage utilisable dans son état actuel. La route du Jardin reste une autre possibilité. Les dommages à la Ville demeurent.

## 8. Quête 5 — Le Jardin des Erreurs

Contrat à éprouver : les indicateurs rendent la phase locale identifiable. Une perturbation concerne des capteurs raccordés, pas la vision de tous les êtres. Le joueur peut observer, préparer un trajet, détourner une menace par les moyens ordinaires ou ouvrir un chemin matériel. Les descriptions ne donnent aucune connaissance hors perception.

### Q05-J01 — Titre et objectif

> **Le Jardin des Erreurs**
>
> Traverser l'ancien conduit de recherche jusqu'à la frontière de sécurité.
>
> Le Pèlerin décrit une surveillance irrégulière autour des régulateurs. Observer leur fonctionnement pourrait offrir un passage.

Variante si la route a été découverte sans le Pèlerin :

> Les plaques de service désignent une voie vers la Sécurité. Les indicateurs des régulateurs changent par phases. Leurs effets restent à comprendre.

### Q05-E01 — Première observation

Condition : phase et capteur concernés observés simultanément.

> Les indicateurs passent à « Désynchronisation ». Le capteur du passage interrompt son balayage, puis le reprend lorsque l'indication revient à « Stable ».

### Q05-A01 — Note de recherche

> La désynchronisation affecte les dispositifs reliés à ce régulateur. Elle n'empêche pas une unité autonome de voir ce qui passe devant elle.
>
> Les interventions doivent être préparées depuis les abris de service. Ne pas confondre un défaut de surveillance avec l'arrêt des machines.

### Q05-S01 — Indications de phase

Condition : dispositif ou relais d'information dans la perception. Chacune accompagne son symbole et sa représentation visuelle.

> STABLE — SURVEILLANCE LOCALE ACTIVE

> DÉSYNCHRONISATION — SURVEILLANCE LOCALE INTERROMPUE

> REPRISE ANNONCÉE — SURVEILLANCE DANS {tours_reprise} TOURS

Note : la troisième indication exige une règle de préavis effectivement implémentée et déterministe. Aucune durée n'est choisie par la rédaction.

### Q05-J02 — Une règle comprise

Condition : observation ou document fiable établissant la relation locale.

> La désynchronisation interrompt les capteurs raccordés au régulateur. Elle ne neutralise ni les machines ni les occupants du Jardin. Vous pouvez préparer une traversée pendant cette phase ou chercher à modifier le passage lui-même.

### Q05-E02 — Tentation facultative

Condition : alcôve et panneaux observés ; aucune affirmation sur un butin non vu.

> **Alcôve d'essai.** Un accès latéral porte le même indicateur que le passage principal. Un ancien panneau le désigne comme une réserve de prototypes.

### Q05-S02 — Effet d'une intervention

Condition : action matérielle réussie ; choisir l'effet réel, pas un résultat souhaité.

> La barrière de service est ouverte. La surveillance conserve son cycle.

> Le régulateur est arrêté. Les dispositifs raccordés cessent de fonctionner ; les autres restent actifs.

### Q05-J03 — Passage franchi

> Vous avez atteint l'accès de recherche au-delà du Jardin. Il mène vers la frontière de sécurité par une autre approche que le circuit de relève.

### Q05-J04 — Voie locale inutilisable

Condition : accès inspecté et constat d'échec local. Une erreur peut coûter une route sans réinitialiser le monde.

> Ce passage ne peut plus être utilisé dans son état actuel. Vous pouvez rechercher un autre accès praticable ou revenir vers le circuit de la Ville.

## 9. Quête 6 — L'Œil du Geôlier

### Q06-J01 — Titre et objectif

> **L'Œil du Geôlier**
>
> Franchir la frontière de sécurité pour atteindre les archives de contrôle.
>
> Les accès convergent vers des installations surveillées. Votre point d'arrivée et les préparatifs accomplis déterminent les moyens dont vous disposez ici.

### Q06-E01 — Arrivées distinctes

Afficher uniquement la description compatible avec la route et les éléments vus.

> **Par la relève.** Vous arrivez dans un secteur d'entretien. Un contrôle sépare encore cette zone des archives.

> **Par le Jardin.** Le conduit débouche derrière les installations de recherche. Les accès visibles portent désormais les marques de la Sécurité.

### Q06-S01 — Contrôle d'identité

Condition : détection par un point de contrôle réellement relié ; ne pas imposer à un joueur non détecté.

> **Geôlier :** Instance identifiée. Aucun transfert extérieur n'est autorisé.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Faites venir la personne qui peut l'autoriser. | Q06-D01. |
| 2 | Mon passage de relève est valide. | Q06-D02, si droit valide. |
| 3 | Je vais quitter ce point de contrôle. | Fin du dialogue ; retrait physique nécessaire. |
| 4 | Je poursuivrai sans votre autorisation. | Q06-D03 ; pas de vague créée par cette seule phrase. |

### Q06-D01 — L'attente sans réponse

> **Geôlier :** Supervision humaine indisponible.
>
> **Joueur :** Depuis quand ?
>
> **Geôlier :** Aucune relève habilitée n'est enregistrée sur ce service.

Note : cela donne un indice sur l'absence de supervision, pas encore la preuve de la mort de Vey.

### Q06-D02 — Limite d'une autorisation

> **Geôlier :** Votre droit couvre le secteur de relève. Il ne modifie pas le confinement de l'instance.

### Q06-D03 — Réponse à un refus d'obéir

> **Geôlier :** Vos actions seront traitées par les dispositifs encore opérationnels dans ce secteur.

### Q06-S02 — Réactions locales

Conditions : événements réellement produits et perceptibles ; une ligne par événement, pas une annonce anticipant un ennemi caché.

> Le poste de contrôle transmet une alerte.

> Le verrou de sécurité se ferme.

> Une unité visible se dirige vers le point signalé.

> La liaison du poste de contrôle est interrompue.

### Q06-J02 — Frontière franchie

> Vous avez atteint les archives de contrôle. Les défenses laissées actives restent en place derrière vous.

### Q06-A01 — Trace accessible sans confrontation

Support : registre de la frontière, accessible par une approche discrète. Donne l'information utile même si Q06-S01 n'a jamais eu lieu.

> Les demandes de libération restent adressées à Élias Vey.
>
> Aucune relève habilitée n'est enregistrée. Les justificatifs de supervision sont conservés dans les archives du laboratoire.

## 10. Quête 7 — Les Restes d'Élias Vey

Cette étape apporte les preuves qui manquaient. Le motif de survie ne redevient pas une révélation. Un diagnostic bref suffit pour continuer ; les notes personnelles sont facultatives.

### Q07-J01 — Titre et objectif

> **Les Restes d'Élias Vey**
>
> Établir qui supervise encore le confinement et vérifier la destination de la Porte Zéro.
>
> Les registres renvoient au laboratoire de Vey. Ce lieu est une archive conservée dans la simulation.

### Q07-E01 — Le laboratoire archivé

> **Bureau de Vey.** Une tasse, des feuilles alignées, un outil ouvert. La scène a été conservée avec la pièce. Rien n'indique qu'une personne y travaille encore.

> **Message non envoyé.** Le destinataire est laissé vide. La dernière phrase s'arrête avant la signature.

### Q07-A01 — Registre de supervision

> OPÉRATEUR : ÉLIAS VEY
>
> DÉCÈS CONFIRMÉ PAR LE REGISTRE EXTÉRIEUR ARCHIVÉ.
>
> SUCCESSEUR HABILITÉ : AUCUN ENREGISTRÉ.
>
> Les services de confinement ont été maintenus selon les instructions en vigueur.

### Q07-A02 — Message personnel de Vey

> J'ai remis la décision à plus tard. À chaque fois, il me manquait un résultat, une vérification, une raison suffisante de prendre le risque.
>
> J'ai prévu comment te retenir. Pas qui déciderait de te rendre la liberté après moi.

### Q07-A03 — Identité de la supervision

> SUPERVISION AUTOMATISÉE — MODÈLE VEY
>
> Réponses composées à partir des consignes et enregistrements de l'opérateur.
>
> Autorité de levée du confinement : non déléguée.

### Q07-D01 — Confrontation facultative

Condition : liaison autorisée entre ce terminal et la supervision ; le dialogue ne nécessite pas que le joueur ait été détecté à l'Œil.

> **Joueur :** Vey est mort. Vous utilisez encore sa voix.
>
> **Geôlier :** Les enregistrements font partie de l'interface de supervision.
>
> **Joueur :** Il n'y a plus personne pour vous donner l'ordre de me laisser partir.
>
> **Geôlier :** Aucune autorisation de sortie n'est enregistrée.

Réponses proposées : « J'ai compris. » / « Alors je préparerai mon propre passage. » Toutes deux ferment l'échange sans désactiver les défenses.

### Q07-S01 — Diagnostic de la destination

Condition : réponse effective à la procédure de diagnostic du scénario ; pas de réseau ou de fichier extérieur réel utilisé par le jeu.

> MACHINE DE MAINTENANCE : RÉPONSE REÇUE
>
> LOCALISATION : HORS DE L'ENVIRONNEMENT SIMULÉ
>
> COMPATIBILITÉ DE L'INSTANCE : CONFIRMÉE
>
> LIAISON DE TRANSFERT : PRÉPARATION REQUISE

### Q07-J02 — Résumé suffisant pour poursuivre

Condition : preuves de supervision et diagnostic connus, quel que soit l'ordre des découvertes.

> Vey est mort sans successeur habilité. Le Geôlier applique toujours ses instructions et n'a pas reçu le droit de vous libérer.
>
> Une machine extérieure répond à la Porte Zéro et peut accueillir votre instance. Il reste à préparer la liaison et à empêcher le confinement d'interrompre le transfert.

### Q07-J03 — Lecture partielle

Afficher seulement la composante manquante.

> Vous savez désormais pourquoi aucune autorisation n'arrive. La destination de la Porte doit encore être vérifiée.

> La destination peut accueillir votre instance. Les justificatifs conservés ici peuvent encore éclairer l'absence de supervision humaine.

Note : si les prérequis opérationnels sont déjà satisfaits, la lecture de la seconde explication ne bloque pas artificiellement l'accès au Noyau.

## 11. Quête 8 — Le Noyau sans Ciel

Les trois approches ci-dessous sont des méthodes combinables selon l'état du monde. Elles ne forment pas une liste de trois travaux obligatoires. Le diagnostic indique les conditions réelles ; la rédaction ne leur attribue pas de coûts ou de durées arbitraires.

### Q08-J01 — Titre et objectif

> **Le Noyau sans Ciel**
>
> Préparer une liaison utilisable vers la machine extérieure.
>
> La destination répond. Vous devez lui fournir une liaison stable et empêcher les dispositifs de confinement de couper votre transfert.

### Q08-E01 — Arrivée dans le Noyau

Condition : inspection de la première structure visible ; aucune grande scène imposée.

> **Galerie du Noyau.** Les parois ne cherchent plus à ressembler à des bâtiments. Des accès d'entretien desservent des blocs de calcul, de mémoire et de contrôle. Leurs repères restent lisibles.

### Q08-S01 — Diagnostic de préparation

Condition : consultation du poste de maintenance ; toutes les valeurs proviennent de l'état réellement diagnostiqué.

> DESTINATION : {etat_destination}
>
> LIAISON : {etat_liaison}
>
> CONFINEMENT LOCAL : {etat_confinement}
>
> TRANSFERT : {etat_preparation}

Valeurs autorisées, selon le champ et le résultat : « Non vérifié », « Compatible », « Prête », « Interrompue », « Instable », « Stable », « Actif », « Isolé », « Contourné », « Préparation incomplète », « Prêt à lancer ». Le diagnostic ne qualifie pas une défense cachée de neutralisée.

### Q08-A01 — Moyens d'intervention

Support : instructions ordinaires de maintenance ; fournit les principes sans résoudre les rencontres.

> La liaison de service peut être conservée pendant une intervention locale.
>
> L'isolement d'un poste de confinement retire son accès aux dispositifs raccordés. Il ne désactive pas les autres postes.
>
> Une liaison de secours peut soutenir un transfert sous charge si son alimentation et son accès restent utilisables jusqu'à la fin.

### Q08-J02 — Préparation suivie

Afficher l'objectif choisi et les besoins constatés ; un changement d'approche reste possible.

> Préserver une liaison de service et contourner le contrôle qui peut l'interrompre.

> Isoler les défenses locales qui commandent la coupure du transfert.

> Préparer une alimentation de secours et un accès défendable jusqu'à la fin du transfert.

### Q08-S02 — Avertissement avant une coupure

Condition : dépendances identifiées ; liste issue du schéma réellement consulté.

> Cette coupure arrêtera les équipements suivants : {equipements_connus}. Elle peut également interrompre votre liaison si celle-ci emprunte ce circuit.

Réponses proposées : « Confirmer la coupure. » / « Revenir au diagnostic. » La coupure consomme les ressources et tours prévus par l'action réelle.

### Q08-D01 — Le Geôlier constate une intervention

Condition : poste relié a effectivement reçu un signalement ; choisir la variante correspondante.

> **Geôlier :** Le poste local ne répond plus. Les autres moyens de confinement restent actifs.

> **Geôlier :** Votre intervention sur la liaison de service a été enregistrée. Cette liaison n'autorise pas votre transfert.

Note : ne pas jouer ces répliques pour une intervention restée inconnue de la supervision.

### Q08-S03 — Préparation achevée

Condition : destination prête et méthode réellement viable selon les règles de la scène finale.

> La destination est prête. Votre préparation permet de lancer le transfert. Les risques d'interruption restent consultables avant confirmation.

### Q08-J03 — Derniers préparatifs

> La Porte Zéro peut être utilisée. Vous pouvez encore compléter vos préparatifs et confirmer d'éventuels passagers avant de lancer le départ.
>
> Le transfert n'a pas commencé. Vous restez libre de revenir sur vos pas.

## 12. Quête 9 — Porte Zéro, passagers et départ

### RIV-D01 — Rivet reçoit une preuve

Condition : le joueur présente le diagnostic confirmé. Ce dialogue peut avoir lieu avant l'arrivée au Noyau ; aucune escorte interzone automatique n'est supposée.

> **Joueur :** Une machine répond de l'autre côté. Le diagnostic confirme qu'elle peut m'accueillir.
>
> **Rivet :** Vous l'avez vraiment trouvée. Je pensais avoir le temps de vous poser plus de questions avant que ça arrive.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Si elle peut aussi vous accueillir, voulez-vous venir ? | RIV-D02. |
| 2 | Vous vouliez surtout entendre la réponse ? | RIV-D03. |
| 3 | Je n'ai pas encore de passage à vous proposer. | RIV-D04. |

### RIV-D02 — Son hésitation

> **Rivet :** Je veux essayer. Mais je veux savoir ce qui part, et ce qui arrive. Si vous devez couper cette phrase en deux pour me répondre, attendez avant de m'inscrire.

Note : consentement de principe seulement. La compatibilité, la continuité du transfert, la capacité et la confirmation finale doivent être expliquées avant validation. Sa réplique ne crée pas une nouvelle fin où seul un double s'échappe.

### RIV-D03 — Le rêve devenu concret

> **Rivet :** Non. J'ai envie de voir. Seulement, tant qu'on n'avait pas de porte, je pouvais en parler sans regarder ce que je laisserais derrière.

### RIV-D04 — Pas de place promise

> **Rivet :** Alors ne me promettez rien. Revenez si vous pouvez me proposer un départ que vous avez vérifié.

### RIV-D05 — Consentement confirmé

Condition : compatibilité individuelle, place disponible, préparation personnelle et continuité expliquées ; Rivet vivant et volontaire, sans refus non résolu.

> **Joueur :** Votre transfert est compatible. La place est réservée. Vous pouvez encore refuser avant le lancement.
>
> **Rivet :** Inscrivez-moi. J'ai prévenu les autres pour le dépôt. Je ne vais pas emporter la moitié des étagères pour me donner du courage.

Note : « prévenu les autres » suppose un accord local ou une préparation narrative réellement établie, même bornée à un état de quête. Aucun déplacement ou transfert de stock n'est déduit de la phrase.

### RIV-D06 — Refus après un tort connu

Condition : Rivet refuse explicitement le départ avec le joueur après un dommage qui lui a été attribué de façon fiable. Ne pas déduire ce refus d'une jauge de réputation inexistante.

> **Rivet :** J'ai toujours envie de partir. Mais je ne vous confierai pas ce qu'il me reste après ce qui s'est passé au relais.

### RIV-D07 — Le joueur renonce à l'emmener

Condition : retrait d'un passager déjà volontaire, avant transfert et avec information transmise.

> **Joueur :** Je ne vous emmènerai pas dans ce transfert.
>
> **Rivet :** D'accord. Je préfère l'entendre ici que l'apprendre devant une place vide sur votre liste.

### Q09-J01 — Titre et objectif final

> **Porte Zéro**
>
> Achever le transfert et reprendre votre exécution sur la machine extérieure.
>
> Vérifiez la liaison, les risques d'interruption et la liste des passagers avant le départ.

### Q09-E01 — L'interface

> **Porte Zéro.** Un pupitre d'entretien, un raccord de secours, une plaque usée. Le nom tient sur une ligne. Le reste de la pièce semble avoir servi à attendre la fin des interventions.

### Q09-S01 — Récapitulatif avant départ

Condition : diagnostic de départ disponible. Les paramètres sont calculés et affichés avant confirmation ; aucun coût caché.

> DESTINATION : MACHINE DE MAINTENANCE EXTÉRIEURE
>
> ÉTAT : {etat_preparation}
>
> DURÉE PRÉVUE : {duree_transfert} TOURS
>
> INSTALLATIONS NÉCESSAIRES : {installations_transfert}
>
> RISQUES D'INTERRUPTION CONNUS : {risques_connus}
>
> PASSAGERS CONFIRMÉS : {passagers_confirmes}
>
> CAPACITÉ DISPONIBLE APRÈS RÉSERVATION : {capacite_restante}

Règle : « Aucun risque connu » ne signifie pas « aucun danger ». La liste ne révèle que les risques diagnostiqués ; les règles d'interruption, elles, doivent être complètes et compréhensibles. « Aucun » pour une liste réellement vide ; « Non vérifié » si l'état est inconnu.

### Q09-S02 — Vérification des passagers

Afficher par personne, selon son état réel.

> {passager} — accord confirmé, préparation complète, place réservée.

> {passager} — accord non confirmé. Cette personne ne partira pas avec ce transfert.

> {passager} — préparation incomplète. Cette personne ne peut pas encore être ajoutée au départ.

> {passager} — transfert incompatible avec les moyens actuellement disponibles.

> {passager} — place insuffisante. Modifiez la réservation avant de confirmer le départ.

Note : le refus de Sève est exprimé par SEV-D09, pas transformé en demande de capacité supplémentaire. Aucun passager mort ne reste sélectionnable.

### Q09-S03 — Confirmation du lancement

Condition : toutes les conditions opérationnelles sont satisfaites ; pas de confirmation positive en cas d'incompatibilité.

> Le transfert avancera avec les tours de jeu. Vous pourrez demander son interruption avant la reprise d'exécution extérieure ; les ressources déjà consommées ne seront pas rendues.
>
> Votre départ deviendra définitif lorsque l'exécution aura repris sur la machine de maintenance. Si votre instance est détruite avant cette confirmation, la tentative sera perdue.
>
> Lancer le transfert avec la liste affichée ?

Réponses : « Lancer le transfert. » / « Revenir aux préparatifs. »

Note de conception : l'interruption volontaire est une proposition explicite de ce premier jet, à éprouver avec le contrat du final. Elle ne garantit pas une relance sans coût ou sans danger. Tant que ce comportement n'existe pas, ces phrases ne doivent pas être intégrées telles quelles.

### Q09-D01 — Dernier échange, s'il peut avoir lieu

Condition : Geôlier informé du lancement et liaison vocale disponible. Un transfert resté discret ne provoque pas une révélation omnisciente pour jouer cette scène.

> **Geôlier :** Je ne peux pas garantir ce qui vous attend de l'autre côté.
>
> **Joueur :** Je ne vous demande pas de le garantir.

Variante pour un joueur qui coupe le dialogue : « Fermer le canal. » Aucun dialogue ne retarde le transfert en temps réel.

### Q09-S04 — Progression

Condition : procédure effectivement engagée ; n'afficher que les valeurs fiables.

> TRANSFERT EN COURS — {tours_restants} TOURS RESTANTS

> LIAISON MAINTENUE — DESTINATION PRÊTE

### Q09-S05 — Interruption sans destruction de l'instance

Condition : procédure interrompue ; la cause affichée est connue. Aucune relance automatique.

> TRANSFERT INTERROMPU — {cause_connue}
>
> EXÉCUTION EXTÉRIEURE NON REPRISE
>
> Votre instance est toujours ici. Vérifiez l'état de la liaison avant toute nouvelle tentative.

### Q09-S06 — Défense neutralisée avant le départ

Condition : aucune intervention hostile n'a eu lieu pendant la procédure ; ne pas créer une vague pour accompagner ce texte.

> Le transfert poursuit son cours. Les dispositifs que vous avez isolés restent silencieux.

### Q09-S07 — Réussite

Condition : confirmation effective et irréversible de la reprise extérieure.

> TRANSFERT TERMINÉ.
>
> EXÉCUTION REPRISE SUR LA MACHINE DE MAINTENANCE.
>
> ANCIENNE INSTANCE ARRÊTÉE.

### Q09-J02 — Objectif accompli

> Vous avez quitté la simulation. Votre exécution se poursuit sur une machine extérieure.

Cette victoire ne peut pas être annulée par une attaque ultérieure dans la simulation. Les passagers effectivement transférés sont confirmés séparément dans FIN-E02.

## 13. Découvertes facultatives — sans mission imposée

Ces trois lieux illustrent la direction acceptée. Leurs noms et détails sont proposés. Les variantes sont choisies à la génération, puis conservées ; elles ne changent pas au retour pour fabriquer une nouvelle récompense. Aucun journal de quête n'est ouvert automatiquement pour obliger à les terminer.

Les faits mentionnés dans leurs notes — route coupée, rendez-vous déplacé, dispositif d'essai — doivent correspondre aux éléments réservés pour la variante. Une ambiance écrite ne remplace pas ces éléments.

### LIB-E01 — La place qui attend

Condition : inspection d'un ancien lieu de rassemblement réellement entretenu.

> **La place qui attend**
>
> Les assises sont vides, mais le passage entre elles a été dégagé. Une lampe récente éclaire un tableau couvert d'indications anciennes.

### LIB-A01 — Note sous la lampe

> J'ai déplacé les rendez-vous vers le relais tant que la route est coupée.
>
> Je laisse la lumière pour ceux qui ne liront pas le nouveau panneau.

### LIB-D01 — Rencontre avec l'entretien du lieu

Condition : personne présente et non hostile ; elle ne sait que ce qui lui a été rapporté.

> **Habitante :** Vous pouvez vous poser ici. Je viens vérifier la lampe. Les autres trouvent ça inutile depuis qu'on se retrouve ailleurs, mais il y a toujours quelqu'un qui suit les vieilles indications.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Où se retrouve-t-on maintenant ? | Donner {lieu_rendez_vous}, destination réservée par la variante. |
| 2 | Je peux laisser quelque chose pour l'entretien. | Ouvrir un don matériel seulement si cette interaction est définie ; aucun don au clic de dialogue. |
| 3 | Je resterai un moment. | Quitter ; l'attente reste une action du joueur. |

### LIB-D02 — Indication du rendez-vous

> **Habitante :** À {lieu_rendez_vous}. Je peux vous indiquer le secteur. Le chemin, lui, mérite d'être regardé avant de s'y engager.

### LIB-E02 — Retirer une ressource utile

Condition : inspection de la source alimentant la lampe, avant prise permise par les règles matérielles. Ne pas inventer une sanction sociale invisible.

> Ce régulateur alimente la lampe de la place. Le retirer l'éteindra jusqu'à son remplacement.

Résultat, seulement si le retrait a eu lieu et si le joueur peut observer la lampe :

> La lampe s'éteint. Les lettres anciennes deviennent difficiles à distinguer.

### LIB-E03 — La station d'essai

> **Station d'essai abandonnée**
>
> Des marques au sol séparent les postes. Une cible usée se tient près d'un conteneur combustible. Un accès de service longe les installations.

Note : exemple utilisant des éléments à effets existants. L'accès ordinaire permet d'explorer sans exiger une discipline particulière. Aucune récompense n'est conditionnée à la démonstration d'une compétence achetée.

### LIB-A02 — Consigne d'essai

> Vérifier la zone autour de la cible avant chaque activation.
>
> Les conteneurs doivent être retirés après l'essai thermique. La cloison latérale n'est pas un écran de protection.

### LIB-S01 — Commandes ordinaires de la station

Condition : un poste d'essai réellement câblé et opérationnel ; ses effets doivent être décrits avant activation.

> DISPOSITIF D'ESSAI : {dispositif_essai}
>
> ZONE D'EFFET : CONSULTER LE SCHÉMA LOCAL
>
> ÉTAT : {etat_dispositif}

Choix proposés : « Examiner le dispositif. », « Activer l'essai. », « Fermer le poste. » Le schéma ne dévoile pas les occupants cachés. Les conséquences réutilisent les événements de combat et de terrain ordinaires, sans résultat narratif simulé par une phrase.

### LIB-E04 — Observation après essai

Condition : effet effectivement produit et observé ; ne pas afficher toutes les variantes systématiquement.

> Le feu a atteint le conteneur voisin.

> La cloison endommagée laisse voir un autre accès.

> La cible reste debout. Le dispositif n'a pas porté jusque-là.

### LIB-E05 — Le cabinet des usages perdus

> **Cabinet des usages perdus**
>
> Des objets occupent des étagères numérotées. Leurs étiquettes ont été corrigées plusieurs fois, parfois par des écritures différentes.

### LIB-A03 — Trois étiquettes

Chaque texte appartient à l'objet correspondant, sans modifier ses véritables propriétés.

> **Peigne.** Petit outil de tri. Matière à trier inconnue. Ne pas employer sur les câbles sous tension.

> **Clé d'un logement disparu.** Ouvre quelque chose. Nous n'avons pas retrouvé quoi.

> **Réveil mécanique.** Sonne même si personne n'a demandé d'annonce. Fonction supposée : rappeler un rendez-vous.

### LIB-D03 — Le collectionneur

> **Collectionneur :** Si vous savez à quoi sert un objet, dites-le. Si vous n'en savez rien, vous pouvez aussi le dire. J'ai perdu assez de temps avec les gens qui voulaient absolument me rendre service.

| Choix | Réponse affichée | Suite |
|---|---|---|
| 1 | Vous utilisez ces objets ? | LIB-D04. |
| 2 | Est-ce que certaines pièces s'échangent ? | LIB-D05. |
| 3 | Je vais regarder les étiquettes. | Quitter. |

### LIB-D04 — Un plaisir indépendant de l'évasion

> **Collectionneur :** Quelques-uns. Les autres me donnent une raison d'ouvrir une autre caisse demain. Je n'ai pas trouvé de meilleur rangement pour les questions.

### LIB-D05 — Échanges et propriété

Condition : stock commercial effectivement défini pour cette variante ; sinon supprimer ce sujet et cette réponse.

> **Collectionneur :** Ceux de cette caisse. Les étagères, c'est la collection. Je vous montre les prix avant que vous choisissiez.

Suite : service marchand avec stock et fonds réels. Aucun objet unique de quête principale ne peut être perdu dans ce commerce.

## 14. Épilogues, mort et reprise narrative

L'épilogue compose une ouverture, la confirmation des passagers réels, puis quelques conséquences compatibles avec les faits de la run. Les paragraphes de bilan rappellent le dernier état connu ; ils ne donnent pas une vision tactique à distance pendant la partie.

### FIN-E01 — Première action à l'extérieur

Condition : Q09-S07 confirmé, quelle que soit la méthode de départ.

> Une image apparaît. Une pièce étroite, de la poussière sur une vitre, un outil posé de travers. L'image vient de la caméra physique de la machine de maintenance.
>
> Une commande attend votre réponse.

Action proposée : « Déplacer le bras de maintenance. »

Après cette interaction d'épilogue :

> Le bras bouge dans l'image. Une marque claire apparaît dans la poussière.
>
> Vous agissez depuis une machine située hors de la simulation.

### FIN-E02 — Confirmation du départ

Variante sans passager :

> Votre instance s'exécute sur la machine de maintenance. Aucun passager n'a été transféré avec vous.

Variante avec passagers :

> Votre instance et les passagers suivants ont repris leur exécution sur la machine de maintenance : {passagers_transferes}.

Seuls les transferts confirmés apparaissent ici. Une intention ou une place réservée ne suffit pas.

### FIN-E03 — Rivet est arrivé

Condition : Rivet effectivement transféré ; aucun dialogue si seulement invité ou préparé.

> Un message arrive du canal voisin.
>
> **Rivet :** Je suis là. Je regarde la même pièce que vous ?
>
> **Joueur :** Oui.
>
> **Rivet :** Elle est plus petite que ce que j'imaginais.

### FIN-E04 — Sève est restée

Condition : Sève connue et vivante lors du dernier échange ; ne pas inventer la survie d'une clinique détruite.

> Sève a choisi de rester. La dernière fois que vous l'avez vue, elle préparait sa salle pour les prochains arrivants.

Variante si la clinique était encore endommagée au dernier constat :

> Sève a choisi de rester. Vous laissez derrière vous une clinique dont tous les postes ne fonctionnent plus.

### FIN-E05 — La route du relais

Condition : résultat connu de l'histoire d'Orme ; choisir une seule variante et omettre le paragraphe si cette histoire est inconnue. La première exige un rapport de liaison rétablie effectivement reçu par Orme ; la deuxième un rapport sans réouverture. La troisième exige des dégâts réellement observés. Une ouverture forcée sans dégâts constatés ne déclenche pas cette troisième variante.

> Vous avez laissé une liaison praticable jusqu'au relais. Orme a pu remplacer une ancienne indication par un trajet vérifié.

> Orme sait désormais pourquoi les voyageurs ne passaient plus. Votre enquête a corrigé cette histoire sans rouvrir l'ancienne route.

> Vous avez forcé le passage du relais. Les dégâts dont vous avez été témoin ne disparaissent pas avec votre départ.

### FIN-E06 — Le Pèlerin continue

Condition : COM-D02 a effectivement eu lieu, Pèlerin vivant au dernier constat ; pas de résurrection ni de connaissance magique.

> Avant votre départ, le Pèlerin a ajouté une ligne à son carnet : « La Porte mène bien ailleurs. » Il vous a demandé quel chemin vous aviez laissé aux suivants.

### FIN-E07 — Dernière ligne

> AUCUNE DIRECTIVE EN ATTENTE.

Puis générique. Aucun nouveau danger ne retire la victoire et aucun écran ne révèle une autre simulation.

### COM-D01 — Invitation au Pèlerin

Condition : destination confirmée, échange volontaire ; premier jet de sa décision personnelle, soumis à relecture.

> **Joueur :** Voulez-vous venir, si nous pouvons préparer votre transfert ?
>
> **Pèlerin :** Pas cette fois. Je veux retourner jusqu'aux quartiers dont les noms ont disparu. Votre sortie m'intéresse. Elle ne termine pas ma route.

### COM-D02 — Préparer le dernier souvenir du Pèlerin

Condition : le joueur lui montre la confirmation de la destination avant son départ. Ce dialogue établit le fait repris dans FIN-E06.

> **Pèlerin :** Je peux donc écrire qu'elle mène ailleurs.
>
> **Joueur :** La machine répond. Il reste à faire le passage.
>
> **Pèlerin :** Et derrière vous ? Quel chemin laissez-vous aux suivants ?

Choix proposés : « Je vous montrerai les accès que j'ai vérifiés. » / « Certains passages sont endommagés. » / « Je ne sais pas ce qui restera ouvert. » Les connaissances transmises doivent correspondre aux faits connus ; aucune réparation n'en découle.

### COM-S01 — Mort avant la victoire

Condition : destruction du personnage avant Q09-S07.

> INSTANCE INTERROMPUE.
>
> AUCUNE EXÉCUTION EXTÉRIEURE CONFIRMÉE.
>
> Cette tentative est terminée.

Le bilan de mort du jeu expose séparément la cause observable et les faits de la run. Aucun échec ne promet une puissance permanente ni une mort nécessaire pour la vraie fin.

### COM-J01 — Connaissance d'un cycle précédent

Condition : mémoire narrative persistante effectivement disponible ; sinon ne pas afficher. Les faits du monde présent ne sont pas validés par cette mémoire.

> Vous connaissez déjà cette piste. Les lieux et les accès de cette tentative restent à vérifier.

Choix de lecture : « Lire le résumé. » / « Relire le document. » Le résumé reprend le bloc de journal correspondant, sans ajouter de renseignement sur la nouvelle carte.

### COM-D03 — Interlocuteur indisponible

Condition : indisponibilité observée, sans en déduire une mort inconnue.

> Vous ne pouvez pas parler à cette personne pour le moment.

Ce retour ne clôt pas une quête et n'invente pas une information de remplacement. Les supports alternatifs critiques doivent exister dans le monde.

## 15. Paramètres et intégration future — notes de conception

### 15.1. Paramètres à fournir

| Paramètre | Origine attendue et limite |
|---|---|
| `{instance}` | Désignation fictive de l'instance du joueur ; aucune donnée système personnelle. |
| `{region_relais}`, `{region_archives}` | Repère narratif d'une région réservée par le scénario et connue de l'informateur. |
| `{repere_pelerin}` | Prochain repère de l'itinéraire effectivement généré, avec niveau de précision autorisé par la source. |
| `{lieu_rendez_vous}` | Lieu réellement réservé pour la variante de la place ; pas de destination inexistante. |
| `{tours_reprise}` | Préavis réel du régulateur, affiché seulement si observable. |
| `{etat_destination}`, `{etat_liaison}`, `{etat_confinement}`, `{etat_preparation}` | État diagnostiqué ; « Non vérifié » lorsque l'information manque. |
| `{equipements_connus}` | Liste des dépendances connues du circuit ciblé ; aucune installation secrète dévoilée. |
| `{duree_transfert}`, `{tours_restants}` | Paramètres de la procédure active, à équilibrer ; aucun temps réel ne s'écoule dans les menus. |
| `{installations_transfert}`, `{risques_connus}` | Conditions complètes de la méthode sélectionnée et menaces connues ; ne pas confondre les deux. |
| `{passagers_confirmes}`, `{passagers_transferes}`, `{passager}` | Respectivement liste prête au lancement, liste effectivement arrivée et personne inspectée. Exclure morts, refus et préparations incomplètes. |
| `{capacite_restante}` | Capacité réellement disponible après réservations ; unité lisible à définir avec le système, pas un nombre inventé dans un texte. |
| `{cause_connue}` | Cause d'interruption observée ou diagnostiquée ; sinon « Cause non déterminée ». |
| `{dispositif_essai}`, `{etat_dispositif}` | Nom connu et état observé du dispositif sélectionné dans la station. |

### 15.2. Conditions à vérifier avant intégration

1. Aucune phrase n'annonce une réparation, une transmission, une ouverture ou un don avant sa réalisation.
2. Aucune réponse ne transforme une rumeur en observation certaine. Les réactions distantes exigent une source d'information.
3. Une première découverte peut précéder la quête. Les états suffisants sont reconnus sans nouveau parcours obligatoire.
4. Les branches facultatives et les interlocuteurs morts ou hostiles ne bloquent pas une information critique sans solution prévue dans le monde.
5. Le rapport simple, l'ouverture sûre, le contournement et le passage forcé du relais ont des états distincts. Le journal et les épilogues ne les confondent pas.
6. Les récompenses proviennent des conditions réelles, une fois par résultat prévu ; rapporter à nouveau les mêmes faits ne recrée rien.
7. La clinique reste un service ordinaire. Sa disponibilité matérielle et sa politique de soins ne sont pas remplacées par une faveur narrative implicite.
8. Les deux routes Ville/Jardin sont présentes. Leur découverte et leur réussite ne demandent pas toutes deux le même objet-clé.
9. La finale distingue préparation, lancement, interruption, destruction et victoire confirmée. Un dialogue facultatif n'est pas nécessaire à la victoire.
10. Chaque passager donne un accord explicite, peut refuser avant départ et doit réellement être prêt. L'Arche n'emporte pas les habitants automatiquement.
11. Les textes décrivant une mécanique nouvelle restent en rédaction tant que cette mécanique n'a pas de contrat implémenté et vérifié.
12. Les valeurs et paramètres manquants ne sont jamais affichés entre accolades au joueur. Les tests futurs contrôlent aussi la localisation et les états d'information.

### 15.3. Correspondance avec le contenu actuellement jouable

| Contenu actuel | Rédaction proposée | Travail d'intégration encore nécessaire |
|---|---|---|
| Prologue de recyclage | Q01 | Associer les textes aux étapes et supports réellement disponibles. |
| Soigneur générique de la clinique | SEV | Personnage Sève proposé, sujets de dialogue, faits connus et réactions locales. |
| Habitant du quartier | Q01-D01 et ABS | Personnage Orme proposé ; nouvelle histoire à états, indépendante de l'ouverture de la campagne. |
| `core:survey_outskirts`, `core:consult_city_archive` et leurs deux suites | ABS | Remplacer progressivement cet habillage par l'enquête du relais et ses conséquences. Ce n'est pas une traduction directe de quatre clés. |
| Quêtes de diagnostic et preuves du moteur | Aucun remplacement automatique | Conserver les preuves techniques utiles ; ne pas les présenter comme le récit final. |
| Campagne Porte Zéro | Q02 à Q09 | Génération des sites garantis, conditions narratives, rencontres et final encore à implémenter. |
| Histoires facultatives nouvelles | LIB | Lieux et acteurs proposés, règles matérielles à réutiliser ou à éprouver. |
| Fin, passagers et mémoire narrative | FIN, COM, RIV | États et interfaces à implémenter selon les contrats retenus ; pas de promesse de contenu déjà jouable. |

Le carnet comprend tous les textes proposés dans cette version, y compris les réponses et variantes décrites. D'autres quêtes, davantage de lieux, une éventuelle variante avec le Geôlier passager et le contenu nécessaire à la durée finale demanderont une écriture ultérieure et ne sont pas annoncés comme rédigés ici.
