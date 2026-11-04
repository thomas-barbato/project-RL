"""Check the narrow hood revision and build a native two-direction review board."""
import hashlib
import json
from PIL import Image, ImageChops, ImageDraw, ImageFont
from check_biped_armed03 import ROOT

OUT = ROOT/'artifacts/animation-polish-01/biped-lateral-03/r02'
PREVIOUS = OUT.parent/'r01'
SOURCE = ROOT/'assets/source-art/animation-polish-01/biped-lateral-03'


def main():
    checks = json.loads((OUT/'current/checks.json').read_text(encoding='utf-8'))
    manifest = json.loads((SOURCE/'r02/manifest.json').read_text(encoding='utf-8'))
    old_manifest = json.loads((SOURCE/'r01/manifest.json').read_text(encoding='utf-8'))
    assert [p['number'] for p in manifest['profiles']] == [145, 164, 165, 5, 14, 156, 144]
    assert [i['previous_walk'] for i in checks['items']] == [False, True, True, False, False, False, False]
    assert all(s['player_phase'] == s['monster_phase'] for s in checks['synchronized_phase_checks'])
    unchanged = before = 0
    for item in checks['items']:
        revised = item['id'] in ['MON-164-0', 'MON-165-0']
        assert len(item['captures']) == 66 and item['poses_each_direction'] == [8, 8]
        for file in item['captures']:
            current = Image.open(OUT/'current'/item['id']/file).convert('RGB')
            old = Image.open(PREVIOUS/'current'/item['id']/file).convert('RGB')
            assert current.size == (480, 192)
            if revised:
                assert ImageChops.difference(current.crop((160, 32, 320, 160)), old.crop((320, 32, 480, 160))).getbbox() is None
                before += 1
            else:
                assert ImageChops.difference(current.crop((0, 32, 480, 160)), old.crop((0, 32, 480, 160))).getbbox() is None
                unchanged += 1
        animation = Image.open(OUT/'current'/item['id']/'native.webp'); duration = 0
        for frame in range(animation.n_frames):
            animation.seek(frame); animation.load(); duration += animation.info['duration']
        assert duration == 2400
    frozen = 0
    for profile in manifest['profiles']:
        assert hashlib.sha256((ROOT/profile['atlas']).read_bytes()).hexdigest() == profile['sha256']
        if profile['number'] not in [164, 165]:
            old = next(p for p in old_manifest['profiles'] if p['number'] == profile['number'])
            assert profile['atlas'] == old['atlas'] and profile['sha256'] == old['sha256']
            frozen += 1
    board = Image.new('RGBA', (512, 288), '#1e2831'); draw = ImageDraw.Draw(board)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 11)
    for row, n in enumerate([164, 165]):
        for col, (rev, side) in enumerate([('r01', 0), ('r02', 0), ('r01', 1), ('r02', 1)]):
            record = next(p for p in (old_manifest if rev == 'r01' else manifest)['profiles'] if p['number'] == n)
            atlas = Image.open(ROOT/record['atlas']).convert('RGBA')
            board.alpha_composite(atlas.crop((0, side*128, 128, (side+1)*128)), (col*128, row*144+16))
            draw.text((col*128+3, row*144+2), f'{n} / {"Gauche" if side == 0 else "Droite"} / {rev}', font=font, fill='white')
    board.convert('RGB').save(OUT/'hood-direction-summary.png')
    board.resize((1024, 576), Image.Resampling.NEAREST).save(OUT/'hood-direction-summary-x2.png')
    report = {'review': 'BIPED-LATERAL03-r02', 'captures': 462, 'synchronized_phase_samples': len(checks['synchronized_phase_checks']),
              'unchanged_models': [145, 5, 14, 156, 144], 'unchanged_atlases_checked': frozen,
              'unchanged_native_captures': unchanged, 'r01_comparison_panels_pixel_identical': before,
              'step_ms': 400, 'cycle_ms': 800, 'webp_loop_ms': 2400, 'normal_game_enabled': False,
              'head_direction_visual_review': 'pending player review; inspect all 32 moving heads, not inferred from hashes'}
    (OUT/'native-validation.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print(json.dumps(report))


if __name__ == '__main__': main()
