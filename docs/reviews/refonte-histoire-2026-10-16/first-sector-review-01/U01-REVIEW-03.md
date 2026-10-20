# U01 — correction de la vue de dessus

18 octobre 2026. **Apparences corrigées acceptées ; non intégrées au jeu.**

L’accord suivant « oui ok, continue d'ajouter des elements de décors » est
enregistré dans `visual-acceptance-r03.json` pour la voiture 17 r04 et les
bâtiments 18–20 r02. Il ne valide pas leur disposition concrète. Le joueur
demande ensuite une illustration différente et variée, présentée dans
[U01-ILLUSTRATION-04](U01-ILLUSTRATION-04.md).

Le joueur rappelle : « n'oublie pas que la vue est de haut . C'est pas la bonne
perspective.. ». Les élévations DEC-U01-18/19/20-r01 sont refusées pour leur
perspective frontale. La trace est dans `u01-projection-correction-r03.json`.
Elles restent des sources historiques et ne sont plus présentées comme
la solution courante. L'agencement accepté du chapitre reste conservé.

## Correction visuelle

**DEC-U01-18-r02** représente désormais un bâtiment complet vu de dessus :
toit plat dominant, parapet bas, ventilations et câbles. Son canevas de
384 × 256 px correspond à 6 × 4 cases.

**DEC-U01-19-r02** montre le toit métallique d'un garage, sur 256 × 192 px,
soit 4 × 3 cases. Le rideau fermé est indiqué par une courte tranche de mur.

**DEC-U01-20-r02** montre un bâtiment à toiture partiellement effondrée,
avec poutres et dalles vues depuis le dessus, sur 384 × 256 px. L'entrée
reste condamnée ; l'ouverture dans le toit n'ajoute pas un intérieur praticable.

Le sol suit des axes horizontaux et verticaux, sans rotation en losange.
Le dessin montre les plans horizontaux et une faible hauteur de mur, en
cohérence avec le camp. La hauteur de porte d'environ 70 px de la revue
précédente n'est plus un critère : elle concernait une élévation inadaptée.
En vue de dessus, ce sont la largeur du passage et l'emprise au sol qui
permettent d'apprécier l'échelle. Le joueur accepté reste affiché à l'échelle 1.

**DEC-U01-17-r04** corrige également la voiture en vue de dessus : toit,
pare-brise et capot visibles, sans grand profil latéral de roues. Elle garde
le canevas maximal de 128 × 64 px, soit 2 × 1 cases. La proposition de flammes
**FX-U01-CAR-01-r02** reste contenue dans ce canevas, sur huit images par seconde.
Elle réemploie les dessins de flammes existants, sans nouvelle règle de combat.

## Assemblage de revue

Le plan **MAP-U01-01-r03** conserve les mêmes cases praticables, obstacles
et raccords que r02. Les îlots B01 à B04 reçoivent des instances des bâtiments
complets, à leur taille native, au lieu de grandes façades plaquées contre
une toiture générique. Les instances occupent exclusivement des cellules
bloquées de leurs îlots. Aucun bâtiment ni sprite n'est tourné ou réfléchi.

Les 536 cases praticables restent reliées, avec les mêmes chemins vers P00,
F00 et U02 et le même passage continu de trois cases de large dans la rue.
Ces contrôles portent sur le prototype Python ; ils ne certifient pas les
collisions ou la visibilité dans le moteur. Les bâtiments visitables avec
toiture retirée demanderont une revue distincte de leur intérieur et de
leurs entrées, comme prévu.

L'aperçu courant `artifacts/first-sector-review-01/index.html` montre les
nouveaux modèles, la voiture animée et les vues de rue corrigées. La planche
des bâtiments est `u01-buildings-native-r02.png` ; la voiture animée est
`u01-car-fire-native-r04.gif`. Les images de jeu gardent 64 px par case.

## Sources et reproduction

Les dessins ont été refaits avec **image_gen intégré**, avec l'essai précédent
comme cible à corriger et le camp accepté comme référence de projection et
de style. Le prompt exact est `u01-topdown-prompt-r01.txt`. La source originale
et les exports sont dans `assets/source-art/first-sector-review-01/u01-r03/`.
La source est conservée sans retouche ; les exports sont découpés et contenus
dans leurs canevas sans déformer les proportions ni supprimer leur alpha.

```text
python -X utf8 tools/reviews/build_u01_city_review.py --revision r03
```

Le manifeste est `u01-decor-manifest-r03.json`, les contrôles de grille sont
dans `u01-layout-checks-r03.json`, l'examen des images et de l'animation dans
`u01-review-inspection-r03.json`. Les anciens dessins ne sont pas intégrés,
et cette correction ne vaut pas acceptation automatique des nouveaux dessins.
Le jeu normal, les sauvegardes et les données de l'éditeur ne sont pas modifiés.
