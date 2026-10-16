# Personnages, couleurs de classes et changement d’arme — 4 octobre 2026

Correction locale préparée à la demande de l’utilisateur : personnages et monstres mieux cadrés en 32 pixels, une couleur par classe, et conservation de la tenue lorsque seule l’arme active change.

## Rendu

- Brèche : armure rouge ; Vigie : armure bleue ; Creuset : armure verte.
- Trois corps de classe stables. La sélection d’une arme ne choisit plus une variante complète de tenue ou de casque.
- 101 calques d’armes portées, un par modèle de base chargé : 61 armes à feu et 40 armes de mêlée. Ils proviennent des illustrations individuelles déjà présentes. Leur identité est indépendante du corps et des affixes.
- PNJ et bestiaire : mêmes silhouettes validées, recadrées sur leurs parties visibles avant export. Les humains utilisent désormais environ 30 pixels de hauteur, contre 25–26 auparavant ; la plus grande dimension des autres silhouettes approche 30 pixels. Les pieds sont alignés sur une ligne commune, avec leurs ombres recalculées.
- La banque comprend 3 corps de classe et 24 références de bestiaire. Le zombie demeure une référence inutilisée ; aucun ennemi n’est ajouté au contenu.

Les armures individuelles gardent leurs illustrations d’inventaire et de cible. Leurs calques portés interchangeables par modèle restent à réaliser ; la correction présente sépare la tenue de classe et les armes actives.

Les décors, les icônes de butin et les 150 illustrations d’inventaire restent identiques : leurs empreintes SHA-256 ont été vérifiées. Aucun changement dans les générateurs, plans des lieux, placements de mobilier, règles de butin ou formats de sauvegarde.

## Captures du client

- [Brèche rouge](cyberpunk-class-breche/cold-start.png).
- [Vigie bleue](cyberpunk-class-vigie/cold-start.png).
- [Creuset verte, lame](cyberpunk-class-creuset/cold-start.png).
- [Creuset verte, lance-flammes](cyberpunk-class-creuset-ranged/cold-start.png).
- [Personnages et bestiaire dans une scène de contrôle isolée](cyberpunk-scale/cold-start.png), sans objet posé au sol.

Les quatre captures de classes utilisent la carte normale, en ne changeant que la classe affichée et le choix de l’arme équipée pour comparaison. Aucun décor, butin ou acteur n’est ajouté à cette carte. Les trois piles de butin de la carte entière restent celles de la génération normale. La scène du bestiaire est un diagnostic distinct, pas un nouveau lieu du jeu.

Chaque capture confirme une simulation identique avant et après les trois images de rendu. Les profils de contrôle sont isolés de la sauvegarde du joueur.

## Vérifications

- Compilations debug et release : réussies.
- `cargo fmt --all --check`, `cargo check --locked`, `git diff --check` : réussis.
- Reproduction des banques avec les deux scripts et `--check` : identique.
- [Tests graphiques ciblés](graphics-tests.log) : 58 réussis. La couverture vérifie notamment tous les modèles d’armes et le changement effectif de l’emplacement d’arme sélectionné dans le client, en conservant le même corps de classe et l’état moteur.
- [Comparaison de pixels entre les deux captures du Creuset](weapon-swap-pixel-check.json) : 254 pixels de corps contrôlés hors des deux calques d’armes, zéro modification. La tolérance de localisation compense l’alpha du dessin sur le sol ; la comparaison entre les deux captures est stricte.
- [Diagnostic natif de performance](performance/timings.json) : dix scènes de 120 images terminées, simulation inchangée. Monde : médiane CPU 8,06 ms, p95 12,13 ms ; inventaire : médiane 11,79 ms, p95 13,05 ms. Première image du monde : 340 ms, incluant la création de l’atlas. Mesure debug locale, sans conclusion comparative.

La suite générale n’a pas été répétée pour ce changement de présentation. Les échecs client déjà reproduits sur `HEAD` restent documentés dans [le rapport initial](../cyberpunk-integration-2026-10-04/VALIDATION.md).

## Sources et reproduction

La base active des trois classes est [classes-couleurs-v3.png](classes-couleurs-v3.png), dérivée avec imagegen intégré des figures frontales précédentes. Les [prompts complets](PROMPTS.md) et les variantes intermédiaires sont conservés. Ce sont des retouches de grandes planches exportées en 32 pixels, pas des dessins manuels natifs pixel par pixel. Le bestiaire actif conserve `bestiaire-face-v2.png` et ses cadres validés.

Export : `python tools/art/build_cyberpunk_assets.py`. Vérification : même commande avec `--check`. Les atlas et le catalogue embarqués sont dans `assets/sprites/cyberpunk/` et `src/game_art_catalog.rs`. Le client n’utilise pas Python ni le serveur d’aperçu à l’exécution. Les banques de personnages et le script précédents sont conservés dans `before/`.

Aucun commit ni push.
