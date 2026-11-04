"""Native comparison of the previous rifle candidate and a slight enlargement."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/rifles-r01/current'

def build():
    if not (OUT/'checks.json').exists():
        return '', ''
    checks=json.loads((OUT/'checks.json').read_text())
    files=list(OUT.glob('rifle-*.png'))
    assert len(files)==checks['boards']==896
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    for file in files:
        image=Image.open(file).convert('RGB')
        before=image.crop((0,32,448,256)).tobytes()
        draw=ImageDraw.Draw(image)
        draw.rectangle((0,0,447,31),fill=(30,40,49))
        for col,label in enumerate(['01 · Essai précédent','02 · Légèrement agrandi']):
            draw.text((8+col*224,8),label,font=font,fill=(232,240,242))
        assert before==image.crop((0,32,448,256)).tobytes()
        image.save(file)
    section='''<section id="fusils"><h2>Fusils : très léger agrandissement</h2>
<p><strong>02 / Légèrement agrandi</strong> ajoute un peu de longueur et d’épaisseur au fusil de l’essai précédent. Le canon et la crosse se détachent mieux ; les prises, les poses et l’éclairage Discret sont conservés.</p>
<p class="pending">À gauche : le précédent candidat 01, jugé trop petit. À droite : l’ajustement 02, à examiner avant intégration. Cette comparaison concerne les fusils d’assaut, à pompe et similaires ; les armes lourdes gardent leur revue séparée plus bas.</p>
<div class="controls"><label>Personnage<select id="rifle-body"><option value="3">Bastion</option><option value="5">Traqueuse</option><option value="6">Arcaniste</option><option value="7">Métamorphe</option></select></label>
<label>Fusil<select id="rifle-weapon"><option value="2">Fusil d’assaut</option><option value="57">Fusil à pompe</option><option value="7">Neral E12</option><option value="35">Fusil de guetteur</option></select></label>
<label>Armure<select id="rifle-tier"><option value="3">Lourde</option><option value="1">Légère</option><option value="4">Très lourde</option></select></label>
<label>Vue<select id="rifle-view"><option value="front">Face</option><option value="back">Dos</option><option value="left">Gauche</option><option value="right">Droite</option></select></label>
<button id="rifle-play">Lire la marche</button><label>Vitesse<select id="rifle-speed"><option value="1">Normale</option><option value="3">Ralenti ×3</option></select></label></div>
<div class="native" style="max-width:448px"><canvas id="rifle-board" width="448" height="256" style="display:block;image-rendering:pixelated;max-width:none" aria-label="Fusil précédent et candidat 02 dans le même décor à taille native"></canvas></div>
<div class="timeline"><label>Phase<input id="rifle-phase" type="range" min="0" max="6" value="6"></label><output id="rifle-time">Repos frontal</output></div>
<p id="rifle-status" class="note" role="status"></p>
<details><summary>Zoom entier ×2 des fusils</summary><div class="native"><canvas id="rifle-zoom" width="896" height="512" style="display:block;image-rendering:pixelated;max-width:none"></canvas></div></details>
<p class="note">Quatre classes, quatre fusils en armure lourde. Le fusil d’assaut et le fusil à pompe sont aussi montrés avec les armures légère et très lourde sur chaque classe. Cases natives de 64 px, quatre orientations, six phases de marche et vrai repos frontal.</p></section>'''
    script='''let rifleFrames=[],rifleGeneration=0,riflePaused=true,riflePhase=6,rifleStart=performance.now();
function paintRifle(){const image=rifleFrames[riflePhase];if(!image?.complete)return;for(const [id,scale] of [['rifle-board',1],['rifle-zoom',2]]){const context=$(id).getContext('2d');context.imageSmoothingEnabled=false;context.drawImage(image,0,0,448*scale,256*scale);}$('rifle-phase').value=riflePhase;$('rifle-time').textContent=riflePhase===6?'Repos frontal':`${riflePhase+1} / 6`;}
async function loadRifle(){const extra=['2','57'].includes($('rifle-weapon').value);for(const option of $('rifle-tier').options)option.disabled=option.value!=='3'&&!extra;if(!extra)$('rifle-tier').value='3';const generation=++rifleGeneration;$('rifle-status').textContent='Préparation du cycle…';const images=Array.from({length:7},(_,n)=>{const image=new Image();image.src=`rifles-r01/current/rifle-${$('rifle-body').value}-${$('rifle-tier').value}-${$('rifle-weapon').value}-${$('rifle-view').value}-${n}.png`;return image;});try{await Promise.all(images.map(image=>image.decode()));if(generation!==rifleGeneration)return;rifleFrames=images;rifleStart=performance.now();paintRifle();$('rifle-status').textContent='01 précédent → 02 légèrement agrandi · même position et même lumière.';}catch(error){$('rifle-status').textContent='Capture manquante : '+error.message;}}
for(const id of ['rifle-body','rifle-weapon','rifle-tier','rifle-view'])$(id).onchange=loadRifle;
$('rifle-play').onclick=()=>{riflePaused=!riflePaused;$('rifle-play').textContent=riflePaused?'Lire la marche':'Pause';rifleStart=performance.now();};$('rifle-phase').oninput=()=>{riflePaused=true;$('rifle-play').textContent='Lire la marche';riflePhase=+$('rifle-phase').value;paintRifle();};
function animateRifle(now){if(!riflePaused){const phase=Math.floor(((now-rifleStart)/(+$('rifle-speed').value))%800/(800/6));if(phase!==riflePhase){riflePhase=phase;paintRifle();}}requestAnimationFrame(animateRifle);}loadRifle();requestAnimationFrame(animateRifle);'''
    integration=ROOT/'docs/reviews/animation-polish-01/FIREARM02-INTEGRATION.json'
    if integration.exists() and json.loads(integration.read_text(encoding='utf-8'))['normal_game_enabled']:
        section=section.replace('Fusils : très léger agrandissement','Fusils : version 02 intégrée')
        section=section.replace('À gauche : le précédent candidat 01, jugé trop petit. À droite : l’ajustement 02, à examiner avant intégration.','À gauche : le précédent essai 01. À droite : la version 02, validée et active en jeu, au repos et en marche.')
        script=script.replace('01 précédent → 02 légèrement agrandi · même position et même lumière.','01 précédent → 02 intégré en jeu · même position et même lumière.')
    return section,script
