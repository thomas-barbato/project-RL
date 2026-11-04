"""Pack the two-direction candidate sheets for the rest of the bestiary."""
import argparse
import hashlib
import json
import statistics
from PIL import Image
from prepare_class_lateral_r02 import center
from prepare_monster_vertical01 import ROOT, support_root, verify_frozen_assets

OUT = ROOT / 'assets/source-art/animation-polish-01/monster-vertical-all/r01'
REVIEW = 'MONSTER-VERTICAL-ALL-r01'
REPAIRED = {12, 96, 101, 107, 112, 157}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')


def grid_cuts(mask, horizontal, cells):
    """Find actual transparent gutters near the expected cell boundaries."""
    length = mask.width if horizontal else mask.height
    flat = mask.tobytes()
    if horizontal:
        occupancy = [sum(flat[x::mask.width]) for x in range(length)]
    else:
        occupancy = [sum(flat[y * mask.width:(y + 1) * mask.width]) for y in range(length)]
    cuts = [0]
    for index in range(1, cells):
        expected = round(index * length / cells)
        tolerance = round(length * .05)
        cut = min(range(expected - tolerance, expected + tolerance + 1),
                  key=lambda p: (occupancy[p], abs(p - expected)))
        if occupancy[cut]:
            raise ValueError(f'Gouttière {index} occupée')
        cuts.append(cut)
    return cuts + [length]


def read_figures(source, row_count=4):
    with Image.open(source) as original:
        sheet = original.convert('RGBA')
    mask = sheet.getchannel('A').point(lambda alpha: 255 if alpha >= 128 else 0)
    if any(mask.getpixel(p) for p in ((0, 0), (0, sheet.height - 1),
                                     (sheet.width - 1, 0), (sheet.width - 1, sheet.height - 1))):
        raise ValueError('Fond opaque')
    rows = grid_cuts(mask, horizontal=False, cells=row_count)
    figures = []
    for row, (top, bottom) in enumerate(zip(rows, rows[1:])):
        columns = grid_cuts(mask.crop((0, top, mask.width, bottom)), horizontal=True, cells=4)
        for col, (left, right) in enumerate(zip(columns, columns[1:])):
            bounds = (left, top, right, bottom)
            part = mask.crop(bounds)
            box = part.getbbox()
            if box is None or (box[2] - box[0]) * (box[3] - box[1]) < part.width * part.height * .02:
                raise ValueError(f'Pose {4 * row + col + 1} absente')
            touches_outer_edge = (
                (col == 0 and box[0] == 0) or (col == 3 and box[2] == part.width)
                or (row == 0 and box[1] == 0) or (row == row_count - 1 and box[3] == part.height)
            )
            if touches_outer_edge:
                raise ValueError(f'Pose {4 * row + col + 1} au bord de sa case')
            figures.append(sheet.crop(bounds).crop(box))
    for start in range(0, len(figures), 8):
        heights = [p.height for p in figures[start:start + 8]]
        if max(heights) > 1.45 * min(heights):
            raise ValueError('Écart de taille entre poses')
    return figures


def selected_sheet(job):
    suffix = '-repair-sheet.png' if job['number'] in REPAIRED else '-sheet.png'
    return OUT / f'MON-{job["number"]:03}{suffix}'


def pack_row(atlas, group, idle, ref, side):
    scale = ref['reference_height'] / statistics.median(p.height for p in group)
    poses = []
    for figure in group:
        native = figure.resize(
            (max(1, round(figure.width * scale)), max(1, round(figure.height * scale))),
            Image.Resampling.NEAREST,
        )
        x = round(80 - center(figure) * scale)
        y = ref.get('atlas_anchor_y', 112) - ref['reference_height']
        if x < 0 or x + native.width > 160 or y < 0 or y + native.height > 128:
            raise ValueError('Pose hors de la surface native')
        pose = Image.new('RGBA', (160, 128))
        pose.alpha_composite(native, (x, y))
        poses.append(pose)
    if len({pose.tobytes() for pose in poses}) != 8:
        raise ValueError('Doublons dans le cycle')
    # Use one translation for the entire row to preserve the authored gait.
    shift = round(support_root(idle) - statistics.mean(map(support_root, poses)))
    for phase, pose in enumerate(poses):
        left, _, right, _ = pose.getchannel('A').getbbox()
        if left + shift < 0 or right + shift > 160:
            raise ValueError('Appuis hors de la surface native')
        aligned = Image.new('RGBA', (160, 128))
        aligned.alpha_composite(pose, (shift, 0))
        atlas.paste(aligned, (phase * 160, side * 128))
    atlas.paste(idle, (1280, side * 128))


def make_atlas(job, ref):
    stem = f'MON-{job["number"]:03}'
    source = selected_sheet(job)
    figures = read_figures(source)
    idle_file = ROOT / ref['idle_sprite']
    if digest(idle_file) != ref['idle_sha256']:
        raise ValueError('Frontal de référence modifié')
    with Image.open(idle_file) as original:
        idle = original.convert('RGBA')
    atlas = Image.new('RGBA', (1440, 256))
    for side in (0, 1):
        pack_row(atlas, figures[8 * side:8 * (side + 1)], idle, ref, side)
    destination = OUT / f'{stem}-atlas.png'
    atlas.save(destination)
    return {
        **ref,
        'review_number': job['review_number'],
        'atlas': destination.relative_to(ROOT).as_posix(),
        'sha256': digest(destination),
        'sources': [source.relative_to(ROOT).as_posix()],
        'source_sha256': digest(source),
        'casts_shadow': ref['current_vertical']['shadow'],
        'locomotion': job['declared_rig'],
        'locomotor_legs': job['declared_locomotor_legs'],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--report-only', action='store_true')
    args = parser.parse_args()
    jobs = json.loads((OUT / 'jobs.json').read_text(encoding='utf-8'))
    refs = {r['id']: r for r in json.loads((OUT / 'references.json').read_text(encoding='utf-8'))}
    profiles, issues = [], {}
    for job in jobs:
        try:
            if args.report_only:
                read_figures(selected_sheet(job))
            else:
                profiles.append(make_atlas(job, refs[job['id']]))
        except (ValueError, OSError) as error:
            issues[job['id']] = str(error) if not isinstance(error, FileNotFoundError) else 'Source en attente'
    print(json.dumps({'total': len(jobs), 'usable': len(jobs) - len(issues), 'issues': issues}, ensure_ascii=True))
    if args.report_only:
        return
    if issues:
        raise SystemExit(1)
    if not profiles:
        raise SystemExit('Aucun candidat prêt')
    write_json(OUT / 'manifest.json', {
        'review': REVIEW, 'normal_game_enabled': False, 'native_cell_px': 64,
        'step_ms': 400, 'cycle_ms': 800, 'profiles': profiles,
        'missing': issues, 'frozen_source_hashes_verified': verify_frozen_assets(),
    })


if __name__ == '__main__':
    main()
