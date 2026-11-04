"""Publish native HELM05 engine captures in the existing animation review."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont
from build_helmet_eye_review import build as build_eye_review

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/helmet-worn-r01/current'
LABELS=['Tête de la tenue','01 · Très léger','02 · Léger','03 · Intermédiaire','04 · Lourd','05 · Très lourd']
CLASSES=['Bastion','Traqueuse','Arcaniste','Métamorphe']


def compact(image):
    result=Image.new('RGB',(336,730),(30,40,49))
    draw=ImageDraw.Draw(result)
    font=lambda size:ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)
    draw.text((8,10),'HELM05 · mêmes casques, port ajusté',font=font(15),fill='white')
    for col,name in enumerate(CLASSES):
        draw.text((4+col*84,47),name,font=font(11),fill='white')
    for row,label in enumerate(LABELS):
        draw.text((8,80+row*100),label,font=font(12),fill=(174,191,200))
        for col in range(4):
            x,y=22+col*128,100+row*100
            result.paste(image.crop((x,y,x+80,y+80)),(2+col*84,100+row*100))
    draw.text((8,704),'Taille native · pas de redimensionnement des figures',font=font(11),fill=(174,191,200))
    return result


def motion_pair(before, after, col=0, row=2):
    result=Image.new('RGB',(336,338),(30,40,49))
    draw=ImageDraw.Draw(result)
    font=lambda size:ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)
    for n,(title,image) in enumerate([('02 · Trop grande',before),('03 · Taille et netteté',after)]):
        draw.text((14+n*168,6),title,font=font(13),fill='white')
        tile=image.crop((22+col*128,100+row*100,102+col*128,180+row*100))
        result.paste(tile,(44+n*168,30))
        result.paste(tile.resize((160,160),Image.Resampling.NEAREST),(4+n*168,154))
    draw.text((8,115),'Taille native · mêmes phases',font=font(11),fill=(174,191,200))
    draw.text((8,316),'Zoom entier ×2',font=font(11),fill=(174,191,200))
    return result


def build():
    if not (OUT/'checks.json').exists():return '', ''
    frames=[compact(Image.open(OUT/f'worn-tier4-front-{n}.png').convert('RGB')) for n in range(7)]
    frames[0].save(OUT/'HELM05-native.webp',save_all=True,append_images=frames[1:],duration=[133]*6+[550],loop=0,lossless=True,method=6)
    frames[0].save(OUT/'HELM05-native.png')
    before=OUT/'motion03/before'
    if (before/'worn-tier2-front-0.png').exists():
        for direction in ['front','back','left','right']:
            pairs=[motion_pair(Image.open(before/f'worn-tier2-{direction}-{n}.png').convert('RGB'),Image.open(OUT/f'worn-tier2-{direction}-{n}.png').convert('RGB')) for n in range(6)]
            pairs[0].save(OUT/f'motion03/{direction}-compare.webp',save_all=True,append_images=pairs[1:],duration=[133,133,134,133,133,134],loop=0,lossless=True,method=6)
    section='''<section id="casques-portes"><h2>03 · Proportions et netteté</h2>
<p>La tête complète était trop grosse et paraissait plus floue que l’armure. Cette proposition réduit son volume selon le personnage et l’armure. Elle conserve la tête et le casque comme un seul ensemble, avec des pixels fixes pendant la marche.</p>
<div class="controls"><label>Armure<select id="worn-armor"><option value="2">Intermédiaire</option><option value="3">Lourde</option><option value="4">Très lourde</option><option value="0">Très légère / base</option><option value="1">Légère</option></select></label><label>Vue<select id="worn-view"><option value="front">Face</option><option value="back">Dos</option><option value="left">Gauche</option><option value="right">Droite</option></select></label><button id="worn-pause">Pause</button><label>Vitesse<select id="worn-speed"><option value="1">Normale</option><option value="3">Ralenti ×3</option></select></label><label>Cycle<select id="worn-cycle"><option value="walk">Marche continue</option><option value="stop">Marche puis arrêt</option></select></label></div>
<div class="controls"><label>Personnage<select id="worn-class"><option value="0">Bastion</option><option value="1">Traqueuse</option><option value="2">Arcaniste</option><option value="3">Métamorphe</option></select></label><label>Casque<select id="worn-model"><option value="2">02 · Léger</option><option value="1">01 · Très léger</option><option value="3">03 · Intermédiaire</option><option value="4">04 · Lourd</option><option value="5">05 · Très lourd</option></select></label></div>
<div class="native" style="max-width:336px"><canvas id="worn-motion" width="336" height="338" style="display:block;width:336px;height:338px;image-rendering:pixelated" aria-label="Version 02 trop volumineuse et proposition 03 plus petite sans interpolation, synchronisées à taille native et au zoom entier ×2"></canvas></div>
<details><summary>Les cinq casques sur les quatre classes à taille native</summary><div class="native" style="max-width:336px"><canvas id="worn-compact" width="336" height="730" style="display:block;width:336px;height:730px;image-rendering:pixelated" aria-label="Les cinq nouveaux casques sur les quatre classes à taille native"></canvas></div></details>
<div class="timeline"><label for="worn-phase">Phase</label><input id="worn-phase" type="range" min="0" max="6" value="0" style="width:180px"><output id="worn-time"></output></div><p id="worn-status" role="status" class="note"></p>
<details><summary>Planche complète du moteur et zoom ×2</summary><div class="native" style="max-width:600px"><img id="worn-full" width="600" height="730" style="display:block;width:600px;height:730px;max-width:none;image-rendering:pixelated" alt="Capture native complète du moteur"></div><div class="native"><img id="worn-zoom" width="1200" height="1460" style="display:block;width:1200px;height:1460px;max-width:none;image-rendering:pixelated" alt="Même capture du moteur au zoom entier ×2"></div></details>
<p class="note">Les corps, bras, armes et six phases de marche sont conservés. Les têtes portées et leur ajustement sont de nouveaux candidats. La lecture continue permet d’examiner le raccord sans retour frontal entre les pas ; « Marche puis arrêt » montre aussi le vrai repos.</p>
<details><summary>Voir les cinq modèles dans le camp</summary><p class="note">Colonnes : casques 01 à 05. Lignes : Bastion, Traqueuse, Arcaniste, Métamorphe en armure très lourde. Éclairage et matériaux du camp rendus par le moteur.</p><div class="native" style="max-width:640px"><img src="helmet-worn-r01/current/worn-camp.png" width="640" height="610" style="display:block;max-width:none;image-rendering:pixelated" alt="Vingt personnages casqués dans le camp, à taille native"></div></details>
<details><summary>Examiner les têtes du casque et de l’armure sélectionnés</summary><p class="note">Colonnes : face, dos, gauche, droite. Lignes : Bastion, Traqueuse, Arcaniste, Métamorphe. Zoom entier ×4.</p><div class="native" style="max-width:512px"><img id="worn-heads" src="helmet-worn-r01/current/motion03/H03-W02-tier3-heads-zoom4.png" width="512" height="512" style="display:block;max-width:none;image-rendering:pixelated" alt="Quatre vues des têtes aux tailles finales pour les quatre personnages"></div></details></section>'''
    script='''let wornImages=[],wornPrevious=[],wornGeneration=0,wornPaused=false,wornPhase=0,wornStarted=performance.now();
const wornNames=['Bastion','Traqueuse','Arcaniste','Métamorphe'];
const wornLabels=['Tête de la tenue','01 · Très léger','02 · Léger','03 · Intermédiaire','04 · Lourd','05 · Très lourd'];
function paintMotion(){const ctx=$('worn-motion').getContext('2d');ctx.imageSmoothingEnabled=false;ctx.fillStyle='#1e2831';ctx.fillRect(0,0,336,338);ctx.fillStyle='#e4ebee';ctx.font='13px system-ui';ctx.fillText('Avant',14,20);ctx.fillText('01 · Raccord au torse',174,20);const x=22+128*(+$('worn-class').value),y=100+100*(+$('worn-model').value);[wornPrevious[wornPhase],wornImages[wornPhase]].forEach((image,col)=>{if(image?.complete){ctx.drawImage(image,x,y,80,80,44+168*col,30,80,80);ctx.drawImage(image,x,y,80,80,4+168*col,154,160,160);}});ctx.font='11px system-ui';ctx.fillStyle='#aebfc8';ctx.fillText('Taille native · mêmes phases',8,129);ctx.fillText('Zoom entier ×2',8,330);}
function paintWorn(){const image=wornImages[wornPhase];if(!image?.complete)return;paintMotion();const ctx=$('worn-compact').getContext('2d');ctx.imageSmoothingEnabled=false;ctx.fillStyle='#1e2831';ctx.fillRect(0,0,336,730);ctx.fillStyle='#e4ebee';ctx.font='15px system-ui';ctx.fillText('HELM05 · mêmes casques, port ajusté',8,25);ctx.font='11px system-ui';for(let col=0;col<4;col++)ctx.fillText(wornNames[col],4+84*col,59);ctx.font='12px system-ui';for(let row=0;row<6;row++){ctx.fillStyle='#aebfc8';ctx.fillText(wornLabels[row],8,94+row*100);for(let col=0;col<4;col++)ctx.drawImage(image,22+col*128,100+row*100,80,80,2+col*84,100+row*100,80,80);}ctx.font='11px system-ui';ctx.fillText('Taille native · figures conservées pixel pour pixel',8,718);for(const id of ['worn-full','worn-zoom'])if($(id).src!==image.src)$(id).src=image.src;$('worn-phase').value=wornPhase;$('worn-time').textContent=wornPhase===6?'Repos frontal':`${wornPhase+1} / 6`;}
async function loadWorn(){const generation=++wornGeneration;const images=Array.from({length:14},(_,n)=>{const image=new Image();image.src=`helmet-worn-r01/current/${n<7?'motion01/before/':''}worn-tier${$('worn-armor').value}-${$('worn-view').value}-${n%7}.png`;return image;});try{await Promise.all(images.map(image=>image.decode()));if(generation!==wornGeneration)return;wornPrevious=images.slice(0,7);wornImages=images.slice(7);wornPhase=0;wornStarted=performance.now();$('worn-status').textContent='Avant / raccord 01 · même phase et mêmes dessins. Le résultat reste à juger en mouvement.';paintWorn();}catch(error){$('worn-status').textContent='Capture manquante : '+error.message;}}
for(const id of ['worn-armor','worn-view'])$(id).onchange=loadWorn;
for(const id of ['worn-class','worn-model'])$(id).onchange=paintWorn;
$('worn-pause').onclick=()=>{wornPaused=!wornPaused;$('worn-pause').textContent=wornPaused?'Lecture':'Pause';wornStarted=performance.now();};
$('worn-phase').oninput=()=>{wornPaused=true;$('worn-pause').textContent='Lecture';wornPhase=+$('worn-phase').value;paintWorn();};
function animateWorn(now){if(!wornPaused){const elapsed=(now-wornStarted)/(+$('worn-speed').value);const continuous=$('worn-cycle').value==='walk';const duration=continuous?800:1350;const time=elapsed%duration;const next=time<800?Math.min(5,Math.floor(time/(800/6))):6;if(next!==wornPhase){wornPhase=next;paintWorn();}}requestAnimationFrame(animateWorn);}
loadWorn();requestAnimationFrame(animateWorn);'''
    eye_section,eye_script=build_eye_review()
    script=script.replace("ctx.fillText('Avant',14,20);ctx.fillText('01 · Raccord au torse',174,20);", "ctx.fillText('02 · Trop grande',8,20);ctx.fillText('03 · Taille et netteté',174,20);")
    script=script.replace("'motion01/before/'", "'motion03/before/'")
    script=script.replace("Avant / raccord 01 · même phase et mêmes dessins. Le résultat reste à juger en mouvement.", "02 refusée / 03 proposée · même corps, même phase. Proportions et netteté à juger.")
    script=script.replace('paintMotion();const ctx=', "paintMotion();const heads=`helmet-worn-r01/current/motion03/H03-W${$('worn-model').value.padStart(2,'0')}-tier${$('worn-armor').value}-heads-zoom4.png`;if($('worn-heads').getAttribute('src')!==heads)$('worn-heads').src=heads;const ctx=")
    return section+'\n<details><summary>Ancienne étude de l’optique, avant la reprise des têtes</summary>'+eye_section+'</details>',eye_script+'\n'+script


if __name__=='__main__':
    build()
    print(json.dumps({'preview':str(OUT),'native_summary':[336,730]}))
