# MARKER-PULSE-02 — Centrage du remplissage

Le 18 octobre 2026, le joueur choisit explicitement la pulsation progressive,
mais signale que le remplissage n'est pas centré sur le trou :
« pulsation progressive mais parcontre, la pulsation n'est pas centrée sur le trou ».
Le rythme et les couleurs de MARKER-PULSE-01 sont conservés. Cette révision
concerne uniquement le centrage et l'ajustement du remplissage à l'ouverture.
Le joueur valide ensuite explicitement le centrage corrigé : « Je valide » ;
il demande un avis sur l’intérêt de faire pulser les nombres de déplacement.

Le disque précédent utilisait 20 côtés contre 30 pour le contour du cercle,
avec un rayon inférieur à celui de son ouverture (2,6 contre 3,5 px pour
les repères de zone). Cela laissait une marge irrégulière à cette taille.
Le nouveau rendu donne au fond, au remplissage et au contour un seul centre,
ajusté ensemble au demi-pixel, et les mêmes sommets : 64 pour un cercle,
4 pour les repères invalides en losange. Le remplissage rejoint exactement
le bord intérieur du contour et épouse aussi les ouvertures en losange.
La taille ne varie pas pendant la pulsation : seule l'opacité change.

Les teintes, le cycle progressif de 1,1 seconde, les chiffres, les trajets et
leurs règles restent les mêmes. La coupure avant la dalle E est conservée.
Le rendu centré validé est maintenant intégré dans `src/preview_markers.rs`,
compilé dans le jeu normal et en release. Seul le contrôleur de capture reste
spécifique au débogage. Aucun fichier sous `artifacts/` n’est chargé par le jeu.

L'aperçu courant est réutilisé :
`artifacts/walk-preview-review-01/cercles-animes.html`.
Le mode natif `--ui-cold-marker-pulse <dossier>` reproduit cette révision.

La capture native et le code de production de la revue ont été compilés.
Le contrôle release passe avec le seul avertissement préexistant concernant
`draw_inspection_panel`. Les tests mécaniques ne sont pas relancés pour cette
correction visuelle confinée à la revue. Le formatage et `git diff --check` passent.
Le contrôle d'un repère circulaire sur la capture confirme le même centre
vertical pour le remplissage et son contour, sans écart de pixel.
L'animation native est `marker-pulse/cercles-progressifs-centres.png`, en
1280 × 400, sans redimensionnement. Ses douze images décodées sont identiques
aux pixels capturés dans le moteur. Le bilan est dans `marker-pulse-02-native.json`.

## Intégration après validation

Dernier message : « Je valide, par-contre je sais pas s'il est judicieux
d'appliquer ces clignottement aux nombres de déplacement, qu'en penses tu ? ».
La recommandation est de garder les nombres et leurs cercles fixes pour lire
le trajet sans animation concurrente. Leur dessin statique déjà accepté dans
WALK-VIS-03-E est donc conservé tel quel. Cette décision n'ajoute aucun rendu
non présenté pour ces nombres. Les petits repères de la dalle E et ceux des
zones de compétence reçoivent la pulsation progressive et le centrage validés.
Les teintes, contours et règles de déplacement restent identiques.

Le cycle visuel de 1,1 seconde utilise l’horloge de rendu, indépendamment des
tours du jeu. Une seule phase est calculée pour chaque image et partagée entre
les repères. L’option existante de réduction des animations conserve un
remplissage fixe à l’intensité maximale acceptée. Les chiffres et tous leurs
cercles restent fixes dans les deux modes.

La capture native contrôle aussi le moteur en fonctionnement normal sans
phase de revue : variation réelle de couleur des petits repères, stabilité
des régions numérotées et arrêt de la pulsation avec réduction des animations.
Le contrôle après intégration est conservé séparément de la revue initiale
dans `marker-pulse-02-integration-native.json`.

Contrôles de l'intégration : compilation debug, contrôle release, formatage
et `git diff --check` réussis ; 20 tests ciblés de déplacement et de visée passent.
Le seul avertissement release reste l'ancien `draw_inspection_panel` inutilisé.
La capture mesure une variation des petits repères de 169 niveaux RVB pour
la zone et 157 pour l'arrivée, sans phase de revue ; les régions numérotées
restent identiques. Avec réduction des animations, leur couleur est fixe.
Les empreintes des trois états de jeu restent inchangées.

Les douze images de l’animation APNG intégrée sont vérifiées sans perte.
Les onze PNG intermédiaires inutiles sont envoyés à la Corbeille ; l’animation,
la phase maximale utilisée par le bouton de pause, les sources et les bilans
sont conservés. Aucun fichier d’édition de carte ni sauvegarde n’est touché.
L’aperçu courant utilise `marker-pulse/cercles-integres.png`.
