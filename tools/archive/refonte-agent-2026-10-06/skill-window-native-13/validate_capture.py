from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat

OUT = Path(__file__).resolve().parent
capture = json.loads((OUT / 'capture-result.json').read_text(encoding='utf-8-sig'))
assert capture['process_exit'] == 0
review = json.loads((OUT.parent / 'skill-icon-review-37/review.json').read_text(encoding='utf-8'))
c = review['candidates'][0]
assert c['variant'] == 'M03-C05' and c['visual_status'] == 'accepted'
image = Image.open(capture['file']).convert('RGB')
assert image.size == (960,540)
source = Image.open(OUT.parent / 'skill-icon-review-37' / c['files']['64']['file']).convert('RGB')
possibilities = []
for x in (239,367,495):
    for y in (122,270,418):
        crop = image.crop((x,y,x+64,y+64))
        error = sum(ImageStat.Stat(ImageChops.difference(crop,source)).mean) / 3
        possibilities.append((error,x,y,crop))
error,x,y,crop = min(possibilities,key=lambda p:p[0])
assert error < 15,error
crop.save(OUT / 'M03-native-icon.png')
result = {'skill':'M03','variant':'M03-C05','number':8,'native_dimensions':[960,540],
          'native_icon_position':[x,y],'mean_channel_error':round(error,3),'tolerance':15,
          'accepted_source_match':True,'process_exit':0}
(OUT / 'native-icon-match.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
(OUT / 'index.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Puissance métamorphe intégrée · jeu</title><style>body{margin:0;padding:22px;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{max-width:960px;margin:auto}h1{font-size:23px}p{color:#a6b6c1;line-height:1.6}a{color:#b49ce5}img{display:block;width:960px;height:540px}</style><main><h1>8 · Puissance métamorphe intégrée</h1><p>Le symbole de quatre griffures M03-C05 remplace l’ancienne icône.<br>Capture réelle du jeu à 960 × 540 pixels. <a href="../skill-icon-review-34/index.html">Revenir aux quatre transformations à valider</a></p><img src="M03-960/cold-start.png" width="960" height="540" alt="Fenêtre des compétences avec le symbole de Puissance métamorphe"></main></html>''',encoding='utf-8')
(OUT / 'README.md').write_text('''# SKILLS-NATIVE13 · Puissance métamorphe

Le joueur valide M03-C05 par « oui ca me va :) » en réponse à IC37. Le symbole exact de quatre griffures est intégré à la banque. M03-C02 est archivée comme version remplacée ; les essais C03 et C04 restent non validés. Texte, mécanique et illustration détaillée inchangés. Les quatre propositions IC34 restent à valider.

Accord dans ../skill-icon-review-37/visual-acceptance-01.json. Script record_integration.py, état précédent et contrôles de préservation archivés ici. 62 compactes, 16 illustrations, 94 compétences ; seul M03-compact.png change parmi les ressources d’icônes actives.

Tests ciblés dans tests.log, compilation dans build.log, format dans format.log. Capture réelle 960 × 540 dans M03-960/cold-start.png ; native-icon-match.json contrôle la correspondance avec l’export 64 px validé.
''',encoding='utf-8')
print(json.dumps(result))
