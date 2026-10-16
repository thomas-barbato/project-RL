# Lisibilité : correction et comparatifs avant déclinaison

Le 5 octobre 2026, l'utilisateur a accepté l'ordre proposé dans l'audit :
correction des recolorations, essais limités (deux tenues, trois armes),
comparaison native sur plusieurs sols puis validation avant déclinaison.
Pendant le travail, il a signalé une tache grise au milieu du nouveau pistolet.
Cette tache a été retirée dans `pistolet-source.png` ; l'essai initial est
conservé dans `pistolet-v1-source.png`.

## Intégré au jeu

Seul `src/game_art_palette.rs` a changé depuis la référence de cette étape.
La protection des visières est limitée à leurs fenêtres réelles, plutôt
qu'à toutes les couleurs orange du haut du personnage. Le visage exposé
reste couleur chair ; le masque rouge fermé suit la palette de la tenue.
Des limites locales empêchent aussi le bord du casque et les manches ocre
du technicien d'être classés comme peinture de l'arme.

Les accents lumineux des visières, la rouille des armes et leurs couleurs
indépendantes sont conservés. Aucun déplacement de pixel, remplacement
de figure, modification de collision, d'objet, de nom, de statistique,
de génération ou de sauvegarde.

`correction-casque.png` compare l'ancien rendu bleu et la fonction CPU
actuelle. `controle-recoloration.json` vérifie 1904 variantes (17 tenues,
8 poses, 7 palettes, deux tailles), leurs comptes de pixels visibles,
les pixels natifs de la palette d'origine et les 16 casques du technicien
en bleu (8 poses à 48/128px). Aucune peinture orange conservée au-dessus
du visage dans ces 16 contrôles.

## Propositions à valider

- Technicien et Bastion : sans arme, épée à une main, pistolet, fusil.
- Fusil d'assaut, pistolet Cogneur, épée industrielle au sol.
- Corps mesurés à 30px, ligne des pieds à 46 dans une toile transparente
  de 48px ; les armes peuvent dépasser le corps.
- Volumes plus groupés, bordures et reflets plus larges, moins de grain.

Les sources ont été dessinées avec l'outil intégré `image_gen`. Les prompts
exacts sont dans `PROMPTS.md`. `build_previews.py` utilise le cadrage du
packer existant puis fait uniquement recadrage, nettoyage alpha, réduction
nearest et disposition. Il ne peint ni ne quantifie les couleurs.

Les PNG conservent encore de nombreuses variations RGB : ces essais ne
constituent pas une palette strictement indexée à huit ou douze teintes.
Le gain visuel repose surtout sur les masses et les contrastes. Les mesures
brutes sont disponibles dans `mesures-propositions.json`, sans en faire
une note automatique de lisibilité. L'épée est surtout une proposition
de silhouette ; son icône d'origine était déjà relativement simple.

## À regarder

- `comparatif-tenues.png` : personnages à taille native et zoom x2.
- `comparatif-inspection.png` : vues d'inspection à 128px.
- `comparatif-armes.png` : armes 32px, zoom x4 et illustration 128px.
  Pour l'épée, l'ancienne grande vue est un agrandissement de l'icône
  commune de sol, et non une illustration d'objet particulier.
- `capture-outfits-sols.png` : huit paires de figures natives sur quatre sols.
- `capture-weapons-sols.png` : trois armes natives et zooms x2 sur quatre sols.

Les deux dernières captures proviennent de `preview.rs`, un exécutable
Macroquad séparé qui emploie les fonctions actives de sols et de sang.
Il ne charge pas de partie et ne remplace pas les banques du jeu. Les zooms
d'armes agrandissent l'arme uniquement, pas le sol. Les quatre fonds sont
béton, métal, grille sombre et béton avec sang.

Les cinq nouvelles sources restent dans ce dossier. Aucun PNG actif n'a
été remplacé : la déclinaison aux autres modèles et aux huit poses attend
la validation des comparatifs, selon l'ordre accepté par l'utilisateur.

## Validation

- 16 tests graphiques réussis, dont trois nouveaux tests de régression :
  `tests-art.log`.
- `cargo check --locked --all-targets`, compilation debug et release réussis.
  La release conserve les avertissements existants : `build-release.log`.
- `rustfmt --check` sur le fichier modifié et `git diff --check` réussis.
  Le contrôle de format de tout le dépôt signale deux mises en forme dans
  `src/armor_models_tests.rs` (lignes 47 et 127), fichier préexistant inchangé.
- Packers des tenues et des armes de tir reproductibles : `packer-check.log`.
- Contrôle de 650 fichiers actifs : seul le renderer de palettes a changé,
  aucune image ou donnée de contenu modifiée : `preservation.json`.
- La tache du pistolet : 4266 pixels gris dans la zone signalée avant,
  zéro après : `controle-tache-pistolet.json`.
- Diagnostic de performance debug terminé, dix scènes de 120 frames.
  Carte : médiane 8,21ms contre 8,39ms à l'étape précédente ; inventaire :
  11,06ms contre 11,61ms. Première frame respectivement 351,77/32,63ms
  contre 362,15/34,87ms. Pas de régression observée dans ce relevé local ;
  cette fluctuation n'est pas présentée comme une optimisation.
  `comparaison-performance.json` et `performance-correction/timings.json`.

La suite client complète n'a pas été relancée ici ; ses échecs documentés
à l'étape précédente ne sont pas déclarés résolus par ce travail graphique.
Aucun commit, push ou déploiement.
