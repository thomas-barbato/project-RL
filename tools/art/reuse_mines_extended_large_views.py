"""Retain unchanged radius4 views; paired base targets stay stationary."""
import hashlib,json,os
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
old=BASE/'skill-animation-review-70/native'
new=BASE/'skill-animation-review-71/native'
entries=json.loads((new/'capture.json').read_text(encoding='utf8'))
assert len(entries)==6 and all(e['profile']=='R1' for e in entries)
reused=[]
for e in json.loads((old/'capture.json').read_text(encoding='utf8')):
    if e['profile']!='R4':continue
    dest=new/e['folder'];dest.mkdir()
    for p in (old/e['folder']).iterdir():os.link(p,dest/p.name)
    reused.append({'variant':e['variant'],'source_review':70,'byte_identical':True,'hashes':{str(n):hashlib.sha256((dest/f'{n:03}.png').read_bytes()).hexdigest() for n in (14,92,100,124,180)}})
    entries.append(e)
entries.sort(key=lambda e:({'FEU':0,'POISON':1,'FROID':2}[e['type']],e['case']=='CLOISON',e['profile']=='R4'))
(new/'capture.json').write_text(json.dumps(entries,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
(new.parent/'reused-views.json').write_text(json.dumps(reused,indent=2)+'\n',encoding='utf8')
print(len(reused),'unchanged radius4 views reused')
