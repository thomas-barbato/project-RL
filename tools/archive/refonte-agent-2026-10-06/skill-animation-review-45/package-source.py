"""ANIM-S18: Abysse own-action bitmap packaging and native-capture QA."""
from __future__ import annotations
import argparse
import hashlib
import json
import time
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_abysse as previous

ROOT = previous.ROOT
BASE = previous.OUT
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-45'
base = previous.base


def hashes():
    names = json.loads((BASE / 'review-protection.json').read_text(encoding='utf-8'))
    for p in BASE.rglob('*'):
        if p.is_file():
            names[p.relative_to(ROOT).as_posix()] = ''
    return {name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest() for name in names}


def protect():
    assert not (OUT / 'review-protection.json').exists()
    base.dump(OUT / 'review-protection.json', hashes())
    print(json.dumps({'protected_baseline': str(OUT)}))


def prepare(sources):
    assert (BASE / 'visual-acceptance-01.json').exists()
    assert (OUT / 'review-protection.json').exists()
    packs = []
    for kind, source in sources.items():
        (OUT / f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as im:
            assert im.mode == 'RGBA' and im.getchannel('A').getextrema()[0] == 0
            assert im.width * 2 == im.height * 3
            side = im.width // 3
            atlas = Image.new('RGBA', (1536, 256))
            for i in range(6):
                x, y = (i % 3) * side, (i // 3) * side
                cell = im.crop((x, y, x + side, y + side))
                assert cell.getchannel('A').getbbox()
                atlas.paste(cell.resize((256, 256), Image.Resampling.LANCZOS), (i * 256, 0))
            atlas.save(OUT / f'{kind}-atlas.png')
            packs.append({'kind': kind, 'source_size': list(im.size), 'layout': [3, 2], 'cell_size': [256, 256], 'sha256': hashlib.sha256(source.read_bytes()).hexdigest()})
    base.dump(OUT / 'drawing-pack.json', {'tool': 'Built-in imagegen', 'postprocessing': 'Equal square cell crop and uniform resizing only; no Python repainting.', 'atlases': packs, 'approved_body_reused': 'review44/TR65-BODY02', 'approved_shadow_reused': 'review44/abysse_fx-atlas.png'})
    launcher = (BASE / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-abysse', '--lot-abysse-actions')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher, encoding='utf-8-sig')
    print(json.dumps({'new_atlases': len(packs), 'approved_body_redrawn': False}))


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
    raise RuntimeError('Capture did not complete.')


def save_animation(path, images, allow_static=False):
    samples = [images[0].resize((256, round(images[0].height * 256 / images[0].width)), Image.Resampling.NEAREST)]
    for offset in range(0, len(images), 96):
        for frame in (18, 19, 20, 21, 23, 25, 27, 30, 35):
            samples.append(images[offset + frame].resize((384, round(images[0].height * 384 / images[0].width)), Image.Resampling.NEAREST))
    sheet = Image.new('RGB', (384, sum(p.height for p in samples)))
    y = 0
    for im in samples:
        sheet.paste(im, (0, y)); y += im.height
    pixels = [p for im in samples[1:] for p in im.get_flattened_data()]
    colours = list(sheet.quantize(colors=126, method=Image.Quantize.MEDIANCUT).getpalette()[:378]) + [224,231,231,165,185,197]
    for selected in ([p for p in pixels if p[2] > p[1] + 18 and p[0] > p[1] + 10],
                     [p for p in pixels if p[0] > p[1] + 25 and p[0] > p[2] + 20]):
        if not selected:
            selected = pixels
        strip = Image.new('RGB', (256, (len(selected) + 255) // 256)); strip.putdata(selected)
        colours += strip.quantize(colors=64, method=Image.Quantize.MEDIANCUT).getpalette()[:192]
    palette = Image.new('P', (1,1)); palette.putpalette(colours)
    encoded = [im.quantize(palette=palette, dither=Image.Dither.NONE) for im in images]
    bounds = [round(i * 1000 / 24 / 10) * 10 for i in range(len(images) + 1)]
    encoded[0].save(path, save_all=True, append_images=encoded[1:], duration=[b-a for a,b in zip(bounds,bounds[1:])], loop=0, disposal=2, optimize=False)
    with Image.open(path) as im:
        elapsed = 0; unique = set()
        for i in range(im.n_frames):
            im.seek(i); unique.add(hashlib.sha256(im.convert('RGB').tobytes()).hexdigest())
            elapsed += im.info['duration']
        assert elapsed == bounds[-1]
        # A refused launch with no legal preview cell is intentionally static.
        assert len(unique) >= 1 if allow_static else len(unique) > 10
    errors = []
    for native, encoded_frame in zip(images[21].get_flattened_data(), encoded[21].convert('RGB').get_flattened_data()):
        if native[2] > native[1] + 25 and native[0] > native[1] + 10 or native[0] > native[1] + 35 and native[0] > native[2] + 25:
            errors.append(max(abs(a-b) for a,b in zip(native,encoded_frame)))
    mean = sum(errors)/len(errors) if errors else 0
    p95 = sorted(errors)[int((len(errors)-1)*.95)] if errors else 0
    assert mean < 18 and p95 < 45, (path.name, mean, p95)
    return {'file':path.name, 'native_size':list(images[0].size), 'duration_ms':elapsed, 'distinct_frames':len(unique),
        'mean_chromatic_error':round(mean,2), 'p95_chromatic_error':p95,
        'palette':'126 neutral +2 text +64 purple +64 red, common across frames'}


def package():
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert hashes() == baseline
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    assert len(entries) == 29
    checks = []; full_sequences = {}; animations = []
    for entry in entries:
        folder = OUT / 'native' / entry['folder']
        timeline = json.loads((folder / 'timeline.json').read_text(encoding='utf-8'))
        x,y,w,h = entry['crop']; bounds = (x,y,x+w,y+h)
        frames = []; originals = []; differences = {}
        dest = OUT / 'frames' / entry['variant']; dest.mkdir(parents=True,exist_ok=True)
        for frame in range(96):
            with Image.open(folder / f'{frame:03}.png') as im:
                native = im.convert('RGB'); originals.append(native)
                crop = native.crop(bounds); frames.append(crop)
                crop.save(dest / f'{frame:03}.png',compress_level=1)
        for frame in entry['control_frames']:
            with Image.open(folder / f'control-{frame:03}.png') as im:
                diff = ImageChops.difference(frames[frame], im.crop(bounds).convert('RGB'))
            differences[frame] = diff
            for cy,row in enumerate(entry['map'].splitlines()):
                for cx,symbol in enumerate(row):
                    if symbol == '#':
                        assert diff.crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None, (entry['variant'],frame,cx,cy)
        permitted = entry['permitted']
        assert timeline[17]['launched'] == permitted
        assert timeline[17]['hits'] == timeline[47]['hits'] == timeline[95]['hits']
        if not permitted:
            assert timeline[17]['player'] == entry['player_start']
            assert timeline[17]['cooldown'] == 0 and not timeline[17]['hits']
            assert all(d.getbbox() is None for d in differences.values())
            assert timeline[95]['turn'] == 0 and timeline[95]['form_remaining'] == 22
        else:
            assert any(differences[i].getbbox() for i in (18,19,20,21,23,25,27,30))
            assert differences[35].getbbox() is None, 'VFX linger beyond their short animation.'
            assert timeline[17]['cooldown'] == entry['cooldown']
            assert timeline[95]['turn'] == (0 if entry['quiet'] else 3)
            assert timeline[95]['form_remaining'] == (22 if entry['quiet'] else 19)
            assert timeline[95]['cooldown'] == (entry['cooldown'] if entry['quiet'] else max(0,entry['cooldown']-3))
            if entry['id'] == 'TR65-PAS':
                assert timeline[17]['player'] == entry['destination'] and not timeline[17]['hits']
            else:
                assert timeline[17]['player'] == entry['player_start']
            # Preview disappears as soon as launch is confirmed; only damage VFX
            # and the common target recoil differ from the unchanged controls.
            assert differences[47].getbbox() is None and differences[95].getbbox() is None
        if entry['case'] == 'HAUT':
            if entry['id'] == 'TR65-ALLONGE': assert timeline[17]['hits'] == [[4,3]]
            if entry['id'] == 'TR65-FENTE': assert timeline[17]['hits'] == [[4,4],[4,3],[4,1]]
        if entry['case'] == 'PARTIEL':
            assert timeline[17]['hits'] == [[4,4],[4,3]]
        checks.append({'variant':entry['variant'],'walls_clear':True,'preview_ends_on_launch':True,
            'timers_require_explicit_turns':True,'logical_positions_and_contacts_checked':True,
            'permitted':permitted,'reaction_and_vfx_end_before_frame35':True})
        if entry['case'] == 'HAUT':
            full_sequences[entry['id']] = originals
            for phase in (14,17,19,21,23,27,35):
                originals[phase].save(OUT / f"{entry['id']}-phase-{phase:03}.png")
        animations.append(save_animation(OUT / f"{entry['variant']}-64.gif",frames,allow_static=not permitted))
    # Sequential native-scale overview, no resized montage or32px intermediate.
    overview = []
    for sid in ['TR65-ALLONGE','TR65-FENTE','TR65-PAS']:
        overview.extend(full_sequences[sid])
    animations.append(save_animation(OUT / 'abysse-actions-64.gif', overview))
    base.dump(OUT / 'verification.json',{'native_views':29,'controls':checks,'animations':animations,
        'protected_files_unchanged':len(baseline),'resampling_of_native_capture':False})
    base.dump(OUT / 'review.json',{'batch':'ANIM-S18','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'id':sid,'name':title,'variant':sid+'-A01','visual_status':'awaiting_visual_review'} for n,sid,title in [(44,'TR65-ALLONGE','Allonge'),(45,'TR65-FENTE','Fente abyssale'),(46,'TR65-PAS','Pas d’ombre')]],
        'approved_body':'TR65-BODY02 / review44, unchanged',
        'visuals':'Extending purple/ivory forearm, dark jagged line laceration, rapid shadow disappearance/reappearance with residual former silhouette.',
        'rules_retained':'Previously shown2/4/3-cell ranges, cooldown0/4/3 and cost0/6/5. Silence blocks only shadow step. Prepared visibility, free destination adjacent to opaque obstacle; no new diagonal/metric rule.',
        'reaction':'Shared game hit_reaction_offset on actual prepared struck targets, .18s/17%tile. Blood on physical wounds, no new Bleeding status. Shadow step causes no damage.',
        'scope':'Visual fixture only: no real damage, HP, energy, AI, loot, equipment, attributes, save or campaign combat implementation.'})
    write_html(entries)
    for source,name in [('src/agent_skill_lab_review/fire_painted/abysse_actions.rs','render-source.rs'),
                        ('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),
                        ('src/combat_visuals.rs','reaction-source.rs'),
                        ('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((ROOT/source).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'native_views':29,'animations':len(animations),'protected_files':len(baseline),'controls':'passed'}))


def write_html(entries):
    manifest = [{k:e[k] for k in ('case','variant','id','permitted','quiet','cooldown','title','case_title','turn_frames')} for e in entries]
    script = '''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){const f=Math.floor(elapsed*24/1000);if(!launched||!e.permitted)return f%17;const start=turn===0?17:e.turn_frames[turn-1];const end=e.quiet?47:turn===3?95:turn===0?47:e.turn_frames[turn]-1;return Math.min(start+f,end);}
function ui(){const any=entries().some(e=>e.permitted);cast.disabled=!ready||launched||!any;next.disabled=!ready||!launched||!any||turn>=3||elapsed<650||entries().every(e=>e.quiet);document.getElementById('status').textContent=!launched?'Aperçu clignotant · avant lancement':any?'Tour '+turn+' · forme '+(22-turn)+' tours · '+entries().map(e=>e.title+': '+(e.permitted?Math.max(0,e.cooldown-turn)+' tours de recharge':'indisponible')).join(' · '):'Lancement refusé · aucun déplacement ni recharge';}
async function load(){ready=false;ui();const n=++token,es=entries();const data=await Promise.all(es.map(async e=>[e.id,await Promise.all(Array.from({length:96},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src='frames/'+e.variant+'/'+String(i).padStart(3,'0')+'.png';})))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===canvas.dataset.skill);canvas.parentElement.hidden=!e;if(e)canvas.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};
cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};
document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};
scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};
document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    options = ''.join(f'<option value="{key}">{name}</option>' for key,name in [
        ('HAUT','Vers le haut'),('DROITE','Vers la droite'),('GAUCHE','Vers la gauche'),('BAS','Vers le bas'),
        ('CLOISON','Cloison / cible cachée'),('OBSTACLE','Obstacle / destination déjà repérée'),
        ('ATTENTE','Attente sans tour'),('SILENCE','Silence'),('PARTIEL','Fente coupée après deux cases'),
        ('OCCUPEE','Destination occupée'),('CACHEE','Destination cachée'),('SANS-OMBRE','Destination sans ombre'),
        ('HORS-PORTEE','Destination hors portée')])
    cards = ''.join(f'<article><h2>{n} · {title}</h2><p class="muted">{sid}-A01 · TR65-BODY02</p><canvas data-skill="{sid}" width="576" height="576"></canvas></article>' for n,sid,title in [(44,'TR65-ALLONGE','Allonge'),(45,'TR65-FENTE','Fente abyssale'),(46,'TR65-PAS','Pas d’ombre')])
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Abysse, actions</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}[hidden]{display:none}@media(max-width:1296px){.grid{grid-template-columns:608px}}</style>
<h1>Abysse · trois actions</h1><p class="muted">ANIM-S18 · candidats 44–46 · corps BODY02 validé et conservé · cases natives 64 px</p>
<div class="controls"><select id="scenario">OPTIONS</select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="grid">CARDS</main>
<p>Allonge étire un membre jusqu’à une cible à deux cases. Fente abyssale lacère les ennemis d’une ligne de quatre cases sans déplacer l’agent ; le mur coupe la ligne. Les contacts préparés montrent le sang des blessures physiques et le petit aller-retour des victimes, sans ajouter de statut Saignement.</p>
<p>Pas d’ombre laisse une rémanence au départ et fait apparaître le corps à une destination libre, visible et voisine d’un obstacle opaque. Le laboratoire natif permet de choisir une case proposée. La scène « Obstacle / destination déjà repérée » fournit une visibilité préparée, sans ajouter de pouvoir de vision au travers des murs. Les autres scènes montrent les refus de destination et Silence, qui conserve les deux attaques naturelles.</p>
<p>Les transitions et impacts sont brefs ; l’attente seule ne diminue pas la durée de forme ou les recharges. Les aperçus disparaissent au lancement. Les trois nouvelles compositions attendent votre accord visuel.</p>
<p class="muted">Prototype isolé : aucun dégât chiffré, PV, énergie, IA, attribut, équipement, loot ou sauvegarde n’est calculé. Les destinations sont axiales dans ces cas préparés ; ils ne décident pas les diagonales ou une métrique générale de jeu.</p>
<p><a href="abysse-actions-64.gif">Les trois actions animées, à la suite, en taille native</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> · <a href="review.json">Fiche de revue</a></p>
<p class="muted">Dessins créés avec imagegen intégré : <a href="allonge-source.png">Membre extensible</a> / <a href="imagegen-allonge-prompt.txt">prompt</a> · <a href="fente-source.png">Lacération</a> / <a href="imagegen-fente-prompt.txt">prompt</a>. Corps et ombres réutilisés depuis la revue 44.</p><script src="gallery-script.js"></script></html>'''.replace('OPTIONS',options).replace('CARDS',cards)
    (OUT / 'index.html').write_text(html,encoding='utf-8')
    (OUT / 'gallery-script.js').write_text(script,encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--protect-only', action='store_true')
    parser.add_argument('--prepare', action='store_true')
    parser.add_argument('--convert-stream', action='store_true')
    parser.add_argument('--allonge', type=Path)
    parser.add_argument('--fente', type=Path)
    args = parser.parse_args()
    if args.protect_only:
        protect()
    elif args.prepare:
        assert args.allonge and args.fente
        prepare({'allonge': args.allonge, 'fente': args.fente})
    elif args.convert_stream:
        convert_stream()
    else:
        package()
