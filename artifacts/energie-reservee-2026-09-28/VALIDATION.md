# Énergie réservée et compagnons — validation locale

## Résultat

Les nouvelles parties utilisent la génération 126 : 5 places de compagnons, pas de bande passante de joueur, pas de jauge de chaleur générale. La barre d'énergie sépare disponible, récupérable et réservé. Les réservations reprennent les coûts existants : 10 E pour Camouflage actif, 6 E pour Brouillage, sans entretien supplémentaire. Les anciennes générations conservent leurs règles.

Le panneau de gestion s'ouvre par clic sur Énergie ou Compagnons, ou par la touche État/cible puis la confirmation configurée. Consulter est gratuit ; arrêter un effet ou renvoyer un compagnon fait passer un tour.

## Vérifications

- `cargo check --locked --all-targets`, `cargo build --locked`, `cargo fmt --all -- --check` et `git diff --check` : réussis.
- Suite complète : 775 tests moteur réussis. Les trois échecs de scénarios touchés côté client ont été corrigés et relancés avec succès dans les vérifications ciblées ci-dessous.
- 9 tests ciblés de mécanique réussis : cumul, énergie libre nulle, refus sans coût/temps, plafond de régénération, arrêt, expiration, diminution de capacité, contrôle de plusieurs dispositifs, compagnons de quête exclus, sixième invocation refusée, compagnons hors zone, instantané natif.
- 15 tests UX réussis : panneau gratuit et modal, arrêt rejouable, textes sans coûts de bande passante, commandes reconfigurées, générations antérieures, interfaces existantes.
- 2 tests munitions/énergie réussis, puis contrôle complémentaire des descriptions actuelles réussi.
- Captures HUD vérifiées à 960 × 540 et 1920 × 1061 ; panneau vérifié en petite fenêtre et à 150 % ; onglet F1 vérifié en petite fenêtre. Pas de texte débordant des boutons ni de superposition avec les nouvelles barres.

## Deux échecs préexistants hors périmètre

Les tests `combat_balance_white_equipment_matrix_matches_real_impacts` et `combat_balance_surface_roles_remain_distinct_with_starting_white_weapons` échouent avec `InsufficientMatter { required: 1, available: 0 }` dans leurs scénarios d'équilibrage sans munitions. Leurs fichiers n'ont pas été modifiés. Le journal complet est conservé dans `tests.log` ; les reprises ciblées sont dans `maintained-tests.log`, `ux-tests.log`, `supplies-tests.log` et `copy-tests.log`.

## Portée

La limite est configurable pour de futurs bonus ; aucun bonus de dépassement n'est ajouté. Les origines recruté, acheté, invoqué et PNJ de quête sont distinguées dans le moteur ; les mécanismes de recrutement et d'achat ne sont pas inventés ici. Les compagnons hors zone restent comptés mais il faut les rejoindre pour les renvoyer. Les durées déjà définies des techniques ne sont pas rééquilibrées.

Les liens, ordres et dispositifs maintenus locaux s'arrêtent au changement de zone ; le camouflage peut suivre le joueur. Le surcadencement et la surchauffe des machines ennemies gardent leurs règles spécifiques. Les descriptions complètes de F1 sont exportées par les captures dans `RELECTURE_STATISTIQUES.md` pour relecture.

## Correction F1 après relecture

La rubrique de chaleur a été supprimée du F1 des nouvelles parties, y compris la référence à une jauge dans la résistance thermique. Les cinq attributs se trouvent immédiatement après les ressources. Les captures `captures/corrected-f1` montrent séparément ressources, attributs et défenses. Les 15 tests UX ont été relancés avec succès après cette correction. `RELECTURE_STATISTIQUES.md` rassemble le texte actuel et les compléments secondaires encore proposés, sans les intégrer avant relecture.
