# MONSTER-VERTICAL19-r01 · Marches haut et bas

- 55 / MON-087-0 · Quadrupède trapu. Le corps reste bas et la gorge ocre
  sert de repère de face.
- 56 / MON-088-0 · Oiseau aux ailes rouges repliées, deux serres au sol.
  Le cou ivoire se découvre de face, puis les plumes du dos le masquent
  en partie quand l'oiseau s'éloigne.
- 57 / MON-089-0 · Créature sombre aux quatre pattes ocre et anguleuses.

Huit poses entières par direction à 400 ms par pas et 800 ms par cycle.
Les frontaux installés servent de pose de repos. Le joueur valide le lot
à taille native (« oui je valide on peut continuer :) »).
Voir `MONSTER-VERTICAL19-ACCEPTANCE-r01.json`.

Les planches et les prompts sont dans
`assets/source-art/animation-polish-01/monster-vertical-19/r01/`.

```powershell
python tools/art/prepare_monster_vertical15.py --batch 19 --references
python tools/art/prepare_monster_vertical15.py --batch 19
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical19 artifacts/animation-polish-01/monster-vertical-19/r01/current
python tools/art/build_monster_vertical15_review.py --batch 19
```
