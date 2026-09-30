# Optimisation des actions et de la récupération — 28 septembre 2026

Deuxième passe, après [l'optimisation du rendu](OPTIMISATION_RENDU.md).

## Corrections

- Lors d'un déplacement, les tirs de réaction filtrent d'abord les personnages
  ayant une réaction préparée et couvrant la case d'arrivée. La perception n'est
  interrogée que pour ceux qui pourraient effectivement tirer. Auparavant,
  chaque déplacement recalculait le champ de vision de tous les personnages,
  y compris dans les zones hors écran.
- Une interrogation de visibilité sur une seule case utilise les mêmes règles
  de portée, murs, portes et angles, sans construire tout le champ de vision.
  Le suivi d'une position déjà connue évite aussi ce calcul inutile.
- Les trajets des personnages hors écran, des résidents et des ouvriers
  consultent un ensemble de cases occupées construit pour leur recherche. Les
  mouvements suivants voient toujours les positions actualisées ; aucun
  résultat de trajet n'est conservé entre deux actions.
- La récupération encode les données de présentation par référence, transfère
  les octets moteur en bloc et sérialise la présentation en un seul parcours.
  Les empreintes sont calculées au fil du texte `Debug`, sans en allouer une
  copie complète. Le format binaire et les empreintes historiques restent identiques.

La fréquence des points de récupération, les deux fichiers alternés, la
synchronisation des écritures sur disque et les vérifications par instantané
et par rejeu de la suspension manuelle sont conservées.

## Protocole reproductible

```powershell
cargo build --locked
.\target\debug\project-rl.exe --ui-cold-performance-actions artifacts\nouvelle-mesure-actions
```

Choisir un nouveau dossier de sortie. Le diagnostic utilise la graine de
démarrage et ses propres fichiers de récupération et de suspension. Il ne charge
ni ne remplace une partie de l'utilisateur. Il ne rend pas d'images pendant les
mesures. Ne pas lancer de compilation ou de tests simultanément.

Le parcours passe par le recyclage, le centre de départ, une région de surface
et la première couche. Chaque étape mesure huit actions d'attente successives,
puis huit répétitions de chaque opération de sauvegarde sur le même état.
La médiane rapportée est le cinquième échantillon trié. Les voyages avec
génération et la suspension finale vérifiée sont chacun mesurés une fois.
La suspension finale comporte 239 commandes acceptées.

Les rapports de référence sont `before/actions.json` et `verified/actions.json`
dans `artifacts/optimisation-actions-2026-09-28/`. Le script `compare.py` vérifie
les empreintes et les fichiers de sauvegarde complets, puis produit
`comparison.json`. Les autres relevés de ce dossier sont exploratoires.

## Résultats et validation

Compilation de développement, même machine et même parcours :

| Opération | Avant | Après |
|---|---:|---:|
| Tour au recyclage | 17,46 ms | 9,62 ms |
| Tour au centre de départ | 7,18 ms | 6,61 ms |
| Tour en surface, avec le centre hors écran | 45,14 ms | 8,50 ms |
| Tour dans la première couche, avec deux zones hors écran | 137,98 ms | 1,48 ms |
| Point de récupération au centre | 172,02 ms | 134,85 ms |
| Point de récupération en surface | 193,16 ms | 156,88 ms |
| Point de récupération dans la première couche | 204,53 ms | 175,20 ms |
| Génération et entrée dans la région de surface | 1 478,09 ms | 1 518,70 ms |
| Génération et entrée dans la première couche | 761,71 ms | 570,79 ms |
| Suspension vérifiée après 239 commandes | 17,65 s | 8,74 s |

Les opérations « tour » comprennent la commande et ses retours client, sans
dessin ni point de récupération. Les lignes de sauvegarde comprennent les
opérations complètes indiquées ; leurs durées ne doivent pas être additionnées
entre elles. La génération de surface n'a pas de gain démontré.

Les quatre états moteur et leurs instantanés binaires ont les mêmes empreintes
avant et après. La comparaison des deux fichiers JSON finaux ne trouve qu'une
différence : l'identifiant de compilation. Le journal, les données encodées de
reprise, la mémoire visuelle et toutes les autres valeurs sont identiques.

Les nouveaux tests comparent la visibilité d'une case au champ complet pour
les deux métriques, plusieurs rayons, les portes ouvertes/fermées, les murs,
les angles et les limites de carte ; ils vérifient aussi les empreintes `Debug`
historiques et l'égalité binaire de l'ancien et du nouvel encodeur de reprise.

Compilation, formatage et vérification du diff : OK. Validation finalisée le
30 septembre 2026 à partir de la suite exécutée le 28 septembre :

- 778 tests moteur réussis.
- 438 tests client réussis, 14 ignorés et 2 échecs.
- Les trois nouveaux tests de visibilité, d'empreintes et de format binaire
  passent, ainsi que les scénarios existants de tirs de réaction, de voyage,
  de récupération et de compatibilité des anciennes suspensions.
- Les deux échecs sont ceux déjà reproduits avant l'optimisation :
  `combat_balance_surface_roles_remain_distinct_with_starting_white_weapons`
  et `combat_balance_white_equipment_matrix_matches_real_impacts`.
  Ils attendent une attaque acceptée alors que le personnage dispose de zéro
  munition (`InsufficientMatter { required: 1, available: 0 }`).

Le journal complet est dans
`artifacts/optimisation-actions-2026-09-28/tests.log`. L'exécutable de
développement a été reconstruit avec les corrections retenues. L'essai sur le
stockage interne de la recherche de chemins n'avait pas de gain net sur ce
parcours et a été retiré ; l'algorithme de recherche de chemins reste inchangé.

## Limites

Ce parcours de développement ne représente pas toutes les graines, les combats
les plus denses ni une campagne très longue. Il ne mesure pas les FPS et ne
constitue pas une mesure de compilation `release`.

La récupération et la génération restent synchrones. La suspension volontaire
rejoue toujours tout le journal avant de fermer. Ces traitements peuvent encore
produire une pause, surtout quand le monde et le journal deviennent volumineux.
