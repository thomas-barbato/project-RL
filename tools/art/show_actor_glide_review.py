"""Embed the native glide contact sheet in the existing reusable equipment page."""
import base64
import hashlib
import json
import re
from io import BytesIO
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / 'artifacts/apercu-equipement64.html'
CAPTURE = ROOT / 'artifacts/actor-glide-check/native-glide-atlas.png'
REVIEW = ROOT / 'docs/reviews/refonte-agent-2026-10-06/actor-glide-01'


def main():
    html = OUTPUT.read_text('utf-8')
    html = re.sub(r'<!-- GLIDE64-BEGIN -->.*?<!-- GLIDE64-END -->', '', html, flags=re.S)
    html = re.sub(r'// GLIDE64-SCRIPT-BEGIN.*?// GLIDE64-SCRIPT-END', '', html, flags=re.S)
    with Image.open(CAPTURE) as atlas:
        assert atlas.getextrema()[-1] == (255, 255), 'Framebuffer must be opaque'
        packed = BytesIO()
        atlas.convert('RGB').save(packed, format='PNG', optimize=True)
    pixels = packed.getvalue()
    uri = 'data:image/png;base64,' + base64.b64encode(pixels).decode()
    section = '''<!-- GLIDE64-BEGIN --><section id="glide" class="section">
<h2>Déplacement entre cases · 80 ms</h2>
<p class="note">Rendu du jeu, en 64 px. Les quatre directions sont montrées à tour de rôle.
Le joueur garde sa place à l’écran pendant que la caméra suit son déplacement ; les autres exemples montrent le glissement sur un sol fixe.
L’équipement, l’ombre et les effets suivent le porteur. Les commandes restent disponibles pendant le mouvement.</p>
<button id="glidepause" type="button">Pause</button> <button id="glidereplay" type="button">Rejouer</button>
<button id="glidespeed" type="button">Voir au ralenti ×4</button> <button id="glidezoom" type="button">Voir en zoom ×2</button>
<p id="glidecaption">Vitesse du jeu · une pause sépare les exemples.</p>
<div class="capture"><canvas id="glidestage" width="896" height="640" style="display:block;width:896px;height:640px;max-width:none;image-rendering:pixelated" aria-label="Glissement du joueur, d’un monstre gelé, d’un drone et d’une transformation dans les quatre directions"></canvas></div>
<img id="glideatlas" hidden alt="36 images du diagnostic natif" src="URI">
</section><!-- GLIDE64-END -->'''.replace('URI', uri)
    html = html.replace('<section id="breathing" class="section">', '<section id="breathing" class="section" hidden>')
    html = html.replace('data-tab="breathing" class="active"', 'data-tab="breathing"')
    if 'data-tab="glide"' not in html:
        html = html.replace('<button data-tab="breathing"', '<button data-tab="glide" class="active">Déplacement</button><button data-tab="breathing"', 1)
    html = html.replace('<section id="breathing"', section + '<section id="breathing"', 1)
    html = html.replace("let selected='breathing';", "let selected='glide';")
    if "selected==='glide'" not in html:
        html = html.replace("selected==='breathing'", "(selected==='breathing'||selected==='glide')")
    script = '''// GLIDE64-SCRIPT-BEGIN
const glideCanvas=document.querySelector('#glidestage'),glideContext=glideCanvas.getContext('2d'),glideAtlas=document.querySelector('#glideatlas');
let glidePlaying=!matchMedia('(prefers-reduced-motion: reduce)').matches,glideClock=0,glideLast=performance.now(),glideSlow=false;
function drawGlide(now){const elapsed=Math.min(now-glideLast,100);glideLast=now;
 if(glidePlaying&&selected==='glide')glideClock+=elapsed/(glideSlow?4:1);
 const direction=Math.floor(glideClock/530)%4,age=glideClock%530,frame=direction*9+Math.min(8,Math.floor(age/10));
 if(glideAtlas.complete&&glideAtlas.naturalWidth){glideContext.imageSmoothingEnabled=false;glideContext.drawImage(glideAtlas,(frame%6)*896,Math.floor(frame/6)*640,896,640,0,0,896,640);}
 requestAnimationFrame(drawGlide);}
document.querySelector('#glidepause').textContent=glidePlaying?'Pause':'Reprendre';
document.querySelector('#glidepause').onclick=()=>{glidePlaying=!glidePlaying;document.querySelector('#glidepause').textContent=glidePlaying?'Pause':'Reprendre';};
document.querySelector('#glidereplay').onclick=()=>{glideClock=0;glidePlaying=true;document.querySelector('#glidepause').textContent='Pause';};
document.querySelector('#glidespeed').onclick=()=>{glideSlow=!glideSlow;document.querySelector('#glidespeed').textContent=glideSlow?'Revenir à la vitesse du jeu':'Voir au ralenti ×4';document.querySelector('#glidecaption').textContent=glideSlow?'Ralenti de revue ×4 · le jeu conserve un déplacement de 80 ms.':'Vitesse du jeu · une pause sépare les exemples.';};
document.querySelector('#glidezoom').onclick=()=>{const big=glideCanvas.style.width==='1792px';glideCanvas.style.width=big?'896px':'1792px';glideCanvas.style.height=big?'640px':'1280px';document.querySelector('#glidezoom').textContent=big?'Voir en zoom ×2':'Revenir à la taille native';};
requestAnimationFrame(drawGlide);
document.querySelector('[data-tab="glide"]').click();
// GLIDE64-SCRIPT-END
'''
    html = html.replace('</script>', script + '</script>', 1)
    OUTPUT.write_text(html, 'utf-8')
    validation = json.loads((CAPTURE.parent / 'art-validation.json').read_text('utf-8'))
    validation.update({
        'atlas_sha256': hashlib.sha256(CAPTURE.read_bytes()).hexdigest(),
        'atlas_bytes': CAPTURE.stat().st_size,
        'embedded_atlas_bytes': len(pixels),
        'embedded_atlas_sha256': hashlib.sha256(pixels).hexdigest(),
        'packing': 'Lossless PNG compression; opaque framebuffer RGB preserved exactly',
        'reusable_gallery': str(OUTPUT.relative_to(ROOT)),
        'gallery_bytes': OUTPUT.stat().st_size,
        'native_frames_embedded': 36,
    })
    (REVIEW / 'native-validation-01.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n', 'utf-8')
    print(json.dumps(validation, ensure_ascii=False))


if __name__ == '__main__':
    main()
