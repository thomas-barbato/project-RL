"""Check review coverage and layer invariants; produce diagnostic contact sheets."""
from pathlib import Path
from collections import Counter
import json
from PIL import Image, ImageDraw, ImageFont
import build_comic_catalogue_03 as art

out = art.OUT
data = json.loads((out / 'catalogue.json').read_text('utf-8'))
old = {e['review_id']: e for e in art.ENTRIES}
report = {'review_id': 'BD-CATALOGUE03', 'checks': {}, 'scope': 'review resources only, no runtime integration'}
checks = report['checks']
assert len(data['entries']) == 178
assert sum(len(e['compositions']) for e in data['entries']) == 822
assert {e['review_id'] for e in data['entries']} == set(old)
checks['complete_catalogue'] = {'models': 178, 'compositions': 822, 'kinds': dict(Counter(e['kind'] for e in data['entries']))}
for e in data['entries']:
    for key in ['content_id', 'name', 'kind', 'family', 'hold']:
        assert e.get(key) == old[e['review_id']].get(key), (e['review_id'], key)
checks['identities_and_rules_unchanged'] = True
checks['distinct_source_drawings'] = len({art.sha(art.ROOT / e['source_path']) for e in data['entries']})
assert checks['distinct_source_drawings'] == 178
clipped = []
slot_checks = 0
for e in data['entries']:
    for key, size in [('icon32', (32, 32)), ('icon64', (64, 64))]:
        assert Image.open(out / e[key]).size == size
    for c in e['compositions']:
        native = Image.open(out / c['native32']).convert('RGBA')
        display = Image.open(out / c['display64']).convert('RGBA')
        assert native.size == (64, 64)
        assert display.size == (128, 128)
        assert Image.open(out / c['normal']).size == (512, 512)
        assert native.resize((128, 128), Image.Resampling.NEAREST).tobytes() == display.tobytes()
        assert c['registration']['uniform_scale'] is True
        bbox = art.bounds(native)
        if bbox[0] <= 0 or bbox[1] <= 0 or bbox[2] >= 64 or bbox[3] >= 64:
            clipped.append(c['native32'])
        if e['kind'] in ('head', 'hands', 'feet'):
            baseline = art.body(c['character']).resize((64, 64), Image.Resampling.LANCZOS)
            assert native.tobytes() == baseline.tobytes(), c['native32']
            slot_checks += 1
checks['exact_native32_to_display64_doubling'] = 822
checks['edge_clipping_at_alpha96'] = clipped
assert not clipped, clipped
checks['non_torso_slots_keep_whole_character_unchanged'] = slot_checks
assert slot_checks == 90
head_checks = 0
for i in range(1, 6):
    baseline = art.body(i).crop((0, 0, art.MS, 24 * art.F)).tobytes()
    for pose in ['firearm', 'pole', 'heavy']+(['sidearm'] if i in art.SIDEARM else []):
        assert baseline == art.body(i, pose).crop((0, 0, art.MS, 24 * art.F)).tobytes(), (i, pose)
        head_checks += 1
checks['fixed_upper_head_pixels_across_arm_poses'] = {'comparisons': head_checks, 'native_region': [0, 0, 64, 24]}
ref = json.loads((art.BASE / 'comic-style-reference-03.json').read_text('utf-8'))
for f in ref['files']:
    assert art.sha(art.BASE / f['path']) == f['binary_sha256']
checks['original_reference_files_unchanged'] = len(ref['files'])
for path, digest in data['active_files_before_after'].items():
    assert art.sha(art.ROOT / path) == digest
checks['active_game_rig_files_unchanged'] = len(data['active_files_before_after'])
report['limits'] = ['Visual acceptance pending.', 'Weapons shown on the five canonical complete outfits.', 'The 17 torso outfits are reviewed individually; all possible torso and weapon combinations are not shown.', 'Numerical checks do not validate artistic quality.']
(out / 'validation.json').write_text(json.dumps(report, ensure_ascii=False, indent=2), 'utf-8')

entries = {e['review_id']: e for e in data['entries']}
font = lambda n: ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', n)
samples = [('BD-W014', 2), ('BD-W022', 3), ('BD-W108', 5), ('BD-W008', 4), ('BD-EQ-LP01', 2), ('BD-W062', 2), ('BD-W068', 5), ('BD-W082', 4), ('BD-W095', 3), ('BD-W096', 5), ('BD-EQ-F01', 5), ('BD-EQ-S06', 1), ('BD-A018', 2), ('BD-A022', 3), ('BD-A032', 4)]
for offset in range(0, len(samples), 5):
    board = Image.new('RGB', (1500, 450), (14, 21, 29))
    d = ImageDraw.Draw(board)
    for col, (ident, character) in enumerate(samples[offset:offset+5]):
        e = entries[ident]
        c = next(c for c in e['compositions'] if c['character'] == character)
        x = col * 300
        d.text((x+8, 5), ident + ' · ' + c['name'], font=font(14), fill='white')
        d.text((x+8, 27), e['name'][:31], font=font(13), fill=(155, 181, 198))
        im = Image.open(out / c['normal']).convert('RGBA')
        im.thumbnail((300, 300), Image.Resampling.LANCZOS)
        board.paste(im, (x, 50), im)
        native = Image.open(out / c['display64']).convert('RGBA')
        board.paste(native, (x+70, 314), native)
    board.save(out / 'audit' / f'families-{offset//5+1:02}.png')
for family, stem in [(6, 'pistols'), (7, 'smgs')]:
    guns = [e for e in data['entries'] if e.get('family') == family]
    board = Image.new('RGB', (1300, len(guns)*270), (14, 21, 29))
    d = ImageDraw.Draw(board)
    for row, e in enumerate(guns):
        for col, c in enumerate(e['compositions']):
            x, y = col*260, row*270
            d.text((x+8, y+4), e['review_id']+' · '+c['name'], font=font(13), fill='white')
            im = Image.open(out / c['normal']).convert('RGBA')
            im.thumbnail((260, 260), Image.Resampling.LANCZOS)
            board.paste(im, (x, y+15), im)
    board.save(out / 'audit' / (stem+'-all-poses.png'))
board = Image.new('RGB', (1200, 780), (14, 21, 29))
d = ImageDraw.Draw(board)
guns = [e for e in data['entries'] if e.get('family') in (6,7)]
for n, e in enumerate(guns):
    x, y = n%4*300, n//4*260
    ob = art.raw(e['review_id']).crop(art.bounds(art.raw(e['review_id'])))
    w, h = ob.size
    u, v = art.GUN_GRIPS[int(e['review_id'][4:])]
    ob.thumbnail((280,220),Image.Resampling.LANCZOS)
    board.paste(ob,(x+10,y+30),ob)
    px, py = x+10+ob.width*u, y+30+ob.height*v
    d.ellipse((px-5,py-5,px+5,py+5),outline='#ff704f',width=2)
    d.text((x+8,y+4),e['review_id']+' · point de prise',font=font(13),fill='white')
board.save(out/'audit'/'sidearm-source-grips.png')
print(json.dumps(report, ensure_ascii=False))
