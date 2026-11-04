"""Present the first four lateral redraws in the current native review page."""
from pathlib import Path
import json
import re
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/class-lateral-02/r01/current'

SECTION='''<section id="reprise-laterale"><h2>02 · Quatre nouvelles marches latérales</h2>
<p><strong>Premier lot : armures lourdes, sans arme.</strong> Les nouveaux cycles gardent la tenue de face et reprennent des appuis plus compacts : soldat lourd, châssis mécanique, robe et silhouette souple.</p>
<p class="pending">Nouveaux dessins à examiner : <strong>01 Bastion · 02 Technomancien · 03 Arcaniste · 04 Métamorphe</strong>. Les autres armures et les prises d’armes suivront les démarches retenues.</p>
<div class="controls"><label>Côtés à comparer<select id="lat2-side"><option value="left">Gauche</option><option value="right">Droite</option><option value="both">Les deux côtés</option></select></label><button id="lat2-play">Pause</button><label>Lecture des nouveaux cycles<select id="lat2-mode"><option value="loop">Marche continue</option><option value="stop">Deux pas puis repos</option></select></label><label>Vitesse des nouveaux cycles<select id="lat2-speed"><option value="1">400 ms par pas</option><option value="3">Ralenti ×3</option></select></label></div>
<p class="note">Une ligne par classe. Colonnes : repos actuel · ancienne marche · nouvelle marche. Taille native, cases de 64 px et éclairage Discret.</p>
<div class="native" style="max-width:640px"><canvas id="lat2-board" width="384" height="640" style="display:block;width:384px;height:640px;image-rendering:pixelated" aria-label="Les quatre classes : repos actuel, anciens côtés et nouveaux cycles"></canvas></div>
<div class="timeline"><label>Phase des nouveaux cycles<input id="lat2-phase" type="range" min="0" max="6" value="0"></label><output id="lat2-time">1 / 6</output></div><p id="lat2-status" class="note" role="status">Préparation des captures…</p>
<details id="lat2-zoom-details"><summary>Zoom entier ×2 des nouvelles marches</summary><div class="native"><canvas id="lat2-zoom" width="768" height="1280" style="display:block;width:768px;height:1280px;max-width:none;image-rendering:pixelated"></canvas></div></details>
<p class="note">Le mode « Deux pas puis repos » revient au frontal actuel ; sa pause finale sert à comparer les silhouettes. La Traqueuse conserve son cycle. Ces candidats restent dans l’aperçu avant intégration.</p>
<p class="note">Bipèdes du jeu : 87 créatures déclarées à deux appuis ont été repérées pour une revue par morphologie, en plus des PNJ. Ce repérage ne confirme pas un défaut sur chaque créature.</p>
<p class="note"><a href="class-lateral-02/r01/current/CLASS-LATERAL02-native.webp">Animation native complète</a> · <a href="class-lateral-02/r01/current/checks.json">Contrôles du moteur</a> · <a href="class-lateral-02/biped-inventory.json">Repérage des bipèdes</a></p>
</section>'''

SCRIPT='''let lat2Frames=[],lat2Generation=0,lat2Paused=false,lat2Phase=0,lat2Elapsed=0,lat2Last=0;
function paintLat2(){const image=lat2Frames[lat2Phase];if(!image?.complete)return;const side=$('lat2-side').value,columns=side==='both'?[0,1,2,3,4]:side==='right'?[0,3,4]:[0,1,2],board=$('lat2-board'),width=columns.length*128;if(board.width!==width){board.width=width;board.style.width=width+'px';}const context=board.getContext('2d');context.imageSmoothingEnabled=false;columns.forEach((col,n)=>context.drawImage(image,col*128,0,128,640,n*128,0,128,640));if($('lat2-zoom-details').open){const z=$('lat2-zoom');if(z.width!==width*2){z.width=width*2;z.style.width=width*2+'px';}const zoom=z.getContext('2d');zoom.imageSmoothingEnabled=false;zoom.drawImage(board,0,0,width*2,1280);}$('lat2-phase').value=lat2Phase;$('lat2-time').textContent=lat2Phase===6?'Repos frontal actuel':`${lat2Phase+1} / 6`;}
async function loadLat2(){const generation=++lat2Generation;const images=Array.from({length:7},(_,phase)=>{const im=new Image();im.src=`class-lateral-02/r01/current/lateral-3-${phase}.png?revision=CLASS-LATERAL02-r01`;return im;});try{await Promise.all(images.map(im=>im.decode()));if(generation!==lat2Generation)return;lat2Frames=images;paintLat2();$('lat2-status').textContent='7 / 7 captures prêtes · quatre nouveaux cycles en revue · sans arme.';}catch(error){$('lat2-status').textContent='Capture indisponible : '+error.message;}}
$('lat2-side').onchange=paintLat2;$('lat2-play').onclick=()=>{lat2Paused=!lat2Paused;$('lat2-play').textContent=lat2Paused?'Lire les nouveaux cycles':'Pause';};$('lat2-phase').oninput=()=>{lat2Paused=true;$('lat2-play').textContent='Lire les nouveaux cycles';lat2Phase=+$('lat2-phase').value;lat2Elapsed=lat2Phase*(800/6);paintLat2();};$('lat2-mode').onchange=()=>{lat2Elapsed=0;};$('lat2-zoom-details').ontoggle=paintLat2;
function animateLat2(now){if(lat2Last&&!lat2Paused&&lat2Frames.length){lat2Elapsed+=(now-lat2Last)/(+$('lat2-speed').value);const duration=$('lat2-mode').value==='loop'?800:1320,t=lat2Elapsed%duration,phase=t<800?Math.min(5,Math.floor(t/(800/6))):6;if(phase!==lat2Phase){lat2Phase=phase;paintLat2();}}lat2Last=now;requestAnimationFrame(animateLat2);}loadLat2();requestAnimationFrame(animateLat2);'''


def publish_archived_r01():
    report=json.loads((OUT/'checks.json').read_text(encoding='utf-8-sig'))
    assert len(report['profiles'])==4 and all(p['tier']==3 for p in report['profiles'])
    assert not report['normal_game_enabled'] and report['traqueuse_unchanged']
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',10)
    frames=[]
    for phase in range(7):
        path=OUT/f'lateral-3-{phase}.png';im=Image.open(path).convert('RGB');draw=ImageDraw.Draw(im)
        before=[im.crop((0,row*160+32,640,(row+1)*160)).tobytes() for row in range(4)]
        for row,cls in enumerate(report['classes']):
            top=row*160;draw.rectangle((0,top,639,top+31),fill=(30,40,49))
            draw.text((6,top+1),f'{row+1:02d} · {cls["name"]}',fill='white',font=font)
            draw.text((6,top+17),'Repos actuel',fill='#b6cbd2',font=small)
            for col,(a,b) in enumerate([('Ancien','gauche'),('Nouveau','gauche'),('Ancien','droite'),('Nouveau','droite')],1):
                draw.text((col*128+8,top+1),a,fill='#a5dfba' if a=='Nouveau' else '#c1c6ca',font=font)
                draw.text((col*128+8,top+17),'Repos frontal' if phase==6 else b,fill='#b6cbd2',font=small)
        assert before==[im.crop((0,row*160+32,640,(row+1)*160)).tobytes() for row in range(4)]
        if phase==6:
            for row in range(4):
                refs=[im.crop((col*128,row*160+32,(col+1)*128,(row+1)*160)).tobytes() for col in range(5)]
                assert len(set(refs))==1
        im.save(path);frames.append(im)
    frames[0].save(OUT/'CLASS-LATERAL02-native.webp',save_all=True,append_images=frames[1:],duration=[133,134,133,133,134,133,520],loop=0,lossless=True,method=6)
    page=ROOT/'artifacts/animation-polish-01/apercu.html';html=page.read_text(encoding='utf-8')
    if '<section id="reprise-laterale">' in html:
        html=re.sub(r'<section id="reprise-laterale">.*?</section>',lambda m:SECTION,html,count=1,flags=re.S)
        html=re.sub(r'let lat2Frames=.*?loadLat2\(\);requestAnimationFrame\(animateLat2\);',lambda m:SCRIPT,html,count=1,flags=re.S)
    else:
        html=html.replace('<section id="revue-classes">',SECTION+'\n<section id="revue-classes">',1)
        html=html.replace('</script></html>',SCRIPT+'\n</script></html>')
        html=html.replace('<nav>','<nav><a href="#reprise-laterale">Nouvelles marches</a> · ',1)
    html=re.sub(r'<title>.*?</title>','<title>Project RL · Nouvelles marches · CLASS-LATERAL02</title>',html,count=1)
    html=re.sub(r'<h1>.*?</h1>','<h1>Reprise des marches latérales</h1>',html,count=1)
    html=re.sub(r'(<div class="tag">).*?(</div>)',r'\g<1>CLASS-LATERAL02-r01 · QUATRE NOUVEAUX CYCLES\g<2>',html,count=1)
    html=html.replace('Comparaison du repos et des déplacements : identité, proportions, appuis et équipement.','Bastion, Technomancien, Arcaniste et Métamorphe : premiers candidats à comparer.')
    html=html.replace('Quatre classes, cinq types d’armure et quatre équipements témoins : 80 configurations à examiner ensemble.','Premier lot : quatre nouvelles démarches en armure lourde, sans arme, comparées aux côtés actuels.')
    html=re.sub(r'<div class="pending">.*?</div>','<div class="pending"><strong>Nouveaux cycles à examiner.</strong> Le comparatif du rendu précédent est conservé plus bas.</div>',html,count=1,flags=re.S)
    page.write_text(html,encoding='utf-8')
    inventory=ROOT/'docs/reviews/animation-polish-01/BIPED-MOTION01-INVENTORY.json'
    (OUT.parent.parent/'biped-inventory.json').write_text(inventory.read_text(encoding='utf-8'),encoding='utf-8')
    print('Published four complete class cycles, native engine comparison and biped inventory')


def main():
    from build_class_lateral_r02_review import publish
    ready=(ROOT/'artifacts/animation-polish-01/class-lateral-02/r02/current/checks.json').exists()
    publish(pending=not ready)


if __name__=='__main__':main()
