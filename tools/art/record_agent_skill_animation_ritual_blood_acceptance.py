"""Record the accepted fluid blood rendering, independently of campaign integration."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-38'
receipt = OUT / 'visual-acceptance-01.json'
assert not receipt.exists(), 'Preserve existing approval evidence.'
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
assert {c['variant'] for c in review['candidates']} == {'OC04-A03', 'OC07-A03'}
assert all(c['status'] == 'awaiting_visual_review' for c in review['candidates'])
checks = json.loads((OUT / 'redraw-verification.json').read_text(encoding='utf-8'))
assert checks['timelines_identical_to_review37'] and checks['actors_and_terrain_identical']
assert checks['ivory_ally_marker_identical']
for local, source in [('render-source.rs', 'src/agent_skill_lab_review/fire_painted/occult_rituals.rs'), ('clip-source.rs', 'src/agent_skill_lab_review/fire_painted/zones.rs')]:
    assert (OUT / local).read_bytes() == (ROOT / source).read_bytes()
files = ['prison-source.png', 'prison-atlas.png', 'reanimation-source.png', 'reanimation-atlas.png',
         'imagegen-prison-prompt.txt', 'imagegen-prison-geometry-prompt.txt', 'imagegen-reanimation-prompt.txt',
         'drawing-pack.json', 'render-source.rs', 'clip-source.rs', 'render-parent-source.rs',
         'index.html', 'gallery-script.js', 'verification.json', 'redraw-verification.json',
         'gallery-controller-verification.json', 'native/capture.json', 'Ouvrir_laboratoire.ps1']
files += sorted(p.name for p in OUT.glob('*.gif'))
files += sorted(p.relative_to(OUT).as_posix() for p in (OUT / 'frames').rglob('*.png'))
files += sorted(p.relative_to(OUT).as_posix() for p in (OUT / 'native').rglob('timeline.json'))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date': '2026-10-13', 'user_reply': 'ca me semble bien mieux ui',
    'presented_preview': 'skill-animation-review-38/rituels-haut-64.gif',
    'presented_gallery': 'skill-animation-review-38/index.html',
    'accepted': [{'number': c['number'], 'id': c['id'], 'variant': c['variant'], 'status': 'visually_approved', 'native_tile_size': 64} for c in review['candidates']],
    'scope': 'The latest presented fluid blood rendering of both skills: circular independent blood segments and visible breach for Prison, separate liquid floor seal and short resurrection ribbons for Reanimation, with the shown ivory allied skull. Approval is for these rendered candidates and their shown timing.',
    'retained_rules': 'Previously agreed numerical rules, targeting, independent destruction and turn-driven durations keep their own approval status. The isolated fixtures do not establish real combat, segment HP resolution, AI, healing, loot, XP, saves or campaign integration.',
    'other_candidates': 'Earlier ritual variants and unpublished curvature/thickness trials are archived, not selected visual references. This does not approve a global blood style for other skills.',
    'artifact_hashes': proof,
}, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
for candidate in review['candidates']:
    # Correct a metadata-only UTF-8/locale decoding error; preserve the approved names.
    candidate['name'] = {'OC04': 'Prison sigillaire', 'OC07': 'R\u00e9animation'}[candidate['id']]
    candidate['status'] = 'visually_approved'; candidate['acceptance'] = receipt.name
review['status'] = 'visually_approved'; review['acceptance'] = receipt.name
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
validation = json.loads((OUT / 'validation.json').read_text(encoding='utf-8'))
validation['visual_approval'] = 'OC04-A03 and OC07-A03 visually approved.'
validation['acceptance'] = receipt.name
(OUT / 'validation.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
assert all(hashlib.sha256((OUT / name).read_bytes()).hexdigest() == digest for name, digest in proof.items())
print(json.dumps({'approved': ['OC04-A03', 'OC07-A03'], 'hashed_artifacts': len(proof), 'campaign_integration': False}))
