# WALK07-M57 à M60 — dixième lot validé, sans nouvelle intégration

Le joueur répond « ok je valide :) » après la présentation des quatre candidats.
Leurs sources,56 poses natives et calages exacts sont figés dans
`visual-acceptance-creatures-lot10.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot10/`.
Les57 cycles acceptés totalisent798 poses natives. M61/M62 ne font pas partie
de cet accord ; ils sont présentés ensuite dans `CREATURES-LOT11.md`.

« ok on continue » accepte les six candidats M51–M56 du lot09. Leurs sources,
84 poses natives et calages exacts sont figés dans
`visual-acceptance-creatures-lot09.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot09/`.
Les53 cycles acceptés gardent leurs pixels et calages. M12, M23 et M44 restent
différés et ne sont pas validés par extension.

| Candidat | Identité existante | Anatomie à conserver |
|---|---|---|
| M57 | MON-009-0 | Six membres porteurs, trois pointes dorsales sombres, grande bouche verticale et gabarit original |
| M58 | MON-020-0 | Base d'appendices recourbés, plaques pâles anguleuses et structures latérales sombres rigides |
| M59 | MON-140-0 | Corps larvaire segmenté, tête brune, petits appendices ventraux et extrémité effilée |
| M60 | MON-158-0 | Deux grandes pinces avant, deux petits appuis arrière, carapace bleue et pièces buccales distinctes |

Les deux bipèdes M61 (MON-162-0) et M62 (MON-156-0) sont déjà dessinés,
mais réservés au lot suivant. Leurs sources et prompts sont conservés dans
`creature-prepared-bipeds-lot11.json` : préparés, non présentés, non acceptés.
Le lot courant rassemble quatre morphologies différentes à échelle native.

Les noms servent à la revue et ne renomment pas le contenu du jeu. M58 progresse
par contractions des appendices existants, sans pieds humains ajoutés. M59
garde ses petits appuis sous les segments ; ils ne deviennent pas de grandes
jambes articulées. Les pièces buccales et pointes de tête de M60 ne sont pas
des pattes supplémentaires. Les catégories techniques anciennes ne suffisent
pas à définir l'anatomie.

## Sources et fabrication

Fabrication avec l'**outil image_gen intégré**, sans CLI de génération.
Les sources sélectionnées sont dans
`assets/source-art/player-walk-review-07/<identité>-cycle-lot10-r*.png`.
Les prompts complets et références restent dans `creature-jobs-lot10.json`,
`prompts/<identité>-lot10-r*.txt` et les journaux
`generation-creatures-lot10-r*.json`.

Les quatre sources sélectionnées sont R01. Les trois tentatives R02 de
nettoyage de contours ne sont pas sélectionnées : figures reliées pour M57,
franges encore présentes pour M58 et modification du contour pour M62.
Les franges résiduelles des R01 ne sont pas présentées comme entièrement corrigées.
L'ordre des deux rangées sources est réassocié pour M57 et M58 ; M59 et M60
conservent leur ordre. Les figures et l'ordre des phases ne sont pas retournés.

Les références actuelles exactes restent dans
`references/<identité>-native-current.png`. Les agrandissements servent à
l'identité ; les cycles acceptés guident seulement le mouvement et les appuis,
sans transfert de matière, espèce, équipement ou appendices. Chaque cycle
possède14 figures entières : repos et six phases par direction proche de la
face. Le conditionnement extrait, réduit uniformément et aligne les figures
complètes. Aucun membre n'est greffé, étiré ou déplacé par bandes. Une source
touchant le bord extérieur avec des pixels opaques est refusée.

## Taille native et placement

La case reste64×64px. M57 conserve la hauteur native114px de sa source et
ses six appuis ; il n'est pas réduit pour tenir dans une case. Le canevas128
garde seulement davantage de marge transparente au-dessus, avec un repère
d'appui interne à118px au lieu de112. La conversion en coordonnées de scène
compense ce repère : les pieds et l'ombre restent au même niveau logique.
Le rendu actif et la taille du jeu ne sont pas modifiés par cet aperçu.

Le zoom×2 reprend un carré128 de la scène. Pour la silhouette de M57 dépassant
96px de haut, son cadrage remonte d'une valeur fixe calculée sur l'enveloppe
du cycle afin de montrer la tête, les pointes et les pieds avec la case entière.
Il ne suit pas chaque phase et ne change pas l'échelle. Les cadres et calages
des53 cycles déjà acceptés restent identiques. La largeur disponible du
canevas peut aller jusqu'à126px pour M57, sans perte de sa hauteur native.

WALK07-P01 reste appliqué aux petites silhouettes : un seul décalage vertical
centre la figure entière et son ombre lorsque l'enveloppe mesure moins de64px.
Les grandes silhouettes gardent leur ancrage. Aucun recentrage indépendant
des phases ou redimensionnement pendant la marche n'est appliqué.

Les hauteurs visibles sur les14 poses sont respectivement110–114,68–72,
29–34 et57–60px. Les décalages verticaux uniques sont0,0,−15 et−2px.

## Comparaison et cadence

« Dessin actuel » montre la référence exacte et « Nouvel essai » le candidat.
Le détail×2 reprend les pixels de cette scène et sa grille.400ms reste le
défaut et120/200/400ms restent sélectionnables, avec sols sombre/clair/texturé,
pause et pas manuel. Trois phases accompagnent une case, puis les trois
suivantes la suivante, avec un vrai repos à l'arrivée.
Les520ms de pause de démonstration ne deviennent pas un délai du jeu.
Les mouvements haut/bas, tours, positions logiques et collisions restent inchangés.

## Contrôles et portée

Les planches sources et les poses natives sont examinées pour les directions,
appuis, proportions, couleurs et appendices. Les dimensions et différences
de pixels ne prouvent pas à elles seules une bonne démarche ou une anatomie.
Les contrôles du lecteur utilisent Node avec DOM/canvas simulé ; aucune
exécution réelle de ces candidats en navigateur ou dans le jeu n'est revendiquée.

Les contrôles vérifient aussi l'absence de découpe de la référence native et
la présence de toute l'enveloppe des14 poses et de la case64 dans le zoom.
Preuves : `creature-packing-lot10.json`, `creature-validation-lot10.json`,
`renderer-checks.json`, `source-manifest.json` et `bastion-transfer-checks-r07.json`.
Comparaison native : `artifacts/player-walk-review-07/creature-lot10-comparison-native.png`.

Contrôles terminés le24 octobre2026 :56 poses nouvelles, références natives
copiées sans découpe,2 982 couples de sélection,300 fixtures de rendu et601
échantillons de cadence lors de la proposition. Les six fragments restent sous1Mo ;
le fragment du lot10 mesurait862 283octets sur disque. Le zoom contient le cycle
complet et la case entière, y compris pour M57. Les742 poses acceptées
précédemment restent identiques. Le contrôle de transfert joueur couvre16 380
poses pour dix tenues ; les cinq atlas WALK06 actifs sont inchangés.

Ce lot a reçu son accord visuel ; il reste hors du rendu actif. Les798
poses acceptées, leurs sources et les cinq atlas WALK06 actifs restent exacts.
Aucune identité, statistique, règle ou ressource du rendu actif n'est remplacée.
Le reste du bestiaire et ses variantes gardent leur propre revue.
