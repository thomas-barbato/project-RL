"""Five starting torso items omitted from review 01. Candidates only."""
import json
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
import colorsys,html
import torso_icons_review_01 as prior

ROOT=prior.ROOT
ART=ROOT/'assets/source-art/torso-starting-review-02'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/torso-starting-review-02'
OUT=ROOT/'artifacts/torso-icons-review-01'
ENTRIES=[
 (200,1,1,'Tenue d’assaut','tenue_assaut','A sleeveless dirty beige fabric protective tank top / vest, olive drab waist harness and dark low belt, thick slightly ragged cloth, no sleeves. This is an empty fabric ITEM, not a muscular torso, not flesh, no arms or skin. Keep the compact simple chest garment of the reference.'),
 (201,14,2,'Tenue technique','tenue_technique','A compact dark desaturated navy-blue and charcoal industrial technical vest with angular dark steel shoulder caps, dark chest plates and utility seams. Keep the closed collar and worn dark-blue protective torso. No visor, helmet, mask, gadgets or glowing lights.'),
 (202,15,2,'Tenue de traque','tenue_traque','A worn muted deep-blue sleeveless short hunter coat / vest with dark grey central layer, practical crossed dark straps and a low beige belt. Short loose shoulder caps, natural hip-length hem. No hood, trousers, long skirt or scarf covering the neck opening.'),
 (203,16,2,'Tenue rituelle','tenue_rituelle','The upper part of a dark crimson red ritual robe: loose red short sleeves, layered red chest cloth over charcoal central fabric and a dark broad belt, with a compact hip-length hem. No hood, mask, long skirt, ritual symbol or magic.'),
 (204,17,2,'Tenue métamorphe','tenue_metamorphe','A compact mustard-ochre / olive worn short coat torso with dark straps, pale dirty-ivory chest inset, charcoal short sleeves and a dark waist belt. Practical heavy cloth, no hood, mask, mutation or bone ornaments. Preserve the ochre / ivory separation.'),
]

def prepare():
 for p in [ART,ART/'generation',ART/'references',ART/'native',DOC]:p.mkdir(parents=True,exist_ok=True)
 jobs=[]
 original=json.loads((prior.ART/'prompts.json').read_text('utf-8'))['jobs'][0]['prompt']
 common=original.removesuffix(prior.ENTRIES[0][-1])
 for ident,n,batch,label,key,detail in ENTRIES:
  ref=ROOT/f'assets/sprites/art-finish-{batch:02}/FIN-{n:02}.png';target=ART/'references'/f'TOR-{ident}-outfit.png'
  Image.open(ref).resize((1024,1024),Image.Resampling.NEAREST).save(target)
  jobs.append(dict(id=f'TOR-{ident}',illustration=ident,outfit=ident,finish_number=n,batch=batch,label=label,identity='core:'+key,prompt=common+detail,referenced_image_paths=[target.as_posix()],transparent_background=True,output=(ART/f'TOR-{ident}-source.png').as_posix()))
 prior.write(ART/'prompts.json',dict(review='TORSO-STARTING-02',mode='built-in imagegen',status='candidate_not_integrated',jobs=jobs))
 prior.write(DOC/'request.json',dict(review='TORSO-STARTING-02',status='candidate_not_integrated',reason='Live content inspection found five equippable starting torso items absent from the first 14-item review',approval_required=True,first_review_independent=True))
 print('Prepared five missing starting torso icons; existing portraits and worn bodies unchanged')

def build():
 assert not (DOC/'acceptance.json').exists(),'Starting candidates already frozen'
 jobs=json.loads((ART/'prompts.json').read_text('utf-8'))['jobs'];entries=[]
 for job in jobs:
  ident=job['illustration'];n=job['finish_number'];icon,box=prior.normalize(job)
  mask,_=prior.icon_material(icon,{200:113,201:104,202:110,203:109,204:103}[ident])
  if ident==204:
   labels=[]
   for r,g,b,a in icon.get_flattened_data():
    h,l,s=colorsys.rgb_to_hls(r/255,g/255,b/255)
    labels.append((int(27<h*360<100 and s>.25 and .08<l<.68),0,0,255) if a else (0,0,0,0))
   mask.putdata(labels)
  mask_path=ART/'native'/f'TOR-{ident}-material.png';mask.save(mask_path)
  worn=ROOT/'assets/sprites/art-finish-completion-06/worn'
  body=Image.open(worn/f'FIN-{n:02}-base.png').convert('RGBA');body_mask=Image.open(worn/f'FIN-{n:02}-base-mask.png').convert('RGBA')
  overrides={}
  if ident==200:
   # A new reviewed mask includes the shirt, leaving the exposed arms alone.
   for part in ['base','core']:
    original=Image.open(worn/f'FIN-{n:02}-{part}.png').convert('RGBA');mapped=Image.open(worn/f'FIN-{n:02}-{part}-mask.png').convert('RGBA');selection=Image.new('L',mapped.size)
    ImageDraw.Draw(selection).polygon([(57,49),(70,49),(74,63),(72,75),(56,75),(55,62)],fill=255)
    pixels=[]
    for i,(p,m) in enumerate(zip(original.get_flattened_data(),mapped.get_flattened_data())):
     pixels.append((1,0,0,255) if p[3] and selection.getpixel((i%128,i//128)) else m)
    mapped.putdata(pixels);path=ART/'native'/f'FIN-01-{part}-torso-material.png';mapped.save(path);overrides[part]=path.relative_to(ROOT).as_posix()
    if part=='base':body_mask=mapped
  variants=[]
  for letter,label,ramp in prior.PALETTES+([] if ident==203 else [prior.RED_PALETTE]):
   item=prior.recolor(icon,mask,ramp);actor=prior.recolor(body,body_mask,ramp)
   assert item.getchannel('A').tobytes()==icon.getchannel('A').tobytes()
   assert actor.getchannel('A').tobytes()==body.getchannel('A').tobytes()
   if ident==200:assert actor.crop((0,0,128,44)).tobytes()==body.crop((0,0,128,44)).tobytes()
   ip=ART/'native'/f'TOR-{ident}-{letter}.png';bp=ART/'native'/f'TOR-{ident}-{letter}-worn.png';item.save(ip);actor.save(bp)
   variants.append(dict(code=f'TOR-{ident}-{letter}',label=label,icon=ip.relative_to(ROOT).as_posix(),worn=bp.relative_to(ROOT).as_posix(),icon_sha256=prior.sha(ip),worn_sha256=prior.sha(bp)))
  entries.append(dict(id=job['id'],illustration=ident,outfit=ident,finish_number=n,label=job['label'],identity=job['identity'],source=Path(job['output']).relative_to(ROOT).as_posix(),source_sha256=prior.sha(Path(job['output'])),material_mask=mask_path.relative_to(ROOT).as_posix(),new_palette_mask_overrides=overrides,variants=variants))
 assert sum(len(e['variants']) for e in entries)==24
 prior.write(DOC/'candidates.json',dict(review='TORSO-STARTING-02',status='awaiting_human_approval',models=5,appearances=24,entries=entries))
 print('Five starting armor proposals, 24 icon/outfit pairs, not integrated')

def publish():
 entries=json.loads((DOC/'candidates.json').read_text('utf-8'))['entries'];page=Image.new('RGB',(1468,112+5*160),'#11191d');d=ImageDraw.Draw(page)
 d.text((18,12),'Tenues de départ · cinq objets manquants · à valider séparément',font=prior.font(22),fill='#decca7')
 d.text((18,46),'Ancien objet / nouveau torse et tenue · taille native · rouge A conservé sur la tenue rituelle',font=prior.font(16),fill='#abbab5')
 for row,e in enumerate(entries):
  y=100+row*160;d.rectangle((12,y,1456,y+150),fill='#1b272d');d.text((24,y+8),e['id']+' · '+e['label'],font=prior.font(18),fill='#decca7')
  old=Image.open(ROOT/f'assets/sprites/art-finish-completion-06/worn/icon-{e["illustration"]}.png').convert('RGBA');prior.paste(page,old,24,y+63)
  for col,v in enumerate(e['variants']):
   x=148+col*260;d.text((x,y+38),v['code'][-1]+' · '+v['label'],font=prior.font(14),fill='#bbc8c0');d.rectangle((x,y+65,x+63,y+128),fill='#0d1216',outline='#5b625d');prior.paste(page,Image.open(ROOT/v['icon']).convert('RGBA'),x,y+65)
   d.rectangle((x+108,y+65,x+171,y+128),fill='#454841');prior.paste(page,Image.open(ROOT/v['worn']).convert('RGBA'),x+76,y+29)
   prior.paste(page,Image.open(ROOT/v['icon']).convert('RGBA').resize((40,40),Image.Resampling.NEAREST),x+197,y+77)
 page.save(OUT/'depart-native.png')
 parts=['''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Tenues de départ · TORSO-STARTING-02</title><style>body{background:#11191d;color:#d1dad6;margin:24px;font:17px Segoe UI;line-height:1.5}h1,h2{color:#decca7}a{color:#dfc58e}.scroll{overflow:auto;max-width:100%;border:1px solid #43514b}img{max-width:none}</style><h1>Les cinq tenues de départ · complément à valider</h1><p>Le premier inventaire avait omis ces cinq objets équipables. Le lot des quatorze autres armures est accepté séparément. Ces nouveaux dessins TOR-200 à TOR-204 et leurs couleurs sont des candidats ; les portraits et tenues d’origine restent actifs.</p><p>A : origine ; B : bleu ardoise ; C : vert mousse ; D : rouille ; E : rouge sombre. La tenue rituelle est déjà rouge en A. Pour la tenue d’assaut, les nouvelles couleurs incluent explicitement le tissu du torse, avec la peau protégée. Les anciennes couleurs restent inchangées.</p><div class="scroll"><img src="depart-native.png" width="1468" height="912" alt="Cinq tenues de départ et leurs nouvelles icônes à taille native"></div>''']
 for e in entries:
  parts.append(f'<h2>{e["id"]} · {html.escape(e["label"])}</h2><p><a href="../../{e["source"]}">Dessin source</a></p>')
 parts.append('<p><a href="index.html#integration">Lot validé et captures en jeu</a> · <a href="../../assets/source-art/torso-starting-review-02/prompts.json">Prompts imagegen intégrés</a></p></html>')
 (OUT/'depart.html').write_text('\n'.join(parts),encoding='utf-8')
 prior.write(DOC/'checks.json',dict(models=5,appearances=24,alpha_preserved=True,native_icon_side=64,inventory_preview_side=40,source_count=5,starting_bastion_shirt_mask_proposed=True,runtime_integration=False,proof_sha256=prior.sha(OUT/'depart-native.png')))

if __name__=='__main__':
 import sys
 if '--prepare' in sys.argv:prepare()
 if '--build' in sys.argv:build()
 if '--publish' in sys.argv:publish()
