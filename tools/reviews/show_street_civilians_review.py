"""Native in-engine review: civilian bodies, gore and blood, CIVILS-U01-08."""
from pathlib import Path
import hashlib
import json
import re
from PIL import Image
from check_street_style_integration import changed_pixels, VIEW

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/street-defects-review-01'
ENGINE = OUT / 'engine08'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-civilians-review-08'
MAP = ROOT / 'assets/maps/prison-street-expanded-01'


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build():
    accepted = (DOC / 'acceptance.json').exists()
    acceptance = read(DOC / 'acceptance.json') if accepted else None
    checks = read(ENGINE / 'checks.json')
    proposal = read(DOC / 'proposal.json')
    assert checks['review'] == proposal['review'] == 'CIVILS-U01-08'
    assert checks['render_read_only'] and checks['camp_round_trip'] and checks['same_lighting']
    assert not checks['normal_integration'] and checks['native_cell'] == 64
    for file, digest in proposal['production_sha256'].items():
        if accepted and file == 'street.json':
            continue
        assert sha(MAP / file) == digest, file
    if accepted:
        for asset in acceptance['runtime_assets']:
            assert sha(ROOT / asset['file']) == asset['sha256']
        for file, digest in acceptance['approved_references'].items():
            assert sha(ENGINE / file) == digest
    # Removing the candidate overlay must reproduce the accepted current map.
    current = Image.open(ENGINE / 'map-current.png')
    candidate = Image.open(ENGINE / 'map-candidate.png')
    assert current.size == candidate.size == (1664, 4224)
    assert changed_pixels(current, Image.open(OUT / 'engine06/map-candidate.png')) == 0
    changes = {}
    pairs = {}
    for scene in checks['scenes']:
        name = scene['id']
        pair = [Image.open(ENGINE / f'{name}-{mode}.png').crop(VIEW) for mode in ['current', 'candidate']]
        changes[name] = changed_pixels(*pair)
        pairs[name] = pair
    # Both house interiors lie under their closed opaque roofs in this view.
    # Their extra bodies/blood must not bleed through the exterior roof art.
    assert changes['house-closed'] == 0, ('Corpses revealed through closed roofs', changes['house-closed'])
    assert all(changes[name] > 100 for name in ['house-open', 'shop-open', 'first-house-open'])
    selected = {
        'house-open': 'Logement nord · deux victimes à l’intérieur',
        'shop-open': 'Boutique · corps et traînée de sang',
        'first-house-open': 'Premier logement près de la prison',
        'burned-car': 'Véhicule incendié et rue sud',
        'north-crossroad': 'Carrefour nord · effondrement et victime',
        'warehouse': 'Entrepôt et façades détruites',
        'courtyard': 'Cour et local technique',
        'house-closed': 'Contrôle · corps masqués par les toits fermés',
    }
    sections = []
    for name, title in selected.items():
        for mode, image in zip(['current', 'candidate'], pairs[name]):
            image.save(OUT / f'r08-{name}-{mode}.png')
        sections.append(f'''<section id="{name}"><h2>{title}</h2>
<div class="buttons"><button data-scene="{name}" data-mode="current">Actuel</button><button data-scene="{name}" data-mode="candidate" aria-pressed="true">Avec les nouveaux décors</button></div>
<div class="scroll"><img id="view-{name}" src="r08-{name}-candidate.png" width="1218" height="704" alt="{title} à taille réelle"></div></section>''')
    batch = sha(ENGINE / 'map-candidate.png')[:12]
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Civils morts et sang · CIVILS-U01-08</title><style>
body{margin:22px;background:#11191d;color:#d4dcd9;font:17px Segoe UI,sans-serif}h1,h2{color:#d9c6a1}p{max-width:1100px;line-height:1.5}a{color:#dec496}
button{font:inherit;color:#ddd7c6;background:#25353c;border:1px solid #687c80;padding:9px 14px;cursor:pointer}button[aria-pressed=true]{background:#414537;border-color:#dcc69c}
.buttons,nav{display:flex;gap:12px;flex-wrap:wrap;margin:12px 0}.scroll{overflow:auto;border:1px solid #40525b;margin:12px 0}img{display:block;max-width:none}section{margin:32px 0}.note{color:#aebfbe}
#map{width:100%;height:min(78vh,900px);min-height:450px;background:#060b0e;border:1px solid #40525b;touch-action:none;cursor:grab;display:block}#status{align-self:center}
</style><h1>Corps civils et traces de violence</h1>
<p>CIVILS-U01-08 · proposition à valider · échelle native · rendu du moteur.</p>
<p>Cinq dessins de civils morts : abattu, lacérée, écrasé, démembré et carbonisé. Ils forment 18 nouveaux placements, dont cinq dans les trois intérieurs visitables. Deux décors de sang séparés servent aux 19 flaques et traînées associées aux scènes.</p>
<p>Les corps sont regroupés près des effondrements, des bâtiments touchés, du véhicule incendié et dans les pièces occupées avant l’attaque. Le dallage usé validé, les personnages et les vraies lumières restent ceux du jeu actuel. Les décors au sol n'ajoutent aucune collision.</p>
<nav><a href="#board">Dessins à taille native</a><a href="#overview">Carte entière</a><a href="#house-open">Logement nord</a><a href="#shop-open">Boutique</a><a href="#first-house-open">Premier logement</a><a href="#burned-car">Véhicule incendié</a><a href="#house-closed">Toits fermés</a></nav>
<section id="board"><h2>Les dessins, à leur taille de jeu</h2>
<p>Les codes servent à accepter ou corriger chaque dessin. Le civil déjà présent est inclus comme référence. Les variantes sont tournées à angle droit lors de leur placement, sans agrandissement.</p>
<div class="scroll"><img src="civilians-native-board.png" width="960" height="420" alt="Cinq corps et deux décors de sang, avec la référence existante, à taille native"></div></section>
<section id="overview"><h2>La proposition dans la carte</h2>
<p>Fais glisser pour parcourir la rue ; la molette zoome vers le pointeur. « Taille de jeu » conserve 64 px par case à un zoom de navigateur de 100 %.</p>
<div class="buttons"><button id="current">Carte actuelle</button><button id="candidate" aria-pressed="true">Avec les nouveaux décors</button><button id="fit">Toute la carte</button><button id="native">Taille de jeu · 64 px</button><span id="status">Chargement…</span></div>
<canvas id="map" aria-label="Carte entière, déplaçable et zoomable"></canvas>
<p><a href="engine08/map-candidate.png">Image entière proposée · 1664 × 4224 px</a> · <a href="engine08/map-current.png">Carte actuelle</a></p>
<p class="note">Les toits sont dans leur état initial : les cinq corps intérieurs restent cachés. Ils apparaissent dans les vues ouvertes ci-dessous. Cette vue d'ensemble révèle le décor pour la revue ; les captures de jeu conservent la vision réelle de six cases.</p></section>
<h2>Vues de jeu avec le personnage</h2><p>Chaque comparaison garde la même position, la même caméra et le même éclairage.</p>
__SECTIONS__
<p class="note">Ces dessins et leurs placements attendent ta validation avant leur intégration au lancement normal. La variante CIV-U01-09 n'a pas pu être générée et ne fait pas partie du lot.</p>
<script>
const canvas=document.getElementById('map'),ctx=canvas.getContext('2d'),maps={current:new Image(),candidate:new Image()},batch='__BATCH__';
let mode='candidate',zoom=1,ox=0,oy=0,w=0,h=0,drag=null,ready=0;
function paint(){const d=window.devicePixelRatio||1;ctx.setTransform(d,0,0,d,0,0);ctx.clearRect(0,0,w,h);ctx.imageSmoothingEnabled=zoom<1;if(maps[mode].complete&&maps[mode].naturalWidth)ctx.drawImage(maps[mode],ox,oy,1664*zoom,4224*zoom);document.getElementById('status').textContent=Math.round(zoom*100)+' % · '+(64*zoom).toFixed(1)+' px/case';}
function fit(){zoom=Math.min((w-24)/1664,(h-24)/4224);ox=(w-1664*zoom)/2;oy=(h-4224*zoom)/2;paint();}
function resize(){const r=canvas.getBoundingClientRect(),d=window.devicePixelRatio||1;w=r.width;h=r.height;canvas.width=Math.round(w*d);canvas.height=Math.round(h*d);fit();}
function zoomAt(value,x,y){const n=Math.max(.10,Math.min(3,value));ox=x-(x-ox)*n/zoom;oy=y-(y-oy)*n/zoom;zoom=n;paint();}
for(const name of ['current','candidate']){maps[name].onload=()=>{if(++ready===2)fit();};maps[name].onerror=()=>document.getElementById('status').textContent='Image indisponible';maps[name].src='engine08/map-'+name+'.png?v='+batch;document.getElementById(name).onclick=()=>{mode=name;for(const n of ['current','candidate'])document.getElementById(n).setAttribute('aria-pressed',String(n===mode));paint();};}
document.getElementById('fit').onclick=fit;document.getElementById('native').onclick=()=>zoomAt(1,w/2,h/2);
canvas.onwheel=e=>{e.preventDefault();const r=canvas.getBoundingClientRect();zoomAt(zoom*Math.exp(-e.deltaY*.0014),e.clientX-r.left,e.clientY-r.top);};
canvas.onpointerdown=e=>{drag={x:e.clientX,y:e.clientY,ox,oy};canvas.setPointerCapture(e.pointerId);canvas.style.cursor='grabbing';};
canvas.onpointermove=e=>{if(drag){ox=drag.ox+e.clientX-drag.x;oy=drag.oy+e.clientY-drag.y;paint();}};
canvas.onpointerup=canvas.onpointercancel=()=>{drag=null;canvas.style.cursor='grab';};
document.querySelectorAll('button[data-scene]').forEach(b=>b.onclick=()=>{const id=b.dataset.scene;document.getElementById('view-'+id).src='r08-'+id+'-'+b.dataset.mode+'.png?v='+batch;document.querySelectorAll('button[data-scene="'+id+'"]').forEach(o=>o.setAttribute('aria-pressed',String(o===b)));});
window.addEventListener('resize',resize);resize();
</script></html>'''.replace('__SECTIONS__', '\n'.join(sections)).replace('__BATCH__', batch)
    for file in list(OUT.glob('r08-*.png')) + [OUT / 'civilians-native-board.png'] + list(ENGINE.glob('map-*.png')):
        rel = file.relative_to(OUT).as_posix()
        html = html.replace(rel + '"', rel + '?v=' + batch + '"')
    if accepted:
        html = html.replace('proposition à valider', 'lot validé et intégré')
        html = html.replace('Carte actuelle', 'Avant les ajouts').replace('>Actuel</button>', '>Avant les ajouts</button>')
        html = html.replace('Avec les nouveaux décors', 'Décors intégrés')
        html = html.replace('Ces dessins et leurs placements attendent ta validation avant leur intégration au lancement normal.',
                            'Ces dessins et leurs placements sont validés et actifs au lancement normal.')
    if (ROOT / 'artifacts/art-quality-audit-01/index.html').exists():
        html = html.replace('<nav>', '<p><a href="../art-quality-audit-01/index.html">Audit demandé : comparer les corps, personnages, armes et monstres à taille native</a></p><nav>', 1)
    (OUT / 'index.html').write_text(html, encoding='utf-8')
    for file in re.findall(r'(?:src|href)="([^"#]+\.png[^\"]*)"', html):
        assert (OUT / file.split('?')[0]).is_file(), file
    checks.update(review_status='accepted_and_integrated' if accepted else 'candidate_not_integrated',
                  changed_pixels=changes, current_normal_map_matches_accepted_paving=True,
                  roof_closed_hides_candidates=True, roof_open_reveals_candidates=True,
                  production_hashes_unchanged=not accepted, proposal=proposal,
                  review_page='artifacts/street-defects-review-01/index.html')
    (DOC / 'checks.json').write_text(json.dumps(checks, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(review=checks['review'], scenes=len(changes), changed_pixels=changes,
                          normal_assets_unchanged=True, closed_roof_changed_pixels=changes['house-closed'])))


if __name__ == '__main__':
    build()
