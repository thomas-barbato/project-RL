"""ANIM-S14: isolated natural attacks for two approved64px forms."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_transformations as previous

ROOT = previous.ROOT
BASE = previous.OUT
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-40'
base = previous.base
encoding = previous.encoding


def protected():
    hashes = previous.protected()
    hashes.update({p.relative_to(ROOT).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in BASE.rglob('*') if p.is_file()})
    return hashes


def protect():
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT / 'review-protection.json').exists()
    base.dump(OUT / 'review-protection.json', protected())
    print(json.dumps({'protected_baseline': str(OUT)}))


def prepare(sources):
    assert (BASE / 'visual-acceptance-01.json').exists()
    packs = []
    for kind, source in sources.items():
        (OUT / f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as im:
            assert im.mode == 'RGBA' and im.getchannel('A').getextrema()[0] == 0
            assert im.width * 2 == im.height * 3
            side = im.width // 3
            atlas = Image.new('RGBA', (1536, 256))
            for i in range(6):
                x, y = (i % 3) * side, (i // 3) * side
                atlas.paste(im.crop((x, y, x + side, y + side)).resize((256, 256), Image.Resampling.LANCZOS), (i * 256, 0))
            atlas.save(OUT / f'{kind}-atlas.png')
            packs.append({'kind': kind, 'source_size': list(im.size), 'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(), 'layout': [3, 2], 'cell_size': [256, 256]})
    base.dump(OUT / 'drawing-pack.json', {'mode': 'Built-in imagegen', 'packing': packs, 'postprocessing': 'Equal-cell crop and uniform resize only, no repainting. Approved64px bodies unchanged.'})
    launcher = (BASE / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-transformations', '--lot-attaques-transformees')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher, encoding='utf-8-sig')
    print(json.dumps({'prepared': str(OUT), 'atlases': len(packs)}))


def save_animation(path, images, fps):
    # Short physical flashes and red drops need palette entries despite longer grey-floor waits.
    samples = [images[0].resize((256, round(images[0].height * 256 / images[0].width)))]
    for i in (19, 21, 23, 25, 27, 35, 47):
        if images[0].width == 576: samples.append(images[i].crop((160, 112, 432, 384)))
        else: samples.append(images[i].resize((384, round(images[i].height * 384 / images[i].width))))
    background = samples[0].quantize(colors=96, method=Image.Quantize.MEDIANCUT)
    colourful = [p for im in samples[1:] for p in im.getdata() if max(p) - min(p) > 24]
    assert colourful
    strip = Image.new('RGB', (256, (len(colourful) + 255) // 256)); strip.putdata(colourful)
    foreground = strip.quantize(colors=160, method=Image.Quantize.MEDIANCUT)
    palette = Image.new('P', (1, 1)); palette.putpalette(background.getpalette()[:288] + foreground.getpalette()[:480])
    quantized = [im.quantize(palette=palette, dither=Image.Dither.NONE) for im in images]
    bounds = [round(i * 1000 / fps / 10) * 10 for i in range(len(images) + 1)]
    quantized[0].save(path, save_all=True, append_images=quantized[1:], duration=[b-a for a,b in zip(bounds,bounds[1:])], loop=0, optimize=False, disposal=2)
    with Image.open(path) as check:
        assert check.size == images[0].size
        unique=set();elapsed=0
        for i in range(check.n_frames):
            check.seek(i);unique.add(hashlib.sha256(check.convert('RGB').tobytes()).hexdigest());elapsed+=check.info.get('duration',0)
        assert elapsed==bounds[-1]
        assert len(unique)>10 or ('bloque' in path.name and len(unique)==1), (path.name,len(unique))
        return {'file':path.name,'native_size':list(check.size),'duration_ms':elapsed,'distinct_frames':len(unique),'palette':'96 background +160 chromatic, common across frames'}


def package():
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8')); assert protected() == baseline
    encoding.OUT = OUT; encoding.convert()
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8')); assert len(entries) == 21
    sequences={};checks=[];animations=[];candidates=[]
    for e in entries:
        folder=OUT/'native'/e['folder'];timeline=json.loads((folder/'timeline.json').read_text(encoding='utf-8'))
        pair=[]
        for j,v in enumerate(e['views']):
            out=OUT/'frames'/v['variant'];out.mkdir(parents=True,exist_ok=True)
            x,y,w,h=v['crop'];bounds=(x,y,x+w,y+h);frames=[];diffs={}
            for i in range(96):
                with Image.open(folder/f'{i:03}.png') as im:frame=im.crop(bounds).convert('RGB')
                frame.save(out/f'{i:03}.png',compress_level=1);frames.append(frame)
            for i in e['control_frames']:
                with Image.open(folder/f'control-{i:03}.png') as im:diffs[i]=ImageChops.difference(frames[i],im.crop(bounds).convert('RGB'))
                for cy,row in enumerate(e['map'].splitlines()):
                    for cx,symbol in enumerate(row):
                        if symbol=='#':assert diffs[i].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None,(v['variant'],i,cx,cy)
            rows=[r['states'][j] for r in timeline];blocked=e['case']=='BLOQUE' and v['id']!='TR61-ONDE'
            assert rows[17]['launched'] is not blocked
            assert rows[17]['hits']==rows[47]['hits']  # No wall-clock extra strikes.
            if blocked:
                assert not rows[17]['hits'] and rows[17]['player']==[4,4] and rows[17]['cooldown']==0
                assert all(diffs[i].getbbox() is None for i in e['control_frames'])
            else:
                assert any(diffs[i].getbbox() is not None for i in (19,21,23,25,27,30))
                if v['id']=='M01-BOND':
                    assert rows[17]['player']==[4,2] and not rows[17]['hits']
                if v['id'] in ('M01-GRIFFE','M01-LACERATION'):
                    assert rows[47]['bleed']==3
                    assert diffs[47].getbbox() is not None
                if v['id']=='TR61-ONDE':assert rows[47]['stun']==1 and diffs[47].getbbox() is not None
                if e['quiet']:
                    assert rows[95]['turn']==0 and rows[95]['cooldown']==rows[17]['cooldown']
                    assert rows[95]['bleed']==rows[17]['bleed'] and rows[95]['stun']==rows[17]['stun']
                else:
                    assert rows[95]['turn']==3 and rows[95]['bleed']==rows[95]['stun']==0
                    assert diffs[95].getbbox() is None
            sequences[v['variant']]=frames;pair.append(frames)
            checks.append({'variant':v['variant'],'walls_clear':True,'blocked_refusal_checked':blocked,'no_clock_driven_hits':True,'turn_based_statuses':True,'movement_checked':v['id']=='M01-BOND'})
            if e['case']=='HAUT':
                animations.append(save_animation(OUT/f"{v['id']}-A01-64.gif",frames,24));candidates.append(v)
        if e['case'] in ('HAUT','OBSTACLE','BLOQUE'):
            boards=[]
            for i in range(96):
                board=Image.new('RGB',(1232,650),'#0b1218');d=ImageDraw.Draw(board)
                for j,v in enumerate(e['views']):
                    x=12+j*616;d.text((x,10),f"{v['number']} · {v['title']}",font=base.font(20,True),fill='#e0e7e7');d.text((x,40),v['variant'],font=base.font(15),fill='#a5b9c5');board.paste(pair[j][i],(x,66))
                boards.append(board)
            animations.append(save_animation(OUT/f"attaques-{e['folder'].lower()}-64.gif",boards,24))
    candidates.sort(key=lambda v:v['number'])
    overview=[]
    for frame in range(96):
        board=Image.new('RGB',(944,1578),'#0b1218');d=ImageDraw.Draw(board)
        for j,skill in enumerate(['M01-GRIFFE','TR61-COUP','M01-LACERATION','TR61-ECRASEMENT','M01-BOND','TR61-ONDE']):
            v=next(v for v in candidates if v['id']==skill);x=12+(j%2)*472;y=(j//2)*526
            d.text((x,y+8),f"{v['number']} · {v['title']}",font=base.font(20,True),fill='#e0e7e7')
            d.text((x,y+38),'64 px natifs · vue centrale',font=base.font(15),fill='#a5b9c5')
            board.paste(sequences[v['variant']][frame].crop((64,64,512,512)),(x,y+65))
        overview.append(board)
    animations.append(save_animation(OUT/'attaques-transformees-64.gif',overview,24))
    for source,name in [('src/agent_skill_lab_review/fire_painted/form_attacks.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]: (OUT/name).write_bytes((ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    base.dump(OUT/'review.json',{'batch':'ANIM-S14','status':'awaiting_visual_review','native_tile_size':64,'candidates':[{'number':v['number'],'id':v['id'],'name':v['title'],'variant':v['id']+'-A01','status':'awaiting_visual_review'} for v in candidates],
        'approved_bodies':'M01-BODY01 and TR61-BODY01, unchanged from accepted review39',
        'visuals':'Rapid painted four-claw sweeps, heavy ivory/slate impact fragments, compact two-cell leap, radial ground shock. Blood sprays on physical wounds; small falling drops on bleeding targets, orbiting ivory marker while stunned.',
        'rules_retained':'Known footprints and durations only:3 front cells for Laceration;2-cell clear path for Bond;1-cell contact hits and1-cell shock radius. Prepared successes show3-turn Bleeding and1-turn Stun, with cooldowns0/3/3/0/4/4. No damage/RNG/energy calculation.',
        'scope':'Isolated debug visual fixture, no campaign integration, HP, stats, loot, equipment, real AI, saves or extra leap damage. Native bodies stay frontal. Movement is logically applied immediately; drawing finishes a compact cosmetic leap.'})
    base.dump(OUT/'verification.json',{'protected_files_unchanged':len(baseline),'controls':checks,'animations':animations,'native_views':len(checks),'resampling_of_native_capture':False})
    write_html(entries,candidates)
    print(json.dumps({'views':len(checks),'animations':len(animations),'protected_files':len(baseline),'controls':'passed'}))


def write_html(entries,candidates):
    manifest=[{k:e[k] for k in ('folder','case','quiet','turn_frames','views')} for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return f%17;if(entries()[0].quiet)return Math.min(17+f,47);const start=turn===0?17:entries()[0].turn_frames[turn-1];const end=turn===3?95:turn===0?47:entries()[0].turn_frames[turn]-1;return Math.min(start+f,end);}
function ui(){cast.disabled=!ready||launched;next.disabled=!ready||!launched||turn>=3||elapsed<650||entries()[0].quiet;document.getElementById('status').textContent=!launched?'Aperçu avant lancement':`Tour ${turn} · ${entries()[0].quiet?'attente sans tour':'tours manuels'} · ${scenario.value==='BLOQUE'?'front bloqué : seule l’onde peut partir':''}`;}
async function load(){ready=false;ui();const n=++token;const views=entries().flatMap(e=>e.views);const data=await Promise.all(views.map(async v=>[v.id,await Promise.all(Array.from({length:96},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${v.variant}/${String(i).padStart(3,'0')}.png`;})))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas'))canvas.getContext('2d').drawImage(sequences.get(canvas.dataset.skill)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};
document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    cards=''.join(f'''<article><h2>{v['number']} · {v['title']}</h2><p class="muted">{v['id']}-A01</p><canvas data-skill="{v['id']}" width="576" height="576"></canvas><p><a href="{v['id']}-A01-64.gif">Séquence animée</a></p></article>''' for v in sorted(candidates,key=lambda v: ['M01-GRIFFE','TR61-COUP','M01-LACERATION','TR61-ECRASEMENT','M01-BOND','TR61-ONDE'].index(v['id'])))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Attaques transformées</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0;position:sticky;top:0;background:#0b1218;padding:12px 0;z-index:2}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Attaques du Prédateur et du Colosse</h1><p class="muted">ANIM-S14 · six animations proposées · cases natives64 px · corps validés conservés</p><div class="controls"><select id="scenario"><option value="HAUT">Vers le haut</option><option value="DROITE">Vers la droite</option><option value="GAUCHE">Vers la gauche</option><option value="BAS">Vers le bas</option><option value="OBSTACLE">Obstacle partiel</option><option value="BLOQUE">Front bloqué</option><option value="ATTENTE">Attente sans tour</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="pair">CARDS</main>
<p>Les attaques ont un impact bref ; leur animation se termine sans devoir passer un autre tour. Le Saignement reste visible sur les cibles pendant trois tours, le repère d’assommement pendant un tour. Les réussites sont préparées pour observer ces statuts : aucune chance n’est tirée. Attendre conserve leurs compteurs.</p><p>Bond est un déplacement de deux cases sur un trajet libre : aucun dégât ajouté. Le Prédateur rejoint sa case dès l’action ; le dessin accompagne ce déplacement par un saut compact. Les murs coupent les VFX et le front bloqué refuse les attaques de contact et le Bond. L’Onde conserve son rayon d’une case et ne repousse personne.</p><p class="muted">Prototype visuel isolé : aucun calcul de PV, dégâts, énergie, attributs, précision, IA, loot ou sauvegarde. Les six animations, le sang de blessures et le repère d’assommement attendent leur validation ; les corps restent ceux de la revue39 déjà acceptée.</p>
<p><a href="attaques-transformees-64.gif">Les six animations en vue centrale native</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p><p class="muted">Dessins créés avec imagegen intégré : <a href="claws-source.png">Griffures</a> / <a href="imagegen-claws-prompt.txt">prompt</a> · <a href="impact-source.png">Impacts</a> / <a href="imagegen-impact-prompt.txt">prompt</a> · <a href="blood-source.png">Sang des blessures</a> / <a href="imagegen-blood-prompt.txt">prompt</a>.</p><script>SCRIPT</script></html>'''.replace('CARDS',cards).replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--protect-only', action='store_true'); parser.add_argument('--prepare', action='store_true')
    for kind in ('claws', 'impact', 'blood'): parser.add_argument('--' + kind, type=Path)
    args = parser.parse_args()
    if args.protect_only: protect()
    elif args.prepare: prepare({k: getattr(args, k) for k in ('claws', 'impact', 'blood')})
    else: package()
