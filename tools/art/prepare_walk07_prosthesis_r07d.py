"""Correct the right-facing anatomical chain; never alternate armor by time.

r07c is rejected: a temporal three-frame grouping is not a limb identifier.
Left-facing r07b is retained unchanged. The right view needs its far/right
hip, knee and calf to remain one leg in all six existing walking poses.
"""
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
from build_player_walk_review_04 import components
ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
source=SRC/'BASTION-TRANSFER-10-r07c-right.png'
im=Image.open(source).convert('RGBA');guide=im.copy();draw=ImageDraw.Draw(guide)
font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',20)
parts=sorted(components(im),key=lambda p:p['area'],reverse=True)[:7]
parts.sort(key=lambda p:p['center'][0])
for n,part in enumerate(parts):
    x0,y0,x1,y1=part['box'];w,h=x1-x0,y1-y0
    draw.text((x0,max(10,y0-38)),f'{n}: SAME HIP -> SAME LEG',fill='#eeeeee',font=font)
    if n:
        points=[(x0+w*.35,y0+h*.53),(x0+w*.27,y0+h*.72),(x0+w*.20,y0+h*.82)]
        draw.line(points,fill='#37d6ff',width=5)
        for x,y in points:draw.ellipse((x-4,y-4,x+4,y+4),fill='#37d6ff')
        # Calf on the other visible chain must remain dark in EVERY walk pose.
        x,y=x0+w*(.40 if n<=3 else .63),y0+h*.81
        draw.ellipse((x-w*.13,y-h*.08,x+w*.13,y+h*.08),outline='#ff5a9b',width=5)
guide_path=DOC/'references/CLASS-10-r07d-right-hip-chain-guide.png';guide.save(guide_path)
prompt='''Use case: precise-object-edit.
INPUT1 EDIT TARGET: seven complete huntress figures in one row, all facing RIGHT: one idle then SIX existing walking poses. INPUT2 is ONLY an anatomical correction guide. INPUT3 is canonical character appearance.
The current armor TELEPORTS between legs in the row. Remove that error without changing the existing foot motion. There is NO temporal switch of armor: the mechanical RIGHT leg remains the SAME leg in ALL SEVEN figures.
Idle column0 is already correct: mechanical silver shin on the VIEWER-LEFT leg under the cargo-pocket thigh, ordinary dark cloth calf on viewer-right. Keep this idle exactly.
In this right-facing front-three-quarter view, the character's anatomical RIGHT leg is the FAR leg, visibly emerging from the LEFT side of her pelvis. During ALL SIX existing walking poses it is the BENT REAR LEG at the LEFT side of the image. Its hip, thigh, cargo pocket, knee, mechanical silver calf shield and ankle must form ONE coherent anatomical chain (cyan hip-to-rear-knee guide in input2). The planted FRONT LEG at screen-right is her anatomical LEFT leg and has ordinary DARK BLUE TROUSER CLOTH below the knee in ALL SIX walking poses (pink outline). No phase-dependent swapping.
In walking columns1,2,3: return the current front silver calf to dark blue trouser cloth, and place the SAME silver mechanical shield and ankle on the existing bent rear calf, just as the rear calf is already silver in columns4,5,6. In ALL six walking columns, make the cargo-pocket thigh belong to that SAME REAR RIGHT LEG: attach the pocket to the outer rear thigh emerging from the left hip, behind the plain front thigh; remove any pocket still on the opposite front thigh. Clarify the rear-thigh-to-rear-knee connection under the pelvis so it cannot be mistaken for the front leg. Do not swap hips or knees.
KEEP the current gait: existing hip level, torso position, each knee height, each boot location, ankle bend, lifted rear-foot height and silhouette. Do not generate new walking poses, reduce the leg motion, straighten the bent leg, change the camera, mirror or graft limbs. Preserve the face looking RIGHT, head, hair, torso, arms, empty hands, colors, proportions and pixel-art finish. Both legs retain dark blue cloth thighs; just ONE silver mechanical shin per figure. No naked calves. The pink/cyan guides must not appear.
Return exactly SEVEN COMPLETE figures in the SAME ONE-ROW layout and order, equal scale, full hair/boots, generous transparent margins, TRUE transparent background. No labels, text, ground, background, weapon, extra equipment or effects.'''
(DOC/'prompts/CLASS-10-prosthesis-r07d-right.txt').write_text(prompt,'utf8')
job={'id':'CLASS-10-prosthesis-r07d-right','references':[str(source),str(guide_path),str(ROOT/'docs/reviews/player-walk-06/references/CLASS-10-canonical.png')],
     'prompt':prompt,'destination':str(SRC/'BASTION-TRANSFER-10-r07d-right.png'),
     'retained_left_source':'assets/source-art/player-walk-review-07/BASTION-TRANSFER-10-r07b-left.png',
     'rejected_assumption':'three-frame half-cycle determines anatomical leg ownership'}
(DOC/'prosthesis-job-r07d.json').write_text(json.dumps(job,ensure_ascii=False,indent=2)+'\n','utf8')
