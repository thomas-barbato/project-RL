"""Prepare bounded weapon/forearm restoration; approved gait is immutable."""
import json
from pathlib import Path
from PIL import Image,ImageDraw
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';REF=DOC/'references'
jobs=[]
for index,slug,stage in [(10,'traqueuse',0),(24,'metamorphe',4)]:
    atlas=Image.open(ROOT/f'assets/animations/walk-06/{slug}-rifle.png').convert('RGBA')
    for side,row in [('left',0),('right',1)]:
        base=atlas.crop((0,stage*128+row*64,448,stage*128+row*64+64))
        native=REF/f'CLASS-{index:02}-accepted-{side}-native.png';base.save(native)
        guide=Image.new('RGBA',(1792,600))
        guide.alpha_composite(base.resize((1792,256),Image.Resampling.NEAREST),(0,144))
        target=REF/f'CLASS-{index:02}-retouch-{side}-input.png';guide.save(target)
        mask=guide.copy();draw=ImageDraw.Draw(mask)
        for col in range(7):draw.rectangle((col*256,144+20*4,(col+1)*256-1,144+35*4-1),fill=(255,0,255,255))
        mask_path=REF/f'CLASS-{index:02}-retouch-{side}-region.png';mask.save(mask_path)
        prompt=f'''Use case: precise-object-edit, local sprite restoration.
INPUT1 is the EDIT TARGET: seven ALREADY APPROVED small pixel-art animation frames of ONE character, all facing screen{side.upper()}, one row. INPUT2 is a REGION GUIDE: its magenta bands mark ONLY the rifle, gripping hands, forearms and lower chest that require restoration. The magenta is not part of the design.
Change ONLY that marked narrow upper-body band: remove the rifle and its remaining old gripping-finger fragments completely. Restore the continuous original chest armor/clothing behind it. Draw two complete anatomically joined forearms and CLOSED EMPTY hands at belt/chest height, elbows modestly bent; keep hands high enough to remain inside the marked band. Both arms belong to the original character, connected to the original shoulders. Do not leave holes, an extra hand or weapon remnants.
Keep the EXACT original character, armor stage, colors, materials, build, pixel-art finish, head, hair/helmet, shoulders and pose in EACH corresponding frame. All SEVEN heads, faces and bodies continue pointing screen{side.upper()} as in INPUT1, near-front three quarters, NOT side profile. Preserve the original seven-frame order and positions.
The legs, hips, knees, boots, stance, foot positions, body height and whole existing walking animation are ALREADY CORRECT. Do not redraw, reposition, bend, rescale, shorten or animate any leg. Outside the marked band keep INPUT1 unchanged. Keep the same source pixel density and block shapes: this is a LOCAL RESTORATION, not new high-resolution character art or new animation.
Output the SAME SEVEN COMPLETE figures in one horizontal row with the same relative layout and generous transparency. No extra frames or rows. Preserve all heads and both complete boots. No added weapon, equipment, labels, magenta, guides, floor, shadows, background or effects. True transparent background. Only restore the masked rifle/forearm/chest area.'''
        job={'index':index,'id':f'CLASS-{index:02}','slug':slug,'stage':stage,'side':side,'native':str(native),
            'target':str(target),'region_guide':str(mask_path),'edit_band_native':[20,35],
            'destination':str(ROOT/f'assets/source-art/player-walk-review-07/LOCAL-{index:02}-r05-{side}.png'),'prompt':prompt}
        jobs.append(job);(DOC/'prompts'/f'LOCAL-{index:02}-r05-{side}.txt').write_text(prompt,'utf8')
(DOC/'local-retouch-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
print('4 upper-body restoration jobs; accepted gait pixels protected')
