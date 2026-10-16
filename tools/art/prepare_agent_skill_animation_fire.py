"""ANIM-S04: native fire fixtures and exact accepted-art preservation."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw

import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_melee as gallery
import prepare_agent_skill_animation_assault as assault
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-05'
CANDIDATES = [
    (10, 'FE02', 'FE02-A01', 'Souffle de feu', 'Un torrent de flammes en cône, avec turbulence, braises et flammes sur les cibles.'),
    (11, 'FE03', 'FE03-A01', 'Mur de feu', 'Un rideau traversable de flammes, fumée et braises. Une cible quitte le mur et reste brûlée.'),
    (12, 'FE04', 'FE04-A01', 'Émanation soudaine', 'Des fissures annoncent la zone. Après un tour, une gerbe de feu jaillit du sol.'),
]


def protected() -> dict[str, str]:
    hashes = assault.protected()
    previous = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-04'
    for name in ['ME05-A01-64.gif', 'ME06-A01-64.gif', 'visual-acceptance-01.json']:
        path = previous / name
        hashes[path.relative_to(base.ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes


def prepare() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT / 'review-protection.json').exists(), 'Preserve the existing baseline.'
    base.dump(OUT / 'review-protection.json', protected())
    base.dump(OUT / 'review.json', {
        'batch': 'ANIM-S04', 'status': 'awaiting_visual_review', 'native_tile_size': 64,
        'candidates': [{'number': n, 'skill_id': sid, 'variant': var, 'name': title,
                        'description': description, 'status': 'awaiting_visual_review'}
                       for n, sid, var, title, description in CANDIDATES],
        'scope': 'Isolated native visual proposals. No GameState, RNG rolls, damage calculations or campaign integration.',
        'breath': '90-degree cone, five-cell reach, already active on launch. The 3-turn duration is not measured in animation seconds.',
        'wall': 'Five cells, passable. A scripted target exits the wall; the demonstrated successful burn remains on it.',
        'eruption': 'Radius-one cross and incandescent ground fissures, then upward jets. The capture scripts a +1-turn event at 0.85 seconds solely for review; the interactive lab waits for +1 turn.',
        'blood_policy': 'ANIM-BLOOD01: fire uses flames, smoke and embers. No blood or new Bleeding mechanic.',
        'laboratory_ui_candidate': {'variant': 'LAB-S04', 'status': 'awaiting_separate_review'},
        'unchanged': 'All accepted actors, icons, catalogue rules and previous animated GIFs.',
    })
    launch = (base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-04/Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launch.replace('--lot-assaut', '--lot-feu'), encoding='utf-8-sig')


def package() -> None:
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert protected() == baseline, 'Accepted files changed.'
    entries = json.loads((OUT / 'native-final/capture.json').read_text(encoding='utf-8'))
    assert [(e['number'], e['id'], e['variant']) for e in entries] == [(n, sid, var) for n, sid, var, _, _ in CANDIDATES]
    sequences = {}
    checks = []
    for entry in entries:
        assert entry['tile'] == 64 and entry['frames'] == 120 and entry['fps'] == 30
        files = sorted((OUT / 'native-final' / entry['folder']).glob('*.png'))
        assert len(files) == entry['frames']
        x, y, w, h = entry['crop']
        frames = []
        folder = OUT / 'frames' / entry['folder']
        folder.mkdir(parents=True, exist_ok=True)
        for file in files:
            with Image.open(file) as im:
                assert list(im.size) == entry['framebuffer_size'] == [512, 512]
                frame = im.crop((x, y, x + w, y + h)).convert('RGB')
                frame.save(folder / file.name)
                frames.append(frame)
        sequences[entry['variant']] = frames
        checks.append(save_animation(OUT / f"{entry['variant']}-64.gif", frames, 30))
        checks.append(save_animation(OUT / f"{entry['variant']}-64-slow.gif", frames, 30, True))
    boards = []
    for index in range(120):
        board = Image.new('RGB', (1480, 625), '#0b1218')
        draw = ImageDraw.Draw(board)
        draw.text((20, 12), 'ANIM-S04 · Flammes, rideau et éruption', font=base.font(24, True), fill='#e0e7e7')
        draw.text((20, 47), '10–12 · À valider · cases natives de 64 px', font=base.font(15), fill='#a0b0b8')
        for col, (number, _, variant, title, _) in enumerate(CANDIDATES):
            left = 20 + col * 490
            draw.text((left, 86), f'{number:02} · {title}', font=base.font(19, True), fill='#e0e7e7')
            draw.text((left, 113), variant, font=base.font(13), fill='#9ab0bc')
            board.paste(sequences[variant][index], (left, 146))
        boards.append(board)
    checks.append(save_animation(OUT / 'animations-lot-04.gif', boards, 30))
    sheet = Image.new('RGB', (1860, 1455), '#0b1218')
    draw = ImageDraw.Draw(sheet)
    for row, (number, _, variant, title, _) in enumerate(CANDIDATES):
        top = 12 + row * 480
        draw.text((12, top), f'{number:02} · {title}', font=base.font(18, True), fill='#e0e7e7')
        for col, index in enumerate([5, 26, 35, 65]):
            sheet.paste(sequences[variant][index], (12 + col * 460, top + 28))
    sheet.save(OUT / 'inspection-phases.png')
    base.dump(OUT / 'verification.json', {'status': 'awaiting_visual_review', 'accepted_files_unchanged': len(baseline),
        'native_capture': 'native-final/capture.json', 'source': 'Macroquad framebuffer', 'resampling': False, 'animations': checks})
    for source, target in [('src/agent_skill_lab_review/fire.rs', 'render-source.rs'),
                           ('src/agent_skill_lab_review.rs', 'render-parent-source.rs')]:
        (OUT / target).write_bytes((base.ROOT / source).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    (OUT / 'gallery-source.py').write_bytes(Path(gallery.__file__).read_bytes())
    gallery.OUT = OUT
    gallery.CANDIDATES = CANDIDATES
    gallery.write_html(entries, batch='ANIM-S04', title='Animations · Lot 04', canvas_height=448,
        introduction='Trois sorts de feu : cône turbulent, rideau traversable et gerbe jaillissant du sol. Flammes, fumée et braises, sans sang.',
        footer='Aperçus visuels isolés, avant intégration au combat. L’animation ne fait pas avancer les tours : le montage d’Émanation montre explicitement un tour de préparation puis l’explosion. Dans le laboratoire, utilisez +1 tour pour la déclencher.')
    print(json.dumps({'output': str(OUT), 'accepted_files_unchanged': len(baseline), 'animated_files': len(checks)}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--prepare', action='store_true')
    args = parser.parse_args()
    prepare() if args.prepare else package()
