"""ANIM-S06: native fireball and combustion review, manual turns and cardinal views."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_fire_projectile as previous
from prepare_agent_skill_animation_revision import save_animation

OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-18'

def protected():
    hashes=previous.protected()
    for p in sorted(previous.OUT.rglob('*')):
        if p.is_file():hashes[p.relative_to(base.ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes

def prepare():
    assert not (OUT/'review-protection.json').exists()
    base.dump(OUT/'review-protection.json',protected())
    base.dump(OUT/'review.json',{
        'batch':'ANIM-S06-A03','status':'awaiting_visual_review','native_tile_size':64,'visual_revision':{'projectile_size_pixels':96,'projectile_opacity':0.90,'blast_size_pixels':352,'blast_opacity':0.52,'combustion_plume_scales':[0.52,0.42],'combustion_opacity':0.65,'combustion_corona_removed':True},
        'candidates':[{'number':n,'skill_id':sid,'variant':sid+('-A03' if sid=='FE05' else '-A02'),'status':'awaiting_visual_review'} for n,sid in ((13,'FE05'),(14,'FE06'))],
        'fireball':'The drawn fireball travels once from the caster, hits the first monster or wall in its path, then explodes at that collision point. Projectile flight and rise are visual motion only; the impact stays animated until the next explicit turn.',
        'combustion':'Existing flames close onto the single burning target, then a pair of small translucent flames, without a circular explosion corona. Its previous Burn is consumed. The prepared success/no-success cases show whether attached Burn returns after the shock.',
        'timing':'One-turn visual impacts loop while waiting. Burn and cooldowns decrease only with explicit turns. Gallery buttons control turns; native montage uses scripted turn events at 2.5/3.0/3.5 seconds.',
        'scope':'Isolated visual fixture. No damage, RNG, campaign integration, balance changes or new permanent skill text. Circular radius-three footprint and wall-contact scene illustrate the approved art policy; final combat geometry and collision resolution are not implemented here.',
        'art':'Built-in image_gen source sheet, exact prompt saved in imagegen-prompt.txt. Transparent crop/resize/packing only; earlier sources and characters untouched. Eight drawn samples per second via the four-frame ping-pong sequence.',
        'directions':'Right, left, up and down, with a full cloison in the down-facing example. NPCs and caster stay front-facing.',
        'blood':'Pure Fire effects contain no blood.',
        'laboratory':'Manual turns, disabled skills during cooldown, combustion requires Burn, configurable prepared new-Burn success and four directions. Native laboratory is an isolated review fixture.',
    })
    launcher=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launcher.replace('--lot-feu-tours','--lot-feu-final'),encoding='utf-8-sig')

def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'));assert protected()==baseline
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert len(entries)==10
    sequences,controls={},[]
    animations=[]
    for e in entries:
        x,y,w,h=e['crop'];bounds=(x,y,x+w,y+h);folder=OUT/'native'/e['folder'];target=OUT/'frames'/e['variant'];target.mkdir(parents=True,exist_ok=True)
        frames=[]
        for i in range(96):
            with Image.open(folder/f'{i:03}.png') as im:
                assert im.size==(1280,720)
                frame=im.crop(bounds).convert('RGB');frame.save(target/f'{i:03}.png');frames.append(frame)
        diffs={}
        for phase in e['control_frames']:
            with Image.open(folder/f'control-{phase:03}.png') as im:diffs[phase]=ImageChops.difference(frames[phase],im.crop(bounds).convert('RGB'))
            for cy,row in enumerate(e['map'].splitlines()):
                for cx,symbol in enumerate(row):
                    if symbol=='#':assert diffs[phase].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None, f'Fire on wall: {e["variant"]}, frame {phase}'
        assert diffs[40].getbbox() and diffs[52].getbbox(),'Main impact ended before the turn.'
        assert diffs[95].getbbox() is None,'Effects did not finish with their turns.'
        # The preview is identical to the quiet capture: it is a targeting UI, not damage.
        if e['id']=='FE05':assert diffs[3].getbbox() is None and diffs[14].getbbox() is None
        cx,cy=e['affected_cells'][0];sample=(cx*64+3,cy*64+25)
        bright,dim=frames[3].getpixel(sample),frames[14].getpixel(sample)
        assert bright[1]>dim[1]+25,'Targeting preview does not blink.'
        for phase in (17,40,52):
            red,green,blue=frames[phase].getpixel(sample)
            assert not (red<100 and green>140 and blue>140),'Targeting preview survived launch.'
        if e['fresh_burn']:assert diffs[61].getbbox(),'Burn missing after the large impact.'
        else:assert diffs[61].getbbox() is None,'Consumed Burn returned in the failed-reburn case.'
        for actor in e['targets']:
            if not actor['affected']:
                cx,cy=actor['cell'];body=(cx*64+12,cy*64+12,cx*64+52,cy*64+52)
                # Drawn spill may occupy the floor beside a body; physical cloison must protect F.
                if e['folder']=='CLOISON-BAS' and actor['label']=='F':
                    assert all(diffs[i].crop(body).getbbox() is None for i in (40,52,61))
        timeline=json.loads((folder/'timeline.json').read_text(encoding='utf-8'))
        assert all(t['remaining']==1 and t['turn']==0 for t in timeline[17:60])
        assert all(t['remaining']==0 for t in timeline[60:])
        if e['id']=='FE05':
            assert all(t['projectile_visible'] and not t['explosion_visible'] for t in timeline[17:33])
            assert all(t['explosion_visible'] and not t['projectile_visible'] for t in timeline[33:60])
            if e['collision_target'] is not None:
                target=next(actor for actor in e['targets'] if actor['label']==e['collision_target'])
                assert e['impact_cell']==target['cell'],'Explosion not centred on the collided monster.'
            if e['folder']=='INTERCEPTION':assert e['collision_target']=='D'
            elif e['folder']=='CLOISON-BAS':assert e['collision_target'] is None
            else:assert e['collision_target']=='A'
        controls.append({'variant':e['variant'],'physical_walls_clear':True,'animated_until_explicit_turn':True,'burn_after_impact':e['fresh_burn'],'final_effects_expired':True})
        sequences[e['variant']]=frames
        animations.append(save_animation(OUT/f"{e['variant']}-64.gif",frames,24))
    for folder in ('DROITE','CLOISON-BAS','INTERCEPTION'):
        subset=[e for e in entries if e['folder']==folder];boards=[]
        for i in range(96):
            board=Image.new('RGB',(1232,650),'#0b1218');draw=ImageDraw.Draw(board)
            for col,e in enumerate(subset):
                left=12+col*616;draw.text((left,10),f'{e["number"]} · {e["title"]}',font=base.font(20,True),fill='#e0e7e7');draw.text((left,40),e['variant'],font=base.font(15),fill='#a4b9c6');board.paste(sequences[e['variant']][i],(left,66))
            boards.append(board)
        stem={'DROITE':'feu-final','CLOISON-BAS':'feu-final-cloison','INTERCEPTION':'feu-final-interception'}[folder]
        animations.append(save_animation(OUT/f'{stem}-64.gif',boards,24));boards[40].save(OUT/f'{stem}-impact.png');boards[61].save(OUT/f'{stem}-brulure.png')
    base.dump(OUT/'verification.json',{'protected_files_unchanged':len(baseline),'resampling_of_native_capture':False,'controls':controls,'animations':animations,'source_art_mode':'Built-in image_gen; prompt and source sheet saved locally.'})
    for source,name in [('src/agent_skill_lab_review/fire_painted/finale.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes());write_html(entries)
    print(json.dumps({'native_views':len(entries),'animations':len(animations),'protected_files_unchanged':len(baseline),'controls':'passed'}))

def write_html(entries):
    manifest=[{'variant':e['variant'],'folder':e['folder'],'id':e['id'],'frames':96,'fps':24,'fresh_burn':e['fresh_burn']} for e in entries]
    script='''const manifest=MANIFEST,sequences=new Map();let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,loadToken=0;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast');
function ui(){document.getElementById('status').textContent=launched?`Tour ${turn} · ${turn===0?'Impacts : 1 tour restant':turn<3?'Brûlure en cours':'Effets terminés'} · recharges : Boule de feu ${Math.max(0,8-turn)}, Combustion ${Math.max(0,3-turn)} tours`:'Visée · avant lancement';cast.disabled=launched;next.disabled=!launched||turn>=8;}
function frame(){const f=Math.floor(elapsed*24/1000);if(!launched)return f%17;if(turn===0)return f<43?17+f:36+(f-43)%24;if(turn===1)return 60+f%12;if(turn===2)return 72+f%12;return 84+f%12;}
async function load(){ready=false;const token=++loadToken,selected=scenario.value;document.getElementById('loading').textContent='Chargement…';const loaded=await Promise.all(manifest.filter(e=>e.folder===selected).map(async e=>{const images=await Promise.all(Array.from({length:96},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${e.variant}/${String(i).padStart(3,'0')}.png`;})));return [e.variant,images];}));if(token!==loadToken)return;sequences.clear();for(const[key,images]of loaded)sequences.set(key,images);elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · cases natives 64 px';}
function tick(now){if(last&&!paused)elapsed+=now-last;last=now;if(ready)for(const c of document.querySelectorAll('canvas')){const e=manifest.find(e=>e.folder===scenario.value&&e.id===c.dataset.skill);c.getContext('2d').drawImage(sequences.get(e.variant)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(launched&&turn<8){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!launched){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;ui();load().catch(()=>document.getElementById('loading').textContent='Chargement impossible.');};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};ui();load().catch(()=>document.getElementById('loading').textContent='Conserver le sous-dossier frames avec la galerie.');requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Boule de feu et Combustion</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:19px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}#status{color:#c4dcdc;font-weight:600}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Boule de feu et Combustion</h1><p class="muted">ANIM-S06-A03 · 13 / FE05-A03 · 14 / FE06-A02 · nouveaux rendus à valider</p><p>La boule part du lanceur, parcourt son trajet, puis explose sur la cible qu'elle touche. Si un monstre intercepte le projectile, l'explosion se produit sur ce monstre. Combustion concentre les flammes sur sa cible. Les impacts restent animés tant que vous ne passez pas leur tour.</p>
<div class="controls"><select id="scenario"><option value="DROITE">Zone ouverte · droite</option><option value="GAUCHE">Zone ouverte · gauche</option><option value="HAUT">Zone ouverte · haut</option><option value="INTERCEPTION">Un monstre intercepte la boule</option><option value="CLOISON-BAS">Cloison · bas · Combustion sans nouvelle Brûlure</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l'aperçu</button><button id="pause">Pause</button><span id="loading">Chargement…</span></div><p id="status"></p>
<main class="pair"><article><h2>13 · Boule de feu</h2><canvas data-skill="FE05" width="576" height="576"></canvas></article><article><h2>14 · Combustion</h2><canvas data-skill="FE06" width="576" height="576"></canvas></article></main>
<p>Dans les zones ouvertes, une nouvelle Brûlure est montrée après Combustion. Dans la scène de cloison, cet exemple ne reçoit pas de nouvelle Brûlure : l'effet ciblé disparaît au passage du tour. La cloison coupe l'explosion de Boule de feu et protège F.</p>
<p class="muted">Les cas de Brûlure sont préparés pour la revue. Le maintien visuel ne répète pas les dégâts. Les personnages gardent leurs dessins et leur pose de face. Les paramètres de compétences restent conservés ; l'empreinte circulaire et l'impact près du mur sont des scènes de revue, avant raccordement au combat.</p><p><a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> : lancement, tours manuels, directions, recharge et cas de Brûlure.</p><p class="muted">Dessins générés avec l'outil imagegen intégré : <a href="drawn-source.png">planche source</a> · <a href="imagegen-prompt.txt">prompt exact</a>.</p><script>SCRIPT</script></html>'''.replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args();prepare() if args.prepare else package()
