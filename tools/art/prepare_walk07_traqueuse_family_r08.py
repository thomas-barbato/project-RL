"""Freeze the accepted pilot, then prepare identity-locked whole-pose extensions."""
import hashlib,json
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
read=lambda p:json.loads(p.read_text('utf8'))
write=lambda p,data:p.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n','utf8')
accepted=SRC/'accepted/traqueuse-r07f';accepted.mkdir(parents=True,exist_ok=True)
acceptance=DOC/'visual-acceptance-r07f.json'
if not acceptance.exists():
    atlas=Image.new('RGBA',(448,128))
    for side in range(2):
        sheet=Image.open(SRC/f'rigs/CLASS-10-{side}.png').convert('RGBA')
        for col in range(7):
            body=sheet.crop((0,col*128,128,(col+1)*128))
            for part in [1,3]:body.alpha_composite(sheet.crop((part*128,col*128,(part+1)*128,(col+1)*128)))
            atlas.paste(body.crop((32,32,96,96)),(col*64,side*64))
    native=accepted/'whole-body-native64.png';atlas.save(native)
    files=[SRC/f'BASTION-TRANSFER-10-r07f-{side}.png' for side in ['left','right']]+[native]
    write(acceptance,{'review_id':'WALK07-C11-r07f','date':'2026-10-24','timezone':'Europe/Paris',
        'player_quote':'ok on peut peut passer à la suite','status':'visual_accepted',
        'scope':'Traqueuse outfit1, bilateral metal calves, whole-figure lateral gait, left/right idle and six walking poses; presented equipment composition',
        'all_weapon_pairings_or_other_outfits_accepted':False,'runtime_integration_authorized':False,
        'duration_default_ms':400,'duration_choice_retained':True,'vertical_movement_modified':False,
        'files':[{'file':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in files]})
old=read(ROOT/'docs/reviews/player-walk-06/jobs.json');jobs=[]
details={11:'close blue hood, visible eyes, blue mouth mask, silver forehead band, small shoulder plates and upper-arm guards; retain the partly exposed forearm exactly as input2',
12:'open blue-gray helmet with pale cyan forehead visor and dark mouth mask, visible brown eyes and bun, silver shoulder and chest plates, small hip plates',
13:'closed blue-gray helmet around visible eyes and pale cyan visor strip, silver shoulder/chest and large thigh plates, mostly covered limbs',
14:'fully enclosed heavy silver helmet with cyan visor and NO visible face or hair, bulky silver chest/hip/thigh/shin plates, covered hands and arms'}
for index in [11,12,13,14]:
    for side in ['left','right']:
        prompt=f'''Use case: identity-preserve. Game animation outfit extension of an ACCEPTED whole-figure gait.
INPUT1 is the exact accepted motion/edit target: SEVEN full huntress figures in ONE ROW facing {side.upper()}, column0 idle and columns1..6 the six existing walking poses. INPUT2 supplies the EXACT OUTFIT identity for CLASS-{index:02}, Traqueuse outfit{index%5+1}: {details[index]}.
Redraw the complete figures of INPUT1 wearing the precise outfit/helmet of INPUT2. Preserve that outfit's silhouette, armor-stage coverage, plate shapes, colors, materials, visible face/hair or closed helmet, sleeves, gloves and proportions. Do not copy the guide's uncovered head/arms onto a covered outfit. Use INPUT2 for appearance ONLY, never its stationary leg pose or head direction.
Keep INPUT1's whole-body motion and camera: the same hip, knee and boot locations, compact crossing/raised foot positions, six-frame phase order, measured movement and true two-feet-grounded idle. Head/eyes/helmet visor, torso, knees and boots ALL face {side.upper()} in the same near-front three-quarter view. Both shoulders/chest remain visible. No head-only turn, no front-facing body with sideways head, no back or pure profile.
BOTH lower legs have MATCHING SILVER METAL calf armor and articulated ankles in EVERY pose, including idle, adapted to this outfit's plate design. No single cloth-only calf and no alternating armor. Preserve the blue cloth and other existing armor above the knees. Draw two COMPLETE relaxed empty arms with closed hands beside the hips, elbows a little away from the torso; wrists, sleeves and gloves connect without gaps. Arms remain available for later weapon attachment. No weapon or rifle from any reference.
Exactly SEVEN COMPLETE figures in ONE ROW, same scale and consistent height, full helmet/hair and both boots, generous transparent gutters, same seven-column order as INPUT1. Preserve the matte native64-readable game-art finish. True transparent background with clean cutout: no ground, shadow, halo, labels, text, extra equipment or effects. Do not redesign the gait.'''
        name=f'CLASS-{index:02}-family-r08-{side}'
        (DOC/'prompts'/f'{name}.txt').write_text(prompt,'utf8')
        jobs.append({'id':name,'index':index,'side':side,'references':[str(SRC/f'BASTION-TRANSFER-10-r07f-{side}.png'),old[index]['identity_reference']],
                     'prompt':prompt,'destination':str(SRC/f'BASTION-TRANSFER-{index:02}-r08-{side}.png')})
write(DOC/'traqueuse-family-jobs-r08.json',jobs)
print(json.dumps({'accepted_pilot':'WALK07-C11-r07f','outfits_to_review':[11,12,13,14],'directional_jobs':len(jobs)}))
