"""Freeze the visual approval of the stronger drains, without campaign changes."""
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-35'
receipt=OUT/'visual-acceptance-01.json'
assert not receipt.exists(), 'Preserve the existing approval evidence.'
review=json.loads((OUT/'review.json').read_text(encoding='utf-8'))
assert {c['variant'] for c in review['candidates']}=={'HY05-A03','OC05-A02'}
assert all(c['status']=='awaiting_visual_review' for c in review['candidates'])
checks=json.loads((OUT/'intensity-verification.json').read_text(encoding='utf-8'))
assert checks['timelines_identical'] and checks['source_drawings_and_alpha_identical']
assert all(c['new_visible_pixels']>c['old_visible_pixels']*2 for c in checks['larger_visible_effects'])
assert (OUT/'render-source.rs').read_bytes()==(ROOT/'src/agent_skill_lab_review/fire_painted/occult_drains.rs').read_bytes()
files=['spirit-source.png','spirit-atlas.png','ray-source.png','ray-atlas.png',
       'imagegen-spirit-prompt.txt','imagegen-ray-prompt.txt','drawing-pack.json',
       'render-source.rs','clip-source.rs','render-parent-source.rs','index.html',
       'gallery-script.js','verification.json','intensity-verification.json',
       'gallery-controller-verification.json','native/capture.json','Ouvrir_laboratoire.ps1']
files+=sorted(p.name for p in OUT.glob('*.gif'))
files+=sorted(p.relative_to(OUT).as_posix() for p in (OUT/'frames').rglob('*.png'))
files+=sorted(p.relative_to(OUT).as_posix() for p in (OUT/'native').rglob('timeline.json'))
proof={name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({
    'date':'2026-10-13','user_reply':'beaucoup mieux :)',
    'reviewed_page':'skill-animation-review-35/drains-haut-64.gif',
    'accepted':[{'number':c['number'],'id':c['id'],'variant':c['variant'],'status':'visually_approved','native_tile_size':64} for c in review['candidates']],
    'scope':'The strengthened visual assembly shown for both drains: dense moving essence and larger initial shadow for Ponction, thick layered filament and stronger returning pulse for Rayon. No new damage, healing, numerical rules, targeting, AI, status or campaign integration is approved.',
    'timing':'Retain the shown short one-shot Ponction and the turn-driven Rayon with one returning pulse per prepared drain. Visual layers do not add resolutions.',
    'other_candidates':'Other reviews keep their recorded status. Earlier drain variants and their acceptance history remain archived; this selects HY05-A03 and OC05-A02 as the visual references.',
    'artifact_hashes':proof,
},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
for c in review['candidates']:
    c['status']='visually_approved';c['acceptance']=receipt.name
review['status']='visually_approved';review['acceptance']=receipt.name
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation['visual_approval']='approved';validation['acceptance']=receipt.name
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
assert all(hashlib.sha256((OUT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
print(json.dumps({'approved':['HY05-A03','OC05-A02'],'hashed_artifacts':len(proof),'campaign_integration':False}))
