# Jauges et descriptions — vérification du 28 septembre 2026

Les jauges PV, énergie, munitions, bande passante et chaleur sont affichées dans le HUD compact et dans la colonne latérale. Elles partagent les valeurs du moteur. Les munitions restent visibles lorsqu'une arme de mêlée est active ; les alertes ne remplacent plus les ressources. L'expérience conserve sa jauge dans la colonne latérale.

F1 propose désormais Commandes essentielles, Symboles et Statistiques et jauges. Ce dernier onglet contient 42 descriptions regroupées en six sections. Les raccourcis continuent d'utiliser les affectations réelles. Le document `RELECTURE_STATISTIQUES.md` est exporté directement depuis ces descriptions, pour éviter un écart entre le document soumis à relecture et l'interface.

La chaleur personnelle et la chaleur propre à un système électronique ciblé sont distinctes dans le moteur. La première signale les franchissements de seuil ; les conséquences supplémentaires dépendent des modules ou effets. Le texte ne promet donc pas de dégâts automatiques à chaque dépassement de la jauge personnelle. Les effets du niveau et la récupération d'énergie sont décrits à partir des règles de la partie en cours.

Validation :

- `cargo build --locked` et formatage réussis, sans avertissement de compilation introduit.
- 13 tests UX distincts réussis, dont les dépenses réelles, la production de chaleur, la réservation de bande passante, la navigation F1 et les règles d'une ancienne sauvegarde.
- 2 tests de ressources d'armes, 14 tests de vue et 18 tests de commandes réussis.
- `git diff --check` réussi.
- Captures inspectées en 960 × 540, 1280 × 800 avec interface agrandie et grand format. Vérification des jauges vides, d'une batterie partiellement dépensée, d'un drone actif, des alertes et du défilement du glossaire jusqu'à la fin. Les contrôles automatiques de texte ont réussi ; ils ne mesurent pas un contraste normalisé.

Les captures de `lecture-finale/` remplacent les versions correspondantes de `final/`. Les copies antérieures aux modifications de cette demande sont conservées dans `baseline/`. Les modifications concernent la présentation, l'aide et leurs tests, sans changer les règles de dépense ou de combat.
