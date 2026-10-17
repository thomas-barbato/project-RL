"""Validate native review composition and preservation; never changes game data."""
import ast
import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'


def read(name):
    return json.loads((REVIEW/name).read_text(encoding='utf-8'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--review', help='Batch manifest; defaults to the latest catalogued batch.')
    args = parser.parse_args()
    listing = read('monster-candidate-list.json')
    batch_name = args.review or listing.get('latest_batch', 'cultists-zombies-spirits-24.json')
    new = read(batch_name)
    suffix = new['review_id'].rsplit('-', 1)[-1]
    expected_count = new.get('expected_catalogue_count', 110 if suffix == '25' else 102)
    expected_new_numbers = set(new.get('expected_new_numbers', range(132, 140) if suffix == '25' else range(116, 132)))
    layout = read(listing['catalogue_layout'])
    main_review = read('review.json')
    numbers = [e['number'] for e in listing['records']]
    by_number = {e['number']: e for e in listing['records']}
    assert len(numbers) == len(set(numbers)) == listing['candidate_count'] == expected_count
    assert len({e['code'] for e in listing['records']}) == expected_count
    assert set(range(60, 116))-{94, 95, 97, 110, 114} <= set(numbers)
    assert set(range(116, 132)) <= set(numbers)
    assert expected_new_numbers <= set(numbers)
    assert {13, 14, 15, 16, 17, 22, 23, 24, 25, 28} <= set(numbers)
    assert not set(numbers) & {94, 95, 97, 110, 114}
    assert main_review['catalogued_monster_numbers'] == numbers
    assert main_review['pending_monster_candidate_numbers'] == [
        e['number'] for e in listing['records'] if not e['visual_approved']]
    assert sum(e['visual_approved'] for e in listing['records']) == 13
    assert all(not e['runtime_connected'] for e in listing['records'])
    assert all(e['code'] == f"{e['id']}-{e['revision']}" for e in listing['records'])
    for e in listing['records']:
        assert sha(ROOT/e['source']) == e['source_sha256']
        for v in e['variants']:
            assert sha(ROOT/v['file']) == v['sha256']
            assert v['cell'] == 64
        assert sha(ROOT/e['contact_shadow']['file']) == e['contact_shadow']['sha256']

    snapshot_name = new.get('preservation_snapshot', 'preservation-necromancers-25.json' if suffix == '25' else None)
    snapshot = read(snapshot_name) if snapshot_name else None
    preserved = snapshot['files'] if snapshot else read('preservation-catalogue-24.json')
    allowed_updates = {listing['catalogue_image']} if snapshot else {(REVIEW/'review.json').relative_to(ROOT).as_posix()}
    mismatches = [name for name, expected in preserved.items()
                  if name not in allowed_updates and (not (ROOT/name).is_file() or sha(ROOT/name) != expected)]
    assert not mismatches, mismatches
    if snapshot:
        for previous in snapshot['listing_records']:
            current = by_number[previous['number']]
            for key in ['code', 'source_sha256', 'native_body_size', 'visual_approved', 'scale_approved', 'name_approved', 'rules_approved']:
                assert current[key] == previous[key], (previous['number'], key)
            assert [v['sha256'] for v in current['variants']] == [v['sha256'] for v in previous['variants']]
    registry = read('demons-style-20-registry-preservation.json')
    records_sha = hashlib.sha256(json.dumps(main_review['records'], sort_keys=True).encode()).hexdigest()
    assert records_sha == registry['records_sha256']
    assert len(main_review['records']) == registry['record_count'] == 18
    assert sum(e.get('visual_approved', False) for e in main_review['records']) == registry['accepted_count'] == 17

    assert len(new['records']) == new['generated_source_count'] == len(expected_new_numbers)
    assert {e['number'] for e in new['records']} == expected_new_numbers
    native_variants = 0
    for e in new['records']:
        assert not any(e[k] for k in ['visual_approved', 'scale_approved', 'name_approved', 'rules_approved'])
        assert sha(ROOT/e['source']) == sha(Path(e['source_generated'])) == e['source_sha256']
        assert len(e['variants']) == 2
        variants = []
        for v in e['variants']:
            art = Image.open(ROOT/v['file']).convert('RGBA')
            assert sha(ROOT/v['file']) == v['sha256']
            assert art.size == (128, 128) and v['foot_anchor'] == [64, 96]
            pixels = list(art.get_flattened_data())
            assert {p[3] for p in pixels} <= {0, 255}
            assert len({p[:3] for p in pixels if p[3]}) <= e['palette_limit']
            bounds = art.getbbox()
            assert list(bounds) == v['occupied']
            assert bounds[2]-bounds[0] <= e['maximum_body_width64']
            assert bounds[3]-bounds[1] <= e['maximum_body_height64']
            assert [bounds[2]-bounds[0], bounds[3]-bounds[1]] == by_number[e['number']]['native_body_size']
            variants.append(pixels)
            native_variants += 1
        assert [p[3] for p in variants[0]] == [p[3] for p in variants[1]]
        # A single indexed color maps to exactly one B color; contours stay dark.
        mapping = {}
        for a, b in zip(*variants):
            if not a[3]:
                continue
            assert mapping.setdefault(a, b) == b
            if max(a[:3]) < 48:
                assert a == b
        shadow = e['contact_shadow']
        assert sha(ROOT/shadow['file']) == shadow['sha256']
        assert Image.open(ROOT/shadow['file']).size == (128, 128)

    image = Image.open(ROOT/layout['image']).convert('RGB')
    assert list(image.size) == layout['size'] and image.width == 920
    assert sha(ROOT/layout['image']) == layout['image_sha256']
    assert layout['actor_resize_operations'] == 0 and layout['scale'] == 1
    checked_pixels = 0
    for placement in layout['placements']:
        source = Image.open(ROOT/placement['file']).convert('RGBA')
        crop = source.crop(placement['source_crop'])
        assert list(crop.size) == placement['size'] and placement['scale'] == 1
        assert sha(ROOT/placement['file']) == placement['sha256']
        x, y = placement['position']
        assert x >= 0 and y >= 0 and x+crop.width <= image.width and y+crop.height <= image.height
        target = image.crop((x, y, x+crop.width, y+crop.height))
        for a, b in zip(crop.get_flattened_data(), target.get_flattened_data()):
            if a[3] == 255:
                assert a[:3] == b, (placement['number'], placement['variant'])
                checked_pixels += 1
    assert len(layout['placements']) == sum(len(e['variants']) for e in listing['records'])
    assert by_number[18]['native_body_size'][1] == 192
    assert by_number[19]['native_body_size'][1] == 224
    assert not by_number[19]['scale_approved']
    for ref in [new['style_reference'], new['player_reference']]:
        assert sha(ROOT/ref['source']) == ref['sha256']
    if new.get('combined_preview'):
        combined = Image.open(ROOT/new['combined_preview']).convert('RGB')
        y = 0
        for name in new['preview_sheets']:
            board = Image.open(ROOT/name).convert('RGB')
            assert combined.crop((0, y, board.width, y+board.height)).tobytes() == board.tobytes()
            y += board.height
        assert combined.size == (920, y)
    if new.get('native_floor_review'):
        floors = Image.open(ROOT/new['native_floor_review']).convert('RGB')
        assert floors.size == (920, 1748)
        for panel in range(3):
            for i, e in enumerate(new['records']):
                art = Image.open(ROOT/e['variants'][0]['file']).convert('RGBA')
                anchor = e['variants'][0]['foot_anchor']
                x = 12+(i%6)*128+64-anchor[0]
                y = panel*582+46+(i//6)*128+112-anchor[1]
                target = floors.crop((x, y, x+art.width, y+art.height))
                for a, b in zip(art.get_flattened_data(), target.get_flattened_data()):
                    if a[3] == 255:
                        assert a[:3] == b, (panel, e['number'])

    removed = read('removed-monsters-23.json')
    deleted_paths = [f['path'] for e in removed['records'] for f in e['files']]
    deleted_paths += [f['path'] for f in removed['obsolete_previews']]
    assert len(deleted_paths) == 18
    assert not any(Path(path).exists() for path in deleted_paths)
    for script in ['show_monster_catalogue.py', 'show_demon_gallery_review.py', 'validate_monster_catalogue.py']:
        ast.parse((ROOT/'tools/art'/script).read_text(encoding='utf-8'))
    assert 'width:920px;max-width:none' in (ROOT/listing['catalogue_viewer']).read_text(encoding='utf-8')
    md = (REVIEW/'CANDIDATS-MONSTRES.md').read_text(encoding='utf-8')
    assert f"]({(ROOT/layout['image']).as_posix()})" in md
    assert md.count('\n| ') == expected_count+2
    result = dict(review_id=f'CATALOGUE-{suffix}-VALIDATION', status='passed', count=expected_count,
        restored_cultists=[13, 14, 15, 16, 17, 22, 23, 24], new_sources=len(new['records']),
        native_variants=native_variants, separate_new_shadows=len(new['records']),
        body_placements=len(layout['placements']), exact_opaque_pixels=checked_pixels,
        image_size=list(image.size), native_scale=1, actor_resize_operations=0,
        visually_approved_count=13, pending_visual_count=expected_count-13,
        preserved_files=len(preserved)-len(allowed_updates), preserved_rust_sources=sum(n.startswith('src/') and n.endswith('.rs') for n in preserved),
        original_registry_records_preserved=18, original_registry_approved_count=17,
        intentionally_updated_registry_fields='Gallery pointers only; original records hash unchanged.',
        previously_deleted_files_still_absent=18,
        visual_inspection=['New native boards', 'Catalogue native placements'],
        runtime_changed=False, rust_tests_run=False)
    (REVIEW/f'catalogue-{suffix}-validation.json').write_text(json.dumps(result, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print(json.dumps(result, ensure_ascii=True))


if __name__ == '__main__':
    main()
