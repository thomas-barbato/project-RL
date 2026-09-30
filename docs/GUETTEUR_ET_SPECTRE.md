# Guetteur et Spectre — intégration v114

27 septembre 2026. Deux noms validés explicitement ; la suite de l'intégration
a été autorisée. À cette étape, les six autres noms du deuxième lot restaient
des propositions ; ils ont depuis été validés ensemble. Le
[Cracheur des mares](CRACHEUR_DES_MARES.md) poursuit l'intégration en v115.

## Rencontres

| Créature | Placement | Comportement |
|---|---|---|
| Guetteur | Réseaux, couches 4–7 ; un groupe de deux sur sol de ruines ou gravier | Repère le joueur et transmet sa dernière position vue à des voisins compatibles. Combat de contact ensuite. |
| Spectre | Corruption, couches 5–7 ; un individu sur boue ou broussailles | Annonce la case visée et une case latérale, frappe à son action suivante, puis reste immobile pendant deux occasions d'action. |

Les passages gardent une distance minimale d'apparition de 20 cases selon
les règles régionales existantes. Un habitat insuffisant peut empêcher un
groupe d'apparaître. Aucun effectif des autres biomes n'est augmenté.

Ce sont deux organismes étranges, pas des machines ni des humanoïdes.
Le Spectre ne traverse pas les murs. Aucun objet fabriqué, ressource sans
usage ou butin garanti n'est ajouté ; les caches restent indépendantes.
L'écho n'est ni un acteur ni une deuxième source d'expérience.

## Limites tactiques

Une alerte part uniquement d'un Guetteur non encore engagé qui voit réellement
le joueur. La furtivité, les murs, les lieux protégés et son territoire restent
respectés. Chaque émission touche au plus trois voisins sur deux liaisons,
de quatre cases maximum chacune, avec ligne de vue entre voisins. Seuls les
Guetteurs retransmettent ; seuls Guetteurs et Spectres hostiles, de même
affiliation et encore non alertés, acceptent ce signal. Les gardes, machines,
PNJ et animaux neutres ne sont pas enrôlés par ce mécanisme.

L'information est une position figée, pas un accès au joueur caché. Une
réception ne relance pas la durée d'une recherche en cours ; une alerte reçue
ne devient pas une nouvelle observation. Aucune invocation ni renfort créé.
Dans cette première tranche, les deux espèces vivent dans des biomes distincts :
les Guetteurs alertent surtout leur compagnon ; le Spectre sait aussi recevoir
le signal si une rencontre les réunit ultérieurement.

Le Spectre vise à quatre cases maximum. Son écho se trouve une case sur le
côté de l'impact, déterminée par l'axe initial. Les deux impacts sont simultanés,
distincts et infligent chacun les dégâts normaux de l'attaque aux occupants.
Une case obstruée, hors portée ou protégée ne reçoit pas l'écho. L'empreinte
est recalculée contre les obstacles actuels, comme les autres attaques annoncées,
mais ne suit jamais une cible qui se déplace. Déplacer le Spectre annule sa frappe.

## Puissance, présentation et persistance

Les profils de base sont volontairement fragiles : 8 PV pour le Guetteur,
10 pour le Spectre, sans armure innée. L'équilibrage v113 adapte ensuite leur
niveau, vitalité, dégâts et expérience à la couche, jamais au niveau du joueur.
Les essais des profils réels sur huit graines par profondeur restent à 14 PV
maximum ; ce plafond observé n'est pas une nouvelle règle universelle.

Inspection, ciblage, légende et CAPTEURS utilisent les noms **Guetteur** et
**Spectre**, avec des glyphes distincts (`q`, `z`). Le journal annonce simplement
l'alerte ou la double frappe. Les cases annoncées sont filtrées par la visibilité
du joueur et ne dépendent pas de F2. L'impact du Spectre utilise le rendu d'écho
terminal existant, avec un profil visuel propre, sans réutiliser des flammes
ou une décharge électrique.

La visée, la récupération et les recherches sont persistantes. Une attaque
déjà engagée peut finir dans une zone quittée, sans viser un joueur distant ni
publier d'événements à l'écran ; aucune nouvelle alerte ne part hors écran.

La génération passe à 114. Les parties de génération 113 et antérieure
conservent leurs populations et empreintes : l'adaptateur retire uniquement
`core:watcher` des Réseaux et `core:spectre` de la Corruption du monde de base.
Il préserve les autres espèces et mondes moddés. Une ancienne partie n'est pas
migrée automatiquement vers ce nouveau peuplement.

## Vérifications

- Moteur : portée, nombre de destinataires, relais borné, murs, protection,
  neutralité, dernière position connue, expiration sans rafraîchissement.
- Spectre : deux cases réelles, dégâts uniques, esquive, déplacement interruptif,
  protection, couverture, récupération et continuation hors écran.
- Client : profils issus du catalogue, niveaux par profondeur, noms et aperçu,
  instantanés et reprise déterministe, génération de régions et arrivées sûres.
- Compatibilité : reprise et rejeu des générations 113 et 114 ; anciennes
  empreintes couvertes par la suite de régression du client.

Diagnostics isolés : `--ui-cold-watcher`, `--ui-cold-spectre`,
`--ui-cold-spectre-impact` et `--ui-cold-spectre-recovery`, suivis d'un dossier
de capture neuf. Ces diagnostics n'écrasent pas la partie du joueur.

Présentation complétée ensuite par un [terrain local et des transmissions
visibles](GUETTEURS_DECOR_ET_SIGNAUX.md), sans modification de ces règles de combat.

Résultats de validation du 27 septembre : **734 tests bibliothèque et 367
tests client réussis**, un test manuel de suspension externe ignoré. Formatage,
compilation et contrôle des différences validés. Captures natives inspectées :
`target/ui-watcher-20260927/cold-start.png`,
`target/ui-spectre-20260927/cold-start.png` et
`target/ui-spectre-impact-live-20260927/cold-start.png`.
La dernière capture vérifie l'impact animé et l'état de récupération ; il ne
s'agit pas d'une campagne complète d'équilibrage en situation réelle.
