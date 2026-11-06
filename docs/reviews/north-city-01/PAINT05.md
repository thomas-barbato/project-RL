# CITY-NORTH01-PAINT05-r04 — étude du dessin et des volumes

5 novembre 2026. **Refusée par le joueur pour ses perspectives. Aucune
intégration à la campagne ou au prototype jouable.**

L'étude suivante est [PROJECTION06.md](PROJECTION06.md). Le présent document
conserve l'historique de fabrication de 05 ; ses contrôles ne valident pas
son assemblage artistique.

Le joueur juge DECOR04 très loin de la qualité de la première ville. Il signale
ensuite les nappes bleues extérieures, puis une perspective où les murs semblent
au-dessus des toits et les toits au niveau du sol. Les contrôles techniques de
DECOR04 restent des contrôles de circulation, sans accord artistique.

## Référence effectivement employée

`SCENE-U01-07-r01.png` est l'illustration de référence acceptée, avec la direction
et le prompt exact conservés dans la revue historique `first-sector-review-01/`.
Sa finition est définie par `VIS-COH01-RULE01` : aplats ombrés, contours fermes,
matières calmes, usure choisie, projection orthogonale et acteurs à taille native64.
L'albédo est neutre ; les lumières chaudes sont celles du moteur.

## Étapes graphiques

1. `B01-PAINT05-r01.png` : reprise du logement en utilisant la véritable
   illustration validée comme référence de dessin. Les pièces, ouvertures et
   objets restent les repères du plan précédent. Le nouveau dessin ajoute des
   détails domestiques et structurels aux murs. Source originale imagegen intacte.
2. `B01-PAINT05-r02.png` : essai limité aux couvertures extérieures, abandonné
   après le retour du joueur sur le manque de volume. L'olive ne corrigeait pas
   une toiture dessinée au niveau du sol.
3. `B01-PAINT05-r03-rejete.png` : essai de façade rejeté par l'agent avant revue
   finale, car des équipements de toiture et un effondrement étaient peints sur
   une surface de façade verticale. Il ne sert à aucun rendu courant.
4. **r04, étude actuelle** : le logement peint de r01 est conservé et les voisins
   sont assemblés avec les bâtiments complets de la première carte
   (`east-long-native.png`) à **265 × 563 px, sans changement d'échelle**. Leurs
   couvertures, corniches, façades et socles restent solidaires. Le cadrage coupe
   naturellement une partie de leurs toits, tandis que les façades se terminent
   au pied des volumes. Une étroite ombre de contact de 17 px remplace le résidu
   de bande bleue à la jonction nord.

Les originaux et leur provenance sont dans
`assets/source-art/north-city-01/paint05/manifest.json` ; les trois prompts exacts
sont les fichiers `PAINT05-PROMPT-*.txt` de ce dossier de revue. Génération par
l'outil intégré `image_gen.imagegen`, sans CLI. L'assemblage final est une
composition du moteur, sans modification des dessins sources.

## Comparaison et portée

`paint05-comparaison.png`, 1024 × 416 px, compare l'albédo de la première ville et
le candidat à **64 px/case**, avec le même acteur approuvé à 64 px des deux côtés.
`paint05-logement.png`, 1536 × 960 px, montre l'ensemble. Le nouveau grand dessin
est enregistré sur les axes du plan à taille de jeu pour cette revue ; cette
registration ne vaut pas validation de toutes ses portes ou collisions.

Le contrôle avant/après de la retouche extérieure exige l'égalité exacte des
pixels dans la zone protégée à partir de `(132,167)` jusqu'au bas/droite. Aucune
pièce ni aucun meuble n'est redessiné par l'assemblage des voisins. Les pixels
du bandeau de contact nord et de l'extérieur sont les seules zones retouchées.
Le rendu ne modifie aucun état de jeu. Les fichiers de carte, le modèle et les
modules de rendu du prototype DECOR04, ainsi que l'illustration validée de la
première ville, conservent leurs empreintes. Preuve : `PAINT05-VALIDATION-r04.json`.

Cette étude sert à arrêter le traitement visuel avant de le décliner au quartier.
Elle n'est pas encore le fond d'une zone jouable : séparation des couches,
ouvertures, éléments ajoutés et collisions doivent être rapprochés du plan et
revus dans le moteur avant une validation en jeu. Le prototype DECOR04 demeure
disponible pour sa circulation, avec son dessin antérieur jugé insuffisant.

```text
cargo run --locked --example north_city_paint_review -- artifacts/north-city-01
python tools/show_north_city_review.py
```

Les sources sont lues/décodées sur le service commun de préparation, puis
chargées sur le thread graphique avant cette revue. Les captures sous
`artifacts/` ne sont pas des dépendances du jeu ou du prototype.
