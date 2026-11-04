"""Calibrate HELM05 collar motion against the painted upper torso, offline."""
from pathlib import Path
import hashlib
import json
import shutil
from PIL import Image, ImageChops, ImageStat

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'assets/source-art/animation-polish-01/helmet-worn-r01'
OUT = ROOT / 'artifacts/animation-polish-01/helmet-worn-r01/current'


def prepare():
    # Preserve the actual engine baseline once; reruns cannot replace it with
    # captures of the candidate.
    before = OUT / 'motion01/before'
    if not (before / 'worn-tier4-front-0.png').exists():
        before.mkdir(parents=True, exist_ok=True)
        for file in OUT.glob('worn-tier*-*.png'):
            shutil.copy2(file, before / file.name)
    manifest = json.loads((ROOT / 'assets/animations/vertical-18/manifest.json').read_text())
    profiles = []
    for profile in manifest['profiles']:
        outfit = profile['outfit']
        if outfit // 5 not in (0, 2, 3, 4):
            continue
        family = profile['families']['ranged']
        raw = ROOT / family['body_file']
        sheet = Image.open(raw).convert('RGBA')
        w, h = family['cell']
        frames = []
        for n in range(13):
            frame = Image.new('RGBA', (256, 256))
            frame.paste(sheet.crop((n % 6 * w, n // 6 * h, (n % 6 + 1) * w, (n // 6 + 1) * h)), tuple(family['origin']))
            frames.append(frame)
        targets, samples = [], []
        for first in (0, 6):
            # Same orientation, integer translation of a central chest patch:
            # no head, weapon, glove, leg, or silhouette bounding-box matching.
            head = family['frames'][first]['head_anchor']
            x, y = round(head[0]), round(head[1] + 11)
            patch = frames[first].crop((x - 6, y, x + 7, y + 10))
            for n in range(first, first + 6):
                choices = []
                for dy in range(-3, 4):
                    for dx in range(-2, 3):
                        other = frames[n].crop((x - 6 + dx, y + dy, x + 7 + dx, y + 10 + dy))
                        score = sum(ImageStat.Stat(ImageChops.difference(patch, other)).mean) / 4
                        choices.append((score, abs(dx) + abs(dy), dx, dy))
                error, _, dx, dy = min(choices)
                original = family['frames'][n]['head_anchor']
                target = [head[0] + dx, head[1] + 6 + dy]
                # Bound the correction, without hiding rejected fits.
                shift = [target[0] - original[0], target[1] - original[1] - 6]
                assert max(map(abs, shift)) <= 2, (profile['id'], n, shift)
                targets.append(target)
                samples.append({'frame': n, 'chest_delta': [dx, dy], 'mean_rgba_error': round(error, 3), 'head_shift': shift})
        head = family['frames'][12]['head_anchor']
        targets.append([head[0], head[1] + 6])
        profiles.append({'outfit': outfit, 'id': profile['id'], 'family': 'ranged', 'file': family['body_file'],
                         'sha256': hashlib.sha256(raw.read_bytes()).hexdigest(), 'targets': targets, 'measurements': samples})
    assert len(profiles) == 20
    data = {'review': 'HELM05-MOTION01', 'method': 'integer chest-patch registration per orientation; fixed collar offset, no scaling of anatomy',
            'profiles': profiles, 'normal_game_enabled': False}
    (SOURCE / 'HELM05-MOTION01-anchors.json').write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({'profiles': len(profiles), 'frames': len(profiles)*13, 'baseline_sheets': len(list(before.glob('worn-tier*-*.png')))}))


if __name__ == '__main__':
    prepare()
