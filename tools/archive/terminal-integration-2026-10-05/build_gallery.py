from pathlib import Path
from PIL import Image
import json
import shutil

ROOT = Path(__file__).resolve().parent
SERVER = ROOT.parent / 'armures-bicolores-completes-2026-10-05'
DEST = SERVER / 'terminal-integre'
DEST.mkdir(exist_ok=True)
views = [
    ('Inventaire violet', 'terminal-inventory-live-hands-equipped-violet', '#9780ff'),
    ('Plateau violet', 'terminal-hud-live-violet', '#9780ff'),
    ('Inventaire bleu', 'terminal-inventory-live-hands-equipped-blue', '#36b9ff'),
    ('Plateau bleu', 'terminal-hud-live-blue', '#36b9ff'),
    ('Inventaire vert', 'terminal-inventory-live-hands-equipped-green', '#87e875'),
    ('Plateau vert', 'terminal-hud-live-green', '#87e875'),
    ('Inventaire rouge', 'terminal-inventory-live-hands-equipped-red', '#ff6e8a'),
    ('Plateau rouge', 'terminal-hud-live-red', '#ff6e8a'),
    ('Inventaire compact', 'terminal-inventory-live-hands-violet-small', '#9780ff'),
    ('Plateau compact', 'terminal-hud-live-violet-small', '#9780ff'),
    ('Inventaire · UI 150 %', 'terminal-inventory-live-hands-violet-largeui', '#9780ff'),
    ('Plateau · UI 150 %', 'terminal-hud-live-violet-largeui', '#9780ff'),
    ('Plateau · 1920 × 1080', 'terminal-hud-live-violet-wide', '#9780ff'),
    ('Détails d’une arme', 'terminal-inventory-live-bag-blue', '#36b9ff'),
    ('Écran de mort', 'ux-death', '#9780ff'),
]
metadata = []
for label, scene, color in views:
    path = ROOT / 'captures' / scene / 'cold-start.png'
    assert path.exists(), scene
    shutil.copyfile(path, DEST / f'{scene}.png')
    metadata.append(dict(label=label, scene=scene, color=color, size=Image.open(path).size))
html = '''<!doctype html><html lang="fr"><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1"><title>Terminal et inventaire intégrés — Project RL</title>
<style>
:root{color-scheme:dark;--accent:#9780ff}*{box-sizing:border-box}
body{margin:0;background:#080b10;color:#e1e7ed;font:15px/1.55 system-ui,sans-serif}
main{max-width:1370px;margin:auto;padding:20px 24px 30px}h1{font-size:25px;margin:4px 0 7px}
p{margin:6px 0;color:#b5c0ca}.tag{font:12px monospace;letter-spacing:.07em;color:var(--accent)}
.status{border-left:3px solid var(--accent);background:#11141d;padding:10px 14px;margin:15px 0}
.status strong{color:#e1e7ed}nav{display:flex;gap:6px;flex-wrap:wrap;margin:16px 0}
button{font:12px monospace;color:#d8e1ea;background:#101620;border:1px solid #374252;padding:8px 10px;cursor:pointer}
button[aria-pressed=true]{background:#201c31;border-color:var(--accent);color:white}
button:hover{border-color:#dbe4ef}button:focus-visible{outline:2px solid white;outline-offset:3px}
.toolbar{display:flex;align-items:center;justify-content:space-between;gap:10px;margin:10px 0;flex-wrap:wrap}
.viewer{overflow:auto;border:1px solid #354052;background:#030507}.viewer img{display:block;width:100%;height:auto}
.viewer.native img{width:auto;max-width:none}small{display:block;margin-top:10px;color:#97a5b1}
.health{color:#7ae096}.energy{color:#6dbaff}.heat{color:#edba61}a{color:var(--accent)}
</style><main><div class="tag">PROJECT RL / VERSION INTÉGRÉE / 05.10.2026</div>
<h1>Le terminal, jusque dans l’inventaire</h1>
<p>Captures réelles du jeu. Les cadres, la silhouette et les boutons suivent le thème choisi dans les options.</p>
<div class="status"><strong>Couleurs des jauges corrigées :</strong> <span class="health">PV verts</span>, <span class="energy">énergie bleue</span>, <span class="heat">chaleur selon ses seuils</span>, expérience dans la couleur du thème. Les alertes critiques restent rouges.<br>
La silhouette, les emplacements, les couleurs des objets et leurs détails au survol sont conservés. Le plateau utilise maintenant le terminal validé.</div>
<nav id="choices" aria-label="Captures numérotées"></nav>
<div class="toolbar"><p id="caption"></p><button id="scale" aria-pressed="false">Taille réelle (1:1)</button></div>
<div id="viewer" class="viewer"><img id="screen" alt="Inventaire dans le jeu"></div>
<small>Le mode 1:1 permet de juger les pixels sans réduction. Les vues compactes et l’interface à 150 % vérifient les emplacements et la lisibilité.</small>
</main><script>const views=__VIEWS__;
const choices=document.querySelector('#choices'),screen=document.querySelector('#screen'),caption=document.querySelector('#caption');
function show(i){const v=views[i];screen.src='terminal-integre/'+v.scene+'.png';screen.alt=v.label;caption.textContent=(i+1)+' · '+v.label+' — '+v.size.join(' × ');document.documentElement.style.setProperty('--accent',v.color);choices.querySelectorAll('button').forEach((b,j)=>b.setAttribute('aria-pressed',String(i===j)))}
views.forEach((v,i)=>{const b=document.createElement('button');b.textContent=(i+1)+' · '+v.label;b.setAttribute('aria-pressed','false');b.onclick=()=>show(i);choices.append(b)});
document.querySelector('#scale').onclick=e=>{const on=document.querySelector('#viewer').classList.toggle('native');e.currentTarget.setAttribute('aria-pressed',String(on));e.currentTarget.textContent=on?'Adapter à la fenêtre':'Taille réelle (1:1)'};show(0);
</script></html>'''.replace('__VIEWS__', json.dumps(metadata, ensure_ascii=False))
(ROOT / 'galerie-terminal.html').write_text(html, encoding='utf-8')
if not (SERVER / 'galerie-terminal-proposition.html').exists():
    shutil.copyfile(SERVER / 'galerie-terminal.html', SERVER / 'galerie-terminal-proposition.html')
(SERVER / 'galerie-terminal.html').write_text(html, encoding='utf-8')
print(json.dumps({'captures': len(views), 'url': 'http://127.0.0.1:8769/galerie-terminal.html'}))
