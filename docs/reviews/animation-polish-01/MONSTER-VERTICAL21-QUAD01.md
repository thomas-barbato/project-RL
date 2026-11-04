# MONSTER-VERTICAL21-QUAD01-r01 — reprise du n°21 sur quatre appuis

Le 3 novembre 2026, le joueur valide les n°22–24 puis précise :
« je valide sauf le 21 : il n'est pas bipede lui donc je sais pas pourquoi
il se leve sur 2 jambes :) . »

Le n°21 est MON-032-0. Sa validation antérieure est retirée pour la marche
bipède. Les n°19–20 restent acceptés ; l’accord du lot 07 est amendé dans
`MONSTER-VERTICAL07-AMENDMENT-r02.json`, avec copie de la décision initiale.
L’accord des n°22–24 est enregistré dans
`MONSTER-VERTICAL08-ACCEPTANCE-r01.json`. Aucun de leurs dessins n’est modifié.

L’ancien classement bipède venait des métadonnées de la revue WALK07. Il
était erroné : le retour du joueur et les quatre membres présents dans le
dessin d’origine priment. L’inventaire de cette revue classe désormais
MON-032 comme quadrupède et conserve la provenance de la correction.
Son ID, ses variantes, son rôle de jeu et ses règles ne changent pas.

Reprendre deux planches de huit figures entières : corps bas, dos arrondi,
ventre près du sol, pattes avant porteuses et appuis alternés avec les pattes
arrière. Aucun redressement sur les seuls membres arrière, bras pendants,
torse humain, jambes allongées ou membres ajoutés. Garder la tête recourbée,
la petite tache claire de l’œil frontal, les matières et la silhouette du
modèle. De dos, masquer l’œil, le visage et le ventre frontal. Le repos
conserve exactement le frontal installé, son ancrage et son ombre.

La cadence reste de 400 ms par avancée et 800 ms par cycle. Les phases du
lecteur parlent des pattes et des groupes d’appuis pour ce modèle ; les
autres modèles conservent leurs phases propres. Le nouveau n°21 remplace
son essai bipède dans le sélecteur : 24 modèles uniques, dont 23 références
conservées. Les anciens fichiers du n°21 sont conservés comme essai rejeté.

Sources et atlas :
`assets/source-art/animation-polish-01/monster-vertical-21-quad/r01/manifest.json`.
Planches produites avec l’outil intégré image_gen, en mode édition ciblée,
sans guide de marche bipède. Prompts et références exacts dans
`MONSTER-VERTICAL21-QUAD01-PROMPTS.json`.

La préparation préserve 142 empreintes : les 129 fichiers antérieurs, dont
les quatre anciens fichiers du n°21 désormais retirés de l’acceptation,
et les treize ressources du lot 22–24 accepté. Préparer les images et les
textures avant les captures, sans lecture lourde dans le dessin. Les dix
atlas latéraux intégrés restent inchangés.

Statut : correction du n°21 acceptée par « je valide :) on continue ».
Les quatre ressources sont figées dans
`MONSTER-VERTICAL21-QUAD01-ACCEPTANCE-r01.json`, sans activation dans la partie normale. Les données de partie et préférences ne sont pas
chargées par la capture.

```powershell
python tools/art/prepare_monster_vertical21_quad.py --references
python tools/art/prepare_monster_vertical21_quad.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo run --locked --bin project-rl -- --ui-cold-monster-vertical21-quad artifacts/animation-polish-01/monster-vertical-21-quad/r01/current
python tools/art/build_monster_vertical21_quad_review.py
python tools/art/check_monster_vertical21_quad.py
```

Vérification finale du 3 novembre 2026 : compilation verrouillée et cinq
tests de palette antérieurs réussis. Les 66 captures du n°21 présentent
16 figures mobiles distinctes et deux retours au frontal exact. Les 142
empreintes antérieures sont conservées, dont les quatre fichiers de l’essai
bipède rejeté. Le lot 07 est partiellement accepté (MON-026/028) ; le lot 08
est accepté (MON-008/034/036). Les sources, leurs références et leurs
empreintes sont dans `MONSTER-VERTICAL21-QUAD01-GENERATIONS.json` et le
fichier de prompts. Formatage Rust ciblé, syntaxe Python et
`git diff --check` passent.

Dans l’onglet courant, les 24 modèles uniques chargent chacun 66 images,
avec une seule entrée MON-032 utilisant la nouvelle banque quadrupède.
Les 18 positions manuelles (huit phases et un repos par direction),
l’aller-retour automatique à 400 ms et le bouton de pose suivante ont été
exercés. Les phases bipèdes des autres modèles restent disponibles ; le
lecteur du n°21 utilise ses noms de groupes d’appuis. L’inventaire conserve
129 fiches et classe MON-032 comme quadrupède. Comparatif de 576 × 320 px
sans redimensionnement CSS, aucune erreur console après l’ouverture finale.
Ce contrôle technique ne remplace pas la validation visuelle de la nouvelle
démarche par le joueur.

Bilan : `MONSTER-VERTICAL21-QUAD01-VALIDATION.json`. Preuves et compte rendu
du lecteur sous `artifacts/animation-polish-01/monster-vertical-21-quad/r01/` :
`browser-proof.png`, `browser-back-proof.png`, `native-proof.png` et
`browser-checks.json`. Ces captures ne sont pas des dépendances du jeu.
