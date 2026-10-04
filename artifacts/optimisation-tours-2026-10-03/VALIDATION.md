# Pics pendant les tours — 3 octobre 2026

La cause mesurée est la recherche de chemins des ouvriers des installations.
Sur la séquence de ville, l'IA de combat coûte quelques millisecondes ; certaines
recherches vers un accès d'installation épuisent **15 000 expansions** et
prennent 50 à 65 ms à elles seules. Par exemple, depuis (29, 64), le trajet vers
(45, 66) est trouvé en 0,30 ms, tandis que le candidat voisin (47, 66) épuise la
limite en 64,97 ms sans résultat. Ces deux candidats font partie du même choix
d'accès. `profile-paths/turns.json` conserve les requêtes et leur résultat.

## Correction

Le choix conserve le classement historique (longueur du chemin, coordonnées
lexicographiques) et le budget de chaque recherche. Les voisins sont essayés
selon leur distance minimale. Un candidat n'est plus recherché lorsqu'aucun
chemin vers lui ne peut battre le meilleur chemin déjà obtenu.

Avant de rechercher un candidat, une exploration inverse de sa zone statique
est limitée à 128 cases. Seule une petite zone entièrement explorée sans y
trouver l'ouvrier permet de rejeter le candidat comme inaccessible. Si
l'exploration atteint l'ouvrier ou sa limite, le calcul A* historique s'applique.
Les portes fermées que l'ouvrier peut ouvrir sont traversables dans cette
vérification, les portes verrouillées ou hors tension ne le sont pas. Le calcul
est refait à chaque demande ; aucune réponse périmée n'est gardée en cache.
Les acteurs bloquants et les règles de déplacement restent dans la recherche
normale. Le précontrôle ne raccourcit jamais le budget d'une recherche valide.

Cette correction concerne seulement le choix d'une case voisine d'installation.
L'algorithme A* public est inchangé ; ses seules modifications sont les
annotations de mesure debug. Aucun stockage indexé ni cache de chemins n'est
ajouté. `action_profile.rs` et ses annotations sont exclus des builds release,
et ne sont activés que pendant le diagnostic explicite en debug. Les données
de mesure ne figurent jamais dans l'état du jeu ou ses sauvegardes.

## Mesures

Mesures **debug**, sans compilation ni tests simultanés, sur cette machine.
Chaque séquence comprend 80 commandes identiques, avec récupération périodique
et rendu désactivés. Les colonnes sont en millisecondes :

| Séquence | Médiane avant | Médiane après | p95 avant | p95 après | Maximum avant | Maximum après |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| hub-walk | 3.10 | 2.60 | 73.32 | 3.83 | 88.57 | 4.84 |
| hub-wait | 1.63 | 1.56 | 68.63 | 1.91 | 74.63 | 2.64 |

`profile-before/` contient la mesure avant toute correction ; `profile-after/`
montre l'effet limité de la première élimination des candidats moins bons.
`profile-flood/` contient le premier essai du précontrôle de connectivité.
`profile-final/` correspond au code final. Les modes avec et sans instrumentation
produisent chacun exactement les mêmes octets de suspension.

Le test natif utilise 60 déplacements, une commande toutes les dix frames,
600 frames par mode, 1280×800, et douze récupérations périodiques. Le binaire
initial conservé dans `before/` est relancé dans `native-before/` ; le code final
est mesuré dans `native-final/`. Avec récupération en arrière-plan active,
le p95 du traitement d'un déplacement passe de **98,18 à 4,43 ms**, et son
maximum de **99,83 à 4,78 ms**. Récupération comprise, le maximum du traitement
de commande final est de **8,87 ms**.

Le p99 des frames avec présentation passe de 117,61 à 30,78 ms ; leur maximum
de 128,36 à 33,20 ms. Le rendu reste actif pendant la récupération. Les médianes
de rendu varient entre les processus et ne sont pas attribuées à cette seule
correction. Ces chiffres ne constituent pas une mesure runtime release, ni
une garantie de coût maximal sur toute carte possible.

## Comportement et sauvegardes

- Comparaison différentielle de **1 440 cas** avec la sélection exhaustive
  précédente : terrains, portes fermées/verrouillées/hors tension, acteurs
  bloquants, égalités de chemins et cinq budgets. Les octets de carte, de
  registre d'acteurs et les événements du déplacement sont identiques.
- Test de rejet des candidats ne pouvant gagner l'égalité : une recherche suffit.
- Test des portes modifiées après une vérification et d'une zone dépassant la
  limite du précontrôle : retour au calcul normal correctement conservé.
- `comparison.json` : les suspensions complètes après les 80 déplacements et
  80 attentes diffèrent de celles du binaire instrumenté initial uniquement par
  l'identifiant de compilation. Instantanés moteur, présentation, RNG, journal,
  équipements et compteurs sont donc conservés dans ces scénarios.
- Le diagnostic des quatre lieux reprend exactement la séquence précédente
  (recyclage, ville, surface, première profondeur). Les empreintes d'état et
  d'instantané moteur ainsi que les nombres de commandes sont identiques.
  La suspension finale complète diffère uniquement par l'identifiant de build.
- Le diagnostic natif final vérifie également les octets moteur, de présentation
  et de suspension identiques entre récupération synchrone et en arrière-plan,
  ainsi que la présence du dernier déplacement dans le fichier durable.
- La suspension finale du diagnostic d'actions vérifie sa restauration par
  instantané et par rejeu avant écriture, comme dans le jeu.

## Validation

- `library-tests.log` : **796 tests moteur réussis**, dont les trois nouveaux
  tests d'accès d'installation.
- Build debug final, vérification de toutes les cibles, vérification release
  du client, formatage et contrôle des différences : tous réussis. Journaux
  `build-final.log`, `check-all-targets.log`, `check-release.log`.
- Conservation des octets historiques d'instantané et restauration sans
  rejouer le journal : **les deux tests réussissent**. Journaux `snapshot-bytes-test.log` et
  `snapshot-restore-test.log`.
- Les diagnostics finaux `profile-final`, `native-final` et `actions-final`
  quittent avec le code 0. Le binaire initial écrit également son rapport natif
  après toutes les assertions, sans erreur de diagnostic.
- La suite complète du client n'est pas relancée. Les deux échecs préexistants
  de fixtures de commerce/reprise restent documentés dans
  `../munitions-reprise-2026-10-03/VALIDATION.md` ; aucune affirmation de réussite
  de cette suite complète.

Tous les fichiers de diagnostic sont isolés des parties et réglages du joueur.
`before/` conserve les sources initiales et le binaire, et `integration.patch`
contient uniquement les changements de cette intervention. Aucun commit ni push.
