# Modules — intégration du 3 octobre 2026

Le rendu reprend la proposition Modules validée : noyau clair/cyan pour le joueur, cadre vert pour les NPC neutres ou de service, triangle rouge pour les NPC hostiles. Les dessins restent en vue de dessus et occupent une seule case. Leur palette est indépendante du thème de l'interface.

Le joueur conserve la sélection visuelle selon l'arme active : neutre, pince de mêlée ou émetteur à distance. Ses déplacements horizontaux retournent le module ; les déplacements verticaux conservent le dernier côté. Pour les NPC, les événements de déplacement, d'annonce et d'exécution d'attaque orientent le dessin. Le dernier emplacement d'attaque exécuté détermine la pince ou l'émetteur ; en l'absence d'attaque, le module reste neutre.

L'atlas de 180 × 20 pixels regroupe les neuf combinaisons de rôle et de pose. Il est créé et chargé une fois au démarrage, puis réutilisé avec un filtre au plus proche et un retournement horizontal. Le PNG humanoïde rejeté n'est plus compilé ni décodé. Les anciens dessins d'acteurs ont été retirés de l'atlas des décors ; les portes et les accessoires conservent leurs pixels et leur rotation.

Les apparences des NPC visibles sont calculées une seule fois par image. Le rendu ne recherche pas à nouveau chaque acteur par position. Les acteurs cachés, les capteurs qui remplacent leur symbole, les objets destructibles et les balises ne reçoivent pas un dessin de NPC.

L'orientation des NPC et le dernier emplacement d'attaque sont un cache de présentation en mémoire, nettoyé à la disparition de l'acteur. Au chargement, il repart vers la droite avec le premier emplacement d'attaque. Aucune structure de sauvegarde, règle, commande ou génération n'a été modifiée. Les anciennes générations sans relation explicite conservent l'identification hostile par leur symbole.

## Vérification

- `cargo check --locked --all-targets` et `cargo build --locked` : réussis. L'avertissement préexistant sur `wall_joins` reste présent.
- `cargo fmt --all -- --check` et `git diff --check` sur les sources concernées : réussis.
- `cargo test --locked --bin project-rl terminal_view::` : 34 tests réussis, dont les portes, la perception, les marqueurs et les six zooms.
- `cargo test --locked --bin project-rl player_art` : 2 tests réussis, couvrant les armes actives, le clavier et les commandes de marche à la souris.
- `cargo test --locked --bin project-rl npc_art` : 2 tests réussis. Le premier utilise un déplacement réel de l'IA ; le second vérifie l'attaque exécutée, l'annonce d'attaque, les acteurs cachés et le nettoyage du cache. Les mises à jour graphiques seules conservent les octets du moteur et l'historique.
- `borrowed_recovery_snapshot_preserves_legacy_bytes_and_fingerprints` et `same_build_snapshot_restores_without_executing_the_journal` : 2 tests réussis.
- Total : **40 tests ciblés distincts réussis**. La suite complète n'a pas été relancée ; les échecs antérieurs restent consignés dans `../neon-integration-2026-10-03/VALIDATION.md`.

## Captures

- `gallery/cold-start.png` : les trois rôles, trois poses et deux orientations, agrandissement ×4 puis tailles réelles 24 et 20 pixels. Cette scène utilise le rendu du jeu.
- `game-melee-right-20/cold-start.png` : début de partie, joueur en mêlée vers la droite, thème violet, 20 pixels.
- `game-ranged-left-24/cold-start.png` : début de partie, joueur à distance vers la gauche, thème bleu, 24 pixels.
- `npc-left-20/cold-start.png` et `npc-right-24/cold-start.png` : salle de diagnostic déterministe, NPC neutre, hostile en mêlée et hostile à distance, dans les deux sens.
- Les quatre scènes en jeu passent leurs diagnostics de présence de texte. Ces contrôles ne constituent pas une certification générale du contraste.

`before/` contient les quatre sources à l'ouverture de cette intervention, `before-game/` la capture de l'exécutable précédent. `integration.patch` ne contient que cette intervention comparée à ces copies. Les autres modifications de l'espace de travail ont été préservées. Aucun commit ni push.
