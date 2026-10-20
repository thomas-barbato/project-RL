# ART-FINISH-05 — quatre ennemis existants

19 octobre 2026. **Lot validé et intégré : « Valider le lot complet ».** Après présentation
des quatre dessins sur trois fonds et dans deux scènes éclairées, le joueur
valide l'ensemble. `acceptance.json` fige les empreintes des quatre exports.
Ce lot reprend quatre dessins de campagne existants, sans nouveau nom,
pouvoir ou peuplement.

- FIN-28 — Cultiste, 014 A : pistolet et uniforme gris/rouge, masque ivoire.
- FIN-29 — Cultiste, 016 A : masse, bouclier et armure gris/rouge, masque ivoire.
- FIN-30 — Possédé, 006 A : hybride quadrupède, chair pâle et machine orange.
- FIN-31 — Zombie, 120 A : détenu réanimé, uniforme sombre et bande jaune.

Les précisions servent uniquement à désigner les dessins dans la revue. Les
identités, rôles et noms de jeu restent ceux du manifeste actuel. Le lot porte
sur les seules variantes A ; les autres palettes ne sont pas remplacées ici.
Les boss de secteur et leurs patterns restent un chantier distinct.

## Fabrication et invariants

Quatre maîtres complets sont générés avec le skill imagegen et l'outil intégré,
en prenant le dessin existant pour cible et FIN-06 ou FIN-08 pour référence de
finition. Les prompts et chemins de sortie sont conservés avec les sources dans
`assets/source-art/art-finish-review-05/`. FIN-30 fait l'objet d'une correction
de cadrage : sa pince touchait le bord inférieur de la première source. Le
maître corrigé est complet avec marge transparente, sans changement de design.
`repair-prompts.json` conserve la demande exacte et l'original refusé est identifié.

Le sujet entier est réduit avec Lanczos dans sa toile actuelle de 128 × 128 px,
sans étirement et sans agrandir sa taille mondiale. Les dimensions cibles et
le pied (64,96) viennent du manifeste ; le filtre du moteur reste Nearest.
Les ombres de contact restent séparées et réutilisent le gabarit de chaque
monstre. Le pistolet et la masse de ces dessins de monstre conservent leurs
prises d'origine ; ce lot ne crée aucune pièce interchangeable du joueur.

Les modes de revue sont explicites, désactivés par défaut et exclus des builds
release. Les sources ne sont lues que par ce prototype. Pendant la préparation,
aucune ressource compilée de `assets/sprites/` n'est modifiée. Les figures en
scène sont dessinées, jamais insérées dans la simulation. Les empreintes de
partie sont contrôlées avant et après. L'intégration acceptée conserve cartes,
règles, collisions, sauvegardes et répartition des ennemis.

## Présentation et contrôles

La page courante `artifacts/art-quality-audit-01/index.html` conserve les quatre
comparaisons à taille native, quatre références acceptées et les mêmes figures
sur fond clair, sombre et sol de la rue. Deux paires avant/après utilisent le
rendu réel avec les lumières et la visibilité du moteur. Les dimensions des
monstres restent celles de leurs entrées existantes, et Bastion reste accepté.

```text
python tools/reviews/art_finish_batch_05.py
python tools/reviews/art_finish_batch_05.py --condition
cargo build --locked
target/debug/project-rl.exe --ui-cold-art-finish-05-review artifacts/art-quality-audit-01/finish05-engine
python tools/reviews/show_art_finish_review_05.py
target/debug/project-rl.exe --ui-cold-art-finish-04-integration artifacts/art-quality-audit-01/finish05-normal-check
python tools/reviews/validate_art_finish_review_05.py
```

`review.json` conserve les sources, exports et bornes ; `production-before.json`
fige les ressources ordinaires avant préparation. Les bilans sont enregistrés
séparément. La présentation puis l'accord explicite suivent la procédure de
[AGENTS.md](../../../../AGENTS.md). Les planches acceptées et les références de
scène restent figées ; les scripts de préparation refusent de les régénérer.

La compilation debug et le contrôle de compilation release passent. Les
24 panneaux conservent les pixels GPU exacts. Le rendu normal reste identique
sur huit échantillons et deux vues acceptés de FIN-26/27 ; les 626 ressources
précédentes restent inchangées. Ce contrôle visuel ciblé ne constitue pas une
exécution de la suite complète de tests. Les captures de référence normales
redondantes sont retirées après comparaison, avec sources et revue conservées.

## Intégration acceptée

Les quatre exports exacts vont dans `assets/sprites/art-finish-05/` et sont
embarqués par `src/game_art_finish_catalog_05.rs`. Le fournisseur commun de
campagne sert au plateau et à l'inspection. Seules les liaisons 014 A, 016 A,
006 A et 120 A sont remplacées ; les autres variantes restent sur leurs images
précédentes. Les ombres de contact, pieds et dimensions de toile sont conservés.
Le traitement des morts conserve sa convention existante de corps source
assombri et tourné ; aucune nouvelle pose ou animation de mort n'est ajoutée.

```text
python tools/reviews/integrate_art_finish_review_05.py
cargo test --locked --bin project-rl game_art:: -- --test-threads=1
cargo build --locked
target/debug/project-rl.exe --ui-cold-art-finish-05-integration artifacts/art-quality-audit-01/finish05-integration
python tools/reviews/verify_art_finish_integration_05.py
cargo check --locked --release
cargo fmt --all -- --check
git diff --check
```

`integration-checks.json` compare le rendu ordinaire aux douze échantillons
et aux deux scènes acceptés. `integration-validation.json` consigne les
contrôles réalisés et leurs limites. Les captures temporaires devenues
redondantes sont retirées dans `integration-cleanup.json`, avec maintien des
sources, planches acceptées et références utiles. Le jeu ne dépend d'aucun
fichier sous `artifacts/` ni de la banque source de fabrication.

L'audit du mobilier, en commençant par le lit signalé, reste planifié après la
refonte des acteurs. Aucun mobilier n'est redimensionné pendant ce lot.
