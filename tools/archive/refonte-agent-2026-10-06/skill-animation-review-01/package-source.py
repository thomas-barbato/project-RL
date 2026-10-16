"""Package actual Macroquad frame captures; never modify accepted actor artwork."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-01'
CANDIDATES = [
    ('ME02', 'Fauchage', 'Un arc de lame court traverse les trois cases devant l’agent.'),
    ('FE01', 'Braise', 'Une braise file vers la cible ; un point incandescent reste sur elle.'),
    ('FR01', 'Aiguille de givre', 'Une aiguille de glace frappe la cible ; le givre immobilise ses pieds.'),
]


def dump(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def protected() -> dict[str, str]:
    paths = []
    for folder in ['assets/skills/agent', 'assets/sprites/flat-agents', 'assets/sprites/bestiary']:
        paths.extend(p for p in (ROOT / folder).rglob('*') if p.is_file())
    return {p.relative_to(ROOT).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths)}


def prepare() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    receipt = OUT / 'review-protection.json'
    if receipt.exists():
        raise RuntimeError('A baseline already exists; preserve it.')
    dump(receipt, protected())
    dump(OUT / 'review.json', {
        'status': 'awaiting_visual_review',
        'batch': 'ANIM-S01',
        'workflow': 'Animated numbered batches, explicit approval after each preview.',
        'user_instruction': 'oui on va faire comme les images de compétences, sauf que là ca sera pour les animations (et que donc ca sera animé)',
        'batch_choice': 'Valider par lots après chaque aperçu',
        'candidates': [{'number': i + 1, 'skill_id': sid, 'variant': sid + '-A01', 'name': name,
                        'status': 'awaiting_visual_review', 'description': description}
                       for i, (sid, name, description) in enumerate(CANDIDATES)],
        'laboratory_ui_candidate': {'variant': 'LAB-S01', 'status': 'awaiting_separate_review'},
        'scope': 'Debug-only native visual prototype; combat damage and real character progression not connected.',
        'timing': 'Visual real time is separate from turns and status/cooldown timers.',
        'freeze_preview': 'Gel is deliberately shown; this preview does not change the accepted chance of applying it.',
        'unchanged': 'Accepted skill icons, actor art, catalogue parameters, campaign and saves.',
    })
    launch = '''$ErrorActionPreference = 'Stop'
$reviewRoot = $PSScriptRoot
$projectRoot = (Resolve-Path (Join-Path $reviewRoot '../../..')).Path
$reviewBinary = Join-Path $projectRoot 'target/debug/project-rl.exe'
if (-not (Test-Path -LiteralPath $reviewBinary)) { throw 'Compiler le projet en mode debug avant de lancer cet aperçu.' }
Push-Location -LiteralPath $projectRoot
try { & $reviewBinary '--apercu-laboratoire-competences' }
finally { Pop-Location }
'''
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launch, encoding='utf-8-sig')


def font(size: int, bold: bool = False) -> ImageFont.FreeTypeFont:
    return ImageFont.truetype('C:/Windows/Fonts/' + ('segoeuib.ttf' if bold else 'segoeui.ttf'), size)


def palette_for(images: list[Image.Image]) -> Image.Image:
    # A common palette keeps static actor sprites from changing colour between frames.
    sheet = Image.new('RGB', (images[0].width * len(images), images[0].height))
    for i, frame in enumerate(images):
        sheet.paste(frame, (i * frame.width, 0))
    return sheet.quantize(colors=256, method=Image.Quantize.MEDIANCUT)


def gif(path: Path, images: list[Image.Image], milliseconds: int) -> dict:
    palette = palette_for(images)
    quantized = [im.quantize(palette=palette, dither=Image.Dither.NONE) for im in images]
    quantized[0].save(path, save_all=True, append_images=quantized[1:], duration=milliseconds,
                      loop=0, disposal=2, optimize=False)
    with Image.open(path) as check:
        hashes = set()
        duration = 0
        for i in range(check.n_frames):
            check.seek(i)
            duration += check.info.get('duration', 0)
            hashes.add(hashlib.sha256(check.convert('RGB').tobytes()).hexdigest())
        if len(hashes) < 5 or duration != len(images) * milliseconds:
            raise AssertionError(f'Incomplete or static animation: {path.name}')
        return {'file': path.name, 'size': list(check.size), 'frames': check.n_frames,
                'distinct_frames': len(hashes), 'duration_ms': duration, 'loop': check.info.get('loop')}


def package() -> None:
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    if protected() != baseline:
        raise AssertionError('Accepted artwork or catalogue changed while preparing this prototype.')
    capture = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    sequences = {}
    verification = []
    for entry in capture:
        x, y, width, height = entry['crop']
        files = sorted((OUT / 'native' / entry['folder']).glob('*.png'))
        if len(files) != entry['frames']:
            raise AssertionError('Unexpected frame count')
        images = []
        cropped_directory = OUT / 'frames' / entry['folder']
        cropped_directory.mkdir(parents=True, exist_ok=True)
        for file in files:
            with Image.open(file) as image:
                if image.size != (1280, 800):
                    raise AssertionError('Unexpected framebuffer dimensions')
                # Extract the native stage without scaling or redrawing the artwork.
                images.append(image.crop((x, y, x + width, y + height)).convert('RGB'))
                images[-1].save(cropped_directory / file.name)
        sequences[(entry['id'], entry['tile'])] = images
        stem = f"{entry['id']}-A01-{entry['tile']}"
        verification.append(gif(OUT / (stem + '.gif'), images, 50))
        verification.append(gif(OUT / (stem + '-slow.gif'), images, 100))
        images[5].save(OUT / (stem + '-phase.png'))
    boards = []
    for frame in range(32):
        image = Image.new('RGB', (1260, 696), '#0b1218')
        draw = ImageDraw.Draw(image)
        draw.text((20, 12), 'ANIM-S01 · Premières animations de compétences', font=font(24, True), fill='#e0e7e7')
        draw.text((20, 45), 'Propositions à valider · scènes natives 64 et 32 px · animation du jeu capturée', font=font(15), fill='#a0b0b8')
        for i, (sid, name, _) in enumerate(CANDIDATES):
            left = 18 + i * 416
            draw.text((left, 85), f'0{i + 1} · {name}', font=font(20, True), fill='#e0e7e7')
            draw.text((left, 114), sid + '-A01', font=font(13), fill='#8ea1af')
            draw.text((left, 143), 'Cases natives 64 px', font=font(14), fill='#a0b0b8')
            image.paste(sequences[(sid, 64)][frame], (left, 169))
            draw.text((left, 510), 'Cases natives 32 px', font=font(14), fill='#a0b0b8')
            image.paste(sequences[(sid, 32)][frame], (left, 536))
        boards.append(image)
    verification.append(gif(OUT / 'animations-lot-01.gif', boards, 50))
    boards[5].save(OUT / 'animations-lot-01-phase.png')
    dump(OUT / 'verification.json', {'accepted_assets_unchanged': len(baseline), 'animations': verification,
                                    'native_capture': 'native/capture.json', 'resampling': False,
                                    'source': 'Macroquad native framebuffer', 'status': 'awaiting_visual_review'})
    write_html()
    print(json.dumps({'output': str(OUT), 'protected_files': len(baseline), 'animated_files': len(verification)}, ensure_ascii=False))


def write_html() -> None:
    cards = ''
    for i, (sid, name, description) in enumerate(CANDIDATES):
        cards += f'''<article><h2>0{i+1} · {name}</h2><p class="id">{sid}-A01 · à valider</p>
<p>{description}</p><div class="scene"><canvas data-skill="{sid}" width="384" height="320" aria-label="Animation de {name}"></canvas></div>
<p class="hint">La scène fait 6 × 5 cases. Les personnages sont affichés à leur taille native.</p></article>'''
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Project RL · Animations · Lot 01</title><style>
*{box-sizing:border-box}body{margin:0;background:#0b1218;color:#dce4e5;font:16px Segoe UI,sans-serif;padding:24px;max-width:1390px;margin:auto}
h1{font-size:26px;margin:0 0 10px}h2{font-size:20px;margin:0 0 8px}p{line-height:1.5;margin:8px 0}header{margin-bottom:22px}.muted,.hint,.id{color:#96abb7}.id{font-size:13px}.hint{font-size:13px}
.controls{display:flex;gap:10px;flex-wrap:wrap;margin:20px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:9px 13px;font:inherit;cursor:pointer}
button:hover{border-color:#96b7c9}.grid{display:grid;grid-template-columns:repeat(3,minmax(400px,1fr));gap:16px}article{padding:16px;background:#101b23;border:1px solid #273e4c;border-radius:8px;min-width:400px}article>p:not(.id):not(.hint){min-height:49px}
.scene{height:340px;display:grid;place-items:center;background:#091014;overflow:visible}canvas{image-rendering:auto}.lab{margin-top:28px;padding:20px;background:#101b23;border:1px solid #273e4c;border-radius:8px}.lab img{display:block;width:100%;max-width:1280px;height:auto;margin:18px 0}a{color:#93c1e0}.notice{border-left:3px solid #7584a3;padding-left:12px}
@media(max-width:1320px){.grid{grid-template-columns:repeat(2,400px)}}@media(max-width:880px){.grid{grid-template-columns:400px}}
</style><header><h1>Project RL · Animations · Lot 01</h1><p class="muted">ANIM-S01 · trois nouveaux candidats · aucune animation encore validée</p>
<p>Rejouer les mouvements, ralentir la lecture et comparer les scènes en 32 et 64 px.</p><p class="notice">Le Gel est montré pour examiner son rendu. Les probabilités, les dégâts et la progression réelle du personnage seront raccordés au combat après cette revue.</p></header>
<div class="controls"><select id="size" aria-label="Taille native"><option value="64">Cases 64 px</option><option value="32">Cases 32 px</option></select>
<select id="speed" aria-label="Vitesse de lecture"><option value="1">Vitesse ×1</option><option value="0.5">Vitesse ×0,5</option></select><button id="pause">Pause</button><button id="replay">Rejouer</button><span class="muted" id="loading">Chargement des séquences…</span></div>
<main class="grid">CARDS</main><section class="lab"><h2>LAB-S01 · Proposition de laboratoire</h2><p>Choix de compétence, rang appris et bonus d’objet, lecture et pause, avance d’un tour et recharges libres. Les réglages de niveau et d’attributs préparent les essais de combat à venir.</p>
<p>Cette interface attend une validation séparée. Le prototype natif s’ouvre avec <a href="Ouvrir_laboratoire.ps1">Ouvrir_laboratoire.ps1</a>, depuis le dossier de cette revue.</p>
<img src="native/laboratoire-prototype.png" alt="Capture réelle du prototype natif du laboratoire"><p class="muted">Entrée : lancer · Espace : avancer d’un tour · R : rejouer · Échap : fermer</p></section>
<script>
const manifest=CAPTURE_MANIFEST;
const ids=['ME02','FE01','FR01'];const sequences=new Map();let ready=false,paused=false,elapsed=0,last=0;
const size=document.getElementById('size'),speed=document.getElementById('speed'),pause=document.getElementById('pause');
async function loadFrames(){for(const id of ids){for(const tile of [64,32]){const entry=manifest.find(e=>e.id===id&&e.tile===tile);
const images=await Promise.all(Array.from({length:entry.frames},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${entry.folder}/${String(i).padStart(3,'0')}.png`;})));sequences.set(`${id}-${tile}`,{entry,images});}}ready=true;document.getElementById('loading').textContent='Prêt · boucle animée';}
function frame(now){if(last&&!paused)elapsed+=(now-last)*Number(speed.value);last=now;if(ready){const index=Math.floor((elapsed%1600)/50);const tile=Number(size.value);for(const canvas of document.querySelectorAll('canvas')){const {entry,images}=sequences.get(`${canvas.dataset.skill}-${tile}`);const [x,y,w,h]=entry.crop;if(canvas.width!==w){canvas.width=w;canvas.height=h;}const ctx=canvas.getContext('2d');ctx.imageSmoothingEnabled=false;ctx.drawImage(images[index],0,0,w,h);}}requestAnimationFrame(frame);}
pause.onclick=()=>{paused=!paused;pause.textContent=paused?'Lecture':'Pause';};document.getElementById('replay').onclick=()=>{elapsed=0;};size.onchange=()=>{elapsed=0;};loadFrames().catch(()=>document.getElementById('loading').textContent='Séquence introuvable : conserver les sous-dossiers de la revue avec ce fichier.');requestAnimationFrame(frame);
</script></html>'''.replace('CARDS', cards).replace('CAPTURE_MANIFEST', (OUT / 'native/capture.json').read_text(encoding='utf-8'))
    (OUT / 'index.html').write_text(html, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--prepare', action='store_true')
    arguments = parser.parse_args()
    prepare() if arguments.prepare else package()
