from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat

OUT = Path(__file__).resolve().parent
capture = json.loads((OUT / 'capture-result.json').read_text(encoding='utf-8-sig'))
assert capture['process_exit'] == 0
review = json.loads((OUT.parent / 'skill-icon-review-41/review.json').read_text(encoding='utf-8'))
c = review['candidates'][0]
assert c['variant'] == 'M02-C03' and c['visual_status'] == 'accepted'
image = Image.open(capture['file']).convert('RGB')
assert image.size == (960,540)
source = Image.open(OUT.parent / 'skill-icon-review-41' / c['files']['64']['file']).convert('RGB')
possibilities = []
for x in (239,367,495):
    for y in (122,270,418):
        crop = image.crop((x,y,x+64,y+64))
        error = sum(ImageStat.Stat(ImageChops.difference(crop,source)).mean) / 3
        possibilities.append((error,x,y,crop))
error,x,y,crop = min(possibilities,key=lambda p:p[0])
assert error < 15,error
crop.save(OUT / 'M02-native-icon.png')
result = {'skill':'M02','variant':'M02-C03','number':69,'native_dimensions':[960,540],
          'native_icon_position':[x,y],'mean_channel_error':round(error,3),'tolerance':15,
          'accepted_source_match':True,'process_exit':0}
(OUT / 'native-icon-match.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
(OUT / 'index.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Décharge mutagène intégrée · jeu</title><style>body{margin:0;padding:22px;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{max-width:960px;margin:auto}h1{font-size:23px}p{color:#a6b6c1;line-height:1.6}a{color:#b49ce5}.capture{overflow-x:auto}img{display:block;width:960px;height:540px}</style><main><h1>69 · Décharge mutagène intégrée</h1><p>Le dessin vert sans rouge M02-C03 est utilisé tel que validé.<br>Capture réelle du jeu à 960 × 540 pixels. <a href="../skill-icon-review-42/index.html">Voir les deux pouvoirs hybrides encore à valider</a></p><div class="capture"><img src="M02-960/cold-start.png" width="960" height="540" alt="Fenêtre native avec l’icône verte validée de Décharge mutagène"></div></main></html>''',encoding='utf-8')
(OUT / 'README.md').write_text('''# SKILLS-NATIVE15 · Décharge mutagène

Le joueur valide « ok je valide ceci » avec une pièce jointe correspondant pixel pour pixel au master M02-C03. Seule cette icône 69 est intégrée. Les propositions HY04-C01 et HY06-C01 restent à valider. Aucun changement de texte, règle, animation ou type de dégâts.

Accord dans ../skill-icon-review-41/visual-acceptance-01.json. La banque compte 67 compactes et 16 illustrations pour 94 compétences. integration-validation.json contrôle les cinq exports, le master, le fichier actif exact et 715 fichiers antérieurs.

Tests ciblés dans tests.log, compilation dans build.log, format dans format.log. Capture réelle dans M02-960/cold-start.png, correspondance visuelle contrôlée par native-icon-match.json.
''',encoding='utf-8')
print(json.dumps(result))
