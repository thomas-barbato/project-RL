# WALK07-M39 à M44 — septième lot, cinq candidats acceptés

« ok on continue :) » accepte les six candidats M33–M38 du lot06. Leurs sources,
84 poses natives et calages exacts sont figés dans
`visual-acceptance-creatures-lot06.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot06/`.
Les36 cycles acceptés restent conservés. La larve M12 et l'esprit M23 restent
différés ; ils n'entrent pas dans l'accord ni dans ce nouvel aperçu.

| Candidat | Identité existante | Anatomie à conserver |
|---|---|---|
| M39 | MON-033-0 | Forme rampante pâle, ouverture frontale rouge et deux épines dorsales sombres |
| M40 | MON-143-0 | Quatre pattes courtes, dos pâle segmenté, tête sombre et deux défenses |
| M41 | MON-154-0 | Quatre pattes, face plissée, trois nodules dorsaux violets et courte queue |
| M42 | MON-155-0 | Deux jambes, deux longs bras, large chapeau ocre et couches d'épaule, aucune aile |
| M43 | MON-157-0 | Deux jambes, deux bras, deux ailes bleues, deux cornes et une longue queue |
| M44, différé | MON-151-0 | Brassard et directions encore à conserver ensemble, absent de l'aperçu |

Les noms servent à la revue et ne renomment pas le contenu du jeu. Les catégories
techniques anciennes ne déterminent pas le nombre d'appuis : M42 reste bipède
malgré son ancien libellé `crawl`. Les défenses, épines, nodules, ailes et queues
ne deviennent pas des jambes. Le brassard de M44 appartient au bras droit du
personnage, visible à gauche de l'image dans la référence proche de la face ;
il ne doit pas migrer sur l'autre bras entre les poses ou orientations.
Les essais M44 n'y parviennent pas encore : il reste différé, hors des cinq
candidats M39–M43 présentés et hors de tout accord sur ce lot.

## Sources et fabrication

Fabrication avec l'**outil image_gen intégré**, sans CLI de génération.
Les sources sélectionnées sont dans
`assets/source-art/player-walk-review-07/<identité>-cycle-lot07-r*.png`.
Les prompts complets et références sont dans `creature-jobs-lot07.json`,
`prompts/<identité>-lot07-r01.txt` et les journaux
`generation-creatures-lot07-r*.json`.

La sélection est r01 pour M39, M40, M41 et M42, r02 pour M43.
`creature-jobs-lot07.json` contient seulement ces cinq candidats sélectionnés.
M43 r01 regardait du même côté sur les deux rangées ; r02 reprend les figures
entières avec deux orientations distinctes. Les essais M44 r01/r02 échangent
le bras équipé ; r03 conserve le brassard à gauche de l'image mais ses deux
rangées regardent à droite. Aucun de ces essais n'est présenté comme réussi.
Les prompts, références et motifs restent dans `creature-corrections-lot07-r02.json`,
`creature-arm-lot07-r03.json`, leurs journaux de génération et
`creature-deferred-lot07.json`. Le dessin canonique et son équipement ne sont
pas modifiés pour supprimer cette asymétrie.

Les références actuelles exactes restent dans
`references/<identité>-native-current.png`. Les agrandissements servent à
l'identité ; les cycles déjà acceptés guident le mouvement sans transfert
de leur espèce, silhouette, matière ou équipement. Chaque nouvelle source
contient14 figures entières : repos et six phases pour chaque direction.
Le conditionnement extrait, réduit uniformément et aligne les figures entières.
Il ne greffe ni jambes ni tête, n'étire pas les membres et ne déplace pas
des bandes pour simuler la marche. Un pixel opaque au bord extérieur de la
source bloque son export.
Le joueur signale ensuite « attention, M42 marche à l'envers. ».
La précédente interprétation de ses directions avait inversé à tort les
deux séries de sept figures. WALK07-M42-DIR01 rétablit l'ordre de la source :
gauche puis droite. Les14 dessins et l'ordre des six phases restent exacts,
sans miroir, retournement d'une tête ou retouche de membre. L'aperçu corrigé
s'ouvre sur M42 pour la revue. Les quatre autres candidats restent identiques.
Preuve ciblée : `m42-direction-correction-01.json`.

Le joueur répond « parfait :) » à cette correction ciblée. L'accord de M42,
ses14 poses natives et sa source inchangée sont figés dans
`visual-acceptance-m42-direction-01.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot07/MON-155-0-whole-body-native.png`.
M42 est accepté visuellement dans son orientation corrigée. Cette réponse
ne vaut pas accord séparé sur M39, M40, M41 ou M43 et n'intègre rien au jeu.

Le joueur complète ensuite l'accord par « ok on valide tout ca et on continue :) ».
Il accepte les cinq candidats présentés M39–M43, avec la correction de M42.
L'accord complémentaire, les cinq sources et70 poses natives exactes sont
figés dans `visual-acceptance-creatures-lot07.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot07/`.
Le premier gel de M42 est conservé à l'identique. M44 reste différé.

## Placement et comparaison

Le lecteur courant est réutilisé. « Dessin actuel » est la référence exacte,
« Nouvel essai » le candidat à examiner, et le détail×2 reprend les mêmes pixels
de la scène et sa grille. Les cases restent64×64px. Le canevas128 ne sert qu'aux
marges et grandes silhouettes ; les différences de taille sont conservées.

WALK07-P01 centre la figure entière et son ombre si l'enveloppe du cycle mesure
moins de64px de haut, avec un seul décalage fixe pour toutes les poses. Les
grandes créatures gardent leur ancrage. Aucun recentrage indépendant des phases
ou changement de taille pendant la lecture n'est appliqué.

Les sols sombre/clair/texturé, la pause et le pas manuel restent disponibles.
400ms reste le défaut et120/200/400ms restent sélectionnables. Trois phases
accompagnent une case, puis les trois suivantes la suivante, avec un vrai repos
à l'arrivée. Les520ms de pause de démonstration ne deviennent pas un délai de jeu.
Les déplacements haut/bas, tours, positions logiques et collisions restent inchangés.

## Contrôles et portée

Les sources et les poses natives sont examinées pour les directions, appuis,
appendices, proportions et matières. Les dimensions et différences de pixels
ne prouvent pas à elles seules une bonne démarche ou le côté d'un équipement.
Les contrôles du lecteur utilisent Node et un DOM/canvas simulé ; aucune
exécution réelle de ces candidats en navigateur ou en jeu n'est revendiquée.

Preuves : `creature-packing-lot07.json`, `creature-validation-lot07.json`,
`renderer-checks.json`, `source-manifest.json` et `bastion-transfer-checks-r07.json`.
Comparaison native : `artifacts/player-walk-review-07/creature-lot07-comparison-native.png`.

Les cinq candidats M39–M43 sont acceptés visuellement et restent hors du rendu
actif. Les574 poses des41
cycles acceptés, leurs sources et les cinq atlas WALK06 actifs restent
conservés. Aucun dessin du rendu actif, règle, statistique ou identité n'est
remplacé. Le reste du bestiaire et ses variantes gardent leur propre revue.

Résultats du24 octobre :70 poses proposées pour les cinq candidats M39–M43,
six dessins de marche distincts par direction et un vrai repos. Les hauteurs
natives sont21–23,53–55,54–57,84–86 et74–79px ; les décalages fixes sont−20,
−4,−3,0 et0px. Les cinq sources sélectionnées contiennent chacune14 figures,
sans pixel opaque au seuil128/255 sur leur bord extérieur. Les504 poses déjà
acceptées, leurs sources et les cinq atlas WALK06 actifs restent exacts.

Les2966 sélections du lecteur,300 fixtures du compositeur et601 instants de
cadence passent les contrôles. Les six aperçus courants restent sous1Mo,
avec des références et sources sans perte de pixels. Ces contrôles techniques
ne constituent pas l'accord artistique du joueur sur les cinq candidats.
