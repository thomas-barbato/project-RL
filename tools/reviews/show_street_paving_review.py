"""PAV-U01-06: actual-engine paving comparison in the existing street review."""
from pathlib import Path
import hashlib
import json
import re
from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/street-defects-review-01'
ENGINE = OUT / 'engine06'
ART = ROOT / 'assets/source-art/first-sector-review-01/u01-corrections-r06'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-paving-review-06'
VIEW = (334, 78, 1552, 782)


def changed(a, b):
    ch = ImageChops.difference(a.convert('RGB'), b.convert('RGB')).split()
    return sum(ImageChops.lighter(ImageChops.lighter(ch[0], ch[1]), ch[2]).histogram()[1:])


def build():
    if (OUT/'engine08/checks.json').exists():
        from show_street_civilians_review import build as latest
        latest()
        return
    accepted = (DOC / 'acceptance.json').exists()
    acceptance = json.loads((DOC / 'acceptance.json').read_text(encoding='utf-8')) if accepted else None
    checks = json.loads((ENGINE / 'checks.json').read_text(encoding='utf-8'))
    source = json.loads((ART / 'composition-checks.json').read_text(encoding='utf-8'))
    assert checks['review'] == source['review'] == 'PAV-U01-06'
    assert checks['native_cell'] == 64 and checks['render_read_only'] and checks['camp_round_trip']
    assert checks['same_lighting'] and not checks['normal_integration']
    for file, digest in source['active_sha256'].items():
        if accepted and file in ['street.png', 'street.json']:
            continue
        assert hashlib.sha256((ROOT / 'assets/maps/prison-street-expanded-01' / file).read_bytes()).hexdigest() == digest
    if accepted:
        background = acceptance['runtime_background']
        assert hashlib.sha256((ROOT / background['file']).read_bytes()).hexdigest() == background['sha256']
        for file, digest in acceptance['approved_references'].items():
            assert hashlib.sha256((ENGINE / file).read_bytes()).hexdigest() == digest
    maps = {m: Image.open(ENGINE / f'map-{m}.png').convert('RGB') for m in ['current', 'candidate']}
    assert all(im.size == (1664, 4224) for im in maps.values())
    assert changed(maps['current'], Image.open(OUT / 'engine05/map-normal.png')) == 0, 'Current scenery regression'
    labels = {'north-street': 'Rue nord', 'middle-street': 'Rue centrale',
              'courtyard': 'Cour et machines', 'warehouse': 'Entrepôt et trottoir',
              'south-street': 'Jonction avec la rue de départ'}
    sections = []
    differences = {}
    for scene in checks['scenes']:
        name = scene['id']
        pair = [Image.open(ENGINE / f'{name}-{mode}.png').crop(VIEW) for mode in ['current', 'candidate']]
        differences[name] = changed(*pair)
        if name not in labels:
            continue
        for mode, im in zip(['current', 'candidate'], pair):
            im.save(OUT / f'r06-{name}-{mode}.png')
        sections.append(f'''<section id="{name}"><h2>{labels[name]}</h2>
<div class="buttons"><button data-view="{name}" data-mode="current">Sol actuel</button><button data-view="{name}" data-mode="candidate" aria-pressed="true">Sol corrigé</button></div>
<div class="scroll"><img id="view-{name}" src="r06-{name}-candidate.png" width="1218" height="704" alt="{labels[name]} à taille native"></div></section>''')
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 17)
    # Native crop from real lit map exports; no enlargement or synthetic relight.
    box = (768, 448, 1152, 704)
    board = Image.new('RGB', (800, 305), '#11191d')
    draw = ImageDraw.Draw(board)
    for x, mode, title in [(10, 'current', 'Actuel'), (410, 'candidate', 'PAV-U01-06 · corrigé')]:
        draw.text((x, 9), title, font=font, fill='#d9c6a1')
        board.paste(maps[mode].crop(box), (x, 39))
    board.save(OUT / 'paving-native-comparison.png')
    batch = hashlib.sha256((ENGINE / 'map-candidate.png').read_bytes()).hexdigest()[:12]
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Sol de la rue · PAV-U01-06</title><style>
body{margin:22px;background:#11191d;color:#d4dcd9;font:17px Segoe UI,sans-serif}h1,h2{color:#d9c6a1}p{max-width:1100px;line-height:1.5}a{color:#dec496}
button{font:inherit;color:#ddd7c6;background:#25353c;border:1px solid #687c80;padding:9px 14px;cursor:pointer}button[aria-pressed=true]{background:#414537;border-color:#dcc69c}
.buttons,nav{display:flex;gap:12px;flex-wrap:wrap;margin:12px 0}.scroll{overflow:auto;border:1px solid #40525b;margin:12px 0}img{display:block;max-width:none}section{margin:32px 0}.note{color:#aebfbe}
#map{width:100%;height:min(78vh,900px);min-height:450px;background:#060b0e;border:1px solid #40525b;touch-action:none;cursor:grab;display:block}#status{align-self:center}
</style><h1>Le sol de la rue · correction ciblée</h1>
<p>PAV-U01-06 · proposition à valider · cases de 64 px · captures du moteur.</p>
<p>Les joints intérieurs parasites disparaissent. Les séparations entre dalles sont d'intensité variable : quelques joints restent lisibles, beaucoup sont interrompus ou presque effacés. Leur répartition est irrégulière et reste fixe pendant le jeu. L'herbe n'est plus intégrée au morceau de sol répété : quelques petites touffes sont placées séparément, en groupes irréguliers près des murs et des bordures.</p>
<p>L'usure grimdark déjà validée et les vraies lumières sont présentes dans les deux versions. La correction porte sur le dallage de l'extension de la rue ; la composition du départ de la prison et les sols intérieurs sont conservés.</p>
<nav><a href="#comparison">Détail à taille native</a><a href="#overview">Carte entière</a><a href="#north-street">Rue nord</a><a href="#middle-street">Rue centrale</a><a href="#courtyard">Cour</a><a href="#warehouse">Entrepôt</a></nav>
<section id="comparison"><h2>Comparaison à taille réelle</h2><div class="scroll"><img src="paving-native-comparison.png" width="800" height="305" alt="Même portion de rue, avant et après, sans agrandissement"></div></section>
<section id="overview"><h2>La correction dans toute la carte</h2><p>Fais glisser la carte pour la parcourir ; la molette zoome vers le pointeur. « Taille de jeu » conserve 64 px par case à un zoom de navigateur de 100 %.</p>
<div class="buttons"><button id="current">Sol actuel</button><button id="candidate" aria-pressed="true">Sol corrigé</button><button id="fit">Toute la carte</button><button id="native">Taille de jeu · 64 px</button><span id="status">Chargement…</span></div>
<canvas id="map" aria-label="Carte complète, déplaçable et zoomable"></canvas>
<p><a href="engine06/map-candidate.png">Image entière corrigée · 1664 × 4224 px</a> · <a href="engine06/map-current.png">Sol actuel</a></p>
<p class="note">La vue d'ensemble montre tout le décor pour la revue. Les captures de jeu ci-dessous conservent le cercle de vision réel de six cases.</p></section>
<h2>Dans le jeu, avec le personnage</h2><p>Même position, caméra, éclairage et traitement de surface pour chaque comparaison.</p>
__SECTIONS__
<p class="note">Cette correction attend ta validation avant son activation normale. Bâtiments, portes, collisions, personnages et règles de jeu sont conservés.</p>
<script>
const canvas=document.getElementById('map'),ctx=canvas.getContext('2d'),maps={current:new Image(),candidate:new Image()},batch='__BATCH__';
let mode='candidate',zoom=1,ox=0,oy=0,w=0,h=0,drag=null,ready=0;
function paint(){const d=window.devicePixelRatio||1;ctx.setTransform(d,0,0,d,0,0);ctx.clearRect(0,0,w,h);ctx.imageSmoothingEnabled=zoom<1;if(maps[mode].complete&&maps[mode].naturalWidth)ctx.drawImage(maps[mode],ox,oy,1664*zoom,4224*zoom);document.getElementById('status').textContent=Math.round(zoom*100)+' % · '+(64*zoom).toFixed(1)+' px/case';}
function fit(){zoom=Math.min((w-24)/1664,(h-24)/4224);ox=(w-1664*zoom)/2;oy=(h-4224*zoom)/2;paint();}
function resize(){const r=canvas.getBoundingClientRect(),d=window.devicePixelRatio||1;w=r.width;h=r.height;canvas.width=Math.round(w*d);canvas.height=Math.round(h*d);fit();}
function zoomAt(value,x,y){const n=Math.max(.10,Math.min(3,value));ox=x-(x-ox)*n/zoom;oy=y-(y-oy)*n/zoom;zoom=n;paint();}
for(const name of ['current','candidate']){maps[name].onload=()=>{if(++ready===2)fit();};maps[name].onerror=()=>document.getElementById('status').textContent='Image indisponible';maps[name].src='engine06/map-'+name+'.png?v='+batch;document.getElementById(name).onclick=()=>{mode=name;for(const n of ['current','candidate'])document.getElementById(n).setAttribute('aria-pressed',String(n===mode));paint();};}
document.getElementById('fit').onclick=fit;document.getElementById('native').onclick=()=>zoomAt(1,w/2,h/2);
canvas.onwheel=e=>{e.preventDefault();const r=canvas.getBoundingClientRect();zoomAt(zoom*Math.exp(-e.deltaY*.0014),e.clientX-r.left,e.clientY-r.top);};
canvas.onpointerdown=e=>{drag={x:e.clientX,y:e.clientY,ox,oy};canvas.setPointerCapture(e.pointerId);canvas.style.cursor='grabbing';};
canvas.onpointermove=e=>{if(drag){ox=drag.ox+e.clientX-drag.x;oy=drag.oy+e.clientY-drag.y;paint();}};
canvas.onpointerup=canvas.onpointercancel=()=>{drag=null;canvas.style.cursor='grab';};
document.querySelectorAll('button[data-view]').forEach(b=>b.onclick=()=>{const id=b.dataset.view;document.getElementById('view-'+id).src='r06-'+id+'-'+b.dataset.mode+'.png?v='+batch;document.querySelectorAll('button[data-view="'+id+'"]').forEach(o=>o.setAttribute('aria-pressed',String(o===b)));});
window.addEventListener('resize',resize);resize();
</script></html>'''.replace('__SECTIONS__', '\n'.join(sections)).replace('__BATCH__', batch)
    for file in list(OUT.glob('r06-*.png')) + [OUT / 'paving-native-comparison.png'] + list(ENGINE.glob('map-*.png')):
        rel = file.relative_to(OUT).as_posix()
        html = html.replace(rel + '"', rel + '?v=' + batch + '"')
    if accepted:
        html = html.replace('proposition à valider', 'rendu validé et intégré')
        html = html.replace('Sol actuel', 'Ancien sol').replace('Sol corrigé', 'Sol intégré')
        html = html.replace('>Actuel</', '>Ancien sol</')
        html = html.replace('Cette correction attend ta validation avant son activation normale.',
                            'Ce sol est validé et actif dans le lancement normal du jeu.')
    (OUT / 'index.html').write_text(html, encoding='utf-8')
    for file in re.findall(r'(?:src|href)="([^"#]+\.png[^\"]*)"', html):
        assert (OUT / file.split('?')[0]).is_file(), file
    checks.update(status='accepted_and_integrated' if accepted else 'candidate_not_integrated', composition=source,
                  changed_pixels=differences, active_map_matches_approved_normal_capture=True,
                  normal_runtime_assets_unchanged=not accepted, review_page='artifacts/street-defects-review-01/index.html')
    DOC.mkdir(parents=True, exist_ok=True)
    (DOC / 'checks.json').write_text(json.dumps(checks, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(review=checks['review'], scenes=len(differences), changed_pixels=differences,
                          normal_assets_unchanged=not accepted, integrated=accepted, map_size=[1664, 4224]), ensure_ascii=False))


if __name__ == '__main__':
    build()
