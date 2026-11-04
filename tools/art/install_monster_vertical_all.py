"""Install the exact accepted monster poses, without resampling."""
import argparse
import json
from pathlib import Path

from PIL import Image

from prepare_monster_vertical_all import ROOT, digest, write_json
from prepare_monster_vertical_all_corrections import ACCEPTANCE, CORRECTIONS
from prepare_monster_vertical01 import REV, verify_frozen_assets

DESTINATION = ROOT / 'assets/animations/monster-vertical-01'


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def profiles():
    acceptance = read(ACCEPTANCE)
    manifest = read(CORRECTIONS / 'manifest.json')
    revised = {p['id']: p for p in manifest['profiles']}
    assert {p['id'] for p in acceptance['corrections']} == revised.keys()
    corrected = []
    for choice in acceptance['corrections']:
        p = revised[choice['id']]
        assert p['review_number'] == choice['review_number']
        assert digest(ROOT / p['atlas']) == p['sha256']
        assert digest(ROOT / p['sources'][0]) == p['source_sha256']
        assert digest(ROOT / p['idle_sprite']) == p['idle_sha256']
        corrected.append({'review_number': p['review_number'], 'id': p['id'], 'atlas': p['atlas'], 'sha256': p['sha256']})
    approval_path = REV / 'MONSTER-VERTICAL-ALL-ACCEPTANCE-r02.json'
    approval = {
        'review': manifest['review'], 'status': 'accepted', 'player_quote': 'ok je valide tout, tu peux donc tout integrer',
        'also_accepted': ACCEPTANCE.name, 'variant': 0, 'step_ms': 400, 'native_cell_px': 64,
        'directions': ['down', 'up'], 'corrections': corrected,
    }
    if approval_path.exists():
        assert read(approval_path) == approval, 'La correction approuvée a changé'
    else:
        write_json(approval_path, approval)

    for choice in acceptance['accepted']:
        path = (ROOT / choice['atlas']).with_name('manifest.json')
        source = next(p for p in read(path)['profiles'] if p['id'] == choice['id'])
        assert source['atlas'] == choice['atlas']
        revised[choice['id']] = {**source, 'review_number': choice['review_number'],
                                 'material_palette': choice['material_palette']}
    assert {p['review_number'] for p in revised.values()} == set(range(1, 130))
    roster = read(REV / 'MONSTER-VERTICAL01-COVERAGE.json')
    assert set(revised) == {entry['id'] for entry in roster['entries']}
    return sorted(revised.values(), key=lambda p: p['number'])


def prepare(p, baked):
    atlas_path = ROOT / p['atlas']
    assert digest(atlas_path) == p['sha256']
    with Image.open(atlas_path) as source:
        raw = source.convert('RGBA')
    assert raw.size == (1440, 256)
    source = raw
    if baked is not None and p.get('material_palette'):
        source = Image.frombytes('RGBA', raw.size, (baked / f'{p["id"]}.rgba').read_bytes())
        assert source.getchannel('A').tobytes() == raw.getchannel('A').tobytes()
        for row in range(2):
            rest = (1280, row * 128, 1440, (row + 1) * 128)
            assert source.crop(rest).tobytes() == raw.crop(rest).tobytes()
    frames = [source.crop((i * 160, row * 128, (i + 1) * 160, (row + 1) * 128))
              for row in range(2) for i in range(8)]
    bounds = [frame.getbbox(alpha_only=True) for frame in frames]
    assert all(bounds)
    assert all(len({frame.tobytes() for frame in frames[i:i + 8]}) == 8 for i in (0, 8))
    left = max(0, min(b[0] for b in bounds) - 1)
    top = max(0, min(b[1] for b in bounds) - 1)
    right = min(160, max(b[2] for b in bounds) + 1)
    bottom = min(128, max(b[3] for b in bounds) + 1)
    width, height = right - left, bottom - top
    output = DESTINATION / f'{p["id"]}.png'
    if baked is not None:
        atlas = Image.new('RGBA', (width * 8, height * 2))
        for i, frame in enumerate(frames):
            compact = frame.crop((left, top, right, bottom))
            atlas.paste(compact, (i % 8 * width, i // 8 * height))
        atlas.save(output, optimize=True)
        with Image.open(output) as installed:
            assert installed.tobytes() == atlas.tobytes()
    return {
        'number': p['number'], 'variant': 0, 'file': output.relative_to(ROOT).as_posix(),
        'cell': [width, height], 'anchor': [80 - left, p.get('atlas_anchor_y', 112) - top],
        'vertical_offset': p['vertical_offset'], 'casts_shadow': p.get('casts_shadow', True),
    }


def main(baked):
    frozen = verify_frozen_assets()
    selected = profiles()
    DESTINATION.mkdir(parents=True, exist_ok=True)
    entries = [prepare(p, baked) for p in selected]
    write_json(DESTINATION / 'manifest.json', {
        'format_version': 1, 'native_tile_px': 64, 'step_ms': 400, 'cycle_ms': 800, 'entries': entries,
    })
    if baked is not None:
        write_json(REV / 'MONSTER-VERTICAL-INTEGRATION01-ASSETS.json', {
            'review': 'MONSTER-VERTICAL-INTEGRATION01', 'frozen_source_hashes_verified': frozen,
            'models': [{'review_number': p['review_number'], 'source': p['atlas'], 'source_sha256': p['sha256'],
                        'material_palette': p.get('material_palette'), 'file': e['file'], 'sha256': digest(ROOT / e['file'])}
                       for p, e in zip(selected, entries)],
        })
    print(f'{len(entries)} atlases {"installed" if baked is not None else "planned"}; {frozen} frozen hashes verified')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--baked', type=Path)
    main(parser.parse_args().baked)
