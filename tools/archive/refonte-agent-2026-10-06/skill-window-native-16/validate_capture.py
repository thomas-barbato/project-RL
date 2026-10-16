from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat

OUT = Path(__file__).resolve().parent
REVIEW = OUT.parent / 'skill-icon-review-42'
capture = json.loads((OUT / 'capture-result.json').read_text(encoding='utf-8-sig'))
assert capture['process_exit'] == 0
review = json.loads((REVIEW / 'review.json').read_text(encoding='utf-8'))
assert review['status'] == 'accepted'
image = Image.open(capture['file']).convert('RGB')
assert image.size == (960, 540)
matches = []
for candidate, (x, y) in zip(review['candidates'], ((239, 122), (367, 122))):
    assert candidate['visual_status'] == 'accepted'
    source = Image.open(REVIEW / candidate['files']['64']['file']).convert('RGB')
    crop = image.crop((x, y, x + 64, y + 64))
    error = sum(ImageStat.Stat(ImageChops.difference(crop, source)).mean) / 3
    assert error < 15, (candidate['id'], error)
    crop.save(OUT / (candidate['id'] + '-native-icon.png'))
    matches.append({'skill': candidate['id'], 'variant': candidate['variant'],
                    'position': [x, y], 'mean_channel_error': round(error, 3),
                    'tolerance': 15, 'accepted_source_match': True})
result = {'native_dimensions': [960, 540], 'matches': matches, 'process_exit': 0}
(OUT / 'native-icon-match.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
(OUT / 'index.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Pouvoirs hybrides intégrés · jeu</title><style>body{margin:0;padding:22px;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{max-width:960px;margin:auto}h1{font-size:23px}p{color:#a6b6c1;line-height:1.6}a{color:#b49ce5}.capture{overflow-x:auto}img{display:block;width:960px;height:540px}</style><main><h1>70–71 · Pouvoirs hybrides intégrés</h1><p>Échange de position et Lien de domination utilisent les dessins exacts validés.<br>Capture réelle du jeu à 960 × 540 pixels. <a href="../skill-icon-review-43/index.html">Voir les six prochaines propositions</a></p><div class="capture"><img src="HY04-960/cold-start.png" width="960" height="540" alt="Fenêtre native avec les deux icônes hybrides validées"></div></main></html>''', encoding='utf-8')
(OUT / 'README.md').write_text('''# SKILLS-NATIVE16 · Deux pouvoirs hybrides

Le joueur valide « ok parfait on continue » en réponse à la planche IC42 présentant les deux dessins 70 HY04-C01 et 71 HY06-C01. Ils sont intégrés sans retouche depuis leurs exports 64 px d’IC39. M02-C03 était déjà validée. Aucun changement de texte, règle, animation ou type de dégâts.

Accord dans ../skill-icon-review-42/visual-acceptance-01.json. La banque compte 69 compactes et 16 illustrations pour 94 compétences. integration-validation.json contrôle les dix exports, les deux masters, les fichiers actifs exacts et la préservation de 689 fichiers antérieurs.

Tests ciblés dans tests.log, compilation dans build.log, format dans format.log. Capture réelle dans HY04-960/cold-start.png, correspondance visuelle contrôlée par native-icon-match.json. Les propositions 72–77 d’IC43 attendent leur propre validation.
''', encoding='utf-8')
print(json.dumps(result))
