"""Prepare the complete5classes x5armor-stages review, source-locked to WALK05."""
from __future__ import annotations

import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-06'
REF=DOC/'references'
SOURCE=ROOT/'assets/source-art/player-walk-review-06'
CLASSES=['Bastion','Technomancien','Traqueuse','Arcaniste','Métamorphe']
LAYOUT=['idle-left','left-0','left-1','left-2','left-3','left-4','left-5','idle-left',
        'idle-right','right-0','right-1','right-2','right-3','right-4','right-5','idle-right']


def main():
    for directory in (DOC,REF,SOURCE):directory.mkdir(parents=True,exist_ok=True)
    accepted=ROOT/'assets/source-art/player-walk-review-05/accepted/native64'
    walk=Image.open(accepted/'traqueuse-rifle-walk.png').convert('RGBA')
    idle=Image.open(accepted/'traqueuse-rifle-idle.png').convert('RGBA')
    guide=Image.new('RGBA',(1024,1024))
    for i,key in enumerate(LAYOUT):
        side=7 if 'left' in key else 1
        if key.startswith('idle'):
            tile=idle.crop((side%4*64,side//4*64,side%4*64+64,side//4*64+64))
        else:
            frame=int(key.rsplit('-',1)[1]);tile=walk.crop((frame*64,side*64,(frame+1)*64,(side+1)*64))
        guide.alpha_composite(tile.resize((256,256),Image.Resampling.NEAREST),(i%4*256,i//4*256))
    guide_path=REF/'accepted-walk16-rifle.png';guide.save(guide_path)

    old=json.loads((ROOT/'docs/reviews/player-walk-04/jobs.json').read_text('utf-8'))
    armor=json.loads((ROOT/'assets/sprites/class-armors-01/manifest.json').read_text('utf-8'))
    originals={}
    for item in armor['source_acceptances']:
        decision=json.loads((ROOT/item['path']).read_text('utf-8'))
        for entry in decision['entries']:originals[entry['candidate_id']]=ROOT/entry['source']['path']
    jobs=[];board=Image.new('RGB',(1000,1500),'#293132')
    font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14);draw=ImageDraw.Draw(board)
    for j in old:
        record=j['record'];index=int(j['id'].split('-')[1]);class_index=index//5;stage=index%5
        identity=originals[record['candidate']]
        im=Image.open(identity).convert('RGBA');b=im.getchannel('A').point(lambda a:255 if a>=48 else 0).getbbox();cropped=im.crop(b)
        identity_path=REF/f'{j["id"]}-canonical.png'
        if max(cropped.size)<512:
            factor=512/max(cropped.size)
            cropped=cropped.resize((round(cropped.width*factor),round(cropped.height*factor)),Image.Resampling.NEAREST)
        cropped.save(identity_path)
        factor=min(180/cropped.width,258/cropped.height)
        preview=cropped.resize((round(cropped.width*factor),round(cropped.height*factor)),Image.Resampling.LANCZOS)
        board.paste(preview,(stage*200+(200-preview.width)//2,class_index*300+32),preview)
        draw.text((stage*200+8,class_index*300+6),f'{index+1:02} {CLASSES[class_index]} / {stage+1}',font=font,fill='#eeeade')
        equipped=Image.open(ROOT/j['fronts'][3]).convert('RGBA').resize((512,512),Image.Resampling.NEAREST)
        equipped_path=REF/f'{j["id"]}-rifle-equipped.png';equipped.save(equipped_path)
        prompt=f'''Use case: identity-preserve. One full-body walk-and-idle sprite atlas for {record['candidate']}, {CLASSES[class_index]} armor stage{stage+1}.
INPUT1 is the ACCEPTED animation pose guide, not the appearance to copy. INPUT2 is the EXACT accepted identity and armor/outfit: keep this character's build, face or helmet, hair, all clothing/armor shapes, colors, materials and proportions. INPUT3 is its actual rifle-equipped game reference: preserve this rifle model, scale and grips.
Replace the woman in EVERY cell of INPUT1 with the character from INPUT2, carrying its actual rifle as INPUT3. Keep EACH corresponding whole-figure pose and near-front three-quarter camera of INPUT1, scaled to this character's own build. Do not copy the woman's bun/face/slender build/outfit onto another character. Do not add missing gear or change armor stage. Preserve asymmetry on its correct anatomical side.
Output exactly FOUR columns and FOUR rows, sixteen COMPLETE figures on a transparent1536x1536canvas, equal384square cells, bodyheight312 and solebaseline356 per cell. Clear alpha gutters, full heads/boots/barrels, no clipping/overlap. No labels/text/grid/background/floor/shadow/effects.
The TOP TWO ROWS, all eight cells, are FRONT-LEFT three quarters, head/torso/knees/boots/rifle all directed screen LEFT. The BOTTOM TWO ROWS are FRONT-RIGHT three quarters, all directed RIGHT. Keep both shoulders and front of chest visible like INPUT1; no side profile/back. Nose and gaze must agree with feet and muzzle.
Exact cells in reading order:
0 idleLEFT BOTH FEET FLAT;1 left walk0 first-foot contact;2 left walk1 first-foot support/other foot passing low;3 left walk2 opposite foot reaching;
4 left walk3 opposite-foot contact;5 left walk4 opposite-foot support/first foot passing low;6 left walk5 first foot reaching;7 same idleLEFT as cell0;
8 idleRIGHT BOTH FEET FLAT;9 right walk0 first-foot contact;10 right walk1 support/passing;11 right walk2 opposite foot reaching;
12 right walk3 opposite-foot contact;13 right walk4 support/passing;14 right walk5 first foot reaching;15 same idleRIGHT as cell8.
Copy INPUT1's compact measured WALK poses for each cell. Two truly different supporting feet per cycle, not the same leg always forward, no lifted leg in idle, no hopping/running/high knee/wide splits. Minimal bob. Robes/coats follow the exact source length and hide the correct leg regions; do not invent exposed legs through fabric.
Rifle points LEFT above, RIGHT below, buttstock physically at shoulder. TWO complete connected arms/shoulders/elbows/sleeves/wrists/hands, rear grip and fore-end support correct. No transparent wedges, missing arm strips, abandoned old hand or third arm. Rifle rigid, straight and identical in all cells, no bending/change of barrel or stock. Character same anatomy/build/head/armor in all sixteen. Matte readable game art matching INPUT2, crisp controlled contours, native64 readability. Complete coherent figures drawn in every cell, no grafted legs or sheared standing pictures.'''
        path=DOC/'prompts'/f'{j["id"]}-rifle-r01.txt';path.parent.mkdir(parents=True,exist_ok=True);path.write_text(prompt,encoding='utf-8')
        jobs.append({'id':j['id'],'class_index':class_index,'stage':stage,'number':index+1,
            'label':f'{CLASSES[class_index]} · tenue{stage+1}','record':record,
            'reference':str(guide_path),'identity_reference':str(identity_path),'canonical_source':str(identity),'equipped_reference':str(equipped_path),
            'source_destination':str(SOURCE/f'{j["id"]}-rifle-r01.png'),'prompt':prompt,
            'layout':LAYOUT,'status':'accepted_reference' if index==10 else 'pending'})
    board.save(REF/'all25-canonical-identities.png')
    (DOC/'jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    contract={'classes':CLASSES,'stages_per_class':5,'canonical_silhouettes':25,
        'torso_models':[None]+list(range(101,115))+list(range(200,205)),
        'helmet_models':[None]+list(range(115,121)),
        'physical_item_stage_aliases':{'none':[None,108,112,113,200,201,202,203,204],
            'stage2':[110,114],'stage3':[102,105,106,111],'stage4':[101,104,107,109],'stage5':[103]},
        'class_torso_helmet_cases':5*20*7,'unique_class_stage_helmet_combinations':25*7,
        'initial_animation_configuration':'rifle only; other grips/helmets remain separate required work',
        'gameplay_mapping_modified':False,'runtime_integrated':False}
    (DOC/'compatibility-contract.json').write_text(json.dumps(contract,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'jobs':len(jobs),'pending':24,'guide':str(guide_path),'contract_cases':700}))


if __name__=='__main__':main()
