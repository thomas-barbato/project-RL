"""Read-only native64 front/walk audit of the currently accepted player art."""
from pathlib import Path
import hashlib
import json
import statistics
from PIL import Image, ImageDraw
from audit_walk09_body_volume import measure

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-consistency-13'
ART = ROOT / 'artifacts/player-consistency-review-13'
NAMES = ['Bastion', 'Technomancien', 'Traqueuse', 'Arcaniste', 'Metamorphe']


def main():
    DOC.mkdir(parents=True, exist_ok=True)
    ART.mkdir(parents=True, exist_ok=True)
    manifest = json.loads((ROOT / 'assets/animations/walk-12/player/manifest.json').read_text())
    bank = Image.open(ROOT / 'assets/sprites/class-armors-01/base.png').convert('RGBA')
    assets = {ROOT / 'assets/sprites/class-armors-01/base.png',
              ROOT / 'assets/animations/walk-12/player/manifest.json'}
    records = []
    fronts = []
    for index in range(25):
        x, y = index % 16 * 128, index // 16 * 128
        front = bank.crop((x, y, x+128, y+128))
        fronts.append(front)
        fm = measure(front)
        figures = {}
        poses = []
        for e in manifest:
            if e['id'] != f'CLASS-{index:02}':
                continue
            path = ROOT / e['file']; assets.add(path)
            sheet = Image.open(path).convert('RGBA'); yy = e['sheet_row'] * 128
            im = sheet.crop((0, yy, 128, yy+128))
            for part in [1, 3]:
                im.alpha_composite(sheet.crop((part*128, yy, (part+1)*128, yy+128)))
            m = measure(im)
            figures[e['facing'], e['frame']] = im
            poses.append(dict(facing=e['facing'], frame=e['frame'], **m,
                              height_ratio=m['height']/fm['height'],
                              upper_width_ratio=m['upper_body_mean_occupied_width']/fm['upper_body_mean_occupied_width']))
        cls, rank = divmod(index, 5)
        records.append(dict(index=index, name=NAMES[cls], outfit=rank+1, front=fm, poses=poses))
        if rank == 0:
            board = Image.new('RGB', (970, 850), '#253136')
            draw = ImageDraw.Draw(board)
            draw.text((8, 6), f'{NAMES[cls]} - rendu actuel - natif64 / zoom x2', fill='white')
        yy = 30 + rank * 162
        selected = [front] + [figures[d, f] for d, f in [(-1, 1), (-1, 4), (1, 1), (1, 4)]]
        for col, (label, im) in enumerate(zip(['Face', 'Gauche 2', 'Gauche 5', 'Droite 2', 'Droite 5'], selected)):
            xx = 8 + col * 192
            tile = im.crop((32, 32, 96, 96))
            draw.text((xx, yy), f'{rank+1}/5 {label}', fill='white')
            board.paste(tile, (xx, yy+32), tile)
            zoom = tile.resize((128, 128), Image.Resampling.NEAREST)
            board.paste(zoom, (xx+62, yy+24), zoom)
        if rank == 4:
            board.save(DOC / f'{NAMES[cls]}-audit-face-marche-native64.png')
    # One compact cross-class board for the same five armour stages.
    board = Image.new('RGB', (910, 490), '#253136'); draw = ImageDraw.Draw(board)
    for cls, name in enumerate(NAMES):
        for rank in range(5):
            tile = fronts[cls*5+rank].crop((32, 32, 96, 96))
            x, y = 12 + cls*178, 30 + rank*90
            board.paste(tile, (x, y), tile)
            zoom = tile.resize((96, 96), Image.Resampling.NEAREST)
            board.paste(zoom, (x+64, y-6), zoom)
        draw.text((12+cls*178, 8), name, fill='white')
    board.save(DOC / 'cinq-classes-face-actuelle-native64.png')
    report = {'revision': 'PLAYER-CONSISTENCY13-AUDIT-r01', 'native_tile': 64,
              'method': 'Current unarmed front bank vs complete WALK12 figures; alpha>=96 and upper band27..55 percent; occupied widths are 2D silhouettes, not physical volume or art-quality scores.',
              'runtime_changed': False, 'classes': 5, 'outfits': records,
              'source_sha256': {p.relative_to(ROOT).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(assets)}}
    (DOC / 'audit.json').write_text(json.dumps(report, indent=2)+'\n')
    for r in records:
        moving = [p for p in r['poses'] if p['frame'] != 8]
        print(json.dumps({'class':r['name'],'outfit':r['outfit'],'front_height':r['front']['height'],
              'front_upper_width':r['front']['upper_body_mean_occupied_width'],
              'walk_height_range':[min(p['height'] for p in moving),max(p['height'] for p in moving)],
              'walk_upper_width_median':round(statistics.median(p['upper_body_mean_occupied_width'] for p in moving),2),
              'walk_front_upper_width_ratio':round(statistics.median(p['upper_width_ratio'] for p in moving),3)}))


if __name__ == '__main__':
    main()
