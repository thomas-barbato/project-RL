# MONSTER-VERTICAL17-r01 · Marches haut et bas

- 49 / MON-081-0 · Quadrupède à carapace brune et à pattes courtes.
- 50 / MON-082-0 · Bête rouge à cornes et à quatre sabots. Son dessin
  sert de référence pour la démarche, avec le masque tourné vers le bas
  et la croupe visible vers le haut.
- 51 / MON-083-0 · Scorpion rouille. La carapace et la queue se lisent de
  dos, avec les pinces au premier plan de face.

Huit poses entières par direction à 400 ms par pas et 800 ms par cycle.
Les frontaux installés servent de pose de repos. Le joueur valide le lot
à taille native (« je valide », puis « et on continue »).
Voir `MONSTER-VERTICAL17-ACCEPTANCE-r01.json`.

Les planches et les prompts sont dans
`assets/source-art/animation-polish-01/monster-vertical-17/r01/`.

```powershell
python tools/art/prepare_monster_vertical15.py --batch 17 --references
python tools/art/prepare_monster_vertical15.py --batch 17
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical17 artifacts/animation-polish-01/monster-vertical-17/r01/current
python tools/art/build_monster_vertical15_review.py --batch 17
```
