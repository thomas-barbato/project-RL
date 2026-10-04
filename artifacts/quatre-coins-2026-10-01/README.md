# Reprise des murs et des quatre coins

Essai local du 1 octobre 2026, après rejet des raccords montrés dans les premières captures.

## Proposition actuelle

- Les quatre coins sont des choix distincts dans la palette : haut gauche, haut droit, bas gauche et bas droit.
- Le mur horizontal, le mur vertical et les quatre coins proviennent du même lot artistique. Le bord clair suit le pourtour des bâtiments rectangulaires.
- R conserve les quatre orientations manuelles du mur. La prévisualisation, la pipette, la copie, la sauvegarde et l'annulation conservent ce choix.
- Les rangées automatiques prennent leur face auprès du coin à leur extrémité. A rétablit le mode automatique après une rotation manuelle.
- Les coins sont des images entières. La préparation à 64 pixels ajuste leur silhouette et leurs bandes de connexion extérieures à la section commune des murs droits.

## Images et prompt

Le générateur d'images intégré a produit la nouvelle planche, avec les anciens murs comme référence de matériau. Mode utilisé : outil intégré `image_gen`, fond transparent, aucune API ou CLI externe.

- Source utilisée : [walls-and-corners-source-v4.png](../../assets/prototypes/surface64/walls-and-corners-source-v4.png).
- Prompt exact : [COHERENT-WALLS-PROMPT.txt](COHERENT-WALLS-PROMPT.txt).
- Les quatre sources v3 et leurs prompts individuels restent archivés. Elles ont été rejetées pour incohérence avec les murs droits et ne sont plus utilisées.

Le prompt demandait une vue plus uniforme et un éclairage fixe. La planche obtenue conserve un bord clair extérieur marqué. Le rendu actuel adapte donc les faces des murs à ces coins ; il ne constitue pas une validation de la direction artistique du jeu.

## Captures natives vérifiées

Les captures du dossier `coherent-captures` viennent de l'exécutable Macroquad de l'éditeur, à 64 pixels par case pour les raccords :

- [Quatre coins placés manuellement](coherent-captures/four-corners-manual.png) et [carte JSON correspondante](coherent-captures/four-corners-manual.json).
- [Contours et faces calculés automatiquement](coherent-captures/wall-corners.png).
- [Les quatre orientations des murs et des coins](coherent-captures/wall-rotations.png).
- [Portes ouvertes](coherent-captures/wall-corners-open-doors.png).
- [Démo meublée](coherent-captures/editor-1360.png).

## Validation

- `cargo test --locked --example map_editor --example textured_surface_preview` : 23 tests de l'éditeur et 9 tests de l'essai de surface passent.
- `cargo check --locked --examples` : succès.
- `cargo build --locked --example map_editor --example textured_surface_preview` : succès ; exécutable local de l'éditeur remplacé.
- Capture native complète en 1360 × 840 et 960 × 540 : succès.
- Revue visuelle des constructions manuelles et automatiques : le bord clair suit leur contour et les quatre coins raccordent les segments droits.

Le dessin diffère du lot précédent et attend l'avis de l'utilisateur. Les portes, extrémités et jonctions en T ou en croix restent à harmoniser artistiquement. Le client de campagne ne charge pas ces images et n'est pas modifié par cet essai.

Pour ouvrir directement la construction manuelle de contrôle depuis la racine du projet :

```powershell
.\target\debug\examples\map_editor.exe --map artifacts/quatre-coins-2026-10-01/coherent-captures/four-corners-manual.json
```
