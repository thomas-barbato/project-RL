# Rue — FIXES-U01-02

19 octobre 2026. Le joueur signale des passages encore bloqués, notamment depuis
X 16 · Y 31, des PNJ cachés derrière un mur malgré leur présence dans le cercle,
et demande une direction grimdark / glauque pour le jeu.

**Suite de revue : FIXES-U01-03** conserve les preuves des corrections ci-dessous.
Le joueur apprécie les deux morts de GRIM-U01-01 mais demande un placement plus
cohérent. GRIM-U01-02 est refusé pour cette rue et retiré de l'aperçu courant.
Les nouveaux corps et matériaux sont proposés dans `../street-defects-review-03/`.
Les descriptions artistiques ci-dessous restent l'historique de l'essai r02.

## Corrections intégrées

Les 61 passages de COL-U01-01 étaient restés des candidats. La nouvelle demande
confirme le défaut de circulation à corriger. La liste explicite est désormais
appliquée aux nouvelles rues en génération 159, et aux rues déjà visitées après
vérification complète de l'ancienne sauvegarde. Les murs réels, voitures, gros
objets et limites demeurent solides. La case 16,31 était déjà praticable ; ce
sont ses voisines vers l'ouest qui formaient une barrière invisible.

Le trajet exact 16,31 → 7,31 est vérifié par neuf déplacements réels vers l'ouest,
avec neuf tours consommés. Les corrections sont des données séparées dans
`assets/maps/prison-street-expanded-01/collision-repairs.json` ; aucun dessin n'est
redécoupé et aucune baisse générale du seuil d'alpha n'est appliquée.

Le joueur veut que les personnages dans son cercle soient visibles même derrière
les murs. `circular_player_vision` applique cette règle à la perception réelle à
six cases, aux déplacements, aux transitions de zone et aux mises à jour de carte.
Les PNJ et ennemis utilisent la même perception du joueur. La vision propre aux
monstres et les tests de trajectoire des attaques continuent à tenir compte des
murs. Les toits fermés conservent leur couche opaque et leur masquage accepté.

## Sauvegardes et reprise

Les règles historiques restent inchangées pendant la vérification de la partie.
L'installation enregistre ensuite `EnableCircularPlayerVision`, puis si nécessaire
`RepairStaticZoneFloors`. Aucun tour ni tirage aléatoire n'est consommé par ces
commandes internes. Le cache de récupération et le rejeu reconnaissent le changement.
La carte encore différée est corrigée à sa première matérialisation ; une carte
déjà visitée peut être corrigée dans la zone active ou inactive. Les anciens
formats de petites rues ne sont pas remplacés par une grande carte.

## Proposition visuelle grimdark — en attente

La demande générale est inscrite dans `docs/DIRECTION_ARTISTIQUE_TILES.md`.
Le premier lot conserve le plan, la projection, les acteurs et les tailles :

- **GRIM-L02** : pénombre froide RGB [0.32, 0.34, 0.40], avec les sources réelles
  existantes et les couleurs fonctionnelles des portes.
- **GRIM-U01-01** : barricade de quarantaine détruite, victimes adultes et sang,
  128 × 64 px. Candidat précédent conservé, encore non validé.
- **GRIM-U01-02** : station médicale de contrainte abandonnée avec un adulte
  entièrement vêtu et cagoulé, machine et traces de sang, 96 × 80 px.

Les nouvelles sources sont entières et isolées sur fond transparent. L'export
conserve l'alpha et ne fait qu'un cadrage technique avec marge et une réduction
à taille native. Le corps sert de repère d'échelle ; aucune image n'est agrandie.
Les placements de revue sont respectivement [22,29] et [22,32], hors du trajet
de la rangée 31. Ils ne créent aucun acteur ni pouvoir. L'apparence est à valider ;
la validation ne déduit pas de nouvelles collisions ou interactions de ces objets.

L'outil intégré **imagegen** a servi à GRIM-U01-02. La source, l'export et le prompt
exact sont conservés dans
`assets/source-art/first-sector-review-01/u01-corrections-r02/` (`generation.json`).
La référence de projection est l'entrepôt accepté et celle d'échelle est le joueur.
Les images du lot et son nouvel éclairage ne sont pas actifs dans le lancement
normal, conformément à AGENTS.md : « Ils doivent être validés explicitement par
le joueur. » Aucun nouveau dialogue, nom d'acteur ou effet de combat n'est ajouté.

## Vérification

- 5 tests `player_vision` réussis : portée, règles historiques, adoption sans tour,
  sauvegardes visitées et différées, idempotence, rejeu et cache après correction,
  refus atomique d'une correction contenant une case invalide.
- 12 tests `prison_street` réussis, dont les 61 cellules raccordées, les objets
  réellement solides, les portes/toits et les anciennes rues 155/156.
- 1 test moteur `circular_player_vision` réussi : acteur derrière un mur visible,
  limites euclidiennes à six cases, septième case cachée, vision des PNJ et test
  de ligne de tir inchangés.
- Compilation debug, contrôle release, formatage et `git diff --check` réussis.
- Capture GPU du lancement normal : ancien état reproduit puis corrigé, 7 images,
  PNJ occulté avant et visible après sans déplacement ni tour, parcours réel de la
  rangée 31 et retour au camp. Les captures artistiques sont des variantes de
  rendu sans mutation du monde. Le PNJ et la rue grimdark sont examinés à taille
  native. Voir `capture-checks.json` pour positions et dimensions.

Le joueur avait le jeu ouvert pendant une première compilation ; Windows
refusait alors le remplacement du binaire. Il l'a fermé, puis la compilation
normale et le lancement des captures ont réussi. Aucun processus utilisateur
n'a été arrêté et aucune sauvegarde utilisateur n'a été utilisée ou réécrite.

```powershell
target/debug/project-rl.exe --ui-cold-world-fixes artifacts/street-defects-review-01/engine02
python tools/reviews/show_world_fixes_review.py
```

La revue réutilise `artifacts/street-defects-review-01/index.html`. Les contrôles
sont ciblés ; ils ne constituent pas une exécution complète de toute la suite.
