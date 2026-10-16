"""FR03-A02: varied joinable blocks and continuous rise, isolated review 24."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_frost as original
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-24'
BEFORE = original.OUT
SOURCE = Path(r'C:\Users\User\.codex\generated_images\01a11010-433f-7fc2-ac29-ffb696dec5ea\exec-0af80e19-57ff-40fe-932c-83989eb6c859.png')
original_protected = original.protected


def protected():
    result = original_protected()
    for p in sorted(BEFORE.rglob('*')):
        if p.is_file():
            result[p.relative_to(base.ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return result


def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    if (OUT / 'review-protection.json').exists():
        assert protected() == json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    else:
        base.dump(OUT / 'review-protection.json', protected())
    (OUT / 'wall-source.png').write_bytes(SOURCE.read_bytes())
    with Image.open(SOURCE) as image:
        assert image.mode == 'RGBA' and image.width == image.height and image.width % 3 == 0
        assert image.getchannel('A').getextrema()[0] == 0
        atlas = Image.new('RGBA', (9*256, 256))
        packing = []
        # The delivered sheet has three rows at these measured offsets. Use one
        # scale for all modules, then translate their ground base to a common anchor.
        # This is mechanical sprite registration; no repainting or per-module fitting.
        cell_size = image.width // 3
        for row, top in enumerate((0, round(image.height*415/1280), round(image.height*820/1280))):
            for col in range(3):
                left = col*cell_size
                cell = image.crop((left, top, left+cell_size, top+cell_size)).resize((256, 256), Image.Resampling.LANCZOS)
                alpha = cell.getchannel('A').point(lambda n: 255 if n > 96 else 0)
                bbox = alpha.getbbox()
                assert bbox is not None
                shift = 232-bbox[3]
                registered = Image.new('RGBA', (256, 256))
                registered.paste(cell, (0, shift))
                frame = row*3+col
                atlas.paste(registered, (frame*256, 0))
                packing.append({'frame': frame, 'source_crop': [left, top, cell_size, cell_size],
                    'uniform_scale': 256/cell_size, 'translation_y': shift, 'ground_anchor': [.5, .9]})
        atlas.save(OUT / 'wall-atlas.png')
    base.dump(OUT / 'drawing-pack.json', {'mode': 'Built-in imagegen',
        'source_sha256': hashlib.sha256(SOURCE.read_bytes()).hexdigest(), 'frames': packing,
        'postprocessing': 'Equal-scale cropping and ground registration, preserved alpha. No Python painting.',
        'variants': ['A: high left', 'B: broken centre ridge', 'C: high right'],
        'states': ['intact', 'cracked', 'shattered']})
    review = json.loads((BEFORE / 'review.json').read_text(encoding='utf-8'))
    review.update({'batch': 'ANIM-S07-A02', 'status': 'awaiting_wall_visual_review',
        'revision_scope': 'Only Mur de glace. Nova and attached Gel retain their previous images, positions and timing.',
        'wall': 'Three joined irregular block models, 76 px drawing width with a shared ground base. Differently shaped ridges and cracks. Continuous 0.22 second rise, independent 0.30 second collapse per block. Same cells, wall lifetime, Gel and prepared combat fixture.',
        'user_feedback': "au pire sinon, vu que y'aura surement plus que 3 blocs :) tu n'as qu'à faire des blocs qui ne sont pas identiques :)",
        'previous': '../skill-animation-review-23/index.html'})
    for c in review['candidates']:
        if c['skill_id'] == 'FR03':
            c['variant'] = 'FR03-A02'
            c['status'] = 'awaiting_visual_review'
        else:
            c['status'] = 'unchanged_from_previous_review_not_newly_approved'
    base.dump(OUT / 'review.json', review)
    launch = (BEFORE / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    launch = launch.replace("'--lot-froid-ouverture'", "'--lot-froid-ouverture' '--mur-glace-v2'")
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launch, encoding='utf-8-sig')
    print(json.dumps({'prepared': str(OUT), 'atlas': [2304, 256], 'registered_blocks': 9}))


def write_html(entries):
    manifest = [{k: e[k] for k in ('variant', 'folder', 'frames')} for e in entries if e['id'] == 'FR03']
    script = '''const manifest=MANIFEST,sequences=new Map();let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,loadToken=0;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),stages=[17,72,89,106,123,140],ends=[71,88,105,122,139,167];
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return f%17;if(turn===0)return f<55?17+f:48+(f-55)%24;return Math.min(stages[turn]+f,ends[turn]);}
function ui(){const moving=launched&&elapsed<(turn===0?500:300);document.getElementById('status').textContent=launched?`Tour ${turn} · ${turn===0?(moving?'Apparition du mur':'Mur maintenu'):turn===1?'Premier contact : fissure et Gel':turn===2?'Un bloc se brise':turn===3?'Le monstre traverse la brèche':turn===4?'Mur : 1 tour restant':'Expiration du mur'}`:'Prévisualisation · avant lancement';cast.disabled=launched||!ready;next.disabled=!ready||!launched||turn>=5||moving;}
async function load(){ready=false;const token=++loadToken,selected=scenario.value;document.getElementById('loading').textContent='Chargement…';const e=manifest.find(e=>e.folder===selected);const loaded=await Promise.all(['before','after'].map(async key=>{const prefix=key==='before'?`../skill-animation-review-23/frames/FR03-A01-${selected}`:`frames/${e.variant}`;const images=await Promise.all(Array.from({length:e.frames},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`${prefix}/${String(i).padStart(3,'0')}.png`;})));return[key,images];}));if(token!==loadToken)return;sequences.clear();for(const[key,images]of loaded)sequences.set(key,images);elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · cases natives 64 px';ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const c of document.querySelectorAll('canvas'))c.getContext('2d').drawImage(sequences.get(c.dataset.version)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;ui();load().catch(()=>document.getElementById('loading').textContent='Chargement impossible.');};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};ui();load().catch(()=>document.getElementById('loading').textContent='Conserver les dossiers des revues 23 et 24 ensemble.');requestAnimationFrame(tick);'''.replace('MANIFEST', json.dumps(manifest))
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Mur de glace varié</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:19px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}#status{color:#c4dcdc;font-weight:600}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>16 · Mur de glace — blocs variés</h1><p class="muted">FR03-A02 · proposition visuelle en 64 px</p>
<p>Trois formes de blocs forment une barrière plus continue : sommets irréguliers, faces épaisses et base commune. La glace monte rapidement depuis le sol. Chaque bloc possède sa fissure et peut s’effondrer séparément.</p>
<div class="controls"><select id="scenario"><option value="HAUT">Sept blocs · haut</option><option value="DROITE">Sept blocs · droite</option><option value="GAUCHE">Sept blocs · gauche</option><option value="BAS">Sept blocs · bas</option><option value="CLOISON">Quatre blocs devant la cloison</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button><span id="loading">Chargement…</span></div><p id="status"></p>
<main class="pair"><article><h2>Avant · FR03-A01</h2><canvas data-version="before" width="576" height="576"></canvas></article><article><h2>Blocs variés · FR03-A02</h2><canvas data-version="after" width="576" height="576"></canvas></article></main>
<p>« +1 tour » montre le contact du monstre, une fissure, la destruction d’un bloc puis le passage par la brèche. Sans passage de tour, le mur reste en place. La prévisualisation clignotante disparaît au lancement.</p>
<p class="muted">Règles, Gel, Nova et acteurs conservés. Les coups et réussites de Gel sont préparés pour cette revue. Aucun dégât ou comportement de combat de la campagne n’est modifié. Le nouveau dessin et sa montée attendent leur validation.</p>
<p><a href="mur-varie-comparaison-64.gif">Comparaison animée · sept blocs</a> · <a href="mur-varie-cloison-64.gif">Comparaison · cloison</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p>
<p class="muted">Création avec imagegen intégré : <a href="wall-source.png">planche source</a> · <a href="imagegen-wall-prompt.txt">prompt</a>. Assemblage et alignement mécanique des dessins ; aucun personnage redessiné.</p><script>SCRIPT</script></html>'''.replace('SCRIPT', script)
    (OUT / 'index.html').write_text(html, encoding='utf-8')
    (OUT / 'gallery-script.js').write_text(script, encoding='utf-8')


def package():
    original.OUT = OUT
    original.protected = protected
    original.write_html = write_html
    original.package()
    finish_package()


def finish_package():
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    preservation = []
    for e in entries:
        if e['id'] == 'FR02':
            label_differences = []
            for i in range(168):
                a = Image.open(OUT / 'frames' / e['variant'] / f'{i:03}.png').convert('RGB')
                b = Image.open(BEFORE / 'frames' / e['variant'] / f'{i:03}.png').convert('RGB')
                diff = ImageChops.difference(a, b)
                bbox = diff.getbbox()
                if bbox:
                    # GL may rasterize a few pixels of the faded floating label
                    # differently across captures. No actor/terrain/Nova pixel may vary.
                    age = (i-17)/24
                    label = (242, 288-43-age*7-18, 353, 288-43-age*7+3)
                    assert 0 <= age < .7 and all((bbox[n] >= label[n] if n < 2 else bbox[n] <= label[n]) for n in range(4)), f'Nova/body/terrain changed: {e["variant"]}, {i}, {bbox}'
                    label_differences.append({'frame': i, 'bbox': list(bbox)})
            preservation.append({'variant': e['variant'], 'all_168_native_frames_identical_outside_faded_name': True,
                'faded_name_rasterization_differences': label_differences})
        else:
            a = json.loads((OUT / 'native' / e['folder'] / 'timeline.json').read_text())
            b = json.loads((BEFORE / 'native' / e['folder'] / 'timeline.json').read_text())
            assert a == b, 'Wall cells or prepared turn rules changed.'
    animations = []
    for folder, stem in [('HAUT', 'mur-varie-comparaison'), ('CLOISON', 'mur-varie-cloison')]:
        boards = []
        for i in range(168):
            board = Image.new('RGB', (1232, 650), '#0b1218')
            draw = ImageDraw.Draw(board)
            for col, (root, variant, title) in enumerate([(BEFORE, f'FR03-A01-{folder}', 'Avant · blocs identiques'), (OUT, f'FR03-A02-{folder}', 'Blocs variés · base commune')]):
                x = 12+col*616
                draw.text((x, 10), title, font=base.font(20, True), fill='#e0e7e7')
                draw.text((x, 40), variant, font=base.font(15), fill='#a4b9c6')
                with Image.open(root / 'frames' / variant / f'{i:03}.png') as image:
                    board.paste(image.convert('RGB'), (x, 66))
            boards.append(board)
        animations.append(save_animation(OUT / f'{stem}-64.gif', boards, 24))
        for phase, name in [(23, 'mur'), (76, 'fissure'), (93, 'effondrement'), (108, 'breche')]:
            boards[phase].save(OUT / f'{stem}-{name}.png')
    base.dump(OUT / 'revision-verification.json', {'nova_preservation': preservation,
        'wall_timelines_identical': True, 'comparison_animations': animations,
        'gel_uses_exact_original_atlas': True, 'drawing_only_revision': True})
    (OUT / 'base-package-source.py').write_bytes(Path(original.__file__).read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'revision_controls': 'passed', 'comparison_gifs': 2}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--prepare', action='store_true')
    parser.add_argument('--finish', action='store_true')
    args = parser.parse_args()
    prepare() if args.prepare else finish_package() if args.finish else package()
