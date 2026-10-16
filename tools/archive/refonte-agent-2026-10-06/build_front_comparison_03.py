"""Prepare the numbered frontal candidate without integrating it into the game."""
import json
from PIL import Image
import build_comic_catalogue_03 as art

out=art.OUT
source=Image.open(out/'sources'/'BD-FRONT01.png').convert('RGBA')
box=art.bounds(source)
anchor=[(box[0]+box[2])/2,box[3]]
master=art.affine(source,anchor,[32*art.F,47*art.F],29*art.F/(box[3]-box[1]))
art.outputs(master,'BD-FRONT01')
art.outputs(art.body(1),'BD-C01-unarmed-reference')
review={'review_id':'BD-FRONT01','status':'candidate_pending_visual_approval','source':'sources/BD-FRONT01.png','reference':'sources/BD-C01-unarmed.png','logical_tile':32,'display_tile':64,'body_height_native':29,'torso_identity':'Armure Bastion','method':'source-authorized frontal redrawing; uniform sampling; no change to active assets'}
(out/'front-review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2),'utf-8')
html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>BD-FRONT01 · Bastion de face</title><style>
body{background:#0e151d;color:#ebf0f4;font:16px system-ui;max-width:1180px;margin:25px auto;padding:0 20px}a{color:#b5e0ee}p{line-height:1.5;color:#a0b6c4}.columns{display:flex;gap:24px;flex-wrap:wrap}section{width:512px;max-width:100%}.large{display:block;width:512px;max-width:100%;height:auto;background:#111a23}.small{display:flex;gap:20px;align-items:center}.small img{image-rendering:pixelated;background:#afb8ba}h1{font-size:25px}h2{font-size:20px}
</style><h1>BD-FRONT01 · Bastion de face</h1><p>Même armure, vue frontale. Le candidat conserve une hauteur de personnage de 29 pixels dans une case logique de 32 pixels. Les vues 64 px reprennent exactement les pixels 32 px, agrandis par deux.</p><p><a href="index.html">Retour au catalogue complet</a> · <a href="sources/BD-FRONT01.png" target="_blank">Source du candidat en grand</a></p><div class="columns">
<section><h2>Référence précédente · trois quarts</h2><img class="large" src="normal/BD-C01-unarmed-reference.png" alt="Bastion précédent sans armes"><div class="small"><div><p>Case 32 px</p><img width="64" height="64" src="native32/BD-C01-unarmed-reference.png" alt="Bastion précédent 32 px"></div><div><p>Case 64 px · zoom ×2</p><img width="128" height="128" src="display64/BD-C01-unarmed-reference.png" alt="Bastion précédent 64 px"></div></div></section>
<section><h2>BD-FRONT01 · face</h2><img class="large" src="normal/BD-FRONT01.png" alt="Bastion frontal sans armes"><div class="small"><div><p>Case 32 px</p><img width="64" height="64" src="native32/BD-FRONT01.png" alt="Bastion frontal 32 px"></div><div><p>Case 64 px · zoom ×2</p><img width="128" height="128" src="display64/BD-FRONT01.png" alt="Bastion frontal 64 px"></div></div></section></div><p>Candidat de revue à examiner. La vue frontale n’est pas encore étendue aux autres tenues ni intégrée au jeu.</p></html>'''
(out/'front-bastion.html').write_text(html,'utf-8')
print('Frontal comparison ready')
