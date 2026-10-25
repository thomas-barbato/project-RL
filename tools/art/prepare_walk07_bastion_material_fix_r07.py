import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
jobs=json.loads((DOC/'bastion-transfer-jobs-r07.json').read_text('utf8'))
for j in jobs:
    if j['index']==10 and j['side']=='right':
        j['prompt']+='\nCRITICAL MATERIAL IDENTITY: all ordinary leg areas are DARK BLUE-GRAY PANTS down to the boot, NOT exposed skin. No orange, tan or bare calf below the waist. NEVER copy the orange Bastion greaves or its materials. Only the SAME anatomical right mechanical shin is silver metal; do not exchange its material with the other leg.'
        Path(j['prompt_file']).write_text(j['prompt'],'utf8')
(DOC/'bastion-transfer-jobs-r07.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
fix={'id':'CLASS-10-left-material','index':10,'side':'left','references':[str(SRC/'BASTION-TRANSFER-10-r07a-left-material-rejected.png'),str(ROOT/'docs/reviews/player-walk-06/references/CLASS-10-canonical.png')],
    'destination':str(SRC/'BASTION-TRANSFER-10-r07-left.png'),
    'prompt':'''Use case: precise-object-edit / clothing material correction. INPUT1 is the EDIT TARGET: exactly seven complete huntress figures in one row, idle then six walking poses. INPUT2 is the exact outfit/material reference, NOT a new pose guide.
Change ONLY the incorrect orange/tan exposed calf patches in INPUT1 to the original continuous DARK BLUE-GRAY TROUSER fabric down to the ordinary boot, with appropriate original seam/fold shading. There is NO bare leg skin and NO orange greave below the waist in this outfit. Preserve the ONE silver mechanical shin attached to its existing SAME anatomical hip/knee in every pose, and all silver prosthetic details; do not transfer the prosthesis to the other leg or make both shins metal.
Keep ALL SEVEN whole figures, each EXACT walking pose, hip/knee/ankle/boot positions, leg swing, outlines, body proportions, full head and bun, face gaze toward screenLEFT, torso direction, complete arms, hands, armor shapes, scale, pixel-art finish, order and row layout as INPUT1. Do NOT redraw the gait, turn a head, mirror any body or change a foot. This is only restoring the cloth material over the orange patches. Output the same seven complete figures in one horizontal row with generous transparent margins, TRUE transparency, no label, background, floor, shadow, weapon or effect.'''}
(DOC/'prompts/BASTION-TRANSFER-10-r07-left-material.txt').write_text(fix['prompt'],'utf8')
(DOC/'bastion-material-job-r07.json').write_text(json.dumps(fix,ensure_ascii=False,indent=2)+'\n','utf8')
