"""Condition proposed whole figures; normal play retains accepted WALK11."""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw
from build_player_walk_matrix_06 import grid_figures
from pack_walk11_proportions import grip_ownership, box
from audit_walk09_body_volume import measure

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-12'
OUT = ROOT / 'assets/animations/walk-12/player'
SOURCES = ROOT / 'assets/source-art/player-walk-review-12'


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    old = json.loads((ROOT / 'assets/animations/walk-11/player/manifest.json').read_text())
    entries = []
    records = []
    pending = []
    jobs = json.loads((DOC / 'jobs.json').read_text())
    for job in jobs:
        index = job['index']
        path = Path(job['destination'])
        if not path.exists():
            pending.append(index)
            continue
        figures, warnings = grid_figures(Image.open(path).convert('RGBA'), 6, 2)
        assert len(figures) == 12 and not warnings, (index, warnings)
        front = Image.open(ROOT / f'assets/source-art/player-walk-review-11/CLASS-{index:02}-front-native128.png').convert('RGBA')
        fm = measure(front)
        target = fm['height']
        scale = target / max(box(f)[3]-box(f)[1] for f in figures)
        metrics = []
        for row, (facing, side) in enumerate([(-1, 'left'), (1, 'right')]):
            sheet = Image.new('RGBA', (640, 896))
            prior = next(e for e in old if e['id'] == job['id'] and e['facing'] == facing and e['frame'] == 8)
            idle = Image.open(ROOT / prior['file']).convert('RGBA').crop((0, 0, 640, 128))
            sheet.paste(idle, (0, 0))
            file = f'assets/animations/walk-12/player/CLASS-{index:02}-{side}.png'
            entries.append(dict(prior, file=file, proposal_revision='WALK12-COMMON-r01', idle_unchanged=True))
            for phase, figure in enumerate(figures[row*6:(row+1)*6]):
                small = figure.resize((round(figure.width*scale), round(figure.height*scale)), Image.Resampling.LANCZOS)
                sb = box(small)
                native = Image.new('RGBA', (128, 128))
                native.alpha_composite(small, (round(64-(sb[0]+sb[2])/2), 96-sb[3]))
                parts, meta = grip_ownership(native, index, facing, phase, legacy_contact_seams=False)
                sheet.paste(parts, (0, (phase+1)*128))
                entries.append(dict(id=job['id'], facing=facing, frame=phase, sheet_row=phase+1,
                                    file=file, **meta, proposal_revision='WALK12-COMMON-r01'))
                metrics.append(dict(facing=facing, phase=phase, **measure(native)))
            sheet.save(ROOT / file, optimize=True)
        records.append({'index': index, 'source': str(path.relative_to(ROOT)).replace('\\', '/'),
                        'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
                        'uniform_scale_for_all_twelve_poses': scale, 'front': fm, 'poses': metrics})
    (OUT / 'manifest.json').write_text(json.dumps(entries, indent=2)+'\n')
    (DOC / 'conditioning.json').write_text(json.dumps({'revision': 'WALK12-COMMON-r01',
        'status': 'proposal_only', 'normal_game_uses_walk11': True,
        'entries': records, 'pending_outfits': pending, 'whole_walking_poses': len(records)*12,
        'idle': 'unchanged WALK11 reference; player uses canonical front at arrival',
        'geometry': 'one uniform XY scale per outfit; complete painted figures, no leg graft or shear'}, indent=2)+'\n')
    names = ['Bastion', 'Technomancien', 'Traqueuse', 'Arcaniste', 'Metamorphe']
    for cls, name in enumerate(names):
        board = Image.new('RGB', (620, 780), '#303b41')
        draw = ImageDraw.Draw(board)
        draw.text((8, 5), f'{name} - PROPOSITION WALK12 - natif64', fill='white')
        for rank in range(5):
            index = cls*5+rank
            y = 30+rank*148
            draw.text((8, y), f'Tenue{rank+1} | gauche puis droite', fill='white')
            for direction, facing in enumerate([-1, 1]):
                selected = [e for e in entries if e['id'] == f'CLASS-{index:02}' and e['facing'] == facing and e['frame'] != 8]
                for e in selected:
                    sheet = Image.open(ROOT / e['file']).convert('RGBA')
                    row = e['sheet_row']*128
                    whole = sheet.crop((0, row, 128, row+128))
                    for arm in [1, 3]:
                        whole.alpha_composite(sheet.crop((arm*128, row, (arm+1)*128, row+128)))
                    tile = whole.crop((32, 32, 96, 96))
                    board.paste(tile, (12+e['frame']*98, y+15+direction*64), tile)
        board.save(DOC / f'{name}-proposition-sources-native64.png')
    print(json.dumps({'ready': len(records), 'pending': pending, 'manifest_entries': len(entries)}))


if __name__ == '__main__':
    main()
