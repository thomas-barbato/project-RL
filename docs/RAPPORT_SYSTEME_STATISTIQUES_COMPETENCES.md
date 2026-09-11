# Rapport — Statistiques et compétences, livraison documentaire 0.6

10 septembre 2026. Livraison initiale : étapes 1 à 6 et première vérification documentaire de l'étape 8 ; étape 7 ensuite terminée au niveau rédactionnel, après quinze validations individuelles puis délégation pour les textes restants. **État actuel : les cinq corrections proposées à la revue finale ont été acceptées puis appliquées aux documents et à leurs contrôles ; bilan en section 10.** Les sections 8 et 9 conservent l'historique antérieur à cet accord, avec leurs anciens résultats. Aucun code de jeu, contenu d'équipement ou texte d'interface exécuté n'est modifié par ces livraisons. Les autres paramètres restent des propositions de travail, pas un équilibrage final.

## 1. Ce qui a été ajouté

| Étape | Livrable | Statut |
|---|---|---|
| 1 — Intrusion | Formules, droits locaux, sessions, tentative native, Purge, traces et échecs coûteux. | Règles communes §13, proposition. |
| 2 — Autres secondaires | Détection/Analyse déterministes, charge/traction/poussée ; proposition sans secondaire Contrôle distant supplémentaire. | Règles communes §14. |
| 3 — Ressources | Énergie, chaleur, dissipation, bande passante, alimentation et réparation ordinaires. | Règles communes §15. |
| 4 — Règles transversales | Temps, prélèvements, durée, récupération, cumuls, protections anti-répétition, réactions, zones. | Règles communes §16. |
| 5 — Progression | Budget initial, rangs, XP, points primaires, exemples de fin de run et réattribution proposée. | Règles communes §17, chiffres à valider/tester. |
| 6 — Fiches | 94 profils principaux + 10 variantes après fusion de Retraite méthodique dans Pas de dégagement. | Catalogue §16, coûts et effets d'essai ; correction validée en §15.7. |
| 7 — Textes joueur | Quinze infobulles validées individuellement ; autres infobulles, aides, 94 descriptions principales, 10 variantes, états et messages rédigés sous délégation. | Règles communes §19 et document de textes joueur ; rédaction achevée et corrections synchronisées, non intégrée au jeu. |
| 8 — Vérification | Couverture, prérequis, calculs, budgets, risques et contrats de données/sauvegarde. | Présent rapport + règles communes §§18–19 ; gameplay encore à tester. |

Documents de référence : [règles communes](STATISTIQUES_ET_COMPETENCES.md), [catalogue et annexe des 104 profils](PROPOSITION_COMPETENCES_v0.1.md), [textes joueur de l'étape 7](TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md). Les documents fondateurs renvoient déjà aux règles communes et au catalogue ; ils ne sont pas modifiés par le complément rédactionnel ni par les cinq corrections.

## 2. Décisions nouvelles proposées, non imposées

- Pas de douzième secondaire de contrôle distant : le matériel, les canaux et les techniques remplissent déjà ce rôle.
- Détection/Analyse déterministes dans les conditions observables, pour ne pas récompenser les inspections répétées sans changement.
- Profil d'essai : 20 niveaux, 2 points de compétence initiaux inclus dans les acquis de classe, puis 1 par niveau gagné ; total 21. Un point primaire tous les quatre niveaux, cinq au maximum.
- Pas de bonus automatique de rang, ni de soin ou recharge au niveau supérieur. Réattribution et achat après cinq choix non ouverts dans le premier profil ; cette absence reste à valider.
- Réserves matérielles de laboratoire : 100 E, dissipation 5 H/UT, 4 B, contrôleur de deux drones. Elles ne définissent pas la classe ou le corps final.
- Infection propose un sabotage thermique périodique direct, distinct de Surchauffe qui augmente la jauge de chaleur. Ce typage est une proposition technique, pas un nom ou un effet déjà validé dans l'interface.

Les classes détaillées, quêtes, scénario, Radiation/Corruption et capacités extérieures n'ont pas été développés. Le dossier actuel peut les accueillir sans les inventer comme prérequis.

## 3. Contrôles reproductibles

Commande depuis la racine du projet :

```text
node tools/validate_stats_docs.mjs
node tools/review_stats_edge_cases.mjs
```

Le [contrôleur documentaire](../tools/validate_stats_docs.mjs) fonctionne en lecture seule, sans dépendance externe. Il vérifie : structure des tableaux/liens, couverture exacte des 104 profils, absence des quatre anciens identifiants retirés ou fusionnés parmi les achats, prérequis et absence d'impasse sur chaque parcours légal, 49 lignes numériques extraites des règles, bornes/monotonie, six budgets et séquences d'apprentissage, puis quelques modèles isolés de ressources, de temps et de non-duplication. Il utilise le [modèle de progression documentaire](../tools/stats_progression_checks.mjs), également partagé par le [contrôle des cas limites](../tools/review_stats_edge_cases.mjs). Ce dernier distingue désormais les cinq corrections vérifiées des deux observations d'équilibrage conservées.

Ces vérifications ne testent **pas** les fonctions Rust du jeu. Les petits modèles JavaScript illustrent les invariants proposés ; ils ne prouvent pas le comportement de sauvegarde, la ligne de vue, la sécurité, les explosions, les réactions ou les drones du prototype. Les valeurs de leurs formules sont codées pour vérifier les exemples documentés, pas un format de contenu destiné au moteur. Aucun benchmark de difficulté ni taux de victoire n'est mesuré.

Résultat historique de la livraison initiale, avant les compléments : **10 groupes de contrôles réussis**, 105 profils couverts, 49 lignes numériques recalculées et 6 builds vérifiés. La vérification d'empreintes avant/après avait également retrouvé les 59 fichiers inspectés de sources/contenu et documents hors périmètre inchangés. Les corrections faites pendant cette passe concernaient les tableaux et les précisions documentaires, pas le moteur. Aucun test Cargo ou essai visuel n'avait été lancé. Les résultats actuels sont consignés en section 10.

## 4. Six répartitions légales de fin de profil

Budgets à niveau 20, en comptant les deux points de départ. Ce ne sont pas des classes imposées. Le script contient pour chacun une répartition de primaires initiale/finale valide et une séquence concrète de choix avec ses prérequis ; les rangs seuls ne garantiraient pas ce dernier point.

| Build | Rangs | Coût total | Techniques choisies | Limite principale à éprouver |
|---|---|---:|---:|---|
| Combattant mobile | Mêlée 5, Manœuvre 3, Furtivité 2, Reconnaissance 2, Ingénierie 3 | 21 | 15 | Approche sous le feu, lourdeur/ancrage des adversaires, réparation finie. |
| Tireur observateur | Tir 5, Reconnaissance 4, Manœuvre 3, Furtivité 2 | 21 | 14 | Munitions, ruptures de vue et coût des préparations. |
| Démolisseur discret | Démolition 5, Furtivité 3, Manœuvre 3, Reconnaissance 3 | 21 | 14 | Stocks de dispositifs, terrain réellement destructible et risques pour les tiers. |
| Saboteur électronique | Guerre électronique 5, Intrusion 4, Reconnaissance 3, Manœuvre 2 | 21 | 14 | Cibles incompatibles, chaleur/énergie ; sa séquence d'essai doit utiliser un nettoyage natif faute de Purge apprise. |
| Opérateur de drones | Drones 5, Ingénierie 4, Tir 3, Reconnaissance 2 | 21 | 14 | Perte de matériel, saturation des canaux et réponse aux ruptures de liaison. |
| Généraliste | Tir 3, Manœuvre 3, Reconnaissance 3, Ingénierie 3, Guerre électronique 3, Furtivité 1 | 21 | 16 | Plus de choix, aucun rang 4/5 ; dépend davantage des fonctions natives du matériel. |

Deux maîtrises coûtent 18 points et laissent 3 points de soutien. Dix maîtrises coûteraient 90. Le catalogue complet n'est donc pas acquis dans une seule run. L'intérêt de ces choix dépendra cependant du contenu effectivement rencontré et de la disponibilité des outils.

## 5. Comparaisons arithmétiques et risques

### Temps contre efficacité immédiate

Profils ordinaires, précision 70 %, cible de Blindage 5, armes neutres ; toutes les conditions de portée sont remplies, tirs dans la première moitié de portée sans couvert ni recul pénalisant. On compare un cycle attaque/préparation ou attaque/récupération, sans compter une éventuelle action utile de soutien pendant R1. Les dégâts espérés incluent les ratés, pas une simulation d'IA.

| Attaque de référence | Dégâts bruts si touche | Durée du cycle en UT | Dégâts espérés par UT |
|---|---:|---:|---:|
| Frappe ordinaire | 12 | 1 | 4,9 |
| Frappe puissante | 18 | 2 | 4,55 |
| Tir simple | 10 | 1 | 3,5 |
| Tir visé, précision portée à 90 % | 10 | 2 | 2,25 |

Conclusion : préparer n'est pas automatiquement rentable en dégâts par unité de temps contre une cible moyenne. Cela économise potentiellement des munitions, franchit des seuils d'armure ou prépare une ouverture, mais le coût en temps peut être excessif. Tester contre une cible à forte Esquive, un Blindage élevé et une fenêtre d'exposition courte avant de renforcer systématiquement les nombres. Visée persistante et suppression changent également la comparaison sur plusieurs actions.

### Électronique et petits impacts

À 25 % de résistance électrique, Surcharge sur trois cibles exposées peut infliger 36 dégâts au total pour 20 E ; Cascade fait 27 au total (12 + 9 + 6) pour 22 E. Cette différence n'est pas nécessairement une erreur : Surcharge exige la proximité et expose les alliés, Cascade atteint plus loin et choisit ses cibles perçues. Elle doit toutefois être éprouvée en salles et en couloirs, pas équilibrée par un total abstrait seulement.

Surchauffe standard, sans autre action adverse, dissipation initiale 5 et résistance thermique 25 % : à partir de H=0, ses trois phases donnent H=39 sans dégât critique ; à partir de H=90, elles donnent H=129 et 25 dégâts après résistance durant ces trois phases. D'autres dégâts peuvent suivre tant que la cible reste chaude. La compétence est situationnelle, pas un tir magique identique sur toutes les machines ; tester refroidissement, arrêt du module compromis et disponibilité de Purge.

Infection contagieuse à 2 dégâts thermiques par tic contre 75 % de résistance tombe à **zéro** après arrondi. La variante peut donc être inutile contre ce profil même sans immunité. Ce constat respecte l'absence de minimum universel de 1 dégât, mais devra orienter le montant des tics, leur regroupement éventuel ou le rôle de la variante ; aucune correction cachée n'est appliquée au moteur.

### Points sensibles restants

| Risque | Constat / correction à expérimenter |
|---|---|
| Maîtrise de Reconnaissance chère | Le cinquième choix coûte 3 sans pouvoir exclusif ; mesurer sa valeur réelle, ne pas ajouter un pouvoir uniquement pour remplir le rang. |
| Retraite méthodique — achat corrigé | Maintien de consigne fusionné sans achat supplémentaire dans MAN-01 après accord utilisateur ; MAN-07 réservé à l'historique. Reste à vérifier l'ergonomie en jeu, sans nouveau bonus ni économie d'action. |
| Défenses par préparation | Parade, Surveillance et Esquive ne se cumulent pas sur un même acteur ; plusieurs drones peuvent néanmoins défendre le groupe. Deux unités constituent la première limite de laboratoire. |
| Saturation numérique | Deux routines avancées peuvent occuper les 4 B du contrôleur ; une commande demandant +1 B exige suspension explicite d'un processus ou meilleur matériel. Ne pas ignorer le coût pour faire fonctionner le bouton. |
| Taxe d'Ingénierie | Le soin ordinaire restaure 20 en 1 UT sans achat ; Ingénierie vise composants/choix matériels. Comparer les stocks consommés, pas seulement les quantités nominales restaurées. |
| Achats morts dans une version partielle | Règle validée : différer une discipline si un parcours légal ne peut pas atteindre cinq choix. Le contrôle documentaire passe ; le raccorder aux fonctionnalités réellement livrées, aux classes et aux sauvegardes reste à faire. |
| Irréversibilité d'un mauvais choix | L'absence de réattribution est une proposition, pas une exigence acquise. Réexaminer après essais ; le texte conditionnel UI-REATTRIBUTION n'est affichable que si le mode joué désactive réellement la réattribution. |

## 6. Scénarios jouables à exécuter plus tard

Les scénarios suivants sont des **critères d'acceptation non exécutés dans le moteur** :

1. Mêlée : raté, zéro dégât, Parade puis Riposte, Entrave résistée/subie et expiration de sa protection ; aucune boucle de réactions.
2. Tir : rafale et munitions exactes, cible qui quitte la vue, repli d'Esquive encore couvert par l'explosion, dégâts aux composants sans double perte de PV du personnage et de Durabilité du matériel.
3. Temps : P2+A1 interrompue à chaque étape, dépenses et CD corrects, sauvegarde au milieu de la préparation, aucun avancement par menu.
4. Ressources : hausse/baisse de capacité, batterie retirée puis réinstallée, source vide, H au-delà de 100, panne de contrôleur et réservations libérées une seule fois.
5. Perception : même seed et mêmes commandes au clavier/souris, trois modes de fenêtre et plusieurs résolutions ; états/sources d'information strictement identiques. Cible cachée jamais confirmée par aperçu.
6. Exploration : trace expirée, secret difficile, plusieurs inspections identiques, paroi réellement fragile et accès essentiel accessible autrement.
7. Numérique : interface incompatible, accès partiel, audit avant/après falsification, tentative ennemie de reprise, rupture de liaison sans effacement des preuves transmises.
8. Infection : chaîne cyclique, plusieurs sources, purge de l'hôte initial, expiration globale, transmission hors perception sans icône révélatrice ; hôtes/tics plafonnés.
9. Terrain : première explosion bloquée par un mur, seconde traversant la brèche réelle ; mine/balise déclenchée une fois, plusieurs poussées sans impacts infinis dans une UT.
10. Drones : ordres groupés sans actions offertes, unité hors liaison, objet disparu, deux interpositions concurrentes, retour impossible et consommation des réserves réelles.
11. Progression : récompense multi-niveaux, mort, revisite, résolution pacifique puis élimination du même obstacle, fabrication/destruction répétées ; pas de doublon de récompense ni confusion avec les déblocages de classe.
12. Routes complètes : plusieurs seeds fixes par build et plusieurs façons de résoudre les obstacles ; vérifier les moyens réellement présents avant un passage obligatoire, les ressources et les causes de mort. Aucun taux de victoire n'est fixé avant ces essais.

## 7. Raccordement au prototype et conclusion

L'inspection ciblée constate toujours un minimum de dégâts à 1, une pénétration appliquée aux pourcentages de résistance et une courbe d'XP d'amorçage. Les règles proposées ici ne correspondent donc pas encore au comportement final du jeu. Les statuts existants offrent remplacement/rafraîchissement/cumuls, mais ne prouvent pas les nouvelles familles de protection et leurs échéances. Les adapter nécessite une tâche d'implémentation et des tests séparés, sans migration silencieuse des objets/sauvegardes.

La livraison donne une base documentaire continue pour développer et tester le système actuel. Elle ne prétend pas résoudre l'équilibrage à l'avance. L'étape 7 a d'abord avancé texte par texte, puis l'utilisateur a explicitement autorisé la rédaction de tous les textes restants. Les formulations rédigées sous délégation sont distinguées des quinze infobulles relues et validées individuellement.

Suivi terminologique ultérieur : l'utilisateur a validé « Points de vie (PV) » pour les personnages/créatures et « Durabilité » pour les équipements/objets destructibles, à la place de l'ancien libellé de réserve « Intégrité ». Les exemples et règles gardent les mêmes valeurs ; les identifiants du prototype restent inchangés. Les textes individuels validés et cette décision sont suivis en sections 19.1 et 19.2 des règles communes.

## 8. Complément rédactionnel — achèvement du point 7

Historique du complément, avant les cinq corrections : les nombres 105 et 198 ci-dessous décrivent cette livraison, pas l'inventaire actuel de la section 10.

L'utilisateur a validé l'Efficacité d'intrusion puis demandé de terminer les textes restants avant de passer au dernier point. La délégation est consignée en section 19.3 des règles communes ; elle n'autorise ni nouvelle mécanique ni modification de l'équilibrage.

Livraison complémentaire :

- Défense numérique et explications des ressources, valeurs matérielles, temps et réactions ; les quinze textes déjà validés restent inchangés.
- Aides à la création, aux rangs, aux choix, aux variantes et à la progression de run ; textes conditionnels pour les décisions encore ouvertes, notamment la réattribution.
- Présentation des dix disciplines et couverture exacte des 95 techniques/améliorations principales et 10 variantes, avec leurs conditions déterminantes.
- États, champs chiffrés alimentés par le profil actif, refus gratuits fondés uniquement sur les informations connues, avertissements avant engagement et notifications de résultats observables.
- Suivi documentaire harmonisé : l'ancienne obligation d'attendre chaque validation individuelle est remplacée par la délégation actuelle, sans réécrire l'historique des accords.

Le vérificateur couvre désormais aussi le document rédactionnel : noms et identifiants des 105 profils, techniques mères, dix disciplines, clés de texte uniques, substitutions encadrées et références locales. Ces contrôles structurels complètent une relecture sémantique ; ils ne prouvent pas qu'une future interface respectera effectivement les coûts et la perception.

Résultat local du complément : **12 groupes de contrôles réussis**, 105 descriptions de techniques/variantes et 198 autres entrées rédactionnelles (infobulles, aides, états, libellés et messages), en plus des quinze infobulles validées individuellement. Les 49 lignes numériques et les six builds du contrôle initial passent toujours. La comparaison d'empreintes retrouve les quatorze textes déjà enregistrés avant ce complément inchangés, ainsi que les 55 fichiers présents sous `src/` et `content/`. Le quinzième texte reproduit la dernière formulation validée. Aucun essai du moteur ou du rendu n'a été exécuté.

Le dernier point du plan est bien **l'étape 8 — vérification et mise à l'épreuve**. La première passe arithmétique et documentaire de ce rapport a déjà été exécutée ; les contrôles rédactionnels la complètent. La revue des arbitrages ouverts et les scénarios de la section 6 restent le support de la suite. Leur exécution en jeu attendra une implémentation dédiée, sans prétendre qu'un document ou un validateur JavaScript constitue un équilibrage jouable.

## 9. Revue finale du point 8 — constats et décisions proposées

**Historique de la revue, avant validation :** effectuée après la demande utilisateur « d'accord allons y :) ». Périmètre : cohérence du système documenté, valeur des choix, risques de blocage, préparation des essais. Les corrections ci-dessous n'étaient pas encore appliquées aux règles, aux profils ou aux textes joueur au moment de ces constats. Les mentions « à valider » de cette section retracent cet état antérieur ; l'accord suivant et son application sont consignés en section 10. Aucun développement de gameplay n'a été entrepris par cette revue.

### 9.1. Verdict et preuves

Le dossier couvre désormais le système actuel au niveau conception et rédaction, mais il ne doit pas encore être traité comme une spécification sans ambiguïté prête à coder intégralement. Plusieurs règles bien intentionnées produisent des cas limites lorsque leurs durées, coûts et dépendances sont combinés. Les clarifier avant l'implémentation coûtera moins qu'une correction de l'ordonnanceur ou des sauvegardes après coup.

Contrôles relancés :

```text
node tools/validate_stats_docs.mjs
node tools/review_stats_edge_cases.mjs
```

Le premier conserve ses **12 groupes réussis**, 105 profils/descriptions, 198 autres entrées rédactionnelles, 49 lignes numériques et 6 builds. Le second, [reproduction des cas limites](../tools/review_stats_edge_cases.mjs), reproduit **7 groupes de constats** : chaleur, retardateurs, registres, Reconnaissance dans une version partielle, Retraite méthodique, attaques situationnelles, puis Infection et canaux. Ses assertions confirment que les contre-exemples décrits sont reproductibles dans les petits modèles ; leur succès ne signifie pas que les problèmes ont été corrigés.

Les hypothèses de chronologie sont explicites dans le script et les paragraphes ci-dessous. Ce ne sont pas des bugs reproduits dans Rust : les systèmes correspondants ne sont pas testés par ces modèles. Les paramètres lus dans les documents sont contrôlés pour signaler une dérive si les règles changent. Aucun taux de victoire, parcours complet ou comportement réel de PNJ n'a été mesuré.

### 9.2. Ambiguïtés à lever avant implémentation

#### REV-01 — La sécurité thermique ne doit pas supprimer les actions de secours

Source : [règles communes](STATISTIQUES_ET_COMPETENCES.md), §§15.2–15.3 ; textes UI-SEUIL-CRITIQUE et MSG-REFUS-CHALEUR dans les [textes joueur](TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md).

Le refus d'une action « projetant H au-delà de 100 » peut être lu comme un contrôle global de toutes les actions. À H=115, attendre ou effectuer une action à +0 H projette encore H=115 : la lecture large les refuse. Pourtant, attendre doit laisser refroidir, les attaques ennemies peuvent dépasser le seuil et la surchauffe ne doit pas provoquer de paralysie automatique. Si les commandes refusées n'avancent pas le temps, cette lecture peut même empêcher le refroidissement attendu.

**Recommandation à valider :** restreindre ce refus aux actions qui ajoutent volontairement de la chaleur au-delà de la limite de leur mode. Une action à apport nul ou refroidissant reste admissible du point de vue thermique, sans ignorer ses autres préconditions. Les apports périodiques ennemis et les dégâts thermiques continuent normalement ; ce n'est pas une immunité. Exemples d'acceptation proposés : H=115/+0 autorisé, H=115/−5 autorisé, H=95/+6 refusé en mode ordinaire, H=95/+5 autorisé. Les modes de Surcadencement garderaient leur limite propre.

Statut : ambiguïté de portée de règle, confiance élevée dans le risque de lecture ; aucune immobilisation du jeu actuel n'est affirmée.

#### REV-02 — Distinguer durée d'effet et délai laissant une réponse

Sources : règles communes §16.2 ; catalogue annexe 16, DEM-03, DEM-04 et GEL-08.

Les effets périodiques peuvent agir à la phase environnementale suivant immédiatement leur application pendant les actions. Si cette convention s'applique aussi aux retardateurs, poser une Charge de brèche avec délai 1 UT déclenche l'explosion à la fin du cycle de pose, avant la prochaine action du joueur. Le poseur adjacent est dans son rayon 1. La possibilité annoncée de se retirer après la pose n'a alors aucune fenêtre pour lui. Ce n'est pas une garantie que tous les poseurs meurent, mais une contradiction entre l'usage annoncé et cette lecture de l'échéance.

Même difficulté défensive : une Implosion ennemie avec délai 2 UT, implantée après l'action du joueur, laisse seulement une action du joueur avant la détonation si la première phase environnementale compte immédiatement. Le nettoyage natif prend 2 UT et ne peut pas se terminer à temps dans ce cas. Une Purge spécialisée en 1 UT ou une autre réponse locale peut rester possible ; il ne faut donc pas qualifier tout le mécanisme d'incontrable.

**Recommandation à valider :** donner aux retardateurs annoncés un décompte distinct des effets périodiques : ne pas compter la phase environnementale du cycle où ils sont armés. Dans le modèle de référence, un délai 1 laisse alors une prochaine action ordinaire de 1 UT avant détonation ; un délai 2 laisse deux étapes de réponse. Ne pas déplacer silencieusement tous les tics de poison, chaleur, états ou cooldowns pour résoudre le seul problème des fusibles.

Cette convention doit aussi préciser la phase d'armement des mines, les annonces d'effondrement, les deux charges d'une Détonation combinée et les reprises après sauvegarde. Le délai laisse une possibilité, pas la réussite : déplacement ralenti, passage bloqué, ressources insuffisantes ou nettoyage manqué peuvent toujours rendre une réponse inefficace. Ne pas garantir qu'une action de durée quelconque tienne dans une fenêtre de 1 UT.

Statut : convention temporelle insuffisamment explicite ; contre-exemples conditionnels reproduits, pas exécution du moteur.

#### REV-03 — Donner une fenêtre utile à Falsification de registre

Sources : règles communes §§13.1 et 13.3 ; catalogue INT-08, annexe 16.9.

Cas le plus favorable d'un accès neuf : réussite au premier essai, bon droit obtenu immédiatement, événement à modifier déjà identifié, aucune sonde ou extraction supplémentaire nécessaire. La trace est créée dès le début de l'intrusion ; audit après 3 UT dans le profil de référence.

| Cycle | Action du joueur | État de la sécurité avec le décompte immédiat |
|---|---|---|
| 1 | Première étape de l'accès natif ; trace créée. | Premier cycle de délai écoulé. |
| 2 | Accès obtenu, deuxième étape terminée. | Deuxième cycle écoulé. |
| 3 | Préparation de Falsification de registre. | Audit à la phase environnementale : la preuve initiale est déjà exploitée. |
| 4 | Falsification terminée au plus tôt. | Trop tard pour annuler cet audit ou les copies déjà transmises. |

Une session préexistante peut permettre de traiter un nouvel événement à temps : la technique n'est pas universellement inutile. Mais son usage intuitif « entrer, puis nettoyer la trace de cette entrée » est impossible dans cette chronologie de référence, même sans échec.

**Recommandation à valider :** proposer un délai d'audit de référence de **5 UT**, en conservant les 2 UT d'accès et les 2 UT de falsification. Cela laisse une UT de marge dans ce cas favorable. Ce nombre reste un réglage de laboratoire, pas une attente imposée à tous les ennemis ; les témoins et alarmes déjà transmis restent des preuves indépendantes. Après décision sur REV-02, recalculer les audits avec leur convention déclarée, sans additionner deux délais de grâce par erreur. La durée d'audit n'est affichée au joueur que si une source lui permet de la connaître.

Il faut également spécifier si la falsification génère elle-même un événement suspect et quand celui-ci peut être examiné. La règle générique de trace à chaque tentative hostile ne doit pas créer une obligation infinie d'effacer la trace de l'effacement. Recommandation de principe : une modification déjà autorisée par les droits se résout comme une commande, sans nouvelle tentative d'intrusion ; un audit ultérieur peut néanmoins détecter la falsification selon ses règles explicites. Ne pas transformer cette recommandation en effacement automatique des soupçons.

Statut : défaut du cas d'usage dans le calendrier de référence ; paramètres et politique d'audit à approuver.

#### REV-04 — Vérifier les choix disponibles pour chaque version livrée

Sources : règles communes §§17.4 et 18.1 ; catalogue Reconnaissance et règles de dépendances ; vérificateur documentaire, groupe des prérequis.

Le catalogue complet permet les cinq choix de chaque discipline et chaque technique est atteignable. Le contrôle existant vérifie aussi un exemple de Reconnaissance sans Diagnostic énergétique. Cela ne prouve pas que tous les masques de fonctionnalités d'une version partielle conviennent.

Contre-exemple de version hypothétique sans traces, secrets ni diagnostic énergétique : REC-02, REC-03 et REC-08 deviennent indisponibles. Il reste quatre choix au total, même en comptant Analyse multiple : le cinquième rang ne peut pas être acheté légalement. Le nouveau modèle explore tous les ensembles d'apprentissage atteignables pour ce cas et trouve zéro possibilité au rang 5. Il ne prétend pas détecter automatiquement les systèmes disponibles dans le prototype actuel.

**Recommandation à valider :** rendre explicite la disponibilité des disciplines et de leur progression dans chaque version de test, puis vérifier chaque suite légale de choix, pas seulement un build témoin. Par défaut, différer l'ouverture d'une discipline incomplète jusqu'à ce que ses cinq choix soient réellement possibles ; si l'on veut un plafond temporaire inférieur, le décider et l'annoncer comme tel. Ne pas inventer de nouvelles techniques pour remplir les rangs et ne pas prendre les points d'un rang sans choix possible.

Cette vérification doit être rejouée lors du retrait d'une entrée comme MAN-07, de la désactivation d'une dépendance, de l'ajout d'une classe ou d'un chargement de sauvegarde. Elle ne justifie pas d'effacer des acquis existants silencieusement.

### 9.3. Un achat à revoir : Retraite méthodique

Sources : catalogue MAN-01 et MAN-07, annexe 16.5 ; description joueur conservée en l'état.

Sur trois retraits successifs ordinaires, Pas de dégagement répété manuellement et Retraite méthodique donnent tous deux trois déplacements, trois UT et +20 Esquive contre chaque interception admissible. MAN-07 maintient essentiellement la consigne. Ses règles ne lui donnent ni meilleure protection, ni action économisée, ni nouvelle possibilité de déplacement.

**Avis : ce bénéfice ergonomique seul ne justifie pas un choix spécialisé aussi tardif.** Une commande répétée ne devrait pas devenir pénible uniquement pour donner une valeur à son automatisation.

**Recommandation à valider :** intégrer ce maintien de consigne à Pas de dégagement sans achat supplémentaire et retirer MAN-07 des choix actifs, en conservant son identifiant dans l'historique. Cela ferait passer le catalogue à 94 entrées principales et 10 variantes, sans ajouter une capacité pour compenser artificiellement. Les cinq rangs et cinq choix de Manœuvre pourraient rester inchangés, sous réserve du contrôle exhaustif des prérequis après retrait. Aucune de ces modifications n'est appliquée par cette revue.

Alternative si l'utilisateur tient à une technique distincte : concevoir un véritable avantage tactique avec sa contrepartie, puis le valider. Ne pas lui ajouter discrètement une immunité aux interceptions ou un mouvement gratuit.

### 9.4. Ce qui relève surtout des essais d'équilibrage

#### Les attaques préparées doivent rester situationnelles

Modèles à 70 % de toucher en mêlée, Impact de référence 10, frappe ordinaire de 12, Frappe puissante de 18 sur un cycle de deux UT, aucune interruption ni action de soutien valorisée pendant R1 :

| Blindage testé | Frappe ordinaire : dégâts espérés par UT | Frappe puissante : dégâts espérés par UT |
|---:|---:|---:|
| 0 | 8,4 | 6,3 |
| 6 | 4,2 | 4,2 |
| 8 | 2,8 | 3,5 |
| 12 | 0 | 2,1 |

La frappe puissante devient intéressante contre le Blindage sans devoir dominer tous les échanges. Pour Tir visé, projectile de 10 sans Blindage, préparation puis tir, bonus +20 appliqué à une chance initiale non plafonnée dans les exemples :

| Chance initiale | Tir simple : dégâts espérés par UT | Tir visé : dégâts espérés par UT |
|---:|---:|---:|
| 10 % | 1 | 1,5 |
| 20 % | 2 | 2 |
| 70 % | 7 | 4,5 |

Le tir visé peut améliorer l'efficacité contre une cible très difficile et économiser des munitions ; perdre du débit contre une cible facile n'est pas en soi un défaut. **Recommandation : conserver ces profils pour les premiers essais**, avec ennemis mobiles, fenêtres de tir et risque d'interruption, avant d'augmenter leurs bonus.

#### Infection : une zone d'inefficacité plus large que le seul plafond de résistance

Infection contagieuse inflige 2 dégâts thermiques bruts par tic. Avec arrondi inférieur, elle tombe à zéro dès une résistance entière de **51 %**, pas seulement à 75 %. À 50 %, elle inflige encore 1 par tic ; à 75 %, zéro. Sans résistance, elle inflige au maximum 6 par hôte sur trois tics, hors limitation par expiration globale. Son nombre maximal d'hôtes ne garantit pas qu'ils existent, soient accessibles ou soient infectés avec succès.

Recommandation : garder l'absence de minimum universel de 1 dégât, voulue par l'utilisateur. Tester cette variante contre plusieurs distributions de résistances et mesurer les dégâts réellement infligés, le coût et les occasions de propagation. Si elle devient trop souvent un achat sans effet, réviser son montant, sa cadence ou son compromis de propagation explicitement ; ne pas contourner l'armure ou inventer un nouveau type de dégâts pour la sauver. Le typage thermique lui-même reste une proposition identifiée, distincte de la hausse de chaleur de Surchauffe.

#### Ressources, drones et Reconnaissance

- Deux drones avec une routine avancée chacun occupent 4 B sur 4 ; une commande demandant +1 B ne peut pas démarrer directement. C'est un arbitrage matériel possible, pas nécessairement un bug. Il faut cependant une commande de suspension/libération réellement accessible lorsque la capacité est pleine, avec temps, destinataire et effet local explicites ; sinon la contre-mesure indiquée dans l'interface serait inexécutable.
- La portée courte et les ressources des drones limitent le contrôle ; elles ne prouvent pas que plusieurs réactions de groupe restent équilibrées. Mesurer les actions et les dommages évités par tout le groupe, pas seulement par son propriétaire.
- Le rang 5 de Reconnaissance coûte 3 points pour un cinquième choix antérieur. Son absence de pouvoir exclusif ne le rend pas automatiquement mauvais : une option horizontale peut compléter un build. Conserver le tarif commun au premier essai et mesurer sa valeur ; ne pas ajouter un pouvoir de remplissage.
- Ingénierie doit rester optionnelle. Vérifier pour chaque route les réparations ordinaires, pièces, munitions et recharges réellement accessibles ; une formule de soin valide ne garantit pas que la carte fournisse le consommable.
- Les 21 points de compétences, 20 niveaux et cinq augmentations primaires forment un budget d'essai cohérent, pas une décision finale sur la durée d'une run. Les classes, leurs cadeaux de départ et l'XP disponible devront respecter ce budget ou le réviser explicitement.
- L'absence de réattribution reste à décider pour le jeu final. Pour évaluer les erreurs de build, comparer d'abord plusieurs configurations initiales séparées ; un outil de test permettant de changer de build n'impose pas une réattribution accessible au joueur.

### 9.5. Plan d'essais : critères et limites

Les scénarios de la section 6 restent la base. Les exécuter après clarification des règles, sur des fixtures petites et déterministes avant les parcours complets. L'ensemble des 105 textes constitue une cible documentaire, pas une obligation d'implémenter tous les systèmes simultanément.

| Lot | Cas minimaux | Critère d'acceptation |
|---|---|---|
| Temps et réponses | Charge posée par le joueur, Implosion ennemie avant/après son action, acteur ralenti, sauvegarde avant échéance. | La fenêtre annoncée existe réellement selon la convention retenue ; ni double explosion ni délai recréé au chargement. |
| Chaleur et réserve vide | Chaleur 115 avec attente, déplacement sans chaleur, Purge, action chauffante ; énergie zéro. | Les actions de secours légales restent possibles ; dégâts, refroidissement et dépenses continuent selon les règles. |
| Sécurité | Accès neuf/préexistant, tentative échouée, falsification à temps/trop tard, témoin et copie distante. | Un cas normal de falsification réussissable existe ; aucune suppression de preuve déjà transmise ni boucle infinie d'effacement de traces. |
| Achats et versions | Catalogue complet, plusieurs systèmes absents, acquis de classe et ancienne sauvegarde. | Chaque progression proposée possède ses choix éligibles ; aucun achat vide, prérequis retiré ou perte silencieuse d'acquis. |
| Défenses et contre-mesures | Coups faibles contre Blindage, tirs difficiles, dégâts mixtes, entrave et protection de fin. | Aucune défense comptée deux fois, aucun minimum de dégâts réintroduit, aucune neutralisation perpétuelle par répétition. |
| Perception | Même carte et commandes avec différents écrans, indices incertains, rapports de drone, échec hors vue. | Informations autorisées identiques ; aucune existence, position, défense ou mort cachée dévoilée par un aperçu, refus ou journal. |
| Économie et groupe | Réparation/démontage, rechargement réel, 4 B occupés, ordre et réaction de plusieurs drones. | Aucune duplication ; suspension possible sans canal libre fictif ; chaque unité paie ses actions et ressources. |
| Routes et difficulté | Plusieurs seeds fixes partagés entre les six builds, chemins de combat, discrétion et intrusion. | Une réponse accessible aux obstacles essentiels ; morts attribuables à des risques compréhensibles, sans prétendre garantir chaque mauvais choix. |

Mesurer par cas : résultat et cause, actions normales/réactions, temps exposé, dégâts bruts et finaux, PV/énergie/chaleur avant-après, coûts de contrôle, occasions d'utiliser une technique, raisons d'impossibilité et récompenses attribuées. Une technique jamais utilisable faute de contenu n'est pas jugée simplement « trop faible ».

Pour les routes complètes, comparer mêmes conditions de départ, mêmes ensembles de seeds et règles de choix explicites ; un bot rudimentaire ou une poignée de runs ne mesure pas à lui seul la difficulté pour un humain. Séparer pertes dues à une règle, manque de ressources, décision tactique ou information trompeuse. Aucun taux de victoire cible ni difficulté adaptative n'est ajouté.

### 9.6. État de sortie historique de cette revue

- **Fait :** revue documentaire, nouvelles reproductions ciblées, priorités et critères d'acceptation consignés ; contrôles antérieurs relancés.
- **À valider avant correction des règles :** portée de la sécurité thermique ; calendrier des retardateurs ; fenêtre et traçabilité des audits ; politique des disciplines incomplètes ; fusion proposée de Retraite méthodique.
- **À tester après implémentation :** barèmes, économie, valeur des cinq choix, propagation, réactions, équité des routes et difficulté réelle.
- **Hors périmètre :** classes détaillées, quêtes/scénario, nouvelles factions et capacités extérieures ; aucun achat de technique supplémentaire inventé.

Lecture ciblée du prototype confirmée pendant cette revue : [dégâts](../src/combat/damage.rs) contient encore le minimum de 1 par défaut et une pénétration retranchée au pourcentage de résistance ; [expérience](../src/progression/experience.rs) conserve les seuils [10, 25, 45, 70, 100, 140, 190]. Ce sont des écarts de raccordement déjà connus, pas des régressions introduites par les textes. Aucun test Cargo ou essai graphique n'est présenté comme exécuté ici.

À la sortie de cette revue, le point 8 était **réalisé pour sa revue documentaire**, mais pas clos au sens « jeu implémenté, équilibré et validé ». L'action alors proposée était d'approuver les corrections de principe ci-dessus, puis de les reporter ensemble dans les règles, les profils concernés, les textes affectés et leurs contrôles. Cet accord a depuis été obtenu et appliqué : voir section 10. Les formulations déjà validées individuellement ne doivent pas être réécrites sans nouvel accord.

## 10. Application des cinq corrections validées

L'utilisateur a accepté les cinq corrections proposées (« oui :) »). Cette passe les applique aux quatre documents du système et à leurs vérificateurs, sans développement de gameplay ni modification silencieuse des autres paramètres d'essai.

### 10.1. Décisions appliquées

| Correction | Règle désormais documentée | Limite conservée |
|---|---|---|
| Sécurité thermique | Refus seulement si l'action ajoute de la chaleur et dépasse la limite du mode : 100 ordinaire, 140 Surcadencement compatible. À H=115, attendre ou refroidir n'est pas bloqué pour ce motif. | Les autres préconditions, dégâts thermiques et refroidissement restent actifs ; aucune immunité n'est ajoutée. |
| Retardateurs annoncés | Le cycle d'armement C est exclu : événement de délai D en phase environnementale C+D, qu'il soit créé pendant les actions ou l'environnement. Charge à 1 UT, Implosion à 2 UT ; mines, effondrements et séquences suivent cette convention. | Les effets périodiques, expirations, cooldowns et audits ne gagnent pas ce cycle. Une réponse lente peut ne pas tenir ; une réponse à temps peut échouer. |
| Audit de référence | Délai porté de 3 à 5 UT. Trace en actions du cycle 1, accès fini en 2, falsification en 4, audit en fin de 5. Une trace créée dans l'environnement commence son décompte au cycle suivant, sans seconde grâce. | Falsifier avec un droit acquis ne relance pas une intrusion récursive. Témoins, copies et alarmes ne sont pas effacés ; un délai caché reste inconnu. |
| Retraite méthodique | Maintien de consigne inclus dans Pas de dégagement sans achat supplémentaire. MAN-07 sort des lignes actives et garde un identifiant historique réservé. Manœuvre conserve neuf techniques, cinq rangs et cinq choix. | Chaque pas conserve son temps et le seul +20 Esquive contre l'interception concernée. Aucun bonus cumulatif, mouvement gratuit ou remplacement de sauvegarde implicite. |
| Disciplines incomplètes | Ouverture différée dès qu'un parcours légal ne peut pas être prolongé jusqu'à cinq choix, après désactivation des prérequis transitifs. Contrôle des acquis de classe et de la compatibilité des choix sauvegardés. | Pas de plafond provisoire inférieur, de technique de remplissage ou de perte silencieuse d'acquis. Les fonctions natives du matériel restent distinctes. |

Références : règles communes §§13.3, 15.3, 16.2, 17.4 et 19.4 ; catalogue §15.7 et fiches MAN-01, DEM-03/04/08/09/10, INT-08 et GEL-08 ; textes thermiques, délais, disponibilité des disciplines et description de Pas de dégagement synchronisés.

Les échéances conservent leur catégorie, leur origine, leur date due et leur état d'exécution dans le contrat de sauvegarde : pas de réarmement au chargement ni d'événement exécuté deux fois. Il s'agit d'une exigence pour l'implémentation future, pas d'une migration effectuée dans le prototype.

### 10.2. Vérifications locales après application

- Contrôleur documentaire : **13 groupes réussis**, **104 profils et descriptions** (94 principaux + 10 variantes), **201 autres entrées rédactionnelles**, **49 lignes numériques** recalculées et **6 builds** légaux.
- Progression : tous les états d'apprentissage atteignables du catalogue complet permettent de finir les cinq choix ; Manœuvre reste complète après fusion. Les 128 masques hypothétiques des sept entrées de Reconnaissance sont comparés à une exploration indépendante. Cas négatifs : cinq fiches mais aucun troisième choix légal, dépendance transitive absente, acquis en double, rang/prérequis manquant et ancien MAN-07.
- Cas limites : **5 groupes de corrections vérifiés**, chaleur, retardateurs, audit, version partielle et fusion ; **2 groupes d'observations d'équilibrage conservés**, attaques situationnelles puis Infection/canaux. Les anciens constats de la section 9 restent l'historique du diagnostic, pas les résultats actuels du script.
- Les modèles de délais couvrent création pendant les actions du joueur, celles des PNJ ou l'environnement, détonations séquencées, sauvegarde simulée avant/à/après échéance et absence de double exécution. Les tics périodiques restent distincts. Une sérialisation JSON isolée ne teste pas les sauvegardes Rust.
- Les quinze infobulles approuvées individuellement sont conservées à l'identique ; comparaison d'empreintes avant/après sur leurs blocs de texte. Les 59 fichiers inspectés hors périmètre — sources, contenu, documents fondateurs, trame narrative et documentation moteur — restent inchangés par cette passe.

Les deux commandes de la section 3 ont été relancées. Aucun test Cargo, essai graphique, simulation de partie ou test réel de sécurité/sauvegarde n'est présenté comme exécuté.

### 10.3. État de sortie actuel

Le plan documentaire est terminé pour le système actuel, y compris le point 7 et la revue/correction documentaire du point 8. Les cinq corrections sont adoptées dans la conception ; leurs comportements ne sont pas encore implémentés ni validés dans le moteur par cette livraison.

L'équilibrage pratique reste à éprouver avec les scénarios des sections 6 et 9.5 : ressources, attaques préparées, résistance à Infection, valeur du cinquième choix de Reconnaissance et réactions de groupe. Les barèmes, le typage proposé d'Infection et la question de la réattribution gardent leur statut antérieur. Les classes détaillées, quêtes/scénario et capacités extérieures restent hors périmètre. Aucun commit, aucune migration ni intégration au jeu n'accompagne cette mise à jour.
