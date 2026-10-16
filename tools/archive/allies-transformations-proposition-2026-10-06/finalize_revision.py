"""Persist all generation provenance and explicit visual revision decisions."""
from pathlib import Path
import json, hashlib, shutil
OUT=Path(__file__).resolve().parent
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
results=[json.loads(p.read_text(encoding='utf-8')) for p in sorted((OUT/'revision-results').glob('*.json'))]
jobs=json.loads((OUT/'revision-prompts.json').read_text(encoding='utf-8'))
extra=json.loads((OUT/'revision-additional-prompts.json').read_text(encoding='utf-8'))
jobs=[e for e in jobs if e.get('revision')!=3]+extra
assert {e['source'] for e in jobs}=={e['source'] for e in results}
by_source={e['source']:e for e in results}
for job in jobs:
 r=by_source[job['source']]
 assert r['status']=='saved' and (OUT/r['source']).exists()
 r['source_sha256']=sha(OUT/r['source']);r['original_sha256']=sha(Path(r['generated_source']))
 assert r['source_sha256']==r['original_sha256']
 job.update(r)
 job['reference_sha256']={p:sha(Path(p)) for p in job['referenced_image_paths']}
(OUT/'revision-prompts.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2),encoding='utf-8')
(OUT/'revision-generation-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2),encoding='utf-8')
catalog=json.loads((OUT/'catalog.json').read_text(encoding='utf-8'))
for e in catalog:
 if e['number'] in (57,59,60):
  path=f'references/{e["number"]:03}-vivant-r1.png'
  shutil.copyfile(OUT/f'revisions/r1/tiles/{e["number"]:03}-vivant.png',OUT/path)
  e['previous_living']=path
 if e['number']==60:
  e['source']='sources/060-r3.png';e['revision']=3
  e['description']=extra[0]['prompt']
  e['review_description']='Châssis industriel carré, capteur encastré, jambes mécaniques courtes et large bouclier ivoire/bleu.'
(OUT/'catalog.json').write_text(json.dumps(catalog,ensure_ascii=False,indent=2),encoding='utf-8')
gallery=(OUT/'gallery.html').read_text(encoding='utf-8')
if 'Version précédente du modèle' not in gallery:
 old='grid.append(c)}document.getElementById'
 new="if(e.previous_living){const old=document.createElement('div');old.className='old';old.append(image(e.previous_living,'Version précédente du modèle'));const note=document.createElement('span');note.textContent='Version précédente / 1:1 natif';old.append(note);c.append(old)}grid.append(c)}document.getElementById"
 assert old in gallery
 (OUT/'gallery.html').write_text(gallery.replace(old,new),encoding='utf-8')
decisions=dict(date='2026-10-06',review_status='awaiting opinion on revised drawings; no game integration',requested_changes={'machines':'No blood, dry wrecks without substituted fluid','57':'Crate on tracks, no face','59':'Replace maintenance robot by armed tracked drone','60':'More visible shield and more industrial robot','66':'Additional robot guardian with shield'},living_redrawn=[57,59,60],living_created=[66],living_preserved=[21,22,23,24,42,43,56,58],organic_drawings_preserved=[9,61,62,63,64,65],gameplay_changes=[])
(OUT/'revision-decisions.json').write_text(json.dumps(decisions,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(dict(images_saved=len(results),numbers=sorted({e['number'] for e in jobs})),ensure_ascii=False))
