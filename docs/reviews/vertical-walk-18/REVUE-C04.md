# VERT18-C04 — têtes découvertes et six protections de tête

Préparé après l'accord du 29 octobre 2026 sur C03, conservé avec ses sources
dans `acceptance-C03.json`. **C04 reste une nouvelle proposition, sans activation
dans le jeu normal.** Il répond à la demande explicite de versions casquées et
non casquées, avec maintien de la même tête équipée au retour de face.

## Contenu du comparatif

Les quatre armures lourdes de C03 sont présentées avec sept choix de tête :

0. Sans casque : visage, cheveux et nuque propres à chaque personnage.
1. Calotte Cognefer, modèle 115.
2. Capuche de mue, modèle 116.
3. Casque Cerbère, modèle 117.
4. Heaume Bastion, modèle 118.
5. Masque Chrysalide, modèle 119.
6. Visière Fantôme, modèle 120.

Le Bastion retrouve ses cheveux et sa barbe bruns ; la Traqueuse, ses cheveux
bruns et son optique latérale ; l'Arcaniste, ses cheveux blancs et son visage,
avec sa capuche rouge abaissée et son masque retiré. Le Métamorphe conserve sa
tête découverte existante et son bandeau. Les casques ouverts laissent voir
la tête de la bonne classe, sans ajouter un visage générique masculin.

Les six dessins frontaux et leurs ouvertures proviennent exactement des
ressources actuelles. Les six dos sont nouvellement dessinés, sans répliquer
une visière ou un visage à l'arrière. Les pièces sont indépendantes du corps,
suivent son attache de tête et sont composées avant les armes. Au repos, le
compositeur frontal conserve ses couches de corps, d'armes et de doigts ;
seule la partie tête/cou est remplacée avant recomposition des objets tenus.

Les cycles corporels et les prises de C03 sont conservés. Les 48 images de
corps restent identiques sous la zone de tête, à partir de y=60 dans la toile
128 px. Les opérations de casque vérifient aussi ce maintien à la préparation.
Les changements de tête sont placés dans un espace de clés de cache distinct,
pour conserver la reproduction de C01/C02/C03 et éviter les mélanges de casque.

Chaque choix est montré avec et sans la paire pistolet 104 / couteau 70, en
marche de dos, de face et au retour frontal. Les vues dans le camp montrent
les variantes découvertes et le Heaume Bastion sous les lumières réelles.
Les animations sans perte sont contrôlées après décodage ; leurs pauses de
lecture de 650 ms ne changent pas le pas de 400 ms.

## Sources et reproduction

Les trois éditions de têtes découvertes et la planche des six dos sont créées
avec **imagegen intégré**, avec transparence. Les figures sources entières et
les références sont conservées dans
`assets/source-art/vertical-walk-review-18/headgear-C04/`, avec les prompts et
empreintes dans `prompts-C04.json`. Le conditionnement extrait les pièces
tête/cou et les attache aux corps C03 ; il ne remplace aucune jambe ni armure.
Les ressources conditionnées sont dans `candidate-C04/`.

```powershell
python tools/art/pack_vertical_walk_headgear18.py
cargo build --locked
& ./target/debug/project-rl.exe --ui-cold-vertical-headgear artifacts/vertical-walk-review-18/current/headgear-C04/engine
python tools/art/publish_vertical_walk_headgear18.py
```

Le chargement et les compositions précèdent chaque séquence. Les pièces de
tête et de casque sont conditionnées en petites cellules de 32 px dans des
toiles transparentes ; le jeu reste en cases de 64 px. Les caches restent
bornés à 4 Mio pour les sources et 8 Mio pour les textures préparées. Le
dessin ne lit/décode aucun fichier et n'effectue aucune création de texture.
Les captures ne chargent ni n'écrivent de sauvegarde normale ou de préférence.

## Portée et suite

Ce lot porte sur les quatre armures lourdes actuelles, en palette Original.
La règle de composition est réutilisable pour les autres tenues, mais celles-ci,
le Technomancien de compatibilité, les palettes applicables et les prises à
deux mains restent à décliner. Les autres PNJ gardent leurs identités et
vêtements propres. L'accord C03 ne valide pas ces nouvelles têtes et ces six dos.
DUAL17-C02 reste une revue séparée ; l'intégration de VERT18 reste distincte
de la validation graphique des lots.

## Signalement de gabarit — 29 octobre 2026

Le joueur constate une variation de taille entre marche et repos et demande de
toutes les comparer, puis de tout corriger. C04 reste non validé. La reprise
complète des neuf corps présentés et des variantes de tête est préparée dans
[VERT18-C05](REVUE-C05.md), avec les sources C04 conservées comme références.
