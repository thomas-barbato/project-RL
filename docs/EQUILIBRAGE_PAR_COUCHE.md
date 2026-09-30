# Équilibrage des rencontres par couche

27 septembre 2026 · génération 113 · première passe, à affiner en jeu.

Complément : les [essais de combat avec équipements ordinaires](ESSAIS_COMBAT_PAR_COUCHE.md)
mesurent maintenant les touches nécessaires, les dégâts après protection et
les possibilités de sortir des attaques annoncées. Ils conservent les valeurs
113 et distinguent ces mesures d'un véritable essai de progression.

## Règle retenue

La profondeur de la région détermine la puissance des rencontres générées.
Le niveau du joueur n'intervient jamais : revenir plus fort dans une ancienne
zone ne renforce pas ses habitants. Chaque individu reçoit une petite variation
déterministe, conservée avec ses statistiques dans la sauvegarde.

Les bandes actuelles sont celles du monde de prototype, pas une nouvelle
décision sur le nombre définitif de couches de la campagne.

| Profondeur | Niveaux des rencontres |
|---|---|
| 0 — surface | 1–3 |
| 1 | 4–7 |
| 2 | 8–11 |
| 3 | 12–15 |
| 4 | 16–19 |
| 5 | 20–23 |
| 6 | 24–27 |
| 7 | 28–31 |

Les profils lourds à attaque annoncée utilisent la partie supérieure de la
bande. Ce niveau est un indicateur de menace, pas un remplacement de l'espèce :
deux ennemis de même niveau peuvent conserver des capacités très différentes.

## Ce qui change réellement

- Les PV de base et les dégâts des attaques naturelles progressent avec la
  profondeur, relativement à la première profondeur du biome concerné.
- La variation individuelle représente 95–105 % des PV de base et 100–110 %
  des dégâts naturels à cette profondeur de référence, avec arrondi entier.
- Les bonus corporels existants restent ajoutés normalement. Les PV affichés
  ne sont donc pas nécessairement le simple produit des PV initiaux.
- L'expérience de base suit le coefficient de vitalité. Les renforts sans
  récompense restent à zéro XP ; les règles existantes de progression continuent
  de s'appliquer au niveau de menace obtenu.
- Le panneau de cible affiche le niveau effectivement attribué.

Les coefficients sont déclarés dans
[`simulation.json5`](../content/core/regional_worlds/simulation.json5), champ
`encounter_balance`. Ils sont validés au chargement : couverture de toutes les
profondeurs, bandes ordonnées et coefficients bornés. Un monde sans ce champ
garde son fonctionnement précédent.

La formule utilise le rapport entre le coefficient de la couche et celui de
la première couche du biome. Les valeurs écrites dans les fiches des espèces
restent donc des références locales, pas des valeurs de surface appliquées
aveuglément à tous les monstres souterrains.

## Rôles, équipements et groupes

Les armures, primaires, portées, déplacements, délais de préparation et de
récupération ne changent pas. Les escarmoucheurs non blindés restent non blindés.
Les composants de la Sentinelle gardent leurs durabilités : leur destruction
reste une réponse tactique à sa puissance croissante.

Les ennemis portant une arme utilisent déjà un équipement tiré selon la
profondeur. Leurs dégâts d'arme ne reçoivent pas un deuxième multiplicateur.
Les tables d'armes, affixes et butins ne sont pas modifiées par cette passe.
Cela ne remplace pas une calibration complète contre les équipements et
compétences réellement disponibles au joueur.

Le nombre de groupes, leurs positions, les plafonds et les délais des renforts
restent identiques. La menace du groupe augmente par ses membres, sans hausse
simultanée de la densité. Le budget de sélection de la faune conserve ses coûts
nominaux par espèce : ce n'est pas une somme des nouveaux niveaux affichés.
Cette passe n'ajoute pas un nouveau système de composition dynamique des groupes.

## Exemple mesuré dans le moteur

Même Sentinelle, même graine et même arène de diagnostic ; seuls les paramètres
de profondeur changent. Les PV comprennent le bonus de Résilience existant.
Les dégâts indiqués sont ceux résolus avant les résistances de la cible.

| Couche | Niveau tiré | PV effectifs | Dégâts électriques |
|---|---|---|---|
| 3 | 14 | 39 | 7 |
| 4 | 18 | 46 | 8 |
| 5 | 22 | 54 | 9 |
| 6 | 26 | 59 | 10 |

Son armure reste à 3, sa préparation à deux occasions d'action et sa
récupération à trois. Ce tableau décrit une graine, pas toutes les Sentinelles.

## Périmètre et anciennes parties

L'ajustement s'applique aux populations régionales générées, à la faune, aux
modèles de renforts et aux humanoïdes générés dans les friches de départ.
Les villes et leurs services, les contacts protégés et les personnages fixes
des démonstrations de quêtes ne sont pas réécrits globalement.

Le tirage utilise une graine séparée : il ne décale pas les placements, les
décors, les lieux ni les butins. Un marqueur persistant empêche de recalculer
la puissance à la revisite ou après une reprise. Le calcul n'est pas une
opération de soin et ignore les acteurs blessés.

Les générations 112 et antérieures retirent seulement ces nouvelles métadonnées
avant génération et calcul d'empreinte. Elles conservent leurs anciennes règles.
Il faut une **nouvelle partie** pour profiter de l'équilibrage 113. Le format
du cache moteur reste v6 ; les champs déjà sérialisés suffisent.

## Vérifications

Validation finale : **728 tests moteur et 358 tests client réussis**, aucun
échec. Un test manuel de suspension externe reste ignoré. Compilation,
`cargo check --locked --all-targets`, `cargo fmt --all -- --check` et
`git diff --check` passent. Aucun commit ni push effectué.

Les tests dédiés couvrent la progression sur 64 graines, la variation bornée,
la conservation des rôles, l'initialisation corporelle, les dégâts réellement
résolus, l'indépendance de l'ordre de génération et l'absence de double application.
Les régions sont comparées avec et sans l'ajustement sur quatre graines et
quatre biomes ; toutes les villes du monde sont également comparées.
Les continuations par instantané et rejeu sont contrôlées en versions 112 et 113.
L'empreinte historique 112 reste fixée à sa valeur relevée avant modification.

Deux diagnostics natifs isolés ont été inspectés :
[couche 3](../target/ui-layer-balance-three-20260927/cold-start.png) et
[couche 6](../target/ui-layer-balance-six-20260927/cold-start.png).
Le niveau de cible passe bien de 14 à 26, sans changer le cône annoncé ni sa
lisibilité ; leur contrôle automatique de contraste passe. Ces arènes utilisent
l'interface de départ pour comparer les spécimens : leur en-tête ne représente
pas un voyage dans une région réelle. Aucune sauvegarde du joueur n'est utilisée.

Reste à jouer une progression complète pour régler les combats trop longs,
les pics de dégâts et les rencontres combinées selon les équipements obtenus.
Les tests de cohérence ne prouvent pas à eux seuls un équilibrage final agréable.
