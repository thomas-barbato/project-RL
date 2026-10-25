"""Two targeted whole-figure corrections after inspecting r06b."""
import json
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';REF=DOC/'references';SRC=ROOT/'assets/source-art/player-walk-review-07'
jobs=[]
tr=Image.open(SRC/'ANATOMY-10-r06b.png').convert('RGBA');top=tr.crop((0,0,tr.width,tr.height//2))
padded=Image.new('RGBA',(top.width+64,top.height+96));padded.paste(top,(32,48));trpath=REF/'CLASS-10-r06c-left-target.png';padded.save(trpath)
prompt='''Use case: precise-object-edit. This is an EDIT TARGET of exactly SEVEN complete figures of the SAME woman in ONE horizontal row. Correct ONLY their HEADS and gaze so that ALL SEVEN look toward SCREEN LEFT, the LEFT EDGE OF THE IMAGE. Currently they all incorrectly look screenRIGHT. Turn the face, nose and eyes toward screenLEFT while keeping a near-frontal three-quarter face, not side profile/back. Move the hair/temple device consistently with this small head rotation. Retain the original bun and character identity.
Keep EVERY existing body pose, both arms, empty hands, armor, metallic forearm, thighs, knees, silver mechanical lower leg and boots EXACTLY as they are in the corresponding figure. The silver lower leg must remain viewerLEFT in ALL seven figures. Do NOT mirror any figure or body; only turn the head/gaze toward screenLEFT. Do not change their compact foot poses, scale, pixel-art style or colors. Keep the seven figures in the same order and ONE ROW, all complete, equal size, entire hair and boots visible, generous transparent margin around all figures including top. No text, labels, floor, background, shadows, new weapon or effects. True transparent background.'''
jobs.append({'id':'CLASS-10-left','index':10,'references':[str(trpath)],'destination':str(SRC/'ANATOMY-10-r06c-left.png'),'prompt':prompt})
me=Image.open(SRC/'ANATOMY-24-r06b.png').convert('RGBA')
from build_player_walk_review_04 import components
parts=sorted(components(me),key=lambda p:p['area'],reverse=True)[:14];parts.sort(key=lambda p:p['center'][1]);parts=[p for s in range(2) for p in sorted(parts[s*7:(s+1)*7],key=lambda p:p['center'][0])]
target=Image.new('RGBA',(768,512))
for side in range(2):
    idle=me.crop(parts[side*7]['box']);target.paste(idle,(64+side*384,64),idle)
mepath=REF/'CLASS-24-r06c-support-target.png';target.save(mepath)
prompt='''Use case: precise-object-edit. EDIT TARGET: exactly TWO complete armored druid figures, one facing screenLEFT on the left, one facing screenRIGHT on the right. Preserve their directions, identity, head, upper body, fur, ivory/olive armor, empty raised hands, exact stocky proportions, pixel-art clusters and scale.
Change BOTH figures from idle into ONE specific LOW PASSING walking pose: the VIEWER-LEFT leg is STRAIGHT and bears the body weight, viewer-left boot sole FIRMLY PLANTED on the baseline. The VIEWER-RIGHT leg is the MOVING leg: slightly bend its knee and lift its complete boot 6 native pixels above the baseline. The legs remain clearly separated side-by-side and attached to their own hips; never cross. In BOTH figures the planted boot is on the LEFT SIDE OF THEIR BODY IN THE IMAGE, the lifted boot is on the RIGHT SIDE OF THEIR BODY IN THE IMAGE. Do not lift the viewer-left foot. Do not mirror the pose between the two figures: only the face direction differs.
Keep the moving foot low, no high knee, running, big split or wide kick. Fixed original head and hip height, same full body height as target; do not shrink a figure to accommodate the raised foot. This represents anatomical RIGHT support / LEFT passing in a near-frontal camera and corrects the first half-step only. Output exactly TWO COMPLETE figures in ONE horizontal row, same order and scale, generous transparent gutters, complete hair and both boots. True transparent background; no label, text, guide, floor, shadow, equipment, extra arm or effects.'''
jobs.append({'id':'CLASS-24-first-support','index':24,'references':[str(mepath)],'destination':str(SRC/'ANATOMY-24-r06c-first-support.png'),'prompt':prompt})
for j in jobs:(DOC/f'prompts/{j["id"]}-r06c.txt').write_text(j['prompt'],'utf8')
(DOC/'anatomy-detail-jobs-r06c.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
