"""Package reviewed skill copy and ONLY accepted drawings for the native UI.

This does not enable combat effects or purchases. Unimplemented skills remain
read-only; their displayed rank is read from the real run, never the HTML demo.
"""
from pathlib import Path
import hashlib
import json
import re
import shutil

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'artifacts/refonte-agent-2026-10-06'
OUT = ROOT / 'assets/skills/agent'


def package():
    OUT.mkdir(parents=True, exist_ok=True)
    source = REVIEW / 'skill-window-review-05/skills.json'
    skills = json.loads(source.read_text('utf-8'))['skills']
    js = (REVIEW / 'skill-window-review-05/review.js').read_text('utf-8')
    labels = {}
    for name in ('LABELS', 'rawLabels'):
        block = re.search(r'const ' + name + r'=\{(.*?)\};', js, re.S)[1]
        labels.update(re.findall(r"(\w+):'([^']*)'", block))
    acceptance = json.loads((REVIEW / 'skill-icon-acceptance.json').read_text('utf-8'))
    art = {}
    frozen = []
    for category, entries, size in [('compact', acceptance['compact_accepted'], '64'),
                                    ('illustration', acceptance['accepted'], '192')]:
        for entry in entries:
            original = entry['files'][size]
            path = ROOT / original['file']
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            assert digest == original['sha256'], path
            filename = entry['id'] + '-' + category + '.png'
            shutil.copyfile(path, OUT / filename)
            art.setdefault(entry['id'], {})[category] = filename
            frozen.append({'skill': entry['id'], 'category': category, 'file': filename,
                           'source': original['file'], 'sha256': digest})
    keys = {key for skill in skills for key in skill['parameters']}
    assert keys <= labels.keys(), sorted(keys - labels.keys())
    # Keep the approved curve objects intact, including their binding scopes.
    data = {'skills': skills, 'labels': labels, 'art': art}
    # Preserve the explicitly requested maximum-distance revision on repackaging.
    if (REVIEW / 'spectral-step-range-and-choice-decisions.json').exists():
        from apply_spectral_step_range_policy import apply_policy
        apply_policy(data)
    if (REVIEW / 'afflictions-area-acceptance-01.json').exists():
        from apply_affliction_area_policy import apply_policy as apply_affliction_area
        apply_affliction_area(data)
    if (REVIEW / 'agent-next-action-cadence-acceptance.json').exists():
        from apply_agent_recast_policy import apply as apply_recast_policy
        apply_recast_policy(data)
    (OUT / 'catalogue.json').write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')
    receipt = {'review_id': 'SKILLS-NATIVE01', 'source': str(source.relative_to(ROOT)),
               'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
               'skills': len(skills), 'drawings': frozen,
               'scope': 'Reviewed catalogue reader; combat effects and purchases are not enabled.'}
    (OUT / 'manifest.json').write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + '\n', 'utf-8')
    print(f"Packaged {len(skills)} skills, {len(frozen)} accepted drawings.")


if __name__ == '__main__':
    package()
