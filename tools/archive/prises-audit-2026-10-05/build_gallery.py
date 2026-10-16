"""Actual engine screenshots plus a clearly labelled idle animation proposal."""
import base64
import html
import io
import json
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
ART=ROOT/'assets/sprites/cyberpunk'
manifest=json.loads((ART/'manifest.json').read_text(encoding='utf-8'))
names=['BRÈCHE','VIGIE','CREUSET','Cerbère','Cognefer','Chrysalide','Azurite','Fantôme','Robe de mue','Bastion','Ferrailleur','Seconde Ombre','Membranes','Peau-Seconde','Technicien','Aviateur','Nomade']
pages=[
    ('01 · Les 18 familles','crawl-weapons'),
    ('02 · Une main / tenues 01–12','crawl-grips-one-1'),
    ('03 · Une main / tenues 13–17 + NPC','crawl-grips-one-2'),
    ('04 · Deux mains / tenues 01–12','crawl-grips-two-1'),
    ('05 · Deux mains / tenues 13–17 + NPC','crawl-grips-two-2'),
    ('06 · Ambidextrie','crawl-grips-dual'),
    ('07 · Plateau de jeu','terminal-hud-live'),
    ('08 · Inventaire et survol','terminal-inventory-live-equipped'),
    ('09 · Groupe à une main','terminal-inventory-live-hands'),
    ('10 · Écran de mort','ux-death'),
]
buttons=[];sections=[];reports=[]
for i,(title,scene) in enumerate(pages):
    folder=HERE/'captures'/f'final-{scene}'
    replacement=HERE/'captures'/f'final-{scene}-v2'
    if (replacement/'cold-start.png').exists():folder=replacement
    image=folder/'cold-start.png'
    if not image.exists():continue
    data=base64.b64encode(image.read_bytes()).decode()
    buttons.append(f'<button onclick="show({i})">{html.escape(title)}</button>')
    sections.append(f'<section id="p{i}" hidden><h2>{html.escape(title)}</h2><p>Capture réelle du moteur. Les planches 02 à 05 montrent les sprites à leur taille native : corps de 30 px, cases de 32 px. G / D désignent les côtés de l’écran.</p><div class="scroll"><img class="capture" src="data:image/png;base64,{data}" alt="{html.escape(title)}"></div></section>')
    if (folder/'art-validation.json').exists():reports.append(json.loads((folder/'art-validation.json').read_text()))
buttons.append('<button onclick="show(10)">11 · Essai d’animation idle</button>')
sections.append('''<section id="p10" hidden><h2>11 · Respiration discrète — proposition</h2>
<p>Cet aperçu web n’est pas encore intégré au jeu. Deux positions du buste, avec un déplacement d’un pixel toutes les 1,5 secondes. Les pieds restent fixes ; les mains et l’arme suivent le buste. Ce mouvement visuel pourrait être désactivé avec l’option de réduction des animations.</p>
<label>Tenue <select id="body"></select></label> <label>Arme <select id="family"></select></label>
<div class="demo"><div><h3>Pose fixe · taille native</h3><canvas id="still" width="48" height="48"></canvas><h3>Zoom ×4</h3><canvas id="stillZoom" width="48" height="48" class="zoom"></canvas></div>
<div><h3>Respiration · taille native</h3><canvas id="idle" width="48" height="48"></canvas><h3>Zoom ×4</h3><canvas id="idleZoom" width="48" height="48" class="zoom"></canvas></div></div></section>''')

def cells(name,side,count):
    atlas=Image.open(ART/f'crawl-{name}.png').convert('RGBA');result=[]
    for i in range(count):
        x=i%16*side;y=i//16*side;cell=atlas.crop((x,y,x+side,y+side));buf=io.BytesIO();cell.save(buf,'PNG')
        result.append('data:image/png;base64,'+base64.b64encode(buf.getvalue()).decode())
    return result
data=dict(names=names,weapons=[m['name'] for m in manifest['weapon_families']],
          bodies=cells('bodies-32',32,17),arms=cells('arms-48',48,18),
          fingers=cells('fingers-48',48,18),held=cells('weapons-48',48,18))
doc='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Project RL · prises corrigées</title><style>
body{margin:18px;background:#11191f;color:#d1e6e2;font:15px monospace}h1{font-size:22px}h2{font-size:18px}h3{font-size:14px}p{line-height:1.5;max-width:950px}
nav{display:flex;gap:8px;flex-wrap:wrap}button,select{font:inherit;color:#c1e8e0;background:#202e35;border:1px solid #56796f;padding:8px;cursor:pointer}button[aria-current=true]{background:#31584c}
.scroll{overflow:auto;border:1px solid #38534c}.capture{display:block;width:1280px;height:800px;max-width:none;image-rendering:pixelated}a{color:#9cdccc}
.demo{display:flex;gap:70px;flex-wrap:wrap;padding:20px;background:#1b272f;margin-top:20px}canvas{image-rendering:pixelated;background:#26343d}.zoom{width:192px;height:192px}
</style><h1>Project RL · prises corrigées</h1>
<p>18 familles couvrent les 113 modèles d’armes. Revue des 17 tenues et des 6 bases humanoïdes de NPC : 575 prises simples, plus les associations d’ambidextrie. Couteau, hache et grande épée présentent leur tranchant vers l’extérieur.</p>
<p>Les dessins de corps et leurs palettes sont conservés. Les manches, doigts et points de prise ont été corrigés ; les icônes au sol utilisent la silhouette d’arme nettoyée. Les captures ci-dessous proviennent du rendu actif. L’onglet 11 est un essai séparé pour l’animation.</p><nav>'''+''.join(buttons)+'</nav>'+''.join(sections)+'''
<script>
function show(n){document.querySelectorAll('section').forEach(s=>s.hidden=s.id!=='p'+n);document.querySelectorAll('nav button').forEach(b=>b.setAttribute('aria-current',b.getAttribute('onclick')==='show('+n+')'));}
const DATA='''+json.dumps(data,ensure_ascii=False)+''';
const bodySelect=document.querySelector('#body'),familySelect=document.querySelector('#family');
DATA.names.forEach((name,i)=>bodySelect.add(new Option(String(i+1).padStart(2,'0')+' '+name,i)));
DATA.weapons.forEach((name,i)=>familySelect.add(new Option(String(i+1).padStart(2,'0')+' '+name,i)));
bodySelect.value='9';familySelect.value='6';
const banks={};let ready=false;
Promise.all(['bodies','arms','fingers','held'].map(async bank=>{banks[bank]=await Promise.all(DATA[bank].map(url=>new Promise(resolve=>{const img=new Image();img.onload=()=>resolve(img);img.src=url;})));})).then(()=>ready=true);
let lastBody=-1,lastFamily=-1,fixedBody;
function drawPose(id,offset){
 const body=+bodySelect.value,family=+familySelect.value;
 if(body!==lastBody||family!==lastFamily){
  fixedBody=document.createElement('canvas');fixedBody.width=fixedBody.height=32;
  const ctx=fixedBody.getContext('2d');ctx.drawImage(banks.bodies[body],0,0);
  const single=[0,1,4,5,6,7,16].includes(family),raised=[6,7].includes(family),start=single?(raised?19:18):17;
  ctx.clearRect(22,start,8,23-start);if(!single)ctx.clearRect(3,17,7,6);
  lastBody=body;lastFamily=family;
 }
 const canvas=document.querySelector('#'+id),ctx=canvas.getContext('2d');ctx.imageSmoothingEnabled=false;ctx.clearRect(0,0,48,48);
 ctx.drawImage(fixedBody,0,0,32,23,8,8+offset,32,23);
 ctx.drawImage(fixedBody,0,23,32,9,8,31,32,9);
 ['arms','held','fingers'].forEach(bank=>ctx.drawImage(banks[bank][family],0,offset));
}
function animate(time){if(ready){const offset=Math.floor(time/1500)%2;drawPose('still',0);drawPose('stillZoom',0);drawPose('idle',offset);drawPose('idleZoom',offset);}requestAnimationFrame(animate);}
requestAnimationFrame(animate);show(0);
</script></html>'''
(HERE/'galerie-prises.html').write_text(doc,encoding='utf-8')
server=ROOT/'artifacts/armures-bicolores-completes-2026-10-05'
for filename in ['galerie-prises.html','galerie-integree.html']:(server/filename).write_text(doc,encoding='utf-8')
(HERE/'render-validation.json').write_text(json.dumps(reports,ensure_ascii=False,indent=2),encoding='utf-8')
print(f'Gallery: {len(sections)-1} engine captures + isolated idle proposal')
