from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent
captures = json.loads((OUT / 'capture-result.json').read_text('utf-8-sig'))
assert len(captures) == 1 and captures[0]['process_exit'] == 0
image = Image.open(captures[0]['file']).convert('RGB')
assert image.size == (960, 540)
ledger = json.loads((BASE / 'skill-icon-acceptance.json').read_text('utf-8'))
candidates = [c for c in ledger['compact_accepted'] if c['id'].startswith('SG')]
assert {c['id'] for c in candidates} == {'SG00', 'SG01', 'SG02', 'SG03', 'SG04'}
positions = {'SG00': (239, 122), 'SG01': (367, 122), 'SG02': (495, 122),
             'SG03': (239, 270), 'SG04': (367, 270)}
matches = []
for c in candidates:
    source = Image.open(ROOT / c['files']['64']['file']).convert('RGB')
    x, y = positions[c['id']]
    crop = image.crop((x, y, x+64, y+64))
    error = sum(ImageStat.Stat(ImageChops.difference(source, crop)).mean) / 3
    assert error < 15, (c['id'], error)
    crop.save(OUT / (c['id'] + '-native-icon.png'))
    matches.append({'id': c['id'], 'variant': c['variant'], 'mean_channel_error': round(error, 3), 'tolerance': 15})
(OUT / 'native-capture-validation.json').write_text(json.dumps({'native_dimensions': [960, 540], 'all_process_exits': 0, 'matches': matches}, indent=2)+'\n', 'utf-8')
(OUT / 'index.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Grenades · cinq icônes intégrées</title><style>body{margin:0;padding:22px;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{max-width:960px;margin:auto}h1{font-size:23px}p{color:#a6b6c1;line-height:1.6}a{color:#b49ce5}.capture{overflow-x:auto}img{display:block;width:960px;height:540px}</style><main><h1>Grenades · cinq icônes intégrées</h1><p>Les trois dessins 92, 95 et 96 validés d’IC54 complètent la branche Grenades.<br>Le catalogue dispose maintenant de 94 icônes compactes validées pour 94 compétences. Capture réelle à 960 × 540 pixels.</p><div class="capture"><img src="SG00-960/cold-start.png" width="960" height="540" alt="Fenêtre réelle des compétences de Grenades avec ses cinq icônes validées"></div></main></html>''', 'utf-8')
print(json.dumps(matches))
