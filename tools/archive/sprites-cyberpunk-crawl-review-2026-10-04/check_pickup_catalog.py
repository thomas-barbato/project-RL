"""Inspect generated item images; never edit their pixels."""
from pathlib import Path
from PIL import Image
import hashlib
import json

OUT = Path(__file__).resolve().parent
supplies = json.loads((OUT / 'pickup-catalog.json').read_text(encoding='utf-8'))
equipment = json.loads((OUT / 'equipment-catalog.json').read_text(encoding='utf-8'))
records = []
hashes = set()
for model in supplies['models']:
    path = OUT / model['atlas']
    with Image.open(path) as image:
        assert image.mode == 'RGBA', f'No RGBA transparency: {path.name}'
        alpha = image.getchannel('A')
        assert alpha.getextrema() == (0, 255), f'Missing transparency: {path.name}'
        assert alpha.getpixel((0, 0)) == 0, f'Opaque background corner: {path.name}'
        histogram = alpha.histogram()
        coverage = sum(histogram[160:]) / (image.width * image.height)
        assert .015 < coverage < .95, f'Empty or opaque item: {path.name}'
        digest = hashlib.sha256(image.tobytes()).hexdigest()
        assert digest not in hashes, f'Duplicate image: {path.name}'
        hashes.add(digest)
        records.append(dict(id=model['id'], image=path.name, dimensions=list(image.size), opaque_coverage=round(coverage, 4), sha256=digest))
ids = [model['id'] for model in equipment['models'] + supplies['models']]
assert len(ids) == len(set(ids)) == supplies['total_pickup_count']
report = dict(status='passed', supply_icons=len(records), total_pickup_references=len(ids), missing_images=0, duplicate_source_images=0, images=records)
(OUT / 'pickup-coverage.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print(f'{len(records)} transparent supply icons, {len(ids)} unique base pickup references; coverage passed')
