"""ANIM-S46: compact verified native Guardian/algorithm review."""
import hashlib, html, json, sys
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
REVIEW=BASE/'skill-animation-review-93'
assert not (REVIEW/'visual-acceptance-01.json').exists()
metadata=json.loads((REVIEW/'review.json').read_text(encoding='utf-8'))
assert metadata.get('candidate_status', {}).get('O03-A01') != 'No dedicated animation required by the player; the circuit is removed.', 'Do not republish the removed O03 circuit.'
entries=json.loads((REVIEW/'_capture/capture.json').read_text(encoding='utf-8'))
assert len(entries)==metadata['scenes']==26
checks=[]
repack=next((a.split('=',1)[1] for a in sys.argv[1:] if a.startswith('--repack-case=')),None)
retained={e['variant']:e for e in json.loads((REVIEW/'animations-before-wall.json').read_text(encoding='utf-8'))} if repack else {}
old_checks={e['variant']:e for e in json.loads((REVIEW/'validation-before-wall.json').read_text(encoding='utf-8'))['checks']} if repack else {}
for e in entries:
 before,first,last=(e['timeline'][n] for n in [0,24,143])
 refused=e['id']=='D66' and e['case'] in ['ENERGIE','EXISTANT']
 if refused:
  assert before['refusal'] and not before['preview']
  assert all(t==before for t in e['timeline'])
 else:
  assert first['launched'] and first['deployed'] and not first['preview']
  turns=0 if e['case']=='ATTENTE' else 3
  assert last['turn']==turns
  expected_impacts=0 if e['case']=='CLOISON' or (e['id']=='O03' and e['case']=='SOUTIEN') else 1+turns
  assert last['impacts']==expected_impacts
  if e['id']=='D66':
   assert all(t['energy']==86 and t['deployed'] and t['deployments']==1 and t['self_protected'] and t['player_protected'] for t in e['timeline'][18:])
   assert last['cooldown']==6-turns and last['received_hits']==turns
   protected=e['case'] not in ['CLOISON','HORS_PORTEE']
   assert last['ally_protected']==protected
   assert last['ally_hp']==120-turns*(19 if protected else 20)
  else:
   assert all(t['energy']==(13 if e['case']=='ENERGIE' else 100) and t['cooldown']==0 and t['deployments']==0 and t['passive_bonus_percent']==25 for t in e['timeline'])
   assert last['received_hits']==0 and last['ally_hp']==120
  for t in e['timeline']:
   assert t['enemy_hp']==160-t['impacts']*t['damage_fixture']
   assert t['blood_events']==(0 if e['case']=='MACHINE' else t['impacts'])
   assert t['attacks_scheduled']<=1+t['turn']
  if e['case']=='ATTENTE': assert all(t==e['timeline'][36] for t in e['timeline'][36:])
 cols=e.pop('native_columns');count=e['unique_images'];rows=(count+cols-1)//cols
 if repack and e['case']!=repack:
  old=retained[e['variant']]
  assert all(old[k]==v for k,v in e.items()) and old['columns']==cols
  check=old_checks[e['variant']]
  assert hashlib.sha256((BASE/old['atlas']).read_bytes()).hexdigest()==check['atlas_sha256']
  e.update(atlas=old['atlas'],columns=cols);checks.append(check);continue
 with Image.open(REVIEW/'_capture'/e['folder']/'atlas.ppm') as source:
  source=source.convert('RGB');atlas=Image.new('RGB',source.size);hashes=[]
  for i in range(count):
   x,y=i%cols*576,(rows-1-i//cols)*576
   frame=source.crop((x,y,x+576,y+576));atlas.paste(frame,(x,i//cols*576))
   hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
 name='apercu-animations-data/'+e['variant']+'.webp'
 atlas.save(BASE/name,'WEBP',lossless=True,method=1)
 with Image.open(BASE/name) as packed:
  for i,digest in enumerate(hashes):
   x,y=i%cols*576,i//cols*576
   assert hashlib.sha256(packed.crop((x,y,x+576,y+576)).convert('RGB').tobytes()).hexdigest()==digest
 e.update(atlas=name,columns=cols)
 checks.append({'variant':e['variant'],'pixel_equality':True,'lifecycle_cost_and_hit_checks':True,'frame_rgb_sha256':hashes,'atlas_bytes':(BASE/name).stat().st_size,'atlas_sha256':hashlib.sha256((BASE/name).read_bytes()).hexdigest()})
 print(e['variant'],count,(BASE/name).stat().st_size,flush=True)
for x in json.loads((REVIEW/'previous-review-preservation.json').read_text(encoding='utf-8')):
 assert hashlib.sha256((ROOT/x['path']).read_bytes()).hexdigest()==x['sha256'], x['path']
text=json.dumps(entries,ensure_ascii=False,separators=(',',':'))
(REVIEW/'animations.json').write_text(text,encoding='utf-8')
(REVIEW/'validation.json').write_text(json.dumps({'native_tile':64,'scenes':26,'lossless':True,'previous_sources_preserved':True,'checks':checks},indent=2)+'\n',encoding='utf-8')
template=(ROOT/'tools/art/drone_guardian_review_template.html').read_text(encoding='utf-8')
replacements={'__ENTRIES__':text,'__BATCH__':metadata['batch'],'__SKILL_IDS__':json.dumps([c['id'] for c in metadata['candidates']]),'__VARIANTS__':' et '.join(f"{c['number']} / {c['variant']}" for c in metadata['candidates'])}
for side,c,copy in zip(['LEFT','RIGHT'],metadata['candidates'],metadata['preview_copy']):
 replacements[f'__{side}_TITLE__']=html.escape(f"{c['number']} · {c['title']}")
 replacements[f'__{side}_COPY__']=html.escape(copy)
for key,value in replacements.items():template=template.replace(key,value)
assert '__' not in template
(BASE/'apercu-animations.html').write_text(template,encoding='utf-8')
print('Playback MiB',round(sum(c['atlas_bytes'] for c in checks)/2**20,2))
