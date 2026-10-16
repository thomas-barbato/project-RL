"""ANIM-S27: package native64 amplified blasts and impact healing for review."""
import hashlib, json
from pathlib import Path
from PIL import Image, ImageChops
from add_browser_animation_comparison import add_comparison
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-57'
FRAMES=128

def dump(name,obj): (OUT/name).write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
def gif(path,images):
    samples=[images[i] for off in range(0,len(images),FRAMES) for i in [off+n for n in [14,20,24,28,32,64,68,72]]]
    pixels=[]
    for im in samples: pixels.extend(im.resize((192,192),Image.Resampling.NEAREST).get_flattened_data())
    groups=[(96,pixels),(96,[p for p in pixels if p[0]>p[1]*1.35 and p[0]>p[2]*1.35]),(64,[p for p in pixels if p[1]>p[0]+15 and p[1]>p[2]+15])]
    colours=[]
    for count,chosen in groups:
        chosen=chosen or pixels;strip=Image.new('RGB',(256,(len(chosen)+255)//256));strip.putdata(chosen)
        colours+=strip.quantize(colors=count,method=Image.Quantize.MEDIANCUT).getpalette()[:count*3]
    palette=Image.new('P',(1,1));palette.putpalette(colours)
    cached={};encoded=[]
    for im in images:
        key=hashlib.sha256(im.tobytes()).hexdigest()
        if key not in cached: cached[key]=im.quantize(palette=palette,dither=Image.Dither.NONE)
        encoded.append(cached[key])
    times=[round(i*1000/24/10)*10 for i in range(len(images)+1)]
    encoded[0].save(path,save_all=True,append_images=encoded[1:],duration=[b-a for a,b in zip(times,times[1:])],loop=0,disposal=2,optimize=False)
    with Image.open(path)as decoded:
        total=0;unique=set()
        for n in range(decoded.n_frames):decoded.seek(n);total+=decoded.info['duration'];unique.add(hashlib.sha256(decoded.convert('RGB').tobytes()).hexdigest())
        assert total==times[-1]
    return {'file':path.name,'native_size':list(images[0].size),'duration_ms':total,'distinct_frames':len(unique),'palette':'96 neutral, 96 warm, 64 green'}

def package():
    assert not (OUT/'visual-acceptance-01.json').exists()
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h for n,h in proof.items())
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'));checks=[];animations=[];overview=[]
    for e in entries:
        directory=OUT/'native'/e['folder'];timeline=json.loads((directory/'timeline.json').read_text(encoding='utf8'))
        originals=[Image.open(directory/f'{n:03}.png').convert('RGB') for n in range(FRAMES)]
        frames=[im.crop((32,88,608,664)) for im in originals];dest=OUT/'frames'/e['variant'];dest.mkdir(parents=True,exist_ok=True)
        canonical={};indices=[]
        for n,im in enumerate(frames):
            key=hashlib.sha256(im.tobytes()).hexdigest()
            if key not in canonical:canonical[key]=n;im.save(dest/f'{n:03}.png',compress_level=1)
            indices.append(canonical[key])
        differences={}
        for n in e['control_frames']:
            control=Image.open(directory/f'control-{n:03}.png').convert('RGB')
            delta=ImageChops.difference(originals[n],control).crop((32,88,608,664));differences[n]=delta
            for y,row in enumerate(e['map'].splitlines()):
                for x,value in enumerate(row):
                    if value=='#':assert delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox() is None,(e['variant'],n,x,y)
        if not e['permitted']:
            assert all(t['turn']==0 and t['energy']==timeline[0]['energy'] and t['hp']==timeline[0]['hp'] and t['cooldown']==0 and not any(t['damage']) for t in timeline)
            assert all(d.getbbox() is None for d in differences.values())
        else:
            assert timeline[14]['preview_visible'] and not timeline[17]['preview_visible']
            assert timeline[17]['turn']==1
            assert differences[20].getbbox() is not None
            assert all(differences[n].getbbox() is None for n in [44,60,84,100,124])
            if e['id']=='HL05':
                assert all(t['energy']==100 and t['cooldown']==0 and t['healed']==0 and t['hp']==50 for t in timeline)
                expected=[0,20,0,0] if not timeline[-1]['explosive'] else [20,20,20,0 if e['case']=='CLOISON' else 20]
                assert timeline[-1]['damage']==expected
                assert timeline[-1]['radius']==(0 if not timeline[-1]['explosive'] else 4)
            else:
                assert timeline[17]['energy']==84 and timeline[17]['cooldown']==10
                expected_heal={'PV-PLEINS':0,'PV-MANQUANTS':3,'CIBLE-10-PV':6,'PROTECTIONS':12}.get(e['case'],15)
                assert timeline[-1]['healed']==expected_heal
                assert timeline[-1]['hp']==timeline[0]['hp']+expected_heal
                assert timeline[-1]['damage'][1]==({'CIBLE-10-PV':10,'PROTECTIONS':20}.get(e['case'],40))
                assert timeline[-1]['damage'][3]==0
                if e['case']=='RETARDEMENT':
                    assert timeline[60]['pending'] and not any(timeline[60]['damage']) and timeline[60]['hp']==50
                    assert timeline[64]['resolved'] and timeline[64]['healed']==15 and differences[68].getbbox() is not None
                if e['quiet']:assert timeline[-1]['cooldown']==10 and timeline[-1]['turn']==1
                else:assert timeline[-1]['cooldown']==7 and timeline[-1]['turn']==4
        checks.append({'variant':e['variant'],'obstacles_clear':True,'manual_turns_and_refusals_checked':True,'sample_damage_and_heal_checked':True,'preview_removed_on_launch':e['permitted']})
        animations.append(gif(OUT/f"{e['variant']}-64.gif",frames));e.update(timeline=timeline,image_indices=indices)
        if e['case']=='HAUT':
            overview.extend(originals)
            for n in [14,20,24,28,32,44,64,68]:originals[n].save(OUT/f"{e['id']}-phase-{n:03}.png")
        print(e['variant']+' packaged',flush=True)
    animations.append(gif(OUT/'deflagration-drain-64.gif',overview))
    dump('verification.json',{'native_views':len(entries),'native_tile_size':64,'resampled':False,'protected_files_unchanged':len(proof),'checks':checks,'animations':animations})
    dump('review.json',{'batch':'ANIM-S27','date':'2026-10-14','status':'awaiting_visual_review','campaign_integration':False,
        'candidates':[{'number':63,'id':'HL05','name':'Déflagration amplifiée','variant':'HL05-A01'},{'number':64,'id':'HL06','name':'Drain d’impact','variant':'HL06-A01'}],
        'approved_cards':['review-individual-description-HL05-02.json','review-individual-description-HL06-02.json'],
        'mechanics':'Rank11: explosive heavy weapon radius+3 (sample1 becomes4), no extra damage and no new explosion on nonexplosive weapons. Drain: damage200%, life return60% of actual primary HP removed, shared per-use cap15% maxHP, limited by missingHP, energy16, cooldown10. Delayed explosion heals when it resolves.',
        'visuals':'Accepted50px steel rocket and0.20s flight. Same painted burst scaled478px for enlarged radius,134px for Drain;0.42s burst. Physical blood, shared recoil, visible blinking preview removed on launch. New brief outlined green pulse on player and+X PV at heal. No temporary buff for either skill.',
        'prepared_numbers':'Primary ordinary sample20; Drain40, or10 actually removed from low-HP primary, or20 after prepared protection. Player maxHP100, initial50/97/100, heals15/3/0/6/12. Adjacent victims give no life return. No final stats, variance, rounding, damage curve or armor approved by these examples.',
        'scope':'Independent prepared scenes, fixed victims and events. No campaign GameState, save, AI, RNG, loot or actual targeting/resource resolver. FullHP case demonstrates zero return, not final cast eligibility. One-turn delayed scene is an example, not a general heavy-weapon delay rule. Missing ammo-box uses later accepted equipment direction; no matter/ammo resource counter.',
        'pending':'Real weapon-family native sequences, damage types/defenses, per-use burst heal budget, exact rounding/event/snapshot/cooldown order, fullHP cast legality, affiliation changes, ground effects and familiar eligibility remain separate.',
        'temporary_actors':'Existing enemy32 artwork temporarily enlarged64; bestiary adaptation after animations.',
        'passive_scope':'Heavy weapon explosions only; no extension of stun radius or fixed Déluge targeting area, no automatic explosion for a heavy machine gun.',
    })
    write_html(entries);add_comparison(OUT)
    (OUT/'render-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/heavy_finale.rs').read_bytes())
    (OUT/'shared-art-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/heavy_opening.rs').read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    dump('progress-audit.json',{'date':'2026-10-14','previous_unshown':49,'newly_shown':['HL05','HL06'],'remaining_unshown':47,'visual_review_status':'HL05-A01 and HL06-A01 awaiting approval; other outstanding reviews preserved.'})
    print(json.dumps({'native_views':len(entries),'animations':len(animations),'checks':'passed'}))

def write_html(entries):
    manifest=[{k:e[k] for k in ['case','case_title','variant','id','title','permitted','quiet','timeline','image_indices']} for e in entries]
    script=r"""const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=false,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(!e.permitted)return 14;const step=Math.floor(elapsed*24/1000);if(turn===0)return step<27?17+step:44+(step-27)%13;const base=[17,64,88,112][turn],span=turn===3?12:20;return step<span?base+step:base+span+(step-span)%4;}
function ui(){const es=entries();next.disabled=!ready||!launched||es.every(e=>e.quiet)||!es.some(e=>e.permitted)||turn>=3||elapsed<850;cast.disabled=!ready||launched||!es.some(e=>e.permitted);status.textContent=!ready?'Chargement…':!launched?'Prévisualisation clignotante · confirmer pour commencer':es.map(e=>{const t=e.timeline[frame(e)];return e.title+(e.permitted?(e.id==='HL05'?' : rayon '+t.radius+' · dégâts inchangés':' : PV '+t.hp+'/100 · soin +'+t.healed+' · recharge '+t.cooldown+(t.pending?' · explosion en attente':'')):' : indisponible');}).join(' · ');}
async function load(){ready=false;ui();const n=++token,cache=new Map(),data=await Promise.all(entries().map(async e=>[e.id,await Promise.all(Array.from({length:128},(_,i)=>{const src='frames/'+e.variant+'/'+String(e.image_indices[i]).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=src;}));return cache.get(src);}))]));if(n!==token)return;sequences=new Map(data);elapsed=0;turn=0;launched=false;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===canvas.dataset.skill);canvas.parentElement.hidden=!e;if(e)canvas.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
scenario.onchange=load;cast.onclick=()=>{if(cast.disabled)return;launched=true;elapsed=0;turn=0;ui();};next.onclick=()=>{if(next.disabled)return;turn++;elapsed=0;ui();};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};document.getElementById('pause').onclick=ev=>{paused=!paused;ev.target.textContent=paused?'Lecture':'Pause';};load().catch(e=>status.textContent='Chargement impossible : '+e);requestAnimationFrame(tick);
""".replace('MANIFEST',json.dumps(manifest,ensure_ascii=False))
    options=''.join(f'<option value="{e["case"]}">{e["case_title"] if e["case"] not in ["HAUT","DROITE","GAUCHE","BAS"] else e["case"].capitalize()}</option>' for e in {e['case']:e for e in entries}.values())
    articles=''.join(f'<article><h2>{n} · {title}</h2><p>{desc}</p><canvas width="576" height="576" data-skill="{skill}"></canvas></article>' for n,skill,title,desc in [(63,'HL05','Déflagration amplifiée','L’explosion de l’arme couvre un rayon plus large ; les dégâts restent identiques.'),(64,'HL06','Drain d’impact','Un impact habituel, puis une impulsion verte et le soin reçu sur le joueur.')])
    gallery=''.join(f'<a href="{e["variant"]}-64.gif">{e["title"]} · {e["case"]}</a>' for e in entries)
    page="""<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Déflagration et drain · ANIM-S27</title>
<style>*{box-sizing:border-box}body{background:#0a0f13;color:#e0e7e7;font:16px system-ui;margin:0 auto;padding:24px;max-width:1280px}h1{font-size:27px}h2{font-size:21px}p{color:#a5b9c5;line-height:1.5}button,select{font:inherit;color:#e0e7e7;background:#202931;border:1px solid #566574;border-radius:4px;padding:8px 12px}button:disabled{opacity:.4}.controls{display:flex;gap:8px;flex-wrap:wrap}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:12px;overflow:auto}article{background:#11181e;padding:16px;position:relative}article p{min-height:48px}canvas{display:block;width:576px;height:576px;background:#080c0e}.gallery{display:flex;gap:12px;flex-wrap:wrap}a{color:#bbc7ff}details{margin-top:22px}summary{cursor:pointer}.note{font-size:13px}</style>
<main><h1>Armes lourdes · lot 27 · cases de 64 px</h1><p>Deux candidats : une explosion agrandie et un tir qui rend de la vie. Les nombres servent d’exemple.</p><p><a href="cote-a-cote.html">Voir les deux animations côte à côte</a></p><div class="controls"><select id="scenario" aria-label="Scénario">OPTIONS</select><button id="cast">Lancer</button><button id="next">Action suivante</button><button id="reset">Recommencer</button><button id="pause">Pause</button></div><p id="status">Chargement…</p><div class="grid">ARTICLES</div>
<p class="note">Les tours avancent uniquement avec « Action suivante ». L’impact instantané se termine, et le soin est reçu à l’impact. Dans l’exemple à retardement, cliquez sur « Action suivante » pour faire exploser le projectile.</p>
<p class="note">Déflagration est un passif : elle agrandit l’explosion sans ajouter de dégâts ni créer d’explosion sur une arme qui n’en possède pas. Drain soigne à partir de la cible principale ; les voisins touchés ne donnent aucun soin supplémentaire.</p><details><summary>GIF par compétence et scénario</summary><div class="gallery">GALLERY</div></details><p class="note">Aperçus proposés avant intégration. Monstres provisoires ; leur adaptation suivra les animations.</p></main><script>SCRIPT</script></html>"""
    (OUT/'index.html').write_text(page.replace('OPTIONS',options).replace('ARTICLES',articles).replace('GALLERY',gallery).replace('SCRIPT',script),encoding='utf8')
if __name__=='__main__':package()
