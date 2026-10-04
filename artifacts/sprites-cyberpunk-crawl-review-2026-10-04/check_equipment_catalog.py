"""Inspect source alpha coverage without editing or exporting any image."""
from pathlib import Path
from PIL import Image
import json
import hashlib

OUT = Path(__file__).resolve().parent
catalogue = json.loads((OUT / 'equipment-catalog.json').read_text(encoding='utf-8'))
records = catalogue['models']
images = {name: Image.open(OUT / name).convert('RGBA') for name in {m['atlas'] for m in records}}
empty = []
used = set()
hashes = {}
duplicates = []
for model in records:
    image = images[model['atlas']]
    columns, rows, index = model['columns'], model['rows'], model['index']
    key = (model['atlas'], index)
    assert key not in used, f'Two models share one atlas cell: {key}'
    used.add(key)
    assert 0 <= index < columns * rows, f'Out of bounds: {model["id"]}'
    x, y = index % columns, index // columns
    cell = image.crop((round(x * image.width / columns), round(y * image.height / rows),
                       round((x+1) * image.width / columns), round((y+1) * image.height / rows)))
    histogram = cell.getchannel('A').histogram()
    coverage = sum(histogram[160:]) / cell.width / cell.height
    if coverage < .015:
        empty.append(model['id'])
    digest = hashlib.sha256(cell.tobytes()).hexdigest()
    if digest in hashes:
        duplicates.append((hashes[digest], model['id']))
    hashes[digest] = model['id']
assert not empty, f'Empty mapped cells: {empty}'
assert not duplicates, f'Identical source cells: {duplicates}'
assert len(records) == len({m['id'] for m in records})
assert all(m['affixes_included'] is False for m in records)
defined_weapons = len(list((OUT.parents[1] / 'content/core/weapons').glob('*.json5')))
assert sum(m['group'] != 'armor' for m in records) == defined_weapons
report = dict(models=len(records),counts=catalogue['counts'],unique_ids=len({m['id'] for m in records}),
              unique_mapped_cells=len(used),empty_cells=empty,identical_source_cells=duplicates,
              images={name:dict(size=image.size,alpha_extrema=image.getchannel('A').getextrema()) for name,image in images.items()},
              limitation='Checks coverage and cell identity, not the semantic accuracy or final worn model of each generated item.')
(OUT / 'equipment-coverage.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps(report,ensure_ascii=True))
