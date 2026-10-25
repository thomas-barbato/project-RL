"""Pack seven whole right-facing TECH01 drawings without changing other poses."""
from pathlib import Path
import hashlib
import json

from PIL import Image, ImageDraw
from build_player_walk_matrix_06 import grid_figures
from build_walk_rigs_02 import rig

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-08'
DEST = ROOT / 'assets/animations/walk-08/player'
SOURCE = ROOT / 'assets/source-art/player-walk-review-08/TECH01-right-r03-source.png'


def main():
    proof = json.loads((DOC / 'TECH01-r03.json').read_text('utf-8'))
    manifest = json.loads(json.dumps(proof['manifest_before']))
    left = Image.open(DEST / 'CLASS-05-0.png').convert('RGBA')
    left_poses = []
    for row in range(7):
        whole = Image.new('RGBA', (128, 128))
        for part in [0, 1, 3, 2, 4]:
            whole.alpha_composite(left.crop((part*128, row*128, (part+1)*128, (row+1)*128)))
        left_poses.append(whole)
    target = max(p.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()[3]
                 - p.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()[1]
                 for p in left_poses)
    figures, warnings = grid_figures(Image.open(SOURCE).convert('RGBA'), 4, 2)
    figures = [f.crop(f.getbbox()) for f in figures]
    scale = min(target / max(f.height for f in figures), 58 / max(f.width for f in figures))
    sheet = Image.new('RGBA', (640, 896))
    new_poses = []
    filename = 'CLASS-05-1-r03.png'
    for row, figure in enumerate(figures[:7]):
        small = figure.resize((round(figure.width*scale), round(figure.height*scale)), Image.Resampling.LANCZOS)
        whole = Image.new('RGBA', (128, 128))
        whole.alpha_composite(small, (64-small.width//2, 96-small.height))
        new_poses.append(whole)
        parts, meta = rig(whole, 5)
        sheet.alpha_composite(parts, (0, row*128))
        entry = next(e for e in manifest if e['id']=='CLASS-05' and e['facing']==1 and e['sheet_row']==row)
        entry.update(meta)
        entry['file'] = f'assets/animations/walk-08/player/{filename}'
    sheet.save(DEST / filename, optimize=True)
    (DEST / 'manifest.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2)+'\n', 'utf-8')
    board = Image.new('RGB', (896, 288), '#303a40')
    draw = ImageDraw.Draw(board)
    for side, poses in enumerate([left_poses, new_poses]):
        draw.text((8, side*144+4), 'Gauche conservee' if side==0 else 'Droite - nouvel essai TECH01-r03', fill='white')
        for frame, whole in enumerate(poses):
            tile = whole.crop((32, 32, 96, 96)).resize((128, 128), Image.Resampling.NEAREST)
            board.paste(tile, (frame*128, side*144+16), tile)
    board.save(DOC / 'TECH01-r03-pair-x2.png')
    proof.update(source=str(SOURCE.relative_to(ROOT)).replace('\\','/'),
                 source_sha256=hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
                 output=f'assets/animations/walk-08/player/{filename}',
                 uniform_scale=scale, target_height=target, warnings=warnings,
                 changed_pose_count=7, whole_figure_conditioning=True,
                 all_original_player_atlases_unchanged=all(hashlib.sha256((DEST / f).read_bytes()).hexdigest()==h for f,h in proof['player_asset_hashes_before'].items()))
    (DOC / 'TECH01-r03.json').write_text(json.dumps(proof, ensure_ascii=False, indent=2)+'\n','utf-8')
    print(json.dumps({k:proof[k] for k in ['target_height','warnings','changed_pose_count','all_original_player_atlases_unchanged']}))


if __name__ == '__main__':
    main()
