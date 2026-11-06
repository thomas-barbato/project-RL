"""Refresh the single current first-city review from native captures."""
from pathlib import Path
import html
import json

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/city-cyberpunk12'


def main():
    if (ROOT/'assets/source-art/city-cyberpunk12/review-current.json').exists():
        from show_city_cyberpunk12_lot02 import render
        return render()
    manifest=json.loads((ROOT/'assets/source-art/city-cyberpunk12/manifest.json').read_text(encoding='utf-8'))
    stamp=str((OUT/'checks.json').stat().st_mtime_ns)
    names=['Garage ouest','Bâtiment bleu à l’est','Annexe de service au sud-est']
    cards=[]
    for number,(item,name) in enumerate(zip(manifest['items'],names),1):
        options=''.join('<option value="'+html.escape(p['id'],quote=True)+'">'+html.escape(p['id'].split('-D')[0].split('-west')[0].split('-east')[0]+'/D'+p['id'].split('-D')[1])+'</option>' for p in item['parts'])
        cards.append(f'''<section class="card"><h2>V1-B0{number} · {name}</h2><div class="scroll"><img class="detail" data-name="V1-B0{number}" src="V1-B0{number}-apres.png?{stamp}" width="{640 if number==1 else 608}" height="{672 if number==1 else 736}" alt="{name} avec les nouveaux équipements et le personnage à taille native"></div><label>Élément pour une retouche ciblée <select aria-label="Élément de V1-B0{number}">{options}</select></label></section>''')
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Première ville · reprise futuriste grimdark</title>
<style>body{margin:0;background:#10171c;color:#d9dfdb;font:16px/1.5 system-ui,sans-serif}main{max-width:1720px;margin:auto;padding:24px}h1,h2{color:#e2c48e}h1{font-size:27px}h2{font-size:20px}p{max-width:1100px}a{color:#b5dbde}button,select{font:inherit;color:#e4e5dc;background:#26343c;border:1px solid #62737c;border-radius:4px;padding:9px 12px}button{cursor:pointer}button[aria-pressed=true]{border-color:#dfc28e;color:#f5d89c}.toolbar{display:flex;gap:12px;flex-wrap:wrap;align-items:center;margin:16px 0}.scroll{overflow:auto;background:#090f13;border:1px solid #45515a}.main-image{display:block;width:100%;height:auto;max-width:1664px}.native .main-image{width:1664px;max-width:none}.detail{display:block;max-width:none}.cards{display:flex;gap:20px;flex-wrap:wrap}.card{min-width:0;max-width:100%;flex:1 1 610px;padding-bottom:16px}.card label{display:block;margin-top:12px}.card select{max-width:100%;display:block;margin-top:4px}.note{border-left:3px solid #c0a773;padding:10px 16px;background:#19232b}.muted{color:#a5b2b7}</style>
<main><h1>Première ville · futuriste, grimdark, sale et abîmée</h1><p>Ordre de reprise : <b>première ville, puis nouvelle ville</b>. Leurs tracés sont conservés ; les égouts restent en l’état.</p><p class="note"><b>Lot 01 à pré-valider : trois bâtiments de la cour centrale.</b> Les nouveaux équipements sont séparés pour les retouches. Les autres bâtiments et les intérieurs restent à reprendre. Cette proposition n’est pas intégrée à la campagne.</p>
<div class="toolbar"><button id="after" aria-pressed="true">Après · nouveaux équipements</button><button id="before" aria-pressed="false">Avant · décor actuel</button><button id="native" aria-pressed="false">Afficher à taille native</button><a id="full" href="secteur-apres.png?__STAMP__" target="_blank">Ouvrir l’image complète</a></div><div class="scroll" id="frame"><img id="main" class="main-image" src="secteur-apres.png?__STAMP__" width="1664" height="1056" alt="Même secteur de la première ville, trois bâtiments avec équipements futuristes, mêmes routes et mêmes emprises"></div><p id="state">Après · 64 px par case. Le personnage d’origine garde sa taille native.</p>
<div class="cards">__CARDS__</div><p class="muted">La validation définitive de l’éclairage et de la circulation se fera en jeu. Les plans, portes, sols et passages existants sont conservés.</p><p><a href="../../docs/reviews/city-cyberpunk12/REVUE.md" target="_blank">Périmètre, sources et prompts</a> · <a href="../north-city-01/apercu.html#b01" target="_blank">B01 r04 · référence déjà pré-validée</a></p>
<script>const stamp='__STAMP__';let mode='apres';function show(next){mode=next;document.getElementById('main').src='secteur-'+mode+'.png?'+stamp;document.getElementById('full').href=document.getElementById('main').src;document.querySelectorAll('.detail').forEach(img=>img.src=img.dataset.name+'-'+mode+'.png?'+stamp);document.getElementById('after').setAttribute('aria-pressed',mode==='apres');document.getElementById('before').setAttribute('aria-pressed',mode==='avant');document.getElementById('state').textContent=(mode==='apres'?'Après · nouveaux équipements.':'Avant · décor actuel.')+' 64 px par case ; même personnage natif.'}document.getElementById('after').onclick=()=>show('apres');document.getElementById('before').onclick=()=>show('avant');document.getElementById('native').onclick=()=>{const on=document.getElementById('frame').classList.toggle('native');document.getElementById('native').setAttribute('aria-pressed',on);document.getElementById('native').textContent=on?'Adapter à la page':'Afficher à taille native'};</script></main></html>'''
    (OUT/'apercu.html').write_text(page.replace('__STAMP__',stamp).replace('__CARDS__',''.join(cards)),encoding='utf-8')
    print(OUT/'apercu.html')


if __name__=='__main__':
    main()
