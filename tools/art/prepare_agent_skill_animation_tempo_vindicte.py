"""ANIM-S22: native charge/reflection review using existing accepted drawings."""
import argparse,hashlib,json
from pathlib import Path
from PIL import Image,ImageChops
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_hybrids as converter
from add_browser_animation_comparison import add_comparison
ROOT=base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-51'

def prepare():
    assert not OUT.exists();OUT.mkdir()
    prior=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-50';receipt=json.loads((prior/'visual-acceptance-01.json').read_text(encoding='utf8'));proof={}
    for name,digest in receipt['artifact_hashes'].items():
        p=prior/name;assert hashlib.sha256(p.read_bytes()).hexdigest()==digest;proof[p.relative_to(ROOT).as_posix()]=digest
    reuse=['assets/skills/agent/BM03-compact.png','assets/skills/agent/BM05-compact.png',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-32/bolt-atlas.png',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-39/predator-64.png']
    for f in reuse:proof[f]=hashlib.sha256((ROOT/f).read_bytes()).hexdigest()
    base.dump(OUT/'review-protection.json',proof)
    base.dump(OUT/'drawing-pack.json',{'new_raster_generation':False,'sources_reused_unchanged':reuse,
        'palette':'Existing violet-franc shader contrast3; applies only to spell VFX.',
        'motion':'Same projectile image and scale72, opposite directions on incoming and return; short54px impacts.',
        'tool':'Macroquad native render; mechanical PNG crop and GIF encoding only.'})
    (OUT/'Ouvrir_laboratoire.ps1').write_text((prior/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-buffs-melee','--lot-tempo-vindicte'),encoding='utf-8-sig')
    print(json.dumps({'protected_files':len(proof),'reused_drawings':len(reuse)}))

def save_gif(path,images,static=False):
    samples=[images[i].resize((384,round(images[i].height*384/images[i].width)),Image.Resampling.NEAREST) for offset in range(0,len(images),192) for i in [offset+j for j in [14,17,24,26,29,31,34,38,53,58,77,110,174]]]
    sheet=Image.new('RGB',(384,sum(i.height for i in samples)));y=0
    for im in samples:sheet.paste(im,(0,y));y+=im.height
    pixels=list(sheet.get_flattened_data());palette=sheet.quantize(colors=126,method=Image.Quantize.MEDIANCUT).getpalette()[:378]+[224,231,231,165,185,197]
    for count,chosen in [(48,[p for p in pixels if p[0]>p[1]*1.25 and p[0]>p[2]*1.5]),(64,[p for p in pixels if p[2]>p[1]+12 and p[0]>p[1]+5]),(16,[p for p in pixels if p[1]>p[0]+2 and p[1]>p[2]+8])]:
        chosen=chosen or pixels;strip=Image.new('RGB',(256,(len(chosen)+255)//256));strip.putdata(chosen);palette+=strip.quantize(colors=count,method=Image.Quantize.MEDIANCUT).getpalette()[:count*3]
    pal=Image.new('P',(1,1));pal.putpalette(palette);frames=[im.quantize(palette=pal,dither=Image.Dither.NONE) for im in images]
    bounds=[round(i*1000/24/10)*10 for i in range(len(images)+1)]
    frames[0].save(path,save_all=True,append_images=frames[1:],duration=[b-a for a,b in zip(bounds,bounds[1:])],loop=0,disposal=2,optimize=False)
    with Image.open(path) as gif:
        elapsed=0;hashes=set()
        for f in range(gif.n_frames):gif.seek(f);elapsed+=gif.info['duration'];hashes.add(hashlib.sha256(gif.convert('RGB').tobytes()).hexdigest())
        assert elapsed==bounds[-1] and (static or len(hashes)>10)
    errors=[]
    for f in [26,29,31,34]:
        for a,b in zip(images[f].get_flattened_data(),frames[f].convert('RGB').get_flattened_data()):
            if a[2]>a[1]+12 and a[0]>a[1]+5:errors.append(max(abs(x-y) for x,y in zip(a,b)))
    mean=sum(errors)/len(errors) if errors else 0;p95=sorted(errors)[int((len(errors)-1)*.95)] if errors else 0
    assert mean<18 and p95<40,(path.name,mean,p95)
    return {'file':path.name,'size':list(images[0].size),'duration_ms':elapsed,'distinct_frames':len(hashes),'purple_mean_error':round(mean,2),'purple_p95_error':p95}

def package(reuse_frames=False):
    assert not (OUT/'visual-acceptance-01.json').exists()
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'));proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h for n,h in proof.items())
    frames_root=OUT/'frames';frames_root.mkdir(exist_ok=True);checks=[];animations=[];overview=[]
    for e in entries:
        native=OUT/'native'/e['folder'];timeline=json.loads((native/'timeline.json').read_text(encoding='utf8'));originals=[Image.open(native/f'{f:03}.png').convert('RGB') for f in range(192)]
        frames=[im.crop((32,88,608,664)) for im in originals];folder=frames_root/e['variant'];folder.mkdir(exist_ok=True)
        if not reuse_frames or not all((folder/f'{f:03}.png').exists() for f in range(192)):
            for f,im in enumerate(frames):im.save(folder/f'{f:03}.png',compress_level=1)
        deltas={}
        for f in e['control_frames']:
            control=Image.open(native/f'control-{f:03}.png').convert('RGB');d=ImageChops.difference(originals[f],control).crop((32,88,608,664));deltas[f]=d
            for y,row in enumerate(e['map'].splitlines()):
                for x,v in enumerate(row):
                    if v=='#':assert d.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox() is None,(e['variant'],f,x,y)
        assert timeline[17]['begun']==e['permitted']
        if e['permitted'] and e['case'] not in ['CLOISON','CACHE','POISON','INDIRECT']:
            assert deltas[26 if e['id']=='BM03' else 29].getbbox() is not None
            if e['id']=='BM05' and e['case']!='MELEE':assert deltas[31].getbbox() is not None
        if not e['permitted']:assert timeline[-1]['hits']==0 and all(d.getbbox() is None for d in deltas.values())
        elif e['id']=='BM03':
            assert all(t['charges']<=3 and t['icon_visible']==(t['charges']>0) for t in timeline)
            if e['case'] not in ['CLOISON','CACHE']:
                assert timeline[24]['charges']==1 and timeline[24]['damage']==20
                if e['quiet']:assert timeline[-1]['charges']==1 and timeline[-1]['turn']==0
                else:
                    assert timeline[48]['damage']==25 and timeline[48]['charges']==2
                    if e['case'] in ['MANQUE','DEPLACEMENT','AUTRE-ACTION']:assert timeline[72]['charges']==0
                    elif e['case']=='SANS-TOUR':assert timeline[72]['charges']==2 and timeline[72]['turn']==1
                    else:assert timeline[72]['charges']==3 and timeline[96]['damage']==35
        else:
            for f,t in enumerate(timeline[17:],17):
                turns=sum(k<=f for k in e['action_frames']);assert t['remaining']==max(0,6-turns) and t['cooldown']==max(0,8-turns)
                assert t['icon_visible']==(t['remaining']>0)
            if e['case'] not in ['CLOISON','CACHE']:
                assert timeline[24]['damage']==20 and timeline[24]['returned']==(0 if e['case'] in ['POISON','INDIRECT','MELEE'] else 8)
                if not e['quiet']:assert timeline[168]['returned']==0 and timeline[168]['damage']==20
            if e['quiet']:assert timeline[-1]['remaining']==6
        assert deltas[44].getbbox() is None and deltas[68].getbbox() is None and deltas[190].getbbox() is None
        checks.append({'variant':e['variant'],'world_walls_clear':True,'waiting_does_not_replay_impact':True,'charges_or_turn_duration_checked':True,'received_damage_retained_and_exclusions_checked':e['id']=='BM05','preview_ends_on_start':True})
        animations.append(save_gif(OUT/f"{e['variant']}-64.gif",frames,static=not e['permitted'] or e['case'] in ['CLOISON','CACHE']))
        e['timeline']=timeline
        canonical={};indices=[]
        for f,im in enumerate(frames):
            digest=hashlib.sha256(im.tobytes()).hexdigest();canonical.setdefault(digest,f);indices.append(canonical[digest])
        e['image_indices']=indices
        if e['case']=='HAUT':
            overview.extend(originals)
            for f in [14,17,24,26,29,31,34,38,48,58,72,96,168,174,190]:originals[f].save(OUT/f"{e['id']}-phase-{f:03}.png")
    animations.append(save_gif(OUT/'tempo-vindicte-64.gif',overview))
    base.dump(OUT/'verification.json',{'native_views':len(entries),'controls':checks,'animations':animations,'protected_files_unchanged':len(proof),'resampling_of_native_capture':False})
    base.dump(OUT/'review.json',{'batch':'ANIM-S22','status':'awaiting_visual_review','native_tile_size':64,'campaign_integration':False,
        'candidates':[{'number':53,'id':'BM03','name':'Tempo offensif','variant':'BM03-A01'},{'number':54,'id':'BM05','name':'Sceau de vindicte','variant':'BM05-A01'}],
        'approved_values':'BM03 +25% per charge, max3, max75%. BM05 direct magic return40% of damage suffered, duration6, energy13, cooldown8.',
        'prepared_numbers':'Base20 physical gives20/25/30/35 on consecutive hits, with0/5/10/15 included bonus. Incoming20 direct occult damage after prepared protections gives8 returned. These are illustrated fixtures, not weapon/monster stats or a new global variance/defense/rounding formula.',
        'visuals':'Compact buff icons, automatic charge count and hover bonus without a timer; original occult projectile turns back with unchanged appearance; brief shared harmful reaction; no magical blood.',
        'cases':'Open four directions, quiet waiting, walls/hidden contact, active ranged weapon and Silence; miss/move/other action reset series; no-time menu preserves; poison/indirect/melee do not reflect; human-form casting restriction; six-turn expiry still allows prepared damage without return.',
        'pending':'Mixed elements, multi-hit/target, absorption/overkill/lethal damage, reflected victim defenses/recursive reflection, hidden/offscreen author eligibility, transformation compatibility for Tempo, and dense-combat composition not resolved by this visual review.',
        'turn_scope':'Steps are independent prepared actions per panel, no new campaign action resolution order. Tempo panel rests after fourth strike without a time-consuming wait. Menu stage consumes no game turn. BM05 hidden-author scene has no prepared incoming hit and establishes no new visibility restriction on reactive return.',
        'temporary_actors':'Existing enemy32px art enlarged to64; bestiary adaptation scheduled after animations.'})
    write_html(entries);add_comparison(OUT)
    for source,name in [('src/agent_skill_lab_review/fire_painted/tempo_vindicte.rs','render-source.rs'),('src/combat_visuals.rs','reaction-source.rs')]: (OUT/name).write_bytes((ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes());print(json.dumps({'views':len(entries),'animations':len(animations),'protected':len(proof),'checks':'passed'}))

def write_html(entries):
    manifest=[]
    for e in entries:
        item={k:e[k] for k in ['case','variant','id','title','permitted','quiet','image_indices']};item['timeline']=e['timeline'];manifest.append(item)
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(!e.permitted)return 14;const step=Math.floor(elapsed*24/1000);if(turn===0){if(step<27)return 17+step;return 44+step%4;}const base=[48,72,96,120,144,168][Math.min(5,turn-1)];if(step<20)return base+step;return base+20+step%4;}
function ui(){const es=entries();next.disabled=!ready||!launched||!es.some(e=>e.permitted)||es.every(e=>e.quiet)||turn>=6;cast.disabled=!ready||launched||!es.some(e=>e.permitted);status.textContent=!ready?'Chargement…':!launched?'Aperçu clignotant · confirmer pour commencer':'Étape '+turn+' · '+es.map(e=>{const t=e.timeline[frame(e)];return e.title+(e.permitted?(e.id==='BM03'?' : '+t.charges+' charges · +'+t.bonus_percent+' %':' : sceau '+t.remaining+' tours · recharge '+t.cooldown):' : indisponible');}).join(' · ');}
async function load(){ready=false;ui();const n=++token,cache=new Map(),data=await Promise.all(entries().map(async e=>[e.id,await Promise.all(Array.from({length:192},(_,i)=>{const src='frames/'+e.variant+'/'+String(e.image_indices[i]).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=src;}));return cache.get(src);}))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const c of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===c.dataset.skill);c.parentElement.hidden=!e;if(e)c.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
for(const canvas of document.querySelectorAll('canvas')){canvas.onmousemove=ev=>{const e=entries().find(e=>e.id===canvas.dataset.skill),box=canvas.getBoundingClientRect(),x=ev.clientX-box.left,y=ev.clientY-box.top,tip=canvas.parentElement.querySelector('.tip'),t=e?.timeline[frame(e)];tip.hidden=!(ready&&launched&&t?.icon_visible&&x>=488&&x<=526&&y>=66&&y<=104);if(!tip.hidden)tip.textContent=e.title+(e.id==='BM03'?' · '+t.charges+'/3 charges · bonus +'+t.bonus_percent+' %':' · renvoi magique direct40 % · '+t.remaining+' tours restants');};canvas.onmouseleave=()=>canvas.parentElement.querySelector('.tip').hidden=true;}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest,separators=(',',':')))
    labels={'HAUT':'Vers le haut','DROITE':'Vers la droite','GAUCHE':'Vers la gauche','BAS':'Vers le bas','ATTENTE':'Attente sans action','CLOISON':'Obstacle','CACHE':'Cible cachée','DISTANCE':'Arme à distance active','SILENCE':'Silence','SURVOL':'Survol des buffs','MANQUE':'Coup manqué','DEPLACEMENT':'Déplacement','AUTRE-ACTION':'Autre action','SANS-TOUR':'Menu sans tour','POISON':'Poison exclu','INDIRECT':'Dégâts indirects exclus','MELEE':'Coup de mêlée exclu','TRANSFORME':'Forme transformée'}
    options=''.join(f'<option value="{k}">{v}</option>'for k,v in labels.items());cards=''.join(f'<article><h2>{n} · {title}</h2><p class="muted">{sid}-A01 · proposé</p><canvas data-skill="{sid}" width="576" height="576"></canvas><div class="tip" hidden></div></article>'for n,sid,title in [(53,'BM03','Tempo offensif'),(54,'BM05','Sceau de vindicte')])
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Tempo et vindicte</title><style>*{box-sizing:border-box}body{margin:auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{position:relative;padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}.tip{position:absolute;right:20px;top:214px;width:270px;padding:12px;background:#0b1218;border:1px solid #6c7d88;pointer-events:none}a{color:#93c1e0}[hidden]{display:none}@media(max-width:1296px){.grid{grid-template-columns:608px}}</style><h1>Tempo offensif et Sceau de vindicte</h1><p class="muted">ANIM-S22 · candidats53–54 · cases natives64px · rendu à valider</p><div class="controls"><select id="scenario">OPTIONS</select><button id="next">Action suivante</button><button id="cast">Commencer l’aperçu</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="grid">CARDS</main>
<p>Tempo offensif gagne des charges automatiquement après ses frappes : le bonus profite aux coups suivants. L’icône compte les charges, sans timer. Les nombres affichent le total puis le bonus déjà compris dans ce total. Un coup manqué, un déplacement ou une autre action rompent la série ; un menu sans passage de tour la conserve.</p>
<p>Sceau de vindicte laisse le projectile vous toucher, puis le renvoie vers son auteur avec son apparence d’origine. Vous subissez toujours les dégâts initiaux. L’icône indique les six tours du buff ; après son expiration, l’attaque suivante ne provoque aucun retour. Poison, dégâts indirects et coups de mêlée sont exclus.</p>
<p class="muted">Chaque panneau est une scène indépendante. Tempo reste immobile après sa quatrième frappe tandis que les étapes suivantes avancent le compteur du sceau. Les chiffres20/25/30/35 et le retour8 sont des exemples préparés : aucune statistique de combat ni variance générale ajoutée. Monstres32px temporaires, adaptation prévue ensuite.</p>
<p><a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> · <a href="tempo-vindicte-64.gif">Aperçu animé</a> · <a href="review.json">Périmètre</a> · <a href="verification.json">Vérifications</a></p><script>SCRIPT</script></html>'''.replace('OPTIONS',options).replace('CARDS',cards).replace('SCRIPT',script)
    for a,b in [('candidats53–54','candidats 53–54'),('natives64px','natives 64 px'),('chiffres20/','chiffres 20/'),('retour8','retour 8'),('Monstres32px','Monstres 32 px'),('direct40 %','direct 40 %')]:html=html.replace(a,b)
    (OUT/'index.html').write_text(html,encoding='utf8')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--prepare',action='store_true');p.add_argument('--convert-stream',action='store_true');p.add_argument('--reuse-frames',action='store_true');a=p.parse_args()
    if a.prepare:prepare()
    elif a.convert_stream:converter.OUT=OUT;converter.convert()
    else:package(reuse_frames=a.reuse_frames)
