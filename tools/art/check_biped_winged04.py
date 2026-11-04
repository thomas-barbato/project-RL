"""Check preserved canonical fronts, accepted drawings and native presentation."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/animation-polish-01/biped-lateral-04/r01'


def equal(a, b):
    return ImageChops.difference(a.convert('RGB'), b.convert('RGB')).getbbox() is None


def main():
    manifest = json.loads((ROOT/'assets/source-art/animation-polish-01/biped-lateral-04/r01/manifest.json').read_text(encoding='utf-8'))
    checks = json.loads((OUT/'current/checks.json').read_text(encoding='utf-8'))
    assert checks['panel_width'] == 192 and checks['step_ms'] == 400 and checks['cycle_ms'] == 800
    assert [i['accepted'] for i in checks['items']] == [False, False, False, True]
    assets = []
    for name in ['BIPED-LATERAL03-ACCEPTANCE-r03-partial.json', 'BIPED-LATERAL03-ACCEPTANCE-r04.json']:
        record = json.loads((ROOT/'docs/reviews/animation-polish-01'/name).read_text(encoding='utf-8'))
        assets += record['assets']+record.get('front_references_retained', [])
    for asset in assets:
        assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest() == asset['sha256']
    for p in manifest['profiles']:
        assert hashlib.sha256((ROOT/p['atlas']).read_bytes()).hexdigest() == p['sha256']
        if p['number'] == 144: continue
        idle = Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        w, h = p['front_cell']; front = Image.open(ROOT/p['front_source']).convert('RGBA').crop((0, 2*h, w, 3*h))
        expected = Image.new('RGBA', (160, 128)); expected.alpha_composite(front, (80-p['front_anchor'][0], 112-p['front_anchor'][1]))
        assert expected.tobytes() == idle.tobytes()
        atlas = Image.open(ROOT/p['atlas']).convert('RGBA')
        assert atlas.size == (1440, 256)
        for side in range(2):
            assert atlas.crop((8*160, side*128, 9*160, (side+1)*128)).tobytes() == idle.tobytes()
    unchanged = stops = fronts = 0
    for item in checks['items']:
        assert len(item['captures']) == 66 and item['poses_each_direction'] == [8, 8]
        for file in item['captures']:
            current = Image.open(OUT/'current'/item['id']/file).convert('RGB'); assert current.size == (576, 192)
            if item['accepted']:
                old = Image.open(OUT.parents[1]/'biped-lateral-03/r04/current'/item['id']/file).convert('RGB')
                for col in range(3):
                    assert equal(current.crop((col*192+16, 32, col*192+176, 192)), old.crop((col*160, 32, (col+1)*160, 192))), (item['id'], file, col)
                unchanged += 1
            else:
                assert equal(current.crop((0, 32, 192, 192)), current.crop((192, 32, 384, 192))), (item['id'], file, 'frontal')
                fronts += 1
                if file.endswith('-32.png'):
                    assert equal(current.crop((192, 32, 384, 192)), current.crop((384, 32, 576, 192)))
                    stops += 1
        animation = Image.open(OUT/'current'/item['id']/'native.webp'); duration = 0
        for frame in range(animation.n_frames):
            animation.seek(frame); animation.load(); duration += animation.info['duration']
        assert duration == 2400
    board = Image.new('RGB', (768, 576), '#1e2831'); draw = ImageDraw.Draw(board)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    for row, n in enumerate([39, 146, 151]):
        root = OUT/'current'/f'MON-{n:03}-0'
        left = Image.open(root/'left-00.png').convert('RGB'); right = Image.open(root/'right-00.png').convert('RGB')
        for col, (image, source_col, title) in enumerate([(left, 0, 'Référence du jeu'), (left, 1, 'Frontal conservé'), (left, 2, 'Marche gauche'), (right, 2, 'Marche droite')]):
            board.paste(image.crop((source_col*192, 0, (source_col+1)*192, 192)), (col*192, row*192))
            draw.rectangle((col*192, row*192, (col+1)*192-1, row*192+31), fill='#1e2831')
            draw.text((col*192+4, row*192+3), title, font=font, fill='white')
            draw.text((col*192+4, row*192+17), f'{row+1:02} · MON-{n:03}', font=font, fill='#b6cbd2')
    board.save(OUT/'winged-summary.png')
    report = {'review':'BIPED-LATERAL04-r01', 'captures':264, 'frozen_asset_hashes_checked':len(assets),
              'accepted_reference_captures_identical':unchanged, 'canonical_front_captures_identical':fronts,
              'canonical_frontal_arrivals':stops, 'native_board':[576,192], 'native_pair':[384,192],
              'step_ms':400,'cycle_ms':800,'webp_loop_ms':2400,'normal_game_enabled':False,'new_walks_accepted':False}
    (OUT/'native-validation.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print(json.dumps(report))


if __name__ == '__main__': main()
