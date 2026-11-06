"""Render the current first-city batch; keep prior accepted captures intact."""
from pathlib import Path
import html
import json
import re

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/city-cyberpunk12'


def render():
    current=json.loads((ROOT/'assets/source-art/city-cyberpunk12/review-current.json').read_text(encoding='utf-8'))
    manifest=json.loads((ROOT/current['manifest']).read_text(encoding='utf-8'))
    security=json.loads((ROOT/manifest['security_revision']).read_text(encoding='utf-8')) if manifest.get('security_revision') else {'items':[]}
    parts={item['id']:[{'id':p['id'],'label':p.get('label',p['id'])} for p in item['parts']] for item in [*manifest['items'],*manifest['roofs'],*security['items']]}
    stamp=str((OUT/'checks.json').stat().st_mtime_ns)
    data=json.dumps({'views':current['views'],'parts':parts,'defaultView':current['default_view'],'stamp':stamp},ensure_ascii=False).replace('</','<\\/')
    options=''.join('<option value="'+html.escape(v['id'],quote=True)+'">'+html.escape(v['name'])+'</option>' for v in current['views'])
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Première ville · lot 02</title>
<style>body{margin:0;background:#10171c;color:#dae0dc;font:16px/1.45 system-ui,sans-serif}main{max-width:1700px;margin:auto;padding:22px}h1{font-size:25px;color:#e3c58f;margin:0 0 12px}p{max-width:1100px}a{color:#b7dcde}.note{padding:10px 14px;background:#1a252c;border-left:3px solid #b9a57b}.toolbar{display:flex;align-items:center;gap:10px;flex-wrap:wrap;margin:12px 0}button,select{font:inherit;background:#26343c;color:#e6e4d9;border:1px solid #60717b;border-radius:4px;padding:8px 11px;max-width:100%}button{cursor:pointer}button[aria-pressed=true]{color:#f2d79b;border-color:#cfb67e}.scroll{overflow:auto;border:1px solid #45535c;background:#080e12;width:fit-content;max-width:100%}.scene{display:block;width:100%;max-width:var(--native-width);height:auto}.native .scene{width:var(--native-width);max-width:none}.muted{color:#a8b6b9}summary{cursor:pointer;color:#e3c58f;margin:12px 0}.old img{display:block;max-width:none}label{display:flex;gap:8px;align-items:center;flex-wrap:wrap}</style>
<main><h1>Première ville · lot 02</h1><p>Logement, boutique et entrepôt en L. <b>Lot 01 pré-validé, avec correction ciblée des accès.</b> Les tracés, les accès et les égouts restent en l’état.</p><p class="note"><b>Nouvelle proposition à inspecter.</b> Deux intérieurs aux sols distincts, leurs toitures et l’équipement de l’entrepôt. Les accès ordinaires n’ont plus de scanner ; les toits portent du refroidissement ou des antennes sans commande de sécurité exposée. La reprise de la première ville reste en cours, avant la nouvelle ville.</p>
<div class="toolbar"><label>Vue <select id="view" aria-label="Vue à inspecter">__OPTIONS__</select></label><button id="after" aria-pressed="true">Après</button><button id="before" aria-pressed="false">Avant</button><button id="native" aria-pressed="false">Taille native</button><a id="full" target="_blank">Image complète</a></div>
<div id="frame" class="scroll"><img id="scene" class="scene" alt="Logement futuriste usé, avec le personnage à taille native et les murs d’origine"></div><p id="state"></p>
<div class="toolbar"><label>Repère de retouche <select id="part" aria-label="Élément pour une retouche ciblée"></select></label></div>
<p class="muted">Les vues avec et sans toit servent à inspecter le dessin. La validation définitive des portes, de la circulation et de l’éclairage se fera en jeu.</p>
<details><summary>Historique du lot 01 · avant correction des accès</summary><div class="scroll old"><img src="secteur-apres.png" width="1664" height="1056" alt="Premier lot accepté avant la demande de correction ciblée des accès"></div></details>
<p><a href="../../docs/reviews/city-cyberpunk12/LOT02.md" target="_blank">Sources, prompts et périmètre du lot 02</a> · <a href="../../docs/reviews/city-cyberpunk12/LOT01-ACCEPTANCE-r01.json" target="_blank">Accord sur le lot 01</a></p>
<script id="data" type="application/json">__DATA__</script><script>
const data=JSON.parse(document.getElementById('data').textContent);const picker=document.getElementById('view'),scene=document.getElementById('scene'),frame=document.getElementById('frame'),part=document.getElementById('part');let selected=data.defaultView,mode='apres';
function show(){const v=data.views.find(v=>v.id===selected);picker.value=selected;scene.src=v.id+'-'+mode+'.png?'+data.stamp;scene.width=v.rect[2];scene.height=v.rect[3]+32;scene.alt=v.name+' · '+(mode==='apres'?'proposition futuriste grimdark':'état précédent')+' · même personnage natif';frame.style.setProperty('--native-width',v.rect[2]+'px');document.getElementById('full').href=scene.src;document.getElementById('state').textContent=v.name+' · '+(mode==='apres'?'après':'avant')+' · 64 px par case, personnage original de 64 px.';document.getElementById('after').setAttribute('aria-pressed',mode==='apres');document.getElementById('before').setAttribute('aria-pressed',mode==='avant');part.replaceChildren();for(const p of data.parts[v.item]||[]){const option=document.createElement('option');option.value=p.id;option.textContent=p.label;part.append(option)}part.disabled=part.options.length===0;}
picker.onchange=()=>{selected=picker.value;show()};document.getElementById('after').onclick=()=>{mode='apres';show()};document.getElementById('before').onclick=()=>{mode='avant';show()};document.getElementById('native').onclick=()=>{const on=frame.classList.toggle('native');document.getElementById('native').setAttribute('aria-pressed',on);document.getElementById('native').textContent=on?'Adapter à la page':'Taille native'};show();
</script></main></html>'''
    if current.get('revision_note'):
        detail=html.escape(current.get('revision_note_detail', ''))
        note='<p class="note"><b>'+html.escape(current['revision_note'])+'</b> '+detail+'</p>'
        page=re.sub(r'<p class="note">.*?</p>',lambda _:note,page,count=1,flags=re.S)
    title=html.escape(current.get('title', 'Première ville · lot 02'))
    page=page.replace('<title>Première ville · lot 02</title>', f'<title>{title}</title>')
    page=page.replace('<h1>Première ville · lot 02</h1>', f'<h1>{title}</h1>')
    if current.get('intro'):
        intro = html.escape(current['summary'] + ' ' + current['intro'])
        page = re.sub(r'(<h1>.*?</h1>)<p>.*?</p>', lambda match: match[1] + f'<p>{intro}</p>', page, count=1)
    else:
        page = page.replace('Logement, boutique et entrepôt en L.', html.escape(current.get('summary', 'Logement, boutique et entrepôt en L.')), 1)
    doc=html.escape(current.get('review_doc', 'LOT02.md'), quote=True)
    page=page.replace('/city-cyberpunk12/LOT02.md', f'/city-cyberpunk12/{doc}')
    page=page.replace('Sources, prompts et périmètre du lot 02', 'Sources et prompts de la revue')
    (OUT/'apercu.html').write_text(page.replace('__OPTIONS__',options).replace('__DATA__',data),encoding='utf-8')
    print(OUT/'apercu.html')


if __name__=='__main__':
    render()
