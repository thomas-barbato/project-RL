"""ANIM-OCC01: package unchanged painted sprites clipped by native terrain."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw

import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_melee as gallery
import prepare_agent_skill_animation_painted_fire as previous
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-07'
CANDIDATES = [
    (10, 'FE02', 'FE02-A03-COULOIR', 'Souffle de feu · couloir',
     'Couloir d’une case débouchant sur une pièce. A, B et C sont exposés ; D reste protégé par l’angle du mur.'),
    (10, 'FE02', 'FE02-A03-CLOISON', 'Souffle de feu · cloison devant',
     'A est devant la cloison. B et C sont derrière : ni le souffle ni les flammes de brûlure ne les atteignent.'),
    (10, 'FE02', 'FE02-A03-OBSTACLE', 'Souffle de feu · obstacle partiel',
     'Les flammes passent de part et d’autre de l’obstacle vers A et B. C est protégé dans son ombre.'),
]


def protected() -> dict[str, str]:
    hashes = previous.protected()
    # Preserve the whole previous proposal, including its atlases and source drawings.
    for path in sorted(previous.OUT.rglob('*')):
        if path.is_file():
            hashes[path.relative_to(base.ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes


def prepare() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT / 'review-protection.json').exists(), 'Preserve the original baseline.'
    base.dump(OUT / 'review-protection.json', protected())
    base.dump(OUT / 'review.json', {
        'batch': 'ANIM-OCC01', 'status': 'awaiting_visual_review', 'native_tile_size': 64,
        'user_instruction': 'ok il faut tester pour voir ce qui se passe dans un couloir ou quand il y a un obstacle devant',
        'candidates': [{'number': n, 'skill_id': sid, 'variant': v, 'name': name,
                        'description': desc, 'status': 'awaiting_visual_review'}
                       for n, sid, v, name, desc in CANDIDATES],
        'scope': 'Isolated debug visual review. No campaign, GameState, damage, RNG, save or balance changes.',
        'source_art': 'Exact unchanged ANIM-STYLE01 flame and cone atlases. GPU clipping only; no repainting or new generated art.',
        'fixture_geometry': 'Forward 90-degree cell cone, depth five at effective rank 11. Walkable cells with the engine has_line_of_sight, closed corners blocked. Exact cone geometry is proposed, not canonized.',
        'mask': 'Same affected-cell list for targeting preview, cone clip and demonstrated burned targets. The origin cell only contains the emitter. Nearest cell mask with a two-pixel inward fade; no flame on blocked cells.',
        'presentation': 'The first 0.7 seconds show targeting; the remaining 3.3 seconds show the launched spell. This is an edited review sequence, not a new preparation delay or world-turn advancement.',
        'targets': 'Laboratory targets are all visible for comparison, including protected ones. This does not grant hidden-actor reveal in the game. Successful burns are demonstration fixtures only.',
        'blood_policy': 'Fire without blood. Characters and equipment unchanged.',
        'rule_acceptance': 'Walls already stop FE02. Exact discretization, clipping appearance and laboratory UI still await review.',
        'style_rule': 'Universal drawn animation style remains pending explicit acceptance.',
        'laboratory_ui_candidate': {'variant': 'LAB-OCC01', 'status': 'awaiting_separate_review'},
    })
    launch = (previous.OUT / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(
        launch.replace('--lot-feu-dessine', '--lot-souffle-obstacles'), encoding='utf-8-sig')


def package() -> None:
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert protected() == baseline, 'Earlier approved art or painted proposals changed.'
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    assert [(e['number'], e['id'], e['variant']) for e in entries] == [(n, sid, v) for n, sid, v, _, _ in CANDIDATES]
    sequences = {}; checks = []; clipping_checks = []
    for entry in entries:
        folder = OUT / 'native' / entry['folder']
        files = sorted(folder.glob('[0-9][0-9][0-9].png'))
        assert len(files) == entry['frames'] == 120 and entry['fps'] == 30 and entry['tile'] == 64
        frames = []; target = OUT / 'frames' / entry['folder']
        target.mkdir(parents=True, exist_ok=True)
        x, y, w, h = entry['crop']
        for file in files:
            with Image.open(file) as im:
                assert im.size == (512, 512)
                cropped = im.crop((x, y, x+w, y+h)).convert('RGB')
                cropped.save(target / file.name); frames.append(cropped)
        with Image.open(folder / 'control.png') as im:
            control = im.crop((x, y, x+w, y+h)).convert('RGB')
        diff = ImageChops.difference(frames[entry['control_frame']], control)
        assert diff.getbbox(), 'The drawn animation must actually be visible.'
        allowed = {tuple(p) for p in entry['affected_cells']} | {tuple(entry['origin'])}
        unchanged = 0
        for cy in range(7):
            for cx in range(7):
                if (cx, cy) not in allowed:
                    region = diff.crop((cx*64, cy*64, (cx+1)*64, (cy+1)*64))
                    assert region.getbbox() is None, f'{entry["variant"]}: effect leaked into blocked cell {(cx, cy)}'
                    unchanged += 1
        clipping_checks.append({'variant': entry['variant'], 'blocked_cells_without_any_changed_pixel': unchanged,
                                'protected_targets': [t['label'] for t in entry['targets'] if not t['affected']],
                                'control_frame': entry['control_frame'], 'gpu_mask_verified': True})
        sequences[entry['variant']] = frames
        checks.append(save_animation(OUT / f"{entry['variant']}-64.gif", frames, 30))
        checks.append(save_animation(OUT / f"{entry['variant']}-64-slow.gif", frames, 30, True))
    sheet = Image.new('RGB', (936, 1470), '#0b1218'); draw = ImageDraw.Draw(sheet)
    for row, (_, _, variant, title, _) in enumerate(CANDIDATES):
        top = 12+row*485
        draw.text((12, top), title, font=base.font(18, True), fill='#e0e7e7')
        for col, frame in enumerate([10, 60]):
            sheet.paste(sequences[variant][frame], (12+col*460, top+28))
    sheet.save(OUT / 'inspection-phases.png')
    base.dump(OUT / 'verification.json', {
        'status': 'awaiting_visual_review', 'protected_files_unchanged': len(baseline),
        'native_capture': 'native/capture.json', 'scene_resampling': False,
        'source': 'Native Macroquad framebuffer, actual shader mask, unchanged drawn atlases',
        'occlusion_checks': clipping_checks, 'animations': checks,
    })
    for source, target in [
        ('src/agent_skill_lab_review/fire_painted/obstacles.rs', 'render-source.rs'),
        ('src/agent_skill_lab_review/fire_painted.rs', 'render-painted-source.rs'),
        ('src/agent_skill_lab_review.rs', 'render-parent-source.rs'),
    ]:
        (OUT / target).write_bytes((base.ROOT / source).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    gallery.OUT = OUT; gallery.CANDIDATES = CANDIDATES
    gallery.write_html(entries, batch='ANIM-OCC01', title='Souffle de feu · couloir et obstacles', canvas_height=448,
        introduction='Trois situations avec les flammes dessinées précédentes. Le début de chaque boucle montre les cases touchées avant le lancement. Les lettres permettent de comparer les cibles exposées et protégées.',
        footer='Propositions à examiner : pas encore intégrées au combat. Le couloir ne modifie pas les dégâts ni la portée. La scène montre les cibles protégées pour la comparaison ; aucun pouvoir de révélation n’est ajouté.')
    print(json.dumps({'output': str(OUT), 'protected_files_unchanged': len(baseline),
                      'animated_files': len(checks), 'gpu_checks': clipping_checks}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--prepare', action='store_true')
    args = parser.parse_args()
    prepare() if args.prepare else package()
