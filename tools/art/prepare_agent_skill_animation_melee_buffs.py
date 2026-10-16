"""ANIM-S21: crop/pack generated jaw frames and package native GPU captures."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_hybrids as converter
from add_browser_animation_comparison import add_comparison
ROOT=base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-50'

def prepare():
    assert not (OUT/'visual-acceptance-01.json').exists()
    with Image.open(OUT/'jaw-source.png') as im:
        assert im.mode=='RGBA' and im.size==(1536,1024)
        assert im.getchannel('A').getextrema()[0]==0
        atlas=Image.new('RGBA',(1536,256))
        for frame in range(6):
            x,y=(frame%3)*512,(frame//3)*512
            cell=im.crop((x,y,x+512,y+512));assert cell.getchannel('A').getbbox()
            atlas.paste(cell.resize((256,256),Image.Resampling.LANCZOS),(frame*256,0))
        atlas.save(OUT/'jaw-atlas.png')
    base.dump(OUT/'drawing-pack.json',{'tool':'Built-in imagegen','postprocessing':'Equal cell crops and direct resize only; alpha preserved, no painting or recoloring.',
        'layout':[3,2],'native_effect_canvas':[92,92],'atlas':[1536,256],
        'source_sha256':hashlib.sha256((OUT/'jaw-source.png').read_bytes()).hexdigest(),
        'reused':'Approved BM01/BM02 compact icons, melee cut, review40 blood, fixed64 player body.'})
    proof={};prior=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-49'
    receipt=json.loads((prior/'visual-acceptance-01.json').read_text(encoding='utf-8'))
    for name,digest in receipt['artifact_hashes'].items():
        path=prior/name;assert hashlib.sha256(path.read_bytes()).hexdigest()==digest;proof[path.relative_to(ROOT).as_posix()]=digest
    for relative in ['assets/skills/agent/BM01-compact.png','assets/skills/agent/BM02-compact.png',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png','artifacts/refonte-agent-2026-10-06/skill-animation-review-39/predator-64.png']:
        proof[relative]=hashlib.sha256((ROOT/relative).read_bytes()).hexdigest()
    base.dump(OUT/'review-protection.json',proof)
    launcher=(prior/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-echange-domination','--lot-buffs-melee')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launcher,encoding='utf-8-sig')
    print(json.dumps({'new_atlases':1,'protected_files':len(proof)}))

def save_animation(path,images,static=False):
    samples=[images[i].resize((384,round(images[i].height*384/images[i].width)),Image.Resampling.NEAREST)
        for offset in range(0,len(images),128) for i in [offset+j for j in [14,17,24,26,28,31,36,53,55,58,88,127]]]
    sheet=Image.new('RGB',(384,sum(p.height for p in samples)));y=0
    for im in samples:sheet.paste(im,(0,y));y+=im.height
    pixels=list(sheet.get_flattened_data());palette=sheet.quantize(colors=126,method=Image.Quantize.MEDIANCUT).getpalette()[:378]+[224,231,231,165,185,197]
    groups=[(64,[p for p in pixels if p[0]>p[1]*1.5 and p[0]>p[2]*1.5]),
        (32,[p for p in pixels if p[1]>p[0]+20 and p[1]>p[2]+20]),
        (16,[p for p in pixels if p[0]>p[1]+20 and p[1]>p[2]+15 and p[0]>120]),
        (16,[p for p in pixels if p[2]>p[1]+12 and p[0]>p[1]+5])]
    for count,selected in groups:
        selected=selected or pixels;strip=Image.new('RGB',(256,(len(selected)+255)//256));strip.putdata(selected)
        palette+=strip.quantize(colors=count,method=Image.Quantize.MEDIANCUT).getpalette()[:count*3]
    pal=Image.new('P',(1,1));pal.putpalette(palette)
    encoded=[im.quantize(palette=pal,dither=Image.Dither.NONE) for im in images]
    bounds=[round(i*1000/24/10)*10 for i in range(len(images)+1)]
    encoded[0].save(path,save_all=True,append_images=encoded[1:],duration=[b-a for a,b in zip(bounds,bounds[1:])],loop=0,disposal=2,optimize=False)
    with Image.open(path) as gif:
        elapsed=0;unique=set()
        for frame in range(gif.n_frames):
            gif.seek(frame);elapsed+=gif.info['duration'];unique.add(hashlib.sha256(gif.convert('RGB').tobytes()).hexdigest())
        assert elapsed==bounds[-1] and (static or len(unique)>10)
    errors=[]
    for frame in [26,28,36]:
        for p,q in zip(images[frame].get_flattened_data(),encoded[frame].convert('RGB').get_flattened_data()):
            if p[2]>p[1]+12 and p[0]>p[1]+5:errors.append(max(abs(a-b) for a,b in zip(p,q)))
    mean=sum(errors)/len(errors) if errors else 0;p95=sorted(errors)[int((len(errors)-1)*.95)] if errors else 0
    assert mean<18 and p95<40,(path.name,mean,p95)
    return {'file':path.name,'native_size':list(images[0].size),'duration_ms':elapsed,'distinct_frames':len(unique),
        'purple_actor_mean_error':round(mean,2),'purple_actor_p95_error':p95}

def package(gifs_only=False):
    assert not (OUT/'visual-acceptance-01.json').exists()
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert all(hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
    frames_root=OUT/'frames';frames_root.mkdir(exist_ok=True);checks=[];animations=[];overview=[]
    for e in entries:
        native=OUT/'native'/e['folder'];timeline=json.loads((native/'timeline.json').read_text(encoding='utf-8'))
        originals=[Image.open(native/f'{n:03}.png').convert('RGB') for n in range(128)]
        frames=[im.crop((32,88,608,664)) for im in originals]
        folder=frames_root/e['variant'];folder.mkdir(exist_ok=True)
        if not gifs_only:
            for n,im in enumerate(frames):im.save(folder/f'{n:03}.png',compress_level=1)
        deltas={}
        for f in e['control_frames']:
            control=Image.open(native/f'control-{f:03}.png').convert('RGB')
            delta=ImageChops.difference(originals[f],control).crop((32,88,608,664));deltas[f]=delta
            wall_delta=delta.copy()
            # The cast-name HUD is deliberately above terrain (like damage text).
            # On the adjacent-wall fixture its label crosses the wall, whereas
            # the jaw, blood, weapon stroke and actor recoil are still clipped.
            # Exclude ONLY that neutral label, at its sole sampled visible instant.
            hud_excluded=e['case']=='CLOISON' and f==17
            if hud_excluded:
                wall_delta.paste((0,0,0),(216,224,436,249))
            for y,row in enumerate(e['map'].splitlines()):
                for x,v in enumerate(row):
                    if v=='#':assert wall_delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox() is None,(e['variant'],f,x,y)
        assert timeline[17]['launched']==e['permitted']
        assert timeline[17]['preview_visible']==False
        if not e['permitted']:
            assert all(d.getbbox() is None for d in deltas.values())
            assert timeline[-1]['hits']==0 and timeline[17]['cooldown']==0
        else:
            for f,row in enumerate(timeline[17:],17):
                turns=sum(t<=f for t in e['turn_frames'])
                assert row['turn']==turns and row['remaining']==max(0,e['duration']-turns)
                assert row['cooldown']==max(0,e['cooldown']-turns)
                assert row['icon_visible']==(row['remaining']>0)
            assert all(deltas[f].getbbox() is None for f in [47,76,88,112,127])
            if e['strike_permitted']:
                assert deltas[26].getbbox() is not None and timeline[24]['hits']==1
                assert timeline[24]['fire']==(14 if e['id']=='BM01' else 0)
                assert timeline[24]['heal']==(8 if e['id']=='BM02' else 0)
                assert timeline[-1]['hits']==(1 if e['quiet'] else 2)
            else:assert timeline[-1]['hits']==0
            if e['quiet']:assert timeline[-1]['remaining']==e['duration']
            else:assert timeline[-1]['remaining']==0
        checks.append({'variant':e['variant'],'world_vfx_walls_clear':True,'neutral_cast_name_hud_excluded_from_wall_comparison':e['case']=='CLOISON','turn_timers_and_icon_expiry_checked':True,
            'quiet_has_no_replayed_impact':True,'prepared_integer_damage_and_heal':True,'preview_ends_on_launch':True})
        animations.append(save_animation(OUT/f"{e['variant']}-64.gif",frames,static=not e['permitted']))
        if e['case']=='HAUT':
            overview.extend(originals)
            if not gifs_only:
                for f in [14,17,24,26,28,31,36,47,53,55,58,88,127]:originals[f].save(OUT/f"{e['id']}-phase-{f:03}.png")
    animations.append(save_animation(OUT/'buffs-melee-64.gif',overview))
    base.dump(OUT/'verification.json',{'native_views':len(entries),'controls':checks,'animations':animations,
        'protected_files_unchanged':len(proof),'resampling_of_native_capture':False})
    base.dump(OUT/'review.json',{'batch':'ANIM-S21','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':51,'id':'BM01','name':'Lame incendiaire','variant':'BM01-A01'},{'number':52,'id':'BM02','name':'Faim vampirique','variant':'BM02-A01'}],
        'campaign_integration':False,'visuals':'Persistent compact buff icons with turn timers and tooltips; colored damage components without weapon flame overlays; brief vampire jaw on each prepared successful strike, common recoil and physical wound blood.',
        'approved_card_values':'BM01 rank11 +14fire/duration6/cost11/CD6. BM02 rank11 life40%/sharedlimit10%maxHP/duration4/cost13/CD12.',
        'fixture_numbers':'20 physical HP removed by a prepared melee strike, target has no armor/resistance in this fixture. BM01 displays34 (20physical+14fire); BM02 displays+8HP for40% of20, chosen missingHP/turnbudget sufficient. Not actual weapon/monster stats, no variance, rounding, overkill or healing solver introduced.',
        'pending_cases':'Miss, fully absorbed hit, full health, exhausted heal cap, multi-hit/dual-wield and transformed natural attack trigger details remain separately pending.',
        'turn_fixture':'First strike atframe24, explicit turn1 at52 then second strike53. Remaining turns explicit52/76/88/100/112/124. Casting/strike action resolution order not established by fixture playback.',
        'temporary_actors':'Existing enemy32px art enlarged to64; bestiary adaptation scheduled after animations.'})
    write_html(entries)
    add_comparison(OUT)
    for source,name in [('src/agent_skill_lab_review/fire_painted/melee_buffs.rs','render-source.rs'),('src/combat_visuals.rs','reaction-source.rs')]:
        (OUT/name).write_bytes((ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'native_views':len(entries),'animations':len(animations),'protected_files':len(proof),'controls':'passed'}))

def write_html(entries):
    manifest=[{k:e[k] for k in ['case','variant','id','title','permitted','quiet','cooldown','duration','strike_permitted']} for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(!e.permitted)return 14;
const step=Math.floor(elapsed*24/1000);if(turn===0){if(step<27)return 17+step;return 44+step%8;}
const base=[52,76,88,100,112,124][Math.min(5,turn-1)];if(turn===1){if(step<21)return 52+step;return 73+step%3;}if(turn>=6)return 124+step%4;return base+step%12;}
function ui(){const es=entries();next.disabled=!ready||!launched||!es.some(e=>e.permitted)||es.every(e=>e.quiet)||turn>=6;cast.disabled=!ready||launched||!es.some(e=>e.permitted);status.textContent=!ready?'Chargement…':!launched?'Aperçu clignotant · confirmer pour lancer':'Tour '+turn+' · '+es.map(e=>e.title+(e.permitted?' : buff '+Math.max(0,e.duration-turn)+' tours · recharge '+Math.max(0,e.cooldown-turn):' : indisponible')).join(' · ');}
async function load(){ready=false;ui();const n=++token,es=entries();const data=await Promise.all(es.map(async e=>[e.id,await Promise.all(Array.from({length:128},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src='frames/'+e.variant+'/'+String(i).padStart(3,'0')+'.png';})))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===canvas.dataset.skill);canvas.parentElement.hidden=!e;if(e)canvas.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
for(const canvas of document.querySelectorAll('canvas')){canvas.onmousemove=ev=>{const e=entries().find(e=>e.id===canvas.dataset.skill),box=canvas.getBoundingClientRect(),x=ev.clientX-box.left,y=ev.clientY-box.top,tip=canvas.parentElement.querySelector('.tip');const left=e?Math.max(0,e.duration-turn):0;tip.hidden=!(ready&&launched&&e?.permitted&&left&&x>=488&&x<=526&&y>=66&&y<=104);if(!tip.hidden)tip.textContent=e.title+' · '+(e.id==='BM01'?'+14 dégâts de feu / frappe':'40 % de vie rendue · limite 10 % des PV max / tour')+' · '+left+' tours restants';};canvas.onmouseleave=()=>canvas.parentElement.querySelector('.tip').hidden=true;}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};
document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    labels={'HAUT':'Vers le haut','DROITE':'Vers la droite','GAUCHE':'Vers la gauche','BAS':'Vers le bas','ATTENTE':'Attente sans tour','CLOISON':'Mur devant la frappe','CACHE':'Cible cachée','SILENCE':'Silence','DISTANCE':'Arme à distance active','TRANSFORME':'Forme transformée','SURVOL':'Survol du buff'}
    options=''.join(f'<option value="{k}">{v}</option>' for k,v in labels.items())
    cards=''.join(f'<article><h2>{n} · {title}</h2><p class="muted">{sid}-A01 · proposé</p><canvas data-skill="{sid}" width="576" height="576"></canvas><div class="tip" hidden></div></article>' for n,sid,title in [(51,'BM01','Lame incendiaire'),(52,'BM02','Faim vampirique')])
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Buffs de mêlée</title><style>*{box-sizing:border-box}body{margin:auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{position:relative;padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}.tip{position:absolute;right:20px;top:214px;width:270px;padding:12px;background:#0b1218;border:1px solid #6c7d88;pointer-events:none}a{color:#93c1e0}[hidden]{display:none}@media(max-width:1296px){.grid{grid-template-columns:608px}}</style>
<h1>Lame incendiaire et Faim vampirique</h1><p class="muted">ANIM-S21 · candidats51–52 · cases natives64px · rendu à valider</p><div class="controls"><select id="scenario">OPTIONS</select><button id="next">+1 tour</button><button id="cast">Lancer le buff</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="grid">CARDS</main>
<p>Les icônes de buff persistent jusqu’à leur expiration en tours. Survolez-les pour voir le nom, l’effet et la durée restante. Une première frappe suit le lancement ; le premier tour manuel montre une seconde frappe. Les impacts s’arrêtent pendant l’attente.</p>
<p>Lame incendiaire colore la composante feu des dégâts affichés : aucune flamme ajoutée à l’arme, aucun statut Brûlure. Faim vampirique referme une mâchoire sur la cible et affiche le soin sur l’agent. Les blessures physiques gardent leur sang et la réaction commune.</p>
<p class="muted">Valeurs de frappe préparées :20 dégâts physiques, sans défense dans cette scène. Le +14 feu et les40 % de vie rendue reprennent le rang11 validé ; ce prototype n’ajoute aucune statistique d’arme ou formule d’équilibrage. Les cas de soin nul, miss, absorption et plafond atteint restent à définir. Les monstres32px agrandis seront adaptés après les animations.</p>
<p><a href="Ouvrir_laboratoire.ps1">Laboratoire natif (Lancer, Frapper, tours manuels)</a> · <a href="buffs-melee-64.gif">Aperçu animé</a> · <a href="jaw-source.png">Planche source — imagegen intégré</a> · <a href="jaw-prompt.txt">Prompt</a> · <a href="review.json">Périmètre de la revue</a> · <a href="verification.json">Vérifications</a></p><script>SCRIPT</script></html>'''.replace('OPTIONS',options).replace('CARDS',cards).replace('SCRIPT',script)
    for before,after in [('candidats51–52','candidats 51–52'),('natives64px','natives 64 px'),
        ('préparées :20','préparées : 20'),('les40 %','les 40 %'),('rang11','rang 11'),('monstres32px','monstres 32 px')]:
        html=html.replace(before,after)
    (OUT/'index.html').write_text(html,encoding='utf-8')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--prepare',action='store_true');p.add_argument('--convert-stream',action='store_true');p.add_argument('--gifs-only',action='store_true');a=p.parse_args()
    if a.prepare:prepare()
    elif a.convert_stream:converter.OUT=OUT;converter.convert()
    else:package(gifs_only=a.gifs_only)
