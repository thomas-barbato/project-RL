"""Publish HELM03 icons and a visor-only pulse without changing runtime artwork."""
from pathlib import Path
import hashlib
import json
import math
import shutil

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'assets/source-art/animation-polish-01/helmet-icons-r01'
REVIEW = ROOT / 'docs/reviews/animation-polish-01'
OUT = ROOT / 'artifacts/animation-polish-01/helmet-icons-r01/current'
LABELS = ['Coque industrielle', 'Optique asymétrique', 'Coque organique']
PULSE_PERIOD_MS = 3600
PULSE_START_MS = 3000
PULSE_DURATION_MS = 400
PULSE_DEPTH = 0.38


def visor_mask(image, number):
    """Explicit glass windows plus color selection, checked in the review atlas."""
    mask = Image.new('RGBA', (64, 64))
    for y in range(64):
        for x in range(64):
            r, g, b, a = image.getpixel((x, y))
            if number == 1:
                hit = 30 <= x <= 55 and 31 <= y <= 33 and r > g+35 and g > b+45 and g >= 100
            elif number == 2:
                hit = 24 <= x <= 37 and 29 <= y <= 34 and g > r+25 and b > r+25 and g >= 95
            else:
                hit = 28 <= x <= 49 and 33 <= y <= 36 and g >= r+3 and g > b+15 and g >= 70
            if hit and a >= 200:
                mask.putpixel((x, y), (255, 255, 255, 255))
    assert mask.getbbox(), f'Empty visor mask for candidate {number}'
    return mask


def dimmed(image, mask, brightness):
    result = image.copy()
    for y in range(64):
        for x in range(64):
            if mask.getpixel((x, y))[3]:
                r, g, b, a = image.getpixel((x, y))
                result.putpixel((x, y), (round(r*brightness), round(g*brightness), round(b*brightness), a))
    return result


def native_sheet(images):
    sheet = Image.new('RGB', (720, 292), (30, 40, 49))
    draw = ImageDraw.Draw(sheet)
    font = lambda size: ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', size)
    draw.text((16, 10), 'HELM03 · trois nouvelles icônes à taille native', font=font(18), fill='white')
    draw.text((16, 36), 'Même dessin sur fond sombre et clair · 64 px · propositions à examiner', font=font(13), fill=(174,191,200))
    for n, (image, label) in enumerate(zip(images, LABELS)):
        x = n*240
        draw.text((x+16, 72), f'{n+1:02} · {label}', font=font(15), fill='white')
        sheet.paste(image, (x+36, 106), image)
        draw.rectangle((x+122, 100, x+202, 180), fill=(180,170,160))
        sheet.paste(image, (x+130, 108), image)
        silhouette = Image.new('RGBA',(64,64),(226,233,236,0))
        silhouette.putalpha(image.getchannel('A'))
        sheet.paste(silhouette,(x+82,210),silhouette)
    draw.text((16, 186), 'Lecture des silhouettes', font=font(12), fill=(174,191,200))
    return sheet


def build():
    if not (SOURCE/'H03-I03-native64.png').exists():
        return '', ''
    OUT.mkdir(parents=True,exist_ok=True)
    records, images, masks = [], [], []
    proof = Image.new('RGB',(576,204),(30,40,49))
    proof_draw = ImageDraw.Draw(proof)
    for n in range(1,4):
        label = f'H03-I{n:02}'
        source = SOURCE/f'{label}-native64.png'
        image = Image.open(source).convert('RGBA')
        assert image.size == (64,64)
        mask = visor_mask(image,n)
        mask_path = SOURCE/f'{label}-visor-mask.png'
        mask.save(mask_path)
        shutil.copyfile(source,OUT/source.name)
        shutil.copyfile(mask_path,OUT/mask_path.name)
        dim = dimmed(image,mask,1-PULSE_DEPTH)
        dim.save(OUT/f'{label}-dim.png')
        for col, im in enumerate([image,mask,dim]):
            proof.paste(im,((n-1)*192+col*64,32),im)
        proof_draw.text(((n-1)*192+4,10),f'{label}: fixe / masque / minimum',fill='white')
        proof.paste(image.resize((128,128),Image.Resampling.NEAREST),((n-1)*192+32,76),image.resize((128,128),Image.Resampling.NEAREST))
        selected = mask.getchannel('A').histogram()[255]
        records.append({'id':label,'native':source.relative_to(ROOT).as_posix(),
                        'native_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                        'visor_mask':mask_path.relative_to(ROOT).as_posix(),
                        'mask_sha256':hashlib.sha256(mask_path.read_bytes()).hexdigest(),
                        'visor_pixels':selected,'visor_bounds':list(mask.getbbox()),
                        'native_alpha16_bounds':list(image.getchannel('A').point(lambda a:255 if a>=16 else 0).getbbox())})
        images.append(image);masks.append(mask)
    proof.save(OUT/'visor-mask-review.png')
    native_sheet(images).save(OUT/'H03-native.png')
    # Native export of the same material animation; no re-decoding during playback.
    frames=[]
    for frame in range(36):
        tick=frame*100
        local=(tick-PULSE_START_MS)/PULSE_DURATION_MS
        brightness=1-PULSE_DEPTH*math.sin(math.pi*local)**2 if 0<=local<=1 else 1
        sheet=Image.new('RGB',(288,104),(30,40,49))
        draw=ImageDraw.Draw(sheet)
        for n,(image,mask) in enumerate(zip(images,masks)):
            draw.text((n*96+10,6),f'0{n+1}',fill='white')
            im=dimmed(image,mask,brightness)
            sheet.paste(im,(n*96+16,29),im)
        frames.append(sheet)
    frames[0].save(OUT/'H03-visors-native.webp',save_all=True,append_images=frames[1:],duration=100,loop=0,lossless=True,method=6)
    report={'review':'ANIM-POLISH01-HELM03-ICONS-r01','native_tile_px':64,'icons':records,
            'effect':'H03-P01','period_ms':PULSE_PERIOD_MS,'pulse_start_ms':PULSE_START_MS,
            'dim_duration_ms':PULSE_DURATION_MS,'minimum_brightness':1-PULSE_DEPTH,
            'shape_position_alpha_constant':True,'scope':'visor pixels only',
            'normal_game_enabled':False,'visual_acceptance_received':False}
    (REVIEW/'HELM03-PULSE.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    cards=''
    descriptions=[
        'Couronne large et basse, front en gradins, mâchoire anguleuse ; cuivre mat et visière ambre.',
        'Silhouette effilée, optique cyan décentrée et petit capteur latéral.',
        'Grandes plaques osseuses segmentées, crête courte et masque fendu ; ocre et olive.'
    ]
    for n,(label,description) in enumerate(zip(LABELS,descriptions),1):
        cards+=f'''<article class="helmet-icon-card"><h3>{n:02} · {label}</h3><div class="icon-labels"><span>Fixe</span><span>Visière animée</span></div><div class="icon-well"><canvas id="new-helmet-icon-{n}" width="192" height="84" style="display:block;width:192px;height:84px;image-rendering:pixelated" aria-label="Icône {n:02} native fixe et avec pulsation de visière"></canvas></div><p class="note">{description}</p><details><summary>Examiner le dessin au zoom ×4</summary><div class="icon-well"><img src="helmet-icons-r01/current/H03-I{n:02}-native64.png" width="256" height="256" style="display:block;width:256px;height:256px;max-width:none;image-rendering:pixelated" alt="{label}, même icône agrandie ×4"></div></details></article>'''
    section=f'''<section id="nouvelles-icones" class="helmet-icons-review" data-ground="dark"><style>.helmet-icon-cards{{display:flex;gap:16px;flex-wrap:wrap;align-items:start}}.helmet-icon-card{{width:256px;border:1px solid #3c505b;padding:12px;box-sizing:border-box}}.helmet-icon-card h3{{font-size:16px;margin:0 0 16px}}.icon-well{{background:#1e2831;overflow:auto}}.helmet-icons-review[data-ground="light"] .icon-well{{background:#b4aaa0}}.icon-labels{{display:flex;gap:24px;margin:8px 14px;font-size:12px;color:#acbdc6}}.helmet-icon-card .note{{font-size:13px;min-height:72px}}.helmet-icon-cards details{{font-size:13px}}#helmet-icons-status{{color:#acbdc6;font-size:13px}}</style>
<h2>HELM03 · nouvelles icônes de casque</h2><p>Trois dessins proposés pour des familles vraiment distinctes, à comparer avant de travailler leur version portée. Les intitulés servent à cette revue ; le nombre final de modèles reste à choisir.</p>
<div class="controls"><label>Fond<select id="helmet-icons-ground"><option value="dark">Sombre</option><option value="light">Clair</option></select></label><button id="helmet-icons-toggle">Visières : animation activée</button><button id="helmet-icons-replay">Rejouer le clignotement</button></div>
<div class="icon-well" style="width:336px;max-width:100%"><canvas id="new-helmet-icons-overview" width="336" height="106" style="display:block;width:336px;height:106px;image-rendering:pixelated" aria-label="Les trois nouvelles icônes côte à côte à taille native, 01 industrielle, 02 optique et 03 organique"></canvas></div><p class="note">Les trois propositions côte à côte à taille de jeu. Comparaisons fixes et animées détaillées ci-dessous.</p>
<div class="helmet-icon-cards">{cards}</div><p id="helmet-icons-status" role="status">Chargement des icônes…</p>
<p class="note">Chaque icône est affichée à 64 px. La comparaison de droite anime uniquement le verre coloré : une brève baisse de luminosité toutes les 3,6 secondes, avec une version fixe à gauche. Le dessin et l’animation attendent des avis distincts.</p>
<details><summary>Voir aussi les silhouettes et les deux fonds à taille native</summary><div class="native" style="max-width:720px"><img src="helmet-icons-r01/current/H03-native.png" width="720" height="292" style="display:block;max-width:none;image-rendering:pixelated" alt="Trois nouvelles icônes natives, sur fond sombre et clair, avec leurs silhouettes"></div></details>
<p class="note">Étape suivante après le choix des icônes : adapter le casque porté à chaque classe et à son armure, avec des formes cohérentes de face, de dos et de côté. Les six objets actuels ne sont pas encore affectés à ces pistes.</p></section>'''
    script='''let helmetIconFrames=[],helmetIconAnimation=true,helmetIconStart=performance.now();
const helmetIconStatus=$('helmet-icons-status');
function helmetIconBrightness(t){const phase=((t%3600)+3600)%3600;return phase>=3000&&phase<=3400?1-.38*Math.sin(Math.PI*(phase-3000)/400)**2:1;}
async function loadHelmetIconArt(){try{const data=await Promise.all([1,2,3].map(async n=>{const images=await Promise.all(['native64','visor-mask'].map(async kind=>{const im=new Image();im.src=`helmet-icons-r01/current/H03-I0${n}-${kind}.png`;await im.decode();return im;}));const c=document.createElement('canvas');c.width=c.height=64;const ctx=c.getContext('2d',{willReadFrequently:true});ctx.drawImage(images[0],0,0);const original=ctx.getImageData(0,0,64,64);ctx.clearRect(0,0,64,64);ctx.drawImage(images[1],0,0);const mask=ctx.getImageData(0,0,64,64).data;const levels=[];for(let level=0;level<17;level++){const pixels=new ImageData(new Uint8ClampedArray(original.data),64,64);const b=1-.38*level/16;for(let p=0;p<pixels.data.length;p+=4)if(mask[p+3])for(let ch=0;ch<3;ch++)pixels.data[p+ch]=Math.round(pixels.data[p+ch]*b);const frame=document.createElement('canvas');frame.width=frame.height=64;frame.getContext('2d').putImageData(pixels,0,0);levels.push(frame);}return {levels,number:n,canvas:$(`new-helmet-icon-${n}`),level:-1};}));helmetIconFrames=data;helmetIconStatus.textContent='Icônes natives prêtes · fixe à gauche, essai de visière à droite.';}catch(error){helmetIconStatus.textContent='Impossible de charger une icône : '+error.message;}}
$('helmet-icons-ground').onchange=()=>{document.getElementById('nouvelles-icones').dataset.ground=$('helmet-icons-ground').value;for(const row of helmetIconFrames)row.level=-1;};
$('helmet-icons-toggle').onclick=()=>{helmetIconAnimation=!helmetIconAnimation;$('helmet-icons-toggle').textContent=helmetIconAnimation?'Visières : animation activée':'Visières : fixes';};
$('helmet-icons-replay').onclick=()=>{helmetIconAnimation=true;helmetIconStart=performance.now()-2900;$('helmet-icons-toggle').textContent='Visières : animation activée';};
function paintHelmetIconArt(now){const b=helmetIconAnimation?helmetIconBrightness(now-helmetIconStart):1;const level=Math.round((1-b)/.38*16);for(const row of helmetIconFrames)if(row.level!==level){const ctx=row.canvas.getContext('2d');ctx.imageSmoothingEnabled=false;ctx.clearRect(0,0,192,84);ctx.drawImage(row.levels[0],16,8);ctx.drawImage(row.levels[level],112,8);const overview=$('new-helmet-icons-overview').getContext('2d');const x=(row.number-1)*112;overview.clearRect(x,0,112,106);overview.imageSmoothingEnabled=false;overview.font='13px system-ui';overview.fillStyle=$('helmet-icons-ground').value==='light'?'#11191e':'#e4ebee';overview.fillText(`0${row.number}`,x+24,17);overview.drawImage(row.levels[level],x+24,28);row.level=level;}requestAnimationFrame(paintHelmetIconArt);}
loadHelmetIconArt();requestAnimationFrame(paintHelmetIconArt);'''
    return section,script


if __name__=='__main__':
    build()
    print(json.dumps({'preview':str(OUT),'icon_candidates':3}))
