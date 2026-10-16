"""Review 38: fluid painted blood, isolated native animation review."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_ritual_visibility as visibility
import prepare_agent_skill_animation_occult_rituals as rituals

ROOT = rituals.ROOT
BASE = visibility.OUT
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-38'
old_write_html = rituals.write_html


def protected():
    hashes = visibility.protected()
    for p in BASE.rglob('*'):
        # Unpublished raw grading trial is retained, outside the presented gallery baseline.
        if p.is_file() and 'native-first-trial' not in p.relative_to(BASE).parts:
            hashes[p.relative_to(ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def protect():
    OUT.mkdir(parents=True, exist_ok=True)
    path = OUT / 'review-protection.json'
    assert not path.exists()
    rituals.base.dump(path, protected())
    print(json.dumps({'protected_baseline': str(path)}))


def prepare(prison, reanimation):
    OUT.mkdir(parents=True, exist_ok=True)
    if not (OUT / 'review-protection.json').exists():
        protect()
    packs = []
    for kind, source in [('prison', prison), ('reanimation', reanimation)]:
        (OUT / f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as image:
            assert image.mode == 'RGBA' and image.width * 2 == image.height * 3
            assert image.getchannel('A').getextrema()[0] == 0
            cell = image.width // 3
            atlas = Image.new('RGBA', (1536, 256))
            for i in range(6):
                x = i % 3 * cell; y = i // 3 * cell
                atlas.paste(image.crop((x, y, x + cell, y + cell)).resize((256, 256), Image.Resampling.LANCZOS), (i * 256, 0))
            if kind == 'reanimation':
                # Mechanical layer assembly: preserve the exact old marker, no repainting.
                with Image.open(BASE / 'reanimation-atlas.png') as old:
                    atlas.paste(old.crop((1280, 0, 1536, 256)), (1280, 0))
            atlas.save(OUT / f'{kind}-atlas.png')
        packs.append({'kind': kind, 'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(), 'source_size': list(image.size), 'layout': [3, 2], 'cell_size': [256, 256], 'alpha_preserved': True, 'ivory_skull_cell_preserved_from_previous_atlas': kind == 'reanimation'})
    rituals.base.dump(OUT / 'drawing-pack.json', {'mode': 'Built-in imagegen', 'packing': packs, 'prompts': ['imagegen-prison-prompt.txt', 'imagegen-prison-geometry-prompt.txt', 'imagegen-reanimation-prompt.txt'], 'prison_edit_chain': 'review37 reference -> prison-source-curvature-trial.png -> prison-source.png; second edit corrects curvature only', 'postprocessing': 'Equal-cell cropping and uniform resizing; exact previous skull layer assembled into the atlas. No Python repainting.'})
    review = json.loads((BASE / 'review.json').read_text(encoding='utf-8'))
    for c in review['candidates']:
        c['variant'] = c['id'] + '-A03'; c['status'] = 'awaiting_visual_review'
    review['feedback'] = "est-ce que tu peux refaire un peu l'effet du sang ? pour qu'il soit un peu plus... je sais pas ... jolis ?"
    review['revision'] = 'New painted blood drawings: fluid crimson arcs with corrected curvature, simpler floor seal and fluid rising resurrection ribbons; limited droplets and liquid highlights. Narrow radial layering reinforces circle thickness; seal and surge use single drawings to retain volumes. Opt-in 1.5 exposure retains painted colours and shadows.'
    review['retained'] = 'Eight independent segments, free centre, exact old ivory skull marker, actors, terrain, placement and original prepared timelines. New palette belongs only to these prototype drawings; previous occult effects and rules unchanged.'
    review['previous_approval'] = 'Previous ritual variants remain awaiting review; no approval inferred.'
    rituals.base.dump(OUT / 'review.json', review)
    launcher = (BASE / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--rituels-lisibles'", "'--rituels-sang-fluide'")
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher, encoding='utf-8-sig')
    for name in ('verify-gallery.cjs', 'verify-links.py'):
        text = (BASE / name).read_text(encoding='utf-8').replace('skill-animation-review-36', 'skill-animation-review-37').replace('-A02-', '-A03-').replace('-A01-', '-A02-')
        (OUT / name).write_text(text, encoding='utf-8')
    print(json.dumps({'prepared': str(OUT), 'candidates': ['OC04-A03', 'OC07-A03']}))


def write_html(entries):
    old_write_html(entries)
    path = OUT / 'index.html'; html = path.read_text(encoding='utf-8')
    html = html.replace('OC04-A01 ·', 'OC04-A03 ·').replace('OC07-A01 ·', 'OC07-A03 ·')
    html = html.replace('ANIM-S12 · deux nouveaux rendus à valider', 'ANIM-S12 · sang redessiné · à valider')
    html = html.replace('<select id="scenario">', '<select id="comparison"><option value="new">Sang redessiné</option><option value="old">Version précédente</option></select><select id="scenario">')
    html = html.replace("im.src=`frames/${v.variant}/${String(i).padStart(3,'0')}.png`;", "const old=document.getElementById('comparison').value==='old';const variant=old?v.variant.replace('-A03-','-A02-'):v.variant;im.src=`${old?'../skill-animation-review-37/':''}frames/${variant}/${String(i).padStart(3,'0')}.png`;")
    html = html.replace('scenario.onchange=()=>{', "document.getElementById('comparison').onchange=()=>{launched=true;turn=0;elapsed=0;load();};scenario.onchange=()=>{")
    html = html.replace('<p>Le cercle laisse', '<p><a href="sang-comparaison-64.gif">Comparer le sang avant et après à taille native</a></p><p>Le cercle laisse')
    html = html.replace('<a href="imagegen-prison-prompt.txt">prompt</a>', '<a href="imagegen-prison-prompt.txt">premier prompt</a> / <a href="imagegen-prison-geometry-prompt.txt">correction de courbure</a>')
    path.write_text(html, encoding='utf-8')
    (OUT / 'gallery-script.js').write_text(html.split('<script>', 1)[1].split('</script>', 1)[0], encoding='utf-8')


def package():
    rituals.OUT = OUT; rituals.protected = protected; rituals.write_html = write_html
    rituals.package()
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    for entry in entries:
        assert (OUT / 'native' / entry['folder'] / 'timeline.json').read_bytes() == (BASE / 'native' / entry['folder'] / 'timeline.json').read_bytes()
        for view in entry['views']:
            x, y, w, h = view['crop']; bounds = (x, y, x + w, y + h)
            for frame in entry['control_frames']:
                with Image.open(OUT / 'native' / entry['folder'] / f'control-{frame:03}.png') as new, Image.open(BASE / 'native' / entry['folder'] / f'control-{frame:03}.png') as old:
                    diff = ImageChops.difference(new.crop(bounds).convert('RGB'), old.crop(bounds).convert('RGB')); visibility.clear_name(diff, view, frame)
                    assert diff.getbbox() is None, ('Actors/terrain changed', view['variant'], frame)
            if view['id'] == 'OC07' and view['valid'] and not entry['quiet']:
                with Image.open(OUT / 'frames' / view['variant'] / '100.png') as new, Image.open(BASE / 'frames' / view['variant'].replace('-A03-', '-A02-') / '100.png') as old:
                    assert ImageChops.difference(new.convert('RGB'), old.convert('RGB')).getbbox() is None
    rituals.base.dump(OUT / 'redraw-verification.json', {'timelines_identical_to_review37': True, 'actors_and_terrain_identical': True, 'ivory_ally_marker_identical': True, 'native_drawings_changed_only': True, 'comparison_limit': 'Only the known floating name antialias rectangle is excluded at frames 17-33.'})
    boards = []
    for i in range(144):
        board = Image.new('RGB', (1232, 1238), '#0b1218'); draw = ImageDraw.Draw(board)
        for row, (sid, title) in enumerate([('OC04', '25 · Prison sigillaire'), ('OC07', '26 · Réanimation')]):
            for col, (root, version, label) in enumerate([(BASE, 'A02', 'Version précédente'), (OUT, 'A03', 'Sang redessiné')]):
                x = 12 + col * 616; y = 12 + row * 616
                draw.text((x, y), f'{title} · {label}', font=rituals.base.font(19, True), fill='#e0e7e7')
                with Image.open(root / 'frames' / f'{sid}-{version}-HAUT' / f'{i:03}.png') as image: board.paste(image, (x, y + 30))
        boards.append(board)
    rituals.previous.drains.save_animation(OUT / 'sang-comparaison-64.gif', boards, 24)
    (OUT / 'package-base-source.py').write_bytes(Path(rituals.__file__).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'comparison': 'native 64px', 'timelines_unchanged': True, 'status': 'both redrawn variants awaiting visual review'}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--protect-only', action='store_true'); parser.add_argument('--prepare', action='store_true'); parser.add_argument('--prison', type=Path); parser.add_argument('--reanimation', type=Path); args = parser.parse_args()
    if args.protect_only: protect()
    elif args.prepare:
        assert args.prison and args.reanimation
        prepare(args.prison, args.reanimation)
    else: package()
