from pathlib import Path
import json

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
assert (OUT / 'native-capture-validation.json').exists()
assert '3 passed; 0 failed' in (OUT / 'tests.log').read_text('utf-8')
assert 'Finished' in (OUT / 'build.log').read_text('utf-8')
assert not (OUT / 'format.log').read_text('utf-8').strip()
path = BASE / 'skill-icon-review-52/review.json'
review = json.loads(path.read_text('utf-8'))
for c in review['candidates']:
    if c['id'] in {'SG00', 'SG03', 'SG04'}:
        c['visual_status'] = 'accepted'
        c['resolved_accepted_variant'] = c['variant']
        c['resolved_acceptance_reference'] = 'artifacts/refonte-agent-2026-10-06/skill-icon-review-54/visual-acceptance-01.json'
review['status'] = 'all_grenade_icons_accepted_two_revisions_resolved'
path.write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', 'utf-8')
path = BASE / 'skill-icon-review-54/index.html'
html = path.read_text('utf-8').replace('Ces trois icônes restent à valider avant intégration.', 'Ces trois icônes sont validées et intégrées. <a href="../skill-window-native-23/index.html">Capture réelle de la branche Grenades complète</a>.')
path.write_text(html, 'utf-8')
validation = json.loads((OUT / 'integration-validation.json').read_text('utf-8'))
validation.update({'targeted_tests': {'passed': 3, 'failed': 0, 'log': 'tests.log'},
                   'cargo_fmt_check': 'passed', 'cargo_build_locked': 'passed',
                   'native_capture': 'passed', 'native_capture_validation': 'native-capture-validation.json',
                   'compact_icon_catalogue_complete': True})
(OUT / 'validation-summary.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2)+'\n', 'utf-8')
print('Recorded complete compact icon catalogue, native capture and successful checks.')
