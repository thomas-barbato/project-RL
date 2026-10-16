"""108 / D56-BODY01: direct64 export and isolated native review packaging."""
import argparse
import hashlib
import json
import re
import shutil
from pathlib import Path

from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-97'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(name, value):
    (REVIEW / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def uniform_blocks(im):
    count = same = 0
    for y in range(0, im.height - 1, 2):
        for x in range(0, im.width - 1, 2):
            pixels = [im.getpixel((x + dx, y + dy)) for dx, dy in [(0, 0), (1, 0), (0, 1), (1, 1)]]
            if any(p[3] >= 8 for p in pixels):
                count += 1
                same += int(all(p == pixels[0] for p in pixels))
    return {'occupied': count, 'uniform': same, 'uniform_percent': round(100 * same / max(1, count), 2)}


def export(generated):
    assert not (REVIEW / 'preservation.json').exists(), 'Use a new candidate instead of replacing the current proposal.'
    preserved = list(json.loads((BASE / 'skill-animation-review-96/preservation.json').read_text(encoding='utf-8')))
    preserved += [
        'assets/sprites/bestiary/tiles/056-living-32.png',
        'assets/sprites/bestiary/tiles/056-corpse-32.png',
        'assets/skills/agent/D56-compact.png',
        'src/agent_skill_lab_review/fire_painted/drone_support.rs',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-96/D58-BODY01-native128.png',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-96/visual-acceptance-01.json',
    ]
    write('preservation.json', {p: sha(ROOT / p) for p in preserved})
    source_path = REVIEW / 'D56-BODY01-source.png'
    shutil.copyfile(generated, source_path)
    source = Image.open(source_path).convert('RGBA')
    bounds = source.getchannel('A').point(lambda a: 255 if a >= 8 else 0).getbbox()
    assert bounds and source.getpixel((0, 0))[3] == 0
    crop = source.crop(bounds)
    width = 56
    height = round(crop.height * width / crop.width)
    assert 28 <= height <= 34, (width, height)
    body = crop.resize((width, height), Image.Resampling.LANCZOS)
    native = Image.new('RGBA', (128, 128))
    native.alpha_composite(body, (36, 82 - height))
    dest = REVIEW / 'D56-BODY01-native128.png'
    native.save(dest)
    old = Image.open(ROOT / 'assets/sprites/bestiary/tiles/056-living-32.png').convert('RGBA')
    old64 = old.resize((64, 64), Image.Resampling.NEAREST)
    old_blocks = uniform_blocks(old64)
    new_blocks = uniform_blocks(native)
    assert old_blocks['uniform_percent'] == 100
    assert new_blocks['uniform_percent'] < 25
    write('export-verification.json', {
        'candidate': 'D56-BODY01', 'number': 108, 'status': 'visual_proposal_not_accepted',
        'source_size': source.size, 'source_alpha8_bounds': bounds,
        'logical_tile': [64, 64], 'native_canvas': [128, 128], 'body_size': [width, height],
        'body_alpha8_bounds': native.getchannel('A').point(lambda a: 255 if a >= 8 else 0).getbbox(),
        'direct_high_resolution_reduction': True, 'no_nonuniform_stretch': True,
        'previous_repeated_2x2_blocks': old_blocks, 'new_repeated_2x2_blocks': new_blocks,
        'source_sha256': sha(source_path), 'native_sha256': sha(dest),
        'hovering_body_bottom_offset_from_cell_center': 18,
        'shadow': 'Separate scene layer below the hovering body; none baked into the new source.',
        'active_assets_or_accepted_animations_changed': False,
    })
    print(json.dumps({'body': [width, height], 'source_size': source.size, 'uniform_blocks': new_blocks}))


def package():
    preservation = json.loads((REVIEW / 'preservation.json').read_text(encoding='utf-8'))
    for p, expected in preservation.items():
        assert sha(ROOT / p) == expected, p
    before = Image.open(REVIEW / '_capture/scene-before.png').convert('RGB')
    after = Image.open(REVIEW / '_capture/scene-after.png').convert('RGB')
    assert before.size == after.size == (576, 576)
    bounds = ImageChops.difference(before, after).getbbox()
    assert bounds and bounds[0] >= 55 and bounds[1] >= 250 and bounds[2] <= 137 and bounds[3] <= 330, bounds
    before.save(REVIEW / 'scene-before.png')
    after.save(REVIEW / 'scene-after.png')
    template = (ROOT / 'tools/art/scout_body_review_template.html').read_text(encoding='utf-8')
    for p in re.findall(r'(?:src|href)="([^"]+)"', template):
        assert (BASE / p).is_file(), p
    (BASE / 'apercu-animations.html').write_text(template, encoding='utf-8')
    export_data = json.loads((REVIEW / 'export-verification.json').read_text(encoding='utf-8'))
    write('review.json', {
        'date': '2026-10-15', 'review_id': 'D56-RES64-01', 'number': 108, 'candidate': 'D56-BODY01',
        'status': 'visual_proposal_not_accepted', 'user_authorization': "ok on va l'améliorer :)",
        'scope': 'Redraw and present the existing Scout at direct64 resolution, pending visual approval.',
        'source': 'D56-BODY01-source.png', 'native': 'D56-BODY01-native128.png', 'prompt': 'prompt.json',
        'logical_tile': [64, 64], 'body_size': export_data['body_size'], 'canvas': [128, 128],
        'retained_identity': ['Compact hovering scout', 'Frontal rounded ivory sensor shell',
            'Dark visor with blue optic', 'Two ivory side thruster pods', 'Graphite underbody',
            'Unarmed reconnaissance role and content ID D56'],
        'new_art_to_approve': ['Source drawing', 'Direct64 export and shown static composition'],
        'campaign_bank_skill_icon_corpse_or_accepted_animations_changed': False,
        'source_sha256': export_data['source_sha256'], 'native_sha256': export_data['native_sha256'],
    })
    write('validation.json', {
        'native_scenes': 2, 'native_tile': 64, 'only_scout_region_differs': True,
        'changed_bounds': bounds, 'preserved_files': len(preservation),
        'scene_before_sha256': sha(REVIEW / 'scene-before.png'),
        'scene_after_sha256': sha(REVIEW / 'scene-after.png'),
        'current_page_paths_exist': True,
        'method': 'Native Macroquad captures plus offline pixel checks; no browser-render verification.',
    })
    print(json.dumps({'scenes': 2, 'changed_bounds': bounds, 'preserved': len(preservation)}))


def export_size02():
    """A larger direct reduction, preserving the existing source and first export."""
    source_path = REVIEW / 'D56-BODY01-source.png'
    source = Image.open(source_path).convert('RGBA')
    bounds = source.getchannel('A').point(lambda a: 255 if a >= 8 else 0).getbbox()
    crop = source.crop(bounds)
    width = 70
    height = round(crop.height * width / crop.width)
    assert height == 40
    native = Image.new('RGBA', (128, 128))
    native.alpha_composite(crop.resize((width, height), Image.Resampling.LANCZOS), (29, 82 - height))
    dest = REVIEW / 'D56-BODY01-SIZE02-native128.png'
    assert not dest.exists(), 'Do not overwrite an existing candidate.'
    native.save(dest)
    write('size02-export-verification.json', {
        'candidate': 'D56-BODY01-SIZE02', 'number': 108, 'status': 'visual_proposal_not_accepted',
        'source': 'D56-BODY01-source.png', 'source_sha256': sha(source_path),
        'native_sha256': sha(dest), 'logical_tile': [64, 64], 'native_canvas': [128, 128],
        'body_size': [width, height], 'previous_body_size': [56, 32],
        'body_alpha8_bounds': native.getchannel('A').point(lambda a: 255 if a >= 8 else 0).getbbox(),
        'direct_high_resolution_reduction': True, 'not_upscaled_from_first_export': True,
        'no_nonuniform_stretch': True, 'horizontal_overhang_each_side': 3,
        'rendered_body_pixel_area_increase_percent': 56.25,
        'same_hovering_body_bottom': True, 'new_repeated_2x2_blocks': uniform_blocks(native),
        'campaign_bank_skill_icon_corpse_or_accepted_animations_changed': False,
    })
    print(json.dumps({'body': [width, height], 'native': dest.name}))


def package_size02():
    preservation = json.loads((REVIEW / 'preservation.json').read_text(encoding='utf-8'))
    for p, expected in preservation.items():
        assert sha(ROOT / p) == expected, p
    before = Image.open(REVIEW / '_capture-size02/scene-before.png').convert('RGB')
    after = Image.open(REVIEW / '_capture-size02/scene-after.png').convert('RGB')
    assert before.size == after.size == (576, 576)
    first_proposal = Image.open(REVIEW / 'scene-after.png').convert('RGB')
    assert not ImageChops.difference(before, first_proposal).getbbox(), 'Before must be the actual previous proposal.'
    bounds = ImageChops.difference(before, after).getbbox()
    assert bounds and bounds[0] >= 55 and bounds[1] >= 250 and bounds[2] <= 137 and bounds[3] <= 330, bounds
    after.save(REVIEW / 'scene-size02.png')
    template = (ROOT / 'tools/art/scout_body_review_template.html').read_text(encoding='utf-8')
    for p in re.findall(r'(?:src|href)="([^"]+)"', template):
        assert (BASE / p).is_file(), p
    (BASE / 'apercu-animations.html').write_text(template, encoding='utf-8')
    export_data = json.loads((REVIEW / 'size02-export-verification.json').read_text(encoding='utf-8'))
    write('size02-review.json', {
        'date': '2026-10-15', 'review_id': 'D56-RES64-02', 'number': 108,
        'candidate': 'D56-BODY01-SIZE02', 'status': 'visual_proposal_not_accepted',
        'user_feedback': 'euh le drone est toujours plus pixelisé que les autres',
        'first_reduction_status': 'refused_as_still_too_pixelated',
        'scope': 'Propose more actual screen pixels with the same HD source, identity, pose and palette.',
        'source': 'D56-BODY01-source.png', 'native': 'D56-BODY01-SIZE02-native128.png',
        'source_sha256': export_data['source_sha256'], 'native_sha256': export_data['native_sha256'],
        'logical_tile': [64, 64], 'body_size': [70, 40], 'previous_body_size': [56, 32],
        'new_art_to_approve': ['Larger direct export', 'Size and three-pixel horizontal overhang', 'Native composition'],
        'campaign_bank_skill_icon_corpse_or_accepted_animations_changed': False,
    })
    write('size02-validation.json', {
        'native_scenes': 2, 'native_tile': 64, 'changed_bounds': bounds,
        'before_equals_first_proposal_pixel_for_pixel': True,
        'only_scout_region_differs': True, 'preserved_files': len(preservation),
        'previous_scene_sha256': sha(REVIEW / 'scene-after.png'),
        'new_scene_sha256': sha(REVIEW / 'scene-size02.png'), 'current_page_paths_exist': True,
        'method': 'Native Macroquad captures and offline pixel checks; no browser-render verification.',
    })
    print(json.dumps({'scenes': 2, 'changed_bounds': bounds, 'preserved': len(preservation)}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=['export', 'package', 'export-size02', 'package-size02'])
    parser.add_argument('--generated', type=Path)
    args = parser.parse_args()
    assert not (REVIEW / 'visual-acceptance-01.json').exists(), 'Do not overwrite an accepted review.'
    if args.mode == 'export':
        assert args.generated and args.generated.is_file()
        export(args.generated)
    elif args.mode == 'package':
        package()
    elif args.mode == 'export-size02':
        export_size02()
    else:
        package_size02()


if __name__ == '__main__':
    main()
