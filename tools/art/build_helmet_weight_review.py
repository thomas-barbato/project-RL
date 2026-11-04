"""HELM04: five weight-stage icons with one shared icon/worn identity per model."""
from pathlib import Path
import hashlib
import json
import math
import shutil
from PIL import Image, ImageDraw, ImageFont
from build_helmet_icons_review import dimmed

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT/'assets/source-art/animation-polish-01/helmet-weight-r01'
PRIOR = ROOT/'assets/source-art/animation-polish-01/helmet-icons-r01'
REVIEW = ROOT/'docs/reviews/animation-polish-01'
OUT = ROOT/'artifacts/animation-polish-01/helmet-weight-r01/current'
ENTRIES = [
    {'number':1,'weight':'Très léger','label':'Protection ouverte','prior':None,
     'description':'Arceaux, sangles et petites protections des tempes ; visage et mâchoire dégagés.'},
    {'number':2,'weight':'Léger','label':'Optique asymétrique','prior':'H03-I02',
     'description':'Coque effilée, découpe ouverte sous la tempe, optique cyan décentrée.'},
    {'number':3,'weight':'Intermédiaire','label':'Coque organique','prior':'H03-I03',
     'description':'Grandes plaques segmentées, crête courte et masque inférieur fendu.'},
    {'number':4,'weight':'Lourd','label':'Coque industrielle','prior':'H03-I01',
     'description':'Couronne large, front en gradins et mâchoire anguleuse ; cuivre mat et visière ambre.'},
    {'number':5,'weight':'Très lourd','label':'Protection intégrale','prior':None,
     'description':'Coque fermée enveloppante, front épais et large anneau de verrouillage de nuque.'},
]


def font(size):
    return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prepare():
    SOURCE.mkdir(parents=True,exist_ok=True)
    records=[]
    for entry in ENTRIES:
        label=f'H04-W{entry["number"]:02}'
        target=SOURCE/f'{label}-native64.png'
        if entry['prior']:
            raw=PRIOR/f'{entry["prior"]}-generated.png'
            original=PRIOR/f'{entry["prior"]}-native64.png'
            shutil.copyfile(original,target)
            detail={'native_reused_from':original.relative_to(ROOT).as_posix(),
                    'pixel_identical_to_prior':sha(original)==sha(target)}
        else:
            raw=SOURCE/f'{label}-generated.png'
            image=Image.open(raw).convert('RGBA')
            assert image.getchannel('A').getextrema()[0]==0
            bounds=image.getchannel('A').point(lambda a:255 if a>=16 else 0).getbbox()
            crop=image.crop(bounds)
            scale=54/max(crop.size)
            size=tuple(max(1,round(v*scale)) for v in crop.size)
            native=crop.convert('RGBa').resize(size,Image.Resampling.LANCZOS).convert('RGBA')
            canvas=Image.new('RGBA',(64,64))
            origin=tuple((64-v)//2 for v in size)
            canvas.paste(native,origin)
            canvas.save(target)
            detail={'source_alpha16_bounds':list(bounds),'conditioned_object_size':list(size),
                    'native_origin':list(origin),'conditioning':'premultiplied alpha, aspect-preserving Lanczos to max54px, centered on native64; no repainting'}
        record={**entry,'id':label,'raw':raw.relative_to(ROOT).as_posix(),
                'raw_sha256':sha(raw),'native':target.relative_to(ROOT).as_posix(),
                'native_sha256':sha(target),**detail}
        records.append(record)
    return records


def new_visor_mask(image):
    mask=Image.new('RGBA',(64,64))
    for y in range(26,37):
        for x in range(30,58):
            r,g,b,a=image.getpixel((x,y))
            if a>=200 and r>=120 and r>g+70 and g>b+5:
                mask.putpixel((x,y),(255,255,255,255))
    assert mask.getbbox(),'Missing red-orange visor pixels'
    return mask


def native_sheet(images,silhouettes=False):
    sheet=Image.new('RGB',(800,178 if silhouettes else 280),(30,40,49))
    draw=ImageDraw.Draw(sheet)
    draw.text((16,10),'HELM04 · cinq casques selon les poids visuels d’armure',font=font(18),fill='white')
    for n,(entry,image) in enumerate(zip(ENTRIES,images)):
        x=n*160
        draw.text((x+10,43),f'{n+1:02} · {entry["weight"]}',font=font(13),fill=(227,235,238))
        if silhouettes:
            tile=Image.new('RGBA',(64,64),(227,235,238,0))
            tile.putalpha(image.getchannel('A'))
            sheet.paste(tile,(x+48,74),tile)
        else:
            sheet.paste(image,(x+48,67),image)
            draw.rectangle((x+40,153,x+120,233),fill=(180,170,160))
            sheet.paste(image,(x+48,161),image)
    draw.text((16,sheet.height-25),'Icônes natives 64 px · mêmes modèles sur toutes les classes · dessins à juger',font=font(12),fill=(174,191,200))
    return sheet


def build():
    if not (SOURCE/'H04-W05-generated.png').exists():
        return '', ''
    records=prepare()
    OUT.mkdir(parents=True,exist_ok=True)
    images,masks=[],[]
    for row in records:
        image=Image.open(ROOT/row['native']).convert('RGBA')
        assert image.size==(64,64)
        if row['prior']:
            mask=Image.open(PRIOR/f'{row["prior"]}-visor-mask.png').convert('RGBA')
        elif row['number']==1:
            mask=Image.new('RGBA',(64,64))
        else:
            mask=new_visor_mask(image)
        mask_path=SOURCE/f'{row["id"]}-visor-mask.png'
        mask.save(mask_path)
        row.update({'visor_mask':mask_path.relative_to(ROOT).as_posix(),
                    'visor_pixels':mask.getchannel('A').histogram()[255],
                    'visor_bounds':list(mask.getbbox()) if mask.getbbox() else None,
                    'mask_sha256':sha(mask_path)})
        shutil.copyfile(ROOT/row['native'],OUT/f'{row["id"]}-native64.png')
        shutil.copyfile(mask_path,OUT/mask_path.name)
        dimmed(image,mask,.62).save(OUT/f'{row["id"]}-dim.png')
        images.append(image);masks.append(mask)
    native_sheet(images).save(OUT/'H04-native.png')
    native_sheet(images,True).save(OUT/'H04-silhouettes.png')
    proof=Image.new('RGB',(960,100),(30,40,49))
    for n,(image,mask) in enumerate(zip(images,masks)):
        for col,tile in enumerate([image,mask,dimmed(image,mask,.62)]):
            proof.paste(tile,(n*192+col*64,28),tile)
        ImageDraw.Draw(proof).text((n*192+4,7),f'W{n+1:02} fixe / masque / creux',fill='white')
    proof.save(OUT/'visor-mask-review.png')
    frames=[]
    for frame in range(36):
        t=frame*100
        local=(t-3000)/400
        brightness=1-.38*math.sin(math.pi*local)**2 if 0<=local<=1 else 1
        sheet=Image.new('RGB',(480,110),(30,40,49))
        draw=ImageDraw.Draw(sheet)
        for n,(row,image,mask) in enumerate(zip(records,images,masks)):
            draw.text((n*96+8,8),f'{n+1:02}',font=font(12),fill='white')
            tile=dimmed(image,mask,brightness)
            sheet.paste(tile,(n*96+16,31),tile)
        frames.append(sheet)
    frames[0].save(OUT/'H04-visors-native.webp',save_all=True,append_images=frames[1:],duration=100,loop=0,lossless=True,method=6)
    accepted=(REVIEW/'HELM04-ACCEPTANCE.json').exists()
    (REVIEW/'HELM04-SOURCES.json').write_text(json.dumps({'review':'ANIM-POLISH01-HELM04-WEIGHTS-r01',
        'native_tile_px':64,'icons':records,'new_drawings':2,'reused_unchanged_drawings':3,
        'pulse':{'period_ms':3600,'dim_duration_ms':400,'minimum_brightness':.62,'scope':'visor material only'},
        'common_design_all_classes':True,'normal_game_enabled':False,'visual_acceptance_received':accepted},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    overview=''.join(f'''<div class="weight-overview-item"><span>{row['number']:02} · {row['weight']}</span><canvas id="weight-overview-{row['number']}" width="96" height="84" aria-label="Casque {row['number']:02}, {row['weight']}, taille native"></canvas></div>''' for row in records)
    cards=''
    for row in records:
        n=row['number']
        caption='Visière animée' if row['visor_pixels'] else 'Sans visière'
        cards+=f'''<article class="weight-helmet-card"><h3>{n:02} · {row['weight']}</h3><p>{row['label']}</p><div class="weight-pair-labels"><span>Fixe</span><span>{caption}</span></div><div class="weight-well"><canvas id="weight-pair-{n}" width="192" height="84" aria-label="Casque {n:02} fixe et essai de visière à taille native"></canvas></div><p class="note">{row['description']}</p><details><summary>Zoom ×4 sur ce dessin</summary><div class="weight-well"><img src="helmet-weight-r01/current/{row['id']}-native64.png" width="256" height="256" style="display:block;width:256px;height:256px;max-width:none;image-rendering:pixelated" alt="{row['label']}, même icône au zoom entier ×4"></div></details></article>'''
    section=f'''<section id="nouvelles-icones" class="weight-helmet-review" data-ground="dark"><style>.weight-overview{{display:flex;flex-wrap:wrap;gap:8px;max-width:552px}}.weight-overview-item{{width:96px;text-align:center}}.weight-overview-item span{{display:block;font-size:12px;padding:4px 0}}.weight-overview-item canvas{{display:block;width:96px;height:84px;image-rendering:pixelated}}.weight-well,.weight-overview-item{{background:#1e2831}}.weight-helmet-review[data-ground="light"] .weight-well,.weight-helmet-review[data-ground="light"] .weight-overview-item{{background:#b4aaa0;color:#11191e}}.weight-helmet-cards{{display:flex;flex-wrap:wrap;gap:16px;align-items:start;margin-top:24px}}.weight-helmet-card{{width:256px;box-sizing:border-box;border:1px solid #3c505b;padding:12px}}.weight-helmet-card h3{{font-size:16px;margin:0 0 10px}}.weight-helmet-card>p{{font-size:13px}}.weight-helmet-card .note{{min-height:72px}}.weight-pair-labels{{display:flex;gap:26px;font-size:12px;color:#acbdc6;margin:8px 14px}}.weight-well{{overflow:auto}}.weight-well canvas{{display:block;width:192px;height:84px;image-rendering:pixelated}}</style>
<h2 id="gamme-casques">HELM04 · les cinq casques</h2><p>Un modèle par degré de poids visuel d’armure, du très léger au très lourd. Chaque casque gardera le même dessin dans l’icône et sur tous les porteurs ; ses dimensions et son placement s’adapteront à leur carrure et à leur armure.</p>
<div class="controls"><label>Fond<select id="weight-helmet-ground"><option value="dark">Sombre</option><option value="light">Clair</option></select></label><button id="weight-helmet-toggle">Visières : animation activée</button><button id="weight-helmet-replay">Rejouer le clignotement</button></div>
<div class="weight-overview">{overview}</div><p id="weight-helmet-status" role="status" class="note">Chargement des cinq icônes…</p>
<p class="note">Les cinq modèles sont affichés à 64 px. La très légère reste fixe ; les quatre visières colorées ont un essai de clignotement discret. Leurs dessins et l’animation restent à juger.</p>
<details><summary>Comparer les cinq sur les deux fonds</summary><div class="native" style="max-width:800px"><img src="helmet-weight-r01/current/H04-native.png" width="800" height="280" style="display:block;max-width:none;image-rendering:pixelated" alt="Les cinq casques à taille native sur fonds sombre et clair"></div></details><details><summary>Comparer seulement leurs silhouettes</summary><div class="native" style="max-width:800px"><img src="helmet-weight-r01/current/H04-silhouettes.png" width="800" height="178" style="display:block;max-width:none;image-rendering:pixelated" alt="Silhouettes des cinq casques, du très léger au très lourd"></div></details>
<div class="weight-helmet-cards">{cards}</div><p class="note">Cette revue porte d’abord sur les icônes. Les vues portées montreront ensuite ces mêmes objets de face, de dos et de côté, ajustés au personnage et à l’armure. Les six objets actuels ne sont pas encore réaffectés à cette gamme de cinq dessins.</p></section>'''
    entries=[{'number':r['number'],'id':r['id'],'animated':bool(r['visor_pixels'])} for r in records]
    if accepted:
        section=section.replace('Leurs dessins et l’animation restent à juger.','Les cinq dessins d’icônes sont validés. Les vues portées et leur ajustement sont présentés séparément ; l’animation reste un essai optionnel.')
    script='const weightHelmetEntries='+json.dumps(entries)+';\n'+PLAYER_SCRIPT
    return section,script


PLAYER_SCRIPT='''let weightHelmetFrames=[],weightHelmetMotion=true,weightHelmetStarted=performance.now();
function weightHelmetBrightness(t){const phase=((t%3600)+3600)%3600;return phase>=3000&&phase<=3400?1-.38*Math.sin(Math.PI*(phase-3000)/400)**2:1;}
async function prepareWeightHelmetIcons(){try{const rows=await Promise.all(weightHelmetEntries.map(async entry=>{const images=await Promise.all(['native64','visor-mask'].map(async kind=>{const im=new Image();im.src=`helmet-weight-r01/current/${entry.id}-${kind}.png`;await im.decode();return im;}));const work=document.createElement('canvas');work.width=work.height=64;const ctx=work.getContext('2d',{willReadFrequently:true});ctx.drawImage(images[0],0,0);const original=ctx.getImageData(0,0,64,64);ctx.clearRect(0,0,64,64);ctx.drawImage(images[1],0,0);const mask=ctx.getImageData(0,0,64,64).data;const levels=[];for(let level=0;level<(entry.animated?17:1);level++){const pixels=new ImageData(new Uint8ClampedArray(original.data),64,64);const b=1-.38*level/16;for(let p=0;p<pixels.data.length;p+=4)if(mask[p+3])for(let ch=0;ch<3;ch++)pixels.data[p+ch]=Math.round(pixels.data[p+ch]*b);const frame=document.createElement('canvas');frame.width=frame.height=64;frame.getContext('2d').putImageData(pixels,0,0);levels.push(frame);}return {...entry,levels,overview:$(`weight-overview-${entry.number}`),pair:$(`weight-pair-${entry.number}`),last:-1};}));weightHelmetFrames=rows;$('weight-helmet-status').textContent='Les cinq icônes natives sont prêtes.';}catch(error){$('weight-helmet-status').textContent='Icône manquante : '+error.message;}}
$('weight-helmet-ground').onchange=()=>{document.getElementById('nouvelles-icones').dataset.ground=$('weight-helmet-ground').value;};
$('weight-helmet-toggle').onclick=()=>{weightHelmetMotion=!weightHelmetMotion;$('weight-helmet-toggle').textContent=weightHelmetMotion?'Visières : animation activée':'Visières : fixes';};
$('weight-helmet-replay').onclick=()=>{weightHelmetMotion=true;weightHelmetStarted=performance.now()-2900;$('weight-helmet-toggle').textContent='Visières : animation activée';};
function drawWeightHelmetIcons(now){const b=weightHelmetMotion?weightHelmetBrightness(now-weightHelmetStarted):1;const level=Math.round((1-b)/.38*16);for(const row of weightHelmetFrames){const current=row.animated?level:0;if(current===row.last)continue;const ctx=row.pair.getContext('2d');ctx.imageSmoothingEnabled=false;ctx.clearRect(0,0,192,84);ctx.drawImage(row.levels[0],16,8);ctx.drawImage(row.levels[current],112,8);const overview=row.overview.getContext('2d');overview.imageSmoothingEnabled=false;overview.clearRect(0,0,96,84);overview.drawImage(row.levels[current],16,8);row.last=current;}requestAnimationFrame(drawWeightHelmetIcons);}
prepareWeightHelmetIcons();requestAnimationFrame(drawWeightHelmetIcons);'''


if __name__=='__main__':
    build()
    print(json.dumps({'icons':5,'output':str(OUT)}))
