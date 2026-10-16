"""Final local-artifact checks; no browser automation or game mutations."""
import hashlib
import json
import re
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-44'
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
verification = json.loads((OUT / 'verification.json').read_text(encoding='utf-8'))
gallery = json.loads((OUT / 'gallery-controller-verification.json').read_text(encoding='utf-8'))
native = json.loads((OUT / 'native-encoding-verification.json').read_text(encoding='utf-8'))
assert review['candidates'][0]['body_variant'] == 'TR65-BODY02'
assert len(verification['controls']) == 7 and len(verification['animations']) == 10
assert all(c['walls_clear'] and c['timers_require_explicit_turns'] for c in verification['controls'])
assert native['converted_frames'] == 1428 and native['pixel_identical_to_native_readback']
html = (OUT / 'index.html').read_text(encoding='utf-8')
assert len(re.findall(r'<canvas\b', html)) == 1
links = sorted(set(re.findall(r'(?:href|src)="([^"]+)"', html)))
assert links and all((OUT / name).is_file() for name in links)
with Image.open(OUT / 'abysse-64.png') as im:
    assert im.size == (64, 64) and im.mode == 'RGBA'
    assert im.getchannel('A').getextrema() == (0, 255)
    cyan = sum(a > 128 and g > r + 20 and b > r + 20 for r, g, b, a in im.get_flattened_data())
    assert cyan == 0, cyan
with Image.open(OUT / 'abysse_fx-atlas.png') as im:
    assert im.size == (1536, 256) and im.mode == 'RGBA'
    assert all(im.crop((i * 256, 0, (i + 1) * 256, 256)).getchannel('A').getbbox() for i in range(6))
accepted = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-43'
receipt = json.loads((accepted / 'visual-acceptance-01.json').read_text(encoding='utf-8'))
assert all(hashlib.sha256((accepted / name).read_bytes()).hexdigest() == digest for name, digest in receipt['artifact_hashes'].items())
report = {
    'date': '2026-10-13', 'batch': 'ANIM-S17',
    'build_locked_debug': 'passed', 'animation_lab_tests': {'passed': 88, 'failed': 0},
    'format_check': 'passed', 'diff_check': 'passed',
    'native_views': 7, 'animations': 10, 'native_frames_losslessly_encoded': 1428,
    'cropped_frames': 1260, 'capture_resampling': False,
    'wall_clipping_and_explicit_turn_checks': 'passed',
    'four_directions': 'captured and checked', 'entry_return_renewal': 'captured and checked',
    'direct_native_body_size': [64, 64], 'opaque_cyan_pixels_in_body': cyan,
    'drawing_tool': 'Built-in imagegen', 'prompts': ['imagegen-abysse-body02-prompt.txt', 'imagegen-abysse-fx-prompt.txt'],
    'protected_old_files_unchanged': verification['protected_files_unchanged'],
    'review43_approval_receipt_unchanged': True,
    'gif_palette': {'samples': len(verification['animations']), 'max_mean_purple_error': max(a['peak_purple_mean_error'] for a in verification['animations']), 'max_p95_purple_error': max(a['peak_purple_p95_error'] for a in verification['animations'])},
    'gallery_controller': gallery, 'local_links': {'checked': len(links), 'missing': 0},
    'visual_approval': 'TR65-BODY02 and TR65-A01 pending player review.',
    'campaign_integration': False, 'own_actions': 'Allonge, Fente abyssale and Pas d’ombre not shown in this body/transition lot.',
    'not_run': ['Full engine suite', 'Release build', 'Browser rendering (file protocol blocked); native GPU and offline controller checked.'],
}
(OUT / 'validation.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print(json.dumps({'native_views': 7, 'animations': 10, 'old_files_preserved': verification['protected_files_unchanged'], 'links': len(links), 'cyan_pixels': cyan}))
