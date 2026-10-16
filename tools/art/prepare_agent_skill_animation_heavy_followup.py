"""ANIM-S26: native channel/mastery review, with manual turns in the browser."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops
import prepare_agent_skill_animation_aim_recovery as encoder
from add_browser_animation_comparison import add_comparison

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-56'
FRAMES = 160

def dump(name, value):
    (OUT / name).write_text(json.dumps(value, ensure_ascii=False, indent=2), encoding='utf8')

def package():
    assert not (OUT / 'visual-acceptance-01.json').exists()
    proof = json.loads((OUT / 'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT / n).read_bytes()).hexdigest() == h for n, h in proof.items())
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf8'))
    checks, animations, overview = [], [], []
    previous = {a['file']: a for a in json.loads((OUT / 'verification.json').read_text(encoding='utf8'))['animations']} if (OUT / 'verification.json').exists() else {}
    reused_channel_gifs = {}
    encoder.FRAMES = FRAMES
    for e in entries:
        directory = OUT / 'native' / e['folder']
        timeline = json.loads((directory / 'timeline.json').read_text(encoding='utf8'))
        originals = [Image.open(directory / f'{n:03}.png').convert('RGB') for n in range(FRAMES)]
        frames = [im.crop((32, 88, 608, 664)) for im in originals]
        destination = OUT / 'frames' / e['variant']
        destination.mkdir(parents=True, exist_ok=True)
        canonical, indices = {}, []
        for n, im in enumerate(frames):
            key = hashlib.sha256(im.tobytes()).hexdigest()
            if key not in canonical:
                canonical[key] = n
                im.save(destination / f'{n:03}.png', compress_level=1)
            indices.append(canonical[key])
        differences = {}
        for n in e['control_frames']:
            control = Image.open(directory / f'control-{n:03}.png').convert('RGB')
            delta = ImageChops.difference(originals[n], control).crop((32, 88, 608, 664))
            differences[n] = delta
            for y, row in enumerate(e['map'].splitlines()):
                for x, value in enumerate(row):
                    if value == '#':
                        assert delta.crop((x*64, y*64, (x+1)*64, (y+1)*64)).getbbox() is None, (e['variant'], n, x, y)
        if not e['permitted']:
            assert all(t['shots'] == 0 and t['turn'] == 0 and t['energy'] == 100 and t['cooldown'] == 0 for t in timeline)
            assert all(d.getbbox() is None for d in differences.values())
        else:
            assert timeline[14]['preview_visible'] and not timeline[17]['preview_visible']
            assert timeline[17]['shots'] == 1 and timeline[17]['turn'] == 1
            assert any(differences[n].getbbox() is not None for n in [20, 22, 24, 26, 28, 32])
            assert all(differences[n].getbbox() is None for n in [44, 60, 84, 158])
            if e['id'] == 'HL03':
                assert timeline[17]['cooldown'] == 0 and timeline[17]['remaining'] == 4
                assert timeline[17]['energy'] == (4 if e['case'] == 'ENERGIE' else 92)
                if e['quiet']:
                    assert timeline[-1]['active'] and timeline[-1]['shots'] == 1 and timeline[-1]['remaining'] == 4
                    assert timeline[-1]['damage'] == [46, 46, 0, 0]
                elif e['case'] in ['ENERGIE', 'DEPLACEMENT', 'AUTRE-ATTAQUE', 'AUTRE-COMPETENCE', 'PLUS-DE-CIBLE']:
                    assert timeline[64]['shots'] == 1 and timeline[64]['cooldown'] == 8
                    assert not timeline[64]['active'] and timeline[-1]['cooldown'] == 5
                    assert timeline[-1]['energy'] == (4 if e['case'] == 'ENERGIE' else 92)
                else:
                    assert [timeline[n]['shots'] for n in [17, 64, 88, 112]] == [1, 2, 3, 4]
                    assert timeline[112]['active'] and timeline[112]['remaining'] == 1
                    assert not timeline[124]['active'] and timeline[124]['cooldown'] == 8
                    assert timeline[-1]['shots'] == 4 and timeline[-1]['energy'] == 68
                    assert timeline[-1]['damage'] == [138, 184, 92, 0]
                assert timeline[-1]['hp'] == (95 if e['case'] == 'DEGATS-RECUS' else 100)
            else:
                assert all(not t['active'] and t['remaining'] == 0 and t['energy'] == 100 and t['cooldown'] == 0 for t in timeline)
                excluded = e['case'] in ['LEGERE', 'MELEE']
                shots = 1 if e['quiet'] else 5
                assert timeline[-1]['shots'] == shots
                assert timeline[-1]['last_damage'][1] == (20 if excluded else 42)
                assert timeline[-1]['mastery_bonus_percent'] == (0 if excluded else 110)
                expected = [0, 20*shots, 0, 0] if excluded else [42*shots]*3 + [0]
                assert timeline[-1]['damage'] == expected
        checks.append({'variant': e['variant'], 'walls_clear': True,
                       'manual_turns_and_refusals_checked': True,
                       'preview_removed_on_launch': e['permitted'],
                       'permanent_mastery_without_timed_status': e['id'] == 'HL04'})
        filename = f"{e['variant']}-64.gif"
        if e['id'] == 'HL03' and filename in previous:
            reused_channel_gifs[filename] = hashlib.sha256((OUT / filename).read_bytes()).hexdigest()
            animations.append(previous[filename])
        else:
            animations.append(encoder.gif(OUT / filename, frames))
        e.update(timeline=timeline, image_indices=indices)
        if e['case'] == 'HAUT':
            overview.extend(originals)
            for n in [14, 17, 20, 24, 28, 36, 44, 64, 72, 88, 112, 124, 136, 148]:
                originals[n].save(OUT / f"{e['id']}-phase-{n:03}.png")
        print(e['variant'] + ' packaged', flush=True)
    animations.append(encoder.gif(OUT / 'deluge-maitrise-64.gif', overview))
    assert all(hashlib.sha256((OUT / n).read_bytes()).hexdigest() == h for n, h in reused_channel_gifs.items())
    if reused_channel_gifs:
        dump('channel-gif-reuse-verification.json', {'unchanged_gifs': reused_channel_gifs})
    dump('verification.json', {'native_views': len(entries), 'native_tile_size': 64,
                               'native_capture_resampled': False, 'protected_files_unchanged': len(proof),
                               'checks': checks, 'animations': animations})
    dump('review.json', {
        'batch': 'ANIM-S26', 'date': '2026-10-14', 'status': 'awaiting_visual_review', 'campaign_integration': False,
        'candidates': [{'number': 61, 'id': 'HL03', 'name': 'Déluge d’acier', 'variant': 'HL03-A01'},
                       {'number': 62, 'id': 'HL04', 'name': 'Maîtrise des armes lourdes', 'variant': 'HL04-A01'}],
        'approved_cards': ['review-individual-description-HL03-02.json', 'review-individual-description-HL04-02.json'],
        'mechanics': 'Rank11: Déluge weapon damage230%, fixed aim circle radius2, one native sequence per turn for up to4turns, cost8energy per fire, cooldown8after end or interruption. Movement, other attack/skill, missing observed target or inability to pay stop it; HP damage does not. Mastery permanently adds110% to heavy weapon damage, including explosions and weapon skills.',
        'visuals': 'Accepted50px steel rocket, fast0.20s flight, same134px burst and physical blood. New fixed amber aim-zone boundary during the channel,32px player status icon and remaining turns. Shared target/player recoil. Mastery uses the same ordinary heavy shot and numerical bonus with no timed buff or separate cast; adjacent damage readouts are staggered vertically for readability.',
        'prepared_numbers': 'Base20 becomes42(+22) with mastery, or remains20 with the prepared light/melee weapon. Déluge uses46 per blast, including overlaps; floating values show the individual new impact, while timeline totals accumulate. Caster HP100to95 in the received-hit fixture.',
        'scope': 'Independent prepared scenes. Channel target sequence and turn events are fixed for visual review; they do not define final AI/target priority, initial cost/tick order, cooldown boundary or actual combat resolver. Interruption scenes show channel removal, without executing a replacement attack or skill. Missing-box refusal follows later equipment direction; no matter/ammo counter introduced.',
        'pending': 'Actual stats/weapon variance/defenses, heavy-skill modifier stacking, future heavy family native modes, delayed hits after interruption, missing-target causality and edge cases remain separate. Passive applicability to another owner/familiar is not specified by this review.',
        'temporary_actors': 'Existing32px enemies enlarged64; adaptation after animations.',
        'excluded_drafts': ['native-preview-draft', 'native-mastery-preview-draft', 'native-label-layout-draft'],
        'draft_note': 'Drafts are unpresented; final mastery area preview, readable numeric label and staggered hit values in native/ are the candidate. Channel views copied byte-identically from the initial readback.',
    })
    write_html(entries)
    add_comparison(OUT)
    launcher = (OUT.with_name('skill-animation-review-55') / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-armes-lourdes-ouverture', '--lot-armes-lourdes-suite')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher, encoding='utf-8-sig')
    (OUT / 'render-source.rs').write_bytes((ROOT / 'src/agent_skill_lab_review/fire_painted/heavy_followup.rs').read_bytes())
    (OUT / 'shared-art-source.rs').write_bytes((ROOT / 'src/agent_skill_lab_review/fire_painted/heavy_opening.rs').read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'views': len(entries), 'animations': len(animations), 'checks': 'passed'}))

def write_html(entries):
    manifest = [{k: e[k] for k in ['case','case_title','variant','id','title','permitted','quiet','timeline','image_indices']} for e in entries]
    script = '''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=false,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(!e.permitted)return 14;const step=Math.floor(elapsed*24/1000);if(turn===0)return step<27?17+step:44+(step-27)%13;const base=[17,64,88,112,136][turn];return step<20?base+step:base+20+(step-20)%4;}
function ui(){const es=entries();next.disabled=!ready||!launched||es.every(e=>e.quiet)||!es.some(e=>e.permitted)||turn>=4||elapsed<850;cast.disabled=!ready||launched||!es.some(e=>e.permitted);status.textContent=!ready?'Chargement…':!launched?'Prévisualisation clignotante · confirmer pour commencer':es.map(e=>{const t=e.timeline[frame(e)];return e.title+(e.permitted?(e.id==='HL03'?' : tirs '+t.shots+' · durée '+t.remaining+' tours · recharge '+t.cooldown:' : dernier impact '+t.last_damage[1]+' · bonus '+t.mastery_bonus_percent+' %'):' : indisponible');}).join(' · ');}
async function load(){ready=false;ui();const n=++token,cache=new Map(),data=await Promise.all(entries().map(async e=>[e.id,await Promise.all(Array.from({length:160},(_,i)=>{const src='frames/'+e.variant+'/'+String(e.image_indices[i]).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=src;}));return cache.get(src);}))]));if(n!==token)return;sequences=new Map(data);elapsed=0;turn=0;launched=false;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===canvas.dataset.skill);canvas.parentElement.hidden=!e;if(e)canvas.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
for(const canvas of document.querySelectorAll('canvas')){canvas.onmousemove=ev=>{const e=entries().find(e=>e.id===canvas.dataset.skill),box=canvas.getBoundingClientRect(),x=ev.clientX-box.left,y=ev.clientY-box.top,tip=canvas.parentElement.querySelector('.tip'),t=e?.timeline[frame(e)];tip.hidden=!(ready&&launched&&e?.id==='HL03'&&t?.active&&x>=488&&x<=528&&y>=16&&y<=56);if(!tip.hidden)tip.textContent='Déluge d’acier · dégâts ×2,30 · '+t.remaining+' tours restants';};canvas.onmouseleave=()=>canvas.parentElement.querySelector('.tip').hidden=true;}
scenario.onchange=load;cast.onclick=()=>{launched=true;elapsed=0;turn=0;ui();};next.onclick=()=>{turn++;elapsed=0;ui();};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};document.getElementById('pause').onclick=ev=>{paused=!paused;ev.target.textContent=paused?'Lecture':'Pause';};load().catch(e=>status.textContent='Chargement impossible : '+e);requestAnimationFrame(tick);'''.replace('MANIFEST', json.dumps(manifest, ensure_ascii=False))
    options = ''.join(f'<option value="{e["case"]}">{e["case_title"] if e["case"] not in ["HAUT","DROITE","GAUCHE","BAS"] else e["case"].capitalize()}</option>' for e in {e['case']:e for e in entries}.values())
    articles = ''.join(f'<article><h2>{number} · {title}</h2><p>{subtitle}</p><canvas width="576" height="576" data-skill="{skill}"></canvas><div class="tip" hidden></div></article>' for number,skill,title,subtitle in [(61,'HL03','Déluge d’acier','Un tir à chaque tour dans la zone choisie, pendant quatre tirs maximum.'),(62,'HL04','Maîtrise des armes lourdes','La même attaque ordinaire bénéficie du bonus permanent de dégâts.')])
    gallery = ''.join(f'<a href="{e["variant"]}-64.gif">{e["title"]} · {e["case"]}</a>' for e in entries)
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Déluge et maîtrise · ANIM-S26</title>
<style>*{box-sizing:border-box}body{background:#0a0f13;color:#e0e7e7;font:16px system-ui;margin:0 auto;padding:24px;max-width:1280px}h1{font-size:27px}h2{font-size:21px}p{color:#a5b9c5;line-height:1.5}button,select{font:inherit;color:#e0e7e7;background:#202931;border:1px solid #566574;border-radius:4px;padding:8px 12px}button:disabled{opacity:.4}.controls{display:flex;gap:8px;flex-wrap:wrap}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:12px;overflow:auto}article{background:#11181e;padding:16px;position:relative}article p{min-height:48px}canvas{display:block;width:576px;height:576px;background:#080c0e}.gallery{display:flex;gap:12px;flex-wrap:wrap}a{color:#bbc7ff}details{margin-top:22px}summary{cursor:pointer}.note{font-size:13px}.tip{position:absolute;right:26px;top:185px;background:#10151b;border:1px solid #737f91;color:#e0e7e7;max-width:282px;padding:12px;pointer-events:none}</style>
<main><h1>Armes lourdes · lot 26 · cases de 64 px</h1><p>Deux nouveaux candidats : bombardement par tour et bonus permanent. Les nombres illustrent les impacts, sans fixer l’équilibrage.</p>
<p><a href="cote-a-cote.html">Voir les deux animations côte à côte</a></p><div class="controls"><select id="scenario" aria-label="Scénario">OPTIONS</select><button id="cast">Commencer</button><button id="next">Action suivante</button><button id="reset">Recommencer</button><button id="pause">Pause</button></div><p id="status">Chargement…</p><div class="grid">ARTICLES</div>
<p class="note">Les tours avancent uniquement avec « Action suivante ». Déluge d’acier poursuit ses tirs à chaque tour ; son icône montre la durée restante. Le contour doré indique la zone de ciblage active. Maîtrise n’ajoute pas de buff temporaire. Survolez l’icône du bombardement pour lire son effet.</p>
<p class="note">Scénarios préparés : interruptions, énergie limitée, perte des cibles, attaque reçue et armes exclues. Le bombardement résiste aux dégâts reçus. Les acteurs et valeurs restent provisoires.</p><details><summary>GIF par compétence et scénario</summary><p class="note">Ces enregistrements avancent automatiquement les actions.</p><div class="gallery">GALLERY</div></details><p class="note">Propositions à valider avant raccordement au combat. Projectile, explosions et sang réutilisent les dessins acceptés.</p></main><script>SCRIPT</script></html>'''
    (OUT / 'index.html').write_text(page.replace('OPTIONS',options).replace('ARTICLES',articles).replace('GALLERY',gallery).replace('SCRIPT',script),encoding='utf8')

if __name__ == '__main__':
    package()
