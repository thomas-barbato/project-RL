"""Record the explicit approval of FE04-A05 and turn-driven visual presence."""
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-15'
receipt=OUT/'visual-acceptance-01.json'
assert not receipt.exists(), 'Approval already recorded.'
files=['FE04-A05-TOUR-64.gif','maintien-par-tours-64.gif','render-source.rs','render-parent-source.rs','gallery-script.js']
proof={name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in files}
receipt.write_text(json.dumps({'date':'2026-10-12','user_reply':'parfait je valide et on continue :)','accepted':{'number':12,'skill_id':'FE04','variant':'FE04-A05-TOUR','status':'visually_approved'},'rule':'ANIM-TEMPS01: visual motion in real time; effect presence ends only when its game turns pass. Waiting does not expire effects or cooldowns.','scope':'Corrected visual lifetime and shown preview accepted. Earlier drawing and cadence approvals retained. No change to damage, campaign combat or skill figures.','artifact_hashes':proof},ensure_ascii=False,indent=2),encoding='utf-8')
review=json.loads((OUT/'review.json').read_text(encoding='utf-8'));review['status']='visually_approved';review['acceptance']='visual-acceptance-01.json'
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2),encoding='utf-8')
html=(OUT/'index.html').read_text(encoding='utf-8').replace('correction de durée visuelle','maintien visuel validé')
(OUT/'index.html').write_text(html,encoding='utf-8')
print('FE04-A05 and ANIM-TEMPS01 recorded as approved.')
