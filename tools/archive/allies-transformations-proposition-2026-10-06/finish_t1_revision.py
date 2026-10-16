"""Record exact prompts/provenance, merged review aliases and current decisions."""
from pathlib import Path
import json,hashlib
OUT=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
jobs=json.loads((OUT/'t1-guardian-prompts.json').read_text(encoding='utf-8'))
jobs=[j for j in jobs if j['source']!='sources/059-r4.png']
results=[]
for j in jobs:
 r=json.loads((OUT/f't1-guardian-generation/{j["number"]}.json').read_text(encoding='utf-8'))
 r['source_sha256']=sha(OUT/r['source']);r['original_sha256']=sha(Path(r['generated_source']))
 assert r['source_sha256']==r['original_sha256']
 j.update(r);j['reference_sha256']={p:sha(Path(p)) for p in j['referenced_image_paths']}
 results.append(r)
if (OUT/'t1-cleanup-prompt.json').exists():
 j=json.loads((OUT/'t1-cleanup-prompt.json').read_text(encoding='utf-8'))
 r=json.loads((OUT/'t1-guardian-generation/59-cleanup.json').read_text(encoding='utf-8'))
 r['source_sha256']=sha(OUT/r['source']);r['original_sha256']=sha(Path(r['generated_source']))
 assert r['source_sha256']==r['original_sha256']
 j.update(r);j['reference_sha256']={p:sha(Path(p)) for p in j['referenced_image_paths']}
 j['revision_purpose']='Simplify metal and track surface texture at native32px while retaining T-1 silhouette.'
 jobs.append(j);results.append(r)
(OUT/'t1-guardian-prompts.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2),encoding='utf-8')
(OUT/'t1-guardian-generation-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2),encoding='utf-8')
catalog=json.loads((OUT/'catalog.json').read_text(encoding='utf-8'))
for e in catalog:
 if e['number']==66:e['maximum']=[30,29]
 if e['number']==59 and (OUT/'sources/059-r4.png').exists():e['source']='sources/059-r4.png';e['revision']=4
(OUT/'catalog.json').write_text(json.dumps(catalog,ensure_ascii=False,indent=2),encoding='utf-8')
gallery=(OUT/'gallery.html').read_text(encoding='utf-8')
old="String(e.number).padStart(2,'0').includes(term)||e.name.toLocaleLowerCase('fr').includes(term)"
new="[e.number,...(e.merged_review_numbers||[])].some(n=>String(n).padStart(2,'0').includes(term))||[e.name,...(e.search_aliases||[])].some(n=>n.toLocaleLowerCase('fr').includes(term))"
if old in gallery:gallery=gallery.replace(old,new)
elif new not in gallery:raise AssertionError('Search expression has changed.')
(OUT/'gallery.html').write_text(gallery,encoding='utf-8')
print(json.dumps(dict(generated=[j['number'] for j in jobs],retired=[60],guardian_number=66,preview_only=True),ensure_ascii=False))
