"""Prepare M33-M38, preserving actual source limbs and species."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-07'
SRC = ROOT / 'assets/source-art/player-walk-review-07'
target = DOC / 'creature-jobs-lot06.json'
if target.exists():
    raise SystemExit('Preserve existing selected lot06 jobs and corrections')
records = {r['id']: r for r in json.loads((ROOT / 'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']}
specs = [
    ('MON-142-0', 'Forme rampante violette', 0, 'creatures-lot01/MON-141-0',
     'NO humanoid legs, feet, arms or wings. Preserve the LOW HORIZONTAL purple segmented body, small dark front opening, paired pale front tusks/mandibles, original pale underside spurs and tapering curled tail tip. Keep every existing spur attached to its own segment: do not turn them into articulated walking feet or add rows of insect legs. Advance by small traveling segment contractions, with a level low silhouette and nearly constant length. INPUT2 guides coherent segment motion only; do not copy its upright coil, round worm head or mouth. The creature stays horizontal and close to the ground.'),
    ('MON-089-0', 'Créature à quatre pattes anguleuses', 4, 'creatures-lot03/MON-103-0',
     'Exactly FOUR long ochre/brown angular walking legs, two front and two rear, with dark clawed tips. Preserve the pointed dark charcoal head/back covering and its original upward points, narrow low body and sharp mandibles. The points above the head are not new wings or legs. No extra arms, abdominal tail, shell from the guide or missing rear legs. Compact four-foot alternating support with restrained joint changes; preserve the original angular leg length without a wide split or stretching.'),
    ('MON-088-0', 'Oiseau aux ailes rouges', 2, 'creatures-lot05/MON-066-0',
     'Exactly TWO long walking bird legs ending in dark talons, TWO original RED FEATHERED wings and NO separate arms or hands. Preserve the long curved pale neck and head, dark beak, red feathered body and two low folded red wings. This is the source bird silhouette, not a muscular demon or bat. Compact grounded bird steps with alternating talons, a stable neck and only slight feather motion. No added horns, teeth, fingers, membranes, clothing, new tail or flight. INPUT2 guides compact grounded supports only; none of its anatomy or materials may be copied.'),
    ('MON-149-0', 'Créature ailée pâle massive', 2, 'creatures-lot04/MON-148-0',
     'Exactly TWO hind walking legs, TWO muscular arms and TWO separate dark charcoal bat wings. Preserve the PALE heavy hunched body, ivory/brown horns, dark face/beard/chest area, black claws and brown hanging waist cloth. Keep its original forward lean and heavy proportions, without copying the orange reference skin or enlarging the belly. Heavy compact alternating two-foot supports, arms carried low and coherent, wings carried spread with tiny joint motion. No extra feet, bright eyes, added tail, weapon or armor.'),
    ('MON-150-0', 'Créature ailée verte', 2, 'creatures-lot04/MON-148-0',
     'Exactly TWO walking legs, TWO arms and TWO separate green bat wings. Preserve the olive GREEN flesh, rust/ochre chest and abdomen, ivory lateral horns, dark brow and claws, brown waist cloth and green wing membranes. Keep the original athletic torso, not the guide heavy orange bulk. Compact alternating grounded two-foot gait with very little bob; wings remain carried open, not flapping for flight. No new tail, bracers, weapon, costume, new armor or extra limbs.'),
    ('MON-153-0', 'Créature ailée au visage allongé', 2, 'creatures-lot05/MON-066-0',
     'Exactly TWO walking legs, TWO separate long arms/hands and TWO separate bat wings: SIX appendages, unlike INPUT2 wing-bearing arms. Preserve the lean ochre/brown body, dark long claws, brown waist cloth, muted mauve/brown wing membranes and the original LONG NARROW PALE BEAK-LIKE FACE pointing down at the front. Do not replace the long face with a human skull, new mask, snout, bull horns or the guide head; no new metal material. Compact grounded alternating two-foot steps, arms stay attached, wings carried spread with tiny joint motion, low restrained bob. No flight, extra tail or equipment.')
]
jobs = []
for number, (key, label, legs, guide, motion) in enumerate(specs, 33):
    identity = DOC / 'references' / f'{key}-identity-detail.png'
    native = DOC / 'references' / f'{key}-native-current.png'
    guide = SRC / 'accepted' / f'{guide}-whole-body-native.png'
    assert identity.exists() and native.exists() and guide.exists(), key
    prompt = f'''Use case: identity-preserve. Complete lateral game-creature animation for existing identity {key}.
INPUT1 is the exact canonical APPEARANCE. Preserve anatomy, head/face, supports, all original appendages, materials, colors and proportions. Isolated colored fringe specks in the old export are not glow or anatomy.
INPUT2 is ONLY an accepted WHOLE-FIGURE motion and layout guide. Do not copy its species, face, bulk, armor, colors, limbs or objects. Use compact supports and the same matte native64-readable game-art finish.
SPECIFIC ANATOMY AND MOVEMENT: {motion}
OUTPUT EXACTLY FOURTEEN separate COMPLETE figures, exactly TWO ROWS of exactly SEVEN figures on a wide transparent atlas approximately1792x640. No extra rows, eighth column, labels or duplicates. Each whole figure occupies at most75percent of its cell WIDTH AND HEIGHT, all using the SAME scale. Generous transparent margins around every claw, spur, wing and tail; at least24px from the OUTER canvas edge. No figures touch and no appendages are clipped. Do not fill the cells.
TOP ROW: ALL face screenLEFT in near-front three quarters, idle then L0,L1,L2,L3,L4,L5. BOTTOM ROW: ALL face RIGHT, idle then R0,R1,R2,R3,R4,R5. Nose/beak, chest/front, feet/base all face the SAME direction together. No back view, pure profile, head-only reversal or direction changes inside a row. The two rows must have visibly different lateral orientations. A rearward-curving horn or crest does not define the facing direction: the FACE and chest do.
Six motion poses form ONE full compact cycle, three phases on one grid cell and the next three on the next cell,400ms per cell. Real stable resting idle. Use actual source morphology, never a human gait for a legless form or extra arms on a bird.
Draw EACH creature WHOLE and coherent. Constant apparent scale, mass, head size, anatomy and materials across all14 poses; restrained bob, no great split, stretching, hopping or swelling. No grafted parts, procedural strips or detached limbs. Canonical source proportions stay unchanged despite empty cell space.
Genuine transparent alpha, clean dark matte game-art contours and readable shapes. No floor, ground shadow, text, grid, effects, particles, halo, colored fringe/specks, extra equipment or redesign.'''
    prompt_path = DOC / 'prompts' / f'{key}-lot06-r01.txt'
    prompt_path.write_text(prompt, 'utf8')
    jobs.append({'id': key, 'review_id': f'WALK07-M{number:02}', 'kind': 'creature', 'label': label,
                 'record': records[key], 'locomotor_legs': legs, 'motion': motion,
                 'reference': str(identity), 'references': [str(identity), str(guide)],
                 'source_destination': str(SRC / f'{key}-cycle-lot06-r01.png'),
                 'source_columns': 7, 'source_rows': 2, 'selected_source_indices': list(range(14)),
                 'shadow': True, 'strict_source_outer_margin': True,
                 'prompt_file': str(prompt_path.relative_to(ROOT)), 'prompt': prompt})
target.write_text(json.dumps(jobs, ensure_ascii=False, indent=2) + '\n', 'utf8')
print(json.dumps({'review_ids': [j['review_id'] for j in jobs], 'supports': [j['locomotor_legs'] for j in jobs], 'runtime_integration': False}))
