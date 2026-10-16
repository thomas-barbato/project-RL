# Armée de drones et provocation du Gardien

15 octobre 2026. Révision de conception `DRONE-ARMY01` / `D66-TAUNT01`.

## Direction validée

Le joueur répond « oui tout à fait :) on va faire ca :) » à la recommandation
présentée. Le reçu est
[DRONE-ARMY01-ACCEPT01 / D66-TAUNT01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/drone-army-and-guardian-taunt-acceptance-01.json).

- Plusieurs Traqueurs, Artilleurs et Gardiens peuvent être déployés.
- L’Éclaireur et la Mule sont chacun limités à un exemplaire actif. La précision
  sur la Mule est validée sous `D57-LIMIT01-ACCEPT01`, dans
  `mule-single-active-acceptance-01.json` ; elle remplace uniquement l’autorisation
  précédente de plusieurs Mules simultanées.
- Une capacité de contrôle commune, progressant avec l’investissement dans la
  branche Drones, remplace le plafond fixe de cinq drones.
- Le Gardien remplace sa réduction de dégâts de proximité par une provocation
  automatique de zone. Les ennemis concernés le prennent temporairement pour
  cible en continuant à combattre. Les obstacles restent respectés.
- Plusieurs Gardiens ne doivent pas faire changer les ennemis continuellement
  de cible. Des ordres de groupe et des drones qui libèrent le passage sont retenus.

Le modèle D66-BODY02, l’icône D66-C03 et les pattes blanc os D58-PAL02 restent
validés. Les identifiants des compétences sont conservés. Cette révision ne
crée pas une nouvelle classe ou une compétence supplémentaire.

La persistance, la destruction à zéro PV, la conservation des PV et du vrai
chargement, l’absence de soin gratuit au rappel et l’absence d’entretien en
énergie par tour sont conservées. Les coûts restent ceux déjà validés ;
AGENT-CADENCE01 autorise une invocation dès l’action suivante, sans attente
intermédiaire. Invoquer n’ajoute pas d’attaque immédiate aux drones déjà présents.
Leur cadence autonome et la recharge propre de provocation restent conservées.
La progression des PV/dégâts et les passifs conservent leur
périmètre. L’Éclaireur et la Mule ne reçoivent pas d’attaque.

## DRONE-ARMY01-NUM01 — capacité proposée pour le premier essai

**Réglage provisoire validé le 15 octobre 2026** par la réponse « Valider ce réglage provisoire ». Reçu : `drone-army-capacity-acceptance-02.json`.

La capacité commence à **4 drones**. Chaque drone déployé utilise **1 place**,
y compris l’Éclaireur et la Mule. Traqueurs, Artilleurs et Gardiens partagent les
autres places libres ; les deux limites par modèle restent applicables.

La capacité gagne **1 place tous les 5 rangs appris au total dans la branche
Drones**. Les rangs de départ déjà appris comptent ; les bonus de rangs de
l’équipement ne comptent pas dans cet essai. Aucune nouvelle compétence de
capacité n’est ajoutée.

Avec **10 rangs appris**, la capacité est de **6 drones**. Avec **20 rangs**,
elle atteint **8 drones** ; avec **40 rangs**, **12 drones**. Les neuf compétences
actuelles possèdent chacune dix rangs apprenables : à **90 rangs appris**, cette
proposition permet **22 drones**. Ces exemples ne dispensent pas d’apprendre
chaque invocation utilisée.

Les neuf compétences comptées sont D56, D57, D58, D59, D66, DP01, DP02, DR01
et O03. La progression est fondée sur leurs rangs appris, pas sur le temps
passé à attendre ou sur les invocations répétées.

## D66-TAUNT01-NUM01 — provocation proposée pour le premier essai

**Réglage provisoire et libellé « Provoqué » validés le 15 octobre 2026** par « Valider pour le premier essai ». Reçu : `guardian-taunt-trial-acceptance-02.json`.

Au rang 1, la provocation couvre un **carré de 5 × 5 cases**, centré sur le
Gardien, soit deux cases autour de lui. Les cases et ennemis effectivement
atteignables par l’effet restent limités par la ligne de vue et les obstacles.

Les ennemis sont provoqués pendant **2 tours**. La provocation possède une
**recharge propre de 4 tours**, distincte de la recharge de l’invocation.

Le Gardien la déclenche automatiquement pendant sa phase normale lorsqu’une
cible admissible proche n’est pas déjà provoquée. Dans cet essai, il **provoque
à la place de son attaque** pour cette phase. Les autres phases admissibles
conservent son attaque au contact ; aucune attaque supplémentaire n’est accordée.

Une cible déjà provoquée garde son Gardien jusqu’à la fin de l’effet : une
seconde provocation ne change pas cette cible et ne rafraîchit pas sa durée.
Les cas de mort, de perte de visibilité et d’impossibilité de poursuivre la
cible restent à détailler avant intégration.

Le déploiement au rang 1 conserve **14 énergie** et peut être renouvelé dès
l’action suivante sous **AGENT-CADENCE01**.
Le rang du Gardien continue à améliorer ses PV et ses dégâts selon les règles
déjà validées ; aucune nouvelle courbe de provocation n’est approuvée ici.

## D66-TAUNT-MULTI01 — plusieurs Gardiens provoquent ensemble

Le joueur valide les règles présentées par « ok parfait :) ». Le reçu
[D66-TAUNT-MULTI01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/guardian-simultaneous-taunts-acceptance-01.json)
conserve le périmètre exact de cet accord. Il ne valide pas les chiffres proposés.

- Chaque monstre possède au plus une provocation, un Gardien source et une durée.
  Les effets, durées et icônes ne se cumulent pas.
- Un monstre déjà provoqué conserve son Gardien encore valide et sa durée
  restante. Une autre provocation ne le détourne pas et ne rafraîchit pas l’effet.
- Si plusieurs Gardiens provoquent ensemble une cible encore libre, le Gardien
  admissible le plus proche est retenu ; une égalité conserve un choix stable.
- Chaque monstre est traité séparément : plusieurs Gardiens peuvent donc attirer
  des ennemis différents lors d’une même phase.
- Si le Gardien source est détruit ou rappelé, sa provocation prend fin et le
  monstre reprend son ciblage normal. Il n’est pas automatiquement transféré à
  un autre Gardien ; une nouvelle provocation doit réellement le toucher.

Le marqueur reste unique au-dessus de la cible. La comparaison exacte des
trajets/distances, le traitement simultané des candidats, le départage technique,
les provocations sans cible gagnée, les pertes de vue et les compteurs restent à
préciser. Le reçu de proposition historique est
[D66-TAUNT-MULTI01](../docs/reviews/refonte-agent-2026-10-06/guardian-simultaneous-taunts-proposal-01.json).

## Rendu et détails restant à examiner

Le joueur demande ensuite un rendu de provocation et une petite icône au-dessus
des monstres concernés. La revue `ANIM-S47` présente `D66-TAUNT-A01`, une brève
impulsion cuivrée derrière les acteurs, et `D66-TAUNT-I01`, un petit symbole
rouge-orangé sans fond. Le marqueur suit la cible et demeure jusqu’à la fin de
l’effet ; il est unique même avec plusieurs Gardiens. Les deux candidats sont
validés après le contrôle final de la revue 103 par « ca semble fonctionner
correctement :) ». Le [reçu visuel](../docs/reviews/refonte-agent-2026-10-06/skill-animation-review-103/visual-acceptance-01.json)
conserve cet accord et les empreintes des rendus de la revue 94.
Les anciens segments de réduction ne constituent plus le rendu prévu du Gardien.
La revue 103 présente leur contrôle final dans deux fenêtres natives, avec les
mêmes images et chronologies de la revue 94 ; la taille du corps reste acceptée.

Le joueur demande ensuite que le Gardien occupe la hauteur d’une case pour le
rendre plus massif. Le candidat 104 / `D66-SIZE01` reprend la source D66-BODY02
avec un corps de 64 px de haut au lieu de 48 px. Le respect des proportions
donne 80 px de large, soit un débordement de 8 px sur chaque côté. L’export est
direct depuis la source détaillée dans un canevas transparent de 128 px, sans
agrandir l’ancienne tile. Le joueur valide cette taille par « parfait :) ».
Le reçu [D66-SIZE01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/skill-animation-review-94/gardien-size01/visual-acceptance-01.json)
conserve les empreintes du corps et de la comparaison native. Cet accord porte
sur la taille ; le rendu animé et le marqueur de provocation sont ensuite
validés séparément en revue 103. Le dessin original, les règles et le calendrier d’intégration dans
la campagne sont conservés.

Le choix des groupes, la présentation des ordres, les déplacements pour libérer
le passage, leurs budgets d’action, les refus, le suivi des instances et les
sauvegardes avec une armée dépassant la capacité restent à préciser. Les
règles de ciblage, résistances et grands ennemis ne reçoivent pas d’exception
implicite dans cette proposition.

Les anciens reçus restent historiques. Le joueur retire finalement le circuit
O03-A01 en revue 104 : ce passif permanent n'a pas d'animation supplémentaire.
La décision `O03-NO-CIRCUIT01` conserve ses bonus et les animations ordinaires
d'attaque ; l'effet de circuit proposé n'est pas validé.
Au moment de cette proposition, les quatre compétences de Tir DI03, DI04,
DI05 et DI07 restaient à montrer ; leur suivi est actualisé plus bas avec
les revues 99 et 100. La révision animée du Gardien est proposée.
B03 et la revue visuelle 90 gardent leur statut distinct.

Les valeurs détaillées sont conservées dans
[la proposition d’essai](../docs/reviews/refonte-agent-2026-10-06/drone-army-review-01/proposal.json).
Le catalogue du jeu, le combat et les sauvegardes n’ont pas été modifiés par
cette étape. L’intégration jouable reste prévue après la revue des animations.

## Présentation des réparations et du blindage — 15 octobre

Le joueur demande une réparation bleue. Maintenance des drones et Réparation
ciblée partagent cette direction de couleur dans la revue `ANIM-S48`. Le joueur
valide les deux rendus par « parfait c'est tres bien :) » ; le reçu
`skill-animation-review-95/visual-acceptance-01.json` conserve leurs empreintes.
Blindage amélioré reste un passif permanent :
à sa demande, il ne reçoit aucune animation sur le plateau. Sa petite icône
existante est réservée à la future fenêtre des compagnons. Cette décision ne
valide pas de nouvelle valeur de réduction et ne crée pas encore cette fenêtre.
Reçu : `drone-passive-visual-policy-decisions.json`.

## Qualité du Traqueur — reprise en revue

Le joueur autorise le redessin du Traqueur pour le 64 px, son ancien rendu
conservant une base 32 px agrandie. Le candidat 107 / `D58-BODY01` garde son
identité quadrupède, sa palette graphite/vert militaire et sa tête/pattes blanc
os. La revue `D58-RES64-01` présente le nouveau corps à taille native avant
remplacement dans les animations. Le joueur valide ensuite le candidat 107 par
« tres bien, je valide :) ». Le reçu
`skill-animation-review-96/visual-acceptance-01.json` fige la source, l’export
direct et la composition montrée. D58-BODY01 devient la référence des prochains
aperçus. Les règles, l’icône, l’épave et les banques actives sont conservées.
La suite des animations de Tir est suivie dans `REVUE_ANIMATIONS_COMPETENCES_AGENT.md` :
DI01 et DI02 sont validées en revue 98, DI03 et DI04 en revue 99.
DI05 et DI07 sont validées en revue 100. La revue 101 corrige le suivi :
AF05 et AF06 avaient déjà été validées. Le joueur demande ensuite une
révision visuelle de Contagion seule, AF06-A02, avec des tentacules organiques.
Cette révision est validée en revue 102 et devient sa référence visuelle.
Cette progression ne valide pas les autres
rendus encore en attente ni les chiffres de capacité ou de provocation.

## Éclaireur — export révisé accepté

Le joueur valide le candidat 108 / `D56-BODY01-SIZE02`, réduit directement à
70 × 40 px depuis la nouvelle source illustrée pour une case de 64 px. Les
trois pixels de débordement par côté font partie de la revue acceptée. Le reçu
`skill-animation-review-97/visual-acceptance-01.json` fige la source, l’export et
la composition native. Cette validation ne modifie pas les collisions, l’icône,
l’épave ou les règles. Ce dessin devient la référence des prochains aperçus.
