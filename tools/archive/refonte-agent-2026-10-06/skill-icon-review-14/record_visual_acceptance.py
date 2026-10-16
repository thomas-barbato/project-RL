"""Record the player's approval of compact drawings 21-28 without changing rules."""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
BASE = HERE.parent
IDS = ['D56', 'D57', 'D58', 'D66', 'DP01', 'DR01', 'DP02', 'HY07']


def read(path):
    return json.loads(path.read_text('utf-8-sig'))


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', 'utf-8')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    receipt = HERE / 'visual-acceptance-01.json'
    if receipt.exists():
        raise RuntimeError('This approval has already been recorded; keep the accepted drawings frozen.')
    review = read(HERE / 'review.json')
    ledger_path = BASE / 'skill-icon-acceptance.json'
    ledger = read(ledger_path)
    assert [c['id'] for c in review['candidates']] == IDS
    assert [c['number'] for c in review['candidates']] == list(range(21, 29))
    assert not set(IDS) & {c['id'] for c in ledger['compact_accepted']}
    previous_exports = [f for c in ledger['accepted'] + ledger['compact_accepted'] for f in c['files'].values()]
    for entry in previous_exports:
        assert digest(ROOT / entry['file']) == entry['sha256'], entry['file']
    for c in review['candidates']:
        for entry in list(c['files'].values()) + [c['master']]:
            assert digest(HERE / entry['file']) == entry['sha256'], entry['file']
    runtime = ROOT / 'assets/skills/agent'
    write(HERE / 'integration-baseline.json', {
        'previous_accepted': ledger['accepted'],
        'previous_compact_accepted': ledger['compact_accepted'],
        'previous_export_count': len(previous_exports),
        'runtime_files': [{'file': p.relative_to(ROOT).as_posix(), 'sha256': digest(p)}
                          for p in sorted(runtime.rglob('*')) if p.is_file()],
        'catalogue': read(runtime / 'catalogue.json'),
    })
    approval = {
        'review_id': 'SKILLS-IC14-ACCEPT01', 'date': '2026-10-11',
        'user_approval': 'ok oui',
        'approval_context': 'Reply to the numbered native-size gallery of eight compact candidates 21-28.',
        'accepted_compact': [c['variant'] for c in review['candidates']],
        'accepted_numbers': list(range(21, 29)),
        'scope': 'The eight compact drawings in IC14. No gameplay, skill copy, illustrations, other pending candidates or new layout approval inferred.',
    }
    acceptance_reference = receipt.relative_to(ROOT).as_posix()
    for c in review['candidates']:
        files = {size: {'file': (HERE / f['file']).relative_to(ROOT).as_posix(), 'sha256': f['sha256']}
                 for size, f in sorted(c['files'].items(), key=lambda item: int(item[0]))}
        ledger['compact_accepted'].append({
            'id': c['id'], 'variant': c['variant'], 'number': c['number'],
            'kind': 'compact', 'title': c['title'], 'visual_status': 'accepted',
            'acceptance_review': 'SKILLS-IC14', 'acceptance_reference': acceptance_reference,
            'files': files, 'master': {**c['master'], 'file': (HERE / c['master']['file']).relative_to(ROOT).as_posix()},
        })
        c['visual_status'] = 'accepted'
        c['acceptance_reference'] = acceptance_reference
    ledger['date'] = approval['date']
    ledger['compact_accepted_count'] = len(ledger['compact_accepted'])
    ledger['accepted_drawing_count'] = len(ledger['accepted']) + len(ledger['compact_accepted'])
    review['status'] = 'accepted'
    review['acceptance_reference'] = acceptance_reference
    write(receipt, approval)
    write(ledger_path, ledger)
    write(HERE / 'review.json', review)
    direction_path = BASE / 'skill-icon-direction.json'
    direction = read(direction_path)
    assert direction.pop('pending_compact_batch_review') == 'SKILLS-IC14'
    batch = direction.pop('pending_compact_batch_candidates')
    direction['accepted_compact_batch_review'] = 'SKILLS-IC14'
    direction['accepted_compact_batch_candidates'] = [
        {**c, 'visual_status': 'accepted', 'acceptance_reference': acceptance_reference} for c in batch]
    write(direction_path, direction)
    page = HERE / 'index.html'
    page.write_text(page.read_text('utf-8').replace('PROPOSITIONS 21–28', 'COMPACTES VALIDÉES 21–28')
                    .replace('Dessins proposés à la revue.', 'Huit dessins validés le 11 octobre 2026.'), 'utf-8')
    print(f'Recorded eight accepted compact drawings. Total: {len(ledger["compact_accepted"])} compact, {len(ledger["accepted"])} illustrated.')


if __name__ == '__main__':
    main()
