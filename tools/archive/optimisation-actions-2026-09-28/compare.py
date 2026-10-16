import json
from pathlib import Path

root = Path(__file__).parent
before = json.loads((root / 'before/actions.json').read_text())
after = json.loads((root / 'verified/actions.json').read_text())
states_before = [row for row in before if row['operation'] == 'state']
states_after = [row for row in after if row['operation'] == 'state']
assert states_before == states_after, 'Simulation/snapshot divergence'
save_before = json.loads((root / 'before/diagnostic-run.json').read_text())
save_after = json.loads((root / 'verified/diagnostic-run.json').read_text())
different = [key for key in save_before if save_before[key] != save_after[key]]
assert different == ['build'], different
assert save_before.keys() == save_after.keys()

rows = []
for old in before:
    if old['operation'] == 'state':
        continue
    new = next(row for row in after if (row['scene'], row['operation']) == (old['scene'], old['operation']))
    field = 'median_ms' if 'median_ms' in old else 'ms'
    rows.append({'scene': old['scene'], 'operation': old['operation'], 'before_ms': old[field],
                 'after_ms': new[field], 'reduction_percent': (1 - new[field] / old[field]) * 100})
    print(f"{old['scene']:12} {old['operation']:22} {old[field]:9.2f} -> {new[field]:9.2f} ms")
report = {'identical_states': True, 'identical_save_except_build': True, 'rows': rows}
(root / 'comparison.json').write_text(json.dumps(report, indent=2) + '\n')
print('Identical world states, engine snapshots, complete journals and save payloads (except build).')
