# Nettoyage avant les premières cartes

18 octobre 2026. Autorisation explicite du joueur : nettoyage des fichiers
devenus inutiles pour libérer de la place, avant stabilité et revue des cartes.

1 309 fichiers générés supprimés : **1 324 482 134 octets**, soit **1,23 Gio**.
Le dossier de compilation séparé `target/visual-review`, les anciens exécutables
d’exemples, les exécutables de comparaison et les grandes captures de netteté
reproductibles ont été retirés. Les aperçus actuels sont conservés.

Le script existant `tools/maintenance/Remove-GeneratedProjectFiles.ps1` vérifie
chaque chemin absolu, son emplacement, ses parents, sa taille et sa date avant
de supprimer les fichiers ordinaires un par un. Aucune suppression récursive
n’est employée. Les sept jonctions du checkout de comparaison sont retirées
sans parcourir leurs cibles. Les dossiers ne sont retirés que lorsqu’ils sont vides.

Le manifeste est
`docs/maintenance/nettoyage-2026-10-16/pre-maps-2026-10-18-plan.json` ; le bilan
brut est le fichier voisin `pre-maps-2026-10-18-result.json`.
Les empreintes avant/après de quinze fichiers protégés, dont les données de
l’éditeur et les exécutables courants, sont inchangées. Voir `completion.json`.
Les sources, les ressources, les validations et les sauvegardes ne font pas
partie des cibles.

Cette opération a réussi et remplace le statut de nettoyage précédemment
bloqué de la revue des ensembles d’armes. La Corbeille n’a pas été vidée.
