"""Publish native FIREARM01 captures without changing actor/scenery pixels."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/firearms-r01/current'

def build():
    if not (OUT/'checks.json').exists():
        return '', ''
    checks=json.loads((OUT/'checks.json').read_text())
    files=list(OUT.glob('firearm-*.png'))
    assert len(files)==checks['boards']==1120
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    for path in files:
        image=Image.open(path).convert('RGB')
        before=image.crop((0,32,448,256)).tobytes()
        draw=ImageDraw.Draw(image)
        draw.rectangle((0,0,447,31),fill=(30,40,49))
        for col,label in enumerate(['En jeu · actuel','01 · Compact et lisible']):
            draw.text((8+col*224,8),label,font=font,fill=(232,240,242))
        assert before==image.crop((0,32,448,256)).tobytes()
        image.save(path)
    section='''<section id="armes-tir"><h2>Armes de tir à deux mains : taille et lisibilité</h2>
<p><strong>01 / Compact et lisible</strong> raccourcit et amincit davantage les armes lourdes. Les fusils gardent un canon et une crosse reconnaissables, avec des arêtes métalliques plus nettes et une ombre de séparation sur l’armure. Les prises des mains restent en place ; les parties portées devant le torse restent masquées de dos.</p>
<p class="pending">Nouveau candidat en revue avant intégration. À gauche : le rendu actuellement en jeu. L’éclairage « Discret » est conservé des deux côtés.</p>
<div class="controls"><label>Personnage<select id="firearm-body"><option value="3">Bastion</option><option value="5">Traqueuse</option><option value="6">Arcaniste</option><option value="7">Métamorphe</option></select></label>
<label>Arme<select id="firearm-weapon"><option value="2">Fusil d’assaut</option><option value="7">Neral E12</option><option value="35">Fusil de guetteur</option><option value="57">Fusil à pompe</option><option value="17">Lance-flammes</option><option value="23">Lance-grenades</option><option value="31">Mitrailleuse lourde</option><option value="51">Lance-roquettes</option></select></label>
<label>Armure<select id="firearm-tier"><option value="3">Lourde</option><option value="0">Tenue de base</option><option value="1">Légère</option><option value="2">Intermédiaire</option><option value="4">Très lourde</option></select></label>
<label>Vue<select id="firearm-view"><option value="front">Face</option><option value="back">Dos</option><option value="left">Gauche</option><option value="right">Droite</option></select></label>
<button id="firearm-play">Lire la marche</button><label>Vitesse<select id="firearm-speed"><option value="1">Normale</option><option value="3">Ralenti ×3</option></select></label></div>
<p class="note">Les huit armes sont montrées sur les quatre classes en armure lourde. Les cinq degrés d’armure sont aussi comparés sur la Traqueuse, avec le fusil d’assaut et la mitrailleuse.</p>
<div class="native" style="max-width:448px"><canvas id="firearm-board" width="448" height="256" style="display:block;image-rendering:pixelated;max-width:none" aria-label="Arme actuelle et candidate 01 dans le même décor à taille native"></canvas></div>
<div class="timeline"><label>Phase<input id="firearm-phase" type="range" min="0" max="6" value="6"></label><output id="firearm-time">Repos frontal</output></div>
<p class="note" id="firearm-status" role="status"></p>
<details><summary>Zoom entier ×2</summary><div class="native"><canvas id="firearm-zoom" width="896" height="512" style="display:block;image-rendering:pixelated;max-width:none"></canvas></div></details>
<p class="note">Figures natives sur des cases de 64 px. Face, dos, deux côtés, six phases de marche et vrai repos frontal. Le décor, la lumière et la position sont identiques dans chaque paire ; aucun filtre de luminosité n’est ajouté dans le navigateur.</p></section>'''
    script='''let firearmFrames=[],firearmGeneration=0,firearmPaused=true,firearmPhase=6,firearmStart=performance.now();
function paintFirearm(){const image=firearmFrames[firearmPhase];if(!image?.complete)return;for(const [id,scale] of [['firearm-board',1],['firearm-zoom',2]]){const canvas=$(id),context=canvas.getContext('2d');context.imageSmoothingEnabled=false;context.drawImage(image,0,0,448*scale,256*scale);}$('firearm-phase').value=firearmPhase;$('firearm-time').textContent=firearmPhase===6?'Repos frontal':`${firearmPhase+1} / 6`;}
async function loadFirearm(){const extra=$('firearm-body').value==='5'&&['2','31'].includes($('firearm-weapon').value);for(const option of $('firearm-tier').options)option.disabled=option.value!=='3'&&!extra;if(!extra)$('firearm-tier').value='3';const generation=++firearmGeneration;$('firearm-status').textContent='Préparation du cycle…';const images=Array.from({length:7},(_,n)=>{const image=new Image();image.src=`firearms-r01/current/firearm-${$('firearm-body').value}-${$('firearm-tier').value}-${$('firearm-weapon').value}-${$('firearm-view').value}-${n}.png`;return image;});try{await Promise.all(images.map(image=>image.decode()));if(generation!==firearmGeneration)return;firearmFrames=images;firearmStart=performance.now();paintFirearm();$('firearm-status').textContent='Même position, même éclairage Discret · candidat 01 en revue.';}catch(error){$('firearm-status').textContent='Capture manquante : '+error.message;}}
for(const id of ['firearm-body','firearm-weapon','firearm-tier','firearm-view'])$(id).onchange=loadFirearm;
$('firearm-play').onclick=()=>{firearmPaused=!firearmPaused;$('firearm-play').textContent=firearmPaused?'Lire la marche':'Pause';firearmStart=performance.now();};$('firearm-phase').oninput=()=>{firearmPaused=true;$('firearm-play').textContent='Lire la marche';firearmPhase=+$('firearm-phase').value;paintFirearm();};
function animateFirearm(now){if(!firearmPaused){const phase=Math.floor(((now-firearmStart)/(+$('firearm-speed').value))%800/(800/6));if(phase!==firearmPhase){firearmPhase=phase;paintFirearm();}}requestAnimationFrame(animateFirearm);}loadFirearm();requestAnimationFrame(animateFirearm);'''
    return section,script
