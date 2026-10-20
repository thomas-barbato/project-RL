# U01 — voiture 2 × 1, flammes et bâtiments

18 octobre 2026. **Révision candidate, non intégrée au jeu.**

**Correction suivante :** le joueur refuse la perspective frontale de ces
bâtiments. [U01-REVIEW-03](U01-REVIEW-03.md) les remplace par des modèles vus
de dessus et corrige aussi la voiture. Les paragraphes ci-dessous décrivent
l'essai antérieur ; la mesure de hauteur de porte et le principe d'élévation
frontale ne sont pas des règles acceptées pour le jeu.

Le joueur demande une voiture d'au plus deux cases de long sur une de haut,
évoque des flammes animées et demande les bâtiments. `u01-direction-r02.json`
conserve sa formulation et distingue cette contrainte de taille de l'examen
des nouveaux dessins. L'agencement ACT1-LAYOUT-01-r01 reste accepté et inchangé.

## Voiture et flammes

**DEC-U01-17-r03** remplace l'export candidat 192 × 128 par un export contenu
dans **128 × 64 px**, sans déformer la source. L'obstacle O02 du plan révisé
occupe **2 × 1 cases** au même emplacement. L'ancienne source et l'export r02
sont conservés pour référence ; ils ne définissent plus la taille demandée.

**FX-U01-CAR-01-r01** propose une boucle d'une seconde, huit images par cycle,
sur le capot et l'habitacle. Elle réutilise les huit images de flammes déjà
présentes dans `assets/animations/agent/skill-animation-review-06/flame-atlas.png`.
Les trois petits foyers ont des phases différentes. La composition complète,
carrosserie et flammes, demeure dans 128 × 64 px sur toutes les images.

Il s'agit d'une animation d'ambiance proposée. Elle n'ajoute pas une source
de dégâts, une explosion, un effet de terrain ou une règle de propagation.
Son affichage en campagne et sa visibilité devront être raccordés au rendu
du monde après accord ; aucun calcul de combat n'est modifié ici.

## Premiers bâtiments

**DEC-U01-18-r01 — habitation.** Façade de deux niveaux, béton usé, fenêtres
brisées, porte fermée et conduites. **DEC-U01-19-r01 — garage fermé.** Un niveau,
rideau métallique et porte de service. **DEC-U01-20-r01 — habitation condamnée.**
Maçonnerie exposée, fenêtres abîmées, porte fermée et planches visibles.

Les façades ne sont pas de petites maisons réduites à une case. Elles ont
une largeur native de **256 px, soit quatre cases**. Les habitations utilisent
un canevas de 256 × 256 px ; le garage, 256 × 192 px avec débordement transparent.
Ce sont des surfaces visibles, pas de nouveaux gabarits de collision d'acteurs.
La hauteur des passages humains dessinés est d'environ 70 px et se compare
au corps visible du joueur accepté, haut de 65 px. Cette échelle est choisie
après examen du dessin source ; réduire ces façades à 192 px aurait raccourci
les portes. Les proportions du dessin sont conservées.

**DEC-U01-21-r01 — toiture.** Une surface de 64 × 64 px permet d'assembler
les toits selon l'emprise des bâtiments. La revue complète les anciens volumes
sombres par ces surfaces et les nouveaux modules. Elle conserve les surfaces
praticables et les accès du plan, avec la seule réduction de l'épave.

Les façades sont dessinées face au bas de l'écran, comme les surfaces visibles
du camp. Elles ne sont ni retournées ni pivotées pour simuler l'autre face.
Les façades opposées donnent à voir les toits et parapets. Une entrée visible
sur une face opposée devra recevoir son dessin orienté et son raccord propre.

Ce premier lot montre des portes fermées ou condamnées. Les futurs bâtiments
visitables demandent un intérieur raccordé, une entrée réellement praticable
et une vue en coupe qui préserve la visibilité. Leur plan et leur rendu seront
présentés ensemble ; une ouverture dessinée ne doit pas mener dans une masse
bloquante. Les cours et intérieurs facultatifs du chapitre restent prévus selon
l'agencement accepté, sans ajouter ici de quête ou de condition d'accès.

## Revue et limites

Le plan révisé est **MAP-U01-01-r02**. Ses 536 cases praticables sont reliées.
Les approches P00, F00 et U02 restent accessibles. La grille de revue conserve
une circulation continue de trois cases de large dans la rue et l'approche
du refuge. Le trajet cardinal illustré vers U02 passe à 36 pas après réduction
de l'obstacle. Ces vérifications ne simulent pas des déplacements dans le moteur.

La page `artifacts/first-sector-review-01/index.html` est réutilisée. Les premiers
éléments présentés sont la planche des bâtiments et la voiture animée, tous
à taille native. Suivent le plan et les vues de rue. La GIF comporte huit images
et une palette partagée afin que les éléments fixes ne changent pas de couleur.
Les vérifications et empreintes sont dans `u01-review-inspection-r02.json`.

Les dessins sont créés avec **image_gen intégré**, en prenant le camp comme
référence de style. Le prompt exact est dans `u01-buildings-prompt-r01.txt`.
La source et les exports sont dans `assets/source-art/first-sector-review-01/u01-r02/`.
Les sources existantes et la carte du camp restent inchangées. L'animation
réemploie les dessins de flammes du jeu ; elle n'a pas nécessité une nouvelle
génération d'images. Les opérations de découpe, export et composition sont
reproductibles :

```text
python -X utf8 tools/reviews/build_u01_city_review.py --revision r02
```

`u01-decor-manifest-r02.json` et `u01-layout-checks-r02.json` décrivent les dessins
et le plan. La précédente revue reste conservée avec `--revision r01`.
Aucune nouvelle carte jouable, sauvegarde, donnée de l'éditeur, règle de combat
ou statistique d'acteur n'est modifiée. Le raccord et les collisions du rendu
complet devront être éprouvés dans le prototype moteur après examen de ces
nouveaux éléments.
