"""Repack only the two corrected cultists; reuse the five other atlas references."""
import argparse
import hashlib
import json
import prepare_biped_armed03 as base

ROOT = base.ROOT
OUT = ROOT/'assets/source-art/animation-polish-01/biped-lateral-03/r02'
PREVIOUS = OUT.parent/'r01'


def references():
    OUT.mkdir(parents=True, exist_ok=True)
    refs = json.loads((PREVIOUS/'references.json').read_text(encoding='utf-8'))
    (OUT/'references.json').write_text(json.dumps([r for r in refs if r['number'] in [164, 165]], indent=2, ensure_ascii=False)+'\n', encoding='utf-8')


def main():
    references()
    base.OUT = OUT
    base.pack(review='BIPED-LATERAL03-r02')
    current = json.loads((OUT/'manifest.json').read_text(encoding='utf-8'))
    previous = json.loads((PREVIOUS/'manifest.json').read_text(encoding='utf-8'))
    profiles = {p['number']: p for p in current['profiles']}
    profiles[145] = next(p for p in previous['profiles'] if p['number'] == 145)
    for n in [164, 165]:
        old = next(p for p in previous['profiles'] if p['number'] == n)
        profiles[n]['compare_atlas'] = old['atlas']
        profiles[n]['compare_sha256'] = old['sha256']
    current['profiles'] = [profiles[n] for n in [145, 164, 165, 5, 14, 156, 144]]
    current['changed_models'] = [164, 165]
    current['unchanged_models'] = [145, 5, 14, 156, 144]
    current['head_direction_review'] = 'hood opening and face follow body in both directions; no independent sway'
    for p in current['profiles']:
        assert hashlib.sha256((ROOT/p['atlas']).read_bytes()).hexdigest() == p['sha256']
    (OUT/'manifest.json').write_text(json.dumps(current, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')
    print('Only MON-164 and MON-165 revised; five other atlases retained')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--references', action='store_true'); args = parser.parse_args()
    if args.references: references()
    else: main()
