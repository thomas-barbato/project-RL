# MONSTER-VERTICAL20-r01 · Marches haut et bas

- 58 / MON-090-0 · Créature brune aux six pattes claires. Corps bas et
  appuis courts.
- 59 / MON-092-0 · Quadrupède pâle aux larges griffes. Le dos et la
  petite queue se découvrent quand il s'éloigne.
- 60 / MON-093-0 · Colosse aux poignets enchaînés. La chaîne pend devant
  le bassin et celui-ci la masque dans la vue de dos.

Huit poses entières par direction à 400 ms par pas et 800 ms par cycle.
Les frontaux installés servent de pose de repos. Le joueur valide les
n°58 à 60 et demande de réunir le reste des modèles dans un aperçu animé.
Voir `MONSTER-VERTICAL20-ACCEPTANCE-r01.json` et `MONSTER-VERTICAL-ALL.md`.

Les planches et les prompts sont dans
`assets/source-art/animation-polish-01/monster-vertical-20/r01/`.

```powershell
python tools/art/prepare_monster_vertical15.py --batch 20 --references
python tools/art/prepare_monster_vertical15.py --batch 20
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical20 artifacts/animation-polish-01/monster-vertical-20/r01/current
python tools/art/build_monster_vertical15_review.py --batch 20
```
