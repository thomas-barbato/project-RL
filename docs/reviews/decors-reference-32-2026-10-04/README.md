# Comparaison des premières textures dans le client

Cette proposition répond au rejet des décors de la première intégration. Les personnages et les objets restent ceux validés par l’utilisateur.

Les matières reprennent les planches existantes `sols-32-review-v1.png` et `murs-32-review-v1.png`. Elles sont échantillonnées comme dans `decor.js`, puis placées dans une banque de vingt cellules de 32 pixels. Ce sont les anciens concepts illustrés, pas de nouveaux dessins créés nativement pixel par pixel.

- [Murs en béton](concrete/cold-start.png).
- [Murs en acier](steel/cold-start.png).

Les deux images viennent du client Macroquad avec les cases à 32 pixels, les raccords, les ombres, les règles de visibilité et le panneau de cible du jeu. Chaque `art-validation.json` confirme que les octets de l’état moteur restent identiques pendant les trois images de capture.

Ces captures archivent la comparaison avant intégration. L’utilisateur a retenu cette direction, en demandant de respecter l’architecture des lieux et les règles de placement des objets. Le chemin de comparaison temporaire a été remplacé par le rendu normal dans `terminal_materials.rs` et `terminal_floor.rs`, avec les murs en béton. La scène dense de collection d’objets reste un diagnostic isolé et n’est pas ajoutée aux cartes du jeu.

Reproduire la banque active : `python tools/art/build_terrain_reference.py` ; vérifier avec `--check`. Les nouvelles captures de lieux normaux et les vérifications sont consignées dans `docs/reviews/decors-integration-2026-10-04/VALIDATION.md`.
