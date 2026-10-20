# MAP-U01-06-r01 — première rue jouable

18 octobre 2026. Perspective acceptée dans `visual-acceptance-r06.json`.
Le camp reste la référence stricte de projection et d’échelle.

Le traitement final **SCENE-U01-07 / LUM-U01-07** est ensuite accepté :
surfaces simplifiées et éclairage nocturne calculé par le moteur. Les sources
et textures numériques validées remplacent la version illustrée antérieure
dans les ressources de la rue. Les règles, identifiants et passages restent
conservés. Voir [U01-STYLE-LUMIERES-07](U01-STYLE-LUMIERES-07.md).

## Comportement

La génération 155 conserve le camp comme départ. Son passage existant mène
aux « Abords de la prison », avec arrivée au portail sud de la rue. Le
passage réciproque ramène au même camp, avec ses PNJ et services conservés.
Les générations 154 et antérieures gardent leur destination historique.
Une nouvelle partie utilise la rue ; une sauvegarde ancienne conserve son monde.

La première habitation à gauche est accessible par son entrée sud. L’intérieur
est en coupe, dans la même carte : chaque case parcourue utilise le déplacement
normal, au clavier ou avec le chemin à la souris. Aucun changement de niveau
ni chargement supplémentaire n’est nécessaire pour cette habitation.
Les autres bâtiments restent des décors pour cette première étape.

Le portail n’ajoute pas de condition de quête. Le dialogue existant de
l’officier et le système de quêtes sont conservés ; les missions seront
traitées dans leur étape dédiée. Aucun nouveau monstre ou butin n’est placé.
La suite des quartiers reste à construire.

## Fabrication et rendu

La grille logique mesure 26 × 18 cases de 64 px. La source illustrée couvre
24 × 16 cases à raison d’un pixel source par pixel du monde. Un décalage
horizontal de 32 px aligne l’ouverture de la maison sur la grille ; ce
décalage ne redimensionne aucun dessin. Les 173 cases praticables sont reliées,
y compris les trottoirs dégagés devant le garage et au sud.

Le fond original de SCENE-U01-06-r01 est conservé sans retouche. La voiture
est une couche indépendante de 128 × 64 px, avec les flammes déjà présentées
et des collisions sur deux cases. Les autres obstacles bas gardent la vue
libre ; les murs arrêtent la vue et le mouvement. Le linteau de la maison
reprend les pixels de la source en couche avant au passage du personnage.

Le rendu conserve le brouillard, la mémoire du terrain et le cadrage du jeu.
Les textures sont chargées une fois, en filtrage nearest ; aucune image n’est
redécodée ou réenvoyée au GPU par image. Cette première carte fixe n’est pas
un générateur de quartiers et ne remplace pas l’agencement accepté du chapitre.

Ressources utilisées par le jeu : `assets/maps/prison-street-01/`.
Fabrication : `tools/reviews/build_u01_playable.py`.
Source : `assets/source-art/first-sector-review-01/u01-scene-r03/`.
Le jeu ne lit aucun fichier sous `artifacts/`.

## Contrôles

Huit tests ciblés du camp et de la rue réussissent : départ des classes,
services et PNJ, ancienne génération, aller-retour au portail, trajet dans
la maison, obstacles et sauvegarde/reprise dans la maison et dans le camp.
Les fixtures consomment les événements comme l’entrée normale du jeu.
La suite complète n’est pas déclarée validée par ces tests ciblés.

Les captures réelles sont produites par :

```text
cargo run --locked -- --ui-cold-prison-street artifacts/first-sector-review-01/engine-r06
```

Elles passent par la création du personnage, le passage du camp et les
commandes réelles de déplacement. Le dossier contient le portail, la rue,
l’entrée de la maison, son intérieur et la branche nord, ainsi que les
contrôles de rendu sans mutation et les cellules de 64 × 64 px. Le chemin
de sauvegarde du diagnostic est isolé dans ce dossier.
