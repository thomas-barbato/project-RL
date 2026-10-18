# WALK-VIS-02 — Six dessins de destination

Demande du joueur : « plusieurs proposition » de dessin de destination,
après hésitation sur le premier double cercle de WALK-VIS-01.
Les six candidats restent **en attente de validation visuelle explicite**.

La couleur, le chemin, ses numéros, la destination, les sprites et leur taille
restent identiques. Les contrôles et règles de déplacement ne changent pas.

- **WALK-VIS-02-A — Balise** : repère dressé avec pointe sur le point d'arrivée,
  base elliptique au sol et numéro dans la tête.
- **WALK-VIS-02-B — Losange** : double contour facetté autour du numéro final.
- **WALK-VIS-02-C — Angles ouverts** : quatre coins lumineux, centre dégagé.
- **WALK-VIS-02-D — Anneau segmenté** : quatre arcs séparés autour du numéro.
- **WALK-VIS-02-E — Dalle lumineuse** : plaque translucide biseautée avec
  deux petits repères sur ses bords.
- **WALK-VIS-02-F — Empreintes** : deux semelles stylisées, numéro placé en bas.

Le numéro final reste celui du dernier pas ; pour cette scène, six cases sont
à parcourir. Le changement de position de ce numéro dans A et F fait partie
du dessin soumis à revue.

Les dessins sont produits directement par le moteur, sans nouvelle image de
personnage, arme ou décor. Ils sont contenus dans le module de diagnostic
`src/walk_preview_review.rs`, inactif dans la partie normale.
Le premier dessin reste celui de l'essai interactif tant qu'aucun nouveau
candidat n'est accepté.

La galerie existante `artifacts/walk-preview-review-01/index.html` est réutilisée.
`destinations.png` est une planche de 1040 × 720 px avec six extraits à échelle
1:1 ; les captures complètes `destination-a.png` à `destination-f.png` font
1280 × 800 px. Les cases mesurent toujours 64 px. Le tampon graphique est
retourné correctement lors de la composition native des extraits.

Reproduction : `--ui-cold-walk-destinations <dossier>`.
Le contrôle natif est conservé dans `navigation-destinations-02-native.json`.
