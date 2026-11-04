# MONSTER-VERTICAL18-r01 · Marches haut et bas

- 52 / MON-084-0 · Démon pâle aux longs bras et aux pas courts. Ses deux
  pieds prennent appui sous son corps voûté.
- 53 / MON-085-0 · Créature à bosse rouge. La bosse garde son asymétrie
  quand la créature se tourne.
- 54 / MON-086-0 · Démon rouge dont les deux lames suivent les avant-bras
  dans les deux sens.

Huit poses entières par direction, à 400 ms par pas et 800 ms par cycle.
Les frontaux installés servent de pose de repos. Le joueur valide le lot
à taille native (« oui je valide :) »).
Voir `MONSTER-VERTICAL18-ACCEPTANCE-r01.json`.

Les planches et les prompts sont dans
`assets/source-art/animation-polish-01/monster-vertical-18/r01/`.

```powershell
python tools/art/prepare_monster_vertical15.py --batch 18 --references
python tools/art/prepare_monster_vertical15.py --batch 18
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical18 artifacts/animation-polish-01/monster-vertical-18/r01/current
python tools/art/build_monster_vertical15_review.py --batch 18
```
