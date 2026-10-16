"""Apply the accepted next-action cadence to current catalogue generation."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RECEIPT = ROOT / 'artifacts/refonte-agent-2026-10-06/agent-next-action-cadence-acceptance.json'


def apply(data):
    decision = json.loads(RECEIPT.read_text(encoding='utf-8'))
    assert decision['status'] == 'accepted'
    by_id = {skill['id']: skill for skill in data['skills']}
    for entry in decision['parameters']:
        parameter = by_id[entry['skill']]['parameters'][entry['key']]
        # Preserve all unrelated descriptive and numeric fields.
        for field in ('base', 'gain_per_rank', 'gain_per_step', 'every_ranks',
                      'rank_offset', 'minimum', 'maximum', 'direction'):
            parameter.pop(field, None)
        parameter.update(kind='constant', value=1,
                         cadence_review=decision['review_id'],
                         cadence='next_player_action')
    return data


if __name__ == '__main__':
    catalogue = ROOT / 'assets/skills/agent/catalogue.json'
    data = apply(json.loads(catalogue.read_text(encoding='utf-8')))
    catalogue.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
