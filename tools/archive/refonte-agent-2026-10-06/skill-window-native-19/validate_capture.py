from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
captures = json.loads((OUT / 'capture-result.json').read_text('utf-8-sig'))
assert len(captures) == 1 and captures[0]['process_exit'] == 0
image = Image.open(captures[0]['file']).convert('RGB')
assert image.size == (960, 540)
review = json.loads((BASE / 'skill-icon-review-49/review.json').read_text('utf-8'))
assert review['status'] == 'accepted'
candidates = review['candidates'] + [json.loads((BASE / 'skill-icon-review-48/review.json').read_text('utf-8'))['candidates'][0]]
positions = {'HL01': (239, 122), 'HL02': (367, 122), 'HL03': (495, 122),
             'HL04': (239, 270), 'HL05': (367, 270), 'HL06': (495, 270)}
matches = []
for c in candidates:
    source_review = 'skill-icon-review-48' if c['id'] == 'HL04' else 'skill-icon-review-49'
    source = Image.open(BASE / source_review / c['files']['64']['file']).convert('RGB')
    x, y = positions[c['id']]
    crop = image.crop((x, y, x+64, y+64))
    error = sum(ImageStat.Stat(ImageChops.difference(source, crop)).mean) / 3
    assert error < 15, (c['id'], error)
    crop.save(OUT / (c['id'] + '-native-icon.png'))
    matches.append({'id': c['id'], 'variant': c['variant'], 'mean_channel_error': round(error, 3), 'tolerance': 15})
(OUT / 'native-capture-validation.json').write_text(json.dumps({'native_dimensions': [960, 540], 'all_process_exits': 0, 'matches': matches}, indent=2)+'\n', 'utf-8')
(OUT / 'index.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Armes lourdes · icônes intégrées</title><style>body{margin:0;padding:22px;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{max-width:960px;margin:auto}h1{font-size:23px}p{color:#a6b6c1;line-height:1.6}a{color:#b49ce5}.capture{overflow-x:auto}img{display:block;width:960px;height:540px}</style><main><h1>Armes lourdes · six icônes intégrées</h1><p>Les cinq dessins validés d’IC49 rejoignent Maîtrise des armes lourdes. Les descriptions restent sans grande image.<br>Capture réelle à 960 × 540 pixels. <a href="../skill-icon-review-50/index.html">Propositions Furtivité et Mines</a></p><div class="capture"><img src="HL01-960/cold-start.png" width="960" height="540" alt="Fenêtre réelle des compétences Armes lourdes"></div></main></html>''', 'utf-8')
print(json.dumps(matches))
