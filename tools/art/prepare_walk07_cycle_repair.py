"""Pose-locked repairs for the two reported class families; review only."""
import json
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07';REF=DOC/'references'
original=json.loads((ROOT/'docs/reviews/player-walk-06/jobs.json').read_text('utf8'))
repairs=[];board=Image.new('RGB',(5*256,2*512),'#293132')
draw=ImageDraw.Draw(board);font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',18)
for n,index in enumerate([10,11,12,13,14,20,21,22,23,24]):
    identity=Path(original[index]['identity_reference'])
    im=Image.open(identity).convert('RGBA');im.thumbnail((256,470),Image.Resampling.LANCZOS)
    board.paste(im,(n%5*256+(256-im.width)//2,n//5*512+32),im)
    draw.text((n%5*256+6,n//5*512+5),f'CLASS-{index:02}',font=font,fill='#eeeade')
    prompt=f'''Use case: identity-preserve, exact pose transfer.
Create a replacement UNARMED lateral walk atlas for the existing game character CLASS-{index:02}, {original[index]['label']}.
INPUT1 is the APPROVED Bastion walk and is the STRICT POSE/GAIT/CAMERA reference. INPUT2 is the character IDENTITY reference. Change the character's appearance to INPUT2 while faithfully copying INPUT1's compact leg sequence and two near-front three-quarter orientations. Preserve the exact face/helmet, hair, build, armor stage, armor pieces, materials, colors, clothes length, knee protection and boots of INPUT2. No new equipment, no redesign, no copying Bastion's orange armor onto this character.
Output EXACTLY EIGHT COLUMNS and TWO ROWS,16 COMPLETE separate full-body figures on true transparency, wide landscape atlas. Do not use four rows. Top row all front-LEFT; bottom row all front-RIGHT. In each row col0=stable IDLE, cols1..6=the SIX SUCCESSIVE WALK PHASES from INPUT1 W0..W5, col7=duplicate IDLE. Copy each corresponding guide pose, do not replace the second half of the walk with another first half. Same near-front viewpoint, front of chest and both shoulders visible. Head, pelvis, knees and toes all face the row's direction.
LEG REQUIREMENT: two anatomical legs retain their identity throughout. Each complete cycle has opposite supports: one foot bears weight as the other passes CLOSE to it, then their roles reverse. The second three phases use the OPPOSITE planted leg. Copy guide shoe positions, knee bends and overlaps pose by pose; short strides with low feet, no kick, run, high knee, split, skating, shortened legs, fused knees, backwards knees, crossed shins or foot that changes anatomical side. Both legs always connect at the pelvis, both knees and BOTH COMPLETE BOOTS visible wherever not naturally occluded. In both IDLE poses both soles rest on the same ground line.
Upper body upright and quiet. Both EMPTY hands relaxed alongside the hips with complete arms; keep the same safe readiness as the unarmed guide, no rifle, weapon, shield, holster, prop or shadow. Preserve whole silhouettes: never erase, cut off or replace a lower body.
CONSISTENT SCALE: all16 figures are the SAME standing size. Identical head size, shoulder level, pelvis level, thigh/shin length and boot size across all columns and both rows; only compact joint angles change. Never scale down the last poses or the bottom row. The guide is a motion reference, not a reason to make a slim character bulky.
FRAMING: ample transparent padding around EVERY full figure, especially BELOW the boots of the BOTTOM ROW. Eight equally spaced columns, two equally spaced rows. Figures occupy at most65% of each row height and65% of each column width. Entire head, hands, knees and boot soles must fit. The bottom row must have a large transparent margin under EVERY boot, with no body intersecting the canvas edge. Do not enlarge drawings to fill available height.
Matte crisp native64-readable game art matching INPUT2's existing finish. No labels, numbers, captions, guide marks, grids, background, ground, particles or cast shadows. Exactly one identity and16 FULL figures. True transparent background.'''
    repair={'id':f'UNARMED-{index:02}','index':index,'reference':str(REF/'bastion-approved-8x2-guide.png'),
        'identity_reference':str(identity),'source_destination':str(ROOT/f'assets/source-art/player-walk-review-07/UNARMED-{index:02}-r02.png'),
        'columns':8,'rows':2,'prompt':prompt}
    repairs.append(repair);(DOC/'prompts'/f'UNARMED-{index:02}-r02.txt').write_text(prompt,'utf8')
(DOC/'cycle-repair-jobs.json').write_text(json.dumps(repairs,ensure_ascii=False,indent=2)+'\n','utf8')
board.save(REF/'reported-family-identity-overview.png')
print(json.dumps({'repair_cycles':len(repairs),'strict_pose_reference':'Bastion approved','layout':[8,2]}))

# Two-direction generations ignored the row-facing instruction; create one
# direction per request, with that exact direction's own standing reference.
from build_player_walk_matrix_06 import grid_figures
guide=Image.open(REF/'bastion-approved-8x2-guide.png').convert('RGBA')
direction_jobs=[]
for job in repairs:
    index=job['index']
    figures,_=grid_figures(Image.open(ROOT/f'assets/source-art/player-walk-review-07/UNARMED-{index:02}-r01.png').convert('RGBA'))
    for side,row in [('left',0),('right',1)]:
        gait=REF/f'bastion-approved-{side}-guide.png'
        guide.crop((0,row*256,1536,row*256+256)).save(gait)
        identity=REF/f'UNARMED-{index:02}-{side}-rest-reference.png'
        figures[row*8].save(identity)
        prompt=f'''Use case: identity-preserve, exact pose transfer. Create ONE DIRECTION walk cycle for the SINGLE EXACT character in INPUT2.
INPUT1 is eight approved Bastion poses: the strict leg-joint, foot-position and gait guide. INPUT2 is the exact character, appearance AND REQUIRED facing.
EVERY FIGURE faces SCREEN {side.upper()}, exactly like INPUT2. All eight noses point toward the {side.upper()} EDGE of the image; chest, hips, knees and toes follow that SAME {side.upper()} direction. Near-front three quarters, both shoulders and chest front visible. No back or pure side view. Do not provide the opposite direction or a second row.
Draw precisely EIGHT COMPLETE figures in ONE HORIZONTAL ROW, separated on a transparent landscape canvas. Read left to right: STANDING IDLE, walk0, walk1, walk2, walk3, walk4, walk5, duplicate STANDING IDLE. Col1..6 copy the corresponding six WALK POSES in INPUT1. In walk0..2 one leg is the support; in walk3..5 the OTHER leg is the support. Feet alternate and pass closely, heels lift only a little; small natural walking, never a kick or high knee. Do not repeat the first three steps as the last three. Preserve two connected thighs, two shins and two complete boots. Keep armor markings on their correct anatomical leg and proper knees, without crossed shins or sliding.
Change the Bastion appearance into the exact INPUT2 character while copying the corresponding compact walking joint positions. Copy only the guide motion; preserve INPUT2 face, hair, armor stage, build, clothes length, gloves, asymmetry, knee armor and boots. No orange Bastion armor and no redesign.
All eight figures SAME head size, body proportions, shoulder and pelvis heights, leg lengths and boot size. Upright quiet torso, complete relaxed EMPTY arms, hands beside hips, no weapon. Both idle feet fully on the same baseline. Draw the ENTIRE actual figure independently in each pose, never cut the lower body or invent legs.
Framing: moderate equal figure height with LARGE transparent margins above the head and below EVERY boot. Eight equal columns in one row. Every head, hand and boot sole fully complete and padded. No body crossing the image boundary. No text, labels, background, grid, ground or shadow. Matte crisp native64-readable game finish matching INPUT2. True transparency.'''
        entry={'id':job['id'],'index':index,'side':side,'reference':str(gait),'identity_reference':str(identity),
            'source_destination':str(ROOT/f'assets/source-art/player-walk-review-07/UNARMED-{index:02}-r03-{side}.png'),'prompt':prompt}
        direction_jobs.append(entry)
        (DOC/'prompts'/f'UNARMED-{index:02}-r03-{side}.txt').write_text(prompt,'utf8')
(DOC/'direction-repair-jobs.json').write_text(json.dumps(direction_jobs,ensure_ascii=False,indent=2)+'\n','utf8')
