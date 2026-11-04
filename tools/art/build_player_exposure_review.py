"""Publish native LIGHT01 captures; only the empty label strip is annotated."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/player-light-r01/current'


def build():
    if not (OUT/'checks.json').exists():
        return '', ''
    checks=json.loads((OUT/'checks.json').read_text(encoding='utf-8'))
    files=list(OUT.glob('light-*.png'))
    assert len(files)==checks['boards']==560
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    for file in files:
        image=Image.open(file).convert('RGB')
        before=image.crop((0,32,672,256)).tobytes()
        draw=ImageDraw.Draw(image)
        draw.rectangle((0,0,671,31),fill=(30,40,49))
        for col,label in enumerate(['0 · Avant','01 · Discret','02 · Mat']):
            draw.text((8+col*224,8),label,font=font,fill=(232,240,242))
        assert before==image.crop((0,32,672,256)).tobytes()
        image.save(file)
    section='''<section id="eclairage"><h2>Éclairage du joueur : deux réglages plus discrets</h2>
<p>Le personnage, sa position, le décor et les lumières sont identiques dans chaque version. <strong>01 / Discret</strong> réduit l’éclaircissement tout en gardant la silhouette bien lisible. <strong>02 / Mat</strong> conserve davantage les ombres. Les reflets déjà dessinés dans les matières sont conservés.</p>
<p class="pending">Nouveaux réglages en revue. Le jeu normal conserve son éclairage actuel ; les PNJ, monstres, objets et portraits ne changent pas.</p>
<div class="controls"><label>Personnage<select id="exposure-body"><option value="3">Bastion</option><option value="5">Traqueuse</option><option value="6">Arcaniste</option><option value="7">Métamorphe</option></select></label>
<label>Armure<select id="exposure-tier"><option value="3">Lourde</option><option value="0">Tenue de base</option><option value="1">Légère</option><option value="2">Intermédiaire</option><option value="4">Très lourde</option></select></label>
<label>Vue<select id="exposure-view"><option value="front">Face</option><option value="back">Dos</option><option value="left">Gauche</option><option value="right">Droite</option></select></label>
<label>Comparaison<select id="exposure-pair"><option value="1">Actuel + 01 / Discret</option><option value="2">Actuel + 02 / Mat</option><option value="all">Les trois versions</option></select></label>
<button id="exposure-play">Lire la marche</button><label>Vitesse<select id="exposure-speed"><option value="1">Normale</option><option value="3">Ralenti ×3</option></select></label></div>
<div class="native" style="max-width:672px"><canvas id="exposure-board" width="448" height="256" style="display:block;image-rendering:pixelated;max-width:none" aria-label="Éclairage actuel et nouvelle proposition dans le même décor à taille native"></canvas></div>
<div class="timeline"><label>Phase<input id="exposure-phase" type="range" min="0" max="6" value="6"></label><output id="exposure-time">Repos frontal</output></div>
<p id="exposure-status" class="note" role="status"></p>
<details><summary>Zoom entier ×2 de ce comparatif</summary><div class="native"><canvas id="exposure-zoom" width="896" height="512" style="display:block;image-rendering:pixelated;max-width:none" aria-label="Même comparatif d’éclairage au zoom entier ×2"></canvas></div></details>
<p class="note">Personnages à taille native de 64 px. Les six phases de marche et le vrai retour au repos utilisent les mêmes réglages. Les captures proviennent du moteur ; le navigateur affiche ces pixels sans filtre ni changement de luminosité.</p></section>'''
    script='''let exposureFrames=[],exposureGeneration=0,exposurePaused=true,exposurePhase=6,exposureStart=performance.now();
function paintExposure(){const image=exposureFrames[exposurePhase];if(!image?.complete)return;const all=$('exposure-pair').value==='all',canvas=$('exposure-board'),zoom=$('exposure-zoom');canvas.width=all?672:448;canvas.height=256;const context=canvas.getContext('2d');context.imageSmoothingEnabled=false;if(all)context.drawImage(image,0,0);else{context.drawImage(image,0,0,224,256,0,0,224,256);context.drawImage(image,224*(+$('exposure-pair').value),0,224,256,224,0,224,256);}zoom.width=canvas.width*2;zoom.height=512;const z=zoom.getContext('2d');z.imageSmoothingEnabled=false;z.drawImage(canvas,0,0,zoom.width,zoom.height);$('exposure-phase').value=exposurePhase;$('exposure-time').textContent=exposurePhase===6?'Repos frontal':`${exposurePhase+1} / 6`;}
async function loadExposure(){const generation=++exposureGeneration;$('exposure-status').textContent='Préparation du cycle…';const images=Array.from({length:7},(_,n)=>{const image=new Image();image.src=`player-light-r01/current/light-${$('exposure-body').value}-${$('exposure-tier').value}-${$('exposure-view').value}-${n}.png`;return image;});try{await Promise.all(images.map(image=>image.decode()));if(generation!==exposureGeneration)return;exposureFrames=images;exposureStart=performance.now();paintExposure();$('exposure-status').textContent='Même position et même lumière · repos et marche disponibles.';}catch(error){$('exposure-status').textContent='Capture manquante : '+error.message;}}
for(const id of ['exposure-body','exposure-tier','exposure-view'])$(id).onchange=loadExposure;
$('exposure-pair').onchange=paintExposure;$('exposure-play').onclick=()=>{exposurePaused=!exposurePaused;$('exposure-play').textContent=exposurePaused?'Lire la marche':'Pause';exposureStart=performance.now();};$('exposure-phase').oninput=()=>{exposurePaused=true;$('exposure-play').textContent='Lire la marche';exposurePhase=+$('exposure-phase').value;paintExposure();};
function animateExposure(now){if(!exposurePaused){const next=Math.floor(((now-exposureStart)/(+$('exposure-speed').value))%800/(800/6));if(next!==exposurePhase){exposurePhase=next;paintExposure();}}requestAnimationFrame(animateExposure);}loadExposure();requestAnimationFrame(animateExposure);'''
    integration=ROOT/'docs/reviews/animation-polish-01/LIGHT01-INTEGRATION.json'
    if integration.exists() and json.loads(integration.read_text(encoding='utf-8'))['normal_game_enabled']:
        section=section.replace('Éclairage du joueur : deux réglages plus discrets','Éclairage du joueur : 01 / Discret intégré')
        section=section.replace('Nouveaux réglages en revue. Le jeu normal conserve son éclairage actuel ; les PNJ, monstres, objets et portraits ne changent pas.','01 / Discret est validé et actif en jeu, au repos et en marche. La colonne « Avant » conserve le rendu précédent ; 02 / Mat reste une alternative non retenue.')
        section=section.replace('Actuel +','Avant +').replace('Éclairage actuel et nouvelle proposition','Éclairage précédent et réglage validé')
    return section,script
