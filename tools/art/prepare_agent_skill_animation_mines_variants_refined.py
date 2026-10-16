"""ANIM-S31-REV02: explosion-only revision, preserved native status drawings."""
import hashlib
import inspect
import json
import re
from pathlib import Path
from PIL import Image, ImageChops
import prepare_agent_skill_animation_mines_opening as base
from add_browser_animation_comparison import add_comparison
from repair_mine_preview_playback import repair_review

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-62'
FRAMES=256

def dump(name,obj):
    (OUT/name).write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n',encoding='utf8')

def gif(path,images):
    source=inspect.getsource(base.encoder.gif).replace('[14,20,24,28,32,64,68,72]','[14,44,54,64,112,114,116,120,136,144,160,184,208,232,254]')
    source=source.replace('p[0]>p[1]*1.35 and p[0]>p[2]*1.35','p[2]>p[0]*1.2 and p[2]>p[1]*1.05').replace('96 neutral, 96 warm, 64 green','96 neutral, 96 cold, 64 green')
    space=dict(base.encoder.__dict__);space['FRAMES']=FRAMES
    exec(compile(source,'<review61-palette-encoder>','exec'),space)
    return space['gif'](path,images)

def write_html(entries):
    base.OUT=OUT;base.write_html(entries)
    page=(OUT/'index.html').read_text(encoding='utf8')
    replacements={
        'ANIM-S30':'ANIM-S31','Mines · lot 30':'Mines élémentaires · lot 31',
        'Une mine métallique au sol, sa LED centrale clignotante et son explosion. À droite, Réseau de mines permet d’en conserver cinq au rang 11.':'Deux variantes sans Résidu persistant : détonation toxique avec Empoisonné, et éclats de glace avec Gel. Les LED verte et bleu glacé clignotent dès la pose.',
        '68 · Pose de mine':'70 · Mines de poison','69 · Réseau de mines':'71 · Mines de glace',
        'data-skill="SM00"':'data-skill="SM02"','data-skill="SM01"':'data-skill="SM03"',
        'Une mine simultanée sans le passif. Le voyant clignote dès la pose ; elle reste inactive avant son armement.':'LED verte, détonation toxique et petites vapeurs attachées aux victimes. Le poison les suit lorsqu’elles s’éloignent.',
        'Cinq mines au rang 11. Le scénario de capacité remplace la plus ancienne par la nouvelle mine.':'LED bleu glacé, éclats de froid et enveloppe de Gel devant les victimes. Un échec du Gel conserve les dégâts.',
        'Attendre conserve la mine et son clignotement, sans avancer les tours.':'Attendre conserve le clignotement et les statuts, sans avancer leurs compteurs. Après l’explosion, les actions suivantes montrent les applications périodiques et la fin des statuts.',
        'Exemple accepté : 13 dégâts de feu, rayon d’une case, 10 énergie et recharge d’un tour. Le joueur voisin est protégé. L’explosion est brève, sans sang, sans résidu ni Brûlure ajoutée dans cet exemple. La fiche Pose de mine affiche la capacité du passif ; celui-ci n’ajoute pas de buff temporaire.':'Exemples provisoires déjà acceptés au rang 11 : 19 dégâts à l’explosion de poison, puis 42 par application pendant cinq applications ; 16 dégâts de froid et Gel de cinq tours, avec une chance affichée de 69,2 %. Le premier poison s’applique dès l’explosion : le premier total affiché est 61 (19 + 42). Les réussites de statut sont préparées pour examiner le rendu, sans tirage aléatoire. Le joueur voisin reste protégé.',
        'Les mines déjà armées du scénario de capacité représentent des poses antérieures. La géométrie complète, les dégâts aux familiers, l’inspection des objets superposés et les interactions de passifs seront raccordés séparément.':'PV ennemis préparés : 400 pour montrer les cinq applications sans mort intermédiaire. Géométrie complète, défenses, jets de statut, familiers, inspection et phases d’expiration définitives seront raccordés séparément. Ce lot n’a pas SM05 : les résidus Feu/Poison/Froid seront présentés dans le lot dédié.',
        'href="mine-source.png"':'href="../skill-animation-review-60/mine-source.png"',
        'href="imagegen-mine-prompt.txt"':'href="../skill-animation-review-60/imagegen-mine-prompt.txt"',
    }
    for old,new in replacements.items():
        assert old in page,old;page=page.replace(old,new)
    manifest=[{k:e[k] for k in ['case','case_title','variant','id','title','timeline','image_indices','hover_images','idle_images','mine_cell','action_frames']}for e in entries]
    script=r'''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=false,turn=0,token=0,sequences,hoverSequences,idleSequences=new Map(),idleStep=-1,idleToken=0;const hovered=new Set();
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){const step=Math.floor(elapsed*24/1000);if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(turn===0)return step<27?17+step:44+(step-27)%18;const base=[17,64,88,112,136,160,184,208,232][turn];if(e.idle_images.some(s=>s.step===turn)||turn===e.action_frames.length)return base+Math.min(step,23);return step<24?base+step:base+6+(step-24)%18;}
function ui(){const es=entries();next.disabled=!ready||!launched||turn>=Math.max(...es.map(e=>e.action_frames.length))||elapsed<850;cast.disabled=!ready||launched;status.textContent=!ready?'Chargement…':!launched?'Prévisualisation clignotante · confirmez la pose':es.map(e=>{const t=e.timeline[frame(e)];return e.title+' : '+(t.count?'mine posée':'mine consommée')+' · '+t.status_name+' '+t.status.join('/')+' · énergie '+t.energy+' · explosions '+t.explosions;}).join(' · ');}
function image(src){return new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=()=>reject(new Error(src));im.src=src;});}
async function loadIdle(){const n=++idleToken,id=turn;idleStep=-1;idleSequences=new Map();const cache=new Map(),es=entries().filter(e=>e.idle_images.some(s=>s.step===id));if(!es.length)return;const data=await Promise.all(es.map(async e=>{const s=e.idle_images.find(s=>s.step===id);return[e.id,await Promise.all(s.image_indices.map(i=>{const src='frames/'+e.variant+'/idle-'+id+'-'+String(i).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,image(src));return cache.get(src);} ))];}));if(n!==idleToken||id!==turn)return;idleSequences=new Map(data);idleStep=id;}
async function load(){ready=false;hovered.clear();idleToken++;idleSequences=new Map();idleStep=-1;ui();const n=++token,cache=new Map();function cached(src){if(!cache.has(src))cache.set(src,image(src));return cache.get(src);}
const data=await Promise.all(entries().map(async e=>[e.id,await Promise.all(e.image_indices.map(i=>cached('frames/'+e.variant+'/'+String(i).padStart(3,'0')+'.png'))),new Map(await Promise.all(e.hover_images.map(h=>cached('frames/'+e.variant+'/'+h.file).then(im=>[h.frame,im]))))]));if(n!==token)return;sequences=new Map(data.map(d=>[d[0],d[1]]));hoverSequences=new Map(data.map(d=>[d[0],d[2]]));elapsed=0;turn=0;launched=false;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===canvas.dataset.skill);canvas.parentElement.hidden=!e;if(e){const f=frame(e),t=e.timeline[f];let im=sequences.get(e.id)[f];if(launched&&elapsed>=1000&&idleStep===turn&&idleSequences.has(e.id))im=idleSequences.get(e.id)[Math.floor((elapsed-1000)*24/1000)%48];if(hovered.has(e.id)&&launched&&t.count){const h=e.hover_images.find(h=>h.step===t.step&&h.led_on===t.led_on)||e.hover_images.find(h=>h.step===t.step);if(h)im=hoverSequences.get(e.id).get(h.frame);}canvas.getContext('2d').drawImage(im,0,0);}}}requestAnimationFrame(tick);}
for(const canvas of document.querySelectorAll('canvas')){canvas.onmousemove=ev=>{const e=entries().find(e=>e.id===canvas.dataset.skill),box=canvas.getBoundingClientRect(),x=ev.clientX-box.left,y=ev.clientY-box.top;if(e&&x>=e.mine_cell[0]*64&&x<(e.mine_cell[0]+1)*64&&y>=e.mine_cell[1]*64&&y<(e.mine_cell[1]+1)*64)hovered.add(e.id);else hovered.delete(canvas.dataset.skill);};canvas.onmouseleave=()=>hovered.delete(canvas.dataset.skill);}
scenario.onchange=load;cast.onclick=()=>{if(cast.disabled)return;launched=true;elapsed=0;turn=0;ui();};next.onclick=()=>{if(next.disabled)return;turn++;elapsed=0;loadIdle().catch(e=>status.textContent='Chargement impossible : '+e);ui();};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;hovered.clear();idleToken++;idleStep=-1;idleSequences=new Map();ui();};document.getElementById('pause').onclick=ev=>{paused=!paused;ev.target.textContent=paused?'Lecture':'Pause';};load().catch(e=>status.textContent='Chargement impossible : '+e);requestAnimationFrame(tick);
'''.replace('MANIFEST',json.dumps(manifest,ensure_ascii=False))
    page=re.sub(r'<script>[\s\S]*?</script>','<script>'+script+'</script>',page,count=1)
    page=page.replace('</main>','<p class="note">Détonation toxique réalisée avec imagegen intégré : <a href="poison-burst-source.png">source</a> · <a href="imagegen-poison-burst-prompt.txt">prompt</a>. Les éclats froids et le Gel reprennent les dessins déjà acceptés.</p></main>',1)
    (OUT/'index.html').write_text(page,encoding='utf8')

def package():
    assert not (OUT/'visual-acceptance-01.json').exists()
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf8'))
    for n,h in proof.items():assert hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h,n
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'));checks=[];animations=[];overview=[]
    for e in entries:
        native=OUT/'native'/e['folder'];timeline=json.loads((native/'timeline.json').read_text(encoding='utf8'))
        originals=[Image.open(native/f'{n:03}.png').convert('RGB')for n in range(FRAMES)]
        frames=[im.crop((32,88,608,664))for im in originals];dest=OUT/'frames'/e['variant'];dest.mkdir(parents=True,exist_ok=True)
        canonical={};indices=[]
        for n,im in enumerate(frames):
            h=hashlib.sha256(im.tobytes()).hexdigest()
            if h not in canonical:canonical[h]=n;im.save(dest/f'{n:03}.png',compress_level=1)
            indices.append(canonical[h])
        hovers=[]
        for n in e['hover_frames']:
            Image.open(native/f'hover-{n:03}.png').crop((32,88,608,664)).save(dest/f'hover-{n:03}.png',compress_level=1)
            hovers.append({'frame':n,'file':f'hover-{n:03}.png','step':timeline[n]['step'],'led_on':timeline[n]['led_on']})
        idle_native=(OUT/e['idle_source']).resolve()
        for idle in e['idle_images']:
            idle_indices=[];keys={};step=idle['step'];base_frame=e['action_frames'][step-1]
            assert idle['hp']==timeline[base_frame+23]['hp']and idle['status']==timeline[base_frame+23]['status']
            for n in range(48):
                im=Image.open(idle_native/f'idle-{step}-{n:03}.png').convert('RGB').crop((32,88,608,664));h=hashlib.sha256(im.tobytes()).hexdigest()
                if h not in keys:keys[h]=n;im.save(dest/f'idle-{step}-{n:03}.png',compress_level=1)
                idle_indices.append(keys[h])
            idle['image_indices']=idle_indices
        for n in e['control_frames']:
            delta=ImageChops.difference(originals[n],Image.open(native/f'control-{n:03}.png').convert('RGB')).crop((32,88,608,664))
            for y,row in enumerate(e['map'].splitlines()):
                for x,c in enumerate(row):
                    if c=='#':assert delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox()is None,(e['variant'],n,x,y)
        assert timeline[14]['preview_visible']and not timeline[17]['preview_visible']
        assert timeline[17]['energy']==90 and not timeline[17]['mines'][0]['armed']
        assert {t['led_on']for t in timeline[18:62]}=={True,False}
        last=timeline[-1];rank=timeline[0]['rank'];damage=(20 if rank==12 else 19)if e['id']=='SM02'else(17 if rank==12 else 16);periodic=47 if rank==12 else 42
        if e['quiet']:
            assert last['explosions']==0 and last['turn']==1 and last['hp']==[400]*3 and last['status']==[0]*3
        else:
            assert last['explosions']==1 and last['status']==[0]*3 and last['count']==0
            exploded=64 if e['case']=='ENNEMI-ARMEMENT'else 112
            assert timeline[exploded]['last_damage'][0]==damage+(periodic if e['id']=='SM02'else 0)
            if e['id']=='SM02':
                assert last['poison_ticks']==([5,5,0]if e['case']=='CLOISON'else[5]*3)
                assert last['hp']==([400-damage-5*periodic]*2+[400]if e['case']=='CLOISON'else[400-damage-5*periodic]*3)
            else:
                assert last['hp']==([400-damage]*2+[400]if e['case']=='CLOISON'else[400-damage]*3)
                assert timeline[112]['status'][0]==(0 if e['case']=='GEL-RESISTE'else(3 if e['case']=='ENNEMI-ARMEMENT'else 5))
            if e['case']=='ELOIGNEMENT':
                assert timeline[136]['primary']==([3,4]if e['id']=='SM02'else[4,5])
                assert timeline[136]['blocked_walk']==(e['id']=='SM03')
        assert all(t['player_hp']==100 and t['energy']==90 for t in timeline[17:])
        assert all(max(t['poison_ticks'])<=5 for t in timeline)
        animations.append(gif(OUT/f'{e["variant"]}-64.gif',frames));e.update(timeline=timeline,image_indices=indices,hover_images=hovers)
        checks.append({'variant':e['variant'],'walls_clear':True,'arming_and_LED':True,'single_explosion':True,'immediate_poison_once_plus_four_following_applications':True,'manual_expiry_and_player_protection':True})
        if e['case']=='HAUT':
            overview.extend(originals)
            for n in [14,44,54,112,116,120,136,144,208,232,254]:originals[n].save(OUT/f'{e["id"]}-phase-{n:03}.png')
        print(e['variant']+' packaged',flush=True)
    animations.append(gif(OUT/'mines-poison-glace-64.gif',overview))
    dump('verification.json',{'native_views':len(entries),'tile':64,'resampled':False,'protected_files_unchanged':len(proof),'checks':checks,'animations':animations})
    dump('review.json',{'batch':'ANIM-S31-REV02','date':'2026-10-14','status':'awaiting_visual_review','campaign_integration':False,
        'candidates':[{'number':70,'id':'SM02','name':'Mines de poison','variant':'SM02-A02'},{'number':71,'id':'SM03','name':'Mines de glace','variant':'SM03-A02'}],
        'approved_cards':['review-individual-description-SM02-07.json','review-individual-description-SM03-02.json','persistent-immediate-application-count-acceptance-decisions.json','mine-led-type-color-palette-acceptance-01.json','mine-explosion-type-and-residue-visual-direction-decisions.json','mine-residue-elemental-ground-visual-direction-decisions.json'],
        'mechanics':'One mine, radius1, energy10, cooldown1, arming1; inherited valid sample. Effective11 poison19+42 per application with5total, rank12=20+47. Cold16/17, Gel5turns, chance69.2/73.8 displayed; prepared outcomes, no RNG. First poison at explosion counts as first; four later ticks only, no extra tick from idle. Exact expiry-phase integration separate.',
        'visuals':'Same immutable mine texture; native shader recolors only red central lens to green or ice-blue, with accepted blink. Revised painted gas-lobe6stage burst180px/.46s/.82alpha; new frost-pressure6stage burst176px/.40s/.82alpha. Old status textures and native idle images reused exactly; only explosions revised on user request. Poison vapour attached to victims, existing foregroundGel coating. Shared recoil on initial hit and periodic poison. Skill-cause icons/durations on victims and colored total with type/cause pictogram; first poison total61 or67 combines explosion and immediate toxin. No blood or terrain residue.',
        'prepared_numbers':'Victims400HP to remain alive. Poison final171HP at11 or145at12; Cold384/383. Wall-protected victim400. Player100 remains protected. Prepared chance outcomes do not define success, defense, attribute curves or variance rules.',
        'scope':'Independent debug fixtures; no GameState, save, campaign, actual AI/perception/RNG, inventory or generalized resource resolver. Native body32 enemy enlarged64 kept until later adaptation. Frost successful/failed cases and diagonal escape are visual fixtures; no universal movement/geometry convention inferred.',
        'pending':'Native rendered palettes, toxic burst, attachedPoison feedback and variantHUD await user approval. Full combat targeting/status aggregation/resistances/importance, timer-expiry ordering, familiar eligibility, diagonals/large footprints, reapplication/Burn interactions and gear snapshot integration separate.',
        'SM05':'Absent in this batch. Next dedicated residue preview will show burning ground forFire, gas cloud forPoison, frozen ground forCold, fixed to terrain, following latestMINE-VFX-TYPE02 direction.'})
    write_html(entries)
    page=(OUT/'index.html').read_text(encoding='utf8').replace('Mines élémentaires · lot 31','Mines élémentaires · lot 31 · révision 2').replace('Les éclats froids et le Gel reprennent les dessins déjà acceptés.','Les explosions Poison et Froid sont redessinées ; le Gel et le statut de Poison sont conservés.').replace('</main>','<p class="note">Explosion de froid réalisée avec imagegen intégré : <a href="frost-burst-source.png">source</a> · <a href="imagegen-frost-burst-prompt.txt">prompt</a>.</p></main>',1)
    (OUT/'index.html').write_text(page,encoding='utf8')
    add_comparison(OUT)
    repair_review(OUT)
    (OUT/'render-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/mines_variants_refined.rs').read_bytes());(OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    dump('drawing-pack.json',{'mine_source':'review60/mine-sprite.png unchanged','mine_lens':'native code material, centralred-only hue mask; hull colors preserved','toxic_source':'poison-burst-source.png','toxic_prompt':'imagegen-poison-burst-prompt.txt','generation_mode':'built-in imagegen','atlas_pack':'six equal512px squares resized uniformly256; alpha preserved','cold_burst':'frost-burst-source.png / frost-burst-atlas.png; new built-in imagegen; prompt imagegen-frost-burst-prompt.txt','Gel':'review28/gel-atlas.png unchanged; foreground crop','new_render_status':'awaiting approval'})
    dump('progress-audit.json',{'date':'2026-10-14','previous_unshown':42,'newly_shown':['SM02','SM03'],'remaining_unshown':40,'status':'SM02-A02 and SM03-A02 await approval. Only explosion forms and impact revised following user feedback; status drawings reused, no new skill added.'})
    print(json.dumps({'views':len(entries),'animations':len(animations),'checks':'passed'}))

if __name__=='__main__':package()
