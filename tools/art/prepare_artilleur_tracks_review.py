"""Condition the imagegen source for the isolated Artilleur review, never live assets."""
from pathlib import Path
import hashlib
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-92/artilleur-body03'
assert not (REVIEW / 'visual-acceptance-01.json').exists()
source = Image.open(REVIEW / 'D59-BODY03-source.png').convert('RGBA')
bounds = source.getchannel('A').point(lambda x: 255 if x >= 8 else 0).getbbox()
subject = source.crop(bounds)

def export(side, width, bottom=None):
    height = round(subject.height * width / subject.width)
    scaled = subject.resize((width, height), Image.Resampling.LANCZOS)
    canvas = Image.new('RGBA', (side, side))
    canvas.alpha_composite(scaled, ((side-width)//2, (side-height)//2 if bottom is None else bottom-height))
    return canvas

for name, im in [
    ('D59-BODY03-native64.png', export(64, 58, 62)),
    ('D59-C06-compact64.png', export(64, 60)),
    ('D59-C06-compact32.png', export(32, 30)),
]:
    im.save(REVIEW / name)
    assert im.mode == 'RGBA' and im.getchannel('A').getextrema() == (0,255)

metadata = {
    'candidate': '100 / D59-BODY03', 'icon': 'D59-C06',
    'scope': 'Tracked running gear replaces wheels; upper silhouette, colors, two barrels and frontal pose retained. Isolated preview only.',
    'status': 'native_render_and_animation_to_review', 'tool': 'built-in image_gen',
    'prompt': 'prompt.json', 'source_size': list(source.size), 'framing_bounds_alpha8': list(bounds),
    'direct64': 'High resolution source directly to64, no intermediate32. Only framing and resampling; no procedural redraw or recoloring.',
    'native_tile': 64, 'body_bottom':62, 'body_width':58,
    'barrel_centres_native64': [[13,35],[51,35]],
    'compact_sizes':[64,32], 'same_body_and_icon_source':True,
    'user_latest_reply':'je valide :)',
    'reply_scope':'Approval received during preparation after the track request; confirms the requested direction, no acceptance inferred for unshown native animation, effects or numbers.',
    'live_assets_modified':False,
    'files': {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in REVIEW.iterdir() if p.is_file() and p.name != 'review.json'},
}
(REVIEW/'review.json').write_text(json.dumps(metadata, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
print('Tracked Artilleur: direct64 body and compact64/32 prepared.')
