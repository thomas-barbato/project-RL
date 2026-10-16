"""Layout CPU renderer outputs only: lossless encoding, crop and nearest zoom."""
from pathlib import Path
import base64, hashlib, json
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
MODELS=json.loads((OUT/'modeles.json').read_text(encoding='utf-8'))
POSES=('Sans arme','Épée','Fusil','Pistolet','Épée à deux mains','Deux épées','Deux pistolets','Épée + pistolet')
PALETTES=("D’origine",'Rouge','Bleu','Violet','Turquoise','Jaune','Rose')
DUOS=('Rouge / ivoire','Bleu / ivoire','Vert / bleu nuit','Ivoire / bleu','Cuivre / bleu nuit','Violet / turquoise','Bleu / ivoire','Anthracite / turquoise','Cuivre / ivoire','Ivoire / bleu','Bleu-vert / ivoire','Violet / ivoire','Olive / bleu nuit','Rouge / anthracite','Jaune / violet','Bleu / ivoire','Rouge / ivoire')
font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',18)
small=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',15)
title=ImageFont.truetype('C:/Windows/Fonts/arialbd.ttf',24)
BG=(16,23,29,255); ROW=(24,32,39,255); INK=(231,237,242); MUTED=(172,190,203); ACCENT=(107,212,213)
sheets={}
for side in (48,128):
    for palette in range(7):
        for version in ('avant','apres'):
            path=OUT/f'{version}-{side}-{palette}.rgba'
            sheet=Image.frombytes('RGBA',(side*8,side*17),path.read_bytes())
            sheet.save(path.with_suffix('.png'))
            sheets[version,side,palette]=sheet

def tile(version,index,pose=0,palette=0,side=48):
    return sheets[version,side,palette].crop((pose*side,index*side,(pose+1)*side,(index+1)*side))

def blit(board,version,index,x,y,pose=0,palette=0,zoom=1,side=48):
    image=tile(version,index,pose,palette,side)
    if zoom!=1: image=image.resize((int(side*zoom),int(side*zoom)),Image.Resampling.NEAREST)
    board.alpha_composite(image,(x,y))

for page,start in enumerate((0,6,12),1):
    count=min(6,17-start)
    board=Image.new('RGBA',(1120,115+count*162),BG);d=ImageDraw.Draw(board)
    d.text((18,12),f'PROPOSITIONS {start+1:02d}–{start+count:02d} / TENUES BICOLORES',font=title,fill=INK)
    d.text((18,44),'Corps de 30 px · taille native et zoom ×2 · propositions de recoloration, dessins existants',font=small,fill=MUTED)
    for x,label in [(228,'Actuel'),(306,'Essai'),(402,'Zoom ×2'),(558,'Pistolet'),(700,'Fusil'),(841,'Deux mains'),(983,'Mixte')]:
        d.text((x,83),label,font=small,fill=ACCENT)
    for row,index in enumerate(range(start,start+count)):
        y=110+row*162
        d.rectangle((8,y,1112,y+154),fill=ROW if row%2==0 else BG)
        d.text((18,y+34),f'{index+1:02d}',font=title,fill=ACCENT)
        name=MODELS[index]['name']
        if d.textbbox((0,0),name,font=font)[2]>200:
            words=name.split();mid=len(words)//2
            d.text((18,y+69),' '.join(words[:mid]),font=font,fill=INK)
            d.text((18,y+92),' '.join(words[mid:]),font=font,fill=INK)
        else:d.text((18,y+69),name,font=font,fill=INK)
        d.text((18,y+125),DUOS[index],font=small,fill=MUTED)
        blit(board,'avant',index,228,y+50)
        blit(board,'apres',index,306,y+50)
        blit(board,'apres',index,390,y+6,zoom=2)
        for x,pose in [(542,3),(684,2),(826,4),(968,7)]:blit(board,'apres',index,x,y+6,pose=pose,zoom=2)
    board.save(OUT/f'propositions-{start+1:02d}-{start+count:02d}.png')
    variants=Image.new('RGBA',(1120,100+count*132),BG);d=ImageDraw.Draw(variants)
    d.text((18,12),f'VARIANTES {start+1:02d}–{start+count:02d} / CORPS 30 PX / ZOOM ×2',font=title,fill=INK)
    for p,name in enumerate(PALETTES):d.text((278+p*116,56),name,font=small,fill=ACCENT)
    for row,index in enumerate(range(start,start+count)):
        y=90+row*132;d.rectangle((8,y,1112,y+126),fill=ROW if row%2==0 else BG)
        d.text((18,y+30),f'{index+1:02d} {MODELS[index]["name"]}',font=font,fill=INK)
        for p in range(7):blit(variants,'apres',index,264+p*116,y,pose=0,palette=p,zoom=2)
    variants.save(OUT/f'variantes-{start+1:02d}-{start+count:02d}.png')

cards=OUT/'fiches';cards.mkdir(exist_ok=True)
for index,model in enumerate(MODELS):
    board=Image.new('RGBA',(1200,120+7*136),BG);d=ImageDraw.Draw(board)
    d.text((18,10),f'{index+1:02d} / {model["name"]} / SEPT VARIANTES ET HUIT POSES',font=title,fill=INK)
    d.text((18,45),'Corps de 30 pixels, zoom ×2. Le duo de couleurs ne change ni objet ni statistiques.',font=small,fill=MUTED)
    for p,name in enumerate(POSES):d.text((134+p*132,81),name,font=small,fill=ACCENT)
    for pal,name in enumerate(PALETTES):
        y=108+pal*136;d.rectangle((8,y,1190,y+126),fill=ROW if pal%2==0 else BG)
        d.text((18,y+45),name,font=font,fill=INK)
        for pose in range(8):blit(board,'apres',index,130+pose*132,y,pose=pose,palette=pal,zoom=2)
    board.save(cards/f'{index+1:02d}.png')

# Compact standalone browser gallery with all 17 numbered models, all seven
# palettes and all eight poses, both 48px/native and inspection sources.
def encoded(image):
    import io
    buffer=io.BytesIO();image.save(buffer,format='PNG')
    return 'data:image/png;base64,'+base64.b64encode(buffer.getvalue()).decode()
gallery={'models':MODELS,'duos':DUOS,'poses':POSES,'palettes':PALETTES,'sheets':{f'{v}-{s}-{p}':encoded(im) for (v,s,p),im in sheets.items()}}
html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>17 tenues — propositions numérotées</title>
<style>body{background:#10171d;color:#e7edf2;font:16px system-ui;margin:24px}h1{font-size:25px;margin-bottom:8px}p{color:#afc4ce}button,select{background:#1e303c;color:#e7edf2;border:1px solid #496672;border-radius:6px;padding:8px;cursor:pointer}button.active{border-color:#71e0d4;color:#71e0d4}nav{display:flex;gap:8px;flex-wrap:wrap;max-width:1240px;margin:20px 0}.controls{display:flex;gap:18px;flex-wrap:wrap}section{background:#182128;border-radius:8px;padding:18px;margin:18px 0}canvas{image-rendering:pixelated}.tiles{display:flex;gap:20px;flex-wrap:wrap}.cell{text-align:center;border:1px solid #3b4e5a;padding:12px;min-width:120px}.cell label{display:block;font-size:13px;color:#afc4ce}.pose{display:flex;gap:12px;flex-wrap:wrap}.pose>div{text-align:center;width:134px}.pose label{display:block;font-size:12px}a{color:#71e0d4}.note{max-width:1050px}</style>
<h1>17 propositions numérotées : armures et tenues de base</h1><p class="note">Recoloration des dessins existants, sans intégration dans le jeu. Corps de 30 pixels, contours et alpha inchangés. Deux zones de couleur, trois valeurs par matière colorée. Les armes et les silhouettes ne sont pas redessinées dans cet essai.</p>
<nav id="models"></nav><div class="controls"><label>Variante <select id="palette"></select></label><label>Pose <select id="pose"></select></label></div>
<section><h2 id="name"></h2><p id="duo"></p><div class="tiles" id="comparison"></div></section><section><h2>Toutes les poses de cette variante</h2><div id="poses" class="pose"></div></section><section><h2>Les sept variantes de cette tenue</h2><div id="variants" class="pose"></div></section>
<p>Les numéros 01–03 sont les tenues de base ; 04–17 sont les 14 modèles d’armure de torse existants. Même numérotation dans les planches et fiches PNG. Pas de modification d’objet, de statistiques, de sauvegarde ou de collision.</p>
<script>const data=__DATA__;let model=3;const cache={};for(const [key,url] of Object.entries(data.sheets)){const im=new Image();im.src=url;cache[key]=im}function option(s,values){values.forEach((v,i)=>s.add(new Option(v,i)))}option(document.querySelector('#palette'),data.palettes);option(document.querySelector('#pose'),data.poses);
function renderCanvas(version,side,palette,pose,scale){const c=document.createElement('canvas');c.width=side*scale;c.height=side*scale;const ctx=c.getContext('2d');ctx.imageSmoothingEnabled=false;const im=cache[`${version}-${side}-${palette}`];const draw=()=>ctx.drawImage(im,pose*side,model*side,side,side,0,0,c.width,c.height);if(im.complete)draw();else im.addEventListener('load',draw,{once:true});return c}
function cell(target,label,version,side,palette,pose,scale){const box=document.createElement('div');box.className='cell';const text=document.createElement('label');text.textContent=label;box.append(text,renderCanvas(version,side,palette,pose,scale));target.append(box)}
function show(){document.querySelectorAll('nav button').forEach((b,i)=>b.classList.toggle('active',i===model));const pal=Number(document.querySelector('#palette').value),pose=Number(document.querySelector('#pose').value);document.querySelector('#name').textContent=String(model+1).padStart(2,'0')+' — '+data.models[model].name;document.querySelector('#duo').textContent='Duo d’origine proposé : '+data.duos[model]+'. Variante sélectionnée : '+data.palettes[pal]+'.';const comparison=document.querySelector('#comparison');comparison.replaceChildren();cell(comparison,'Actuel / corps 30 px','avant',48,pal,pose,1);cell(comparison,'Proposition / corps 30 px','apres',48,pal,pose,1);cell(comparison,'Actuel / zoom ×3','avant',48,pal,pose,3);cell(comparison,'Proposition / zoom ×3','apres',48,pal,pose,3);cell(comparison,'Inspection actuelle / 128 px','avant',128,pal,pose,1);cell(comparison,'Inspection proposée / 128 px','apres',128,pal,pose,1);const all=document.querySelector('#poses');all.replaceChildren();data.poses.forEach((p,i)=>cell(all,p,'apres',48,pal,i,2));const variants=document.querySelector('#variants');variants.replaceChildren();data.palettes.forEach((p,i)=>cell(variants,p,'apres',48,i,pose,2))}
data.models.forEach((m,i)=>{const b=document.createElement('button');b.textContent=String(i+1).padStart(2,'0')+' '+m.name;b.onclick=()=>{model=i;show()};document.querySelector('nav').append(b)});document.querySelectorAll('select').forEach(s=>s.onchange=show);show();</script></html>'''.replace('__DATA__',json.dumps(gallery,ensure_ascii=False))
(OUT/'galerie.html').write_text(html,encoding='utf-8')
checks=json.loads((OUT/'verification.json').read_text())
before=json.loads((OUT/'baseline.json').read_text())
changed=[path for path,digest in before.items() if hashlib.sha256((ROOT/path).read_bytes()).hexdigest()!=digest]
(OUT/'preservation.json').write_text(json.dumps({'active_files_checked':len(before),'changed':changed,'proposal_variants_checked':len(checks)},indent=2))
print(f'Three numbered main boards, three variant boards, 17 full pose/palette cards and browser gallery; active changes: {changed}.')
