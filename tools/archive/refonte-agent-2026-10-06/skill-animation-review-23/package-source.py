"""ANIM-S07: native painted Nova/Mur/Gel review; campaign assets stay protected."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_fire_speed_fast as previous
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-23'
GENERATED = Path(r'C:\Users\User\.codex\generated_images\01a11010-433f-7fc2-ac29-ffb696dec5ea')
SOURCE_NAMES = {'nova': 'exec-286e3544-a8aa-41ba-a0c4-472364c61019.png',
                'wall': 'exec-66682f2e-bfd3-44fb-9d42-eac802e95853.png'}


def protected():
    hashes = previous.protected()
    for p in sorted(previous.OUT.rglob('*')):
        if p.is_file():
            hashes[p.relative_to(base.ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    assert not (OUT / 'review-protection.json').exists()
    OUT.mkdir(parents=True, exist_ok=True)
    base.dump(OUT / 'review-protection.json', protected())
    pack = []
    for kind, source_name in SOURCE_NAMES.items():
        source = GENERATED / source_name
        (OUT / f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as image:
            assert image.mode == 'RGBA' and image.size == (1536, 1024)
            alpha_min, alpha_max = image.getchannel('A').getextrema()
            assert alpha_min == 0 and alpha_max >= 250
            atlas = Image.new('RGBA', (1536, 256))
            for i in range(6):
                x, y = (i % 3)*512, (i // 3)*512
                # Uniform cell resampling only: never fit frames separately or repaint.
                cell = image.crop((x, y, x+512, y+512)).resize((256, 256), Image.Resampling.LANCZOS)
                atlas.paste(cell, (i*256, 0))
            atlas.save(OUT / f'{kind}-atlas.png')
            pack.append({'kind': kind, 'source_size': list(image.size), 'layout': [3, 2],
                         'cell_size': [256, 256], 'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                         'preserve_alpha': True, 'common_anchor': [.5, .5] if kind == 'nova' else [.5, .9]})
    base.dump(OUT / 'drawing-pack.json', {'mode': 'Built-in imagegen', 'packing': pack,
        'postprocessing': 'Mechanical equal-cell cropping and uniform resize; no Python painting.'})
    base.dump(OUT / 'review.json', {
        'batch': 'ANIM-S07', 'status': 'awaiting_visual_review', 'native_tile_size': 64,
        'candidates': [{'number': number, 'skill_id': sid, 'variant': sid+'-A01', 'name': title,
                        'status': 'awaiting_visual_review'} for number, sid, title in
                       [(15, 'FR02', 'Nova de givre'), (16, 'FR03', 'Mur de glace')]],
        'nova': 'One fast expanding ice wave, 0.45 seconds, centred on the caster. Floor spill is allowed; full solid walls clip it. No persistent floor zone. Prepared successful Gel is attached to targets only.',
        'wall': 'Blocks rise in 0.22 seconds. Each block occupies one free cell, blocks movement, and can break separately. Prepared melee contact freezes the attacker; the second prepared hit breaks a block. The frozen attacker can still attack. Once Gel ends it crosses the gap.',
        'gel_candidate': {'variant': 'GEL-A03', 'status': 'awaiting_visual_review',
                          'description': 'Small translucent ice facets around the feet, behind and in front of the unchanged body. Upper body stays readable. The previous FR01 visual is not replaced.'},
        'timing': 'Animation clock never advances turns, cooldowns, Gel or wall duration. Nova ends once; the wall/Gel remain until explicit turns. At rank 11 the fixture uses catalogue Gel=4, wall=5 and contact Gel=2 turns.',
        'scope': 'Debug-only visual fixture. No campaign, GameState, damage, RNG, real AI or save changes. Freeze success and block destruction are prepared examples, not balance claims. Final combat targeting/geometry is separate.',
        'preview': 'Blinking cyan cells disappear upon cast. Four cardinal orientations and solid-cloison case.',
        'blood': 'No blood for pure Frost. Prepared initial Burn is removed by successful Gel.',
        'unchanged': 'Previously approved icons, catalogue, actor bodies/equipment, fire drawings and reviews.'})
    launch = (previous.OUT / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    for flag in ('--lot-feu-final', '--lot-feu-final-peint'):
        launch = launch.replace(flag, '--lot-froid-ouverture')
    if '--lot-froid-ouverture' not in launch:
        # Existing review launcher routes to the last fire lot.
        start = launch.index("try {")
        end = launch.index('\nfinally', start)
        launch = launch[:start] + "try { & $reviewBinary '--apercu-laboratoire-competences' '--lot-froid-ouverture' }" + launch[end:]
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launch, encoding='utf-8-sig')
    print(json.dumps({'prepared': str(OUT), 'protected_files': len(json.loads((OUT / 'review-protection.json').read_text()))}))


def convert_raw_frames():
    native = (OUT / 'native').resolve()
    converted = 0
    for raw in native.rglob('*.rgba'):
        assert raw.resolve().is_relative_to(native)
        pixels = raw.read_bytes()
        assert len(pixels) == 1280*720*4
        png = raw.with_suffix('.png')
        Image.frombytes('RGBA', (1280, 720), pixels).save(png, compress_level=1)
        with Image.open(png) as check:
            assert check.tobytes() == pixels, 'Lossless export changed native pixels.'
        raw.unlink()  # Checked local temporary raw frame; verified PNG preserves all pixels.
        converted += 1
    base.dump(OUT / 'native-encoding-verification.json', {'converted_frames_this_run': converted,
        'lossless_png_frames': sum(1 for _ in native.rglob('*.png')),
        'pixel_identical_to_native_readback': True, 'resampling': False})


def verify_timeline(e, timeline):
    assert all(t['turn'] == 0 and t['cooldowns'] == [6, 8] for t in timeline[17:72])
    assert all(t['nova_active'] for t in timeline[17:28])
    assert all(not t['nova_active'] for t in timeline[28:])
    expected_gel = [4, 4, 4, 0 if e['folder'] == 'CLOISON' else 4, 0]
    count = 4 if e['folder'] == 'CLOISON' else 7
    assert all(t['freeze'][0] == expected_gel and t['burn'][0] == [0]*5
               and t['freeze'][1] == [0]*5 and t['wall_remaining'] == 5
               and len(t['blocks']) == count for t in timeline[17:72])
    assert [timeline[i]['turn'] for i in (72, 89, 106, 123, 140)] == [1, 2, 3, 4, 5]
    assert timeline[72]['freeze'][1][0] == 2 and timeline[72]['attacker'] == [4, 3]
    assert timeline[72]['burn'][1][0] == 0 and len(timeline[72]['blocks']) == count
    assert timeline[89]['freeze'][1][0] == 1 and timeline[89]['attacker'] == [4, 3]
    assert [4, 4] not in timeline[89]['blocks'] and len(timeline[89]['blocks']) == count-1
    assert timeline[106]['freeze'][1][0] == 0 and timeline[106]['attacker'] == [4, 4]
    assert all(t['attacker_walkable'] for t in timeline)
    assert timeline[140]['wall_remaining'] == 0 and not timeline[140]['blocks']
    for start, end in [(72, 89), (89, 106), (106, 123), (123, 140), (140, 168)]:
        assert all(t['freeze'] == timeline[start]['freeze'] and t['cooldowns'] == timeline[start]['cooldowns']
                   and t['wall_remaining'] == timeline[start]['wall_remaining'] for t in timeline[start:end])


def package():
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert protected() == baseline, 'Previously accepted assets changed.'
    convert_raw_frames()
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    assert len(entries) == 10
    sequences, controls, animations = {}, [], []
    for e in entries:
        x, y, w, h = e['crop']
        bounds = (x, y, x+w, y+h)
        folder = OUT / 'native' / e['folder']
        target = OUT / 'frames' / e['variant']
        target.mkdir(parents=True, exist_ok=True)
        frames = []
        for i in range(e['frames']):
            with Image.open(folder / f'{i:03}.png') as im:
                assert im.size == (1280, 720)
                frame = im.crop(bounds).convert('RGB')
                frame.save(target / f'{i:03}.png', compress_level=1)
                frames.append(frame)
        diffs = {}
        for phase in e['control_frames']:
            with Image.open(folder / f'control-{phase:03}.png') as im:
                quiet = im.crop(bounds).convert('RGB')
                diffs[phase] = ImageChops.difference(frames[phase], quiet)
            for cy, row in enumerate(e['map'].splitlines()):
                for cx, symbol in enumerate(row):
                    if symbol == '#':
                        assert diffs[phase].crop((cx*64, cy*64, (cx+1)*64, (cy+1)*64)).getbbox() is None, f'Ice on solid wall: {e["variant"]}, frame {phase}, {cx},{cy}'
        assert diffs[23].getbbox() and diffs[50].getbbox(), 'Cast or persistent status missing.'
        assert diffs[151].getbbox() is None and diffs[167].getbbox() is None, 'Effects survived expiration.'
        cx, cy = e['affected_cells'][0]
        sample = (cx*64+3, cy*64+25)
        bright, dim = frames[3].getpixel(sample), frames[14].getpixel(sample)
        assert bright[1] > dim[1]+25, 'Preview does not blink.'
        for phase in (29, 50):
            red, green, blue = frames[phase].getpixel(sample)
            assert not (red < 100 and green > 140 and blue > 140), 'Preview survived launch.'
        if e['id'] == 'FR02':
            for actor in e['targets']:
                if actor['label'] == 'E' or (e['folder'] == 'CLOISON' and actor['label'] == 'D'):
                    cx, cy = actor['cell']
                    assert diffs[50].crop((cx*64, cy*64, (cx+1)*64, (cy+1)*64)).getbbox() is None, 'Gel reached protected or out-of-area target.'
        timeline = json.loads((folder / 'timeline.json').read_text(encoding='utf-8'))
        verify_timeline(e, timeline)
        controls.append({'variant': e['variant'], 'solid_walls_clear': True, 'preview_blinks_and_ends_on_cast': True,
                         'waiting_does_not_advance_turns': True, 'instant_nova_ends': True,
                         'gel_and_wall_hold_until_turns': True, 'prepared_melee_break_opens_gap': True,
                         'frozen_actor_attacks_without_moving': True, 'final_effects_expire': True})
        sequences[e['variant']] = frames
        animations.append(save_animation(OUT / f"{e['variant']}-64.gif", frames, 24))
    for folder in ('HAUT', 'CLOISON'):
        pair = [next(e for e in entries if e['folder'] == folder and e['id'] == sid) for sid in ('FR02', 'FR03')]
        boards = []
        for i in range(168):
            board = Image.new('RGB', (1232, 650), '#0b1218')
            draw = ImageDraw.Draw(board)
            for col, e in enumerate(pair):
                left = 12+col*616
                draw.text((left, 10), f"{e['number']} · {e['title']}", font=base.font(20, True), fill='#e0e7e7')
                draw.text((left, 40), e['variant'], font=base.font(15), fill='#a4b9c6')
                board.paste(sequences[e['variant']][i], (left, 66))
            boards.append(board)
        stem = 'froid-ouverture' if folder == 'HAUT' else 'froid-cloison'
        animations.append(save_animation(OUT / f'{stem}-64.gif', boards, 24))
        for phase, name in [(23, 'lancement'), (50, 'maintien'), (96, 'breche'), (151, 'fin')]:
            boards[phase].save(OUT / f'{stem}-{name}.png')
    base.dump(OUT / 'verification.json', {'protected_files_unchanged': len(baseline),
        'resampling_of_native_capture': False, 'controls': controls, 'animations': animations,
        'source_art_mode': 'Built-in imagegen, two six-cell alpha sheets; equal-cell mechanical atlas packing.'})
    for source, name in [('src/agent_skill_lab_review/fire_painted/frost.rs', 'render-source.rs'),
                         ('src/agent_skill_lab_review/fire_painted/zones.rs', 'clip-source.rs'),
                         ('src/agent_skill_lab_review.rs', 'render-parent-source.rs')]:
        (OUT / name).write_bytes((base.ROOT / source).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries)
    print(json.dumps({'native_views': len(entries), 'animations': len(animations), 'protected_files_unchanged': len(baseline), 'controls': 'passed'}))


def write_html(entries):
    manifest = [{key: e[key] for key in ('variant', 'folder', 'id', 'frames', 'fps')} for e in entries]
    script = '''const manifest=MANIFEST,sequences=new Map();let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,loadToken=0;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast');
const stages=[17,72,89,106,123,140],ends=[71,88,105,122,139,167];
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return f%17;if(turn===0)return f<55?17+f:48+(f-55)%24;return Math.min(stages[turn]+f,ends[turn]);}
function ui(){const moving=launched&&elapsed<(turn===0?500:300);document.getElementById('status').textContent=launched?`Tour ${turn} · ${turn===0?(moving?'Lancement':'Onde terminée · Mur et Gel conservés'):turn===1?'Contact préparé : Gel sur A':turn===2?'Bloc brisé : A gelé peut encore attaquer':turn===3?'Gel terminé : A traverse la brèche':turn===4?'Nova : Gel terminé · Mur : 1 tour':'Mur expiré'}`:'Visée · avant lancement';cast.disabled=launched||!ready;next.disabled=!ready||!launched||turn>=5||moving;}
async function load(){ready=false;const token=++loadToken,selected=scenario.value;document.getElementById('loading').textContent='Chargement…';const entries=manifest.filter(e=>e.folder===selected);const loaded=await Promise.all(entries.map(async e=>{const images=await Promise.all(Array.from({length:e.frames},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${e.variant}/${String(i).padStart(3,'0')}.png`;})));return[e.id,images];}));if(token!==loadToken)return;sequences.clear();for(const[key,images]of loaded)sequences.set(key,images);elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · cases natives 64 px';ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const c of document.querySelectorAll('canvas'))c.getContext('2d').drawImage(sequences.get(c.dataset.skill)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;ui();load().catch(()=>document.getElementById('loading').textContent='Chargement impossible.');};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};ui();load().catch(()=>document.getElementById('loading').textContent='Conserver le sous-dossier frames avec la galerie.');requestAnimationFrame(tick);'''.replace('MANIFEST', json.dumps(manifest))
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Nova et Mur de glace</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:19px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}#status{color:#c4dcdc;font-weight:600}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Froid · Nova et Mur de glace</h1><p class="muted">ANIM-S07 · deux nouveaux rendus à examiner</p>
<p>Une onde de givre rapide autour de l’agent ; un mur qui surgit en blocs translucides. Le Gel se voit aux pieds des cibles, tandis que leur corps reste lisible. Les acteurs gardent leurs dessins actuels.</p>
<div class="controls"><select id="scenario"><option value="HAUT">Zone ouverte · haut</option><option value="DROITE">Zone ouverte · droite</option><option value="GAUCHE">Zone ouverte · gauche</option><option value="BAS">Zone ouverte · bas</option><option value="CLOISON">Cloison pleine</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button><span id="loading">Chargement…</span></div><p id="status"></p>
<main class="pair"><article><h2>15 · Nova de givre</h2><p class="muted">FR02-A01 · onde de 0,45 seconde</p><canvas data-skill="FR02" width="576" height="576"></canvas></article><article><h2>16 · Mur de glace</h2><p class="muted">FR03-A01 · apparition de 0,22 seconde</p><canvas data-skill="FR03" width="576" height="576"></canvas></article></main>
<p>Sans passer de tour, Nova s’éteint et le mur reste. Le Gel conserve aussi sa durée. Cliquez sur « +1 tour » pour voir le contact avec le mur, la destruction d’un bloc, puis le passage par la brèche. À l’expiration, les derniers blocs se brisent.</p>
<p>Dans cet exemple au rang 11 : Gel de Nova, 4 tours ; mur, 5 tours ; Gel au contact, 2 tours. A brûle initialement : le Gel supprime sa Brûlure. La cloison arrête l’onde et la pose du mur ; le monstre de l’autre côté est protégé.</p>
<p class="muted">Prototype visuel isolé. Les réussites de Gel et les coups qui brisent un bloc sont préparés pour cette revue. Aucun dégât, tirage, sauvegarde ou comportement de combat de la campagne n’est modifié. Ces deux rendus et le nouveau Gel aux pieds attendent leur validation.</p>
<p><a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> · <a href="froid-ouverture-64.gif">Séquence animée complète</a> · <a href="froid-cloison-64.gif">Essai devant une cloison</a></p>
<p class="muted">Dessins créés avec imagegen intégré : <a href="nova-source.png">source Nova</a> / <a href="imagegen-nova-prompt.txt">prompt</a> · <a href="wall-source.png">source Mur</a> / <a href="imagegen-wall-prompt.txt">prompt</a>.</p><script>SCRIPT</script></html>'''.replace('SCRIPT', script)
    (OUT / 'index.html').write_text(html, encoding='utf-8')
    (OUT / 'gallery-script.js').write_text(script, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--prepare', action='store_true')
    args = parser.parse_args()
    prepare() if args.prepare else package()
