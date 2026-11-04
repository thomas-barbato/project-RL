"""Redraw only the two armed walks around their retained r03 front identity."""
import argparse
import copy
import hashlib
import json
from pathlib import Path
from PIL import Image
from prepare_class_lateral_r02 import center

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'assets/source-art/animation-polish-01/biped-lateral-03/r04'
BASE = OUT.parent/'r03/manifest.json'


def references():
    OUT.mkdir(parents=True, exist_ok=True)
    manifest = json.loads(BASE.read_text(encoding='utf-8'))
    for p in manifest['profiles']:
        if p['number'] not in [164, 165]: continue
        image = Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        box = image.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()
        figure = image.crop(box)
        board = Image.new('RGBA', (512, 512))
        zoom = figure.resize((figure.width*6, figure.height*6), Image.Resampling.NEAREST)
        board.alpha_composite(zoom, ((512-zoom.width)//2, 480-zoom.height))
        board.save(OUT/f'MON-{p["number"]:03}-retained-front-reference.png')
    print('Retained r03 fronts prepared as the only appearance references')


def main():
    references()
    accepted = json.loads((ROOT/'docs/reviews/animation-polish-01/BIPED-LATERAL03-ACCEPTANCE-r03-partial.json').read_text(encoding='utf-8'))
    for asset in accepted['assets']+accepted['front_references_retained']:
        assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest() == asset['sha256']
    baseline = json.loads(BASE.read_text(encoding='utf-8'))
    manifest = copy.deepcopy(baseline)
    for p in manifest['profiles']:
        if p['number'] not in [164, 165]:
            p['accepted'] = True
            if p.get('idle_sprite'): p['front_accepted'] = True
            continue
        old_atlas, old_hash = p['atlas'], p['sha256']
        idle = Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        assert hashlib.sha256((ROOT/p['idle_sprite']).read_bytes()).hexdigest() == p['idle_sha256']
        height = idle.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()
        height = height[3]-height[1]
        atlas = Image.new('RGBA', (1152, 256)); records = []; sources = []
        for side, label in enumerate(['left', 'right']):
            path = OUT/f'MON-{p["number"]:03}-{label}-source.png'
            raw = Image.open(path).convert('RGBA')
            assert raw.getchannel('A').getextrema()[0] == 0
            sources.append(path.relative_to(ROOT).as_posix())
            for phase in range(8):
                x, y = phase % 3, phase // 3
                tile = raw.crop((round(x*raw.width/3), round(y*raw.height/3), round((x+1)*raw.width/3), round((y+1)*raw.height/3)))
                box = tile.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()
                assert box
                figure = tile.crop(box); scale = height/figure.height
                native = figure.resize((round(figure.width*scale), height), Image.Resampling.NEAREST)
                xx = round(64-center(figure)*scale); yy = 112-height
                assert 0 <= xx and xx+native.width <= 128, (p['number'], side, phase, native.size)
                pose = Image.new('RGBA', (128, 128)); pose.alpha_composite(native, (xx, yy))
                atlas.alpha_composite(pose, (phase*128, side*128))
                records.append({'side': side, 'phase': phase, 'whole_figure_scale': scale, 'bounds': pose.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()})
            # Never replace the preferred front by a newly generated idle.
            atlas.alpha_composite(idle, (8*128, side*128))
            assert atlas.crop((8*128, side*128, 9*128, (side+1)*128)).tobytes() == idle.tobytes()
            assert len({atlas.crop((i*128, side*128, (i+1)*128, (side+1)*128)).tobytes() for i in range(8)}) == 8
        path = OUT/f'MON-{p["number"]:03}-atlas.png'; atlas.save(path)
        atlas.resize((2304, 512), Image.Resampling.NEAREST).save(OUT/f'MON-{p["number"]:03}-poses-zoom.png')
        p.update(atlas=path.relative_to(ROOT).as_posix(), sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                 sources=sources, bounds=records, compare_atlas=old_atlas, compare_sha256=old_hash,
                 accepted=False, front_accepted=True, retained_front_review=True,
                 front_preference='r03 frontal is retained exactly and defines every moving appearance')
    manifest.update(review='BIPED-LATERAL03-r04', changed_models=[164, 165], unchanged_models=[145, 5, 14, 156, 144],
                    accepted_models=[145, 5, 14, 156, 144], walk_pixels_modified=True,
                    acceptance='docs/reviews/animation-polish-01/BIPED-LATERAL03-ACCEPTANCE-r03-partial.json')
    for p in manifest['profiles']:
        assert hashlib.sha256((ROOT/p['atlas']).read_bytes()).hexdigest() == p['sha256']
    (OUT/'manifest.json').write_text(json.dumps(manifest, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')
    print('Two walk proposals packed; preferred fronts and five validated models retained')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--references', action='store_true'); args = parser.parse_args()
    if args.references: references()
    else: main()
