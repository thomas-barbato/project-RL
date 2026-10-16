# Validation locale — 5 octobre 2026

## Résultat

- `cargo test --locked --lib` : **817 tests réussis, aucun échec**, après les dernières modifications moteur (palette de drone comprise). Voir `tests-engine-final.log`.
- Rendu `game_art` : **12 tests réussis**. Les 136 figures sont présentes et non coupées en 48/128 pixels ; les 17 corps de chacune des huit poses mesurent 30 pixels et gardent leur ligne de bottes. Alpha, contours et couleurs indépendantes sont contrôlés. Voir `tests-art-final.log`.
- Mains et client : **5 tests réussis**, sélection de configuration sans tour ni déséquipement, exclusion active des deux configurations, gate génération 145 et reprise avec palettes. Voir `tests-hands-final.log`.
- Inventaire et tooltip : **4 tests réussis**, mêmes instances survolées dans le sac et équipées, inspection sans mutation, dimensions compactes et grands textes. Voir `tests-inventory-final.log`.
- `cargo check --locked --all-targets`, compilation debug et **release** : réussis. Les avertissements de fonctions/références inutilisées ne bloquent pas la compilation. Aucun `cargo fix` global effectué.
- `rustfmt --check` des 22 fichiers Rust concernés : réussi ; `git diff --check` : réussi. Le contrôle global de formatage conserve les écarts préexistants de `armor_models_tests.rs`.
- Reproductibilité des figures : `build_player_outfits.py --check` réussi. Reproductibilité des familles de tir : `build_ranged_families.py --check` réussi. Les nouvelles images de mains sont embarquées dans les PNG vérifiés.

## Conservation des couleurs et du gameplay

Sept tests spécifiques couvrent les 100 créations d'un même modèle (les sept palettes observées), les 180 morts de monstres comparées à une partie sans couleurs (mêmes modèles, bonus et RNG de combat), le ramassage/équipement/dépôt/amélioration, les quatre voyages et reprises, les refus atomiques, le JSON historique et les instantanés binaires. Le test de commerce vérifie vente, reprise et rachat avec la même palette violette. Le test de drone vérifie la palette rose et la palette d'origine, même après une reprise au milieu du transport, sans nouveau tirage à la livraison.

Les parties antérieures à 145 n'activent pas les tirages cosmétiques. Les représentations Debug omettent les nouveaux champs absents pour conserver les empreintes historiques. `Some(Original)` reste une attribution terminée et ne peut pas être tiré à nouveau. Le RNG cosmétique est enregistré dans le run et ne remplace pas le RNG de simulation.

## Captures et performances

Les captures suivantes proviennent du client exécuté, dans des scènes isolées et avec un chemin de sauvegarde de diagnostic distinct :

- `--ui-cold-equipment-palettes-review` : `capture-palettes-final/cold-start.png`, huit images de contrôle et planche d'armes/tenues aux couleurs indépendantes.
- `--ui-cold-terminal-inventory-live-equipped` : `capture-inventory-final/cold-start.png`, inventaire actif et tooltip de l'armure équipée, couleurs individuelles dans le sac et les slots.
- `--ui-cold-terminal-hud-live` : `capture-world-final/cold-start.png`, carte active, arme au sol et illustration de cible de même couleur.
- `mains-pistolet-final.png` : disposition QA des banques embarquées, figures complètes, mains agrandies sans lissage et corps natifs. Imagegen intégré a repris les mains ; le script QA effectue uniquement crop/scale/layout.

Le premier diagnostic a révélé une recoloration du torse à travers la zone du fusil. Les zones de matériaux ont été resserrées ; la capture finale conserve la couleur du torse au-dessus du fusil et celle du mécanisme de l'arme. Un autre diagnostic a révélé un chargement différé des banques d'objets à l'ouverture de l'inventaire, corrigé par leur initialisation au démarrage, comme le faisait le rendu précédent.

`--ui-cold-performance` mesure 120 frames dans chacune des dix scènes et vérifie que le rendu ne change pas l'empreinte de simulation. Diagnostic final dans `performance-warmed-final/timings.json` :

| Scène | Médiane CPU avant | Médiane CPU finale | Première frame avant | Première frame finale |
|---|---:|---:|---:|---:|
| Carte | 8,04 ms | 8,11 ms | 345,60 ms | 354,19 ms |
| Inventaire | 10,86 ms | 10,94 ms | 25,94 ms | 34,03 ms |

La première ouverture intermédiaire à 582,23 ms (`performance-final`) est corrigée et n'est pas le résultat final. La comparaison de référence vient de `artifacts/joueur-quatre-prises-2026-10-05/performance/timings.json`. Ces chiffres sont des diagnostics debug sur cette machine, pas une mesure release ni une garantie matérielle générale.

## Suite client complète et limites

La suite client complète exécutée pendant cette intégration comptait 640 tests : **598 réussis, 27 échecs, 15 ignorés**. Les 27 noms en échec sont exactement ceux du témoin antérieur isolé (`tests-baseline-failures.log`) : aucun nouveau nom d'échec dans `comparaison-tests-client.json`. Les échecs concernent des empreintes/reprises et parcours déjà présents avant cette étape. Le témoin avait été produit dans un checkout de comparaison ; aucun de ces échecs n'a été masqué ou déclaré corrigé.

Les derniers ajustements de main, de matériaux, de transport et d'initialisation ont ensuite été validés par la suite moteur complète, les 21 tests client ciblés et les diagnostics ci-dessus. La suite client complète n'a pas été relancée après ces ajustements ; elle ne doit donc pas être présentée comme entièrement verte. Les pistolets/pistolets mitrailleurs restent absents du catalogue de butin. Le transport de drone conserve désormais la palette ; son fonctionnement antérieur des autres métadonnées n'est pas redéfini dans cette étape.

`preservation-final.json` compare les 297 fichiers de source et banques présents au démarrage de l'étape. Les changements recensés sont les transferts, gates et rendu concernés, les huit banques de poses de tir, leur métadonnée et le README. Les poses sans arme/mêlée, images de PNJ, terrain, contenu et textes préexistants sont conservés. Aucun commit, push ou déploiement.
