"""Record the explicit numeric and visual approval and publish its catalogue rule."""
import hashlib
import json
from pathlib import Path
from apply_centred_damage_policy import apply_policy, BINDINGS, REPLACED

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-73'
CAT=ROOT/'assets/skills/agent/catalogue.json'
def write(p,data):p.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
before=OUT/'catalogue-before-acceptance.json'
if not before.exists():before.write_bytes(CAT.read_bytes())
data=json.loads(CAT.read_text(encoding='utf8'))
apply_policy(data)
write(CAT,data)
cache={}
hashes={}
for folder in ('native-clear-labels','frames'):
    for p in sorted((OUT/folder).rglob('*.png')):
        st=p.stat();identity=(st.st_ino,st.st_size,st.st_mtime_ns)
        if identity not in cache:cache[identity]=hashlib.sha256(p.read_bytes()).hexdigest()
        hashes[str(p.relative_to(OUT)).replace('\\','/')]=cache[identity]
for p in OUT.glob('*-64.gif'):hashes[p.name]=hashlib.sha256(p.read_bytes()).hexdigest()
approval={
 'review_id':'AOE-FALLOFF01','date':'2026-10-14','user_reply':'parfait :) on le fait alors :)',
 'numeric_approved':True,'visual_approved':True,'grenade_square_3x3_approved':True,
 'centre_percent':100,'edge_percent':50,'integer_rounding':'floor before defenses',
 'application':'direct centred explosions and magical damage; metrics follow existing square/disc/diamond geometry',
 'catalogue_bindings':list(BINDINGS),'replaced_reductions':REPLACED,
 'status_damage_duration_chance_and_controls_changed':False,
 'pending_adaptations':['cones','waves','persistent status and ground damage'],
 'artifacts':hashes,
 'dependencies':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in [
   'artifacts/refonte-agent-2026-10-06/skill-animation-review-72/grenade-source.png',
   'artifacts/refonte-agent-2026-10-06/skill-animation-review-72/fragment-burst-source.png',
   'artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png']},
 'catalogue_sha256':hashlib.sha256(CAT.read_bytes()).hexdigest(),
 'runtime_integration':'shared release-capable damage resolver, catalogue and laboratory; refonte combat actions remain to be connected in the scheduled skill integration',
 'saves_changed':False,
}
write(OUT/'visual-and-numeric-acceptance-01.json',approval)
review=json.loads((OUT/'review.json').read_text(encoding='utf8'))
review.update(status='approved',numeric_profile_approved=True,visual_approved=True,
              acceptance='visual-and-numeric-acceptance-01.json',shared_rule_integrated=True)
write(OUT/'review.json',review)
print(f'{len(BINDINGS)} applications directes ; {len(hashes)} captures acceptées protégées')
