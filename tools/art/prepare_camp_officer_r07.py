"""Place the accepted NPC-01 at cryogenics reception, keeping one actor identity."""
import json
from collections import deque
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'assets/prototypes/handcrafted-camp-01/r06'
OUT = ROOT / 'assets/prototypes/handcrafted-camp-01/r07'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def main():
    c = json.loads((BASE / 'camp.json').read_text(encoding='utf-8'))
    officer = next(a for a in c['actors'] if a['id'] == 'NPC-01')
    previous = officer['at']
    officer['at'] = [6, 7]
    assert len([a for a in c['actors'] if a['id'] == 'NPC-01']) == 1
    c.update(revision='r07', officer_previous_at=previous, officer_at=[6, 7],
             officer_role='Accueil du détenu au réveil et présentation de la mission.',
             officer_dialogue_approved=False, officer_dialogue_connected=False,
             new_actor_art=False, new_actor_identity=False,
             capture_targets=[dict(name='camp', at=[2, 8]), dict(name='officier', at=[6, 8]),
                              dict(name='hall', at=[22, 8])])
    rows = (ROOT / c['layout']).read_text(encoding='utf-8').splitlines()
    blocked = {(p['at'][0] + dx, p['at'][1] + dy) for p in c['props']
               for dx in range(p['width']) for dy in range(p['height'])}
    blocked |= {tuple(a['at']) for a in c['actors'][1:]}
    seen = {tuple(c['spawn'])}
    queue = deque(seen)
    while queue:
        x, y = queue.popleft()
        for nx, ny in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
            p = nx, ny
            if 0 <= nx < c['width'] and 0 <= ny < c['height'] and rows[ny][nx] == '.' and p not in blocked and p not in seen:
                seen.add(p)
                queue.append(p)
    for a in c['actors'][1:]:
        x, y = a['at']
        assert any(p in seen for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]), a['id']
    assert all(tuple(p) in seen for p in c['passages'] + c['exits'])
    assert (6, 8) in seen
    c.update(all_npcs_approachable=True, reachable_floor_cells=len(seen))
    OUT.mkdir(parents=True, exist_ok=True)
    data = json.dumps(c, ensure_ascii=False, indent=2) + '\n'
    (OUT / 'camp.json').write_text(data, encoding='utf-8')
    (REVIEW / 'full-camp-r07.json').write_text(data, encoding='utf-8')
    print(json.dumps(dict(officer_at=officer['at'], actor_count=len(c['actors']),
                          artwork_reused=True, passage_open=True, all_npcs_approachable=True)))


if __name__ == '__main__':
    main()
