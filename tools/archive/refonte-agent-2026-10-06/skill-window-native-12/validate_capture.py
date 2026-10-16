from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
review = json.loads((OUT.parent / 'skill-icon-review-33/review.json').read_text(encoding='utf-8'))
c = review['candidates'][0]
assert c['visual_status'] == 'accepted'
capture = json.loads((OUT / 'capture-result.json').read_text(encoding='utf-8-sig'))
assert capture['process_exit'] == 0
image = Image.open(capture['file']).convert('RGB')
assert image.size == (960, 540)
source = Image.open(OUT.parent / 'skill-icon-review-33' / c['files']['64']['file']).convert('RGB')
possibilities = []
for x in (239, 367, 495):
    for y in (122, 270):
        crop = image.crop((x, y, x+64, y+64))
        error = sum(ImageStat.Stat(ImageChops.difference(crop, source)).mean) / 3
        possibilities.append((error, x, y, crop))
error, x, y, crop = min(possibilities, key=lambda p:p[0])
assert error < 15, error
crop.save(OUT / 'A02-native-icon.png')
result = {'skill':'A02', 'variant':'A02-C02', 'number':60, 'native_dimensions':[960,540],
          'native_icon_position':[x,y], 'mean_channel_error':round(error,3),
          'tolerance':15, 'accepted_source_match':True, 'process_exit':0}
(OUT / 'native-icon-match.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
(OUT / 'index.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Déflagration occulte validée · jeu</title><style>body{margin:0;padding:22px;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{max-width:960px;margin:auto}h1{font-size:23px}p{color:#a6b6c1;line-height:1.6}a{color:#b49ce5}img{display:block;width:960px;height:540px}</style><main><h1>60 · Déflagration occulte intégrée</h1><p>Le dessin A02-C02 validé remplace l’emplacement neutre. Capture réelle du jeu à 960 × 540 pixels.<br>Les autres dessins, textes et effets sont conservés. <a href="../skill-icon-review-34/index.html">Propositions de transformation suivantes</a></p><img src="A02-960/cold-start.png" alt="Fenêtre des compétences avec Déflagration occulte"></main></html>''',encoding='utf-8')
print(json.dumps(result))
