# HELM05-W02-EYE01 — optique et vues frontales

Le joueur conteste la cohérence de l'optique sur la feuille r02 du casque 02 :
elle ne lui paraît pas au même emplacement et disparaît dans un trois-quarts
encore frontal. Son retour est enregistré dans `HELM05-EYE01-RETOUR.json`.
L'affirmation de cohérence donnée avant ce retour était trop générale : les
tests précédents vérifiaient les compositions, les caches et les régions
préservées, pas cette fidélité entre les dessins.

## Reprise ciblée

La source r03 reprend l'optique comme un unique dispositif orbital de l'œil
gauche, orienté vers l'avant sous l'arcade, au-dessus de la découpe de joue.
Dans le trois-quarts opposé, une partie rétrécie reste visible derrière le nez ;
elle n'est pas ajoutée à la tempe proche. La source r03 est générée avec
**imagegen intégré** à partir de r02 et de l'icône acceptée. Le prompt exact et
les chemins sont dans `HELM05-EYE01-PROMPT.json`.

Les modifications de la face et du trois-quarts proche sont limitées ; la
correction la plus visible est la présence de cette partie de verre dans le
trois-quarts éloigné. La reprise reste un candidat de dessin à examiner.
Elle ne constitue pas une preuve de reconstruction 3D du modèle.

Le calibrage physique précédent de la couche est conservé : la même échelle
tirée de la source frontale r02 est appliquée aux trois nouvelles vues. Le vrai
dos est recomposé depuis sa source r02, afin de conserver ses pixels natifs.
Les quatre autres modèles et les cinq icônes acceptées ne sont pas repris.

## Contrôle visuel et régression

La section `#casque02-optique` de l'aperçu courant compare r02 et r03, avec un
sélecteur face / trois-quarts proche / trois-quarts éloigné. Elle juxtapose
les dessins agrandis et leur couche portée au zoom entier ×4. Les trois angles
sont aussi réunis sur une planche. Le comparatif porté de HELM05 utilise r03.

Dans la bande de l'œil de la couche native, r02 compte 4 / 5 / 0 pixels de verre
cyan pour face / trois-quarts proche / trois-quarts éloigné ; r03 compte
4 / 6 / 3 pixels. Le capteur vertical est exclu de cette mesure. Le test ciblé
reproduit l'absence dans l'ancien trois-quarts, puis vérifie la présence du
verre dans les trois angles après placement à taille native sur les quatre
gabarits, cinq niveaux d'armure et trois roulis de tête, soit 180 cas.

Ce test détecte une disparition à petite taille. Il ne valide pas à lui seul
la position artistique de l'optique : la comparaison agrandie et l'avis du
joueur restent nécessaires. Les empreintes des captures antérieures, hors de
la ligne du modèle 02, sont conservées dans `HELM05-EYE01-BEFORE.json` pour
vérifier l'absence de changement des autres casques et des corps témoins.

## Reproduction

```powershell
python tools/art/prepare_helmet_worn_review.py
cargo build --locked --bin project-rl
& .\target\debug\project-rl.exe --ui-cold-helmet-worn artifacts/animation-polish-01/helmet-worn-r01/current
python tools/art/build_animation_polish_review.py
```

Le fichier de sortie artistique est
`assets/source-art/animation-polish-01/helmet-worn-r01/H05-W02-views-r03.png` ;
l'atlas de revue est `H05-W02-atlas.png` dans le même dossier. L'ancienne couche
native est conservée dans `H05-W02-before-eye01-atlas.png`. Le rendu normal du
jeu ne reçoit aucune nouvelle intégration par cette reprise.
