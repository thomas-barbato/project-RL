"""Lossless ANIM-S40 playback in the reusable viewer; no source-art changes."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
REVIEW=BASE/'skill-animation-review-85'
assert not (REVIEW/'visual-acceptance-01.json').exists()
entries=json.loads((REVIEW/'_capture/capture.json').read_text(encoding='utf8'))
assert len(entries)==14
checks=[]
for e in entries:
    before=e['timeline'][0];first=e['timeline'][18];last=e['timeline'][-1]
    magic=e['id']=='A03' and e['case']!='HORS'
    enabled=e['case'] not in ('SANS','HORS')
    cost=(5 if enabled else 9) if magic else 0
    damage=32 if e['id']=='M03' and enabled else 20
    count=1 if e['case']=='ATTENTE' else 3
    assert before['energy']==100 and before['target_hp']==100 and before['actions']==0
    assert first['energy']==100-cost and first['target_hp']==100-damage
    assert last['energy']==100-count*cost and last['target_hp']==100-count*damage and last['actions']==count
    assert all(t['healed']==0 and not t['temporary_buff'] for t in e['timeline'])
    if e['case']=='ATTENTE':assert all(t==first for t in e['timeline'][18:])
    cols=e.pop('native_columns');n=e['unique_images'];rows=(n+cols-1)//cols
    with Image.open(REVIEW/'_capture'/e['folder']/'atlas.png') as source:
        source=source.convert('RGB');atlas=Image.new('RGB',source.size);hashes=[]
        for i in range(n):
            x=i%cols*576;y=(rows-1-i//cols)*576;frame=source.crop((x,y,x+576,y+576))
            atlas.paste(frame,(x,i//cols*576));hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
    name='apercu-animations-data/'+e['variant']+'.webp';atlas.save(BASE/name,'WEBP',lossless=True,method=1)
    with Image.open(BASE/name) as packed:
        for i,h in enumerate(hashes):
            x=i%cols*576;y=i//cols*576
            assert hashlib.sha256(packed.crop((x,y,x+576,y+576)).convert('RGB').tobytes()).hexdigest()==h
    e.update(atlas=name,columns=cols)
    checks.append({'variant':e['variant'],'frames':e['frames'],'unique_images':n,'frame_rgb_sha256':hashes,'pixel_equality':True,'timeline':True,'atlas_bytes':(BASE/name).stat().st_size})
    print(e['variant'],n,(BASE/name).stat().st_size,flush=True)
for p in json.loads((REVIEW/'previous-review-preservation.json').read_text(encoding='utf8')):
    assert hashlib.sha256((ROOT/p['path']).read_bytes()).hexdigest()==p['sha256']
text=json.dumps(entries,ensure_ascii=False,separators=(',',':'))
(REVIEW/'animations.json').write_text(text,encoding='utf8')
(REVIEW/'validation.json').write_text(json.dumps({'native_tile':64,'scenes':14,'lossless':True,'previous_review_and_art_preserved':True,'checks':checks},indent=2)+'\n',encoding='utf8')
template=(ROOT/'tools/art/power_passives_review_template.html').read_text(encoding='utf8')
(BASE/'apercu-animations.html').write_text(template.replace('__ENTRIES__',text),encoding='utf8')
print('Playback MiB:',round(sum(c['atlas_bytes'] for c in checks)/2**20,2))
