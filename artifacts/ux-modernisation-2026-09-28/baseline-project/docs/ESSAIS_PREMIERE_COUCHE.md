# Première couche : disposition des rencontres et essais de parcours

Date : 27 septembre 2026. Génération 120.

Complément : [pression au contact et équipement ordinaire](PREMIERE_COUCHE_PRESSION_ET_EQUIPEMENT.md), avec 36 essais sans affixe et une distinction entre dégâts par coup et dégâts par action du joueur. Les résultats historiques ci-dessous sont conservés.

## Changement intégré

Les cinq lieux réservés utilisent désormais des points de placement propres à leur géométrie : travées des ateliers, coudes des conduits, abords des bassins, travées de galerie et approches latérales de la descente. Ces points varient légèrement selon la graine. Ils guident un tirage existant ; ils ne créent pas un combat supplémentaire ni une espèce obligatoire.

Les groupes de rencontre sont séparés d'au moins huit cases des autres groupes et des acteurs de population déjà placés. Les membres d'un même groupe restent proches. Les arrivées gardent une marge de douze cases pour cette couche de placement, en plus des contraintes propres aux profils. Ces distances portent sur les positions initiales, pas sur une interdiction de poursuite ou de regroupement pendant la partie.

Les caches et les commandes réservent leurs cases, mais ne sont plus traitées comme des acteurs pour l'espacement de ces rencontres. On ne place plus systématiquement les premiers groupes sur les trois caches. Le terrain n'a pas changé, les connexions restent ouvertes et aucune élimination n'est nécessaire pour utiliser la descente.

Les budgets de groupes, nombres de membres autorisés, pondérations d'espèces, PV, armures et dégâts restent ceux de maintenance/production. Les résultats individuels d'un nouveau tirage peuvent différer de 119. Les Vers cuirassés utilisent toujours leurs habitats humides ; les Riveuses conservent leur neutralité. Cette passe concerne les groupes de rencontre, pas une nouvelle scénarisation de toute la faune et de tous les PNJ.

L'option `first_layer_plan.encounters` nécessite les aménagements locaux. Les parties de génération 119 et antérieures la retirent avant génération et calcul d'empreinte. Les régions ordinaires hors du plan, les villes et l'approche de couche 2 conservent leur génération. Aucune ancienne partie n'est repeuplée.

## Protocole du diagnostic

Commande : `cargo test --locked --bin project-rl first_layer_expedition_fixed_sample_without_retries -- --ignored --nocapture`.

Échantillon fixé avant l'analyse : générations 119 et 120 × graines 0 et 1 × deux itinéraires × mêlée, distance et évitement, soit 24 essais. Aucune relance pour remplacer une mort par une réussite.

- Cartes et occupants réellement générés, du Nœud de maintenance à la Couronne de refroidissement, puis retour par le même chemin. Les connexions du diagnostic sont limitées au parcours comparé ; cette restriction n'existe pas dans la campagne.
- Kit fini identique : couteau de camp, fusil de patrouille, douze munitions et deux nécessaires de réparation. PV, corps, précision et règles de combat ordinaires, sans réserve de PV de diagnostic. Ce kit ne représente pas encore un équipement de référence validé pour entrer dans la couche 1.
- Le pilote connaît le terrain et l'itinéraire. Il combat uniquement des hostiles visibles, évite les attaques annoncées, peut ramasser les objets sur son chemin et commence son repli à six PV ou moins lorsqu'il n'a plus de soin. Il ne choisit pas automatiquement un meilleur équipement, n'utilise pas les talents ni les services urbains et ne fouille pas volontairement les annexes.
- Mêlée : frappe au contact ; distance : utilise le fusil hors contact tant qu'il reste des munitions, puis le couteau ; évitement : déplacement prioritaire, combat seulement quand il n'a plus de chemin. La disponibilité des armes est vérifiée séparément : le test de tir consomme réellement une munition.
- Une vie par essai. Maximum 1 200 actions ; douze attentes sans action provoquent un arrêt de pilote, distinct d'une mort ou d'un retour réussi. Les actions refusées sont rapportées, jamais comptées comme des victoires.
- Chaque transition contrôle l'absence de soin et de recharge gratuits. Pour la graine 0 du trajet court en 120, une reprise au premier contact est comparée à la simulation originale sur la commande suivante, événements et snapshot compris.

Un premier lancement a été interrompu parce que le pilote utilisait à tort la prévisualisation des attaques de zone pour autoriser ses attaques monocibles : aucune arme ne tirait. Ses sorties partielles ne sont pas des résultats d'équilibrage. Après correction et ajout d'un test de non-régression du tir, l'échantillon complet est exécuté avec les mêmes graines.

Ces essais ne donnent pas un taux de victoire de campagne. Ils ne valident ni les détours vers les annexes, ni la progression depuis la surface, ni l'intérêt de tous les lieux. Ils servent à repérer des coûts et des points de pression avant des essais joués.

## Résultats

Échantillon complet exécuté : 24 essais, aucun arrêt de pilote ni action refusée. Les morts sont des résultats de simulation, pas des échecs techniques du test.

| Disposition | Aller-retour complet | Repli avant destination | Mort | Arrêt de pilote |
|---|---:|---:|---:|---:|
| 119 | 5 | 3 | 4 | 0 |
| 120 | 5 | 1 | 6 | 0 |

Le nouveau placement n'améliore donc pas uniformément la survie de ce kit modeste. Sur cet échantillon, il conserve cinq allers-retours complets mais transforme certains replis possibles en morts. **La couche n'est pas déclarée équilibrée.** Aucun changement global de PV, dégâts, soins ou butin n'a été appliqué pour masquer ce résultat.

Points à travailler :

- La graine 0 expose les trois approches à des morts dans les pompes de production sur le long trajet ; des coups de huit PV sont observés pour un départ à vingt PV. Il faut examiner l'approche et le repli, pas seulement compter les monstres.
- Le trajet court de la graine 1 devient plus disputé dans les ateliers : le pilote de mêlée se replie, les deux autres meurent. Un espacement initial ne garantit pas que les poursuites ne convergeront pas.
- Le long trajet de la graine 1 reste très calme pour la mêlée et l'évitement : 760 tours sans perte de PV. Sa longueur ne suffit pas à lui donner un intérêt. Ces essais ne passent pas volontairement par les réserves ; leur rentabilité doit être testée séparément.
- Direction désormais confirmée : les abords doivent être abordables avec un équipement ordinaire sans bonus requis. L'exploration préalable ne garantit pas un meilleur tirage. Le petit kit reste donc un cas pertinent, même si les familles d'objets, la progression et les réserves devront aussi varier dans les essais suivants. La préparation n'est pas un prérequis de chance au butin.

### Relevé complet

| Génération | Graine | Trajet | Approche | Résultat | Tours | PV min. | Tirs | Soins |
|---|---|---|---|---|---:|---:|---:|---:|
| 119 | 0 | Court | Mêlée | Aller-retour | 615 | 4 | 0 | 2 |
| 119 | 0 | Court | Distance | Repli avant destination | 269 | 6 | 11 | 2 |
| 119 | 0 | Court | Évitement | Mort au retour | 551 | 0 | 0 | 2 |
| 119 | 0 | Long | Mêlée | Mort à l’aller | 152 | 0 | 0 | 2 |
| 119 | 0 | Long | Distance | Mort à l’aller | 164 | 0 | 12 | 2 |
| 119 | 0 | Long | Évitement | Repli avant destination | 432 | 2 | 0 | 2 |
| 119 | 1 | Court | Mêlée | Repli avant destination | 173 | 2 | 0 | 2 |
| 119 | 1 | Court | Distance | Aller-retour | 422 | 10 | 6 | 2 |
| 119 | 1 | Court | Évitement | Mort au retour | 341 | 0 | 0 | 2 |
| 119 | 1 | Long | Mêlée | Aller-retour | 760 | 20 | 0 | 0 |
| 119 | 1 | Long | Distance | Aller-retour | 768 | 20 | 7 | 0 |
| 119 | 1 | Long | Évitement | Aller-retour | 760 | 20 | 0 | 0 |
| 120 | 0 | Court | Mêlée | Mort à l’aller | 172 | 0 | 0 | 2 |
| 120 | 0 | Court | Distance | Aller-retour | 620 | 12 | 12 | 1 |
| 120 | 0 | Court | Évitement | Aller-retour | 607 | 8 | 0 | 2 |
| 120 | 0 | Long | Mêlée | Mort à l’aller | 129 | 0 | 0 | 2 |
| 120 | 0 | Long | Distance | Mort à l’aller | 165 | 0 | 12 | 2 |
| 120 | 0 | Long | Évitement | Mort à l’aller | 129 | 0 | 0 | 2 |
| 120 | 1 | Court | Mêlée | Repli avant destination | 56 | 4 | 0 | 2 |
| 120 | 1 | Court | Distance | Mort à l’aller | 131 | 0 | 12 | 2 |
| 120 | 1 | Court | Évitement | Mort à l’aller | 85 | 0 | 0 | 2 |
| 120 | 1 | Long | Mêlée | Aller-retour | 760 | 20 | 0 | 0 |
| 120 | 1 | Long | Distance | Aller-retour | 779 | 12 | 12 | 1 |
| 120 | 1 | Long | Évitement | Aller-retour | 760 | 20 | 0 | 0 |

## Reconnaissance avec repli immédiat

Commande : `cargo test --locked --bin project-rl first_layer_plain_kit_scout_retreat_sample -- --ignored --nocapture`.

Ce diagnostic distinct utilise la génération 120, les mêmes graines 0 et 1 et le même petit kit sans bonus. Le pilote d'évitement revient dès qu'un hostile est visible dans la première région extérieure ; si elle reste calme, il revient en approchant de son passage de sortie, sans rejoindre la région suivante. Il connaît toujours le terrain et l'itinéraire. Il ne teste donc ni un combat imposé ni une exploration à l'aveugle.

| Graine | Trajet | Résultat | Tours | PV min. | Tirs | Soins | Contacts relevés |
|---|---|---|---:|---:|---:|---:|---:|
| 0 | Court | Retour | 232 | 20 | 0 | 0 | 0 |
| 0 | Long | Retour | 79 | 20 | 0 | 0 | 1 |
| 1 | Court | Retour | 43 | 20 | 0 | 0 | 2 |
| 1 | Long | Retour | 203 | 20 | 0 | 0 | 0 |

Les quatre essais se terminent sans blessure, refus d'action ni arrêt du pilote, avec deux transitions chacun. Deux rencontrent des hostiles ; les deux autres restent calmes. Les contrôles de reprise au premier contact et de conservation des ressources aux transitions passent.

Sur cet échantillon, un repli immédiat est possible sans bon tirage d'équipement. Cela ne prouve pas que les combats sont accessibles ni que les morts précédentes étaient toutes évitables en décidant de rentrer plus tôt : les parcours et objectifs diffèrent. Les 24 essais précédents restent inchangés. Les pics de dégâts, la convergence des poursuivants et les essais avec plusieurs équipements ordinaires ou médiocres restent à traiter. Aucun paramètre de combat ou de génération n'a été modifié pour ce diagnostic.

## Validation technique de la disposition 120

- 64 graines du placement espacé : déterminisme, budget, profils d'acteurs inchangés, réservations, marges aux arrivées et séparation entre groupes.
- Huit graines du plan : cinq lieux concernés ; terrains, nombre de tirages de butin, villes et régions hors plan conservés. Les 72 cartes du test d'accessibilité restent valides.
- Empreinte historique 119 capturée avant modification : `16365257538551790727`, retrouvée exactement. Les empreintes antérieures et les reprises par snapshot/rejeu jusqu'à 120 passent.
- Suite complète : 749 tests moteur et 402 tests client réussis, deux diagnostics manuels ignorés par défaut. Le diagnostic comparatif ignoré a également été exécuté explicitement et a terminé ses 24 essais.
- `cargo check --locked --all-targets`, `cargo fmt --all -- --check`, `cargo build --locked` et `git diff --check` réussis.
- Deux captures natives inspectées : ateliers en 1280 × 800, pompes en 960 × 540. Le rendu et la perception restent ceux de 119 ; ce contrôle local n'est pas une partie jouée.
