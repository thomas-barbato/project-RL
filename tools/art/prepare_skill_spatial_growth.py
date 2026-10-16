"""Apply delegated rank curves, preserve every non-spatial skill parameter."""
import copy
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CAT = ROOT / 'assets/skills/agent/catalogue.json'
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-spatial-review-01'
OUT.mkdir(exist_ok=True)
before_path = OUT / 'catalogue-before.json'
if not before_path.exists():
    before_path.write_bytes(CAT.read_bytes())
before = json.loads(before_path.read_text(encoding='utf8'))
previous_report = OUT / 'progressions.json'
permitted_hashes = {hashlib.sha256(before_path.read_bytes()).hexdigest()}
approved_falloff = OUT.parent/'skill-animation-review-73/visual-and-numeric-acceptance-01.json'
if approved_falloff.exists(): permitted_hashes.add(json.loads(approved_falloff.read_text(encoding='utf8'))['catalogue_sha256'])
if previous_report.exists(): permitted_hashes.add(json.loads(previous_report.read_text(encoding='utf8'))['catalogue_sha256'])
assert hashlib.sha256(CAT.read_bytes()).hexdigest() in permitted_hashes, 'Le catalogue a changé depuis cette passe ; réconcilier avant de relancer.'
after = copy.deepcopy(before)
changes = []

def value(p, r):
    n = max(0, r - 1)
    if p['kind'] == 'constant': v = p['value']
    elif p['kind'] == 'linear': v = p['base'] + n * p['gain_per_rank']
    elif p['kind'] == 'stepped': v = p['base'] + (n + p.get('rank_offset', 0)) // p['every_ranks'] * p['gain_per_step']
    else: raise ValueError(p)
    return int(min(p.get('maximum', v), max(p.get('minimum', v), v)))

def replace(skill, key, gain=1, every=3, base=None):
    params = skill['parameters']
    old = copy.deepcopy(params.get(key))
    base = base if base is not None else value(old, 1)
    new = {'kind':'stepped', 'base':base, 'every_ranks':every, 'gain_per_step':gain,
           'rank_offset':0, 'direction':'increase',
           'value_scope':'rank_component_only;apply_current_accepted_attributes_equipment_and_target_rules_separately',
           'benefit':True, 'spatial_review':'ZONE-GROWTH02'}
    params[key] = new
    changes.append({'id':skill['id'], 'title':skill['title'], 'key':key, 'before':old, 'after':new,
                    'ranks':{str(r):value(new,r) for r in (1,4,7,10,11,13,16,20)}})

direct = {'range_cells','placement_range_cells','center_range_cells','command_range_cells',
          'maximum_drone_distance_cells','maximum_target_distance_cells','cone_range_cells',
          'maximum_wave_travel_cells','bolt_range_cells','drain_range_cells','shadow_step_range_cells',
          'movement_cells','leap_cells','range_skill_limit_cells'}
radius = {'radius_cells','aim_zone_radius_cells','stun_radius_cells','pulse_radius_cells'}
wide = {'zone_width_cells','zone_height_cells','wave_width_cells','maximum_wall_length_cells'}
excluded = {'SM00','SG00'}
for skill in after['skills']:
    if skill['id'] in excluded or skill['kind'] != 'active': continue
    for key in list(skill['parameters']):
        if key in direct: replace(skill, key)
        elif key in radius: replace(skill, key)
        elif key in wide: replace(skill, key, gain=2)
    if skill['branch'] in ('Attaques à distance','Armes lourdes'):
        replace(skill, 'weapon_range_bonus_cells', base=0)
    if skill['id']=='OC04':
        replace(skill, 'ritual_radius_cells', base=1)
        replace(skill, 'segment_count', gain=8)

after['labels']['weapon_range_bonus_cells'] = 'Portée supplémentaire avec l’arme active'
after['labels']['ritual_radius_cells'] = 'Rayon du cercle'
report = {'review_id':'ZONE-GROWTH02', 'date':'2026-10-14',
          'authorization':'SKILL-RANK-DELEGATE01 + ZONE-GROWTH01 + demande de mise en place',
          'numeric_status':'provisional_under_delegated_rank_balancing',
          'vision':{'review_id':'ZONE-VISION02','previous_radius':8,'candidate_radius':12,
                    'campaign_integrated':False,'numeric_and_visual_approval_pending':True},
          'changes':changes,'skill_count':len(after['skills']),
          'changed_skills':len({c['id'] for c in changes}),
          'melee_impact_expansion':False,'ordinary_weapon_range_changed':False,
          'damage_cost_duration_cooldown_and_status_parameters_changed':False,
          'secondary_single_purchase_and_existing_passive_curves_preserved':True,
          'combat_effect_integration':'catalogue et géométrie partagée ; effets de refonte encore à raccorder au combat'}
vision_acceptance = OUT / 'vision-acceptance-01.json'
if vision_acceptance.exists():
    accepted = json.loads(vision_acceptance.read_text(encoding='utf8'))
    if accepted['numeric_radius_approved'] and accepted['displayed_circle_approved']:
        report['vision'].update(campaign_integrated=True,
                                numeric_and_visual_approval_pending=False,
                                integration_scope=accepted['integration_scope'],
                                acceptance=vision_acceptance.name)
changed = {(c['id'],c['key']) for c in changes}
for old,new in zip(before['skills'],after['skills']):
    assert {k:v for k,v in old.items() if k!='parameters'} == {k:v for k,v in new.items() if k!='parameters'}
    for key, p in old['parameters'].items():
        if (old['id'],key) not in changed: assert new['parameters'][key] == p
    assert old['learned_cap'] == new['learned_cap']
    for key in new['parameters']:
        if (new['id'],key) in changed:
            vals=[value(new['parameters'][key],r) for r in range(1,256)]
            assert all(a<=b for a,b in zip(vals,vals[1:])) and vals[13]>vals[10]
            if key in old['parameters']: assert vals[0] == value(old['parameters'][key],1)
if (OUT.parent/'skill-animation-review-73/visual-and-numeric-acceptance-01.json').exists():
    from apply_centred_damage_policy import apply_policy
    apply_policy(after)
    report['accepted_damage_policy_override']='AOE-FALLOFF01'
CAT.write_text(json.dumps(after,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
report['catalogue_sha256'] = hashlib.sha256(CAT.read_bytes()).hexdigest()
(OUT/'progressions.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(f"{report['changed_skills']} compétences, {len(changes)} paramètres spatiaux ; autres paramètres conservés")
