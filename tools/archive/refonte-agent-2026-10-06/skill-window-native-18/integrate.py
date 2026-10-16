"""Record scoped approvals and verify the exact accepted drawings."""
from pathlib import Path
import hashlib
import json
import sys

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
ROOT = BASE.parent.parent
REVIEW = BASE / 'skill-icon-review-48'
LEDGER = BASE / 'skill-icon-acceptance.json'

def read(path):
    return json.loads(path.read_text('utf-8-sig'))

def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

if sys.argv[1] == 'accept':
    receipt_path = REVIEW / 'visual-acceptance-01.json'
    assert not receipt_path.exists(), 'Approval already recorded.'
    ledger = read(LEDGER)
    assert len(ledger['compact_accepted']) == 75
    assert all(item['id'] != 'HL04' for item in ledger['compact_accepted'])
    write(OUT / 'before-acceptance.json', ledger)
    write(OUT / 'before-catalogue.json', read(ROOT / 'assets/skills/agent/catalogue.json'))
    (OUT / 'before-ui.rs').write_bytes((ROOT / 'src/agent_skill_window.rs').read_bytes())
    protected = {file['file']: file['sha256'] for item in ledger['accepted'] + ledger['compact_accepted'] for file in item['files'].values()}
    protected.update({p.relative_to(ROOT).as_posix(): digest(p) for p in (ROOT / 'assets/skills/agent').glob('*.png')})
    for path, sha in protected.items():
        assert digest(ROOT / path) == sha, path
    write(OUT / 'protected-drawings.json', protected)
    review = read(REVIEW / 'review.json')
    candidate = review['candidates'][0]
    assert candidate['variant'] == 'HL04-C05' and candidate['number'] == 81
    for item in [candidate['master'], *candidate['files'].values()]:
        assert digest(REVIEW / item['file']) == item['sha256'], item['file']
    approval = {'review_id': 'SKILLS-IC48-ACCEPT01', 'date': '2026-10-11', 'user_approval': 'validée',
                'context': 'Reply to the single numbered IC48 board showing 81 HL04-C05. Ambient browser URL does not determine approval.',
                'accepted_compact': ['HL04-C05'], 'accepted_numbers': [81],
                'scope': 'This exact compact drawing only; no other heavy-weapon icons or new mechanics are approved.'}
    write(receipt_path, approval)
    rel = REVIEW.relative_to(ROOT).as_posix()
    entry = {key: candidate[key] for key in ('id', 'number', 'variant', 'title')}
    entry.update({'kind': 'compact', 'visual_status': 'accepted', 'acceptance_review': 'SKILLS-IC48',
                  'acceptance_reference': (receipt_path.relative_to(ROOT)).as_posix(),
                  'files': {size: {'file': rel + '/' + item['file'], 'sha256': item['sha256']} for size, item in candidate['files'].items()},
                  'master': {**candidate['master'], 'file': rel + '/' + candidate['master']['file']}})
    ledger['compact_accepted'].append(entry)
    ledger['compact_accepted_count'] = len(ledger['compact_accepted'])
    write(LEDGER, ledger)
    candidate['visual_status'] = 'accepted'
    candidate['acceptance_reference'] = approval['review_id']
    review['status'] = 'accepted'
    review['acceptance_reference'] = approval['review_id']
    write(REVIEW / 'review.json', review)
    validation = read(REVIEW / 'validation.json')
    validation['status'] = 'accepted'
    validation['approval_reference'] = receipt_path.relative_to(ROOT).as_posix()
    write(REVIEW / 'validation.json', validation)
    layout_approval = {'date': '2026-10-11', 'user_request': 'en faite on va supprimer les images dans les description des compétences :ca ne sert à rien',
                       'scope': 'Remove all large illustrations from skill descriptions. Keep compact icons in skill trees, actions and status bars.',
                       'status': 'explicitly_requested'}
    write(OUT / 'description-layout-request.json', layout_approval)
    direction_path = BASE / 'skill-icon-direction.json'
    direction = read(direction_path)
    direction['detailed_usage'] = 'Removed from skill descriptions by explicit user request; archived drawings are not displayed there.'
    direction['detailed_layout_status'] = 'removed_by_user'
    direction['detailed_layout_removal_reference'] = (OUT / 'description-layout-request.json').relative_to(ROOT).as_posix()
    write(direction_path, direction)
    print('Recorded approval of HL04-C05 and request to remove description illustrations.')
elif sys.argv[1] == 'validate':
    protected = read(OUT / 'protected-drawings.json')
    for path, sha in protected.items():
        assert digest(ROOT / path) == sha, path
    ledger, before = read(LEDGER), read(OUT / 'before-acceptance.json')
    assert ledger['compact_accepted'][:-1] == before['compact_accepted']
    assert ledger['accepted'] == before['accepted']
    assert len(ledger['compact_accepted']) == 76
    catalogue, old = read(ROOT / 'assets/skills/agent/catalogue.json'), read(OUT / 'before-catalogue.json')
    assert catalogue['skills'] == old['skills'] and catalogue['labels'] == old['labels']
    assert {k: v for k, v in catalogue['art'].items() if k != 'HL04'} == old['art']
    candidate = read(REVIEW / 'review.json')['candidates'][0]
    assert digest(ROOT / 'assets/skills/agent/HL04-compact.png') == candidate['files']['64']['sha256']
    for item in [candidate['master'], *candidate['files'].values()]:
        assert digest(REVIEW / item['file']) == item['sha256'], item['file']
    source = (ROOT / 'src/agent_skill_window.rs').read_text('utf-8')
    detail = source.split('fn draw_agent_skill_detail', 1)[1].split('fn draw_agent_skill_values', 1)[0]
    assert '"illustration"' not in detail
    write(OUT / 'integration-validation.json', {'review_id': 'SKILLS-NATIVE18', 'approved_variant': 'HL04-C05', 'number': 81,
          'compact_count': 76, 'skill_count': len(catalogue['skills']), 'preserved_drawings': len(protected),
          'exact_accepted_source_packaged': True, 'prior_ledger_entries_unchanged': True,
          'catalogue_skills_and_labels_unchanged': True, 'description_illustrations_removed': True})
    print('Validated exact icon, unchanged prior drawings and removal of description illustrations.')
else:
    raise ValueError('Expected accept or validate.')
