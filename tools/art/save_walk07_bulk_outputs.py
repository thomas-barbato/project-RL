"""Journal built-in image outputs and copy selected source files into assets."""
import argparse,hashlib,json,shutil,sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
SOURCE=ROOT/'assets/source-art/player-walk-review-07'

rows=json.load(sys.stdin)
parser=argparse.ArgumentParser()
parser.add_argument('--revision',choices=['r01','r02','r03','r04','r05','r06'],default='r01')
args=parser.parse_args()
path=DOC/f'generation-creatures-bulk-{args.revision}.json'
old=json.loads(path.read_text('utf8')) if path.exists() else []
by_id={r['id']:r for r in old}
for row in rows:
    target=Path(row['selected_file']).resolve()
    assert target.parent==SOURCE.resolve(),target
    original=Path(row['original_generated_file'])
    digest=hashlib.sha256(original.read_bytes()).hexdigest()
    if row['id'] in by_id:
        assert by_id[row['id']]['sha256']==digest,'Preserve selected existing source'
        continue
    assert not target.exists(),'Do not overwrite an existing drawing'
    shutil.copyfile(original,target)
    row.update(tool='image_gen integrated',transparent_background=True,sha256=digest)
    old.append(row)
path.write_text(json.dumps(old,ensure_ascii=False,indent=2)+'\n','utf8')
manifest_path=DOC/'bulk-preparation.json'
manifest=json.loads(manifest_path.read_text('utf8'))
for item in manifest['jobs']:
    if any(row['id']==item['id'] for row in old): item['status']='generated_awaiting_checks_and_review'
manifest_path.write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n','utf8')
print(json.dumps({'saved_sources':len(rows),'journal_total':len(old),'runtime_integration':False}))
