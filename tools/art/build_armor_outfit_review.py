"""Retained complete outfits, copied 1:1 from the engine's existing bare rows."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/armor-only/current'
OLD=ROOT/'artifacts/animation-polish-01/helmet-worn-r01/current'
CLASSES=['Bastion','Traqueuse','Arcaniste','Métamorphe']


def build():
    OUT.mkdir(parents=True,exist_ok=True)
    if not (OUT/'outfit-tier4-front-0.png').exists():
        for file in OLD.glob('worn-tier*-*.png'):
            image=Image.open(file).convert('RGB');compact=Image.new('RGB',(336,160),(30,40,49));draw=ImageDraw.Draw(compact)
            font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',11)
            for col,name in enumerate(CLASSES):
                draw.text((4+col*84,12),name,font=font,fill='white')
                tile=image.crop((22+col*128,100,102+col*128,180))
                compact.paste(tile,(2+col*84,40))
                assert compact.crop((2+col*84,40,82+col*84,120)).tobytes()==tile.tobytes()
            draw.text((8,140),'Natif64 · dessin complet de la tenue conservé',font=font,fill=(174,191,200))
            compact.save(OUT/file.name.replace('worn-','outfit-'))
    assert len(list(OUT.glob('outfit-tier*-*.png')))==140
    (OUT/'checks.json').write_text(json.dumps({'review':'HELM06-ARMOR-ONLY','native_tile_px':64,'classes':4,'armor_stages':5,'directions':4,'frames':7,'source':'existing engine bare rows; figures copied RGB 1:1','helmet_overlay':False},indent=2)+'\n',encoding='utf-8')
    section='''<section id="armures"><h2>Apparence du personnage : l’armure uniquement</h2>
<p>Le personnage conserve le dessin complet de sa tenue et de son armure, avec la tête et les protections qui en font déjà partie. Équiper un casque ne modifie plus cette apparence ; l’objet et ses bonus sont conservés.</p>
<div class="controls"><label>Armure<select id="outfit-tier"><option value="2">Intermédiaire</option><option value="0">Très légère / base</option><option value="1">Légère</option><option value="3">Lourde</option><option value="4">Très lourde</option></select></label><label>Vue<select id="outfit-view"><option value="front">Face</option><option value="back">Dos</option><option value="left">Gauche</option><option value="right">Droite</option></select></label><button id="outfit-pause">Pause</button><label>Vitesse<select id="outfit-speed"><option value="1">Normale</option><option value="3">Ralenti ×3</option></select></label></div>
<div class="native" style="max-width:336px"><img id="outfit-image" width="336" height="160" style="display:block;width:336px;height:160px;max-width:none;image-rendering:pixelated" alt="Les quatre personnages avec le seul visuel complet de leur armure"></div>
<div class="timeline"><label>Phase<input id="outfit-phase" type="range" min="0" max="6" value="0"></label><output id="outfit-time"></output></div><p id="outfit-status" role="status" class="note"></p>
<details><summary>Zoom entier ×2</summary><div class="native" style="max-width:672px"><img id="outfit-zoom" width="672" height="320" style="display:block;width:672px;height:320px;max-width:none;image-rendering:pixelated" alt="Même dessin des armures au zoom entier ×2"></div></details></section>'''
    script='''let outfitFrames=[],outfitGeneration=0,outfitPaused=false,outfitPhase=0,outfitStart=performance.now();
function paintOutfit(){const image=outfitFrames[outfitPhase];if(!image?.complete)return;for(const id of ['outfit-image','outfit-zoom'])if($(id).src!==image.src)$(id).src=image.src;$('outfit-phase').value=outfitPhase;$('outfit-time').textContent=outfitPhase===6?'Repos frontal':`${outfitPhase+1} / 6`;}
async function loadOutfit(){const generation=++outfitGeneration;$('outfit-status').textContent='Préparation des poses…';const images=Array.from({length:7},(_,n)=>{const image=new Image();image.src=`armor-only/current/outfit-tier${$('outfit-tier').value}-${$('outfit-view').value}-${n}.png`;return image;});try{await Promise.all(images.map(image=>image.decode()));if(generation!==outfitGeneration)return;outfitFrames=images;outfitPhase=0;outfitStart=performance.now();$('outfit-status').textContent='Visuel de la tenue conservé · aucun casque ajouté.';paintOutfit();}catch(error){$('outfit-status').textContent='Capture manquante : '+error.message;}}
for(const id of ['outfit-tier','outfit-view'])$(id).onchange=loadOutfit;
$('outfit-pause').onclick=()=>{outfitPaused=!outfitPaused;$('outfit-pause').textContent=outfitPaused?'Lecture':'Pause';outfitStart=performance.now();};$('outfit-phase').oninput=()=>{outfitPaused=true;$('outfit-pause').textContent='Lecture';outfitPhase=+$('outfit-phase').value;paintOutfit();};
function animateOutfit(now){if(!outfitPaused){const next=Math.floor(((now-outfitStart)/(+$('outfit-speed').value))%800/(800/6));if(next!==outfitPhase){outfitPhase=next;paintOutfit();}}requestAnimationFrame(animateOutfit);}loadOutfit();requestAnimationFrame(animateOutfit);'''
    return section,script
