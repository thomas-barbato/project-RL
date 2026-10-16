"""Apply the user's SPECTRAL-RANGE01 decision without changing other skill data."""
import copy
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REFERENCE = 'artifacts/refonte-agent-2026-10-06/review-individual-description-T02-03.json'


def apply_policy(catalogue):
    skill = next(s for s in catalogue['skills'] if s['id'] == 'T02')
    old = skill['parameters']['movement_cells']
    skill['parameters']['movement_cells'] = {
        'kind': 'stepped', 'base': 2, 'every_ranks': 3, 'gain_per_step': 2,
        'rank_offset': 0, 'direction': 'increase',
        'value_scope': old['value_scope'], 'benefit': True,
        'spatial_review': 'ZONE-GROWTH02', 'range_decision': 'SPECTRAL-RANGE01',
        'selection': 'up_to_maximum', 'minimum_selected_cells': 1,
    }
    skill['source'] = {
        'reference': REFERENCE,
        'reference_sha256': hashlib.sha256((ROOT / REFERENCE).read_bytes()).hexdigest(),
    }


if __name__ == '__main__':
    path = ROOT / 'assets/skills/agent/catalogue.json'
    before = json.loads(path.read_text(encoding='utf8'))
    after = copy.deepcopy(before)
    apply_policy(after)
    for old, new in zip(before['skills'], after['skills']):
        if old['id'] != 'T02':
            assert old == new, old['id']
        else:
            assert {k: v for k, v in old.items() if k not in ('parameters', 'source')} == {
                k: v for k, v in new.items() if k not in ('parameters', 'source')}
            assert {k: v for k, v in old['parameters'].items() if k != 'movement_cells'} == {
                k: v for k, v in new['parameters'].items() if k != 'movement_cells'}
    assert before['labels'] == after['labels'] and before['art'] == after['art']
    path.write_text(json.dumps(after, ensure_ascii=False, indent=2) + '\n', encoding='utf8')
    print('SPECTRAL-RANGE01: T02 maximum range doubled; all other skills and parameters preserved.')
