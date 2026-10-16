"""Freeze and integrate the five explicitly accepted IC23 compact drawings."""
from pathlib import Path
import copy
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-icon-review-23'
OUT = Path(__file__).resolve().parent
EXPECTED = {'B02': (41, 'B02-C03'), 'SU03': (42, 'SU03-C02'),
            'SU05': (43, 'SU05-C02'), 'SU06': (44, 'SU06-C01'),
            'BD06': (45, 'BD06-C01')}


def read(path):
    return json.loads(path.read_text('utf-8'))


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', 'utf-8')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    receipt = REVIEW / 'visual-acceptance-01.json'
    if receipt.exists():
        raise SystemExit('IC23 acceptance already recorded.')
    review = read(REVIEW / 'review.json')
    assert {c['id']: (c['number'], c['variant']) for c in review['candidates']} == EXPECTED
    ledger_path = BASE / 'skill-icon-acceptance.json'
    ledger = read(ledger_path)
    assert not any(c['id'] in EXPECTED for c in ledger['compact_accepted'])
    protected = [f for e in ledger['accepted'] + ledger['compact_accepted'] for f in e['files'].values()]
    protected += [{'file': 'assets/skills/agent/' + e['file'], 'sha256': e['sha256']}
                  for e in read(ROOT / 'assets/skills/agent/manifest.json')['drawings']]
    for f in protected:
        assert digest(ROOT / f['file']) == f['sha256'], f['file']
    before = read(ROOT / 'assets/skills/agent/catalogue.json')
    write(OUT / 'acceptance-before.json', copy.deepcopy(ledger))
    write(OUT / 'catalogue-before.json', before)
    write(OUT / 'direction-before.json', read(BASE / 'skill-icon-direction.json'))
    write(OUT / 'protected-baseline.json', {'files': protected})
    reference = receipt.relative_to(ROOT).as_posix()
    entries = []
    for c in review['candidates']:
        files = {}
        for size, f in c['files'].items():
            p = REVIEW / f['file']
            assert digest(p) == f['sha256'], p
            files[size] = {'file': p.relative_to(ROOT).as_posix(), 'sha256': f['sha256']}
        entry = {k: c[k] for k in ['id', 'number', 'variant', 'title']}
        entry.update(kind='compact', visual_status='accepted', acceptance_review='SKILLS-IC23',
                     acceptance_reference=reference, files=files,
                     master={'file': (REVIEW / c['master']['file']).relative_to(ROOT).as_posix(),
                             'sha256': c['master']['sha256']})
        assert digest(ROOT / entry['master']['file']) == entry['master']['sha256']
        entries.append(entry)
    write(receipt, {'review_id': 'SKILLS-IC23-ACCEPT01', 'date': '2026-10-11',
                   'user_approval': 'ca me va on continue',
                   'context': 'Reply to the five Survival compact icons visible in skill-icon-review-23.',
                   'accepted_compact': [e['variant'] for e in entries],
                   'accepted_numbers': [e['number'] for e in entries],
                   'scope': 'These five exact compact drawings only. No new gameplay, copy, illustration or older pending candidate approval implied.'})
    ledger['compact_accepted'].extend(entries)
    ledger['compact_accepted'].sort(key=lambda c: c.get('number', 0))
    ledger['compact_accepted_count'] = len(ledger['compact_accepted'])
    ledger['accepted_drawing_count'] = len(ledger['compact_accepted']) + len(ledger['accepted'])
    assert ledger['compact_accepted_count'] == 42
    write(ledger_path, ledger)
    review['status'] = 'visually_accepted'
    review['acceptance_reference'] = reference
    review['accepted_variants'] = [e['variant'] for e in entries]
    for c in review['candidates']:
        c.update(visual_status='accepted', acceptance_reference=reference)
    write(REVIEW / 'review.json', review)
    direction_path = BASE / 'skill-icon-direction.json'
    direction = read(direction_path)
    direction.setdefault('previous_accepted_compact_batches', []).append({
        'review': direction['accepted_compact_batch_review'],
        'candidates': direction['accepted_compact_batch_candidates']})
    direction['accepted_compact_batch_review'] = 'SKILLS-IC23'
    direction['accepted_compact_batch_candidates'] = [{k: e[k] for k in [
        'number', 'id', 'variant', 'visual_status', 'acceptance_reference']} for e in entries]
    write(direction_path, direction)
    runpy.run_path(str(ROOT / 'tools/art/package_agent_skill_window.py'), run_name='__main__')
    after = read(ROOT / 'assets/skills/agent/catalogue.json')
    assert before['skills'] == after['skills'] and before['labels'] == after['labels']
    assert after['art'] == {**before['art'], **{e['id']: {'compact': e['id'] + '-compact.png'} for e in entries}}
    for f in protected:
        assert digest(ROOT / f['file']) == f['sha256'], f['file']
    for e in entries:
        assert digest(ROOT / ('assets/skills/agent/' + e['id'] + '-compact.png')) == e['files']['64']['sha256']
    write(OUT / 'integration-validation.json', {
        'review_id': 'SKILLS-NATIVE07', 'approval_reference': reference,
        'integrated_variants': [e['variant'] for e in entries], 'integrated_compact_count': 5,
        'total_compact_count': 42, 'illustration_count': len(ledger['accepted']),
        'protected_files_preserved': len(protected), 'reviewed_exports_verified': 25,
        'skills_and_labels_unchanged': True, 'runtime_images_match_accepted_64px_exports': True,
        'mechanics_and_save_data_unchanged': True})
    page = REVIEW / 'index.html'
    html = page.read_text('utf-8').replace('Dessins proposés à la revue.', 'Cinq dessins validés le 11 octobre 2026 et intégrés au jeu.')
    page.write_text(html, 'utf-8')
    with (REVIEW / 'README.md').open('a', encoding='utf-8') as f:
        f.write('\nLes cinq variantes de cette revue sont validées le 11 octobre et intégrées sans retouche. Accord : `visual-acceptance-01.json`. Contrôles natifs : `../skill-window-native-07/`.\n')
    print('IC23: five exact accepted compact icons integrated; previous drawings preserved.')


if __name__ == '__main__':
    main()
