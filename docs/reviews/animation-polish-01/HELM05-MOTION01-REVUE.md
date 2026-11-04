# HELM05-MOTION01 — raccord tête, cou et torse

Le 31 octobre 2026, le joueur signale : « la tête semble bouger indépendamment
du reste du corps ». Il précise ensuite : « Dans toutes les vues ».
La reprise EYE01 du dessin de l'optique ne résout pas ce défaut d'animation.

## Cause et proposition 01

Le casque était placé d'après le haut du crâne de chaque phase. Les repères
verticaux restaient centrés en X, alors que le haut du torse pouvait se décaler
d'un pixel sur le côté. Exemple reproductible : Traqueuse / armure lourde /
tir à deux mains, phase 1 vers phase 4. Le torse se déplace de +1 px en X,
pendant que l'ancien repère de tête se déplace de +1 px en Y. Sur les côtés,
la mesure de crâne et l'inclinaison issue d'une ligne de pixels de cou ajoutaient
encore leur variation à la translation du torse employée par ARM14.

La proposition conserve les six corps dessinés et leurs armes. Elle recale
la tête peinte et son casque comme une seule unité, par translation entière,
sans étirer l'anatomie. Le torse, les bras, les prises et les jambes restent
ceux des phases existantes. Les deux rangées de cou conservées servent de
recouvrement au raccord ; les pixels sous ce raccord ne sont pas déplacés.

Face/dos : une petite zone centrale de poitrine du corps nu d'arme sert à
mesurer la translation entre les phases de chaque orientation. Le calcul
est effectué hors moteur par `prepare_helmet_motion_review.py`. Il fournit
260 attaches pour les 20 tenues du pilote, dont 240 phases marchées ; 93 de
ces phases reçoivent une correction de 1 ou 2 px par rapport au repère précédent.
Les mesures, scores et empreintes des sources sont dans
`assets/source-art/animation-polish-01/helmet-worn-r01/HELM05-MOTION01-anchors.json`.
Ce recalage est une proposition mesurée sur le dessin, pas un squelette 3D.

Gauche/droite : la référence de tête et son inclinaison viennent de la même
phase compacte que les bras ARM14. La tête et le casque suivent ensuite
exactement sa translation de torse, au lieu de recalculer leur inclinaison
sur chaque phase. La préparation utilise les ressources et caches existants.
Aucune lecture lourde ni analyse de pixels n'est ajoutée au dessin.

## Revue native

L'aperçu courant conserve `#casques-portes` et présente l'avant à gauche,
la proposition 01 à droite, à la même phase : taille native puis zoom entier
×2. Les sélecteurs couvrent les quatre classes, cinq casques, cinq tenues et
quatre orientations. L'ancien moteur est conservé une fois dans
`helmet-worn-r01/current/motion01/before/` pour ce comparatif reproductible.

Le lecteur propose la marche continue par défaut, afin de juger le raccord
sans arrêt frontal artificiel entre les pas. « Marche puis arrêt » conserve
la revue du vrai repos frontal ; pause, ralenti et sélection de phase restent
disponibles. La planche complète de tous les casques est toujours accessible.

## Limites et état

La proposition porte sur le pilote HELM05 : quatre classes, cinq tenues,
mitrailleuse 31 en couleur d'origine et cinq dessins communs de casque.
Les icônes acceptées et les dessins sources de casques ne sont pas modifiés.
Les autres familles de prises ne sont pas généralisées par ce pilote.

Le changement reste limité au diagnostic Debug lorsque HELM05 est sélectionné.
Le jeu normal, les identités des objets, les règles et les sauvegardes conservent
leur fonctionnement. L'acceptation artistique du mouvement reste distincte
des vérifications techniques.

## Reproduction

```powershell
python tools/art/prepare_helmet_motion_review.py
cargo test --locked --bin project-rl animation_polish_review:: -- --nocapture
cargo build --locked --bin project-rl
& .\target\debug\project-rl.exe --ui-cold-helmet-worn artifacts/animation-polish-01/helmet-worn-r01/current
python tools/art/build_animation_polish_review.py
```

## Vérifications du 31 octobre

Les 13 tests ciblés du lot passent avant la dernière correction de recouvrement
du cou, puis les cinq tests portés passent après celle-ci, dont le nouveau cas
du recul de deux pixels. La compilation Debug finale et le contrôle Release
passent ; la dernière correction appartient au module de revue Debug.
La suite complète du projet n’a pas été exécutée.

Le dernier passage moteur produit 140 planches. Par comparaison RGB avec le
témoin conservé, les 140 lignes de personnages sans nouveau casque, les
2 800 extraits des bras/armes et du bas du corps, et les 20 planches de repos
restent identiques. Les empreintes des 25 fichiers de dessins et d’icônes
contrôlés ainsi que des 20 corps sources sont inchangées. Le navigateur
vérifie les sélecteurs, les quatre vues, la pause, la phase de repos et la
marche continue ; aucune erreur de console n’est relevée. Les résultats sont
détaillés dans `HELM05-MOTION01-VALIDATION.json`.
