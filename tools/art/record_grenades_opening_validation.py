"""Collect the completed native, browser and compatibility checks for review 72."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-72'
read = lambda p: json.loads(p.read_text(encoding='utf8'))
def write(p, data):
    p.write_text(json.dumps(data, ensure_ascii=False, indent=2)+'\n', encoding='utf8')

generation = read(OUT/'generation.json')
for source in generation['sources']:
    path = Path(source['saved_path'])
    with Image.open(path) as image:
        assert image.mode == 'RGBA' and image.getchannel('A').getextrema()[0] == 0
        source['dimensions'] = list(image.size)
    source['sha256'] = hashlib.sha256(path.read_bytes()).hexdigest()
    if source.get('referenced_image_path'):
        source['reference_sha256'] = hashlib.sha256(Path(source['referenced_image_path']).read_bytes()).hexdigest()
write(OUT/'generation.json', generation)
encoding = read(OUT/'native-encoding-verification.json')
native = read(OUT/'native-geometry-verification.json')
browser = read(OUT/'browser-visible-verification.json')
parity = read(OUT/'browser-pixel-parity.json')
assert encoding['converted_frames'] == 3424 and encoding['pixel_identical_to_native_readback']
assert native['views'] == browser['views_checked'] == 32 and not browser['errors']
assert parity['native_canvas_pixel_parity'] == 12
write(OUT/'validation.json', {
    'date': '2026-10-14', 'debug_build': 'passed',
    'locked_all_targets_check': 'passed', 'format_check': 'passed', 'scoped_diff_check': 'passed',
    'targeted_tests': {'player_vision_and_legacy_suspension': 2, 'grenade_review_states': 3},
    'native_readback_frames': 3424, 'views': 32, 'directions': 4, 'scenes': 4,
    'browser': 'Edge Chromium headless, direct file gallery', 'browser_errors': [],
    'browser_native_pixel_identical_samples': 12,
    'wall_clipping': 'passed in all 32 native views',
    'idle_counters': 'passed', 'refused_attack_atomic': 'passed',
    'visual_inspection': ['browser-impact.png', 'browser-projectile.png', 'browser-wall-0.png', 'browser-self-0.png'],
    'accepted_review69_and_dependencies_preserved': 8261,
    'new_grenade_visual_status': 'awaiting_explicit_user_approval',
    'new_grenade_campaign_integration': False,
    'vision12_integration': '../skill-spatial-review-01/vision-acceptance-01.json',
})
print('Validation de la revue 72 enregistrée ; accord visuel attendu.')
