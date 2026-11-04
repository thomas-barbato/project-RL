"""Use the existing true-front idle from the preferred walking drawings."""
import copy
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'assets/source-art/animation-polish-01/biped-lateral-03/r03'


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    previous = json.loads((OUT.parent/'r02/manifest.json').read_text(encoding='utf-8'))
    data = copy.deepcopy(previous)
    data.update(review='BIPED-LATERAL03-r03', changed_models=[164, 165, 14],
                unchanged_models=[145, 5, 156, 144], walk_pixels_modified=False)
    for profile in data['profiles']:
        atlas_path = ROOT/profile['atlas']
        assert hashlib.sha256(atlas_path.read_bytes()).hexdigest() == profile['sha256']
        if profile['number'] not in [164, 165, 14]:
            continue
        atlas = Image.open(atlas_path).convert('RGBA')
        # The ninth complete figure is already a grounded frontal idle.
        # Select one identical front for arrivals from either side.
        idle = atlas.crop((8*128, 0, 9*128, 128))
        path = OUT/f'MON-{profile["number"]:03}-front.png'
        idle.save(path)
        profile.pop('compare_atlas', None)
        profile.pop('compare_sha256', None)
        profile.update(idle_sprite=path.relative_to(ROOT).as_posix(),
                       idle_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                       idle_source={'atlas': profile['atlas'], 'frame': 8, 'side': 0, 'whole_figure': True},
                       previous_front_file=profile.get('front_file'),
                       front_file=path.relative_to(ROOT).as_posix(),
                       front_preference='Use the same drawing identity as the moving figure',
                       front_accepted=False)
    (OUT/'manifest.json').write_text(json.dumps(data, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')
    print('Three existing frontal idles extracted; all seven walk atlases remain byte-identical')


if __name__ == '__main__': main()
