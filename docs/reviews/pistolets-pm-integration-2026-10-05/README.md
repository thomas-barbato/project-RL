# Intégration des pistolets et PM — 5 octobre 2026

Douze armes à une main sont ajoutées à la génération 146 : six Cogneur et six
Crache-fer. Elles réutilisent les dessins de tir industriels, les poses et les
sept palettes déjà retenus. Aucune image de personnage ni aucun PNG existant
n'est redessiné à cette étape.

Les modèles, portées, masses, dégâts, coûts et prix sont décrits dans
[Pistolets et PM](../../docs/PISTOLETS_ET_PM.md). Les anciennes parties conservent
leur catalogue historique. Une nouvelle partie est nécessaire pour trouver les
nouveaux modèles dans le butin, les caches et les boutiques.

## Captures du client réel

- `capture-dual/cold-start.png` : paire pistolet/PM active, armes levées.
- `capture-mixed/cold-start.png` : pistolet à gauche et mêlée à droite.
- `capture-mixed-inverse/cold-start.png` : mêlée à gauche et PM à droite.
- `capture-two-hands/cold-start.png` : fusil actif, armes à une main conservées.
- `capture-inventory-pistol/cold-start.png` : trois slots équipés, tooltip du pistolet.
- `capture-inventory-pm/cold-start.png` : mêmes objets, tooltip du PM.

Ces scènes isolées utilisent les vrais objets, commandes, palettes et rendu du
client. Leur capacité d'inventaire est portée à 24 pour garder l'équipement de
départ et les objets de contrôle. Elles ne modifient pas la sauvegarde normale.

Reproduction : `target/debug/project-rl.exe --ui-cold-one-hand-firearms-dual <nouveau-dossier>`.
Remplacer `dual` par le suffixe d'une capture ci-dessus pour les autres scènes.
Le script `build_content.py` produit les 26 nouveaux fichiers JSON5 depuis
`modeles.json`. Le catalogue de sprites se reconstruit avec
`tools/art/build_cyberpunk_assets.py` ; les contrôles `--check` ne dessinent aucune
nouvelle image. Le client charge les JSON5 natifs et embarque les banques PNG ;
Python n'est pas nécessaire pour jouer.

`avant` contient les sources et banques au début de cette étape ainsi que trois
vraies suspensions historiques (143, 144, 145). `verify_preservation.py` compare
624 fichiers existants sans suivre les anciens checkouts de diagnostic.

Les résultats détaillés sont dans [VALIDATION.md](VALIDATION.md).
