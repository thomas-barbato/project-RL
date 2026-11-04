# MONSTER-VERTICAL15-r01 · Marches de face et de dos

Après validation du n°42 corrigé, trois nouvelles propositions en variante 0 :

- 43 / MON-075-0 : bête brune aux épaules rouges, quatre appuis et corps bas.
- 44 / MON-076-0 : aberration ivoire, deux pieds porteurs et tentacules libres.
- 45 / MON-077-0 : colosse ivoire sans tête, deux appuis et longues griffes.

Huit poses entières par direction, à 400 ms par pas et 800 ms par cycle.
Le frontal installé reste identique au repos. Les nouvelles marches sont
comparées au jeu à taille native. Le joueur valide le lot (« je valide tu
peux continuer :) »), sans activation en partie normale. Voir
`MONSTER-VERTICAL15-ACCEPTANCE-r01.json`.

Sources et prompts : `assets/source-art/animation-polish-01/monster-vertical-15/r01/`.

```powershell
python tools/art/prepare_monster_vertical15.py --references
python tools/art/prepare_monster_vertical15.py
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical15 artifacts/animation-polish-01/monster-vertical-15/r01/current
python tools/art/build_monster_vertical15_review.py
```
