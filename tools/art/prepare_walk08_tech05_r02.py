"""Fix scale of three whole source figures; retain the other eleven poses."""
from pathlib import Path
import hashlib
import json

from PIL import Image, ImageDraw
from build_player_walk_matrix_06 import grid_figures
from build_walk_rigs_02 import rig

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-08'
DEST = ROOT / 'assets/animations/walk-08/player'
SOURCE = ROOT / 'assets/source-art/player-walk-review-07/UNARMED-09-r01.png'


def bounds(image):
    return image.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()


def whole(sheet, row):
    image = Image.new('RGBA', (128, 128))
    for part in [0, 1, 3, 2, 4]:
        image.alpha_composite(sheet.crop((part*128, row*128, (part+1)*128, (row+1)*128)))
    return image


def main():
    proof = json.loads((DOC / 'TECH05-r02.json').read_text('utf-8'))
    manifest = json.loads(json.dumps(proof['manifest_before']))
    old = Image.open(DEST / 'CLASS-09-1.png').convert('RGBA')
    sheet = old.copy()
    idle_bounds = bounds(whole(old, 0))
    target = idle_bounds[3] - idle_bounds[1]
    figures, warnings = grid_figures(Image.open(SOURCE).convert('RGBA'), 4, 4)
    adjustments = []
    filename = 'CLASS-09-1-r02.png'
    for row in [4, 5, 6]:
        figure = figures[8+row]
        bb = bounds(figure)
        scale = target / (bb[3] - bb[1])
        small = figure.resize((round(figure.width*scale), round(figure.height*scale)), Image.Resampling.LANCZOS)
        sb = bounds(small)
        image = Image.new('RGBA', (128, 128))
        image.alpha_composite(small, (round(64-(sb[0]+sb[2])/2), 96-sb[3]))
        parts, meta = rig(image, 9)
        sheet.paste(parts, (0, row*128))
        entry = next(e for e in manifest if e['id']=='CLASS-09' and e['facing']==1 and e['sheet_row']==row)
        entry.update(meta)
        entry['file'] = f'assets/animations/walk-08/player/{filename}'
        adjustments.append({'frame': row-1, 'source_cell':8+row, 'uniform_scale':scale, 'before_height':bounds(whole(old,row))[3]-bounds(whole(old,row))[1], 'after_height':bounds(image)[3]-bounds(image)[1]})
    assert sheet.crop((0,0,640,512)).tobytes()==old.crop((0,0,640,512)).tobytes()
    sheet.save(DEST / filename, optimize=True)
    (DEST / 'manifest.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2)+'\n', 'utf-8')
    board = Image.new('RGB', (896, 288), '#303a40')
    draw = ImageDraw.Draw(board)
    for side, candidate in enumerate([old, sheet]):
        draw.text((8, side*144+3), 'Avant : changement de taille' if side==0 else 'TECH05-r02 : echelle corrigee', fill='white')
        for row in range(7):
            image = whole(candidate, row).crop((32,32,96,96)).resize((128,128), Image.Resampling.NEAREST)
            board.paste(image, (row*128,side*144+16), image)
    board.save(DOC / 'TECH05-r02-comparison-x2.png')
    changed = [(a['id'],a['facing'],a['frame']) for a,b in zip(proof['manifest_before'],manifest,strict=True) if a!=b]
    assert changed==[('CLASS-09',1,3),('CLASS-09',1,4),('CLASS-09',1,5)]
    proof.update(source=SOURCE.relative_to(ROOT).as_posix(),source_sha256=hashlib.sha256(SOURCE.read_bytes()).hexdigest(),output=f'assets/animations/walk-08/player/{filename}',target_height=target,warnings=warnings,adjustments=adjustments,changed_pose_bindings=changed,other_347_pose_bindings_exact=True,first_four_right_poses_exact=True,all_original_atlases_unchanged=all(hashlib.sha256((DEST/f).read_bytes()).hexdigest()==h for f,h in proof['player_asset_hashes_before'].items()),method='Uniform whole-figure export from existing high-resolution source; no redraw or limb deformation')
    (DOC / 'TECH05-r02.json').write_text(json.dumps(proof,ensure_ascii=False,indent=2)+'\n','utf-8')
    print(json.dumps({'target_height':target,'adjustments':adjustments,'other_poses_exact':347,'warnings':warnings}))


if __name__ == '__main__':
    main()
