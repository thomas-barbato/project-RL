"""Record approval of the five exact drawings presented in IC49."""
from pathlib import Path
import hashlib
import json
import sys

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent
REVIEW = BASE / 'skill-icon-review-49'
LEDGER = BASE / 'skill-icon-acceptance.json'
IDS = ['HL01', 'HL02', 'HL03', 'HL05', 'HL06']

def read(path):
    return json.loads(path.read_text('utf-8-sig'))

def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

if sys.argv[1] == 'accept':
    receipt_path = REVIEW / 'visual-acceptance-01.json'
    assert not receipt_path.exists()
    ledger, review = read(LEDGER), read(REVIEW / 'review.json')
    assert len(ledger['compact_accepted']) == 76
    assert [c['id'] for c in review['candidates']] == IDS
    assert not set(IDS) & {c['id'] for c in ledger['compact_accepted']}
    write(OUT / 'before-acceptance.json', ledger)
    write(OUT / 'before-catalogue.json', read(ROOT / 'assets/skills/agent/catalogue.json'))
    protected = {f['file']: f['sha256'] for c in ledger['accepted'] + ledger['compact_accepted'] for f in c['files'].values()}
    protected.update({p.relative_to(ROOT).as_posix(): digest(p) for p in (ROOT / 'assets/skills/agent').glob('*.png')})
    for path, sha in protected.items():
        assert digest(ROOT / path) == sha, path
    write(OUT / 'protected-drawings.json', protected)
    approval = {'review_id': 'SKILLS-IC49-ACCEPT01', 'date': '2026-10-11', 'user_approval': 'parfait on continue',
                'context': 'Reply to the IC49 board showing only the five remaining heavy-weapon icons. Ambient browser URL is unrelated.',
                'accepted_compact': [c['variant'] for c in review['candidates']],
                'accepted_numbers': [c['number'] for c in review['candidates']],
                'scope': 'These five exact compact drawings only. No new copy, effects or next candidates approved.'}
    write(receipt_path, approval)
    rel = REVIEW.relative_to(ROOT).as_posix()
    for c in review['candidates']:
        for f in [c['master'], *c['files'].values()]:
            assert digest(REVIEW / f['file']) == f['sha256'], f['file']
        entry = {key: c[key] for key in ('id', 'number', 'variant', 'title')}
        entry.update({'kind': 'compact', 'visual_status': 'accepted', 'acceptance_review': 'SKILLS-IC49',
                      'acceptance_reference': receipt_path.relative_to(ROOT).as_posix(),
                      'files': {size: {'file': rel + '/' + f['file'], 'sha256': f['sha256']} for size, f in c['files'].items()},
                      'master': {**c['master'], 'file': rel + '/' + c['master']['file']}})
        ledger['compact_accepted'].append(entry)
        c['visual_status'] = 'accepted'
    ledger['compact_accepted_count'] = 81
    write(LEDGER, ledger)
    review['status'] = 'accepted'
    review['acceptance_reference'] = receipt_path.relative_to(ROOT).as_posix()
    write(REVIEW / 'review.json', review)
    validation = read(REVIEW / 'validation.json')
    validation['status'] = 'accepted'
    validation['approval_reference'] = receipt_path.relative_to(ROOT).as_posix()
    write(REVIEW / 'validation.json', validation)
    print('Recorded five exact compact icon approvals.')
elif sys.argv[1] == 'validate':
    protected = read(OUT / 'protected-drawings.json')
    for path, sha in protected.items():
        assert digest(ROOT / path) == sha, path
    ledger, before = read(LEDGER), read(OUT / 'before-acceptance.json')
    assert ledger['compact_accepted'][:-5] == before['compact_accepted']
    assert ledger['accepted'] == before['accepted']
    assert len(ledger['compact_accepted']) == 81
    catalogue, old = read(ROOT / 'assets/skills/agent/catalogue.json'), read(OUT / 'before-catalogue.json')
    assert catalogue['skills'] == old['skills'] and catalogue['labels'] == old['labels']
    assert {k: v for k, v in catalogue['art'].items() if k not in IDS} == old['art']
    for c in read(REVIEW / 'review.json')['candidates']:
        assert digest(ROOT / 'assets/skills/agent' / (c['id'] + '-compact.png')) == c['files']['64']['sha256']
        for f in [c['master'], *c['files'].values()]:
            assert digest(REVIEW / f['file']) == f['sha256'], f['file']
    source = (ROOT / 'src/agent_skill_window.rs').read_text('utf-8')
    assert '"illustration"' not in source.split('fn draw_agent_skill_detail',1)[1].split('fn draw_agent_skill_values',1)[0]
    write(OUT / 'integration-validation.json', {'review_id': 'SKILLS-NATIVE19',
          'approved_variants': [c['variant'] for c in read(REVIEW / 'review.json')['candidates']],
          'compact_count': 81, 'skill_count': len(catalogue['skills']), 'preserved_drawings': len(protected),
          'exact_accepted_sources_packaged': True, 'prior_ledger_entries_unchanged': True,
          'catalogue_skills_and_labels_unchanged': True, 'descriptions_remain_without_illustrations': True})
    print('Validated five packaged icons and all prior drawings.')
else:
    raise ValueError('Expected accept or validate')
