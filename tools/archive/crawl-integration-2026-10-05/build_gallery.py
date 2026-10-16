"""Present the actual renderer captures; never edits the game or its bitmaps."""
import base64
import html
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
pages=[
    ('Armes · 01–18','validated-crawl-weapons'),
    ('Tenues · 01–17','validated-crawl-outfits'),
    ('Palettes · 01–09','validated-crawl-palettes-1'),
    ('Palettes · 10–17','validated-crawl-palettes-2'),
    ('Bestiaire et NPC','validated-crawl-bestiary'),
    ('Toutes les prises','final-player-outfits-review'),
    ('En jeu','shipping-world'),
    ('Inventaire et survol','shipping-inventory-equipped'),
    ('Mains actives','shipping-inventory-hands'),
    ('Écran de mort corrigé','validated-death'),
]
buttons=[];sections=[]
for n,(title,folder) in enumerate(pages):
    path=OUT/'captures'/folder/'cold-start.png'
    if not path.exists(): continue
    encoded=base64.b64encode(path.read_bytes()).decode()
    buttons.append(f'<button onclick="show({n})">{html.escape(title)}</button>')
    sections.append(f'<section id="p{n}" hidden><h2>{html.escape(title)}</h2><div class="scroll"><img src="data:image/png;base64,{encoded}" alt="{html.escape(title)} — capture du renderer actif"></div></section>')
document='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Project RL · tiles intégrées</title><style>
body{margin:18px;background:#11191f;color:#d1e6e2;font:15px monospace}h1{font-size:22px}h2{font-size:17px}
p{line-height:1.5;max-width:900px}nav{display:flex;gap:8px;flex-wrap:wrap}button{font:inherit;color:#a8e2d5;background:#202e35;border:1px solid #56796f;padding:8px;cursor:pointer}
.scroll{overflow:auto;border:1px solid #38534c}img{display:block;width:1280px;height:800px;max-width:none;image-rendering:pixelated}a{color:#9cdccc}
</style><h1>Project RL · nouveaux visuels intégrés</h1>
<p>113 modèles d'armes associés à 18 dessins de famille, 17 tenues et 23 références existantes de bestiaire/NPC.
La hache présente désormais son fer vers l'extérieur. Les slots et le groupe actif conservent leur fonctionnement.</p>
<p>Les captures proviennent du rendu actif. Elles gardent leur taille de 1280 × 800 : faites défiler horizontalement si le panneau est étroit.
Chaque planche précise ses vues natives et ses agrandissements. Le zombie reste une image en réserve.</p><nav>'''+''.join(buttons)+'</nav>'+''.join(sections)+'''
<script>function show(n){document.querySelectorAll('section').forEach(s=>s.hidden=s.id!=='p'+n)}show(0)</script></html>'''
(OUT/'galerie-integree.html').write_text(document,encoding='utf-8')
(ROOT/'artifacts/armures-bicolores-completes-2026-10-05/galerie-integree.html').write_text(document,encoding='utf-8')
report=[]
for _,folder in pages:
    p=OUT/'captures'/folder/'art-validation.json'
    if p.exists():report.append(json.loads(p.read_text()))
(OUT/'render-validation.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print('Gallery:',len(sections),'actual renderer captures')
