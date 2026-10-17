# DEMONS-27 — grand bestiaire démoniaque

Propositions du 17 octobre 2026. Le joueur demande de nombreux démons : vers
et larves, ailés musclés de différentes couleurs, minotaures ailés, formes de
peste et du froid, et silhouettes presque humaines. Il autorise des tailles
supérieures à 64 px. Ce lot présente vingt-quatre modèles originaux répartis
en six familles de quatre, avec deux palettes par dessin.

## Direction et échelle

Le style reste celui du joueur et de ses armes : contours sombres, grandes
masses colorées, quelques plans d'ombre et peu de petits détails. Les formes
rampantes utilisent une vue de trois quarts ; les autres restent presque de face.
Les ailés ont deux bras, deux jambes et deux ailes distinctes. Les minotaures
se reconnaissent à leur tête bovine et leurs sabots ; les références de tailles
restent à côté de chaque dessin. Le gabarit ne fixe aucune collision.

Le froid se lit dans les teintes et les excroissances cristallines ; la peste
dans les masses charnues, les nodules et les formes fongiques ou insectoïdes.
Les noms et les associations de familles sont provisoires. Les ailes, la peste
et le froid ne définissent aucune nouvelle capacité mécanique à ce stade.

La base de comparaison reste **64 px par case**. Les sprites sont présentés
à leur taille native proposée, sans zoom ni ajustement au cadre. Certains
adultes dépassent la stature du joueur ; les vers et larves varient aussi en
longueur et en hauteur. Ombres de contact séparées ; alpha binaire et palette
bornée à 24 couleurs. Les variantes A/B gardent exactement leur géométrie.

## Modèles proposés

| Numéro | Code | Nom provisoire | Famille | Corps natif |
| --- | --- | --- | --- | --- |
| 140 | DEM-V01-r01 | Larve du charnier | Vers et larves démoniaques | 68 × 34 px |
| 141 | DEM-V02-r01 | Ver des fosses | Vers et larves démoniaques | 66 × 78 px |
| 142 | DEM-V03-r01 | Sangsue des égouts noirs | Vers et larves démoniaques | 100 × 31 px |
| 143 | DEM-V04-r01 | Larve boursouflée | Vers et larves démoniaques | 80 × 55 px |
| 144 | DEM-A01-r01 | Écorcheur écarlate | Démons ailés | 118 × 83 px |
| 145 | DEM-A02-r01 | Ailé d'obsidienne | Démons ailés | 111 × 92 px |
| 146 | DEM-A03-r01 | Brute soufrée | Démons ailés | 107 × 80 px |
| 147 | DEM-A04-r01 | Furie violette | Démons ailés | 115 × 84 px |
| 148 | DEM-M01-r01 | Taureau des brèches | Minotaures ailés | 124 × 94 px |
| 149 | DEM-M02-r01 | Minotaure de cendre | Minotaures ailés | 117 × 88 px |
| 150 | DEM-M03-r01 | Cornu vert-de-gris | Minotaures ailés | 116 × 92 px |
| 151 | DEM-M04-r01 | Minotaure pâle ailé | Minotaures ailés | 112 × 94 px |
| 152 | DEM-P01-r01 | Boursouflé de peste | Démons de peste | 78 × 78 px |
| 153 | DEM-P02-r01 | Porte-fièvre | Démons de peste | 100 × 81 px |
| 154 | DEM-P03-r01 | Molosse putride | Démons de peste | 88 × 58 px |
| 155 | DEM-P04-r01 | Moissonneur fongique | Démons de peste | 71 × 88 px |
| 156 | DEM-F01-r01 | Briseur de givre | Démons du froid | 87 × 92 px |
| 157 | DEM-F02-r01 | Rôdeur des glaces | Démons du froid | 69 × 80 px |
| 158 | DEM-F03-r01 | Coléoptère du gel | Démons du froid | 95 × 60 px |
| 159 | DEM-F04-r01 | Étrangleur du blizzard | Démons du froid | 86 × 91 px |
| 160 | DEM-H01-r01 | Faux dévot | Démons humanoïdes | 54 × 70 px |
| 161 | DEM-H02-r01 | Écorchante carmine | Démons humanoïdes | 41 × 74 px |
| 162 | DEM-H03-r01 | Pâle imposteur | Démons humanoïdes | 54 × 76 px |
| 163 | DEM-H04-r01 | Démon des chairs nouées | Démons humanoïdes | 68 × 79 px |

## Grande planche du nouveau lot

![24 nouveaux démons, noms et codes à taille native](C:/Users/User/Desktop/project-RL/artifacts/npc-monsters-review-01/demons-27/demons-140-163-native64.png)

## Contrôle des sols

Scène composée de revue, à la même échelle, sur fond clair, fond sombre et sol
texturé déjà employé dans les aperçus. Les vingt-quatre variantes A sont
répétées à l’identique, avec le joueur au bout de chaque rangée. Cette image
est un montage de contrôle ; elle ne représente pas une capture du jeu actif.

![Contrôle des 24 modèles sur trois sols](C:/Users/User/Desktop/project-RL/artifacts/npc-monsters-review-01/demons-27/controle-trois-sols-native64.png)

## Sources et statut

Création par l'outil intégré `image_gen`, vingt-quatre appels distincts.
Les grandes sources sont conservées dans
`assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/` avec
leurs codes `DEM-V01–04`, `DEM-A01–04`, `DEM-M01–04`, `DEM-P01–04`,
`DEM-F01–04` et `DEM-H01–04`, toutes en r01. Les ombres sont dans
`demons-shadows-27/`. Prompts exacts, références, empreintes et exports :
[demon-bestiary-27.json](demon-bestiary-27.json).

```powershell
python tools/art/show_demon_gallery_review.py --review demon-bestiary-27.json --output demons-27
python tools/art/show_monster_catalogue.py
python tools/art/validate_monster_catalogue.py
```

Le catalogue courant réunit 134 modèles. Les 110 modèles précédents et leurs
statuts sont conservés ; les refus supprimés ne reviennent pas. Les nouveaux
dessins restent candidats pour la sélection individuelle prévue. Les noms,
catégories de rencontre, compétences, résistances, collisions et animations
restent à définir séparément. Aucun nouvel acteur n'est raccordé au jeu.
