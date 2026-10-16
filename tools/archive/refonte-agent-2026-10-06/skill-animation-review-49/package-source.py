"""ANIM-S20: atlas packing and native swap/domination review, no raster repainting."""
import argparse
import hashlib
import json
import re
from pathlib import Path
from PIL import Image,ImageChops
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_hybrids as previous
ROOT=base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-49'

def prepare():
    assert not (OUT/'visual-acceptance-01.json').exists()
    with Image.open(OUT/'fragments-source.png') as image:
        assert image.mode=='RGBA' and image.width*2==image.height*3
        assert image.getchannel('A').getextrema()[0]==0
        side=image.width//3;atlas=Image.new('RGBA',(1536,256))
        for frame in range(6):
            x,y=(frame%3)*side,(frame//3)*side
            cell=image.crop((x,y,x+side,y+side));assert cell.getchannel('A').getbbox()
            atlas.paste(cell.resize((256,256),Image.Resampling.LANCZOS),(frame*256,0))
        atlas.save(OUT/'fragments-atlas.png')
    with Image.open(OUT/'sigil-source.png') as image:
        assert image.mode=='RGBA' and image.getchannel('A').getextrema()[0]==0
        box=image.getchannel('A').point(lambda a:255 if a>32 else 0).getbbox();crop=image.crop(box)
        scale=28/max(crop.size);w,h=round(crop.width*scale),round(crop.height*scale)
        tile=Image.new('RGBA',(32,32));tile.paste(crop.resize((w,h),Image.Resampling.LANCZOS),((32-w)//2,(32-h)//2));tile.save(OUT/'sigil-32.png')
    base.dump(OUT/'drawing-pack.json',{'tool':'Built-in imagegen','postprocessing':'Equal frame crops, alpha-bound symbol crop, direct resizing; no repainting.',
        'fragments_layout':[3,2],'atlas_size':[1536,256],'sigil_canvas_size':[32,32],'sigil_occupied_size':[w,h],
        'sources':{name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in ['fragments-source.png','sigil-source.png']}})
    prior=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-48'
    receipt=json.loads((prior/'visual-acceptance-01.json').read_text(encoding='utf-8'));proof={}
    for name,digest in receipt['artifact_hashes'].items():
        path=prior/name;assert hashlib.sha256(path.read_bytes()).hexdigest()==digest;proof[path.relative_to(ROOT).as_posix()]=digest
    for relative in ['artifacts/refonte-agent-2026-10-06/skill-animation-review-28/gel-atlas.png','artifacts/refonte-agent-2026-10-06/skill-animation-review-39/predator-64.png']:
        proof[relative]=hashlib.sha256((ROOT/relative).read_bytes()).hexdigest()
    base.dump(OUT/'review-protection.json',proof)
    launcher=(prior/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-hybrides','--lot-echange-domination')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launcher,encoding='utf-8-sig')
    print(json.dumps({'new_assets':2,'protected_files':len(proof)}))

def save_animation(path,images,allow_static=False):
    samples=[]
    for offset in range(0,len(images),96):
        for frame in [3,14,17,20,22,25,35,47,60,84]:
            image=images[offset+frame];samples.append(image.resize((384,round(image.height*384/image.width)),Image.Resampling.NEAREST))
    sheet=Image.new('RGB',(384,sum(p.height for p in samples)));y=0
    for image in samples:sheet.paste(image,(0,y));y+=image.height
    pixels=list(sheet.get_flattened_data());colours=sheet.quantize(colors=126,method=Image.Quantize.MEDIANCUT).getpalette()[:378]+[224,231,231,165,185,197]
    for count,selected in [(96,[p for p in pixels if p[2]>p[1]+15 and p[0]>p[1]+8]),(32,[p for p in pixels if p[1]>p[0]+20 and p[1]>p[2]+20])]:
        selected=selected or pixels;strip=Image.new('RGB',(256,(len(selected)+255)//256));strip.putdata(selected)
        colours+=strip.quantize(colors=count,method=Image.Quantize.MEDIANCUT).getpalette()[:count*3]
    palette=Image.new('P',(1,1));palette.putpalette(colours)
    encoded=[image.quantize(palette=palette,dither=Image.Dither.NONE) for image in images]
    bounds=[round(i*1000/24/10)*10 for i in range(len(images)+1)]
    encoded[0].save(path,save_all=True,append_images=encoded[1:],duration=[b-a for a,b in zip(bounds,bounds[1:])],loop=0,disposal=2,optimize=False)
    with Image.open(path) as gif:
        elapsed=0;unique=set()
        for frame in range(gif.n_frames):
            gif.seek(frame);elapsed+=gif.info['duration'];unique.add(hashlib.sha256(gif.convert('RGB').tobytes()).hexdigest())
        assert elapsed==bounds[-1]
        assert len(unique)>=1 if allow_static else len(unique)>10
    errors=[]
    for frame in [20,22,35,47]:
        for original,indexed in zip(images[frame].get_flattened_data(),encoded[frame].convert('RGB').get_flattened_data()):
            if original[2]>original[1]+25 and original[0]>original[1]+15 and original[2]>110:
                errors.append(max(abs(a-b) for a,b in zip(original,indexed)))
    mean=sum(errors)/len(errors) if errors else 0;p95=sorted(errors)[int((len(errors)-1)*.95)] if errors else 0
    assert mean<18 and p95<45,(path.name,mean,p95)
    return {'file':path.name,'native_size':list(images[0].size),'duration_ms':elapsed,'distinct_frames':len(unique),'purple_mean_error':round(mean,2),'purple_p95_error':p95}

def package():
    assert not (OUT/'visual-acceptance-01.json').exists()
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert all(hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
    frames_root=OUT/'frames';frames_root.mkdir(exist_ok=True);checks=[];animations=[];overview=[]
    for entry in entries:
        native=OUT/'native'/entry['folder'];timeline=json.loads((native/'timeline.json').read_text(encoding='utf-8'))
        originals=[Image.open(native/f'{n:03}.png').convert('RGB') for n in range(96)]
        frames=[image.crop((32,88,608,664)) for image in originals]
        folder=frames_root/entry['variant'];folder.mkdir(exist_ok=True)
        for n,image in enumerate(frames):image.save(folder/f'{n:03}.png',compress_level=1)
        differences={}
        for frame in entry['control_frames']:
            control=Image.open(native/f'control-{frame:03}.png').convert('RGB')
            difference=ImageChops.difference(originals[frame],control).crop((32,88,608,664));differences[frame]=difference
            for y,row in enumerate(entry['map'].splitlines()):
                for x,value in enumerate(row):
                    if value=='#':assert difference.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox() is None,(entry['variant'],frame,x,y)
        launch=timeline[17];assert launch['launched']==entry['permitted']
        if not entry['permitted']:
            assert all(d.getbbox() is None for d in differences.values())
            assert launch['cooldown']==0
        else:
            assert any(differences[f].getbbox() is not None for f in [17,20,22])
            for frame,row in enumerate(timeline):
                turns=sum(f<=frame for f in entry['turn_frames']) if frame>=17 else 0
                assert row['turn']==turns
                if frame>=17:assert row['cooldown']==max(0,entry['cooldown']-turns)
            if entry['id']=='HY04':
                assert launch['player']==[4,3] and launch['drone']==[4,6]
                assert all(row['cargo']==timeline[0]['cargo'] for row in timeline)
                assert all(differences[f].getbbox() is None for f in [25,35,47,48,60,72,84,95])
            else:
                assert launch['remaining']==4 and launch['controlled']==[4,3]
                assert [4,3] not in launch['pet_targets']
                if entry['case']=='REMPLACEMENT':
                    assert timeline[14]['controlled']==[4,4] and [4,4] in launch['pet_targets']
                for frame,row in enumerate(timeline[17:],17):
                    turns=sum(f<=frame for f in entry['turn_frames'])
                    assert row['remaining']==max(0,4-turns)
                    assert row['controlled']==([4,3] if row['remaining'] else None)
                assert differences[35].getbbox() is not None and differences[47].getbbox() is not None
                if entry['quiet']:assert differences[95].getbbox() is not None and timeline[95]['remaining']==4
                else:assert differences[95].getbbox() is None and [4,3] in timeline[95]['pet_targets']
        checks.append({'variant':entry['variant'],'walls_clear':True,'timers_require_explicit_turns':True,'permitted':entry['permitted'],
            'positions_cargo_or_status_targets_checked':True,'preview_ends_on_launch':True})
        animations.append(save_animation(OUT/f"{entry['variant']}-64.gif",frames,allow_static=not entry['permitted']))
        if entry['case']=='HAUT':
            overview.extend(originals)
            for phase in [14,17,20,22,25,35,47,60,84]:originals[phase].save(OUT/f"{entry['id']}-phase-{phase:03}.png")
    animations.append(save_animation(OUT/'echange-domination-64.gif',overview))
    base.dump(OUT/'verification.json',{'native_views':len(entries),'controls':checks,'animations':animations,'protected_files_unchanged':len(proof),'resampling_of_native_capture':False})
    base.dump(OUT/'review.json',{'batch':'ANIM-S20','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':49,'id':'HY04','name':'Échange de position','variant':'HY04-A01'},{'number':50,'id':'HY06','name':'Lien de domination','variant':'HY06-A01'}],
        'scope':'Prepared visuals only, no campaign combat, health, energy, cargo, RNG, AI or save integration.',
        'visuals':'Two dark fragment clouds and rapid fade/reformation on exchanged cells; a persistent upright purple occult symbol above the dominated target; common victim recoil.',
        'retained_values':'HY04 range6/cost8/cooldown6. HY06 range4/cost19/cooldown10/duration4/maxone ally; no new chance calculation.',
        'prepared_cases':'Swap legal-destination and boss-immunity UI fixtures do not establish global distance, intermediate obstacle traversal, failed-cast costs, action ordering or monster AI. Existing pet target list switches alliance in the fixture only.',
        'temporary_actors':'Monsters and drones still use their existing32px drawings scaled to64; adaptation scheduled after skills/animations.'})
    write_html(entries)
    for source,name in [('src/agent_skill_lab_review/fire_painted/tactical_hybrids.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/combat_visuals.rs','reaction-source.rs')]:
        (OUT/name).write_bytes((ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'native_views':len(entries),'animations':len(animations),'protected_files':len(proof),'controls':'passed'}))

def write_html(entries):
    manifest=[{k:e[k] for k in ['case','variant','id','title','permitted','quiet','cooldown','turn_frames']} for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(!e.permitted)return 14;
const step=Math.floor(elapsed*24/1000);if(turn===0){if(step<18)return 17+step;return 35+step%12;}
const base=[48,60,72,84][Math.min(3,turn-1)];if(e.id==='HY06'&&turn===1&&step>8)return 56+step%4;return base+Math.min(11,step%12);}
function ui(){const es=entries();next.disabled=!ready||!launched||!es.some(e=>e.permitted)||es.every(e=>e.quiet)||turn>=4;cast.disabled=!ready||launched||!es.some(e=>e.permitted);status.textContent=!ready?'Chargement…':!launched?'Aperçu clignotant · confirmer pour lancer':'Tour '+turn+' · '+es.map(e=>e.title+(e.permitted?' : recharge '+Math.max(0,e.cooldown-turn)+' tours'+(e.id==='HY06'?' · domination '+Math.max(0,4-turn)+' tours':''):' : indisponible')).join(' · ');}
async function load(){ready=false;ui();const n=++token,es=entries();const data=await Promise.all(es.map(async e=>[e.id,await Promise.all(Array.from({length:96},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src='frames/'+e.variant+'/'+String(i).padStart(3,'0')+'.png';})))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===canvas.dataset.skill);canvas.parentElement.hidden=!e;if(e)canvas.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};
document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};
document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    labels={'HAUT':'Vers le haut','DROITE':'Vers la droite','GAUCHE':'Vers la gauche','BAS':'Vers le bas','CLOISON':'Mur','ATTENTE':'Attente sans tour','SILENCE':'Silence','CACHE':'Cible cachée',
        'GELE':'Agent gelé','NON-CONTROLE':'Drone non contrôlé','DESTINATION':'Destination illégale','MULE':'Mule et cargaison','ECLAIREUR':'Éclaireur','TRANSFORME':'Forme transformée',
        'BOSS':'Boss immunisé','HORS-PORTEE':'Cible hors portée','DEGATS':'Dégâts reçus pendant le lien','REMPLACEMENT':'Ancien lien remplacé'}
    options=''.join(f'<option value="{key}">{value}</option>' for key,value in labels.items())
    cards=''.join(f'<article><h2>{n} · {title}</h2><p class="muted">{sid}-A01 · proposé</p><canvas data-skill="{sid}" width="576" height="576"></canvas></article>' for n,sid,title in [(49,'HY04','Échange de position'),(50,'HY06','Lien de domination')])
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Échange et domination</title><style>*{box-sizing:border-box}body{margin:auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}[hidden]{display:none}@media(max-width:1296px){.grid{grid-template-columns:608px}}</style><h1>Échange de position et Lien de domination</h1><p class="muted">ANIM-S20 · candidats49–50 · cases natives64px · rendu à valider</p><div class="controls"><select id="scenario">OPTIONS</select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="grid">CARDS</main>
<p>Échange de position décompose brièvement l’agent et le drone en particules sombres, puis les reforme sur leurs cases échangées. La Mule conserve sa cargaison dans le cas préparé. Il n’y a pas de trajet animé entre les deux positions.</p><p>Lien de domination fait apparaître un symbole occulte violet au-dessus du monstre. Il reste animé pendant les quatre tours du lien, sans collier ni fil vers l’agent. La cible réagit à l’application de l’effet négatif. Les cas montrent le lien conservé après des dégâts, son expiration et le remplacement d’une cible précédemment dominée.</p><p>Dans la scène, le drone considère la cible dominée comme alliée et retire son nom de sa liste de cibles préparée. Aucune IA réelle ni attaque automatique de campagne n’est calculée. Les aperçus disparaissent au lancement ; l’attente ne fait pas avancer les durées ou les recharges.</p><p class="muted">Les refus et les destinations sont des cas préparés : ils ne définissent pas les diagonales, le franchissement d’obstacles intermédiaires ou le coût d’un contrôle raté. Aucun dégât chiffré, PV, énergie, loot, inventaire, chance de réussite ou sauvegarde de campagne n’est résolu ici. Monstres et drones encore en32px agrandis, à adapter après les compétences et animations.</p><p><a href="echange-domination-64.gif">Les deux animations à la suite en taille native</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> · <a href="review.json">Fiche de revue</a></p><p class="muted">Dessins avec imagegen intégré : <a href="fragments-source.png">particules</a> / <a href="imagegen-fragments-prompt.txt">prompt</a> · <a href="sigil-source.png">symbole</a> / <a href="imagegen-sigil-prompt.txt">prompt</a>.</p><script src="gallery-script.js"></script></html>'''.replace('OPTIONS',options).replace('CARDS',cards)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')
    links=set(re.findall(r'(?:href|src)="([^"]+)"',html));assert all((OUT/link).is_file() for link in links)
    base.dump(OUT/'links-verification.json',{'checked':len(links),'missing':0})

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');parser.add_argument('--convert-stream',action='store_true');args=parser.parse_args()
    if args.prepare:prepare()
    elif args.convert_stream:previous.OUT=OUT;previous.convert()
    else:package()
