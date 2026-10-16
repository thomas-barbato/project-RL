"""ANIM-S09: one-shot crystalline explosions around visible frozen enemies."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_frost_fronts as previous
from prepare_agent_skill_animation_revision import save_animation

OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-26'
SOURCE=Path(r'C:\Users\User\.codex\generated_images\01a11010-433f-7fc2-ac29-ffb696dec5ea\exec-087aa978-fc33-40fa-9a43-d42307fb6fe5.png')


def protected():
    hashes=previous.protected()
    for p in sorted(previous.OUT.rglob('*')):
        if p.is_file():hashes[p.relative_to(base.ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True,exist_ok=True);assert not(OUT/'review-protection.json').exists()
    base.dump(OUT/'review-protection.json',protected())
    (OUT/'burst-source.png').write_bytes(SOURCE.read_bytes())
    with Image.open(SOURCE) as image:
        assert image.mode=='RGBA' and image.size==(1536,1024)
        assert image.getchannel('A').getextrema()[0]==0
        atlas=Image.new('RGBA',(1536,256))
        for i in range(6):
            x,y=i%3*512,i//3*512
            # Mechanical extraction excludes generated divider lines. One common
            # crop and scale preserves the relative expansion and alpha in all frames.
            cell=image.crop((x+12,y+12,x+500,y+500)).resize((256,256),Image.Resampling.LANCZOS)
            atlas.paste(cell,(i*256,0))
        atlas.save(OUT/'burst-atlas.png')
    base.dump(OUT/'drawing-pack.json',{'mode':'Built-in imagegen','source_sha256':hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
        'source_size':[1536,1024],'layout':[3,2],'source_cell_inset':12,'common_crop_size':[488,488],
        'cell_size':[256,256],'anchor':[.5,.5],'alpha_preserved':True,
        'postprocessing':'Equal-cell extraction excludes sheet dividers, uniform resize. No repainting.'})
    base.dump(OUT/'review.json',{'batch':'ANIM-S09','status':'awaiting_visual_review','native_tile_size':64,
        'candidate':{'number':19,'skill_id':'FR06','variant':'FR06-A01','name':'Explosion cristalline','status':'awaiting_visual_review'},
        'effect':'Six chronological shatter drawings expand then disappear in .42 seconds; no looping burst, no sphere, no blood. Multiple sources detonate together, with a readable open centre and unchanged bodies in front.',
        'sources':'Prepared snapshot of all visible frozen hostiles. Their Gel is consumed. Nearby unfrozen actors are contacted without becoming new sources and without receiving new Gel. A prepared hidden frozen actor remains excluded and undrawn.',
        'geometry':'Local radius-one cardinal footprints are prepared examples, not final diagonal/corner geometry. Multiple local explosions may contact one neighbour. All physical walls clip the drawing; a full cloison protects the far room.',
        'timing':'Animation ends by real time. Waiting does not advance cooldowns or repeat contacts. Explicit turns alone reduce cooldown and remaining statuses.',
        'scope':'Isolated debug visual fixture. No GameState, damage, death, RNG, actual FOV/AI, save changes or campaign integration. Visible prepared actors survive at their places.',
        'retained':'Approved Blizzard and Mur de glace, pending Vague/Nova/Gel reviews, skill catalogue and actor/equipment artwork.'})
    launch=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--lot-froid-tempetes'","'--lot-explosion-cristalline'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'frames':6}))


def convert():
    native=(OUT/'native').resolve();count=0
    for raw in native.rglob('*.rgba'):
        assert raw.resolve().is_relative_to(native);pixels=raw.read_bytes();assert len(pixels)==720*720*4
        png=raw.with_suffix('.png');Image.frombytes('RGBA',(720,720),pixels).save(png,compress_level=1)
        with Image.open(png) as check:assert check.tobytes()==pixels
        raw.unlink();count+=1  # Verified temporary raw only, inside this review.
    if count:base.dump(OUT/'native-encoding-verification.json',{'converted_frames':count,'pixel_identical_to_native_readback':True,'resampling':False})
    else:assert(OUT/'native-encoding-verification.json').exists()


def timeline_check(e,t):
    pool=[1] if e['single'] else [0,1,2]
    expected=[1,1,1,1,0,0] if e['single'] else [2,3,2,1,1,0]
    if e['folder']=='CLOISON':expected[3:5]=[0,0]
    assert all(r['sources']==pool and r['contacts']==expected and r['cooldown']==8 and r['turn']==0
        and r['gel']==[0,0,0,0,0,3] for r in t[17:48])
    assert all(r['visual_active'] for r in t[17:28]) and all(not r['visual_active'] for r in t[28:])
    assert all(not r['hidden_is_source'] for r in t)
    assert [t[i]['turn'] for i in (48,65,82)]==[1,2,3]
    assert [t[i]['cooldown'] for i in (48,65,82)]==[7,6,5]
    assert [t[i]['gel'][5] for i in (48,65,82)]==[2,1,0]
    for start,end in [(48,65),(65,82),(82,96)]:
        assert all(r['contacts']==expected and r['sources']==pool and r['cooldown']==t[start]['cooldown'] and r['gel']==t[start]['gel'] for r in t[start:end])


def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'));assert protected()==baseline
    convert();entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'));assert len(entries)==7
    checks=[];animations=[]
    for e in entries:
        folder=OUT/'native'/e['folder'];target=OUT/'frames'/e['variant'];target.mkdir(parents=True,exist_ok=True)
        x,y,w,h=e['crop'];bounds=(x,y,x+w,y+h);frames=[]
        for i in range(96):
            with Image.open(folder/f'{i:03}.png') as image:
                assert image.size==(720,720);frame=image.crop(bounds).convert('RGB');frame.save(target/f'{i:03}.png',compress_level=1);frames.append(frame)
        diffs={}
        for phase in e['control_frames']:
            with Image.open(folder/f'control-{phase:03}.png') as image:diffs[phase]=ImageChops.difference(frames[phase],image.crop(bounds).convert('RGB'))
            for cy,row in enumerate(e['map'].splitlines()):
                for cx,symbol in enumerate(row):
                    if symbol=='#':assert diffs[phase].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None,f'Ice on solid wall: {e["variant"]}, {phase}'
        assert diffs[21].getbbox() and diffs[24].getbbox()
        assert all(diffs[p].getbbox() is None for p in (28,35,47,50,66,83,95)), 'Burst or consumed visible Gel survived.'
        cx,cy=e['affected_cells'][0];sample=(cx*64+3,cy*64+25)
        assert frames[3].getpixel(sample)[1]>frames[14].getpixel(sample)[1]+25,'Preview does not blink.'
        red,green,blue=frames[35].getpixel(sample);assert not(red<100 and green>140 and blue>140),'Preview survived cast.'
        if e['folder']=='CLOISON':
            for actor in e['visible_targets']:
                if actor['label'] in ('D','E'):
                    cx,cy=actor['cell'];assert all(diffs[p].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None for p in (17,19,21,24,27))
        timeline_check(e,json.loads((folder/'timeline.json').read_text(encoding='utf-8')))
        checks.append({'variant':e['variant'],'solid_walls_clear':True,'preview_blinks_and_ends':True,'visible_gel_consumed':True,
            'hidden_source_excluded':True,'contacts_do_not_chain_or_repeat':True,'instant_burst_ends_without_a_turn':True,'waiting_preserves_cooldown':True})
        if e['folder'] in ('HAUT','CLOISON','OBSTACLE','UNIQUE'):animations.append(save_animation(OUT/f"{e['variant']}-64.gif",frames,24))
        if e['folder']=='HAUT':
            for phase,name in [(14,'avant'),(21,'eclatement'),(28,'apres')]:frames[phase].save(OUT/f'{name}-64.png')
    base.dump(OUT/'verification.json',{'protected_files_unchanged':len(baseline),'controls':checks,'animations':animations,'resampling_of_native_capture':False})
    for source,name in [('src/agent_skill_lab_review/fire_painted/frost/crystalline.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes());write_html(entries)
    print(json.dumps({'native_views':7,'animations':len(animations),'protected_files':len(baseline),'controls':'passed'}))


def write_html(entries):
    manifest=[{k:e[k] for k in ('folder','variant','frames')} for e in entries]
    script='''const manifest=MANIFEST,sequences=new Map();let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,loadToken=0;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),stages=[17,48,65,82],ends=[47,64,81,95];
function frame(){const f=Math.floor(elapsed*24/1000);return !launched?f%17:Math.min(stages[turn]+f,ends[turn]);}
function ui(){const moving=launched&&turn===0&&elapsed<500;document.getElementById('status').textContent=launched?`Tour ${turn} · ${moving?'Éclatement':'Glace dispersée · Gel consommé'} · recharge ${8-turn}`:'Sources gelées visibles · avant lancement';cast.disabled=launched||!ready;next.disabled=!ready||!launched||turn>=3||moving;}
async function load(){ready=false;const token=++loadToken,e=manifest.find(e=>e.folder===scenario.value);document.getElementById('loading').textContent='Chargement…';const images=await Promise.all(Array.from({length:e.frames},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${e.variant}/${String(i).padStart(3,'0')}.png`;})));if(token!==loadToken)return;sequences.set(e.folder,images);elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · cases natives 64 px';ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();document.getElementById('view').getContext('2d').drawImage(sequences.get(scenario.value)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;ui();load().catch(()=>document.getElementById('loading').textContent='Chargement impossible.');};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};ui();load().catch(()=>document.getElementById('loading').textContent='Conserver les frames avec la galerie.');requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Explosion cristalline</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1060px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}canvas{display:block;width:576px;height:576px;margin:0 auto}a{color:#93c1e0}#status{color:#c4dcdc;font-weight:600}</style>
<h1>19 · Explosion cristalline</h1><p class="muted">FR06-A01 · nouveau rendu à examiner en 64 px</p>
<p>La glace des sources gelées se brise et projette des fragments. Les détonations partent ensemble, puis disparaissent. Les corps restent lisibles et le Gel des sources est consommé.</p>
<div class="controls"><select id="scenario"><option value="HAUT">Trois sources proches</option><option value="DROITE">Disposition tournée à droite</option><option value="GAUCHE">Disposition tournée à gauche</option><option value="BAS">Disposition tournée vers le bas</option><option value="CLOISON">Cloison pleine</option><option value="OBSTACLE">Obstacle local</option><option value="UNIQUE">Une source · voisins non gelés</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button><span id="loading">Chargement…</span></div><p id="status"></p><canvas id="view" width="576" height="576"></canvas>
<p>Dans le premier exemple, A, B et C sont gelés. Chacun déclenche une explosion locale ; B est au croisement des trois zones. D et E peuvent être touchés sans devenir de nouvelles sources. Le sort ne provoque aucun nouveau Gel.</p>
<p>L’éclatement dure 0,42 seconde, puis s’éteint sans attendre un tour. Attendre ne répète aucune détonation et ne réduit pas la recharge. L’aperçu clignotant disparaît à la confirmation. La cloison coupe le dessin et protège les voisins de l’autre côté.</p>
<p class="muted">Prototype isolé : sources, contacts et survivants préparés. Aucun calcul de dégâts, décès, hasard, observation ou IA réelle et aucune modification de campagne ou de sauvegarde. Une source gelée cachée est exclue et reste non affichée. La géométrie diagonale finale reste à traiter séparément. Ce rendu attend sa validation.</p>
<p><a href="FR06-A01-HAUT-64.gif">Séquence animée complète</a> · <a href="FR06-A01-CLOISON-64.gif">Cloison</a> · <a href="FR06-A01-UNIQUE-64.gif">Une source</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p>
<p class="muted">Dessin créé avec imagegen intégré : <a href="burst-source.png">planche source</a> · <a href="imagegen-prompt.txt">prompt</a>. Extraction mécanique des cellules, sans repeindre les acteurs.</p><script>SCRIPT</script></html>'''.replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
