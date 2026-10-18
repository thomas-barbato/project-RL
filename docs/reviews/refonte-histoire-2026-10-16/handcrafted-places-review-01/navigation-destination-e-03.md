# WALK-VIS-03-E — Dalle E avec chemin coupé

Le joueur retient E comme direction, pour sa cohérence avec les autres aperçus,
et demande de couper le trait visible dans la case finale jusqu'au numéro 6.

La dalle, ses couleurs, ses repères et son numéro restent identiques.
Le dernier segment est désormais raccourci jusqu'à la bordure de la case
de destination. Le contour de la dalle est à l'intérieur de cette case :
le tracé laisse donc aussi un petit espace avant ce contour.
Le chevron du dernier segment est repositionné sur la portion raccourcie.
Cette correction s'applique aux arrivées horizontales et verticales de E.
Les autres dessins, le tracé précédent et les règles de déplacement restent
inchangés. Aucun trait ne relie plus le bord de la dalle à son numéro central.

Le joueur valide explicitement la correction et son intégration le 18 octobre
2026 : « ok on peut donc valider et ajouter au jeu tout ca :) ».
Le rendu accepté et les commandes de WALK-VIS-01 sont activés dans la partie
normale. L'essai isolé utilise désormais les mêmes commandes et le même rendu.

Galerie réutilisée : `artifacts/walk-preview-review-01/index.html`.
Extrait natif : `destination-e-detail.png`, 640 × 400 px, avec un extrait de
scène de 480 × 288 px sans redimensionnement. La capture complète reste
`destination-e.png`, 1280 × 800 px. Les cases mesurent 64 px.

Le mode `--ui-cold-walk-preview <dossier>` contrôle le code intégré en 1280 × 800
et demande aussi 2560 × 1440 et 3840 × 2160. Windows limite ces deux fenêtres
à 1924 × 1061 sur le bureau utilisé : les dimensions réelles sont consignées,
sans présenter ces captures comme du 2K/4K natif. Les coordonnées aux deux
résolutions et aux échelles 75/100/150 % sont vérifiées par les tests. L'ancien nom `--ui-cold-walk-destinations` reste un
alias de ce contrôle final. Les six anciens dessins restent documentés dans
la revue 02 ; leur prototype est archivé sous
`tools/archive/handcrafted-places-review-01/walk_preview_review.rs`.
Le contrôle antérieur à l'intégration reste conservé sans modification dans
`navigation-destination-e-03-native.json`.

Le bilan après intégration est dans `navigation-destination-e-03-integration-native.json`.
Le remplissage clignotant demandé ensuite est une revue séparée MARKER-PULSE-01,
qui ne retarde pas les déplacements et la dalle E déjà validés.
