# Essai d’expédition dans le jeu

Cet essai remet l’exploration, les combats et la progression au centre du travail. Le mode texturé et l’enrichissement de l’éditeur sont en pause. La proposition artistique sombre en vue verticale reste une référence pour une éventuelle reprise.

## Lancer l’essai

Ouvrir `Essayer_expedition.cmd` à la racine du projet, puis choisir **Nouvel essai d’expédition**. Choisir son protocole et ses attributs comme dans une nouvelle partie habituelle. Le départ inclut le prologue de recyclage existant avant l’arrivée en ville.

Le lanceur utilise la version compilée dans `target/release/project-rl.exe`. Pour reconstruire cette version après une modification du code : `cargo build --locked --release --bin project-rl`.

## Parcours proposé

Rejoindre la ville, sortir dans les friches, trouver le passage du secteur industriel au sud-est, explorer le souterrain puis rentrer en ville avec ce qui a été trouvé. Le panneau Objectif et le repère de direction accompagnent ce parcours. Une quête suivie dans le journal conserve la priorité.

L’essai réutilise les grandes cartes de la campagne actuelle ; il ne crée pas une nouvelle carte compacte. Les passages, populations, butins, compétences et règles de combat sont ceux du jeu. Le parcours reste une suggestion : l’exploration et la progression peuvent continuer après le retour.

Les commandes affichées en bas de l’écran et l’aide F1 restent celles du jeu. Quitter par le menu ou fermer normalement la fenêtre suspend la partie ; **Reprendre la partie** permet de continuer.

## Sauvegarde séparée

Sous Windows, cet essai utilise `%APPDATA%/ProjectRL/essai-expedition.json`, avec son propre verrou et ses deux fichiers de récupération. Le lancement ordinaire conserve `city-test-run.json`. Le format de sauvegarde et la génération restent inchangés. Les préférences de commandes et d’affichage sont partagées avec le jeu habituel.

## Ce que l’essai doit éclaircir

Le point principal est de voir si une sortie donne envie d’explorer, offre des choix intéressants en combat et donne une raison de récupérer du butin puis de rentrer. La distance jusqu’au souterrain, la lisibilité du trajet et le rythme des rencontres restent à juger en jouant. Un aller-retour automatisé réussi ne suffit pas à valider leur équilibrage.
