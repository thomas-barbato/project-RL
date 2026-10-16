"""Record the explicit six-mine approval; leave both stealth revisions pending."""
from pathlib import Path
import hashlib
import json
import sys

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent
REVIEW = BASE / 'skill-icon-review-50'
LEDGER = BASE / 'skill-icon-acceptance.json'
IDS = ['SM00', 'SM01', 'SM02', 'SM03', 'SM04', 'SM05']

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
    candidates = [c for c in review['candidates'] if c['id'] in IDS]
    assert len(ledger['compact_accepted']) == 81
    assert [c['id'] for c in candidates] == IDS
    assert not set(IDS) & {c['id'] for c in ledger['compact_accepted']}
    write(OUT / 'before-acceptance.json', ledger)
    write(OUT / 'before-catalogue.json', read(ROOT / 'assets/skills/agent/catalogue.json'))
    protected = {f['file']: f['sha256'] for c in ledger['accepted'] + ledger['compact_accepted'] for f in c['files'].values()}
    protected.update({p.relative_to(ROOT).as_posix(): digest(p) for p in (ROOT / 'assets/skills/agent').glob('*.png')})
    for c in review['candidates']:
        for f in [c['master'], *c['files'].values()]:
            protected[(REVIEW / f['file']).relative_to(ROOT).as_posix()] = f['sha256']
    for path, sha in protected.items():
        assert digest(ROOT / path) == sha, path
    write(OUT / 'protected-drawings.json', protected)
    approval = {'review_id': 'SKILLS-IC50-ACCEPT01', 'date': '2026-10-11',
        'user_approval': 'Concernant 84 : fait juste une batterie. \nConcernant 85 : fait une silhouette (dans le meme style que la skill d’invisibilité) mais avec uniquement la silhouette :) \n\nje valide les autres',
        'context': 'Reply to the eight-candidate IC50 board. The exceptions are exactly 84 and 85; the other six drawings are approved.',
        'accepted_compact': [c['variant'] for c in candidates],
        'accepted_numbers': [c['number'] for c in candidates],
        'revision_requested': ['SF01-C01', 'SF02-C01'],
        'scope': 'The six exact mine compact drawings only. No approval of the two revised stealth candidates, copy, mechanics or animations.'}
    write(receipt_path, approval)
    rel = REVIEW.relative_to(ROOT).as_posix()
    for c in candidates:
        entry = {key: c[key] for key in ('id', 'number', 'variant', 'title')}
        entry.update({'kind': 'compact', 'visual_status': 'accepted', 'acceptance_review': 'SKILLS-IC50',
            'acceptance_reference': receipt_path.relative_to(ROOT).as_posix(),
            'files': {size: {'file': rel + '/' + f['file'], 'sha256': f['sha256']} for size, f in c['files'].items()},
            'master': {**c['master'], 'file': rel + '/' + c['master']['file']}})
        ledger['compact_accepted'].append(entry)
        c['visual_status'] = 'accepted'
    for c in review['candidates']:
        if c['id'] not in IDS:
            c['visual_status'] = 'revision_requested'
            c['revision_review'] = 'SKILLS-IC51'
    ledger['compact_accepted_count'] = 87
    write(LEDGER, ledger)
    review['status'] = 'partially_accepted_two_revisions_requested'
    review['acceptance_reference'] = receipt_path.relative_to(ROOT).as_posix()
    write(REVIEW / 'review.json', review)
    validation = read(REVIEW / 'validation.json')
    validation['status'] = review['status']
    validation['approval_reference'] = review['acceptance_reference']
    write(REVIEW / 'validation.json', validation)
    print('Recorded six exact mine compact icon approvals; both stealth revisions remain pending.')
elif sys.argv[1] == 'validate':
    protected = read(OUT / 'protected-drawings.json')
    for path, sha in protected.items():
        assert digest(ROOT / path) == sha, path
    ledger, before = read(LEDGER), read(OUT / 'before-acceptance.json')
    assert ledger['compact_accepted'][:-6] == before['compact_accepted']
    assert ledger['accepted'] == before['accepted']
    assert len(ledger['compact_accepted']) == 87
    assert not {'SF01', 'SF02'} & {c['id'] for c in ledger['compact_accepted']}
    catalogue, old = read(ROOT / 'assets/skills/agent/catalogue.json'), read(OUT / 'before-catalogue.json')
    assert catalogue['skills'] == old['skills'] and catalogue['labels'] == old['labels']
    assert {k: v for k, v in catalogue['art'].items() if k not in IDS} == old['art']
    for c in read(REVIEW / 'review.json')['candidates']:
        if c['id'] in IDS:
            assert digest(ROOT / 'assets/skills/agent' / (c['id'] + '-compact.png')) == c['files']['64']['sha256']
    source = (ROOT / 'src/agent_skill_window.rs').read_text('utf-8')
    assert '"illustration"' not in source.split('fn draw_agent_skill_detail', 1)[1].split('fn draw_agent_skill_values', 1)[0]
    write(OUT / 'integration-validation.json', {'review_id': 'SKILLS-NATIVE20',
        'approved_variants': [c['variant'] for c in read(REVIEW / 'review.json')['candidates'] if c['id'] in IDS],
        'compact_count': 87, 'skill_count': len(catalogue['skills']), 'preserved_drawings': len(protected),
        'exact_accepted_sources_packaged': True, 'prior_ledger_entries_unchanged': True,
        'catalogue_skills_and_labels_unchanged': True, 'two_stealth_revisions_not_integrated': True,
        'descriptions_remain_without_illustrations': True})
    print('Validated six packaged mine icons and all prior drawings.')
else:
    raise ValueError('Expected accept or validate')
