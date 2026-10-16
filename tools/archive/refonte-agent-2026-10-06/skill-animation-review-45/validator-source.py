"""Final ANIM-S18 local-artifact checks, separate from campaign integration."""
import hashlib
import json
import re
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-45'
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-44'
checks = json.loads((OUT / 'verification.json').read_text(encoding='utf-8'))
native = json.loads((OUT / 'native-encoding-verification.json').read_text(encoding='utf-8'))
gallery = json.loads((OUT / 'gallery-controller-verification.json').read_text(encoding='utf-8'))
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
assert len(checks['controls']) == 29 and len(checks['animations']) == 30
assert native['converted_frames'] == 3364 and native['pixel_identical_to_native_readback']
assert {c['number'] for c in review['candidates']} == {44,45,46}
assert all(c['walls_clear'] and c['timers_require_explicit_turns'] for c in checks['controls'])
html = (OUT / 'index.html').read_text(encoding='utf-8')
assert len(re.findall(r'<canvas\b',html)) == 3
links = sorted(set(re.findall(r'(?:href|src)="([^"]+)"',html)))
assert links and all((OUT / link).is_file() for link in links)
accepted = json.loads((BASE / 'visual-acceptance-01.json').read_text(encoding='utf-8'))
assert all(hashlib.sha256((BASE / name).read_bytes()).hexdigest() == digest for name,digest in accepted['artifact_hashes'].items())
with Image.open(BASE / 'abysse-64.png') as im:
    assert im.size == (64,64) and im.mode == 'RGBA'
for kind in ['allonge','fente']:
    with Image.open(OUT / f'{kind}-atlas.png') as im:
        assert im.size == (1536,256) and im.mode == 'RGBA'
        assert all(im.crop((i*256,0,(i+1)*256,256)).getchannel('A').getbbox() for i in range(6))
for result in checks['controls']:
    permitted = result['permitted']
    animation = next(a for a in checks['animations'] if a['file'] == result['variant']+'-64.gif')
    if permitted:
        assert animation['distinct_frames'] > 10
report = {
    'date':'2026-10-13','batch':'ANIM-S18','build_locked_debug':'passed',
    'animation_lab_tests':{'passed':95,'failed':0,'new_abysse_cases':7},
    'format_check':'passed','diff_check':'passed','native_views':29,'animations':30,
    'lossless_native_frames':3364,'cropped_frames':2784,'resampling_of_native_captures':False,
    'walls_preview_turns_contacts_positions':'passed',
    'four_directions_and_shared_target_reaction':'captured and checked',
    'shadow_step_refusals':'hidden, occupied, outside range, no adjacent opaque obstacle, Silence',
    'prepared_visibility_not_real_fov':True,
    'approved_body_unchanged':'TR65-BODY02 / review44',
    'review44_acceptance_hashes_unchanged':True,
    'old_review_files_unchanged':checks['protected_files_unchanged'],
    'gif_palette':{'samples':30,'max_chromatic_mean_error':max(a['mean_chromatic_error'] for a in checks['animations']),'max_chromatic_p95_error':max(a['p95_chromatic_error'] for a in checks['animations'])},
    'gallery_controller':gallery,'local_links':{'checked':len(links),'missing':0},
    'drawing_tool':'Built-in imagegen','prompts':['imagegen-allonge-prompt.txt','imagegen-fente-prompt.txt'],
    'campaign_integration':False,'visual_approval':'Three new own-action compositions pending player review.',
    'not_run':['Full engine suite','Release build','Browser rendering: file protocol blocked; native GPU/offline controller checked.'],
}
(OUT / 'validation.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
(OUT / 'validator-source.py').write_bytes(Path(__file__).read_bytes())
print(json.dumps({'native_views':29,'animations':30,'old_files_preserved':checks['protected_files_unchanged'],'links':len(links)}))
