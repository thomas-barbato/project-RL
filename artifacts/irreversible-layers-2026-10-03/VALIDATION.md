# Passages irréversibles entre couches — 3 octobre 2026

## Résultat

- Les nouvelles parties (génération 142) peuvent descendre et circuler librement dans la couche actuelle, mais ne peuvent pas remonter dans une couche quittée.
- Le refus intervient avant toute génération de destination, consommation de tour ou mutation de l'état. Les itinéraires connus ne traversent plus les accès interdits.
- L'interface annonce « SANS RETOUR » avant de descendre et « ACCÈS FERMÉ » sur les anciens accès.
- L'enquête d'Elias se termine au relais par témoignage de Milo ou lecture du registre, avec 30 crédits et 12 XP attribués une seule fois. Aucun tour de rapport supplémentaire, message à Elias ou mise à jour de son panneau n'est ajouté.
- Les générations 141 et antérieures gardent leurs passages réversibles et leur enquête avec rapport à Elias. Le format binaire RLWS v11 ne change pas ; la règle est reconstruite depuis la génération sauvegardée, comme les autres règles versionnées.
- L'introduction au jeu est identifiée dans le document de conception et reportée à la demande de l'utilisateur.

## Vérification

- `cargo check --locked --all-targets` : réussi.
- `cargo build --locked` : réussi. L'avertissement préexistant sur `TerminalView::wall_joins` demeure.
- `cargo fmt --all -- --check` et `git diff --check` sur les fichiers modifiés : réussis.
- Bibliothèque entière : **793 tests réussis**.
- Narration du client : **13 tests réussis**, dont les deux chemins de résolution sur la vraie carte générée, la récompense unique après reprise, la reprise par instantané et par relecture du journal, et le parcours historique avec retour et rapport.
- Vue terminale : **17 tests réussis**.
- Interactions contextuelles : **48 tests réussis**.
- Plan de la première couche : **6 tests réussis**, dont les trajets et reprises historiques.
- Total : **877 tests distincts réussis**. La suite complète du client n'a pas été relancée ; ses échecs déjà présents lors de la refonte visuelle sont consignés dans `../neon-integration-2026-10-03/VALIDATION.md`.

## Captures natives

- `descent-current/cold-start.png` : passage de surface vers le secteur industriel, avec l'annonce « SANS RETOUR » dans la cible, l'infobulle et la description.
- `return-closed-final/cold-start.png` : ancien accès de retour après descente, avec indication de fermeture et sans raccourci de voyage disponible.

`before/` contient une copie des onze fichiers avant cette intervention. `integration.patch` compare uniquement cette intervention à ces copies, sans inclure les modifications antérieures de l'espace de travail.
