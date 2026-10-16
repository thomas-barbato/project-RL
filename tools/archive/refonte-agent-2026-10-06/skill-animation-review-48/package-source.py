"""ANIM-S19: bitmap atlas packing, native readback conversion and review package."""
import argparse
import hashlib
import json
import re
import time
from pathlib import Path
from PIL import Image, ImageChops
import prepare_agent_skill_animation_review as base

ROOT=base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-48'

def save_animation(path,images,allow_static=False):
    samples=[]
    for offset in range(0,len(images),96):
        for frame in [3,14,17,20,22,25,28,35]:
            image=images[offset+frame]
            samples.append(image.resize((384,round(image.height*384/image.width)),Image.Resampling.NEAREST))
    sheet=Image.new('RGB',(384,sum(p.height for p in samples)))
    y=0
    for image in samples:sheet.paste(image,(0,y));y+=image.height
    pixels=list(sheet.get_flattened_data())
    colours=sheet.quantize(colors=94,method=Image.Quantize.MEDIANCUT).getpalette()[:282]+[224,231,231,165,185,197]
    for count,selected in [(96,[p for p in pixels if p[1]>p[0]+20 and p[1]>p[2]+20]),
                           (64,[p for p in pixels if p[0]>p[1]+25 and p[0]>p[2]+25])]:
        selected=selected or pixels
        strip=Image.new('RGB',(256,(len(selected)+255)//256));strip.putdata(selected)
        colours+=strip.quantize(colors=count,method=Image.Quantize.MEDIANCUT).getpalette()[:count*3]
    palette=Image.new('P',(1,1));palette.putpalette(colours)
    encoded=[image.quantize(palette=palette,dither=Image.Dither.NONE) for image in images]
    bounds=[round(i*1000/24/10)*10 for i in range(len(images)+1)]
    encoded[0].save(path,save_all=True,append_images=encoded[1:],duration=[b-a for a,b in zip(bounds,bounds[1:])],loop=0,disposal=2,optimize=False)
    with Image.open(path) as decoded:
        elapsed=0;unique=set()
        for frame in range(decoded.n_frames):
            decoded.seek(frame);elapsed+=decoded.info['duration'];unique.add(hashlib.sha256(decoded.convert('RGB').tobytes()).hexdigest())
        assert elapsed==bounds[-1]
        assert len(unique)>=1 if allow_static else len(unique)>10
    return {'file':path.name,'native_size':list(images[0].size),'duration_ms':elapsed,'distinct_frames':len(unique),
        'palette':'94 neutral +2 text +96 green +64 red; shared across frames'}

def prepare():
    assert not (OUT/'visual-acceptance-01.json').exists()
    packs=[]
    for kind in ['shot','impact']:
        path=OUT/f'{kind}-source.png'
        with Image.open(path) as image:
            assert image.mode=='RGBA'
            alpha_min,alpha_max=image.getchannel('A').getextrema()
            assert alpha_min==0 and alpha_max>=250
            assert image.width*2==image.height*3
            side=image.width//3
            atlas=Image.new('RGBA',(1536,256))
            for frame in range(6):
                x,y=(frame%3)*side,(frame//3)*side
                cell=image.crop((x,y,x+side,y+side))
                assert cell.getchannel('A').getbbox()
                atlas.paste(cell.resize((256,256),Image.Resampling.LANCZOS),(frame*256,0))
            atlas.save(OUT/f'{kind}-atlas.png')
            packs.append({'kind':kind,'source_size':list(image.size),'layout':[3,2],'cell_size':[256,256],'source_sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
    base.dump(OUT/'drawing-pack.json',{'tool':'Built-in imagegen','postprocessing':'Equal cell crop and direct resize only; no raster repainting.','atlases':packs,
        'fire_reused':'review20/burst-atlas.png','bodies_reused':['review39/predator-64.png','review39/colossus-64.png','review41/cuirasse-64.png','review47/possede-64.png','review44/abysse-64.png']})
    launcher=(ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-45/Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-abysse-actions','--lot-hybrides')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launcher,encoding='utf-8-sig')
    proof={}
    for number in [45,47]:
        folder=ROOT/f'artifacts/refonte-agent-2026-10-06/skill-animation-review-{number:02}'
        receipt=json.loads((folder/'visual-acceptance-01.json').read_text(encoding='utf-8'))
        for name,digest in receipt['artifact_hashes'].items():
            path=folder/name
            assert hashlib.sha256(path.read_bytes()).hexdigest()==digest
            proof[path.relative_to(ROOT).as_posix()]=digest
    for path in [ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-20/burst-atlas.png',
        ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-39/predator-64.png',ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-39/colossus-64.png',
        ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-41/cuirasse-64.png',ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-44/abysse-64.png']:
        proof[path.relative_to(ROOT).as_posix()]=hashlib.sha256(path.read_bytes()).hexdigest()
    base.dump(OUT/'review-protection.json',proof)
    print(json.dumps({'atlases':2,'protected_files':len(proof)}))

def convert():
    native=OUT/'native';count=0;deadline=time.monotonic()+900
    while time.monotonic()<deadline:
        for folder in native.glob('*'):
            if not folder.is_dir() or not (folder/'timeline.json').exists():continue
            for path in folder.glob('*.rgba'):
                assert path.resolve().is_relative_to(native.resolve())
                data=path.read_bytes();assert len(data)==640*720*4
                png=path.with_suffix('.png')
                Image.frombytes('RGBA',(640,720),data).save(png,compress_level=1)
                with Image.open(png) as im:assert im.tobytes()==data
                path.unlink();count+=1
        if (native/'capture.json').exists() and not list(native.rglob('*.rgba')):
            base.dump(OUT/'native-encoding-verification.json',{'converted_frames':count,'pixel_identical_to_native_readback':True,'resampling':False})
            print(json.dumps({'converted_frames':count}));return
        time.sleep(1)
    raise RuntimeError('Native capture did not finish.')

def package():
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert all(hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest for name,digest in proof.items())
    frames_root=OUT/'frames';frames_root.mkdir(exist_ok=True)
    animations=[];checks=[];overview=[]
    for entry in entries:
        native=OUT/'native'/entry['folder']
        timeline=json.loads((native/'timeline.json').read_text(encoding='utf-8'))
        destination=frames_root/entry['variant'];destination.mkdir(exist_ok=True)
        originals=[Image.open(native/f'{n:03}.png').convert('RGB') for n in range(96)]
        frames=[im.crop((32,88,608,664)) for im in originals]
        for frame,im in enumerate(frames):im.save(destination/f'{frame:03}.png',compress_level=1)
        differences={}
        for frame in entry['control_frames']:
            control=Image.open(native/f'control-{frame:03}.png').convert('RGB')
            differences[frame]=ImageChops.difference(originals[frame],control).crop((32,88,608,664))
            # No world effect reaches wall cells, including adjacent victim recoil.
            for y,row in enumerate(entry['map'].splitlines()):
                for x,value in enumerate(row):
                    if value=='#':assert differences[frame].crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox() is None,(entry['variant'],frame,x,y)
        launch=timeline[17]
        assert launch['launched']==entry['permitted']
        assert all(differences[f].getbbox() is None for f in [35,47,48,66,84,95])
        if not entry['permitted']:
            assert all(d.getbbox() is None for d in differences.values())
            assert launch['cooldown']==0 and not launch['destroyed'] and not launch['hits']
        else:
            assert any(differences[f].getbbox() is not None for f in [17,20,22,25,28,32])
            assert launch['cooldown']==entry['cooldown']
            expected_gain=2 if entry['id']=='M02' and entry['form']>0 and entry['case'] not in ['SANS-PV','PLAFOND'] else 0
            assert launch['gained']==expected_gain
            for frame,row in enumerate(timeline):
                turns=sum(f<=frame for f in entry['turn_frames']) if frame>=17 else 0
                assert row['turn']==turns
                if frame>=17:assert row['cooldown']==max(0,entry['cooldown']-turns)
            if entry['id']=='HY07':
                assert launch['destroyed'] and [4,6] not in launch['hits'] and [3,4] not in launch['hits']
                assert len(launch['hits'])==(2 if entry['case']=='CLOISON' else 3)
        if entry['case']=='HAUT' or (entry['id']=='M02' and entry['case']=='POSSEDE'):
            for phase in [14,17,20,22,25,28,35]:originals[phase].save(OUT/f"{entry['id']}-{entry['case']}-phase-{phase:03}.png")
        if (entry['id']=='M02' and entry['case']=='POSSEDE') or (entry['id']=='HY07' and entry['case']=='HAUT'):overview.extend(originals)
        gif_path=OUT/f"{entry['variant']}-64.gif"
        animation=save_animation(gif_path,frames,allow_static=not entry['permitted'])
        green_errors=[]
        with Image.open(gif_path) as decoded:
            elapsed=0
            # GIF encoders merge identical frames: match native timestamps rather
            # than assuming one encoded frame for every24fps native readback.
            for gif_frame in range(decoded.n_frames):
                decoded.seek(gif_frame)
                end=elapsed+decoded.info['duration']
                for frame in [20,22,25,28]:
                    timestamp=round(frame*1000/24/10)*10
                    if elapsed<=timestamp<end:
                        for original,encoded in zip(frames[frame].get_flattened_data(),decoded.convert('RGB').get_flattened_data()):
                            if original[1]>original[0]+25 and original[1]>original[2]+25 and original[1]>100:
                                green_errors.append(max(abs(a-b) for a,b in zip(original,encoded)))
                elapsed=end
        if green_errors:
            mean=sum(green_errors)/len(green_errors);p95=sorted(green_errors)[int((len(green_errors)-1)*.95)]
            assert mean<18 and p95<45,(entry['variant'],mean,p95)
            animation['green_mean_error']=round(mean,2);animation['green_p95_error']=p95
        animations.append(animation)
        checks.append({'variant':entry['variant'],'walls_clear':True,'instant_vfx_end_before_frame35':True,'timers_require_explicit_turns':True,'permitted':entry['permitted']})
    animations.append(save_animation(OUT/'hybrides-64.gif',overview))
    base.dump(OUT/'verification.json',{'native_views':len(entries),'controls':checks,'animations':animations,'protected_files_unchanged':len(proof),'resampling_of_native_capture':False})
    base.dump(OUT/'review.json',{'batch':'ANIM-S19','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':47,'id':'M02','name':'Décharge mutagène','variant':'M02-A01'},{'number':48,'id':'HY07','name':'Autodestruction','variant':'HY07-A01'}],
        'scope':'Prepared visual fixtures only. No real damage, HP, energy, AI, attributes, loot, saves or campaign combat integration.',
        'rules_retained':'M02 displayed range3/cost9/cooldown2, form extension2 within shared4-turn cap only for actual HP removal. HY07 combat drones58/59/66 only, range6/radius2/cost6/cooldown4, enemies only, walls block. Silence prevents both; HY07 needs human form.',
        'visuals':'Green organic projectile and brief hit; short green form pulse on extension. Red drone core, fire and steel fragments; no new poison/burning/bleeding status.',
        'temporary_actors':'Monsters and drones use their existing32px drawings scaled to64. Player requested adaptation and extensive bestiary creation after skills/animations.'})
    write_html(entries)
    for source,name in [('src/agent_skill_lab_review/fire_painted/hybrids.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/combat_visuals.rs','reaction-source.rs')]:
        (OUT/name).write_bytes((ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'native_views':len(entries),'animations':len(animations),'protected_files':len(proof),'controls':'passed'}))

def write_html(entries):
    manifest=[{k:e[k] for k in ['case','variant','id','title','permitted','quiet','form','form_title','cooldown','turn_frames']} for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){const key=scenario.value;return manifest.filter(e=>e.case===key);}
function frame(e){if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(!e.permitted)return 14;const a=Math.min(24,Math.floor(elapsed*24/1000));if(turn===0)return 17+a;if(turn===1)return 48;if(turn===2)return 66;return 84;}
function ui(){const es=entries();next.disabled=!ready||!launched||!es.some(e=>e.permitted)||es.every(e=>e.quiet)||turn>=3;cast.disabled=!ready||launched||!es.some(e=>e.permitted);status.textContent=!ready?'Chargement…':!launched?'Aperçu clignotant · les cases indiquent les effets':'Tour '+turn+' · '+es.map(e=>e.title+(e.permitted?' : recharge '+Math.max(0,e.cooldown-turn)+' tours':' : lancement refusé')).join(' · ');}
async function load(){ready=false;ui();const n=++token,es=entries();const data=await Promise.all(es.map(async e=>[e.id,await Promise.all(Array.from({length:96},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src='frames/'+e.variant+'/'+String(i).padStart(3,'0')+'.png';})))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===canvas.dataset.skill);canvas.parentElement.hidden=!e;if(e)canvas.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};
document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};
document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    labels={'HAUT':'Vers le haut','DROITE':'Vers la droite','GAUCHE':'Vers la gauche','BAS':'Vers le bas','CLOISON':'Mur','ATTENTE':'Attente sans tour','SILENCE':'Silence',
        'PREDATEUR':'Mue prédatrice','COLOSSE':'Colosse','CUIRASSE':'Cuirassé','POSSEDE':'Possédé recoloré','ABYSSE':'Abysse','SANS-PV':'Sans PV retirés','PLAFOND':'Prolongation plafonnée',
        'ARTILLEUR':'Drone Artilleur','GARDIEN':'Drone Gardien','ECLAIREUR':'Éclaireur exclu','MULE':'Mule exclue','CACHE':'Drone caché','TRANSFORME':'Autodestruction sous forme'}
    options=''.join(f'<option value="{key}">{label}</option>' for key,label in labels.items())
    cards=''.join(f'<article><h2>{number} · {title}</h2><p class="muted">{sid}-A01 · proposé</p><canvas data-skill="{sid}" width="576" height="576"></canvas></article>' for number,sid,title in [(47,'M02','Décharge mutagène'),(48,'HY07','Autodestruction')])
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Pouvoirs hybrides</title><style>*{box-sizing:border-box}body{margin:auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}[hidden]{display:none}@media(max-width:1296px){.grid{grid-template-columns:608px}}</style><h1>Pouvoirs hybrides · deux animations</h1><p class="muted">ANIM-S19 · candidats 47–48 · cases natives 64 px · rendu à valider</p><div class="controls"><select id="scenario">OPTIONS</select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="grid">CARDS</main>
<p>Décharge mutagène projette une énergie verte organique vers la cible. Sous transformation, un impact admissible prolonge la forme et fait brièvement pulser son corps. Les cinq formes sont montrées, avec le Possédé aux muscles rouges et tendons chair validé. Les cas sans PV retirés et avec plafond atteint ne prolongent pas la forme.</p><p>Autodestruction fait rougir le cœur du drone sélectionné, puis le détruit dans une explosion de feu et de fragments. L’aperçu montre les cases touchées. L’autre drone et l’agent restent indemnes ; le mur coupe l’effet. Éclaireur et Mule sont exclus. Aucun nouveau statut Brûlure, Poison ou Saignement n’est ajouté.</p><p>Les effets se jouent rapidement puis disparaissent. La durée de forme et les recharges avancent seulement avec les tours manuels. Les aperçus disparaissent dès la confirmation. Le laboratoire natif permet de sélectionner les trois drones de combat et les cinq formes.</p><p class="muted">Monstres et drones encore en 32 px agrandis : leur adaptation et l’élargissement du bestiaire sont prévus après les compétences et animations. Prototype isolé, sans nouveaux dégâts chiffrés, PV, énergie, IA, loot ou sauvegarde. Ces cas préparés ne fixent pas une nouvelle métrique générale de distance.</p><p><a href="hybrides-64.gif">Les deux animations à la suite en taille native</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> · <a href="review.json">Fiche de revue</a></p><p class="muted">Nouveaux dessins avec imagegen intégré : <a href="shot-source.png">projectile</a> / <a href="imagegen-shot-prompt.txt">prompt</a> · <a href="impact-source.png">impact</a> / <a href="imagegen-impact-prompt.txt">prompt</a>. Feu et corps validés réutilisés.</p><script src="gallery-script.js"></script></html>'''.replace('OPTIONS',options).replace('CARDS',cards)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')
    links=set(re.findall(r'(?:href|src)="([^"]+)"',html));assert all((OUT/link).is_file() for link in links)
    base.dump(OUT/'links-verification.json',{'checked':len(links),'missing':0})

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');parser.add_argument('--convert-stream',action='store_true');args=parser.parse_args()
    if args.prepare:prepare()
    elif args.convert_stream:convert()
    else:package()
