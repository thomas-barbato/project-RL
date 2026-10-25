"""Prepare M57-M62, including the original large six-legged silhouette."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-07'
SRC = ROOT / 'assets/source-art/player-walk-review-07'
target = DOC / 'creature-jobs-lot10.json'
if target.exists():
    raise SystemExit('Preserve existing selected lot10 jobs and corrections')
records = {r['id']: r for r in json.loads((ROOT / 'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']}
base = json.loads((DOC / 'creature-jobs-lot09.json').read_text('utf8'))[0]
specs = [
    ('MON-009-0', 'Grande créature à six pattes', 6, 'creatures-lot09/MON-069-0',
     'Exactly SIX thick pale/flesh walking limbs in THREE paired rows, with dark claws, THREE large charcoal conical dorsal spikes, and the original tall central VERTICAL red mouth with small ivory teeth. The vertical mouth IS its front, not a wound or new humanoid face. Preserve the huge flesh mound, original proportions approximately124 units wide by114 high INCLUDING spikes, and original flesh/red/dark palette. No eyes, shell, new head, extra spikes, extra legs, wings or tail. Heavy compact six-foot support groups, very little body bob, fixed body mass/height and spikes; never a wide split. INPUT2 guides six-leg supports only: do not copy its small red chitin species or shrink this large creature.'),
    ('MON-020-0', 'Créature à base tentaculaire', 0, 'creatures-lot01/MON-141-0',
     'NO humanoid walking legs or new feet. Preserve ALL the original curled pale lower appendages, including the THREE prominent visible curled supports, the original tall angular pale front plates, reddish seams/openings and paired rigid dark ribbed/blocklike side structures. Keep the original silhouette and existing appendage count from INPUT1 rather than adding limbs from a generic quad label. The base creeps by restrained traveling curl contractions, with stable upper-body height and mass. Do not bend the rigid upper plates or side structures, add a book, weapon, head, human arms, new tentacles, worm mouth or upright coil copied from INPUT2.'),
    ('MON-140-0', 'Larve segmentée aux petits appuis', 0, 'creatures-lot06/MON-142-0',
     'Preserve the LOW HORIZONTAL cream/tan segmented body, original dark brown chitin front head/jaws, original short pale underside appendages and rounded tapering rear. The existing small underside appendages stay attached to their original segments and may assist the low crawl; never erase them or replace them with long articulated legs, hands or boots. Small traveling segment contractions, stable body thickness, head size and approximate original67-by34 silhouette proportions. No purple shell, tusks, curled tail, new spikes, eyes, wings, extra mouth or new limbs copied from INPUT2. Never an upright worm pose or swelling accordion gait.'),
    ('MON-158-0', 'Créature bleue à deux pinces', 2, 'creatures-lot08/MON-113-0',
     'Exactly FOUR original main limbs total: TWO large articulated FRONT PINCERS and TWO smaller pointed REAR SUPPORTS. Preserve the separate small ivory mouthparts/mandibles and original pale head spurs as head parts, not extra walking legs. Keep the pale BLUE/GRAY angular carapace plates, dark inner body/joints, ivory pincer tips and dark rear points. Compact low scuttling crawl using the existing rear supports and foreclaw contacts; the two pincers remain pincers, never human hands or two extra legs. No additional middle legs, six/eight-legged redesign, tail, wings, new face, flat ivory body or equipment copied from INPUT2. Rigid shell keeps its scale and shape.'),
    ('MON-162-0', 'Créature ailée en tenue rouge', 2, 'creatures-lot04/MON-147-0',
     'Exactly TWO thin humanoid walking legs, TWO long bare arms and TWO separate RED bat wings. Preserve the pale gaunt head and its original dark curved head protrusions, long pale fingers, dark sleeveless upper garment, dark red chest/front hanging cloth, belt and dark gray trousers. Existing hanging strips remain CLOTH, not a tail. Preserve the original bare-arm materials and narrow dark arm details as drawn; no added heavy bracers. Lean compact grounded alternating feet, stable slender torso, wings carried with tiny joint motion. No purple skin, muscular body, new horns, mask, cape, tail, weapons, extra armor or extra limbs copied from INPUT2.'),
    ('MON-156-0', 'Créature massive bleutée', 2, 'creatures-lot07/MON-155-0',
     'Exactly TWO heavy walking legs and TWO long muscular arms, NO wings. Preserve the pale BLUE/GRAY angular plated/faceted body, original broad shoulders and forearm shapes, TWO curved ivory/gray horns, dark claws/talons and brown waist cloth. Keep its original tall muscular proportions and relatively small horned head. Compact heavy grounded two-foot gait, original limb length, complete arms, little bob; true both-feet-grounded idle. No mushroom cap, golden material, new mask, pale flesh palette, wings, tail, weapon, frost aura, particles or new armor copied from INPUT2.')
]
jobs = []
for number, (key, label, legs, guide, motion) in enumerate(specs, 57):
    identity = DOC / 'references' / f'{key}-identity-detail.png'
    native = DOC / 'references' / f'{key}-native-current.png'
    guide = SRC / 'accepted' / f'{guide}-whole-body-native.png'
    assert identity.exists() and native.exists() and guide.exists(), key
    assert base['prompt'].count(base['motion']) == 1
    prompt = base['prompt'].replace(base['id'], key).replace(base['motion'], motion)
    prompt_path = DOC / 'prompts' / f'{key}-lot10-r01.txt'
    prompt_path.write_text(prompt, 'utf8')
    job = {'id': key, 'review_id': f'WALK07-M{number:02}', 'kind': 'creature', 'label': label,
           'record': records[key], 'locomotor_legs': legs, 'motion': motion,
           'reference': str(identity), 'references': [str(identity), str(guide)],
           'source_destination': str(SRC / f'{key}-cycle-lot10-r01.png'),
           'source_columns': 7, 'source_rows': 2, 'selected_source_indices': list(range(14)),
           'shadow': True, 'strict_source_outer_margin': True,
           'prompt_file': str(prompt_path.relative_to(ROOT)), 'prompt': prompt}
    if key == 'MON-020-0': job['support_label'] = 'base tentaculaire'
    if key == 'MON-009-0': job.update({'preserve_native_height': True, 'review_width_limit': 126})
    if key == 'MON-140-0': job['support_label'] = 'corps segmenté et petits appuis'
    if key == 'MON-158-0': job['support_label'] = '2 appuis arrière et 2 pinces'
    jobs.append(job)
target.write_text(json.dumps(jobs, ensure_ascii=False, indent=2) + '\n', 'utf8')
print(json.dumps({'review_ids': [j['review_id'] for j in jobs], 'native_heights': [j['record']['bounds'][3]-j['record']['bounds'][1] for j in jobs], 'runtime_integration': False}))
