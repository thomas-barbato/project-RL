# HELM05-MOTION02 — remplacer la tête peinte par une tête casquée complète

Le joueur refuse MOTION01 : le casque est décalé, sa superposition sur la tête
reste visible et les mouvements de tête paraissent encore étranges.
`HELM05-MOTION02-RETOUR.json` conserve ce retour ; MOTION01 n'est pas accepté.

## Défaut du montage précédent

Le corps latéral contient parfois déjà une protection de tête peinte dans
la tenue. La superposition d'une coque ouverte ajoutait donc un deuxième casque.
Les têtes de face/dos conservaient également des cheveux ou une ancienne
protection hors de la nouvelle coque. Le recalage déplaçait les anciens pixels
de tête de chaque phase, dont le dessin continuait de varier sous la coque.

## Proposition 02

Les modèles ouverts 01 et 02 sont redessinés comme des **têtes portant le
casque** : crâne, visage visible, protection, cheveux contenus et petit raccord
de cou appartiennent au même dessin. Les quatre visages s'appuient sur les
références existantes. Les vues gauche/droite du modèle 02 ont été reprises
séparément après un premier essai montrant deux directions droites ; sa garde
nasale/inférieure est ensuite rétablie pour conserver l'identité de l'icône.
Les trois modèles fermés reprennent leurs dessins existants en têtes complètes.
Les icônes acceptées ne sont pas modifiées.

La fabrication utilise **ImageGen intégré** pour les dessins ouverts ; le
prompt exact de chaque étape est dans `HELM05-MOTION02-PROMPTS.json`.
Les sources sont dans `assets/source-art/animation-polish-01/helmet-head-units-r02/`.
Le conditionnement découpe les vraies bandes de chaque feuille, retire les
petites composantes alpha détachées de détourage, garde le dessin RGB et le
rapport largeur/hauteur, puis produit les couches natives. Le pivot suit la
base du cou visible, pas le centre de la boîte d'un casque asymétrique.
Les 80 vues sont consignées dans `HELM05-MOTION02-SOURCES.json`.

Le montage efface l'ancienne tête dans sa zone de propriété pour **tous** les
modèles, puis pose une seule tête casquée. Il ne conserve aucune ancienne face
animée sous un modèle ouvert. Le même dessin complet sert aux six phases d'une
orientation, avec la translation du torse, sans rotation supplémentaire de
la tête estimée sur le crâne. Le cou latéral suit le même repère de torse
qu'ARM14. Les attaches verticales conservent le déplacement de poitrine mesuré
dans MOTION01, avec un pivot de cou plus bas correspondant à la tête complète.

La proposition couvre les cinq casques, quatre classes, cinq degrés d'armure,
quatre orientations, six phases et le vrai retour frontal du pilote équipé
de la mitrailleuse 31. La taille de base de la tête complète est 17 px, modulée
par les facteurs de carrure/armure précédents ; cette nouvelle taille fait
elle aussi partie de la revue, sans reprendre automatiquement l'accord 04.

## Comparaison et limites

L'aperçu courant `#casques-portes` oppose **01 refusée** à **02 proposée**, à la
même phase, à taille native puis au zoom entier ×2. Il garde pause, ralenti,
marche continue et arrêt réel. La planche de têtes au zoom ×4 permet d'examiner
les quatre orientations et visages du casque sélectionné. L'ancien montage
est conservé une fois dans `current/motion02/before/` pour cette comparaison.

Ce lot prépare une nouvelle proposition visuelle. Il n'est pas activé dans le
jeu normal ; il ne change ni objets, règles, sauvegardes ni les autres familles
de prises. Les tests d'occultation et de stabilité technique ne valident pas
à eux seuls l'identité, la perspective, les proportions ni la qualité du mouvement.

## Reproduction

```powershell
python tools/art/prepare_helmet_head_units.py
cargo test --locked --bin project-rl animation_polish_review:: -- --nocapture
cargo build --locked --bin project-rl
& .\target\debug\project-rl.exe --ui-cold-helmet-worn artifacts/animation-polish-01/helmet-worn-r01/current
python tools/art/build_animation_polish_review.py
```

## Vérifications du 31 octobre

Les 14 tests ciblés du lot passent, puis les cinq tests portés passent à nouveau
après le dernier conditionnement alpha natif. Le nouveau contrôle compare
40 paires de têtes peintes en couleurs opposées : la tête complète affichée
ne dépend plus de la couleur de l’ancien casque peint sous elle.
La compilation Debug finale et le contrôle du binaire Release passent.
La suite complète du projet n’a pas été exécutée.

Le moteur produit 140 planches. Les 140 lignes témoins et les 2 800 extraits
contrôlés des bras/armes et du bas du corps restent identiques en RGB.
Les 20 planches de repos reçoivent la nouvelle tête, comme prévu.
Les empreintes de 25 fichiers d’icônes/dessins antérieurs et des 20 corps
sources restent inchangées. Sur le Bastion en armure lourde avec le modèle 02,
la région couronne/œil recalée sur le torse est identique sur 6/6 phases de face
et 6/6 de dos. Ce contrôle décrit la stabilité du dessin, sans accepter son
apparence artistique. Le navigateur vérifie les quatre orientations, les modèles
ouverts/fermés, le sélecteur des têtes, pause, arrêt et lecture continue, sans
erreur de console. Voir `HELM05-MOTION02-VALIDATION.json`.
