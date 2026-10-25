# WALK07-M33 à M38 — sixième lot accepté, hors du rendu actif

« ok on continue :) » accepte les six candidats M27–M32 du lot05. Leurs sources,
84 poses et calages exacts sont conservés dans
`visual-acceptance-creatures-lot05.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot05/`.
Les30 cycles acceptés restent distincts des nouvelles propositions. La larve
M12 et l'esprit M23 restent différés, hors des accords et de cet aperçu.

Le joueur accepte ensuite les six candidats M33–M38 par « ok on continue :) ».
Les six sources sélectionnées,84 poses natives et calages exacts sont figés
dans `visual-acceptance-creatures-lot06.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot06/`.

| Candidat | Identité existante | Anatomie à conserver |
|---|---|---|
| M33 | MON-142-0 | Corps violet horizontal segmenté, défenses claires, appendices inférieurs et extrémité recourbée |
| M34 | MON-089-0 | Quatre pattes anguleuses ocre, couverture pointue sombre et griffes |
| M35 | MON-088-0 | Deux pattes à serres, long cou et bec pâles, deux ailes rouges, aucun bras ajouté |
| M36 | MON-149-0 | Deux jambes, deux bras, deux ailes sombres, corps pâle massif et cornes |
| M37 | MON-150-0 | Deux jambes, deux bras, deux ailes vertes, torse ocre et cornes claires |
| M38 | MON-153-0 | Deux jambes, deux bras distincts, deux ailes, visage pâle allongé et grandes griffes |

Les noms sont des repères de revue et ne renomment pas les identités du jeu.
La forme M33 reste rampante avec ses appendices existants, sans pieds humains
ou rangées de pattes ajoutées. M34 garde quatre pattes distinctes. M35 conserve
ses ailes comme membres supérieurs ; aucun bras humain ne leur est ajouté.
M36–M38 ont en revanche deux bras distincts de leurs deux ailes.

## Sources et fabrication

Fabrication avec l'**outil image_gen intégré**, sans CLI de génération.
Les sources sélectionnées sont dans
`assets/source-art/player-walk-review-07/<identité>-cycle-lot06-r*.png`.
Les prompts complets et références sont dans `creature-jobs-lot06.json`,
`prompts/<identité>-lot06-r01.txt` et les journaux
`generation-creatures-lot06-r*.json`.

La sélection finale est r02 pour M33 (MON-142-0), M36 (MON-149-0) et
M37 (MON-150-0), r01 pour M34, M35 et M38. La revue native a conduit à
reprendre la longueur de la forme rampante et les proportions des deux grands
ailés, trop proches du gabarit du guide. Les consignes et sorties sont dans
`creature-rampant-lot06-r02.json`, `creature-proportions-lot06-r02.json` et
`generation-creatures-lot06-r02.json`. Les versions r01 ne deviennent pas
des références acceptées par cette reprise.

Les références actuelles exactes restent dans
`references/<identité>-native-current.png`. Les agrandissements servent à
l'identité ; les cycles déjà acceptés guident uniquement le mouvement,
sans transfert d'espèce, de matériaux ou d'appendices. La catégorie technique
ancienne ne remplace pas l'examen du dessin. La direction est vérifiée sur
le visage, le bec, le devant du corps et les pieds ; une crête ou une corne
recourbée vers l'arrière n'indique pas à elle seule le sens de la marche.
M35 a rangé la droite avant la gauche : les deux séries de sept figures
entières sont échangées au conditionnement, sans miroir ou pièce retournée.

Chaque source contient14 figures entières : repos et six phases à gauche,
puis repos et six phases à droite. Les figures sont dessinées complètes,
extraites et réduites avec une seule échelle uniforme par créature, puis
alignées. Aucun membre n'est greffé, déformé ou déplacé par bandes. Une source
ayant des pixels opaques au bord extérieur est refusée avant export.

## Placement et lecture

« Dessin actuel » est la référence existante exacte. « Nouvel essai » est
le candidat proposé ; le détail×2 reprend les pixels de cette scène et sa grille.
La case reste64×64px et les différences de taille entre créatures sont conservées.
Les canevas128 servent aux marges des grandes silhouettes.

WALK07-P01 reste appliqué : pour une enveloppe de cycle moins haute que64px,
un seul décalage vertical centre la figure entière avec son ombre, sans
recentrage indépendant des phases. Les grandes silhouettes gardent leur
ancrage. Les contrôles, sols sombre/clair/texturé, pause et pas manuel restent
ceux de l'aperçu courant réutilisé.

400ms reste le défaut et120/200/400ms restent sélectionnables. Trois phases
accompagnent une case, puis trois la suivante, avec un vrai repos à l'arrivée.
Les520ms de pause de la démo ne deviennent pas un délai de jeu. Aucun mouvement
haut/bas, tour, position logique ou collision n'est modifié.

## Contrôles et portée

Les planches sources et les poses natives sont examinées pour les appuis,
directions, tailles, appendices et matériaux. Les dimensions, empreintes ou
différences de pixels ne prouvent pas seules une bonne démarche. Les contrôles
du lecteur emploient Node avec DOM/canvas simulé ; aucune exécution réelle
de ces candidats en navigateur ou dans le jeu n'est revendiquée.

Preuves : `creature-packing-lot06.json`, `creature-validation-lot06.json`,
`renderer-checks.json`, `source-manifest.json` et `bastion-transfer-checks-r07.json`.
Comparaison native : `artifacts/player-walk-review-07/creature-lot06-comparison-native.png`.

Ce lot est accepté visuellement et reste hors du rendu actif. Les cinq
atlas WALK06 actifs, les sources et les420 poses des30 cycles déjà acceptés
restent conservés. Aucun nouveau dessin n'est intégré au rendu du jeu.
Le reste du bestiaire, ses variantes et les autres équipements gardent leur
propre revue.

Résultats du24 octobre :84 poses proposées, six dessins de marche distincts
par direction et un vrai repos. Les hauteurs natives sont27–31,56–59,57–61,
85–88,89–91 et78–81px ; les décalages verticaux fixes sont−16,−2,−1,0,0 et0px.
Les six sources sélectionnées contiennent14 figures entières et aucun pixel
opaque au seuil128/255 sur le bord extérieur. Les420 poses acceptées, leurs
sources et les cinq atlas WALK06 restent exacts.

Les2961 sélections du lecteur,300 fixtures du compositeur et601 instants de
cadence passent. Les six aperçus courants restent sous1Mo. L'atlas de référence
de ce lot utilise WebP sans perte plutôt que PNG : ses pixels RGBA, y compris
les zones transparentes, sont comparés exactement avant utilisation. Aucun
dessin, détail ou alpha n'est retiré pour réduire la taille du fichier.
L'accord artistique sur ce lot est enregistré séparément.
