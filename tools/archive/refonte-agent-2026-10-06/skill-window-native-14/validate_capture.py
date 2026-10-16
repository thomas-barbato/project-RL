from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat

OUT = Path(__file__).resolve().parent
capture = json.loads((OUT / 'capture-result.json').read_text(encoding='utf-8-sig'))
assert capture['process_exit'] == 0
review = json.loads((OUT.parent / 'skill-icon-review-38/review.json').read_text(encoding='utf-8'))
image = Image.open(capture['file']).convert('RGB')
assert image.size == (960,540)
matches = []
for c in review['candidates']:
    assert c['visual_status'] == 'accepted'
    source = Image.open(OUT.parent / 'skill-icon-review-38' / c['files']['64']['file']).convert('RGB')
    possibilities = []
    for x in (239,367,495):
        for y in (122,270,418):
            crop = image.crop((x,y,x+64,y+64))
            error = sum(ImageStat.Stat(ImageChops.difference(crop,source)).mean) / 3
            possibilities.append((error,x,y,crop))
    error,x,y,crop = min(possibilities,key=lambda p:p[0])
    assert error < 15,(c['id'],error)
    crop.save(OUT / (c['id'] + '-native-icon.png'))
    matches.append({'skill':c['id'],'variant':c['variant'],'number':c['number'],
                    'position':[x,y],'mean_channel_error':round(error,3),'accepted_source_match':True})
assert len({tuple(m['position']) for m in matches}) == 4
result = {'native_dimensions':[960,540],'process_exit':0,'tolerance':15,'matches':matches}
(OUT / 'native-icon-match.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
(OUT / 'index.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Transformations intégrées · jeu</title><style>body{margin:0;padding:22px;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{max-width:960px;margin:auto}h1{font-size:23px}p{color:#a6b6c1;line-height:1.6}a{color:#b49ce5}img{display:block;width:960px;height:540px}</style><main><h1>65–68 · Transformations intégrées</h1><p>Les quatre icônes validées sont utilisées sans retouche dans l’arbre Transformations.<br>Capture réelle du jeu à 960 × 540 pixels. <a href="../skill-icon-review-39/index.html">Voir les nouvelles propositions · Pouvoirs hybrides</a></p><img src="TR61-960/cold-start.png" width="960" height="540" alt="Fenêtre native du jeu avec les six icônes validées de Transformations"></main></html>''',encoding='utf-8')
(OUT / 'README.md').write_text('''# SKILLS-NATIVE14 · Transformations

Le joueur valide les quatre dessins 65–68 (TR61-C01, TR63-C01, TR64-C01 et TR65-C01) par « ok on peut continuer », en réponse à IC38. Cette revue reprend les quatre sources IC34 sans les modifier. Accord exact dans ../skill-icon-review-38/visual-acceptance-01.json.

66 icônes compactes, 16 illustrations et 94 compétences. Les textes, courbes et effets ne changent pas. integration-validation.json vérifie les quatre exports actifs, les 20 exports de revue, les quatre masters et la préservation de 654 fichiers antérieurs.

Tests ciblés dans tests.log, compilation dans build.log, format dans format.log. Capture réelle 960 × 540 dans TR61-960/cold-start.png ; native-icon-match.json vérifie la présence des quatre icônes validées dans le jeu.
''',encoding='utf-8')
print(json.dumps(result))
