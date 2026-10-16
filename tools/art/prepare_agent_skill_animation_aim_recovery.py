"""ANIM-S23: prepare and package real native passive-feedback captures."""
import argparse, hashlib, json
from pathlib import Path
from PIL import Image, ImageChops
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_hybrids as converter
from add_browser_animation_comparison import add_comparison
ROOT=base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-52'
FRAMES=128

def prepare():
    assert not OUT.exists();OUT.mkdir()
    prior=OUT.with_name('skill-animation-review-51');record=json.loads((prior/'visual-acceptance-01.json').read_text(encoding='utf8'));proof={}
    for name,digest in record['artifact_hashes'].items():
        p=prior/name;assert hashlib.sha256(p.read_bytes()).hexdigest()==digest;proof[p.relative_to(ROOT).as_posix()]=digest
    reuse=['assets/skills/agent/BD04-compact.png','assets/skills/agent/BD06-compact.png','artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png']
    for name in reuse:proof[name]=hashlib.sha256((ROOT/name).read_bytes()).hexdigest()
    base.dump(OUT/'review-protection.json',proof)
    base.dump(OUT/'drawing-pack.json',{'new_raster_generation':False,'sources_reused_unchanged':reuse,'new_code_visuals':'Outlined red tightening reticle, short steel streak, violet energy spark traveling to HUD; no character or weapon redraw.'})
    (OUT/'Ouvrir_laboratoire.ps1').write_text((prior/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-tempo-vindicte','--lot-visee-energie'),encoding='utf-8-sig')
    print(json.dumps({'protected':len(proof),'prepared':str(OUT)}))

def gif(path,images):
    samples=[images[i].resize((320,round(images[i].height*320/images[i].width)),Image.Resampling.NEAREST) for offset in range(0,len(images),FRAMES) for i in [offset+n for n in [14,28,31,38,52,76,100,110,126]]]
    sheet=Image.new('RGB',(320,sum(im.height for im in samples)));y=0
    for im in samples:sheet.paste(im,(0,y));y+=im.height
    pixels=list(sheet.get_flattened_data());pal=sheet.quantize(colors=126,method=Image.Quantize.MEDIANCUT).getpalette()[:378]+[224,231,231,165,185,197]
    for count,chosen in [(64,[p for p in pixels if p[0]>p[1]*1.4 and p[0]>p[2]*1.4]),(64,[p for p in pixels if p[2]>p[1]+15 and p[2]>p[0]+10])]:
        chosen=chosen or pixels;strip=Image.new('RGB',(256,(len(chosen)+255)//256));strip.putdata(chosen);pal+=strip.quantize(colors=count,method=Image.Quantize.MEDIANCUT).getpalette()[:count*3]
    palette=Image.new('P',(1,1));palette.putpalette(pal)
    cached={};encoded=[]
    for im in images:
        key=hashlib.sha256(im.tobytes()).hexdigest()
        if key not in cached:cached[key]=im.quantize(palette=palette,dither=Image.Dither.NONE)
        encoded.append(cached[key])
    times=[round(i*1000/24/10)*10 for i in range(len(images)+1)]
    encoded[0].save(path,save_all=True,append_images=encoded[1:],duration=[b-a for a,b in zip(times,times[1:])],loop=0,disposal=2,optimize=False)
    with Image.open(path)as decoded:
        total=0;distinct=set()
        for n in range(decoded.n_frames):decoded.seek(n);total+=decoded.info['duration'];distinct.add(hashlib.sha256(decoded.convert('RGB').tobytes()).hexdigest())
        assert total==times[-1]
    return {'file':path.name,'size':list(images[0].size),'duration_ms':total,'distinct_frames':len(distinct)}

def package(refresh_menu=False):
    assert not (OUT/'visual-acceptance-01.json').exists()
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h for n,h in proof.items())
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'));checks=[];animations=[];overview=[]
    previous={a['file']:a for a in json.loads((OUT/'verification.json').read_text(encoding='utf8'))['animations']}if refresh_menu else {}
    for e in entries:
        directory=OUT/'native'/e['folder'];timeline=json.loads((directory/'timeline.json').read_text(encoding='utf8'))
        originals=[Image.open(directory/f'{n:03}.png').convert('RGB')for n in range(FRAMES)];frames=[im.crop((32,88,608,664))for im in originals]
        destination=OUT/'frames'/e['variant'];destination.mkdir(parents=True,exist_ok=True)
        canonical={};indices=[]
        for n,im in enumerate(frames):
            if refresh_menu and e['case']!='SANS-TOUR':
                assert Image.open(destination/f'{n:03}.png').convert('RGB').tobytes()==im.tobytes()
            else:im.save(destination/f'{n:03}.png',compress_level=1)
            digest=hashlib.sha256(im.tobytes()).hexdigest();canonical.setdefault(digest,n);indices.append(canonical[digest])
        for n in e['control_frames']:
            control=Image.open(directory/f'control-{n:03}.png').convert('RGB');delta=ImageChops.difference(originals[n],control).crop((32,88,608,664))
            for y,row in enumerate(e['map'].splitlines()):
                for x,v in enumerate(row):
                    if v=='#':assert delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox()is None,(e['variant'],n,x,y)
        assert timeline[14]['begun']==False and timeline[17]['begun']==True
        if e['id']=='BD04':
            assert timeline[38]['charges']==1 and timeline[58]['charges']==(1 if e['quiet']else 2)
            expected=1 if e['quiet']else 0 if e['case']in ['DEPLACEMENT','FORCE','MAGIQUE']else 3
            assert timeline[-1]['charges']==expected
            assert all(t['charges']<=3 and t['icon_visible']==(t['charges']>0)for t in timeline)
            if e['case']=='SANS-TOUR':assert timeline[-1]['turn']==3
        else:
            excluded=e['case']in ['FAMILIER','RENVOI','RIPOSTE','LEURRE','CLOISON','CACHE','PLEINE']
            initial=100 if e['case']=='PLEINE'else 95 if e['case']=='LIMITE'else 40
            assert timeline[38]['energy']==initial if excluded else timeline[38]['energy']==min(100,initial+22)
            assert timeline[-1]['energy']==initial if excluded else timeline[-1]['energy']==(100 if initial==95 else 62 if e['quiet']or e['case']=='SANS-TOUR'else 84)
            assert all(not t['icon_visible'] and t['energy']<=100 for t in timeline)
            if not excluded and not e['quiet']:
                assert timeline[58]['gained']==0 and timeline[82]['gained']==0
            if e['case']=='SANS-TOUR':assert timeline[-1]['dead'] and timeline[-1]['turn']==3
        checks.append({'variant':e['variant'],'world_effects_and_recoil_blocked_by_walls':True,'HUD_transfer_separate_from_damage':True,'passive_counts_and_resource_limits':True,'no_idle_actions_or_artificial_buff_timer':True})
        filename=f"{e['variant']}-64.gif"
        animations.append(previous[filename]if refresh_menu and e['case']!='SANS-TOUR'else gif(OUT/filename,frames));e.update(timeline=timeline,image_indices=indices)
        print(e['variant']+' packaged',flush=True)
        if e['case']=='HAUT':
            overview.extend(originals)
            for n in [14,28,31,34,38,58,82,100,110,126]:originals[n].save(OUT/f"{e['id']}-phase-{n:03}.png")
    animations.append(previous['visee-energie-64.gif']if refresh_menu else gif(OUT/'visee-energie-64.gif',overview))
    base.dump(OUT/'verification.json',{'native_views':len(entries),'controls':checks,'animations':animations,'protected_files_unchanged':len(proof),'resampling_of_native_capture':False})
    base.dump(OUT/'review.json',{'batch':'ANIM-S23','status':'awaiting_visual_review','native_tile_size':64,'campaign_integration':False,
        'candidates':[{'number':55,'id':'BD04','name':'Visée stable','variant':'BD04-A01'},{'number':56,'id':'BD06','name':'Récupération d’énergie','variant':'BD06-A01'}],
        'approved_cards':['review-individual-description-BD04-02.json','review-individual-description-BD06-04.json'],
        'approved_values':'BD04 +15% ranged precision per stationary turn, maximum3 charges (+45%). BD06 22% max energy per eligible kill, interval3 turns.',
        'visuals':'Outlined red tightening reticle, charge icon and hover; short violet transfer spark and numerical energy gain at gauge, no timed recovery buff. Existing physical wound blood and shared recoil. No damage or proc just from waiting.',
        'prepared_numbers':'20 damage and100 maximum energy are fixtures only. Energy40 to62 then84, or95 to100. No real weapon/monster/stat curve, hit chance, variance or rounding introduced.',
        'scope':'Two independent prepared passive scenes. HUD resource transfer deliberately reaches the gauge outside the world damage mask. Excluded-source deaths are prepared without a player-fired projectile; no familiar AI or reflection resolver added.',
        'pending':'Actual precision computation and before/after action order, exact weapon/spell/heavy/form eligibility, per-action multi-kill/full-energy interval cases, persistent kill attribution and dense combat composition remain outside this visual review.',
        'temporary_actors':'Existing enemy32px art enlarged to64; adaptation after animations.',
        'excluded_draft':'native-feedback-draft and native-menu-draft are unpresented intermediate captures; exclude both from visual acceptance.'})
    write_html(entries);add_comparison(OUT)
    (OUT/'render-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/aim_recovery.rs').read_bytes());(OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'views':len(entries),'animations':len(animations),'protected':len(proof),'checks':'passed'}))

def write_html(entries):
    manifest=[{k:e[k]for k in ['case','variant','id','title','permitted','quiet','image_indices','timeline']}for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){if(!launched)return 3+Math.floor((elapsed%600)*12/600);const step=Math.floor(elapsed*24/1000);const base=[17,48,72,96][turn],length=turn===0?27:20;if(step<length)return base+step;return base+length+step%4;}
function ui(){const es=entries();next.disabled=!ready||!launched||es.every(e=>e.quiet)||turn>=3;cast.disabled=!ready||launched;status.textContent=!ready?'Chargement…':!launched?'Aperçu clignotant · confirmer pour commencer':'Étape '+turn+' · '+es.map(e=>{const t=e.timeline[frame(e)];return e.title+(e.id==='BD04'?' : '+t.charges+' cumuls · précision +'+t.bonus_percent+' %':' : énergie '+t.energy+'/100 · intervalle '+t.interval+' tours');}).join(' · ');}
async function load(){ready=false;ui();const n=++token,cache=new Map(),data=await Promise.all(entries().map(async e=>[e.id,await Promise.all(Array.from({length:128},(_,i)=>{const src='frames/'+e.variant+'/'+String(e.image_indices[i]).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=src;}));return cache.get(src);}))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const c of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===c.dataset.skill);c.parentElement.hidden=!e;if(e)c.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
for(const canvas of document.querySelectorAll('canvas')){canvas.onmousemove=ev=>{const e=entries().find(e=>e.id===canvas.dataset.skill),box=canvas.getBoundingClientRect(),x=ev.clientX-box.left,y=ev.clientY-box.top,tip=canvas.parentElement.querySelector('.tip'),t=e?.timeline[frame(e)];tip.hidden=!(ready&&launched&&e.id==='BD04'&&t?.icon_visible&&x>=488&&x<=526&&y>=66&&y<=104);if(!tip.hidden)tip.textContent=e.title+' · '+t.charges+'/3 cumuls · précision +'+t.bonus_percent+' %';};canvas.onmouseleave=()=>canvas.parentElement.querySelector('.tip').hidden=true;}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest,separators=(',',':')))
    labels={'HAUT':'Vers le haut','DROITE':'Vers la droite','GAUCHE':'Vers la gauche','BAS':'Vers le bas','ATTENTE':'Attente sans action','CLOISON':'Obstacle','CACHE':'Cible cachée','SURVOL':'Survol des buffs','SANS-TOUR':'Menu sans tour','DEPLACEMENT':'Déplacement','FORCE':'Déplacement forcé','MAGIQUE':'Changement magique de case','PLEINE':'Énergie pleine','LIMITE':'Énergie presque pleine','FAMILIER':'Élimination par un familier','RENVOI':'Renvoi exclu','RIPOSTE':'Riposte automatique exclue','LEURRE':'Leurre exclu'}
    options=''.join(f'<option value="{k}">{v}</option>'for k,v in labels.items());cards=''.join(f'<article><h2>{n} · {title}</h2><p class="muted">{sid}-A01 · proposé</p><canvas data-skill="{sid}" width="576" height="576"></canvas><div class="tip" hidden></div></article>'for n,sid,title in [(55,'BD04','Visée stable'),(56,'BD06','Récupération d’énergie')])
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Visée et énergie</title><style>*{box-sizing:border-box}body{margin:auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{position:relative;padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}.tip{position:absolute;right:20px;top:214px;width:270px;padding:12px;background:#0b1218;border:1px solid #6c7d88;pointer-events:none}a{color:#93c1e0}[hidden]{display:none}@media(max-width:1296px){.grid{grid-template-columns:608px}}</style><h1>Visée stable et Récupération d’énergie</h1><p class="muted">ANIM-S23 · candidats 55–56 · cases natives 64 px · rendu à valider</p><div class="controls"><select id="scenario">OPTIONS</select><button id="next">Action suivante</button><button id="cast">Commencer l’aperçu</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="grid">CARDS</main>
<p>Visée stable : le viseur rouge se resserre avec les trois cumuls. Le bonus de précision apparaît au survol de l’icône, sans timer. Les déplacements le suppriment ; un menu sans tour le conserve.</p>
<p>Récupération d’énergie : une étincelle rejoint la jauge depuis la cible vaincue ; le gain apparaît près de la jauge. Avec 100 énergie maximale dans cet exemple, le gain est de 22, puis de nouveau 22 après trois tours. Avec 95/100, seulement 5 sont récupérés. Il n’y a pas de buff temporisé.</p>
<p class="muted">Deux scènes indépendantes et des victimes préparées à chaque action pour montrer les éliminations. Les chiffres ne définissent aucun équilibrage. Les cas d’élimination par un familier, de renvoi, de riposte et de leurre n’ajoutent pas d’énergie. Les animations de ces sources ne sont pas simulées. Monstres 32 px temporaires ; adaptation après les animations.</p>
<p><a href="visee-energie-64.gif">Aperçu animé</a> · <a href="review.json">Périmètre</a> · <a href="verification.json">Vérifications</a></p><script>SCRIPT</script></html>'''.replace('OPTIONS',options).replace('CARDS',cards).replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf8')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--prepare',action='store_true');p.add_argument('--convert-stream',action='store_true');p.add_argument('--refresh-menu',action='store_true');args=p.parse_args()
    if args.prepare:prepare()
    elif args.convert_stream:converter.OUT=OUT;converter.convert()
    else:package(refresh_menu=args.refresh_menu)
