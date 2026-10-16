"""Check native captures against the exact approved compact exports."""
from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat, ImageDraw

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
review = json.loads((OUT.parent / 'skill-icon-review-31/review.json').read_text(encoding='utf-8'))
captures = json.loads((OUT / 'capture-results.json').read_text(encoding='utf-8-sig'))
results = []
board = Image.new('RGB', (650, 120), '#0b1017')
draw = ImageDraw.Draw(board)
entries = []
for index, candidate in enumerate(review['candidates']):
    capture = next(c for c in captures if c['skill'] == candidate['id'])
    assert capture['process_exit'] == 0
    image = Image.open(capture['file']).convert('RGB')
    assert image.size == (960, 540), image.size
    source = Image.open(OUT.parent / 'skill-icon-review-31' / candidate['files']['64']['file']).convert('RGB')
    possibilities = []
    for x in (239, 367, 495):
        for y in (122, 270):
            crop = image.crop((x, y, x + 64, y + 64))
            error = sum(ImageStat.Stat(ImageChops.difference(crop, source)).mean) / 3
            possibilities.append((error, x, y, crop))
    error, x, y, crop = min(possibilities, key=lambda value: value[0])
    assert error < 15, (candidate['id'], error)
    icon_name = candidate['id'] + '-native-icon.png'
    crop.save(OUT / icon_name)
    results.append({'skill': candidate['id'], 'variant': candidate['variant'],
                    'dimensions': list(image.size), 'native_icon': icon_name,
                    'native_icon_position': [x, y], 'mean_channel_error': round(error, 3),
                    'tolerance': 15, 'accepted_source_match': True})
    board.paste(crop, (index * 130 + 33, 36))
    draw.text((index * 130 + 8, 12), str(candidate['number']) + ' - ' + candidate['id'], fill='#ccd5db')
    entries.append({'id': candidate['id'], 'title': candidate['title'], 'file': candidate['id'] + '-960/cold-start.png'})
board.save(OUT / 'native-selected-icons.png')
(OUT / 'native-icon-matches.json').write_text(json.dumps(results, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
data = json.dumps(entries, ensure_ascii=False)
html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Froid validé · captures du jeu</title>
<style>body{margin:0;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{width:960px;margin:22px auto}h1{font-size:22px;margin:0 0 8px}p{color:#a6b6c1}nav{display:flex;gap:18px;margin:16px 0}a{color:#b49ce5}img{display:block;width:960px;height:540px}figcaption{margin:12px 0}</style>
<main><h1>Froid validé · fenêtre du jeu</h1><p>Cinq compactes intégrées sans retouche. Captures réelles à 960 × 540 pixels.</p><nav></nav><figcaption></figcaption><img alt="Fenêtre des compétences du jeu"></main>
<script>const entries=ENTRIES;const selected=entries.find(e=>e.id===new URLSearchParams(location.search).get('skill'))||entries[4];const nav=document.querySelector('nav');for(const e of entries){const a=document.createElement('a');a.href='?skill='+e.id;a.textContent=e.title;nav.append(a)}document.querySelector('figcaption').textContent=selected.title;document.querySelector('img').src=selected.file;</script></html>'''.replace('ENTRIES', data)
(OUT / 'index.html').write_text(html, encoding='utf-8')
print(json.dumps({'native_captures': len(results), 'source_matches': True,
                  'maximum_mean_channel_error': round(max(r['mean_channel_error'] for r in results), 3)}))
