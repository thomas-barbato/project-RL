# WALK07-r06 — côté anatomique et appuis opposés

Le 24 octobre2026, le joueur signale que la jambe métallique de la Traqueuse
semble changer de côté et que la démarche du Métamorphe est « ratée (mais presque
réussie) ». r05 n'est pas accepté. Le contrôle de copie exacte des anciens pixels
prouvait seulement leur conservation ; il ne prouvait pas la qualité du cycle.

## Deux essais, aucune nouvelle intégration

- **WALK07-C11-r06** : Traqueuse, tenue1. La jambe mécanique reste reliée à la
  même hanche, côté droit anatomique, à gauche de l'image dans les deux vues
  proches de la face. Le croisement des jambes est supprimé pour rendre cette
  continuité lisible. Le bas textile et sa botte sombre restent sur l'autre jambe.
- **WALK07-C25-r06** : Métamorphe, tenue5. Une phase de passage lève le pied
  gauche anatomique pendant l'appui droit ; l'autre lève le droit pendant
  l'appui gauche. Deux dessins entiers spécifiques remplacent les poses où
  le même pied restait au sol. Les poses complètes sont extraites et ramenées
  au gabarit par une mise à l'échelle uniforme, sans cisaillement ni greffe.

La tête, le torse et les pieds gardent deux orientations proches de la face.
Les six phases restent réparties sur deux déplacements, trois dessins par case,
avec une vraie pose de repos aux arrêts.400ms réglables. Les déplacements
verticaux, les règles et les cinq atlas WALK06 utilisés par le jeu ne changent pas.

À gauche de l'aperçu : **essai r05 précédent, sans arme**, pour comparaison.
Au centre : nouvel essai avec l'arme choisie, à taille native ; à droite : détail×2.
Les117 configurations (113 modèles et quatre configurations supplémentaires)
sont disponibles sur chacune de ces deux tenues. Les huit autres tenues de ces
familles attendent encore la reprise après cette revue.

## Sources et fabrication

Outil **image_gen intégré**, mode retouche/génération guidée, sans CLI. Les sources
sont conservées dans `assets/source-art/player-walk-review-07/` :

- `ANATOMY-10-r06b.png` : sept poses du bas seulement (orientation droite).
- `ANATOMY-10-r06c-left.png` : sept poses vers la gauche, correction du regard.
- `ANATOMY-24-r06b.png` : poses complètes de base, sauf les quatre passages.
- `ANATOMY-24-r06c-first-support.png` : deux passages avec appui droit.
- `ANATOMY-24-r06d-second-support.png` : première figure seulement, appui gauche
  en direction gauche ; la seconde figure ne respecte pas l'appui demandé.
- `ANATOMY-24-r06e-second-support-right.png` : appui gauche, direction droite.

Prompts exacts : `prompts/ANATOMY-10-r06b.txt`, `ANATOMY-24-r06b.txt`,
`CLASS-10-left-r06c.txt`, `CLASS-24-first-support-r06c.txt`,
`CLASS-24-second-support-r06d.txt` et `CLASS-24-second-support-right-r06e.txt`.
Les cibles, références d'identité et guides anatomiques sont dans `references/`.
Les entrées structurées sont `anatomy-jobs-r06b.json`,
`anatomy-detail-jobs-r06c.json` et `anatomy-detail-job-r06d.json`.
`source-manifest.json` donne les empreintes des sources effectivement sélectionnées.
Le premier essai `ANATOMY-10-r06a-rejected.png` est écarté : il répétait le croisement.

Le conditionnement utilise chaque figure entière. Les jambes ne sont ni retournées,
ni déplacées par bandes, ni prélevées sur un autre personnage. La découpe des bras
et des doigts reste arrêtée avant les cuisses : ces pixels restent dans le corps
fixe, indépendamment de l'arme. Les armes sont les images réelles du jeu avec
leurs prises et transformations rigides.

## Vérifications et limites

`anatomy-checks-r06.json` vérifie3276 compositions de corps fixe :117 équipements
×14 poses×2 tenues. Les huit passages (deux pieds×deux sens×deux personnages)
ont bien un pied planté et l'autre complètement au-dessus du sol. La garde
minimale est **un pixel natif** ; le talon du Métamorphe allant vers la droite
garde un pixel libre et sa pointe davantage. Les valeurs mesurées sont conservées.
La continuité du métal est contrôlée sur les14 poses de la Traqueuse et les
planches entières ont été examinées à zoom entier. Ce sont des contrôles ciblés,
pas une validation artistique automatique du mouvement.

Les exports Rust de recettes et compositions ont réussi. La prévisualisation
est comparée à204 compositions exportées (écart maximal2/255 aux bords filtrés).
Le contrôleur JavaScript exécuté dans un DOM simulé conserve les directions,
six phases, repos, pas unique, réglages et601 instants de la référence temporelle.
Les six fragments restent chacun sous1Mo. Aucun navigateur ni partie n'a été
exécuté pour r06. Le joueur doit encore juger l'enchaînement animé.

Planches examinées : `artifacts/player-walk-review-07/CLASS-10-r06-anatomy-poses-x2.png`,
`CLASS-24-r06-anatomy-poses-x2.png`, leurs agrandissements×4 et
`cycle-repair-equipment-native.png`. Les images sources et décisions durables
restent séparées des sorties temporaires de revue. Aucun fichier de carte ou
préférence d'éditeur n'est touché.
