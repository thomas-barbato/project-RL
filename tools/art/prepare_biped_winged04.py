"""Three winged bipeds keep their canonical fronts and native body sizes."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image
from prepare_class_lateral_r02 import center

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'assets/source-art/animation-polish-01/biped-lateral-04/r01'
NAMES = {39: 'Petit démon ailé', 146: 'Ailé doré', 151: 'Ailé pâle au brassard'}


def references():
    OUT.mkdir(parents=True, exist_ok=True)
    current = json.loads((ROOT/'assets/animations/vertical-18/manifest.json').read_text(encoding='utf-8'))
    coverage = json.loads((ROOT/'docs/reviews/animation-polish-01/BIPED-LATERAL01-COVERAGE.json').read_text(encoding='utf-8'))
    refs = []
    for n, name in NAMES.items():
        e = next(e for e in current['monsters'] if e['number'] == n and e['variant'] == 0)
        identity = next(e for e in coverage['entries'] if e['number'] == n)
        w, h = e['cell']; image = Image.open(ROOT/e['file']).convert('RGBA')
        front = image.crop((0, 2*h, w, 3*h))
        bounds = front.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()
        idle = Image.new('RGBA', (160, 128))
        idle.alpha_composite(front, (80-e['anchor'][0], 112-e['anchor'][1]))
        path = OUT/f'MON-{n:03}-front.png'; idle.save(path)
        figure = front.crop(bounds)
        figure.resize((figure.width*6, figure.height*6), Image.Resampling.NEAREST).save(OUT/f'MON-{n:03}-front-reference.png')
        refs.append({'id': f'MON-{n:03}-0', 'number': n, 'name': name, 'frame_width': 160,
                     'vertical_offset': e['offset'], 'reference_height': bounds[3]-bounds[1],
                     'idle_sprite': path.relative_to(ROOT).as_posix(), 'idle_sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
                     'front_source': e['file'], 'front_cell': e['cell'], 'front_anchor': e['anchor'],
                     'front_bounds': list(bounds), 'motion_identity': identity['motion'],
                     'front_accepted': True, 'accepted': False,
                     'review_headers': ['Référence du jeu', 'Frontal conservé', 'Marche proposée']})
    (OUT/'references.json').write_text(json.dumps(refs, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')
    print(json.dumps([{'number':r['number'], 'height':r['reference_height']} for r in refs]))


def main():
    profiles = []
    for r in json.loads((OUT/'references.json').read_text(encoding='utf-8')):
        idle = Image.open(ROOT/r['idle_sprite']).convert('RGBA')
        assert hashlib.sha256((ROOT/r['idle_sprite']).read_bytes()).hexdigest() == r['idle_sha256']
        atlas = Image.new('RGBA', (1440, 256)); records = []; sources = []
        for side, label in enumerate(['left', 'right']):
            path = OUT/f'MON-{r["number"]:03}-{label}-source.png'; raw = Image.open(path).convert('RGBA')
            assert raw.getchannel('A').getextrema()[0] == 0
            sources.append(path.relative_to(ROOT).as_posix())
            for phase in range(8):
                x, y = phase % 3, phase // 3
                tile = raw.crop((round(x*raw.width/3), round(y*raw.height/3), round((x+1)*raw.width/3), round((y+1)*raw.height/3)))
                box = tile.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox(); assert box
                figure = tile.crop(box); scale = r['reference_height']/figure.height
                native = figure.resize((round(figure.width*scale), r['reference_height']), Image.Resampling.NEAREST)
                xx = round(80-center(figure)*scale); yy = 112-r['reference_height']
                assert 0 <= xx and xx+native.width <= 160, (r['number'], side, phase, xx, native.size)
                pose = Image.new('RGBA', (160, 128)); pose.alpha_composite(native, (xx, yy))
                atlas.alpha_composite(pose, (phase*160, side*128))
                records.append({'side': side, 'phase': phase, 'whole_figure_scale': scale,
                                'bounds': pose.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()})
            atlas.alpha_composite(idle, (8*160, side*128))
            assert len({atlas.crop((i*160, side*128, (i+1)*160, (side+1)*128)).tobytes() for i in range(8)}) == 8
        path = OUT/f'MON-{r["number"]:03}-atlas.png'; atlas.save(path)
        atlas.resize((2880, 512), Image.Resampling.NEAREST).save(OUT/f'MON-{r["number"]:03}-poses-zoom.png')
        profiles.append({**r, 'atlas': path.relative_to(ROOT).as_posix(), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'sources': sources, 'bounds': records})
    for name in ['BIPED-LATERAL03-ACCEPTANCE-r03-partial.json', 'BIPED-LATERAL03-ACCEPTANCE-r04.json']:
        record = json.loads((ROOT/'docs/reviews/animation-polish-01'/name).read_text(encoding='utf-8'))
        for asset in record['assets']+record.get('front_references_retained', []):
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest() == asset['sha256']
    accepted = json.loads((ROOT/'assets/source-art/animation-polish-01/biped-lateral-03/r04/manifest.json').read_text(encoding='utf-8'))
    reference = next(p for p in accepted['profiles'] if p['number'] == 144)
    profiles.append({**reference, 'accepted': True})
    manifest = {'review': 'BIPED-LATERAL04-r01', 'normal_game_enabled': False, 'native_cell_px': 64,
                'panel_width': 192, 'pixel_snap_candidates': True, 'match_native_shadows': True, 'step_ms': 400, 'cycle_ms': 800,
                'phase_weights': [180, 180, 170, 120]*2, 'profiles': profiles}
    (OUT/'manifest.json').write_text(json.dumps(manifest, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')
    print('Three new winged cycles prepared; canonical fronts and seven accepted identities preserved')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--references', action='store_true'); args = parser.parse_args()
    if args.references: references()
    else: main()
