# AIM-VIS-01 — Lisibilité et esthétique des zones de visée

Propositions du 18 octobre 2026, **en attente de validation visuelle explicite**.

Demande : mieux distinguer les zones valides et invalides, les rendre plus
visibles, avec un beau rendu adapté au jeu. Aucune modification des dégâts,
portées, ressources, commandes ou sauvegardes n'est proposée ici.

## Candidats

- **AIM-VIS-01-A — Hologramme** : cyan pâle, halo fin, trame légère.
- **AIM-VIS-01-B — Sceau occulte** : doré désaturé, angles biseautés,
  petits losanges sur le contour.
- **AIM-VIS-01-C — Gravure** : violet pâle, contour segmenté, petits repères.

Points communs : un contour continu autour de l'empreinte, une sous-couche
sombre pour le contraste sur les sols clairs, un remplissage transparent,
un viseur de coin sur la case ciblée, une zone invalide rouge franc et hachurée.
La demande complémentaire « le rouge pourrait être plus rouge » est appliquée
aux prototypes : RGB 255, 48, 38, en remplacement du rouge rosé initial.
Le viseur est dessiné après les personnages et n'en barre pas le centre.
La surface est dessinée avant les personnages.

## Présentation native

Galerie courante : `artifacts/aim-preview-review-01/index.html`.
Planche : `artifacts/aim-preview-review-01/comparaison.png`.
Captures complètes : `{actuel,a,b,c}-{valide,invalide}.png` dans ce dossier.
Les extraits de la planche sont composés par le moteur, à un pixel pour un
pixel source, avec des cases de 64 px. Le rendu actuel est conservé comme
référence dans la galerie.

Les captures utilisent le laboratoire existant de Déflagration occulte A02,
dans une partie de diagnostic isolée, sans écriture d'une partie utilisateur.
La validité est calculée par le moteur. L'état invalide résulte d'un manque
réel d'énergie après utilisation de la compétence dans cette partie isolée.

Le moteur ne renvoie actuellement plus d'empreinte A02 lorsqu'il manque de
l'énergie. Les trois prototypes réutilisent donc l'empreinte précédemment
résolue à la même position pour la maintenir visible en rouge. Ce maintien
est une **proposition visuelle à valider** ; ce n'est pas une nouvelle règle
de portée ou d'autorisation d'attaque. Le rendu actuel de référence continue
à laisser disparaître cette surface. Une intégration future devra conserver
la géométrie de façon générale pour les différents motifs de rejet.

## Isolation

`src/aim_preview_review.rs` et `src/aim_preview_review_app.rs` sont limités aux
compilations de diagnostic. Le mode `--ui-cold-aim-styles <dossier>` active
les propositions temporairement. Les gardes rétablissent le rendu courant
après chaque capture ; aucun style n'est activé dans une partie normale.
Les cellules non visibles restent exclues par le moteur de rendu existant.
Le jeu n'utilise aucune ressource de la galerie à l'exécution.

Les contrôles natifs de dimensions, de validité et de conservation de l'état
durant le dessin sont consignés dans `aim-visuals-01-native.json`.
La compilation ne constitue pas une validation artistique.

Suite à la validation de AIM-VIS-02, le module de dessin est devenu
`src/attack_preview_visuals.rs`. Le style combiné est désormais celui du jeu.
Les états de cette première revue ci-dessus décrivent la proposition initiale.
