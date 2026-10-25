"""Keep canonical leather boots and the light outfit's wrapped calves."""
import json,shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07'
GEN=Path('C:/Users/User/.codex/generated_images/01a16812-c028-73a3-9d94-d6e3343cb79c')
files={(20,'left'):'exec-de426196-3353-42fd-8b0f-0eeb4905e6c6.png',(20,'right'):'exec-b7398341-61cd-4d27-a525-0ba5fa50a6ea.png',
 (21,'left'):'exec-bb70bcfe-ab18-477e-8e35-85cd32d65f45.png',(21,'right'):'exec-fc730478-fa4d-45c9-ab58-c0b0ca5f12ea.png',
 (22,'left'):'exec-4a2bfadd-2cdd-4d6c-b630-f4c12efaf31a.png',(22,'right'):'exec-818ca795-2dc3-40bb-bf6b-041d2eed7d0d.png',
 (23,'left'):'exec-6d2b3cbf-9370-4748-a989-b258ee54a635.png',(23,'right'):'exec-5a77b5f4-35a7-4cef-bc65-8c077a2ac1e7.png'}
old=json.loads((ROOT/'docs/reviews/player-walk-06/jobs.json').read_text('utf8'));jobs=[]
for (index,side),name in files.items():
    source=SRC/f'BASTION-TRANSFER-{index:02}-r09-{side}.png';shutil.copy2(GEN/name,source)
    if index==20:continue
    calf=('Both calves BETWEEN THE IVORY KNEECAPS AND THE SMALL IVORY ANKLE GUARDS are wrapped OLIVE/BROWN CLOTH AND LEATHER, not long ivory shin plates. Restore that olive/brown wrapped calf material on BOTH legs in all seven poses, including any bent rear leg. Keep the existing ivory knee pads and small ankle guards. '
          if index==21 else 'Keep the current IVORY SHIN PLATES, ivory knee pads and ankle guards exactly as they are. ')
    prompt=f'''Use case: precise-object-edit. INPUT1 is the edit target, exactly SEVEN complete Metamorphe figures in ONE ROW facing {side.upper()}, idle then six walking poses. INPUT2 is canonical outfit{index%5+1}, MATERIAL reference only; ignore its backdrop.
Correct ONLY the lower-leg and boot materials that were copied from a heavier outfit. {calf}ALL SEVEN figures have OLIVE LEATHER BOOT TOE BOXES, like input2. Replace the bright ivory/bone caps on the fronts/tops of both boots with OLIVE/BROWN LEATHER in the same existing boot shape, matte texture and dark outline. Leave the ivory ankle cuffs separate and intact. These outfits do NOT have ivory toe caps. Keep brown/olive leather and dark soles on every boot, including raised rear feet.
Keep ALL pose geometry, silhouette, hip/knee/ankle/foot locations, foot lifts, frame order, standing height, head facing {side.upper()}, hair, face, fur collar, mantle, chest, arms, hands, armor above the knees and existing pixel-art finish unchanged as closely as possible. This is a material-only edit of the SAME complete figures, not a new outfit or gait. Exactly SEVEN whole figures in ONE ROW, same layout, full hair and boots with margins. Genuine clean transparent background, no glow, shadow, floor, text, labels or extra gear.'''
    filename=f'CLASS-{index:02}-material-r09b-{side}.txt';(DOC/'prompts'/filename).write_text(prompt,'utf8')
    jobs.append({'id':f'CLASS-{index:02}-material-r09b-{side}','index':index,'side':side,'references':[str(source),old[index]['identity_reference']],
                 'prompt':prompt,'destination':str(SRC/f'BASTION-TRANSFER-{index:02}-r09b-{side}.png')})
(DOC/'metamorphe-material-jobs-r09b.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
print(json.dumps({'whole_pose_sources_saved':len(files),'material_edits':len(jobs)}))
