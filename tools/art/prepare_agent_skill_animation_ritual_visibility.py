"""Review 37: brighter, thicker ritual drawings. No rule or timeline changes."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_occult_rituals as previous

ROOT = previous.ROOT
BASE = previous.OUT
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-37'
old_protected = previous.protected
old_write_html = previous.write_html


def protected():
    hashes = old_protected()
    for p in BASE.rglob('*'):
        if p.is_file():
            hashes[p.relative_to(ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT / 'review-protection.json').exists()
    previous.base.dump(OUT / 'review-protection.json', protected())
    for name in ('prison-source.png', 'prison-atlas.png', 'reanimation-source.png', 'reanimation-atlas.png', 'drawing-pack.json', 'imagegen-prison-prompt.txt', 'imagegen-reanimation-prompt.txt'):
        (OUT / name).write_bytes((BASE / name).read_bytes())
    review = json.loads((BASE / 'review.json').read_text(encoding='utf-8'))
    review['status'] = 'awaiting_visual_review'
    for c in review['candidates']:
        c['variant'] = c['id'] + '-A02'
        c['status'] = 'awaiting_visual_review'
    review['feedback'] = 'les effets ne sont pas assez visibles :) (couleur trop sombres et taille trop fines)'
    review['revision'] = 'Opt-in scarlet/crimson shader profile; thicker independent curved strokes across their width, larger and thicker seal and resurrection surge. No added bloom or white luminous circle.'
    review['retained'] = 'Exact source PNGs/alpha; circle radius and eight independent segments; central free cell; ivory skull; targeting, durations, cooldowns and scripted event timelines. Existing actors and terrain. No campaign integration.'
    review['previous_approval'] = 'Review 36 pending. Neither revised variant inherits approval.'
    previous.base.dump(OUT / 'review.json', review)
    launcher = (BASE / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--lot-rituels-occultes'", "'--lot-rituels-occultes' '--rituels-lisibles'")
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher, encoding='utf-8-sig')
    for name in ('verify-gallery.cjs', 'verify-links.py'):
        (OUT / name).write_bytes((BASE / name).read_bytes())
    print(json.dumps({'prepared': str(OUT), 'new_drawings': 0, 'candidates': ['OC04-A02', 'OC07-A02']}))


def write_html(entries):
    old_write_html(entries)
    path = OUT / 'index.html'
    html = path.read_text(encoding='utf-8')
    html = html.replace('OC04-A01 ·', 'OC04-A02 ·').replace('OC07-A01 ·', 'OC07-A02 ·')
    html = html.replace('ANIM-S12 · deux nouveaux rendus à valider', 'ANIM-S12 · rituels plus clairs et plus épais · à valider')
    html = html.replace('<select id="scenario">', '<select id="comparison"><option value="new">Tracés renforcés</option><option value="old">Version précédente</option></select><select id="scenario">')
    html = html.replace("im.src=`frames/${v.variant}/${String(i).padStart(3,'0')}.png`;", "const old=document.getElementById('comparison').value==='old';const variant=old?v.variant.replace('-A02-','-A01-'):v.variant;im.src=`${old?'../skill-animation-review-36/':''}frames/${variant}/${String(i).padStart(3,'0')}.png`;")
    html = html.replace('scenario.onchange=()=>{', "document.getElementById('comparison').onchange=()=>{launched=true;turn=0;elapsed=0;load();};scenario.onchange=()=>{")
    html = html.replace('<p>Le cercle laisse', '<p><a href="visibilite-comparaison-64.gif">Comparer les deux versions côte à côte à taille native</a></p><p>Le cercle laisse')
    path.write_text(html, encoding='utf-8')
    (OUT / 'gallery-script.js').write_text(html.split('<script>', 1)[1].split('</script>', 1)[0], encoding='utf-8')


def clear_name(diff, view, frame):
    # Narrow transient skill-name antialias rectangle only; no actor pixel exclusion.
    if view['valid'] and 17 <= frame <= 33:
        px, py = view['player_cell']
        px = (px + .5) * 64
        baseline = (py + .5) * 64 - 43 - (frame - 17) / 24 * 7
        diff.paste((0, 0, 0), (int(px - 59), int(baseline - 19), int(px + 200), int(baseline + 4)))


def effect_metrics(root, entry, view, frame):
    x, y, w, h = view['crop']
    with Image.open(root / 'frames' / view['variant'] / f'{frame:03}.png') as image, Image.open(root / 'native' / entry['folder'] / f'control-{frame:03}.png') as quiet:
        pixels = list(image.convert('RGB').get_flattened_data())
        controls = list(quiet.crop((x, y, x + w, y + h)).convert('RGB').get_flattened_data())
    visible = [(p[0] - q[0], p[0]) for p, q in zip(pixels, controls) if p[0] > q[0] + 35 and p[0] > max(p[1], p[2]) + 35]
    return {'scarlet_pixels': len(visible), 'red_contrast_sum': sum(v[0] for v in visible), 'mean_red': round(sum(v[1] for v in visible) / max(1, len(visible)), 1)}


def package():
    previous.OUT = OUT
    previous.protected = protected
    previous.write_html = write_html
    previous.package()
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    checks = []
    for entry in entries:
        assert (OUT / 'native' / entry['folder'] / 'timeline.json').read_bytes() == (BASE / 'native' / entry['folder'] / 'timeline.json').read_bytes()
        for view in entry['views']:
            x, y, w, h = view['crop']; bounds = (x, y, x + w, y + h)
            for frame in entry['control_frames']:
                with Image.open(OUT / 'native' / entry['folder'] / f'control-{frame:03}.png') as new, Image.open(BASE / 'native' / entry['folder'] / f'control-{frame:03}.png') as old:
                    diff = ImageChops.difference(new.crop(bounds).convert('RGB'), old.crop(bounds).convert('RGB'))
                    clear_name(diff, view, frame)
                    assert diff.getbbox() is None, ('Actors or terrain changed', view['variant'], frame)
            if view['valid']:
                old_view = dict(view, variant=view['variant'].replace('-A02-', '-A01-'))
                frames = [35] + ([65] if view['id'] == 'OC07' and not entry['quiet'] else [])
                for frame in frames:
                    old = effect_metrics(BASE, entry, old_view, frame)
                    new = effect_metrics(OUT, entry, view, frame)
                    assert new['scarlet_pixels'] > old['scarlet_pixels'] * 1.5, (view['variant'], frame, old, new)
                    assert new['red_contrast_sum'] > old['red_contrast_sum'] * 1.5
                    checks.append({'variant': view['variant'], 'frame': frame, 'old': old, 'new': new})
                if view['id'] == 'OC07' and not entry['quiet']:
                    # By frame 100 only the ivory ally marker remains; exact comparison.
                    with Image.open(OUT / 'frames' / view['variant'] / '100.png') as new, Image.open(BASE / 'frames' / old_view['variant'] / '100.png') as old:
                        assert ImageChops.difference(new.convert('RGB'), old.convert('RGB')).getbbox() is None
    for name in ('prison-source.png', 'prison-atlas.png', 'reanimation-source.png', 'reanimation-atlas.png'):
        assert (OUT / name).read_bytes() == (BASE / name).read_bytes()
    previous.base.dump(OUT / 'visibility-verification.json', {'larger_brighter_effects': checks, 'timelines_identical': True, 'actors_and_terrain_identical': True, 'source_drawings_and_alpha_identical': True, 'ivory_ally_marker_identical': True, 'comparison_limit': 'Only the transient floating name antialias rectangle is excluded at frames 17-33; actor bodies remain compared.'})
    boards = []
    for i in range(144):
        board = Image.new('RGB', (1232, 1238), '#0b1218'); draw = ImageDraw.Draw(board)
        for row, (sid, title) in enumerate([('OC04', '25 · Prison sigillaire'), ('OC07', '26 · Réanimation')]):
            for col, (root, version, label) in enumerate([(BASE, 'A01', 'Version précédente'), (OUT, 'A02', 'Tracés renforcés')]):
                x = 12 + col * 616; y = 12 + row * 616
                draw.text((x, y), f'{title} · {label}', font=previous.base.font(19, True), fill='#e0e7e7')
                with Image.open(root / 'frames' / f'{sid}-{version}-HAUT' / f'{i:03}.png') as image:
                    board.paste(image, (x, y + 30))
        boards.append(board)
    previous.previous.drains.save_animation(OUT / 'visibilite-comparaison-64.gif', boards, 24)
    (OUT / 'package-base-source.py').write_bytes(Path(previous.__file__).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'comparison': 'native 64px', 'source_drawings_unchanged': True, 'timelines_unchanged': True, 'status': 'both revisions awaiting review'}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--prepare', action='store_true'); args = parser.parse_args()
    prepare() if args.prepare else package()
