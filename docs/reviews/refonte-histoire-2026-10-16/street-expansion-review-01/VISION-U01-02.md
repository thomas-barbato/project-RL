# Champ de vision des intérieurs — VISION-U01-02

19 octobre 2026 · proposition technique et visuelle, non activée en partie normale.

Le joueur signale des ombres de champ de vision étranges. Les captures
montrent des poches noires qui traversent les intérieurs.

## Diagnostic

Le fond dessiné est masqué par cases de 64 px. Un contour analytique est
déjà appliqué après les toits : il n'était donc pas absent. L'essai
VISION-U01-01, qui rétrécissait cette frange, ne résout pas les découpes et
est abandonné. Le shader et son branchement courant sont rétablis.

La grille des deux nouveaux intérieurs contient des murs à la place de
sol ou de mobilier bas. Par exemple, `[10,20]` coupe la vue entre `[12,20]`
et `[9,20]`, alors que le dessin montre un passage sans cloison. L'opacité
d'une image de bâtiment ne suffit pas à définir ses murs logiques.

## Proposition à examiner

Quatorze cases cessent de bloquer la vue comme des murs dans le seul
prototype VISION-U01-02. Leurs collisions de déplacement sont conservées
pour isoler le changement de visibilité. Les murs extérieurs, les portes,
les rayonnages hauts, le réfrigérateur et l'armoire restent opaques.
La portée, la ligne de vue, les lumières, la frange et les dessins ne changent pas.

Les cases exactes sont dans `room-vision-review-02.json`, sous
`assets/source-art/first-sector-review-01/u01-expansion-r06/`.
La correction exige une compilation debug, le diagnostic
`--ui-cold-street-expansion` et `--vision-layout`. Les parties normales
et leurs sauvegardes ne sont pas modifiées.

Les six comparaisons utilisent les mêmes positions et des lumières figées.
La grille et la visibilité qui en résulte diffèrent volontairement ; le
rendu reste en lecture seule dans chaque état. Les captures montrent une
amélioration des deux pièces. Des limites de visibilité restent présentes
derrière les meubles hauts ; cette revue ne prétend pas supprimer toute
occlusion. Voir `vision-checks-02.json`.

Contrôles réussis : compilation debug, contrôle release, test du passage
visuel `[12,20]` vers `[9,20]`, conservation des déplacements sur les
1 716 cases de la carte, mur extérieur et réfrigérateur encore opaques,
six captures à 64 px par case, retour réel au camp, rendu sans mutation.
Les deux tests existants du champ analytique ont également réussi pendant
l'investigation. Le shader analytique final reste inchangé.

```powershell
target/debug/project-rl.exe --ui-cold-street-expansion artifacts/street-expansion-review-01/vision-02/engine --vision-layout
python tools/reviews/show_street_vision_review.py
```

La baseline vient de `vision-01/engine/{scene}.png`, avant le changement
de grille. Les images `*-soft.png` de cet essai abandonné ne constituent
pas la proposition retenue. Aucun accord sur VISION-U01-02 n'est encore acquis.
