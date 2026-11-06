# CITY-NORTH01-STYLE03-r01 — sources et briefs graphiques

Candidat du 5 novembre 2026, **pré-validation graphique en attente**.
Mode employé : outil intégré `image_gen.imagegen`, sans CLI.
Les trois originaux générés sont conservés sans retouche sous
`assets/source-art/north-city-01/skin-r03/` ; leur manifeste donne les empreintes.

## Références

Les deux images fournies au générateur sont des références de style :

- `assets/source-art/first-sector-review-01/u01-style-r07/references/street-native-lighting.png` ;
- `assets/source-art/first-sector-review-01/u01-style-r07/references/home-native-lighting.png`.

La projection, les contours, les surfaces mates et la densité de détails sont
définis par `docs/STYLE_VISUEL_REFERENCE.md` (`VIS-COH01-RULE01`) et
`docs/DIRECTION_ARTISTIQUE_TILES.md`. Les briefs ci-dessous conservent la
spécification finale de chaque demande ; ils ne constituent pas une transcription
verbatim du transport de l'outil.

## 03-A · materials-source.png

Use case: style-transfer. Asset type: orthogonal ruined-city material atlas.
Match the supplied accepted Project RL street and home drawings: dark drawn
outlines, grouped matte colours, restrained blue-grey shadows, simple readable
shapes, local irregular wear and calm surfaces. Produce a square 3 × 3 atlas,
equal panels, no labels. Row 1: continuous warm-grey sidewalk concrete,
blue-grey worn asphalt with no road markings, worn brown wood planks.
Row 2: warm-grey large floor tiles, dark slate slabs, workshop concrete.
Row 3: painted blue-grey steel, muted russet brickwork, blue-grey corrugated
roofing. Ground axes horizontal/vertical; no isometric perspective, no
photographic texture, no noisy grain, no lighting painted across the entire
sheet. Keep useful details legible after native 64 px world assembly.
Opaque output.

## 03-B · furniture-source.png

Use case: style-transfer. Asset type: transparent orthogonal furniture atlas.
Use the accepted street and home references for drawing, short volumes,
dark contours, matte surfaces and localized wear. Produce 16 isolated whole
objects in a regular 4 × 4 grid, with generous empty spacing and actual
transparency, no labels. Row 1: office desk, chair, wide dining table, sofa.
Row 2: bookcase, narrow green bed, stove, washbasin. Row 3: refrigerator,
locker, streetlamp, bench. Row 4: refuse bin, wide repair bench, industrial
machine, rubble. Preserve feet and whole silhouettes. Square ground axes,
short visible front faces; no isometric angle, no photorealism, no noisy
texture or cast background. Objects must stay readable at game scale.

## 03-C · service-source.png

Use case: style-transfer. Asset type: transparent service-object atlas.
Match the same accepted Project RL references and the furniture atlas:
dark drawn contours, grouped matte colours, short front faces, local wear.
Produce nine separate complete objects in a regular 3 × 3 grid with actual
transparency, no text. Row 1: grocery shelf with cans, street water pump,
bollard. Row 2: descending concrete stairwell, descending steel stairwell,
open ammunition crate. Row 3: armourer's repair bench, unpowered energy
rifle rack, ruined brick wall stump. Keep square ground axes and consistent
object scale; no isometric view, no realistic grain or background.

## Assemblage et périmètre

Le prototype échantillonne les panneaux et les silhouettes à leur échelle de
jeu, conserve les proportions, emprises, rotations et ancres des objets, puis
prépare les images hors du chemin de dessin. Les trois originaux restent intacts.
Les surfaces extérieures sont continues ; les intérieurs gardent leurs matériaux
fonctionnels. Les murs utilisent des profils communs de 28 px et des coins en L
réels avec contours, couvre-murs, chanfreins et faces avant séparés. Les portes
reprennent l'orientation et l'état persistant du moteur.

Cette reprise change uniquement la présentation du candidat isolé. Les fichiers
de carte, identifiants, décors sang/corps, agencements, collisions et choix des
variantes de r02 restent identiques. Le kit générique n'est plus chargé ni utilisé
en repli. Le nouveau dessin n'est pas intégré à la campagne.

La comparaison native montre la référence sous son éclairage validé et le
candidat sous l'ambiance uniforme de la fixture. Elle ne prétend pas valider
l'éclairage final. Les tests de raccord et de navigation ne remplacent pas
la pré-validation du dessin par le joueur, puis son inspection en jeu.
