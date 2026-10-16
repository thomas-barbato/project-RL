"""Evidence/layout only: copy current inputs and enlarge existing head crops."""
from pathlib import Path
import hashlib, json, shutil, subprocess
from PIL import Image, ImageDraw, ImageFont
ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
POSES = ('unarmed','side','two-handed','raised','melee-two-handed','dual-melee','dual-raised','dual-mixed')
paths = [ROOT/'src/game_art_palette.rs'] + sorted((ROOT/'assets/sprites/cyberpunk').glob('*'))
paths += sorted((ROOT/'content/core').rglob('*.json5'))
paths += sorted((ROOT/'src').rglob('*.rs'))
manifest = {}
for path in dict.fromkeys(paths):
    if not path.is_file(): continue
    rel = path.relative_to(ROOT)
    manifest[rel.as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    if rel.as_posix() == 'src/game_art_palette.rs' or rel.parts[:3] == ('assets','sprites','cyberpunk'):
        target = OUT/'avant'/rel
        target.parent.mkdir(parents=True,exist_ok=True)
        if not target.exists(): shutil.copy2(path,target)
(OUT/'baseline.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
(OUT/'avant.diff').write_bytes(subprocess.check_output(['git','diff','--binary'],cwd=ROOT))
font=ImageFont.truetype('C:/Windows/Fonts/consola.ttf',13)
board=Image.new('RGBA',(1180,17*98+40),(21,26,31,255)); d=ImageDraw.Draw(board)
for pose,name in enumerate(POSES): d.text((82+pose*136,8),name,font=font,fill='white')
for index in range(17):
    y=40+index*98; d.text((5,y+24),str(index),font=font,fill='white')
    for pose,name in enumerate(POSES):
        atlas=Image.open(ROOT/f'assets/sprites/cyberpunk/player-poses-{name}-48.png').convert('RGBA')
        tile=atlas.crop((index%16*48,index//16*48,index%16*48+48,index//16*48+48))
        head=tile.crop((19,16,30,26)).resize((88,80),Image.Resampling.NEAREST)
        board.alpha_composite(head,(82+pose*136,y))
board.save(OUT/'tetes-originales.png')
print(f'Baseline: {len(manifest)} files; head board ready.')
