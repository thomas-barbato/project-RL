"""Accept the ray portion of review 33; retain the requested siphon revision."""
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-33'
receipt=OUT/'ray-acceptance-01.json'
assert not receipt.exists()
review=json.loads((OUT/'review.json').read_text(encoding='utf-8'))
ray=next(c for c in review['candidates'] if c['id']=='OC05')
spirit=next(c for c in review['candidates'] if c['id']=='HY05')
assert ray['variant']=='OC05-A01' and spirit['variant']=='HY05-A01'
files=['ray-source.png','ray-atlas.png','render-source.rs','clip-source.rs','verification.json','native/capture.json','index.html','drains-haut-64.gif']
files+=sorted(p.name for p in OUT.glob('OC05-A01-*-64.gif'))
receipt.write_text(json.dumps({'date':'2026-10-13','user_reply':"alors c'est tres bien, mais j'aimerais que la ponction soit plus epaisse :) car là elle est difficilement lisible :)",
    'accepted':{'number':24,'id':'OC05','variant':'OC05-A01','status':'visually_approved'},
    'scope':'Positive feedback on the shown lot, with the revision explicitly limited to Ponction spirituelle. Rayon vorace keeps its shown visual assembly; no new mechanics or campaign integration are approved.',
    'revision_requested':{'number':23,'id':'HY05','variant':'HY05-A01','request':'Thicker travelling essence for readability; submit HY05-A02 separately.'},
    'artifact_hashes':{name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
ray['status']='visually_approved';ray['acceptance']=receipt.name
spirit['status']='revision_requested';spirit['next_variant']='HY05-A02'
review['status']='ray_approved_siphon_revision_requested';review['ray_acceptance']=receipt.name
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
validation['visual_approval']='ray_approved_siphon_revision_requested';validation['ray_acceptance']=receipt.name
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'approved':'OC05-A01','revision_requested':'HY05-A02','campaign_integration':False}))
