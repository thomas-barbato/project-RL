"""ANIM-S02: crop native Macroquad captures without resampling accepted art."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw

import prepare_agent_skill_animation_review as base
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-03'
CANDIDATES = [
    (4, 'B01', 'B01-A01', 'Frappe de rupture', 'Une frappe ample, un impact puissant et une projection de sang sur la cible.'),
    (5, 'ME03', 'ME03-A01', 'Percée', 'Une frappe étroite, des éclats de plaques et une blessure sanglante.'),
    (6, 'ME04', 'ME04-A01-MUR', 'Repousser · mur', 'La cible recule jusqu’à la cloison ; la collision projette des fragments et du sang provenant de la cible.'),
    (7, 'ME04', 'ME04-A01-CIBLE', 'Repousser · monstre', 'La cible heurte un autre monstre ; un second impact blesse les deux créatures.'),
]


def protected() -> dict[str, str]:
    hashes = base.protected()
    previous = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-02'
    for name in ['ME02-A02-64.gif', 'FE01-A02-64.gif', 'FR01-A02-64.gif', 'visual-acceptance-01.json', 'blood-policy-acceptance-01.json']:
        path = previous / name
        hashes[path.relative_to(base.ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes


def prepare() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    receipt = OUT / 'review-protection.json'
    if receipt.exists():
        raise RuntimeError('Preserve the existing protection baseline.')
    base.dump(receipt, protected())
    base.dump(OUT / 'review.json', {
        'batch': 'ANIM-S02', 'status': 'awaiting_visual_review', 'native_tile_size': 64,
        'candidates': [{'number': number, 'skill_id': sid, 'variant': variant, 'name': name,
                        'description': description, 'status': 'awaiting_visual_review'}
                       for number, sid, variant, name, description in CANDIDATES],
        'scope': 'Visual fixtures only; no GameState, combat calculation or campaign integration.',
        'laboratory_ui_candidate': {'variant': 'LAB-S02', 'status': 'awaiting_separate_review'},
        'blood_policy': 'ANIM-BLOOD01: flesh wounds bleed; metal wall fragments do not bleed.',
        'push_preview': 'At effective rank 11, example obstacles stop the three-cell maximum push after two free cells.',
        'timing': 'Frame time is independent of turns, cooldowns and combat resolution.',
        'unchanged': 'Previously approved icons, actor art, catalogue and A02 animations.',
    })
    launch = '''$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path (Join-Path $PSScriptRoot '../../..')).Path
$reviewBinary = Join-Path $projectRoot 'target/debug/project-rl.exe'
if (-not (Test-Path -LiteralPath $reviewBinary)) { throw 'Compiler le projet en mode debug avant cet aperçu.' }
Push-Location -LiteralPath $projectRoot
try { & $reviewBinary '--apercu-laboratoire-competences' '--lot-melee' }
finally { Pop-Location }
'''
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launch, encoding='utf-8-sig')


def package() -> None:
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert protected() == baseline, 'Accepted artwork, catalogue or animations changed.'
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    assert [(e['number'], e['id'], e['variant']) for e in entries] == [(n, sid, variant) for n, sid, variant, _, _ in CANDIDATES]
    sequences = {}
    verification = []
    for entry in entries:
        assert entry['tile'] == 64 and entry['frames'] == 90 and entry['fps'] == 30
        images = []
        x, y, width, height = entry['crop']
        target = OUT / 'frames' / entry['folder']
        target.mkdir(parents=True, exist_ok=True)
        for index in range(entry['frames']):
            name = f'{index:03}.png'
            with Image.open(OUT / 'native' / entry['folder'] / name) as im:
                assert im.size == (1280, 800)
                image = im.crop((x, y, x + width, y + height)).convert('RGB')
                image.save(target / name)
                images.append(image)
        sequences[entry['variant']] = images
        verification.append(save_animation(OUT / f"{entry['variant']}-64.gif", images, entry['fps']))
        verification.append(save_animation(OUT / f"{entry['variant']}-64-slow.gif", images, entry['fps'], True))
    boards = []
    for frame in range(90):
        board = Image.new('RGB', (988, 866), '#0b1218')
        draw = ImageDraw.Draw(board)
        draw.text((20, 12), 'ANIM-S02 · Mêlée et collisions', font=base.font(24, True), fill='#e0e7e7')
        draw.text((20, 47), '04–07 · À valider · cases natives de 64 px', font=base.font(15), fill='#a0b0b8')
        for i, (number, _, variant, title, _) in enumerate(CANDIDATES):
            left = 20 + (i % 2) * 490
            top = 84 + (i // 2) * 383
            draw.text((left, top), f'{number:02} · {title}', font=base.font(19, True), fill='#e0e7e7')
            draw.text((left, top + 27), variant, font=base.font(13), fill='#9ab0bc')
            board.paste(sequences[variant][frame], (left, top + 53))
        boards.append(board)
    verification.append(save_animation(OUT / 'animations-lot-02.gif', boards, 30))
    boards[26].save(OUT / 'animations-lot-02-phase.png')
    # A native contact sheet makes impact, movement and aftermath easy to inspect.
    sheet = Image.new('RGB', (1860, 1430), '#0b1218')
    draw = ImageDraw.Draw(sheet)
    for row, (number, _, variant, title, _) in enumerate(CANDIDATES):
        top = 12 + row * 354
        draw.text((12, top), f'{number:02} · {title}', font=base.font(18, True), fill='#e0e7e7')
        for column, frame in enumerate([5, 10, 24, 36]):
            sheet.paste(sequences[variant][frame], (12 + column * 460, top + 28))
    sheet.save(OUT / 'inspection-phases.png')
    base.dump(OUT / 'verification.json', {
        'status': 'awaiting_visual_review', 'accepted_files_unchanged': len(baseline),
        'source': 'Macroquad native framebuffer', 'native_capture': 'native/capture.json',
        'resampling': False, 'animations': verification,
    })
    (OUT / 'render-source.rs').write_bytes((base.ROOT / 'src/agent_skill_lab_review/melee.rs').read_bytes())
    (OUT / 'render-parent-source.rs').write_bytes((base.ROOT / 'src/agent_skill_lab_review.rs').read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries)
    print(json.dumps({'output': str(OUT), 'accepted_files_unchanged': len(baseline), 'animated_files': len(verification)}))


def write_html(entries: list[dict], *, batch: str = 'ANIM-S02', title: str = 'Animations · Lot 02',
               introduction: str = 'Trois compétences de mêlée, avec les deux collisions de Repousser. Les créatures sont des cibles de chair ; les fragments de cloison restent métalliques.',
               footer: str = 'Rendu visuel isolé : les chiffres de dégâts et les effets réels ne sont pas encore raccordés au combat. Les marques de sang montrent une intention visuelle, sans ajouter le statut Saignement.',
               canvas_height: int = 320) -> None:
    cards = ''.join(f'''<article><h2>{number:02} · {name}</h2><p class="id">{variant} · à valider</p>
<p>{description}</p><canvas data-variant="{variant}" width="448" height="{canvas_height}" aria-label="{name}"></canvas></article>'''
        for number, _, variant, name, description in CANDIDATES)
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Project RL · REVIEW_TITLE</title><style>
*{box-sizing:border-box}body{margin:0 auto;background:#0b1218;color:#dce4e5;font:16px Segoe UI,sans-serif;padding:24px;max-width:1060px}h1{font-size:26px}h2{font-size:19px;margin:0 0 8px}p{line-height:1.5}.muted,.id{color:#96abb7}.id{font-size:13px}.controls{display:flex;gap:10px;flex-wrap:wrap;margin:22px 0}
button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:9px 13px;font:inherit;cursor:pointer}.grid{display:grid;grid-template-columns:repeat(2,480px);gap:16px}article{padding:16px;background:#101b23;border:1px solid #273e4c;border-radius:8px}article>p:not(.id){min-height:75px}canvas{display:block;width:448px;height:320px;background:#091014}a{color:#93c1e0}@media(max-width:1040px){.grid{grid-template-columns:480px}}
</style><header><h1>Project RL · REVIEW_TITLE</h1><p class="muted">REVIEW_BATCH · REVIEW_COUNT aperçus en 64 px natifs · à valider</p>
<p>REVIEW_INTRODUCTION</p></header>
<div class="controls"><select id="speed" aria-label="Vitesse"><option value="1">Vitesse ×1</option><option value="0.5">Vitesse ×0,5</option></select><button id="pause">Pause</button><button id="replay">Rejouer</button><span class="muted" id="loading">Chargement…</span></div>
<main class="grid">CARDS</main><p class="muted">REVIEW_FOOTER</p>
<p><a href="Ouvrir_laboratoire.ps1">Ouvrir ce lot dans le laboratoire natif</a> depuis le dossier de la revue.</p>
<script>
const manifest=MANIFEST;const sequences=new Map();let elapsed=0,last=0,paused=false,ready=false;
const speed=document.getElementById('speed'),pause=document.getElementById('pause');
async function load(){for(const entry of manifest){const images=await Promise.all(Array.from({length:entry.frames},(_,index)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${entry.folder}/${String(index).padStart(3,'0')}.png`;})));sequences.set(entry.variant,{entry,images});}ready=true;document.getElementById('loading').textContent='Prêt · scènes natives 64 px';}
function tick(now){if(last&&!paused)elapsed+=(now-last)*Number(speed.value);last=now;if(ready)for(const canvas of document.querySelectorAll('canvas')){const{entry,images}=sequences.get(canvas.dataset.variant);const index=Math.floor(elapsed*entry.fps/1000)%entry.frames;canvas.getContext('2d').drawImage(images[index],0,0);}requestAnimationFrame(tick);}
pause.onclick=()=>{paused=!paused;pause.textContent=paused?'Lecture':'Pause';};document.getElementById('replay').onclick=()=>{elapsed=0;};load().catch(()=>document.getElementById('loading').textContent='Garder les sous-dossiers à côté de cette galerie.');requestAnimationFrame(tick);
</script></html>'''.replace('CARDS', cards).replace('MANIFEST', json.dumps(entries)).replace('REVIEW_TITLE', title).replace('REVIEW_BATCH', batch).replace('REVIEW_COUNT', str(len(CANDIDATES))).replace('REVIEW_INTRODUCTION', introduction).replace('REVIEW_FOOTER', footer)
    html = html.replace('height:320px', f'height:{canvas_height}px')
    (OUT / 'index.html').write_text(html, encoding='utf-8')
    (OUT / 'gallery-script.js').write_text(html.split('<script>')[1].split('</script>')[0], encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--prepare', action='store_true')
    args = parser.parse_args()
    prepare() if args.prepare else package()
