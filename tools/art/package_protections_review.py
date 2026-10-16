"""Pack the native preview in the one reusable viewer; no duplicated screenshot archive."""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
REVIEWS = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = REVIEWS / 'skill-animation-review-76'
DATA = REVIEWS / 'apercu-animations-data'
NATIVE = REVIEW / '_capture'
DATA.mkdir(exist_ok=True)
entries = json.loads((NATIVE / 'capture.json').read_text(encoding='utf8'))
assert len(entries) == 16
previous = json.loads((REVIEW/'validation.json').read_text())['checks'] if (REVIEW/'validation.json').exists() else []
checks = []
for e in entries:
    t=e['timeline']; protected=e['case']!='SILENCE'
    assert t[0]['hp']==100 and t[0]['energy']==100
    if protected:
        assert t[18]['remaining']==(4 if e['id']=='B02' else 5)
        assert not t[18]['preview'] and t[18]['energy']==(87 if e['id']=='B02' else 85)
        assert t[55]['remaining']==t[32]['remaining'] and t[55]['hp']==t[32]['hp']
        if e['case']=='PERSISTANT':assert t[32]['damage']==10 and t[32]['reflected']==0
        elif e['id']=='B02':assert t[32]['damage']==9 and t[32]['reflected']==0
        else:assert t[32]['damage']==20 and t[32]['reflected']==(8 if e['case'] in ('MELEE','ATTENTE') else 0)
        if e['case']=='ATTENTE':
            assert t[32]==t[-1]
        else:
            assert t[56]['turn']==1 and t[64]['turn']==2 and t[72]['turn']==3 and t[80]['turn']==4 and t[92]['turn']==5
            assert not t[92]['icon_visible'] and t[96]['reflected']==0
            assert t[96]['damage']==(10 if e['case']=='PERSISTANT' else 20)
    else:assert all(s['energy']==100 and s['hp']==100 and s['cooldown']==0 and not s['icon_visible'] for s in t)
    native_hash=hashlib.sha256((NATIVE/e['folder']/'atlas.png').read_bytes()).hexdigest()
    cached=next((c for c in previous if c['variant']==e['variant'] and c.get('native_sha256')==native_hash),None)
    name=f"{e['variant']}.webp"
    if cached and (DATA/name).exists():
        e.update(atlas=f'apercu-animations-data/{name}',columns=e.pop('native_columns'))
        checks.append(cached)
        print(e['variant'],'unchanged',flush=True)
        continue
    # Capture indices are logical time -> deduplicated native image.
    indices=e.pop('indices'); frames=sorted(set(indices)); positions={n:i for i,n in enumerate(frames)}
    cols=e.pop('native_columns'); rows=(len(frames)+cols-1)//cols
    native=Image.open(NATIVE/e['folder']/'atlas.png').convert('RGB')
    assert native.size==(576*cols,576*rows)
    atlas=Image.new('RGB',native.size)
    hashes=[]
    for i,n in enumerate(frames):
        # Macroquad export_png flips the whole GPU image vertically. Each
        # native scene is upright, but atlas rows are consequently reversed.
        y=(rows-1-i//cols)*576
        im=native.crop(((i%cols)*576,y,(i%cols+1)*576,y+576))
        atlas.paste(im,((i%cols)*576,(i//cols)*576))
        hashes.append(hashlib.sha256(im.tobytes()).hexdigest())
    name=f"{e['variant']}.webp"
    atlas.save(DATA/name,'WEBP',lossless=True,method=3)
    # Validate the atlas is lossless and every source frame remains at native size.
    with Image.open(DATA/name) as packed:
        for i,expected in enumerate(hashes):
            im=packed.crop(((i%cols)*576,(i//cols)*576,(i%cols+1)*576,(i//cols+1)*576)).convert('RGB')
            assert hashlib.sha256(im.tobytes()).hexdigest()==expected
    e.update(atlas=f'apercu-animations-data/{name}',columns=cols,indices=[positions[n] for n in indices])
    checks.append(dict(variant=e['variant'],frames=len(indices),unique_images=len(frames),atlas_bytes=(DATA/name).stat().st_size,pixel_equality=True,timeline=True,frame_rgb_sha256=hashes,native_sha256=native_hash))
    print(e['variant'],len(frames),(DATA/name).stat().st_size,flush=True)

(REVIEW/'animations.json').write_text(json.dumps(entries,ensure_ascii=False,separators=(',',':')),encoding='utf8')
template=(ROOT/'tools/art/protections_review_template.html').read_text(encoding='utf8')
(REVIEWS/'apercu-animations.html').write_text(template.replace('__ENTRIES__',json.dumps(entries,ensure_ascii=False,separators=(',',':'))),encoding='utf8')
(REVIEW/'validation.json').write_text(json.dumps(dict(native_tile=64,source='Macroquad laboratory, same scene as interactive lab',scenes=16,lossless=True,checks=checks),ensure_ascii=False,indent=2),encoding='utf8')
print('Total retained atlas MiB:',round(sum(c['atlas_bytes'] for c in checks)/2**20,2))
