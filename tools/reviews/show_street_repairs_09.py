"""Current U01 review: native GPU before/after, full scene and contact audit."""
from pathlib import Path
from html.parser import HTMLParser
from urllib.parse import unquote,urlsplit
import json
import hashlib
import shutil
from PIL import Image,ImageDraw

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/street-repairs-review-09'
PAGE=ROOT/'artifacts/street-defects-review-01/index.html'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-repairs-review-09'
ART=ROOT/'assets/source-art/first-sector-review-01/u01-repairs-r09'
PREFIX='../street-repairs-review-09/'
def read(p):return json.loads(p.read_text('utf-8'))
def write(p,v):p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def build():
    checks=read(OUT/'engine/checks.json');assert checks['collision_repairs']==63
    scenes=[('warehouse','STYLE-U01-09 · Entrepôt en L'),('road14','ROAD-U01-09 · Route en Y 14'),
        ('crater16','COL-U01-09 · Case (14,16)'),('van32','COL-U01-09 · Voiture (15,32)'),
        ('rubble-west','COL-U01-09 · Cases (2,30) et (2,31)'),('rubble40','COL-U01-09 · Case (15,40)'),
        ('junction','JONC-U01-09 · Raccord Y 48–49'),('gate','Départ sud conservé')]
    parts=['''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>U01 · collisions, style et raccords</title><style>body{background:#11191d;color:#d4ddda;font:17px Segoe UI,sans-serif;margin:24px;line-height:1.55}h1,h2,h3{color:#decca7}a{color:#d9c18c}p{max-width:1120px}nav,.buttons{display:flex;gap:14px;flex-wrap:wrap}button{font:inherit;background:#263632;color:#e0d3b6;border:1px solid #64706a;padding:8px 14px;cursor:pointer}button[aria-pressed=true]{background:#4c5039;border-color:#d8c48f}.scroll{overflow:auto;max-width:100%;border:1px solid #43514b;margin:16px 0}img{display:block;max-width:none}.map{height:800px}.note{color:#adbab4}section{margin:36px 0}</style>
<h1>U01-REPAIRS-09 · circulation et raccords</h1><p><strong>63 corrections de collision sont intégrées.</strong> Les abords signalés sont praticables, la carrosserie et les autres obstacles pleins restent bloquants. Les parties précédentes adoptent la correction après vérification, sans tour consommé ; une rue encore différée l’adopte à sa première entrée. Les sauvegardes du joueur ne sont pas modifiées par cette revue.</p>
<p><strong>Les nouveaux pixels ci-dessous attendent ton accord.</strong> Le bâtiment bleu conserve sa forme et son mur sud, avec des teintes plus sourdes et des contours moins uniformes. La texture de route ne répète plus une bordure et ses grilles au milieu de la chaussée. Le raccord sud reprend des haies entières et remplace le fragment coupé de l’angle nord-est par une annexe complète issue du kit existant.</p>
<nav><a href="#warehouse">Entrepôt</a><a href="#road14">Route</a><a href="#junction">Jonction</a><a href="#contacts">Collisions</a><a href="#map">Carte entière</a></nav>''']
    for key,title in scenes:
        for suffix in ['current','candidate']:
            frame=Image.open(OUT/'engine'/f'{key}-{suffix}.png').convert('RGB')
            frame.crop((334,50,1552,894)).save(OUT/f'{key}-{suffix}-native.png')
        parts.append(f'<section id="{key}"><h2>{title}</h2><div class="buttons"><button data-img="{key}-image" data-src="{PREFIX}{key}-current-native.png" aria-pressed="false">Décor actuel</button><button data-img="{key}-image" data-src="{PREFIX}{key}-candidate-native.png" aria-pressed="true">Correction proposée</button></div><div class="scroll"><img id="{key}-image" src="{PREFIX}{key}-candidate-native.png" width="1218" height="844" loading="lazy" alt="Comparaison native dans le moteur"></div><p class="note">Même position, même cercle de six cases et même personnage. <a href="{PREFIX}engine/{key}-candidate.png">Capture complète</a></p></section>')
    parts.append(f'''<section id="map"><h2>Carte entière éclairée · mêmes pixels natifs</h2><p>Cette vue d’ensemble de revue retire seulement le masque de vision pour examiner les raccords. Le jeu conserve le cercle de six cases montré dans les captures ci-dessus. Les acteurs ne figurent pas dans cette exportation du décor.</p><div class="buttons"><button data-img="map-image" data-src="{PREFIX}engine/map-current.png" aria-pressed="false">Décor actuel</button><button data-img="map-image" data-src="{PREFIX}engine/map-candidate.png" aria-pressed="true">Correction proposée</button><button data-focus="0">Quartier nord</button><button data-focus="2850">Jonction sud</button></div><div class="scroll map" id="map-window"><img id="map-image" src="{PREFIX}engine/map-candidate.png" width="1664" height="4224" alt="Carte complète éclairée par le moteur"></div></section>''')
    parts.append('<section id="contacts"><h2>Contrôle de circulation</h2><p>Les collisions reposent désormais sur des emprises au sol explicitement dessinées, distinctes de l’alpha des images. Les nouveaux points libérés apparaissent en cyan ci-dessous ; les grilles servent uniquement au diagnostic.</p>')
    base=Image.open(ROOT/'assets/maps/prison-street-expanded-01/street.png').convert('RGB');draw=ImageDraw.Draw(base)
    repair=read(ROOT/'assets/maps/prison-street-expanded-01/collision-repairs-r09.json')
    for x,y in repair['cells']:draw.ellipse((x*64+25,y*64+25,x*64+39,y*64+39),fill='#22ecf2',outline='black',width=2)
    for key,box in [('contacts-north',(64,704,1600,1152)),('contacts-van',(64,1856,1600,2240)),('contacts-court',(64,2368,1600,2816))]:
        im=base.crop(box);im.save(OUT/f'{key}.png');parts.append(f'<details><summary>{key}</summary><div class="scroll"><img src="{PREFIX}{key}.png" width="{im.width}" height="{im.height}" alt="Emplacements des contacts libérés"></div></details>')
    parts.append('''</section><p><a href="../../docs/reviews/refonte-histoire-2026-10-16/street-repairs-review-09/README.md">Constats, corrections et contrôles</a> · <a href="../../assets/source-art/first-sector-review-01/u01-repairs-r09/imagegen-prompt.json">Prompt de l’entrepôt · imagegen intégré</a> · <a href="../../assets/source-art/first-sector-review-01/u01-repairs-r09/generation-receipts.json">Source et prompt de route · imagegen intégré</a></p><script>document.querySelectorAll('[data-img]').forEach(b=>b.onclick=()=>{document.getElementById(b.dataset.img).src=b.dataset.src;document.querySelectorAll('[data-img="'+b.dataset.img+'"]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));});document.querySelectorAll('[data-focus]').forEach(b=>b.onclick=()=>document.getElementById('map-window').scrollTop=+b.dataset.focus);document.getElementById('map-window').scrollTop=2850;</script></html>''')
    PAGE.write_text('\n'.join(parts),encoding='utf-8')
    class Refs(HTMLParser):
        def __init__(self):super().__init__();self.refs=[]
        def handle_starttag(self,tag,attrs):
            for k,v in attrs:
                if k in ['src','href','data-src']:self.refs.append(v)
    parser=Refs();parser.feed(PAGE.read_text('utf-8'));checked=0
    for v in parser.refs:
        ref=urlsplit(v)
        if ref.scheme or not ref.path:continue
        assert (PAGE.parent/unquote(ref.path)).resolve().is_file(),v
        checked+=1
    proofs=DOC/'proofs';proofs.mkdir(exist_ok=True)
    for p in (OUT/'engine').glob('*.png'):shutil.copy2(p,proofs/p.name)
    write(DOC/'engine-proof-hashes.json',{p.name:sha(p) for p in proofs.glob('*.png')})
    write(DOC/'review-page-checks.json',dict(native_scene_crop=[1218,844],engine_scene_pairs=8,full_map_pairs=1,local_references_checked=checked,current_page=PAGE.relative_to(ROOT).as_posix(),imagegen_mode='built-in',candidate_art_active_normally=False))
    print('Published U01-REPAIRS-09 in the current map review page;',checked,'local references checked')

def integrated():
    engine=OUT/'engine-integration'
    checks=read(engine/'checks.json')
    assert checks['generation']==161 and checks['former_prop_ground_clear']
    assert checks['normal_art_integrated'] and checks['camp_round_trip']
    thumbs=OUT/'integration-native';thumbs.mkdir(exist_ok=True)
    scenes=[('warehouse','Entrepôt harmonisé'),('warehouse-cleared','Trois objets retirés · joints atténués'),
        ('road14','Route en Y 14'),('crater16','Passage en (14,16)'),('van32','Abords de la voiture'),
        ('rubble-west','Passages ouest'),('rubble40','Cour en (15,40)'),
        ('junction','Jonction Y 48–49'),('gate','Départ sud')]
    prefix=PREFIX+'integration-native/'
    parts=['''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>U01 · réparations intégrées</title><style>body{background:#11191d;color:#d4ddda;font:17px Segoe UI,sans-serif;margin:24px;line-height:1.55}h1,h2{color:#decca7}a{color:#d9c18c}p{max-width:1120px}nav,.buttons{display:flex;gap:14px;flex-wrap:wrap}button{font:inherit;background:#263632;color:#e0d3b6;border:1px solid #64706a;padding:8px 14px;cursor:pointer}button[aria-pressed=true]{background:#4c5039;border-color:#d8c48f}.scroll{overflow:auto;max-width:100%;border:1px solid #43514b;margin:16px 0}img{display:block;max-width:none}.map{height:800px}section{margin:36px 0}</style>
<h1>U01-REPAIRS-09 · intégré au jeu</h1>
<p>L’entrepôt moins vif, la route corrigée et le raccord sud recomposé sont actifs. La caisse en bois, le baril bleu et l’armoire verte à côté du bâtiment en L sont retirés. Les dalles révélées reprennent les joints partiellement effacés du reste du quartier, avec les mêmes matériaux et orientations variées.</p>
<p>Les 63 corrections de circulation sont conservées ; la dernière case bloquée sous ces objets est également libérée. Les anciennes parties adoptent ces corrections après vérification, sans tour consommé. Les scènes ci-dessous proviennent du rendu normal du moteur, à 64 px par case, avec la vision de six cases et les vrais éclairages.</p>
<nav><a href="#warehouse-cleared">Sol des objets retirés</a><a href="#warehouse">Entrepôt</a><a href="#road14">Route</a><a href="#junction">Jonction</a><a href="#map">Carte entière</a></nav>''']
    for key,title in scenes:
        before=(engine/f'{key}-candidate.png') if key=='warehouse-cleared' else OUT/'engine'/f'{key}-current.png'
        for suffix,file in [('before',before),('integrated',engine/f'{key}-current.png')]:
            Image.open(file).convert('RGB').crop((334,50,1552,894)).save(thumbs/f'{key}-{suffix}.png')
        parts.append(f'<section id="{key}"><h2>{title}</h2><div class="buttons"><button data-img="{key}" data-src="{prefix}{key}-before.png" aria-pressed="false">Avant</button><button data-img="{key}" data-src="{prefix}{key}-integrated.png" aria-pressed="true">En jeu</button></div><div class="scroll"><img id="{key}-image" src="{prefix}{key}-integrated.png" width="1218" height="844" loading="lazy" alt="Rendu natif intégré"></div><p><a href="{PREFIX}engine-integration/{key}-current.png">Capture complète du moteur</a></p></section>')
    parts.append(f'''<section id="map"><h2>Carte entière éclairée</h2><p>Cette exportation du décor retire le masque de vision et les acteurs pour vérifier les raccords. Le jeu conserve le cercle de six cases visible dans les scènes ci-dessus.</p><div class="buttons"><button data-img="map" data-src="{PREFIX}engine/map-current.png" aria-pressed="false">Avant</button><button data-img="map" data-src="{PREFIX}engine-integration/map-current.png" aria-pressed="true">En jeu</button><button data-focus="0">Nord</button><button data-focus="2850">Jonction sud</button></div><div class="scroll map" id="map-window"><img id="map-image" src="{PREFIX}engine-integration/map-current.png" width="1664" height="4224" alt="Décor intégré complet"></div></section>''')
    parts.append('''<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/street-repairs-review-09/README.md">Décisions et contrôles</a></p><script>document.querySelectorAll('[data-img]').forEach(b=>b.onclick=()=>{document.getElementById(b.dataset.img+'-image').src=b.dataset.src;document.querySelectorAll('[data-img="'+b.dataset.img+'"]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));});document.querySelectorAll('[data-focus]').forEach(b=>b.onclick=()=>document.getElementById('map-window').scrollTop=+b.dataset.focus);document.getElementById('map-window').scrollTop=2850;</script></html>''')
    PAGE.write_text('\n'.join(parts),encoding='utf-8')
    class Refs(HTMLParser):
        def __init__(self):super().__init__();self.refs=[]
        def handle_starttag(self,tag,attrs):
            self.refs.extend(v for k,v in attrs if k in ['src','href','data-src'])
    parser=Refs();parser.feed(PAGE.read_text('utf-8'));checked=0
    for v in parser.refs:
        ref=urlsplit(v)
        if ref.scheme or not ref.path:continue
        assert (PAGE.parent/unquote(ref.path)).resolve().is_file(),v
        checked+=1
    proofs=DOC/'proofs/integration';proofs.mkdir(exist_ok=True)
    for p in engine.glob('*.png'):shutil.copy2(p,proofs/p.name)
    write(DOC/'integration-proof-hashes.json',{p.name:sha(p) for p in proofs.glob('*.png')})
    write(DOC/'integration-page-checks.json',dict(native_scene_crop=[1218,844],engine_scenes=9,
        full_maps=1,local_references_checked=checked,current_page=PAGE.relative_to(ROOT).as_posix(),
        active_normally=True,removed_props=3,paving='accepted fragmented and nearly erased slabs'))
    print('Published integrated U01 map;',checked,'local references checked')

if __name__=='__main__':
    import sys
    if '--integrated' in sys.argv:integrated()
    else:build()
