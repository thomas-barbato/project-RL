"""Whole-map GPU proposal with a native pan/zoom overview and real play views."""
from pathlib import Path
import hashlib
import json
import re
from PIL import Image, ImageChops

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/street-defects-review-01'
ENGINE=OUT/'engine04'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-defects-review-04'
VIEW=(334,78,1552,782)


def build():
    if (OUT/'engine08/checks.json').exists():
        from show_street_civilians_review import build as latest
        latest()
        return
    if (OUT/'engine06/checks.json').exists():
        from show_street_paving_review import build as latest
        latest()
        return
    checks=json.loads((ENGINE/'checks.json').read_text(encoding='utf-8'))
    assert checks['review']=='STYLE-MAP-U01-04'
    assert checks['native_cell']==64 and checks['render_read_only'] and checks['camp_round_trip']
    assert checks['same_lighting'] and checks['living_actor_assets_unchanged']
    for mode in ['current','candidate']:
        assert Image.open(ENGINE/f'map-{mode}.png').size==(1664,4224)
    labels={'gate':'Sortie de la prison','south-street':'Rue sud',
            'courtyard':'Cour et machines','middle-street':'Rue centrale',
            'house-closed':'Maison fermée et toiture','north-street':'Rue nord',
            'warehouse':'Entrepôt','north-exit':'Passage vers la prochaine carte',
            'house-open':'Maison ouverte'}
    sections=[]
    changed={}
    for scene in checks['scenes']:
        name=scene['id'];pair=[]
        assert scene['cell']==64 and scene['render_read_only']
        for mode in ['current','candidate']:
            im=Image.open(ENGINE/f'{name}-{mode}.png').convert('RGB').crop(VIEW)
            im.save(OUT/f'r04-{name}-{mode}.png');pair.append(im)
        diff=ImageChops.difference(*pair).split()
        maximum=ImageChops.lighter(ImageChops.lighter(diff[0],diff[1]),diff[2])
        changed[name]=sum(maximum.histogram()[3:])
        assert changed[name]>100,name
        sections.append(f'''<section id="{name}"><h2>{labels[name]}</h2>
<div class="buttons"><button data-view="{name}" data-mode="current">Actuel</button><button data-view="{name}" data-mode="candidate" aria-pressed="true">Essai de style</button></div>
<div class="scroll"><img class="detail" id="view-{name}" data-scene="{name}" src="r04-{name}-candidate.png" width="1218" height="704" alt="{labels[name]} à taille de jeu"></div></section>''')
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Rue complète · essai de style grimdark</title>
<style>
body{margin:22px;background:#11191d;color:#d4dcd9;font:17px Segoe UI,sans-serif}h1,h2{color:#d9c6a1}
p{max-width:1120px;line-height:1.5}a{color:#dec496}button{font:inherit;color:#ddd7c6;background:#25353c;border:1px solid #687c80;padding:9px 14px;cursor:pointer}
button[aria-pressed=true]{background:#414537;border-color:#dcc69c}.buttons{display:flex;gap:9px;flex-wrap:wrap;margin:12px 0}
.scroll{overflow:auto;border:1px solid #40525b;margin:12px 0}img{display:block;max-width:none}section{margin:32px 0}
#map{width:100%;height:min(78vh,900px);min-height:450px;background:#060b0e;border:1px solid #40525b;touch-action:none;cursor:grab;display:block}
nav{display:flex;gap:16px;flex-wrap:wrap}.note{color:#aebfbe}#status{margin-left:10px;align-self:center}
</style>
<h1>La rue entière · essai de style</h1>
<p>STYLE-MAP-U01-04 · 26 × 66 cases · 64 px par case · rendu du moteur.</p>
<p>Le traitement des matières est étendu aux rues, murs, bâtiments, intérieurs, toits et voiture. Le plan, les passages, les personnages et les vraies lumières restent ceux du jeu actuel. Cet aperçu permet de juger l'ensemble avant validation.</p>
<p class="note">L'usure est commune à toutes les surfaces : peinture terne, traces mates et éclats irréguliers. Les bâtiments et les sols gardent ici leur géométrie d'origine ; leur finition est calculée dans le moteur avec une texture de matière créée pour cet essai. Il s'agit d'une proposition de rendu de surface ; les bâtiments existants ne sont pas tous redessinés.</p>
<nav><a href="#overview">Carte entière</a><a href="#gate">Prison</a><a href="#south-street">Rue sud</a><a href="#courtyard">Cour</a><a href="#middle-street">Rue centrale</a><a href="#house-closed">Toits</a><a href="#warehouse">Entrepôt</a><a href="#house-open">Intérieur</a></nav>
<section id="overview"><h2>Vue d'ensemble</h2>
<p>Déplace la carte en la faisant glisser ; la molette zoome vers le pointeur. « Taille de jeu » affiche les cases à 64 px lorsque le navigateur est à 100 %.</p>
<div class="buttons"><button id="current" aria-pressed="false">Carte actuelle</button><button id="candidate" aria-pressed="true">Carte avec l'essai de style</button><button id="fit">Toute la carte</button><button id="native">Taille de jeu · 64 px</button><span id="status">Chargement…</span></div>
<canvas id="map" aria-label="Carte complète, déplaçable et zoomable"></canvas>
<p><a href="engine04/map-candidate.png">Ouvrir l'image entière · 1664 × 4224 px</a> · <a href="engine04/map-current.png">Image actuelle</a></p>
<p class="note">Cette vue d'ensemble montre volontairement tout le décor. Dans les vues de jeu ci-dessous, le cercle réel de vision reste de six cases. La vue d'ensemble ne change pas les ennemis que le personnage peut voir.</p></section>
<h2>Vues à l'échelle de jeu</h2><p>Chaque paire utilise la même position, la même caméra et le même éclairage. Les corps proposés précédemment sont montrés près de la prison ; la station médicale refusée est absente.</p>
__SECTIONS__
<p class="note">Proposition visuelle, non activée dans le lancement normal. Les ressources séparées, leurs sources et leurs placements sont conservés. Aucun changement de collision, de quête ou de combat.</p>
<script>
const canvas=document.getElementById('map'),ctx=canvas.getContext('2d');
const maps={current:new Image(),candidate:new Image()};
const batch='__BATCH__';
let mode='candidate',zoom=1,ox=0,oy=0,w=0,h=0,drag=null,ready=0;
function paint(){
  const dpr=window.devicePixelRatio||1;
  ctx.setTransform(dpr,0,0,dpr,0,0);ctx.clearRect(0,0,w,h);ctx.imageSmoothingEnabled=zoom<1;
  if(maps[mode].complete&&maps[mode].naturalWidth)ctx.drawImage(maps[mode],ox,oy,1664*zoom,4224*zoom);
  document.getElementById('status').textContent=Math.round(zoom*100)+' % · '+(64*zoom).toFixed(1)+' px/case';
}
function fit(){zoom=Math.min((w-24)/1664,(h-24)/4224);ox=(w-1664*zoom)/2;oy=(h-4224*zoom)/2;paint();}
function resize(){const r=canvas.getBoundingClientRect(),d=window.devicePixelRatio||1;w=r.width;h=r.height;canvas.width=Math.round(w*d);canvas.height=Math.round(h*d);fit();}
function zoomAt(value,x,y){const next=Math.max(.10,Math.min(3,value));ox=x-(x-ox)*next/zoom;oy=y-(y-oy)*next/zoom;zoom=next;paint();}
for(const name of ['current','candidate']){maps[name].onload=()=>{ready++;if(ready===2)fit();};maps[name].onerror=()=>document.getElementById('status').textContent='Image indisponible';maps[name].src='engine04/map-'+name+'.png?v='+batch;}
for(const name of ['current','candidate'])document.getElementById(name).onclick=()=>{mode=name;document.getElementById('current').setAttribute('aria-pressed',String(mode==='current'));document.getElementById('candidate').setAttribute('aria-pressed',String(mode==='candidate'));paint();};
document.getElementById('fit').onclick=fit;
document.getElementById('native').onclick=()=>zoomAt(1,w/2,h/2);
canvas.onwheel=e=>{e.preventDefault();const r=canvas.getBoundingClientRect();zoomAt(zoom*Math.exp(-e.deltaY*.0014),e.clientX-r.left,e.clientY-r.top);};
canvas.onpointerdown=e=>{drag={x:e.clientX,y:e.clientY,ox,oy};canvas.setPointerCapture(e.pointerId);canvas.style.cursor='grabbing';};
canvas.onpointermove=e=>{if(drag){ox=drag.ox+e.clientX-drag.x;oy=drag.oy+e.clientY-drag.y;paint();}};
canvas.onpointerup=canvas.onpointercancel=()=>{drag=null;canvas.style.cursor='grab';};
document.querySelectorAll('button[data-view]').forEach(b=>b.onclick=()=>{const id=b.dataset.view;document.getElementById('view-'+id).src='r04-'+id+'-'+b.dataset.mode+'.png?v='+batch;document.querySelectorAll('button[data-view="'+id+'"]').forEach(o=>o.setAttribute('aria-pressed',String(o===b)));});
window.addEventListener('resize',resize);resize();
</script></html>'''.replace('__SECTIONS__','\n'.join(sections)).replace('__BATCH__',hashlib.sha256((ENGINE/'map-candidate.png').read_bytes()).hexdigest()[:12])
    # Version the engine images without changing the native source pixels.
    for file in list(OUT.glob('r04-*.png'))+list(ENGINE.glob('map-*.png')):
        relative=file.relative_to(OUT).as_posix()
        digest=hashlib.sha256(file.read_bytes()).hexdigest()[:12]
        # Static HTML references; dynamic views use this stable current batch.
        page=page.replace(relative+'"',relative+'?v='+digest+'"')
    if (DOC/'acceptance.json').exists():
        page=page.replace('Rue complète · essai de style grimdark','Rue complète · rendu validé')
        page=page.replace('La rue entière · essai de style','La rue entière · rendu validé')
        page=page.replace('Cet aperçu permet de juger l\'ensemble avant validation.','Le rendu présenté est validé et actif dans le jeu normal sur cette rue.')
        page=page.replace('Carte actuelle','Ancienne carte').replace('Carte avec l\'essai de style','Carte validée')
        page=page.replace('>Actuel</button>','>Ancien rendu</button>').replace('>Essai de style</button>','>Rendu validé</button>')
        page=page.replace('Les corps proposés précédemment','Les corps validés')
        page=page.replace('Proposition visuelle, non activée dans le lancement normal.','Rendu validé et activé sur la rue étendue. Les captures avant/après conservent la comparaison présentée.')
    (OUT/'index.html').write_text(page,encoding='utf-8')
    for path in re.findall(r'(?:src|href)="([^"]+\.png[^\"]*)"',page):
        assert (OUT/path.split('?')[0]).is_file(),path
    DOC.mkdir(parents=True,exist_ok=True)
    checks['changed_pixels']=changed
    checks['review_page']='artifacts/street-defects-review-01/index.html'
    (DOC/'capture-checks.json').write_text(json.dumps(checks,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'review':checks['review'],'native_map':[1664,4224],'scenes':len(sections),'changed_pixels':changed}))


if __name__=='__main__':build()
