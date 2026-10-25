"""Prepare narrowly scoped full-figure edits and anatomical support guides."""
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
REF=DOC/'references'
SRC=ROOT/'assets/source-art/player-walk-review-07'
FONT=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',20)
SCALE=4
W,H=1792,768
JOBS=[]
assert json.loads((DOC/'review-revision.json').read_text('utf8'))['revision']=='r05-local-retouch-frozen-approved-gait','Historical r05 input preparation only; never replace the saved previous-cycle reference with r06'
for index,name in [(10,'Traqueuse'),(24,'Metamorphe')]:
    target=Image.new('RGBA',(W,H))
    guide=Image.new('RGB',(W,H),'#252d32')
    draw=ImageDraw.Draw(guide)
    previous=Image.new('RGBA',(448,128))
    for side in range(2):
        rig=Image.open(SRC/f'rigs/CLASS-{index:02}-{side}.png').convert('RGBA')
        for col in range(7):
            row=rig.crop((0,col*128,640,(col+1)*128))
            body=row.crop((0,0,128,128))
            for arm in [1,3]:body.alpha_composite(row.crop((arm*128,0,(arm+1)*128,128)))
            native=body.crop((32,32,96,96))
            previous.paste(native,(col*64,side*64))
            large=native.resize((256,256),Image.Resampling.NEAREST)
            x,y=col*256,side*384+80
            target.alpha_composite(large,(x,y))
            guide.paste(large,(x,y),large)
            draw.text((x+6,side*384+12),('LEFT' if side==0 else 'RIGHT')+' / '+['IDLE','A0 R planted','A1 R support','A2 L reaches','B0 L planted','B1 L support','B2 R reaches'][col],font=FONT,fill='#f2f0e8')
            # R is the character's anatomical RIGHT: always viewer-left in
            # these near-frontal views. L always viewer-right. No body mirroring.
            feet=[((28,61),(36,61)),((27,61),(36,59)),((28,61),(35,58)),
                  ((29,60),(34,61)),((29,59),(35,61)),((28,58),(36,61)),((27,61),(36,60))][col]
            if side:feet=tuple((2*base-p[0],p[1]) for base,p in zip([28,36],feet))
            for hip,foot,color in zip([(29,35),(35,35)],feet,['#32b9fa','#f49c45']):
                knee=((hip[0]+foot[0])//2,(hip[1]+foot[1])//2)
                points=[(x+px*4,y+py*4) for px,py in [hip,knee,foot]]
                draw.line(points,fill=color,width=5)
                for px,py in points:draw.ellipse((px-5,py-5,px+5,py+5),fill=color)
            draw.line((x+72,y+248,x+184,y+248),fill='#b4bbc0',width=1)
    target_path=REF/f'CLASS-{index:02}-r06-edit-target.png'
    guide_path=REF/f'CLASS-{index:02}-r06-anatomy-guide.png'
    previous.save(REF/f'CLASS-{index:02}-r05-unarmed-cycle.png')
    target.save(target_path);guide.save(guide_path)
    identity=ROOT/f'docs/reviews/player-walk-06/references/CLASS-{index:02}-canonical.png'
    identity_rule=(
        "Her ONE mechanical lower leg is her anatomical RIGHT leg: viewer-LEFT thigh/knee/shin/boot in BOTH near-frontal orientations. Keep its silver shin and ankle attached continuously to that SAME right hip through ALL frames. The anatomical LEFT leg, viewer-RIGHT, stays dark cloth with a small knee pad and normal boot in ALL frames. Her mechanical ARM is anatomical LEFT, viewer-RIGHT; her bare arm stays viewer-LEFT. Do not mirror the armor or cybernetic parts when turning the face. Keep her original blue-gray outfit and bun."
        if index==10 else
        "Keep the exact stocky armored druid identity, original tan/ivory/olive armor plates, brown hair, gauntlets, waist panels and complete boots from image3. Same body mass, head size and leg length in every pose. The current first three walking poses repeat their support in the next three; FIX that repetition: A is RIGHT-leg support, B is LEFT-leg support."
    )
    prompt=f'''Use case: precise-object-edit / identity-preserve.
Asset: corrected compact lateral walking sprite atlas of {name}, for native64 game review.
INPUT1 is the EDIT TARGET, fourteen complete figures: exactly TWO rows, SEVEN columns. Top row faces screenLEFT, bottom row screenRIGHT. Each row is IDLE then six walking frames A0,A1,A2,B0,B1,B2. Preserve this exact layout and corresponding upper-body pose, near-front three-quarter face direction and source pixel-art finish.
INPUT2 is only an ANATOMICAL MOTION GUIDE at the same cell positions. Blue identifies anatomical RIGHT leg, orange identifies anatomical LEFT leg, NOT final colors. Each line connects its OWN hip-knee-foot. Right leg stays viewer-left, left leg viewer-right; legs must NOT cross or swap. The small compact vertical/depth changes indicate distinct grounded support and low passing foot. The guide's text, colors and lines must NOT appear in the result.
INPUT3 is the exact character/armor IDENTITY reference. {identity_rule}
Correct the lower-body anatomy/material continuity and the alternating supports in the WHOLE coherent figures of INPUT1. Keep its head, upper torso, shoulders, arms, raised empty hands and armor identity as close to unchanged as possible. Use the same compact step amplitude as INPUT1, less than one boot length; both thighs remain connected to their own hips. Minimal bob: fixed head height, stable hips and sole baseline. No body size changes.
In each row: IDLE both feet flat; A0 anatomical RIGHT foot plants forward with LEFT trailing low; A1 RIGHT supports, LEFT foot passes low just above ground; A2 LEFT reaches the next contact while RIGHT trails; B0 LEFT plants while RIGHT trails low; B1 LEFT supports, RIGHT passes low; B2 RIGHT reaches while LEFT trails. This MUST be two alternating steps, not the same step copied twice. Keep the front of the chest, both shoulders and both legs visible; never a side profile or back view.
Keep the original near-front pixel density and crisp small-game-art clusters, not a smoother detailed illustration. Draw complete coherent figures for each cell. Do not copy a leg from another character. No weapons, extra equipment, extra limbs, missing knees/ankles, big splits, high kicks, running, crouching, twisted feet, crossed legs or drifting silver parts.
Output only the fourteen COMPLETE figures in exactly the same TWO rows / SEVEN columns on actual TRANSPARENT background, generous gutters, equal figure scale, complete head and boots. No text, labels, arrows, skeleton guides, grid, floor, shadow, effects or background.'''
    prompt_path=DOC/f'prompts/ANATOMY-{index:02}-r06.txt'
    prompt_path.write_text(prompt,'utf8')
    JOBS.append({'index':index,'id':f'CLASS-{index:02}','name':name,
        'references':[str(target_path),str(guide_path),str(identity)],
        'destination':str(SRC/f'ANATOMY-{index:02}-r06.png'),
        'prompt_file':str(prompt_path),'prompt':prompt})
(DOC/'anatomy-jobs-r06.json').write_text(json.dumps(JOBS,ensure_ascii=False,indent=2)+'\n','utf8')
print(json.dumps([{'id':j['id'],'references':j['references']} for j in JOBS],indent=2))
