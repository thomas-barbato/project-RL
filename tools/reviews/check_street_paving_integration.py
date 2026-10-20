"""Compare the normal runtime floor with the exact visually accepted GPU frames."""
from pathlib import Path
import hashlib
import json
from PIL import Image
from check_street_style_integration import changed_pixels, external_hover_border, VIEW

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-paving-review-06'
OUT = ROOT / 'artifacts/street-defects-review-01'
ENGINE = OUT / 'engine07'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()


def build():
    acceptance = json.loads((DOC / 'acceptance.json').read_text(encoding='utf-8'))
    capture = json.loads((ENGINE / 'checks.json').read_text(encoding='utf-8'))
    assert acceptance['status'] == 'accepted'
    assert capture['review'] == acceptance['review'] == 'PAV-U01-06'
    assert capture['normal_capture'] and capture['normal_integration']
    assert capture['render_read_only'] and capture['camp_round_trip']
    asset = acceptance['runtime_background']
    assert sha(ROOT / asset['file']) == asset['sha256']
    active = ROOT / 'assets/maps/prison-street-expanded-01'
    unchanged = {}
    for file, digest in acceptance['previous_runtime_assets'].items():
        if file in ['street.png', 'street.json']:
            continue
        unchanged[file] = sha(active / file) == digest
        assert unchanged[file], file
    # Reconstruct the previous JSON exactly: this proves that no collisions,
    # lights, actors, doors or other scene fields slipped into the integration.
    scene = json.loads((active / 'street.json').read_text(encoding='utf-8'))
    assert scene.pop('paving_review') == acceptance['review']
    assert scene['background_sha256'] == asset['sha256']
    scene['background_sha256'] = asset['previous_sha256']
    old_text = json.dumps(scene, ensure_ascii=False, indent=2) + '\n'
    variants = [old_text.encode(), old_text.replace('\n', '\r\n').encode()]
    assert acceptance['previous_runtime_assets']['street.json'] in [hashlib.sha256(b).hexdigest() for b in variants], 'Unexpected scene metadata changes'
    differences, raw, borders = {}, {}, {}
    for item in capture['scenes']:
        label = item['id']
        approved = OUT / 'engine06' / f'{label}-candidate.png'
        assert sha(approved) == acceptance['approved_references'][approved.name]
        expected = Image.open(approved).crop(VIEW)
        actual = Image.open(ENGINE / f'{label}-normal.png').crop(VIEW)
        count = changed_pixels(expected, actual)
        raw[label] = count
        if count and external_hover_border(expected, actual, count):
            borders[label] = count
            count = 0
        differences[label] = count
        assert count == 0, (label, count)
    approved_map = OUT / 'engine06/map-candidate.png'
    assert sha(approved_map) == acceptance['approved_references'][approved_map.name]
    differences['whole-map'] = changed_pixels(Image.open(approved_map), Image.open(ENGINE / 'map-normal.png'))
    assert differences['whole-map'] == 0
    # Retain one useful native runtime reference; the full accepted map remains
    # on the current review page and all source drawings remain in assets/.
    Image.open(ENGINE / 'middle-street-normal.png').crop(VIEW).save(DOC / 'integrated-native.png')
    report = dict(review=acceptance['review'], status='integrated', date='2026-10-19',
                  native_cell=64, native_views=len(capture['scenes']), map_pixels=[1664, 4224],
                  normal_runtime_matches_accepted=True, changed_pixels=differences,
                  raw_changed_pixels=raw, external_ui_hover_pixels=borders,
                  render_read_only=True, camp_round_trip=True,
                  unchanged_runtime_assets=unchanged, scene_changes=['background_sha256', 'paving_review'],
                  runtime_background=asset, captures='artifacts/street-defects-review-01/engine07')
    (DOC / 'integration-checks.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(integrated=True, native_views=report['native_views'], changed_pixels=differences,
                          unchanged_assets=len(unchanged), scene_fields_verified=True)))


if __name__ == '__main__':
    build()
