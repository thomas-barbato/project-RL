"""Verify that adopting a matching frontal leaves every walking pixel intact."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/animation-polish-01/biped-lateral-03/r03'
SOURCE = ROOT/'assets/source-art/animation-polish-01/biped-lateral-03'


def equal(a, b):
    return ImageChops.difference(a.convert('RGB'), b.convert('RGB')).getbbox() is None


def main():
    manifest = json.loads((SOURCE/'r03/manifest.json').read_text(encoding='utf-8'))
    previous = json.loads((SOURCE/'r02/manifest.json').read_text(encoding='utf-8'))
    checks = json.loads((OUT/'current/checks.json').read_text(encoding='utf-8'))
    assert manifest['changed_models'] == [164, 165, 14]
    assert manifest['unchanged_models'] == [145, 5, 156, 144]
    assert [i['identity_front'] for i in checks['items']] == [False, True, True, False, True, False, False]
    moving = retained = stops = legacy = 0
    for profile in manifest['profiles']:
        old = next(p for p in previous['profiles'] if p['number'] == profile['number'])
        assert profile['atlas'] == old['atlas'] and profile['sha256'] == old['sha256']
        assert hashlib.sha256((ROOT/profile['atlas']).read_bytes()).hexdigest() == profile['sha256']
        if profile['number'] in [164, 165, 14]:
            idle = Image.open(ROOT/profile['idle_sprite']).convert('RGBA')
            atlas = Image.open(ROOT/profile['atlas']).convert('RGBA')
            assert idle.tobytes() == atlas.crop((8*128, 0, 9*128, 128)).tobytes()
            assert hashlib.sha256((ROOT/profile['idle_sprite']).read_bytes()).hexdigest() == profile['idle_sha256']
    for item in checks['items']:
        assert len(item['captures']) == 66 and item['poses_each_direction'] == [8, 8]
        for file in item['captures']:
            current = Image.open(OUT/'current'/item['id']/file).convert('RGB')
            old = Image.open(OUT.parent/'r02/current'/item['id']/file).convert('RGB')
            assert current.size == (480, 192)
            if item['identity_front']:
                original = Image.open(OUT.parent/'r01/current'/item['id']/file).convert('RGB')
                assert equal(current.crop((0, 32, 160, 160)), original.crop((160, 32, 320, 160)))
                legacy += 1
                if file.endswith('-32.png'):
                    assert equal(current.crop((160, 32, 320, 160)), current.crop((320, 32, 480, 160)))
                    assert not equal(current.crop((0, 32, 160, 160)), current.crop((320, 32, 480, 160)))
                    stops += 1
                else:
                    assert equal(current.crop((320, 32, 480, 160)), old.crop((320, 32, 480, 160)))
                    moving += 1
            else:
                assert equal(current.crop((0, 32, 480, 160)), old.crop((0, 32, 480, 160)))
                retained += 1
        animation = Image.open(OUT/'current'/item['id']/'native.webp'); duration = 0
        for frame in range(animation.n_frames):
            animation.seek(frame); animation.load(); duration += animation.info['duration']
        assert duration == 2400
    board = Image.new('RGB', (640, 576), '#1e2831'); draw = ImageDraw.Draw(board)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    for row, n in enumerate([14, 164, 165]):
        root = OUT/'current'/f'MON-{n:03}-0'
        left = Image.open(root/'left-00.png').convert('RGB'); right = Image.open(root/'right-00.png').convert('RGB')
        for col, (image, source_col, title) in enumerate([(left, 0, 'Ancienne face'), (left, 1, 'Face harmonisée'), (left, 2, 'Marche gauche'), (right, 2, 'Marche droite')]):
            tile = image.crop((source_col*160, 0, (source_col+1)*160, 192))
            board.paste(tile, (col*160, row*192))
            draw.rectangle((col*160, row*192, (col+1)*160-1, row*192+31), fill='#1e2831')
            draw.text((col*160+4, row*192+3), title, font=font, fill='white')
            draw.text((col*160+4, row*192+17), f'Cultiste {n}', font=font, fill='#b6cbd2')
    board.save(OUT/'front-identity-summary.png')
    report = {'review': 'BIPED-LATERAL03-r03', 'captures': 462, 'walk_atlases_unchanged': 7,
              'cultist_moving_captures_identical': moving, 'other_model_captures_identical': retained,
              'matching_frontal_arrivals': stops, 'original_frontal_panels_identical': legacy,
              'idle_source': 'existing complete frontal frame 8, side 0, from the same atlas as the current walk',
              'step_ms': 400, 'cycle_ms': 800, 'webp_loop_ms': 2400, 'normal_game_enabled': False}
    (OUT/'native-validation.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print(json.dumps(report))


if __name__ == '__main__': main()
