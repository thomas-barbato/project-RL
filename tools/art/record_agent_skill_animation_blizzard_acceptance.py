"""Record FR05-A01 visual approval and the duration clarification shown to the user."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-25'
receipt = OUT / 'blizzard-acceptance-01.json'
assert not receipt.exists(), 'Approval already recorded; preserve its evidence.'
review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
candidate = next(c for c in review['candidates'] if c['skill_id'] == 'FR05')
assert candidate['variant'] == 'FR05-A01'
files = ['storm-source.png', 'storm-atlas.png', 'imagegen-storm-prompt.txt',
         'drawing-pack.json', 'render-source.rs', 'clip-source.rs', 'gallery-script.js',
         'index.html', 'verification.json', 'gallery-controller-verification.json',
         'native/capture.json', 'tempetes-froid-64.gif', 'tempetes-obstacles-64.gif',
         'tempetes-cloison-64.gif']
files += sorted(p.name for p in OUT.glob('FR05-A01-*-64.gif'))
proof = {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date': '2026-10-12', 'user_reply': 'ok parfait :) je valide :)',
    'preceding_question': "c'est tres jolis, est-ce que le blizzard dure plusieurs tours ?",
    'accepted': {'number': 18, 'skill_id': 'FR05', 'variant': 'FR05-A01',
                 'status': 'visually_approved', 'native_tile_size': 64},
    'timing_confirmation': {'duration_at_rank_1_turns': 3, 'duration_at_rank_11_turns': 5,
        'prepared_impacts_per_application_at_rank_11': 4, 'initial_preparation_complete_turns': 1,
        'duration_and_new_impacts_advance_only_with_game_turns': True,
        'snowfall_remains_visually_animated_between_actions': True},
    'scope': 'Blizzard artwork, animation and the duration/timing explanation shown immediately before this approval. Catalogue values are confirmed, not changed. Prepared hits do not approve damage numbers, RNG, final geometry, real AI or campaign integration.',
    'other_candidates': 'Vague de froid FR04-A01 and the separately pending reused Gel visual retain their prior review status; the Blizzard-focused confirmation does not infer their approval.',
    'artifact_hashes': proof,
}, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
candidate['status'] = 'visually_approved'
candidate['acceptance'] = receipt.name
candidate['shown_timing_status'] = 'confirmed'
review['status'] = 'blizzard_approved_wave_pending'
review['blizzard_acceptance'] = receipt.name
(OUT / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
print('FR05-A01 Blizzard approved; displayed durations confirmed.')
