# AIM-VIS-02 — Hologramme bleu et invalide B

Choix exprimé le 18 octobre 2026 :

> J'aime : A - Hologramme bleu (mais il faudrait ajouter les petits cercles
> comme sur la B. B - "invalide"

- **Zone valide** : reprendre le halo, la transparence et la couleur de A,
  ajouter de petits cercles sur le contour aux emplacements des repères de B.
  Les petits losanges de la proposition initiale B deviennent bien des
  cercles dans la nouvelle zone valide, conformément à la demande.
- **Zone invalide** : reprendre B tel que présenté, avec le rouge franc
  RGB 255, 48, 38, les hachures, les repères et les angles biseautés.

La combinaison finale est explicitement validée par le joueur :
« parfait je valide :) ». Elle est intégrée au rendu normal du jeu dans
`src/attack_preview_visuals.rs`, avec les mêmes formes et couleurs.

La galerie courante `artifacts/aim-preview-review-01/index.html` est réutilisée.
La planche `choix.png` fait 960 × 480 px ; les captures complètes
`choix-valide.png` et `choix-invalide.png` font 1280 × 800 px. Les extraits
restent à l'échelle 1:1 avec des cases de 64 px. La correction d'orientation
du tampon graphique de AIM-VIS-01 est conservée.

Le mode de diagnostic `--ui-cold-aim-choice <dossier>` vérifie maintenant le
rendu normal intégré, sans remplacement de l'empreinte par un cache de revue.
La géométrie des premiers sorts reste calculable pendant un manque d'énergie,
une recharge ou une récupération ; la prévisualisation exécutable et le
lancement vérifient toujours les mêmes conditions. Le manque d'énergie
continue donc à bloquer l'action sans dépense ni tour consommé.
Les cellules cachées, les obstacles et la portée gardent leurs contrôles.
Les règles d'action, les commandes et les ressources ne changent pas.

La proposition d'origine reste documentée dans `aim-visuals-02-native.json`.
Le contrôle après intégration est dans `aim-visuals-02-integration-native.json`.
