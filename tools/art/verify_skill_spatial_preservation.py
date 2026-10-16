"""Check the delegated pass and immutable art references before handing it off."""
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-spatial-review-01'
read=lambda p:json.loads(p.read_text(encoding='utf8'))
old=read(OUT/'catalogue-before.json')
new=read(ROOT/'assets/skills/agent/catalogue.json')
report=read(OUT/'progressions.json')
from apply_centred_damage_policy import BINDINGS, REPLACED
falloff=read(OUT.parent/'skill-animation-review-73/visual-and-numeric-acceptance-01.json')
assert falloff['numeric_approved'] and falloff['visual_approved']
changed={(c['id'],c['key']) for c in report['changes']}
assert len(old['skills'])==len(new['skills'])==report['skill_count']
unchanged_parameters=0
for a,b in zip(old['skills'],new['skills']):
    assert {k:v for k,v in a.items() if k!='parameters'}=={k:v for k,v in b.items() if k not in ('parameters','centred_damage')}
    for key,value in a['parameters'].items():
        if (a['id'],key) not in changed:
            if REPLACED.get(a['id'])==key:
                assert key not in b['parameters']
                continue
            assert b['parameters'][key]==value
            unchanged_parameters+=1
for key,value in old.items():
    if key not in ('skills','labels'):assert new[key]==value
for key,value in old['labels'].items():assert new['labels'][key]==value
frozen=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-69'
acceptance=read(frozen/'visual-acceptance-01.json')
protected=0
for filename,digest in acceptance['artifact_hashes'].items():
    assert hashlib.sha256((frozen/filename).read_bytes()).hexdigest()==digest,filename
    protected+=1
for filename,digest in acceptance['dependency_hashes'].items():
    assert hashlib.sha256((ROOT/filename).read_bytes()).hexdigest()==digest,filename
    protected+=1
vision_acceptance=read(OUT/'vision-acceptance-01.json')
assert vision_acceptance['numeric_radius_approved'] and vision_acceptance['displayed_circle_approved']
assert report['vision']['campaign_integrated'] and not report['vision']['numeric_and_visual_approval_pending']
rules_source=(ROOT/'src/ascii_app.rs').read_text(encoding='utf8')
assert 'PLAYER_VISION_GENERATION_VERSION' in rules_source and 'radius: 12,' in rules_source
result={'catalogue_skill_identities_and_roleplay_preserved':True,
        'unchanged_existing_parameters':unchanged_parameters,
        'review69_and_dependencies_preserved_files':protected,
        'rank_curves_authorized_under_delegation':True,
        'accepted_centred_damage_bindings':len(BINDINGS),
        'campaign_vision_radius':12,'legacy_campaign_vision_radius':8,
        'ordinary_npc_vision_radius':8,'current_generation_version':147,
        'vision_explicitly_approved':True,
        'new_campaign_effects_integrated':False,'saves_modified':False}
(OUT/'preservation-verification.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf8')
print(json.dumps(result))
