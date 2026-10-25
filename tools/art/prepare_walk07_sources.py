"""Source-locked whole-pose drawing jobs; no game integration or limb gait."""
import json
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
REF=DOC/'references'
SRC=ROOT/'assets/source-art/player-walk-review-07'
REF.mkdir(parents=True,exist_ok=True);SRC.mkdir(parents=True,exist_ok=True)
jobs06=json.loads((ROOT/'docs/reviews/player-walk-06/jobs.json').read_text('utf-8'))
baseline=json.loads((ROOT/'docs/reviews/player-walk-04/baseline-data.json').read_text('utf-8'))
SITE=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/sites/player-walk/dist')
font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',16)
guide=Image.open(ROOT/'assets/animations/walk-06/traqueuse-rifle.png').convert('RGBA').crop((0,0,448,128))
guide.resize((1344,384),Image.Resampling.NEAREST).save(REF/'accepted-14-pose-layout.png')
groups=[[i] for i in range(25)]
jobs=[]
prior_path=DOC/'unarmed-jobs.json'
prior={j['id']:j for j in json.loads(prior_path.read_text('utf8'))} if prior_path.exists() else {}
for serial,indices in enumerate(groups):
    identity=Image.new('RGBA',(512,len(indices)*512));draw=ImageDraw.Draw(identity)
    for row,index in enumerate(indices):
        im=Image.open(jobs06[index]['identity_reference']).convert('RGBA')
        im.thumbnail((460,460),Image.Resampling.NEAREST)
        identity.alpha_composite(im,((512-im.width)//2,row*512+36))
        draw.text((8,row*512+8),f'SUBJECT {row+1}: CLASS-{index:02}',font=font,fill='white')
    ref=REF/f'UNARMED-{serial:02}-identities.png';identity.save(ref)
    names='; '.join(f'subject{i+1}=CLASS-{index:02}, {jobs06[index]["label"]}' for i,index in enumerate(indices))
    count=len(indices);rows=count*2
    prompt=f'''Use case: identity-preserve. Game animation asset, whole-body UNARMED walk cycles for exactly {count} accepted characters, {names}.
INPUT1 is the accepted fourteen-pose layout: seven columns, first row front-left and second row front-right. It shows one woman with rifle ONLY to illustrate the measured leg walk and camera. INPUT2 is the exact identity reference: {count} subjects in labeled vertical order. Preserve EACH subject's head, build, armor stage, materials, colors, proportions, face/helmet/hair, coat hem and boots. Do not copy the guide woman's identity onto another subject.
Output ONE transparent regular SEVEN-COLUMN by {rows}-ROW atlas, {7*rows} complete figures, about1344x{rows*192} pixels, cells192square. For subject1: row0 LEFT, row1 RIGHT; subject2 row2 LEFT,row3 RIGHT, etc. Exactly seven poses per row: col0 IDLE BOTH FEET FLAT; cols1..6 walk0 contactA, walk1 passingA, walk2 reachB, walk3 contactB, walk4 passingB, walk5 reachA. All full figures separated by generous transparent gutters; heads and soles fully included. No labels/grid/background/floor/shadows/effects.
Remove the rifle: ALL these characters are UNARMED with two COMPLETE relaxed arms and visible closed hands alongside hips, elbows naturally slightly bent away from torso. No weapons, holsters or extra equipment. Keep the same arm readiness across the seven poses, to support later attached equipment. Shoulders, elbows, wrists and gloves anatomically joined, no transparent holes or abandoned grips. Do not hide hands behind the body. Hands sit outside torso with clear separation from thigh silhouette.
Copy INPUT1's small, grounded alternating-leg walk. The character is drawn ENTIRE in each pose. Both knees and boots take alternating supporting roles. No running, high knees, splits, hopping, lateral foot spread or mechanical leg bending. Minimal upper-body bob. Keep the exact source build; robe-covered legs remain covered by the real robe length.
For every LEFT row head/nose, torso, knees and feet all face screenLEFT in near-front three quarters; every RIGHT row all faceRIGHT. Both shoulders/chest front visible, not pure side/back. No diagonal travel or upper/lower mismatch. Identical standing height across all fourteen poses per subject, but keep different character builds. Native64-readable matte game rendering and crisp controlled outlines matching INPUT2. Exact subject order, seven columns, {rows} rows, true transparent background.'''
    # One identity per drawing prevents armor stages leaking into another row.
    prompt=prompt.replace(f'Output ONE transparent regular SEVEN-COLUMN by {rows}-ROW atlas, {7*rows} complete figures, about1344x{rows*192} pixels, cells192square. For subject1: row0 LEFT, row1 RIGHT; subject2 row2 LEFT,row3 RIGHT, etc. Exactly seven poses per row: col0 IDLE BOTH FEET FLAT; cols1..6 walk0 contactA, walk1 passingA, walk2 reachB, walk3 contactB, walk4 passingB, walk5 reachA.',
        'Output exactly FOUR COLUMNS and FOUR ROWS, sixteen complete figures,1536square canvas and384square cells. TOP TWO ROWS all LEFT; BOTTOM TWO ROWS all RIGHT. Reading order cells0..7: idleLEFT, walkLEFT0 contactA, walkLEFT1 passingA, walkLEFT2 reachB, walkLEFT3 contactB, walkLEFT4 passingB, walkLEFT5 reachA, duplicate idleLEFT. Cells8..15: idleRIGHT, walkRIGHT0 contactA, walkRIGHT1 passingA, walkRIGHT2 reachB, walkRIGHT3 contactB, walkRIGHT4 passingB, walkRIGHT5 reachA, duplicate idleRIGHT. Both idle feet FLAT.')
    prompt=prompt.replace('INPUT1 is the accepted fourteen-pose layout: seven columns, first row front-left and second row front-right.', 'INPUT1 is the accepted sixteen-pose guide: four columns, top two rows all front-left and bottom two rows all front-right.')
    prompt=prompt.replace(f'Exact subject order, seven columns, {rows} rows, true transparent background.', 'Exactly one identity, four columns and four rows, true transparent background.')
    prompt+=' Each384square cell has at least40px transparent padding on all sides, maximum figureheight280 and solebaseline328. Keep the same standing scale in every cell. Do not enlarge figures to fill the canvas; every bottom-row boot must be complete with transparent space below.'
    job={'id':f'UNARMED-{serial:02}','kind':'class_cycles','indices':indices,'ids':[f'CLASS-{i:02}' for i in indices],
        'rows':4,'columns':4,'reference':str(ROOT/'docs/reviews/player-walk-06/references/accepted-walk16-rifle.png'),'identity_reference':str(ref),
        'source_destination':str(SRC/f'UNARMED-{serial:02}-r02.png' if (SRC/f'UNARMED-{serial:02}-r02.png').exists() else SRC/f'UNARMED-{serial:02}-r01.png'),'prompt':prompt}
    for preserved in ['direction_sources','local_retouch_sources','anatomy_sources','bastion_transfer_sources','prosthesis_left_source','prosthesis_right_source','leg_material_mode','review_revision']:
        if preserved in prior.get(job['id'],{}):job[preserved]=prior[job['id']][preserved]
    jobs.append(job)
    (DOC/'prompts').mkdir(exist_ok=True);(DOC/'prompts'/f'{job["id"]}-r01.txt').write_text(prompt,'utf-8')
(DOC/'unarmed-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf-8')

# Inspection board: labels are inventory hints, never proof of leg count.
selected=[]
for rig in ['quad','crawl','serpent','winged','float','machine','roll']:
    records=[e for e in baseline['records'] if e.get('rig')==rig and e.get('variant',0)==0 and e['category']!='Classes'][:12]
    selected.extend(records)
board=Image.new('RGB',(1024,((len(selected)+7)//8)*170),'#293132');draw=ImageDraw.Draw(board)
for index,e in enumerate(selected):
    im=Image.open(SITE/e['image']).convert('RGBA')
    target=REF/f'{e["id"]}-current.png';im.save(target)
    im.thumbnail((128,128),Image.Resampling.NEAREST)
    x=index%8*128;y=index//8*170
    board.paste(im,(x+(128-im.width)//2,y+26),im)
    draw.text((x+3,y+3),e['id'],font=font,fill='#eeeade')
    draw.text((x+3,y+150),e['rig'],font=font,fill='#eeeade')
board.save(REF/'non-biped-current-inventory.png')
(DOC/'morphology-inspection-inventory.json').write_text(json.dumps(selected,ensure_ascii=False,indent=2)+'\n','utf-8')
print(json.dumps({'whole_pose_jobs':len(jobs),'class_cycles':25,'inspection_sources':len(selected)}))
