# U01 r04 — traces d’une attaque de foule

Demande du joueur : « tu peux surement la décorer plus que ca, et l'abimer
aussi :) il faut que ce lieu semble avoir été attaqué par une foule ».

Le joueur précise ensuite : « non il faut que ca ressemble plus à une zone
de guerre que de pillage :) ». Il demande aussi « un peu plus de maison »,
car le bas est satisfaisant mais l’extension paraît vide. Ces corrections
remplacent l’intention de pillage de l’essai intermédiaire de cette révision.

La continuité r03, le départ sud, la grille native64 et la pénombre du moteur
restent la base. La révision ajoute six habitations ou petits ensembles,
codes **BLDG-U01-01 à 06**, dans les vides de l’extension. Elles ont des pans
de toit effondrés, des façades ouvertes sur les décombres et des pièces
exposées. La rue porte des impacts, épaves, positions détruites et effondrements
de maçonnerie. Les sources de l’essai de pillage sont conservées, mais les
chariots, vélos, benne et caisses dispersées ne font pas partie du rendu final.

Les routes conservent leur tracé et les bâtiments sont placés individuellement
dans le repère commun. Les seuils jouables et le retour au camp restent libres.
Les trois anciens bâtiments visitables conservent leur fonctionnement ; les
six nouveaux bâtiments sont condamnés pour ce candidat. Leurs lampes rouges
suivent les lentilles dessinées et utilisent le composite d’éclairage réel du
moteur. Un ancien lampadaire couvert par un nouveau bâtiment est retiré avec
son émetteur ; aucun halo résiduel n’est conservé.

Les gros amas, épaves et impacts ont des obstructions basses, sans consommation
de tour propre ni dégâts de terrain. Les traces plates restent traversables.
La fabrication vérifie que chaque case libre reste reliée au départ et que
les accès protégés restent dégagés. Les épaves occupent au plus 128 × 64 px.

Les dégâts restent candidats avant validation visuelle explicite. Aucun
nouveau combat, événement de scénario ou dialogue n’est déduit de l’image.
Le dossier de la carte active reste conservé ; la révision est montrée via
le mode de revue isolé. Sources et prompts sont conservés avec cette revue.

## Sources et reproduction

Les images sont créées avec le skill **imagegen**, outil intégré, puis copiées
dans `assets/source-art/first-sector-review-01/u01-expansion-r04/`. La liste
complète des consignes est dans `imagegen-prompts-r04.json`. Les deux derniers
prompts (`war_buildings` et `war_ground`) portent la direction corrigée.
Les atlas sources conservent l’alpha ; le conditionnement découpe les motifs,
rogne les marges et exporte à leur taille native sans changer leur proportion.

`new-buildings.json`, `decor-catalogue.json` et `decor-placements.json` conservent
les codes, dimensions, couches et placements. `candidate-r04.json` conserve
les empreintes. Le rendu de revue n’a aucune dépendance à une capture.

```powershell
python tools/reviews/build_street_aftermath_review.py
cargo run --locked -- --ui-cold-street-expansion artifacts/street-expansion-review-01/engine-r04
python tools/reviews/show_street_north_review.py
```

Les scripts utilisent le runtime Python fourni avec Pillow et NumPy.
