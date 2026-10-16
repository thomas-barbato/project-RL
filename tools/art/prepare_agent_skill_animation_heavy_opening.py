"""ANIM-S25: two native 64px heavy-weapon previews with manual turn controls."""
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageChops
import prepare_agent_skill_animation_aim_recovery as encoder
from add_browser_animation_comparison import add_comparison

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-55'
FRAMES = 128


def dump(name, value):
    (OUT / name).write_text(json.dumps(value, ensure_ascii=False, indent=2), encoding='utf8')


def package():
    assert not (OUT / 'visual-acceptance-01.json').exists()
    proof = json.loads((OUT / 'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == digest
               for name, digest in proof.items())
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf8'))
    checks, animations, overview = [], [], []
    encoder.FRAMES = FRAMES
    for e in entries:
        directory = OUT / 'native' / e['folder']
        timeline = json.loads((directory / 'timeline.json').read_text(encoding='utf8'))
        originals = [Image.open(directory / f'{n:03}.png').convert('RGB') for n in range(FRAMES)]
        frames = [im.crop((32, 88, 608, 664)) for im in originals]
        dest = OUT / 'frames' / e['variant']
        dest.mkdir(parents=True, exist_ok=True)
        canonical, indices = {}, []
        for n, im in enumerate(frames):
            key = hashlib.sha256(im.tobytes()).hexdigest()
            if key not in canonical:
                canonical[key] = n
                im.save(dest / f'{n:03}.png', compress_level=1)
            indices.append(canonical[key])
        differences = {}
        for n in e['control_frames']:
            control = Image.open(directory / f'control-{n:03}.png').convert('RGB')
            delta = ImageChops.difference(originals[n], control).crop((32, 88, 608, 664))
            differences[n] = delta
            for y, row in enumerate(e['map'].splitlines()):
                for x, value in enumerate(row):
                    if value == '#':
                        assert delta.crop((x * 64, y * 64, (x + 1) * 64, (y + 1) * 64)).getbbox() is None, (e['variant'], n, x, y)
        if not e['permitted']:
            assert all(t['energy'] == 100 and t['cooldown'] == 0 and t['turn'] == 0 and t['damage'] == [0] * 4 for t in timeline)
            assert all(d.getbbox() is None for d in differences.values())
        else:
            assert timeline[14]['preview_visible'] and not timeline[17]['preview_visible']
            assert timeline[17]['launched'] and timeline[17]['turn'] == 1
            assert timeline[17]['energy'] == (82 if e['id'] == 'HL01' else 86)
            assert timeline[17]['cooldown'] == (5 if e['id'] == 'HL01' else 6)
            expected = [0, 40, 0, 0] if e['case'] == 'UN-ENNEMI' else [80, 120, 80, 0] if e['id'] == 'HL01' else [36, 36, 36, 0]
            assert timeline[36]['damage'] == timeline[-1]['damage'] == expected
            assert any(differences[n].getbbox() is not None for n in [20, 22, 24, 26, 28, 30, 32])
            assert all(differences[n].getbbox() is None for n in [44, 48, 60, 64, 68, 84, 88, 92, 112, 116, 126])
            if e['id'] == 'HL01':
                assert len(e['aims']) == (1 if e['case'] == 'UN-ENNEMI' else 3)
                assert all(t['stun'] == [0] * 4 for t in timeline)
            else:
                stunned = [2, 0, 2, 0] if e['case'] in ['ELITE', 'BOSS'] else [2, 2, 2, 0]
                assert timeline[36]['stun'] == stunned
                assert timeline[36]['stun_checked'] == [True, True, True, False]
                assert timeline[-1]['stun'] == (stunned if e['quiet'] else [0] * 4)
                if not e['quiet']:
                    assert timeline[64]['stun'] == [n // 2 for n in stunned]
                    assert timeline[88]['stun'] == [0] * 4
        checks.append({'variant': e['variant'], 'world_walls_clear': True,
                       'preview_hidden_on_confirmation': e['permitted'],
                       'damage_status_timing_and_atomic_refusals': True})
        animations.append(encoder.gif(OUT / f"{e['variant']}-64.gif", frames))
        e.update(timeline=timeline, image_indices=indices)
        if e['case'] == 'HAUT':
            overview.extend(originals)
            for n in [14, 17, 20, 22, 24, 26, 28, 32, 36, 44, 64, 88, 112]:
                originals[n].save(OUT / f"{e['id']}-phase-{n:03}.png")
        print(e['variant'] + ' packaged', flush=True)
    animations.append(encoder.gif(OUT / 'armes-lourdes-64.gif', overview))
    dump('verification.json', {'native_views': len(entries), 'native_tile_size': 64,
                               'native_capture_resampled': False, 'protected_files_unchanged': len(proof),
                               'checks': checks, 'animations': animations})
    dump('review.json', {
        'batch': 'ANIM-S25', 'date': '2026-10-14', 'status': 'awaiting_visual_review',
        'candidates': [{'number': 59, 'id': 'HL01', 'name': 'Barrage', 'variant': 'HL01-A01'},
                       {'number': 60, 'id': 'HL02', 'name': 'Impact assommant', 'variant': 'HL02-A01'}],
        'approved_cards': ['review-individual-description-HL01-03.json', 'review-individual-description-HL02-03.json'],
        'mechanics': 'Rank11: Barrage up to3 distinct observed enemies, damage200%, energy18, cooldown5. Impact assommant damage180%, ordinary stun60%, elite45%, bosses immune to stun, duration2turns, energy14, cooldown6. Native weapon radius1 and range7 retained in this rocket fixture.',
        'visuals': 'Short new steel rocket, fast travel, accepted burst and physical blood; shared target recoil. Stun orbit and target-state strip persist until turns pass. Numbered blinking affected areas disappear on confirmation.',
        'prepared_values': 'Damage40 per barrage blast and36 per stun blast are fixed pre-defense samples, not final character/weapon values. Overlap yields80/120/80. A fixed50% stun sample succeeds ordinarily but fails against the prepared elite and boss.',
        'scope': 'Independent prepared rocket scenes, with selected enemies and contacts fixed for visual review. No AI, campaign resolver, actual perception, RNG, equipment/stat curve, damage variance or final damage rounding added.',
        'supply_revision': 'Matter/ammunition consumption omitted under later accepted equipped ammunition-box direction. The missing-box scene is a prepared refusal; no new box object or shop/drop implementation.',
        'pending': 'Other heavy-family native shots, native bursts/delays, more-than-three target selection, individual inspected-target navigation, actual armor/element splits, friendly-source damage attribution and dense combat integration remain separate.',
        'temporary_actors': 'Existing enemy32 artwork enlarged64; bestiary adaptation follows completion of animations.',
        'campaign_integration': False,
    })
    write_html(entries)
    add_comparison(OUT)
    launcher = '''$ErrorActionPreference = 'Stop'
$heavyProject = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$heavyBinary = Join-Path $heavyProject 'target/debug/project-rl.exe'
if (-not (Test-Path -LiteralPath $heavyBinary)) { throw 'Compiler le projet en mode debug avant cet aperçu.' }
Push-Location -LiteralPath $heavyProject
try { & $heavyBinary '--apercu-laboratoire-competences' '--lot-armes-lourdes-ouverture' }
finally { Pop-Location }
'''
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher, encoding='utf-8-sig')
    (OUT / 'render-source.rs').write_bytes((ROOT / 'src/agent_skill_lab_review/fire_painted/heavy_opening.rs').read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'views': len(entries), 'animations': len(animations), 'checks': 'passed'}))


def write_html(entries):
    manifest = [{k: e[k] for k in ['case', 'case_title', 'variant', 'id', 'title', 'permitted', 'quiet', 'timeline', 'image_indices']} for e in entries]
    script = '''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=false,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(!e.permitted)return 14;const step=Math.floor(elapsed*24/1000);if(turn===0)return step<27?17+step:44+(step-27)%13;return [17,64,88,112][turn]+step%13;}
function ui(){const es=entries();next.disabled=!ready||!launched||es.every(e=>e.quiet)||!es.some(e=>e.permitted)||turn>=3||elapsed<850;cast.disabled=!ready||launched||!es.some(e=>e.permitted);status.textContent=!ready?'Chargement…':!launched?'Prévisualisation clignotante · confirmer pour commencer':es.map(e=>{const t=e.timeline[frame(e)];return e.title+(e.permitted?' : dégâts '+t.damage.filter(n=>n>0).join(' / ')+(e.id==='HL02'?' · assommement '+Math.max(...t.stun)+' tours':''):' : indisponible');}).join(' · ');}
async function load(){ready=false;ui();const n=++token,cache=new Map(),data=await Promise.all(entries().map(async e=>[e.id,await Promise.all(Array.from({length:128},(_,i)=>{const src='frames/'+e.variant+'/'+String(e.image_indices[i]).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=src;}));return cache.get(src);}))]));if(n!==token)return;sequences=new Map(data);elapsed=0;turn=0;launched=false;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===canvas.dataset.skill);canvas.parentElement.hidden=!e;if(e)canvas.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
scenario.onchange=load;cast.onclick=()=>{launched=true;elapsed=0;turn=0;ui();};next.onclick=()=>{turn++;elapsed=0;ui();};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};document.getElementById('pause').onclick=ev=>{paused=!paused;ev.target.textContent=paused?'Lecture':'Pause';};load().catch(e=>status.textContent='Chargement impossible : '+e);requestAnimationFrame(tick);'''.replace('MANIFEST', json.dumps(manifest, ensure_ascii=False))
    options = ''.join(f'<option value="{e["case"]}">{e["case_title"] if e["case"] not in ["HAUT","DROITE","GAUCHE","BAS"] else e["case"].capitalize()}</option>'
                      for e in {e['case']: e for e in entries}.values())
    articles = ''.join(f'<article><h2>{number} · {title}</h2><p>{subtitle}</p><canvas width="576" height="576" data-skill="{skill}"></canvas></article>'
                       for number, skill, title, subtitle in [(59, 'HL01', 'Barrage', 'Jusqu’à trois tirs, dirigés vers les ennemis présents.'), (60, 'HL02', 'Impact assommant', 'Un impact brutal, puis un signe tournoyant sur les cibles assommées.')])
    gallery = ''.join(f'<a href="{e["variant"]}-64.gif">{e["title"]} · {e["case_title"]} · {e["case"]}</a>' for e in entries)
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Armes lourdes · ANIM-S25</title>
<style>*{box-sizing:border-box}body{background:#0a0f13;color:#e0e7e7;font:16px system-ui;margin:0 auto;padding:24px;max-width:1280px}h1{font-size:27px}h2{font-size:21px}p{color:#a5b9c5;line-height:1.5}button,select{font:inherit;color:#e0e7e7;background:#202931;border:1px solid #566574;border-radius:4px;padding:8px 12px}button:disabled{opacity:.4}.controls{display:flex;gap:8px;flex-wrap:wrap}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:12px;overflow:auto}article{background:#11181e;padding:16px}article p{min-height:48px}canvas{display:block;width:576px;height:576px;background:#080c0e}.gallery{display:flex;gap:12px;flex-wrap:wrap}a{color:#bbc7ff}details{margin-top:22px}summary{cursor:pointer}.note{font-size:13px}</style>
<main><h1>Armes lourdes · lot 25 · cases de 64 px</h1><p>Deux nouveaux candidats à examiner : lance-roquettes rapide, explosions et blessures, puis assommement visible pendant deux tours.</p>
<p><a href="cote-a-cote.html">Voir les deux animations côte à côte</a></p>
<div class="controls"><select id="scenario" aria-label="Scénario">OPTIONS</select><button id="cast">Lancer</button><button id="next">Action suivante</button><button id="reset">Recommencer</button><button id="pause">Pause</button></div><p id="status">Chargement…</p>
<div class="grid">ARTICLES</div><p class="note">Les tours avancent uniquement avec « Action suivante ». Les explosions finissent leur animation ; l’assommement reste tant que ses tours ne sont pas écoulés. Dégâts et réussite préparés pour cet aperçu. Les monstres actuels restent temporaires.</p>
<details><summary>GIF par compétence et scénario</summary><p class="note">Ces enregistrements font avancer les actions automatiquement.</p><div class="gallery">GALLERY</div></details>
<p class="note">Propositions à valider avant intégration au combat. Nouveau projectile créé avec imagegen · <a href="imagegen-rocket-prompt.txt">consigne</a> · <a href="rocket-source.png">source</a>. Les explosions et le sang réutilisent les dessins acceptés.</p></main><script>SCRIPT</script></html>'''
    (OUT / 'index.html').write_text(page.replace('OPTIONS', options).replace('ARTICLES', articles).replace('GALLERY', gallery).replace('SCRIPT', script), encoding='utf8')


if __name__ == '__main__':
    package()
