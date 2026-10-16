"""ANIM-S12: blood prison and marked resurrection, native isolated review."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw
import prepare_agent_skill_animation_drain_intensity as previous

ROOT=previous.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-36'
BASE=previous.OUT
base=previous.drains.base
GEN=Path(r'C:\Users\User\.codex\generated_images\01a11010-433f-7fc2-ac29-ffb696dec5ea')
SOURCES={'prison':'exec-32cfcf4a-5068-40ef-bb3a-97c6aae397c9.png','reanimation':'exec-81876112-7a93-48e9-8d8f-c35d6419cc74.png'}


def protected():
    hashes=previous.protected()
    for p in BASE.rglob('*'):
        if p.is_file():hashes[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'review-protection.json').exists()
    base.dump(OUT/'review-protection.json',protected());packs=[]
    for kind,name in SOURCES.items():
        source=GEN/name;(OUT/f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as image:
            assert image.mode=='RGBA' and image.size==(1536,1024)
            assert image.getchannel('A').getextrema()[0]==0
            atlas=Image.new('RGBA',(1536,256))
            for i in range(6):
                x=i%3*512;y=i//3*512
                atlas.paste(image.crop((x,y,x+512,y+512)).resize((256,256),Image.Resampling.LANCZOS),(i*256,0))
            atlas.save(OUT/f'{kind}-atlas.png')
        packs.append({'kind':kind,'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'source_size':[1536,1024],'layout':[3,2],'cell_size':[256,256],'alpha_preserved':True})
    base.dump(OUT/'drawing-pack.json',{'mode':'Built-in imagegen','packing':packs,'postprocessing':'Equal-cell cropping and uniform resizing only; no repainting.'})
    base.dump(OUT/'review.json',{'batch':'ANIM-S12','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'id':sid,'variant':sid+'-A01','name':title,'status':'awaiting_visual_review'} for n,sid,title in [(25,'OC04','Prison sigillaire'),(26,'OC07','Réanimation')]],
        'prison':'Eight independent blood glyph segments around a free central tile. Rapid .30-second cosmetic tracing, persistent red marks; one prepared segment breaks and disappears, leaving a breach; scripted enemy exits through it.',
        'reanimation':'Blood seal persists under a marked enemy until the prepared death. Red resurrection surge and a short corpse/living-sprite blend, then an ivory skull marker identifies the raised ally. Scripted movement and one bump suggest an autonomous ordinary attack.',
        'timing':'Manual turns only: prison four turns at rank 11; resurrection eleven turns from birth at the prepared death on turn 2. Quiet scene passes no turns automatically and keeps the mark/circle visible.',
        'scope':'Debug-only visual fixture, not real combat/AI. No actual segment damage, healing, loot, XP, skill/stat changes, RNG, inventory, familiar window, save or campaign integration. Existing living/corpse actor artwork retained.'})
    launch=(BASE/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--lot-occulte-drains' '--drains-intenses'","'--lot-rituels-occultes'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'atlases':2,'candidates':['OC04-A01','OC07-A01']}))


def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'));assert protected()==baseline
    previous.drains.OUT=OUT;previous.drains.convert()
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'));assert len(entries)==7
    checks=[];sequences={};animations=[]
    for e in entries:
        folder=OUT/'native'/e['folder'];timeline=json.loads((folder/'timeline.json').read_text(encoding='utf-8'))
        for j,v in enumerate(e['views']):
            x,y,w,h=v['crop'];bounds=(x,y,x+w,y+h);frames=[]
            target=OUT/'frames'/v['variant'];target.mkdir(parents=True,exist_ok=True)
            for i in range(144):
                with Image.open(folder/f'{i:03}.png') as image:frame=image.crop(bounds).convert('RGB')
                frame.save(target/f'{i:03}.png',compress_level=1);frames.append(frame)
            diffs={}
            for p in e['control_frames']:
                with Image.open(folder/f'control-{p:03}.png') as quiet:diffs[p]=ImageChops.difference(frames[p],quiet.crop(bounds).convert('RGB'))
                for cy,row in enumerate(e['map'].splitlines()):
                    for cx,symbol in enumerate(row):
                        if symbol=='#':assert diffs[p].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None,(v['variant'],p,cx,cy)
            rows=[r['states'][j] for r in timeline]
            if not v['valid']:
                assert all(not r['launched'] and r['cooldown']==0 for r in rows)
                assert all(d.getbbox() is None for d in diffs.values())
            elif e['quiet']:
                assert all(r['turn']==0 for r in rows)
                assert diffs[47].getbbox() and diffs[143].getbbox()
                if j==0:assert all(r['remaining']==4 and sum(r['segments'])==8 for r in rows[17:])
                else:assert all(r['marked'] and not r['raised'] for r in rows[17:])
            elif j==0:
                assert sum(rows[47]['segments'])==8 and rows[47]['remaining']==4
                assert sum(rows[48]['segments'])==7 and rows[48]['remaining']==3
                assert sum(rows[60]['segments'])==7 and rows[60]['target']==[4,5]
                assert sum(rows[72]['segments'])==7 and rows[72]['target']==[4,6]
                assert not any(rows[84]['segments']) and rows[84]['remaining']==0
                assert all(diffs[p].getbbox() is None for p in (87,100,137,138,143))
            else:
                assert rows[54]['marked'] and not rows[54]['raised']
                assert not rows[60]['marked'] and rows[60]['raised'] and rows[60]['remaining']==11
                assert rows[72]['raised'] and rows[72]['remaining']==10
                assert rows[137]['raised'] and rows[137]['remaining']==1
                assert rows[138]['expired'] and not rows[138]['raised'] and rows[138]['remaining']==0
                assert diffs[100].getbbox() and diffs[143].getbbox() is None
            if v['valid']:
                cx,cy=v['affected_cells'][0];sample=(cx*64+3,cy*64+25)
                assert frames[3].getpixel(sample)[1]>frames[14].getpixel(sample)[1]+25
                assert diffs[35].getbbox(),'Persistent mark invisible.'
            sequences[v['variant']]=frames
            if v['valid'] and e['folder'] in ('HAUT','OBSTACLE','ATTENTE'):
                animations.append(previous.drains.save_animation(OUT/f"{v['variant']}-64.gif",frames,24))
            checks.append({'variant':v['variant'],'solid_walls_clear':True,'preview_blinks_and_ends':True,'prepared_timeline_checked':True,'refused_placement_atomic':not v['valid'],'circle_segments_independent':True,'persistent_mark_and_ally_duration_checked':True})
        if e['folder']=='HAUT':
            for p in (14,35,54,65,78,100,143):
                with Image.open(folder/f'{p:03}.png') as im:im.save(OUT/f'phase-{p:03}-native.png')
    for scenario in ('HAUT','OBSTACLE','ATTENTE'):
        boards=[]
        for i in range(144):
            board=Image.new('RGB',(1232,650),'#0b1218');draw=ImageDraw.Draw(board)
            for j,(sid,title) in enumerate([('OC04','25 · Prison sigillaire'),('OC07','26 · Réanimation')]):
                x=12+j*616;variant=next(v['variant'] for e in entries if e['folder']==scenario for v in e['views'] if v['id']==sid)
                draw.text((x,10),title,font=base.font(20,True),fill='#e0e7e7');draw.text((x,40),variant,font=base.font(15),fill='#a5b9c5')
                board.paste(sequences[variant][i],(x,66))
            boards.append(board)
        animations.append(previous.drains.save_animation(OUT/f'rituels-{scenario.lower()}-64.gif',boards,24))
    base.dump(OUT/'verification.json',{'protected_files_unchanged':len(baseline),'controls':checks,'animations':animations,'resampling_of_native_capture':False})
    for source,name in [('src/agent_skill_lab_review/fire_painted/occult_rituals.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes());write_html(entries)
    print(json.dumps({'native_views':14,'animations':len(animations),'protected_files':len(baseline),'controls':'passed'}))


def write_html(entries):
    manifest=[{'folder':e['folder'],'quiet':e['quiet'],'views':[{k:v[k] for k in ('id','variant','valid')} for v in e['views']]} for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),starts=[17,48,60,72,84,90,96,102,108,114,120,126,132,138],ends=[47,59,71,83,89,95,101,107,113,119,125,131,137,143];
function entry(){return manifest.find(e=>e.folder===scenario.value);}
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return f%17;if(entry().quiet)return f<18?17+f:36+(f-18)%12;const pulse=(turn===1&&entry().views[0].valid)||(turn===2&&entry().views[1].valid);if((entry().views[0].valid&&turn<4)||(entry().views[1].valid&&turn>=2&&turn<13)){if(f<=ends[turn]-starts[turn])return starts[turn]+f;const start=turn===0?36:pulse?ends[turn]-1:Math.min(starts[turn]+6,ends[turn]);return start+(f-(ends[turn]-starts[turn]+1))%(ends[turn]-start+1);}return Math.min(starts[turn]+f,ends[turn]);}
function ui(){cast.disabled=!ready||launched;next.disabled=!ready||!launched||turn>=13||elapsed<450||entry().quiet;document.getElementById('status').textContent=!launched?'Aperçu clignotant · avant lancement':entry().quiet?'Attente : aucun tour ni événement ajouté':`Tour ${turn} · cercle ${entry().views[0].valid?Math.max(0,4-turn):0} tours · allié ${entry().views[1].valid&&turn>=2?Math.max(0,13-turn):0} tours`;}
async function load(){ready=false;ui();const n=++token,e=entry();const loaded=await Promise.all(e.views.map(async v=>[v.id,await Promise.all(Array.from({length:144},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${v.variant}/${String(i).padStart(3,'0')}.png`;})))]));if(n!==token)return;sequences=new Map(loaded);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas'))canvas.getContext('2d').drawImage(sequences.get(canvas.dataset.skill)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Rituels occultes</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Magie noire · Prison et Réanimation</h1><p class="muted">ANIM-S12 · deux nouveaux rendus à valider · cases natives 64 px</p><div class="controls"><select id="scenario"><option value="HAUT">Vers le haut</option><option value="DROITE">Vers la droite</option><option value="GAUCHE">Vers la gauche</option><option value="BAS">Vers le bas</option><option value="OBSTACLE">Pourtour bloqué</option><option value="CLOISON">Cloison · cible inaccessible</option><option value="ATTENTE">Attente sans tour ni événement</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="pair"><article><h2>25 · Prison sigillaire</h2><p class="muted">OC04-A01 · huit segments sanglants, une brèche</p><canvas data-skill="OC04" width="576" height="576"></canvas></article><article><h2>26 · Réanimation</h2><p class="muted">OC07-A01 · sceau, retour à la vie, tête de mort alliée</p><canvas data-skill="OC07" width="576" height="576"></canvas></article></main>
<p>Le cercle laisse sa case centrale libre et conserve ses huit segments pendant ses tours. Dans cette séquence préparée, un segment est brisé au premier tour ; la cible sort ensuite par la brèche. Le reste du cercle s’efface à son expiration.</p><p>La marque de Réanimation demeure sous la cible jusqu’à sa mort. Dans l’exemple préparé, elle se relève au tour 2, identifiée par une tête de mort claire. Elle se dirige vers un autre ennemi, puis reste visible jusqu’à la fin de ses onze tours. Le dessin du personnage et de sa dépouille sont ceux du bestiaire existant.</p><p>Le lecteur ne passe aucun tour automatiquement. « +1 tour » déroule les événements ; les GIFs accélèrent les derniers tours pour montrer l’expiration. La scène d’attente conserve le cercle et la marque animés sans provoquer de mort ou d’expiration. La prévisualisation clignote avant lancement et disparaît à la confirmation.</p><p class="muted">Prototype isolé : événements, positions, visibilité et compteurs préparés. Aucun PV, dégâts de segment, IA réelle, soin, loot, XP, interface Familiers, sauvegarde ou raccordement à la campagne. Le placement du cercle est refusé si son pourtour est bloqué ; les murs coupent les dessins. Cette revue n’ajoute pas de statut Saignement.</p><p><a href="rituels-haut-64.gif">Séquence des deux effets</a> · <a href="rituels-obstacle-64.gif">Pourtour bloqué</a> · <a href="rituels-attente-64.gif">Attente et effets persistants</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p><p class="muted">Dessins créés avec imagegen intégré : <a href="prison-source.png">source Prison</a> / <a href="imagegen-prison-prompt.txt">prompt</a> · <a href="reanimation-source.png">source Réanimation</a> / <a href="imagegen-reanimation-prompt.txt">prompt</a>.</p><script>SCRIPT</script></html>'''.replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
