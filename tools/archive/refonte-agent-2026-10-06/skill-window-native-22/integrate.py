"""Record approval of the two exact cylindrical grenades presented in IC53."""
from pathlib import Path
import hashlib
import json
import sys

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent
REVIEW = BASE / 'skill-icon-review-53'
LEDGER = BASE / 'skill-icon-acceptance.json'
IDS = ['SG01', 'SG02']
VARIANTS = ['SG01-C02', 'SG02-C02']
PENDING = {'SG00', 'SG03', 'SG04'}


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
    assert len(ledger['compact_accepted']) == 89
    assert [c['id'] for c in review['candidates']] == IDS
    assert [c['variant'] for c in review['candidates']] == VARIANTS
    assert not set(IDS) & {c['id'] for c in ledger['compact_accepted']}
    write(OUT / 'before-acceptance.json', ledger)
    write(OUT / 'before-catalogue.json', read(ROOT / 'assets/skills/agent/catalogue.json'))
    protected = {f['file']: f['sha256'] for c in ledger['accepted'] + ledger['compact_accepted'] for f in c['files'].values()}
    protected.update({p.relative_to(ROOT).as_posix(): digest(p) for p in (ROOT / 'assets/skills/agent').glob('*.png')})
    protected.update({(BASE / 'skill-icon-review-52' / p).relative_to(ROOT).as_posix(): sha for p, sha in read(REVIEW / 'prior-grenade-drawings.json').items()})
    for c in review['candidates']:
        for f in [c['master'], *c['files'].values()]:
            protected[(REVIEW / f['file']).relative_to(ROOT).as_posix()] = f['sha256']
    for path, sha in protected.items():
        assert digest(ROOT / path) == sha, path
    write(OUT / 'protected-drawings.json', protected)
    approval = {'review_id': 'SKILLS-IC53-ACCEPT01', 'date': '2026-10-11',
        'user_approval': 'parfait, je valide :)',
        'context': 'Reply to the final IC53 board showing only the off-white cylindrical SG01-C02 and petrol-blue cylindrical SG02-C02.',
        'accepted_compact': VARIANTS, 'accepted_numbers': [93, 94],
        'scope': 'These two exact compact drawings and colors only. No approval of SG00, SG03, SG04 candidates, copy, mechanics or animations.'}
    write(receipt_path, approval)
    rel = REVIEW.relative_to(ROOT).as_posix()
    for c in review['candidates']:
        entry = {key: c[key] for key in ('id', 'number', 'variant', 'title')}
        entry.update({'kind': 'compact', 'visual_status': 'accepted', 'acceptance_review': 'SKILLS-IC53',
            'acceptance_reference': receipt_path.relative_to(ROOT).as_posix(),
            'files': {size: {'file': rel + '/' + f['file'], 'sha256': f['sha256']} for size, f in c['files'].items()},
            'master': {**c['master'], 'file': rel + '/' + c['master']['file']}})
        ledger['compact_accepted'].append(entry)
        c['visual_status'] = 'accepted'
    ledger['compact_accepted_count'] = 91
    write(LEDGER, ledger)
    review['status'] = 'accepted'
    review['acceptance_reference'] = receipt_path.relative_to(ROOT).as_posix()
    write(REVIEW / 'review.json', review)
    validation = read(REVIEW / 'validation.json')
    validation['status'] = 'accepted'
    validation['approval_reference'] = review['acceptance_reference']
    write(REVIEW / 'validation.json', validation)
    print('Recorded two exact cylindrical grenade compact icon approvals.')
elif sys.argv[1] == 'validate':
    protected = read(OUT / 'protected-drawings.json')
    for path, sha in protected.items():
        assert digest(ROOT / path) == sha, path
    ledger, before = read(LEDGER), read(OUT / 'before-acceptance.json')
    assert ledger['compact_accepted'][:-2] == before['compact_accepted']
    assert ledger['accepted'] == before['accepted']
    assert len(ledger['compact_accepted']) == 91
    assert [c['variant'] for c in ledger['compact_accepted'][-2:]] == VARIANTS
    assert not PENDING & {c['id'] for c in ledger['compact_accepted']}
    catalogue, old = read(ROOT / 'assets/skills/agent/catalogue.json'), read(OUT / 'before-catalogue.json')
    assert catalogue['skills'] == old['skills'] and catalogue['labels'] == old['labels']
    assert {k: v for k, v in catalogue['art'].items() if k not in IDS} == old['art']
    for c in read(REVIEW / 'review.json')['candidates']:
        assert digest(ROOT / 'assets/skills/agent' / (c['id'] + '-compact.png')) == c['files']['64']['sha256']
    source = (ROOT / 'src/agent_skill_window.rs').read_text('utf-8')
    assert '"illustration"' not in source.split('fn draw_agent_skill_detail', 1)[1].split('fn draw_agent_skill_values', 1)[0]
    write(OUT / 'integration-validation.json', {'review_id': 'SKILLS-NATIVE22',
        'approved_variants': VARIANTS, 'compact_count': 91, 'skill_count': len(catalogue['skills']),
        'preserved_drawings': len(protected), 'exact_accepted_sources_packaged': True,
        'prior_ledger_entries_unchanged': True, 'catalogue_skills_and_labels_unchanged': True,
        'remaining_grenade_candidates_not_integrated': True, 'prior_grenade_drawings_unchanged': True,
        'descriptions_remain_without_illustrations': True})
    print('Validated two packaged cylindrical grenades and all prior drawings.')
else:
    raise ValueError('Expected accept or validate')
