# CITY-NORTH01-PROJECTION06-r02 — proportions du volume complet

5 novembre 2026. **Étude de perspective, pré-validation artistique en attente.**

Le joueur refuse l'étude PAINT05-r04 : « Non hum, il y a d'énormes problèmes
de perspectives :) ». Elle n'est plus la proposition courante.

## Défauts identifiés dans 05

Le compositeur précédent recalait le logement avec plusieurs rapports d'échelle
en X et en Y. Les voisins étaient ensuite recadrés dans des bandes de 150 et
132 px, selon le bord de l'image et sans implantation commune au sol. L'intégrité
des dessins sources ne suffisait pas : leurs proportions et leurs ancrages
n'étaient plus cohérents dans la composition. La copie de bâtiments de la
première ville ne validait donc pas leur assemblage avec le logement en coupe.

## Portée de la reprise

L'étude 06 porte uniquement sur une jonction **toiture, corniche, façade, pied
de mur, trottoir et rue**. Un nouveau petit bâtiment fermé est dessiné comme
un seul objet entier, avec une façade complète sous sa couverture. Le garage
de la première ville est affiché entier à côté, dans sa taille native d'origine.
Le même acteur approuvé de 64 px sert aux deux compositions. Ce bâtiment test
ne remplace ni B01 ni ses voisins dans les données de travail.

Le dessin `VOLUME06-01-source.png` est généré avec l'outil intégré
`image_gen.imagegen`, sur transparence réelle. Le prompt exact est conservé
dans `PROJECTION06-PROMPT-r01.txt`. Les références sont le garage complet
`west-garage-native.png` et l'illustration acceptée `SCENE-U01-07-r01.png`.

Source originale : **1288 × 1221 px**. Le cadre utile est
`(116,94,1056,1034)` ; les valeurs alpha résiduelles égales à 1 en dehors de ce
cadre sont des franges isolées, sans contour solide coupé. La r01 employait
une réduction **4/11**, soit **384 × 376 px**. La correction r02 emploie **3/11**,
soit **288 × 282 px**, avec exactement le même facteur en largeur et en hauteur.
Aucun étirement par morceaux, changement de caméra par composant,
rotation, découpage du toit ou recadrage d'une façade n'est employé.

La mise en scène est produite par Macroquad dans un diagnostic séparé. Les
ancrages des pieds de maçonnerie sont définis séparément de la boîte de l'image :
375 px dans le candidat r01, **281,25 px dans r02**, 302 px dans le garage original. Ils sont posés
sur le même niveau de sol `Y=448`. Les accessoires du garage peuvent descendre
légèrement plus bas que son mur. La bordure de chaussée se trouve à `Y=512`.
Les sols continus sont les matières déjà acceptées sous `floor-continuous-01/`
et `ROAD-U01-09-native.png`, sans réintroduire le quadrillage extérieur ni
dupliquer des grilles d'égout ou des plantes d'une capture.

## Correction locale d'échelle — r02

Le joueur juge la perspective « mieux », mais le personnage trop petit par
rapport au bâtiment, notamment à la porte de droite. Ce retour ne vaut pas
validation complète du dessin. Le personnage reste le même dessin 64 × 64,
dont la silhouette opaque mesure 58 px de haut. L'ouverture de porte verticale
repérée dans la source, approximativement `Y=846..1104`, mesurait près de 94 px
dans r01 ; son encadrement atteignait presque deux hauteurs de silhouette.

**Seule la maison de droite est réduite de 25 %**, uniformément. Le passage
est ramené à environ 70 px. Le dessin, la perspective et les proportions
internes sont conservés sans nouvelle génération. La transformation pivote
sur l'axe de porte et le pied de façade : le niveau du sol et la position du
personnage restent identiques.

La référence de gauche, les deux personnages et tous les pixels de sol/rue
hors de l'ancien cadre du bâtiment sont vérifiés identiques aux captures r01.
La légende de droite et les repères de corniche sont actualisés. Les trois vues
inspectées de r01 sont conservées avec le suffixe `-r01.png` dans l'aperçu
courant. Preuves : `ECHELLE06-r02.json` et `PROJECTION06-VALIDATION-r02.json`.
La correction ne modifie aucune carte, collision, sauvegarde ou ressource de jeu.

## Figures

- `projection06-scene.png` : scène seule de 768 × 704 px, à 64 px/case ;
- `projection06-comparaison.png` : référence et candidat de 1024 × 672 px ;
- `projection06-reperes.png` : repères de la corniche, du pied et de la chaussée.

Leur affichage réduit dans la page est une vue d'ensemble. La comparaison
native est accessible séparément, sans redimensionnement. Le sol et l'acteur
ont la même échelle dans les deux vues.

## Limites et conservation

La compilation et la capture du diagnostic passent. Le facteur d'échelle
uniforme est vérifié au lancement ; l'alpha, le cadre utile et les marges du
dessin sont contrôlés. Ces contrôles ne constituent pas un accord artistique
ni une preuve de circulation. L'éclairage final, le passage intérieur et les
collisions restent à étudier après l'accord sur cette projection.

Les cartes, bâtiments, pièces, mobilier, décors gore et variantes du commissariat
du prototype sont conservés. Les sauvegardes et la campagne normale ne sont
pas touchées. Le module de revue 05 et ses sorties restent historiques, sans
nouvelle régénération. Empreintes et résultat : `PROJECTION06-VALIDATION-r01.json`.

```text
cargo run --locked --example north_city_projection_review -- artifacts/north-city-01
python tools/show_north_city_review.py
```

Le diagnostic prépare ses images sur le service commun avant affichage. Les
sources restent dans `assets/`, les captures sous `artifacts/` et les décisions
dans le présent dossier. Aucun affichage normal ne dépend de ces captures.
