# TECHNOMANCIEN-NOROBE-HEADS02 — têtes lisibles et progressives

22 octobre 2026. Le joueur constate que les cinq têtes n’évoluent pas et sont
trop sombres à 64 px. La correction porte uniquement sur les têtes par défaut
des cinq tenues mécaniques sans robe. Aucun dessin n’est intégré au jeu.

Les éditions utilisent l’outil intégré `image_gen` ; les prompts exacts,
références et fichiers générés sont dans `prompts.json`. Les sources complètes,
éditions brutes, masques et exports sont conservés dans
`assets/source-art/class-readability-01/technomancien-no-robe-heads-02/`.

## Progression proposée

1. `TE-NR01-P01-r02` : crâne mécanique découvert en acier plus clair, visage sombre et mâchoire bronze.
2. `TE-NR01-P02-r02` : bande de front et premières protections des tempes.
3. `TE-NR01-P03-r02` : demi-casque clair ouvert, couronne et joues renforcées.
4. `TE-NR01-P04-r02` : protection fermée des joues et du menton, optique dans un retrait sombre.
5. `TE-NR01-P05-r02` : casque fermé, face acier, fente optique et mâchoire compacte.

Les grands plans acier clairs remplacent les masses de crâne presque noires.
L’optique ambrée reste unique. Les têtes gardent une échelle compacte et une
nuque courte ; aucune robe, capuche ni pièce de tissu n’est réintroduite.

Ce sont les têtes fournies par les objets de torse, conformément à la règle
acceptée. Leurs formes de casque ne créent pas de nouveaux objets équipables.
Un véritable casque équipé devra remplacer la tête par défaut après sa revue.

## Conservation exacte des corps

Une édition de source entière peut modifier d’autres pixels malgré sa consigne.
Le script utilise donc un masque de tête explicite et le cadrage de chaque
source précédente. Il conserve tous les pixels natifs hors de cette région,
dont tout le corps à partir de y = 13. Les sources hautes sont aussi assemblées
avec un masque explicite, sans peinture ni recoloration par code.

La réduction reste isotrope Lanczos, avec alpha ; les zooms sont Nearest.
Le corps et la tête se recomposent exactement. Le masque reste un masque
d’étude, pas un rig d’animation ou une prise d’arme validée. La réduction
directe de la source assemblée peut différer à la frontière : l’export natif
restaure explicitement les pixels du corps pour en garantir l’identité.

Les contrôles enregistrent zéro pixel natif changé hors tête pour les cinq
paliers, ainsi que la conservation des 798 images et accords antérieurs.
La luminance moyenne des régions solides de tête passe de 67–76 à 100–111
sur 255. Cette mesure confirme l’éclaircissement ; elle ne remplace pas le
jugement de lisibilité du joueur.

La planche principale conserve les sprites en 64 × 64 à échelle CSS 1.
Les fonds sombre et clair ainsi que le montage sur une capture de rue sont
préparés ; le rendu de ces dessins sous l’éclairage moteur reste à examiner.
Le Bastion cuivre et ses acceptations restent inchangés.

Les nouveaux pixels attendent l’accord visuel. Casques indépendants, prises
d’armes, variantes et intégration restent différés. Les éléments de preuve
de la vérification du navigateur sont dans `ui-checks.json`.
