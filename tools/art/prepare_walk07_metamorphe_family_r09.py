"""Record Traqueuse family approval and extend the presented Metamorphe gait."""
import hashlib,json
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
read=lambda p:json.loads(p.read_text('utf8'))
write=lambda p,d:p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
acceptance=DOC/'visual-acceptance-traqueuse-r08.json'
if not acceptance.exists():
    accepted=SRC/'accepted/traqueuse-r08';accepted.mkdir(parents=True,exist_ok=True);files=[]
    for index in range(11,15):
        atlas=Image.new('RGBA',(448,128))
        for side in range(2):
            sheet=Image.open(SRC/f'rigs/CLASS-{index:02}-{side}.png').convert('RGBA')
            for col in range(7):
                body=sheet.crop((0,col*128,128,(col+1)*128))
                for part in [1,3]:body.alpha_composite(sheet.crop((part*128,col*128,(part+1)*128,(col+1)*128)))
                atlas.paste(body.crop((32,32,96,96)),(col*64,side*64))
        path=accepted/f'CLASS-{index:02}-whole-body-native64.png';atlas.save(path);files.append(path)
        files.extend(SRC/f'BASTION-TRANSFER-{index:02}-r08-{side}.png' for side in ['left','right'])
    write(acceptance,{'review_id':'WALK07-TR-FAMILY-r08','date':'2026-10-24','timezone':'Europe/Paris',
        'player_quote':'ok on peut continuer','status':'visual_accepted','class_indices':[11,12,13,14],
        'scope':'Four additional Traqueuse outfits, whole-figure left/right idle and six walking poses, bilateral metal calves, presented equipment compositions',
        'runtime_integration_authorized':False,'all_possible_weapon_pairs_helmets_palettes_accepted':False,
        'duration_default_ms':400,'duration_choice_retained':True,'vertical_movement_modified':False,
        'files':[{'file':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in files]})
revision=read(DOC/'review-revision.json');revision['accepted_class_indices']=list(range(10,15));revision['awaiting_visual_class_indices']=[24]
revision['accepted_family_reviews']={'traqueuse':['visual-acceptance-r07f.json','visual-acceptance-traqueuse-r08.json']}
write(DOC/'review-revision.json',revision)
old=read(ROOT/'docs/reviews/player-walk-06/jobs.json');jobs=[]
details={
20:'uncovered tousled brown hair, NO headband, NO fur collar, sleeveless olive wrap tunic and bare upper arms, olive/dark cloth thighs, olive knee pads and wrapped leather boots; no ivory chest, shoulder or shin plates and no bone helmet',
21:'olive headband and visible brown hair/face, light ivory fur collar, short olive leaf shoulder cape, small ivory chest/waist plates, bare upper arms, mostly olive cloth legs with ivory knee and ankle guards',
22:'olive headband and brown hair/face, broader ivory fur collar and olive leaf mantle, larger ivory breastplate and waist plates, bare upper arms with ivory bracers, ivory knee and shin plates over olive trousers',
23:'olive headband with small ivory temple guards and visible hair/face, fur collar and leaf mantle, ivory shoulder/chest/upper-arm/thigh/knee/shin plates, covered olive sleeves, articulated leather boots'}
for index in range(20,24):
    for side in ['left','right']:
        prompt=f'''Use case: identity-preserve. Game animation outfit extension.
INPUT1 is the exact seven-pose MOTION/edit target: a whole Metamorphe character in ONE ROW facing {side.upper()}, idle then six walking poses. This is the previously presented heavy outfit5, NOT the outfit to reproduce. INPUT2 is the exact canonical appearance for Metamorphe outfit{index%5+1}: {details[index]}.
Redraw all SEVEN COMPLETE figures using INPUT2's outfit and character identity. Preserve its face, hair/headband or absence of headband, cloth coverage, armor-stage plate shapes, materials and olive/brown/ivory colors. Remove every heavy-outfit plate, fur or accessory from INPUT1 that is absent from INPUT2. Ignore any brown backdrop/glow in INPUT2; it is not part of the sprite.
Retain INPUT1's existing whole-body gait, compact supporting and raised foot positions, hip/knee/ankle locations, idle with both feet flat, camera and frame order. Keep the same underlying muscular build with natural armor bulk appropriate to INPUT2. Constant standing height across all seven figures; no swelling or shrinking. All faces/noses, torso, knees and boots point {side.upper()} together in near-front three quarters, showing chest front and both shoulders. No head-only rotation, profile or back.
Preserve this outfit's OWN leg materials. Cloth/leather stays cloth/leather; ivory plates stay ivory plates. Do NOT copy the Traqueuse's silver metal legs, robotic limbs or blue equipment. Draw two complete empty relaxed arms with closed hands beside the hips, elbows slightly clear of the torso, no holes or extra hands; keep INPUT2's sleeves, bracers and gloves. Weapons will be attached separately.
Exactly SEVEN complete figures in ONE ROW, same order as INPUT1, consistent scale, full hair and boots with generous transparent margins. Matte native64-readable game-art finish matching the motion reference. Genuine transparent cutout with no background, halo, ground, shadow, labels, text, weapons, extra equipment or effects. No new gait.'''
        name=f'CLASS-{index:02}-family-r09-{side}';(DOC/'prompts'/f'{name}.txt').write_text(prompt,'utf8')
        jobs.append({'id':name,'index':index,'side':side,
            'references':[str(SRC/f'BASTION-TRANSFER-24-r07-{side}.png'),old[index]['identity_reference']],
            'prompt':prompt,'destination':str(SRC/f'BASTION-TRANSFER-{index:02}-r09-{side}.png'),
            'scope':'Whole-figure Metamorphe outfit extension; presented outfit5 motion, canonical materials; review only'})
write(DOC/'metamorphe-family-jobs-r09.json',jobs)
print(json.dumps({'accepted_traqueuse_outfits':[11,12,13,14],'metamorphe_pilot_accepted':False,'new_directional_jobs':len(jobs)}))
