"""Record the approval of the three remaining compact grenade icons in IC54."""
from pathlib import Path
import hashlib
import json
import sys

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent
REVIEW = BASE / 'skill-icon-review-54'
LEDGER = BASE / 'skill-icon-acceptance.json'
IDS = ['SG00', 'SG03', 'SG04']
VARIANTS = ['SG00-C01', 'SG03-C01', 'SG04-C01']


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
    assert len(ledger['compact_accepted']) == 91
    assert [c['id'] for c in review['candidates']] == IDS
    assert [c['variant'] for c in review['candidates']] == VARIANTS
    assert all(c['visual_status'] == 'awaiting_visual_review' for c in review['candidates'])
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
    approval = {'review_id': 'SKILLS-IC54-ACCEPT01', 'date': '2026-10-11',
        'user_approval': 'parfait je valide',
        'context': 'Reply to the IC54 board showing only 92 Lancer de grenade, 95 Portée de lancer and 96 Puissance des grenades, unchanged from IC52.',
        'accepted_compact': VARIANTS, 'accepted_numbers': [92, 95, 96],
        'scope': 'These three exact compact drawings only. Copy, gameplay effects and animations remain unchanged.'}
    write(receipt_path, approval)
    rel = REVIEW.relative_to(ROOT).as_posix()
    for c in review['candidates']:
        entry = {key: c[key] for key in ('id', 'number', 'variant', 'title')}
        entry.update({'kind': 'compact', 'visual_status': 'accepted', 'acceptance_review': 'SKILLS-IC54',
            'acceptance_reference': receipt_path.relative_to(ROOT).as_posix(),
            'files': {size: {'file': rel + '/' + f['file'], 'sha256': f['sha256']} for size, f in c['files'].items()},
            'master': {**c['master'], 'file': rel + '/' + c['master']['file']}})
        ledger['compact_accepted'].append(entry)
        c['visual_status'] = 'accepted'
    ledger['compact_accepted_count'] = 94
    write(LEDGER, ledger)
    review['status'] = 'accepted'
    review['acceptance_reference'] = receipt_path.relative_to(ROOT).as_posix()
    write(REVIEW / 'review.json', review)
    validation = read(REVIEW / 'validation.json')
    validation['status'] = 'accepted'
    validation['approval_reference'] = review['acceptance_reference']
    write(REVIEW / 'validation.json', validation)
    print('Recorded approval of the three exact remaining grenade icons.')
elif sys.argv[1] == 'validate':
    protected = read(OUT / 'protected-drawings.json')
    for path, sha in protected.items():
        assert digest(ROOT / path) == sha, path
    ledger, before = read(LEDGER), read(OUT / 'before-acceptance.json')
    assert ledger['compact_accepted'][:-3] == before['compact_accepted']
    assert ledger['accepted'] == before['accepted']
    assert len(ledger['compact_accepted']) == 94
    assert [c['variant'] for c in ledger['compact_accepted'][-3:]] == VARIANTS
    catalogue, old = read(ROOT / 'assets/skills/agent/catalogue.json'), read(OUT / 'before-catalogue.json')
    assert catalogue['skills'] == old['skills'] and catalogue['labels'] == old['labels']
    assert {k: v for k, v in catalogue['art'].items() if k not in IDS} == old['art']
    for c in read(REVIEW / 'review.json')['candidates']:
        assert digest(ROOT / 'assets/skills/agent' / (c['id'] + '-compact.png')) == c['files']['64']['sha256']
    skill_ids = {s['id'] for s in catalogue['skills']}
    approved_ids = {c['id'] for c in ledger['compact_accepted']}
    assert len(skill_ids) == 94 and skill_ids == approved_ids
    assert {k for k, art in catalogue['art'].items() if 'compact' in art} == skill_ids
    source = (ROOT / 'src/agent_skill_window.rs').read_text('utf-8')
    assert '"illustration"' not in source.split('fn draw_agent_skill_detail', 1)[1].split('fn draw_agent_skill_values', 1)[0]
    write(OUT / 'integration-validation.json', {'review_id': 'SKILLS-NATIVE23',
        'approved_variants': VARIANTS, 'compact_count': 94, 'skill_count': len(catalogue['skills']),
        'preserved_drawings': len(protected), 'exact_accepted_sources_packaged': True,
        'prior_ledger_entries_unchanged': True, 'catalogue_skills_and_labels_unchanged': True,
        'every_catalogue_skill_has_an_approved_compact_icon': True,
        'descriptions_remain_without_illustrations': True})
    print('Validated complete coverage: 94 skills, 94 approved compact icons.')
else:
    raise ValueError('Expected accept or validate')
