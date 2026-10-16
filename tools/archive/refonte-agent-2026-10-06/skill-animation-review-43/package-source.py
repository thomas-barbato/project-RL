"""ANIM-REACTION01: six isolated Cuirasse/Possede actions, retained native64px bodies."""
from __future__ import annotations
import argparse
import hashlib
import json
import time
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw
import prepare_agent_skill_animation_more_forms as previous

ROOT=previous.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-43'
base=previous.base
encoding=previous.engine.encoding
GEN=previous.GEN
SOURCES={'closed':'exec-2953210d-69f9-4643-9e51-ba53d2dd69c2.png','plates':'exec-552cc5a7-0bce-4f85-83b3-ed653b846cc7.png','bolt':'exec-deda9493-ebd7-4931-bcc7-ded315ae6ecb.png','pulse':'exec-c0a1ee88-89ca-4201-82fb-628304b55654.png'}
IDS=['TR63-HEURT','TR63-DECHARGE','TR63-FERMETURE','TR64-TRAIT','TR64-PULSATION','TR64-DRAIN']
ORDER=[0,3,1,4,2,5]


def protected():
    hashes=previous.protected()
    for p in previous.OUT.rglob('*'):
        if p.is_file():hashes[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def protect():
    assert not (OUT/'review-protection.json').exists()
    base.dump(OUT/'review-protection.json',protected())
    print(json.dumps({'protected_baseline':str(OUT)}))


def prepare():
    assert (previous.OUT/'visual-acceptance-01.json').exists()
    packs=[]
    for kind,name in SOURCES.items():
        source=GEN/name;(OUT/f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as im:
            assert im.mode=='RGBA' and im.getchannel('A').getextrema()[0]==0
            if kind=='closed':
                with Image.open(previous.OUT/'cuirasse-source.png') as open_body:
                    assert im.size==open_body.size
                    box=open_body.getchannel('A').point(lambda v:255 if v>32 else 0).getbbox()
                crop=im.crop(box);scale=min(62/crop.width,62/crop.height);w,h=round(crop.width*scale),round(crop.height*scale)
                tile=Image.new('RGBA',(64,64));tile.paste(crop.resize((w,h),Image.Resampling.LANCZOS),((64-w)//2,63-h));tile.save(OUT/'closed-64.png')
                packs.append({'kind':kind,'source_size':list(im.size),'bounds_fixed_to_approved_open_body':box,'native_size':[64,64],'postprocess':'Fixed alpha-bounds crop, uniform direct64px resize, no repainting'})
            else:
                assert im.width*2==im.height*3
                side=im.width//3;atlas=Image.new('RGBA',(1536,256))
                for i in range(6):
                    x,y=(i%3)*side,(i//3)*side;atlas.paste(im.crop((x,y,x+side,y+side)).resize((256,256),Image.Resampling.LANCZOS),(256*i,0))
                atlas.save(OUT/f'{kind}-atlas.png');packs.append({'kind':kind,'layout':[3,2],'cell_size':[256,256],'postprocess':'Equal-cell crop and uniform resize only'})
            packs[-1]['source_sha256']=hashlib.sha256(source.read_bytes()).hexdigest()
    base.dump(OUT/'drawing-pack.json',{'mode':'Built-in imagegen','packing':packs,'approved_reuse':['review41:open Cuirasse/Possede bodies','review40:physical impact and wound spray','review33/35:occult drain atlas and thick flow direction']})
    launcher=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-transformations-suite','--lot-cuirasse-possede-actions')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launcher,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'new_body_variant':1,'new_effect_atlases':3}))


def convert_stream():
    native=(OUT/'native').resolve();count=0;deadline=time.monotonic()+900
    while time.monotonic()<deadline:
        for folder in native.glob('*'):
            if not folder.is_dir() or not (folder/'timeline.json').exists():continue
            for raw in folder.glob('*.rgba'):
                assert raw.resolve().is_relative_to(native)
                data=raw.read_bytes();assert len(data)==1280*720*4
                png=raw.with_suffix('.png');Image.frombytes('RGBA',(1280,720),data).save(png,compress_level=1)
                with Image.open(png) as im:assert im.tobytes()==data
                raw.unlink();count+=1
        if (native/'capture.json').exists() and not list(native.rglob('*.rgba')):
            base.dump(OUT/'native-encoding-verification.json',{'converted_frames':count,'pixel_identical_to_native_readback':True,'resampling':False,'completed_folders_only':True})
            print(json.dumps({'losslessly_converted':count}));return
        time.sleep(1)
    raise RuntimeError('Native capture stream did not finish within15 minutes.')


def save_animation(path,images,fps):
    # Reserve entries for scarce red/violet edges and green armour, plus stable terrain greys.
    samples=[images[0].resize((256,round(images[0].height*256/images[0].width)),Image.Resampling.NEAREST)]
    for i in (19,21,23,25,27,35,47,70):
        samples.append(images[i].crop((144,48,464,400)) if images[0].width==576 else images[i].resize((384,round(images[i].height*384/images[i].width))))
    sheet=Image.new('RGB',(384,sum(p.height for p in samples)));y=0
    for sample in samples:sheet.paste(sample,(0,y));y+=sample.height
    colours=list(sheet.quantize(colors=64,method=Image.Quantize.MEDIANCUT).getpalette()[:192]);pixels=[p for im in samples[1:] for p in im.get_flattened_data()]
    for selected in ([p for p in pixels if p[1]>p[0]+30 and p[1]>p[2]+30],[p for p in pixels if p[0]>p[1]+30 and p[0]>p[2]+20],[p for p in pixels if p[2]>p[1]+25 and p[0]>p[1]+20]):
        if not selected:selected=pixels
        strip=Image.new('RGB',(256,(len(selected)+255)//256));strip.putdata(selected);colours+=strip.quantize(colors=64,method=Image.Quantize.MEDIANCUT).getpalette()[:192]
    palette=Image.new('P',(1,1));palette.putpalette(colours);qs=[im.quantize(palette=palette,dither=Image.Dither.NONE) for im in images]
    bounds=[round(i*1000/fps/10)*10 for i in range(len(images)+1)]
    qs[0].save(path,save_all=True,append_images=qs[1:],duration=[b-a for a,b in zip(bounds,bounds[1:])],loop=0,optimize=False,disposal=2)
    with Image.open(path) as im:
        elapsed=0;unique=set()
        for i in range(im.n_frames):im.seek(i);elapsed+=im.info.get('duration',0);unique.add(hashlib.sha256(im.convert('RGB').tobytes()).hexdigest())
        assert elapsed==bounds[-1]
        assert len(unique)>10 or 'bloque' in path.name or 'silence' in path.name
        return {'file':path.name,'native_size':list(im.size),'duration_ms':elapsed,'distinct_frames':len(unique),'palette':'64 background,64 green,64 red,64 violet; stable throughout sequence'}


def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'));assert protected()==baseline
    encoding.OUT=OUT;encoding.convert()
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'));assert len(entries)==30
    checks=[];sequences={};animations=[];candidates=[]
    for e in entries:
        folder=OUT/'native'/e['folder'];timeline=json.loads((folder/'timeline.json').read_text(encoding='utf-8'));pair=[]
        for j,v in enumerate(e['views']):
            out=OUT/'frames'/v['variant'];out.mkdir(parents=True,exist_ok=True);x,y,w,h=v['crop'];bounds=(x,y,x+w,y+h);frames=[];diffs={}
            for i in range(96):
                with Image.open(folder/f'{i:03}.png') as im:frame=im.crop(bounds).convert('RGB')
                frame.save(out/f'{i:03}.png',compress_level=1);frames.append(frame)
            for i in e['control_frames']:
                with Image.open(folder/f'control-{i:03}.png') as im:diffs[i]=ImageChops.difference(frames[i],im.crop(bounds).convert('RGB'))
                for cy,row in enumerate(e['map'].splitlines()):
                    for cx,symbol in enumerate(row):
                        if symbol=='#':assert diffs[i].crop((64*cx,64*cy,64*(cx+1),64*(cy+1))).getbbox() is None,(v['variant'],i,cx,cy)
            rows=[r['states'][j] for r in timeline]
            blocked=e['case']=='BLOQUE' and v['id'] in ('TR63-HEURT','TR64-TRAIT','TR64-DRAIN') or e['case']=='SILENCE' and v['id'] in ('TR64-PULSATION','TR64-DRAIN')
            assert rows[17]['launched'] is not blocked
            if blocked:
                assert all(r['cooldown']==r['resolutions']==0 and not r['hits'] for r in rows)
                assert all(diffs[i].getbbox() is None for i in e['control_frames'])
            else:
                assert all(r['resolutions']==1 for r in rows[17:])
                assert any(diffs[i].getbbox() is not None for i in (19,21,23,25,27,30))
                if v['id']=='TR63-FERMETURE':
                    assert rows[47]['closed']==3 and diffs[47].getbbox() is not None
                    if e['quiet']:assert rows[95]['closed']==3
                    else:assert rows[83]['closed']==1 and rows[84]['closed']==0
                    if e['case']=='AGIR':assert rows[48]['player']==[3,5] and rows[66]['extra_hit'] and rows[66]['closed']==1
                elif not (v['id']=='TR63-DECHARGE' and rows[17]['charged']):
                    assert diffs[35].getbbox() is None,('Transient VFX still visible',v['variant'])
                if e['case']=='RESERVE' and v['id']=='TR63-DECHARGE':assert rows[3]['charged'] and rows[17]['was_charged'] and not rows[17]['charged']
                if not e['quiet']:assert diffs[95].getbbox() is None
            pair.append(frames)
            if e['case']=='HAUT':sequences[v['id']]=frames;candidates.append(v);animations.append(save_animation(OUT/f"{v['id']}-A02-64.gif",frames,24))
            checks.append({'variant':v['variant'],'walls_clear':True,'atomic_refusal':blocked,'single_prepared_resolution':True,'closure_turn_timer':True,'no_real_stats':True})
        if e['case'] in ('HAUT','OBSTACLE','BLOQUE') or e['case']=='AGIR' and e['folder'].endswith('-3') or e['case']=='RESERVE' and e['folder'].endswith('-2') or e['case']=='SILENCE' and e['folder'].endswith('-3'):
            boards=[]
            for i in range(96):
                board=Image.new('RGB',(1232,650),'#0b1218');d=ImageDraw.Draw(board)
                for j,v in enumerate(e['views']):
                    x=12+j*616;d.text((x,10),f"{v['number']} · {v['title']}",font=base.font(20,True),fill='#e0e7e7');d.text((x,40),v['variant'],font=base.font(15),fill='#a5b9c5');board.paste(pair[j][i],(x,66))
                boards.append(board)
            animations.append(save_animation(OUT/f"actions-{e['folder'].lower()}-64.gif",boards,24))
    candidates.sort(key=lambda v:v['number']);overview=[]
    for i in range(96):
        board=Image.new('RGB',(944,1578),'#0b1218');d=ImageDraw.Draw(board)
        for j,k in enumerate(ORDER):
            v=candidates[k];x=12+(j%2)*472;y=(j//2)*526;d.text((x,y+8),f"{v['number']} · {v['title']}",font=base.font(19,True),fill='#e0e7e7');d.text((x,y+38),'64 px natifs · vue centrale',font=base.font(15),fill='#a5b9c5');board.paste(sequences[v['id']][i].crop((64,64,512,512)),(x,y+65))
        overview.append(board)
    animations.append(save_animation(OUT/'cuirasse-possede-actions-64.gif',overview,24))
    for source,name in [('src/agent_skill_lab_review/fire_painted/armoured_possessed_reactions.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]: (OUT/name).write_bytes((ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    base.dump(OUT/'verification.json',{'native_views':len(checks),'protected_files_unchanged':len(baseline),'controls':checks,'animations':animations,'resampling_of_native_capture':False})
    base.dump(OUT/'review.json',{'batch':'ANIM-REACTION01','status':'awaiting_visual_review','native_tile_size':64,'candidates':[{'number':v['number'],'id':v['id'],'name':v['title'],'variant':v['id']+'-A02','status':'awaiting_visual_review'} for v in candidates],
        'approved_bodies':'TR63-BODY01 and TR64-BODY01 retained; new temporary closed-plates variant for Fermeture only.',
        'excluded_trials':['first-capture-badge/**','first-capture-badge-02/**','native-check-01.png','native-check-02.png','first-capture-wall-audit.json'],
        'visuals':'Physical Heurt and radial chitin discharge; visible armour closure with28px plate badge for3 turns; swift red-black corporeal dart; organic radial pulse; thick single blood-coloured return filament for Drain.',
        'rules_retained':'Contact1 cell, discharge radius1, pulse radius2, Trait/Drain targets within4 cells. Closure3 turns; cooldowns0/4/6/0/4/4. Reserve consumption and permitted movement/strike events prepared. Silence case applies only to Possede; blocks Pulse/Drain but not Trait.',
        'scope':'No actual HP, healing, damage, energy, RNG, AI, inventory, equipment, loot, saves or campaign integration. No new Bleeding/Stun or permanent armour reduction. Geometry is a visual review example, not a complete game targeting contract.'})
    write_html(entries,candidates)
    print(json.dumps({'native_views':len(checks),'animations':len(animations),'protected_files':len(baseline),'controls':'passed'}))


def write_html(entries,candidates):
    manifest=[{k:e[k] for k in ('folder','case','quiet','turn_frames','views')} for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return f%17;if(entries()[0].quiet)return Math.min(17+f,47);const start=turn===0?17:entries()[0].turn_frames[turn-1];const end=turn===3?95:turn===0?47:entries()[0].turn_frames[turn]-1;return Math.min(start+f,end);}
function ui(){cast.disabled=!ready||launched;next.disabled=!ready||!launched||turn>=3||elapsed<650||entries()[0].quiet;document.getElementById('status').textContent=!launched?'Aperçu avant lancement':`Tour ${turn} · ${entries()[0].quiet?'attente sans tour':'tours manuels'} · protection ${Math.max(0,3-turn)} tours si Fermeture a été lancée`;}
async function load(){ready=false;ui();const n=++token;const data=await Promise.all(entries().flatMap(e=>e.views).map(async v=>[v.id,await Promise.all(Array.from({length:96},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${v.variant}/${String(i).padStart(3,'0')}.png`;})))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas'))canvas.getContext('2d').drawImage(sequences.get(canvas.dataset.skill)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};
document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    cards=''.join(f'<article><h2>{candidates[k]["number"]} · {candidates[k]["title"]}</h2><p class="muted">{IDS[k]}-A02</p><canvas data-skill="{IDS[k]}" width="576" height="576"></canvas><p><a href="{IDS[k]}-A02-64.gif">Séquence animée</a></p></article>' for k in ORDER)
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Cuirassé et Possédé</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0;position:sticky;top:0;background:#0b1218;padding:12px 0;z-index:2}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Actions du Cuirassé et du Possédé</h1><p class="muted">ANIM-REACTION01 · six animations proposées · cases natives 64 px · corps validés conservés</p><div class="controls"><select id="scenario"><option value="HAUT">Vers le haut</option><option value="DROITE">Vers la droite</option><option value="GAUCHE">Vers la gauche</option><option value="BAS">Vers le bas</option><option value="OBSTACLE">Obstacle partiel</option><option value="BLOQUE">Cloison</option><option value="ATTENTE">Attente sans tour</option><option value="RESERVE">Réserve du Cuirassé préparée</option><option value="AGIR">Se déplacer et frapper sous protection</option><option value="SILENCE">Possédé réduit au silence</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="pair">CARDS</main>
<p>Les impacts, le projectile et le drain sont brefs. Fermeture des plaques garde sa nouvelle apparence pendant trois tours, avec un petit repère de plaque et son compteur. Attendre ne diminue ni protection ni recharges. Dans la scène « Se déplacer et frapper », un déplacement est préparé au premier tour, puis un Heurt au deuxième, pendant que la protection reste active.</p><p>La réserve préparée illustre sa consommation au lancement de Décharge. Les chiffres réels de réserve et de dégâts ne sont pas calculés. La scène Silence concerne uniquement le Possédé : Pulsation et Drain sont refusés, Trait corporel reste disponible. La cloison coupe les effets et refuse les cibles situées derrière elle. Les illustrations gardent les silhouettes de face.</p><p class="muted">Laboratoire visuel isolé : aucune résolution réelle de PV, soin, énergie, dégâts, résistances, précision, hasard, IA, inventaire, équipement, loot ou sauvegarde. Les six animations, l’apparence temporaire des plaques fermées et son petit repère attendent leur validation. Aucun Saignement ou assommement n’est ajouté à ces actions.</p>
<p><a href="cuirasse-possede-actions-64.gif">Les six animations en vue centrale native</a> · <a href="actions-agir-3-64.gif">Protection, déplacement et frappe</a> · <a href="actions-reserve-2-64.gif">Réserve préparée</a> · <a href="actions-silence-3-64.gif">Silence du Possédé</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p>
<p class="muted">Dessins créés avec imagegen intégré : <a href="closed-source.png">Plaques fermées</a> / <a href="imagegen-closed-prompt.txt">prompt</a> · <a href="plates-source.png">Décharge</a> / <a href="imagegen-plates-prompt.txt">prompt</a> · <a href="bolt-source.png">Trait corporel</a> / <a href="imagegen-bolt-prompt.txt">prompt</a> · <a href="pulse-source.png">Pulsation</a> / <a href="imagegen-pulse-prompt.txt">prompt</a>. Heurt et Drain réutilisent les dessins physiques et occultes déjà validés ; leur nouvelle composition reste à revoir.</p><script>SCRIPT</script></html>'''.replace('CARDS',cards).replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--protect-only',action='store_true');parser.add_argument('--prepare',action='store_true');parser.add_argument('--convert-stream',action='store_true');args=parser.parse_args()
    if args.protect_only:protect()
    elif args.prepare:prepare()
    elif args.convert_stream:convert_stream()
    else:package()
