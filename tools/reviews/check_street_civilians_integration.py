"""Verify normal runtime scenery against the exact accepted civilian review."""
from pathlib import Path
import hashlib
import json
from PIL import Image
from check_street_style_integration import changed_pixels, external_hover_border, VIEW

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-civilians-review-08'
OUT = ROOT / 'artifacts/street-defects-review-01'
ENGINE = OUT / 'engine09'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def build():
    acceptance = read(DOC / 'acceptance.json')
    capture = read(ENGINE / 'checks.json')
    assert acceptance['status'] == 'accepted' and capture['normal_integration']
    assert capture['render_read_only'] and capture['camp_round_trip']
    assert capture['review'] == acceptance['review'] == 'CIVILS-U01-08'
    active = ROOT / 'assets/maps/prison-street-expanded-01'
    for file, digest in acceptance['previous_runtime_assets'].items():
        if file != 'street.json':
            assert sha(active / file) == digest, file
    for asset in acceptance['runtime_assets']:
        assert sha(ROOT / asset['file']) == asset['sha256'], asset['file']
    old = read(DOC / 'previous-street.json')
    assert sha(DOC / 'previous-street.json') == acceptance['previous_runtime_assets']['street.json']
    current = read(active / 'street.json')
    assert current.pop('civilians_review') == acceptance['review']
    assert current.pop('scenery_props') == old['scenery_props'] + acceptance['runtime_placements']
    old.pop('scenery_props')
    assert current == old, 'Unrelated street data changed'
    differences, raw, borders = {}, {}, {}
    for item in capture['scenes']:
        label = item['id']
        approved = OUT / 'engine08' / f'{label}-candidate.png'
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
    approved = OUT / 'engine08/map-candidate.png'
    assert sha(approved) == acceptance['approved_references'][approved.name]
    differences['whole-map'] = changed_pixels(Image.open(approved), Image.open(ENGINE / 'map-normal.png'))
    assert differences['whole-map'] == 0
    for label in ['house-open', 'house-closed']:
        Image.open(ENGINE / f'{label}-normal.png').crop(VIEW).save(DOC / f'integrated-{label}-native.png')
    report = dict(review=acceptance['review'], status='integrated', date='2026-10-19',
                  native_cell=64, native_views=len(capture['scenes']),
                  normal_runtime_matches_accepted=True, changed_pixels=differences,
                  raw_changed_pixels=raw, external_ui_hover_pixels=borders,
                  render_read_only=True, camp_round_trip=True, background_unchanged=True,
                  scene_changes=['scenery_props append', 'civilians_review'],
                  collision_lighting_doors_and_other_scene_data_unchanged=True,
                  runtime_assets=acceptance['runtime_assets'], captures='artifacts/street-defects-review-01/engine09')
    (DOC / 'integration-checks.json').write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print(json.dumps(dict(integrated=True, native_views=report['native_views'], changed_pixels=differences,
                          copied_textures=len(acceptance['runtime_assets']), other_scene_data_unchanged=True)))


if __name__ == '__main__':
    build()
