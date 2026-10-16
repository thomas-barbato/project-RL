"""Apply AFFLICTIONS-ZONE01 without enabling unfinished campaign effects."""
import copy
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'artifacts/refonte-agent-2026-10-06'
ACTIVE = ('O02', 'AF02', 'AF03', 'AF04', 'AF05')


def apply_policy(catalogue):
    acceptance = REVIEW / 'afflictions-area-acceptance-01.json'
    approved = json.loads(acceptance.read_text(encoding='utf-8'))
    assert approved['review_id'] == 'AFFLICTIONS-ZONE01-ACCEPT01'
    for skill in catalogue['skills']:
        if skill['id'] not in (*ACTIVE, 'AF06'):
            continue
        passive = skill['id'] == 'AF06'
        skill['parameters']['zone_width_cells'] = {
            'kind': 'stepped', 'base': 5 if passive else 3,
            'every_ranks': 5 if passive else 3, 'gain_per_step': 2,
            'rank_offset': 0, 'direction': 'increase',
            'value_scope': 'rank_component_only;apply_current_accepted_attributes_equipment_and_target_rules_separately',
            'benefit': True, 'spatial_review': 'ZONE-GROWTH02',
            'area_decision': 'AFFLICTIONS-ZONE01',
            **({'maximum': 9} if passive else {}),
        }
        skill['affliction_area'] = {
            'decision': 'AFFLICTIONS-ZONE01', 'shape': 'square',
            'width_parameter': 'zone_width_cells',
            'origin': 'dead_affected_enemy' if passive else 'chosen_visible_cell',
            'recipients': 'all_eligible_hostiles_in_area',
            'status_binding': 'affected_actor', 'persistent_ground': False,
            'individual_resistance_and_immunity': True,
            'obstacles': 'shared_affected_cells',
            'status_potency_falloff': False,
            'propagation_from_transmitted_effect': False if passive else None,
            'acceptance': acceptance.relative_to(ROOT).as_posix(),
            'acceptance_sha256': hashlib.sha256(acceptance.read_bytes()).hexdigest(),
        }


if __name__ == '__main__':
    path = ROOT / 'assets/skills/agent/catalogue.json'
    before = json.loads(path.read_text(encoding='utf-8-sig'))
    after = copy.deepcopy(before)
    apply_policy(after)
    again = copy.deepcopy(after)
    apply_policy(again)
    assert again == after
    for old, new in zip(before['skills'], after['skills']):
        if old['id'] not in (*ACTIVE, 'AF06'):
            assert old == new, old['id']
        else:
            assert {k: v for k, v in old.items() if k not in ('parameters', 'affliction_area')} == {k: v for k, v in new.items() if k not in ('parameters', 'affliction_area')}
            assert {k: v for k, v in old['parameters'].items() if k != 'zone_width_cells'} == {k: v for k, v in new['parameters'].items() if k != 'zone_width_cells'}
    assert before['labels'] == after['labels'] and before['art'] == after['art']
    for skill in after['skills']:
        if skill['id'] not in (*ACTIVE, 'AF06'):
            continue
        p = skill['parameters']['zone_width_cells']
        for rank, width in ([(1, 5), (6, 7), (11, 9), (30, 9)] if skill['id'] == 'AF06' else [(1, 3), (4, 5), (7, 7), (10, 9), (13, 11)]):
            assert min(p.get('maximum', 65535), p['base'] + ((rank - 1) // p['every_ranks']) * p['gain_per_step']) == width
    path.write_text(json.dumps(after, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print('AFFLICTIONS-ZONE01 applied to five active curses and Contagion; idempotence, growth and unrelated content preservation verified.')
