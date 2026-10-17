"""Native review for the two requested ordinary cultists; no gameplay edits."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
from show_npc_monsters_review import conditioned_image
from review_contact_shadows import contact_shadow

ROOT=Path(__file__).resolve().parents[2]
REVIEW=ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
ART=ROOT/'assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01'
OUT=ART/'native64-review-only/cultists-29'
OUT.mkdir(parents=True,exist_ok=True)
records=[]
for number,ident,height,role in [(164,'CULT-08',60,'Tir à deux mains'),(165,'CULT-09',64,'Mêlée à deux mains')]:
    source=ART/f'{ident}-r01.png'
    entry=dict(number=number,id=ident,revision='r01',label='Cultiste',review_label=role,
        family='Cultistes humains',catalogue_section='Cultistes humains',role=role,
        source=source.relative_to(ROOT).as_posix(),source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
        maximum_body_height64=height,maximum_body_width64=72,palette_limit=15,canvas_cells=2,ground_cell=1.5,
        visual_approved=False,scale_approved=False,name_approved=True,rules_approved=False,runtime_authorized=False)
    image,anchor=conditioned_image(entry,64)
    path=OUT/f'{ident}-r01-native64.png'
    image.save(path,optimize=True)
    entry['variants']=[dict(variant='A',file=path.relative_to(ROOT).as_posix(),sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
        cell=64,canvas=image.width,foot_anchor=list(anchor),occupied=list(image.getbbox()),colors=len({p[:3] for p in image.get_flattened_data() if p[3]}))]
    records.append(entry)
reference=json.loads((REVIEW/'cultists-zombies-spirits-24.json').read_text(encoding='utf-8'))['player_reference']
player=Image.open(ROOT/reference['source']).convert('RGBA')
sheet=Image.new('RGB',(736,240),'#10171c')
draw=ImageDraw.Draw(sheet)
bold=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',16)
small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
draw.text((16,10),'Cultistes · armes à deux mains · taille native',font=bold,fill='#e5d2a8')
for column,entry in enumerate(records):
    x=12+column*362
    draw.rectangle((x,42,x+350,224),fill='#1a242b',outline='#35434b')
    draw.text((x+12,50),f"{entry['number']} · {entry['id']} · {entry['role']}",font=small,fill='#e5d2a8')
    for center,image,anchor in [(x+110,Image.open(ROOT/entry['variants'][0]['file']).convert('RGBA'),entry['variants'][0]['foot_anchor']),
                                (x+260,player,reference['foot_anchor'])]:
        shadow,_=contact_shadow(image,anchor)
        sheet.paste(shadow,(center-anchor[0],188-anchor[1]),shadow)
        sheet.paste(image,(center-anchor[0],188-anchor[1]),image)
    draw.text((x+89,198),'Cultiste',font=small,fill='#c3cdd0')
    draw.text((x+241,198),'Joueur',font=small,fill='#c3cdd0')
preview=ROOT/'artifacts/campaign-bestiary-integration-06/cultists-two-handed-native64.png'
preview.parent.mkdir(parents=True,exist_ok=True)
sheet.save(preview,optimize=True)
batch=dict(review_id='CULTISTS-TWO-HANDED-29',date='2026-10-17',generator='built-in image_gen',
    records=records,expected_new_numbers=[164,165],expected_catalogue_count=131,player_reference=reference,
    status='native_review_pending',runtime_connected=False,preview=preview.relative_to(ROOT).as_posix())
(REVIEW/'two-handed-cultists-29.json').write_text(json.dumps(batch,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(preview)
