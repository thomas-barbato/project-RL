"""Record the selected purple reference without approving unrelated visual rules."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-32'
receipt = OUT/'palette-acceptance-01.json'
assert not receipt.exists(), 'Preserve the existing palette decision.'
review = json.loads((OUT/'review.json').read_text(encoding='utf-8'))
assert {c['variant'] for c in review['candidates']} == {'A01-A04','A02-A04'}
files = ['bolt-source.png','bolt-atlas.png','burst-source.png','burst-atlas.png',
         'render-source.rs','clip-source.rs','index.html','gallery-script.js',
         'verification.json','contrast-verification.json','native/capture.json']
files += sorted(p.name for p in OUT.glob('*.gif'))
proof = {name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
decision = {
    'date':'2026-10-13',
    'user_reply':'ca me semble mieux oui',
    'reviewed_page':'skill-animation-review-32/occulte-haut-64.gif',
    'decision':'palette_retained_for_continuation',
    'reference':'ANIM-S10-COLOR03',
    'variants':['A01-A04','A02-A04'],
    'palette':'Bright saturated purple, without lavender or near-white color lifting.',
    'scope':'Color reference for continuing occult animation reviews. This affirmative feedback is not expanded into approval of all animations, mechanical effects, texts, icons or campaign integration.',
    'shader_option':'--occulte-violet-franc',
    'shader_rgb_anchors':[[0.65,0.15,1.0],[0.80,0.30,1.0]],
    'artifact_hashes':proof,
}
receipt.write_text(json.dumps(decision,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
review['palette_status']='retained_for_continuation'
review['palette_acceptance']=receipt.name
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation['palette_approval']='reference_retained'
validation['palette_acceptance']=receipt.name
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
assert all(hashlib.sha256((OUT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
print(json.dumps({'palette':'ANIM-S10-COLOR03','reference_retained':True,'hashed_artifacts':len(proof),'campaign_integration':False}))
