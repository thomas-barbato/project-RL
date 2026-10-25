"""User-requested symmetric metal calves on the existing whole-figure gait."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
jobs=[]
for side,old in [('left','r07b'),('right','r07e')]:
    source=SRC/f'BASTION-TRANSFER-10-{old}-{side}.png'
    prompt=f'''Use case: precise-object-edit.
Edit the supplied ONE-ROW sprite sheet containing exactly SEVEN complete huntress figures facing {side.upper()}: idle followed by six walking poses.
User-requested change: BOTH LOWER LEGS MUST BE METAL in EVERY figure. Keep the existing silver mechanical shin shield on one calf and give the OTHER calf a MATCHING silver mechanical shin shield and articulated ankle. Both the planted front calf and the bent rear calf must be visibly silver metal, from below each knee to its ankle, respecting the existing bend and foreshortening. This applies to ALL SEVEN figures, including idle. Keep dark blue cloth on the thighs and the existing dark boots with silver toe plates. No remaining cloth-only calf, no alternating single armored leg.
Change only the calf armor material/design. Preserve the exact existing seven walking/rest poses, joint and boot positions, lifted foot height, outer silhouette, scale, framing, head direction, face, hair, torso, arms, empty hands, outfit colors and original game-art finish. Do not redesign the gait, move legs, mirror, graft limbs or add equipment elsewhere.
Output exactly the same SEVEN complete figures in ONE ROW, same order and layout, full hair and both boots with margins. Genuine transparent background, clean cutout without halo, shadow, ground, labels or text.'''
    filename=f'CLASS-10-bilateral-metal-r07f-{side}.txt'
    (DOC/'prompts'/filename).write_text(prompt,'utf8')
    jobs.append({'id':f'CLASS-10-bilateral-metal-r07f-{side}','references':[str(source)],'prompt':prompt,
                 'destination':str(SRC/f'BASTION-TRANSFER-10-r07f-{side}.png'),
                 'user_quote':'je pense que tu devrais faire les 2 jambes en metal',
                 'scope':'Traqueuse outfit1; both directions; idle and six existing walking poses; material change only; review before integration'})
(DOC/'bilateral-metal-jobs-r07f.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
