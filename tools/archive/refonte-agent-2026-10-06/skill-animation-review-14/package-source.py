"""Review flame cadence at equal action times, without changing approved artwork."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_fire_zones as previous
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-14'

def protected():
    hashes = previous.protected()
    for p in sorted(previous.OUT.rglob('*')):
        if p.is_file():
            hashes[p.relative_to(base.ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes

def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT / 'review-protection.json').exists(), 'Preserve baseline.'
    base.dump(OUT / 'review-protection.json', protected())
    base.dump(OUT / 'review.json', {
        'batch': 'ANIM-S05-CADENCE01', 'status': 'awaiting_visual_review',
        'user_feedback': 'ca me semble tres bien :) meme si peut-être que l\u0027animation des flammes est un peu rapide ?',
        'previous_review': 'skill-animation-review-13',
        'feedback_scope': 'Favorable render feedback, with flame cadence comparison requested. No final approval inferred for the new cadence.',
        'candidates': [{'number': n, 'skill_id': sid, 'variant': f'{sid}-A04-{case}', 'status': 'awaiting_visual_review'}
                       for case in ('LIBRE', 'CLOISON') for n, sid in ((11, 'FE03'), (12, 'FE04'))],
        'cadences': {'original_drawn_frames_per_second': 12, 'candidate_drawn_frames_per_second': 8},
        'change': 'Only sampling of the existing eight painted flame frames is slowed by one third, including attached Burn. No new art or recolouring.',
        'preserved': 'Movement, rise/fade of the eruption, preparation, targeting blink and expiry, scripted turn events, durations, cooldowns, direction, footprint and clipping retain their original clocks.',
        'scope': 'Separate debug visual preview. Original review and default cadence preserved. Campaign and approved Souffle de feu unchanged.',
    })
    launcher = (previous.OUT / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher.replace('--lot-feu-zones', '--lot-feu-cadence'), encoding='utf-8-sig')

def crop(path, bounds):
    with Image.open(path) as im:
        assert im.size == (1280, 720)
        return im.crop(bounds).convert('RGB')

def quiet_matches(quiet, old_quiet, entry, phase):
    difference = ImageChops.difference(quiet, old_quiet)
    changed = sum(pixel != (0, 0, 0) for pixel in difference.get_flattened_data())
    if not changed:
        return 0
    # Independent GPU captures can differ on a few antialiased marker/name pixels.
    # Limit this allowance to known text rectangles at the one intermediate frame.
    assert phase == 40 and changed <= 100, 'Unexpected quiet-scene difference.'
    draw = ImageDraw.Draw(difference)
    for actor in entry['targets']:
        cx, cy = actor['cell']; px, py = (cx+.5)*64, (cy+.5)*64
        if entry['id'] == 'FE03' and actor['label'] == 'A':
            dx, dy = entry['moving_target_destination']
            progress = min(1, (40/24 - 33/24)/.45)
            px += (dx-cx)*64*progress; py += (dy-cy)*64*progress
        draw.rectangle((px-6, py-45, px+15, py-25), fill='black')
    player = {'HAUT': (4, 7), 'DROITE': (1, 4), 'GAUCHE': (7, 4)}[entry['direction']]
    px, py = (player[0]+.5)*64, (player[1]+.5)*64
    draw.rectangle((px-46, py-65, px+210, py-30), fill='black')
    assert difference.getbbox() is None, 'Actor body, position or lifecycle changed.'
    return changed

def package():
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert protected() == baseline, 'Previous artwork or review changed.'
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    old_entries = json.loads((previous.OUT / 'native/capture.json').read_text(encoding='utf-8'))
    assert len(entries) == len(old_entries) == 4
    sequences, old_sequences, checks, controls = {}, {}, [], []
    manifest = []
    for e, old in zip(entries, old_entries):
        assert e['variant'] == old['variant'].replace('A03', 'A04')
        assert e['flame_texture_fps'] == 8
        assert {k: v for k, v in e.items() if k not in ('variant', 'flame_texture_fps')} == {k: v for k, v in old.items() if k != 'variant'}
        x, y, w, h = e['crop']; bounds = (x, y, x+w, y+h)
        folder = OUT / 'native' / e['folder']
        destination = OUT / 'frames' / e['variant']; destination.mkdir(parents=True, exist_ok=True)
        images, originals = [], []
        for i in range(120):
            image = crop(folder / f'{i:03}.png', bounds)
            image.save(destination / f'{i:03}.png'); images.append(image)
            with Image.open(previous.OUT / 'frames' / old['variant'] / f'{i:03}.png') as im:
                originals.append(im.convert('RGB'))
        diffs, marker_pixels = {}, {}
        for phase in e['control_frames']:
            quiet = crop(folder / f'control-{phase:03}.png', bounds)
            old_quiet = crop(previous.OUT / 'native' / old['folder'] / f'control-{phase:03}.png', bounds)
            marker_pixels[phase] = quiet_matches(quiet, old_quiet, e, phase)
            diffs[phase] = ImageChops.difference(images[phase], quiet)
        for phase in (0, 3, 14, 16, 119):
            assert ImageChops.difference(images[phase], originals[phase]).getbbox() is None, 'Preview or final state changed.'
        for phase in (24, 40, 84):
            for cy, row in enumerate(e['map'].splitlines()):
                for cx, symbol in enumerate(row):
                    if symbol == '#':
                        assert diffs[phase].crop((cx*64, cy*64, (cx+1)*64, (cy+1)*64)).getbbox() is None
        for actor in e['targets']:
            if not actor['affected']:
                cx, cy = actor['cell']; area = (cx*64, cy*64, (cx+1)*64, (cy+1)*64)
                assert all(diffs[i].crop(area).getbbox() is None for i in (24, 40, 84))
        assert diffs[119].getbbox() is None
        assert ImageChops.difference(images[40], originals[40]).getbbox(), 'Flame cadence did not change.'
        if e['id'] == 'FE03':
            dx, dy = e['moving_target_destination']
            assert diffs[40].crop((dx*64, dy*64, (dx+1)*64, (dy+1)*64)).getbbox()
            assert diffs[84].getbbox()
        controls.append({'variant': e['variant'], 'same_turn_events_and_footprint': True,
                         'same_actor_positions_and_labels': True, 'same_preview_and_final_state': True,
                         'walls_and_protected_targets_unchanged': True, 'flame_frames_changed': True,
                         'antialiased_marker_pixel_differences_between_captures': marker_pixels})
        sequences[e['variant']] = images; old_sequences[e['variant']] = originals
        checks.append(save_animation(OUT / f"{e['variant']}-64.gif", images, 24))
        manifest.extend([
            {'variant': old['variant'], 'folder': '../../skill-animation-review-13/frames/' + old['variant'], 'frames': 120, 'fps': 24},
            {'variant': e['variant'], 'folder': 'frames/' + e['variant'], 'frames': 120, 'fps': 24}])
    for e in entries[:2]:
        boards = []
        for i in range(120):
            board = Image.new('RGB', (1232, 650), '#0b1218'); draw = ImageDraw.Draw(board)
            for column, images in enumerate((old_sequences[e['variant']], sequences[e['variant']])):
                left = 12 + column*616
                draw.text((left, 10), f'{e["number"]} · {e["title"]}', font=base.font(20, True), fill='#e0e7e7')
                draw.text((left, 40), 'Cadence actuelle' if column == 0 else 'Flammes un tiers plus lentes', font=base.font(16), fill='#9db3bd')
                board.paste(images[i], (left, 66))
            boards.append(board)
        checks.append(save_animation(OUT / f"{e['id']}-comparaison-64.gif", boards, 24))
        boards[40].save(OUT / f"{e['id']}-comparaison.png")
    base.dump(OUT / 'verification.json', {'status': 'awaiting_visual_review', 'protected_files_unchanged': len(baseline),
                                         'native_tile_size': 64, 'resampling': False, 'controls': controls, 'animations': checks})
    for source, name in [('src/agent_skill_lab_review/fire_painted/zones.rs', 'render-source.rs'),
                         ('src/agent_skill_lab_review.rs', 'render-parent-source.rs')]:
        (OUT / name).write_bytes((base.ROOT / source).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries, manifest)
    print(json.dumps({'protected_files_unchanged': len(baseline), 'animations': len(checks), 'controls': controls}))

def write_html(entries, manifest):
    cards = []
    for e in entries:
        old = e['variant'].replace('A04', 'A03')
        cards.append(f'<section><h2>{e["number"]} · {e["title"]} · {e["folder"].lower()}</h2><div class="pair"><article><h3>Cadence actuelle</h3><canvas data-variant="{old}" width="576" height="576"></canvas></article><article><h3>Flammes un tiers plus lentes</h3><canvas data-variant="{e["variant"]}" width="576" height="576"></canvas></article></div></section>')
    # Relative paths are from the review page, not from a frames folder.
    for entry in manifest:
        entry['folder'] = entry['folder'].replace('../../skill-animation-review-13/', '../skill-animation-review-13/')
    script = '''const manifest=MANIFEST;const sequences=new Map();let elapsed=0,last=0,paused=false,ready=false;
async function load(){for(const e of manifest){const images=await Promise.all(Array.from({length:e.frames},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`${e.folder}/${String(i).padStart(3,'0')}.png`;})));sequences.set(e.variant,{e,images});}elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · cases natives 64 px';}
function tick(now){if(last&&!paused)elapsed+=now-last;last=now;if(ready)for(const c of document.querySelectorAll('canvas')){const{e,images}=sequences.get(c.dataset.variant);c.getContext('2d').drawImage(images[Math.floor(elapsed*e.fps/1000)%e.frames],0,0);}requestAnimationFrame(tick);}
document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};document.getElementById('replay').onclick=()=>{elapsed=0;};load().catch(()=>document.getElementById('loading').textContent='Conserver les deux dossiers de revue côte à côte.');requestAnimationFrame(tick);'''.replace('MANIFEST', json.dumps(manifest))
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Cadence des flammes</title>
<style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}h3{font-size:17px;font-weight:normal;color:#aac1cb}p{line-height:1.5}.muted{color:#9eb1bc}.controls{display:flex;gap:12px;margin:20px 0}button{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:9px 13px;font:inherit;cursor:pointer}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}section{margin:28px 0}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Cadence des flammes</h1><p class="muted">ANIM-S05-CADENCE01 · 11 / FE03-A04 · 12 / FE04-A04 · essai à examiner</p>
<p>À gauche, le mouvement actuel. À droite, les mêmes flammes un tiers plus lentes, y compris Brûlure sur les personnages. Les déplacements, le lancement, la préparation et la fin des effets restent synchronisés.</p>
<div class="controls"><button id="pause">Pause</button><button id="replay">Rejouer</button><span id="loading">Chargement…</span></div>CARDS
<p class="muted">Les tours et recharges ne changent pas. Les montages conservent leur durée de cinq secondes. La cadence précédente et le Souffle de feu accepté restent conservés.</p><p><a href="Ouvrir_laboratoire.ps1">Laboratoire natif de la variante ralentie</a></p><script>SCRIPT</script></html>'''.replace('CARDS', ''.join(cards)).replace('SCRIPT', script)
    (OUT / 'index.html').write_text(html, encoding='utf-8')
    (OUT / 'gallery-script.js').write_text(script, encoding='utf-8')

if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--prepare', action='store_true'); args = parser.parse_args()
    prepare() if args.prepare else package()
