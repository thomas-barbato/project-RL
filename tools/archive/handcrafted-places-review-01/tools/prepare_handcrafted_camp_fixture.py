"""Condition existing prototype furniture for the engine scale review.

No new artwork: source cells are cropped and sampled into native canvases.
Accepted actors are never altered. Game/campaign resources are not replaced.
"""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
DEST = ROOT / 'assets/prototypes/handcrafted-camp-01'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    DEST.mkdir(parents=True, exist_ok=True)
    source = ROOT / 'assets/prototypes/surface64/furniture-source.png'
    sheet = Image.open(source).convert('RGBA')
    specs = [
        ('DEC-CAMP-01', 'Établi', 0, (54, 38)),
        ('DEC-CAMP-02', 'Siège', 1, (20, 28)),
        ('DEC-CAMP-03', 'Armoire', 2, (32, 56)),
        ('DEC-CAMP-04', 'Étagère', 3, (40, 52)),
        ('DEC-CAMP-05', 'Lit', 5, (34, 68)),
        ('DEC-CAMP-06', 'Banc', 6, (46, 24)),
    ]
    records = []
    for code, label, index, target in specs:
        col, row = index % 4, index // 4
        cell = sheet.crop((col * sheet.width // 4, row * sheet.height // 2,
                           (col + 1) * sheet.width // 4, (row + 1) * sheet.height // 2))
        # Source alpha fringe is packaging residue, not furniture geometry.
        cell.putalpha(cell.getchannel('A').point(lambda a: 255 if a >= 200 else 0))
        bounds = cell.getbbox()
        assert bounds
        art = cell.crop(bounds).resize(target, Image.Resampling.NEAREST)
        canvas = Image.new('RGBA', (128, 128))
        canvas.alpha_composite(art, (64 - target[0] // 2, 96 - target[1]))
        path = DEST / f'{code}-r03.png'
        canvas.save(path, optimize=True)
        records.append(dict(id=code, label=label, source=source.relative_to(ROOT).as_posix(),
                            source_cell=index, source_cell_bounds=list(bounds),
                            file=path.relative_to(ROOT).as_posix(), sha256=sha(path),
                            native_body_size=list(target), canvas=[128, 128], foot_anchor=[64, 96],
                            screen_scale=1, art_approved=False))
    data = dict(place='PLACE-CAMP-01', revision='r03', review_only=True,
                method='Existing source art, alpha fringe cleanup and nearest native conditioning; no generated room background.',
                source_sha256=sha(source), actor_body_reference_heights=dict(player=65, npc=60),
                numerical_sizes_status='Proposed visual calibration, not accepted game dimensions or new collision rules.',
                records=records)
    (REVIEW / 'native-furniture-r03.json').write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(native_furniture=len(records), actor_art_changed=False)))


if __name__ == '__main__':
    main()
