"""ANIM-S11: native animated reviews of spirit siphon and voracious ray."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw
import prepare_agent_skill_animation_occult_purple as previous
from prepare_agent_skill_animation_revision import save_animation

ROOT=previous.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-33'
GEN=Path(r'C:\Users\User\.codex\generated_images\01a11010-433f-7fc2-ac29-ffb696dec5ea')
SOURCES={'spirit':'exec-26dc0977-8da6-410c-a4c0-e4cb52577db7.png','ray':'exec-1457caa6-6d9d-4918-b3d0-7a2b476e9833.png'}
base=previous.previous.previous.base


def protected():
    hashes=previous.protected()
    for p in (ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-32').rglob('*'):
        if p.is_file():hashes[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    protection=OUT/'review-protection.json'
    if protection.exists():
        assert protected()==json.loads(protection.read_text(encoding='utf-8'))
    else:base.dump(protection,protected())
    packs=[]
    for kind,name in SOURCES.items():
        source=GEN/name;(OUT/f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as image:
            assert image.mode=='RGBA' and image.size==(1536,1024)
            minimum,maximum=image.getchannel('A').getextrema()
            assert minimum==0 and maximum>0  # Keep genuine source alpha, even if it peaks at 254.
            atlas=Image.new('RGBA',(1536,256))
            for i in range(6):
                x=i%3*512;y=i//3*512
                atlas.paste(image.crop((x,y,x+512,y+512)).resize((256,256),Image.Resampling.LANCZOS),(i*256,0))
            atlas.save(OUT/f'{kind}-atlas.png')
        packs.append({'kind':kind,'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'source_size':[1536,1024],'layout':[3,2],'cell_size':[256,256],'alpha_preserved':True})
    base.dump(OUT/'drawing-pack.json',{'mode':'Built-in imagegen','packing':packs,'postprocessing':'Equal-cell cropping and uniform resize only; no repainting.'})
    base.dump(OUT/'review.json',{'batch':'ANIM-S11','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'id':sid,'variant':sid+'-A01','name':title,'status':'awaiting_visual_review'} for n,sid,title in [(23,'HY05','Ponction spirituelle'),(24,'OC05','Rayon vorace')]],
        'spirit':'Brief shadow contraction below the target, then red essence shards travel toward the caster once. Full .48-second cosmetic sequence.',
        'ray':'Persistent black filament with red veins and saturated purple edge facets. Red pulse toward the caster only on each prepared drain; brief .12-second fade when the link breaks.',
        'timing':'One siphon; maximum three prepared ray resolutions including launch. Only explicit turns add drains or reduce cooldowns; waiting keeps the filament animated without repeating gameplay.',
        'palette_reference':'ANIM-S10-COLOR03; retain the separately accepted crimson fragments for drains.',
        'scope':'Isolated debug fixture, no actual HP, energy, damage, healing, RNG, real AI/FOV, status, save or campaign changes. Numeric curves and targeting contracts are preserved.'})
    launch=(ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-32/Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--lot-occulte-impacts' '--occulte-violet-franc'","'--lot-occulte-drains'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'atlases':2}))


def convert():
    native=(OUT/'native').resolve();count=0
    for raw in native.rglob('*.rgba'):
        assert raw.resolve().is_relative_to(native)
        data=raw.read_bytes();assert len(data)==1280*720*4
        png=raw.with_suffix('.png');Image.frombytes('RGBA',(1280,720),data).save(png,compress_level=1)
        with Image.open(png) as image:assert image.tobytes()==data
        raw.unlink();count+=1
    if count:base.dump(OUT/'native-encoding-verification.json',{'converted_frames':count,'pixel_identical_to_native_readback':True,'resampling':False})
    else:assert (OUT/'native-encoding-verification.json').exists()


def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline
    convert()
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'));assert len(entries)==8
    sequences={};checks=[];animations=[]
    for e in entries:
        folder=OUT/'native'/e['folder'];timeline=json.loads((folder/'timeline.json').read_text(encoding='utf-8'))
        for j,v in enumerate(e['views']):
            target=OUT/'frames'/v['variant'];target.mkdir(parents=True,exist_ok=True)
            x,y,w,h=v['crop'];bounds=(x,y,x+w,y+h);frames=[]
            for i in range(e['frames']):
                with Image.open(folder/f'{i:03}.png') as im:frame=im.crop(bounds).convert('RGB')
                frame.save(target/f'{i:03}.png',compress_level=1);frames.append(frame)
            diffs={}
            for p in e['control_frames']:
                with Image.open(folder/f'control-{p:03}.png') as im:diffs[p]=ImageChops.difference(frames[p],im.crop(bounds).convert('RGB'))
                for cy,row in enumerate(e['map'].splitlines()):
                    for cx,symbol in enumerate(row):
                        if symbol=='#':assert diffs[p].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None,(v['variant'],p,cx,cy)
            rows=[r['states'][j] for r in timeline];cd=6 if v['valid'] else 0
            assert all(r['turn']==0 and r['cooldown']==cd for r in rows[17:60])
            assert [rows[p]['cooldown'] for p in e['turn_frames']]==[max(0,cd-t) for t in (1,2,3)]
            assert [rows[p]['turn'] for p in e['turn_frames']]==[1,2,3]
            assert rows[-1]['resolutions']==v['expected_resolutions']
            if v['valid']:
                assert all(r['resolutions']==1 for r in rows[17:60])
                assert any(diffs[p].getbbox() for p in (19,21,24))
                if j==0:
                    assert all(r['resolutions']==1 for r in rows[17:])
                    assert all(diffs[p].getbbox() is None for p in (35,47,59,64,69,87,94,120,125,143))
                else:
                    assert diffs[47].getbbox() and diffs[59].getbbox(),'Channel disappeared while waiting.'
                    if e['interruption']:
                        assert all(r['resolutions']==1 and not r['channel'] for r in rows[60:])
                        assert all(diffs[p].getbbox() is None for p in (64,69,87,94,120,125,143))
                    else:
                        assert all(r['resolutions']==2 and r['channel'] for r in rows[60:88])
                        assert all(r['resolutions']==3 and r['channel'] for r in rows[88:116])
                        assert all(r['resolutions']==3 and not r['channel'] for r in rows[116:])
                        assert all(diffs[p].getbbox() is None for p in (120,125,143))
                cx,cy=v['target_cell'];sample=(cx*64+3,cy*64+25)
                assert frames[3].getpixel(sample)[1]>frames[14].getpixel(sample)[1]+25
            else:
                assert all(r['resolutions']==0 and not r['launched'] and r['cooldown']==0 for r in rows)
                assert all(d.getbbox() is None for d in diffs.values())
            sequences[v['variant']]=frames
            if v['valid'] and e['folder'] in ('HAUT','DEPLACEMENT','ENERGIE'):
                animations.append(save_animation(OUT/f"{v['variant']}-64.gif",frames,24))
            checks.append({'variant':v['variant'],'solid_walls_clear':True,'prepared_counts_checked':True,'timers_turn_driven':True,'siphon_is_once':True,'ray_persists_while_waiting':True,'interruptions_checked':True,'preview_blinks_and_ends':True})
        if e['folder']=='HAUT':
            for p in (14,19,24,47,64,94,125):
                with Image.open(folder/f'{p:03}.png') as im:im.save(OUT/f'phase-{p:03}-native.png')
    for scenario in ('HAUT','DEPLACEMENT','ENERGIE'):
        boards=[]
        for i in range(144):
            board=Image.new('RGB',(1232,650),'#0b1218');draw=ImageDraw.Draw(board)
            for j,(sid,title) in enumerate([('HY05','23 · Ponction spirituelle'),('OC05','24 · Rayon vorace')]):
                x=12+j*616;variant=f'{sid}-A01-{scenario}'
                draw.text((x,10),title,font=base.font(20,True),fill='#e0e7e7');draw.text((x,40),variant,font=base.font(15),fill='#a5b9c5')
                board.paste(sequences[variant][i],(x,66))
            boards.append(board)
        animations.append(save_animation(OUT/f'drains-{scenario.lower()}-64.gif',boards,24))
    base.dump(OUT/'verification.json',{'protected_files_unchanged':len(baseline),'controls':checks,'animations':animations,'resampling_of_native_capture':False})
    for source,name in [('src/agent_skill_lab_review/fire_painted/occult_drains.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries)
    print(json.dumps({'native_views':16,'animations':len(animations),'protected_files':len(baseline),'controls':'passed'}))


def write_html(entries):
    manifest=[{'folder':e['folder'],'views':[{k:v[k] for k in ('id','variant','valid')} for v in e['views']],'interruption':e['interruption']} for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),starts=[17,60,88,116],ends=[59,87,115,143];
function entry(){return manifest.find(e=>e.folder===scenario.value);}
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return f%17;if(entry().views[1].valid&&turn<3&&(turn===0||!entry().interruption)){const start=[36,72,100][turn];return f<=ends[turn]-starts[turn]?starts[turn]+f:start+(f-(ends[turn]-starts[turn]+1))%(ends[turn]-start+1);}return Math.min(starts[turn]+f,ends[turn]);}
function ui(){cast.disabled=!ready||launched;next.disabled=!ready||!launched||turn>=3||elapsed<500;const valid=entry().views[1].valid;const count=valid?(entry().interruption?1:Math.min(3,turn+1)):0;document.getElementById('status').textContent=launched?`Tour ${turn} · drains du Rayon ${count}/3 · recharge ${valid?Math.max(0,6-turn):0} tours`:'Aperçu clignotant · avant lancement';}
async function load(){ready=false;ui();const n=++token,e=entry();const loaded=await Promise.all(e.views.map(async v=>[v.id,await Promise.all(Array.from({length:144},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${v.variant}/${String(i).padStart(3,'0')}.png`;})))]));if(n!==token)return;sequences=new Map(loaded);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas'))canvas.getContext('2d').drawImage(sequences.get(canvas.dataset.skill)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Drains occultes</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Magie occulte · Ponction et Rayon vorace</h1><p class="muted">ANIM-S11 · deux propositions animées · cases natives 64 px</p><div class="controls"><select id="scenario"><option value="HAUT">Vers le haut</option><option value="DROITE">Vers la droite</option><option value="GAUCHE">Vers la gauche</option><option value="BAS">Vers le bas</option><option value="OBSTACLE">Obstacle partiel</option><option value="CLOISON">Cloison · cible inaccessible</option><option value="DEPLACEMENT">Déplacement au tour 1</option><option value="ENERGIE">Énergie épuisée au tour 1</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="pair"><article><h2>23 · Ponction spirituelle</h2><p class="muted">HY05-A01 · ombre contractée, essence aspirée une fois</p><canvas data-skill="HY05" width="576" height="576"></canvas></article><article><h2>24 · Rayon vorace</h2><p class="muted">OC05-A01 · lien persistant, pulsation par drain</p><canvas data-skill="OC05" width="576" height="576"></canvas></article></main>
<p>Ponction spirituelle contracte l’ombre de la cible, puis aspire des éclats rouges vers l’agent. Cette animation brève se termine seule.</p><p>Rayon vorace conserve son lien animé pendant l’attente. Chaque tour explicite peut déclencher une nouvelle pulsation vers l’agent, jusqu’à trois drains en comptant le lancement. Dans les exemples d’interruption, le lien se rompt au déplacement ou au manque d’énergie.</p><p>Les aperçus clignotent avant lancement et disparaissent à la confirmation. La cloison refuse les deux cibles inaccessibles. Les quatre orientations gardent les personnages de face. Le lecteur ne passe aucun tour automatiquement ; les GIFs montrent un déroulement préparé.</p><p class="muted">Prototype isolé : positions et compteurs préparés, aucun PV, énergie, dégât, soin, hasard, statut ou IA réelle calculé. Ces deux nouveaux rendus restent à valider avant intégration au combat.</p><p><a href="drains-haut-64.gif">Séquence des deux effets</a> · <a href="drains-deplacement-64.gif">Arrêt au déplacement</a> · <a href="drains-energie-64.gif">Arrêt par manque d’énergie</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p><p class="muted">Dessins créés avec imagegen intégré : <a href="spirit-source.png">source Ponction</a> / <a href="imagegen-spirit-prompt.txt">prompt</a> · <a href="ray-source.png">source Rayon</a> / <a href="imagegen-ray-prompt.txt">prompt</a>.</p><script>SCRIPT</script></html>'''.replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
