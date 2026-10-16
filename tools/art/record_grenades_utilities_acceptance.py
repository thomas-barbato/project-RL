"""Record the user's approval of the revised grenade lot without redrawing it."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-75'
read = lambda p: json.loads(p.read_text(encoding='utf8'))
def write(p, data):
    p.write_text(json.dumps(data, ensure_ascii=False, indent=2)+'\n', encoding='utf8')

approval_path = OUT/'visual-acceptance-01.json'
assert not approval_path.exists(), 'Approval already recorded; preserve the existing evidence.'
review = read(OUT/'review.json')
validation = read(OUT/'validation.json')
assert review['status'] == 'awaiting_visual_review'
assert validation['native_views'] == validation['mechanical_timelines_identical'] == 48
assert validation['browser_native_pixel_identical_samples'] == 18
assert validation['browser_errors'] == []

cache = {}
def digest(path):
    stat = path.stat()
    identity = (stat.st_ino, stat.st_size, stat.st_mtime_ns)
    if identity not in cache:
        cache[identity] = hashlib.sha256(path.read_bytes()).hexdigest()
    return cache[identity]

artifacts, native_sources = {}, {}
for entry in read(OUT/'animations.json'):
    for index in set(entry['image_indices']):
        path = OUT/'frames'/entry['variant']/f'{index:03}.png'
        artifacts[path.relative_to(OUT).as_posix()] = digest(path)
    path = OUT/f"{entry['variant']}-64.gif"
    artifacts[path.name] = digest(path)
    for name in entry['native_frame_sources']:
        native_sources[Path(name).as_posix()] = digest(ROOT/name)
for name in ('grenades-simple-source.png', 'eye-symbol.svg', 'prompt.txt', 'sprite-layout.json'):
    artifacts[name] = digest(OUT/name)

approval = {
    'review_id': 'ANIM-S34-REV02-ACCEPT01', 'date': '2026-10-14',
    'user_reply': 'parfait je valide', 'visual_approved': True,
    'candidates': review['candidates'], 'reference_projectile_approved': 'SG00-A03',
    'board_grenades': 'White and blue cylinders and grey/red fragmentation; simple flat colors, approximately 18 native pixels tall.',
    'blindness_marker': 'Almond eye with visible pupil, contrasted red diagonal bar and remaining turns beside it.',
    'approved_scope': 'Rendered corrected lot75 in four directions, including the revised board projectile in the damage comparison.',
    'skill_tree_icons_changed': False, 'new_balance_decision': False,
    'rules_duration_damage_and_campaign_AI_changed': False,
    'prepared_numeric_examples': 'Illustrative values; this approval creates no additional balance rule.',
    'player_self_blindness_eligibility': 'still_not_decided',
    'campaign_integration': False, 'saves_changed': False,
    'remaining_unshown': 33, 'supersedes_visuals': 'SG01-A01, SG02-A01, SG04-A01 and board projectile from review74; old files retained.',
    'artifact_hashes': artifacts, 'native_source_hashes': native_sources,
}
write(approval_path, approval)
review.update(status='approved', visual_approved=True, acceptance=approval_path.name, reference_projectile_visual_approved=True)
write(OUT/'review.json', review)
validation.update(status='approved', visual_approved=True, acceptance=approval_path.name)
write(OUT/'validation.json', validation)
progress = read(OUT/'progress-audit.json')
progress.update(status='approved', acceptance=approval_path.name)
write(OUT/'progress-audit.json', progress)
generation = read(OUT/'generation.json')
generation.update(new_visual_approval=True, acceptance=approval_path.name)
write(OUT/'generation.json', generation)
for name in ('index.html', 'cote-a-cote.html'):
    path = OUT/name
    text = path.read_text(encoding='utf8').replace('Nouveaux rendus proposés à la validation.', 'Rendus visuels validés.')
    path.write_text(text, encoding='utf8')
print(f'Visual approval recorded: {len(artifacts)} displayed drawings/GIFs and {len(native_sources)} native source frames protected.')
