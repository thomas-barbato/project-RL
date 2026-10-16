"""ANIM-S15: direct 64 px form drawings and rapid metamorphosis prototypes."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_ritual_blood as previous

ROOT = previous.ROOT
BASE = previous.OUT
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-41'
base = previous.rituals.base
encoding = previous.rituals.previous.drains


def save_animation(path, images, fps):
    # Keep one palette, weighted towards the brief coloured transitions rather than long grey-floor waits.
    width, height = images[0].size
    centres = [(288, 288)] if width == 576 else [(300, 354), (916, 354)]
    samples = [images[0].resize((256, round(height * 256 / width)), Image.Resampling.NEAREST)]
    for frame in (14, 19, 21, 23, 25, 35, 119, 174):
        for x, y in centres: samples.append(images[frame].crop((x - 112, y - 144, x + 112, y + 64)))
    sheet = Image.new('RGB', (256, sum(p.height for p in samples)))
    y = 0
    for sample in samples: sheet.paste(sample, (0, y)); y += sample.height
    # Reserve colour entries for chromatic VFX/skin/plates; grey floor pixels must not exhaust the palette.
    colourful = [pixel for sample in samples[1:] for pixel in sample.get_flattened_data() if max(pixel) - min(pixel) > 24]
    assert colourful
    strip = Image.new('RGB', (256, (len(colourful) + 255) // 256)); strip.putdata(colourful)
    background = sheet.quantize(colors=64, method=Image.Quantize.MEDIANCUT)
    foreground = strip.quantize(colors=192, method=Image.Quantize.MEDIANCUT)
    palette = Image.new('P', (1, 1)); palette.putpalette(background.getpalette()[:192] + foreground.getpalette()[:576])
    quantized = [im.quantize(palette=palette, dither=Image.Dither.NONE) for im in images]
    bounds = [round(i * 1000 / fps / 10) * 10 for i in range(len(images) + 1)]
    delays = [b - a for a, b in zip(bounds, bounds[1:])]
    quantized[0].save(path, save_all=True, append_images=quantized[1:], duration=delays, loop=0, disposal=2, optimize=False)
    with Image.open(path) as check:
        elapsed = 0; peak = None; unique = set()
        for i in range(check.n_frames):
            check.seek(i); rgba = check.convert('RGB'); unique.add(hashlib.sha256(rgba.tobytes()).hexdigest())
            if elapsed <= bounds[21] < elapsed + check.info['duration']: peak = rgba.copy()
            elapsed += check.info['duration']
        assert elapsed == bounds[-1] and len(unique) > 12 and peak is not None
        errors = []
        for x, y in centres:
            box = (x - 112, y - 144, x + 112, y + 64)
            for native, encoded in zip(images[21].crop(box).get_flattened_data(), peak.crop(box).get_flattened_data()):
                green = native[1] > native[0] + 40 and native[1] > native[2] + 40
                purple = native[2] > native[1] + 40 and native[0] > native[1] + 25
                if green or purple: errors.append(max(abs(a - b) for a, b in zip(native, encoded)))
        assert errors
        mean = sum(errors) / len(errors); p95 = sorted(errors)[int((len(errors) - 1) * .95)]
        assert mean < 18 and p95 < 45, (path.name, mean, p95)
        return {'file': path.name, 'native_size': [width, height], 'distinct_frames': len(unique), 'frames': check.n_frames, 'duration_ms': elapsed, 'loop': 0, 'palette': 'common palette:64 background entries plus192 chromatic transition/actor entries', 'peak_colour_mean_max_channel_error': round(mean, 2), 'peak_colour_p95_max_channel_error': p95}


def gifs_only():
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8')); animations = []
    for entry in entries:
        if entry['folder'] not in ('HAUT', 'MUR', 'RENOUVELLEMENT'): continue
        sequences = []
        for view in entry['views']:
            frames = []
            for i in range(180):
                with Image.open(OUT / 'frames' / view['variant'] / f'{i:03}.png') as im: frames.append(im.convert('RGB'))
            sequences.append(frames); animations.append(save_animation(OUT / f"{view['variant']}-64.gif", frames, 24))
        boards = []
        for i in range(180):
            board = Image.new('RGB', (1232, 650), '#0b1218'); draw = ImageDraw.Draw(board)
            for j, view in enumerate(entry['views']):
                x = 12 + j * 616
                draw.text((x, 10), f"{view['number']} · {view['title']}", font=base.font(20, True), fill='#e0e7e7')
                draw.text((x, 40), view['variant'], font=base.font(15), fill='#a5b9c5'); board.paste(sequences[j][i], (x, 66))
            boards.append(board)
        animations.append(save_animation(OUT / f"transformations-{entry['folder'].lower()}-64.gif", boards, 24))
    report = json.loads((OUT / 'verification.json').read_text(encoding='utf-8')); report['animations'] = animations
    report['gif_palette_revision'] = 'Weighted colour sampling for brief coloured VFX; original PNG captures unchanged.'
    base.dump(OUT / 'verification.json', report)
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'gifs_reencoded': len(animations), 'peak_colour_checks': 'passed', 'native_pngs_unchanged': True}))


def protected():
    hashes = previous.protected()
    for p in BASE.rglob('*'):
        if p.is_file() and not any(part in ('native-curvature-trial', 'native-thickness-trial') for part in p.relative_to(BASE).parts):
            hashes[p.relative_to(ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def protect():
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT / 'review-protection.json').exists()
    base.dump(OUT / 'review-protection.json', protected())
    print(json.dumps({'protected_baseline': str(OUT)}))


def prepare(sources):
    OUT.mkdir(parents=True, exist_ok=True)
    if not (OUT / 'review-protection.json').exists(): protect()
    packs = []
    for kind, source in sources.items():
        (OUT / f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as image:
            assert image.mode == 'RGBA' and image.getchannel('A').getextrema()[0] == 0
            if kind.endswith('_fx'):
                assert image.width * 2 == image.height * 3
                side = image.width // 3; atlas = Image.new('RGBA', (1536, 256))
                for i in range(6):
                    x = i % 3 * side; y = i // 3 * side
                    atlas.paste(image.crop((x, y, x + side, y + side)).resize((256, 256), Image.Resampling.LANCZOS), (i * 256, 0))
                atlas.save(OUT / f'{kind}-atlas.png')
                packs.append({'kind': kind, 'source_size': list(image.size), 'layout': [3, 2], 'cell_size': [256, 256], 'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest()})
            else:
                bounds = image.getchannel('A').point(lambda v: 255 if v > 32 else 0).getbbox(); assert bounds
                crop = image.crop(bounds); scale = min(62 / crop.width, 62 / crop.height)
                width = max(1, round(crop.width * scale)); height = max(1, round(crop.height * scale))
                sprite = crop.resize((width, height), Image.Resampling.LANCZOS)
                tile = Image.new('RGBA', (64, 64)); tile.paste(sprite, ((64 - width) // 2, 63 - height)); tile.save(OUT / f'{kind}-64.png')
                packs.append({'kind': kind, 'source_size': list(image.size), 'source_bounds': bounds, 'native_size': [64, 64], 'resized_directly_from_high_resolution': True, 'no_intermediate_32px': True, 'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest()})
    base.dump(OUT / 'drawing-pack.json', {'mode': 'Built-in imagegen', 'packing': packs, 'postprocessing': 'Alpha-bounds crop and direct uniform 64 px resizing for bodies; equal-cell crop/uniform atlas packing for VFX. No Python repainting.'})
    base.dump(OUT / 'review.json', {'batch': 'ANIM-S15', 'status': 'awaiting_visual_review', 'native_tile_size': 64,
        'candidates': [{'number': n, 'id': sid, 'name': title, 'variant': sid + '-A01', 'body_variant': sid + '-BODY01', 'existing_body_reference': ref, 'visual_status': 'awaiting_visual_review', 'body_status': 'awaiting_visual_review'} for n, sid, title, ref in [(27, 'TR63', 'Cuirassé', 62), (28, 'TR64', 'Possédé', 61)]],
        'visuals': 'New directly rendered 64 px comic bodies preserve the existing reference silhouettes and palettes. Green organic ribbons for Predator, visible purple angular ribbons/facets for Colossus. Rapid .38s entry and .30s return; central body stays visible through the VFX.',
        'rules_retained': 'At rank11/Willpower10 both forms last22 turns with10-turn cooldowns. The form activates logically at launch; animations add no preparation. Renewal after cooldown restores22 turns without an intermediate human pose. The displayed costs are15 and17 energy; the fixture does not resolve any energy or HP.',
        'scope': 'Debug-only appearance/transition proposal. No form attacks yet, real combat, skills/stat bonuses, HP ratio changes, healing, loot, AI, RNG, inventory or saves. No campaign integration and no replacement of the existing bestiary bank.'})
    launcher = (BASE / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--lot-rituels-occultes' '--rituels-sang-fluide'", "'--lot-transformations'")
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher, encoding='utf-8-sig')
    print(json.dumps({'prepared': str(OUT), 'native_bodies': 2, 'effect_atlases': 2}))


def package():
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8')); assert protected() == baseline
    encoding.OUT = OUT; encoding.convert(); encoding.save_animation = save_animation
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8')); assert len(entries) == 7
    checks = []; sequences = {}; animations = []
    for entry in entries:
        folder = OUT / 'native' / entry['folder']; timeline = json.loads((folder / 'timeline.json').read_text(encoding='utf-8'))
        for j, view in enumerate(entry['views']):
            x, y, w, h = view['crop']; bounds = (x, y, x + w, y + h); frames = []
            out = OUT / 'frames' / view['variant']; out.mkdir(parents=True, exist_ok=True)
            for i in range(180):
                with Image.open(folder / f'{i:03}.png') as image: frame = image.crop(bounds).convert('RGB')
                frame.save(out / f'{i:03}.png', compress_level=1); frames.append(frame)
            diffs = {}
            for i in entry['control_frames']:
                with Image.open(folder / f'control-{i:03}.png') as quiet: diffs[i] = ImageChops.difference(frames[i], quiet.crop(bounds).convert('RGB'))
                for cy, row in enumerate(entry['map'].splitlines()):
                    for cx, symbol in enumerate(row):
                        if symbol == '#': assert diffs[i].crop((cx * 64, cy * 64, (cx + 1) * 64, (cy + 1) * 64)).getbbox() is None, (view['variant'], i, cx, cy)
            rows = [r['states'][j] for r in timeline]
            assert rows[17]['active'] and rows[17]['remaining'] == 22 and rows[17]['cooldown'] == 10
            assert all(diffs[i].getbbox() is None for i in (35, 47)), 'Entry effect persists after its brief transition.'
            assert any(diffs[i].getbbox() for i in (19, 21, 23, 25)), 'Entry VFX invisible.'
            if entry['quiet']:
                assert all(r['turn'] == 0 and r['active'] and r['remaining'] == 22 and r['cooldown'] == 10 for r in rows[17:])
                assert diffs[179].getbbox() is None
            elif entry['renew']:
                assert rows[110]['cooldown'] == 0 and rows[110]['remaining'] == 12
                assert rows[115]['active'] and rows[115]['renewal'] and rows[115]['remaining'] == 22 and rows[115]['launches'] == 2
                assert rows[179]['active'] and rows[179]['remaining'] == 10
                assert diffs[119].getbbox() and diffs[130].getbbox() is None
            else:
                assert rows[166]['active'] and rows[166]['remaining'] == 1
                assert not rows[170]['active'] and rows[170]['remaining'] == 0
                assert diffs[174].getbbox() and diffs[179].getbbox() is None
            sample = (4 * 64 + 3, 4 * 64 + 25)
            assert frames[3].getpixel(sample)[1] > frames[14].getpixel(sample)[1] + 25
            sequences[view['variant']] = frames
            if entry['folder'] in ('HAUT', 'MUR', 'RENOUVELLEMENT'):
                animations.append(encoding.save_animation(OUT / f"{view['variant']}-64.gif", frames, 24))
            checks.append({'variant': view['variant'], 'walls_clear': True, 'entry_finishes_while_form_persists': True, 'timers_require_explicit_turns': True, 'renewal_or_expiry_checked': True})
        if entry['folder'] == 'HAUT':
            for i in (14, 21, 27, 35, 166, 174, 179):
                with Image.open(folder / f'{i:03}.png') as image: image.save(OUT / f'phase-{i:03}-native.png')
    for scenario in ('HAUT', 'MUR', 'RENOUVELLEMENT'):
        boards = []
        for i in range(180):
            board = Image.new('RGB', (1232, 650), '#0b1218'); draw = ImageDraw.Draw(board)
            for j, (sid, title) in enumerate([('TR63', '35 · Cuirassé'), ('TR64', '36 · Possédé')]):
                x = 12 + j * 616; variant = f'{sid}-A01-{scenario}'
                draw.text((x, 10), title, font=base.font(20, True), fill='#e0e7e7'); draw.text((x, 40), variant, font=base.font(15), fill='#a5b9c5')
                board.paste(sequences[variant][i], (x, 66))
            boards.append(board)
        animations.append(encoding.save_animation(OUT / f'transformations-{scenario.lower()}-64.gif', boards, 24))
    body_board = Image.new('RGB', (600, 375), '#0b1218'); draw = ImageDraw.Draw(body_board)
    for j, (kind, title, sid) in enumerate([('cuirasse', '35 · Cuirassé 63', 'TR63'), ('possede', '36 · Possédé 64', 'TR64')]):
        with Image.open(OUT / f'{kind}-64.png') as body:
            x = 24 + 300 * j; draw.text((x, 15), title, font=base.font(18, True), fill='#e0e7e7')
            body_board.paste(body.resize((192, 192), Image.Resampling.NEAREST), (x, 52), body.resize((192, 192), Image.Resampling.NEAREST))
            body_board.paste(body, (x + 64, 278), body)
            draw.text((x, 351), sid + '-BODY01 · natif 64 px', font=base.font(14), fill='#a5b9c5')
    body_board.save(OUT / 'bodies-native.png')
    base.dump(OUT / 'verification.json', {'protected_files_unchanged': len(baseline), 'controls': checks, 'animations': animations, 'resampling_of_native_capture': False, 'native_body_sources': 'Direct64 px PNGs from new high-resolution drawings; the 192px views are explicit nearest-neighbour inspection zooms only.'})
    for source, name in [('src/agent_skill_lab_review/fire_painted/more_forms.rs', 'render-source.rs'), ('src/agent_skill_lab_review/fire_painted/zones.rs', 'clip-source.rs'), ('src/agent_skill_lab_review.rs', 'render-parent-source.rs')]: (OUT / name).write_bytes((ROOT / source).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes()); write_html(entries)
    print(json.dumps({'native_views': 14, 'animations': len(animations), 'controls': 'passed', 'protected_files': len(baseline)}))


def write_html(entries):
    manifest = [{k: e[k] for k in ('folder', 'quiet', 'renew', 'turn_frames', 'renewal_frame', 'views')} for e in entries]
    script = '''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences,renewed=false;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast');
function entry(){return manifest.find(e=>e.folder===scenario.value);}
function start(){return turn===0?17:entry().turn_frames[turn-1]||17;}
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return f%17;if(entry().quiet)return Math.min(17+f,47);const end=turn===22?179:turn===0?47:entry().turn_frames[turn]-1;return Math.min(start()+f,end);}
function remaining(){return !launched?0:entry().quiet?22:renewed&&(turn!==10||elapsed>=5*1000/24)?32-turn:Math.max(0,22-turn);}
function ui(){cast.disabled=!ready||launched;next.disabled=!ready||!launched||turn>=22||elapsed<650||entry().quiet;document.getElementById('status').textContent=!launched?'Aperçu clignotant · avant lancement':`Tour ${turn} · forme ${remaining()} tours · ${entry().quiet?'attente sans tour':'tours manuels'}`;}
async function load(){ready=false;ui();const n=++token,e=entry();const data=await Promise.all(e.views.map(async v=>[v.id,await Promise.all(Array.from({length:180},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${v.variant}/${String(i).padStart(3,'0')}.png`;})))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas'))canvas.getContext('2d').drawImage(sequences.get(canvas.dataset.skill)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;if(entry().renew&&turn===10){renewed=true;}ui();}};
cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;renewed=false;elapsed=0;ui();}};
document.getElementById('reset').onclick=()=>{launched=false;turn=0;renewed=false;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;renewed=false;elapsed=0;load();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST', json.dumps(manifest))
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Transformations</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Transformations · Prédateur et Colosse</h1><p class="muted">ANIM-S15 · corps et transitions proposés · cases natives64 px</p><div class="controls"><select id="scenario"><option value="HAUT">Vers le haut</option><option value="DROITE">Vers la droite</option><option value="GAUCHE">Vers la gauche</option><option value="BAS">Vers le bas</option><option value="MUR">Mur proche</option><option value="ATTENTE">Attente sans tour</option><option value="RENOUVELLEMENT">Renouvellement préparé au tour 10</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="pair"><article><h2>35 · Cuirassé</h2><p class="muted">TR63-A01 · corps TR63-BODY01 · référence63</p><canvas data-skill="TR63" width="576" height="576"></canvas></article><article><h2>36 · Possédé</h2><p class="muted">TR64-A01 · corps TR64-BODY01 · référence64</p><canvas data-skill="TR64" width="576" height="576"></canvas></article></main>
<p>La métamorphose est active dès le lancement. La transition visuelle est brève ; la forme reste présente pendant ses22 tours, puis l’agent retrouve son apparence humaine. Attendre sans passer de tour conserve la forme et sa recharge. Dans la scène de renouvellement, un lancement préparé remet la durée à22 tours au tour 10, quand la recharge est prête.</p><p>Les corps sont redessinés dans le style BD et conditionnés directement en64 px depuis leurs sources haute définition. Ils conservent les références du Cuirassé63 et du Possédé64. Les corps et leurs deux transitions attendent leur validation propre. Les attaques de chaque forme feront l’objet de lots suivants.</p><p class="muted">Prototype visuel isolé : aucune statistique, attaque, énergie, soin, IA, loot, inventaire, sauvegarde ou règle de PV n’est appliquée. Les murs coupent les VFX ; le lancement et l’expiration n’ajoutent pas de dégâts aux personnages voisins.</p><p><a href="transformations-haut-64.gif">Séquence animée des deux formes</a> · <a href="transformations-mur-64.gif">Mur proche</a> · <a href="transformations-renouvellement-64.gif">Renouvellement</a> · <a href="bodies-native.png">Corps numérotés : natif64 px et zoom ×3</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p><p class="muted">Dessins créés avec imagegen intégré : <a href="cuirasse-source.png">Prédateur</a> / <a href="imagegen-cuirasse-prompt.txt">prompt</a> · <a href="possede-source.png">Colosse</a> / <a href="imagegen-possede-prompt.txt">prompt</a> · <a href="cuirasse_fx-source.png">Transition mutagénique</a> / <a href="imagegen-cuirasse_fx-prompt.txt">prompt</a> · <a href="possede_fx-source.png">Transition du Colosse</a> / <a href="imagegen-possede_fx-prompt.txt">prompt</a>.</p><script>SCRIPT</script></html>'''.replace('SCRIPT', script)
    (OUT / 'index.html').write_text(html, encoding='utf-8'); (OUT / 'gallery-script.js').write_text(script, encoding='utf-8')


