"""Check native captures, accepted pixels, and exact exported playback duration."""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/animation-polish-01/biped-lateral-03/r01'


def main():
    checks = json.loads((OUT/'current/checks.json').read_text(encoding='utf-8'))
    assert [i['id'] for i in checks['items']] == [f'MON-{n:03}-0' for n in [145, 164, 165, 5, 14, 156, 144]]
    assert [i['accepted'] for i in checks['items']] == [False]*3+[True]*4
    assert all(s['player_phase'] == s['monster_phase'] for s in checks['synchronized_phase_checks'])
    accepted_pixels = 0
    for item in checks['items']:
        assert len(item['captures']) == 66 and item['poses_each_direction'] == [8, 8]
        for file in item['captures']:
            image = Image.open(OUT/'current'/item['id']/file).convert('RGB')
            assert image.size == (480, 192)
            if item['accepted']:
                previous = Image.open(ROOT/'artifacts/animation-polish-01/biped-lateral-02/r01/current'/item['id']/file).convert('RGB')
                assert ImageChops.difference(image.crop((0, 32, 480, 160)), previous.crop((0, 32, 480, 160))).getbbox() is None
                accepted_pixels += 1
        image = Image.open(OUT/'current'/item['id']/'native.webp')
        duration = 0
        for frame in range(image.n_frames):
            image.seek(frame); image.load(); duration += image.info['duration']
        assert duration == 2400, (item['id'], duration)
    frozen = 0
    for file in ['BIPED-LATERAL01-ACCEPTANCE-r03.json', 'BIPED-LATERAL02-ACCEPTANCE-r01.json']:
        record = json.loads((ROOT/'docs/reviews/animation-polish-01'/file).read_text(encoding='utf-8'))
        for asset in record['assets']:
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest() == asset['sha256']
            frozen += 1
    report = {'review': 'BIPED-LATERAL03-r01', 'captures': sum(len(i['captures']) for i in checks['items']),
              'synchronized_phase_samples': len(checks['synchronized_phase_checks']),
              'accepted_pixel_identical_captures': accepted_pixels, 'accepted_asset_hashes_checked': frozen,
              'webp_loop_ms': 2400, 'step_ms': 400, 'cycle_ms': 800, 'normal_game_enabled': False,
              'new_models': [145, 164, 165], 'new_weapon_contacts_accepted': False}
    (OUT/'native-validation.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print(json.dumps(report))


if __name__ == '__main__': main()
