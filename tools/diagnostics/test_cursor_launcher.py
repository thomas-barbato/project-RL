"""Exercise the real Jouer.cmd from another directory without opening the game."""
from pathlib import Path
import hashlib
import json
import os
import subprocess

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/cursor-visibility-01'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def user_data():
    folder = Path(os.environ['APPDATA']) / 'ProjectRL'
    return {p.name: sha(p) for p in folder.glob('*.json') if p.is_file()}


def invoke(label, data):
    environment = dict(os.environ, PROJECT_RL_DATA_DIR=str(data))
    result = subprocess.run([os.environ.get('COMSPEC', 'cmd.exe'), '/d', '/c',
                             str(ROOT / 'Jouer.cmd'), '--verify-assets'],
                            cwd=OUT, env=environment, capture_output=True, text=True,
                            encoding='utf-8', errors='replace', timeout=120)
    (OUT / f'launcher-{label}.log').write_text(result.stdout+result.stderr, encoding='utf-8')
    return result


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    before = user_data()
    package = ROOT / 'dist/Project-RL'
    manifest = package / 'runtime-assets.json'
    assert manifest.is_file(), 'Existing portable data required for the read-only positive test'
    package_before = sha(manifest)
    success = invoke('valid', package)
    assert success.returncode == 0, success.stdout+success.stderr
    assert 'Verified ' in success.stdout
    failure = invoke('missing', OUT / 'intentionally-absent-data')
    assert failure.returncode != 0 and '[ASSETS]' in failure.stderr, failure.stdout+failure.stderr
    assert sha(manifest) == package_before
    assert user_data() == before
    report = {'status': 'passed', 'actual_launcher': 'Jouer.cmd',
              'working_directory': str(OUT), 'arguments_forwarded': ['--verify-assets'],
              'positive_exit_code': success.returncode, 'negative_exit_code': failure.returncode,
              'foreground_synchronous_launch': True, 'game_window_opened': False,
              'existing_package_data_modified': False, 'user_json_files_unchanged': len(before),
              'release_sha256': sha(ROOT / 'target/release/project-rl.exe'),
              'launcher_sha256': sha(ROOT / 'Jouer.cmd')}
    (OUT / 'launcher-tests.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
