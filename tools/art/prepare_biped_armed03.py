"""Prepare native identities and whole-figure atlases for the next biped review."""
from pathlib import Path
import argparse
import hashlib
import json
from PIL import Image
from prepare_class_lateral_r02 import center

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/source-art/animation-polish-01/biped-lateral-03/r01'
NAMES = {145: 'Ailé d’obsidienne', 164: 'Cultiste au fusil', 165: 'Cultiste à la hache'}


def references():
    OUT.mkdir(parents=True, exist_ok=True)
    current = json.loads((ROOT/'assets/animations/vertical-18/manifest.json').read_text(encoding='utf-8'))
    catalog = json.loads((ROOT/'assets/sprites/campaign-monsters/manifest.json').read_text(encoding='utf-8'))
    coverage = json.loads((ROOT/'docs/reviews/animation-polish-01/BIPED-LATERAL01-COVERAGE.json').read_text(encoding='utf-8'))
    refs = []
    for n, name in NAMES.items():
        entry = next(e for e in current['monsters'] if e['number'] == n and e['variant'] == 0)
        bank = next(e for e in catalog['entries'] if e['number'] == n and e['variant'] == 0)
        identity = next(e for e in coverage['entries'] if e['number'] == n)
        w, h = entry['cell']
        sheet = Image.open(ROOT/entry['file']).convert('RGBA')
        front = sheet.crop((0, 2*h, w, 3*h))
        bounds = front.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()
        path = OUT/f'MON-{n:03}-front.png'
        front.save(path)
        front.resize((w*6, h*6), Image.Resampling.NEAREST).save(OUT/f'MON-{n:03}-reference.png')
        refs.append({'id': f'MON-{n:03}-0', 'number': n, 'name': name,
                     'front_file': path.relative_to(ROOT).as_posix(), 'front_source': entry['file'],
                     'canonical_source': bank['source'], 'reference_height': bounds[3]-bounds[1],
                     'vertical_offset': entry['offset'], 'motion_identity': identity['motion']})
    (OUT/'references.json').write_text(json.dumps(refs, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')
    print(json.dumps(refs, ensure_ascii=False))


def pack(review='BIPED-LATERAL03-r01'):
    profiles = []
    for r in json.loads((OUT/'references.json').read_text(encoding='utf-8')):
        n = r['number']; atlas = Image.new('RGBA', (1152, 256)); records = []; sources = []
        for side, name in enumerate(['left', 'right']):
            path = OUT/f'MON-{n:03}-{name}-source.png'
            raw = Image.open(path).convert('RGBA')
            assert raw.getchannel('A').getextrema()[0] == 0
            sources.append(path.relative_to(ROOT).as_posix())
            for phase in range(9):
                x, y = phase % 3, phase // 3
                tile = raw.crop((round(x*raw.width/3), round(y*raw.height/3), round((x+1)*raw.width/3), round((y+1)*raw.height/3)))
                box = tile.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()
                assert box
                figure = tile.crop(box); scale = r['reference_height']/figure.height
                native = figure.resize((round(figure.width*scale), r['reference_height']), Image.Resampling.NEAREST)
                xx = round(64-center(figure)*scale); yy = 112-r['reference_height']
                assert 0 <= xx and xx+native.width <= 128, (n, side, phase, xx, native.size)
                pose = Image.new('RGBA', (128, 128)); pose.alpha_composite(native, (xx, yy))
                atlas.alpha_composite(pose, (phase*128, side*128))
                records.append({'side': side, 'phase': phase, 'whole_figure_scale': scale,
                                'bounds': pose.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()})
        for side in range(2):
            assert len({atlas.crop((p*128, side*128, (p+1)*128, (side+1)*128)).tobytes() for p in range(8)}) == 8
        path = OUT/f'MON-{n:03}-atlas.png'; atlas.save(path)
        atlas.resize((2304, 512), Image.Resampling.NEAREST).save(OUT/f'MON-{n:03}-poses-zoom.png')
        profiles.append({**r, 'atlas': path.relative_to(ROOT).as_posix(),
                         'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
                         'sources': sources, 'bounds': records, 'accepted': False})
    for review in ['BIPED-LATERAL01-ACCEPTANCE-r03.json', 'BIPED-LATERAL02-ACCEPTANCE-r01.json']:
        accepted = json.loads((ROOT/'docs/reviews/animation-polish-01'/review).read_text(encoding='utf-8'))
        for asset in accepted['assets']:
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest() == asset['sha256']
    accepted = json.loads((ROOT/'assets/source-art/animation-polish-01/biped-lateral-02/r01/manifest.json').read_text(encoding='utf-8'))
    profiles.extend({**p, 'accepted': True} for p in accepted['profiles'])
    manifest = {'review': review, 'normal_game_enabled': False, 'native_cell_px': 64,
                'frame': [128, 128], 'anchor': [64, 112], 'step_ms': 400, 'cycle_ms': 800,
                'phase_weights': [180, 180, 170, 120]*2, 'phase_ms_integer_allocation': [111, 111, 104, 74]*2,
                'reference': 'assets/source-art/animation-polish-01/class-lateral-02/r02/bastion-atlas.png',
                'profiles': profiles}
    (OUT/'manifest.json').write_text(json.dumps(manifest, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')
    print(f'Packed {len(profiles)-4} new whole-figure cycles; four accepted atlases reused unchanged')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--references', action='store_true'); args = parser.parse_args()
    if args.references: references()
    else: pack()
