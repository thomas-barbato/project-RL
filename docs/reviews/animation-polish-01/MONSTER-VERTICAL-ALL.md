# MONSTER-VERTICAL-ALL-r01 · Revue groupée

Le joueur valide les n°58 à 60 et demande à examiner ensemble les nouveaux
cycles des 69 modèles de base restants. Leur numérotation suit l'inventaire
des modèles sans cycle présenté, du n°61 au n°129.

Chaque candidat reprend l'identité de la variante 0. Le cycle avance sur
deux cases à 400 ms par case, avec huit poses entières par direction et le
frontal installé au repos. Les appuis, la reptation ou la dérive suivent
l'anatomie de chaque créature. Les 342 autres variantes relèvent d'un travail
distinct. Les nouveaux candidats sont soumis en aperçu.

Les références, les planches et les prompts sont dans
`assets/source-art/animation-polish-01/monster-vertical-all/r01/`.
Le manifeste associe chaque modèle aux dessins réellement présentés.

```powershell
python tools/art/prepare_monster_vertical_all.py
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical-all artifacts/animation-polish-01/monster-vertical-all/r01/current
python tools/art/build_monster_vertical_all_review.py
```

## MONSTER-VERTICAL-ALL-r02 · Corrections après la revue

« Je valide tous les autres » valide les 118 modèles de base sans réserve
dans l’aperçu r01. Les identifiants, atlas et ressources associées sont figés
dans `MONSTER-VERTICAL-ALL-ACCEPTANCE-r01.json`.

- **62** (MON-011) : rendre les deux tentacules latéraux visibles pendant les
  deux déplacements.
- **78, 79, 82** (MON-105/106/109) : rendre le demi-tour lisible sur toute la
  remontée. Montrer la crête et les épaules arrière du n°78, puis la croupe et
  les appuis arrière des n°79 et 82.
- **89** (MON-118) : garder la robe et ses pans sur les hanches en rampant.
- **95** (MON-124) : montrer le dos du manteau. Les bras restent devant le
  torse et sont occultés depuis cette vue.
- **115, 116, 118, 127, 129** (MON-145/147/149/160/162) : placer les racines
  des deux ailes sur le dos visible et faire ressortir le départ des nervures.

Les dessins r02, comparés directement aux atlas r01, gardent les deux repos
frontaux pixel pour pixel et les cycles de face des dix retouches de dos.
La cadence reste à 400 ms par pas. Les onze corrections sont en revue.
Les sources et prompts de génération sont dans
`assets/source-art/animation-polish-01/monster-vertical-all/r02/`.

```powershell
python tools/art/prepare_monster_vertical_all_corrections.py
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical-all artifacts/animation-polish-01/monster-vertical-all/r02/current --corrections
python tools/art/build_monster_vertical_all_review.py --corrections
```

## MONSTER-VERTICAL-INTEGRATION01

Le joueur valide les onze corrections r02 et autorise l’intégration des 129
modèles de base. Les marches vers le haut et le bas utilisent les huit poses
approuvées, sur deux pas de 400 ms avec le réglage par défaut. À l’arrêt et en
mouvement réduit, le rendu sélectionne le frontal habituel. Les 342 variantes
secondaires suivent leurs cycles propres.

Les atlas installés sont dans `assets/animations/monster-vertical-01/`. Les
marges transparentes sont rognées et l’ancrage est déplacé d’autant, sans
rééchantillonnage. Les palettes approuvées sont appliquées à la fabrication.
Le cache commun prépare les atlas des monstres visibles et proches avant le
rendu. Les empreintes sont dans `MONSTER-VERTICAL-INTEGRATION01-ASSETS.json`.

```powershell
python tools/art/install_monster_vertical_all.py
cargo build --locked --bin project-rl
$output = 'artifacts/animation-polish-01/monster-vertical-integration01'
.\target\debug\project-rl.exe --ui-cold-monster-vertical-all "$output/baked" --bake-palettes
python tools/art/install_monster_vertical_all.py --baked "$output/baked"
.\target\debug\project-rl.exe --ui-cold-monster-vertical-all "$output/native" --verify-installed
```
