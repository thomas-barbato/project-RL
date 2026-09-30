# Repli à couvert et armes trouvées

Génération 122, 28 septembre 2026. Ce travail concerne les pilotes de diagnostic : il n'ajoute pas de déplacement automatique au joueur et ne change ni l'IA ennemie, ni les dégâts, ni les tables de butin.

## Comparaison fixée

Référence : les six parcours avec détour du [diagnostic continu](EXPEDITION_CONTINUE_SURFACE_PROFONDEURS.md), graines 0 et 1, approches mêlée, distance et évitement. Ils avaient produit un aller-retour et cinq morts. Le pilote historique est conservé dans le code ; les résultats de référence restent ceux du relevé précédent.

Deux variantes reprennent ces mêmes six départs :

1. **Couvert** : chercher un abri avant de se soigner sous le feu ou pendant le repli.
2. **Couvert et armes** : même comportement, avec remplacement d'une arme par une meilleure version effectivement récupérée du même modèle.

Les PV de départ, les deux soins, les douze munitions, les tables de butin, les seuils de soin (12 PV) et de repli (6 PV sans soin), les règles de progression et les parcours restent ceux du protocole précédent. Des commandes différentes peuvent modifier les tirages suivants : graines identiques ne signifie pas butin final garanti identique. Une mort n'est pas remplacée par une nouvelle graine. Le retour n'est réussi qu'une fois la case de départ rejointe. Les régions sont réellement générées et chaque transition conserve les ressources puis passe par un snapshot moteur.

Commandes :

```powershell
cargo test --locked --bin project-rl expedition_decisions_tests -- --nocapture
cargo test --locked --bin project-rl continuous_cover_sample_ -- --ignored --nocapture --test-threads=4
```

## Recherche de couvert

Le pilote évalue les cases menacées par les attaques des seuls hostiles actuellement visibles : portée, ligne de vue et zones protégées. Il cherche une case hors de ces attaques, atteignable en six déplacements cardinaux au plus. Le coût favorise les chemins les moins exposés, puis la proximité de la destination. Les acteurs, les murs, les portes fermées et les zones d'attaque annoncées bloquent cette recherche locale.

Une attaque potentielle n'est pas une prédiction de l'action ou du jet de l'ennemi. Les adversaires peuvent bouger pendant le déplacement. Le pilote connaît toujours le terrain comme dans le diagnostic initial ; il n'accède pas aux positions des adversaires invisibles. Sans abri trouvé, il conserve ses décisions ordinaires. Les passages accessibles immédiatement restent utilisables et les esquives d'attaques annoncées gardent leur priorité.

La recherche de couvert dispose d'un budget de **24 déplacements par visite de région** (quatre fois l'horizon local de six pas). Une fois ce budget dépensé, le pilote reprend ses décisions ordinaires ; les esquives de zones annoncées restent prioritaires et ne consomment pas ce budget. Seul un changement de région le réinitialise, pas un soin ou un pas vers l'objectif. Il s'agit d'une borne de diagnostic explicite, pas d'un réglage d'équilibrage ni d'une démonstration de stratégie optimale.

Le premier essai, sans cette borne, a exécuté huit vies : six morts sur la graine 0 et deux arrêts techniques sur la graine 1, en mêlée. Ces deux personnages atteignaient la ville de couche 2 puis le secteur initial, mais alternaient entre abri et trajet jusqu'à la limite de 2000 actions, encore vivants à 11 PV. Leur armure absorbait les derniers tirs. Ils ne sont comptés ni comme morts ni comme retours réussis. La comparaison finale rejoue les douze vies avec le même garde-fou pour les deux variantes.

## Exploitation du butin

Le pilote examine son inventaire seulement lorsqu'aucun hostile n'est visible. Il remplace une arme par une instance du **même modèle**, non déjà équipée, si elle conserve tous les bonus numériques actuels et en améliore au moins un. Il ne fait pas de conversion arbitraire entre précision, PV et attributs. L'effet spécial doit rester identique. Le poids ne détermine pas ce choix. Les instances sont examinées dans un ordre déterministe ; un remplacement ultérieur doit encore être une amélioration sans perte.

Le remplacement utilise la vraie commande d'équipement et consomme son action. Les munitions sont partagées par modèle d'arme dans le moteur actuel : changer de copie ne recharge pas le fusil. Augmenter les PV maximum par équipement ne soigne pas les PV courants. Les bonus restent ceux de l'objet tiré, sans nouveau jet ni modification de la base.

Cette politique reste volontairement limitée : pas de choix entre deux familles d'armes, pas d'utilisation des services, pas d'apprentissage de compétences et pas d'optimisation d'affixes présentant un compromis. Elle ne représente pas toutes les décisions possibles d'un joueur.

## Vérifications isolées

- Une menace à distance et un mur en angle : le pilote choisit un pas qui coupe la ligne de vue, respecte un acteur bloquant et une zone annoncée, et n'invente pas une menace absente de la liste visible.
- Le budget autorise exactement 24 déplacements à couvert, bloque les suivants, puis redevient disponible à l'entrée d'une région.
- Un fusil réellement équipé après une attaque : onze munitions restent onze, l'inventaire est conservé et le bonus de Résilience porte le maximum de 20 à 30 PV sans restaurer les PV courants. La reprise du snapshot et l'absence d'alternance vers la copie ordinaire sont vérifiées.

L'objet avec bonus du second test est un montage fonctionnel explicite. Les parcours longs n'accordent aucun objet de cette manière : ils utilisent exclusivement le butin acquis pendant le trajet.

## Relevé final

Les douze vies aboutissent à **quatre retours complets et huit morts**, sans arrêt technique ni refus de commande non traité. Chaque variante obtient deux retours sur six départs, contre un sur six dans la référence historique. Les deux variantes rejouent les mêmes départs : ce ne sont pas douze situations indépendantes ni une estimation du taux de survie des joueurs. Toutes les morts se produisent en surface.

| Graine | Approche | Référence | Couvert | Couvert et armes | PV finaux des deux variantes | Déplacements à couvert |
|---|---|---|---|---|---:|---:|
| 0 | Mêlée | Mort avant descente, t363 | Mort avant descente, t367 | Mort avant descente, t367 | 0 | 18 |
| 0 | Distance | Mort au retour, t1085 | Mort au retour, t1097 | Mort au retour, t1098 | 0 | 10 |
| 0 | Évitement | Mort avant descente, t360 | Mort avant descente, t364 | Mort avant descente, t364 | 0 | 18 |
| 1 | Mêlée | Mort avant descente, t332 | Aller-retour, t1621 | Aller-retour, t1621 | 11/23 | 65 |
| 1 | Distance | Aller-retour, t1586 | Aller-retour, t1590 | Aller-retour, t1590 | 26/26 | 41 |
| 1 | Évitement | Mort avant descente, t160 | Mort avant descente, t207 | Mort avant descente, t207 | 0 | 19 |

Les déplacements à couvert sont comptés sur l'expédition entière ; le budget de 24 s'applique à chaque visite de région. Un seul changement d'arme est effectué : graine 0, approche à distance, variante « couvert et armes ».

### Lecture des résultats

Sur la graine 0, les deux variantes conservent les trois issues fatales. La mêlée meurt au tour 367, l'évitement au tour 364, avant la première descente ; les deux ont consommé leurs soins et effectué 18 déplacements à couvert. Le personnage à distance atteint la ville de couche 2 puis meurt au retour, au tour 1097 avec couvert seul, 1098 avec changement d'arme. Il a consommé ses douze projectiles et ses deux soins, et effectué dix déplacements à couvert. Gagner quelques tours ne constitue pas un repli réussi.

Dans ce dernier cas, le fusil trouvé avec Résilience +2 et Traitement +2 est effectivement équipé par la variante « couvert et armes ». Le maximum passe de 23 à 33 PV, sans hausse des PV courants. L'entrée dans la ville de couche 1 se fait à 7/33 PV, contre 7/23 avec couvert seul. Le maximum accru ne remplace pas une réserve de soins ; le test n'établit pas que les bonus seraient inutiles dans d'autres situations.

Sur la graine 1, la mêlée réussit désormais le trajet entier au tour 1621, à 11/23 PV, dans les deux variantes. Elle atteint la ville de couche 2, équipe une armure trouvée puis revient jusqu'à sa case de départ. Elle a utilisé deux soins et 65 déplacements à couvert répartis entre les visites de régions, sans tirer ni changer d'arme. Le pilote initial mourait au tour 332 avant la descente. Le changement de décisions modifie aussi les combats, la progression et le butin rencontrés : ce n'est pas une mesure isolée d'un nombre de PV « économisés par couvert ».

La distance sur cette graine conserve son retour réussi au niveau 3 avec 26/26 PV et deux soins, après douze tirs et l'équipement d'une armure trouvée. L'évitement reste fatal malgré 19 déplacements à couvert et deux soins ; le maximum observé reste de dix dégâts après une commande. Le couvert ne remplace donc pas un choix de repli assez précoce et ne garantit pas une fuite sûre.

## Conclusion et suite

La prise en compte du couvert améliore une des six situations de référence. Exploiter une meilleure copie d'arme fonctionne sans tricher sur les ressources, mais ne change aucune issue ici. Ce petit échantillon ne justifie ni de déclarer l'équilibrage validé, ni de réduire globalement les statistiques des monstres.

La prochaine comparaison utile est un **repli avant l'épuisement des réserves**, puis, séparément, l'usage des **services de soin en ville avec les crédits réellement disponibles**. Le pilote actuel peut continuer à avancer avec peu de PV tant qu'il n'a pas atteint le seuil fixe de six PV sans soin ; il traverse aussi les villes sans utiliser leurs services. Ces limites doivent être distinguées d'un problème de puissance ennemie.

## Validation locale

- Trois tests isolés de décisions réussis.
- Quatre groupes longs réussis, soit douze vies complètes : 623,06 secondes. Les morts attendues restent des résultats du diagnostic, pas des tests échoués.
- Deux tests ordinaires du parcours continu et deux tests client de progression des PV réussis.
- `cargo check --locked --all-targets`, contrôle de formatage et contrôle des différences réussis.
- Treize groupes de validation des documents de statistiques réussis ; ils ne valident pas à eux seuls l'équilibrage ni ce relevé de parties.
- Pas de suite complète, de nouvelle compilation native ou de contrôle visuel : cette passe n'a modifié ni l'interface ni les règles de jeu. Aucun commit ni push.
