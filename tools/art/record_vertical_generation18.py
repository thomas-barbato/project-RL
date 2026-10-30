"""Persist one unique generation record; concurrent jobs never share a file."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image

root=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('kind',choices=['monsters','weapons']);p.add_argument('id');p.add_argument('generated_source');a=p.parse_args()
doc=json.loads((root/f'docs/reviews/vertical-walk-18/prompts-C06-{a.kind}.json').read_text('utf8'))
job=next(j for j in doc['jobs'] if j['id']==a.id)
asset=root/job['output']
with Image.open(asset) as image:image.verify()
record={**job,'mode':'builtin_image_gen','status':'drawn_pending_native_and_anatomy_review',
        'sha256':hashlib.sha256(asset.read_bytes()).hexdigest(),'generated_source':a.generated_source}
out=root/'docs/reviews/vertical-walk-18/generation-C06';out.mkdir(parents=True,exist_ok=True)
(out/f'{a.id}.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n','utf8')
