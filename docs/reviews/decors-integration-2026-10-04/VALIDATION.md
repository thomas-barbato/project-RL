# Matières intégrées aux lieux existants — 4 octobre 2026

L’utilisateur a retenu le rendu des premières planches de sols et des murs en béton, avec une contrainte explicite : respecter l’architecture actuelle du jeu et ne pas remplir les lieux d’objets pour reproduire la scène de présentation.

## Portée

Les matières sont maintenant utilisées dans une partie normale, en debug comme en release. `terminal_materials.rs` contient uniquement des données de présentation ; `terminal_floor.rs` les place dans son atlas existant. La distinction des types de cases, les raccords de murs et les éclairages restent dans leurs chemins habituels. Les grilles, sols naturels et matières des profondeurs gardent leur rendu existant.

Aucun changement dans les générateurs, les plans de pièces, les passages, les règles de butin ou les positions de mobilier. Les personnages, ombres, icônes et illustrations précédemment validés sont conservés. Le catalogue dense des anciennes captures reste un diagnostic isolé ; il n’est pas ajouté à la partie normale.

La banque active provient des concepts illustrés `sols-32-review-v1.png` et `murs-32-review-v1.png`, échantillonnés en 32 pixels comme dans `decor.js`. Les sources originales sont conservées.

## Captures du vrai client

- [Départ de partie auprès d’Elias](depart-normal/cold-start.png).
- [Rue et passages de la carte normale](ville-normale/cold-start.png), après déplacement par les commandes habituelles.

Ces deux captures utilisent la génération normale, sans ajout de décor, d’acteur ou de butin pour la présentation. Carte de 192 × 128, trois piles de butin dans la carte entière, aucun objet visible sur ces captures. Les deux `art-validation.json` confirment des cases de 32 pixels et des octets d’état moteur strictement identiques avant et après les trois images de rendu. Les captures utilisent un profil isolé et ne modifient pas la sauvegarde du joueur.

## Vérification

- `cargo fmt --all --check`, `cargo check --locked`, `git diff --check` : réussis.
- Compilations debug et release de `project-rl` : réussies.
- `python tools/art/build_cyberpunk_assets.py --check` et `python tools/art/build_terrain_reference.py --check` : banques reproduites à l’identique.
- [Tests graphiques ciblés](graphics-tests.log) : 56 réussis, aucun échec.
- [Contrôle natif des ombres et de la mémoire](memoire-visibilite/shadow-pixels.json) : les cellules cachées gardent la couleur attendue ; aucune modification de la simulation.
- [Diagnostic natif de performance](performance/timings.json) : dix scènes de 120 images terminées. Monde : médiane CPU 7,94 ms, p95 8,30 ms ; inventaire : médiane 11,61 ms, p95 11,90 ms. Première image du monde : 351 ms, incluant la construction ponctuelle de l’atlas. Mesure debug locale, sans conclusion comparative avec les anciennes captures.

La suite générale n’a pas été répétée pour ce remplacement de matières. Les 24 échecs client déjà reproduits sur `HEAD` sont décrits dans [le rapport précédent](../cyberpunk-integration-2026-10-04/VALIDATION.md).

Aucun commit ni push.
