"""Prepare source-faithful references for painted three-quarter poses.

The failed WALK03 yaw/decomposition is deliberately not reused.
"""
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
SITE=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/sites/player-walk/dist')
ART=ROOT/'artifacts/player-walk-review-04'
DOC=ROOT/'docs/reviews/player-walk-04'
OUT=ROOT/'assets/source-art/player-walk-review-04'
CONFIGS=[(0,'Sans arme'),(102,'Un pistolet'),(98,'Marteau à une main'),(3,'Fusil à deux mains'),(67,'Hache à deux mains'),(114,'Deux pistolets')]

def cell(im,n):
    x=n%16*128;y=n//16*128
    return im.crop((x,y,x+128,y+128))

def main():
    for p in [ART,DOC,OUT,DOC/'prompts']:p.mkdir(parents=True,exist_ok=True)
    data=json.loads((SITE/'catalogue/data.json').read_text('utf-8'))
    armor=json.loads((ROOT/'assets/sprites/class-armors-01/manifest.json').read_text('utf-8'))
    identities={}
    for accepted in armor['source_acceptances']:
        decision=json.loads((ROOT/accepted['path']).read_text('utf-8'))
        for entry in decision['entries']:identities[entry['candidate_id']]=str(ROOT/entry['source']['path'])
    previous=DOC/'jobs.json'
    if previous.exists() and not (DOC/'jobs-r01.json').exists():
        (DOC/'jobs-r01.json').write_bytes(previous.read_bytes())
    jobs=[];font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',18)
    for record in data['records']:
        if record['category']!='Classes':continue
        atlas=[Image.open(SITE/p).convert('RGBA') for p in record['pack']]
        identity=Image.open(identities[record['candidate']]).convert('RGBA')
        identity_reference=identities[record['candidate']]
        if max(identity.size)<512:
            identity=identity.resize((identity.width*4,identity.height*4),Image.Resampling.NEAREST)
            p=ART/f'{record["id"]}-canonical-reference.png';identity.save(p);identity_reference=str(p)
        ref=Image.new('RGBA',(768,2304))
        draw=ImageDraw.Draw(ref)
        neutral=cell(atlas[0],0)
        fronts=[]
        for row,(index,label) in enumerate(CONFIGS):
            front=cell(atlas[0],index)
            for layer in atlas[1:]:front.alpha_composite(cell(layer,index))
            original=identity.copy();original.thumbnail((370,350),Image.Resampling.LANCZOS)
            ref.alpha_composite(original,((384-original.width)//2,row*384+26+(350-original.height)//2))
            ref.alpha_composite(front.resize((384,384),Image.Resampling.NEAREST),(384,row*384))
            draw.text((12,row*384+8),f'{row+1:02} {record["label"]}',font=font,fill='white')
            draw.text((396,row*384+8),label,font=font,fill='white')
            f=ART/f'{record["id"]}-{row}-front.png';front.save(f);fronts.append(str(f.relative_to(ROOT)))
        path=ART/f'{record["id"]}-reference-r02.png';ref.save(path)
        job={'id':record['id'],'record':record,'reference':str(path),'fronts':fronts,
            'identity_reference':identity_reference,
            'source_destination':str(OUT/(f'{record["id"]}-quarter-source'+('-r02.png' if 5<=record['classIndex']<=8 else '.png')))}
        prompt=f'''Use case: identity-preserve. Asset type: one complete directional pose atlas for the single Project RL outfit {record['candidate']} ({record['label']}).
Input 1 is the EXACT approved full-size character design: preserve this identity, body build, head, materials, colors and armor stage.
Input 2 contains six equipment reference rows, each with the full-size unarmed design on the LEFT and the real equipped frontal pose on the RIGHT. Preserve each weapon's actual shape and grip configuration. These source columns are references, NOT the desired output directions. Follow input 1 for all character details; never invent a new helmet or physique.
Create a TRANSPARENT, perfectly regular atlas of TWO COLUMNS and SIX ROWS: exactly TWELVE full-body figures. Canvas portrait 768x2304, twelve square 384x384 cells. No text, labels, guides, shadows, ground or background. Transparent margins, complete boots and heads, same character height and foot baseline in every cell. Feet baseline y=350 inside every cell, head top y=35. Keep weapons wholly inside their cell.
Every LEFT column is a TRUE FRONT-LEFT THREE-QUARTER view, turned about 35 degrees and looking screen LEFT. Every RIGHT column is a TRUE FRONT-RIGHT THREE-QUARTER view, turned about 35 degrees and looking screen RIGHT. Both head and torso turn; front plates and shoulder volumes must show the angle. Near-front RPG camera, orthographic slightly elevated, same camera as input 2. NOT frontal sprites squeezed or sheared. NOT pure profile, NOT back views.
Rows in exact source order: (1) no weapon, relaxed complete arms; (2) one pistol held ready in one hand, other arm complete; (3) one-handed hammer, other arm complete; (4) assault rifle across chest with TWO HANDS, rear hand on pistol grip and support hand under handguard; (5) large axe with TWO HANDS on the same shaft; (6) two pistols, one per hand.
Match the actual weapon silhouette and materials in the corresponding source row. For left views, muzzle / readiness direction follows screen LEFT; right follows RIGHT. Rifle stock rests against shoulder, no floating stock or barrel. Keep the character's exact armor stage, helmet/face identity, clothing colors, silhouette, proportions and all body panels. Same body and boots across all six rows. Only arms and held equipment change. Do not add or remove armor.
ANATOMY IS CRITICAL: exactly two arms, each with ONE connected shoulder, upper arm, elbow, forearm, wrist and hand. Paint full continuous sleeves and gloves. No transparent cuts, missing wedges, holes or erased strips inside an arm. Natural empty air between a bent arm and torso is fine. Closed coherent grips with fingers visibly wrapping the handles. No third arm or abandoned old hand. Ensure elbows connect smoothly to sleeves; no disconnected wrists. Full opaque body surfaces and clean transparent exterior.
Clean matte painted game sprite finish with strong readable masses, coherent light, dark controlled outlines, no photorealism, grain or colored noise. Must remain readable at native logical 64x64. Standing neutral ready-to-walk poses, not attacks. No particles or muzzle flash.'''
        (DOC/'prompts'/f'{record["id"]}.txt').write_text(prompt,'utf-8')
        job['prompt']=prompt;jobs.append(job)
    (DOC/'jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2),'utf-8')
    print(json.dumps({'class_jobs':len(jobs),'configs':CONFIGS},ensure_ascii=False))

if __name__=='__main__':main()
