# Raccords des coins avec les murs de référence

Revue du 1 octobre 2026, après signalement d'un décalage entre coins et murs et d'un trait blanc dans les angles.

## Cause et correction

Le profil vertical des coins source contient un liseré clair différent du profil horizontal. L'ancienne préparation remplaçait seulement les bandes de connexion de 8 pixels par celles des murs droits. Le reste du coin gardait son ancien profil ; cela accentuait le changement de largeur et de couleur aux raccords. Copier toute la section du mur introduisait également ses détails de panneau dans ces bandes.

La bordure est maintenant alignée sur toute la longueur du coin. La préparation place temporairement chaque pièce dans le sens haut gauche, applique le profil du mur supérieur aux deux bras et à leur coude, puis remet la pièce dans son orientation. Le panneau central de chaque image et les parties de ses bras hors bordure sont conservés. Le mur supérieur de référence garde ses pixels et les autres murs gardent leurs bordures communes de la revue précédente.

Les quatre choix de coins restent distincts. Les sources PNG et les données des cartes restent inchangées.

## Revue

- [Gros plan du rendu natif à zoom ×2](captures-validees/wall-borders-closeup.png).
- [Deux bâtiments à 64 pixels par case](captures-validees/four-corners-manual.png).
- [Contours automatiques](captures-validees/wall-corners.png).
- [Quatre orientations](captures-validees/wall-rotations.png).
- [Diagnostic des sprites](diagnostic.png) : première rangée, coins source préparés avant correction de leurs bordures ; deuxième rangée, coins corrigés ; troisième rangée, quatre murs orientés. Grossissement ×4 sans interpolation.

Le premier dossier `captures` contient une session interrompue avec des images de 1×1 pixel et ne sert pas à valider le rendu. Les neuf images du dossier `captures-validees` ont bien été enregistrées en 1360 × 840 ; les vues de murs et de coins ont été inspectées. Cette seconde session a ensuite perdu sa surface de rendu lors de la capture suivante. Le contrôle ajouté a arrêté la session au lieu d'enregistrer une image de 1×1 pixel. Les captures ultérieures des dialogues et du test jouable n'ont pas été refaites dans cette revue ciblée des murs.

## Validation et version locale

- `cargo test --locked --example map_editor --example textured_surface_preview` : 25 tests de l'éditeur et 11 tests de l'essai de surface passent.
- Le nouveau test de continuité de profil échouait avant correction. Il compare les pixels RGBA des deux bras de chaque coin avec les murs et vérifie que leur panneau central reste inchangé.
- Les tests existants contrôlent toujours l'absence de trous sur les 16 configurations de voisins, les quatre orientations, la sauvegarde et l'historique.
- Construction des deux exemples : succès. Revue des captures natives des murs et des coins : réalisée ; la session complète de capture s'est arrêtée après les neuf vues utilisables, comme détaillé ci-dessus.
- L'éditeur habituel `target/debug/examples/map_editor.exe` a été recompilé après fermeture de sa fenêtre. Une copie est conservée dans [map_editor.exe](map_editor.exe).

Ce résultat constitue un nouvel essai soumis à l'avis de l'utilisateur. Le client de campagne ne charge pas ces images.
