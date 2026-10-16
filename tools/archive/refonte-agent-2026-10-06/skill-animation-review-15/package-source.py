"""Native review of one-turn visual persistence, with manual browser turns."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_fire_cadence as previous
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-15'

def protected():
    hashes = previous.protected()
    for p in sorted(previous.OUT.rglob('*')):
        if p.is_file(): hashes[p.relative_to(base.ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes

def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT / 'review-protection.json').exists()
    base.dump(OUT / 'review-protection.json', protected())
    base.dump(OUT / 'review.json', {
        'batch':'ANIM-S05-TOURS01','status':'awaiting_corrected_visual_review',
        'candidate':{'number':12,'skill_id':'FE04','variant':'FE04-A05-TOUR'},
        'accepted_parts':'Review 14 drawing and slower cadence remain accepted.',
        'requested_change':'Visual effect stays animated until its game turn passes. Waiting in real time does not remove it.',
        'presentation':'Left is the earlier 0.8-second fade; right holds the same drawings until turn 2. Explicit turn 1 starts the eruption after its existing preparation. Turn 2 ends the large effect; attached Burn then continues for its remaining turns.',
        'scope':'Debug visual presence only. No damage repetition, extra duration of a gameplay effect, cooldown change, actor redraw or campaign integration.',
        'native_tile_size':64,'flame_texture_fps':8,
        'browser':'Manual +1-turn button. Starts during active eruption, so waiting indefinitely demonstrates persistence. Native laboratory supports manual launch, preparation, all four directions, obstacles and turns.',
    })
    launcher = (previous.OUT / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher.replace('--lot-feu-cadence','--lot-feu-tours'),encoding='utf-8-sig')

def package():
    baseline = json.loads((OUT / 'review-protection.json').read_text(encoding='utf-8'))
    assert protected() == baseline, 'Accepted drawings or previous reviews changed.'
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf-8'))
    timeline = json.loads((OUT / 'native/timeline.json').read_text(encoding='utf-8'))
    assert all(e['tile']==64 and e['frames']==144 and e['fps']==24 for e in entries)
    assert all(t['turn']==1 and t['cooldown']==5 and t['held_eruption_visible'] for t in timeline[33:102])
    assert not timeline[60]['old_eruption_visible'] and not timeline[96]['old_eruption_visible']
    assert all(not t['held_eruption_visible'] for t in timeline[102:])
    assert timeline[103]['turn']==2 and timeline[103]['cooldown']==4 and max(timeline[103]['burn'])==2
    assert max(timeline[143]['burn'])==0
    sequences, checks = {}, []
    for e in entries:
        destination = OUT / 'frames' / e['variant']; destination.mkdir(parents=True,exist_ok=True)
        x,y,w,h=e['crop']; bounds=(x,y,x+w,y+h); images=[]; differences={}
        for i in range(144):
            with Image.open(OUT / 'native' / f'{i:03}.png') as im:
                assert im.size==(1280,720)
                frame=im.crop(bounds).convert('RGB'); frame.save(destination/f'{i:03}.png'); images.append(frame)
        for phase in e['control_frames']:
            with Image.open(OUT/'native'/f'control-{phase:03}.png') as im:
                differences[phase]=ImageChops.difference(images[phase],im.crop(bounds).convert('RGB'))
            for cy,row in enumerate(e['map'].splitlines()):
                for cx,symbol in enumerate(row):
                    if symbol=='#':
                        assert differences[phase].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None
        assert differences[143].getbbox() is None
        # This strip is above the targets' attached Burn; it contains the large plume.
        high_flame=(288-42,155,288+42,188)
        if e['held']:
            assert differences[60].crop(high_flame).getbbox() and differences[96].crop(high_flame).getbbox()
            assert ImageChops.difference(images[60],images[63]).getbbox(), 'Held flame froze instead of animating.'
        else:
            assert differences[60].crop(high_flame).getbbox() is None
        assert differences[103].crop(high_flame).getbbox() is None
        assert differences[103].getbbox(), 'Attached Burn should remain after the eruption.'
        sequences[e['variant']]=images
        checks.append(save_animation(OUT/f"{e['variant']}-64.gif",images,24))
    boards=[]
    for i in range(144):
        board=Image.new('RGB',(1232,650),'#0b1218'); draw=ImageDraw.Draw(board)
        for column,e in enumerate(entries):
            left=12+column*616
            draw.text((left,10),'Avant · fin en secondes' if column==0 else '12 · Émanation · fin au prochain tour',font=base.font(20,True),fill='#e0e7e7')
            draw.text((left,40),f"Tour {timeline[i]['turn']} · recharge : {timeline[i]['cooldown']} tours",font=base.font(16),fill='#a4bac7')
            board.paste(sequences[e['variant']][i],(left,66))
        boards.append(board)
    checks.append(save_animation(OUT/'maintien-par-tours-64.gif',boards,24))
    boards[96].save(OUT/'attente-sans-passer-de-tour.png')
    base.dump(OUT/'verification.json',{'protected_files_unchanged':len(baseline),'resampling':False,
        'controls':{'one_turn_held_despite_real_time':True,'animated_while_waiting':True,'ends_at_next_turn':True,
                    'burn_continues_for_remaining_turns':True,'no_flames_on_walls':True,'final_effects_expired':True},
        'animations':checks,'timeline':'native/timeline.json'})
    for source,name in [('src/agent_skill_lab_review/fire_painted/zones.rs','render-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries)
    print(json.dumps({'animations':len(checks),'protected_files_unchanged':len(baseline),'turn_lifetime_controls':'passed'}))

def write_html(entries):
    manifest=[{'variant':e['variant'],'frames':144,'fps':24} for e in entries]
    script='''const manifest=MANIFEST,sequences=new Map();let ready=false,last=0,elapsed=0,paused=false,stage=2;
const next=document.getElementById('next'),cast=document.getElementById('cast'),status=document.getElementById('status');
function ui(){const turn=Math.max(0,stage-1),cd=stage===0?0:Math.max(0,6-turn);status.textContent=stage===0?'Visée · avant lancement':stage===1?'Préparation · en attente du premier tour':`Tour ${turn} · recharge : ${cd} tours · ${stage===2?'Éruption : 1 tour restant':stage<5?'Brûlure : '+(5-stage)+' tours restants':'Effets terminés'}`;cast.disabled=stage!==0&&cd>0;next.disabled=stage===0||stage>=7;}
function frame(){const f=Math.floor(elapsed*24/1000);if(stage===0)return f%17;if(stage===1)return 17+f%16;if(stage===2)return f<69?33+f:60+(f-69)%24;if(stage===3)return 102+f%12;if(stage===4)return 114+f%10;return 143;}
async function load(){for(const e of manifest){const images=await Promise.all(Array.from({length:e.frames},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${e.variant}/${String(i).padStart(3,'0')}.png`;})));sequences.set(e.variant,images);}elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · cases natives 64 px';}
function tick(now){if(last&&!paused)elapsed+=now-last;last=now;if(ready)for(const c of document.querySelectorAll('canvas'))c.getContext('2d').drawImage(sequences.get(c.dataset.variant)[frame()],0,0);requestAnimationFrame(tick);}
next.onclick=()=>{if(stage>0&&stage<7){stage++;elapsed=0;ui();}};cast.onclick=()=>{if(stage===0||stage>=7){stage=1;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{stage=0;elapsed=0;ui();};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};ui();load().catch(()=>document.getElementById('loading').textContent='Conserver le sous-dossier frames avec la galerie.');requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    cards=''.join(f'<article><h2>{"Avant · fin automatique" if not e["held"] else "12 · FE04-A05 · maintien par tours"}</h2><canvas data-variant="{e["variant"]}" width="576" height="576"></canvas></article>' for e in entries)
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Animations et tours</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:19px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}#status{color:#c4dcdc;font-weight:600}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Les flammes restent jusqu'au passage du tour</h1><p class="muted">ANIM-S05-TOURS01 · 12 / FE04-A05 · correction de durée visuelle</p><p>Le test démarre pendant l'éruption. Vous pouvez attendre aussi longtemps que vous le souhaitez : à droite, elle reste animée. Cliquez sur <strong>+1 tour</strong> pour la terminer. La Brûlure reste ensuite sur les corps pendant ses tours restants.</p>
<div class="controls"><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer la démonstration</button><button id="pause">Pause</button><span id="loading">Chargement…</span></div><p id="status"></p><main class="pair">CARDS</main>
<p class="muted">Le mouvement utilise la cadence ralentie validée. Attendre et mettre en pause ne passent aucun tour. Maintenir le dessin ne répète pas les dégâts. Les images de cette démonstration viennent du rendu natif du laboratoire.</p><p><a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> : visée, lancement, préparation, tours manuels, directions et obstacles.</p><script>SCRIPT</script></html>'''.replace('CARDS',cards).replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
