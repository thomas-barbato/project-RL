"""Prepare M39-M44 from inspected anatomy, reusing the established layout."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-07'
SRC = ROOT / 'assets/source-art/player-walk-review-07'
target = DOC / 'creature-jobs-lot07.json'
if target.exists():
    raise SystemExit('Preserve existing selected lot07 jobs and corrections')
records = {r['id']: r for r in json.loads((ROOT / 'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']}
base = json.loads((DOC / 'creature-jobs-lot06.json').read_text('utf8'))[0]
specs = [
    ('MON-033-0', 'Forme rampante à épines', 0, 'creatures-lot06/MON-142-0',
     'NO walking legs, feet, arms or wings. Preserve this small LOW HORIZONTAL cream/pink fleshy crawling form, its broad red horizontal front opening and EXACTLY TWO dark charcoal pointed dorsal spikes, a larger front spike and a smaller rear spike. Preserve its short tapering rear body. No ivory mandibles, purple plates, tail coil or new segments copied from INPUT2. Advance with tiny traveling contractions, little vertical bob and constant body length/volume. The original complete silhouette is41 units wide by23 high INCLUDING its spikes; do not make the body upright or the head spherical.'),
    ('MON-143-0', 'Créature pâle aux défenses', 4, 'creatures-lot02/MON-064-0',
     'Exactly FOUR short supporting legs, two fore and two hind, separate from the TWO pale curved tusks beside the dark forward head. Preserve the huge pale cream segmented/arched back, brown seams, dark chitin face, original forward tusks and short dark-tipped feet. Compact four-foot grounded walking. The rear remains a rounded body, not an extra mouth, human torso or tail. No extra legs, arms, dorsal spikes or armor from the guide.'),
    ('MON-154-0', 'Quadrupède aux nodules dorsaux', 4, 'creatures-lot03/MON-063-0',
     'Exactly FOUR stout clawed walking legs. Preserve the stocky PALE canine-like body, blunt wrinkled face, original small head horns/ears, olive-brown dorsal covering with THREE purple rounded raised nodules, and the single short curved tail. The nodules stay attached to their same dorsal positions in every pose, not new rocks or a saddle. Compact four-foot steps with stable back. No new armor, metallic material, tusks, spots, collars, long tail or extra legs copied from INPUT2.'),
    ('MON-155-0', 'Créature au large chapeau', 2, 'creatures-lot06/MON-153-0',
     'Exactly TWO walking legs and TWO long clawed arms; NO wings. Preserve the pale sinewy/bony body, original oversized ochre GOLDEN MUSHROOM-LIKE CAP and layered shoulder cap/plates, obscured face, burgundy waist cloth, long dark claws and talons. The cap is the original creature head covering, not a new human hat: do not uncover or invent a face/skull underneath. Low compact two-foot alternating walk, heavy cap stays level and rigid, long arms swing only slightly. No wings, beak, tail, horns, metal armor, spores, glow or extra equipment copied from INPUT2. Keep its original lean torso and long arms/legs.'),
    ('MON-157-0', 'Créature ailée grise à longue queue', 2, 'creatures-lot06/MON-153-0',
     'Exactly TWO walking legs, TWO long arms and TWO separate blue-violet bat wings, plus ONE long dark blue tail attached at the central rear of the hips. Preserve the lean PALE GRAY body, dark blue torso/hip areas, TWO long blue-black head horns, ivory forearm/wrist covering and long blue/dark claws. Both arms preserve the original matching materials. Compact grounded alternating talons, little bob; tail follows coherently without becoming a third foot or switching its attachment. Wings carried spread with tiny movement. No beak, new mask, spikes, costume or extra appendages copied from INPUT2.'),
    ('MON-151-0', 'Créature ailée pâle au brassard', 2, 'creatures-lot04/MON-147-0',
     'Exactly TWO walking legs, TWO muscular arms and TWO separate RED bat wings. Preserve the pale athletic body, ivory skull-like face, TWO original dark curved horns, dark waist cloth, dark feet and claws. CRITICAL ASYMMETRY: the SINGLE CHARCOAL BRACER covers ONLY the creature RIGHT forearm, seen on the VIEWER LEFT in INPUT1. It stays on that SAME anatomical arm in EVERY pose in BOTH near-front directions, remaining on viewer LEFT; the viewer RIGHT forearm stays PALE and uncovered. Never mirror the whole figure, swap the bracer, put it on both arms, remove it or turn just the head. Correct complete left-facing AND right-facing bodies, face/chest/knees/toes aligned together, compact grounded two-foot steps with restrained bob. Preserve the athletic torso and leg lengths; no purple skin, new tail, weapon or armor from INPUT2.')
]
jobs = []
for number, (key, label, legs, guide, motion) in enumerate(specs, 39):
    identity = DOC / 'references' / f'{key}-identity-detail.png'
    native = DOC / 'references' / f'{key}-native-current.png'
    guide = SRC / 'accepted' / f'{guide}-whole-body-native.png'
    assert identity.exists() and native.exists() and guide.exists(), key
    assert base['prompt'].count(base['motion']) == 1
    prompt = base['prompt'].replace(base['id'], key).replace(base['motion'], motion)
    prompt_path = DOC / 'prompts' / f'{key}-lot07-r01.txt'
    prompt_path.write_text(prompt, 'utf8')
    jobs.append({'id': key, 'review_id': f'WALK07-M{number:02}', 'kind': 'creature', 'label': label,
                 'record': records[key], 'locomotor_legs': legs, 'motion': motion,
                 'reference': str(identity), 'references': [str(identity), str(guide)],
                 'source_destination': str(SRC / f'{key}-cycle-lot07-r01.png'),
                 'source_columns': 7, 'source_rows': 2, 'selected_source_indices': list(range(14)),
                 'shadow': True, 'strict_source_outer_margin': True,
                 'prompt_file': str(prompt_path.relative_to(ROOT)), 'prompt': prompt})
target.write_text(json.dumps(jobs, ensure_ascii=False, indent=2) + '\n', 'utf8')
print(json.dumps({'review_ids': [j['review_id'] for j in jobs], 'supports': [j['locomotor_legs'] for j in jobs], 'runtime_integration': False}))
