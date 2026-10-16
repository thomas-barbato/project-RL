"""Reuse unchanged A03 Fire/Gas native views; no recapture or repainting."""
import hashlib
import json
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT/'artifacts/refonte-agent-2026-10-06'
source_review = int(sys.argv[1]) if len(sys.argv)>1 else 67
dest_review = int(sys.argv[2]) if len(sys.argv)>2 else 68
assert source_review!=dest_review
old = BASE/f'skill-animation-review-{source_review}/native'
new = BASE/f'skill-animation-review-{dest_review}/native'
assert new.resolve().is_relative_to(ROOT.resolve())
entries = json.loads((new/'capture.json').read_text(encoding='utf8'))
assert len(entries)==2 and all(e['type']=='FROID' for e in entries)
reused=[]
count=0
for e in json.loads((old/'capture.json').read_text(encoding='utf8')):
    if e['type']=='FROID':
        continue
    dest=new/e['folder']
    dest.mkdir()
    for source in (old/e['folder']).iterdir():
        assert source.is_file()
        os.link(source,dest/source.name)
        assert (dest/source.name).stat().st_size == source.stat().st_size
        count += source.suffix=='.png'
    # Hash representative preview, blast, residue and expiry frames.
    proof={str(n): hashlib.sha256((dest/f'{n:03}.png').read_bytes()).hexdigest() for n in (14,116,132,164,464)}
    reused.append({'view':e['variant'],'from_review':source_review,'readback_pixels_unchanged':True,'sha256':proof})
    entries.append(e)
entries.sort(key=lambda e:({'FEU':0,'POISON':1,'FROID':2}[e['type']],e['case']=='CLOISON'))
(new/'capture.json').write_text(json.dumps(entries,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
(new.parent/'reused-views.json').write_text(json.dumps({'native_PNGs_reused':count,'method':'Exact original files via NTFS hardlinks','views':reused},indent=2)+'\n',encoding='utf8')
print(count,'native Fire/Gas PNGs reused')
