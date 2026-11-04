"""Pack the eleven anatomy corrections against the exact r01 atlases."""
import argparse
import json

from PIL import Image

from prepare_monster_vertical_all import OUT, ROOT, digest, pack_row, read_figures, write_json
from prepare_monster_vertical01 import REV, verify_frozen_assets


CORRECTIONS = OUT.with_name('r02')
REVIEW = 'MONSTER-VERTICAL-ALL-r02'
ACCEPTANCE = REV / 'MONSTER-VERTICAL-ALL-ACCEPTANCE-r01.json'


def revise(profile):
    stem = f'MON-{profile["number"]:03}'
    path = ROOT / profile['atlas']
    if digest(path) != profile['sha256']:
        raise ValueError('Atlas r01 modifié')
    with Image.open(path) as image:
        previous = image.convert('RGBA')
    with Image.open(ROOT / profile['idle_sprite']) as image:
        idle = image.convert('RGBA')
    whole_cycle = profile['review_number'] == 62
    source = CORRECTIONS / f'{stem}-sheet.png'
    figures = read_figures(source, row_count=4 if whole_cycle else 2)
    revised = previous.copy()
    if whole_cycle:
        pack_row(revised, figures[:8], idle, profile, 0)
    pack_row(revised, figures[-8:], idle, profile, 1)
    for row in range(2):
        for phase in range(9):
            box = (phase * 160, row * 128, (phase + 1) * 160, (row + 1) * 128)
            old = previous.crop(box)
            new = revised.crop(box)
            should_differ = phase < 8 and (row == 1 or whole_cycle)
            assert (old.tobytes() != new.tobytes()) == should_differ, (stem, row, phase)
            if phase == 8:
                assert new.tobytes() == idle.tobytes(), (stem, row)
    destination = CORRECTIONS / f'{stem}-atlas.png'
    revised.save(destination)
    return {
        **profile,
        'comparison': 'grouped_anatomy_revision',
        'previous_atlas': profile['atlas'],
        'previous_sha256': profile['sha256'],
        'atlas': destination.relative_to(ROOT).as_posix(),
        'sha256': digest(destination),
        'sources': [source.relative_to(ROOT).as_posix()],
        'source_sha256': digest(source),
        'revised_directions': ['down', 'up'] if whole_cycle else ['up'],
    }


def main(report_only=False):
    previous = json.loads((OUT / 'manifest.json').read_text(encoding='utf-8'))
    accepted = json.loads(ACCEPTANCE.read_text(encoding='utf-8'))
    requested = {p['id'] for p in accepted['corrections']}
    targets = [p for p in previous['profiles'] if p['id'] in requested]
    assert {p['id'] for p in targets} == requested and len(requested) == 11
    profiles, issues = [], {}
    for profile in targets:
        try:
            profiles.append(revise(profile))
        except (ValueError, OSError) as error:
            issues[profile['id']] = str(error) if not isinstance(error, FileNotFoundError) else 'Source en attente'
    print(json.dumps({'ready': len(profiles), 'issues': issues}, ensure_ascii=True))
    if report_only:
        return
    if issues:
        raise SystemExit(1)
    write_json(CORRECTIONS / 'manifest.json', {
        **previous,
        'review': REVIEW,
        'profiles': profiles,
        'frozen_source_hashes_verified': verify_frozen_assets(),
    })


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--report-only', action='store_true')
    main(parser.parse_args().report_only)
