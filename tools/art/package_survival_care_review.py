"""Lossless native lot 77 in the existing reusable animated viewer."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
REVIEWS=ROOT/'artifacts/refonte-agent-2026-10-06'
REVIEW=REVIEWS/'skill-animation-review-77'
DATA=REVIEWS/'apercu-animations-data'
NATIVE=REVIEW/'_capture'
DATA.mkdir(exist_ok=True)
entries=json.loads((NATIVE/'capture.json').read_text(encoding='utf8'))
assert len(entries)==12
checks=[]
for e in entries:
    t=e['timeline']; heal=e['id']=='SU04'
    assert t[0]['hp']==40 and t[0]['energy']==100 and t[0]['preview']
    assert not t[12]['preview'] and t[12]['turn']==1
    assert t[12]['hp']==(54 if heal else 40)
    assert t[12]['energy']==(94 if heal else 99)
    assert not t[12]['poison'] and not t[12]['burn']
    if e['case']=='ATTENTE':assert t[12]==t[-1]
    elif e['case']=='COMPLET':
        if heal:
            assert t[48]['hp']==68 and t[48]['energy']==88 and t[48]['doses']==2
            assert t[78]['hp']==82 and t[78]['energy']==82 and t[78]['doses']==3 and not t[78]['channel']
            assert t[-1]['hp']==82 and t[-1]['energy']==82
        else:assert all(s['hp']==40 and s['energy']==99 and not s['poison'] and not s['burn'] for s in t[12:])
    else:
        assert t[48]['interrupted'] and t[48]['turn']==2
        assert t[-1]['hp']==(46 if heal else 40) and t[-1]['energy']==(94 if heal else 99)
        if heal:assert not t[48]['channel'] and t[-1]['doses']==1
        else:assert t[48]['poison'] and not t[48]['burn']
    indices=e.pop('indices'); cols=e.pop('native_columns'); count=e['unique_images']; rows=(count+cols-1)//cols
    native=Image.open(NATIVE/e['folder']/'atlas.png').convert('RGB')
    assert native.size==(576*cols,576*rows)
    atlas=Image.new('RGB',native.size); hashes=[]
    for i in range(count):
        y=(rows-1-i//cols)*576
        frame=native.crop(((i%cols)*576,y,(i%cols+1)*576,y+576))
        atlas.paste(frame,((i%cols)*576,(i//cols)*576))
        hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
    name=e['variant']+'.webp'; atlas.save(DATA/name,'WEBP',lossless=True,method=2)
    with Image.open(DATA/name) as packed:
        for i,h in enumerate(hashes):
            frame=packed.crop(((i%cols)*576,(i//cols)*576,(i%cols+1)*576,(i//cols+1)*576)).convert('RGB')
            assert hashlib.sha256(frame.tobytes()).hexdigest()==h
    e.update(atlas='apercu-animations-data/'+name,columns=cols,indices=indices)
    checks.append(dict(variant=e['variant'],frames=len(indices),unique_images=count,atlas_bytes=(DATA/name).stat().st_size,pixel_equality=True,timeline=True,frame_rgb_sha256=hashes))
    print(e['variant'],count,(DATA/name).stat().st_size,flush=True)
(REVIEW/'animations.json').write_text(json.dumps(entries,ensure_ascii=False,separators=(',',':')),encoding='utf8')
template=(ROOT/'tools/art/survival_care_review_template.html').read_text(encoding='utf8')
(REVIEWS/'apercu-animations.html').write_text(template.replace('__ENTRIES__',json.dumps(entries,ensure_ascii=False,separators=(',',':'))),encoding='utf8')
(REVIEW/'validation.json').write_text(json.dumps(dict(native_tile=64,scenes=12,lossless=True,checks=checks),indent=2),encoding='utf8')
print('Current lot atlas MiB:',round(sum(c['atlas_bytes'] for c in checks)/2**20,2))
