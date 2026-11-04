"""Verify frozen accepted models, retained fronts, and synchronized before/after."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/animation-polish-01/biped-lateral-03/r04'


def equal(a, b):
    return ImageChops.difference(a.convert('RGB'), b.convert('RGB')).getbbox() is None


def main():
    source = ROOT/'assets/source-art/animation-polish-01/biped-lateral-03'
    manifest = json.loads((source/'r04/manifest.json').read_text(encoding='utf-8'))
    previous = json.loads((source/'r03/manifest.json').read_text(encoding='utf-8'))
    checks = json.loads((OUT/'current/checks.json').read_text(encoding='utf-8'))
    assert [i['accepted'] for i in checks['items']] == [True, False, False, True, True, True, True]
    assert [i['retained_front_review'] for i in checks['items']] == [False, True, True, False, False, False, False]
    acceptance = json.loads((ROOT/'docs/reviews/animation-polish-01/BIPED-LATERAL03-ACCEPTANCE-r03-partial.json').read_text(encoding='utf-8'))
    assets = acceptance['assets']+acceptance['front_references_retained']
    for asset in assets:
        assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest() == asset['sha256']
    for p in manifest['profiles']:
        assert hashlib.sha256((ROOT/p['atlas']).read_bytes()).hexdigest() == p['sha256']
        if p['number'] in [164, 165]:
            idle = Image.open(ROOT/p['idle_sprite']).convert('RGBA')
            atlas = Image.open(ROOT/p['atlas']).convert('RGBA')
            old = next(r for r in previous['profiles'] if r['number'] == p['number'])
            assert p['idle_sprite'] == old['idle_sprite'] and p['idle_sha256'] == old['idle_sha256']
            assert p['front_accepted'] and not p['accepted']
            for side in range(2):
                assert atlas.crop((8*128, side*128, 9*128, (side+1)*128)).tobytes() == idle.tobytes()
    unchanged = fronts = before = stops = 0
    for item in checks['items']:
        assert len(item['captures']) == 66 and item['poses_each_direction'] == [8, 8]
        for file in item['captures']:
            current = Image.open(OUT/'current'/item['id']/file).convert('RGB')
            old = Image.open(OUT.parent/'r03/current'/item['id']/file).convert('RGB')
            assert current.size == (480, 192)
            if item['retained_front_review']:
                assert equal(current.crop((0, 32, 160, 160)), old.crop((160, 32, 320, 160)))
                assert equal(current.crop((160, 32, 320, 160)), old.crop((320, 32, 480, 160)))
                fronts += 1; before += 1
                if file.endswith('-32.png'):
                    assert equal(current.crop((0, 32, 160, 160)), current.crop((320, 32, 480, 160)))
                    stops += 1
            else:
                assert equal(current.crop((0, 32, 480, 160)), old.crop((0, 32, 480, 160)))
                unchanged += 1
        animation = Image.open(OUT/'current'/item['id']/'native.webp'); duration = 0
        for frame in range(animation.n_frames):
            animation.seek(frame); animation.load(); duration += animation.info['duration']
        assert duration == 2400
    board = Image.new('RGB', (640, 384), '#1e2831'); draw = ImageDraw.Draw(board)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    for row, n in enumerate([164, 165]):
        root = OUT/'current'/f'MON-{n:03}-0'
        left = Image.open(root/'left-00.png').convert('RGB'); right = Image.open(root/'right-00.png').convert('RGB')
        for col, (image, source_col, title) in enumerate([(left, 0, 'Frontal retenu'), (left, 1, 'Marche r03'), (left, 2, 'r04 gauche'), (right, 2, 'r04 droite')]):
            board.paste(image.crop((source_col*160, 0, (source_col+1)*160, 192)), (col*160, row*192))
            draw.rectangle((col*160, row*192, (col+1)*160-1, row*192+31), fill='#1e2831')
            draw.text((col*160+4, row*192+3), title, font=font, fill='white')
            draw.text((col*160+4, row*192+17), f'Cultiste {n}', font=font, fill='#b6cbd2')
    board.save(OUT/'frontmatch-summary.png')
    report = {'review': 'BIPED-LATERAL03-r04', 'captures': 462, 'frozen_asset_hashes_checked': len(assets),
              'accepted_model_captures_identical': unchanged, 'retained_front_captures_identical': fronts,
              'r03_walk_comparison_panels_identical': before, 'retained_frontal_arrivals': stops,
              'step_ms': 400, 'cycle_ms': 800, 'webp_loop_ms': 2400, 'normal_game_enabled': False,
              'new_walks_accepted': False}
    (OUT/'native-validation.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print(json.dumps(report))


if __name__ == '__main__': main()
