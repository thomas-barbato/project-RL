# IC32 · intégration et correction

L'accord initial « oui je valide » a été précisé par le joueur : toutes les icônes sauf Déflagration occulte. `record_integration.py` conserve la première étape ; `record_correction.py` applique l'exception. Les deux reçus restent dans IC32.

Banque active : 61 compactes et 16 illustrations. Les cinq dessins `A01-C01`, `HY05-C01`, `OC04-C01`, `OC05-C01` et `OC07-C01` sont conservés exactement. `A02-C01` est retirée et archivée dans `withdrawn-art/`, sans modifier les sources de la revue. La proposition suivante `60 A02-C02` attend son accord dans IC33.

Les 641 empreintes protégées lors de la correction sont conservées. Catalogue, paramètres, textes et illustrations inchangés. Vérification dans `integration-validation.json`.

Validation : les trois tests ciblés `agent_skill_window::tests` réussissent, compilation debug réussie, formatage et `git diff --check` réussis. Journaux `tests-after-correction.log` et `build-after-correction.log`.

Les captures initiales sont conservées comme historique. `native-icon-matches.json` vérifie uniquement les cinq dessins restant acceptés contre leurs exports 64 px ; erreur moyenne maximale 4,941/255, tolérance 15. `after-correction/` conserve la capture de la fenêtre avec le champ A02 revenu à son emplacement neutre, après reconstruction de l'exécutable.
