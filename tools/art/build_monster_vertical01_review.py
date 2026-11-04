"""Publish isolated native up/down proposals and the full current roster."""
import argparse
import html
import json
import re
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from build_biped_lateral01_review import SCRIPT

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/monster-vertical-01/r01/current'
PAGE=ROOT/'artifacts/animation-polish-01/apercu.html'
PREFIX='monster-vertical-01/r01/current/'
REVIEW='MONSTER-VERTICAL01-r01'


def main(page_only=False,labels=None,previous=None,intro=None,webp_method=4,workers=1):
    data=json.loads((OUT/'checks.json').read_text(encoding='utf-8-sig'))
    assert data['models']==129 and data['variants']==471 and not data['normal_game_enabled']
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',10)
    labels=labels or ['01 · Cultiste sans arme','02 · Cultiste au fusil','03 · Démon ailé au sol']
    if not page_only:
        def export_item(job):
            item,label=job
            panel_font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
            caption_font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',10)
            for file in item['captures']:
                path=OUT/item['id']/file; im=Image.open(path).convert('RGB'); assert im.size==(576,320)
                actor=im.crop((0,32,576,320)).tobytes(); d=ImageDraw.Draw(im); d.rectangle((0,0,575,31),fill='#1e2831')
                for col,title in enumerate(item.get('headers',['Frontal conservé','Marche actuelle','Nouvelle marche'])):d.text((col*192+5,3),title,font=panel_font,fill='#a5dfba' if col==2 else '#d2dde1')
                d.text((5,17),label+' · '+('bas / face' if file.startswith('down') else 'haut / dos'),font=caption_font,fill='#b6cbd2')
                assert actor==im.crop((0,32,576,320)).tobytes(); im.save(path)
            frames=[Image.open(OUT/item['id']/file).convert('RGB') for file in item['captures']]
            frames[0].save(OUT/item['id']/'native.webp',save_all=True,append_images=frames[1:],duration=([25]*32+[400])*2,loop=0,lossless=True,method=webp_method)
        with ThreadPoolExecutor(max_workers=workers) as pool:
            for _ in pool.map(export_item,zip(data['items'],labels)):
                pass
        for item in data['inventory']:
            path=OUT/item['file']; im=Image.open(path).convert('RGB'); assert im.size==(384,192)
            actor=im.crop((0,32,384,192)).tobytes(); d=ImageDraw.Draw(im); d.rectangle((0,0,383,31),fill='#1e2831')
            d.text((5,3),f'{item["id"]} · face actuelle',font=font,fill='#d2dde1'); d.text((197,3),'Dos actuel',font=font,fill='#d2dde1')
            d.text((5,17),f'{len(item["variants"])} variante(s) existante(s)',font=small,fill='#b6cbd2')
            assert actor==im.crop((0,32,384,192)).tobytes(); im.save(path)
        proof=Image.new('RGB',(576,320*len(data['items'])),'#1e2831')
        for i,item in enumerate(data['items']):
            im=Image.open(OUT/item['id']/('up-25.png' if i==2 else 'down-09.png')).convert('RGB')
            proof.paste(im,(0,i*320))
        proof.save(OUT.parent/'native-proof.png')
    items=[{**item,'capture_base':PREFIX} for item in data['items']]
    if previous:
        items.extend(previous['items']); labels=labels+previous['labels']
    options=''.join(f'<option value="{item["id"]}">{label}</option>' for item,label in zip(items,labels))
    gallery=''.join(f'<figure style="margin:16px 0"><img src="{PREFIX}{item["file"]}" loading="lazy" width="384" height="192" alt="{item["id"]}, face et dos actuels" style="max-width:none;image-rendering:pixelated"><figcaption>{item["id"]} · classement à vérifier : {html.escape(str(item["declared_rig"]))} · {len(item["variants"])} variante(s)</figcaption></figure>' for item in data['inventory'])
    section=f'''<section id="monstres-vertical"><h2>01 · Monstres : reprise des déplacements haut / bas</h2>
<p>Le chantier couvre les <strong>129 modèles du bestiaire et leurs 471 variantes</strong>. Voici les trois premiers essais : cultiste sans arme, cultiste au fusil et démon ailé au sol. Les autres modèles restent à reprendre.</p>
<p class="pending">MONSTER-VERTICAL01-r01 · Nouveaux dessins en revue. <strong>400 ms par pas</strong>, deux appuis alternés sur 800 ms, puis le frontal conservé à l’arrêt. Les animations latérales validées restent inchangées.</p>
<div class="controls"><label>Monstre<select id="mvertical-item">{options}</select></label><label>Direction<select id="mvertical-direction"><option value="both">Bas puis haut</option><option value="down">Vers le bas · face</option><option value="up">Vers le haut · dos</option></select></label><button id="mvertical-play">Pause</button><button id="mvertical-next">Pose suivante</button><label>Cadence<select id="mvertical-pace"><option value="400">400 ms par pas</option><option value="650">650 ms · ralenti</option><option value="800">800 ms · ralenti</option></select></label></div>
<label>Comparaison<select id="mvertical-pair"><option value="walk">Marche actuelle / nouvelle marche</option><option value="idle">Frontal conservé / nouvelle marche</option><option value="all">Les trois colonnes</option></select></label>
<p id="mvertical-columns" class="note">Marche actuelle · nouvelle marche. Taille native, cases de 64 px ; mêmes décor, trajet et éclairage.</p>
<div class="native" style="max-width:576px"><canvas id="mvertical-board" width="384" height="320" style="display:block;width:384px;height:320px;max-width:none;image-rendering:pixelated" aria-label="Comparatif natif des marches haut et bas"></canvas></div>
<div class="controls"><label>Phase<input id="mvertical-phase" type="range" min="0" max="8" value="0"></label><output id="mvertical-time">Contact A</output></div>
<p id="mvertical-status" class="note" role="status">Préparation des captures…</p><p id="mvertical-scope" class="note"></p>
<details id="mvertical-zoom-details"><summary>Zoom entier ×2</summary><div class="native"><canvas id="mvertical-zoom" width="768" height="640" style="display:block;width:768px;height:640px;max-width:none;image-rendering:pixelated"></canvas></div></details>
<p class="note">Vérifier les vrais appuis, le raccord au repos, les mains sur le fusil et les attaches des ailes. Chaque nouvelle figure est dessinée entière. Les autres morphologies suivront leur locomotion propre.</p>
<p><a href="{PREFIX}checks.json">Bilan natif</a> · <a id="mvertical-animation" href="{PREFIX}MON-014-0/native.webp">Animation native sélectionnée</a></p>
<details id="mvertical-inventory"><summary>Inventaire complet · 129 modèles / 471 variantes</summary><p>Face et dos actuels à taille native. Ces cartes recensent le bestiaire ; elles ne sont pas de nouveaux dessins. Le classement anatomique reste à contrôler visuellement.</p><div class="native">{gallery}</div></details>
</section>'''
    section=section.replace('MONSTER-VERTICAL01-r01',REVIEW)
    if intro:
        section=section.replace('Voici les trois premiers essais : cultiste sans arme, cultiste au fusil et démon ailé au sol. Les autres modèles restent à reprendre.',intro)
        section=section.replace('Vérifier les vrais appuis, le raccord au repos, les mains sur le fusil et les attaches des ailes.','Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.')
    section=section.replace(PREFIX+'MON-014-0/native.webp',PREFIX+data['items'][0]['id']+'/native.webp')
    script=SCRIPT.replace('biped','mvertical').replace('Biped','Mvertical')
    script=script.replace('__ITEMS__',json.dumps(items,ensure_ascii=False))
    script=script.replace("'left'","'down'").replace("'right'","'up'").replace('Gauche · ','Bas · face · ').replace('Droite · ','Haut · dos · ')
    script=script.replace('biped-lateral-01/r01/current/',PREFIX).replace('mvertical-lateral-01/r01/current/',PREFIX).replace('BIPED-LATERAL01-r01',REVIEW)
    script=script.replace("image.src='"+PREFIX+"'+id", "image.src=(item.capture_base||'"+PREFIX+"')+id")
    script=script.replace(".href='"+PREFIX+"'+id", ".href=(item.capture_base||'"+PREFIX+"')+id")
    blit="function blitMvertical(canvas,image,scale){const pair=$('mvertical-pair').value,columns=pair==='all'?[0,1,2]:pair==='idle'?[0,2]:[1,2],width=columns.length*192*scale,height=320*scale;if(canvas.width!==width||canvas.height!==height){canvas.width=width;canvas.height=height;canvas.style.width=width+'px';canvas.style.height=height+'px';}const ctx=canvas.getContext('2d');ctx.imageSmoothingEnabled=false;columns.forEach((source,col)=>ctx.drawImage(image,source*192,0,192,320,col*192*scale,0,192*scale,320*scale));}"
    script=re.sub(r'function blitMvertical\(.*?\n',lambda _:blit+'\n',script,count=1)
    script=script.replace("item.player?'05 · Étude des prises : Bastion, tenue lourde, fusil d’assaut partagé, palette d’origine.':'Créature '+item.id+' · palette d’origine · huit poses adaptées à sa morphologie.'","item.name+' · palette d’origine · huit poses de face et huit de dos.'")
    script=script.replace('Reprendre les bipèdes','Reprendre').replace('Repos actuel','Frontal conservé').replace('nouveau cycle','nouvelle marche')
    script=script.replace('mverticalNames[s.phase]',"(mverticalItems.find(i=>i.id===$('mvertical-item').value).phase_names||mverticalNames)[s.phase]")
    script=script.replace("item.name+' · palette d’origine · huit poses de face et huit de dos.'","item.scope_note||item.name+' · palette d’origine · huit poses de face et huit de dos.'")
    page=PAGE.read_text(encoding='utf-8')
    if '<section id="monstres-vertical">' in page:page=re.sub(r'<section id="monstres-vertical">.*?</section>',lambda _:section,page,count=1,flags=re.S)
    else:page=page.replace('<section id="bipedes">',section+'\n<section id="bipedes">',1)
    if '<!-- MONSTER-VERTICAL01 SCRIPT -->' in page:page=re.sub(r'<!-- MONSTER-VERTICAL01 SCRIPT -->.*?<!-- /MONSTER-VERTICAL01 SCRIPT -->',lambda _:'<!-- MONSTER-VERTICAL01 SCRIPT --><script>'+script+'</script><!-- /MONSTER-VERTICAL01 SCRIPT -->',page,count=1,flags=re.S)
    else:page=page.replace('</html>','<!-- MONSTER-VERTICAL01 SCRIPT --><script>'+script+'</script><!-- /MONSTER-VERTICAL01 SCRIPT -->\n</html>')
    if '<a href="#monstres-vertical">' not in page:page=page.replace('<a href="#bipedes">','<a href="#monstres-vertical">Monstres haut / bas</a> · <a href="#bipedes">',1)
    page=re.sub(r'<title>.*?</title>',f'<title>Project RL · {REVIEW} · Monstres haut / bas</title>',page,count=1)
    page=re.sub(r'<h1>.*?</h1><p class="note">.*?</p>','<h1>Reprise des marches haut / bas des monstres</h1><p class="note">Tout le bestiaire est concerné. Premier lot de trois modèles, comparé dans le décor du jeu à taille native.</p>',page,count=1,flags=re.S)
    page=page.replace('<p>Sept modèles validés. Trois nouveaux ailés à examiner à taille native, dans les deux sens.</p>','<p>Dix modèles de marche latérale sont déjà intégrés. Les nouveaux cycles haut / bas sont en revue séparée.</p>')
    page=page.replace('Premier lot de trois modèles, comparé dans le décor du jeu à taille native.','Nouveaux modèles et premier lot de référence, comparés dans le décor du jeu à taille native.' if previous else 'Premier lot de trois modèles, comparé dans le décor du jeu à taille native.')
    page=re.sub(r'(<div class="tag">).*?(</div>)',lambda m:m[1]+REVIEW+' · MARCHES HAUT / BAS'+m[2],page,count=1)
    assert '<section id="monstres-vertical">' in page and 'let mverticalCache=' in page and 'Technomancien' not in page
    PAGE.write_text(page,encoding='utf-8')
    print(f'Published {len(data["items"])} vertical studies and 129 current native inventory cards')


if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--page-only',action='store_true'); args=parser.parse_args(); main(args.page_only)
