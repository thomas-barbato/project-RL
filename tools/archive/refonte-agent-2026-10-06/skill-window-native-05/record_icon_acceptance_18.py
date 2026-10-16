"""Record the explicit acceptance of Harpoon only; preserve all other artwork."""
from pathlib import Path
import copy
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-icon-review-18'
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
        raise SystemExit('IC18 partial acceptance already recorded.')
    review = read(REVIEW / 'review.json')
    candidate = next(c for c in review['candidates'] if c['id'] == 'DI05')
    assert candidate['number'] == 39 and candidate['variant'] == 'DI05-C01'
    ledger_path = BASE / 'skill-icon-acceptance.json'
    ledger = read(ledger_path)
    assert not any(c['id'] == 'DI05' for c in ledger['compact_accepted'])
    old_ledger = copy.deepcopy(ledger)
    protected = [f for e in ledger['accepted'] + ledger['compact_accepted'] for f in e['files'].values()]
    for e in read(ROOT / 'assets/skills/agent/manifest.json')['drawings']:
        protected.append({'file': 'assets/skills/agent/' + e['file'], 'sha256': e['sha256']})
    for f in protected:
        assert digest(ROOT / f['file']) == f['sha256'], f['file']
    before = read(ROOT / 'assets/skills/agent/catalogue.json')
    write(OUT / 'acceptance-before.json', old_ledger)
    write(OUT / 'catalogue-before.json', before)
    write(OUT / 'direction-before.json', read(BASE / 'skill-icon-direction.json'))
    write(OUT / 'protected-baseline.json', {'files': protected})
    files = {}
    for size, f in candidate['files'].items():
        p = REVIEW / f['file']
        assert digest(p) == f['sha256'], p
        files[size] = {'file': p.relative_to(ROOT).as_posix(), 'sha256': f['sha256']}
    reference = receipt_path.relative_to(ROOT).as_posix()
    entry = {k: candidate[k] for k in ['id', 'number', 'variant', 'title']}
    entry.update(kind='compact', visual_status='accepted', acceptance_review='SKILLS-IC18',
                 acceptance_reference=reference, files=files,
                 master={'file': (REVIEW / candidate['master']['file']).relative_to(ROOT).as_posix(),
                         'sha256': candidate['master']['sha256']})
    assert digest(ROOT / entry['master']['file']) == entry['master']['sha256']
    rejected = [c['variant'] for c in review['candidates'] if c['id'] != 'DI05']
    write(receipt_path, {
        'review_id': 'SKILLS-IC18-ACCEPT01', 'date': '2026-10-11',
        'user_approval': "non je n'en valide aucune, sauf tir de harpon, et je peux te dire que pour tir de précision, peut-être qu'on devrait voir un crane à la place de la plaque d'armure :)",
        'accepted_compact': ['DI05-C01'], 'accepted_numbers': [39],
        'rejected_compact': rejected,
        'scope': 'Exactly Harpoon compact DI05-C01. The other five drawings are rejected and require new review. No gameplay, copy or detailed illustration approval implied.'})
    ledger['compact_accepted'].append(entry)
    ledger['compact_accepted'].sort(key=lambda c: c.get('number', 0))
    ledger['compact_accepted_count'] = len(ledger['compact_accepted'])
    ledger['accepted_drawing_count'] = len(ledger['compact_accepted']) + len(ledger['accepted'])
    write(ledger_path, ledger)
    review['status'] = 'partially_accepted_requires_revision'
    review['acceptance_reference'] = reference
    review['accepted_variants'] = ['DI05-C01']
    review['rejected_variants'] = rejected
    for c in review['candidates']:
        c['visual_status'] = 'accepted' if c['id'] == 'DI05' else 'rejected'
        c['feedback_reference'] = reference
        if c['id'] == 'DI05':
            c['acceptance_reference'] = reference
    write(REVIEW / 'review.json', review)
    direction_path = BASE / 'skill-icon-direction.json'
    direction = read(direction_path)
    direction.setdefault('previous_accepted_compact_batches', []).append({
        'review': direction['accepted_compact_batch_review'],
        'candidates': direction['accepted_compact_batch_candidates']})
    direction['accepted_compact_batch_review'] = 'SKILLS-IC18'
    direction['accepted_compact_batch_candidates'] = [{k: entry[k] for k in [
        'number', 'id', 'variant', 'visual_status', 'acceptance_reference']}]
    direction.setdefault('rejected_compact_batches', []).append({
        'review': 'SKILLS-IC18', 'variants': rejected, 'feedback_reference': reference})
    write(direction_path, direction)
    runpy.run_path(str(ROOT / 'tools/art/package_agent_skill_window.py'), run_name='__main__')
    after = read(ROOT / 'assets/skills/agent/catalogue.json')
    assert before['skills'] == after['skills'] and before['labels'] == after['labels']
    assert after['art'] == {**before['art'], 'DI05': {'compact': 'DI05-compact.png'}}
    for f in protected:
        assert digest(ROOT / f['file']) == f['sha256'], f['file']
    assert digest(ROOT / 'assets/skills/agent/DI05-compact.png') == files['64']['sha256']
    assert not any(c in after['art'] for c in ['DI01', 'DI02', 'DI03', 'DI04', 'DI07'])
    write(OUT / 'integration-validation.json', {
        'review_id': 'SKILLS-NATIVE05', 'approval_reference': reference,
        'integrated_variant': 'DI05-C01', 'integrated_compact_count': 1,
        'total_compact_count': ledger['compact_accepted_count'], 'illustration_count': len(ledger['accepted']),
        'protected_files_preserved': len(protected), 'reviewed_exports_verified': 5,
        'skills_and_labels_unchanged': True, 'runtime_image_matches_accepted_64px_export': True,
        'rejected_drawings_not_integrated': True, 'mechanics_and_save_data_unchanged': True})
    print('IC18: Harpoon only integrated; five rejected drawings remain unavailable.')


if __name__ == '__main__':
    main()
