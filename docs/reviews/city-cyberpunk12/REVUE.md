# CITY-CYBERPUNK12 — première ville avant la nouvelle

Le 5 novembre 2026, après la pré-validation de B01 r04, le joueur précise :
« ok donc on va refaire la premiere map de ville, puis la nouvelle , on garde leurs tracés :) ».
La reprise commence donc par la première ville. Le quartier nord conserve
son plan pré-validé et sera repris ensuite. Les égouts restent exclus de la
reprise, conformément à la demande précédente. Les autres familles de décors
du chantier général restent en attente après les deux villes.

La direction reste celle de B01 r04 : futuriste et cyberpunk, grimdark,
sale et abîmée, avec équipements intégrés aux usages, matières mates,
contours cohérents, dispositifs en panne et connexions réparées ou arrachées.
Conserver les tracés n'impose pas de recopier le même bâtiment partout.

## Premier lot — trois bâtiments de la cour centrale

- **V1-B01** : garage ouest, position `[176,2400]`, toile native `326 × 309`.
  Relais de commande, réseau raccordé, caméra et lecteur d'accès.
- **V1-B02** : bâtiment bleu à l'est, position `[1340,2176]`, `233 × 484`.
  Relais de communications, réseau technique et contrôle d'accès.
- **V1-B03** : annexe au sud-est, position `[1350,2740]`, `220 × 359`.
  Distribution électrique, conduites endommagées, plaque de maintenance et terminal.

Onze éléments indépendants sont conservés sous `assets/source-art/city-cyberpunk12/`.
Leurs identifiants complets et positions figurent dans le manifeste voisin.
Les sources imagegen sont des études du module entier ; seules les nouvelles
pièces techniques sélectionnées sont composées sur le vrai décor. Les autres
pixels des études sont écartés. Les murs, silhouettes, portes et sols existants
ne sont pas remplacés par une géométrie approximative issue de la génération.

**Ce lot est une première proposition, pas la première ville terminée.**
Les autres bâtiments, intérieurs, ruines et mobiliers restent à reprendre.
Leur futur dessin sera présenté avant intégration. L'accord sur B01 r04
sert de référence de direction et ne valide pas automatiquement ces pièces.

## Géométrie et retouches

`SCOPE.json` fige les empreintes des cartes de ville, des égouts et du
quartier nord avant ce lot. Aucun fichier actif de carte, collision,
visibilité, passage ou sauvegarde n'est modifié. Le fond candidat a la
même taille et la même couverture alpha que le fond courant. Les pixels
en dehors des onze pièces et dans les ouvertures protégées restent identiques.

Chaque pièce technique possède son propre PNG et son propre identifiant.
Une retouche future modifie la seule pièce demandée, dans une révision
voisine ; les autres couches conservent leurs fichiers et empreintes.
La composition seule se refait avec `tools/art/build_city_cyberpunk12.py --compose-only`,
sans régénérer les dessins. La génération initiale est bloquée après
présentation au joueur, afin de préserver les versions inspectées.

## Sources et préparation

Outil : **image_gen.imagegen intégré**, édition avec références. Sources :

- `assets/source-art/city-cyberpunk12/V1-B01-west-garage-source-r01.png` ;
- `assets/source-art/city-cyberpunk12/V1-B02-east-blue-source-r01.png` ;
- `assets/source-art/city-cyberpunk12/V1-B03-east-south-source-r01.png`.

Les trois prompts exacts sont les fichiers `V1-B0*-PROMPT-r01.txt` de ce dossier.
Le manifeste associe chaque source à son prompt, à ses couches et à leurs empreintes.
Référence approuvée : `docs/reviews/north-city-01/PRODUCTION10-B01-ACCEPTANCE-r04.json`.

`examples/city_cyberpunk12_review.rs` prépare les ressources sur le worker commun,
puis crée les textures sur le thread graphique. Les plafonds du cache restent
ceux du jeu. Les captures sont à 64 px par case, avec le même acteur natif.
Elles emploient les pixels du vrai fond de carte ; l'éclairage dynamique,
la visibilité et le déplacement de campagne ne sont pas évalués par cet aperçu.

Lancer `Apercu_premiere_ville.cmd` : Tab compare avant/après ; haut/bas
parcourent la carte, Échap ferme la revue. Le déplacement concerne la caméra.
L'aperçu courant est `artifacts/city-cyberpunk12/apercu.html` ; ses images
avant/après et ses vues de détail conservent la même échelle et le même acteur.

Statut : **lot 01 pré-validé** le 5 novembre 2026 (« oui ok on continue :) »).
Empreintes figées : `LOT01-ACCEPTANCE-r01.json`. La suite concerne les deux
intérieurs centraux et l'entrepôt, dans le lot 02 ; les pièces du lot 01 restent conservées.
Validation définitive en jeu et intégration à la campagne : en attente.

Le lot courant devient le **lot 02** : logement, boutique et entrepôt en L,
avec vues ouvertes/fermées et correction ciblée des accès demandée ensuite.
Voir `LOT02.md` et `SECURITE-r02.md`. L'historique du lot 01 reste intact ;
la pré-validation du style n'impose pas de conserver ses scanners après
la nouvelle précision du joueur.

Le secteur d’arrivée forme le **lot 03**. La maison visitable, le commerce
et le portail sont présentés à taille native dans `LOT03-ARRIVEE.md`.
La variante du lit raccourci est prévalidée dans `LIT-ACCEPTANCE-r04.json`.
