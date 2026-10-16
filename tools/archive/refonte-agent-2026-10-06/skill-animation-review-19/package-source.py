"""ANIM-S06-A04: finite impacts, smaller/translucent art, turn-based attached Burn."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_fire_softened as previous
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-19'


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
    base.dump(OUT / 'review.json', {
        'batch': 'ANIM-S06-A04', 'status': 'awaiting_visual_review', 'native_tile_size': 64,
        'candidates': [
            {'number': n, 'skill_id': sid, 'variant': sid + ('-A04' if sid == 'FE05' else '-A03'),
             'status': 'awaiting_visual_review'} for n, sid in ((13, 'FE05'), (14, 'FE06'))],
        'visual_revision': {
            'projectile_size_pixels': 96, 'projectile_opacity': .90,
            'blast_size_pixels': 352, 'blast_opacity': .52,
            'combustion_plume_scales': [.52, .42], 'combustion_opacity': .65,
            'combustion_corona_removed': True, 'fade_out_seconds': .16,
            'fireball_flight_seconds': .65, 'blast_seconds': .55, 'combustion_seconds': .60},
        'fireball': 'The projectile travels once and explodes on the first monster or wall. The blast ends automatically. It creates no persistent damage zone.',
        'combustion': 'Two small translucent flames close onto the burning target; no circular explosion. The brief shock ends automatically. The old Burn is consumed; a prepared success may create a new attached Burn.',
        'timing': 'Instant-hit art ends in real time. Burn and cooldowns only decrease with explicit turns. Native montage passes turns at 3, 4 and 5 seconds; interactive idle loops only attached Burn after the impact.',
        'scope': 'Isolated visual fixture: no campaign integration, damage, RNG or balance changes. Final combat geometry/collision is a separate implementation.',
        'art': 'Reuses the existing image_gen source sheet and accepted painted Burn. Scale/alpha/composition changes only. Characters, masks and approved persistent effects unchanged.',
        'directions': 'Four cardinal directions, full cloison and monster interception.',
        'blood': 'Pure Fire contains no blood.',
        'accepted_temporal_policy': 'The latest user clarification distinguishes instantaneous hits from persistent effects. The previously accepted FE04-A05 turn-based display is preserved.',
    })
    (OUT / 'Ouvrir_laboratoire.ps1').write_bytes((previous.OUT / 'Ouvrir_laboratoire.ps1').read_bytes())
    for name in ('drawn-source.png', 'projectile-atlas.png', 'blast-atlas.png', 'imagegen-prompt.txt', 'drawing-pack.json'):
        (OUT / name).write_bytes((previous.OUT / name).read_bytes())


def package():
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert protected() == baseline
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    assert len(entries) == 10
    sequences, controls, animations = {}, [], []
    for e in entries:
        x, y, w, h = e['crop']
        bounds = (x, y, x + w, y + h)
        folder = OUT / 'native' / e['folder']
        target = OUT / 'frames' / e['variant']
        target.mkdir(parents=True, exist_ok=True)
        frames = []
        for i in range(e['frames']):
            with Image.open(folder / f'{i:03}.png') as im:
                assert im.size == (1280, 720)
                frame = im.crop(bounds).convert('RGB')
                frame.save(target / f'{i:03}.png')
                frames.append(frame)
        diffs = {}
        for phase in e['control_frames']:
            with Image.open(folder / f'control-{phase:03}.png') as im:
                diffs[phase] = ImageChops.difference(frames[phase], im.crop(bounds).convert('RGB'))
            for cy, row in enumerate(e['map'].splitlines()):
                for cx, symbol in enumerate(row):
                    if symbol == '#':
                        assert diffs[phase].crop((cx*64, cy*64, (cx+1)*64, (cy+1)*64)).getbbox() is None, f'Fire on wall: {e["variant"]}, frame {phase}'
        assert diffs[24].getbbox(), 'Projectile or focused shock missing.'
        assert diffs[143].getbbox() is None, 'Effects survived their expiration.'
        if e['id'] == 'FE05':
            assert diffs[3].getbbox() is None and diffs[14].getbbox() is None
        cx, cy = e['affected_cells'][0]
        sample = (cx*64+3, cy*64+25)
        bright, dim = frames[3].getpixel(sample), frames[14].getpixel(sample)
        assert bright[1] > dim[1]+25, 'Targeting preview does not blink.'
        for phase in (17, 40, 52):
            red, green, blue = frames[phase].getpixel(sample)
            assert not (red < 100 and green > 140 and blue > 140), 'Preview survived launch.'
        if e['fresh_burn']:
            assert diffs[52].getbbox() and diffs[73].getbbox(), 'Attached Burn missing after the impact.'
        else:
            assert all(diffs[i].getbbox() is None for i in (32, 40, 46, 52, 61, 73)), 'Instant shock did not finish or consumed Burn returned.'
        for actor in e['targets']:
            if e['folder'] == 'CLOISON-BAS' and actor['label'] == 'F':
                cx, cy = actor['cell']
                body = (cx*64+12, cy*64+12, cx*64+52, cy*64+52)
                assert all(diffs[i].crop(body).getbbox() is None for i in (40, 52, 61, 73))
        timeline = json.loads((folder / 'timeline.json').read_text(encoding='utf-8'))
        skill = 0 if e['id'] == 'FE05' else 1
        assert all(t['visual_active'][skill] for t in timeline[17:e['end_frame']])
        assert all(not t['visual_active'][skill] for t in timeline[e['end_frame']:])
        assert all(t['turn'] == 0 and t['cooldowns'] == timeline[17]['cooldowns'] and t['burn'] == timeline[17]['burn'] for t in timeline[17:72])
        assert [timeline[i]['turn'] for i in (72, 96, 120)] == [1, 2, 3]
        if e['id'] == 'FE05':
            assert all(t['projectile_visible'] and not t['explosion_visible'] for t in timeline[17:33])
            assert all(t['explosion_visible'] and not t['projectile_visible'] for t in timeline[33:46])
            assert all(not t['explosion_visible'] and not t['projectile_visible'] for t in timeline[46:])
            if e['collision_target'] is not None:
                hit = next(actor for actor in e['targets'] if actor['label'] == e['collision_target'])
                assert e['impact_cell'] == hit['cell']
            assert e['collision_target'] == ('D' if e['folder'] == 'INTERCEPTION' else None if e['folder'] == 'CLOISON-BAS' else 'A')
        controls.append({'variant': e['variant'], 'physical_walls_clear': True,
                         'instant_impact_ends_before_first_turn': True,
                         'waiting_preserves_burn_and_cooldowns': True,
                         'burn_after_impact': e['fresh_burn'], 'final_effects_expired': True})
        sequences[e['variant']] = frames
        animations.append(save_animation(OUT / f"{e['variant']}-64.gif", frames, 24))
    for folder in ('DROITE', 'CLOISON-BAS', 'INTERCEPTION'):
        subset = [e for e in entries if e['folder'] == folder]
        boards = []
        for i in range(144):
            board = Image.new('RGB', (1232, 650), '#0b1218')
            draw = ImageDraw.Draw(board)
            for col, e in enumerate(subset):
                left = 12+col*616
                draw.text((left, 10), f'{e["number"]} · {e["title"]}', font=base.font(20, True), fill='#e0e7e7')
                draw.text((left, 40), e['variant'], font=base.font(15), fill='#a4b9c6')
                board.paste(sequences[e['variant']][i], (left, 66))
            boards.append(board)
        stem = {'DROITE': 'feu-final', 'CLOISON-BAS': 'feu-final-cloison', 'INTERCEPTION': 'feu-final-interception'}[folder]
        animations.append(save_animation(OUT / f'{stem}-64.gif', boards, 24))
        boards[33].save(OUT / f'{stem}-impact.png')
        boards[52].save(OUT / f'{stem}-brulure.png')
    base.dump(OUT / 'verification.json', {
        'protected_files_unchanged': len(baseline), 'resampling_of_native_capture': False,
        'controls': controls, 'animations': animations, 'source_art_mode': 'Reuses image_gen assets; renderer transforms only.'})
    for source, name in [('src/agent_skill_lab_review/fire_painted/finale.rs', 'render-source.rs'),
                         ('src/agent_skill_lab_review/fire_painted/zones.rs', 'clip-source.rs'),
                         ('src/agent_skill_lab_review.rs', 'render-parent-source.rs')]:
        (OUT / name).write_bytes((base.ROOT / source).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries)
    print(json.dumps({'native_views': len(entries), 'animations': len(animations), 'protected_files_unchanged': len(baseline), 'controls': 'passed'}))


def write_html(entries):
    manifest = [{'variant': e['variant'], 'folder': e['folder'], 'id': e['id'], 'frames': e['frames'], 'fps': 24, 'fresh_burn': e['fresh_burn']} for e in entries]
    script = '''const manifest=MANIFEST,sequences=new Map();let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,loadToken=0;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast');
function ui(){const impact=turn===0&&elapsed<1200;document.getElementById('status').textContent=launched?`Tour ${turn} · ${impact?'Projectile et impacts brefs':turn<3?'Impact terminé · Brûlure en cours':'Effets terminés'} · recharges : Boule de feu ${Math.max(0,8-turn)}, Combustion ${Math.max(0,3-turn)} tours`:'Visée · avant lancement';cast.disabled=launched||!ready;next.disabled=!ready||!launched||turn>=8||impact;}
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return f%17;if(turn===0)return f<55?17+f:48+(f-55)%24;if(turn===1)return 72+f%24;if(turn===2)return 96+f%24;return 120+f%24;}
async function load(){ready=false;const token=++loadToken,selected=scenario.value;document.getElementById('loading').textContent='Chargement…';const loaded=await Promise.all(manifest.filter(e=>e.folder===selected).map(async e=>{const images=await Promise.all(Array.from({length:e.frames},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${e.variant}/${String(i).padStart(3,'0')}.png`;})));return[e.variant,images];}));if(token!==loadToken)return;sequences.clear();for(const[key,images]of loaded)sequences.set(key,images);elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · cases natives 64 px';ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const c of document.querySelectorAll('canvas')){const e=manifest.find(e=>e.folder===scenario.value&&e.id===c.dataset.skill);c.getContext('2d').drawImage(sequences.get(e.variant)[frame()],0,0);}}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;ui();load().catch(()=>document.getElementById('loading').textContent='Chargement impossible.');};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};ui();load().catch(()=>document.getElementById('loading').textContent='Conserver le sous-dossier frames avec la galerie.');requestAnimationFrame(tick);'''.replace('MANIFEST', json.dumps(manifest))
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Impacts de Feu allégés</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:19px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}#status{color:#c4dcdc;font-weight:600}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Boule de feu et Combustion</h1><p class="muted">ANIM-S06-A04 · 13 / FE05-A04 · 14 / FE06-A03 · rendus proposés</p>
<p>Projectile plus petit, explosion plus transparente et Combustion resserrée sur sa cible. Les impacts se terminent seuls. Seule la Brûlure éventuelle continue pendant ses tours.</p>
<div class="controls"><select id="scenario"><option value="DROITE">Zone ouverte · droite</option><option value="GAUCHE">Zone ouverte · gauche</option><option value="HAUT">Zone ouverte · haut</option><option value="INTERCEPTION">Un monstre intercepte la boule</option><option value="CLOISON-BAS">Cloison · bas · Combustion sans nouvelle Brûlure</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l'aperçu</button><button id="pause">Pause</button><span id="loading">Chargement…</span></div><p id="status"></p>
<main class="pair"><article><h2>13 · Boule de feu</h2><canvas data-skill="FE05" width="576" height="576"></canvas></article><article><h2>14 · Combustion</h2><canvas data-skill="FE06" width="576" height="576"></canvas></article></main>
<p>Laissez l'aperçu tourner sans cliquer : le projectile et le choc disparaissent. Les petites flammes restent sur les monstres brûlés jusqu'au passage de leurs tours. Pour revoir le lancement, cliquez sur « Recommencer l'aperçu », puis « Lancer ».</p>
<p>Dans la scène de cloison, Combustion ne réapplique pas de Brûlure : sa cible redevient sans flammes après le choc. La cloison coupe l'explosion et protège F.</p>
<p class="muted">Revue isolée avec des cas de Brûlure préparés. La durée visuelle ne répète pas les dégâts et ne fait pas avancer les tours. Les personnages et les animations persistantes déjà acceptées sont conservés. Les effets de combat de la refonte seront raccordés séparément.</p><p><a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> · <a href="drawn-source.png">Planche dessinée réutilisée</a>.</p><script>SCRIPT</script></html>'''.replace('SCRIPT', script)
    (OUT / 'index.html').write_text(html, encoding='utf-8')
    (OUT / 'gallery-script.js').write_text(script, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--prepare', action='store_true')
    args = parser.parse_args()
    prepare() if args.prepare else package()
