"""Verify accepted native pixels and expose ordinary-compositor captures."""
import json
import re
from pathlib import Path
from PIL import Image, ImageDraw
from prepare_class_readability_review_01 import ROOT, sha, write_json, font

OUT = ROOT/'artifacts/class-armor-integration-01'
DOC = ROOT/'docs/reviews/class-armor-integration-01'
CLASSES = ['Bastion','Technomancien','Traqueuse','Arcaniste','Métamorphe']


def main():
    if (DOC/'melee-grip-04/checks.json').exists():
        # Current revision owns its regression baseline and review section.
        # Older accepted/corrective review reports stay historical evidence.
        from show_class_melee_grip_04 import main as current_review
        return current_review()
    manifest = json.loads((ROOT/'assets/sprites/class-armors-01/manifest.json').read_text('utf-8'))
    sheet = Image.open(ROOT/manifest['atlases']['base']['path']).convert('RGBA')
    for approval in manifest['source_acceptances']:
        assert sha(ROOT/approval['path']) == approval['sha256']
        accepted=json.loads((ROOT/approval['path']).read_text('utf-8'))
        for e in accepted['entries']:
            for reference in [e['source'], e['native'], *e['components'].values()]:
                assert sha(ROOT/reference['path'])==reference['sha256']
    for e in manifest['outfits']:
        n=e['index']; reference=e.get('runtime_native',e['native']);src=ROOT/reference['path']
        assert sha(src)==reference['sha256']
        assert sheet.crop((n%16*128+32,n//16*128+32,n%16*128+96,n//16*128+96)).tobytes()==Image.open(src).convert('RGBA').tobytes()
    capture=OUT/'compositor/capture.json'
    data=json.loads(capture.read_text('utf-8'))
    if data['exported_pngs_need_vertical_flip']:
        for p in (OUT/'compositor').glob('outfit-*.png'):
            im=Image.open(p).transpose(Image.Transpose.FLIP_TOP_BOTTOM)
            im.save(p,optimize=True)
        data['exported_pngs_need_vertical_flip']=False
        write_json(capture,data)
    # Identifiers are enough to distinguish every model; display existing
    # localized names when a content definition supplies one.
    flat=json.loads((ROOT/'assets/sprites/flat-agents/manifest.json').read_text('utf-8'))
    weapons=flat['weapons']
    by_model={}
    locale={}
    for p in sorted((ROOT/'content/core/locales').glob('fr*.json5')):
        locale.update(re.findall(r'"([^"\n]+)"\s*:\s*"([^"\n]*)"',p.read_text('utf-8')))
    for w in weapons:
        model=w['model']; identity=w.get('content_id') or w.get('id') or ''
        if model is None: continue
        if model in by_model: continue
        label=identity
        p=ROOT/'content/core/weapons'/f"{identity.removeprefix('core:')}.json5"
        if p.exists():
            match=re.search(r'"?name_key"?\s*:\s*"([^"]+)"',p.read_text('utf-8'))
            if match: label=locale.get(match[1],identity)
        by_model[model]=label or f'Modèle {model:03}'
    labels=[]
    for label in data['configurations']:
        if label.startswith('weapon-'):
            model=int(label[7:]); label=f'{model:03} — {by_model.get(model,"Arme")}'
        else:
            label={'unarmed':'Sans arme','dual-pistols':'Deux pistolets','dual-melee-ranged':'Mêlée + pistolet','dual-melee':'Deux armes de mêlée','breath-rifle':'Fusil · respiration'}.get(label,label.replace('helmet-','Casque '))
        labels.append(label)
    sample=[0,102,39,62,98,114,118]
    overview=Image.new('RGB',(1040,800),(18,25,30)); draw=ImageDraw.Draw(overview)
    for c,name in enumerate(CLASSES):
        draw.text((10,62+c*145),name,font=font(15),fill='white')
        for j,cell in enumerate(sample):
            n=c*5+4; atlas=Image.open(OUT/f'compositor/outfit-{n:02}.png').convert('RGBA')
            im=atlas.crop((cell%16*128,cell//16*128,cell%16*128+128,cell//16*128+128))
            overview.paste(im,(130+j*128,40+c*145),im)
    for j,label in enumerate(['Sans arme','Pistolet','Fusil','Hache 2M','Marteau 1M','Deux armes','Casque ouvert']):
        draw.text((140+j*128,12),label,font=font(14),fill='white')
    overview.save(OUT/'integrated-native.png')
    revision=DOC/'pose-fixes-03'
    prior=json.loads((revision/'before-cell-hashes.json').read_text('utf-8')) if (revision/'before-cell-hashes.json').exists() else {}
    preserved=0; changed=[]; faces_checked=0
    import hashlib
    for n in range(25):
        current=Image.open(OUT/f'compositor/outfit-{n:02}.png').convert('RGBA')
        plain=current.crop((0,0,128,128))
        centre=manifest['outfits'][n]['head_center']
        face=(int(centre[0])-4,int(centre[1])-4,int(centre[0])+5,int(centre[1])+5)
        for cell,label in enumerate(data['configurations']):
            pistol=(label.startswith('weapon-') and 101<=int(label[7:])<=112) or label in ['dual-pistols','dual-melee-ranged']
            tile=current.crop((cell%16*128,cell//16*128,cell%16*128+128,cell//16*128+128))
            if pistol:
                assert tile.crop(face).tobytes()==plain.crop(face).tobytes(),f'Pistol covers central face: {n}, {label}'
                faces_checked+=1
                continue
            if not prior: continue
            if hashlib.sha256(tile.tobytes()).hexdigest()==prior[f'{n:02}/{label}']:preserved+=1
            else:changed.append(f'{n:02}/{label}')
    if prior:
        assert all(case.startswith('20/') for case in changed),changed
        write_json(revision/'unchanged-compositions.json',{'non_pistol_compositions_exactly_preserved':preserved,'requested_metamorphe_posture_changes':changed,'unexpected_changes':[],'central_face_regions_exactly_preserved_with_pistols':faces_checked,'face_region':'9x9 pixels around each rig head centre'})
        comparison=Image.new('RGBA',(640,690),(18,25,30,255));d=ImageDraw.Draw(comparison)
        for j,label in enumerate(['Avant','Un pistolet','Deux pistolets','Mêlée + pistolet']):
            d.text((116+j*128,12),label,font=font(12),fill='white')
        for c,name in enumerate(CLASSES):
            n=c*5+4;d.text((5,83+c*128),name,font=font(13),fill='white')
            atlas=Image.open(OUT/f'compositor/outfit-{n:02}.png').convert('RGBA')
            images=[Image.open(revision/f'before-{n:02}-weapon-101.png').convert('RGBA')]
            for cell in [102,114,115]:images.append(atlas.crop((cell%16*128,cell//16*128,cell%16*128+128,cell//16*128+128)))
            for j,im in enumerate(images):comparison.alpha_composite(im,(110+j*128,38+c*128))
        comparison.save(OUT/'pistol-raised-native.png');comparison.save(revision/'comparison-native.png')
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Armures intégrées · 64 px</title>
<style>body{background:#11191e;color:#e4e8e9;font:16px system-ui;margin:28px auto;max-width:1100px}h1{font-size:25px}p{color:#bac8ce;max-width:900px}select,button{background:#283943;color:white;border:1px solid #71858e;padding:9px;margin:5px;font:inherit}canvas{image-rendering:pixelated;background:#343b3f;display:block}.row{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}.card{text-align:center;background:#1e2a31;padding:10px}small{display:block;color:#bac8ce;margin:8px}.media{max-width:100%;overflow-x:auto;margin-bottom:16px}img{max-width:none;display:block}a{color:#c9dfe8}select{max-width:95%}</style>
<h1 id="top">Les 25 armures intégrées</h1><p>Chaque personnage conserve sa silhouette. Les armes sont communes aux cinq classes. La case du personnage mesure toujours 64 × 64 px ; la toile de 128 px garde les débordements de l’arme.</p>
<label>Classe <select id="classes"></select></label><label>Configuration <select id="weapons"></select></label><button id="zoom">Agrandissement ×2</button>
<div class="row" id="cards"></div><p id="status"></p><p><a href="#pistols">Pistolets · bras repliés</a> · <a href="#hammers">Les trois marteaux validés</a></p>
<h2 id="pistols">03 · Bras du Bastion et manches de l’Arcaniste corrigés</h2><p>Avant / après, un ou deux pistolets, puis mêlée + pistolet. Dessins à taille native.</p><div class="media"><img src="pistol-raised-native.png" alt="Comparaison avant et après de la correction des bras"></div>
<div class="media"><img src="pose-fixes-03/all-tiers-native.png" alt="Les cinq tenues du Bastion et de l’Arcaniste avec un et deux pistolets, en 64 px"></div>
<h2 id="metamorphe-posture">Métamorphe 01 · posture harmonisée</h2><p>Même buste, même visage et mêmes prises d’armes ; nouveaux appuis pour la première tenue. Casque ouvert et fusil comme sur ta capture, en taille native puis au zoom ×2.</p><div class="media"><img src="pose-fixes-03/metamorphe-rifle-native.png" alt="Ancienne posture de la Métamorphe 01, correction puis tenues 02 à 05 avec casque ouvert et fusil"></div>
<h2>Comparaison à taille réelle · armures lourdes</h2><p>Défilement horizontal si nécessaire : les planches gardent leur taille native.</p><div class="media"><img src="integrated-native.png" alt="Cinq classes avec pistolet, fusil, hache, marteau, deux armes et casque ouvert"></div>
<h2 id="hammers">Marteaux à deux mains · dessins validés</h2><p>01, 02 et 03 : dessins validés. Les marteaux existants restent à une main.</p><div class="media"><img src="hammers/proposals-native.png" alt="Trois marteaux horizontaux validés, taille native et agrandissement"></div>
<h2>Captures du moteur</h2><p>Rendu ordinaire des personnages, objets réellement équipés.</p><div id="gpu"></div>
<script>
const names=CLASSES_JSON,labels=LABELS_JSON;let zoom=1;const cs=document.querySelector('#classes'),ws=document.querySelector('#weapons');names.forEach((n,i)=>cs.add(new Option(n,i)));labels.forEach((n,i)=>ws.add(new Option(n,i)));ws.value=102;
const images=Array.from({length:25},(_,i)=>{let im=new Image();im.src=`compositor/outfit-${String(i).padStart(2,'0')}.png`;im.onload=render;return im});
function render(){const c=Number(cs.value),w=Number(ws.value),host=document.querySelector('#cards');host.replaceChildren();for(let r=0;r<5;r++){const card=document.createElement('div');card.className='card';const canvas=document.createElement('canvas');canvas.width=128;canvas.height=128;canvas.style.width=`${128*zoom}px`;canvas.style.height=`${128*zoom}px`;const ctx=canvas.getContext('2d');ctx.imageSmoothingEnabled=false;if(images[c*5+r].complete)ctx.drawImage(images[c*5+r],w%16*128,Math.floor(w/16)*128,128,128,0,0,128,128);const text=document.createElement('small');text.textContent=`0${r+1} · ${['Très légère','Légère','Intermédiaire','Lourde','Très lourde'][r]}`;card.append(canvas,text);host.append(card)}document.querySelector('#status').textContent=`${names[c]} · ${labels[w]} · affichage ${zoom===1?'natif':'×2'}`}
cs.onchange=ws.onchange=render;document.querySelector('#zoom').onclick=()=>{zoom=zoom===1?2:1;render()};['pistol','left-pistol','dual-pistols','pistol-smg','dual','mixed-left-pistol','two-handed-axe','hammer-one-hand','open-helmet'].forEach(n=>{let box=document.createElement('div');box.className='media';let im=document.createElement('img');im.src=`gpu/native-${n}.png`;im.alt=n;box.append(im);document.querySelector('#gpu').append(box)});render();</script>'''
    (OUT/'index.html').write_text(html.replace('CLASSES_JSON',json.dumps(CLASSES,ensure_ascii=False)).replace('LABELS_JSON',json.dumps(labels,ensure_ascii=False)),encoding='utf-8')
    write_json(DOC/'source-checks.json',{'review_id':'CLASS-ARMOR-INTEGRATION01','source_acceptance_hashes_verified':5,'native_source_hashes_verified':25,'production_base_atlas_exact_original_native_pixels':24,'requested_posture_override':20,'cpu_configurations_per_outfit':len(data['configurations']),'cpu_compositions':25*len(data['configurations']),'source_assets':manifest['source_acceptances']})
    print(json.dumps({'original_bases_unchanged':24,'requested_posture_corrections':1,'cpu_compositions':25*len(labels),'review':str(OUT/'index.html')}))


if __name__=='__main__': main()
