"""Locate the reported shin material errors in their own whole-figure poses."""
import json
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from build_player_walk_review_04 import components
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';REF=DOC/'references';SRC=ROOT/'assets/source-art/player-walk-review-07'
source=SRC/'BASTION-TRANSFER-10-r07-left.png';im=Image.open(source).convert('RGBA');guide=im.copy();d=ImageDraw.Draw(guide);font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',22)
parts=sorted(components(im),key=lambda p:p['area'],reverse=True)[:7];parts.sort(key=lambda p:p['center'][0])
for n,p in enumerate(parts):
    x0,y0,x1,y1=p['box'];w,h=x1-x0,y1-y0
    d.text((x0,max(8,y0-44)),str(n+1)+(' RESTORE SILVER' if n in [2,4,6] else 'KEEP SILVER'),font=font,fill='#f7eded')
    path=[(x0+w*.4,y0+h*.56),(x0+w*.43,y0+h*.72),(x0+w*.29,y0+h*.91)]
    d.line(path,fill='#36c6f2',width=5)
    for x,y in path:d.ellipse((x-5,y-5,x+5,y+5),fill='#36c6f2')
    if n in [2,4,6]:
        d.polygon([(x0+w*.3,y0+h*.72),(x0+w*.54,y0+h*.72),(x0+w*.4,y0+h*.93),(x0+w*.16,y0+h*.93)],outline='#f447bd',width=7)
guide_path=REF/'CLASS-10-r07b-prosthesis-guide.png';guide.save(guide_path)
prompt='''Use case: precise-object-edit. INPUT1 is the EDIT TARGET: exactly SEVEN whole huntress figures in one row, idle then six walking poses. INPUT2 is ONLY a correction guide: blue lines trace the SAME RIGHT HIP -> RIGHT KNEE -> RIGHT ANKLE through each pose; magenta outlines the three incorrectly CLOTH-painted right shins. INPUT3 is the canonical outfit/material reference. Do not draw guide lines in the result.
Fix the SAME anatomical right leg, connected to the CARGO-POCKET THIGH on the LEFT side of the pelvis in this near-front-left view. The front planted shin immediately under that thigh must be the SILVER MECHANICAL SHIN in ALL SEVEN figures. It is currently correct in cells1,2,4,6, but incorrectly becomes dark cloth in cells3,5,7 (counting from1 on the left).
In cells3,5,7 ONLY, replace the DARK FRONT SHIN under the planted front knee with the SAME silver mechanical shin/ankle design already visible in the adjacent correct figures: bright silver shield plate, dark mechanical joint and ankle, connected continuously to its own knee and same pocket thigh. The bent BACK leg on the RIGHT remains ordinary DARK TROUSER CLOTH and normal boot. Do NOT put the prosthesis on the bent rear leg. Each figure has ONE prosthetic shin only.
Keep the full gait geometry EXACTLY: the existing hips, thighs, knees, calves, ankles, boot positions, leg swing, outlines and ground contacts in every corresponding cell must NOT move. No new pose, no head turn, no mirroring, no raised or shortened leg, no swap of thigh connections. Preserve all other pixels/art as closely as possible, including faces looking LEFT, torso orientation, empty arms and hands, blue gray outfit, bun, body size, original pixel clusters and frame order. Cells1,2,4,6 need no modification at all. This is a precise armor MATERIAL CORRECTION on the SAME existing whole figures, not a new walk atlas.
Output the same SEVEN COMPLETE figures in ONE ROW, same layout, equal scale, full hair and boots, generous transparent margins, TRUE TRANSPARENT background. No guides, labels, text, background, floor, shadow, weapon, effect or new gear.'''
job={'id':'CLASS-10-prosthesis-r07b','references':[str(source),str(guide_path),str(ROOT/'docs/reviews/player-walk-06/references/CLASS-10-canonical.png')],
    'prompt':prompt,'destination':str(SRC/'BASTION-TRANSFER-10-r07b-left.png'),'replaces':'BASTION-TRANSFER-10-r07-left.png','specific_cells_to_correct':[3,5,7]}
(DOC/'prompts/CLASS-10-prosthesis-r07b.txt').write_text(prompt,'utf8')
(DOC/'prosthesis-job-r07b.json').write_text(json.dumps(job,ensure_ascii=False,indent=2)+'\n','utf8')
