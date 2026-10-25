# WALK07-M51 à M56 — neuvième lot accepté, hors du rendu actif

« ok je valide :) » accepte les six candidats M45–M50 du lot08. Leurs sources,
84 poses natives et calages exacts sont figés dans
`visual-acceptance-creatures-lot08.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot08/`.
Les47 cycles acceptés gardent leurs pixels et calages propres. M12, M23 et
M44 restent différés et ne sont pas validés par extension.

Le joueur accepte ensuite les six candidats M51–M56 par « ok on continue ».
Les six sources sélectionnées,84 poses et calages exacts sont figés dans
`visual-acceptance-creatures-lot09.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot09/`.

| Candidat | Identité existante | Anatomie à conserver |
|---|---|---|
| M51 | MON-037-0 | Quatre appuis courts, petit corps pâle et ouverture frontale rouge |
| M52 | MON-038-0 | Quatre appuis, couverture bleu/gris et grande ouverture cerclée d'orange/rose |
| M53 | MON-067-0 | Quatre appuis courts, bouche ronde, dos sombre et corps arrière effilé |
| M54 | MON-069-0 | Six pattes anguleuses, corps rouge/charbon et articulations claires |
| M55 | MON-090-0 | Six pattes claires articulées, couverture brune segmentée et pointes sombres |
| M56 | MON-109-0 | Quatre grands appuis périphériques pâles, corps brun et appendice central conservé |

Les noms sont des repères de revue et ne renomment pas le contenu du jeu.
Les paires avant, centrales et arrière des formes à six pattes gardent leurs
attaches propres. La bouche, les pointes dorsales et l'appendice central de
M56 restent distincts des appuis : ils ne deviennent pas de nouvelles jambes.
Les catégories techniques anciennes ne déterminent pas seules l'anatomie.

## Sources et fabrication

Fabrication avec l'**outil image_gen intégré**, sans CLI de génération.
Les sources sélectionnées sont dans
`assets/source-art/player-walk-review-07/<identité>-cycle-lot09-r*.png`.
Les prompts et références sont dans `creature-jobs-lot09.json`,
`prompts/<identité>-lot09-r01.txt` et les journaux
`generation-creatures-lot09-r*.json`.

La sélection est r02 pour M51, r01 pour M52–M56. Le premier essai de M51
arrondissait sa forme sombre frontale en une bille ; r02 reprend le détail
court et anguleux de la référence. Le prompt ciblé, les entrées et la sortie
sélectionnée restent dans `creature-front-lot09-r02.json` et
`generation-creatures-lot09-r02.json`.
Les sources M52 et M56 ont rangé la droite avant la gauche : leurs deux
séries de sept figures entières sont échangées au conditionnement, sans miroir
ou retournement d'une tête ou d'un membre. Les quatre autres gardent l'ordre
gauche puis droite de leurs sources.

Les références actuelles exactes restent dans
`references/<identité>-native-current.png`. Les agrandissements servent à
l'identité, les cycles acceptés au mouvement ; ni leur espèce, ni leurs
matériaux ou appendices ne sont transférés au nouveau candidat.
Chaque cycle possède14 figures entières : repos et six phases pour chaque
orientation latérale proche de la face. Le conditionnement extrait, réduit
uniformément et aligne les figures complètes. Il ne greffe pas de membres,
ne retourne pas une tête seule et ne déplace pas de bandes pour simuler la
marche. Une source ayant des pixels opaques au bord extérieur est refusée.

## Placement et comparaison

« Dessin actuel » est la référence existante exacte, « Nouvel essai » le
candidat proposé. Le détail×2 reprend les mêmes pixels de la scène et sa
grille. Les cases restent64×64px, les tailles relatives sont conservées.
Le canevas128 ne sert qu'aux marges et grandes silhouettes.

WALK07-P01 centre la figure entière et son ombre quand l'enveloppe du cycle
mesure moins de64px de haut, avec un seul décalage fixe pour toutes les
phases. Les plus grandes silhouettes gardent leur ancrage. Aucun recentrage
indépendant des phases ou changement de taille pendant la lecture n'est appliqué.

Trois phases accompagnent une case, les trois suivantes la suivante, avec
un vrai repos à l'arrivée.400ms reste le défaut et120/200/400ms restent
sélectionnables. Sols sombre/clair/texturé, pause et pas manuel sont conservés.
Les520ms de pause de démonstration ne deviennent pas un délai du jeu. Aucun
déplacement haut/bas, tour, position logique ou collision n'est modifié.

## Contrôles et portée

Les sources et poses natives sont examinées pour les appuis, directions,
matières, proportions et appendices. Les dimensions et différences de pixels
ne prouvent pas à elles seules une bonne démarche ou une anatomie correcte.
Les contrôles du lecteur utilisent Node avec DOM/canvas simulé ; aucune
exécution réelle de ces candidats en navigateur ou dans le jeu n'est revendiquée.

Preuves : `creature-packing-lot09.json`, `creature-validation-lot09.json`,
`renderer-checks.json`, `source-manifest.json` et `bastion-transfer-checks-r07.json`.
Comparaison native : `artifacts/player-walk-review-07/creature-lot09-comparison-native.png`.

Ce lot est accepté visuellement et reste hors du rendu actif. Les742 poses
des53 cycles acceptés, leurs sources et les cinq atlas WALK06 restent exacts.
Aucune identité, règle, statistique ou ressource du rendu actif n'est remplacée.
Le reste du bestiaire et ses variantes gardent leur propre revue.

Résultats du24 octobre :84 poses proposées,14 par identité, avec un vrai repos
et six dessins de marche distincts par direction. Les hauteurs natives sont
23–24,30–32,25–28,32–35,26–28 et40–40px ; les décalages fixes sont−20,−16,−18,
−14,−18 et−12px. Les six sources sélectionnées contiennent chacune14 figures
entières et aucun pixel opaque au seuil128/255 sur leur bord extérieur.
Les658 poses acceptées, leurs sources et les cinq atlas WALK06 restent exacts.

Les2978 sélections du lecteur,300 fixtures du compositeur et601 instants de
cadence sont contrôlés ; les six aperçus courants restent sous1Mo, avec des
pixels compressés sans perte. L'accord artistique du joueur est enregistré
séparément pour les six candidats présentés.
