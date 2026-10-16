"""GEL-A04: body-attached frost animation, isolated native review."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_crystalline as previous
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-27'
SOURCE = Path(r'C:\Users\User\.codex\generated_images\01a11010-433f-7fc2-ac29-ffb696dec5ea\exec-be7c579b-5e34-4746-9627-2c2473c542dd.png')


def protected():
    hashes = previous.protected()
    for root in [previous.OUT, base.ROOT/'src/game', base.ROOT/'src/skills', base.ROOT/'src/stats']:
        for p in sorted(root.rglob('*')):
            if p.is_file():
                hashes[p.relative_to(base.ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT/'review-protection.json').exists()
    base.dump(OUT/'review-protection.json', protected())
    (OUT/'gel-source.png').write_bytes(SOURCE.read_bytes())
    with Image.open(SOURCE) as image:
        assert image.mode == 'RGBA' and image.size == (1536,1024)
        assert image.getchannel('A').getextrema()[0] == 0
        atlas = Image.new('RGBA', (1536,256))
        for i in range(6):
            x,y = i%3*512,i//3*512
            sprite = image.crop((x,y,x+512,y+512)).resize((256,256),Image.Resampling.LANCZOS)
            atlas.paste(sprite,(i*256,0))
        atlas.save(OUT/'gel-atlas.png')
    base.dump(OUT/'drawing-pack.json', {'mode':'Built-in imagegen','source_sha256':hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
        'source_size':[1536,1024],'layout':[3,2],'cell_size':[256,256],'anchor':[.5,.5],
        'alpha_preserved':True,'postprocessing':'Equal-cell extraction and uniform resize only. No repainting.'})
    base.dump(OUT/'review.json', {'batch':'ANIM-GEL01','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':20,'id':'GEL','variant':'GEL-A04','name':'Gel','status':'awaiting_visual_review'},
            {'number':19,'id':'FR06','variant':'FR06-A02','name':'Explosion cristalline avec Gel animé','status':'awaiting_visual_review'}],
        'effect':'Translucent ice surrounds the whole actor with a slow six-frame shimmer. Actor body and equipment drawings are preserved.',
        'timing':'Visual loop continues while status is present. Only explicit turns reduce Gel. Removal or crystalline consumption stops the attached effect immediately.',
        'examples':'Frozen player and enemy, ordinary enemy without frost, expiry, replacement by Burn, crystalline consumption, four directions and walls.',
        'scope':'Isolated debug visual fixture. No new status rules, damage, action denial, AI, RNG, saves or campaign integration.',
        'retained':'All earlier review artifacts, approved Fire, Mur and Blizzard art, catalogue and actor artwork.'})
    launch = (previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--lot-explosion-cristalline'", "'--lot-gel-statut' '--gel-status-v2'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch, encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'frames':6}))


def convert():
    native = (OUT/'native').resolve()
    count = 0
    for raw in native.rglob('*.rgba'):
        assert raw.resolve().is_relative_to(native)
        pixels = raw.read_bytes()
        assert len(pixels) == 720*720*4
        png = raw.with_suffix('.png')
        Image.frombytes('RGBA',(720,720),pixels).save(png,compress_level=1)
        with Image.open(png) as check:
            assert check.tobytes() == pixels
        raw.unlink()  # Verified transient readback, strictly inside this review.
        count += 1
    if count:
        base.dump(OUT/'native-encoding-verification.json', {'converted_frames':count,'pixel_identical_to_native_readback':True,'resampling':False})
    else:
        assert (OUT/'native-encoding-verification.json').exists()


def package():
    baseline = json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected() == baseline
    convert()
    entries = json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert len(entries) == 9
    checks,animations = [],[]
    for e in entries:
        folder = OUT/'native'/e['folder']
        target = OUT/'frames'/e['variant']
        target.mkdir(parents=True,exist_ok=True)
        x,y,w,h = e['crop']
        bounds = (x,y,x+w,y+h)
        frames = []
        for i in range(e['frames']):
            with Image.open(folder/f'{i:03}.png') as image:
                frame = image.crop(bounds).convert('RGB')
            frame.save(target/f'{i:03}.png',compress_level=1)
            frames.append(frame)
        diffs = {}
        for phase in e['control_frames']:
            with Image.open(folder/f'control-{phase:03}.png') as image:
                diffs[phase] = ImageChops.difference(frames[phase],image.crop(bounds).convert('RGB'))
            for cy,row in enumerate(e['map'].splitlines()):
                for cx,symbol in enumerate(row):
                    if symbol == '#':
                        assert diffs[phase].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None, (e['variant'],phase,cx,cy)
        timeline = json.loads((folder/'timeline.json').read_text(encoding='utf-8'))
        if e['id'] == 'GEL':
            assert all(r['gel'] == [3,3,0] and r['turn'] == 0 for r in timeline[17:48])
            assert all(diffs[p].getbbox() for p in (17,24,35,47,50))
            assert all(diffs[p].getbbox() is None for p in (83,95))
            for phase in e['control_frames']:
                assert diffs[phase].crop((6*64,4*64,7*64,5*64)).getbbox() is None,'Ordinary enemy received status art.'
            if e['replacement']:
                assert timeline[64]['gel'] == [0,0,0] and timeline[64]['burn'] == [1,1,0]
            else:
                assert [timeline[p]['gel'][0] for p in (48,64,80)] == [2,1,0]
            for stage in range(3):
                for i in range(48):
                    with Image.open(folder/f'idle-{stage}-{i:02}.png') as image:
                        idle = image.crop(bounds).convert('RGB')
                    idle.save(target/f'idle-{stage}-{i:02}.png',compress_level=1)
            with Image.open(folder/'idle-0-00.png') as a,Image.open(folder/'idle-0-12.png') as b:
                assert ImageChops.difference(a.convert('RGB'),b.convert('RGB')).getbbox(),'Frozen status is static.'
            if not e['replacement']:
                animations.append(save_animation(OUT/'GEL-A04-64.gif',frames,24))
                frames[35].save(OUT/'gel-actif-64.png')
            else:
                animations.append(save_animation(OUT/'GEL-A04-BRULURE-64.gif',frames,24))
        else:
            original_entry = dict(e,folder=e['folder'].split('/')[-1])
            previous.timeline_check(original_entry,timeline)
            assert diffs[3].getbbox() and diffs[14].getbbox()
            assert all(diffs[p].getbbox() is None for p in (28,35,47,50,66,83,95))
            for i in range(48):
                with Image.open(folder/f'preview-idle-{i:02}.png') as image:
                    image.crop(bounds).convert('RGB').save(target/f'preview-idle-{i:02}.png',compress_level=1)
            if e['folder'].endswith('/HAUT'):
                animations.append(save_animation(OUT/'FR06-A02-HAUT-64.gif',frames,24))
                frames[14].save(OUT/'avant-explosion-64.png')
            if e['folder'].endswith('/CLOISON'):
                animations.append(save_animation(OUT/'FR06-A02-CLOISON-64.gif',frames,24))
        checks.append({'variant':e['variant'],'solid_walls_clear':True,'turn_state_stable_while_waiting':True,'removed_status_leaves_no_frost':True})
    base.dump(OUT/'verification.json', {'protected_files_unchanged':len(baseline),'controls':checks,'animations':animations,'resampling_of_native_capture':False})
    for src,name in [('src/agent_skill_lab_review/fire_painted/frost/gel_status.rs','render-source.rs'),
        ('src/agent_skill_lab_review/fire_painted/frost/crystalline.rs','crystalline-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/src).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries)
    print(json.dumps({'native_views':len(entries),'animations':len(animations),'protected_files':len(baseline),'controls':'passed'}))


def write_html(entries):
    manifest = [{k:e[k] for k in ('id','folder','variant','frames')} for e in entries]
    script = '''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,applied=true,turn=0,token=0,sequence;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast');
function entry(){return manifest.find(e=>e.folder===scenario.value);}
function frame(){const e=entry(),f=Math.floor(elapsed*24/1000);if(!applied)return e.id==='GEL'?{kind:'main',i:Math.min(f,16)}:{kind:'preview',i:f%48};const starts=e.id==='GEL'?[17,48,64,80]:[17,48,65,82],ends=e.id==='GEL'?[47,63,79,95]:[47,64,81,95];if(e.id==='GEL'&&f>ends[turn]-starts[turn]&&turn<3)return{kind:'idle',stage:turn,i:f%48};return{kind:'main',i:Math.min(starts[turn]+f,ends[turn])};}
function ui(){const e=entry();cast.textContent=e.id==='GEL'?'Appliquer le Gel':'Lancer';cast.disabled=!ready||applied;next.disabled=!ready||!applied||turn>=3||(e.id==='FR06'&&turn===0&&elapsed<500);document.getElementById('status').textContent=!applied?(e.id==='GEL'?'Avant application':'Gel animé · avant explosion'):e.id==='GEL'?`Tour ${turn} · ${turn===3?'Statuts terminés':e.folder==='BRULURE'&&turn===2?'Brûlure remplace le Gel':`Gel : ${3-turn} tours`}`:`Tour ${turn} · Gel consommé · recharge ${8-turn}`;}
async function load(){ready=false;const n=++token,e=entry();function img(p){return new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${e.variant}/${p}`;});}const main=await Promise.all(Array.from({length:e.frames},(_,i)=>img(`${String(i).padStart(3,'0')}.png`)));const idle=e.id==='GEL'?await Promise.all(Array.from({length:3},(_,s)=>Promise.all(Array.from({length:48},(_,i)=>img(`idle-${s}-${String(i).padStart(2,'0')}.png`))))):null;const preview=e.id==='FR06'?await Promise.all(Array.from({length:48},(_,i)=>img(`preview-idle-${String(i).padStart(2,'0')}.png`))):null;if(n!==token)return;sequence={main,idle,preview};elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();const f=frame(),im=f.kind==='main'?sequence.main[f.i]:f.kind==='preview'?sequence.preview[f.i]:sequence.idle[f.stage][f.i];document.getElementById('view').getContext('2d').drawImage(im,0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){applied=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{applied=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{applied=entry().id==='GEL';turn=0;elapsed=0;load().catch(()=>document.getElementById('status').textContent='Chargement impossible.');};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    options = ''.join(f'<option value="{e["folder"]}">{e["title"]} · {e["folder"]}</option>' for e in entries)
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Gel animé</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1000px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:20px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}canvas{display:block;width:576px;height:576px;margin:auto}a{color:#93c1e0}</style>
<h1>20 · Gel animé</h1><p class="muted">GEL-A04 · proposition en cases natives 64 px</p><p>Le givre enveloppe le personnage concerné et ses reflets restent animés tant que le Gel est présent. Le même effet est montré sur l’agent et sur un monstre ; le voisin non gelé reste intact.</p><div class="controls"><select id="scenario">OPTIONS</select><button id="next">+1 tour</button><button id="cast">Appliquer le Gel</button><button id="reset">Recommencer</button><button id="pause">Pause</button></div><p id="status"></p><canvas id="view" width="576" height="576"></canvas>
<p>Attendre ne réduit pas le statut : les dessins seuls continuent à bouger. « +1 tour » réduit sa durée. L’exemple Brûlure montre son remplacement par les flammes déjà acceptées. Avec Explosion cristalline, le Gel disparaît au lancement et les éclats se jouent une fois.</p><p class="muted">Aperçu isolé à valider. Durées et résultats préparés ; aucun nouveau calcul de combat ni changement de campagne. Les illustrations des personnages et les anciens aperçus sont conservés.</p><p><a href="GEL-A04-64.gif">Gel sur agent et monstre</a> · <a href="FR06-A02-HAUT-64.gif">Consommation par Explosion cristalline</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p><p class="muted">Dessin créé avec imagegen intégré : <a href="gel-source.png">source</a> · <a href="imagegen-prompt.txt">prompt</a>.</p><script>SCRIPT</script></html>'''.replace('OPTIONS',options).replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8')
    (OUT/'gallery-script.js').write_text(script,encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--prepare',action='store_true')
    args = parser.parse_args()
    prepare() if args.prepare else package()
