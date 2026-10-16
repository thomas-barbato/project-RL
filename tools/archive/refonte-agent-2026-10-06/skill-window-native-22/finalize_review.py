from pathlib import Path
import json

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent
assert (OUT / 'native-capture-validation.json').exists()
assert '3 passed; 0 failed' in (OUT / 'tests.log').read_text('utf-8')
assert 'Finished' in (OUT / 'build.log').read_text('utf-8')
assert not (OUT / 'format.log').read_text('utf-8').strip()
path = BASE / 'skill-icon-review-52/review.json'
review = json.loads(path.read_text('utf-8'))
resolved = {'SG01': 'SG01-C02', 'SG02': 'SG02-C02'}
for c in review['candidates']:
    if c['id'] in resolved:
        c['resolved_accepted_variant'] = resolved[c['id']]
        c['resolved_acceptance_reference'] = 'artifacts/refonte-agent-2026-10-06/skill-icon-review-53/visual-acceptance-01.json'
review['status'] = 'two_revisions_accepted_three_candidates_awaiting_approval'
path.write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', 'utf-8')
path = BASE / 'skill-icon-review-53/index.html'
html = path.read_text('utf-8').replace('Ces deux variantes attendent leur validation.', 'Ces deux variantes sont validées et intégrées. <a href="../skill-window-native-22/index.html">Capture réelle de Grenades</a>.')
html = html.replace('../skill-icon-review-52/index.html', '../skill-icon-review-54/index.html')
path.write_text(html, 'utf-8')
validation = json.loads((OUT / 'integration-validation.json').read_text('utf-8'))
validation.update({'targeted_tests': {'passed': 3, 'failed': 0, 'log': 'tests.log'},
                   'cargo_fmt_check': 'passed', 'cargo_build_locked': 'passed',
                   'native_capture': 'passed', 'native_capture_validation': 'native-capture-validation.json',
                   'pending_review': '../skill-icon-review-54/index.html'})
(OUT / 'validation-summary.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2)+'\n', 'utf-8')
print('Recorded native capture and successful checks; review statuses updated.')
