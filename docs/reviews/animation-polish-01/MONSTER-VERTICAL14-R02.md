# MONSTER-VERTICAL14-r02 · N°42, marche de dos

Le joueur a signalé que le démon aux griffes dorées (MON-074-0) semblait
toujours de face en marchant vers le haut dans r01. Cette révision propose
huit poses de dos entières : nuque, omoplates, colonne et talons visibles ;
les griffes dorées et la crête restent reconnaissables.

Le frontal à l'arrêt, la marche vers le bas et les n°40 et 41 sont repris
de r01. La cadence reste de 400 ms par pas. Le joueur valide le n°42 dans
cette version (« oui la ca va, je valide et on continue »). Voir
`MONSTER-VERTICAL14-ACCEPTANCE-r02.json`. L'accord est visuel.

Les sources et le prompt de correction sont dans
`assets/source-art/animation-polish-01/monster-vertical-14/r02/`.
Les références du prompt étaient le frontal du n°42, sa planche de dos r01
rejetée et la planche de dos du MON-068 comme repère d'orientation.

```powershell
python tools/art/prepare_monster_vertical14_rear.py
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical14-r02 artifacts/animation-polish-01/monster-vertical-14/r02/current
python tools/art/build_monster_vertical14_review.py --rear
```
