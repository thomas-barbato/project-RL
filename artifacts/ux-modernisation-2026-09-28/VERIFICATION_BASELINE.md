# Vérification des deux échecs de combat préexistants

Une copie de comparaison a été constituée dans `baseline-project/`, avec les fichiers d'interface sauvegardés avant les modifications. Les catalogues et ressources nécessaires sont partagés avec le projet local.

Commande exécutée depuis la racine du projet :

```powershell
cargo test --locked --manifest-path artifacts/ux-modernisation-2026-09-28/baseline-project/Cargo.toml --target-dir target --bin project-rl combat_balance_ -- --quiet
```

Résultat : **3 réussites et 2 échecs**. Les deux échecs sont identiques à ceux du projet après la refonte :

- `combat_balance_surface_roles_remain_distinct_with_starting_white_weapons`, assertion dans `src/combat_balance_app.rs:154` ;
- `combat_balance_white_equipment_matrix_matches_real_impacts`, assertion dans `src/combat_balance_app.rs:421`.

Dans les deux cas, le test attend `Applied`, mais reçoit `Rejected(InsufficientMatter { required: 1, available: 0 })`. Les règles de munitions n'ont pas été modifiées pour faire passer ces tests dans le cadre de la refonte UX.

Le premier passage complet a validé 765 tests de bibliothèque et 420 tests du client ; il a également identifié deux problèmes de tests liés à l'UX qui ont ensuite été corrigés. Un second passage étendu a validé 419 tests du client et identifié un dernier test de clinique utilisant encore les anciennes coordonnées de clic. Ce test utilise maintenant la disposition commune et les 6 tests de clinique réussissent.

Les 56 tests ciblés finaux de la refonte réussissent. Le second passage étendu excluait les deux échecs de combat connus et trois tests longs de génération déjà réussis lors du passage complet. Les journaux de ces passages sont conservés à côté du compte rendu.

