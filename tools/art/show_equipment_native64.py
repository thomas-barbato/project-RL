"""Refresh the single equipment page from the active banks, not old proposals."""
from pathlib import Path
import base64
import json

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'docs/reviews/refonte-agent-2026-10-06/equipment-native64-01'
OUTPUT = ROOT / 'artifacts/apercu-equipement64.html'


def main():
    review = json.loads((REVIEW / 'review.json').read_text('utf-8'))
    icons = [{k: e[k] for k in ('id', 'name', 'kind', 'review_id', 'number', 'variant')} for e in review['icons']]
    outfits = [{k: e[k] for k in ('id', 'name', 'content_id', 'number', 'bound')} for e in review['worn']]
    configs = json.loads((ROOT / 'assets/sprites/flat-agents/manifest.json').read_text('utf-8'))['configurations']
    # These two historical studies use an unbound object, not a model equipped
    # by the game. Current heavy weapons are shown through the active renderer.
    configs = [entry for entry in configs if entry['number'] not in ('10', '11')]
    captures = []
    for folder, title in [('equipment64-runtime-check', 'Objets au sol, icônes, fiches et tenues portées'),
                          ('equipment64-inventory-check', 'Inventaire du Bastion dans le jeu')]:
        path = ROOT / f'artifacts/{folder}/cold-start.png'
        if path.exists():
            uri = 'data:image/png;base64,' + base64.b64encode(path.read_bytes()).decode()
            captures.append(f'<h3>{title}</h3><div class="capture"><img alt="{title}" src="{uri}" width="1280"></div>')
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Équipements 64 px · intégrés</title><style>
*{box-sizing:border-box}body{margin:0;background:#101518;color:#e9e5dc;font:15px/1.5 system-ui,sans-serif}main{max-width:1320px;margin:auto;padding:24px}h1{font-size:28px;margin:0 0 8px}h2{font-size:22px}p{margin:6px 0}header p{max-width:1000px;color:#b8c4ca}.note{padding:12px 14px;border-left:3px solid #b17b50;background:#1e292e;margin:16px 0}.toolbar{position:sticky;top:0;z-index:3;background:#101518f2;display:flex;gap:8px;flex-wrap:wrap;padding:12px 0}button,select,input{font:inherit;color:inherit;background:#233039;border:1px solid #52616a;border-radius:6px;padding:8px 12px}button,select{cursor:pointer}button.active{background:#8f4832;border-color:#d89b69}.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(235px,1fr));gap:12px}.card{background:#192126;border:1px solid #37464e;border-radius:9px;padding:14px;min-width:0}.card h3{font-size:16px;margin:4px 0;min-height:48px}.number{font:12px ui-monospace,monospace;color:#edb584}.id{font:11px ui-monospace,monospace;color:#9db0ba;overflow-wrap:anywhere;min-height:30px}.icon{width:64px;height:64px;margin:12px auto;background-image:url('../assets/sprites/equipment-native64/icons-64.png');background-repeat:no-repeat;image-rendering:pixelated}.worn{width:128px;height:128px;margin:auto;position:relative;background-repeat:no-repeat;image-rendering:pixelated}.worn:after{content:"";position:absolute;left:32px;top:32px;width:64px;height:64px;border:1px solid #62778480;pointer-events:none}.badge{font-size:12px;color:#b9d4bc}.capture{overflow:auto;border:1px solid #37464e}.capture img{display:block;max-width:none;width:1280px}.section[hidden],.card[hidden]{display:none!important}footer{margin-top:28px;color:#b6c6ce;font-size:13px}
</style><main><header><h1>Armes et armures · 64 px intégrés au jeu</h1><p>110 armes, 34 modèles d’armure et 176 exports d’icônes. Les sept tenues supplémentaires rejoignent les douze références détaillées déjà acceptées. Les identités des objets et les statistiques sont conservées.</p><div class="note">Carapace Chrysalide : la variante <strong>olive sombre et os</strong> déjà validée est maintenant aussi sa couleur d’origine. L’ancienne base rose et cyan n’est plus sélectionnée.<br>Les icônes ci-dessous font réellement 64 × 64 px. Le carré au centre d’une tenue portée représente une case de 64 px ; son canevas de 128 px conserve les débordements d’armes. À examiner avec le navigateur à 100 %.</div></header>
<nav class="toolbar"><button data-tab="icons" class="active">Objets et inventaire</button><button data-tab="worn">Tenues portées</button><button data-tab="runtime">Captures du jeu</button><select id="kind"><option value="all">Armes et armures</option><option value="weapon">Armes</option><option value="armor">Armures</option></select><input id="search" type="search" placeholder="Chercher un nom"><span id="count"></span></nav>
<section id="icons" class="section"><p class="note">Ces images proviennent de la banque utilisée par les objets au sol, l’inventaire et les fiches d’objets. Les petits emplacements utilisent une réduction de ces mêmes dessins ; les affichages plus grands utilisent directement la source 64 px.</p><div class="grid" id="iconcards"></div></section>
<section id="worn" class="section" hidden><div class="toolbar"><label>Couleur <select id="variant"><option value="0">D’origine</option><option value="1">Variante 2</option><option value="2">Variante 3</option></select></label><label>Équipement <select id="config"></select></label></div><div class="grid" id="worncards"></div></section>
<section id="runtime" class="section" hidden><p class="note">Captures du rendu actif, en session de diagnostic isolée. Elles sont affichées à leur taille originale ; faire défiler horizontalement si nécessaire. Aucune sauvegarde de campagne n’est modifiée.</p>CAPTURES</section><footer>Sources conservées dans assets/ ; décisions et contrôles dans docs/reviews/refonte-agent-2026-10-06/equipment-native64-01/. Cette page unique remplace la galerie de propositions et embarque les captures pour permettre de supprimer leurs fichiers temporaires.</footer></main><script>
const icons=ICONS, outfits=OUTFITS, configs=CONFIGS;
function card(e){const a=document.createElement('article');a.className='card';a.dataset.name=(e.name+' '+e.id).toLowerCase();a.dataset.kind=e.kind||'outfit';const number=document.createElement('div');number.className='number';number.textContent=e.review_id||'EQ64-'+e.id;const title=document.createElement('h3');title.textContent=e.name;const id=document.createElement('p');id.className='id';id.textContent=e.content_id===null?'Référence sans objet torse associé':e.id;a.append(number,title,id);return a;}
for(const e of icons){const a=card(e),i=e.number-1,s=document.createElement('div');s.className='icon';s.style.backgroundPosition=`${-(i%16)*64}px ${-Math.floor(i/16)*64}px`;const p=document.createElement('p');p.className='badge';p.textContent=e.variant?'Couleur '+(e.variant+1)+' · 64 px actif':'Couleur d’origine · 64 px actif';a.append(s,p);document.querySelector('#iconcards').appendChild(a);}
for(const e of outfits){const a=card(e);a.querySelector('.id').textContent=e.content_id||'Référence sans objet torse associé';const s=document.createElement('div');s.className='worn';const p=document.createElement('p');p.className='badge';p.textContent=e.number<=12?'Référence 64 px conservée':'Tenue 64 px raccordée';a.append(s,p);document.querySelector('#worncards').appendChild(a);}
for(const e of configs){const o=document.createElement('option');o.value=e.number;o.textContent=e.label;document.querySelector('#config').appendChild(o);}
function updateWorn(){const v=+document.querySelector('#variant').value,c=+document.querySelector('#config').value;document.querySelectorAll('#worncards .worn').forEach((s,i)=>{const original=i<12,j=original?i:i-12,index=(j*3+v)*16+c;s.style.backgroundImage=`url('../assets/sprites/${original?'flat-agents/reviewed-128.png':'equipment-native64/outfits-128.png'}')`;s.style.backgroundPosition=`${-(index%16)*128}px ${-Math.floor(index/16)*128}px`;});}
document.querySelector('#variant').onchange=updateWorn;document.querySelector('#config').onchange=updateWorn;updateWorn();
let selected='icons';function filter(){const q=document.querySelector('#search').value.toLowerCase(),kind=document.querySelector('#kind').value;let count=0;document.querySelectorAll('#'+selected+' .card').forEach(a=>{a.hidden=!(a.dataset.name.includes(q)&&(selected==='worn'||kind==='all'||a.dataset.kind===kind));if(!a.hidden)count++;});document.querySelector('#count').textContent=selected==='runtime'?'':count+' dessins affichés';}
document.querySelectorAll('[data-tab]').forEach(b=>b.onclick=()=>{selected=b.dataset.tab;document.querySelectorAll('.section').forEach(s=>s.hidden=s.id!==selected);document.querySelectorAll('[data-tab]').forEach(a=>a.classList.toggle('active',a===b));document.querySelector('#kind').hidden=selected!=='icons';document.querySelector('#search').hidden=selected==='runtime';filter();});document.querySelector('#search').oninput=filter;document.querySelector('#kind').onchange=filter;filter();
</script></html>'''
    for marker, value in [('CAPTURES', ''.join(captures)), ('ICONS', json.dumps(icons, ensure_ascii=False)),
                          ('OUTFITS', json.dumps(outfits, ensure_ascii=False)), ('CONFIGS', json.dumps(configs, ensure_ascii=False))]:
        page = page.replace(marker, value)
    OUTPUT.write_text(page, 'utf-8')
    print(json.dumps({'active_icons': len(icons), 'active_outfits': len(outfits),
                      'embedded_captures': len(captures), 'html_bytes': OUTPUT.stat().st_size}))


if __name__ == '__main__':
    main()
