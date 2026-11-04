# MONSTER-VERTICAL16-r01 · Marches de face et de dos

Trois nouveaux candidats en variante 0 :

- 46 / MON-078-0 : colosse à carapace sombre, deux appuis et grandes griffes.
- 47 / MON-079-0 : démon rouge avec lame solidaire de l'avant-bras droit.
- 48 / MON-080-0 : brute voûtée ivoire et rouge, tête rentrée et poings bas.

Huit poses entières par direction, à 400 ms par pas et 800 ms par cycle.
Les frontaux installés sont conservés au repos. Le joueur valide les trois
marches à taille native le 4 novembre (« ok je valide et on reprend :) »).
Voir `MONSTER-VERTICAL16-ACCEPTANCE-r01.json`.

Sources et prompts : `assets/source-art/animation-polish-01/monster-vertical-16/r01/`.

```powershell
python tools/art/prepare_monster_vertical15.py --batch 16 --references
python tools/art/prepare_monster_vertical15.py --batch 16
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical16 artifacts/animation-polish-01/monster-vertical-16/r01/current
python tools/art/build_monster_vertical15_review.py --batch 16
```
