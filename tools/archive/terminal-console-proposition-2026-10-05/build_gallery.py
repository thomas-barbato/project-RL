from pathlib import Path
import json
import shutil

ROOT = Path(__file__).resolve().parent
SERVER = ROOT.parent / 'armures-bicolores-completes-2026-10-05'
DEST = SERVER / 'terminal-console'
DEST.mkdir(exist_ok=True)
scenes = ['terminal-console-proposal-violet', 'terminal-console-proposal-blue',
          'terminal-console-proposal-green', 'terminal-console-proposal-red',
          'terminal-console-proposal-violet-small', 'terminal-hud-live',
          'crawl-idle', 'crawl-idle-reduced']
for scene in scenes:
    shutil.copyfile(ROOT / 'captures' / scene / 'cold-start.png', DEST / f'{scene}.png')
for pose in range(2):
    shutil.copyfile(ROOT / 'captures' / 'crawl-idle' / f'frame-{pose:02}.png', DEST / f'idle-{pose}.png')

html = '''<!doctype html><html lang="fr"><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Proposition terminal — captures du jeu</title>
<style>
:root{color-scheme:dark;--accent:#9780ff}*{box-sizing:border-box}
body{margin:0;background:#080c11;color:#e1e8ec;font:15px/1.55 system-ui,sans-serif}
main{max-width:1370px;margin:auto;padding:22px 26px 32px}
h1{font-size:25px;margin:0 0 7px}p{margin:7px 0;color:#b8c2cb}
.tag{font:12px monospace;letter-spacing:.07em;color:var(--accent)}
.status{border-left:3px solid var(--accent);padding:8px 14px;background:#10131e;margin:17px 0}
nav{display:flex;flex-wrap:wrap;gap:7px;margin:15px 0}button{font:13px monospace;background:#101720;border:1px solid #394657;color:#dbe6ec;padding:9px 12px;cursor:pointer;border-radius:0}
button[aria-pressed=true]{border-color:var(--accent);background:#201b36;color:white}
button:hover{border-color:#dce5eb}button:focus-visible{outline:2px solid white;outline-offset:3px}
.features{display:flex;flex-wrap:wrap;gap:6px 25px;padding:0;margin:13px 0 20px;list-style:none;color:#c3ccd4;font-size:13px}
.features b{color:var(--accent);font-family:monospace;margin-right:7px}
.toolbar{display:flex;justify-content:space-between;align-items:center;gap:14px;margin:18px 0 8px;flex-wrap:wrap}
#caption{font-size:14px;color:#c4cdd5;margin:0}.viewer{background:#030609;border:1px solid #31394c;overflow:auto}
.viewer img{display:block;width:100%;max-width:1280px;margin:auto;height:auto}.viewer.native img{width:auto;max-width:none;margin:0}
small{display:block;color:#96a3b0;margin-top:13px}a{color:#b9aaff}
</style><main>
<div class="tag">PROJET RL / PROPOSITION VISUELLE / 05.10.2026</div>
<h1>Un terminal plus présent, un plateau qui reste lisible</h1>
<p>Les vues ci-dessous sont capturées dans le moteur du jeu, avec la même scène, les mêmes objets et les mêmes valeurs.</p>
<div class="status"><strong>Respiration : intégrée.</strong> Le haut du corps et l’équipement bougent d’un pixel, les pieds restent fixes. L’option « mouvements réduits » désactive l’idle.<br>
<strong>Interface : proposition à juger.</strong> Le HUD normal reste en place ; ce nouveau dessin est réservé à la prévisualisation.</div>
<ul class="features">
<li><b>A</b>Cadres angulaires et modules numérotés</li>
<li><b>B</b>Jauges segmentées, valeurs conservées</li>
<li><b>C</b>Minicarte teintée par le thème choisi</li>
<li><b>D</b>Inspection et objectif mieux séparés</li>
<li><b>E</b>Raccourcis en touches de terminal</li>
<li><b>F</b>Journal sous forme de lignes « &gt; »</li></ul>
<nav aria-label="Captures numérotées">
<button data-view="violet" aria-pressed="true">1 · Violet</button>
<button data-view="blue" aria-pressed="false">2 · Bleu</button>
<button data-view="green" aria-pressed="false">3 · Vert</button>
<button data-view="red" aria-pressed="false">4 · Rouge</button>
<button data-view="compact" aria-pressed="false">5 · Fenêtre compacte</button>
<button data-view="idle" aria-pressed="false">6 · Respiration dans le moteur</button>
<button data-view="reduced" aria-pressed="false">7 · Mouvements réduits</button>
<button data-view="before" aria-pressed="false">0 · Interface actuelle</button></nav>
<div class="toolbar"><p id="caption"></p><button id="scale" aria-pressed="false">Afficher à taille réelle (1:1)</button></div>
<div class="viewer" id="viewer"><img id="screen" alt="Capture réelle de la proposition terminal"></div>
<small>1280 × 800 pour les grands écrans ; 960 × 540 pour la vue compacte. Le mode 1:1 permet d’inspecter les pixels sans réduction. Les jauges PV, énergie et munitions conservent leurs couleurs fonctionnelles dans tous les thèmes.</small>
<small>La proposition concerne ici le plateau et son HUD. Après ton avis, ce langage visuel pourra être décliné dans les autres fenêtres, en conservant l’inventaire approuvé.</small>
</main><script>
const views={
violet:['terminal-console-proposal-violet','1 · Thème violet — proposition', '#9780ff'],
blue:['terminal-console-proposal-blue','2 · Thème bleu — proposition', '#36b9ff'],
green:['terminal-console-proposal-green','3 · Thème vert — proposition', '#87e875'],
red:['terminal-console-proposal-red','4 · Thème rouge — proposition', '#ff6e8a'],
compact:['terminal-console-proposal-violet-small','5 · Vue compacte — proposition, 960 × 540', '#9780ff'],
idle:['idle-0','6 · Respiration intégrée — alternance de deux captures du moteur, 1,5 s par pose', '#9780ff'],
reduced:['crawl-idle-reduced','7 · Réduction des mouvements — pose fixe', '#9780ff'],
before:['terminal-hud-live','0 · Interface actuellement utilisée — même scène', '#9780ff']};
const screen=document.querySelector('#screen'),caption=document.querySelector('#caption');let active='violet',pose=0;
function show(view){active=view;pose=0;const [file,text,color]=views[view];screen.src='terminal-console/'+file+'.png';caption.textContent=text;document.documentElement.style.setProperty('--accent',color);document.querySelectorAll('[data-view]').forEach(b=>b.setAttribute('aria-pressed',String(b.dataset.view===view)));screen.alt=text}
document.querySelectorAll('[data-view]').forEach(b=>b.addEventListener('click',()=>show(b.dataset.view)));
document.querySelector('#scale').addEventListener('click',e=>{const enabled=document.querySelector('#viewer').classList.toggle('native');e.currentTarget.setAttribute('aria-pressed',String(enabled));e.currentTarget.textContent=enabled?'Adapter à la fenêtre':'Afficher à taille réelle (1:1)'});
setInterval(()=>{if(active==='idle'){pose=1-pose;screen.src='terminal-console/idle-'+pose+'.png'}},1500);
show('violet');
</script></html>'''
(ROOT / 'galerie-terminal.html').write_text(html, encoding='utf-8')
(SERVER / 'galerie-terminal.html').write_text(html, encoding='utf-8')
print(json.dumps({'url': 'http://127.0.0.1:8769/galerie-terminal.html', 'captures': len(scenes), 'engine_idle_poses': 2}))
