"""An unambiguous diagram guide, without the rejected crossed-leg drawings."""
import json
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';REF=DOC/'references'
guide=Image.new('RGB',(1792,768),'#252d32');d=ImageDraw.Draw(guide)
font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',18)
for side in range(2):
 for col in range(7):
    x,y=col*256,side*384+64
    xy=lambda p:(x+p[0]*4,y+p[1]*4)
    d.text((x+6,side*384+8),('LEFT' if side==0 else 'RIGHT')+' / '+['IDLE','A0 R planted','A1 R support','A2 L reaches','B0 L planted','B1 L support','B2 R reaches'][col],font=font,fill='#f2f0e8')
    d.ellipse((x+26*4,y+5*4,x+38*4,y+18*4),outline='#c7ccd0',width=4)
    nose=[(27,11),(24,13),(27,14)] if side==0 else [(37,11),(40,13),(37,14)]
    d.line([xy(p) for p in nose],fill='#c7ccd0',width=4)
    d.polygon([xy(p) for p in [(25,20),(39,20),(37,34),(27,34)]],outline='#c7ccd0',width=4)
    d.line([xy(p) for p in [(25,21),(22,29),(28,27)]],fill='#c7ccd0',width=4)
    d.line([xy(p) for p in [(39,21),(42,29),(36,27)]],fill='#c7ccd0',width=4)
    feet=[((27,61),(37,61)),((26,61),(38,60)),((27,61),(37,58)),((28,60),(36,61)),
          ((28,60),(36,61)),((27,58),(37,61)),((26,61),(38,60))][col]
    if side:feet=tuple((2*base-p[0],p[1]) for base,p in zip([27,37],feet))
    for n,(hip,foot,color) in enumerate(zip([(28,35),(36,35)],feet,['#32b9fa','#f49c45'])):
        knee=((hip[0]+foot[0])//2+(0 if col not in [2,5] else (-1 if n==0 else 1)),47)
        d.line([xy(p) for p in [hip,knee,foot]],fill=color,width=10)
        for p in [hip,knee,foot]:
            px,py=xy(p);d.ellipse((px-6,py-6,px+6,py+6),fill=color)
        fx,fy=xy(foot);d.line((fx-10,fy,fx+10,fy),fill=color,width=6)
    d.line((x+72,y+248,x+184,y+248),fill='#aab2b8',width=1)
guide.save(REF/'r06-clean-anatomical-motion-guide.png')
jobs=json.loads((DOC/'anatomy-jobs-r06.json').read_text('utf8'))
for j in jobs:
    i=j['index'];previous=Image.open(REF/f'CLASS-{i:02}-r05-unarmed-cycle.png').convert('RGBA')
    idle=Image.new('RGBA',(512,256))
    for side in range(2):idle.paste(previous.crop((0,side*64,64,side*64+64)).resize((256,256),Image.Resampling.NEAREST),(side*256,0))
    idle.save(REF/f'CLASS-{i:02}-r06-pixel-style-idle.png')
    j['references']=[str(REF/'r06-clean-anatomical-motion-guide.png'),j['references'][2],str(REF/f'CLASS-{i:02}-r06-pixel-style-idle.png')]
    identity=('The silver mechanical SHIN is ONLY anatomical RIGHT, always the viewer-LEFT leg in ALL14 drawings. The other leg is dark cloth and dark boot. Both thighs and knees stay side-by-side: never let the silver shin cross to the viewer-right half. Keep the bare upper arm viewer-left and mechanical forearm viewer-right; keep the original blue gray outfit and bun. Never mirror the armor.' if i==10 else 'Keep the ivory tan olive druid armor, stocky build, brown hair, fur collar, gauntlets and boots. Keep the same width and height in all drawings.')
    j['prompt']=f'''Use case: identity-preserve. Draw one game sprite sheet of {j['name']} in precisely TWO rows and SEVEN columns,14 complete coherent figures.
Image1 is the EXACT POSE AND ANATOMY GUIDE, not final appearance. Image2 is the CHARACTER AND OUTFIT IDENTITY. Image3 is the small-game-art PIXEL STYLE, scale and raised EMPTY-hand posture, not a walking pose guide.
Replace every stick figure in image1 with the character from image2 in the pixel-art finish of image3. Follow the guide's hip-knee-foot paths exactly, retaining TWO clear separated leg lanes. No crossed legs! This is a very compact measured near-front walk, tiny changes in foot depth and height, no large lateral swing. All legs connect to their OWN hip, never swap.
{identity}
Top row faces LEFT, bottom row faces RIGHT, always near-FRONTAL three quarters showing both shoulders and front chest, never side profile, never back. The entire atlas is the SAME individual, outfit, armor asymmetry, scale and proportion. Upper torso/head stay stable. Hands empty, fists held at lower chest/belt for later weapons, both complete arms connected to shoulders.
Each row in exact order: IDLE both feet grounded; A0 RIGHT leg planted/LEFT trails; A1 RIGHT supports/LEFT passes slightly lifted; A2 LEFT reaches while RIGHT trails; B0 LEFT planted/RIGHT trails; B1 LEFT supports/RIGHT passes slightly lifted; B2 RIGHT reaches while LEFT trails. A and B must alternate supporting feet. Do NOT repeat the same leg support in the second half. Blue guide leg is anatomical RIGHT (viewer-left), orange guide leg is anatomical LEFT (viewer-right), irrespective of direction faced. Do NOT use the diagram colors in final art.
Keep tiny steps below one boot length, low knees, full boots, stable head/hips/body height, equal figure sizes. Crisp intentional pixel clusters matching image3, not a new smooth high resolution illustration. Transparent background, clear gutters. Output only the14 figures in precisely2rows7columns. No text, guide, colored lines, background, floor, shadow, weapons, labels, additional frames or effects.'''
    j['destination']=str(ROOT/f'assets/source-art/player-walk-review-07/ANATOMY-{i:02}-r06b.png')
    Path(j['prompt_file']).with_name(f'ANATOMY-{i:02}-r06b.txt').write_text(j['prompt'],'utf8')
(DOC/'anatomy-jobs-r06b.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
