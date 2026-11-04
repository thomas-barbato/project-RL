# CLASS-LATERAL02-r02 — cadence et vrais pas, roster actuel

Le 2 novembre 2026, le joueur rejette r01 : « l’animation est à la fois trop
rapide et les jambes ne forment pas vraiment un pas ». Il rappelle que le
Technomancien n’existe plus parmi les classes jouables à la création.

## Classes et conservation

La revue charge le vrai catalogue via `ascii_character_class_catalog()` et
`CharacterClassCatalog::iter()`, puis exclut la Traqueuse déjà travaillée.
Elle présente uniquement Bastion, Arcaniste et Métamorphe, avec leurs tenues
lourdes sans arme. Un test vérifie cette liste et le statut retiré du
Technomancien. Son contenu de compatibilité avec les sauvegardes n’est pas
supprimé. Les anciens lecteurs de ces revues ne republient plus r01.

## Proposition de mouvement

Le nouveau cycle comporte huit poses entières : contact A, charge sur A,
passage du pied B, reprise vers l’avant de B, puis les quatre poses avec
l’appui opposé. Une neuvième figure dessine un vrai repos latéral. La tête,
le torse et les membres sont redessinés ensemble, sans greffe ni déformation
de bandes. La tenue frontale reste la référence de chaque classe.

La cadence proposée est de 650 ms par pas, soit 1 300 ms pour le cycle.
Les durées des huit poses sont 180, 180, 170, 120, 180, 180, 170 et 120 ms :
le contact et la prise d’appui durent plus longtemps que la reprise du pied.
Le lecteur permet aussi 800 ms et le repère précédent de 400 ms par pas.
Cette proposition ne change pas la durée des commandes du jeu normal.

L’étude montre un aller-retour : deux pas sur 40 px, un repos frontal puis
le retour. Ce déplacement sert à examiner les appuis ; il ne représente
pas une nouvelle distance de commande sur la grille. La translation concerne
la figure entière. Les articulations restent celles des dessins sources.
Le détail sur place et l’arrêt sur chaque pose sont conservés.

## Rendu et vérification

Les captures passent par le moteur, dans le camp accepté et la lumière Discret,
avec préparation préalable des textures. Les 32 échantillons par trajet
assurent une translation plus continue que les seules huit poses articulées.
Le cycle précédent est montré au même rythme pour comparer les dessins.
Le retour au repos emploie le rendu frontal normal à la position d’arrivée.
La Traqueuse et les autres banques du jeu restent conservées.

Les sources, le guide anatomique schématique et les atlas sont sous
`assets/source-art/animation-polish-01/class-lateral-02/r02/`. Le guide de
postures n’est pas un sprite du jeu. Les trois planches finales proviennent
de l’outil intégré image_gen ; leurs prompts sont consignés dans
`CLASS-LATERAL02-PROMPTS-r02.md`.

Le bilan natif et le contrôle du lecteur sont conservés dans
`CLASS-LATERAL02-VALIDATION-r02.json`. Les captures reproductibles et l’aperçu
restent dans `artifacts/animation-polish-01/class-lateral-02/r02/`.
R02 demeure une proposition de rythme et de dessin avant intégration ; les
autres armures, prises d’armes et bipèdes gardent leur revue distincte.
