"""Narrow material edits to individual whole poses, never grafted limb patches."""
import json
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from pack_walk07_cycle_repair import strip
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
REF=DOC/'references';figures=strip(SRC/'BASTION-TRANSFER-10-r07c-right.png',7)
jobs=[]
reference=Image.new('RGBA',(512,768));ref=figures[4];reference.alpha_composite(ref,((512-ref.width)//2,(768-ref.height)//2));reference.save(REF/'CLASS-10-r07e-right-rear-material.png')
for col in [1,2,3]:
    figure=figures[col];target=Image.new('RGBA',(512,768));dx=(512-figure.width)//2;dy=(768-figure.height)//2
    target.alpha_composite(figure,(dx,dy));path=REF/f'CLASS-10-r07e-right-target-{col}.png';target.save(path)
    guide=target.copy();draw=ImageDraw.Draw(guide);w,h=figure.size
    # Locate the two calf materials in this individual target, not across rows.
    front=(.43 if col<=2 else .60,.81);rear=(.18,.82)
    for (x,y),color in [(front,'#ff5a9b'),(rear,'#37d6ff')]:
        cx,cy=dx+w*x,dy+h*y
        draw.ellipse((cx-w*.17,cy-h*.10,cx+w*.17,cy+h*.10),outline=color,width=5)
    gp=REF/f'CLASS-10-r07e-right-guide-{col}.png';guide.save(gp)
    prompt='''Use case: precise-object-edit. Edit input1: ONE complete female huntress in ONE walking pose, facing RIGHT. Input2 is an annotation guide for this exact pose. Input3 shows the SAME character with the CORRECT rear-leg materials in a DIFFERENT pose; do not copy its pose. Input4 is canonical character appearance.
Change ONLY leg materials/armor ownership on the existing input1 pose. The character has TWO separate legs. Her PLANTED FRONT leg on screen-RIGHT (pink ring) must have DARK BLUE TROUSER CLOTH BELOW ITS KNEE, with the same dark boot. Remove its silver shin shield. Her BENT REAR leg on screen-LEFT (cyan ring) must have the SILVER MECHANICAL SHIN SHIELD and mechanical ankle under its own rear knee, just like the silver rear leg in input3. Add the silver shield to that bent rear calf, foreshortened along its current angle. ONE silver mechanical calf total. Keep knees silver-blue as before; knees are not calves. Keep both thighs dark blue. The cargo pocket belongs to the rear thigh connected to this same left-side rear knee, never the front thigh; clarify that connection if necessary.
Keep the original exact walking pose of input1: same pelvis, hip and knee positions, same leg bends, same boot positions, same lifted-foot height, same planted foot, silhouette, head looking RIGHT, hair, body, arms, hands, face, colors, outfit and illustration finish. Do not mirror, scale, move, straighten or replace a limb, do not change to the pose of input3. Do not add skin, weapons or gear. The head and torso remain as input1. Return ONE complete whole character, same framing with full hair and both boots. True transparent background; no annotations, rings, text, labels, shadow, floor or backdrop.'''
    promptpath=DOC/f'prompts/CLASS-10-prosthesis-r07e-right-{col}.txt';promptpath.write_text(prompt,'utf8')
    jobs.append({'id':f'CLASS-10-prosthesis-r07e-right-{col}','column':col,'references':[str(path),str(gp),str(REF/'CLASS-10-r07e-right-rear-material.png'),str(ROOT/'docs/reviews/player-walk-06/references/CLASS-10-canonical.png')],
                 'prompt':prompt,'destination':str(SRC/f'CLASS-10-r07e-right-pose-{col}.png')})
(DOC/'prosthesis-jobs-r07e.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
