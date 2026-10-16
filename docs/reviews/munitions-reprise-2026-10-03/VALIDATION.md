# Munitions et points de récupération — 3 octobre 2026

La jauge de munitions utilise un ambre fixe (RGB 229, 183, 109), indépendamment
du thème. La réserve vide conserve son indication rouge. Les deux présentations
du HUD et les deux types de réserve passent par la même valeur résolue.
Les captures natives bleu, violet, vert, rouge à 1280×800 et bleu à 960×540
sont relues. `ammo-colors.json` vérifie les pixels des cinq barres.

## Récupération périodique

Le fil du jeu copie l'état du moteur, la présentation, le journal et le contenu.
Un seul travailleur encode cette copie, calcule les empreintes et écrit le
point de récupération. Les commandes suivantes ne modifient pas cette copie.
La file est bornée à une écriture ; elle rattrape les commandes accumulées dès
que le travailleur finit, selon le même seuil de cinq commandes. Le compteur
n'avance qu'après l'écriture réussie. Un échec conserve l'autre emplacement et
ne relance pas une écriture à chaque frame immobile.

Les deux emplacements alternés, les formats JSON/bincode et leur validation
restent inchangés. La création du premier point reste synchrone. La suspension
manuelle vérifie toujours la restauration rapide et le rejeu avant l'écriture.
Le nettoyage, la reprise, le remplacement de partie et la libération du verrou
de session attendent l'ancien travailleur. Une erreur de remplacement nettoie
aussi son fichier temporaire. En cas d'arrêt brutal, le point repris est le
plus récent dont l'écriture a abouti ; une écriture en cours n'est pas durable.

## Mesures

Toutes les mesures runtime sont **debug**, sur cette machine, sans compilation
ni tests simultanés. Il ne s'agit pas d'une mesure du runtime release.

Comparaison CPU isolée, médianes de huit échantillons en ms :

| Lieu | Ancien blocage complet | Copie seule | Copie et lancement |
| --- | ---: | ---: | ---: |
| recycling | 108.23 | 6.18 | 7.46 |
| hub | 112.04 | 7.87 | 8.11 |
| surface | 163.10 | 8.66 | 9.30 |
| depth-one | 167.83 | 9.13 | 9.84 |

Le coût total d'encodage et d'écriture demeure ; il est déporté. La copie coûte
plus avec plusieurs zones enregistrées. Les timings détaillés et le total de
l'écriture sont dans `after-actions/actions.json`.

Deux comparaisons natives dans le même binaire testent chacune 60 déplacements
en ville, 600 frames par mode, une commande toutes les dix frames et douze
points périodiques. `native-recovery-stages/timings.json` contient la mesure
la plus détaillée : le coût médian du point sur le fil du jeu passe de **114,17
à 6,11 ms**. Le p95 des frames, présentation comprise, passe de **91,30 à
32,96 ms** ; leur médiane reste proche (19,16 et 20,23 ms). Les octets finaux
du moteur, de la présentation et de la suspension sont strictement identiques
entre les deux modes, et le dernier déplacement figure dans le point durable.
La première série est conservée dans `native-recovery/timings.json`.

Des pics demeurent dans le traitement des tours : le p95 de `execute_command`
est de 86,60 ms en synchrone et 91,40 ms avec le travailleur. Le traitement des
événements coûte moins de 0,07 ms dans les deux modes. La fluidité globale ne
peut donc pas être déclarée entièrement corrigée. Le coût de l'IA et de la
simulation pendant la marche est la prochaine cible à profiler séparément.

## Compatibilité et contrôles

- `comparison.json` : état, journal, taille et empreinte des instantanés moteur
  identiques à l'ancien binaire dans les quatre lieux. La suspension complète
  finale diffère uniquement par l'identifiant de compilation.
- Quatre comparaisons de suspension capturée sur le travailleur : tous les
  octets JSON identiques à la voie synchrone du même binaire.
- `cargo test --locked --bin project-rl recovery_ -- --nocapture` : **17 tests
  réussis**, dont sept nouveaux tests de copie immuable, file bornée, échec,
  nettoyage/redémarrage, reprise, création de personnage et verrou à la fermeture.
- `library-tests.log` : **793 tests réussis**.
- `suspension-tests.log` : **36 réussis, 2 échecs préexistants, 1 ignoré**.
  Les échecs `suspension_replays_buy_sell_and_weighted_gamble` et
  `suspension_replays_the_complete_run_and_is_consumed_only_once` ont les mêmes
  causes que `../neon-integration-2026-10-03/baseline-failures.log` : offre
  marchande indisponible et tables de butin incompatibles dans les fixtures.
  Ils ne sont pas attribués à cette correction. La suite complète du client
  n'a pas été relancée ; aucune affirmation de réussite intégrale du client.
- Build debug, check de toutes les cibles, check release du client, formatage
  et `git diff --check` sur les fichiers concernés : réussis. Les avertissements
  d'import/méthodes inutilisés préexistants restent présents.
- Les deux séries natives et les cinq captures terminent sans erreur de
  diagnostic ; chaque série native écrit son rapport final après les assertions.
  Tous les diagnostics emploient des fichiers isolés des parties et réglages.

`before/` conserve les sources initiales et le binaire de référence.
`integration.patch` ne comprend que les changements de cette intervention.
Les propositions de combat, d'objectifs et de services en ville restent celles
de `../suite-demandes-2026-10-03/VALIDATION.md`. Aucun commit ni push.
