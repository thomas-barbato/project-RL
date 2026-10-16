"""ANIM-S30: native mine fixtures and direct two-pane browser review."""
import hashlib
import inspect
import json
from pathlib import Path

from PIL import Image, ImageChops
import prepare_agent_skill_animation_heavy_finale as encoder
from add_browser_animation_comparison import add_comparison

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-60'
FRAMES = 160

def dump(name, obj):
    (OUT / name).write_text(json.dumps(obj, ensure_ascii=False, indent=2) + '\n', encoding='utf8')

def gif(path, images):
    source = inspect.getsource(encoder.gif).replace('[14,20,24,28,32,64,68,72]', '[14,18,44,54,64,68,76,88,96,112,114,116,120,124,142]')
    space = dict(encoder.__dict__)
    space['FRAMES'] = FRAMES
    exec(compile(source, '<review60-gif-encoder>', 'exec'), space)
    return space['gif'](path, images)

def write_html(entries):
    manifest = [{k: e[k] for k in ['case', 'case_title', 'variant', 'id', 'title', 'quiet', 'timeline', 'image_indices', 'hover_images', 'mine_cell', 'action_frames']} for e in entries]
    script = r'''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=false,turn=0,token=0,sequences,hoverSequences;const hovered=new Set();
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){const step=Math.floor(elapsed*24/1000);if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(turn===0)return step<27?17+step:44+(step-27)%18;const base=[17,64,88,112][turn];if(e.case==='ENNEMI-ARMEMENT'&&turn===1)return base+Math.min(step,23);if(e.case==='COUCHE'&&turn===2)return base+Math.min(step,23);if(turn===3)return step<24?112+step:142+(step-24)%18;return step<24?base+step:base+6+(step-24)%18;}
function ui(){const es=entries();next.disabled=!ready||!launched||turn>=Math.max(...es.map(e=>e.action_frames.length))||elapsed<850;cast.disabled=!ready||launched;status.textContent=!ready?'Chargement…':!launched?'Prévisualisation clignotante · confirmez la pose':es.map(e=>{const t=e.timeline[frame(e)],m=t.mines.find(m=>m.at[0]===4&&m.at[1]===5);return e.title+' : '+t.count+'/'+t.capacity+' mines · '+(m?(m.armed?'armée':'pas encore armée'):'mine retirée')+' · énergie '+t.energy+' · explosions '+t.explosions;}).join(' · ');}
async function load(){ready=false;hovered.clear();ui();const n=++token,cache=new Map();function image(src){if(!cache.has(src))cache.set(src,new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=()=>reject(new Error(src));im.src=src;}));return cache.get(src);}
const data=await Promise.all(entries().map(async e=>[e.id,await Promise.all(e.image_indices.map(i=>image('frames/'+e.variant+'/'+String(i).padStart(3,'0')+'.png'))),new Map(await Promise.all(e.hover_images.map(h=>image('frames/'+e.variant+'/'+h.file).then(im=>[h.frame,im]))))]));if(n!==token)return;sequences=new Map(data.map(d=>[d[0],d[1]]));hoverSequences=new Map(data.map(d=>[d[0],d[2]]));elapsed=0;turn=0;launched=false;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===canvas.dataset.skill);canvas.parentElement.hidden=!e;if(e){const f=frame(e),t=e.timeline[f];let im=sequences.get(e.id)[f];if(hovered.has(e.id)&&launched&&t.mines.some(m=>m.at[0]===4&&m.at[1]===5)){const h=e.hover_images.find(h=>h.step===t.step&&h.led_on===t.led_on)||e.hover_images.find(h=>h.step===t.step);if(h)im=hoverSequences.get(e.id).get(h.frame);}canvas.getContext('2d').drawImage(im,0,0);}}}requestAnimationFrame(tick);}
for(const canvas of document.querySelectorAll('canvas')){canvas.onmousemove=ev=>{const e=entries().find(e=>e.id===canvas.dataset.skill),box=canvas.getBoundingClientRect(),x=ev.clientX-box.left,y=ev.clientY-box.top;if(e&&x>=e.mine_cell[0]*64&&x<(e.mine_cell[0]+1)*64&&y>=e.mine_cell[1]*64&&y<(e.mine_cell[1]+1)*64)hovered.add(e.id);else hovered.delete(canvas.dataset.skill);};canvas.onmouseleave=()=>hovered.delete(canvas.dataset.skill);}
scenario.onchange=load;cast.onclick=()=>{if(cast.disabled)return;launched=true;elapsed=0;turn=0;ui();};next.onclick=()=>{if(next.disabled)return;turn++;elapsed=0;ui();};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;hovered.clear();ui();};document.getElementById('pause').onclick=ev=>{paused=!paused;ev.target.textContent=paused?'Lecture':'Pause';};load().catch(e=>status.textContent='Chargement impossible : '+e);requestAnimationFrame(tick);
'''.replace('MANIFEST', json.dumps(manifest, ensure_ascii=False))
    options = ''.join(f'<option value="{e["case"]}">{e["case"].capitalize() if e["case"] in ["HAUT","DROITE","GAUCHE","BAS"] else e["case_title"]}</option>' for e in {e['case']: e for e in entries}.values())
    gallery = ''.join(f'<a href="{e["variant"]}-64.gif">{e["title"]} · {e["case"]}</a>' for e in entries)
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Mines · ANIM-S30</title>
<style>*{box-sizing:border-box}body{background:#0a0f13;color:#e0e7e7;font:16px system-ui;margin:0 auto;padding:24px;max-width:1280px}h1{font-size:27px}h2{font-size:21px}p{color:#a5b9c5;line-height:1.5}button,select{font:inherit;color:#e0e7e7;background:#202931;border:1px solid #566574;border-radius:4px;padding:8px 12px}button:disabled{opacity:.4}.controls{display:flex;gap:8px;flex-wrap:wrap}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:12px;overflow:auto}article{background:#11181e;padding:16px;position:relative}article p{min-height:48px}canvas{display:block;width:576px;height:576px;background:#080c0e}.gallery{display:flex;gap:12px;flex-wrap:wrap}a{color:#bbc7ff}details{margin-top:22px}summary{cursor:pointer}.note{font-size:13px}</style>
<main><h1>Mines · lot 30 · cases de 64 px</h1><p>Une mine métallique au sol, sa LED centrale clignotante et son explosion. À droite, Réseau de mines permet d’en conserver cinq au rang 11.</p><div class="controls"><select id="scenario" aria-label="Scénario">OPTIONS</select><button id="cast">Poser la mine</button><button id="next">Action suivante</button><button id="reset">Recommencer</button><button id="pause">Pause</button></div><p id="status">Chargement…</p><div class="grid"><article><h2>68 · Pose de mine</h2><p>Une mine simultanée sans le passif. Le voyant clignote dès la pose ; elle reste inactive avant son armement.</p><canvas width="576" height="576" data-skill="SM00"></canvas></article><article><h2>69 · Réseau de mines</h2><p>Cinq mines au rang 11. Le scénario de capacité remplace la plus ancienne par la nouvelle mine.</p><canvas width="576" height="576" data-skill="SM01"></canvas></article></div>
<p class="note">Après la pose, « Action suivante » passe le tour d’armement, puis montre l’approche de l’ennemi et enfin son entrée sur la mine. Attendre conserve la mine et son clignotement, sans avancer les tours. Survolez la case de la mine centrale pour voir son rayon.</p><p class="note">Exemple accepté : 13 dégâts de feu, rayon d’une case, 10 énergie et recharge d’un tour. Le joueur voisin est protégé. L’explosion est brève, sans sang, sans résidu ni Brûlure ajoutée dans cet exemple. La fiche Pose de mine affiche la capacité du passif ; celui-ci n’ajoute pas de buff temporaire.</p><p class="note">Scènes préparées pour examiner le rendu : pas de combat de campagne, d’IA ni de calcul définitif d’équilibrage. Les mines déjà armées du scénario de capacité représentent des poses antérieures. La géométrie complète, les dégâts aux familiers, l’inspection des objets superposés et les interactions de passifs seront raccordés séparément.</p><details><summary>GIF par compétence et scénario</summary><div class="gallery">GALLERY</div></details><p class="note">Monstres provisoires ; leur adaptation suivra les animations. Dessin de mine réalisé avec le skill imagegen et l’outil intégré : <a href="mine-source.png">source</a> · <a href="imagegen-mine-prompt.txt">prompt</a>.</p></main><script>SCRIPT</script></html>'''
    (OUT / 'index.html').write_text(page.replace('OPTIONS', options).replace('GALLERY', gallery).replace('SCRIPT', script), encoding='utf8')

def package():
    assert not (OUT / 'visual-acceptance-01.json').exists()
    protection = json.loads((OUT / 'review-protection.json').read_text(encoding='utf8'))
    for name, digest in protection.items():
        assert hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == digest, name
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf8'))
    checks, animations, overview = [], [], []
    for e in entries:
        native = OUT / 'native' / e['folder']
        timeline = json.loads((native / 'timeline.json').read_text(encoding='utf8'))
        originals = [Image.open(native / f'{n:03}.png').convert('RGB') for n in range(FRAMES)]
        frames = [im.crop((32,88,608,664)) for im in originals]
        dest = OUT / 'frames' / e['variant']; dest.mkdir(parents=True, exist_ok=True)
        canonical, indices = {}, []
        for n, im in enumerate(frames):
            key = hashlib.sha256(im.tobytes()).hexdigest()
            if key not in canonical:
                canonical[key] = n
                im.save(dest / f'{n:03}.png', compress_level=1)
            indices.append(canonical[key])
        hover_images = []
        for n in e['hover_frames']:
            Image.open(native / f'hover-{n:03}.png').crop((32,88,608,664)).save(dest / f'hover-{n:03}.png', compress_level=1)
            hover_images.append({'frame':n, 'file':f'hover-{n:03}.png', 'step':timeline[n]['step'], 'led_on':timeline[n]['led_on']})
        for n in e['control_frames']:
            control = Image.open(native / f'control-{n:03}.png').convert('RGB')
            delta = ImageChops.difference(originals[n],control).crop((32,88,608,664))
            for y,row in enumerate(e['map'].splitlines()):
                for x,cell in enumerate(row):
                    if cell=='#': assert delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox() is None,(e['variant'],n,x,y)
        assert timeline[14]['preview_visible'] and not timeline[17]['preview_visible']
        assert timeline[17]['energy']==90 and timeline[17]['cooldown']==1
        placed = next(m for m in timeline[17]['mines'] if m['at']==[4,5])
        assert not placed['armed']
        assert {t['led_on'] for t in timeline[18:62]}=={True,False}
        assert all(not next(m for m in t['mines'] if m['at']==[4,5])['armed'] for t in timeline[18:62])
        assert all(t['player_hp']==100 and t['energy']==90 for t in timeline[17:])
        assert all(t['count']<=t['capacity'] for t in timeline)
        last = timeline[-1]
        if e['quiet']:
            assert last['turn']==1 and last['explosions']==0 and last['hp']==[100]*3 and last['count']==1
        elif e['case']=='COUCHE':
            assert timeline[88]['layer_changed'] and last['count']==0 and last['explosions']==0
        elif e['case']=='ENNEMI-ARMEMENT':
            assert timeline[44]['primary']==[4,5] and timeline[44]['explosions']==0
            assert timeline[64]['explosions']==1 and timeline[64]['count']==0 and last['hp']==[87]*3
        else:
            assert timeline[64]['cooldown']==0 and next(m for m in timeline[64]['mines'] if m['at']==[4,5])['armed']
            assert timeline[88]['explosions']==0 and timeline[112]['explosions']==1 and last['explosions']==1
            assert last['hp']==([87,87,100] if e['case']=='CLOISON' else [87]*3)
        if e['case']=='CAPACITE':
            assert timeline[14]['count']==timeline[14]['capacity']
            assert timeline[17]['removed_oldest']==1 and timeline[17]['count']==timeline[17]['capacity']
            assert [3,6] not in [m['at'] for m in timeline[17]['mines']]
            assert last['count']==last['capacity']-1
        assert timeline[0]['capacity']==(1 if e['id']=='SM00' else 5)
        animations.append(gif(OUT / f'{e["variant"]}-64.gif',frames))
        e.update(timeline=timeline,image_indices=indices,hover_images=hover_images)
        checks.append({'variant':e['variant'],'walls_clear':True,'arming_and_manual_turns':True,'single_explosion':True,'capacity_and_player_protection':True,'led_blinks_before_and_after_arming':True})
        if e['case']=='HAUT':
            overview.extend(originals)
            for n in [14,18,44,54,64,76,112,114,116,120,132,148]:originals[n].save(OUT / f'{e["id"]}-phase-{n:03}.png')
        print(e['variant']+' packaged',flush=True)
    animations.append(gif(OUT / 'mines-ouverture-64.gif',overview))
    dump('verification.json',{'native_views':len(entries),'native_tile_size':64,'resampled':False,'protected_files_unchanged':len(protection),'checks':checks,'animations':animations})
    dump('review.json',{'batch':'ANIM-S30','date':'2026-10-14','status':'awaiting_visual_review','campaign_integration':False,
        'candidates':[{'number':68,'id':'SM00','name':'Pose de mine','variant':'SM00-A01'},{'number':69,'id':'SM01','name':'Réseau de mines','variant':'SM01-A01'}],
        'approved_cards':['review-individual-description-SM00-02.json','review-individual-description-SM01-02.json','mine-arming-trigger-acceptance-decisions.json'],
        'new_user_direction':'Centre LED blinks once placed, explicitly requested during this batch; arming still requires1turn. Supersedes the old visual direction only-once-armed.',
        'mechanics':'Accepted illustrative mine13fire before defenses, radius1, placement1, energy10, cooldown1, arming1. Capacity1 withoutSM01;5 at effective11/12. Consumption once, hostile arrival after placement and presence on arming triggers. Oldest replacement after valid placement, layer cleanup, protected player. No new bonus progression.',
        'visuals':'New illustrated steel ground mine matching acceptedSM00 icon. Central red lens blinks from placement, .75s cycle with7/18 duty. Small placement label; accepted6stage fire burst164px/.42s and .90opacity. No blood or newBurn/residue. Shared victim recoil, total13andfire split. Bright preplacement preview clears on confirmation; hover displays radius and native tooltip. Capacity updates the mine HUD without timed buff.',
        'prepared_numbers':'EnemyHP100 becomes87 once per eligible victim; playerHP100 preserved in blast. Only card sample13, no defenses/variance or full attribute curve. Historical armed mines supplied only in capacity scene; no prior movement/placement timeline inferred.',
        'scope':'Isolated debug native scenes; no campaignGameState, save, AI, RNG, loot or full resource/targeting resolver. Radius1 cardinal fixture is an example, not a universal diagonal/corner/large-footprint convention. Enemy32 art temporarily enlarged64, to adapt after animations.',
        'pending':'Full rank curve above12, resistance/damage/snapshot order, geometry/doors/diagonals/large footprints, familiar and allied damage eligibility, simultaneous arming/chains, rejected-placement atomicity, mixed-types/elemental and residue passives, gear rank reductions, layered inspection, actual battlefield integration. Mine art, blink period, burst and hover/capacity presentation await approval.'})
    write_html(entries);add_comparison(OUT)
    (OUT/'render-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/mines_opening.rs').read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    dump('drawing-pack.json',{'mine':{'source':'mine-source.png','packed':'mine-sprite.png','mode':'built-in imagegen','prompt':'imagegen-mine-prompt.txt','native_visible_width_approx':44,'alpha_preserved':True},'burst':'review20/burst-atlas.png reused unchanged','blink':'code overlay aligned to central generated lens','integration':'review debug scene only'})
    dump('progress-audit.json',{'date':'2026-10-14','previous_unshown':44,'newly_shown':['SM00','SM01'],'remaining_unshown':42,'visual_review_status':'SM00-A01 and SM01-A01 await approval; previous accepted lots preserved.'})
    print(json.dumps({'views':len(entries),'animations':len(animations),'checks':'passed'}))

if __name__=='__main__':package()
