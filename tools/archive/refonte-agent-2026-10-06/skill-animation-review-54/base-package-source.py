"""ANIM-S24: native mark/mastery gallery with accepted physical feedback."""
import argparse,hashlib,json,shutil
from pathlib import Path
from PIL import Image,ImageChops
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_hybrids as converter
import prepare_agent_skill_animation_aim_recovery as encoder
from add_browser_animation_comparison import add_comparison
ROOT=base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-53'
FRAMES=160
CANDIDATES=[{'number':57,'id':'T01','name':'Marquage balistique','variant':'T01-A01'},{'number':58,'id':'T03','name':'Maîtrise des attaques à distance','variant':'T03-A01'}]
REUSED_GIFS={}

def prepare():
    assert not OUT.exists();OUT.mkdir()
    previous=OUT.with_name('skill-animation-review-52');receipt=json.loads((previous/'visual-acceptance-01.json').read_text(encoding='utf8'));proof={}
    for name,digest in receipt['artifact_hashes'].items():
        p=previous/name;assert hashlib.sha256(p.read_bytes()).hexdigest()==digest;proof[p.relative_to(ROOT).as_posix()]=digest
    reused=['assets/skills/agent/T01-compact.png','assets/skills/agent/T03-compact.png','artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png','artifacts/refonte-agent-2026-10-06/skill-animation-review-39/predator-64.png','assets/sprites/bestiary/living-32.png']
    for name in reused:proof[name]=hashlib.sha256((ROOT/name).read_bytes()).hexdigest()
    base.dump(OUT/'review-protection.json',proof)
    base.dump(OUT/'drawing-pack.json',{'new_raster_generation':False,'sources_reused_unchanged':reused,'motion':'Outlined red converging reticle and brighter steel projectile based on accepted review52 native shapes; actors/poses unchanged.'})
    (OUT/'Ouvrir_laboratoire.ps1').write_text((previous/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-visee-energie','--lot-marquage-maitrise'),encoding='utf-8-sig')
    base.dump(OUT/'progress-audit.json',{'date':'2026-10-13','catalogue_skills':94,'skills_with_explicit_animation_approval':35,'new_skills_not_yet_shown':55,'new_active_skills':32,'new_passive_skills':23,'previously_drawn_awaiting_full_animation_review':['FR02','FR04','A01','A02'],'numbering_note':'Gallery56 includes transformation attacks, Gel and separate collision variants; not56 distinct catalogue skills. Some passive skills only alter existing effects or values. This lot newly presentsT01 andT03, without approval.'})
    print(json.dumps({'protected':len(proof),'prepared':str(OUT)}))

def package():
    assert not (OUT/'visual-acceptance-01.json').exists()
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf8'));assert all(hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h for n,h in proof.items())
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'));checks=[];animations=[];overview=[]
    encoder.FRAMES=FRAMES
    for e in entries:
        directory=OUT/'native'/e['folder'];timeline=json.loads((directory/'timeline.json').read_text(encoding='utf8'))
        originals=[Image.open(directory/f'{n:03}.png').convert('RGB')for n in range(FRAMES)];frames=[im.crop((32,88,608,664))for im in originals]
        destination=OUT/'frames'/e['variant'];destination.mkdir(parents=True,exist_ok=True);canonical={};indices=[]
        for n,im in enumerate(frames):
            im.save(destination/f'{n:03}.png',compress_level=1);digest=hashlib.sha256(im.tobytes()).hexdigest();canonical.setdefault(digest,n);indices.append(canonical[digest])
        deltas={}
        for n in e['control_frames']:
            control=Image.open(directory/f'control-{n:03}.png').convert('RGB');delta=ImageChops.difference(originals[n],control).crop((32,88,608,664));deltas[n]=delta
            for y,row in enumerate(e['map'].splitlines()):
                for x,v in enumerate(row):
                    if v=='#':assert delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox()is None,(e['variant'],n,x,y)
        if not e['permitted']:
            assert all(t['remaining']==0 and t['energy']==100 and t['hits']==0 for t in timeline)
            assert all(d.getbbox()is None for d in deltas.values())
        elif e['id']=='T01':
            cx,cy={0:(480,288),1:(96,288),2:(288,96),3:(288,480)}[e['direction']]
            assert deltas[20].crop((cx-8,cy-12,cx+8,cy+14)).getbbox()is not None,(e['variant'],'mark reaction missing')
            assert timeline[17]['remaining']==6 and timeline[17]['cooldown']==3 and timeline[17]['energy']==91 and timeline[17]['turn']==0
            assert all(t['icon_visible']==(t['remaining']>0)for t in timeline)
            if e['quiet']:assert timeline[-1]['remaining']==6 and timeline[-1]['turn']==0 and timeline[-1]['hits']==0
            elif e['case']=='EXPIRATION':
                assert timeline[128]['remaining']==1 and timeline[148]['remaining']==0 and timeline[-1]['damage']==20 and not timeline[-1]['used_mark']
            elif e['case']=='REMPLACEMENT':
                assert timeline[88]['marked_target']==[6,2] and timeline[88]['remaining']==6 and timeline[88]['energy']==82
                assert timeline[-1]['remaining']==3 and timeline[-1]['hits']==0
            else:
                assert timeline[48]['remaining']==0 and timeline[48]['used_mark']
                assert timeline[60]['damage']==(0 if e['case']=='MANQUE'else 60 if e['case']=='RAFALE'else 20)
        else:
            assert all(not t['icon_visible'] and t['remaining']==0 and t['cooldown']==0 and t['energy']==100 for t in timeline)
            if e['case']in ['CLOISON','CACHE']:assert timeline[-1]['hits']==0
            else:assert timeline[34]['damage']==(20 if e['case']in ['MELEE','DRONE']else 28) and timeline[34]['bonus']==(0 if e['case']in ['MELEE','DRONE']else 8)
            if e['quiet']:assert timeline[-1]['hits']==1
        checks.append({'variant':e['variant'],'world_walls_clear':True,'mark_cast_refusals_or_consume_expiry_checked':True,'mark_applies_shared_negative_reaction':e['id']=='T01'and e['permitted'],'no_idle_timer_advance_or_replayed_hit':True,'mastery_no_timed_status':True,'damage_totals_include_bonus':True})
        filename=f"{e['variant']}-64.gif"
        if e['variant']in REUSED_GIFS:
            reused=REUSED_GIFS[e['variant']];shutil.copyfile(reused['path'],OUT/filename);animations.append(reused['record'])
        else:animations.append(encoder.gif(OUT/filename,frames))
        e.update(timeline=timeline,image_indices=indices);print(e['variant']+' packaged',flush=True)
        if e['case']=='HAUT':
            overview.extend(originals)
            for n in [14,17,20,28,34,44,50,52,55,60,68,88,148,152,158]:originals[n].save(OUT/f"{e['id']}-phase-{n:03}.png")
    animations.append(encoder.gif(OUT/'marquage-maitrise-64.gif',overview))
    base.dump(OUT/'verification.json',{'native_views':len(entries),'controls':checks,'animations':animations,'protected_files_unchanged':len(proof),'resampling_of_native_capture':False})
    base.dump(OUT/'review.json',{'batch':'ANIM-S24','status':'awaiting_visual_review','native_tile_size':64,'campaign_integration':False,
        'candidates':CANDIDATES,
        'approved_cards':['review-individual-description-T01-02.json','review-individual-description-T03-02.json'],
        'approved_values':'T01 precision multiplier1.35, max6turns, range6, energy9, cooldown3; instant mark consumes no turn. T03 permanent ranged damage+40%, excludes own melee and drone attacks.',
        'visuals':'Converging red reticle, player buff and target-analysis status icon with timer, name above caster; mark disappears when shot is launched even on miss. Brighter projectile core and inclusive total/bonus/physical component; no timed mastery buff. Physical blood and shared recoil.',
        'prepared_numbers':'Base20 gives28(+8) for mastery; prepared burst three20 hits gives60. Fixed contacts and precision multiplier snapshot135% establish no actual accuracy roll, equipment range/variance, monster HP, damage curve or real combat ordering.',
        'scope':'Independent prepared panels. Reapplying after three prepared turns replaces the marked target; no actual targeting/AI/FOV/campaign resolver. Target analysis strip is a visual status fixture, not a new full analysis interface.',
        'pending':'Actual precision and damage defenses, real weapon spread/variance, mage/form attack eligibility and heavy weapon scope, multi-target burst, real perception/targeting, rounding and dense combat integration remain separate.',
        'temporary_actors':'Existing enemy and drone32 art enlarged64; adaptation after animations.',
        'excluded_draft':'native-mark-reaction-draft is an unpresented initial capture, superseded by the common mark-application reaction; exclude it from acceptance.'})
    write_html(entries);add_comparison(OUT)
    (OUT/'render-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/shot_buffs.rs').read_bytes());(OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'views':len(entries),'animations':len(animations),'protected':len(proof),'checks':'passed'}))

def write_html(entries):
    manifest=[{k:e[k]for k in ['case','variant','id','title','permitted','quiet','image_indices','timeline']}for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value);}
function frame(e){if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(!e.permitted)return 14;const step=Math.floor(elapsed*24/1000),base=[17,48,68,88,108,128,148][turn],length=turn===0?27:turn===6?8:16;if(step<length)return base+step;return base+length+step%4;}
function ui(){const es=entries();next.disabled=!ready||!launched||es.every(e=>e.quiet)||!es.some(e=>e.permitted)||turn>=6;cast.disabled=!ready||launched||!es.some(e=>e.permitted);status.textContent=!ready?'Chargement…':!launched?'Aperçu clignotant · confirmer pour commencer':'Étape '+turn+' · '+es.map(e=>{const t=e.timeline[frame(e)];return e.title+(e.permitted?(e.id==='T01'?' : marque '+t.remaining+' tours · recharge '+t.cooldown:' : dégâts '+t.damage+' (+'+t.bonus+')'):' : indisponible');}).join(' · ');}
async function load(){ready=false;ui();const n=++token,cache=new Map(),data=await Promise.all(entries().map(async e=>[e.id,await Promise.all(Array.from({length:160},(_,i)=>{const src='frames/'+e.variant+'/'+String(e.image_indices[i]).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=src;}));return cache.get(src);}))]));if(n!==token)return;sequences=new Map(data);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const c of document.querySelectorAll('canvas')){const e=entries().find(e=>e.id===c.dataset.skill);c.parentElement.hidden=!e;if(e)c.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);}}requestAnimationFrame(tick);}
for(const canvas of document.querySelectorAll('canvas')){canvas.onmousemove=ev=>{const e=entries().find(e=>e.id===canvas.dataset.skill),box=canvas.getBoundingClientRect(),x=ev.clientX-box.left,y=ev.clientY-box.top,tip=canvas.parentElement.querySelector('.tip'),t=e?.timeline[frame(e)];tip.hidden=!(ready&&launched&&e.id==='T01'&&t?.icon_visible&&x>=488&&x<=526&&y>=66&&y<=104);if(!tip.hidden)tip.textContent=e.title+' · précision ×1,35 · '+t.remaining+' tours restants';};canvas.onmouseleave=()=>canvas.parentElement.querySelector('.tip').hidden=true;}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest,separators=(',',':')))
    labels={'HAUT':'Vers le haut','DROITE':'Vers la droite','GAUCHE':'Vers la gauche','BAS':'Vers le bas','ATTENTE':'Attente sans action','CLOISON':'Obstacle','CACHE':'Cible cachée','SURVOL':'Survol','MELEE':'Arme de mêlée active','SILENCE':'Silence','TRANSFORME':'Forme transformée','MANQUE':'Tir manqué','RAFALE':'Rafale préparée','EXPIRATION':'Expiration sans tir','REMPLACEMENT':'Nouvelle cible','DRONE':'Tir d’un drone'}
    options=''.join(f'<option value="{k}">{v}</option>'for k,v in labels.items());cards=''.join(f'<article><h2>{c["number"]} · {c["name"]}</h2><p class="muted">{c["variant"]} · proposé</p><canvas data-skill="{c["id"]}" width="576" height="576"></canvas><div class="tip" hidden></div></article>'for c in CANDIDATES)
    template=(ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-52/index.html').read_text(encoding='utf8')
    style=template.split('<style>',1)[1].split('</style>',1)[0]
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Marquage et maîtrise</title><style>STYLE</style><h1>Marquage balistique et Maîtrise des attaques à distance</h1><p class="muted">ANIM-S24 · candidats 57–58 · cases natives 64 px · rendu à valider</p><div class="controls"><select id="scenario">OPTIONS</select><button id="next">Action suivante</button><button id="cast">Commencer l’aperçu</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="grid">CARDS</main>
<p>Marquage balistique : les traits rouges se referment sur la cible. La durée reste visible dans votre barre de buffs et dans la bande de statuts de la cible. Le tir suivant consomme la marque, même s’il manque ; une rafale préparée conserve son bonus pour ses trois projectiles. Sans action, la marque reste active.</p>
<p>Maîtrise des attaques à distance : le projectile possède un cœur plus vif. Les dégâts affichent 28 (+8), puis leur composante physique : le bonus est déjà inclus dans le total. Aucune nouvelle icône de buff temporaire n’est ajoutée pour ce bonus permanent.</p>
<p class="muted">Les contacts et nombres sont préparés, sans jet de précision ni nouvelle variance. La bande d’analyse est un aperçu de statut, pas la refonte complète de cette fenêtre. Les scènes sont indépendantes. Les exemples de mêlée et de drone ne bénéficient pas de la maîtrise. Monstres et drones 32 px temporaires ; adaptation prévue après les animations.</p>
<p><a href="marquage-maitrise-64.gif">Aperçu animé</a> · <a href="review.json">Périmètre</a> · <a href="verification.json">Vérifications</a></p><script>SCRIPT</script></html>'''.replace('STYLE',style).replace('OPTIONS',options).replace('CARDS',cards).replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf8')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--prepare',action='store_true');p.add_argument('--convert-stream',action='store_true');a=p.parse_args()
    if a.prepare:prepare()
    elif a.convert_stream:converter.OUT=OUT;converter.convert()
    else:package()
