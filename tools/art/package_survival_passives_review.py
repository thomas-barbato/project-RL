"""Pack ANIM-S39 native renders losslessly; keep no disposable screenshots."""
import hashlib
import json
import argparse
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
parser=argparse.ArgumentParser()
parser.add_argument('--review',type=int,default=82)
parser.add_argument('--wings-only',action='store_true')
args=parser.parse_args()
assert args.review in (82,84)
REVIEW=BASE/f'skill-animation-review-{args.review}'
assert not (REVIEW/'visual-acceptance-01.json').exists()
entries=json.loads((REVIEW/'_capture/capture.json').read_text(encoding='utf8'))
assert len(entries)==(8 if args.wings_only else 16)
if args.wings_only: assert all(e['id']=='SU06' for e in entries)
if args.wings_only:
    previous=json.loads((BASE/'skill-animation-review-82/animations.json').read_text(encoding='utf8'))
    for entry in entries:
        old=next(e for e in previous if e['id']==entry['id'] and e['case']==entry['case'] and e['direction']==entry['direction'])
        assert entry['timeline']==old['timeline'], 'A visual revision must preserve prepared results.'
checks=[]
old_checks={}
if (REVIEW/'validation.json').exists():
    old_checks={c['variant']:c for c in json.loads((REVIEW/'validation.json').read_text(encoding='utf8'))['checks']}
for e in entries:
    count=e['unique_images'];cols=e.pop('native_columns');rows=(count+cols-1)//cols
    native=Image.open(REVIEW/'_capture'/e['folder']/'atlas.png').convert('RGB')
    assert native.size==(576*cols,576*rows)
    atlas=Image.new('RGB',native.size);hashes=[]
    for i in range(count):
        y=(rows-1-i//cols)*576
        frame=native.crop((i%cols*576,y,(i%cols+1)*576,y+576))
        atlas.paste(frame,(i%cols*576,i//cols*576))
        hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
    name='apercu-animations-data/'+e['variant']+'.webp';output=BASE/name
    previous=old_checks.get(e['variant'])
    if not (output.exists() and previous and previous['frame_rgb_sha256']==hashes):
        atlas.save(output,'WEBP',lossless=True,method=2)
    with Image.open(output) as packed:
        for i,h in enumerate(hashes):
            frame=packed.crop((i%cols*576,i//cols*576,(i%cols+1)*576,(i//cols+1)*576)).convert('RGB')
            assert hashlib.sha256(frame.tobytes()).hexdigest()==h
    e.update(atlas=name,columns=cols)
    if args.wings_only:
        assert e['variant'].startswith('SU06-A03-')
        e['presentation']='rear_halo'
    before,applied,hit,after=(e['timeline'][i] for i in (0,18,30,111))
    assert all(s['healed']==0 and s['energy']==100 and not s['aura'] and not s['invulnerable'] for s in e['timeline'])
    if e['id']=='B03':
        assert before['maximum']==100 and applied['maximum']==150
        assert before['hp']==applied['hp']==60 and applied['turn']==0
        assert after['maximum']==150 and after['saves']==0
        assert after['hp']==(60 if e['case']=='ATTENTE' else 25 if e['case']=='SECOND' else 30)
    else:
        assert all(s['maximum']==100 for s in e['timeline'])
        if e['case']=='PERSISTANT':
            assert hit['hp']==0 and hit['saves']==0 and hit['cooldown']==0
        elif e['case']=='RECHARGE':
            assert before['cooldown']==3 and hit['hp']==0 and hit['saves']==0 and after['cooldown']==2
        else:
            assert hit['hp']==1 and hit['saves']==1 and hit['cooldown']==60 and hit['damage']==19
            if e['case']=='SECOND':assert after['hp']==0 and after['saves']==1 and after['cooldown']==59
            elif e['case']=='ATTENTE':assert after==hit
            else:assert after['hp']==1 and after['cooldown']==57
    checks.append(dict(variant=e['variant'],frames=e['frames'],unique_images=count,pixel_equality=True,
                       timeline=True,atlas_bytes=output.stat().st_size,frame_rgb_sha256=hashes))
    print(e['variant'],count,output.stat().st_size,flush=True)
text=json.dumps(entries,ensure_ascii=False,separators=(',',':'))
(REVIEW/'animations.json').write_text(text,encoding='utf8')
(REVIEW/'validation.json').write_text(json.dumps(dict(native_tile=64,scenes=len(entries),lossless=True,
    no_healing=True,passives_not_active_commands=True,campaign_integration=False,checks=checks),indent=2),encoding='utf8')
template_name='survival_wings_review_template.html' if args.wings_only else 'survival_passives_review_template.html'
template=(ROOT/'tools/art'/template_name).read_text(encoding='utf8')
(BASE/'apercu-animations.html').write_text(template.replace('__ENTRIES__',text),encoding='utf8')
print('Packed MiB:',round(sum(c['atlas_bytes'] for c in checks)/2**20,2))
