# WALK07-M45 à M50 — huitième lot accepté, hors du rendu actif

« ok on valide tout ca et on continue :) » accepte le reste du lot07 présenté,
soit M39–M43 avec la correction de M42. Les cinq sources et70 poses exactes
sont figées dans `visual-acceptance-creatures-lot07.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot07/`.
Le premier gel de M42 reste identique. Les41 cycles acceptés gardent leurs
pixels et calages. M12, M23 et M44 restent différés, hors de cet accord.

Le joueur accepte ensuite les six candidats M45–M50 par « ok je valide :) ».
Les six sources,84 poses natives et calages exacts sont figés dans
`visual-acceptance-creatures-lot08.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot08/`.

| Candidat | Identité existante | Anatomie à conserver |
|---|---|---|
| M45 | MON-104-0 | Quatre membres porteurs, dos pâle massif, tête basse et griffes sombres |
| M46 | MON-107-0 | Quatre pattes courtes, carapace ovale olive/brune à nervures claires et petite tête |
| M47 | MON-113-0 | Quatre grands appuis périphériques, corps central pâle et court appendice central conservé |
| M48 | MON-118-0 | Humanoïde sur deux mains et deux appuis arrière, capuche sombre et étoffe rouge |
| M49 | MON-122-0 | Humanoïde chauve sur deux mains et deux appuis arrière, tenue sombre et panneau ocre |
| M50 | MON-131-0 | Quatre appuis, corps sombre rouge/charbon et flammes déjà présentes sur la source |

Les noms sont des repères de revue et ne renomment pas le contenu du jeu.
M48 et M49 gardent deux bras et deux jambes ; leurs mains participent à la
locomotion, sans les transformer en animaux à quatre jambes identiques.
Les appuis périphériques de M47 restent distincts de son appendice central,
qui ne doit ni disparaître ni devenir une nouvelle longue jambe articulée.
Les flammes de M50 appartiennent à son dessin canonique : aucun nouvel effet,
gain de luminosité, aura, braise ou mécanique n'est ajouté.

## Sources et fabrication

Fabrication avec l'**outil image_gen intégré**, sans CLI de génération.
Les sources sélectionnées sont dans
`assets/source-art/player-walk-review-07/<identité>-cycle-lot08-r*.png`.
Les prompts complets et références sont dans `creature-jobs-lot08.json`,
`prompts/<identité>-lot08-r01.txt` et les journaux
`generation-creatures-lot08-r*.json`.

Les références actuelles exactes restent dans
`references/<identité>-native-current.png`. Les agrandissements servent à
l'identité ; les cycles acceptés guident les contacts au sol sans copier
leurs matériaux, appendices, équipement ou espèce. Les catégories techniques
anciennes ne remplacent pas l'examen du dessin.

Chaque source possède14 figures entières : un repos et six phases pour
chaque orientation latérale proche de la face. Le conditionnement extrait,
réduit uniformément et aligne les figures entières ; il ne greffe pas de
membres, ne retourne pas une tête isolée et ne déplace pas de bandes de
jambes. Un pixel opaque au bord extérieur de la source bloque son export.

## Placement et comparaison

« Dessin actuel » est la référence existante exacte. « Nouvel essai » est
le candidat proposé ; le détail×2 reprend les pixels de la même scène et
de sa grille. Les cases restent64×64px et les différences de taille restent
conservées. Les canevas128 ne servent qu'aux marges et grandes silhouettes.

WALK07-P01 centre la figure entière et son ombre si l'enveloppe du cycle est
moins haute que64px, avec un seul décalage fixe pour toutes les phases. Les
grandes figures gardent leur ancrage. Aucun recentrage indépendant des images
ou redimensionnement pendant la lecture n'est appliqué.

Trois phases accompagnent une case, puis les trois suivantes la suivante,
avec un vrai repos à l'arrivée.400ms reste le défaut,120/200/400ms restent
sélectionnables. Les sols sombre/clair/texturé, la pause et le pas manuel
sont conservés. Les520ms de pause de démonstration ne deviennent pas un délai
de jeu. Aucun déplacement haut/bas, tour, position logique ou collision
n'est modifié.

Les six sources sont déjà ordonnées gauche puis droite ; aucun échange de
séries directionnelles ou miroir n'est appliqué. La sélection est r01 pour
les six candidats M45–M50. Les mains des deux humanoïdes restent distinctes
de leurs pieds dans les libellés de revue et dans les dessins.

## Contrôles et portée

Les sources et poses natives sont examinées pour les appuis, directions,
matières, proportions et appendices. Les dimensions, empreintes et différences
de pixels ne prouvent pas à elles seules une bonne démarche ou une anatomie
correcte. Les contrôles du lecteur utilisent Node avec DOM/canvas simulé ;
aucune exécution réelle de ce lot en navigateur ou en jeu n'est revendiquée.

Preuves : `creature-packing-lot08.json`, `creature-validation-lot08.json`,
`renderer-checks.json`, `source-manifest.json` et `bastion-transfer-checks-r07.json`.
Comparaison native : `artifacts/player-walk-review-07/creature-lot08-comparison-native.png`.

Ce lot est accepté visuellement et reste hors du rendu actif. Les658 poses
des47 cycles acceptés, leurs sources et les cinq atlas WALK06 restent exacts.
Aucun dessin du rendu actif, règle, statistique ou identité n'est remplacé.
Le reste du bestiaire et ses variantes gardent leur propre revue.

Résultats du24 octobre :84 poses proposées,14 par identité, avec un vrai repos
et six dessins de marche distincts par direction. Les hauteurs natives sont
33–37,56–56,35–38,34–38,26–30 et45–48px ; les décalages verticaux fixes sont−13,
−4,−13,−13,−17 et−8px. Les six sources contiennent chacune14 figures entières,
sans pixel opaque au seuil128/255 sur leur bord extérieur. Les574 poses
acceptées, leurs sources et les cinq atlas WALK06 restent exacts.

Les2972 sélections du lecteur,300 fixtures du compositeur et601 instants de
cadence sont contrôlés ; les six aperçus courants restent sous1Mo, avec
compression sans perte des pixels. L'accord artistique du joueur est
enregistré séparément pour les six candidats présentés.
