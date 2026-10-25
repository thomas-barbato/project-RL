# WALK07-M15 à M20 — troisième lot anatomique

**Accord visuel du24 octobre2026 : « Ok on continue :) ».** Les six cycles
M15–M20 sont acceptés. Les sources sélectionnées, les84 poses natives exactes
et leurs valeurs de centrage sont figées dans
`visual-acceptance-creatures-lot03.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot03/`.
Cette validation ne vaut pas nouvelle demande d'intégration au jeu.

Le joueur accepte le placement vertical WALK07-P01 par « parfait on continue :) ».
L'accord et les valeurs des13 cycles précédents sont figés dans
`visual-acceptance-placement-01.json` et
`assets/source-art/player-walk-review-07/accepted/vertical-placement-01/`.
Le présent lot poursuit la même méthode avec six identités existantes.
Les messages « ok on continue » reçus pendant sa préparation autorisent
la poursuite du travail ; ils ne constituent pas une validation de ces nouveaux
cycles encore à présenter.

| Candidat | Identité | Anatomie conservée |
|---|---|---|
| M15 | MON-063-0 | Quatre appuis et une queue, membres clairs et dos brun |
| M16 | MON-087-0 | Quatre pattes, corps trapu olive et ventre ocre |
| M17 | MON-103-0 | Quatre appuis, corps pâle et plaques dorsales sombres |
| M18 | MON-039-0 | Deux jambes, deux bras et deux ailes, petite silhouette rouge |
| M19 | MON-126-0 | Esprit étroit sans pieds, capuche et longs pans de tissu |
| M20 | MON-146-0 | Deux jambes, deux bras, deux ailes, surfaces dorées et membranes sombres |

Ces descriptions servent à identifier les silhouettes pendant la revue.
Elles ne renomment pas les monstres et ne créent pas de nouvelles espèces.
La larve M12 reste différée ; aucune nouvelle tentative n'est faite sur elle.

## Fabrication et sources

Mode : **outil image_gen intégré**, sans CLI ni API de génération externe.
Chaque planche utilise le dessin canonique natif de la créature comme cible
d'apparence et un cycle entier accepté de morphologie compatible comme guide
de mouvement. Les matériaux, appendices, proportions et équipements viennent
de la nouvelle identité ; le guide ne lui impose ni corps, ni armure, ni couleur.

Les six sources sélectionnées sont conservées dans
`assets/source-art/player-walk-review-07/` :

- `MON-063-0-cycle-lot03-r02.png`.
- `MON-087-0-cycle-lot03-r01.png`.
- `MON-103-0-cycle-lot03-r01.png`.
- `MON-039-0-cycle-lot03-r01.png`.
- `MON-126-0-cycle-lot03-r01.png`.
- `MON-146-0-cycle-lot03-r01.png`.

Les références exactes, les prompts complets et les destinations sont dans
`creature-jobs-lot03.json`, `prompts/<identité>-lot03-r01.txt` et
`generation-creatures-lot03-r01.json`. Ce dernier conserve les fichiers générés
originaux et les empreintes des copies du projet. Les dessins de référence
actuels restent dans `references/<identité>-native-current.png`, avec leurs
agrandissements de fabrication `*-identity-detail.png`.

Le premier atlas de M15 touchait le bord droit sur une extrémité de queue.
Il est écarté au profit d'une reprise avec des marges transparentes plus larges,
sans amputer la queue. Le prompt et la source de cette reprise sont conservés
dans `creature-margin-lot03-r02.json` et son journal de génération.

Les planches contiennent14 figures entières, en deux rangées de sept : un vrai
repos et six phases par direction. Les rangées de MON-087-0, MON-039-0 et
MON-126-0 ont été constatées dans l'ordre droite/gauche ; le conditionnement
les remet dans l'ordre gauche/droite, sans miroir, greffe ou retouche de membre.
L'examen rapproché des visages, corps et appendices complète le contrôle global
des silhouettes.

## Présentation et placement

Les petits monstres conservent le centrage vertical accepté : un décalage fixe
sur l'enveloppe complète du cycle, appliqué à la figure entière et à son ombre.
Les silhouettes hautes d'au moins64px gardent leur ancrage. L'esprit n'a pas
de nouvelle ombre de contact ni de pieds. Les corps ne sont ni redimensionnés
ni recentrés indépendamment à chaque image. La grille reste64×64px ; les
canevas128 conservent seulement les marges nécessaires aux appendices.

L'aperçu courant est réutilisé. Il montre le dessin actuel et le nouveau cycle
à l'échelle native, avec trois sols et un détail×2 qui reprend la même scène
et le même placement. Les commandes conservent400ms par défaut, le choix
120/200/400ms, trois phases par case puis trois au pas suivant, un vrai repos,
la pause et le pas manuel. Les520ms de pause de démonstration ne deviennent
pas un délai de jeu. Aucun déplacement vertical de gameplay n'est changé.

## Contrôles et portée

Les84 poses du lot sont non vides, restent dans leurs marges et possèdent
six dessins de marche distincts par direction. Les sources n'ont aucun pixel
opaque à128/255 ou plus sur leur bord extérieur. Une seule échelle uniforme
est appliquée par créature. Les hauteurs natives sont44–46,35–36,43–45,40–43,
63–64 et77–80px ; les décalages verticaux sont respectivement−9,−14,−9,−10,
0 et0px. Les références natives sont copiées sans redimensionnement.

Les182 poses acceptées des deux premiers lots, leurs sources et la règle de
placement figée restent exactes. Les cinq atlas WALK06 actuellement embarqués
restent inchangés. Les contrôles du lecteur utilisent Node avec DOM/canvas
simulé ; aucune exécution réelle de ce lot en navigateur ou dans le jeu n'est
revendiquée. Les planches complètes sont examinées à taille native. Le nombre
de pixels différents et les dimensions ne prouvent pas à eux seuls une bonne
démarche ou l'identité d'une patte.

Preuves : `creature-packing-lot03.json`, `creature-validation-lot03.json`,
`renderer-checks.json`, `source-manifest.json` et `bastion-transfer-checks-r07.json`.
Comparaison native : `artifacts/player-walk-review-07/creature-lot03-comparison-native.png`.

Le lot M15–M20 est accepté visuellement et reste hors du rendu actif.
Les deux premiers lots, la règle de placement et les familles de joueurs
précédemment acceptées gardent leurs accords propres. Ce lot ne valide pas
les autres variantes de matière, équipements, transformations ou références
du bestiaire et ne modifie aucune règle de jeu.
