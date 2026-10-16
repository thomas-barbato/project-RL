"""ANIM-S08: native turn-driven frost wave and delayed Blizzard review."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_ice_wall as previous
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-25'
GENERATED = Path(r'C:\Users\User\.codex\generated_images\01a11010-433f-7fc2-ac29-ffb696dec5ea')
SOURCES = {'wave': 'exec-8850b1a6-afba-4f3c-96d5-661720a9745e.png',
           'storm': 'exec-a09fae67-b3a0-45b3-b19a-820d9a7dfb6d.png'}
TURNS = [48,65,82,99,116,133,150,167]


def protected():
    hashes = previous.protected()
    for p in sorted(previous.OUT.rglob('*')):
        if p.is_file():
            hashes[p.relative_to(base.ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    baseline = OUT / 'review-protection.json'
    assert not baseline.exists(), 'Preserve the protection baseline.'
    base.dump(baseline, protected())
    packing = []
    for kind, name in SOURCES.items():
        source = GENERATED / name
        (OUT / f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as image:
            assert image.mode == 'RGBA' and image.size == (1536,1024)
            assert image.getchannel('A').getextrema()[0] == 0
            atlas = Image.new('RGBA',(1536,256))
            for i in range(6):
                x,y = i%3*512,i//3*512
                sprite = image.crop((x,y,x+512,y+512)).resize((256,256),Image.Resampling.LANCZOS)
                atlas.paste(sprite,(i*256,0))
            atlas.save(OUT / f'{kind}-atlas.png')
        packing.append({'kind':kind,'source_size':[1536,1024],'layout':[3,2],
            'cell_size':[256,256],'uniform_scale':.5,'anchor':[.5,.5],
            'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'alpha_preserved':True})
    base.dump(OUT/'drawing-pack.json',{'mode':'Built-in imagegen','packing':packing,
        'postprocessing':'Equal-cell cropping and uniform resampling only; no painting.'})
    base.dump(OUT/'review.json',{'batch':'ANIM-S08','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'skill_id':sid,'variant':sid+'-A01','name':title,'status':'awaiting_visual_review'}
            for n,sid,title in [(17,'FR04','Vague de froid'),(18,'FR05','Blizzard')]],
        'wave':'One broad icy front across three cells, chosen adjacent to the caster. First contact at launch; subsequent movement one cell per explicit turn, maximum five. A .18-second interpolation is visual only. Blocked lanes never return. No lingering cold zone.',
        'blizzard':'Airy downward sleet. Visual formation starts on cast; the previously approved explicit one-turn delay before first hits remains. Four prepared distinct legal impact cells per application at rank 11. Five applications over five active turns. Visual snowfall loops without producing more impacts.',
        'timing':'Real time controls only moving drawings. Cooldowns, Gel, wave movement, impact applications and zone expiry only advance with explicit turns. No extra real-time damage ticks.',
        'freeze':'Reuse the unchanged attached-foot Gel candidate from review 23, without inferring new approval. Prepared successful Gel removes initial Burn. No blood in pure Frost.',
        'preview':'Blinking cyan footprint disappears on cast. Wave shows only its adjacent starting band with an arrow; no full-travel preview is added. Native lab lets the band turn around the same caster before launch.',
        'geometry':'Open scenes in four cardinal orientations, partial obstacles, separate full-cloison cases and a starting-band placement on the right. Physics cells are prepared fixtures; final campaign targeting and large-actor rules remain separate.',
        'scope':'Isolated debug visual proposal: no GameState, campaign, saves, damage, RNG or real AI. Hits/Gel are prepared examples; no balancing claims.',
        'unchanged':'Catalogue, approved wall FR03-A02, all previous reviews and actor/equipment artwork.'})
    launch = (previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    launch = launch.replace("'--lot-froid-ouverture' '--mur-glace-v2'","'--lot-froid-tempetes'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'atlases':2}))


def convert():
    native=(OUT/'native').resolve();count=0
    for raw in native.rglob('*.rgba'):
        assert raw.resolve().is_relative_to(native)
        pixels=raw.read_bytes();assert len(pixels)==1280*720*4
        png=raw.with_suffix('.png');Image.frombytes('RGBA',(1280,720),pixels).save(png,compress_level=1)
        with Image.open(png) as check:assert check.tobytes()==pixels
        raw.unlink()  # Verified temporary raw frame inside this review only.
        count+=1
    if count:
        base.dump(OUT/'native-encoding-verification.json',{'converted_frames_this_run':count,'pixel_identical_to_native_readback':True,'resampling':False})
    else:
        assert (OUT/'native-encoding-verification.json').exists(), 'Missing native export proof.'


def verify_timeline(e,t):
    assert all(r['turn']==0 and r['cooldowns']==[8,10] and r['wave_distance']==1 and r['pending']==1
        and r['ticks']==0 and r['gel']==t[17]['gel'] and r['burn']==t[17]['burn'] for r in t[17:48])
    assert [t[i]['turn'] for i in TURNS]==list(range(1,9))
    for i,(start,end) in enumerate(zip(TURNS,TURNS[1:]+[192])):
        assert all(r['gel']==t[start]['gel'] and r['cooldowns']==t[start]['cooldowns'] and r['front']==t[start]['front']
            and r['ticks']==t[start]['ticks'] and r['remaining']==t[start]['remaining'] and r['exposure']==t[start]['exposure'] for r in t[start:end])
    assert t[48]['ticks']==1 and t[116]['ticks']==5 and t[133]['ticks']==5 and t[133]['remaining']==0
    assert all(r['gel']==[[0]*6,[0]*6] for r in t[167:])
    assert t[48]['burn'][1][0]==0 and t[48]['gel'][1][0]==3
    if e['folder']=='OBSTACLES':
        assert t[65]['lanes']==[False,True,True] and t[82]['lanes']==[False,True,False]
        assert t[99]['gel'][0][5]==0 and t[99]['gel'][0][3]==3
    if e['folder']=='CLOISON':
        assert not any(t[82]['lanes'])
        assert all(r['gel'][1][2]==0 and r['gel'][1][4]==0 and r['gel'][0][3]==0 and r['gel'][0][5]==0 for r in t)
    if e['folder'] not in ('OBSTACLES','CLOISON','PLACEMENT-DROITE'):
        assert t[99]['wave_distance']==5 and not any(t[116]['lanes'])
    for r in t:
        assert len(r['hits'])==len({tuple(p) for p in r['hits']})
        if r['ticks']>0:assert len(r['hits'])==4


def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline,'Previously reviewed assets changed.'
    convert();entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'));assert len(entries)==14
    sequences={};checks=[];animations=[]
    for e in entries:
        x,y,w,h=e['crop'];bounds=(x,y,x+w,y+h);folder=OUT/'native'/e['folder'];target=OUT/'frames'/e['variant'];target.mkdir(parents=True,exist_ok=True)
        frames=[]
        for i in range(192):
            with Image.open(folder/f'{i:03}.png') as image:
                assert image.size==(1280,720);frame=image.crop(bounds).convert('RGB');frame.save(target/f'{i:03}.png',compress_level=1);frames.append(frame)
        diffs={}
        for phase in e['control_frames']:
            with Image.open(folder/f'control-{phase:03}.png') as image:diffs[phase]=ImageChops.difference(frames[phase],image.crop(bounds).convert('RGB'))
            for cy,row in enumerate(e['map'].splitlines()):
                for cx,symbol in enumerate(row):
                    if symbol=='#':assert diffs[phase].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None,f'Frost on solid wall: {e["variant"]}, {phase}, {cx},{cy}'
        assert diffs[23].getbbox() and diffs[51].getbbox()
        assert diffs[183].getbbox() is None and diffs[191].getbbox() is None,'Expired effect remains.'
        cx,cy=e['affected_cells'][0];sample=(cx*64+3,cy*64+25)
        assert frames[3].getpixel(sample)[1]>frames[14].getpixel(sample)[1]+25,'Preview does not blink.'
        for phase in (23,40):
            red,green,blue=frames[phase].getpixel(sample)
            assert not(red<100 and green>140 and blue>140),'Preview survived launch.'
        timeline=json.loads((folder/'timeline.json').read_text(encoding='utf-8'));verify_timeline(e,timeline)
        for stage,end in enumerate((47,64,81,98,115,132)):
            with Image.open(folder/f'control-{end:03}.png') as image:
                quiet=image.crop(bounds).convert('RGB')
            for i in range(6):
                with Image.open(folder/f'idle-{stage}-{i}.png') as image:
                    idle=image.crop(bounds).convert('RGB')
                idle.save(target/f'idle-{stage}-{i}.png',compress_level=1)
                diff=ImageChops.difference(idle,quiet)
                for cy,row in enumerate(e['map'].splitlines()):
                    for cx,symbol in enumerate(row):
                        if symbol=='#':assert diff.crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None,'Idle frost crossed a solid wall.'
        if e['folder']=='CLOISON':
            protected_labels=('D','F') if e['id']=='FR04' else ('C','E')
            for actor in e['targets']:
                if actor['label'] in protected_labels:
                    cx,cy=actor['cell'];body=(cx*64+10,cy*64+8,cx*64+54,cy*64+57)
                    assert all(diffs[p].crop(body).getbbox() is None for p in (40,51,68,85,102,119,136,153,170))
        if e['id']=='FR04' and e['folder']=='OBSTACLES':
            f=next(a for a in e['targets'] if a['label']=='F');cx,cy=f['cell']
            assert all(diffs[p].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None for p in (102,119)), 'Stopped wave lane returned behind obstacle.'
        checks.append({'variant':e['variant'],'solid_walls_clear':True,'preview_blinks_and_ends':True,'real_time_preserves_turn_state':True,
            'wave_movement_and_storm_applications_turn_driven':True,'explicit_blizzard_delay_preserved':True,'prepared_hits_distinct':True,'final_effects_expired':True})
        sequences[e['variant']]=frames
        if e['folder'] in ('HAUT','OBSTACLES','CLOISON','PLACEMENT-DROITE'):
            animations.append(save_animation(OUT/f"{e['variant']}-64.gif",frames,24))
    for folder,stem in [('HAUT','tempetes-froid'),('OBSTACLES','tempetes-obstacles'),('CLOISON','tempetes-cloison')]:
        pair=[next(e for e in entries if e['folder']==folder and e['id']==sid) for sid in ('FR04','FR05')];boards=[]
        for i in range(192):
            board=Image.new('RGB',(1232,650),'#0b1218');draw=ImageDraw.Draw(board)
            for col,e in enumerate(pair):
                left=12+col*616;draw.text((left,10),f"{e['number']} · {e['title']}",font=base.font(20,True),fill='#e0e7e7')
                draw.text((left,40),e['variant'],font=base.font(15),fill='#a4b9c6');board.paste(sequences[e['variant']][i],(left,66))
            boards.append(board)
        animations.append(save_animation(OUT/f'{stem}-64.gif',boards,24))
        for phase,name in [(23,'depart'),(51,'premier-tour'),(85,'troisieme-case'),(119,'limite'),(183,'fin')]:boards[phase].save(OUT/f'{stem}-{name}.png')
    base.dump(OUT/'verification.json',{'protected_files_unchanged':len(baseline),'controls':checks,'animations':animations,'resampling_of_native_capture':False,
        'scope':'Prepared visual fixtures, not campaign damage or AI validation.'})
    for source,name in [('src/agent_skill_lab_review/fire_painted/frost/fronts.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes());write_html(entries)
    print(json.dumps({'native_views':14,'animations':len(animations),'protected_files':len(baseline),'controls':'passed'}))


def write_html(entries):
    manifest=[{k:e[k] for k in ('id','folder','variant','frames')} for e in entries]
    script='''const manifest=MANIFEST,sequences=new Map();let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,loadToken=0;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),stages=[17,48,65,82,99,116,133,150,167],ends=[47,64,81,98,115,132,149,166,191];
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return {kind:'main',index:f%17};const start=stages[turn],end=ends[turn],count=end-start+1;if(f<count)return {kind:'main',index:start+f};if(turn<6)return {kind:'idle',stage:turn,index:Math.floor((f-count)/4)%6};return {kind:'main',index:end};}
function ui(){const moving=launched&&elapsed<350;document.getElementById('status').textContent=launched?`Tour ${turn} · ${turn===0?'Vague lancée · Blizzard en formation':turn<=4?'Vague : +1 case · Blizzard : nouvelles frappes':turn===5?'Vague terminée · dernière application de Blizzard':turn===6?'Tempête terminée · Gel restant':turn===7?'Derniers effets de Gel':'Effets terminés'}`:'Bande de départ et zone visée · avant lancement';cast.disabled=launched||!ready;next.disabled=!ready||!launched||turn>=8||moving;}
async function load(){ready=false;const token=++loadToken,selected=scenario.value;document.getElementById('loading').textContent='Chargement…';function image(path){return new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=path;});}const loaded=await Promise.all(manifest.filter(e=>e.folder===selected).map(async e=>{const prefix=`frames/${e.variant}/`;const main=await Promise.all(Array.from({length:e.frames},(_,i)=>image(prefix+`${String(i).padStart(3,'0')}.png`)));const idle=await Promise.all(Array.from({length:6},(_,stage)=>Promise.all(Array.from({length:6},(_,i)=>image(prefix+`idle-${stage}-${i}.png`)))));return[e.id,{main,idle}];}));if(token!==loadToken)return;sequences.clear();for(const[key,images]of loaded)sequences.set(key,images);elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · cases natives 64 px';ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();const f=frame();for(const c of document.querySelectorAll('canvas')){const sequence=sequences.get(c.dataset.skill),image=f.kind==='main'?sequence.main[f.index]:sequence.idle[f.stage][f.index];c.getContext('2d').drawImage(image,0,0);}}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;ui();load().catch(()=>document.getElementById('loading').textContent='Chargement impossible.');};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};ui();load().catch(()=>document.getElementById('loading').textContent='Conserver les frames avec la galerie.');requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Vague de froid et Blizzard</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:19px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}#status{color:#c4dcdc;font-weight:600}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Froid · Vague et Blizzard</h1><p class="muted">ANIM-S08 · nouveaux dessins et animations à examiner</p>
<div class="controls"><select id="scenario"><option value="HAUT">Vers le haut</option><option value="DROITE">Vers la droite</option><option value="GAUCHE">Vers la gauche</option><option value="BAS">Vers le bas</option><option value="OBSTACLES">Obstacles partiels</option><option value="CLOISON">Cloison pleine</option><option value="PLACEMENT-DROITE">Bande de départ à droite du même agent</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button><span id="loading">Chargement…</span></div><p id="status"></p>
<main class="pair"><article><h2>17 · Vague de froid</h2><p class="muted">FR04-A01 · front large, une case par tour</p><canvas data-skill="FR04" width="576" height="576"></canvas></article><article><h2>18 · Blizzard</h2><p class="muted">FR05-A01 · grêle descendante et frappes dispersées</p><canvas data-skill="FR05" width="576" height="576"></canvas></article></main>
<p>La Vague touche sa première bande au lancement. « +1 tour » la fait avancer d’une case, jusqu’à cinq cases. Chaque portion arrêtée par un mur disparaît et ne revient pas derrière lui. La Vague ne laisse pas de zone persistante.</p>
<p>Blizzard se forme dès le lancement. Son délai d’un tour avant les premières frappes, déjà défini, est conservé. Cet exemple au rang 11 montre quatre cases frappées par application, puis cinq applications. La neige qui continue à tomber entre les tours ne produit pas de dégâts supplémentaires.</p>
<p>Sans passer de tour, les dessins restent animés, mais la Vague ne se déplace pas et Blizzard ne frappe pas de nouveau. Le Gel reste attaché aux cibles ; un Gel réussi retire leur Brûlure. Le laboratoire natif permet de changer la bande autour du même agent avant le lancement.</p>
<p class="muted">Prototype isolé : impacts et réussites de Gel préparés, sans dégâts, tirage, IA réelle ni changement de campagne ou de sauvegarde. Les règles du catalogue sont conservées. Ces rendus attendent ta validation.</p>
<p><a href="tempetes-froid-64.gif">Séquence animée complète</a> · <a href="tempetes-obstacles-64.gif">Obstacles partiels</a> · <a href="tempetes-cloison-64.gif">Cloison</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p>
<p class="muted">Dessins créés avec imagegen intégré : <a href="wave-source.png">source Vague</a> / <a href="imagegen-wave-prompt.txt">prompt</a> · <a href="storm-source.png">source Blizzard</a> / <a href="imagegen-storm-prompt.txt">prompt</a>.</p><script>SCRIPT</script></html>'''.replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
