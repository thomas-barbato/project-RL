"""ANIM-S17: one direct64px Abysse body and isolated entry/return/renewal."""
from __future__ import annotations
import argparse
import hashlib
import json
import time
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_more_forms as previous

ROOT = previous.ROOT
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-44'
PRIOR = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-43'
GEN = Path('C:/Users/User/.codex/generated_images/01a11010-433f-7fc2-ac29-ffb696dec5ea')
base = previous.base
SOURCES = {
    'abysse': 'exec-65e86acc-714f-422d-946d-404389cd8663.png',
    'abysse_fx': 'exec-4de50acd-d62a-41fd-9ad4-ca03d2a74593.png',
    'cyan-trial': 'exec-146c4628-d90b-491d-8dd6-4e5a2a91e5b2.png',
}


def hashes():
    names = json.loads((PRIOR / 'review-protection.json').read_text(encoding='utf-8'))
    for p in PRIOR.rglob('*'):
        if p.is_file():
            names[p.relative_to(ROOT).as_posix()] = ''
    return {name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest() for name in names}


def protect():
    assert not (OUT / 'review-protection.json').exists()
    base.dump(OUT / 'review-protection.json', hashes())
    print(json.dumps({'protected_baseline': str(OUT)}))


def prepare():
    assert (OUT / 'review-protection.json').exists()
    packs = []
    for kind, filename in SOURCES.items():
        source = GEN / filename
        (OUT / f'{kind}-source.png').write_bytes(source.read_bytes())
        if kind == 'cyan-trial':
            continue
        with Image.open(source) as im:
            assert im.mode == 'RGBA' and im.getchannel('A').getextrema()[0] == 0
            if kind.endswith('_fx'):
                assert im.width * 2 == im.height * 3
                side = im.width // 3
                atlas = Image.new('RGBA', (1536, 256))
                for i in range(6):
                    x, y = (i % 3) * side, (i // 3) * side
                    cell = im.crop((x, y, x + side, y + side))
                    assert cell.getchannel('A').getbbox()
                    atlas.paste(cell.resize((256, 256), Image.Resampling.LANCZOS), (i * 256, 0))
                atlas.save(OUT / 'abysse_fx-atlas.png')
                packs.append({'kind': kind, 'source_size': list(im.size), 'layout': [3, 2], 'cell_size': [256, 256]})
            else:
                bounds = im.getchannel('A').point(lambda a: 255 if a > 32 else 0).getbbox()
                assert bounds
                crop = im.crop(bounds)
                scale = min(62 / crop.width, 62 / crop.height)
                w, h = round(crop.width * scale), round(crop.height * scale)
                tile = Image.new('RGBA', (64, 64))
                tile.paste(crop.resize((w, h), Image.Resampling.LANCZOS), ((64 - w) // 2, 63 - h))
                tile.save(OUT / 'abysse-64.png')
                packs.append({'kind': kind, 'source_size': list(im.size), 'bounds': bounds, 'native_size': [64, 64], 'direct_from_high_resolution': True, 'intermediate_32px': False})
    base.dump(OUT / 'drawing-pack.json', {'mode': 'Built-in imagegen', 'packing': packs, 'postprocessing': 'Alpha-bounds crop and direct uniform64px resizing; equal-cell crop/uniform atlas packing. No Python repainting.'})
    src = (ROOT / 'src/agent_skill_lab_review/fire_painted/more_forms.rs').read_text(encoding='utf-8')
    for old, new in [('ANIM-S15', 'ANIM-S17'), ('skill-animation-review-41', 'skill-animation-review-44'),
        ('const IDS: [&str; 2] = ["TR63", "TR64"];', 'const IDS: [&str; 1] = ["TR65"];'),
        ('const TITLES: [&str; 2] = ["Cuirassé", "Possédé"];', 'const TITLES: [&str; 1] = ["Abysse"];'),
        ('bodies: [Texture2D; 2]', 'bodies: [Texture2D; 1]'), ('effects: [Texture2D; 2]', 'effects: [Texture2D; 1]'),
        ('cuirasse', 'abysse'), ('35 +', '43 +'), ('"number":27+i', '"number":43+i'),
        ('(0..2).map', '(0..1).map'), ('for skill in 0..2', 'for skill in 0..1'),
        ('-BODY01', '-BODY02'), ('both_catalogue_forms', 'abysse_form')]:
        assert old in src, old
        src = src.replace(old, new)
    # Only one form; no duplicate proposal or fabricated second candidate.
    for ending in ('possede-64.png', 'possede_fx-atlas.png'):
        block = f'                load(include_bytes!(\n                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-44/{ending}"\n                )),\n'
        assert block in src
        src = src.replace(block, '')
    old_states = '''        let mut states = [
            State::new(0, map, direction, quiet),
            State::new(1, map, direction, quiet),
        ];'''
    assert old_states in src
    src = src.replace(old_states, '        let mut states = [State::new(0, map, direction, quiet)];')
    start = src.index('            let size = if s.skill == 0 {')
    end = src.index('            draw_texture_ex(', start)
    src = src[:start] + '            let size = vec2(168.0, 178.0);\n' + src[end:]
    src = src.replace('request_new_screen_size(1280.0, 720.0)', 'request_new_screen_size(640.0, 720.0)')
    src = src.replace('render_target(1280, 720)', 'render_target(640, 720)')
    src = src.replace('Rect::new(0.0, 0.0, 1280.0, 720.0)', 'Rect::new(0.0, 0.0, 640.0, 720.0)')
    src = src.replace('"capture_size":[1280,720]', '"capture_size":[640,720]')
    (ROOT / 'src/agent_skill_lab_review/fire_painted/abysse.rs').write_text(src, encoding='utf-8')
    launcher = (previous.OUT / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-transformations-suite', '--lot-abysse')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher, encoding='utf-8-sig')
    print(json.dumps({'prepared': str(OUT), 'body': 'TR65-BODY02', 'transition': 'TR65-A01'}))


def save_animation(path, images, fps=24):
    w, h = images[0].size
    centre = (288, 288) if w == 576 else (300, 354) if w == 616 else (320, 376)
    x, y = centre
    samples = [images[0].resize((256, round(h * 256 / w)), Image.Resampling.NEAREST)]
    for frame in (14, 19, 21, 23, 25, 35, 119, 174):
        samples.append(images[frame].crop((x - 112, y - 144, x + 112, y + 64)))
    sheet = Image.new('RGB', (256, sum(p.height for p in samples)))
    row = 0
    for sample in samples:
        sheet.paste(sample, (0, row))
        row += sample.height
    pixels = [p for im in samples[1:] for p in im.get_flattened_data()]
    colours = list(sheet.quantize(colors=64, method=Image.Quantize.MEDIANCUT).getpalette()[:192])
    for selected in ([p for p in pixels if p[2] > p[1] + 15 and p[0] > p[1] + 10],
                     [p for p in pixels if min(p) > 80 and p[0] > p[2] + 10],
                     [p for p in pixels if max(p) - min(p) > 18]):
        assert selected
        strip = Image.new('RGB', (256, (len(selected) + 255) // 256))
        strip.putdata(selected)
        colours += strip.quantize(colors=64, method=Image.Quantize.MEDIANCUT).getpalette()[:192]
    palette = Image.new('P', (1, 1))
    palette.putpalette(colours)
    encoded = [im.quantize(palette=palette, dither=Image.Dither.NONE) for im in images]
    bounds = [round(i * 1000 / fps / 10) * 10 for i in range(len(images) + 1)]
    encoded[0].save(path, save_all=True, append_images=encoded[1:], duration=[b - a for a, b in zip(bounds, bounds[1:])], loop=0, disposal=2, optimize=False)
    with Image.open(path) as check:
        elapsed = 0
        unique = set()
        peak = None
        for i in range(check.n_frames):
            check.seek(i)
            rgb = check.convert('RGB')
            unique.add(hashlib.sha256(rgb.tobytes()).hexdigest())
            if elapsed <= bounds[21] < elapsed + check.info['duration']:
                peak = rgb.copy()
            elapsed += check.info['duration']
        assert elapsed == bounds[-1] and len(unique) > 12 and peak is not None
        errors = []
        box = (x - 112, y - 144, x + 112, y + 64)
        for native, result in zip(images[21].crop(box).get_flattened_data(), peak.crop(box).get_flattened_data()):
            if native[2] > native[1] + 25 and native[0] > native[1] + 15:
                errors.append(max(abs(a - b) for a, b in zip(native, result)))
        assert errors
        mean = sum(errors) / len(errors)
        p95 = sorted(errors)[int((len(errors) - 1) * .95)]
        assert mean < 18 and p95 < 45, (path.name, mean, p95)
        return {'file': path.name, 'native_size': [w, h], 'duration_ms': elapsed, 'distinct_frames': len(unique), 'peak_purple_mean_error': round(mean, 2), 'peak_purple_p95_error': p95}


def package():
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert hashes() == baseline
    # Same timing/obstacle verifier as the approved forms; a single native panel.
    source = Path(previous.engine.__file__).read_text(encoding='utf-8')
    source = source.split("if __name__ == '__main__':")[0]
    for old, new in [('skill-animation-review-39', 'skill-animation-review-44'), ('ANIM-S13', 'ANIM-S17'),
        ("[('M01', '27 · Mue prédatrice'), ('TR61', '28 · Colosse biomécanique')]", "[('TR65', '43 · Abysse')]"),
        ("[('predator', '27 · Prédateur 62', 'M01'), ('colossus', '28 · Colosse 61', 'TR61')]", "[('abysse', '43 · Abysse 65', 'TR65')]"),
        ("'transformations.rs'", "'abysse.rs'"), ('fire_painted/transformations.rs', 'fire_painted/abysse.rs'),
        ("encoding.convert();", ""), ('(1232, 650)', '(616, 650)'), ('(600, 375)', '(320, 375)'),
        ('-BODY01', '-BODY02'), ("'native_views': 14", "'native_views': 7")]:
        source = source.replace(old, new)
    namespace = {'__name__': 'abysse_package', '__file__': str(Path(previous.engine.__file__))}
    exec(compile(source, namespace['__file__'], 'exec'), namespace)
    namespace['OUT'] = OUT
    namespace['protected'] = hashes
    namespace['save_animation'] = save_animation
    namespace['write_html'] = write_html
    namespace['package']()
    # Validate/export every cardinal direction and waiting, not only the three summaries.
    check = json.loads((OUT / 'verification.json').read_text(encoding='utf-8'))
    existing = {a['file'] for a in check['animations']}
    for e in json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8')):
        variant = e['views'][0]['variant']
        filename = variant + '-64.gif'
        if filename in existing:
            continue
        images = []
        for frame in range(180):
            with Image.open(OUT / 'frames' / variant / f'{frame:03}.png') as im:
                images.append(im.convert('RGB'))
        check['animations'].append(save_animation(OUT / filename, images))
    check['native_views'] = 7
    base.dump(OUT / 'verification.json', check)
    base.dump(OUT / 'review.json', {
        'batch': 'ANIM-S17', 'status': 'awaiting_visual_review', 'native_tile_size': 64,
        'candidates': [{'number': 43, 'id': 'TR65', 'name': 'Abysse', 'variant': 'TR65-A01', 'body_variant': 'TR65-BODY02', 'body_status': 'awaiting_visual_review', 'visual_status': 'awaiting_visual_review'}],
        'revision': 'Body02 replaces cyan limbs with organic purple flesh and weathered ivory ridges following the user request. Body01 cyan trial retained, not proposed for integration.',
        'visuals': 'Single eye, elongated purple limbs, ivory organic ridges; thick purple-black shadow curls enter .38s, return .30s and renew without human intermediate.',
        'rules_retained': 'Previously agreed rank11 example:22 turns, cooldown10, cost17 energy. Timers advance only on explicit turns. Renewal on turn10 restores22. No HP restoration or own-action cooldown reset.',
        'scope': 'Body and transition prototype only; no Allonge/Fente/Pas d’ombre yet. No new stats, HP, energy, AI, damage, loot, save or campaign integration.',
        'excluded_trials': ['cyan-trial-source.png']})
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    (OUT / 'adapted-engine-source.py').write_text(source, encoding='utf-8')
    print(json.dumps({'native_views': 7, 'animations': len(check['animations']), 'body': 'TR65-BODY02'}))


def write_html(entries):
    # Reuse the already reviewed manual-turn controller, reduced to one skill/canvas.
    script = (previous.OUT / 'gallery-script.js').read_text(encoding='utf-8')
    manifest = [{k: e[k] for k in ('folder', 'quiet', 'renew', 'turn_frames', 'renewal_frame', 'views')} for e in entries]
    start = script.index(';let ready')
    script = 'const manifest=' + json.dumps(manifest) + script[start:]
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Abysse</title>
<style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1120px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px;width:608px;max-width:none}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}.bodyboard{display:block;width:320px;height:375px}</style>
<h1>43 · Abysse</h1><p class="muted">ANIM-S17 · TR65-BODY02 / TR65-A01 · corps et transitions proposés · cases natives 64 px</p>
<div class="controls"><select id="scenario"><option value="HAUT">Vers le haut</option><option value="DROITE">Vers la droite</option><option value="GAUCHE">Vers la gauche</option><option value="BAS">Vers le bas</option><option value="MUR">Mur proche</option><option value="ATTENTE">Attente sans tour</option><option value="RENOUVELLEMENT">Renouvellement préparé au tour 10</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div>
<p id="status"></p><article><h2>Abysse · entrée et retour</h2><p class="muted">TR65-A01 · corps TR65-BODY02 · référence 65</p><canvas data-skill="TR65" width="576" height="576"></canvas></article>
<p>Entrée, retour et renouvellement de la forme. Le corps reste frontal dans les quatre orientations de la carte. La transition est brève ; la forme reste présente pendant ses 22 tours. Le renouvellement préparé au tour 10 remet la durée à 22 tours sans passer par le corps humain. Attendre sans avancer de tour ne diminue ni durée ni recharge.</p>
<p>Le cyan a été retiré : chair violette, creux sombres, crêtes ivoire. Dessin exporté directement depuis la source haute définition en 64 px. Ce nouveau corps et ses transitions attendent votre validation visuelle.</p>
<img class="bodyboard" src="bodies-native.png" alt="Abysse en 64 px natifs et zoom de contrôle ×3">
<p class="muted">Prototype isolé : Allonge, Fente abyssale et Pas d’ombre seront montrés dans le lot suivant. Aucun dégât, attribut, PV, énergie, soin, IA, loot ou sauvegarde n’est calculé ici.</p>
<p><a href="transformations-haut-64.gif">Séquence animée</a> · <a href="transformations-mur-64.gif">Mur proche</a> · <a href="transformations-renouvellement-64.gif">Renouvellement</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p>
<p class="muted">Dessins créés avec imagegen intégré : <a href="abysse-source.png">Corps sans cyan</a> / <a href="imagegen-abysse-body02-prompt.txt">prompt</a> · <a href="abysse_fx-source.png">Transition</a> / <a href="imagegen-abysse-fx-prompt.txt">prompt</a>. La première proposition cyan est conservée comme essai écarté.</p>
<script src="gallery-script.js"></script></html>'''
    (OUT / 'index.html').write_text(html, encoding='utf-8')
    (OUT / 'gallery-script.js').write_text(script, encoding='utf-8')


def convert_stream():
    native = (OUT / 'native').resolve()
    count = 0
    deadline = time.monotonic() + 900
    while time.monotonic() < deadline:
        for folder in native.glob('*'):
            if not folder.is_dir() or not (folder / 'timeline.json').exists():
                continue
            for raw in folder.glob('*.rgba'):
                assert raw.resolve().is_relative_to(native)
                data = raw.read_bytes()
                assert len(data) == 640 * 720 * 4
                png = raw.with_suffix('.png')
                Image.frombytes('RGBA', (640, 720), data).save(png, compress_level=1)
                with Image.open(png) as im:
                    assert im.tobytes() == data
                raw.unlink()
                count += 1
        if (native / 'capture.json').exists() and not list(native.rglob('*.rgba')):
            base.dump(OUT / 'native-encoding-verification.json', {'converted_frames': count, 'pixel_identical_to_native_readback': True, 'resampling': False, 'completed_folders_only': True})
            print(json.dumps({'losslessly_converted': count}))
            return
        time.sleep(1)
    raise RuntimeError('Native capture stream did not finish.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--protect-only', action='store_true')
    parser.add_argument('--prepare', action='store_true')
    parser.add_argument('--convert-stream', action='store_true')
    args = parser.parse_args()
    if args.convert_stream:
        convert_stream()
    elif args.protect_only:
        protect()
    elif args.prepare:
        prepare()
    else:
        package()
