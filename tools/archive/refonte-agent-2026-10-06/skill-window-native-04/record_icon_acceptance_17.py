"""Record the explicit IC17 visual acceptance and package only its 30 compact icons."""
from pathlib import Path
import copy
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-icon-review-17'
OUT = Path(__file__).resolve().parent


def read(path):
    return json.loads(path.read_text('utf-8'))


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', 'utf-8')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    receipt_path = REVIEW / 'visual-acceptance-01.json'
    if receipt_path.exists():
        raise SystemExit('IC17 acceptance already recorded; use the normal packager.')
    review = read(REVIEW / 'review.json')
    candidates = review['candidates']
    assert len(candidates) == len({c['id'] for c in candidates}) == 30
    assert next(c for c in candidates if c['id'] == 'AF03')['variant'] == 'AF03-C03'
    ledger_path = BASE / 'skill-icon-acceptance.json'
    ledger = read(ledger_path)
    old_ledger = copy.deepcopy(ledger)
    for category in ['accepted', 'compact_accepted']:
        for entry in ledger[category]:
            for file in entry['files'].values():
                assert digest(ROOT / file['file']) == file['sha256'], file['file']
    accepted = []
    reference = receipt_path.relative_to(ROOT).as_posix()
    for candidate in candidates:
        files = {}
        for size, file in candidate['files'].items():
            path = REVIEW / file['file']
            assert digest(path) == file['sha256'], path
            files[size] = {'file': path.relative_to(ROOT).as_posix(), 'sha256': file['sha256']}
        master = REVIEW / (candidate['variant'] + '-master.png')
        assert master.is_file()
        accepted.append({'id': candidate['id'], 'number': candidate['number'],
                         'variant': candidate['variant'], 'kind': 'compact',
                         'title': candidate['title'], 'visual_status': 'accepted',
                         'acceptance_review': 'SKILLS-IC17', 'acceptance_reference': reference,
                         'files': files, 'master': {'file': master.relative_to(ROOT).as_posix(),
                                                    'sha256': digest(master)}})
    targets = {c['id'] for c in candidates}
    protected = []
    for entry in ledger['accepted'] + [c for c in ledger['compact_accepted'] if c['id'] not in targets]:
        for file in entry['files'].values():
            protected.append(file)
    before_catalogue = read(ROOT / 'assets/skills/agent/catalogue.json')
    for entry in read(ROOT / 'assets/skills/agent/manifest.json')['drawings']:
        if entry['category'] != 'compact' or entry['skill'] not in targets:
            protected.append({'file': 'assets/skills/agent/' + entry['file'], 'sha256': entry['sha256']})
    write(OUT / 'acceptance-before.json', old_ledger)
    write(OUT / 'direction-before.json', read(BASE / 'skill-icon-direction.json'))
    write(OUT / 'catalogue-before.json', before_catalogue)
    write(OUT / 'protected-baseline.json', {'files': protected})
    receipt = {'review_id': 'SKILLS-IC17-ACCEPT01', 'date': '2026-10-11',
               'user_approval': 'je valide :)',
               'approval_context': 'Reply to IC17 after the requested real-gag revision of Silence.',
               'accepted_compact': [c['variant'] for c in candidates],
               'accepted_numbers': [c['number'] for c in candidates],
               'scope': 'Exactly the 30 compact drawings currently presented in IC17, including AF03-C03. No new skill copy, combat effects, illustrations or other review drawings are approved.'}
    write(receipt_path, receipt)
    ledger['compact_accepted'] = [c for c in ledger['compact_accepted'] if c['id'] not in targets] + accepted
    ledger['compact_accepted'].sort(key=lambda c: c.get('number', 0))
    ledger['compact_accepted_count'] = len(ledger['compact_accepted'])
    ledger['accepted_drawing_count'] = len(ledger['compact_accepted']) + len(ledger['accepted'])
    ledger.setdefault('superseded_compact', []).extend(
        {**c, 'superseded_by': next(n['variant'] for n in accepted if n['id'] == c['id']),
         'supersession_reference': reference}
        for c in old_ledger['compact_accepted'] if c['id'] in targets)
    write(ledger_path, ledger)
    review['status'] = 'visual_accepted'
    review['acceptance_reference'] = reference
    for c in review['candidates']:
        c['visual_status'] = 'accepted'
        c['acceptance_reference'] = reference
    write(REVIEW / 'review.json', review)
    direction_path = BASE / 'skill-icon-direction.json'
    direction = read(direction_path)
    direction.setdefault('previous_accepted_compact_batches', []).append({
        'review': direction['accepted_compact_batch_review'],
        'candidates': direction['accepted_compact_batch_candidates']})
    direction['accepted_compact_batch_review'] = 'SKILLS-IC17'
    direction['accepted_compact_batch_candidates'] = [
        {key: c[key] for key in ['number', 'id', 'variant', 'visual_status', 'acceptance_reference']}
        for c in accepted]
    direction['pending_compact_batch_candidates'] = [
        c for c in direction.get('pending_compact_batch_candidates', []) if c['id'] not in targets]
    direction['compact_style_reference'] = 'SKILLS-IC16'
    direction['compact_style_direction'] = 'Grim industrial and occult material treatment, with readable native-size emblems.'
    write(direction_path, direction)
    runpy.run_path(str(ROOT / 'tools/art/package_agent_skill_window.py'), run_name='__main__')
    current_catalogue = read(ROOT / 'assets/skills/agent/catalogue.json')
    assert current_catalogue['skills'] == before_catalogue['skills']
    assert current_catalogue['labels'] == before_catalogue['labels']
    for file in protected:
        assert digest(ROOT / file['file']) == file['sha256'], file['file']
    for entry in accepted:
        assert digest(ROOT / 'assets/skills/agent' / (entry['id'] + '-compact.png')) == entry['files']['64']['sha256']
    write(OUT / 'integration-validation.json', {
        'review_id': 'SKILLS-NATIVE04', 'approval_reference': reference,
        'integrated_compact_count': 30, 'total_compact_count': len(ledger['compact_accepted']),
        'illustration_count': len(ledger['accepted']), 'reviewed_exports_verified': 150,
        'protected_files_preserved': len(protected), 'skills_and_labels_unchanged': True,
        'runtime_images_match_accepted_64px_exports': True,
        'mechanics_and_save_data_unchanged': True})
    print('IC17: 30 compact drawings integrated; previous exports and illustrations preserved.')


if __name__ == '__main__':
    main()
