"""Inventory this cleanup and verify preserved files; never delete anything."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import stat

ROOT = Path(__file__).resolve().parents[2]
REPORT = ROOT / 'docs/maintenance/nettoyage-2026-10-25'
KEEP_ARTIFACTS = {'map-editor', 'refonte-agent-2026-10-06',
                  'walk-preview-review-01', 'itemisation-loot-01'}
EDITED_DOCS = {'docs/GESTION_FICHIERS_GENERES.md',
               'docs/reviews/player-walk-12/helmet-attachment-01/TOUS-LES-CASQUES.md',
               'docs/reviews/player-walk-12/helmet-attachment-01/VALIDATION.md'}


def sha(path):
    result = hashlib.sha256()
    with path.open('rb') as source:
        for block in iter(lambda: source.read(1024 * 1024), b''):
            result.update(block)
    return result.hexdigest()


def regular(path):
    info = path.lstat()
    if info.st_file_attributes & stat.FILE_ATTRIBUTE_REPARSE_POINT:
        raise RuntimeError(f'Linked path must be reviewed separately: {path}')
    return info


def files(path):
    regular(path)
    if path.is_file():
        return [path]
    result = []
    for current, directories, names in os.walk(path, followlinks=False):
        for name in directories:
            regular(Path(current) / name)
        for name in names:
            entry = Path(current) / name
            regular(entry)
            result.append(entry)
    return sorted(result)


def plan():
    REPORT.mkdir(exist_ok=True)
    if (REPORT / 'preserved-before.json').exists():
        raise RuntimeError('Preserve the original baseline; do not overwrite it')
    targets = []

    def add(path, reason):
        path = ROOT / path
        if not path.exists():
            return
        path.resolve(strict=True).relative_to(ROOT.resolve())
        rows = [{'path': p.relative_to(ROOT).as_posix(), 'bytes': p.stat().st_size,
                 'sha256': sha(p)} for p in files(path)]
        targets.append({'path': path.relative_to(ROOT).as_posix(), 'reason': reason,
                        'files': rows, 'bytes': sum(r['bytes'] for r in rows)})

    for path in sorted((ROOT / 'artifacts').iterdir()):
        if path.name not in KEEP_ARTIFACTS:
            add(path, 'Superseded review, capture, raw compositor export or diagnostic output')

    street = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-repairs-review-09/proofs'
    kept_street = {'integration/map-candidate.png', 'integration/warehouse-cleared-candidate.png',
                   'integration/gate-candidate.png', 'integration/junction-candidate.png'}
    for path in street.rglob('*.png'):
        if path.relative_to(street).as_posix() not in kept_street:
            add(path, 'Redundant native scene capture; four final representative captures retained')

    walk11 = ROOT / 'docs/reviews/player-walk-11'
    for path in (walk11 / 'native-grips-r03').glob('*.png'):
        if path.name not in {'player-contact-sheet.png', 'catalogue-040.png'}:
            add(path, 'Redundant hand-position catalogue capture; overview and representative case retained')
    add(walk11 / 'audit/audit.json', 'Reproducible detailed geometry measurements; concise validation retained')
    for path in walk11.glob('*-native64-axe.gif'):
        if not path.name.startswith('Bastion-'):
            add(path, 'Older repeated proportion preview; accepted sources and representative Bastion retained')
    for path in (ROOT / 'docs/reviews/player-walk-10').glob('*.gif'):
        add(path, 'Superseded proportion prototype preview')

    walk12 = ROOT / 'docs/reviews/player-walk-12'
    for path in walk12.glob('*.gif'):
        if path.name not in {'partie-reelle-marche-et-arret.gif', 'Bastion-demarche-commune-proposee.gif'}:
            add(path, 'Redundant intermediate walk comparison; current gait and actual-game example retained')
    for pattern in ['*-controle-phase*.png', '*-proposition-phase*.png']:
        for path in walk12.glob(pattern):
            add(path, 'Individual frame export; whole source sheets and representative evidence retained')
    helmets = walk12 / 'helmet-attachment-01'
    for path in helmets.iterdir():
        if path.suffix in {'.gif', '.png'} and path.name not in {
            'tous-les-casques-cinq-classes-native64.gif', 'tous-les-casques-controle-native64.png'}:
            add(path, 'Redundant helmet preview; single current six-model overview retained')
    for path in (ROOT / 'tools').rglob('__pycache__'):
        add(path, 'Regenerable Python bytecode cache')
    add('target/flycheck0', 'Disposable editor compiler diagnostics')

    deleted = {row['path'] for target in targets for row in target['files']}
    assert len(deleted) == sum(len(t['files']) for t in targets), 'Overlapping cleanup targets'
    baseline = []
    roots = ['assets', 'src', 'content', 'mods', 'vendor', 'tests', 'examples', 'tools',
             'docs/reviews', 'docs/maintenance', 'dist', 'target/release']
    roots += ['artifacts/' + name for name in KEEP_ARTIFACTS]
    preserved = []
    for name in roots:
        path = ROOT / name
        if path.exists():
            preserved.extend(files(path))
    preserved.extend(p for p in ROOT.iterdir() if p.is_file())
    preserved.extend(p for p in (ROOT / '.git').glob('*') if p.name in {'HEAD', 'index', 'packed-refs'})
    if (ROOT / '.git/refs').exists():
        preserved.extend(files(ROOT / '.git/refs'))
    user_folder = Path(os.environ['APPDATA']) / 'ProjectRL'
    if user_folder.exists():
        preserved.extend(files(user_folder))
    for path in sorted(set(preserved)):
        if path.is_relative_to(ROOT):
            name = path.relative_to(ROOT).as_posix()
            if name in deleted or name in EDITED_DOCS or path.is_relative_to(REPORT):
                continue
        else:
            name = str(path)
        baseline.append({'path': name, 'bytes': path.stat().st_size, 'sha256': sha(path)})
    manifest = {'status': 'ready_after_validation', 'root': str(ROOT),
                'mode': 'Recycle Bin, without emptying it; Cargo handles Debug cache separately',
                'protected_artifact_roots': sorted(KEEP_ARTIFACTS), 'targets': targets,
                'file_count': len(deleted), 'bytes': sum(t['bytes'] for t in targets)}
    (REPORT / 'plan.json').write_text(json.dumps(manifest, indent=2) + '\n')
    (REPORT / 'preserved-before.json').write_text(json.dumps(baseline, indent=2) + '\n')
    print(json.dumps({'targets': len(targets), 'files': len(deleted),
                      'MiB': round(manifest['bytes'] / 1048576, 2),
                      'preserved_files_hashed': len(baseline)}))


def verify():
    before = json.loads((REPORT / 'preserved-before.json').read_text())
    changed = []
    for row in before:
        path = Path(row['path'])
        if not path.is_absolute():
            path = ROOT / path
        if not path.is_file() or path.stat().st_size != row['bytes'] or sha(path) != row['sha256']:
            changed.append(row['path'])
    result = {'status': 'passed' if not changed else 'failed',
              'files_verified': len(before), 'changed': changed}
    (REPORT / 'preservation-after.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result))
    assert not changed, changed


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('action', choices=['plan', 'verify'])
    {'plan': plan, 'verify': verify}[parser.parse_args().action]()
