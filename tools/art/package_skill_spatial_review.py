"""Lossless native tiles and a freely targetable browser mirror of Rust geometry."""
import hashlib
import json
import sys
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
OUT=BASE/(sys.argv[1] if len(sys.argv)>1 else 'skill-spatial-review-01')
assert OUT.resolve().is_relative_to(BASE.resolve())
NATIVE=OUT/'native'
metadata=json.loads((NATIVE/'capture.json').read_text(encoding='utf8'))
verified=[]
for item in metadata['sprites']+metadata['captures']:
    raw=NATIVE/(item['name']+'.rgba')
    data=raw.read_bytes()
    assert len(data)==item['width']*item['height']*4
    path=raw.with_suffix('.png')
    Image.frombytes('RGBA',(item['width'],item['height']),data).save(path,compress_level=3)
    assert Image.open(path).convert('RGBA').tobytes()==data
    verified.append({'file':path.name,'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'rgba_identical':True})
cat=json.loads((ROOT/'assets/skills/agent/catalogue.json').read_text(encoding='utf8'))
metadata['skills']=[s for s in cat['skills'] if s.get('centred_damage') or any(p.get('spatial_review')=='ZONE-GROWTH02' for p in s['parameters'].values())]
metadata['labels']=cat['labels']
script='const DATA='+json.dumps(metadata,ensure_ascii=False).replace('</','<\\/')+';\n'+(ROOT/'tools/art/spatial_growth_player.js').read_text(encoding='utf8')
(OUT/'player.js').write_text(script,encoding='utf8')
html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width"><title>Portées, zones et vision · Project RL</title>
<style>html{color-scheme:dark}*{box-sizing:border-box}body{margin:0;background:#0c131a;color:#d9e0e4;font:16px system-ui,sans-serif}main{max-width:1500px;margin:auto;padding:22px}h1{font-size:26px;margin:0 0 10px}h2{font-size:21px;margin:0 0 10px}h3{font-size:17px;margin:18px 0 10px}p{margin:8px 0;line-height:1.5}.muted{color:#a5b6c1}.layout{display:grid;grid-template-columns:minmax(500px,1fr) 340px;gap:22px;margin-top:20px}#viewport{height:740px;overflow:auto;background:#080d12;border:1px solid #465360}#world{display:block;width:1984px;height:1600px}aside{background:#141e27;padding:18px;border:1px solid #34424f}label{display:block;margin:14px 0 6px}select,button{font:inherit;border:1px solid #53606c;border-radius:4px;background:#202e39;color:#e4ebef;padding:8px}select{width:100%}input[type=range]{width:100%;accent-color:#69c9e9}button{cursor:pointer}button:disabled{opacity:.45;cursor:default}.controls{display:flex;flex-wrap:wrap;gap:8px;margin:10px 0}.ok{color:#8edab7}.bad{color:#f8a08c}#mini{display:block;width:248px;height:200px;margin:16px auto 6px;cursor:crosshair}details{margin:22px 0}a{color:#8aceef}.examples{display:grid;grid-template-columns:repeat(4,1fr);gap:14px}.examples a{background:#17232c;padding:14px;line-height:1.5}em{color:#b9c7ce}@media(max-width:950px){.layout{grid-template-columns:1fr}#viewport{height:600px}.examples{grid-template-columns:repeat(2,1fr)}}</style>
<main><h1>Portées, zones et vision</h1><p>ZONE-GROWTH02 · paliers provisoires du catalogue · ZONE-VISION02 · cercle de vision validé à 12 cases</p><p class="muted">Cliquez sur le plateau pour placer le centre. Les cases sont affichées en 64 px réels : déplacez la vue avec les barres de défilement ou la petite carte. Les flèches et ZQSD déplacent le joueur.</p>
<div class="layout"><section><div id="viewport"><canvas id="world" width="1984" height="1600"></canvas></div><div class="controls"><button id="centre">Centrer sur le joueur</button><button id="test">Tester le placement</button><button id="reset">Réinitialiser</button><span id="test-count"></span></div><p id="valid"></p><p id="player-zone" class="muted"></p><p class="muted">Ce laboratoire teste la portée et l’emprise : il ne lance pas une nouvelle animation de combat. Les cases et acteurs hors champ restent masqués. Le bestiaire actuel sera adapté après les animations.</p></section>
<aside><label for="skill">Compétence</label><select id="skill"></select><label for="learned">Rang appris : <span id="learned-value">10</span></label><input id="learned" type="range" min="1" max="10" value="10"><label for="bonus">Bonus d’équipement : <span id="bonus-value">3</span></label><input id="bonus" type="range" min="0" max="10" value="3"><p id="effective"></p><label for="vision">Cercle de vision</label><select id="vision"><option value="8">Référence · 8 cases</option><option value="12" selected>Validé · 12 cases</option></select><label for="scene">Décor</label><select id="scene"><option value="0">Salle ouverte</option><option value="1">Obstacle isolé</option><option value="2">Cloison complète</option><option value="3">Couloir puis pièce</option></select><label for="direction">Direction du cône, mur ou de la vague</label><select id="direction"><option value="0">Droite</option><option value="1">Gauche</option><option value="2">Haut</option><option value="3">Bas</option></select><canvas id="mini" width="248" height="200"></canvas><p class="muted">Vue d’ensemble · cliquez pour déplacer la caméra.</p><h2 id="title"></h2><p><em id="rp"></em></p><h3>Effets actuels de portée et de zone</h3><div id="present"></div><h3 id="nexttitle">Prochain rang</h3><div id="next"></div></aside></div>
<details><summary>Captures du rendu natif du laboratoire</summary><p>Chaque image conserve les cases en 64 px, sans agrandissement d’un dessin 32 px du joueur. Ouvrez-la pour examiner toute la scène.</p><div class="examples">CAPTURES</div></details>
<p>Les 46 compétences concernées ont leurs nouveaux paliers dans le catalogue de la fenêtre U. Les dégâts, coûts, durées, recharges, règles d’armes actives et impacts courts de mêlée sont conservés. Les grenades et mines gardent leurs passifs de progression dédiés.</p><p><strong>Le rayon de vision 12 est validé et intégré aux nouvelles parties et au laboratoire. Les anciennes sauvegardes gardent leur rayon de huit cases.</strong> Les effets de combat de la refonte restent à raccorder selon les animations acceptées.</p><p><a href="progressions.json">Relevé des paramètres modifiés</a> · <a href="../skill-animation-review-71/cote-a-cote.html">Animation d’Explosion étendue</a></p><script src="player.js"></script></main></html>'''
labels=['Vision 8','Vision 12','Obstacle isolé','Cloison complète','Couloir puis pièce','Premier rang','Charge · impact court','Échange de position']
links=''.join(f'<a href="native/{item["name"]}.png">{label}<br>Rang {item["state"]["rank"]} · vision {item["state"]["vision"]}</a>' for item,label in zip(metadata['captures'],labels))
html=html.replace('Les dégâts, coûts, durées, recharges, règles d’armes actives et impacts courts de mêlée sont conservés.',
    'Les dégâts de base, coûts, durées, recharges, règles d’armes actives et impacts courts de mêlée sont conservés. AOE-FALLOFF01 applique la dégressivité aux dégâts directs des zones centrées concernées ; les pourcentages sur les cases montrent la part conservée avant les défenses.')
(OUT/'index.html').write_text(html.replace('CAPTURES',links),encoding='utf8')
(OUT/'native-packing-verification.json').write_text(json.dumps({'tile':64,'images':verified,'geometry_fixtures':len(metadata['fixtures']),'no_resampling':True},indent=2)+'\n',encoding='utf8')
print(f'{len(verified)} images natives vérifiées, {len(metadata["fixtures"])} scènes de géométrie exportées')
