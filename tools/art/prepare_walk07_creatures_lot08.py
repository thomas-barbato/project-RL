"""Prepare M45-M50 with separate hand-supported and four-legged gaits."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-07'
SRC = ROOT / 'assets/source-art/player-walk-review-07'
target = DOC / 'creature-jobs-lot08.json'
if target.exists():
    raise SystemExit('Preserve existing selected lot08 jobs and corrections')
records = {r['id']: r for r in json.loads((ROOT / 'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']}
base = json.loads((DOC / 'creature-jobs-lot07.json').read_text('utf8'))[0]
specs = [
    ('MON-104-0', 'Rampant massif pâle', 4, 'creatures-lot04/MON-092-0',
     'FOUR supporting clawed limbs, two large forelimbs and two smaller hindlimbs. Preserve the LOW heavy pale arched back, folds and muscles, dark obscured forward head below the shoulders and broad dark claws. Compact four-limb ground support, with very little back movement. Keep its original broad squat proportions: no upright humanoid, new animal snout, horns, tail, plates, shell, tusks or extra face copied from INPUT2.'),
    ('MON-107-0', 'Créature à carapace ovale', 4, 'creatures-lot05/MON-101-0',
     'Exactly FOUR short supporting legs with brown/dark clawed feet, two front and two rear, and the original small pale head beneath the front of the shell. Preserve the tall OVAL olive/brown organic carapace and its original thin cream seam/ridges, dark brown limbs and pale front mandibles. Shell keeps a constant rigid size, height and volume while compact four-foot steps alternate below it. No extra insect legs, metal helmet, vertical grate bars, round armored dome, wings, antennae or tail copied from INPUT2.'),
    ('MON-113-0', 'Créature pâle aux appuis écartés', 4, 'creatures-lot06/MON-089-0',
     'Preserve FOUR large OUTER pale angular walking supports with dark pointed tips, arranged around the low flat ivory central body. ALSO preserve the original SHORT CENTRAL downward/front appendage under the central body, exactly as in INPUT1: do not erase it, duplicate it, make it another long articulated walking leg or invent a new face there. Keep the pale flat upper outline and its existing raised corners. Small four-outer-support stepping gait, original leg lengths, restrained bob and stable low central body. No dark horned helmet from INPUT2, human hands, wings, tail, new eyes, equipment or extra legs.'),
    ('MON-118-0', 'Rampant à capuche rouge', 2, 'creatures-lot02/MON-064-0',
     'This is a HUMANOID crawling on FOUR supports: TWO ARMS/HANDS and TWO HIND LEGS/FEET, not an animal with four interchangeable legs. Preserve the dark hood/robe, red hood lining, sleeves and front cloth, pale partly obscured face, exposed pale forearms/hands, dark fingers/claws, dark trousers and rear footwear. Alternate the two hands with the two hind supports in a low measured crawl. Human elbows, wrists, knees and feet remain coherent and distinct. Keep the original low posture and body length; never stand upright, turn arms into animal legs, add a tail, tear off cloth or copy naked animal anatomy from INPUT2. Do not invent a face under the hood or add a weapon.'),
    ('MON-122-0', 'Rampant chauve en tenue sombre', 2, 'creatures-lot02/MON-064-0',
     'This is a HUMANOID crawling on FOUR supports: TWO ARMS/HANDS and TWO HIND LEGS/FEET. Preserve its bald pale bowed head, dark gray/black clothing, original ochre upper-torso/back cloth panel, pale exposed forearms and long bare hands, dark trousers and the two rear boots. Keep the original very LOW horizontal crawling silhouette, small head and long body. Alternate hand contacts and opposite hind support with compact steps, not an animal four-leg gait or upright zombie walk. Preserve both arm/hand identities and human joints. No new wound, red blood, hat, weapon, tail, naked back or extra limbs from INPUT2.'),
    ('MON-131-0', 'Quadrupède sombre aux flammes', 4, 'creatures-lot03/MON-063-0',
     'Exactly FOUR short clawed supporting limbs. Preserve the low dark charcoal/red sinewy body, original small lowered front head, dark claws and the EXISTING orange/yellow flame crests at the front/back of the body. The original flame shapes, palette and coverage belong to this canonical identity; keep them consistent in size through all14 poses, with no extra fire, embers, aura, particles, lighting or new effect. Compact four-limb walking supports below a stable low back; no upright biped, new animal snout, horns, coat, tail or equipment copied from INPUT2. Do not hide the original flames or change their brightness.')
]
jobs = []
for number, (key, label, legs, guide, motion) in enumerate(specs, 45):
    identity = DOC / 'references' / f'{key}-identity-detail.png'
    native = DOC / 'references' / f'{key}-native-current.png'
    guide = SRC / 'accepted' / f'{guide}-whole-body-native.png'
    assert identity.exists() and native.exists() and guide.exists(), key
    assert base['prompt'].count(base['motion']) == 1
    prompt = base['prompt'].replace(base['id'], key).replace(base['motion'], motion)
    if key == 'MON-131-0':
        prompt = prompt.replace('No floor, ground shadow, text, grid, effects, particles, halo, colored fringe/specks, extra equipment or redesign.',
                                'No floor, ground shadow, text, grid, NEW effects, particles, halo, colored fringe/specks, extra equipment or redesign. Preserve only the existing canonical flame crests stated above.')
    prompt_path = DOC / 'prompts' / f'{key}-lot08-r01.txt'
    prompt_path.write_text(prompt, 'utf8')
    job = {'id': key, 'review_id': f'WALK07-M{number:02}', 'kind': 'creature', 'label': label,
           'record': records[key], 'locomotor_legs': legs, 'motion': motion,
           'reference': str(identity), 'references': [str(identity), str(guide)],
           'source_destination': str(SRC / f'{key}-cycle-lot08-r01.png'),
           'source_columns': 7, 'source_rows': 2, 'selected_source_indices': list(range(14)),
           'shadow': True, 'strict_source_outer_margin': True,
           'prompt_file': str(prompt_path.relative_to(ROOT)), 'prompt': prompt}
    if key in ['MON-118-0', 'MON-122-0']:
        job.update({'locomotor_supports': 4, 'supporting_hands': 2,
                    'support_label': '4 appuis (mains et pieds)'})
    jobs.append(job)
target.write_text(json.dumps(jobs, ensure_ascii=False, indent=2) + '\n', 'utf8')
print(json.dumps({'review_ids': [j['review_id'] for j in jobs], 'supports': [j.get('locomotor_supports', j['locomotor_legs']) for j in jobs], 'runtime_integration': False}))
