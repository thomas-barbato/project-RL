from pathlib import Path
import json

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
review50 = BASE / 'skill-icon-review-50/review.json'
review = json.loads(review50.read_text('utf-8'))
resolved = {'SF01': 'SF01-C02', 'SF02': 'SF02-C03'}
for c in review['candidates']:
    if c['id'] in resolved:
        c['resolved_accepted_variant'] = resolved[c['id']]
        c['resolved_acceptance_reference'] = 'artifacts/refonte-agent-2026-10-06/skill-icon-review-51/visual-acceptance-01.json'
review['status'] = 'six_original_mines_accepted_two_stealth_revisions_resolved'
review50.write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', 'utf-8')
page = BASE / 'skill-icon-review-51/index.html'
html = page.read_text('utf-8').replace('Ces deux variantes attendent leur validation.', 'Ces deux variantes sont validées et intégrées. <a href="../skill-window-native-21/index.html">Capture réelle de Furtivité</a>.')
page.write_text(html, 'utf-8')
print('Archived review statuses updated; approved drawings preserved.')
