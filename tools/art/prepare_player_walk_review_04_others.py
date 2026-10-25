"""Prepare whole-sprite 3/4 atlases for the remaining real actor catalogue."""
import json
from collections import defaultdict
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'artifacts/player-walk-review-04'
DOC=ROOT/'docs/reviews/player-walk-04'
SRC=ROOT/'assets/source-art/player-walk-review-04'
SITE=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/sites/player-walk/dist')
CAMERA=ROOT/'assets/source-art/player-walk-review-02/WA02-03-rifle-source.png'

def main():
    data=json.loads((SITE/'catalogue/data.json').read_text('utf-8'))
    (DOC/'baseline-data.json').write_text(json.dumps(data,ensure_ascii=False,separators=(',',':')),'utf-8')
    font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',17)
    jobs=[]
    # Frozen historical garments are still identifiable sources, not substitutes
    # for the accepted current class outfits.
    configurations=[0,1,2,4,9,11]
    legacy_labels=['Sans arme','Un pistolet','Deux pistolets','Lame et pistolet','Fusil à deux mains','Hampe à deux mains']
    for record in data['records']:
        if record.get('configs')!='legacy' or not record['id'].endswith('-0'):continue
        atlas=Image.open(SITE/record['pack'][0]).convert('RGBA')
        ref=Image.new('RGBA',(768,2304));draw=ImageDraw.Draw(ref);fronts=[]
        for row,c in enumerate(configurations):
            front=atlas.crop((c*128,0,(c+1)*128,128))
            neutral=atlas.crop((0,0,128,128))
            ref.alpha_composite(neutral.resize((384,384),Image.Resampling.NEAREST),(0,row*384))
            ref.alpha_composite(front.resize((384,384),Image.Resampling.NEAREST),(384,row*384))
            draw.text((12,row*384+8),record['label'],font=font,fill='white')
            draw.text((396,row*384+8),legacy_labels[row],font=font,fill='white')
            p=ART/f'{record["id"]}-{row}-front.png';front.save(p);fronts.append(str(p.relative_to(ROOT)))
        reference=ART/f'{record["id"]}-reference.png';ref.save(reference)
        prompt=f'''Use case: identity-preserve. Create ONE native64 RPG pose-atlas asset for the exact historical outfit {record['label']} shown in source 1.
Source 1 has six rows, each showing the SAME unarmed body on the left and the equipped FRONT reference on the right. These columns are NOT output directions. This is the only subject reference; preserve its exact design and material finish. The chosen camera is near-front three quarters, turned 35 degrees left/right.
Output a transparent, regular TWO-COLUMN SIX-ROW atlas: TWELVE COMPLETE full-body figures. Portrait 768x2304. Each LEFT figure turns head AND torso 35 degrees toward screen LEFT (front-left three quarters); each RIGHT figure turns toward screen RIGHT (front-right three quarters). These must be genuinely turned drawings, not squashed or sheared frontal images. No pure side/back views.
Six rows exactly match input equipment: no weapon; one pistol; two pistols; blade plus pistol; two-handed rifle; two-handed pole weapon. Keep the real weapon shapes and colors, exact outfit stage, face/head, clothing palette, body height, boots and torso design across all poses. No extra helmet, armor, limbs, gear or details absent from source 1.
Exactly two connected arms. Every shoulder, upper arm, elbow, sleeve, forearm, wrist, glove and finger must be complete and anatomically joined. NO transparent cuts, missing wedges or holes inside a limb. Natural air under bent elbows is allowed. Fingers wrap the physical handles; a rifle has rear grip and front support hand. Both hands on a pole share its shaft. No floating weapon, third arm, abandoned hand or wrist gap. Carry weapons above the hips or clearly outside the legs. Neutral standing ready-to-walk poses, no attack.
Generous transparent margins in EVERY cell; all heads, boots and entire barrels / blades must fit. Identical character scale in all rows. No ground, shadow, text, labels, particles or background. Preserve source 1's identity; ignore tiny colored registration specks outside the source silhouette. Clean readable matte game art suitable for native64.'''
        jobs.append({'id':record['id'],'kind':'legacy','record':record,'reference':str(reference),'fronts':fronts,'labels':legacy_labels,
            'source_destination':str(SRC/f'{record["id"]}-quarter-source.png'),'prompt':prompt,'rows':6,'camera':str(CAMERA)})
    # Four source-locked rows form one bestiary-atlas asset. Group matching
    # morphology, so no human gait or anatomy is imposed on unrelated bodies.
    singles=[e for e in data['records'] if (e['category']=='Campagne' and e['variant']==0) or e['id'].startswith('BANK-') or e['category']=='PNJ et formes' or e['id'] in ['ACTOR-00','ACTOR-01','ACTOR-02','ACTOR-26']]
    groups=defaultdict(list)
    for e in singles:groups[(e['category'],e['rig'])].append(e)
    serial=0
    for (category,rig),entries in sorted(groups.items()):
        for first in range(0,len(entries),4):
            subset=entries[first:first+4];key=f'ACTOR-ATLAS-{serial:03}';serial+=1
            reference=ART/f'{key}-reference.png';ref=Image.new('RGBA',(768,384*len(subset)));draw=ImageDraw.Draw(ref)
            for row,e in enumerate(subset):
                im=Image.open(SITE/e['image']).convert('RGBA');whole=im.resize((384,384),Image.Resampling.NEAREST)
                box=im.getbbox();detail=im.crop(box);detail.thumbnail((340,330),Image.Resampling.NEAREST)
                ref.alpha_composite(whole,(0,row*384));ref.alpha_composite(detail,(384+(384-detail.width)//2,row*384+34+(340-detail.height)//2))
                draw.text((12,row*384+8),f'{row+1:02} {e["id"]}',font=font,fill='white')
                draw.text((396,row*384+8),e['label'][:35],font=font,fill='white')
            ref.save(reference)
            names='; '.join(f'row {i+1}: {e["id"]}, {e["label"]}, {e["rigLabel"]}' for i,e in enumerate(subset))
            rows=len(subset)
            prompt=f'''Use case: identity-preserve. Asset: ONE directional bestiary sprite-atlas with {rows} locked rows, native64 square-grid RPG.
Input 1 is the exact source catalogue. Every row shows a whole FRONT subject at left and an enlarged detail of the SAME subject at right. Keep each row's distinct creature identity; never mix designs between rows. Rows: {names}.
Input 1 is the ONLY appearance reference. The selected camera is a near-front three-quarter view, not a new costume or anatomy.
Output exactly TWO COLUMNS and {rows} ROWS, transparent background, complete individual full-body sprites. LEFT column shows a genuinely drawn front-left three-quarter orientation toward screen LEFT, RIGHT column a front-right three-quarter orientation toward screen RIGHT, about 35 degrees away from front. BOTH the head and body turn; show coherent visible sides and volume. Not frontal pictures stretched or sheared. Not pure profile or back views. Same slightly elevated orthographic RPG camera. Generous transparent padding around EVERY figure; no clipping or overlap between rows.
Preserve the exact source morphology, head/face, number of legs, arms, wings and tentacles, materials, body colors, armor and carried items. A worm remains a worm; quadrupeds stay quadrupeds; floating spirits remain their actual shape; tracked machines retain tracks and no invented legs. Do not add humanoid limbs or armor. Keep anatomical joints coherent and appendages firmly attached as in the source. Deliberately disjoint spirit parts remain intentional.
For any humanoid or equipped subject, exactly the existing arms, full solid sleeves and limbs, no transparent wedges or holes inside them. Correct shoulder-elbow-wrist connection, complete hands physically gripping their original equipment. No floating weapon, third arm or abandoned old hand. Turn a carried gun with its wielder. No new weapons, abilities, effects or decoration.
Neutral ready-to-move standing / crouching / floating poses appropriate to the exact subject. No attacks, gore effects, particles, ground, shadows, text, labels or background. Match source 1's actual species and palette. Strong readable masses and controlled outlines, clean matte native64 game readability. Ignore isolated registration pixels outside source silhouettes.'''
            jobs.append({'id':key,'kind':'actors','records':subset,'reference':str(reference),'rows':rows,
                'source_destination':str(SRC/f'{key}-quarter-source.png'),'prompt':prompt,'camera':str(CAMERA)})
    for j in jobs:(DOC/'prompts'/f'{j["id"]}.txt').write_text(j['prompt'],'utf-8')
    (DOC/'other-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2),'utf-8')
    print(json.dumps({'legacy_outfit_atlases':19,'actor_atlases':serial,'individual_actor_models':len(singles),'total_jobs':len(jobs)}))

if __name__=='__main__':main()
