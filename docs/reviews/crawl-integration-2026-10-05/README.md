# Intégration des tiles natives — 5 octobre 2026

Autorisation du joueur : adapter les armes visibles à toutes les armes du jeu,
puis intégrer les créations présentées. Correction ajoutée : fer de hache vers
l'extérieur, avec retournement de la couche d'arme uniquement.

## Contenu actif

- 17 corps de tenue : trois protocoles et quatorze modèles de torse existants.
- 23 références existantes de bestiaire/NPC. Le zombie reste une image en réserve.
- 113 modèles d'armes avec correspondance explicite vers 18 familles visibles.
- Une arme par main, deux armes à une main ou une arme à deux mains selon le groupe actif.
- Armes portées réellement par les NPC humains, dans le monde et l'inspection.
- 162 objets couverts par le catalogue. Icônes et portraits d'armes utilisent la même famille.
- Les casques, gants et bottes conservent leurs dessins distincts, conditionnés en 32 px.
- Corps natifs de 30 px, cases de 32 px, composition de 48 px avec débordement visuel.
- Variantes de couleur existantes conservées, avec deux matériaux de vêtement et
  palettes indépendantes pour les armes de gauche et de droite.

Les noms, IDs, statistiques, collisions, données d'instance, règles de mains,
loot et formats de sauvegarde ne sont pas modifiés par ce pipeline de rendu.
La tenue complète reste sélectionnée par le torse ; les autres slots gardent
leurs effets et leur présence dans l'inventaire validé.

## Sources et fabrication

`../crawl-methode-2026-10-05/` conserve les sources de personnages/monstres,
tiles natives et quatre poses de bras déjà présentées. `held-source.png` contient
18 couches d'armes ; `axe-outward-source.png` remplace sa quatrième case.
Ces deux dessins ont été créés avec l'outil intégré **imagegen**, sans API externe.
Prompts exacts : `held-prompt.txt` et `axe-prompt.txt`.

Le constructeur `tools/art/build_crawl_assets.py` ne dessine pas de bitmap :
découpe, seuil alpha, réduction nearest, encodage sans dithering, pivots et
masques déclarés, puis atlases. Il conserve les indices du catalogue enregistré
dans `catalog-source.json`. Les anciens points d'entrée délèguent à ce constructeur.

Le runtime compose corps → manches → arme → doigts. Les masques natifs utilisent
0 pour les matériaux protégés, 1 pour la couleur principale et 2 pour la secondaire.
Ils sont indépendants de la posture ; la recoloration ne recherche aucune teinte
orange ni rectangle d'arme dans le personnage composé. La paire de couleurs
secondaires conserve le contraste. Les images d'inspection dérivent du dessin natif.

Les caches ont des plafonds de 192 personnages composés et 256 objets. Les tiers
partageant famille, posture et palettes partagent la même texture. La résolution
du panneau ne crée pas une autre texture ; les appels répétés réutilisent le cache.

## Reproduction

```powershell
python tools/art/build_crawl_assets.py
python tools/art/build_crawl_assets.py --check
cargo fmt --check
cargo check --locked
cargo test --locked --bin project-rl game_art::
cargo build --locked
cargo build --release --locked
```

Captures isolées avec le renderer actif, sans écrire dans la partie normale :

```powershell
target/debug/project-rl.exe --ui-cold-crawl-weapons artifacts/crawl-integration-2026-10-05/captures/weapons
target/debug/project-rl.exe --ui-cold-crawl-outfits artifacts/crawl-integration-2026-10-05/captures/outfits
target/debug/project-rl.exe --ui-cold-crawl-palettes-1 artifacts/crawl-integration-2026-10-05/captures/palettes-1
target/debug/project-rl.exe --ui-cold-crawl-palettes-2 artifacts/crawl-integration-2026-10-05/captures/palettes-2
target/debug/project-rl.exe --ui-cold-crawl-bestiary artifacts/crawl-integration-2026-10-05/captures/bestiary
```

Les fichiers `art-validation.json` vérifient que le rendu ne change pas l'état de
la partie et enregistrent les créations de textures avant/après les frames répétées.
Les résultats finaux de compilation, tests et mesures sont dans `VALIDATION.md`.
