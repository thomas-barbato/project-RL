# Premier catalogue de surface

Ces images sont des sources expérimentales pour `examples/map_editor.rs` et `examples/textured_surface_preview.rs`. Elles ne sont pas chargées par le client de campagne.

Les PNG ont été produits avec le générateur d'images intégré lors des essais de ce chat. Les originaux et les prompts du premier lot sont conservés dans `artifacts/textures-lot1-review-2026-10-01` :

| Source du catalogue | Original et prompt |
|---|---|
| `terrain-and-objects-source.png` | `atlas-source.png` et `PROMPT.txt` |
| `walls-source.png` | `walls-correction-source.png` et `WALLS-PROMPT.txt` |
| `furniture-source.png` | `furniture-source.png` et `FURNITURE-PROMPT.txt` |
| `floors-borderless-source-v2.png` | Édition des 10 sols sans bordure ; prompt dans `artifacts/sols-sans-contours-2026-10-01/PROMPT.txt`. |
| `walls-and-corners-source-v4.png` | Lot commun : mur horizontal, mur vertical et quatre coins. Générateur intégré ; `artifacts/quatre-coins-2026-10-01/COHERENT-WALLS-PROMPT.txt`. |
| `wall-corner-nw-source-v3.png` | Coin haut gauche ; `artifacts/quatre-coins-2026-10-01/PROMPT-nw.txt`. |
| `wall-corner-ne-source-v3.png` | Coin haut droit ; `artifacts/quatre-coins-2026-10-01/PROMPT-ne.txt`. |
| `wall-corner-sw-source-v3.png` | Coin bas gauche ; `artifacts/quatre-coins-2026-10-01/PROMPT-sw.txt`. |
| `wall-corner-se-source-v3.png` | Coin bas droit ; `artifacts/quatre-coins-2026-10-01/PROMPT-se.txt`. |

Le renderer lit les cellules des planches à 64 × 64 pixels, sans agrandir les anciens échantillons à 32 pixels. Il normalise les silhouettes des murs et des portes pour leur donner des raccords communs. Les PNG sources restent inchangés.

Les murs droits et les quatre coins actifs proviennent désormais d'une même planche v4. Les coins v3 dessinés individuellement sont conservés comme essais rejetés : ils ne sont plus chargés. Chaque coin v4 est lu comme une pièce entière puis adapté aux raccords de 64 pixels. Ses bordures suivent le profil du mur de référence sur toute leur longueur ; son panneau reste celui de l'image dédiée. La rotation d'un coin choisit l'image correspondante. Les murs horizontaux et verticaux ont des sources distinctes ; leurs faces opposées utilisent une rotation de 180°. Le bord clair suit le pourtour du bâtiment. Les marges transparentes des murs et portes reprennent le terrain situé de leur côté : le sol intérieur ne déborde plus par-delà le mur. Le sol enregistré sous la structure et le seuil d'une porte sont conservés.

La bordure des angles est la référence visuelle retenue par l'utilisateur. La préparation des murs droits reprend les profils extérieur et intérieur des parties droites du coin haut gauche, en conservant leurs panneaux centraux. Le liseré sombre et le biseau gris gardent ainsi les proportions du coin, plutôt que celles du recadrage indépendant du mur. Les bandes de raccord des coins utilisent ensuite ces mêmes profils. Les images sources restent inchangées ; captures et diagnostic dans `artifacts/bordures-murs-2026-10-01`.

Après revue des orientations, le mur orienté vers le haut constitue la référence commune préférée par l'utilisateur. Sa bordure est conservée et les murs verticaux reprennent exactement ses pixels par rotation de 90°. Les faces opposées utilisent les rotations habituelles de 180°. Les panneaux des murs verticaux restent distincts ; leurs bordures ne sont plus échantillonnées séparément sur le bras vertical du coin. Un test vérifie l'égalité des pixels de bordure des quatre faces après remise dans le même sens. La revue actuelle est conservée dans `artifacts/murs-orientations-2026-10-01`.

La revue suivante corrige les coins sur la totalité de leurs bordures. Le remplacement limité aux extrémités laissait un profil différent dans les coins, avec un trait blanc sur leur bras vertical et un décalage au raccord. Les deux bras et leur coude utilisent désormais le profil commun, tandis que les panneaux centraux restent conservés. Les tests vérifient aussi cette continuité de couleur et la conservation du panneau. Les dernières captures utilisables sont dans `artifacts/raccords-coins-2026-10-01/captures-validees`.

Les sols 0 à 9 proviennent désormais de `floors-borderless-source-v2.png`. Les cellules d'objets de cette nouvelle planche ne sont pas utilisées : caisse et terminal conservent exactement leur source originale. Les cadres des plaques et dalles et les bords de cellules sont retirés. La grille de l'éditeur est masquée par défaut et peut être affichée avec G.

Les indices sont fixes pour ce lot : sols 0 à 9, caisse 14, terminal 15 ; meubles 0 à 7 dans la planche de mobilier. Les pièces de murs servent à construire les 16 configurations de voisins. Les portes ouvertes et fermées possèdent quatre rotations.

La rotation manuelle des murs droits utilise leur orientation enregistrée. R fixe la forme et la tourne ; A rétablit les raccords automatiques. Les segments droits automatiques prennent leur face auprès du coin situé au bout de leur rangée. Un mur automatique isolé présente la même pièce droite que sa prévisualisation. Quatre choix de coins dans la palette permettent aussi de placer directement chaque angle, sans dépendre des voisins. Les jonctions en T, croisements, extrémités et portes restent des pièces expérimentales à revoir artistiquement.

La palette initiale contient 10 sols, un outil de murs, deux états de porte, le départ du joueur et 10 objets. Les objets occupent une case et tournent actuellement par rotation de leur image. Cette méthode et la cohérence de l'éclairage demandent une revue artistique avant adoption dans le jeu.

La bibliothèque extensible, les identifiants stables et les variantes dessinées par orientation sont décrits dans `docs/EDITEUR_CARTES.md`.
