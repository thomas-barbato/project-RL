# WALK07-M27 à M32 — cinquième lot accepté, hors du rendu actif

« ok parfait :) je valide :) » accepte les cinq candidats montrés du lot04 :
M21, M22, M24, M25 et M26. Leurs sources et70 poses natives exactes sont figées
dans `visual-acceptance-creatures-lot04.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot04/`.
Les24 cycles acceptés gardent leurs pixels et calages ; M12 et M23 restent
différés. Le joueur accepte ensuite M27–M32 par « ok on continue :) ».
Leurs six sources,84 poses natives et calages exacts sont figés dans
`visual-acceptance-creatures-lot05.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot05/`.

| Candidat | Identité existante | Anatomie et mouvement à conserver |
|---|---|---|
| M27 | MON-035-0 | Quatre appuis courts, chair pâle, ouverture rouge et griffes sombres |
| M28 | MON-076-0 | Deux pieds porteurs visibles et deux appendices latéraux recourbés, dos pâle fendu de rouge et crête sombre |
| M29 | MON-101-0 | Quatre appuis, grosse carapace bronze à côtes verticales et membres pâles |
| M30 | MON-070-0 | Quatre pattes de support et deux avant-bras recourbés distincts |
| M31 | MON-065-0 | Deux bras, corps serpentiforme continu, base enroulée, aucune jambe |
| M32 | MON-066-0 | Deux jambes et deux membres supérieurs portant les ailes, sans paire de bras ajoutée |

Les noms sont des repères de revue et ne renomment pas le contenu du jeu.
Les références exactes sont copiées dans `references/<identité>-native-current.png`.
Les détails agrandis servent à vérifier l'identité. Les catégories techniques
historiques ne déterminent pas le nombre de jambes : les bras, membranes,
griffes et appendices latéraux sont examinés séparément.
La première consigne de M28 comptait quatre appuis en plus des appendices.
La revue rapprochée du dessin et des figures produites retient deux pieds
porteurs et deux appendices recourbés, quatre membres visibles en tout.
Le prompt d'origine reste conservé comme historique ; son compte initial
n'est pas utilisé comme preuve d'anatomie et aucun pied supplémentaire
n'est ajouté à la figure pour s'y conformer.

## Dessins et prompts

Fabrication avec l'**outil image_gen intégré**, sans CLI de génération.
Les prompts complets et références sont dans `creature-jobs-lot05.json` et
`prompts/<identité>-lot05-r01.txt`. Les journaux `generation-creatures-lot05-r*.json`
conservent les sorties originales et les versions sélectionnées.
Les dessins sélectionnés sont dans
`assets/source-art/player-walk-review-07/<identité>-cycle-lot05-r*.png`.

Chaque cycle possède14 figures entières : repos et six phases vers la gauche,
puis repos et six phases vers la droite. La figure entière est dessinée pour
chaque pose. Le conditionnement extrait et réduit uniformément les figures ;
il ne greffe pas de membres, ne déplace pas de bandes et ne retourne pas une
tête isolée. Les guides acceptés servent au mouvement, sans transfert de leur
silhouette, de leur matière ou d'appendices absents de l'identité d'origine.
Une source touchant le bord extérieur avec des pixels opaques est refusée.

## Placement et comparaison

Le lecteur réutilise l'aperçu courant. « Dessin actuel » est la référence
existante exacte ; « Nouvel essai » est le candidat à examiner, et le détail×2
reprend les mêmes pixels de la scène et de sa grille. Les cases restent64×64px.
Le canevas128 ne sert qu'aux marges pour les grandes silhouettes.

Les sources M27, M29, M30, M31 et M32 ont placé la droite sur leur première
rangée et la gauche sur la seconde. Le conditionnement échange les deux
séries entières pour obtenir gauche puis droite ; aucune figure, tête,
carapace, aile ou partie de membre n'est retournée séparément.

Le placement suit WALK07-P01 : une seule translation verticale de la figure
entière et de l'ombre, calculée sur l'enveloppe des14 poses si elle mesure moins
de64px de haut. Les grandes figures gardent leur ancrage. Aucun recentrage
indépendant des phases, redimensionnement en cours de cycle ou changement de
position logique n'est appliqué.

Trois phases accompagnent une case, les trois suivantes la suivante, puis un
vrai repos.400ms reste le défaut et120/200/400ms restent sélectionnables. La pause
de520ms de la démonstration n'est pas un délai de jeu. Les sols sombre, clair
et texturé, la pause et le pas manuel sont conservés.

## Contrôles et portée

Les planches sources et les poses natives doivent être examinées pour les
directions, appuis, matériaux, tailles et appendices. Les empreintes, nombres
de poses et différences de pixels ne prouvent pas à eux seuls une bonne marche.
Les contrôles du lecteur utilisent Node et un DOM/canvas simulé ; aucune
exécution réelle de ce lot en navigateur ou dans le jeu n'est revendiquée.

Les preuves techniques sont `creature-packing-lot05.json`,
`creature-validation-lot05.json`, `renderer-checks.json`, `source-manifest.json`
et `bastion-transfer-checks-r07.json`. La comparaison native est dans
`artifacts/player-walk-review-07/creature-lot05-comparison-native.png`.

Ce lot est accepté visuellement et reste hors du rendu actif. WALK06 conserve ses
cinq atlas actifs. Aucune règle, statistique, collision, durée de tour ou
animation de déplacement vertical n'est modifiée. Les autres espèces,
variantes et configurations d'équipement gardent leur propre revue.

Résultats du24 octobre :84 poses proposées,14 par identité, avec six dessins
de marche distincts pour chaque direction. Les hauteurs natives sont27–30,
59–62,48–54,70–74,58–63 et56–58px. Les décalages verticaux fixes sont−17,−1,
−5,0,0 et−3px. Les six sources conservent leurs14 figures et n'ont aucun pixel
opaque au seuil128/255 sur leur bord extérieur. Les336 poses acceptées des
quatre lots précédents, leurs sources et les cinq atlas WALK06 restent exacts.
Les2955 sélections du lecteur,300 fixtures du compositeur et601 instants de
cadence passent les contrôles ; les six aperçus courants restent sous1Mo.
L'accord artistique du joueur est enregistré séparément pour ce lot.
