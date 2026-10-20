# Jeu normal et retrait des lanceurs d'essai

20 octobre 2026. Demande du joueur : « met tout ca dans le vrai jeu :) et
supprime les fichiers .cmd dont ont a plus besoin :) ».

Le camp, la rue peuplée, le passage réversible des égouts, les cinq tenues
cosmétiques et les flammes acceptées utilisent le parcours de partie normale,
à partir de la création du personnage. Le contrôle supplémentaire
`normal_character_creation_enables_validated_outfits_city_and_sewer_without_trial_mode`
utilise ce parcours pour les cinq classes, sans activer les modes d'essai.

`Jouer.cmd` compile et ouvre ce jeu normal ; `Editeur_de_cartes.cmd` conserve
son usage de production. Les douze autres lanceurs sont supprimés : deux
essais à la racine, deux anciens essais du camp/dialogue et huit laboratoires
archivés. `removed-launchers.json` garde leurs chemins, tailles, empreintes
et contenus pour la traçabilité. Ce retrait libère 3 084 octets.

Les sources des dessins, les ressources utilisées par le jeu, les décisions
validées, les sauvegardes et `artifacts/map-editor/` sont conservés. Les scripts
de revue et les pages courantes orientent désormais vers `Jouer.cmd`.
Les registres historiques conservent leurs anciens chemins avec ce bilan comme
explication du retrait ; ils ne désignent plus des lanceurs disponibles.

La révision des dégâts/PV reste au stade de diagnostic OFFENSE-START-02 :
aucun nouveau coefficient chiffré n'a été présenté ou appliqué. Les changements
déjà validés sont les tenues à zéro, les flammes détaillées et les peuplements.
Les campagnes antérieures conservent leur génération et leurs valeurs.
