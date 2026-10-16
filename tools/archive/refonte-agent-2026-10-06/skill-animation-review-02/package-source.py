"""Second animated review: stronger VFX, 64 px only, isolated from accepted assets."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw

import prepare_agent_skill_animation_review as base

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-02'
NATIVE = OUT / 'native-final'
CANDIDATES = [
    ('ME02', 'Fauchage', 'Un large croissant de lame, une traînée lumineuse et des impacts francs sur les trois cibles.'),
    ('FE01', 'Braise', 'Une braise incandescente, une longue traînée de feu, un impact flamboyant et des flammes sur la cible.'),
    ('FR01', 'Aiguille de givre', 'Une aiguille lumineuse, un sillage de givre, une gerbe d’éclats et des cristaux autour des pieds.'),
]


def prepare() -> None:
    base.OUT = OUT
    base.CANDIDATES = CANDIDATES
    base.prepare()
    review = json.loads((OUT / 'review.json').read_text(encoding='utf-8'))
    review['revision'] = 2
    review['user_instruction'] = 'il va falloir des animations globalement beaucoup plus marquées que ca (et penses les pour des cases en 64 px, à terme le 32 px va disparaitre) . il faut que ca en mette pleins les yeux :)'
    review['native_tile_size'] = 64
    review['previous_candidates'] = 'ME02-A01, FE01-A01, FR01-A01: too discreet; retained for comparison, not approved.'
    for item in review['candidates']:
        item['variant'] = item['skill_id'] + '-A02'
    base.dump(OUT / 'review.json', review)
    previous = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-01/review.json'
    first = json.loads(previous.read_text(encoding='utf-8'))
    first['status'] = 'revision_requested'
    for item in first['candidates']:
        item['status'] = 'revision_requested'
    first['feedback'] = review['user_instruction']
    first['next_review'] = 'skill-animation-review-02'
    base.dump(previous, first)


def save_animation(path: Path, images: list[Image.Image], fps: int, slow: bool = False) -> dict:
    # GIF delays are multiples of 10 ms. Distribute rounding over the whole sequence.
    factor = 2 if slow else 1
    boundaries = [round(i * 1000 / fps / 10) * 10 * factor for i in range(len(images) + 1)]
    delays = [b - a for a, b in zip(boundaries, boundaries[1:])]
    palette = base.palette_for(images)
    quantized = [im.quantize(palette=palette, dither=Image.Dither.NONE) for im in images]
    quantized[0].save(path, save_all=True, append_images=quantized[1:], duration=delays,
                      loop=0, disposal=2, optimize=False)
    with Image.open(path) as check:
        unique = set()
        duration = 0
        for i in range(check.n_frames):
            check.seek(i)
            unique.add(hashlib.sha256(check.convert('RGB').tobytes()).hexdigest())
            duration += check.info['duration']
        assert len(unique) > 12 and duration == boundaries[-1]
        return {'file': path.name, 'native_size': list(check.size), 'distinct_frames': len(unique),
                'frames': check.n_frames, 'duration_ms': duration, 'loop': check.info['loop']}


def package() -> None:
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert base.protected() == baseline, 'Accepted icons, catalogue or actor art changed'
    entries = json.loads((NATIVE / 'capture.json').read_text(encoding='utf-8'))
    assert len(entries) == 3 and all(e['tile'] == 64 for e in entries)
    frames = {}
    verification = []
    for entry in entries:
        images = []
        target = OUT / 'frames' / entry['folder']
        target.mkdir(parents=True, exist_ok=True)
        x, y, w, h = entry['crop']
        for index in range(entry['frames']):
            filename = f'{index:03}.png'
            with Image.open(NATIVE / entry['folder'] / filename) as im:
                assert im.size == (1280, 800)
                image = im.crop((x, y, x + w, y + h)).convert('RGB')
                image.save(target / filename)
                images.append(image)
        frames[entry['id']] = images
        verification.append(save_animation(OUT / f"{entry['id']}-A02-64.gif", images, entry['fps']))
        verification.append(save_animation(OUT / f"{entry['id']}-A02-64-slow.gif", images, entry['fps'], True))
        images[10].save(OUT / f"{entry['id']}-A02-phase.png")
    boards = []
    for index in range(entries[0]['frames']):
        board = Image.new('RGB', (1260, 534), '#0b1218')
        draw = ImageDraw.Draw(board)
        draw.text((20, 12), 'ANIM-S01 · Animations renforcées · Révision 02', font=base.font(24, True), fill='#e0e7e7')
        draw.text((20, 47), 'Propositions à valider · conçues pour des cases natives de 64 px', font=base.font(15), fill='#a0b0b8')
        for i, (sid, name, _) in enumerate(CANDIDATES):
            left = 18 + i * 416
            draw.text((left, 87), f'0{i+1} · {name}', font=base.font(20, True), fill='#e0e7e7')
            draw.text((left, 116), sid + '-A02 · 64 px natifs', font=base.font(13), fill='#9ab0bc')
            board.paste(frames[sid][index], (left, 154))
        draw.text((20, 495), 'Intensité visuelle augmentée ; zones de dégâts et règles de combat conservées.', font=base.font(15), fill='#a0b0b8')
        boards.append(board)
    verification.append(save_animation(OUT / 'animations-lot-01-revision-02.gif', boards, entries[0]['fps']))
    boards[10].save(OUT / 'animations-lot-01-revision-02-phase.png')
    base.dump(OUT / 'verification.json', {'accepted_assets_unchanged': len(baseline), 'native_tile_size': 64,
               'animations': verification, 'resampling': False, 'source': 'Macroquad native framebuffer',
               'native_capture': 'native-final/capture.json', 'status': 'awaiting_visual_review'})
    write_html(entries)
    print(json.dumps({'output': str(OUT), 'protected_files': len(baseline), 'animated_files': len(verification)}, ensure_ascii=False))


def write_html(entries: list[dict]) -> None:
    cards = ''.join(f'''<article><h2>0{i+1} · {name}</h2><p class="id">{sid}-A02 · à valider</p><p>{description}</p>
<canvas data-skill="{sid}" width="384" height="320" aria-label="Animation de {name}"></canvas></article>'''
        for i, (sid, name, description) in enumerate(CANDIDATES))
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Project RL · Animations renforcées · 64 px</title><style>
*{box-sizing:border-box}body{margin:0;background:#0b1218;color:#dce4e5;font:16px Segoe UI,sans-serif;padding:24px;max-width:1380px;margin:auto}h1{font-size:26px;margin:0 0 10px}h2{font-size:20px;margin:0 0 8px}p{line-height:1.5;margin:8px 0}.muted,.id{color:#96abb7}.id{font-size:13px}.controls{display:flex;gap:10px;flex-wrap:wrap;margin:22px 0}
button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:9px 13px;font:inherit;cursor:pointer}button:hover{border-color:#96b7c9}.grid{display:grid;grid-template-columns:repeat(3,416px);gap:16px}article{padding:16px;background:#101b23;border:1px solid #273e4c;border-radius:8px}article>p:not(.id){min-height:75px}canvas{display:block;background:#091014;width:384px;height:320px}.lab{margin-top:28px;border-top:1px solid #405664;padding-top:20px}.lab img{width:100%;max-width:1280px;height:auto}a{color:#93c1e0}@media(max-width:1340px){.grid{grid-template-columns:repeat(2,416px)}}@media(max-width:920px){.grid{grid-template-columns:416px}}
</style><header><h1>Project RL · Animations renforcées</h1><p class="muted">ANIM-S01 · révision 02 · trois candidats en 64 px natifs</p><p>Traînées, impacts et statuts plus marqués, avec les règles de combat déjà définies.</p><p class="muted">Le Gel est déclenché pour examiner son apparence. Le laboratoire ne calcule pas encore les dégâts réels.</p></header>
<div class="controls"><select id="speed" aria-label="Vitesse"><option value="1">Vitesse ×1</option><option value="0.5">Vitesse ×0,5</option></select><button id="pause">Pause</button><button id="replay">Rejouer</button><span class="muted" id="loading">Chargement…</span></div>
<main class="grid">CARDS</main><section class="lab"><h2>LAB-S01 · Prototype du laboratoire</h2><p>Interface proposée séparément. <a href="Ouvrir_laboratoire.ps1">Ouvrir le prototype natif</a> depuis le dossier de cette revue.</p><img src="native-final/laboratoire-prototype.png" alt="Capture du prototype du laboratoire"></section>
<script>
const manifest=MANIFEST;const sequences=new Map();let elapsed=0,last=0,paused=false,ready=false;
const speed=document.getElementById('speed'),pause=document.getElementById('pause');
async function load(){for(const entry of manifest){const images=await Promise.all(Array.from({length:entry.frames},(_,index)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${entry.folder}/${String(index).padStart(3,'0')}.png`;})));sequences.set(entry.id,{entry,images});}ready=true;document.getElementById('loading').textContent='Prêt · scènes natives 64 px';}
function tick(now){if(last&&!paused)elapsed+=(now-last)*Number(speed.value);last=now;if(ready)for(const canvas of document.querySelectorAll('canvas')){const{entry,images}=sequences.get(canvas.dataset.skill);const index=Math.floor(elapsed*entry.fps/1000)%entry.frames;const ctx=canvas.getContext('2d');ctx.imageSmoothingEnabled=false;ctx.drawImage(images[index],0,0);}requestAnimationFrame(tick);}
pause.onclick=()=>{paused=!paused;pause.textContent=paused?'Lecture':'Pause';};document.getElementById('replay').onclick=()=>{elapsed=0;};load().catch(()=>document.getElementById('loading').textContent='Garder les sous-dossiers à côté de cette galerie.');requestAnimationFrame(tick);
</script></html>'''.replace('CARDS', cards).replace('MANIFEST', json.dumps(entries))
    (OUT / 'index.html').write_text(html, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--prepare', action='store_true')
    args = parser.parse_args()
    prepare() if args.prepare else package()
