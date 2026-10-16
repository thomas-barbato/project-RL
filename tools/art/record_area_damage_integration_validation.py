import hashlib
import json
from pathlib import Path
from apply_centred_damage_policy import BINDINGS, REPLACED
ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
OUT=BASE/'skill-area-damage-integration-01'
read=lambda p:json.loads(p.read_text(encoding='utf8'))
before=read(BASE/'skill-animation-review-73/catalogue-before-acceptance.json')
after=read(ROOT/'assets/skills/agent/catalogue.json')
for a,b in zip(before['skills'],after['skills']):
    assert {k:v for k,v in a.items()if k!='parameters'}=={k:v for k,v in b.items()if k not in ('parameters','centred_damage')}
    for key,value in a['parameters'].items():
        if REPLACED.get(a['id'])==key:
            assert key not in b['parameters']
        else:
            assert b['parameters'][key]==value
    assert bool(b.get('centred_damage'))==(a['id'] in BINDINGS)
for key,value in before.items():
    if key!='skills':assert after[key]==value
browser=read(OUT/'verification.json')
assert browser['skill_geometry_parity']==48 and browser['native_geometry_fixture_parity']==1152
assert browser['centred_damage_retention_parity']==12530 and not browser['errors']
record={
 'date':'2026-10-14','approval':'../skill-animation-review-73/visual-and-numeric-acceptance-01.json',
 'shared_rule':'src/skills/area_damage.rs','catalogue_bindings':list(BINDINGS),
 'changed_parameter_removals':REPLACED,'all_other_existing_parameters_preserved':True,
 'roleplay_names_icons_and_skill_ids_preserved':True,
 'debug_build':'passed','locked_all_targets_check':'passed','format_check':'passed','scoped_diff_check':'passed',
 'targeted_tests':{'integer_spatial_damage_and_defense_pipeline':6,'catalogue_and_animation_lab':146},
 'stale_nova_test_corrected':'Previous expectation excluded a fifth victim already within the accepted rank-11 radius. No Gel mechanic changed.',
 'native_geometry_fixtures':1152,'browser_retention_values':12530,'native_skill_window_captures':['window-SG00/cold-start.png','window-FE04/cold-start.png','window-FR06/cold-start.png'],
 'approved_review73_images_and_gifs_preserved':4024,'review69_and_dependencies_preserved':8261,
 'browser_errors':[],'save_schema_and_existing_save_files_changed':False,
 'campaign_refonte_actions':'To be connected during the already scheduled combat skill integration after animation reviews.',
 'pending_adaptations':['directional damage','persistent status and ground damage'],
 'remaining_unshown_animations':36,
 'catalogue_sha256':hashlib.sha256((ROOT/'assets/skills/agent/catalogue.json').read_bytes()).hexdigest(),
}
(OUT/'validation.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('Intégration de la règle validée : 10 applications, 152 tests et 12 530 valeurs comparées.')
