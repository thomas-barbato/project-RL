"""Use the exact original canvas02 whole-body poses as the only motion guide."""
import hashlib,json
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';REF=DOC/'references';ART=ROOT/'artifacts/player-walk-review-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
walk=Image.open(REF/'exact-original-canvas02-atlas.png').convert('RGBA');idle=Image.open(REF/'exact-original-canvas02-idle.png').convert('RGBA')
assert walk.tobytes()==Image.open(ROOT/'assets/source-art/player-walk-review-01/accepted/native64/bastion-walk-native64.png').convert('RGBA').tobytes()
board=Image.new('RGB',(1792,576),'#293132');d=ImageDraw.Draw(board);font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',18)
jobs=[]
for side,row in [('left',7),('right',1)]:
    tiles=[idle.crop((row%4*64,row//4*64,row%4*64+64,row//4*64+64))]+[walk.crop((n*64,row*64,n*64+64,row*64+64)) for n in range(6)]
    target=Image.new('RGBA',(1792,600));native=Image.new('RGBA',(448,64))
    for n,tile in enumerate(tiles):
        native.paste(tile,(n*64,0));large=tile.resize((256,256),Image.Resampling.NEAREST);target.paste(large,(n*256,144))
        by=0 if side=='left' else 288;board.paste(large,(n*256,by+32),large)
        d.text((n*256+4,by+4),'BASTION '+side+' / '+('repos' if n==0 else str(n-1)),font=font,fill='#eeeade')
    path=REF/f'EXACT-BASTION-r07-{side}-motion.png';target.save(path);native.save(REF/f'EXACT-BASTION-r07-{side}-native.png')
    for index,name in [(10,'Traqueuse'),(24,'Metamorphe')]:
        identity=ROOT/f'docs/reviews/player-walk-06/references/CLASS-{index:02}-canonical.png'
        slug='traqueuse' if index==10 else 'metamorphe';bank=Image.open(ROOT/f'assets/animations/walk-06/{slug}-rifle.png').convert('RGBA')
        start=(index%5)*128+(0 if side=='left' else 64)
        head=bank.crop((0,start,64,start+23));head_path=REF/f'CLASS-{index:02}-r07-{side}-head-direction.png'
        head.resize((512,184),Image.Resampling.NEAREST).save(head_path)
        identity_rule=("This is the blue-gray cybernetic huntress with bun. Preserve her one bare upper arm, mechanical LEFT forearm/hand, and mechanical RIGHT lower leg, all attached to their OWN correct anatomical shoulder or hip through every step. Do not swap which thigh owns the silver shin. Its screen position may naturally change as the same leg swings in three-quarter view: preserve its hip-to-knee-to-ankle connection, not a fixed left-screen paint patch. The other leg remains cloth with ordinary boot."
            if index==10 else "This is the ivory/olive armored druid with brown hair, fur collar and stocky proportions. Keep the same mass, head scale, armor construction, waist panels, gauntlets and boots through all steps. Do not replace the outfit with Bastion armor.")
        prompt=f'''Use case: identity-preserve / whole-body animation transfer.
INPUT1 IS THE EDIT TARGET AND EXACT MOVEMENT REFERENCE: seven complete Bastion game sprites, one horizontal row, first idle then six distinct walking phases. They ALL face SCREEN {side.upper()}. This exact movement is the user's accepted animation. INPUT2 IS ONLY THE NEW CHARACTER IDENTITY AND OUTFIT: {name}. Do not copy input2's standing pose or preferred face direction. INPUT3 is ONLY the character's correct head direction, looking SCREEN {side.upper()}; retain that direction while the body follows INPUT1.
Replace the ENTIRE Bastion figure in EACH of the SEVEN corresponding cells of INPUT1 with the character of INPUT2, preserving EACH exact WHOLE-BODY POSE and orientation of INPUT1. COPY the Bastion's hip direction, thigh angles, knee flexion, ankle trajectories, foot contacts, low passing foot, stride amplitude, chest orientation, head direction and frame order. Keep BOTH shoulders and the front of the chest visible as in INPUT1, a near-front THREE-QUARTER turn, not profile or back. The face, chest, hips, knees and toes all follow SCREEN {side.upper()} together. Do not rotate only the head. Do not make frontal stationary figures lifting their feet in place. Keep the actual forward/back leg swing and measured WALK of the Bastion.
{identity_rule}
Preserve the NEW character's own build, height proportions, face or hair, outfit plates/materials/colors. Draw each character as ONE coherent complete figure, same identity in all seven. First pose idle with both feet flat. Next six poses COPY INPUT1 one-to-one: two distinct alternate supporting legs. Do not repeat one step twice, do not replace the motion with marching or shuffling. Keep input1's stable head/hip height and body placement. No shrinking, widening, big split, running or high knee.
Draw two complete connected arms with empty closed hands at lower chest, ABOVE the waist, modestly bent elbows, original outfit sleeves, no missing arm strips or old gripping hand. Weapons are separate game layers added later. This arm preparation MUST NOT change the legs or turn the chest away from input1's orientation.
Keep the same small-game pixel-art finish, controlled contours and block shapes as INPUT1. True transparent background. EXACTLY SEVEN FULL FIGURES IN ONE ROW, same corresponding spacing and order as INPUT1, full hair/head and both boots, generous alpha margins. No additional rows or poses, no labels/text, no guide lines, grid, floor, shadow, weapon, glow or effects.'''
        prompt_file=DOC/f'prompts/BASTION-TRANSFER-{index:02}-r07-{side}.txt';prompt_file.write_text(prompt,'utf8')
        jobs.append({'id':f'CLASS-{index:02}-{side}','index':index,'side':side,'references':[str(path),str(identity),str(head_path)],'prompt':prompt,'prompt_file':str(prompt_file),'destination':str(SRC/f'BASTION-TRANSFER-{index:02}-r07-{side}.png')})
board.save(ART/'exact-bastion-r07-whole-poses-x4.png')
(DOC/'bastion-transfer-jobs-r07.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
print(json.dumps([{'id':j['id'],'references':j['references']} for j in jobs],indent=2))
